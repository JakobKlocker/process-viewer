use crate::app::App;
use crate::app::AppState;
use syscalls::*;
use std::{thread, time::Duration};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};
use std::io::{self, stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    state: ListState,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Ok(Self {
            terminal,
            state: ListState::default(),
        })
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(frame.area());

            let items: Vec<ListItem> = app
                .processes
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let content = format!("{:<9} {}", item.pid, item.name);
                    let style = if i == app.selected_proc {
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

            let filter_display = Paragraph::new(format!("Filter: {}", app.filter_string))
                .block(Block::bordered().title("Filter Input"))
                .style(Style::new().fg(if app.filtering {
                    ratatui::style::Color::Yellow
                } else {
                    ratatui::style::Color::White
                }));

            frame.render_stateful_widget(list, chunks[0], &mut self.state);
            frame.render_widget(filter_display, chunks[1]);

            if app.state == AppState::ProcessMenu {
                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(40),
                        Constraint::Length(5), // Popup height
                        Constraint::Percentage(40),
                    ])
                    .split(frame.area());

                let popup_block = Block::bordered().title("Process Actions");

                let options = vec![
                    ListItem::new("  [k] Kill Process"),
                    ListItem::new("  [f] Bring to foreground"),
                    ListItem::new("  [b] Back to Process List"),
                ];

                let options_list = List::new(options).block(popup_block);

                frame.render_widget(Clear, frame.area());
                frame.render_widget(options_list, popup_layout[1]); // Draw popup in the center
            }
        })?;
        Ok(())
    }

    pub fn handle_input_normal(&mut self, app: &mut App) -> Result<(), ()> {
        if let event::Event::Key(key) = event::read().map_err(|_| ())? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Err(()),
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.processes.is_empty() != true && app.selected_proc < app.processes.len() - 1 {
                            app.selected_proc += 1;
                            self.state.select(Some(app.selected_proc));
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected_proc > 0 {
                            app.selected_proc -= 1;
                            self.state.select(Some(app.selected_proc));
                        }
                    }
                    KeyCode::Left => app.sort_descending(),
                    KeyCode::Right => app.sort_ascending(),
                    KeyCode::Enter => app.state = AppState::ProcessMenu,
                    KeyCode::Char('/') => app.state = AppState::Filterting,
                    KeyCode::Char('r') => {
                        app.reload_processes();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    // To-do: remove the sleep after killing, find a better solution
    pub fn handle_input_processmenu(&mut self, app: &mut App) -> Result<(), ()> {
        if let event::Event::Key(key) = event::read().map_err(|_| ())? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('k') => unsafe {
                        syscall!(Sysno::kill, app.processes[app.selected_proc].pid, 9);
                        app.state = AppState::Normal;
                        thread::sleep(Duration::from_millis(100));
                        app.reload_processes();
                    },
                    KeyCode::Char('b') => {
                        app.state = AppState::Normal;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn handle_input_filtering(&mut self, app: &mut App) -> Result<(), ()> {
        if let event::Event::Key(key) = event::read().map_err(|_| ())? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => app.state = AppState::Normal,
                    KeyCode::Backspace => {
                        app.filter_string.pop();
                        app.apply_filter();
                    }
                    KeyCode::Char(c) => {
                        app.filter_string.push(c);
                        app.apply_filter();
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
