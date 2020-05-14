extern crate chrono;
extern crate dbus;
extern crate mpd;

mod config;
mod utils;

#[derive(Debug, Clone)]
struct TrackInfo {
    title: String,
    artist: String,
    position: String,
    duration: String,
    status: String,
}

fn main() {
    let conf = config::read();
    // cpu          - cpu usage bar
    // mem          - mem usage bar
    // mpris        - player info using MPRIS2 interface
    // mpd          - player info using MPD native interface
    // utctime      - utc time
    // localtime    - local time
    match conf.action {
        config::Action::Cpu => utils::cpu_load_bar(15, &conf),
        config::Action::Mem => utils::mem_load_bar(15, &conf),
        config::Action::Mpris => utils::mpris(&conf),
        config::Action::Utctime => utils::get_time(true, conf.ut_format),
        config::Action::Localtime => utils::get_time(false, conf.lt_format),
        config::Action::Mpd => utils::mpd(&conf),
    }
}
