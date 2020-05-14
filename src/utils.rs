use crate::config;
use crate::dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use chrono::{DateTime, Local, Utc};
use dbus::{arg, blocking::Connection};
use std::time::Duration;
use sys_info;
use mpd::Client;
use std::process;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    title: String,
    artist: String,
    position: String,
    duration: String,
    status: String,
}

pub fn to_bar(value: i32, max: i32, low: f32, mid: f32, config: &config::Config) {
    let mut bar = "".to_string();
    let bar_sym = "▮".to_string();
    if (value as f32) / (max as f32) < low {
        bar.push_str(&config.color_low);
    } else if (value as f32) / (max as f32) < mid {
        bar.push_str(&config.color_mid);
    } else {
        bar.push_str(&config.color_high);
    }
    for i in 0..max {
        if i < value as i32 {
            bar.push_str(&bar_sym);
        } else {
            bar.push_str(" ")
        }
    }
    bar.push_str(&config.color_end);
    bar.push_str("|");
    print!("{}", bar)
}

pub fn mem_load_bar(bar_len: i32, config: &config::Config) {
    let memory;
    match sys_info::mem_info() {
        Err(w) => panic!("{:?}", w),
        Ok(mem_data) => memory = mem_data,
    }
    let len =
        ((memory.total - memory.avail) as f32 / (memory.total as f32) * bar_len as f32) as i32;
    to_bar(len, bar_len, 0.7, 0.9, config);
    print!("{:.0} MiB#[default]", memory.avail / 1024);
}

pub fn cpu_load_bar(bar_len: i32, config: &config::Config) {
    let cpu_count = match sys_info::cpu_num() {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e),
    };
    let la_one: f32 = match sys_info::loadavg() {
        Ok(l) => l.one as f32,
        Err(e) => panic!("{:?}", e),
    };
    let len: f32 = la_one as f32 / cpu_count as f32 * bar_len as f32;
    to_bar(len as i32, bar_len, 0.3, 0.7, config);
    print!("{:.2} LA1#[default]", la_one);
}

pub fn get_player() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
    let mut players: Vec<String> = Vec::new();
    for name in names {
        if name.contains("org.mpris.MediaPlayer2") {
            players.push(name);
        }
    }

    Ok(players)
}

pub fn player_info(players: Vec<String>) -> Result<TrackInfo, Box<dyn std::error::Error>> {
    let mut players_vec: Vec<TrackInfo> = Vec::new();
    for player in players {
        let mut track_info = TrackInfo {
            artist: "".to_string(),
            title: "".to_string(),
            position: "".to_string(),
            duration: "".to_string(),
            status: "".to_string(),
        };
        let conn = Connection::new_session()?;
        let proxy = conn.with_proxy(
            player,
            "/org/mpris/MediaPlayer2",
            Duration::from_millis(5000),
        );
        let metadata: Box<dyn arg::RefArg> =
            proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")?;
        let mut iter = metadata.as_iter().unwrap();
        while let Some(key) = iter.next() {
            if key.as_str() == Some("xesam:title") {
                if let Some(title) = iter.next().unwrap().as_str() {
                    track_info.title = title.to_string();
                }
            }
            if key.as_str() == Some("mpris:length") {
                if let Some(length) = iter.next().unwrap().as_i64() {
                    track_info.duration = format_time(length / 1000000);
                }
            }
            if key.as_str() == Some("xesam:artist") {
                if let Some(mut artists) = iter.next().unwrap().as_iter() {
                    while let Some(artist) = artists.next() {
                        if let Some(mut line) = artist.as_iter() {
                            track_info.artist = line.next().unwrap().as_str().unwrap().to_string();
                        }
                    }
                }
            }
        }
        let position: Box<dyn arg::RefArg> =
            proxy.get("org.mpris.MediaPlayer2.Player", "Position")?;
        track_info.position = format_time(position.as_i64().unwrap() / 1000000);
        // ugly
        let _status_text_box: Box<dyn arg::RefArg> =
            proxy.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;
        let _status_text = _status_text_box.as_str().unwrap();
        match _status_text.as_ref() {
            "Playing" => track_info.status = "▶".to_string(),
            "Paused" => track_info.status = "⏸".to_string(),
            _ => track_info.status = "⏹".to_string(),
        };
        players_vec.push(track_info);
    }
    for player in &players_vec {
        if player.status == "▶".to_string() {
            return Ok(player.clone());
        }
    }
    Ok(players_vec[players_vec.len() - 1].clone())
}

