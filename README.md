# Task Runner

A fast and flexible CLI task runner for managing development workflows. Built in Rust for maximum performance and reliability.

## Features

- ⚡ **Fast Execution** - Built in Rust for optimal performance
- 🔄 **Parallel & Sequential** - Run tasks in parallel or sequentially
- 📋 **Dependency Management** - Automatic dependency resolution
- 🎨 **Rich Output** - Beautiful, colored terminal output with progress indicators
- 📁 **Multiple Formats** - Support for JSON, YAML, and TOML configuration
- 🔧 **Environment Variables** - Per-task and global environment configuration
- ⏱️ **Timeout Support** - Set timeouts for long-running tasks
- 🛡️ **Error Handling** - Robust error handling with continue-on-error options

## Installation

### From NPM (Recommended)

```bash
npm install -g task-runner
```

### From Source

```bash
# Clone the repository
git clone https://github.com/Zemerik/task-runner.git
cd task-runner

# Build and install
cargo build --release
cargo install --path .
```

## Quick Start

1. **Create a configuration file** (`task-runner.json`):

```json
{
  "env": {
    "NODE_ENV": "development"
  },
  "tasks": {
    "dev": {
      "description": "Start development environment",
      "commands": [
        "npm run build:watch",
        "npm run server:dev",
        "npm run test:watch"
      ],
      "parallel": true
    },
    "build": {
      "description": "Build for production",
      "dependencies": ["clean"],
      "commands": [
        "npm run compile",
        "npm run bundle",
        "npm run optimize"
      ]
    },
    "deploy": {
      "description": "Deploy to production",
      "dependencies": ["build", "test"],
      "commands": [
        "npm run deploy:staging",
        "npm run health-check",
        "npm run deploy:prod"
      ],
      "env": {
        "NODE_ENV": "production"
      }
    }
  }
}
```

2. **Run tasks**:

```bash
# List available tasks
task-runner list

# Run a single task
task-runner run build

# Run multiple tasks in parallel
task-runner run dev --parallel

# Run with dependencies
task-runner run deploy
```

## Configuration

Task Runner supports multiple configuration file formats:

- `task-runner.json` (JSON)
- `task-runner.yaml` or `task-runner.yml` (YAML)
- `task-runner.toml` (TOML)

### Configuration Structure

```json
{
  "env": {
    "GLOBAL_VAR": "value"
  },
  "default_timeout": 300,
  "default_working_dir": "./src",
  "tasks": {
    "task-name": {
      "description": "Task description",
      "commands": ["command1", "command2"],
      "dependencies": ["other-task"],
      "env": {
        "TASK_VAR": "value"
      },
      "parallel": false,
      "sequential": true,
      "working_dir": "./custom/path",
      "timeout": 60,
      "continue_on_error": false,
      "hidden": false
    }
  }
}
```

### Task Properties

| Property | Type | Description |
|----------|------|-------------|
| `description` | string | Human-readable task description |
| `commands` | string[] | Commands to execute |
| `dependencies` | string[] | Tasks that must run before this task |
| `env` | object | Environment variables for this task |
| `parallel` | boolean | Run commands in parallel |
| `sequential` | boolean | Run commands sequentially |
| `working_dir` | string | Working directory for task execution |
| `timeout` | number | Timeout in seconds |
| `continue_on_error` | boolean | Continue if commands fail |
| `hidden` | boolean | Hide from task list |

## Usage

### Commands

#### List Tasks

```bash
# List all tasks
task-runner list

# List with details
task-runner list --details
```

#### Run Tasks

```bash
# Run a single task
task-runner run build

# Run multiple tasks
task-runner run build test deploy

# Run in parallel
task-runner run build test --parallel

# Run sequentially
task-runner run build test --sequential

# Continue on error
task-runner run build test --continue-on-error
```

#### Task Information

```bash
# Show task details
task-runner info build
```

#### Validate Configuration

```bash
# Validate config file
task-runner validate
```

### Options

| Option | Description |
|--------|-------------|
| `--config, -c` | Specify configuration file path |
| `--verbose, -v` | Enable verbose output |
| `--env, -e` | Set environment for task execution |

## Examples

### Frontend Development Workflow

```json
{
  "tasks": {
    "dev": {
      "description": "Start development server",
      "commands": [
        "npm run dev",
        "npm run storybook"
      ],
      "parallel": true
    },
    "build": {
      "description": "Build for production",
      "dependencies": ["clean"],
      "commands": [
        "npm run build",
        "npm run test:ci"
      ]
    },
    "deploy": {
      "description": "Deploy to staging",
      "dependencies": ["build"],
      "commands": [
        "npm run deploy:staging"
      ]
    }
  }
}
```

### Full-Stack Development

```yaml
env:
  NODE_ENV: development
  DATABASE_URL: postgresql://localhost/dev

tasks:
  backend:
    description: Start backend server
    commands:
      - "cargo run"
      - "cargo test --watch"
    parallel: true
    working_dir: "./backend"
    
  frontend:
    description: Start frontend development
    commands:
      - "npm run dev"
      - "npm run test:watch"
    parallel: true
    working_dir: "./frontend"
    
  dev:
    description: Start full development environment
    dependencies: ["backend", "frontend"]
    commands: []
    
  test:
    description: Run all tests
    commands:
      - "cargo test"
      - "npm run test"
    working_dir: "."
```

## Error Handling

Task Runner provides comprehensive error handling:

- **Task not found**: Clear error message with available tasks
- **Circular dependencies**: Detection and reporting of dependency cycles
- **Command failures**: Detailed error output with exit codes
- **Configuration errors**: Validation with helpful error messages

## Performance

Built in Rust, Task Runner is designed for speed:

- **Fast startup**: Minimal overhead for task execution
- **Efficient parallelization**: Optimal use of system resources
- **Memory efficient**: Low memory footprint even with many tasks
- **Cross-platform**: Native binaries for Windows, macOS, and Linux

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Support

- 📖 [Documentation](https://github.com/Zemerik/task-runner#readme)
- 🐛 [Issues](https://github.com/Zemerik/task-runner/issues)
- 💬 [Discussions](https://github.com/Zemerik/task-runner/discussions)
