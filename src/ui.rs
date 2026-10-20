use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::state::AppState;
use crate::big_text::BigText;

pub fn draw(frame: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(7), // Big text area
            Constraint::Length(5), // Input sparkline
            Constraint::Length(5), // Output sparkline
            Constraint::Min(0),    // Footer
        ])
        .split(frame.area());

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Format numbers
    let input_str = format!("{}", app.input_tokens_total);
    let output_str = format!("{}", app.output_tokens_total);

    // Input total big text
    let input_big_text = BigText::new(&input_str).style(Style::default().fg(Color::Cyan));
    frame.render_widget(input_big_text, header_chunks[0]);
    frame.render_widget(
        Paragraph::new("Input Tokens").style(Style::default().fg(Color::Cyan)),
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(5), Constraint::Length(2)]).split(header_chunks[0])[1]
    );

    // Output total big text
    let output_big_text = BigText::new(&output_str).style(Style::default().fg(Color::Yellow));
    frame.render_widget(output_big_text, header_chunks[1]);
    frame.render_widget(
        Paragraph::new("Output Tokens").style(Style::default().fg(Color::Yellow)),
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(5), Constraint::Length(2)]).split(header_chunks[1])[1]
    );

    // Input Sparkline
    let input_hist: Vec<u64> = app.input_history.iter().copied().collect();
    let max_in = input_hist.iter().max().copied().unwrap_or(1).max(1);
    let in_sparkline = Sparkline::default()
        .block(Block::default().title("Input (tokens/s)").borders(Borders::NONE))
        .data(&input_hist)
        .max(max_in)
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(in_sparkline, chunks[1]);

    // Output Sparkline
    let output_hist: Vec<u64> = app.output_history.iter().copied().collect();
    let max_out = output_hist.iter().max().copied().unwrap_or(1).max(1);
    let out_sparkline = Sparkline::default()
        .block(Block::default().title("Output (tokens/s)").borders(Borders::NONE))
        .data(&output_hist)
        .max(max_out)
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(out_sparkline, chunks[2]);

    // Footer
    let footer_text = format!(
        "flow-claude · Input: {} · Output: {} · Press 'q' to quit",
        app.input_tokens_total, app.output_tokens_total
    );
    let footer = Paragraph::new(Span::raw(footer_text))
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, chunks[3]);
}
