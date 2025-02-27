mod process;

use std::time::Duration;

use process::Processes;

fn main() {
    let mut proc = Processes::new();
    for pro in &proc.processes{
        println!("{:?}", pro);
    } 
    loop{
        proc.get_new_proc_update();
        std::thread::sleep(Duration::from_secs(1));
    }
}