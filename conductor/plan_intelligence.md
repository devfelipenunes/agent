# AetherMind Intelligence Overhaul Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a dynamic Orchestrator (PM) that handles responsive chat and background squad delegation.

---

### Task 1: Python - The Orchestrator (PM) & Router

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Implement `orchestrator_node`**
This node acts as the entry point. It uses the LLM to decide:
1. Is this a simple greeting/chat? -> Respond directly and end.
2. Does this require research? -> Spawn Research Squad.
3. Does this require artifact generation? -> Spawn Content Squad.

```python
def orchestrator_node(state: AgentState):
    query = state["query"]
    prompt = f"Você é o Aether-PM. Analise a entrada do usuário: '{query}'.\nSe for um oi/cumprimento, responda de forma curta e amigável.\nSe for um pedido de pesquisa ou escrita, diga que vai montar o time e responda com 'SQUAD_REQUIRED: [Tipo de Squad]'."
    response = llm.invoke(prompt).content
    
    if "SQUAD_REQUIRED" in response:
        squad_type = response.split(":")[1].strip()
        emit_event("message", "Aether-PM", f"Entendido. Montando a equipe de {squad_type} para você...")
        return {"current_status": "squad_needed", "user_intent": squad_type}
    else:
        emit_event("message", "Aether-PM", response)
        return {"current_status": "chat_only"}
```

- [ ] **Step 2: Update Graph Router**
Add a conditional edge from `orchestrator` to either `END` or `librarian`.

- [ ] **Step 3: Commit**
`git commit -m "feat(python): implement primary orchestrator and task router"`

---

### Task 2: Python - The Web Squad (Advanced Scraper)

**Files:**
- Modify: `agent/src/core/graph.py`

- [ ] **Step 1: Add `web_squad_node`**
Focus specifically on identifying top article sites (ArXiv, Substack, etc.) using Tavily with specific domain filters.

```python
def web_squad_node(state: AgentState):
    emit_radio("Web-Squad", "Analyst", "Buscando papers e artigos técnicos em fontes de elite...")
    # Tavily call with include_domains=["arxiv.org", "medium.com", "substack.com"]
    return {"current_status": "web_mining_done"}
```

- [ ] **Step 2: Integrate into graph**
Place it parallel to or after the Scout node.

---

### Task 3: Rust - Bridge & UI Visibility

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Distinct message handling**
Ensure messages from "Aether-PM" are styled prominently in the chat.

- [ ] **Step 2: Auto-scroll stabilization**
Improve the auto-scroll logic so it doesn't jump if the user is manually scrolling.
