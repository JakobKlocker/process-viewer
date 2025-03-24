mod app;
mod process;
mod tui;
mod webserver;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use app::App;
use app::AppState;
use std::io;
use tui::Tui;

fn main() -> io::Result<()> {
    let app_arc = Arc::new(Mutex::new(App::new()));
    
    let app_for_http = Arc::clone(&app_arc);
    webserver::start_http_server(app_for_http);
    let mut tui = Tui::new()?;

    loop {
        let mut app = app_arc.lock().unwrap();
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
        std::thread::sleep(Duration::from_millis(50));
    }
    tui.cleanup()?;
    Ok(())
}