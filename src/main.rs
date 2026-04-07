mod ollama;
mod agent;
mod tools;
mod skills;
mod memory;
mod ui;

use clap::Parser;
use anyhow::Result;
use ollama::OllamaClient;
use agent::Agent;
use tools::Toolbox;
use tools::fs::{ReadFileTool, WriteFileTool, ListDirTool};
use tools::shell::ShellTool;
use tools::obsidian::KnowledgeSearchTool;
use skills::load_skills_from_dir;
use memory::KnowledgeBase;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Parser)]
#[command(name = "ollag")]
#[command(about = "Ollama Agent in Rust", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "qwen2.5-coder:7b")]
    model: String,

    #[arg(short, long)]
    index: bool,

    #[arg(help = "The task for the agent to perform")]
    task: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = OllamaClient::new(cli.model.clone());
    let kb_path = "knowledge_base.bin";

    if cli.index {
        let mut kb = KnowledgeBase::new();
        kb.index_directory("/l/disk0/fnunes/obsidian/", &client).await?;
        kb.save(kb_path)?;
        println!("🚀 Knowledge base indexed and saved to {}", kb_path);
        return Ok(());
    }

    let task = cli.task.ok_or_else(|| anyhow::anyhow!("Please provide a task or use --index"))?;

    println!("🚀 Ollag starting with model: {}", cli.model);

    // Load Knowledge Base
    let kb = if std::path::Path::new(kb_path).exists() {
        println!("  - Loading knowledge base...");
        KnowledgeBase::load(kb_path)?
    } else {
        println!("  - No knowledge base found. Use --index to create one.");
        KnowledgeBase::new()
    };
    let kb = Arc::new(Mutex::new(kb));

    let mut toolbox = Toolbox::new();

    // Register built-in tools
    toolbox.register(Box::new(ReadFileTool));
    toolbox.register(Box::new(WriteFileTool));
    toolbox.register(Box::new(ListDirTool));
    toolbox.register(Box::new(ShellTool));
    toolbox.register(Box::new(KnowledgeSearchTool {
        kb: kb.clone(),
        client: client.clone(),
    }));

    // Load dynamic skills
    let dynamic_skills = load_skills_from_dir("skills")?;
    for skill in dynamic_skills {
        println!("  - Skill loaded: {}", skill.name());
        toolbox.register(skill);
    }

    let agent = Agent::new(client, toolbox);

    if let Err(e) = agent.run(&task).await {
        eprintln!("❌ Error: {}", e);
    }

    Ok(())
}
