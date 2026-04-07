use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::ollama::OllamaClient;

#[derive(Serialize, Deserialize, Clone)]
pub struct Document {
    pub path: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub documents: Vec<Document>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self { documents: Vec::new() }
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let encoded: Vec<u8> = bincode::serialize(self)?;
        fs::write(path, encoded)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self> {
        let data = fs::read(path)?;
        let decoded: Self = bincode::deserialize(&data)?;
        Ok(decoded)
    }

    pub async fn index_directory(&mut self, dir: &str, client: &OllamaClient) -> Result<()> {
        let path = Path::new(dir);
        if !path.exists() {
            return Err(anyhow::anyhow!("Directory not found: {}", dir));
        }

        self.documents.clear();
        println!("Indexing Obsidian vault at {}...", dir);

        for entry in walkdir::WalkDir::new(path) {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "md") {
                let content = fs::read_to_string(entry.path())?;
                if content.trim().is_empty() { continue; }

                println!("  - Embedding: {}", entry.path().display());
                let embedding = client.embed(content.clone()).await?;
                
                self.documents.push(Document {
                    path: entry.path().to_string_lossy().to_string(),
                    content,
                    embedding,
                });
            }
        }
        println!("Indexing complete. {} documents indexed.", self.documents.len());
        Ok(())
    }

    pub fn search(&self, query_vector: Vec<f32>, top_k: usize) -> Vec<Document> {
        let mut results: Vec<(f32, Document)> = self.documents.iter()
            .map(|doc| {
                let sim = cosine_similarity(&query_vector, &doc.embedding);
                (sim, doc.clone())
            })
            .collect();

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        results.into_iter().take(top_k).map(|(_, doc)| doc).collect()
    }
}

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2).map(|(a, b)| a * b).sum();
    let norm1: f32 = v1.iter().map(|a| a * a).sum::<f32>().sqrt();
    let norm2: f32 = v2.iter().map(|a| a * a).sum::<f32>().sqrt();
    dot_product / (norm1 * norm2)
}
