# Tmux helper
Small app that perform system check and print TMUX friendly output.

<img width="1495" height="1264" alt="image" src="https://github.com/user-attachments/assets/7b9ffc97-0b59-4028-9b5d-f29347d16000" />


### Building 
`cargo build --release`
or get binary on release page

### Fetures
```shell
Utility for printing system info for tmux status line.

Usage: tmux-helper [OPTIONS]

Options:
  -c, --cpu
          Print cpu load bar.
  -m, --mem
          Print mem usage bar.
      --low <low>
          Low threshold (0.0 - 1.0) [default: 0.7]
      --mid <mid>
          Mid threshold (0.0 - 1.0) [default: 0.9]
  -p, --mpris
          Show player info using MPRIS2 interface.
  -d, --mpd
          Show mpd player using MPD native protocol.
  -l, --localtime [<localtime>]
          Local time
  -u, --utctime [<utctime>]
          UTC time
  -s, --symbol [<bar_symbol>]
          Symbol to build bar [default: ▮]
  -e, --empty-symbol [<bar_empty_symbol>]
          Symbol to represent the empty part of the bar [default: ▯]
  -a, --mpd-address <mpd_address>
          <ADDR>:<PORT> of MPD server. [default: 127.0.0.1:6600]
      --COLOR_LOW <COLOR_LOW>
          CPU and MEM bar color while low usage. [default: 119]
      --COLOR_MID <COLOR_MID>
          CPU and MEM bar color while mid usage. [default: 220]
      --COLOR_HIGH <COLOR_HIGH>
          CPU and MEM bar color while high usage. [default: 197]
      --COLOR_TRACK_NAME <COLOR_TRACK_NAME>
          Color of track name filed. [default: 46]
      --COLOR_TRACK_ARTIST <COLOR_TRACK_ARTIST>
          Color of artist name filed. [default: 46]
      --COLOR_TRACK_TIME <COLOR_TRACK_TIME>
          Color of playing time field. [default: 153]
      --COLOR_END <COLOR_END>
          Default color using to terminate others. [default: 153]
  -h, --help
          Print help
  -V, --version
          Print version
```

