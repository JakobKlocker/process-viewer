use std::{collections::HashSet, fs, os::unix::fs::PermissionsExt, path::PathBuf};

#[derive(Debug, Clone, Eq, Hash)]
pub struct ProcessInfo{
    pub pid : u32,
    pub name : String
}

impl ProcessInfo{
    pub fn new(pid: u32, name: String) -> Self {
        Self {pid, name}
    }
}

pub struct Processes{
    pub processes:  Vec<ProcessInfo>
}

impl PartialEq for ProcessInfo{
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid && self.name == other.name
    }
}

impl Processes {
    pub fn new() -> Self {
        let list = Self::get_pid_name(); 
        let proc = list.unwrap_or_else(|_| Vec::new());
        Self{
            processes: proc
        }
    }

    //first called to fill process vector
    pub fn get_pid_name() -> std::io::Result<Vec<ProcessInfo>>{
        let mut ret :Vec<ProcessInfo> = Vec::new();
        for entry in fs::read_dir("/proc/")?{
            let dir_entry = entry?;
            let filename = dir_entry.file_name();
            let filename_path = filename.to_string_lossy();
            if let Ok(pid) = filename_path.parse::<u32>(){
                let proc_name = fs::read_to_string(dir_entry.path().join("comm"))
                .map( |s| s.trim().to_owned())
                .unwrap_or_else( |_| "[Unknown]".into());
                ret.push(ProcessInfo::new(pid, proc_name));
            } 
        } 
        Ok(ret)
    }


    pub fn update_proc(&mut self, n_proc: &Vec<ProcessInfo>){
        let old_set: HashSet<ProcessInfo> = self.processes.iter().cloned().collect();
        let new_set: HashSet<ProcessInfo> = n_proc.iter().cloned().collect();
        let added: Vec<_> = new_set.difference(&old_set).cloned().collect();
        let removed: Vec<_> = old_set.difference(&new_set).cloned().collect();

        for p in &added{ 
            println!("Added: {:?}", p); 
        }
        for p in &removed{
            println!("Removed: {:?}", p);
        }
        self.processes = n_proc.clone();
   }

    pub fn get_new_proc_update(&mut self){ 
        let mut new_procs = Self::get_pid_name(); 
        match new_procs{ Ok(n_proccesses) =>{
                self.update_proc(&n_proccesses);
            }
            Err(err) => {
                println!("Error getting new procceses");
            }
        }
    }
}