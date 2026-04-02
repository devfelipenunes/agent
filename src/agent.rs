use crate::ollama::OllamaClient;
use crate::tools::Toolbox;
use anyhow::Result;
use serde_json::json;

pub struct Agent {
    client: OllamaClient,
    toolbox: Toolbox,
}

impl Agent {
    pub fn new(client: OllamaClient, toolbox: Toolbox) -> Self {
        Self { client, toolbox }
    }

    pub async fn run(&self, task: &str) -> Result<()> {
        let tools_desc = self.toolbox.list_tools()
            .iter()
            .map(|(n, d)| format!("- {}: {}", n, d))
            .collect::<Vec<_>>()
            .join("\n");

        let system_prompt = format!(
            "You are a helpful AI assistant with access to local tools. 
            You must follow a ReAct loop: Thought, then Action, then Observation.
            
            Available Tools:
            {}
            
            Format your response exactly like this:
            Thought: Your reasoning here.
            Action: ```json
            {{
                \"tool\": \"tool_name\",
                \"args\": {{ \"arg1\": \"value1\" }}
            }}
            ```
            
            After the system provides an Observation, you continue the loop.
            When you have the final answer, start with 'Final Answer: '.",
            tools_desc
        );

        let mut history = format!("System: {}\nUser: {}\n", system_prompt, task);

        loop {
            println!("--- Thinking ---");
            let response = self.client.generate(history.clone()).await?;
            println!("{}", response);

            if response.contains("Final Answer:") {
                break;
            }

            if let Some(action_json) = self.extract_action(&response) {
                let tool_name = action_json["tool"].as_str().unwrap_or("");
                let args = action_json["args"].clone();

                println!("--- Executing Tool: {} ---", tool_name);
                let observation = match self.toolbox.call(tool_name, args).await {
                    Ok(out) => out,
                    Err(e) => format!("Error: {}", e),
                };

                println!("--- Observation ---\n{}", observation);
                history.push_str(&format!("\nAssistant: {}\nObservation: {}\n", response, observation));
            } else {
                history.push_str(&format!("\nAssistant: {}\n", response));
                // Se não houver ação nem resposta final, o loop pode travar.
                // Idealmente, pediríamos ao modelo para seguir o formato.
                if !response.contains("Thought:") {
                    break; 
                }
            }
        }

        Ok(())
    }

    fn extract_action(&self, text: &str) -> Option<serde_json::Value> {
        let start = text.find("```json")?;
        let rest = &text[start + 7..];
        let end = rest.find("```")?;
        let json_str = &rest[..end].trim();
        serde_json::from_str(json_str).ok()
    }
}
