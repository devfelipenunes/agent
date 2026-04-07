# AetherMind v3 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform AetherMind into a fluid, tab-based terminal workspace with professional article generation capabilities.

---

### Task 1: Python - Brain Upgrade (Writer & Critic)

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Add Writer node with real LLM call**
Use `llm.invoke` to generate a Markdown article based on findings.

- [ ] **Step 2: Add Critic node for quality control**
Review the writer's output. If it's too short or misses key facts, go back to Writer.

- [ ] **Step 3: Update Event System**
Add `artifact_full` event to send the entire generated text.

- [ ] **Step 4: Commit**
`git commit -m "feat(python): implement V8-Writer and V8-Critic with real LLM synthesis"`

---

### Task 2: Rust - UI Refactor (Tabs & Views)

**Files:**
- Modify: `src/ui.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Update `AppUi` to support Tabs**
Implement a top bar showing "1: Chat | 2: Reader | 3: Radio | 4: Squad".

- [ ] **Step 2: Implement View Switching logic**
Map keys `1`-`4` to change the active tab.

- [ ] **Step 3: Create specialized rendering for each tab**
- Chat: Centered, clean.
- Reader: Wide layout with Markdown-like styling (bold headers).
- Radio: Dense logs.
- Squad: Table of agent statuses.

- [ ] **Step 4: Commit**
`git commit -m "feat(rust): implement tabbed UI and view switching"`

---

### Task 3: Final Polish & Navigation

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Improve Input Handling**
Add support for clearing input and better cursor movement if possible.

- [ ] **Step 2: Implement Scroll for all views**
Ensure each tab remembers its scroll position.

- [ ] **Step 3: Commit**
`git commit -m "feat(rust): improve navigation and scroll persistence"`
