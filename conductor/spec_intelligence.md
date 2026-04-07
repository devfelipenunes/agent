# Spec: AetherMind Intelligence & Orchestration Overhaul

**Status:** Draft
**Date:** 2026-04-07

## 1. Goal
Transform the agent intelligence from a hardcoded linear graph into a dynamic, responsive orchestration system. The "Primary Agent" will act as a project manager, capable of having a real chat with the user and spawning specialized research squads in the background.

## 2. Dynamic Squad Architecture (Python)

### 2.1 The Orchestrator (Aether-PM)
- **Role:** The user's primary interface.
- **Capabilities:**
    - **Chat:** Immediate responses to greetings and simple questions.
    - **Decision Making:** Analyzes user intent to decide if a "Research Squad" is needed.
    - **Delegation:** If a complex task is identified (e.g., "write an article about X"), it spawns a background worker thread/process or a specialized sub-graph.

### 2.2 Specialized Background Squads
- **Research Squad:** (Scout + Librarian + Analyst) -> Focuses on deep data mining.
- **Content Squad:** (Architect + Writer + Critic) -> Focuses on artifact production.
- **Web Squad (New):** Focused specifically on finding and scraping high-quality article sites (Medium, Substack, ArXiv, etc.).

## 3. Communication Overhaul
- **Async Execution:** The Orchestrator stays active in the chat while the Squads work.
- **Feedback Loop:** Squads send "Status Briefings" to the PM, who summarizes them for the user in the main chat.

## 4. Technical Implementation (Python/LangGraph)
- Use a **Router** approach in LangGraph.
- The `initial_node` will be the PM. 
- It can either end the interaction (simple chat) or branch into a background execution path.
- **State Management:** Add a `user_intent` flag and `active_squads` list to the `AgentState`.

## 5. UI Synergy (Rust)
- No major UI changes, but the Rust side must handle the "Streaming" of the PM's thoughts vs the Squad's radio logs more distinctly.
