mod app;
mod process;
mod tui;

use app::App;
use app::AppState;
use std::io;
use tui::Tui;

fn main() -> io::Result<()> {
    let mut tui = Tui::new()?;
    let mut app = App::new();

    loop {
        tui.draw(&mut app)?;
        let result = match app.state{
            AppState::Normal => tui.handle_input_normal(&mut app),
            AppState::Filterting => tui.handle_input_filtering(&mut app),
            AppState::ProcessMenu => tui.handle_input_processmenu(&mut app),
            _ => Ok(())
        };

        if result.is_err(){
            break;
        }
    }

    tui.cleanup()?;
    Ok(())
}
