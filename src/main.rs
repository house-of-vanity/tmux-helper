mod config;
mod utils;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub position: String,
    pub duration: String,
    pub status: String,
}

fn main() {
    let conf = config::read();

    match conf.action {
        config::Action::Cpu => utils::cpu_load_bar(15, &conf),
        config::Action::Mem => utils::mem_load_bar(15, &conf),
        config::Action::Mpris => utils::mpris(&conf),
        config::Action::Utctime => utils::get_time(true, conf.ut_format),
        config::Action::Localtime => utils::get_time(false, conf.lt_format),
        config::Action::Mpd => utils::mpd(&conf),
    }
}
