use crate::tools::Tool;
use crate::memory::KnowledgeBase;
use crate::ollama::OllamaClient;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct KnowledgeSearchTool {
    pub kb: Arc<Mutex<KnowledgeBase>>,
    pub client: OllamaClient,
}

#[async_trait]
impl Tool for KnowledgeSearchTool {
    fn name(&self) -> &str { "search_knowledge" }
    fn description(&self) -> &str { "Search your personal Obsidian vault for information. Args: { \"query\": string }" }
    async fn run(&self, args: Value) -> Result<String> {
        let query = args["query"].as_str().ok_or_else(|| anyhow::anyhow!("Missing query"))?;
        let query_vector = self.client.embed(query.to_string()).await?;
        
        let kb = self.kb.lock().await;
        let results = kb.search(query_vector, 3);
        
        if results.is_empty() {
            return Ok("No relevant information found in the knowledge base.".to_string());
        }

        let mut output = String::from("Found relevant information in your notes:\n");
        for doc in results {
            output.push_str(&format!("\n--- From: {} ---\n{}\n", doc.path, doc.content));
        }
        Ok(output)
    }
}
