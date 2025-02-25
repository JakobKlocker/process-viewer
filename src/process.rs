use std::fs;

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
        for path in paths{
            println!("Name : {}", path.unwrap().path().display());
        }
    }
}