# GTFU 

This tool is meant to help stubborn developers (like me) get up and touch grass every once in a while.

## Why?

1. I can!
2. I need a simple, fast, and customizable breaktimer.

## Features(WIP)!

- [X] Will automate the timer and break start/end.
- [ ] Will support idle reset.
    - To check for linux, i had to install libxscrnsaver, libx11, and pkg-config
- [ ] Will force you to take breaks by blocking the screen.
- [ ] Will have minimal setup required.
     - [ ] Will support startup process.
     - [ ] Will save user config. (Use confy)
-  [ ] Will have an interactive TUI
-  [ ] Will be cross-platform.
-  [ ] Will supports multiple breaks.

## Where will it run?

- MacOS
- Linux
- Windows

### Future plans

- Mobile
- ??? 

## How will it work?

### TUI - PRIORITY

Ratatui and crossterm to manage the TUI and user interactions. I can handle the screen blocking through other terminals or possibly windows like Winit

### Some GUI - MAYBE

Wry & Winit seem to be a good solution to render a webView and have some GUI for users.

### GUI - NONO (For now) 

Tauri seems like a good approach to build native desktop and mobile apps in one source code.



