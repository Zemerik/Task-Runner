#!/bin/bash

echo "🔍 Task Runner CLI - Structure Validation"
echo "========================================="
echo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

VALIDATION_PASSED=0
VALIDATION_FAILED=0

validate_file() {
    local file="$1"
    local description="$2"
    
    echo -n "Checking: $description... "
    
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ EXISTS${NC}"
        ((VALIDATION_PASSED++))
    else
        echo -e "${RED}✗ MISSING${NC}"
        ((VALIDATION_FAILED++))
    fi
}

validate_directory() {
    local dir="$1"
    local description="$2"
    
    echo -n "Checking: $description... "
    
    if [ -d "$dir" ]; then
        echo -e "${GREEN}✓ EXISTS${NC}"
        ((VALIDATION_PASSED++))
    else
        echo -e "${RED}✗ MISSING${NC}"
        ((VALIDATION_FAILED++))
    fi
}

# Check essential files
validate_file "Cargo.toml" "Cargo.toml (Rust project config)"
validate_file "package.json" "package.json (NPM package config)"
validate_file "README.md" "README.md (Documentation)"
validate_file "LICENSE" "LICENSE (MIT license)"
validate_file ".gitignore" ".gitignore (Git ignore rules)"

# Check source directory
validate_directory "src" "src/ (Source code directory)"
validate_file "src/main.rs" "src/main.rs (CLI entry point)"
validate_file "src/lib.rs" "src/lib.rs (Library exports)"
validate_file "src/config.rs" "src/config.rs (Configuration handling)"
validate_file "src/executor.rs" "src/executor.rs (Task execution)"
validate_file "src/task.rs" "src/task.rs (Task definitions)"
validate_file "src/error.rs" "src/error.rs (Error handling)"
validate_file "src/utils.rs" "src/utils.rs (Utility functions)"

# Check examples
validate_directory "examples" "examples/ (Example configurations)"
validate_file "examples/task-runner.json" "examples/task-runner.json (JSON example)"
validate_file "examples/task-runner.yaml" "examples/task-runner.yaml (YAML example)"

# Check scripts
validate_directory "scripts" "scripts/ (Utility scripts)"
validate_file "scripts/test-project.sh" "scripts/test-project.sh (Project test script)"
validate_file "scripts/test-all.sh" "scripts/test-all.sh (Comprehensive test script)"

# Check tests
validate_file "tests/test-config.json" "tests/test-config.json (Test configuration)"

echo
echo "========================================="
echo -e "${BLUE}📊 Validation Results:${NC}"
echo -e "  ${GREEN}Passed: $VALIDATION_PASSED${NC}"
echo -e "  ${RED}Failed: $VALIDATION_FAILED${NC}"
echo -e "  Total: $((VALIDATION_PASSED + VALIDATION_FAILED))"
echo

# Check for common issues
echo -e "${BLUE}🔍 Additional Checks:${NC}"

# Check if Cargo.toml has required dependencies
if grep -q "clap" Cargo.toml; then
    echo -e "  ${GREEN}✓ clap dependency found${NC}"
else
    echo -e "  ${RED}✗ clap dependency missing${NC}"
    ((VALIDATION_FAILED++))
fi

if grep -q "tokio" Cargo.toml; then
    echo -e "  ${GREEN}✓ tokio dependency found${NC}"
else
    echo -e "  ${RED}✗ tokio dependency missing${NC}"
    ((VALIDATION_FAILED++))
fi

if grep -q "serde" Cargo.toml; then
    echo -e "  ${GREEN}✓ serde dependency found${NC}"
else
    echo -e "  ${RED}✗ serde dependency missing${NC}"
    ((VALIDATION_FAILED++))
fi

# Check if package.json has required fields
if grep -q '"name"' package.json; then
    echo -e "  ${GREEN}✓ package.json has name field${NC}"
else
    echo -e "  ${RED}✗ package.json missing name field${NC}"
    ((VALIDATION_FAILED++))
fi

if grep -q '"bin"' package.json; then
    echo -e "  ${GREEN}✓ package.json has bin field${NC}"
else
    echo -e "  ${RED}✗ package.json missing bin field${NC}"
    ((VALIDATION_FAILED++))
fi

# Check if README has essential sections
if grep -q "## Installation" README.md; then
    echo -e "  ${GREEN}✓ README has installation section${NC}"
else
    echo -e "  ${YELLOW}⚠ README missing installation section${NC}"
fi

if grep -q "## Usage" README.md; then
    echo -e "  ${GREEN}✓ README has usage section${NC}"
else
    echo -e "  ${YELLOW}⚠ README missing usage section${NC}"
fi

echo
if [ $VALIDATION_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All validations passed!${NC}"
    echo
    echo "🚀 Project is ready for development:"
    echo "  1. Install Rust: https://rustup.rs/"
    echo "  2. Run: cargo build --release"
    echo "  3. Run: ./scripts/test-all.sh"
else
    echo -e "${RED}❌ Some validations failed${NC}"
    echo "Please fix the issues above before proceeding."
    exit 1
fi 