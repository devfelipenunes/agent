# AetherMind v4 Implementation Plan - Autonomous RAG & ArXiv

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement ArXiv search, a self-updating Obsidian RAG loop, and a dedicated Squad Dashboard in the UI.

---

### Task 1: Python - The ArXiv Tool

**Files:**
- Create: `agent/src/tools/arxiv.py`

- [ ] **Step 1: Implement `search_arxiv` function**
Create a Python module that uses `urllib` and `xml.etree.ElementTree` to query `http://export.arxiv.org/api/query` based on the user's provided snippet.
It should take a `query` string and `max_results` int, returning a formatted Markdown string of results.

- [ ] **Step 2: Commit**
`git add agent/src/tools/arxiv.py && git commit -m "feat(python): implement arxiv search tool"`

---

### Task 2: Python - Obsidian Writing & RAG Loop Update

**Files:**
- Modify: `agent/src/tools/mcp_obsidian.py`
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Add `write_note` to ObsidianTool**
Extend the Obsidian class to allow creating a new Markdown file in the vault.
```python
def write_note(self, title: str, content: str):
    # Sanitize title and save to self.vault_path
```

- [ ] **Step 2: Integrate ArXiv in `scout_node`**
If the local context was poor (add a flag in state), the Scout should call the new ArXiv tool in addition to Tavily. Append the ArXiv findings to the state.

- [ ] **Step 3: Update `librarian_node` for Ingestion**
Add logic *after* the analysis/writing phase (or within Analyst) to check if new external data was discovered. If so, call `obsidian.write_note()` to save the new knowledge.

- [ ] **Step 4: Commit**
`git add agent/src/tools/mcp_obsidian.py agent/src/core/graph.py && git commit -m "feat(python): implement bidirectional Obsidian RAG and ArXiv integration"`

---

### Task 3: Rust - Squad Dashboard UI

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Redesign UI Tabs**
Change `F3` to SQUADS (Dashboard) and `F4` to LOGS.

- [ ] **Step 2: Implement Squad Dashboard rendering**
Instead of a single scrolling text block for Squads, create a structured view (e.g., a grid or a list) that shows the current state of each agent (Librarian, Scout, Analyst, Writer, Critic).
Update `main.rs` to track individual agent statuses in a `HashMap` or `struct` rather than a single string buffer.

- [ ] **Step 3: Commit**
`git add src/ui.rs src/main.rs && git commit -m "feat(rust): implement structured Squad Dashboard view"`

---

### Task 4: Testing the Ecosystem

**Files:**
- Modify: `agent/tests/test_nodes.py`
- Modify: `src/bridge_tests.rs`

- [ ] **Step 1: Python Tests**
Write a unit test ensuring that `scout_node` calls the ArXiv tool correctly when invoked.

- [ ] **Step 2: Rust Tests**
Write a test to ensure the UI state correctly parses and updates individual agent statuses when multiple `status` events are received.

- [ ] **Step 3: Commit**
`git add agent/tests/test_nodes.py src/bridge_tests.rs && git commit -m "test: add arxiv and squad dashboard tests"`
