mod process;
use process::ProcessInfo;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem},
};
use std::io::{self, stdout};

fn main() {
    // Setup Tui
    enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // get proc Info
    let proc: Vec<String> = process::Processes::get_pid_name()
        .unwrap()
        .iter()
        .map(|p| p.to_string())
        .collect();
    

    let selected_proc: usize = 0;
    loop {
        terminal
            .draw(|frame| {
                let items: Vec<ListItem> = proc
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let style = if i == selected_proc{
                            Style::new().fg(ratatui::style::Color::Yellow)
                        } else {
                            Style::new()
                            };
                     ListItem::new(item.clone()).style(style)
                         })
                    .collect();

                let list = List::new(items)
                    .block(Block::bordered().title("Process Info"))
                    .style(Style::new().white());

                frame.render_widget(list, frame.area());
            })
            .unwrap();

        // Exit on key press
        if matches!(event::read().unwrap(), Event::Key(_)) {
            break;
        }
    }

    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
