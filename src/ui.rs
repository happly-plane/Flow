use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Row, Table},
    Frame,
};

use crate::state::AppState;

pub fn draw(frame: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // CPU
            Constraint::Length(3), // Mem
            Constraint::Min(0),    // Processes
        ])
        .split(frame.area());

    // CPU Usage
    let sys = &app.sys;
    let global_cpu = sys.global_cpu_usage();
    let cpu_gauge = Gauge::default()
        .block(Block::default().title(" CPU Usage ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .percent(global_cpu as u16);
    frame.render_widget(cpu_gauge, chunks[0]);

    // Memory Usage
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_percent = if total_mem > 0 {
        ((used_mem as f64 / total_mem as f64) * 100.0) as u16
    } else {
        0
    };

    let mem_label = format!("{:.1} GB / {:.1} GB", used_mem as f64 / 1_073_741_824.0, total_mem as f64 / 1_073_741_824.0);

    let mem_gauge = Gauge::default()
        .block(Block::default().title(" Memory Usage ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .percent(mem_percent)
        .label(mem_label);
    frame.render_widget(mem_gauge, chunks[1]);

    // Processes
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

    let mut rows = vec![];
    for p in processes.iter().take(50) { // Limit to top 50 to save rendering cost
        let mem_mb = p.memory() as f64 / 1_048_576.0;
        let row = Row::new(vec![
            p.pid().to_string(),
            p.name().to_str().unwrap_or("").to_string(),
            format!("{:.1}%", p.cpu_usage()),
            format!("{:.1} MB", mem_mb),
        ]);
        rows.push(row);
    }

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Min(20),
            Constraint::Length(10),
            Constraint::Length(15),
        ],
    )
    .header(
        Row::new(vec!["PID", "Name", "CPU", "Memory"])
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    )
    .block(Block::default().title(" Processes ").borders(Borders::ALL))
    .column_spacing(2);

    frame.render_widget(table, chunks[2]);
}
