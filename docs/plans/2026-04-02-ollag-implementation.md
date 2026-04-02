# Ollag: Ollama Agent in Rust Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build an autonomous CLI agent in Rust that uses Ollama to perform file and shell tasks.

**Architecture:** ReAct loop (Think-Act-Observe) using `qwen2.5-coder:7b` for reasoning and tool invocation via JSON.

**Tech Stack:** Rust (Tokio, Ollama-rs, Clap, Serde).

---

### Task 1: Project Setup and Dependencies

**Files:**
- Create: `Cargo.toml`
- Modify: `src/main.rs`

- [ ] **Step 1: Configure Cargo.toml**

```toml
[package]
name = "ollag"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
ollama-rs = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
dialoguer = "0.11"
futures-util = "0.3"
```

- [ ] **Step 2: Basic main.rs**

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ollag: Starting...");
    Ok(())
}
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: Success

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src/main.rs
git commit -m "chore: initial project setup"
```

### Task 2: Ollama Client Implementation

**Files:**
- Create: `src/ollama.rs`

- [ ] **Step 1: Implement Ollama client wrapper**

```rust
use ollama_rs::Ollama;
use anyhow::Result;

pub struct OllamaClient {
    client: Ollama,
    model: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        Self {
            client: Ollama::default(),
            model,
        }
    }
}
```

- [ ] **Step 2: Add simple generation method**

```rust
impl OllamaClient {
    pub async fn generate(&self, prompt: String) -> Result<String> {
        let res = self.client.generate(self.model.clone(), prompt).await?;
        Ok(res.response)
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/ollama.rs
git commit -m "feat: add ollama client wrapper"
```

### Task 3: Tool Registry and Common Traits

**Files:**
- Create: `src/tools/mod.rs`

- [ ] **Step 1: Define Tool trait**

```rust
use serde_json::Value;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn run(&self, args: Value) -> Result<String>;
}
```

- [ ] **Step 2: Implement Tool Registry**

```rust
use std::collections::HashMap;

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
}
```

- [ ] **Step 3: Commit**

```bash
git add src/tools/mod.rs
git commit -m "feat: implement tool system and registry"
```

### Task 4: File System Tools

**Files:**
- Create: `src/tools/fs.rs`

- [ ] **Step 1: Implement ReadFileTool**

```rust
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
```

- [ ] **Step 2: Implement WriteFileTool (with safety)**

```rust
pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &str { "write_file" }
    fn description(&self) -> &str { "Write content to a file. Args: { \"path\": string, \"content\": string }" }
    async fn run(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
        let content = args["content"].as_str().ok_or_else(|| anyhow::anyhow!("Missing content"))?;
        fs::write(path, content)?;
        Ok(format!("File written to {}", path))
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/tools/fs.rs
git commit -m "feat: add filesystem tools"
```

### Task 5: Shell Tool (run_command)

**Files:**
- Create: `src/tools/shell.rs`

- [ ] **Step 1: Implement ShellTool**

```rust
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
        let output = Command::new("sh").arg("-c").arg(cmd).output()?;
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/tools/shell.rs
git commit -m "feat: add shell tool"
```

### Task 6: ReAct loop (Orchestrator)

**Files:**
- Create: `src/agent.rs`

- [ ] **Step 1: Implement Agent loop logic**

```rust
use crate::ollama::OllamaClient;
use crate::tools::Toolbox;
use anyhow::Result;

pub struct Agent {
    client: OllamaClient,
    toolbox: Toolbox,
}

impl Agent {
    pub async fn run(&self, task: &str) -> Result<()> {
        let system_prompt = format!(
            "You are an AI assistant using tools. Available tools: {:?}. Respond with Thought and Action (JSON).",
            self.toolbox.list_tools() // Add this method to Toolbox later
        );
        // Implement the actual loop here
        Ok(())
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/agent.rs
git commit -m "feat: implement basic agent orchestrator"
```

### Task 7: Final CLI Integration

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Implement CLI parsing and Agent start**

```rust
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "qwen2.5-coder:7b")]
    model: String,
    task: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    // Initialize Agent and run
    Ok(())
}
```

- [ ] **Step 2: Final Verify and Test**

Run: `cargo build`
Expected: Success

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: final cli integration"
```
