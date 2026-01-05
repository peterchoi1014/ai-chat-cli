use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::executor::AIExecutor;
use crate::ollama::Message;
use std::fs;
use serde_json;

pub struct ChatCLI {
    executor: AIExecutor,
    history: Vec<Message>,
}

impl ChatCLI {
    pub fn new(executor: AIExecutor) -> Self {
        Self {
            executor,
            history: Vec::new(),
        }
    }

    pub fn save_conversation(&self, filename: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.history)?;
        fs::write(filename, json)?;
        println!("Conversation saved to {}", filename);
        Ok(())
    }

    pub fn load_conversation(&mut self, filename: &str) -> Result<()> {
        let json = fs::read_to_string(filename)?;
        self.history = serde_json::from_str(&json)?;
        println!("Conversation loaded from {}", filename);
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        self.print_welcome();

        let mut rl = DefaultEditor::new()?;

        loop {
            let prompt = format!("{} ", "You:".bright_green().bold());
            
            match rl.readline(&prompt) {
                Ok(line) => {
                    let input = line.trim();
                    
                    if input.is_empty() {
                        continue;
                    }

                    // Handle commands
                    if input.starts_with('/') {
                        if !self.handle_command(input).await? {
                            break;
                        }
                        continue;
                    }

                    // Add user message to history
                    self.history.push(Message {
                        role: "user".to_string(),
                        content: input.to_string(),
                    });

                    // Get AI response
                    print!("{} ", "AI:".bright_blue().bold());
                    
                    match self.executor.chat(self.history.clone()).await {
                        Ok(response) => {
                            println!("{}\n", response.bright_white());
                            
                            // Add assistant response to history
                            self.history.push(Message {
                                role: "assistant".to_string(),
                                content: response,
                            });
                        }
                        Err(e) => {
                            eprintln!("{} {}\n", "Error:".bright_red().bold(), e);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}",  "Use /quit to exit".yellow());
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_command(&mut self, cmd: &str) -> Result<bool> {
        if cmd.starts_with("/batch ") {
            let file = cmd.strip_prefix("/batch ").unwrap().trim();
            self.process_batch_file(file).await?;
            return Ok(true);
        }
        match cmd {
            "/quit" | "/exit" => {
                println!("{}", "Goodbye!".bright_cyan());
                return Ok(false);
            }
            "/clear" => {
                self.history.clear();
                println!("{}", "Conversation history cleared.".yellow());
            }
            "/history" => {
                self.show_history();
            }
            "/help" => {
                self.show_help();
            }
            "/model" => {
                println!("Current model: {}", self.executor.get_model().bright_cyan());
            }
            cmd if cmd.starts_with("/model ") => {
                let model = cmd.strip_prefix("/model ").unwrap().trim();
                match self.executor.switch_model(model.to_string()).await {
                    Ok(_) => {
                        println!("{} Switched to model: {}", "✓".bright_green(), model.bright_cyan());
                        self.history.clear(); // Clear history when switching models
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error:".bright_red(), e);
                    }
                }
            }
            _ => {
                println!("{} {}", "Unknown command:".bright_red(), cmd);
                println!("Type {} for available commands", "/help".bright_cyan());
            }
        }
        Ok(true)
    }
    
    async fn process_batch_file(&self, filename: &str) -> Result<()> {
        let content = fs::read_to_string(filename)?;
        let prompts: Vec<String> = content.lines()
            .map(|s: &str| s.to_string())
            .collect();
    
        println!("Processing {} prompts...", prompts.len());
    
        for (i, prompt) in prompts.iter().enumerate() {
            println!("\n[{}/{}] {}", i + 1, prompts.len(), prompt);
            let response = self.executor.chat(vec![Message {
                role: "user".to_string(),
                content: prompt.clone(),
            }]).await?;
            println!("Response: {}", response);
        }
    
        Ok(())
    }

    fn print_welcome(&self) {
        println!("\n{}", "=".repeat(60).bright_cyan());
        println!("{}", "  AI Chat CLI - Powered by Repartir".bright_cyan().bold());
        println!("{}", "=".repeat(60).bright_cyan());
        println!("\n{}", "Commands:".bright_yellow().bold());
        println!("  {} - Show this help message", "/help".bright_cyan());
        println!("  {} - Clear conversation history", "/clear".bright_cyan());
        println!("  {} - Show conversation history", "/history".bright_cyan());
        println!("  {} - Show current model", "/model".bright_cyan());
        println!("  {} <name> - Switch to different model", "/model".bright_cyan());
        println!("  {} - Exit the chat", "/quit".bright_cyan());
        println!("\n{}\n", "Start chatting! (Ctrl+C to interrupt, /quit to exit)".bright_white());
    }

    fn show_help(&self) {
        println!("\n{}", "Available Commands:".bright_yellow().bold());
        println!("  {} - Show this help message", "/help".bright_cyan());
        println!("  {} - Clear conversation history", "/clear".bright_cyan());
        println!("  {} - Show conversation history", "/history".bright_cyan());
        println!("  {} - Show current model", "/model".bright_cyan());
        println!("  {} <name> - Switch to different model", "/model".bright_cyan());
        println!("  {} - Exit the chat\n", "/quit".bright_cyan());
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("{}", "No conversation history yet.".yellow());
            return;
        }

        println!("\n{}", "Conversation History:".bright_yellow().bold());
        println!("{}", "-".repeat(60).bright_black());
        
        for (i, msg) in self.history.iter().enumerate() {
            let role = if msg.role == "user" {
                "You".bright_green().bold()
            } else {
                "AI".bright_blue().bold()
            };
            
            println!("{} [{}]: {}", role, i + 1, msg.content);
        }
        println!("{}\n", "-".repeat(60).bright_black());
    }
}
