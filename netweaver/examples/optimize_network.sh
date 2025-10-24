#!/bin/bash
# NetWeaver Example: Network Optimization
# Demonstrates performance tuning and optimization features

set -e

NETWEAVER="./target/release/netweaver"

if [ ! -f "$NETWEAVER" ]; then
    echo "Building NetWeaver..."
    cargo build --release
fi

echo "======================================="
echo "NetWeaver Network Optimization Examples"
echo "======================================="
echo ""

# Example 1: DNS optimization
echo "Example 1: DNS Resolver Benchmarking"
echo "-------------------------------------"
$NETWEAVER optimize --dns --dry-run

echo ""
echo "Example 2: MTU optimization"
echo "---------------------------"
$NETWEAVER optimize --mtu --dry-run

echo ""
echo "Example 3: TCP parameter tuning"
echo "--------------------------------"
$NETWEAVER optimize --tcp --dry-run

echo ""
echo "Example 4: Comprehensive optimization analysis"
echo "-----------------------------------------------"
$NETWEAVER optimize --all --dry-run

echo ""
echo "Example 5: Turbo mode (adaptive optimization)"
echo "----------------------------------------------"
echo "Note: Shows recommendations without applying them"
$NETWEAVER optimize --turbo --dry-run

echo ""
echo "✓ Optimization analysis completed!"
echo ""
echo "To apply optimizations (requires root):"
echo "  sudo $NETWEAVER optimize --all"
