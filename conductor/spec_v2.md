# Spec: AetherMind UI & Agent Overhaul

**Status:** Draft
**Date:** 2026-04-07

## 1. Goal
Improve the terminal user experience by providing better visibility into agent coordination and adding navigation capabilities. Make the "V3/V8 Squad" feel alive and collaborative.

## 2. UI Layout (Three-Column System)
The TUI will be split into three distinct vertical zones:

### 2.1 Main Chat (Left - 30%)
- Displays the high-level conversation between the user and the primary agent.
- Clean and concise.
- **Color:** Yellow for User, Blue for System responses.

### 2.2 Squad Radio (Middle - 40%)
- Displays the "Internal Monologue" and dialogue between agents.
- Format: `[Agent Name] -> [Target Agent]: Message`
- This is where the complexity of the research is shown without cluttering the main chat.
- **Colors:** Each agent has a dedicated color (Scout: Green, Librarian: Cyan, Analyst: Purple, Architect: White).

### 2.3 Discovery Board (Right - 30%)
- **Top:** Live Status Table (Agent name | Activity | Pulse).
- **Middle:** Findings List (Recent links, note snippets).
- **Bottom:** Artifact Status (Progress bars for generated articles/spreadsheets).

## 3. Navigation & Interaction
- **Input:** Bottom bar remains for text entry.
- **Focus Mode:** Use `Tab` to cycle between Chat, Radio, and Discovery Board.
- **Scrolling:** Use `Up/Down` or `PageUp/PageDown` to scroll the focused pane.
- **Commands:** 
    - `/clear`: Clear logs.
    - `/save`: Manual save of current artifact.

## 4. Multi-Agent Interactivity (Python)
- **New Events:**
    - `monologue`: Internal thought of an agent.
    - `dialogue`: Agent A talking to Agent B.
- **Updated Nodes:**
    - **V3-Analyst:** Now explicitly validates findings from Scout and Librarian.
    - **V8-Critic:** Reviews the draft from Architect/Writer and sends it back if quality is low.
- **Logic:** Agents will "post" to the radio when they start a task or reach a conclusion.

## 5. Technical Implementation
- **Rust:** Use `ratatui` with multiple `Paragraph` widgets and manual scroll state management.
- **Python:** Use `LangGraph` with specialized "communication" wrappers for nodes to emit events automatically.
