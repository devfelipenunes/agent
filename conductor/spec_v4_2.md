# Spec: AetherMind v4.2 - Multi-Mode Startup

**Status:** Draft
**Date:** 2026-04-07

## 1. Goal
Provide the user with a choice at launch: **Local Mode** (Privacy/Offline) or **Token Mode** (Performance/OpenAI).

## 2. Modes Definition
- **Local Mode (L):**
    - Chat Agent: Gemma 4 (26b) via Ollama.
    - Background Agents: Gemma 4 (26b) via Ollama.
    - Pros: 100% Private, no API costs.
- **Token Mode (T):**
    - Chat Agent: GPT-4o via OpenAI.
    - Background Agents: Gemma 4 (26b) via Ollama.
    - Pros: Faster chat responses, higher reasoning for PM.

## 3. UI Flow (Startup Selection)
Before entering the main workspace (Tabs), AetherMind will display a full-screen selection menu.
- **Options:**
    1. `[L] Local Mode (Ollama Only)`
    2. `[T] Token Mode (OpenAI + Ollama)`
- **Keybindings:** Use `L` or `T` keys to select and enter.

## 4. Bridge Protocol Update
The first message from Rust to Python will now be a configuration object:
```json
{
  "type": "config",
  "mode": "local" | "token"
}
```

## 5. Backend Flexibility (Python)
The `llm_chat` instance will be initialized dynamically based on the received config message. The Python process will wait for this message before processing any queries.
