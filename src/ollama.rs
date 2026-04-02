use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use anyhow::Result;

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
}
