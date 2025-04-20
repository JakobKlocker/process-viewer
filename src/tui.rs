use crate::app::App;
use crate::app::AppState;
use std::{thread, time::Duration};
use syscalls::*;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Cell, Clear, List, ListItem, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::io::{self, stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    state: TableState,
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
            state: TableState::default(),
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
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(frame.area());

                let header = Row::new(vec![
                    Cell::from("PID"),
                    Cell::from("Name"),
                    Cell::from("Memory(KB)"),
                    Cell::from("CPU-Time"),
                    Cell::from("CPU%"),
                ])
                .style(Style::default().bold());

                let rows = app.processes.iter().enumerate().map(|(i, item)| {
                    let memory_kb = item.memory / 1024;
                    let style = if i == app.selected_proc {
                        Style::default().fg(ratatui::style::Color::Yellow)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(item.pid.to_string()),
                        Cell::from(item.name.clone()),
                        Cell::from(memory_kb.to_string()),
                        Cell::from(item.cpu_time.to_string()),
                        Cell::from(format!("{:.1}", item.cpu_percent)),
                    ])
                    .style(style)
                });

            let widths =
            [
                Constraint::Length(9),
                Constraint::Length(25),
                Constraint::Length(12),
                Constraint::Length(10),
                Constraint::Length(10),
            ];
            let table = Table::new(rows, widths)
                .header(header)
                .block(Block::bordered().title("Process Info").bg(ratatui::style::Color::Black).border_style(if app.state == AppState::Normal {
                    ratatui::style::Color::LightRed
                } else {
                    ratatui::style::Color::White
                }))
               .column_spacing(1); // optional: space between columns

            let filter_display = Paragraph::new(format!("Filter: {}", app.filter_string))
                .block(Block::bordered().title("Filter Input:").bg(ratatui::style::Color::Black).border_style(if app.state == AppState::Filtering {
                    ratatui::style::Color::LightRed
                } else {
                    ratatui::style::Color::White
                }));
                let (help_msg, mode_str) = match app.state {
                    AppState::Filtering => (
                        "Esc: stop filtering",
                        "Mode: Filtering",
                    ),
                    AppState::Normal => (
                        "↑[k]/↓[j]: Navigate || Enter: Select || q: Quit || /: Filter || r: reload Processes || ←: sort desc. || →: sort asc.",
                        "Mode: Normal",
                    ),
                    AppState::ProcessMenu => (
                        "↑[k]/↓[j]: Navigate || k: Kill Process || f: bring to foreground (not working) || b: back to Process List",
                        "Mode: Process Menu",
                    ),
                };

                let help_text = Paragraph::new(help_msg)
                    .style(Style::new().bg(ratatui::style::Color::Black).fg(ratatui::style::Color::White)).alignment(Alignment::Center);


                let mode_display = Paragraph::new(mode_str)
                    .style(Style::new().bg(ratatui::style::Color::Black).fg(ratatui::style::Color::Red)).alignment(Alignment::Center);


            frame.render_widget(&mode_display, chunks[0]);
            frame.render_widget(&help_text, chunks[1]);
            frame.render_stateful_widget(table, chunks[2], &mut self.state);
            frame.render_widget(filter_display, chunks[3]);

            if app.state == AppState::ProcessMenu {
                let popup_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Percentage(40),
                        Constraint::Length(5),
                        Constraint::Percentage(40),
                    ])
                    .split(frame.area());

                let popup_block = Block::bordered().title("Process Actions").bg(ratatui::style::Color::Black);

                let options = vec![
                    ListItem::new("  [k] Kill Process"),
                    ListItem::new("  [f] Bring to foreground (not working)"),
                    ListItem::new("  [b] Back to Process List"),
                ];

                let options_list = List::new(options).block(popup_block);
                let black_bg = Block::default().style(Style::default().bg(ratatui::style::Color::Black));

                frame.render_widget(Clear, frame.area());
                frame.render_widget(mode_display, popup_layout[0]);
                frame.render_widget(help_text, popup_layout[1]);
                frame.render_widget(&black_bg, popup_layout[2]);
                frame.render_widget(options_list, popup_layout[3]);
                frame.render_widget(&black_bg, popup_layout[4]);
                }
        })?;
        Ok(())
    }

    pub fn handle_input_normal(&mut self, app: &mut App) -> Result<(), ()> {
        if event::poll(Duration::from_millis(50)).map_err(|_| ())? {
            if let event::Event::Key(key) = event::read().map_err(|_| ())? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Err(()),
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.processes.is_empty() != true
                                && app.selected_proc < app.processes.len() - 1
                            {
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
                        KeyCode::Char('/') => app.state = AppState::Filtering,
                        KeyCode::Char('r') => {
                            app.reload_processes();
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    // To-do: remove the sleep after killing, find a better solution
    pub fn handle_input_processmenu(&mut self, app: &mut App) -> Result<(), ()> {
        if event::poll(Duration::from_millis(50)).map_err(|_| ())? {
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
        }
        Ok(())
    }

    pub fn handle_input_filtering(&mut self, app: &mut App) -> Result<(), ()> {
        if event::poll(Duration::from_millis(50)).map_err(|_| ())? {
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
        }
        Ok(())
    }
}
