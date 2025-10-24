#!/bin/bash
# NetWeaver Example: Basic Network Scan
# This script demonstrates common scanning patterns

echo "==================================="
echo "NetWeaver Network Scanning Examples"
echo "==================================="

# Example 1: Quick LAN scan
echo -e "\n[1] Quick LAN scan with default settings"
echo "Command: netweaver scan --lan"
echo "This automatically detects your network range and scans common ports"

# Example 2: Detailed scan with topology
echo -e "\n[2] Detailed scan with network topology"
echo "Command: netweaver scan --lan --topology --output scan-results.json"
echo "Generates a visual network map and exports data for later analysis"

# Example 3: Targeted scan
echo -e "\n[3] Scan specific subnet with custom ports"
echo "Command: netweaver scan --target 192.168.1.0/24 --ports 22,80,443,3389"
echo "Focuses on specific network range and critical ports"

# Example 4: High-speed scan
echo -e "\n[4] High-speed scan with more threads"
echo "Command: netweaver scan --lan --threads 250"
echo "Increases concurrent threads for faster scanning"

# Example 5: Port range scan
echo -e "\n[5] Comprehensive port range scan"
echo "Command: netweaver scan --target 192.168.1.100 --ports 1-10000"
echo "Scans all ports from 1-10000 on a single host"

echo -e "\n==================================="
echo "Run any of these commands to try them!"
echo "==================================="
