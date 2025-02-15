# GTFU 

This tool is meant to help stubborn developers (like me) get up and touch grass every once in a while.

## Why?

1. I can!
2. I need a simple, fast, and customizable breaktimer.

## How will it work?

- TUI functionality using Ratatui and crossterm. Winit for managing windows to block screen
- MAYBE: Wry and Winit can be used to render a webview for a GUI exp

## Features(WIP)!

- [X] Automates the timer and break start/end.
- [X] Supports idle reset.
- [ ] Will force you to take breaks by blocking the screen.
- [ ] Will have minimal setup required.
     - [ ] Will support startup process.
     - [ ] Will save user config. (Use confy)
-  [ ] Will have an interactive TUI
-  [ ] Will be cross-platform.
-  [ ] Will supports multiple breaks.

---

# Contributing

## Self notes
To run checks (cli and within lsp) for linux `target_os` on my macbook, I had to install `libxscrnsaver`, `libx11`, and `pkg-config`. I also needed to insall the specfic linux target using `rustup`, and use the following configuration: 

``` toml
# .cargo/config.toml

[build]
target = "aarch64-unknown-linux-gnu"

[env]
PKG_CONFIG_SYSROOT_DIR= "/"

```

