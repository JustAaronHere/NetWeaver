# NetWeaver Quick Start Guide

Get up and running with NetWeaver in minutes.

## Installation

### Step 1: Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libpcap-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Fedora/RHEL/CentOS:**
```bash
sudo dnf install gcc make pkg-config libpcap-devel curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**macOS:**
```bash
# Install Homebrew if needed: https://brew.sh
brew install libpcap rust
```

### Step 2: Build NetWeaver

```bash
cd netweaver
cargo build --release
```

The first build will take several minutes as it downloads and compiles dependencies.

### Step 3: Install (Optional)

```bash
sudo make install
```

Or use the install script:
```bash
./scripts/install.sh --system
```

## First Commands

### Scan Your Network

```bash
# Simple LAN scan
netweaver scan --lan

# With topology visualization
netweaver scan --lan --topology

# Save results
netweaver scan --lan --output devices.json
```

### Trace a Route

```bash
# Basic trace
netweaver trace --target google.com

# Detailed trace
netweaver trace --target 1.1.1.1 --max-hops 20 --probes 5
```

### Monitor Network

```bash
# Real-time dashboard (requires sudo)
sudo netweaver monitor --realtime

# Snapshot statistics
netweaver monitor
```

### Optimize Performance

```bash
# See recommendations
netweaver optimize --all --dry-run

# Apply DNS optimization (requires sudo)
sudo netweaver optimize --dns

# Full optimization (requires sudo)
sudo netweaver optimize --all
```

### Security Audit

```bash
# Run all security checks
sudo netweaver security --all

# Specific checks
sudo netweaver security --arp-detect
netweaver security --vpn-test
```

## Common Workflows

### Morning Network Check

```bash
#!/bin/bash
echo "Running morning network check..."

# Scan network
netweaver scan --lan --output morning-scan.json

# Check gateway latency
netweaver trace --target 192.168.1.1 --max-hops 5

# Verify DNS performance
netweaver optimize --dns --dry-run

echo "Check complete!"
```

### Performance Troubleshooting

```bash
#!/bin/bash
echo "Diagnosing network performance..."

# Check route to problem destination
netweaver trace --target slow-site.com

# Look for local issues
netweaver optimize --all --dry-run

# Monitor in real-time
sudo netweaver monitor --realtime --protocol tcp
```

### Security Audit

```bash
#!/bin/bash
echo "Running security audit..."

# Check for local vulnerabilities
sudo netweaver security --port-scan

# Monitor for attacks
sudo netweaver security --arp-detect --mitm-detect

# Test VPN if connected
netweaver security --vpn-test

echo "Audit complete!"
```

## Tips & Tricks

### Running Without sudo

On Linux, you can grant capabilities to avoid using sudo:

```bash
sudo setcap cap_net_raw,cap_net_admin=eip /usr/local/bin/netweaver
```

Now most commands work without sudo:
```bash
netweaver scan --lan  # No sudo needed!
```

### Scheduling Regular Scans

Add to crontab:
```bash
# Scan network every hour
0 * * * * /usr/local/bin/netweaver scan --lan --output /var/log/netweaver/scan-$(date +\%Y\%m\%d-\%H).json

# Daily security audit
0 2 * * * /usr/local/bin/netweaver security --all > /var/log/netweaver/security-$(date +\%Y\%m\%d).log
```

### Integrating with Scripts

NetWeaver outputs valid JSON for easy parsing:

```bash
#!/bin/bash
# Count devices on network
DEVICE_COUNT=$(netweaver scan --lan --output /tmp/scan.json && \
               jq '.responsive_hosts' /tmp/scan.json)

echo "Found $DEVICE_COUNT devices"

# Alert if count changes significantly
if [ $DEVICE_COUNT -gt 50 ]; then
    echo "Alert: Unusual number of devices detected!"
fi
```

### Optimizing for Speed

For faster scans on large networks:

```bash
# Increase threads (use with caution)
netweaver scan --target 10.0.0.0/16 --threads 500

# Scan only critical ports
netweaver scan --lan --ports 22,80,443

# Skip OS detection for speed
netweaver scan --lan --fast
```

## Troubleshooting

### "Permission denied" errors

Most features need elevated privileges:
```bash
sudo netweaver [command]
```

Or grant capabilities (Linux):
```bash
sudo setcap cap_net_raw,cap_net_admin=eip $(which netweaver)
```

### Build errors

Ensure all dependencies are installed:
```bash
# Check Rust
rustc --version

# Check C compiler
gcc --version

# Reinstall dependencies
sudo apt install build-essential pkg-config libpcap-dev
```

Then clean and rebuild:
```bash
cargo clean
cargo build --release
```

### Slow scans

- Reduce thread count: `--threads 50`
- Scan fewer ports: `--ports 22,80,443`
- Check your network connection
- Scan smaller ranges

### No devices found

- Verify network connectivity
- Check if you're on the correct interface
- Try specific target: `--target 192.168.1.0/24`
- Run with sudo for better host detection

## Next Steps

- Read the full [README.md](README.md) for comprehensive documentation
- Check [ARCHITECTURE.md](docs/ARCHITECTURE.md) to understand the internals
- Explore [examples/](examples/) for more usage patterns
- Join the community and contribute!

## Getting Help

- Run `netweaver --help` for command overview
- Run `netweaver [command] --help` for specific command help
- Check GitHub issues for known problems
- Join discussions for questions and tips

Happy networking! 🚀
