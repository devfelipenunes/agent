# AetherMind Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the AetherMind hybrid architecture (Rust TUI + Python LangGraph back-end) with the V3/V8 Squad for asynchronous research and fluid chat.

**Architecture:** 
1. Refactor the Python agent to use LangGraph with the V3/V8 Squad roles.
2. Implement a JSON-RPC streaming bridge over stdout between Python and Rust.
3. Build a split-pane Ratatui interface in Rust (Chat + Sidebar).

**Tech Stack:** Rust (Tokio, Ratatui, Serde), Python (LangGraph, LangChain, Cognee, Ollama).

---

### Task 1: Python - Define the V3/V8 Squad State and Nodes

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Update AgentState to support squad roles and findings**

```python
from typing import TypedDict, List, Dict, Any
from langgraph.graph import StateGraph, END
from langchain_ollama import ChatOllama
import json
import sys

class AgentState(TypedDict):
    query: str
    chat_history: List[Dict[str, str]]
    findings: List[str]
    artifact: str
    current_status: str
```

- [ ] **Step 2: Implement the bridge logging function**

```python
def emit_event(event_type: str, agent: str, message: str):
    """Emits JSON-RPC style events to stdout for Rust to consume."""
    event = {
        "type": event_type,
        "agent": agent,
        "message": message
    }
    print(json.dumps(event), flush=True)
```

- [ ] **Step 3: Implement V3-Librarian Node (Immediate Response)**

```python
def librarian_node(state: AgentState):
    query = state["query"]
    emit_event("status", "V3-Librarian", "Buscando contexto local (Obsidian)...")
    
    # Simulação de busca no Cognee/Obsidian
    local_context = f"Contexto local para: {query}"
    
    emit_event("message", "V3-Librarian", f"Baseado nas suas notas, {query} é um tópico interessante. Iniciando pesquisa profunda...")
    
    return {"current_status": "librarian_done"}
```

- [ ] **Step 4: Implement V3-Scout Node (Web/Local Search)**

```python
def scout_node(state: AgentState):
    query = state["query"]
    emit_event("status", "V3-Scout", "Minerando a Web e GitHub...")
    
    # Simulação de busca
    finding = f"Artigo recente sobre {query} encontrado."
    emit_event("finding", "V3-Scout", finding)
    
    return {"findings": [finding], "current_status": "scout_done"}
```

- [ ] **Step 5: Implement V8-Architect Node (Artifact Generation)**

```python
def architect_node(state: AgentState):
    emit_event("status", "V8-Architect", "Estruturando o artefato final...")
    findings = "\n".join(state.get("findings", []))
    
    artifact = f"# Draft: {state['query']}\n\nBaseado nas pesquisas:\n{findings}"
    emit_event("artifact_update", "V8-Architect", "Rascunho gerado.")
    
    return {"artifact": artifact, "current_status": "architect_done"}
```

- [ ] **Step 6: Compile the Graph**

```python
workflow = StateGraph(AgentState)
workflow.add_node("librarian", librarian_node)
workflow.add_node("scout", scout_node)
workflow.add_node("architect", architect_node)

workflow.set_entry_point("librarian")
workflow.add_edge("librarian", "scout")
workflow.add_edge("scout", "architect")
workflow.add_edge("architect", END)

app = workflow.compile()
```

- [ ] **Step 7: Update main execution for JSON stdin/stdout loop**

```python
def main_loop():
    emit_event("system", "System", "AetherMind Python Backend Started")
    for line in sys.stdin:
        try:
            req = json.loads(line)
            query = req.get("query", "")
            if query:
                inputs = {"query": query, "chat_history": [], "findings": [], "artifact": "", "current_status": ""}
                # Executa o grafo sincrono para o exemplo, idealmente async
                app.invoke(inputs)
                emit_event("system", "System", "Workflow completed")
        except Exception as e:
            emit_event("error", "System", str(e))

if __name__ == "__main__":
    main_loop()
```

- [ ] **Step 8: Commit**

```bash
git add agent/src/core/graph.py
git commit -m "feat: implement V3/V8 squad nodes and JSON-RPC bridge in Python"
```

### Task 2: Rust - Add Ratatui Dependencies and UI Scaffolding

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/main.rs`
- Create: `src/ui.rs`

- [ ] **Step 1: Add dependencies to Cargo.toml**

Run: `cargo add ratatui crossterm tokio-util --features tokio-util/codec`

- [ ] **Step 2: Create UI Module Skeleton (src/ui.rs)**

```rust
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
                .split(f.size());

            let chat_block = Paragraph::new(chat).block(Block::default().title(" Chat ").borders(Borders::ALL));
            f.render_widget(chat_block, chunks[0]);

            let sidebar_block = Paragraph::new(sidebar).block(Block::default().title(" Intelligence ").borders(Borders::ALL));
            f.render_widget(sidebar_block, chunks[1]);
        })?;
        Ok(())
    }
}
```

- [ ] **Step 3: Expose UI in main.rs**

```rust
// Add to top of src/main.rs
// mod ui;
```
*(Wait to fully integrate until the bridge is built).*

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src/ui.rs src/main.rs
git commit -m "feat: add ratatui dependency and ui scaffolding"
```

### Task 3: Rust - Python Bridge (JSON-RPC)

**Files:**
- Create: `src/bridge.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Implement the Bridge**

```rust
use tokio::process::{Command, Child, ChildStdin, ChildStdout};
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
```

- [ ] **Step 2: Commit**

```bash
git add src/bridge.rs src/main.rs
git commit -m "feat: implement rust to python subprocess bridge"
```

### Task 4: Integration - The TUI Loop

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Wire it all together**

```rust
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
```

- [ ] **Step 2: Verify Build**

Run: `cargo build`
Expected: Success

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: integrate tui loop with python bridge"
```