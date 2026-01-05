mod cli;
mod executor;
mod ollama;

use anyhow::{Context, Result};
use colored::*;
use executor::AIExecutor;
use cli::ChatCLI;

#[tokio::main]
async fn main() -> Result<()> {
    // Configuration
    let model = "llama3.2:1b"; // Change this to your preferred model
    let cpu_workers = 6; // Adjust based on your Mac's CPU cores

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

    // Create executor with Repartir
    let executor = AIExecutor::new(model.to_string(), cpu_workers)
        .await
        .context("Failed to create AI executor")?;

    println!("{} Repartir pool initialized with {} workers", 
        "✓".bright_green(), cpu_workers);

    // Create and run CLI
    let mut cli = ChatCLI::new(executor);
    cli.run().await?;

    Ok(())
}
