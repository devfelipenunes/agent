# Spec: AetherMind - Elite Research & Chat System

**Status:** Draft / Design phase
**Date:** 2026-04-07
**Project:** AetherMind (formerly Ollag)

## 1. Vision & Goals
AetherMind is an elite personal assistant designed for fluid research and artifact generation (articles, spreadsheets, blog posts). It leverages a multi-agent "V3/V8 Squad" to perform background research while maintaining a responsive, high-performance chat interface.

### Key Objectives:
- **Fluidity:** Asynchronous research that doesn't block the user chat.
- **Autonomous Intelligence:** Agents that "understand" each other, resolve contradictions, and refine artifacts.
- **Hybrid Architecture:** Rust for UI/Performance + Python for Agent Logic/AI Ecosystem.
- **Deep Memory:** Semantic connection of Obsidian notes via Cognee/RAG.

## 2. Architecture: The Hybrid Engine

### 2.1 Front-end (Rust / Ratatui)
The "Armor": Responsible for the Terminal User Interface (TUI).
- **Chat Engine:** Handles user input and streams LLM responses.
- **Sidebar Manager:** Displays real-time agent status and background findings.
- **Communication Layer:** JSON-RPC bridge (Stdin/Stdout) to the Python background process.

### 2.2 Back-end (Python / LangGraph)
The "Brain": Responsible for multi-agent orchestration.
- **Orchestrator:** A LangGraph state machine managing the V3/V8 Squad.
- **Memory Mesh:** Cognee-powered RAG for Obsidian vault integration.
- **Tooling:** Tavily (Web Search), Obsidian (Local Files), GitHub API.

## 3. The V3/V8 Elite Squad

### Intelligence Team (Research Phase)
- **V3-Scout (Explorer):** Parallel mining of Web (Tavily) + Local (Obsidian).
- **V3-Librarian (Memory):** Concepts connector via Cognee. Identifies semantic links between new data and old notes.
- **V3-Analyst (Logic):** Identifies trade-offs, validates facts, and resolves data conflicts.

### Delivery Team (Artifact Phase)
- **V8-Architect (Structure):** Designs schemas for CSVs, Outlines for Blogs, and Mermaid diagrams.
- **V8-Writer (Author):** High-density technical writing with professional authority.
- **V8-Critic (Editor):** Red-teaming, fact-checking, and tone-of-voice alignment.

## 4. UI Design: The "A/B Hybrid" TUI
- **Main Chat (Left/Center - 70%):** Minimalist stream of conversation. Subtle color-coding per agent (Blue: Librarian, Green: Scout).
- **Intelligence Sidebar (Right - 30%):** 
    - **Live Agent Status:** (e.g., `🛰️ Scout searching Web...`).
    - **Context Feed:** List of links/notes found during research.
    - **Artifact Preview:** Small widget showing the progress of generated files.

## 5. Workflow Example: "Write a Blog on Interoperability"
1. **User:** "Write a blog on interoperability."
2. **Rust UI:** Instantly sends command to Python.
3. **Librarian (Python):** Responds in chat with initial local context from Obsidian.
4. **Scout & Analyst (Background):** Start deep web research. Sidebar updates with progress.
5. **Architect:** Proposes an outline in the sidebar.
6. **Writer & Critic:** Generate and refine the final Markdown file in a temporary buffer.
7. **User Approval:** User reviews in-chat, and Architect saves it to the Obsidian Vault.

## 6. Implementation Strategy
- **Phase 1:** Refactor Python `agent/` to support the full V3/V8 Squad using LangGraph.
- **Phase 2:** Implement the JSON-RPC event stream in Python.
- **Phase 3:** Build the Rust `ratatui` TUI with a split-pane layout (Chat + Sidebar).
- **Phase 4:** Connect the bridge and integrate Cognee for deep Obsidian indexing.
