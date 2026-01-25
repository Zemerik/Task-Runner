# Contributing to Task Runner

Thank you for your interest in contributing to Task Runner! 🚀

## 📋 Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Issue Guidelines](#issue-guidelines)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Release Process](#release-process)

## 🚀 Getting Started

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Node.js**: Version 14 or higher
- **Git**: For version control

### Installation

1. **Fork the repository**
   ```bash
   git clone https://github.com/Zemerik/task-runner.git
   cd task-runner
   ```

2. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Build the project**
   ```bash
   cargo build
   ```

## 🔧 Development Setup

### Project Structure

```
task-runner/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports
│   ├── config.rs          # Configuration handling
│   ├── executor.rs        # Task execution
│   ├── task.rs            # Task definitions
│   ├── error.rs           # Error handling
│   └── utils.rs           # Utility functions
├── examples/              # Example configurations
├── scripts/               # Build and test scripts
├── tests/                 # Test configurations
├── .github/               # GitHub templates and workflows
└── docs/                  # Documentation
```

### Development Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code style
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Run the CLI locally
cargo run -- --help

# Test with example config
cargo run -- --config examples/task-runner.json list
```

## 🎨 Code Style

### Rust Code Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Write meaningful commit messages

### Commit Message Format

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(executor): add parallel task execution support
fix(config): resolve circular dependency detection issue
docs(readme): update installation instructions
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run integration tests
./scripts/test-all.sh
```

### Writing Tests

- Write unit tests for all new functionality
- Include integration tests for CLI commands
- Test error conditions and edge cases
- Use descriptive test names

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

## 📝 Submitting Changes

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write clean, well-documented code
- Add tests for new functionality
- Update documentation if needed
- Follow the code style guidelines

### 3. Test Your Changes

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat(scope): description of changes"
```

### 5. Push and Create a Pull Request

```bash
git push origin feature/your-feature-name
```

## 🐛 Issue Guidelines

### Before Creating an Issue

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** for solutions
3. **Try the latest version** of Task Runner

### Issue Template

Use the provided issue templates:
- 🐛 **Bug Report**: For bugs and issues
- ✨ **Feature Request**: For new features
- ❓ **Question**: For questions and help

### Good Issue Examples

- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment information
- Relevant configuration files

## 🔄 Pull Request Guidelines

### Before Submitting

- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation is updated
- [ ] No new warnings are generated
- [ ] Self-review completed

### PR Template

Use the provided PR template with:
- Description of changes
- Type of change
- Testing information
- Related issues

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Testing** on multiple platforms
4. **Documentation** review

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Steps

1. **Update version** in `Cargo.toml` and `package.json`
2. **Update changelog** in `v101.md` (or create new version file)
3. **Create release branch** if needed
4. **Run full test suite**
5. **Create git tag**
6. **Push to trigger release workflow**

### Creating a Release

```bash
# Update version
# Edit Cargo.toml and package.json

# Create and push tag
git tag v1.0.3
git push origin v1.0.3
```

## 🤝 Community Guidelines

### Code of Conduct

- Be respectful and inclusive
- Help others learn and grow
- Provide constructive feedback
- Follow project conventions

### Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and discussions
- **Documentation**: Check README and examples first

## 📚 Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clap Documentation](https://docs.rs/clap/)
- [Tokio Documentation](https://tokio.rs/)

## 🙏 Thank You

Thank you for contributing to Task Runner! Your contributions help make this tool better for everyone.

---

**Happy coding! 🚀** 