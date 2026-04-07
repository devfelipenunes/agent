use serde_json::Value;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod fs;
pub mod shell;
pub mod obsidian;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn run(&self, args: Value) -> Result<String>;
}

pub struct Toolbox {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn list_tools(&self) -> Vec<(&str, &str)> {
        self.tools.values()
            .map(|t| (t.name(), t.description()))
            .collect()
    }

    pub async fn call(&self, name: &str, args: Value) -> Result<String> {
        let tool = self.tools.get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;
        tool.run(args).await
    }
}
