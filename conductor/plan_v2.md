# AetherMind UI & Agent Overhaul Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Overhaul the TUI to a three-column navigable layout and expand the V3/V8 squad interactivity.

---

### Task 1: Python - Expand Squad Interactivity

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Update `emit_event` to support more types**

```python
def emit_radio(agent: str, target: str, message: str):
    emit_event("radio", agent, f"-> {target}: {message}")

def emit_thought(agent: str, message: str):
    emit_event("thought", agent, message)
```

- [ ] **Step 2: Implement V3-Analyst node**
The Analyst should review findings from Librarian and Scout.

```python
def analyst_node(state: AgentState):
    emit_radio("V3-Analyst", "Squad", "Iniciando análise de dados coletados...")
    # Lógica de análise (dummy por enquanto, ou pequena chamada LLM)
    return {"current_status": "analyst_done"}
```

- [ ] **Step 3: Update Graph topology**
Add Analyst and edges.

- [ ] **Step 4: Commit**
`git commit -m "feat(python): add radio events and analyst node"`

---

### Task 2: Rust - Three-Column UI Refactor

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Update `AppUi::draw` to accept more buffers**

```rust
pub fn draw(&mut self, chat: &str, radio: &str, discovery: &str, input: &str, focus: usize) -> Result<()>
```

- [ ] **Step 2: Use `Layout` to create three columns**
- Col 1: Chat (30%)
- Col 2: Radio (40%)
- Col 3: Discovery (30%)

- [ ] **Step 3: Add visual feedback for focus**
Highlight the title of the focused column.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): refactor TUI to three-column layout"`

---

### Task 4: Rust - Navigation & Scrolling

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Implement Scroll State for each column**
Use `u16` offsets.

- [ ] **Step 2: Handle `Tab` and `Arrows`**
- `Tab`: increment `focus` counter.
- `Up/Down`: increment/decrement scroll offset of the focused pane.

- [ ] **Step 3: Implement Auto-scroll**
If scroll is at the very bottom, keep it at the bottom when new content arrives.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): implement navigation and scrolling in TUI"`
