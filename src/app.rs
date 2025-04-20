use crate::process::{ProcessInfo, Processes};
use std::cmp::Reverse;

#[derive(PartialEq, Debug)]
pub enum AppState {
    Normal,      // Default, navigating
    Filtering,   // Filterting Processes
    ProcessMenu, // When selecting a process with Enter
}

#[derive(PartialEq)]
pub enum ProcessOrder {
    Ascending,
    Descending,
}
pub struct App {
    pub all_processes: Vec<ProcessInfo>,
    pub processes: Vec<ProcessInfo>,
    pub selected_proc: usize,
    pub process_order: ProcessOrder,
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
            process_order: ProcessOrder::Ascending,
            filter_string: String::new(),
            state: AppState::Normal,
        }
    }

    pub fn sort_ascending(&mut self) {
        self.process_order = ProcessOrder::Ascending;
        self.processes.sort_by_key(|p| p.pid);
    }

    pub fn sort_descending(&mut self) {
        self.process_order = ProcessOrder::Descending;
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

    pub fn reload_processes(&mut self) {
        let all_processes = Processes::fetch_process_list().unwrap_or_default();
        self.all_processes = all_processes;
        self.apply_filter();
        if self.process_order == ProcessOrder::Ascending {
            self.sort_ascending();
        } else {
            self.sort_descending();
        }
        //self.selected_proc = 0;    since reloading happening all the time, this might crash if its high and on reload proceses are less than before...
    }
}

#[test]
fn test_app_initialization() {
    let app = App::new();
    assert_eq!(app.processes, app.all_processes);
    assert_eq!(app.selected_proc, 0);
    assert_eq!(app.filter_string, "");
    assert_eq!(app.state, AppState::Normal);
}

#[test]
fn test_sort_ascending_by_pid() {
    let mut app = App {
        all_processes: vec![],
        processes: vec![
            ProcessInfo::new(3, "c".into(), 0, 0),
            ProcessInfo::new(1, "a".into(), 0, 0),
            ProcessInfo::new(2, "b".into(), 0, 0),
        ],
        selected_proc: 0,
        filter_string: String::new(),
        state: AppState::Normal,
    };

    app.sort_ascending();

    let pids: Vec<u32> = app.processes.iter().map(|p| p.pid).collect();
    assert_eq!(pids, vec![1, 2, 3]);
}

#[test]
fn test_sort_descending_by_pid() {
    let mut app = App {
        all_processes: vec![],
        processes: vec![
            ProcessInfo::new(1, "a".into(), 0, 0),
            ProcessInfo::new(3, "c".into(), 0, 0),
            ProcessInfo::new(2, "b".into(), 0, 0),
        ],
        selected_proc: 0,
        filter_string: String::new(),
        state: AppState::Normal,
    };

    app.sort_descending();

    let pids: Vec<u32> = app.processes.iter().map(|p| p.pid).collect();
    assert_eq!(pids, vec![3, 2, 1]);
}

#[test]
fn test_apply_filter_matches_lowercase() {
    let all = vec![
        ProcessInfo::new(1, "firefox".into(), 0, 0),
        ProcessInfo::new(2, "sshd".into(), 0, 0),
        ProcessInfo::new(3, "gnome-shell".into(), 0, 0),
    ];

    let mut app = App {
        all_processes: all.clone(),
        processes: all.clone(),
        selected_proc: 0,
        filter_string: "fire".into(),
        state: AppState::Filtering,
    };

    app.apply_filter();

    assert_eq!(app.processes.len(), 1);
    assert_eq!(app.processes[0].name, "firefox");
}

