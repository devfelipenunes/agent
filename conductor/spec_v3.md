# Spec: AetherMind v3 - Fluid UX & Specialized Agents

**Status:** Draft
**Date:** 2026-04-07

## 1. Vision
AetherMind v3 transforms from a cramped multi-column terminal into a fluid, tab-based workspace. It introduces specialized agents for high-quality artifact generation and a "Reader Mode" to view results without leaving the terminal.

## 2. Advanced Multi-Agent Workflow (Python)
The squad is expanded to ensure high-quality output:

- **V3-Librarian & V3-Scout:** Data gatherers (unchanged).
- **V3-Analyst:** Now creates a detailed "Brief" for the writer.
- **V8-Writer (New):** Uses Gemma 4 to write a multi-section Markdown article based on the Analyst's brief.
- **V8-Critic (New):** Acts as an editor. Checks for tone, accuracy, and formatting. Can request a "Rewrite" from the Writer.

### 2.1 Event Protocol Enhancements
- `artifact_full`: Sends the entire Markdown content of the generated article.
- `progress`: Percentage indicator for long-running LLM tasks.

## 3. Terminal UX Overhaul (Rust)
The UI moves from fixed columns to a dynamic Tab-based system.

### 3.1 Views
1. **💬 CHAT (Alt+1):** High-level user interaction. Clean, wide layout.
2. **📖 READER (Alt+2):** A full-screen Markdown viewer for the generated article.
3. **📡 RADIO (Alt+3):** Technical logs and agent-to-agent dialogue.
4. **🧭 SQUAD (Alt+4):** A dashboard showing agent health, current task, and discovered "Intelligence Nuggets".

### 3.2 Navigation
- **Tabs:** Keyboard shortcuts `1`, `2`, `3`, `4` to switch views.
- **Improved Input:** Better handling of multiline input and command history.
- **Auto-Focus:** When an artifact is completed, the UI can optionally suggest switching to the Reader view.

## 4. Technical Changes
- **Rust:** Update `src/ui.rs` to use `Tabs` and separate `State` for each view.
- **Python:** Refactor `agent/src/core/graph.py` to include real LLM calls for `Writer` and `Critic`.
