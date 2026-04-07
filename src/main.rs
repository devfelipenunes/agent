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

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(100);
    let mut bridge = PythonBridge::start(tx).await?;
    
    let mut app_ui = AppUi::new()?;
    let mut chat_history = String::from("AetherMind V3/V8 Squad Initialized.\nType your research query:\n> ");
    let mut sidebar_content = String::from("Status: Idle\n\nFindings:\n");

    // Simulate an initial query for testing the loop without input handling
    bridge.send_query("Interoperability in Blockchain").await?;

    loop {
        // Poll events
        if let Ok(event) = tokio::time::timeout(Duration::from_millis(100), rx.recv()).await {
            if let Some(ev) = event {
                match ev.r#type.as_str() {
                    "message" => chat_history.push_str(&format!("\n[{}] {}\n", ev.agent, ev.message)),
                    "status" => sidebar_content = format!("Status: {} is {}\n\n{}", ev.agent, ev.message, sidebar_content),
                    "finding" => sidebar_content.push_str(&format!("- {}\n", ev.message)),
                    "artifact_update" => sidebar_content.push_str(&format!("\n[Artifact] {}\n", ev.message)),
                    "system" => if ev.message == "Workflow completed" { break; },
                    _ => {}
                }
            }
        }
        
        app_ui.draw(&chat_history, &sidebar_content)?;
        
        // Simple exit condition for the test
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    app_ui.destroy()?;
    println!("AetherMind session ended.");
    Ok(())
}
