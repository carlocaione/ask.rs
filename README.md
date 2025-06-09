# ask

A simple Rust CLI tool that converts natural language descriptions into terminal commands using Claude AI.

## What it does

`ask` takes your description of what you want to do in the terminal and returns the exact command you need to run. It uses Anthropic's Claude AI to generate precise, copy-paste ready commands.

## Features

- 🤖 Natural language to terminal command conversion
- 📋 Automatic clipboard copying of results (can be disabled)
- ⚡ Fast async execution with progress spinner
- 🔧 Verbose mode for API usage details
- 🎯 Designed for Linux and macOS environments

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd ask
```

2. Build the project:
```bash
cargo build --release
```

3. Set up your Anthropic API key:
```bash
export ASK_ANTHROPIC_API_KEY="your-api-key-here"
```

## Usage

### Command line argument
```bash
ask "list all files in the current directory"
```

### Interactive mode
```bash
ask
# You'll be prompted to enter your query
```

### Verbose mode
```bash
ask --verbose "find all python files"
```

### Skip clipboard
```bash
ask --skip-clipboard "list files"
# or
ask -s "list files"
```

## Examples

**Find files:**
```bash
$ ask "find all .js files in src directory"
find src -name "*.js"
```

**Process management:**
```bash
$ ask "kill all processes running on port 3000"
lsof -ti:3000 | xargs kill -9
```

**File operations:**
```bash
$ ask "compress folder into zip file"
zip -r folder.zip folder/
```

**System information:**
```bash
$ ask "show disk usage in human readable format"
df -h
```

**Git operations:**
```bash
$ ask "create new branch and switch to it"
git checkout -b new-branch
```

## Configuration

The tool requires the `ASK_ANTHROPIC_API_KEY` environment variable. You can set this permanently by adding it to your shell profile:

```bash
# Add to ~/.bashrc, ~/.zshrc, etc.
export ASK_ANTHROPIC_API_KEY="your-api-key-here"
```

## Development

### Build commands
- `cargo build` - Build the project
- `cargo run` - Run the application
- `cargo check` - Check code without building
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

### Project structure
- `src/main.rs` - Main application logic
- `src/llm/` - AI provider abstractions
- `src/llm/anthropic.rs` - Anthropic API integration

## Dependencies

- **reqwest** - HTTP client for API requests
- **tokio** - Async runtime
- **clap** - Command line argument parsing
- **arboard** - Clipboard integration
- **indicatif** - Progress spinner
- **inquire** - Interactive prompts
- **owo-colors** - Terminal colors
- **serde_json** - JSON handling
- **thiserror** - Error handling

## License

MIT