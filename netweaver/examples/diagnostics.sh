#!/bin/bash
# NetWeaver Example: Network Diagnostics
# Demonstrates traceroute and deep packet inspection

set -e

NETWEAVER="./target/release/netweaver"

if [ ! -f "$NETWEAVER" ]; then
    echo "Building NetWeaver..."
    cargo build --release
fi

echo "====================================="
echo "NetWeaver Diagnostics Examples"
echo "====================================="
echo ""

# Example 1: Basic traceroute
echo "Example 1: Traceroute to google.com"
echo "------------------------------------"
$NETWEAVER trace --target google.com --max-hops 15

echo ""
echo "Example 2: Detailed traceroute with export"
echo "-------------------------------------------"
$NETWEAVER trace --target 1.1.1.1 --probes 5 --output trace_results.json

echo ""
echo "Example 3: Traceroute to multiple targets"
echo "------------------------------------------"
for target in "8.8.8.8" "1.1.1.1" "cloudflare.com"; do
    echo ""
    echo "Tracing route to $target..."
    $NETWEAVER trace --target "$target" --max-hops 10 --probes 3
done

echo ""
echo "✓ Diagnostics examples completed!"
echo "Trace data saved to trace_results.json"