pub fn format_time(sec: i64) -> String {
    let minutes = sec / 60;
    let secondes = sec % 60;
    let result = format!("{:02}:{:02}", minutes, secondes);
    result.to_string()
}

pub fn get_time(utc: bool, format: Option<String>) {
    // Format reference: https://docs.rs/chrono/0.4.10/chrono/format/strftime/index.html
    let fmt = match format {
        Some(format) => format,
        None => "%H:%M".to_string(),
    };

    if utc {
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        println!("{}", utc_time.format(&fmt));
    } else {
        let local_time = Local::now();
        println!("{}", local_time.format(&fmt));
    }
}

fn format_player(track_info: TrackInfo, config: &config::Config) {
    let mut title_len = 30;
    let mut artist_len = 30;
    let mut artist_line: String = String::new();
    let mut title_line: String = String::new();
    let mut separator: String = " — ".to_string();
    let max_shift = 6;
    if track_info.artist.chars().count() == 0 {
        separator = "".to_string();
        title_len += artist_len;
    }
    if artist_len + max_shift >= track_info.artist.chars().count() {
        artist_len = track_info.artist.chars().count()
    }
    if track_info.artist.len() > artist_len {
        let mut counter = 0;
        for ch in track_info.artist.chars() {
            if counter == artist_len {
                artist_line.push_str("..");
                break;
            }
            artist_line.push(ch);
            counter += 1;
        }

    }
    if title_len + max_shift >= track_info.title.chars().count() {
        title_len = track_info.title.chars().count()
    }
    if track_info.title.len() > title_len {
        let mut counter = 0;
        for ch in track_info.title.chars() {
            if counter == title_len {
                title_line.push_str("..");
                break;
            }
            title_line.push(ch);
            counter += 1;
        }
    }
    println!(
        "#[none]#[bold]{}{}{}#[none]{}{}{}{} {}[{}/{}] {} {}#[default]",
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
        config.color_end,
    );
}

pub fn mpris(config: &config::Config) {
    match player_info(get_player().unwrap()) {
        Ok(track_info) => format_player(track_info, config),
        Err(_e) => println!("No music playing"),
    }
}

pub fn mpd(config: &config::Config) {
    let mut conn = match Client::connect(&config.mpd_server) {
        Ok(conn) => conn,
        Err(e) => {println!("Can't connect to MPD server. {}", e); process::exit(0x0001)}
    };
    let mut track_info = TrackInfo {
        title: String::new(),
        artist: String::new(),
        position: String::new(),
        duration: String::new(),
        status: String::new(),
    };
    if let Some(song) = conn.currentsong().unwrap() {
        if let Some(title) = song.title {
            track_info.title = title
        }
        if let Some(artist) = song.tags.get("Artist") {
            track_info.artist = artist.to_string()
        }
    }
    if let Some(time) = conn.status().unwrap().time {
            track_info.position = format_time(time.0.num_seconds() as i64);
            track_info.duration = format_time(time.2.num_seconds() as i64);
    }
    let status = match conn.status() {
        Ok(status) => {
            match status.state {
                mpd::State::Play => "▶".to_string(),
                mpd::State::Pause => "⏸".to_string(),
                mpd::State::Stop => "⏹".to_string(),
            }
        }
        Err(_) => {"⏹".to_string()},
    };
    track_info.status = status;
    format_player(track_info, config)
}
