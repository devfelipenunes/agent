# AetherMind v3.2 Implementation Plan - Streaming & Stability

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement real-time character-by-character streaming and resolve terminal crashes (BrokenPipeError).

---

### Task 1: Python - Token Streaming & Graceful Shutdown

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Implement `emit_token`**
Send partial text chunks to Rust.

```python
def emit_token(agent: str, token: str):
    emit_event("token", agent, token)
```

- [ ] **Step 2: Refactor `orchestrator_node` to use `llm.stream()`**
Stream the response character-by-character (or chunk-by-chunk).

```python
def orchestrator_node(state: AgentState):
    # ... prompt logic ...
    full_response = ""
    for chunk in llm.stream(prompt):
        content = chunk.content
        full_response += content
        emit_token("Aether-PM", content)
    
    # After streaming, handle routing logic based on full_response
    # ...
```

- [ ] **Step 3: Handle Shutdown Signals**
Ensure the loop breaks correctly on EOF or signal.

- [ ] **Step 4: Commit**
`git commit -m "feat(python): implement token streaming and improved error handling"`

---

### Task 2: Rust - Fluid UI Streaming

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Update event loop for `token` events**
Instead of `push_str` with formatting, `token` events should just append to the end of the `chat_buffer`.

- [ ] **Step 2: Track "Last Agent"**
Keep track of who sent the last message to avoid re-printing the agent name on every token.

- [ ] **Step 3: Commit**
`git commit -m "feat(rust): support character-level streaming in TUI"`

---

### Task 3: Bridge - Process Cleanup

**Files:**
- Modify: `src/bridge.rs`

- [ ] **Step 1: Implement Drop for `PythonBridge`**
Kill the child process when the bridge is dropped to prevent zombie processes and broken pipes.

```rust
impl Drop for PythonBridge {
    fn drop(&mut self) {
        // ... kill logic ...
    }
}
```

- [ ] **Step 2: Commit**
`git commit -m "fix(rust): ensure clean termination of python subprocess"`
