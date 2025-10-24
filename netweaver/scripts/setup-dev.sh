#!/bin/bash
# Development Environment Setup Script

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Setting up NetWeaver development environment${NC}"

# Install Rust components
echo "Installing Rust components..."
rustup component add rustfmt clippy

# Install development tools
echo "Installing cargo tools..."
cargo install cargo-watch 2>/dev/null || true
cargo install cargo-edit 2>/dev/null || true

# Set up git hooks
echo "Setting up git hooks..."
mkdir -p .git/hooks

cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook for NetWeaver

set -e

echo "Running pre-commit checks..."

# Format check
echo "Checking formatting..."
cargo fmt -- --check

# Clippy
echo "Running clippy..."
cargo clippy -- -D warnings

# Tests
echo "Running tests..."
cargo test

echo "All checks passed!"
EOF

chmod +x .git/hooks/pre-commit

echo -e "${GREEN}Development environment ready!${NC}"
echo ""
echo "Useful commands:"
echo "  cargo watch -x check     # Auto-check on file changes"
echo "  cargo watch -x test      # Auto-test on file changes"
echo "  make help                # See all make targets"
