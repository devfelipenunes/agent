mod ollama;
mod agent;
mod tools;

use clap::Parser;
use anyhow::Result;
use ollama::OllamaClient;
use agent::Agent;
use tools::Toolbox;
use tools::fs::{ReadFileTool, WriteFileTool, ListDirTool};
use tools::shell::ShellTool;

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

    // Register tools
    toolbox.register(Box::new(ReadFileTool));
    toolbox.register(Box::new(WriteFileTool));
    toolbox.register(Box::new(ListDirTool));
    toolbox.register(Box::new(ShellTool));

    let agent = Agent::new(client, toolbox);

    if let Err(e) = agent.run(&cli.task).await {
        eprintln!("❌ Error: {}", e);
    }

    Ok(())
}
