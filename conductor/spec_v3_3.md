# Spec: AetherMind v3.3 - Reliability & Structured UX

**Status:** Draft
**Date:** 2026-04-07

## 1. Goal
Ensure the system is 100% reliable through rigorous testing and provide a professional, structured chat experience. Stop relying on fragile string concatenation for rendering and move to a structured message system.

## 2. Structured Chat History (Rust)
Instead of `String`, use a `ChatMessage` enum:
```rust
enum ChatMessage {
    User(String),
    Agent { name: String, content: String, is_streaming: bool },
    Thought { name: String, content: String },
    System(String),
}
```
- **Rendering:** The TUI will iterate over this list. Streaming messages will have their `content` updated dynamically.
- **Auto-scroll:** Logic will be tied to the number of messages and their height.

## 3. High-Fidelity Streaming
- **Token Handling:** Rust will look for the *active* streaming message from an agent and append tokens to it.
- **Indicator:** Show a "pulsing" or "typing" indicator when an agent is streaming.

## 4. Comprehensive Testing Strategy
### 4.1 Rust: Integration Tests
- `test_full_workflow`: Mock stdin/stdout to verify that a "Query" results in the expected sequence of events.
- `test_error_resilience`: Simulate a subprocess crash and verify the UI shows an error instead of hanging.

### 4.2 Python: Node Tests
- Verify that `orchestrator_node` correctly tags `RESEARCH` for complex queries.
- Verify that `writer_node` emits at least 10 `token` events for a standard prompt.

## 5. UI Improvements
- **Radio View:** Make it look like a real system log (timestamped).
- **Reader View:** Add a "Loading" state while the artifact is being streamed.
