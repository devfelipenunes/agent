# AetherMind v4.1 Implementation Plan - Persistence & Performance

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix chat persistence, improve loading visuals, and optimize LLM latency.

---

### Task 1: Rust - Chat History Persistence & Auto-scroll

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Refactor `chat_buffer` management**
Instead of blindly appending to a single string, maintain a `chat_log: Vec<String>`.
When a `message_start` or `token` arrives, update the *last* entry if it's from the same agent, otherwise push a new one.

- [ ] **Step 2: Stabilize Auto-scroll**
Update `scrolls[0]` calculation to properly account for the height of the formatted text block.

- [ ] **Step 3: Commit**
`git commit -m "fix(rust): implement persistent chat log and stable auto-scroll"`

---

### Task 2: Rust - Professional Loading Spinner

**Files:**
- Modify: `src/main.rs`
- Modify: `src/ui.rs`

- [ ] **Step 1: Add `spinner_index` state**
In `main.rs`, add a counter that increments every loop iteration.

- [ ] **Step 2: Implement Spinner Frame logic**
```rust
let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
let current_frame = frames[spinner_index % frames.len()];
```

- [ ] **Step 3: Update `AppUi::draw`**
Render the `current_frame` next to the `background_status` bar when an agent is active.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): add animated loading spinner to background monitor"`

---

### Task 3: Python - Latency & Parallelism

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Low-Latency Orchestrator**
Modify `orchestrator_node` to start streaming its *decision* (e.g., "Entendido, vou chamar o time...") immediately while it calculates the intent.

- [ ] **Step 2: Async Graph Execution**
Refactor nodes to use `async def` and ensure the LangGraph is run using `ainvoke()`.

- [ ] **Step 3: Parallel Scout/Librarian (Optional but recommended)**
Adjust the graph edges so `librarian` and `scout` run in parallel.

- [ ] **Step 4: Commit**
`git commit -m "perf(python): switch to async graph and optimize TTFT"`
