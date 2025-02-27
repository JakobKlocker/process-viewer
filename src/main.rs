mod process;

use process::Processes;

fn main() {
    println!("Hello, world!");
    let mut proc = Processes::new();
    proc.get_pid_name().unwrap();
    for pro in proc.processes{
        println!("{:?}", pro);
    } 
}