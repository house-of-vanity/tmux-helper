extern crate chrono;
extern crate dbus;

use crate::dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use chrono::{DateTime, Local, Utc};
use dbus::{arg, blocking::Connection};
use std::{env, time::Duration};
use sys_info;

const LOW: &str = "#[fg=colour119]";
const MID: &str = "#[fg=colour220]";
const HIGH: &str = "#[fg=colour197]";
const END: &str = "#[fg=colour153]";
const TRACK_NAME: &str = "#[fg=colour46]";
const TRACK_ARTIST: &str = "#[fg=colour46]";
const TRACK_TIME: &str = "#[fg=colour153]";

#[derive(Debug, Clone)]
struct TrackInfo {
    title: String,
    artist: String,
    position: String,
    duration: String,
    status: String,
}

fn to_bar(value: i32, max: i32, low: f32, mid: f32) {
    let mut bar = "".to_string();
    let bar_sym = "▮".to_string();
    if (value as f32) / (max as f32) < low {
        bar.push_str(LOW);
    } else if (value as f32) / (max as f32) < mid {
        bar.push_str(MID);
    } else {
        bar.push_str(HIGH);
    }
    for i in 0..max {
        if i < value as i32 {
            bar.push_str(&bar_sym);
        } else {
            bar.push_str(" ")
        }
    }
    bar.push_str(END);
    bar.push_str("|");
    print!("{}", bar)
}

fn mem_load_bar(bar_len: i32) {
    let memory;
    match sys_info::mem_info() {
        Err(w) => panic!("{:?}", w),
        Ok(mem_data) => memory = mem_data,
    }
    let len =
        ((memory.total - memory.avail) as f32 / (memory.total as f32) * bar_len as f32) as i32;
    to_bar(len, bar_len, 0.7, 0.9);
    print!("{:.0} MiB#[default]", memory.avail / 1024);
}

fn cpu_load_bar(bar_len: i32) {
    let cpu_count = match sys_info::cpu_num() {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e),
    };
    let la_one: f32 = match sys_info::loadavg() {
        Ok(l) => l.one as f32,
        Err(e) => panic!("{:?}", e),
    };
    let len: f32 = la_one as f32 / cpu_count as f32 * bar_len as f32;
    to_bar(len as i32, bar_len, 0.3, 0.7);
    print!("{:.2} LA1#[default]", la_one);
}

fn get_player() -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn player_info(players: Vec<String>) -> Result<TrackInfo, Box<dyn std::error::Error>> {
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

fn format_time(sec: i64) -> String {
    let minutes = sec / 60;
    let secondes = sec % 60;
    let result = format!("{:02}:{:02}", minutes, secondes);
    result.to_string()
}

fn get_time(utc: bool, mut format: &str) {
    // Format reference: https://docs.rs/chrono/0.4.10/chrono/format/strftime/index.html
    if format.len() == 0 {
        format = "%H:%M";
    }
    if utc {
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        println!("{}", utc_time.format(format));
    } else {
        let local_time = Local::now();
        println!("{}", local_time.format(format));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let help_text: &str = "Available commands -mb, -cb, -tl <TIME FORMAT>, -tu <TIME FORMAT>, -p";
    match args.len() {
        1 => {
            panic!(help_text);
        }
        2 => match args[1].as_ref() {
            "-cb" => cpu_load_bar(15),
            "-mb" => mem_load_bar(15),
            "-tl" => get_time(false, ""),
            "-tu" => get_time(true, ""),
            "-p" => match player_info(get_player().unwrap()) {
                Ok(mut track_info) => {
                    let mut title_len = 30;
                    let mut artist_len = 30;
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
                        let mut artist: String = String::new();
                        let mut counter = 0;
                        for ch in track_info.artist.chars() {
                            if counter == artist_len {
                                artist.push_str("..");
                                break;
                            }
                            artist.push(ch);
                            counter += 1;
                        }
                        track_info.artist = artist;
                    }
                    if title_len + max_shift >= track_info.title.chars().count() {
                        title_len = track_info.title.chars().count()
                    }
                    if track_info.title.len() > title_len {
                        let mut title: String = String::new();
                        let mut counter = 0;
                        for ch in track_info.title.chars() {
                            if counter == title_len {
                                title.push_str("..");
                                break;
                            }
                            title.push(ch);
                            counter += 1;
                        }
                        track_info.title = title;
                    }
                    println!(
                        "#[none]#[bold]{}{}{}#[none]{}{}{}{} {}[{}/{}] {} {}#[default]",
                        TRACK_NAME,
                        track_info.title,
                        END,
                        separator,
                        TRACK_ARTIST,
                        track_info.artist,
                        END,
                        TRACK_TIME,
                        track_info.position,
                        track_info.duration,
                        track_info.status,
                        END,
                    );
                }
                Err(_e) => println!("No music playing"),
            },
            _ => panic!(help_text),
        },
        3 => match args[1].as_ref() {
            "-tl" => get_time(false, args[2].as_ref()),
            "-tu" => get_time(true, args[2].as_ref()),
            _ => panic!(help_text),
        },
        _ => {
            panic!(help_text);
        }
    }
}
