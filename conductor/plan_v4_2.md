# AetherMind v4.2 Implementation Plan - Multi-Mode Startup

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a startup mode selection (Local vs Token) in Rust and dynamic LLM initialization in Python.

---

### Task 1: Rust - Startup Selection Menu

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Add `draw_startup` to `AppUi`**
Create a centered menu with the two mode options.

- [ ] **Step 2: Implement Startup State in `main.rs`**
Add an `AppState` enum: `enum AppState { Startup, Running }`.
Initial state is `Startup`.

- [ ] **Step 3: Handle mode selection keys**
Listen for `L` and `T` in the `Startup` state. When pressed, switch to `Running` and send the config to Python.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): add mode selection menu at startup"`

---

### Task 2: Python - Dynamic LLM Initialization

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Refactor LLM initialization**
Move `llm_chat` initialization into a function or delayed logic that waits for the `config` message.

- [ ] **Step 2: Update `main_loop` to handle `config`**
Wait for the first message from stdin. If it's a `config` type, set the global `llm_chat` variable.

- [ ] **Step 3: Commit**
`git commit -m "feat(python): implement dynamic LLM initialization based on mode"`

---

### Task 3: Bridge & Integration

**Files:**
- Modify: `src/main.rs`
- Modify: `src/bridge.rs`

- [ ] **Step 1: Implement `send_config` in `PythonBridge`**
Method to send the JSON config message.

- [ ] **Step 2: Wire it up in `main.rs`**
Call `bridge.send_config(mode)` as soon as the user makes their choice.

- [ ] **Step 3: Commit**
`git commit -m "feat: integrate mode signaling between Rust and Python"`
