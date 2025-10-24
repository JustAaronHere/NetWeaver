#!/bin/bash
# NetWeaver Example: Network Monitoring
# Demonstrates real-time monitoring and reporting

set -e

NETWEAVER="./target/release/netweaver"

if [ ! -f "$NETWEAVER" ]; then
    echo "Building NetWeaver..."
    cargo build --release
fi

echo "====================================="
echo "NetWeaver Network Monitoring Examples"
echo "====================================="
echo ""

# Example 1: Quick snapshot
echo "Example 1: Network statistics snapshot"
echo "---------------------------------------"
$NETWEAVER monitor

echo ""
echo "Example 2: Real-time monitoring (10 seconds)"
echo "---------------------------------------------"
echo "Press 'q' to quit early"
timeout 10 $NETWEAVER monitor --realtime || true

echo ""
echo "Example 3: Generate comprehensive report"
echo "-----------------------------------------"
$NETWEAVER report --export network_report.html --format html --graphs

echo ""
echo "Example 4: JSON report for automation"
echo "--------------------------------------"
$NETWEAVER report --export network_report.json --format json

echo ""
echo "✓ Monitoring examples completed!"
echo "Reports saved to network_report.html and network_report.json"
