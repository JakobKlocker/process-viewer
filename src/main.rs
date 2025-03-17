mod process;
fn main() -> io::Result<()> {
    // Setup Tui
    // get proc Info
    let mut proc: Vec<ProcessInfo> = process::Processes::get_pid_name().unwrap();

    let mut selected_proc: usize = 0;
    let mut state = ListState::default();
    let mut filtering = false;
    let mut filter_string = String::new();
    state.select(Some(selected_proc));
    loop {
        terminal
           .unwrap();
        if let Err(()) = handle_key(
            &mut selected_proc,
            &mut state,
            &mut proc,
            &mut filtering,
            &mut filter_string,
        ) {
            cleanup();
            return Ok(());
        }
    }
}

fn apply_filter(filter_string: &str, proc: &mut Vec<ProcessInfo>) {
    *proc = proc
        .iter()
        .filter(|p| p.name.to_lowercase().contains(&filter_string))
        .cloned()
        .collect();
}

