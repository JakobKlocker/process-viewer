use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

#[derive(Debug)]
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

impl Processes {
    pub fn new() -> Self {
        Self{
            processes: Vec::new(),
        }
    }

    fn get_proc_name_path(&self, path: PathBuf) -> Result<String, std::io::Error>{
        fs::read_to_string(path)
    }

    pub fn get_pid_name(&mut self) -> std::io::Result<()>{
        for entry in fs::read_dir("/proc/")?{
            let dir_entry = entry?;
            let filename = dir_entry.file_name();
            let filename_path = filename.to_string_lossy();
            if let Ok(pid) = filename_path.parse::<u32>(){
                let proc_name = fs::read_to_string(dir_entry.path().join("comm"))
                .map( |s| s.trim().to_owned())
                .unwrap_or_else( |_| "[Unknown]".into());
                println!("pid: {}  ---  name:  {}", pid, proc_name);
                self.processes.push(ProcessInfo::new(pid, proc_name));
            }
        }
        Ok(())
    }   
}