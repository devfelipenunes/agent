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
pub struct BridgeRequest {
    pub query: String,
}

pub struct PythonBridge {
    stdin: ChildStdin,
}

impl PythonBridge {
    pub async fn start(tx: mpsc::Sender<BridgeEvent>) -> Result<Self> {
        let mut child = Command::new("python3")
            .arg("agent/src/core/graph.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

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

        Ok(Self { stdin })
    }

    pub async fn send_query(&mut self, query: &str) -> Result<()> {
        let req = BridgeRequest { query: query.to_string() };
        let json = serde_json::to_string(&req)?;
        self.stdin.write_all(format!("{}\n", json).as_bytes()).await?;
        self.stdin.flush().await?;
        Ok(())
    }
}
