use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table},
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
            Constraint::Length(4), // ARM Stats
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

    // ARM Stats
    let mut arm_lines = vec![];
    let gpu_freq_str = match app.arm_data.gpu_freq_mhz {
        Some(freq) => format!("{} MHz", freq),
        None => "N/A".to_string(),
    };
    let gpu_load_str = match app.arm_data.gpu_load_percent {
        Some(load) => format!("{}%", load),
        None => "N/A".to_string(),
    };
    arm_lines.push(Line::from(vec![
        Span::styled("GPU Freq: ", Style::default().fg(Color::Yellow)),
        Span::raw(gpu_freq_str),
        Span::raw(" | "),
        Span::styled("GPU Load: ", Style::default().fg(Color::Yellow)),
        Span::raw(gpu_load_str),
    ]));

    let temps_str = if app.arm_data.temps.is_empty() {
        "N/A".to_string()
    } else {
        app.arm_data.temps.iter().map(|(name, temp)| format!("{}: {:.1}°C", name, temp)).collect::<Vec<_>>().join(", ")
    };
    arm_lines.push(Line::from(vec![
        Span::styled("Temps: ", Style::default().fg(Color::Red)),
        Span::raw(temps_str),
    ]));

    let arm_para = Paragraph::new(arm_lines)
        .block(Block::default().title(" ARM Sensors ").borders(Borders::ALL));
    frame.render_widget(arm_para, chunks[2]);


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

    frame.render_widget(table, chunks[3]);
}
