# Spec: AetherMind v3.2 - Real-time Streaming & Stability

**Status:** Draft
**Date:** 2026-04-07

## 1. Communication Protocol Analysis
The current protocol uses JSON-RPC over stdout.
- **Messages:** Sent as complete strings after the LLM finishes generating.
- **Status/Radio/Findings:** Sent as complete strings upon event trigger.
- **Rendering:** Rust receives the JSON, appends it to a buffer, and Ratatui rerenders the whole buffer. This is "line-by-line" or "event-by-event", not "character-by-character".

## 2. Real-time Streaming (Character-by-Character)
To achieve "fluid" character streaming:
1. **Python side:** Must use `llm.stream()` instead of `llm.invoke()`.
2. **Event Type:** Introduce a new event type `token` which contains a small chunk of text.
3. **Rust side:** When a `token` event is received, append it to the *last* message in the buffer instead of creating a new line.

## 3. Stability: Fixing BrokenPipeError
The `BrokenPipeError` occurs because the Python subprocess tries to write to stdout after the Rust process has closed the pipe (e.g., during shutdown or crash).
- **Fix:** Python must rigorously handle `BrokenPipeError` and `EOFError`.
- **Fix:** Rust must explicitly signal shutdown to Python or terminate the process group cleanly.

## 4. Enhanced Logging & Observability
- **Python:** Log every transition in the LangGraph to the `radio` channel.
- **Rust:** Implement a `Debug` view (F5) or a persistent log file to track the raw JSON exchanges.

## 5. Technical Changes
- **agent/src/core/graph.py:** Refactor nodes to use `stream` and emit `token` events.
- **src/main.rs:** Update event loop to handle `token` and `artifact_chunk` events.
- **src/bridge.rs:** Improve child process management.
