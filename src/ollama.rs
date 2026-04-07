use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use anyhow::Result;

#[derive(Clone)]
pub struct OllamaClient {
    client: Ollama,
    model: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        Self {
            client: Ollama::default(),
            model,
        }
    }

    pub async fn generate(&self, prompt: String) -> Result<String> {
        let request = GenerationRequest::new(self.model.clone(), prompt);
        let res = self.client.generate(request).await?;
        Ok(res.response)
    }

    pub async fn embed(&self, text: String) -> Result<Vec<f32>> {
        // Na v0.1.9, generate_embeddings retorna Vec<f64>
        let res = self.client.generate_embeddings("nomic-embed-text".to_string(), text, None).await?;
        let embeddings_f32 = res.embeddings.into_iter().map(|v| v as f32).collect();
        Ok(embeddings_f32)
    }
}
