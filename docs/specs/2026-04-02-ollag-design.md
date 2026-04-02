# Ollag: Ollama Agent in Rust - Design Spec

**Date:** 2026-04-02
**Author:** Gemini CLI
**Status:** Draft (Awaiting Review)

## 🎯 Goal
Implement an autonomous CLI agent in Rust that interacts with a local Ollama instance (specifically `qwen2.5-coder:7b`) to perform technical tasks using a ReAct (Reasoning and Acting) loop.

## 🏗️ Architecture
The system will follow the ReAct pattern where the agent follows a loop:
1.  **Reasoning (Thought):** The agent analyzes the task and decides which tool to use.
2.  **Action (Act):** The agent generates a structured JSON call for a tool.
3.  **Observation (Observe):** The system executes the tool and returns the output to the agent.
4.  **Completion:** The agent repeats the loop or provides the final answer.

### Key Components
- **Orchestrator:** Manages the ReAct loop and state.
- **Provider (Ollama):** Handles communication with the local Ollama API via `ollama-rs`.
- **Toolbox:** A registry of Rust functions that the agent can invoke.
- **Context Manager:** Maintains history and applies sliding window or compression to stay within the 32k/128k context window of Qwen.

## 🛠️ Tools (MVP)
The agent will have access to the following tools:
- `read_file(path)`: Returns the content of a file.
- `write_file(path, content)`: Writes or overwrites a file.
- `list_dir(path)`: Lists files and subdirectories.
- `run_command(cmd)`: Executes a shell command (requires manual confirmation for security).

## 🚀 Tech Stack
- **Linguagem:** Rust (Edition 2021).
- **Runtime:** `tokio` (Async/Await).
- **Ollama API:** `ollama-rs`.
- **CLI Framework:** `clap`.
- **Serialization:** `serde` & `serde_json`.
- **Error Handling:** `anyhow` or `thiserror`.

## 🔒 Security
- **Command Confirmation:** All `run_command` and `write_file` actions will prompt the user for confirmation (Y/n) before execution.
- **Local Isolation:** The agent only interacts with local resources (Ollama and File System).

## 📝 Success Criteria
- [ ] Successfully initializes a session with `qwen2.5-coder:7b`.
- [ ] Correctly executes a `read_file` -> `thought` -> `write_file` loop to refactor a piece of code.
- [ ] Gracefully handles tool errors (e.g., file not found).
- [ ] Provides a clean, streaming response to the user.
