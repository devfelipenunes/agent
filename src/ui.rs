use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, BorderType, Wrap, Tabs},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Style, Color, Modifier},
    text::{Line, Span},
    Terminal,
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io::{self, stdout};
use anyhow::Result;

pub struct AppUi {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl AppUi {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn destroy(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw_startup(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Length(10),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(area);

            let menu_block = Block::default()
                .title(" AetherMind Protocol Selection ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

            let menu_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  [L] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw("Local Mode (All Ollama / Offline)"),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  [T] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::raw("Token Mode (OpenAI Chat + Ollama Workers)"),
                ]),
                Line::from(""),
                Line::from("  Press the corresponding key to initialize..."),
            ];

            let menu_para = Paragraph::new(menu_text)
                .block(menu_block)
                .alignment(Alignment::Center);

            f.render_widget(menu_para, chunks[1]);
        })?;
        Ok(())
    }

    pub fn draw(
        &mut self, 
        chat: &str, 
        reader: &str,
        radio: &str, 
        squad: &str,
        input: &str, 
        active_tab: usize,
        scrolls: &[u16; 4],
        background_status: &str
    ) -> Result<()> {
        self.terminal.draw(|f| {
            let area = f.area();
            
            // Layout: Tabs | Content | Background Monitor | Input
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Tabs
                    Constraint::Min(3),    // Main Content
                    Constraint::Length(1), // Background Monitor (New)
                    Constraint::Length(3), // Input
                ].as_ref())
                .split(area);

            // 1. Render Tabs
            let titles = vec![
                Line::from(Span::styled(" F1:CHAT ", Style::default().fg(Color::Yellow))),
                Line::from(Span::styled(" F2:READER ", Style::default().fg(Color::Cyan))),
                Line::from(Span::styled(" F3:RADIO ", Style::default().fg(Color::Magenta))),
                Line::from(Span::styled(" F4:SQUAD ", Style::default().fg(Color::Green))),
            ];
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title(" AetherMind v3.1 Workspace "))
                .select(active_tab)
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            f.render_widget(tabs, chunks[0]);

            // 2. Render Active View
            match active_tab {
                0 => {
                    let chat_block = Block::default()
                        .title(" Dialogue Stream ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow));
                    let chat_para = Paragraph::new(chat)
                        .block(chat_block)
                        .wrap(Wrap { trim: true })
                        .scroll((scrolls[0], 0));
                    f.render_widget(chat_para, chunks[1]);
                }
                1 => {
                    let reader_block = Block::default()
                        .title(" Documentation Reader ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan));
                    let reader_para = Paragraph::new(reader)
                        .block(reader_block)
                        .wrap(Wrap { trim: true })
                        .scroll((scrolls[1], 0));
                    f.render_widget(reader_para, chunks[1]);
                }
                2 => {
                    let radio_block = Block::default()
                        .title(" Internal Frequencies ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Magenta));
                    let radio_para = Paragraph::new(radio)
                        .block(radio_block)
                        .wrap(Wrap { trim: true })
                        .style(Style::default().fg(Color::Gray))
                        .scroll((scrolls[2], 0));
                    f.render_widget(radio_para, chunks[1]);
                }
                3 => {
                    let squad_block = Block::default()
                        .title(" Squad Operations Center ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green));
                    let squad_para = Paragraph::new(squad)
                        .block(squad_block)
                        .scroll((scrolls[3], 0));
                    f.render_widget(squad_para, chunks[1]);
                }
                _ => {}
            }

            // 3. Render Background Monitor
            let status_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC);
            let status_para = Paragraph::new(Span::styled(format!("  🛰️  {}", background_status), status_style));
            f.render_widget(status_para, chunks[2]);

            // 4. Render Input
            let input_block = Block::default()
                .title(" ⌨️  INPUT (Enter to Send, F1-F4 Switch Tabs, Esc to Quit) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White));
            let input_para = Paragraph::new(input).block(input_block);
            f.render_widget(input_para, chunks[3]);
        })?;
        Ok(())
    }
}
