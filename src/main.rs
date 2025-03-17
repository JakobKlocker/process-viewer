mod app;
mod process;
mod tui;

use app::App;
use std::io;
use tui::Tui;

fn main() -> io::Result<()> {
    let mut tui = Tui::new()?;
    let mut app = App::new();

    loop {
        tui.draw(&mut app)?;
        if let Err(()) = tui.handle_input(&mut app) {
            break;
        }
    }

    tui.cleanup()?;
    Ok(())
}
