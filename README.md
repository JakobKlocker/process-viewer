# Process Viewer

A lightweight, terminal-based process viewer written in Rust.  
Easily filter, kill, navigate, and monitor running processes on your system.

<p align="center">
  <img src="https://github.com/user-attachments/assets/33ba5a18-576e-4a61-97f8-08bacfc109b9" width="300"/>
  <img src="https://github.com/user-attachments/assets/b7b3fd4b-ffd0-4b60-9c3b-98a943cd5bed" width="300"/>
  <img src="https://github.com/user-attachments/assets/91ea83ff-7223-43eb-b5d8-636038ea5137" width="300"/>
</p>

## Features

- View all running processes in a clean, interactive terminal UI
- Filter processes by name using a search input
- Kill processes
- Automatically refresh and reload process list
- Simple built-in web server that serves all process information as JSON via [`http://localhost:4242/processes`](http://localhost:4242/processes)

## Installation

Clone the repository and build it using Cargo:

```bash
git clone https://github.com/JakobKlocker/process-viewer.git
cd process-viewer
cargo build --release
```
## Motivation

I created this project to deepen my understanding of Rust and explore the possibilities of process management and manipulation in Linux.
A major focus was also learning how to build terminal user interfaces (TUI) in Rust, using libraries like ratatui to create interactive and responsive layouts.
One of my initial goals was to display the process UI through a web server, but due to the limitations around Wayland, I’ve decided to put that idea on hold for now.

I might continue adding more process-altering functionality over time as the project evolves.
