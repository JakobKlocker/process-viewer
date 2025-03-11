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
    let mut proc: Vec<ProcessInfo> = process::Processes::get_pid_name().unwrap();

    let mut selected_proc: usize = 0;
    let mut state = ListState::default();
    let mut filtering = false;
    let mut filter_string = String::new();
    state.select(Some(selected_proc));
    loop {
        terminal
            .draw(|frame| {
                let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Min(1),  // Process list takes most space
                    ratatui::layout::Constraint::Length(3), // Bottom filter field
                ])
                .split(frame.area());

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
                let filter_display = ratatui::widgets::Paragraph::new(format!("Filter: {}", filter_string))
                .block(Block::bordered().title("Filter Input"))
                .style(Style::new().fg(if filtering { ratatui::style::Color::Yellow } else { ratatui::style::Color::White }));

                frame.render_stateful_widget(list, chunks[0], &mut state);
                frame.render_widget(filter_display,chunks[1]);
            })
            .unwrap();
        if let Err(()) = handle_key(&mut selected_proc, &mut state, &mut proc, &mut filtering, &mut filter_string) {
            cleanup();
            return Ok(());
        }
    }
}

fn cleanup() {
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

fn apply_filter(filter_string: &str, proc: &mut Vec<ProcessInfo>){
    *proc = proc.iter()
    .filter(|p| p.name.to_lowercase().contains(&filter_string)).cloned().collect();
}

fn handle_key(
    selected_proc: &mut usize,
    state: &mut ListState,
    proc: &mut Vec<ProcessInfo>,
    filter: &mut bool,
    filter_string: &mut String
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
                KeyCode::Char('/') => {
                    *filter = true;
                    loop{
                        if let event::Event::Key(key) = event::read().map_err(|_| ())? {
                            match key.code {
                                KeyCode::Esc => break, // Exit input mode
                                KeyCode::Enter => {
                                    apply_filter(&filter_string, proc);
                                    break;
                                },
                                KeyCode::Backspace => {
                                    filter_string.pop();
                                },
                                KeyCode::Char(c) => {
                                    filter_string.push(c);
                                },
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    } else {
        return Err(());
    }
}
