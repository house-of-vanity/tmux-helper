use clap::{App, Arg};

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
    // Parse opts and args
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        // Flags
        .arg(
            Arg::with_name("cpu")
                .short("c")
                .long("cpu")
                .help("Print cpu load bar.")
                .conflicts_with_all(&["mem", "mpris", "mpd", "localtime", "utctime"])
                .required(false),
        )
        .arg(
            Arg::with_name("mem")
                .short("m")
                .long("mem")
                .help("Print mem usage bar.")
                //              .conflicts_with("cpu")
                //              .conflicts_with("mpris")
                //              .conflicts_with("mpd")
                //              .conflicts_with("localtime")
                //              .conflicts_with("utctime")
                .required(false),
        )
        .arg(
            Arg::with_name("mpris")
                .short("p")
                .long("mpris")
                .help("Show player info using MPRIS2 interface.")
                //              .conflicts_with("cpu")
                //              .conflicts_with("mem")
                //              .conflicts_with("localtime")
                //              .conflicts_with("mpd")
                //              .conflicts_with("utctime")
                .required(false),
        )
        .arg(
            Arg::with_name("mpd")
                .short("d")
                .long("mpd")
                .help("Show mpd player using MPD native protocol.")
                //              .conflicts_with("cpu")
                //              .conflicts_with("mem")
                //              .conflicts_with("localtime")
                //              .conflicts_with("mpris")
                //              .conflicts_with("utctime")
                .required(false),
        )
        // Options
        .arg(
            Arg::with_name("localtime")
                .short("l")
                .long("localtime")
                .help("Local time")
                //              .conflicts_with_all(&["mem", "mpris", "mpd", "cpu", "utctime"])
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("utctime")
                .short("u")
                .long("utctime")
                .help("UTC time")
                //              .conflicts_with_all(&["mem", "mpris", "mpd", "cpu", "localtime"])
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("mpd_address")
                .short("a")
                .long("mpd-address")
                .help("<ADDR>:<PORT> of MPD server.")
                .takes_value(true)
                .default_value("127.0.0.1:6600")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_LOW")
                .long("COLOR_LOW")
                .help("CPU and MEM bar color while low usage.")
                .takes_value(true)
                .default_value("119")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_MID")
                .long("COLOR_MID")
                .help("CPU and MEM bar color while mid usage.")
                .takes_value(true)
                .default_value("220")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_HIGH")
                .long("COLOR_HIGH")
                .help("CPU and MEM bar color while high usage.")
                .takes_value(true)
                .default_value("197")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_TRACK_NAME")
                .long("COLOR_TRACK_NAME")
                .help("Color of track name filed.")
                .takes_value(true)
                .default_value("46")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_TRACK_ARTIST")
                .long("COLOR_TRACK_ARTIST")
                .help("Color of artist name filed.")
                .takes_value(true)
                .default_value("46")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_TRACK_TIME")
                .long("COLOR_TRACK_TIME")
                .help("Color of playing time field.")
                .takes_value(true)
                .default_value("153")
                .required(false),
        )
        .arg(
            Arg::with_name("COLOR_END")
                .long("COLOR_END")
                .help("Default color using to terminate others.")
                .takes_value(true)
                .default_value("153")
                .required(false),
        )
        .get_matches();

    // cpu          - cpu usage bar
    // mem          - mem usage bar
    // mpris        - player info using MPRIS2 interface
    // mpd          - player info using MPD native interface
    // utctime      - utc time
    // localtime    - local time
    // lt_format    - local time format
    // ut_format    - utc time format

    let lt_format = Some(match cli_args.value_of("localtime") {
        Some(format) => format.to_string(),
        None => "%H:%M".to_string(),
    });
    let ut_format = Some(match cli_args.value_of("utctime") {
        Some(format) => format.to_string(),
        None => "%H:%M".to_string(),
    });

    let mut cfg = Config {
        action: Action::Cpu,
        mpd_server: cli_args.value_of("mpd_address").unwrap().to_string(),
        lt_format: lt_format,
        ut_format: ut_format,
        color_low: colorize(cli_args.value_of("COLOR_LOW").unwrap().to_string()),
        color_mid: colorize(cli_args.value_of("COLOR_MID").unwrap().to_string()),
        color_high: colorize(cli_args.value_of("COLOR_HIGH").unwrap().to_string()),
        color_track_name: colorize(cli_args.value_of("COLOR_TRACK_NAME").unwrap().to_string()),
        color_track_artist: colorize(cli_args.value_of("COLOR_TRACK_ARTIST").unwrap().to_string()),
        color_track_time: colorize(cli_args.value_of("COLOR_TRACK_TIME").unwrap().to_string()),
        color_end: colorize(cli_args.value_of("COLOR_END").unwrap().to_string()),
    };

    if cli_args.is_present("cpu") {
        cfg.action = Action::Cpu;
    }
    if cli_args.is_present("mem") {
        cfg.action = Action::Mem;
    }
    if cli_args.is_present("localtime") {
        cfg.action = Action::Localtime;
    }
    if cli_args.is_present("utctime") {
        cfg.action = Action::Utctime;
    }
    if cli_args.is_present("mpris") {
        cfg.action = Action::Mpris;
    }
    if cli_args.is_present("mpd") {
        cfg.action = Action::Mpd;
    }
    cfg
}
