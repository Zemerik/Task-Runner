#!/bin/bash

set -e

echo "🧪 Task Runner CLI - Comprehensive Test Suite"
echo "=============================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Test function
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_exit_code="${3:-0}"
    
    echo -n "Testing: $test_name... "
    
    if eval "$test_command" > /tmp/test-output.log 2>&1; then
        local exit_code=$?
        if [ $exit_code -eq $expected_exit_code ]; then
            echo -e "${GREEN}✓ PASS${NC}"
            ((TESTS_PASSED++))
        else
            echo -e "${RED}✗ FAIL (exit code $exit_code, expected $expected_exit_code)${NC}"
            ((TESTS_FAILED++))
        fi
    else
        local exit_code=$?
        if [ $exit_code -eq $expected_exit_code ]; then
            echo -e "${GREEN}✓ PASS${NC}"
            ((TESTS_PASSED++))
        else
            echo -e "${RED}✗ FAIL (exit code $exit_code, expected $expected_exit_code)${NC}"
            cat /tmp/test-output.log
            ((TESTS_FAILED++))
        fi
    fi
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo is not installed${NC}"
    echo "Please install Rust first: https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✅ Rust/Cargo is installed${NC}"
echo "Version: $(cargo --version)"
echo

# Build the project
echo -e "${BLUE}🔨 Building project...${NC}"
if cargo build --release; then
    echo -e "${GREEN}✅ Build successful!${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo

# Test 1: Help command
run_test "Help command" "./target/release/task-runner --help"

# Test 2: Version command
run_test "Version command" "./target/release/task-runner --version"

# Test 3: List tasks (should fail without config)
run_test "List tasks without config" "./target/release/task-runner list" 1

# Test 4: Validate config
run_test "Validate test config" "./target/release/task-runner --config tests/test-config.json validate"

# Test 5: List tasks with config
run_test "List tasks with config" "./target/release/task-runner --config tests/test-config.json list"

# Test 6: List tasks with details
run_test "List tasks with details" "./target/release/task-runner --config tests/test-config.json list --details"

# Test 7: Show task info
run_test "Show task info" "./target/release/task-runner --config tests/test-config.json info hello"

# Test 8: Run simple task
run_test "Run simple task" "./target/release/task-runner --config tests/test-config.json run hello"

# Test 9: Run task with environment variable
run_test "Run task with env var" "./target/release/task-runner --config tests/test-config.json run test-echo"

# Test 10: Run sequential task
run_test "Run sequential task" "./target/release/task-runner --config tests/test-config.json run test-sequential"

# Test 11: Run parallel task
run_test "Run parallel task" "./target/release/task-runner --config tests/test-config.json run test-parallel --parallel"

# Test 12: Run task with dependencies
run_test "Run task with dependencies" "./target/release/task-runner --config tests/test-config.json run test-deps"

# Test 13: Run multiple tasks
run_test "Run multiple tasks" "./target/release/task-runner --config tests/test-config.json run hello test-success"

# Test 14: Run task with continue on error
run_test "Run task with continue on error" "./target/release/task-runner --config tests/test-config.json run test-error test-success --continue-on-error"

# Test 15: Run non-existent task (should fail)
run_test "Run non-existent task" "./target/release/task-runner --config tests/test-config.json run non-existent" 1

# Test 16: Test YAML config
if [ -f "examples/task-runner.yaml" ]; then
    run_test "Validate YAML config" "./target/release/task-runner --config examples/task-runner.yaml validate"
fi

# Test 17: Test JSON config
if [ -f "examples/task-runner.json" ]; then
    run_test "Validate JSON config" "./target/release/task-runner --config examples/task-runner.json validate"
fi

echo
echo "=============================================="
echo -e "${BLUE}📊 Test Results:${NC}"
echo -e "  ${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "  ${RED}Failed: $TESTS_FAILED${NC}"
echo -e "  Total: $((TESTS_PASSED + TESTS_FAILED))"
echo

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    echo
    echo "🚀 Ready to use:"
    echo "  ./target/release/task-runner --help"
    echo "  ./target/release/task-runner --config tests/test-config.json list"
    echo "  ./target/release/task-runner --config tests/test-config.json run hello"
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi

# Cleanup
rm -f /tmp/test-output.log 