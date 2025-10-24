#!/bin/bash
# NetWeaver Example: Network Scanning
# Demonstrates various scanning capabilities

set -e

NETWEAVER="./target/release/netweaver"

if [ ! -f "$NETWEAVER" ]; then
    echo "Building NetWeaver..."
    cargo build --release
fi

echo "==================================="
echo "NetWeaver Network Scanning Examples"
echo "==================================="
echo ""

# Example 1: Scan local network
echo "Example 1: Scanning local network (first 10 hosts)"
echo "---------------------------------------------------"
$NETWEAVER scan --lan --target "192.168.1.0/28" --threads 50

echo ""
echo "Example 2: Scan specific host with common ports"
echo "-----------------------------------------------"
$NETWEAVER scan --target "8.8.8.8" --ports "53,80,443"

echo ""
echo "Example 3: Scan with topology visualization and export"
echo "------------------------------------------------------"
$NETWEAVER scan --target "192.168.1.0/29" --topology --output scan_results.json

echo ""
echo "Example 4: Fast port range scan"
echo "--------------------------------"
$NETWEAVER scan --target "127.0.0.1" --ports "1-1024" --threads 100

echo ""
echo "✓ All scanning examples completed successfully!"
echo "Check scan_results.json for detailed output"
