mod process;
use process::ProcessInfo;
use crossterm::event::KeyModifiers;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState},
};
use std::io::{self, stdout};

fn main() -> io::Result<()>{
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

                frame.render_stateful_widget(list, frame.area(), &mut state);
            })
            .unwrap();
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    cleanup();
                    return Ok(());
                }
            if key.code == KeyCode::Down && selected_proc < proc.len() - 1{
                selected_proc += 1;
                state.select(Some(selected_proc));
            }
            
            if key.code == KeyCode::Up && selected_proc > 0{
                selected_proc -= 1;
                state.select(Some(selected_proc));
            }
            }
        }
    }

    cleanup();
}

fn cleanup(){
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}