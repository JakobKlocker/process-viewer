use crate::process::{ProcessInfo, Processes};
use std::cmp::Reverse;

#[derive(PartialEq)]
pub enum AppState {
    Normal,     // Default, navigating
    Filtering, // Filterting Processes
    ProcessMenu, // When selecting a process with Enter
}
pub struct App {
    pub all_processes: Vec<ProcessInfo>,
    pub processes: Vec<ProcessInfo>,
    pub selected_proc: usize,
    pub filter_string: String,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        let all_processes = Processes::fetch_process_list().unwrap_or_default();
        let processes = all_processes.clone();
        Self {
            all_processes,
            processes,
            selected_proc: 0,
            filter_string: String::new(),
            state: AppState::Normal,
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

    pub fn reload_processes(&mut self){
        let all_processes = Processes::fetch_process_list().unwrap_or_default();
        self.all_processes = all_processes;
        self.apply_filter();
        self.selected_proc = 0;
    }
}
