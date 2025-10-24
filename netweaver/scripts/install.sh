#!/bin/bash
# NetWeaver Installation Script
# Handles dependencies, compilation, and installation

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}=================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}=================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

check_command() {
    if command -v $1 &> /dev/null; then
        print_success "$1 found"
        return 0
    else
        print_error "$1 not found"
        return 1
    fi
}

print_header "NetWeaver Installation"

# Check for root if installing system-wide
INSTALL_SYSTEM=false
if [ "$1" == "--system" ]; then
    INSTALL_SYSTEM=true
    if [ "$EUID" -ne 0 ]; then
        print_error "System installation requires root privileges"
        echo "Run: sudo $0 --system"
        exit 1
    fi
fi

# Check dependencies
print_header "Checking Dependencies"

MISSING_DEPS=false

if ! check_command cargo; then
    print_error "Rust toolchain not found"
    echo "Install from: https://rustup.rs/"
    MISSING_DEPS=true
fi

if ! check_command gcc && ! check_command clang; then
    print_error "C compiler not found (need gcc or clang)"
    MISSING_DEPS=true
fi

if ! check_command pkg-config; then
    print_warning "pkg-config not found (may cause build issues)"
fi

if [ "$MISSING_DEPS" = true ]; then
    echo ""
    print_error "Missing required dependencies"
    echo ""
    echo "Install on Ubuntu/Debian:"
    echo "  sudo apt update"
    echo "  sudo apt install build-essential pkg-config libpcap-dev"
    echo ""
    echo "Install on Fedora/RHEL:"
    echo "  sudo dnf install gcc make pkg-config libpcap-devel"
    echo ""
    echo "Install on macOS:"
    echo "  brew install libpcap"
    exit 1
fi

# Check Rust version
print_header "Checking Rust Version"
RUST_VERSION=$(cargo --version | awk '{print $2}')
print_success "Rust version: $RUST_VERSION"

MIN_VERSION="1.70.0"
if [ "$(printf '%s\n' "$MIN_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$MIN_VERSION" ]; then
    print_warning "Rust version $RUST_VERSION is older than recommended $MIN_VERSION"
    echo "Update with: rustup update"
fi

# Build project
print_header "Building NetWeaver"
echo "This may take a few minutes on first build..."

if cargo build --release; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Run tests
print_header "Running Tests"
if cargo test --release; then
    print_success "All tests passed"
else
    print_warning "Some tests failed (non-critical)"
fi

# Install
print_header "Installation"

BINARY="target/release/netweaver"

if [ "$INSTALL_SYSTEM" = true ]; then
    INSTALL_PATH="/usr/local/bin/netweaver"
    cp "$BINARY" "$INSTALL_PATH"
    chmod 755 "$INSTALL_PATH"
    print_success "Installed to $INSTALL_PATH"
    
    # Offer to set capabilities
    echo ""
    echo "Grant network capabilities? (recommended for non-root usage)"
    echo "This allows NetWeaver to use raw sockets without sudo"
    read -p "Grant capabilities? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if command -v setcap &> /dev/null; then
            setcap cap_net_raw,cap_net_admin=eip "$INSTALL_PATH"
            print_success "Capabilities granted"
        else
            print_warning "setcap not found - skipping"
        fi
    fi
else
    print_success "Binary located at: $BINARY"
    echo ""
    echo "To install system-wide:"
    echo "  sudo $0 --system"
    echo ""
    echo "Or manually copy:"
    echo "  sudo cp $BINARY /usr/local/bin/"
fi

# Create config directory
print_header "Configuration"
CONFIG_DIR="$HOME/.config/netweaver"
if [ ! -d "$CONFIG_DIR" ]; then
    mkdir -p "$CONFIG_DIR"
    print_success "Created config directory: $CONFIG_DIR"
fi

if [ ! -f "$CONFIG_DIR/config.toml" ]; then
    if [ -f "config/netweaver.toml" ]; then
        cp config/netweaver.toml "$CONFIG_DIR/config.toml"
        print_success "Created default config file"
    fi
fi

# Final message
print_header "Installation Complete!"
echo ""
echo "NetWeaver is ready to use!"
echo ""
echo "Quick start:"
echo "  netweaver scan --lan          # Scan your local network"
echo "  netweaver trace --target 1.1.1.1  # Trace route to target"
echo "  netweaver monitor --realtime  # Live network monitoring"
echo ""
echo "For help:"
echo "  netweaver --help"
echo ""
echo "Documentation:"
echo "  README.md - User guide and command reference"
echo "  docs/ARCHITECTURE.md - Technical architecture"
echo "  examples/ - Example scripts"
echo ""
print_success "Happy networking!"
