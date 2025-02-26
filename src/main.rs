mod process;

use process::Processes;

fn main() {
    println!("Hello, world!");
    let proc = Processes::new();
    proc.print_folders();
}