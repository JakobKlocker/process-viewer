mod process;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use process::ProcessInfo;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState},
};
use std::cmp::Reverse;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
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

    let mut proc: Vec<ProcessInfo> = process::Processes::get_pid_name().unwrap();

    let mut selected_proc: usize = 0;
    let mut state = ListState::default();
    state.select(Some(selected_proc));
    loop {
        terminal
            .draw(|frame| {
                let items: Vec<ListItem> = proc
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let content = format!("{:<9} {}", item.pid, item.name);
                        let style = if i == selected_proc {
                            Style::new().fg(ratatui::style::Color::Yellow)
                        } else {
                            Style::new()
                        };
                        ListItem::new(content).style(style)
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::bordered().title("Process Info"))
                    .style(Style::new().white());

                frame.render_stateful_widget(list, frame.area(), &mut state);
            })
            .unwrap();
        if let Err(()) = handle_key(&mut selected_proc, &mut state, &mut proc) {
            cleanup();
            return Ok(());
        }
    }
}

fn cleanup() {
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

fn handle_key(
    selected_proc: &mut usize,
    state: &mut ListState,
    proc: &mut Vec<ProcessInfo>,
) -> Result<(), ()> {
    if let event::Event::Key(key) = event::read().map_err(|_| ())? {
        if key.kind == KeyEventKind::Press {
            match key.code{
                KeyCode::Char('q') =>  return Err(()),
                KeyCode::Down | KeyCode::Char('j') => {
                    if *selected_proc < proc.len() - 1{
                        *selected_proc += 1;
                        state.select(Some(*selected_proc));
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    if *selected_proc > 0{
                        *selected_proc -= 1;
                        state.select(Some(*selected_proc));
                        
                    }
                },
                KeyCode::Left => {
                    proc.sort_by_key(|p| Reverse(p.pid));
                },
                KeyCode::Right => {
                    proc.sort_by_key(|p| p.pid);
                },
                _ => {}
            }
        }
        return Ok(());
    } else {
        return Err(());
    }
}
