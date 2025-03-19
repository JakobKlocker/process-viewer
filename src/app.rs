use crate::process::{ProcessInfo, Processes};
use std::cmp::Reverse;

#[derive(PartialEq)]
pub enum AppState {
    Normal,     // Default, navigating
    Filterting, // Filterting Processes
    ProcessMenu, // When selecting a process with Enter
}
pub struct App {
    pub all_processes: Vec<ProcessInfo>,
    pub processes: Vec<ProcessInfo>,
    pub selected_proc: usize,
    pub filtering: bool,
    pub filter_string: String,
    pub state: AppState,
    pub selected_menu: usize,
}

impl App {
    pub fn new() -> Self {
        let all_processes = Processes::fetch_process_list().unwrap_or_default();
        let processes = all_processes.clone();
        Self {
            all_processes,
            processes,
            selected_proc: 0,
            filtering: false,
            filter_string: String::new(),
            state: AppState::Normal,
            selected_menu: 0,
        }
    }

    pub fn sort_ascending(&mut self) {
        self.processes.sort_by_key(|p| p.pid);
    }

    pub fn sort_descending(&mut self) {
        self.processes.sort_by_key(|p| Reverse(p.pid));
    }
    pub fn apply_filter(&mut self) {
        self.processes = if self.filter_string.is_empty() {
            self.all_processes.clone()
        } else {
            self.all_processes
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&self.filter_string))
                .cloned()
                .collect()
        };
    }
}
