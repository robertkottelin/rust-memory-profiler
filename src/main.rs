use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt, Pid};
use std::collections::HashMap;

fn main() {
    loop {
        print_memory_usage();
        thread::sleep(Duration::from_secs(1));
    }
}

fn print_memory_usage() {
    // Clears the terminal
    print!("{esc}[2J", esc = 27 as char);

    // Create a new system instance
    let mut system = System::new_all();
    system.refresh_all();

    println!("Name\t\t\tMemory");

    // Create a new HashMap to store the process name and memory usage
    let mut process_memory_map = HashMap::new();

    // Iterate over the processes in the system
    for (_, process) in system.processes() {
        // Get the name of the process
        let process_name = process.name().to_string();

        // Get the memory usage of the process
        let memory = process.memory(); // Memory usage in bytes.

        // Convert the memory to megabytes
        let memory_mb = (memory as f64) / (1024.0 * 1024.0);

        // Group processes by name and sum their memory usage
        let total_memory = process_memory_map.entry(process_name).or_insert(0.0);
        *total_memory += memory_mb;
    }

    // Sort processes by memory usage
    let mut sorted_processes: Vec<(&String, &f64)> = process_memory_map.iter().collect();
    sorted_processes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // Print the top 10 processes
    for (process_name, memory_mb) in sorted_processes.iter().take(10) {
        println!("{:.15}\t\t{:.2} MB", process_name, memory_mb);
    }
}
