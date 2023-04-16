use std::{
    io, thread, time::Duration,
    collections::HashMap,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Row, Cell, Table, BarChart, Dataset},
    layout::{Layout, Constraint, Direction},
    Terminal, text::Text,
    style::{Color, Modifier, Style},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Draw TUI with memory usage
        terminal.draw(|f| {
            let size = f.size();
            let memory_usage = get_memory_usage();

            let table_rows: Vec<Row> = memory_usage
                .iter()
                .map(|(process_name, memory_mb)| {
                    let memory_str = format!("{:.2} MB", memory_mb);
                    Row::new(vec![
                        Cell::from(process_name.as_str()),
                        Cell::from(memory_str)
                    ])
                    .height(1)
                })
                .collect();

            let table = Table::new(table_rows)
                .header(Row::new(vec![
                    Cell::from("Name"),
                    Cell::from("Memory")
                ]).height(1))
                .block(Block::default().title("Memory Usage").borders(Borders::ALL))
                .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

            f.render_widget(table, size);
        })?;

        thread::sleep(Duration::from_secs(1));
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn get_memory_usage() -> Vec<(String, f64)> {
    // Create a new system instance
    let mut system = System::new_all();
    system.refresh_all();

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
    let mut sorted_processes: Vec<(String, f64)> = process_memory_map.into_iter().collect();
    sorted_processes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Return the top 10 processes
    sorted_processes.into_iter().take(10).collect()
}
