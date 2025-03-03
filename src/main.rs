mod process;
use process::ProcessInfo;

use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, List, ListItem, ListState},
    layout::{Alignment, Layout, Constraint, Direction, },
    text::Span,
    style::{Color, Style},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use process::Processes;

fn list_from_vec(vec: &Vec<ProcessInfo>) -> List{
    let items: Vec<ListItem> = vec.iter()
    .map(|p| ListItem::new(format!("Pid: {} - {}", p.pid, p.name)))
    .collect();

    List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default())
    .highlight_symbol(">>")
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut proc = Processes::new();
    for pro in &proc.processes{
        println!("{:?}", pro);
    } 

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut list_state = ListState::default();
    loop{

        terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
               
        let binding = Processes::get_pid_name().unwrap_or_else(|_| vec![]);
        let l = list_from_vec(&binding);
        f.render_stateful_widget(l, size, &mut list_state);

        })?;
    }
    thread::sleep(Duration::from_millis(5000));
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
    
    /*loop{
        proc.get_new_proc_update();
        std::thread::sleep(Duration::from_secs(1));
    }
        */
}