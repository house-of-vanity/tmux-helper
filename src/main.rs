extern crate dbus;

use std::fs;
use std::env;
use std::time::Duration;
use dbus::blocking::Connection;
use sys_info;

const LOW: &str = "#[fg=colour186]";
const MID: &str = "#[fg=colour208]";
const HIGH: &str = "#[fg=colour160]";
const END: &str = "#[fg=colour7]";

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Cant read file.")
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
        } else {bar.push_str(" ")}
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
    let len = ((memory.total - memory.avail) as f32 / (memory.total as f32) * bar_len as f32) as i32;
    to_bar(len, bar_len, 0.7, 0.9);
    print!("{:.0} MiB", memory.avail/1024);
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

fn player_info() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.mpris.MediaPlayer2.cmus", "/org/mpris/MediaPlayer2", Duration::from_millis(5000));

    // Now make the method call. The ListNames method call takes zero input parameters and
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
    let (names,): (Vec<String>,) = proxy.method_call("org.mpris.MediaPlayer2.cmus", "Metadata", ())?;

    // Let's print all the names to stdout.
    for name in names { println!("{}", name); }

    Ok(())
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let help_text: &str = "Available commands -mb, -cb";
  match args.len() {
        1 => {
            panic!(help_text);
        },
        2 => {
            match args[1].as_ref() {
                "-cb" => cpu_load_bar(15),
                "-mb" => mem_load_bar(15),
                "-p" => { let x = player_info();},
                _ => panic!(help_text),
            }
        },
        _ => {
            panic!(help_text);
        }
  }
}
