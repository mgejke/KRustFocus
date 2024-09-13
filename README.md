# K(ey)RustFocus

A simple Windows tool that runs in the background for switching window focus with hotkeys. 

## Usage:

`KrustFocus.exe --mod-keys CTRL ALT --left LEFT --right RIGHT`

For now, the application won't display any output(since I don't want a console window to be opened), so this generated usage information won't be shown:

```
Tool for focusing windows based on position

Usage: KRustFocus.exe [OPTIONS]

Options:
      --mode <MODE>             Use left or window avg for sorting [default: avg] [possible values: left, avg]
  -m, --mod-keys <MOD_KEYS>...  [default: CTRL ALT]
  -l, --left <LEFT>             [default: LEFT]
  -r, --right <RIGHT>           [default: RIGHT]
  -h, --help                    Print help
  -V, --version                 Print version
```

This will setup `CTRL-ALT-Left Arrow` and `CTRL-ALT-RightArrow` for switching focus left and right based on the windows average x position.

The application will add an icon in the System Tray. 
`CTRL-ALT-SHIFT-L` always exits the application.

## Building

- Clone it
- `cargo build`

## Installing

- Clone it
- `cargo install --path .`

## Todo:

- Test on Windows 10
- Fix conditional console window handling

