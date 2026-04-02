mod ollama;
mod agent;
mod tools;
mod skills;

use clap::Parser;
use anyhow::Result;
use ollama::OllamaClient;
use agent::Agent;
use tools::Toolbox;
use tools::fs::{ReadFileTool, WriteFileTool, ListDirTool};
use tools::shell::ShellTool;
use skills::load_skills_from_dir;

#[derive(Parser)]
#[command(name = "ollag")]
#[command(about = "Ollama Agent in Rust", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "qwen2.5-coder:7b")]
    model: String,

    #[arg(help = "The task for the agent to perform")]
    task: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("🚀 Ollag starting with model: {}", cli.model);

    let client = OllamaClient::new(cli.model);
    let mut toolbox = Toolbox::new();

    // Register built-in tools
    toolbox.register(Box::new(ReadFileTool));
    toolbox.register(Box::new(WriteFileTool));
    toolbox.register(Box::new(ListDirTool));
    toolbox.register(Box::new(ShellTool));

    // Load dynamic skills
    let dynamic_skills = load_skills_from_dir("skills")?;
    for skill in dynamic_skills {
        println!("  - Skill loaded: {}", skill.name());
        toolbox.register(skill);
    }

    let agent = Agent::new(client, toolbox);

    if let Err(e) = agent.run(&cli.task).await {
        eprintln!("❌ Error: {}", e);
    }

    Ok(())
}
