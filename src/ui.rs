use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, BorderType, Wrap},
    layout::{Layout, Constraint, Direction, Alignment},
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

    pub fn draw(
        &mut self, 
        chat: &str, 
        radio: &str, 
        discovery: &str, 
        input: &str, 
        focus: usize,
        scrolls: &[u16; 3]
    ) -> Result<()> {
        self.terminal.draw(|f| {
            let area = f.area();

            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(area);

            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30), // Chat
                    Constraint::Percentage(40), // Squad Radio
                    Constraint::Percentage(30), // Discovery
                ].as_ref())
                .split(outer_layout[0]);

            let focus_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);

            // 1. Main Chat
            let chat_block = Block::default()
                .title(" 💬 MAIN CHAT ")
                .borders(Borders::ALL)
                .border_type(if focus == 0 { BorderType::Thick } else { BorderType::Plain })
                .border_style(if focus == 0 { focus_style } else { normal_style });
            let chat_para = Paragraph::new(chat)
                .block(chat_block)
                .wrap(Wrap { trim: true })
                .scroll((scrolls[0], 0));
            f.render_widget(chat_para, main_layout[0]);

            // 2. Squad Radio
            let radio_block = Block::default()
                .title(" 📡 SQUAD RADIO (Internal) ")
                .borders(Borders::ALL)
                .border_type(if focus == 1 { BorderType::Thick } else { BorderType::Plain })
                .border_style(if focus == 1 { focus_style } else { normal_style });
            let radio_para = Paragraph::new(radio)
                .block(radio_block)
                .style(Style::default().fg(Color::Gray))
                .wrap(Wrap { trim: true })
                .scroll((scrolls[1], 0));
            f.render_widget(radio_para, main_layout[1]);

            // 3. Discovery Board
            let discovery_block = Block::default()
                .title(" 🧭 DISCOVERY BOARD ")
                .borders(Borders::ALL)
                .border_type(if focus == 2 { BorderType::Thick } else { BorderType::Plain })
                .border_style(if focus == 2 { focus_style } else { normal_style });
            let discovery_para = Paragraph::new(discovery)
                .block(discovery_block)
                .wrap(Wrap { trim: true })
                .scroll((scrolls[2], 0));
            f.render_widget(discovery_para, main_layout[2]);

            // 4. Input Bar
            let input_block = Block::default()
                .title(" ⌨️  INPUT (Enter to Send, Tab to Focus, Esc to Quit) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            let input_para = Paragraph::new(input).block(input_block);
            f.render_widget(input_para, outer_layout[1]);
        })?;
        Ok(())
    }
}


