# AetherMind v3.1 - Responsiveness & Background Monitoring

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the silence between user input and agent response, and add a persistent "Agent Monitor" bar to the UI.

---

### Task 1: Python - Acknowledgment & Error Handling

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Add immediate acknowledgment in `main_loop`**
Print a system event as soon as a line is received from stdin to confirm the bridge is alive.

- [ ] **Step 2: Add granular `status` updates in every node**
Ensure every node emits a `status` event at the very beginning of its execution.

- [ ] **Step 3: Wrap LLM calls in try/except**
Emit an `error` event if the Ollama connection fails or times out.

- [ ] **Step 4: Commit**
`git commit -m "fix(python): add immediate query acknowledgment and granular status updates"`

---

### Task 2: Rust - Persistent Background Monitor

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Update UI Layout**
Add a 1-line "Activity Bar" above the input field.

- [ ] **Step 2: Implement "Live Status" storage**
Create a variable in `main.rs` to store the *latest* status from any agent.

- [ ] **Step 3: Render the status bar**
Show `[Agent Name] -> [Current Activity]` in the new UI chunk.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): add persistent background activity monitor bar"`

---

### Task 3: Rust - Bridge Stabilization

**Files:**
- Modify: `src/bridge.rs`

- [ ] **Step 1: Ensure flushes**
Verify that `stdin.flush()` is called after every query.

- [ ] **Step 2: Add bridge logging**
Add a way to see if the subprocess crashed (e.g., checking `child.try_wait()`).
