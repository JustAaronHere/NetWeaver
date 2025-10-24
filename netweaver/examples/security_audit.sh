#!/bin/bash
# NetWeaver Example: Security Auditing
# Demonstrates security scanning and vulnerability detection

set -e

NETWEAVER="./target/release/netweaver"

if [ ! -f "$NETWEAVER" ]; then
    echo "Building NetWeaver..."
    cargo build --release
fi

echo "====================================="
echo "NetWeaver Security Auditing Examples"
echo "====================================="
echo ""

# Example 1: ARP spoofing detection
echo "Example 1: ARP Spoofing Detection"
echo "----------------------------------"
$NETWEAVER security --arp-detect

echo ""
echo "Example 2: Open port scan (security perspective)"
echo "-------------------------------------------------"
$NETWEAVER security --port-scan

echo ""
echo "Example 3: VPN integrity test"
echo "------------------------------"
echo "Note: This test checks for DNS and IPv6 leaks"
$NETWEAVER security --vpn-test

echo ""
echo "Example 4: MITM detection"
echo "-------------------------"
$NETWEAVER security --mitm-detect

echo ""
echo "Example 5: Comprehensive security audit"
echo "----------------------------------------"
$NETWEAVER security --all

echo ""
echo "✓ Security audit completed!"
echo "Review the output above for any security concerns"
