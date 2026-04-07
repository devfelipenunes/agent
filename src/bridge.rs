use tokio::process::{Command, ChildStdin};
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Deserialize, Debug)]
pub struct BridgeEvent {
    pub r#type: String,
    pub agent: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct BridgeConfig {
    pub r#type: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct BridgeRequest {
    pub query: String,
}

pub struct PythonBridge {
    stdin: ChildStdin,
    child_id: u32,
}

impl PythonBridge {
    pub async fn start(tx: mpsc::Sender<BridgeEvent>) -> Result<Self> {
        let mut child = Command::new("agent/venv-elite/bin/python3")
            .arg("agent/src/core/graph.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let child_id = child.id().expect("Failed to get child ID");
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Ok(event) = serde_json::from_str::<BridgeEvent>(&line) {
                    let _ = tx.send(event).await;
                }
            }
        });

        Ok(Self { stdin, child_id })
    }

    pub async fn send_query(&mut self, query: &str) -> Result<()> {
        let req = BridgeRequest { query: query.to_string() };
        let json = serde_json::to_string(&req)?;
        self.stdin.write_all(format!("{}\n", json).as_bytes()).await?;
        self.stdin.flush().await?;
        Ok(())
    }

    pub async fn send_config(&mut self, mode: &str) -> Result<()> {
        let cfg = BridgeConfig { 
            r#type: "config".to_string(), 
            mode: mode.to_string() 
        };
        let json = serde_json::to_string(&cfg)?;
        self.stdin.write_all(format!("{}\n", json).as_bytes()).await?;
        self.stdin.flush().await?;
        Ok(())
    }
}

impl Drop for PythonBridge {
    fn drop(&mut self) {
        // Enviar sinal de término via sistema operacional para garantir que o processo morra
        let pid = self.child_id;
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
    }
}

#[cfg(test)]
#[path = "bridge_tests.rs"]
mod bridge_tests;
