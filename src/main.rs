mod cli;
mod executor;
mod ollama;
mod mcp_config;
mod mcp_client;
mod mcp_manager;
mod builtin_tools;

use anyhow::{Context, Result};
use colored::*;
use executor::AIExecutor;
use cli::ChatCLI;
use mcp_manager::McpManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Configuration
    let model = "llama3.2:1b";
    let cpu_workers = 4;

    println!("{}", "Initializing AI Chat CLI...".bright_cyan());

    // Check if Ollama is running
    let client = ollama::OllamaClient::new();
    match client.list_models().await {
        Ok(models) => {
            println!("{} {}", "✓".bright_green(), "Connected to Ollama".bright_white());
            
            if !models.iter().any(|m| m.starts_with(model)) {
                eprintln!(
                    "{} Model '{}' not found. Available models: {:?}",
                    "Warning:".bright_yellow(),
                    model,
                    models
                );
                eprintln!("\nInstall the model with: {}", 
                    format!("ollama pull {}", model).bright_cyan());
                std::process::exit(1);
            }
            
            println!("{} Using model: {}", "✓".bright_green(), model.bright_cyan());
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".bright_red().bold(), e);
            eprintln!("\n{}", "Make sure Ollama is running:".bright_yellow());
            eprintln!("  {}", "ollama serve".bright_cyan());
            std::process::exit(1);
        }
    }

    // Initialize MCP
    let mcp_manager = match McpManager::new().await {
        Ok(manager) => {
            if manager.has_tools() {
                let tool_count = manager.list_tools().len();
                println!("{} Loaded {} MCP tool(s)", 
                    "✓".bright_green(), tool_count);
                Some(manager)
            } else {
                println!("{} No MCP tools configured (create ~/.ai-chat-cli/mcp.json)", 
                    "ℹ".bright_blue());
                None
            }
        }
        Err(e) => {
            eprintln!("{} Failed to initialize MCP: {}", 
                "Warning:".bright_yellow(), e);
            None
        }
    };

    // Create executor
    let executor = AIExecutor::new(model.to_string(), cpu_workers)
        .await
        .context("Failed to create AI executor")?;

    println!("{} AI executor ready", "✓".bright_green());

    // Create and run CLI
    let mut cli = ChatCLI::new(executor, mcp_manager);
    cli.run().await?;

    Ok(())
}
