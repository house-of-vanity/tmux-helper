use crate::config;
use chrono::{Local, Utc};
use dbus::arg::RefArg;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::{arg, blocking::Connection};
use mpd::Client;
use size_format::SizeFormatterBinary;
use std::process;
use std::thread;
use std::time::Duration;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub position: String,
    pub duration: String,
    pub status: String,
}

pub fn to_bar(value: i32, max: i32, low: f32, mid: f32, config: &config::Config) {
    let mut bar = "".to_string();
    let ratio = (value as f32) / (max as f32);
    bar.push_str(if ratio < low {
        &config.color_low
    } else if ratio < mid {
        &config.color_mid
    } else {
        &config.color_high
    });
    let symbol = config.bar_symbol.as_deref().unwrap_or("█");
    let empty = config.bar_empty_symbol.as_deref().unwrap_or(" ");
    for i in 0..max {
        bar.push_str(if i < value { symbol } else { empty });
    }
    bar.push_str(&config.color_end);
    bar.push('|');
    print!("{}", bar);
}

pub fn mem_load_bar(bar_len: i32, config: &config::Config) {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    
    // On macOS sysinfo.used_memory() includes caches and compressed memory
    // Try a more conservative estimate
    // Usually real used memory is about 30-50% of what sysinfo shows
    #[cfg(target_os = "macos")]
    let actual_used = used_memory * 40 / 100; // Approximately 40% of sysinfo.used_memory
    
    #[cfg(not(target_os = "macos"))]
    let actual_used = used_memory;
    
    let actual_free = total_memory - actual_used;
    
    let used_ratio = actual_used as f32 / total_memory as f32;
    let len = (used_ratio * bar_len as f32) as i32;
    
    to_bar(
        len,
        bar_len,
        config.low_threshold,
        config.mid_threshold,
        config,
    );
    
    // Show: used/free
    print!(
        "{}/{} #[default]",
        SizeFormatterBinary::new(actual_used),
        SizeFormatterBinary::new(actual_free)
    );
}

pub fn cpu_load_bar(bar_len: i32, config: &config::Config) {
    let mut sys = System::new_all();
    
    // Update CPU information
    sys.refresh_cpu_all();
    
    // Wait a bit to get accurate CPU usage data
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    
    let cpu_count = sys.cpus().len();
    
    // Get average CPU usage
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpu_count as f32;
    let cpu_load_ratio = cpu_usage / 100.0; // sysinfo returns percentages
    
    let len = (cpu_load_ratio * bar_len as f32).round() as i32;
    
    to_bar(
        len,
        bar_len,
        config.low_threshold,
        config.mid_threshold,
        config,
    );

    print!("{:.1}% CPU#[default]", cpu_usage);
}

pub fn get_player() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(5));
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
    Ok(names
        .into_iter()
        .filter(|n| n.contains("org.mpris.MediaPlayer2"))
        .collect())
}

pub fn player_info(players: Vec<String>) -> Result<TrackInfo, Box<dyn std::error::Error>> {
    for player in players {
        let conn = Connection::new_session()?;
        let proxy = conn.with_proxy(player, "/org/mpris/MediaPlayer2", Duration::from_secs(5));
        let metadata: arg::PropMap = proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

        let title = metadata
            .get("xesam:title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let artist = metadata
            .get("xesam:artist")
            .and_then(|v| v.as_iter())
            .and_then(|mut artists| artists.next().and_then(|a| a.as_str()))
            .unwrap_or("")
            .to_string();
        let duration_us = metadata
            .get("mpris:length")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let position_us: i64 = proxy.get("org.mpris.MediaPlayer2.Player", "Position")?;
        let status_text: String = proxy.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;

        let status = match status_text.as_str() {
            "Playing" => "▶",
            "Paused" => "⏸",
            _ => "⏹",
        }
        .to_string();

        let track_info = TrackInfo {
            title,
            artist,
            position: format_time(position_us / 1_000_000),
            duration: format_time(duration_us / 1_000_000),
            status,
        };

        if track_info.status == "▶" {
            return Ok(track_info);
        }
    }
    Err("No active player".into())
}

pub fn format_time(sec: i64) -> String {
    format!("{:02}:{:02}", sec / 60, sec % 60)
}

pub fn get_time(utc: bool, format: Option<String>) {
    let fmt = format.unwrap_or_else(|| "%H:%M".to_string());
    let now = if utc {
        Utc::now().format(&fmt)
    } else {
        Local::now().format(&fmt)
    };
    println!("{}", now);
}

fn shorten(line: &str, max_len: usize) -> String {
    if line.chars().count() > max_len {
        format!("{}..", line.chars().take(max_len).collect::<String>())
    } else {
        line.to_string()
    }
}

fn format_player(track_info: TrackInfo, config: &config::Config) {
    let separator = if track_info.artist.is_empty() {
        ""
    } else {
        " — "
    };
    let max_len = if track_info.artist.is_empty() { 60 } else { 30 };

    let artist_line = shorten(&track_info.artist, max_len);
    let title_line = shorten(&track_info.title, max_len);

    if track_info.position == "00:00" || track_info.duration.is_empty() {
        println!(
            "#[bold]{}{}{}{}{}{} {}{} {}#[default]",
            config.color_track_name,
            title_line,
            config.color_end,
            separator,
            config.color_track_artist,
            artist_line,
            config.color_end,
            config.color_track_time,
            track_info.status
        );
    } else {
        println!(
            "#[bold]{}{}{}{}{}{} {}[{}/{}] {}{}{}#[default]",
            config.color_track_name,
            title_line,
            config.color_end,
            separator,
            config.color_track_artist,
            artist_line,
            config.color_end,
            config.color_track_time,
            track_info.position,
            track_info.duration,
            track_info.status,
            config.color_end
        );
    }
}

pub fn mpris(config: &config::Config) {
    match player_info(get_player().unwrap_or_default()) {
        Ok(track_info) => format_player(track_info, config),
        Err(_) => println!("No music playing"),
    }
}

pub fn mpd(config: &config::Config) {
    let mut conn = Client::connect(&config.mpd_server).unwrap_or_else(|e| {
        println!("Can't connect to MPD server. {}", e);
        process::exit(1);
    });

    let song = conn.currentsong().unwrap_or(None);
    let status = conn.status().unwrap();

    let artist = song
        .as_ref()
        .and_then(|s| s.tags.iter().find(|(k, _)| k == "Artist").map(|(_, v)| v))
        .cloned()
        .unwrap_or_default();

    let title = song
        .as_ref()
        .and_then(|s| s.title.clone().or_else(|| s.name.clone()))
        .unwrap_or_default();

    let (position, duration) = status.time.unwrap_or_default();

    let track_info = TrackInfo {
        title,
        artist,
        position: format_time(position.as_secs() as i64),
        duration: format_time(duration.as_secs() as i64),
        status: match status.state {
            mpd::State::Play => "▶",
            mpd::State::Pause => "⏸",
            mpd::State::Stop => "⏹",
        }
        .to_string(),
    };

    format_player(track_info, config);
}
