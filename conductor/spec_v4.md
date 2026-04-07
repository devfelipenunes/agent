# Spec: AetherMind v4 - Autonomous RAG & ArXiv Integration

**Status:** Draft
**Date:** 2026-04-07

## 1. Goal
Evolve AetherMind into a truly autonomous research squad that not only answers questions but actively grows its own knowledge base (Obsidian) by scraping high-quality sources like ArXiv when local data is insufficient. Enhance the terminal UI to provide deep visibility into each squad's operations.

## 2. Advanced Research Workflow (The RAG Loop)
The "Memory Loop" is the core of v4. The system should learn over time.

### 2.1 The Flow
1. **Orchestrator:** Receives a complex query (e.g., "CBDC Implementation Case Studies"). Routes to `RESEARCH`.
2. **Librarian (Obsidian Check):** Searches the local Obsidian vault.
3. **Scout (Data Gathering):**
    - If Librarian found rich data: Scout just does a light web check to confirm recency.
    - If Librarian found poor/no data: Scout performs a deep dive using **Tavily (Web)** AND **ArXiv (Academic Papers)**.
4. **Analyst (Synthesis):** Consolidates local and external findings.
5. **Librarian (Knowledge Ingestion):** If new external data was heavily relied upon, the Librarian creates a *new Markdown note* in the Obsidian vault summarizing the external findings (e.g., `AetherMind/ArXiv_CBDC_Cases.md`).
6. **Writer & Critic:** Generate the final artifact for the user.

## 3. Tool Integrations
- **ArXiv API:** A custom Python tool using `urllib` and `xml.etree.ElementTree` to query `http://export.arxiv.org/api/query`. It parses Atom feeds to extract titles, authors, and summaries.
- **Obsidian MCP:** Extended to allow *writing* new notes, not just reading.

## 4. UI/UX Overhaul (Rust)
The terminal interface must clearly show what the squads are doing and allow the user to interact or intervene.

### 4.1 Views Redesign
- **F1: CHAT:** The main communication channel.
- **F2: READER:** Viewer for the final generated artifacts.
- **F3: SQUADS (New Dashboard):**
    - A multi-panel view showing the real-time status of each agent (Librarian, Scout, Analyst, Writer, Critic).
    - Shows exactly what tool they are calling (e.g., `Scout: Searching ArXiv for 'digital identity'`).
- **F4: LOGS:** Raw system and technical logs (formerly Radio).

## 5. Testing & Validation
- **Python Node Tests:** Validate the RAG loop logic. Ensure Scout calls ArXiv and Librarian triggers the "Save Note" action when external data is prioritized.
- **Rust Integration Tests:** Verify that the UI correctly receives and routes `artifact_update`, `token`, and `status` events to the right buffers.
