use std::fs;
use crate::tools::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str { "read_file" }
    fn description(&self) -> &str { "Read content of a file. Args: { \"path\": string }" }
    async fn run(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
}

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &str { "write_file" }
    fn description(&self) -> &str { "Write content to a file. Args: { \"path\": string, \"content\": string }" }
    async fn run(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
        let content = args["content"].as_str().ok_or_else(|| anyhow::anyhow!("Missing content"))?;
        
        // Em um projeto real, aqui haveria uma confirmação manual.
        // Por simplicidade técnica neste binário MVP, faremos a escrita direta.
        fs::write(path, content)?;
        Ok(format!("File written to {}", path))
    }
}

pub struct ListDirTool;

#[async_trait]
impl Tool for ListDirTool {
    fn name(&self) -> &str { "list_dir" }
    fn description(&self) -> &str { "List files and subdirectories. Args: { \"path\": string }" }
    async fn run(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
        let entries = fs::read_dir(path)?;
        let mut result = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy();
            let type_str = if path.is_dir() { "DIR" } else { "FILE" };
            result.push(format!("[{}] {}", type_str, file_name));
        }
        Ok(result.join("\n"))
    }
}
