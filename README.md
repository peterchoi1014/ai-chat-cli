# AI Chat CLI

A powerful command-line AI chat application built with Rust, featuring local AI model inference through Ollama and distributed task execution capabilities via Repartir.

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.92.0-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux-lightgrey.svg)

</div>

## ✨ Features

- 🤖 **Local AI Models** - Run AI models completely offline using Ollama
- 💬 **Interactive Chat** - Beautiful colored terminal interface with conversation history
- 💾 **Conversation Management** - Save and load chat sessions as JSON files
- 📦 **Batch Processing** - Process multiple prompts from text files
- 🔄 **Model Switching** - Switch between different AI models on the fly
- 🎨 **Colored Output** - Syntax-highlighted responses with emoji indicators
- ⌨️ **Readline Support** - Command history with up/down arrow navigation
- ⚡ **Built with Rust** - Fast, safe, and memory-efficient

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
  - [Basic Chat](#basic-chat)
  - [Commands](#commands)
  - [Batch Processing](#batch-processing)
  - [Conversation Management](#conversation-management)
- [Available Models](#available-models)
- [Architecture](#architecture)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## 🔧 Prerequisites

### Required

- **Rust** 1.92.0 or later ([Install Rust](https://rustup.rs/))
- **Ollama** - Local AI runtime ([Install Ollama](https://ollama.ai/))

### System Requirements

- macOS (Apple Silicon or Intel) or Linux
- 8GB+ RAM (16GB+ recommended for larger models)
- 10GB+ free disk space for AI models

## 📥 Installation

### 1. Install Ollama

```bash
# macOS
brew install ollama

# Linux
curl -fsSL https://ollama.ai/install.sh | sh

# Start Ollama service
ollama serve
```

### 2. Download an AI Model

```bash
# Recommended: Fast and lightweight (1.3GB)
ollama pull llama3.2:1b

# Or choose another model:
ollama pull llama3.2:3b    # Medium size (2GB)
ollama pull phi3:mini      # Microsoft's small model (2.3GB)
ollama pull mistral:7b     # High quality (4.1GB)
```

### 3. Clone and Build

```bash
# Clone the repository
git clone https://github.com/peterchoi1014/ai-chat-cli.git
cd ai-chat-cli

# Build in release mode
cargo build --release

# Binary will be available at: ./target/release/ai-chat-cli
```

## 🚀 Quick Start

```bash
# Make sure Ollama is running
ollama serve

# In another terminal, run the CLI
cargo run --release

# Or run the compiled binary directly
./target/release/ai-chat-cli
```

You should see:

```
============================================================
  AI Chat CLI - Powered by Repartir
============================================================

Commands:
  /help - Show this help message
  /clear - Clear conversation history
  /history - Show conversation history
  /model - Show current model
  /model <n> - Switch to different model
  /save <f> - Save conversation to file
  /load <f> - Load conversation from file
  /batch <f> - Process batch file
  /quit - Exit the chat

Start chatting! (Ctrl+C to interrupt, /quit to exit)

You: 
```

## 💡 Usage

### Basic Chat

Simply type your message and press Enter:

```
You: What is Rust programming language?
AI: Rust is a systems programming language that focuses on safety, 
    speed, and concurrency. It achieves memory safety without using 
    garbage collection...

You: Can you write a hello world program?
AI: Sure! Here's a simple hello world in Rust:

    fn main() {
        println!("Hello, World!");
    }

You: Thanks!
AI: You're welcome! Feel free to ask if you have more questions.
```

### Commands

#### `/help` - Show available commands

```
You: /help

Available Commands:
  /help - Show this help message
  /clear - Clear conversation history
  /history - Show conversation history
  ...
```

#### `/model` - View or switch models

```
You: /model
Current model: llama3.2:1b

You: /model mistral:7b
✓ Switched to model: mistral:7b
```

#### `/history` - View conversation history

```
You: /history

Conversation History:
------------------------------------------------------------
You [1]: What is Rust?
AI [2]: Rust is a systems programming language...
You [3]: Can you show me an example?
AI [4]: Sure! Here's an example...
------------------------------------------------------------
```

#### `/clear` - Clear conversation history

```
You: /clear
Conversation history cleared.
```

#### `/quit` or `/exit` - Exit the application

```
You: /quit
Goodbye!
```

### Batch Processing

Process multiple prompts from a text file:

#### 1. Create a prompts file

Create `prompts.txt`:
```
What is Rust?
Write a hello world program in Python
Explain how recursion works
What is the difference between a vector and an array?
```

#### 2. Run batch processing

```
You: /batch prompts.txt

📋 Processing 4 prompts...

▶ [1/4] What is Rust?
✓ Rust is a systems programming language...

▶ [2/4] Write a hello world program in Python
✓ print("Hello, World!")

▶ [3/4] Explain how recursion works
✓ Recursion is a programming technique...

▶ [4/4] What is the difference between a vector and an array?
✓ A vector is a dynamic array...

✓ Batch processing complete
```

### Conversation Management

#### Save a conversation

```
You: Hello, my name is Alice
AI: Nice to meet you, Alice! How can I help you today?

You: Tell me about machine learning
AI: Machine learning is a subset of artificial intelligence...

You: /save my_chat.json
✓ Conversation saved to my_chat.json
```

#### Load a conversation

```
You: /load my_chat.json
✓ Conversation loaded from my_chat.json

You: What's my name?
AI: Your name is Alice.
```

The loaded conversation maintains full context, so the AI remembers previous interactions.

## 🤖 Available Models

Popular models you can use with Ollama:

| Model | Size | Speed | Quality | Use Case |
|-------|------|-------|---------|----------|
| `llama3.2:1b` | 1.3GB | ⚡⚡⚡ | ⭐⭐ | Quick responses, limited hardware |
| `llama3.2:3b` | 2GB | ⚡⚡ | ⭐⭐⭐ | Balanced performance |
| `phi3:mini` | 2.3GB | ⚡⚡ | ⭐⭐⭐ | Microsoft's efficient model |
| `mistral:7b` | 4.1GB | ⚡ | ⭐⭐⭐⭐ | High-quality responses |
| `llama3:8b` | 4.7GB | ⚡ | ⭐⭐⭐⭐ | Latest Llama model |

Install any model with:
```bash
ollama pull <model-name>
```

List installed models:
```bash
ollama list
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│                   CLI Interface                      │
│              (Colored Terminal UI)                   │
└─────────────────┬───────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────┐
│                  AI Executor                         │
│         (Task Management & Coordination)             │
└─────────────────┬───────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────┐
│                Ollama Client                         │
│         (HTTP API Communication)                     │
└─────────────────┬───────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────┐
│              Ollama Service                          │
│         (Local AI Model Inference)                   │
└─────────────────────────────────────────────────────┘
```

### Project Structure

```
ai-chat-cli/
├── src/
│   ├── main.rs           # Application entry point
│   ├── cli.rs            # Terminal interface & command handling
│   ├── executor.rs       # AI task executor
│   └── ollama.rs         # Ollama API client
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

### Key Components

- **CLI Module** (`cli.rs`) - Handles user interaction, command parsing, and colored output
- **Executor Module** (`executor.rs`) - Manages AI inference tasks and model switching
- **Ollama Client** (`ollama.rs`) - Communicates with Ollama API for model inference
- **Main** (`main.rs`) - Initializes components and starts the application

## 🛠️ Development

### Build from source

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### Run tests

```bash
cargo test
```

### Check code

```bash
# Check for errors
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```

### Dependencies

Main dependencies:

- `tokio` - Async runtime
- `reqwest` - HTTP client for Ollama API
- `serde` / `serde_json` - JSON serialization
- `colored` - Terminal colors
- `rustyline` - Readline-like input
- `anyhow` - Error handling

See `Cargo.toml` for complete list.

## 🐛 Troubleshooting

### Ollama connection failed

**Error**: `Failed to send request to Ollama`

**Solution**:
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# If not running, start it
ollama serve
```

### Model not found

**Error**: `Model 'llama3.2:1b' not found`

**Solution**:
```bash
# List installed models
ollama list

# Pull the required model
ollama pull llama3.2:1b
```

### Slow responses

**Solutions**:
1. Use a smaller model: `llama3.2:1b` instead of `mistral:7b`
2. Close other applications to free up RAM
3. Use GPU acceleration if available (Ollama automatic)

### Command not recognized

Make sure you're using the correct prefix:
- ✅ `/help` (correct)
- ❌ `help` (incorrect - missing slash)

### File save/load errors

**Error**: `Permission denied` or `No such file or directory`

**Solution**:
```bash
# Use absolute path
/save /Users/username/chats/conversation.json

# Or ensure current directory is writable
ls -la
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

### Development Guidelines

- Follow Rust style guidelines (`cargo fmt`)
- Add tests for new features
- Update documentation
- Keep commits atomic and well-described

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[Ollama](https://ollama.ai/)** - Local AI model runtime
- **[Repartir](https://github.com/paiml/repartir)** - Distributed computing framework
- **Rust Community** - For amazing tools and libraries

## 📚 Resources

- [Ollama Documentation](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Repartir Documentation](https://paiml.github.io/repartir/)

## 🗺️ Roadmap

- [ ] Streaming responses for real-time output
- [ ] RAG (Retrieval Augmented Generation) support
- [ ] Multi-modal support (images, audio)
- [ ] Web interface
- [ ] Distributed inference across remote workers
- [ ] Conversation search and tagging
- [ ] Export to different formats (Markdown, PDF)
- [ ] Plugin system for extensibility

---

<div align="center">

**Built with ❤️ using Rust and Ollama**

[Report Bug](https://github.com/peterchoi1014/ai-chat-cli/issues) · [Request Feature](https://github.com/peterchoi1014/ai-chat-cli/issues)

</div>
