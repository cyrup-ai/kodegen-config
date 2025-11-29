#!/bin/bash
# Test script to verify path validation works correctly

echo "=== Test 1: Malicious XDG_CONFIG_HOME (path traversal to /etc) ==="
export XDG_CONFIG_HOME="/tmp/../../etc"
cargo run --example test_paths 2>&1 | grep -E "(WARN|Rejecting|config_dir)"

echo ""
echo "=== Test 2: Valid custom path in /tmp ==="
mkdir -p /tmp/test_kodegen_config
export XDG_CONFIG_HOME="/tmp/test_kodegen_config"
cargo run --example test_paths 2>&1 | grep -E "(WARN|config_dir)"

echo ""
echo "=== Test 3: Override flag with malicious path ==="
export KODEGEN_ALLOW_CUSTOM_PATHS=1
export XDG_CONFIG_HOME="/etc"
cargo run --example test_paths 2>&1 | grep -E "(WARN|UNSAFE|config_dir)"

echo ""
echo "=== Test 4: Fallback to defaults (invalid path) ==="
unset KODEGEN_ALLOW_CUSTOM_PATHS
export XDG_CONFIG_HOME="/nonexistent/path/that/does/not/exist"
cargo run --example test_paths 2>&1 | grep -E "(WARN|Rejecting|config_dir)"

echo ""
echo "=== Test 5: No environment variables (use system defaults) ==="
unset XDG_CONFIG_HOME
cargo run --example test_paths 2>&1 | grep -E "config_dir"
