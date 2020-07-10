# Tmux helper
Small app that perform system check and print TMUX friendly output.

![Preview](.github/prev.png)

### Building 
`cargo build --release`
or get binary on release page

### Fetures
```shell
tmux-helper 0.3.2
Ultra Desu <ultradesu@hexor.ru>
Utility for printing system info for tmux status line.

USAGE:
    tmux-helper [FLAGS] [OPTIONS]

FLAGS:
    -c, --cpu        Print cpu load bar.
    -h, --help       Prints help information
    -m, --mem        Print mem usage bar.
    -d, --mpd        Show mpd player using MPD native protocol.
    -p, --mpris      Show player info using MPRIS2 interface.
    -V, --version    Prints version information

OPTIONS:
        --COLOR_END <COLOR_END>                      Default color using to terminate others.
        --COLOR_HIGH <COLOR_HIGH>                    CPU and MEM bar color while high usage.
        --COLOR_LOW <COLOR_LOW>                      CPU and MEM bar color while low usage.
        --COLOR_MID <COLOR_MID>                      CPU and MEM bar color while mid usage.
        --COLOR_TRACK_ARTIST <COLOR_TRACK_ARTIST>    Color of artist name filed.
        --COLOR_TRACK_NAME <COLOR_TRACK_NAME>        Color of track name filed.
        --COLOR_TRACK_TIME <COLOR_TRACK_TIME>        Color of playing time field.
    -l, --localtime <localtime>                      Local time
    -a, --mpd-address <mpd_address>                  <ADDR>:<PORT> of MPD server.
    -u, --utctime <utctime>                          UTC time
```

