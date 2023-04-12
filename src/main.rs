use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
use tokio::time;

// The sysinfo crate provides the System struct to access system information.
// It is necessary to call the refresh_all or refresh_memory method before
// getting the memory usage information.

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        print_memory_usage();
    }
}

fn print_memory_usage() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("PID\tName\t\tMemory");

    for (pid, process) in system.get_processes() {
        let process_name = process.name();
        let memory = process.memory(); // Memory usage in kilobytes.

        println!("{}\t{:.10}\t{} KB", pid, process_name, memory);
    }
}
