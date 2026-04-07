use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
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

    pub fn draw(&mut self, chat: &str, sidebar: &str) -> Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.area());

            let chat_block = Paragraph::new(chat).block(Block::default().title(" Chat ").borders(Borders::ALL));
            f.render_widget(chat_block, chunks[0]);

            let sidebar_block = Paragraph::new(sidebar).block(Block::default().title(" Intelligence ").borders(Borders::ALL));
            f.render_widget(sidebar_block, chunks[1]);
        })?;
        Ok(())
    }
}
