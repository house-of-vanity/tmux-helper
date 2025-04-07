use clap::{Arg, Command};

#[derive(Debug)]
pub enum Action {
    Mem,
    Cpu,
    Mpris,
    Mpd,
    Localtime,
    Utctime,
}

impl Default for Action {
    fn default() -> Action {
        Action::Cpu
    }
}

#[derive(Default, Debug)]
pub struct Config {
    pub action: Action,
    pub mpd_server: String,
    pub lt_format: Option<String>,
    pub ut_format: Option<String>,
    pub bar_symbol: Option<String>,
    pub bar_empty_symbol: Option<String>,
    pub low_threshold: f32,
    pub mid_threshold: f32,
    pub color_low: String,
    pub color_mid: String,
    pub color_high: String,
    pub color_track_name: String,
    pub color_track_artist: String,
    pub color_track_time: String,
    pub color_end: String,
}

fn colorize(color: String) -> String {
    format!("#[fg=colour{}]", color)
}

pub fn read() -> Config {
    let cli_args = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("cpu")
                .short('c')
                .long("cpu")
                .help("Print cpu load bar.")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with_all(["mem", "mpris", "mpd", "localtime", "utctime"]),
        )
        .arg(
            Arg::new("mem")
                .short('m')
                .long("mem")
                .action(clap::ArgAction::SetTrue)
                .help("Print mem usage bar."),
        )
        .arg(
            Arg::new("low")
                .long("low")
                .help("Low threshold (0.0 - 1.0)")
                .value_parser(clap::value_parser!(f32))
                .default_value("0.7"),
        )
        .arg(
            Arg::new("mid")
                .long("mid")
                .help("Mid threshold (0.0 - 1.0)")
                .value_parser(clap::value_parser!(f32))
                .default_value("0.9"),
        )
        .arg(
            Arg::new("mpris")
                .short('p')
                .long("mpris")
                .action(clap::ArgAction::SetTrue)
                .help("Show player info using MPRIS2 interface."),
        )
        .arg(
            Arg::new("mpd")
                .short('d')
                .long("mpd")
                .action(clap::ArgAction::SetTrue)
                .help("Show mpd player using MPD native protocol."),
        )
        .arg(
            Arg::new("localtime")
                .short('l')
                .long("localtime")
                .help("Local time")
                .num_args(0..=1)
                .default_missing_value("%H:%M"),
        )
        .arg(
            Arg::new("utctime")
                .short('u')
                .long("utctime")
                .help("UTC time")
                .num_args(0..=1)
                .default_missing_value("%H:%M"),
        )
        .arg(
            Arg::new("bar_symbol")
                .short('s')
                .long("symbol")
                .help("Symbol to build bar")
                .num_args(0..=1)
                .default_value("▮"),
        )
        .arg(
            Arg::new("bar_empty_symbol")
                .short('e')
                .long("empty-symbol")
                .help("Symbol to represent the empty part of the bar")
                .num_args(0..=1)
                .default_value("▯"),
        )
        .arg(
            Arg::new("mpd_address")
                .short('a')
                .long("mpd-address")
                .help("<ADDR>:<PORT> of MPD server.")
                .default_value("127.0.0.1:6600"),
        )
        .arg(
            Arg::new("COLOR_LOW")
                .long("COLOR_LOW")
                .help("CPU and MEM bar color while low usage.")
                .default_value("119"),
        )
        .arg(
            Arg::new("COLOR_MID")
                .long("COLOR_MID")
                .help("CPU and MEM bar color while mid usage.")
                .default_value("220"),
        )
        .arg(
            Arg::new("COLOR_HIGH")
                .long("COLOR_HIGH")
                .help("CPU and MEM bar color while high usage.")
                .default_value("197"),
        )
        .arg(
            Arg::new("COLOR_TRACK_NAME")
                .long("COLOR_TRACK_NAME")
                .help("Color of track name filed.")
                .default_value("46"),
        )
        .arg(
            Arg::new("COLOR_TRACK_ARTIST")
                .long("COLOR_TRACK_ARTIST")
                .help("Color of artist name filed.")
                .default_value("46"),
        )
        .arg(
            Arg::new("COLOR_TRACK_TIME")
                .long("COLOR_TRACK_TIME")
                .help("Color of playing time field.")
                .default_value("153"),
        )
        .arg(
            Arg::new("COLOR_END")
                .long("COLOR_END")
                .help("Default color using to terminate others.")
                .default_value("153"),
        )
        .get_matches();

    let lt_format = cli_args
        .get_one::<String>("localtime")
        .map(|s| s.to_string());
    let ut_format = cli_args.get_one::<String>("utctime").map(|s| s.to_string());
    let bar_symbol = cli_args
        .get_one::<String>("bar_symbol")
        .map(|s| s.to_string());
    let bar_empty_symbol = cli_args
        .get_one::<String>("bar_empty_symbol")
        .map(|s| s.to_string());

    let mut cfg = Config {
        action: Action::Cpu,
        mpd_server: cli_args
            .get_one::<String>("mpd_address")
            .unwrap()
            .to_string(),
        lt_format,
        ut_format,
        bar_symbol,
        bar_empty_symbol,
        low_threshold: *cli_args.get_one::<f32>("low").unwrap(),
        mid_threshold: *cli_args.get_one::<f32>("mid").unwrap(),
        color_low: colorize(cli_args.get_one::<String>("COLOR_LOW").unwrap().to_string()),
        color_mid: colorize(cli_args.get_one::<String>("COLOR_MID").unwrap().to_string()),
        color_high: colorize(
            cli_args
                .get_one::<String>("COLOR_HIGH")
                .unwrap()
                .to_string(),
        ),
        color_track_name: colorize(
            cli_args
                .get_one::<String>("COLOR_TRACK_NAME")
                .unwrap()
                .to_string(),
        ),
        color_track_artist: colorize(
            cli_args
                .get_one::<String>("COLOR_TRACK_ARTIST")
                .unwrap()
                .to_string(),
        ),
        color_track_time: colorize(
            cli_args
                .get_one::<String>("COLOR_TRACK_TIME")
                .unwrap()
                .to_string(),
        ),
        color_end: colorize(cli_args.get_one::<String>("COLOR_END").unwrap().to_string()),
    };

    if cli_args.get_flag("cpu") {
        cfg.action = Action::Cpu;
    }
    if cli_args.get_flag("mem") {
        cfg.action = Action::Mem;
    }
    if cli_args.contains_id("localtime") {
        cfg.action = Action::Localtime;
    }
    if cli_args.contains_id("utctime") {
        cfg.action = Action::Utctime;
    }
    if cli_args.get_flag("mpris") {
        cfg.action = Action::Mpris;
    }
    if cli_args.get_flag("mpd") {
        cfg.action = Action::Mpd;
    }

    cfg
}
