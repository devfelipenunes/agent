mod ui;
mod bridge;
mod ollama;
mod agent;
mod tools;
mod skills;
mod memory;

use ui::AppUi;
use bridge::PythonBridge;
use tokio::sync::mpsc;
use anyhow::Result;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(100);
    let mut bridge = PythonBridge::start(tx).await?;
    
    let mut app_ui = AppUi::new()?;
    let mut chat_history = String::from("--- AetherMind Protocol V3/V8 ---\n");
    let mut radio_log = String::from("--- Squad Radio Frequency ---\n");
    let mut discovery_board = String::from("--- Discovery & Artifacts ---\n");
    let mut input_buffer = String::new();
    
    let mut focus: usize = 0; // 0: Chat, 1: Radio, 2: Discovery
    let mut scrolls: [u16; 3] = [0, 0, 0];

    loop {
        // 1. Process Eventos do Python
        while let Ok(event) = rx.try_recv() {
            match event.r#type.as_str() {
                "message" => {
                    chat_history.push_str(&format!("\n[{}] {}\n", event.agent, event.message));
                    scrolls[0] = chat_history.lines().count() as u16; // Auto-scroll
                }
                "radio" => {
                    radio_log.push_str(&format!("[{}] {}\n", event.agent, event.message));
                    scrolls[1] = radio_log.lines().count() as u16;
                }
                "thought" => {
                    radio_log.push_str(&format!("* {} thinks: {}\n", event.agent, event.message));
                    scrolls[1] = radio_log.lines().count() as u16;
                }
                "finding" => {
                    discovery_board.push_str(&format!("• {}\n", event.message));
                    scrolls[2] = discovery_board.lines().count() as u16;
                }
                "artifact_update" => {
                    discovery_board.push_str(&format!("\n[ARTIFACT] {}\n", event.message));
                    scrolls[2] = discovery_board.lines().count() as u16;
                }
                "status" => {
                    discovery_board = format!("Active Task: {} -> {}\n{}", event.agent, event.message, discovery_board);
                }
                "error" => {
                    chat_history.push_str(&format!("\n❌ SYSTEM ERROR: {}\n", event.message));
                }
                _ => {}
            }
        }
        
        // 2. Desenhar UI com suporte a 3 colunas e scroll
        app_ui.draw(&chat_history, &radio_log, &discovery_board, &input_buffer, focus, &scrolls)?;
        
        // 3. Processar Input e Navegação
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Tab => {
                        focus = (focus + 1) % 3;
                    }
                    KeyCode::Up => {
                        if scrolls[focus] > 0 {
                            scrolls[focus] -= 1;
                        }
                    }
                    KeyCode::Down => {
                        scrolls[focus] += 1;
                    }
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Enter => {
                        if !input_buffer.is_empty() {
                            chat_history.push_str(&format!("\nVocê: {}\n", input_buffer));
                            bridge.send_query(&input_buffer).await?;
                            input_buffer.clear();
                            scrolls[0] = chat_history.lines().count() as u16;
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    app_ui.destroy()?;
    println!("AetherMind Protocol Terminated.");
    Ok(())
}
