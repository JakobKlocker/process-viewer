use crate::app::App;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Paragraph},
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
                .split(frame.size());

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
        })?;
        Ok(())
    }

    pub fn handle_input(&mut self, app: &mut App) -> Result<(), ()> {
        if let event::Event::Key(key) = event::read().map_err(|_| ())? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Err(()),
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected_proc < app.processes.len() - 1 {
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
                    KeyCode::Char('/') => {
                        app.filtering = true;
                        loop {
                            if let event::Event::Key(key) = event::read().map_err(|_| ())? {
                                match key.code {
                                    KeyCode::Esc => break, // Exit input mode
                                    KeyCode::Enter => {
                                        app.apply_filter();
                                        break;
                                    }
                                    KeyCode::Backspace => {
                                        app.filter_string.pop();
                                    }
                                    KeyCode::Char(c) => {
                                        app.filter_string.push(c);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
