use std::fs;
use std::env;
use sys_info;

const LOW: &str = "#[fg=colour186]";
const MID: &str = "#[fg=colour208]";
const HIGH: &str = "#[fg=colour160]";
const END: &str = "#[fg=colour7]";

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Cant read file.")
}

fn mem() {
    let memory;
    match sys_info::mem_info() {
        Err(w) => panic!("{:?}", w),
        Ok(mem_data) => memory = mem_data,
    }
    let mem_color: &str;
    if memory.free+memory.cached <= memory.total / 100 {
        mem_color = HIGH;
    } else if memory.free+memory.cached <= memory.total / 30{
        mem_color = MID;
    } else {
        mem_color = LOW;
    }
    let mem_total = memory.total as f32;
    println!("MEM: {:.1}GiB avail:{}{}MiB{}",
        mem_total/1024./1024.,
        mem_color,
        memory.free/1024+memory.cached/1024,
        END);
}

fn cpu_load() {
    let load = read_file("/proc/loadavg");
    let load_data = load.split_whitespace().collect::<Vec<&str>>();
    let _cpu_count = read_file("/proc/cpuinfo");
    let cpu_count = _cpu_count.matches("model name").count();
    let one: f32 = load_data[0].parse().unwrap();
    let five: f32 = load_data[1].parse().unwrap();
    let fiveteen: f32 = load_data[2].parse().unwrap();
    let load_color: &str;
    if one + five + fiveteen > (cpu_count * 3) as f32 {
        load_color = HIGH;
    } else if one + five + fiveteen > cpu_count as f32{
        load_color = MID;
    } else {
        load_color = LOW;
    }

    println!("CPU: {}|{}{:.2} {:.2} {:.2}{}",
        cpu_count,
        load_color,
        one,
        five,
        fiveteen,
        END);
}


fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
        1 => {
            panic!("Available commands -m, -c");
        },
        2 => {
            match args[1].as_ref() {
                "-c" => cpu_load(),
                "-m" => mem(),
                _ => panic!("Available commands -m, -c"),
            }
        },
        _ => {
            panic!("Available commands -m, -c");
        }
  }
}
