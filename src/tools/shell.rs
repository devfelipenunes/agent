use std::process::Command;
use crate::tools::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub struct ShellTool;

#[async_trait]
impl Tool for ShellTool {
    fn name(&self) -> &str { "run_command" }
    fn description(&self) -> &str { "Execute a shell command. Args: { \"command\": string }" }
    async fn run(&self, args: Value) -> Result<String> {
        let cmd = args["command"].as_str().ok_or_else(|| anyhow::anyhow!("Missing command"))?;
        
        // Em um projeto real, aqui haveria uma confirmação manual.
        // Por simplicidade técnica neste binário MVP, faremos a execução direta.
        let output = Command::new("sh").arg("-c").arg(cmd).output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        if output.status.success() {
            Ok(stdout)
        } else {
            Ok(format!("Error (exit code {}): {}", output.status.code().unwrap_or(-1), stderr))
        }
    }
}
