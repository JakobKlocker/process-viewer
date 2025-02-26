use std::{fs, os::unix::fs::PermissionsExt};

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

    pub fn print_folders(&self){
        let paths = fs::read_dir("/proc/").unwrap();
        for entry in paths{
           match entry{
            Ok(dir_entry) => {
                let metadata = dir_entry.metadata().unwrap();
                let filename = dir_entry.file_name();

                if let Some(filename_str) = filename.to_str(){
                    match filename_str.parse::<u32>(){
                        Ok(pid) =>{
                            println!("Pid: {}", pid)
                        }
                        Err(err) =>{
                            println!("Err: {}", err);
                        }
                    }
                }
                //println!("Pid :{:#x},  Permision: {:?}", filename.to_string_lossy().parse::<u32>().unwrap(), metadata.permissions());
            }
            Err(err) => {
                println!("{}", err);
            }
           } 
        }
    }
}