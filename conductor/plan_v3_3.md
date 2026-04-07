# AetherMind v3.3 Implementation Plan - Reliability & Structured UX

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Overhaul chat rendering using a structured message system and implement comprehensive tests for both Rust and Python components.

---

### Task 1: Robust Test Suite (The Foundation)

**Files:**
- Modify: `src/bridge_tests.rs`
- Create: `agent/tests/test_nodes.py`

- [ ] **Step 1: Expand Rust Integration Tests**
Test the bridge's ability to handle interleaved events (tokens from one agent, thoughts from another).

- [ ] **Step 2: Create Python Unit Tests**
Test `orchestrator_node` routing and `emit_token` behavior using mocks for LLM and Tavily.

- [ ] **Step 3: Commit**
`git commit -m "test: add comprehensive bridge and node tests"`

---

### Task 2: Rust - Structured Message History

**Files:**
- Modify: `src/main.rs`
- Modify: `src/ui.rs`

- [ ] **Step 1: Define `ChatMessage` Enum**
Implement the structured message types in `main.rs`.

- [ ] **Step 2: Refactor State Management**
Replace `chat_buffer: String` with `chat_history: Vec<ChatMessage>`.

- [ ] **Step 3: Update Event Processing**
- `message_start` -> Push new `Agent` message to history.
- `token` -> Find the last `Agent` message and append.
- `thought` -> Push to `radio` buffer AND optionally to `chat_history` as a subtle entry.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): implement structured chat history and refined event processing"`

---

### Task 3: UI - High-Fidelity Rendering

**Files:**
- Modify: `src/ui.rs`

- [ ] **Step 1: Use `Text` and `Span` for Chat**
Iterate over `chat_history` and build a rich `Text` object with different colors for each message type.

- [ ] **Step 2: Implement Streaming Indicator**
Add a small "..." or "typing" icon next to the agent's name if `is_streaming` is true.

- [ ] **Step 3: Commit**
`git commit -m "feat(ui): improve chat rendering with rich text and streaming indicators"`

---

### Task 4: Python - Aether-PM Personality & Speed

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Improve PM Prompt**
Make the PM more proactive and explain *why* it's calling specific agents.

- [ ] **Step 2: Low-Latency Acknowledgments**
Ensure the very first token from the PM is emitted within <500ms of receiving the query.

- [ ] **Step 3: Commit**
`git commit -m "feat(python): enhance PM personality and reduce response latency"`
