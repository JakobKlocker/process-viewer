use crate::process::{ProcessInfo, Processes};
use std::cmp::Reverse;

pub struct App {
    pub processes: Vec<ProcessInfo>,
    pub selected_proc: usize,
    pub filtering: bool,
    pub filter_string: String,
}

impl App {
    pub fn new() -> Self {
        let processes = Processes::fetch_process_list().unwrap_or_default();
        Self {
            processes,
            selected_proc: 0,
            filtering: false,
            filter_string: String::new(),
        }
    }

    pub fn sort_ascending(&mut self) {
        self.processes.sort_by_key(|p| p.pid);
    }

    pub fn sort_descending(&mut self) {
        self.processes.sort_by_key(|p| Reverse(p.pid));
    }

    pub fn apply_filter(&mut self) {
        self.processes
            .retain(|p| p.name.to_lowercase().contains(&self.filter_string));
    }
}
