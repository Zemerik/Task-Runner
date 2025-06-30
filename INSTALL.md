# Installation Guide

## Prerequisites

### Install Rust

1. **Visit [rustup.rs](https://rustup.rs/)** or run:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Restart your terminal** or run:
   ```bash
   source ~/.cargo/env
   ```

3. **Verify installation**:
   ```bash
   cargo --version
   ```

## Building the Task Runner

### From Source

1. **Clone or navigate to the project directory**:
   ```bash
   cd /path/to/task-runner
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Test the installation**:
   ```bash
   ./target/release/task-runner --help
   ```

### Install Globally

1. **Install from source**:
   ```bash
   cargo install --path .
   ```

2. **Or install from npm** (after publishing):
   ```bash
   npm install -g task-runner
   ```

## Quick Test

1. **Create a test configuration** (`task-runner.json`):
   ```json
   {
     "tasks": {
       "hello": {
         "description": "Say hello",
         "commands": ["echo 'Hello from Task Runner!'"]
       }
     }
   }
   ```

2. **Run the task**:
   ```bash
   task-runner run hello
   ```

## Troubleshooting

### Common Issues

1. **"command not found: cargo"**
   - Rust is not installed or not in PATH
   - Run: `source ~/.cargo/env`

2. **Build errors**
   - Check Rust version: `rustc --version`
   - Update Rust: `rustup update`

3. **Permission errors**
   - Use `sudo` for global installation
   - Or install to user directory: `cargo install --path . --user`

### Platform-Specific Notes

- **macOS**: Works out of the box with rustup
- **Linux**: May need additional build tools: `sudo apt install build-essential`
- **Windows**: Install Visual Studio Build Tools or use WSL

## Development Setup

1. **Install development dependencies**:
   ```bash
   cargo install cargo-watch  # For development
   ```

2. **Run tests**:
   ```bash
   cargo test
   ```

3. **Development mode**:
   ```bash
   cargo run -- --help
   ```

## Publishing to NPM

1. **Build for all platforms**:
   ```bash
   # Cross-compilation setup (optional)
   rustup target add x86_64-unknown-linux-gnu
   rustup target add x86_64-pc-windows-gnu
   rustup target add x86_64-apple-darwin
   ```

2. **Package for npm**:
   ```bash
   npm run build
   npm run package
   npm publish
   ```

## Support

- 📖 [Documentation](README.md)
- 🐛 [Issues](https://github.com/yourusername/task-runner/issues)
- 💬 [Discussions](https://github.com/yourusername/task-runner/discussions) 