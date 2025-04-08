mod app;
mod process;
mod tui;
mod webserver;
mod cpu_tracker;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use app::App;
use app::AppState;
use std::io;
use tui::Tui;
use cpu_tracker::CpuTracker;

fn main() -> io::Result<()> {
    let app_arc = Arc::new(Mutex::new(App::new()));
    let app_for_refresh = Arc::clone(&app_arc);
    let mut cpu_tracker = CpuTracker::new(); 
    
    let app_for_http = Arc::clone(&app_arc);
    webserver::start_http_server(app_for_http);
    let mut tui = Tui::new()?;
    
    std::thread::spawn(move || {
        let mut cpu_tracker = cpu_tracker;
        loop {
            {
                let mut app = app_for_refresh.lock().unwrap();
                 if let Ok(mut new_proc_list) = process::Processes::fetch_process_list() {
                    app.reload_processes();
                    cpu_tracker.update_process_cpu(&mut app.processes);
                }
           }
            std::thread::sleep(Duration::from_millis(1000));
        }
    });
    

    loop {
        {
            let mut app = app_arc.lock().unwrap();
            tui.draw(&mut app)?;
            let result = match app.state{
                AppState::Normal => tui.handle_input_normal(&mut app),
                AppState::Filtering => tui.handle_input_filtering(&mut app),
                AppState::ProcessMenu => tui.handle_input_processmenu(&mut app),
            };

            if result.is_err(){
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    tui.cleanup()?;
    Ok(())
}