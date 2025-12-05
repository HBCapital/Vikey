// demo_ui.rs - Terminal UI demo for Vikey Vietnamese Telex

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use vikey_core::{Engine, Action};
use vikey_vietnamese::VietnamesePlugin;

struct App {
    engine: Engine,
    output: String,
    status: String,
}

impl App {
    fn new() -> Result<Self> {
        let mut engine = Engine::new();
        engine.register(Box::new(VietnamesePlugin::new()))?;
        engine.set_language("vietnamese")?;
        engine.set_input_method("telex")?;
        
        Ok(Self {
            engine,
            output: String::new(),
            status: "Ready".to_string(),
        })
    }
    
    fn process_key(&mut self, c: char) {
        let action = self.engine.process(c);
        
        match action {
            Action::Commit(text) => {
                self.output.push_str(&text);
                self.status = format!("Committed: {}", text);
            }
            Action::Replace { backspace_count, text } => {
                // Remove last N characters
                for _ in 0..backspace_count {
                    self.output.pop();
                }
                self.output.push_str(&text);
                self.status = format!("Replaced: {}", text);
            }
            Action::DoNothing => {
                self.status = "No action".to_string();
            }
        }
    }
    
    fn process_backspace(&mut self) {
        let action = self.engine.process_backspace();
        
        match action {
            Action::Replace { backspace_count, text } => {
                for _ in 0..backspace_count {
                    self.output.pop();
                }
                if !text.is_empty() {
                    self.output.push_str(&text);
                }
                self.status = "Backspace".to_string();
            }
            _ => {
                self.status = "Nothing to delete".to_string();
            }
        }
    }
    
    fn process_space(&mut self) {
        self.output.push(' ');
        self.engine.reset();
        self.status = "Space - buffer reset".to_string();
    }
    
    fn process_enter(&mut self) {
        self.output.push('\n');
        self.engine.reset();
        self.status = "Enter - buffer reset".to_string();
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(5),  // Input buffer
            Constraint::Min(5),     // Output
            Constraint::Length(7),  // Help
            Constraint::Length(3),  // Status
        ])
        .split(f.size());
    
    // Title
    let title = Paragraph::new("🇻🇳 Vikey Demo - Vietnamese Telex Input Method")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
    
    // Input buffer
    let buffer_content = app.engine.buffer_content();
    let buffer_text = vec![
        Line::from(vec![
            Span::styled("Buffer: ", Style::default().fg(Color::Yellow)),
            Span::raw(&buffer_content),
        ]),
        Line::from(vec![
            Span::styled("Length: ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{} chars", buffer_content.len())),
        ]),
    ];
    let buffer = Paragraph::new(buffer_text)
        .block(Block::default().title("Input Buffer").borders(Borders::ALL));
    f.render_widget(buffer, chunks[1]);
    
    // Output
    let output = Paragraph::new(app.output.clone())
        .style(Style::default().fg(Color::Green))
        .block(Block::default().title("Output").borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    f.render_widget(output, chunks[2]);
    
    // Help
    let help_text = vec![
        Line::from("Commands:"),
        Line::from("  Type Vietnamese: aa→â, aw→ă, s→sắc, f→huyền, etc."),
        Line::from("  Space: Commit and reset buffer"),
        Line::from("  Backspace: Undo last transformation"),
        Line::from("  Ctrl+C or Esc: Exit"),
    ];
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().title("Help").borders(Borders::ALL));
    f.render_widget(help, chunks[3]);
    
    // Status
    let status = Paragraph::new(app.status.clone())
        .style(Style::default().fg(Color::Magenta))
        .block(Block::default().title("Status").borders(Borders::ALL));
    f.render_widget(status, chunks[4]);
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app
    let mut app = App::new()?;
    
    // Main loop
    loop {
        terminal.draw(|f| ui(f, &app))?;
        
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Char(c) => {
                    app.process_key(c);
                }
                KeyCode::Backspace => {
                    app.process_backspace();
                }
                KeyCode::Enter => {
                    app.process_enter();
                }
                KeyCode::Tab => {
                    app.process_space();
                }
                _ => {}
            }
        }
    }
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    Ok(())
}
