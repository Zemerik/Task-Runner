#!/bin/bash

echo "🔍 Task Runner CLI - Project Structure Check"
echo "============================================="
echo

# Check if Rust is installed
if command -v cargo &> /dev/null; then
    echo "✅ Rust/Cargo is installed"
    echo "   Version: $(cargo --version)"
    echo
    
    echo "🔨 Building project..."
    if cargo build; then
        echo "✅ Build successful!"
    else
        echo "❌ Build failed"
        exit 1
    fi
else
    echo "❌ Rust/Cargo is not installed"
    echo
    echo "📦 Installation instructions:"
    echo "   1. Visit https://rustup.rs/"
    echo "   2. Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "   3. Restart your terminal"
    echo "   4. Run: cargo --version"
    echo
fi

echo
echo "📁 Project Structure:"
echo "   ├── src/"
echo "   │   ├── main.rs          (CLI entry point)"
echo "   │   ├── lib.rs           (Library exports)"
echo "   │   ├── config.rs        (Configuration handling)"
echo "   │   ├── executor.rs      (Task execution)"
echo "   │   ├── task.rs          (Task definitions)"
echo "   │   ├── error.rs         (Error handling)"
echo "   │   └── utils.rs         (Utility functions)"
echo "   ├── examples/"
echo "   │   ├── task-runner.json (JSON config example)"
echo "   │   └── task-runner.yaml (YAML config example)"
echo "   ├── Cargo.toml           (Rust dependencies)"
echo "   ├── package.json         (NPM package config)"
echo "   └── README.md            (Documentation)"
echo

echo "🚀 Next Steps:"
echo "   1. Install Rust (if not already installed)"
echo "   2. Run: cargo build --release"
echo "   3. Test with: ./target/release/task-runner --help"
echo "   4. Create a config file and test tasks"
echo

echo "📚 Documentation:"
echo "   - README.md contains full usage instructions"
echo "   - examples/ directory has sample configurations"
echo "   - Run: ./target/release/task-runner --help for CLI help" 