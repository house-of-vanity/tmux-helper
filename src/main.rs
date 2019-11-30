extern crate dbus;
use crate::dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::{arg, blocking::Connection};
use std::{collections::HashMap, env, fs, time::Duration};
use sys_info;

const LOW: &str = "#[fg=colour186]";
const MID: &str = "#[fg=colour208]";
const HIGH: &str = "#[fg=colour160]";
const END: &str = "#[fg=colour7]";

struct Track_info<'a> {
    title: &'a str,
    artist: &'a str,
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Cant read file.")
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
    print!("{:.0} MiB", memory.avail / 1024);
}

fn cpu_load_bar(bar_len: i32) {
    let load = read_file("/proc/loadavg");
    let load_data = load.split_whitespace().collect::<Vec<&str>>();
    let _cpu_count = read_file("/proc/cpuinfo");
    let cpu_count = _cpu_count.matches("model name").count();
    let one: f32 = load_data[0].parse().unwrap();
    let len: f32 = one as f32 / cpu_count as f32 * bar_len as f32;
    to_bar(len as i32, bar_len, 0.3, 0.7);
    print!("{:.2} LA1", one);
}

fn print_refarg(value: &dyn arg::RefArg) {
    if let Some(s) = value.as_str() {
        println!("{}", s);
    } else if let Some(i) = value.as_i64() {
        println!("{}", i);
    } else if let Some(mut c) = value.as_iter() {
        while let Some(val) = c.next() {
            if let Some(mut line) = val.as_iter() {
                print!("{:?}", line.next().unwrap());
            }
        }
        println!("");
    } else {
        println!("{:?}", value);
    }
}

//fn player_info(player: &str) -> Result<(), Box<dyn std::error::Error>> {
fn player_info<'a>(player: &'a str) -> Result<Track_info, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let mut service: String = "org.mpris.MediaPlayer2.".to_owned();
    service.push_str(player);
    let proxy = conn.with_proxy(
        service,
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );
    let metadata: Box<dyn arg::RefArg> = proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")?;
    let mut iter = metadata.as_iter().unwrap();
    let mut track_info = Track_info {
        artist: "",
        title: "",
    };
    while let Some(key) = iter.next() {
        if key.as_str() == Some("xesam:title") {
            if let Some(title) = iter.next().unwrap().as_str() {
                track_info.title = title;
            }
        }
        if key.as_str() == Some("xesam:artist") {
            if let Some(mut artists) = iter.next().unwrap().as_iter() {
                while let Some(artist) = artists.next() {
                    if let Some(mut line) = artist.as_iter() {
                        track_info.artist = line.next().unwrap().as_str().unwrap();
                    }
                }
            }
        }
    }
    //Ok(Box::new(track_info))
    Ok(track_info)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let help_text: &str = "Available commands -mb, -cb";
    match args.len() {
        1 => {
            panic!(help_text);
        }
        2 => match args[1].as_ref() {
            "-cb" => cpu_load_bar(15),
            "-mb" => mem_load_bar(15),
            "-p" => {
                match player_info("cmus") {
                    Ok(track_info) => println!("{:?}, {:?}", track_info.title, track_info.artist),
                    Err(e) => panic!("Error"),
                }
            }
            _ => panic!(help_text),
        },
        _ => {
            panic!(help_text);
        }
    }
}
