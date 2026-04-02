# Ollag: Elite Researcher & Personal Assistant Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform Ollag into an elite personal assistant with multi-agent research, dynamic skills, and semantic memory (RAG).

**Architecture:** Multi-agent roles + Tool/Skill Plugin System + Local Vector Store (RAG).

**Tech Stack:** Rust (Tokio, Ollama-rs, Reqwest, Serde, LanceDB/Qdrant-rs).

---

### Task 1: Skill System (Dynamic Tool Loading)
Allow the agent to load new capabilities from a `skills/` directory.

**Files:**
- Modify: `src/tools/mod.rs`
- Create: `src/skills/mod.rs`

- [ ] **Step 1: Define a dynamic Skill loader**
Implement a system that can parse YAML/JSON skill definitions and map them to shell scripts or internal tools.

- [ ] **Step 2: Commit**
```bash
git add src/tools/mod.rs src/skills/mod.rs
git commit -m "feat: implement dynamic skill loading system"
```

### Task 2: RAG Integration (Semantic Memory)
Giving the agent long-term memory and deep knowledge of your Obsidian vault.

**Files:**
- Modify: `Cargo.toml`
- Create: `src/memory/mod.rs`
- Create: `src/memory/vector_store.rs`

- [ ] **Step 1: Add Vector DB dependencies**
```toml
[dependencies]
lancedb = "0.4" # Or qdrant-client
```

- [ ] **Step 2: Implement Obsidian Indexer**
Create a tool that embeds Markdown files from `/l/disk0/fnunes/obsidian/` and stores them in the vector DB.

- [ ] **Step 3: Implement RAG Retrieval**
Modify the Agent loop to search the vector DB before answering, providing relevant context.

- [ ] **Step 4: Commit**
```bash
git add Cargo.toml src/memory/ src/tools/obsidian.rs
git commit -m "feat: add RAG support for Obsidian vault"
```

### Task 3: Web Search & Multi-Agent Evolution
Integrating real-time data and specialized roles.

**Files:**
- Create: `src/tools/search.rs`
- Modify: `src/agent.rs`

- [ ] **Step 1: Add Web Search (Tavily/Serper)**
- [ ] **Step 2: Implement Scout, Librarian, and Analyst roles**

- [ ] **Step 3: Commit**
```bash
git add src/tools/search.rs src/agent.rs
git commit -m "feat: implement multi-agent roles and web search"
```

### Task 4: Interactive TUI (The Command Center)
A rich terminal interface for your assistant.

**Files:**
- Modify: `src/main.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Integrate `ratatui` for a professional TUI**
- [ ] **Step 2: Commit**
```bash
git add Cargo.toml src/main.rs
git commit -m "feat: add professional TUI dashboard"
```
