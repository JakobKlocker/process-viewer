use libc;
use serde::Serialize;
use std::hash::{Hash, Hasher};
use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_time: u64,
    pub memory: u64,
    pub cpu_percent: f64,
}

impl Eq for ProcessInfo {}

impl Hash for ProcessInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pid.hash(state);
        self.name.hash(state);
        self.cpu_time.hash(state);
        self.memory.hash(state);
    }
}

impl ProcessInfo {
    pub fn new(pid: u32, name: String, cpu_time: u64, memory: u64) -> Self {
        Self {
            pid,
            name,
            cpu_time,
            memory,
            cpu_percent: 0.0,
        }
    }
}

pub struct Processes {
    pub processes: Vec<ProcessInfo>,
}

impl PartialEq for ProcessInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid && self.name == other.name
    }
}

impl Processes {
    pub fn new() -> Self {
        Self {
            processes: Self::fetch_process_list().unwrap_or_default(),
        }
    }

    //first called to fill process vector
    pub fn fetch_process_list() -> std::io::Result<Vec<ProcessInfo>> {
        let mut ret: Vec<ProcessInfo> = Vec::new();
        for entry in fs::read_dir("/proc/")? {
            let dir_entry = entry?;
            let filename = dir_entry.file_name();
            let filename_path = filename.to_string_lossy();
            if let Ok(pid) = filename_path.parse::<u32>() {
                let proc_name = fs::read_to_string(dir_entry.path().join("comm"))
                    .map(|s| s.trim().to_owned())
                    .unwrap_or_else(|_| "[Unknown]".into());

                let mut memory = 0;
                let mut cpu_time = 0;
                let stat_path = dir_entry.path().join("stat");
                let stat_content = fs::read_to_string(&stat_path).unwrap_or_default();
                let stat_fields: Vec<&str> = stat_content.split_whitespace().collect();
                if stat_fields.len() > 23 {
                    let utime = stat_fields[13].parse::<u64>().unwrap_or(0);
                    let stime = stat_fields[14].parse::<u64>().unwrap_or(0);
                    let rss_pages = stat_fields[23].parse::<u64>().unwrap_or(0);
                    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64;
                    memory = rss_pages * page_size;
                    cpu_time = utime + stime;
                }

                ret.push(ProcessInfo::new(pid, proc_name, cpu_time, memory));
            }
        }
        Ok(ret)
    }

    pub fn update_proc(&mut self, n_proc: &Vec<ProcessInfo>) {
        let old_set: HashSet<ProcessInfo> = self.processes.iter().cloned().collect();
        let new_set: HashSet<ProcessInfo> = n_proc.iter().cloned().collect();
        let added: Vec<_> = new_set.difference(&old_set).cloned().collect();
        let removed: Vec<_> = old_set.difference(&new_set).cloned().collect();

        for p in &added {
            println!("Added: {:?}", p);
        }
        for p in &removed {
            println!("Removed: {:?}", p);
        }
        self.processes = n_proc.clone();
    }

    pub fn refresh_processses(&mut self) {
        match Self::fetch_process_list() {
            Ok(n_processes) => {
                self.update_proc(&n_processes);
            }
            Err(_) => {
                println!("Error getting new procceses");
            }
        }
    }
}

#[test]
fn test_processinfo_new() {
    let proc = ProcessInfo::new(42, "testproc".to_string(), 123, 4096);
    assert_eq!(proc.pid, 42);
    assert_eq!(proc.name, "testproc");
    assert_eq!(proc.cpu_time, 123);
    assert_eq!(proc.memory, 4096);
    assert_eq!(proc.cpu_percent, 0.0);
}

#[test]
fn test_processinfo_equality() {
    let p1 = ProcessInfo::new(1, "bash".into(), 100, 2000);
    let p2 = ProcessInfo::new(1, "bash".into(), 999, 9999); 
    assert_eq!(p1, p2); 
}

#[test]
fn test_update_proc_detects_changes() {
    let mut processes = Processes { processes: vec![
        ProcessInfo::new(1, "a".into(), 10, 100),
        ProcessInfo::new(2, "b".into(), 20, 200),
    ]};

    let new_list = vec![
        ProcessInfo::new(2, "b".into(), 20, 200), 
        ProcessInfo::new(3, "c".into(), 30, 300),
    ];

    processes.update_proc(&new_list);
    assert_eq!(processes.processes.len(), 2);
    assert!(processes.processes.iter().any(|p| p.pid == 3));
    assert!(!processes.processes.iter().any(|p| p.pid == 1));
}

#[test]
fn test_refresh_processes_doesnt_panic() {
    let mut p = Processes { processes: vec![] };
    p.refresh_processses(); 
}
