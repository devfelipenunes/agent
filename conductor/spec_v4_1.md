# Spec: AetherMind v4.1 - Persistence & Performance

**Status:** Draft
**Date:** 2026-04-07

## 1. Goal
Fix the chat history disappearing issue, implement professional loading indicators, and optimize LLM response latency to make the system feel "snappy" and reliable.

## 2. Chat Persistence (Rust)
- **Problem:** Currently, the `chat_buffer` might be getting overwritten or improperly managed during rapid events.
- **Solution:** Use a dedicated `Vec<String>` to store historical messages and ensure the `chat_buffer` is an accumulation of all messages plus the current streaming content.
- **Scroll Logic:** Improve auto-scroll to always point to the last line of the accumulated history.

## 3. Advanced Loading Indicators
- **Visuals:** Add a dedicated "Spinner" or "Progress Bar" in the Background Monitor area.
- **States:**
    - `THINKING`: Spinner animation while waiting for the first token.
    - `STREAMING`: Pulse effect while tokens are arriving.
    - `RESEARCHING`: Progress percentage based on squad task completion.

## 4. Latency Optimization (Python)
- **Streaming by Default:** Ensure *every* node that uses an LLM uses `llm.stream()` to reduce "Time to First Token" (TTFT).
- **Parallel Research:** Allow `V3-Scout` to start web/ArXiv search *while* the PM is still finishing its introductory message.
- **Model Parameters:** Tune `temperature` and `max_tokens` for faster generation in the Orchestrator.

## 5. Technical Changes
- **Rust:** Implement a `MessageHistory` struct to manage the lifecycle of user/agent messages.
- **Python:** Implement an `AsyncGraph` to allow parallel node execution where possible.
