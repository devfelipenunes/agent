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
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(100);
    let mut bridge = PythonBridge::start(tx).await?;
    
    let mut app_ui = AppUi::new()?;
    
    let mut chat_buffer = String::from("AetherMind v4 Autonomous System Online.\nType a message to begin.\n");
    let mut reader_buffer = String::from("# Waiting for artifacts...\nAsk me to write an article or report to view the result here.");
    let mut radio_buffer = String::from("--- Internal Squad Frequencies ---\n");
    
    // Squad State Tracking
    let mut agents_status: HashMap<String, String> = HashMap::new();
    let default_agents = vec!["Aether-PM", "V3-Librarian", "V3-Scout", "V3-Analyst", "V8-Writer", "V8-Critic"];
    for agent in &default_agents {
        agents_status.insert(agent.to_string(), "Idle".to_string());
    }
    
    let mut latest_background_status = String::from("System: Ready");
    
    let mut input_buffer = String::new();
    let mut active_tab: usize = 0;
    let mut scrolls: [u16; 4] = [0, 0, 0, 0];

    loop {
        let mut ui_needs_update = false;

        while let Ok(event) = rx.try_recv() {
            ui_needs_update = true;
            match event.r#type.as_str() {
                "message_start" => {
                    chat_buffer.push_str(&format!("\n\n[🤖 {}]: ", event.agent));
                    scrolls[0] = chat_buffer.lines().count().saturating_sub(1) as u16;
                }
                "token" => {
                    chat_buffer.push_str(&event.message);
                    scrolls[0] = chat_buffer.lines().count().saturating_sub(1) as u16;
                }
                "artifact_start" => {
                    reader_buffer.clear();
                }
                "artifact_chunk" => {
                    reader_buffer.push_str(&event.message);
                    scrolls[1] = reader_buffer.lines().count().saturating_sub(1) as u16;
                }
                "artifact_full" => {
                    chat_buffer.push_str("\n\n✅ [SISTEMA] Novo artefato gerado com sucesso! Mude para a aba READER (F2) para visualizar o documento completo.");
                    scrolls[0] = chat_buffer.lines().count().saturating_sub(1) as u16;
                }
                "radio" | "thought" => {
                    let prefix = if event.r#type == "thought" { "💡" } else { "📡" };
                    radio_buffer.push_str(&format!("{} [{}] {}\n", prefix, event.agent, event.message));
                    scrolls[2] = radio_buffer.lines().count().saturating_sub(1) as u16;
                }
                "status" => {
                    agents_status.insert(event.agent.clone(), event.message.clone());
                    latest_background_status = format!("{} -> {}", event.agent, event.message);
                }
                "system" => {
                    latest_background_status = format!("System: {}", event.message);
                    radio_buffer.push_str(&format!("⚙️ SYSTEM: {}\n", event.message));
                    scrolls[2] = radio_buffer.lines().count().saturating_sub(1) as u16;
                }
                "error" => {
                    latest_background_status = format!("ERROR: {}", event.message);
                    chat_buffer.push_str(&format!("\n\n❌ ERRO FATAL [{}]: {}\n", event.agent, event.message));
                    scrolls[0] = chat_buffer.lines().count().saturating_sub(1) as u16;
                }
                _ => {}
            }
        }
        
        // Build the squad status text
        let mut squad_display = String::from("🛡️ AETHERMIND SQUAD STATUS 🛡️\n=================================\n\n");
        for agent in &default_agents {
            let status = agents_status.get(*agent).unwrap_or(&"Idle".to_string()).clone();
            let icon = if status == "Idle" { "💤" } else { "🔄" };
            squad_display.push_str(&format!("{} {:<15} | {}\n\n", icon, agent, status));
        }

        // Draw TUI
        app_ui.draw(&chat_buffer, &reader_buffer, &radio_buffer, &squad_display, &input_buffer, active_tab, &scrolls, &latest_background_status)?;
        
        if event::poll(Duration::from_millis(20))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::F(1) => active_tab = 0,
                    KeyCode::F(2) => active_tab = 1,
                    KeyCode::F(3) => active_tab = 2,
                    KeyCode::F(4) => active_tab = 3,
                    KeyCode::Up => scrolls[active_tab] = scrolls[active_tab].saturating_sub(1),
                    KeyCode::Down => scrolls[active_tab] = scrolls[active_tab].saturating_add(1),
                    KeyCode::Char(c) => input_buffer.push(c),
                    KeyCode::Backspace => { input_buffer.pop(); }
                    KeyCode::Enter => {
                        if !input_buffer.is_empty() {
                            chat_buffer.push_str(&format!("\n\n[👤 Você]: {}\n", input_buffer));
                            latest_background_status = String::from("Aether-PM -> Processando query...");
                            bridge.send_query(&input_buffer).await?;
                            input_buffer.clear();
                            active_tab = 0;
                            scrolls[0] = chat_buffer.lines().count().saturating_sub(1) as u16;
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    app_ui.destroy()?;
    Ok(())
}
