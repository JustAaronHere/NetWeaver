# NetWeaver

> **From packets to perfection** — The network intelligence framework that finally does it all.

NetWeaver is a high-performance, open-source network toolkit that combines the raw speed of C with the safety and elegance of Rust. It's designed for developers, system administrators, and security professionals who need deep network visibility without juggling multiple tools.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20BSD-lightgrey.svg)]()

## What Makes NetWeaver Different

Most network tools make you choose between power and usability. NetWeaver gives you both. It's a single binary that handles everything from network discovery to performance optimization, built on a hybrid architecture that leverages C for performance-critical operations and Rust for safe, concurrent processing.

**No GUI. No bloat. Just powerful network intelligence at your fingertips.**

## Features

### 🌍 Network Discovery & Mapping
- Multi-threaded LAN scanner with adaptive scheduling that automatically adjusts to network conditions
- Real-time device detection with MAC vendor identification and OS fingerprinting
- Port scanning with service detection across common and custom port ranges
- Topology visualization showing your network structure in clean ASCII or exportable formats
- Export capabilities to JSON, YAML for integration with dashboards and monitoring tools

### 🧠 Smart Diagnostics
- Advanced traceroute with multi-probe hop analysis
- Real-time packet capture and deep inspection
- Latency pattern analysis with anomaly detection
- Bandwidth measurement and trend analysis
- Historical route tracking to spot network path changes over time

### ⚡ Performance Optimization
- **Turbo Mode**: Automated network tuning that learns your usage patterns
- DNS benchmarking to find and configure the fastest resolver for your location
- MTU optimization for maximum throughput with minimal fragmentation
- TCP parameter tuning (window scaling, congestion control, fast open)
- All changes can be previewed in dry-run mode before applying

### 📊 Real-Time Monitoring
- Beautiful terminal dashboard built with ratatui for live network stats
- Protocol-specific filtering (TCP, UDP, ICMP, or see everything)
- Connection tracking showing active flows and their states
- Daemon mode for continuous background monitoring
- Exportable reports with customizable formats (JSON, YAML, HTML)

### 🔐 Security Auditing
- ARP spoofing detection to catch MITM attempts early
- VPN integrity testing including DNS leak and IPv6 leak detection
- Local port scanning to identify exposed services
- SSL/TLS certificate verification
- Comprehensive security reports with actionable recommendations

## Installation

### Prerequisites

**System Requirements:**
- Linux (kernel 3.10+), macOS (10.13+), or BSD
- Rust 1.70 or higher
- GCC or Clang for C compilation
- libpcap development headers (optional, for packet capture)

**Development Tools:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libpcap-dev

# Fedora/RHEL/CentOS
sudo dnf install gcc make pkg-config libpcap-devel

# macOS (requires Homebrew)
brew install libpcap
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/netweaver.git
cd netweaver

# Build the project (this compiles both C and Rust code)
cargo build --release

# The binary will be at target/release/netweaver
# Optionally install it system-wide
sudo cp target/release/netweaver /usr/local/bin/
```

**Note:** Some features require elevated privileges. On Linux/macOS, you may need to run NetWeaver with `sudo` or grant the binary specific capabilities:

```bash
# Grant raw socket capabilities (Linux only)
sudo setcap cap_net_raw,cap_net_admin=eip /usr/local/bin/netweaver
```

## Quick Start

### Scan Your Local Network

Discover all devices on your LAN with open port detection:

```bash
netweaver scan --lan --threads 100
```

This will:
- Automatically detect your network range (e.g., 192.168.1.0/24)
- Scan all hosts using 100 concurrent threads
- Show IP addresses, MAC addresses, vendors, and open ports
- Display approximate latency to each device

Want to scan specific ports or a custom range?

```bash
# Scan specific target with custom ports
netweaver scan --target 192.168.1.0/24 --ports 22,80,443,3306,5432

# Scan port ranges
netweaver scan --target 10.0.0.0/24 --ports 1-1024,8000-9000

# Generate network topology visualization
netweaver scan --lan --topology --output network.json
```

### Trace Routes with Intelligence

See exactly how your packets travel to their destination:

```bash
netweaver trace --target github.com
```

Each hop shows:
- IP address and hostname (when available)
- Multiple RTT measurements per hop
- Average latency and packet loss percentage
- Anomaly detection for unusual delays

Export trace data for analysis:

```bash
netweaver trace --target 8.8.8.8 --max-hops 20 --probes 5 --output trace.json
```

### Optimize Your Connection

Let NetWeaver analyze and tune your network settings:

```bash
# Dry run - see recommendations without making changes
netweaver optimize --all --dry-run

# Apply all optimizations (requires root)
sudo netweaver optimize --all

# Optimize specific components
sudo netweaver optimize --dns      # Find fastest DNS resolver
sudo netweaver optimize --mtu      # Detect optimal MTU
sudo netweaver optimize --tcp      # Tune TCP parameters

# Enable turbo mode for adaptive learning
sudo netweaver optimize --turbo
```

Turbo mode analyzes your network usage patterns and automatically applies the best configuration profiles for your workload.

### Monitor in Real-Time

Watch your network traffic live with the interactive dashboard:

```bash
# Launch real-time TUI dashboard
sudo netweaver monitor --realtime

# Monitor specific interface
sudo netweaver monitor --realtime --interface eth0

# Filter by protocol
sudo netweaver monitor --realtime --protocol tcp

# Run as background daemon with logging
sudo netweaver monitor --daemon --log /var/log/netweaver.log
```

The dashboard shows:
- Current bandwidth usage (sent/received)
- Packet counters and error rates
- Active connections with states
- Protocol distribution

Press `q` to quit the real-time monitor.

### Generate Reports

Create detailed network analysis reports:

```bash
# Generate JSON report
netweaver report --export network-report.json

# Generate HTML report with graphs
netweaver report --export report.html --format html --graphs

# Include historical data
netweaver report --export analysis.yaml --format yaml --history
```

### Deep Packet Inspection

Capture and analyze network traffic:

```bash
# Capture 1000 packets
sudo netweaver inspect --interface eth0 --count 1000

# Apply BPF filter
sudo netweaver inspect --filter "tcp port 443" --count 500

# Save capture to file
sudo netweaver inspect --output capture.pcap --count 10000

# Capture and analyze
sudo netweaver inspect --interface any --analyze
```

### Security Auditing

Run comprehensive security checks:

```bash
# Run all security checks
sudo netweaver security --all

# Individual checks
sudo netweaver security --arp-detect    # Check for ARP spoofing
netweaver security --vpn-test           # Test VPN integrity
sudo netweaver security --port-scan     # Scan for open ports
sudo netweaver security --mitm-detect   # Look for MITM indicators
```

## Command Reference

### Global Options

```
-v, --verbose    Enable verbose output with debugging information
-q, --quiet      Minimize output (errors only)
-h, --help       Show help information
-V, --version    Print version
```

### `scan` - Network Discovery

Discover and map devices on your network.

```bash
netweaver scan [OPTIONS]
```

**Options:**
- `--lan` - Automatically scan local network (detects range)
- `--target <CIDR>` - Specific target (e.g., `192.168.1.0/24`, `10.0.0.1/32`)
- `--threads <N>` - Concurrent scanning threads (default: 100)
- `--ports <LIST>` - Ports to scan (e.g., `22,80,443` or `1-1024`)
- `--topology` - Generate network topology visualization
- `--output <FILE>` - Export results to JSON/YAML

**Examples:**
```bash
netweaver scan --lan
netweaver scan --target 172.16.0.0/16 --threads 200 --ports 1-65535
netweaver scan --lan --topology --output devices.yaml
```

### `trace` - Advanced Traceroute

Trace the path packets take to reach a destination.

```bash
netweaver trace --target <HOST> [OPTIONS]
```

**Options:**
- `--target <HOST>` - Target hostname or IP (required)
- `--max-hops <N>` - Maximum TTL (default: 30)
- `--probes <N>` - Probes per hop (default: 3)
- `--history` - Show historical route data
- `--output <FILE>` - Export trace results

**Examples:**
```bash
netweaver trace --target google.com
netweaver trace --target 1.1.1.1 --max-hops 15 --probes 5
netweaver trace --target amazon.com --output trace-amazon.json
```

### `optimize` - Network Optimization

Analyze and optimize network performance.

```bash
netweaver optimize [OPTIONS]
```

**Options:**
- `--all` - Run all optimizations
- `--turbo` - Enable adaptive turbo mode
- `--dns` - Benchmark and optimize DNS resolver
- `--mtu` - Optimize MTU settings
- `--tcp` - Tune TCP parameters
- `--dry-run` - Show recommendations only (no changes)

**Examples:**
```bash
netweaver optimize --all --dry-run
sudo netweaver optimize --dns
sudo netweaver optimize --turbo
```

### `monitor` - Network Monitoring

Monitor network activity in real-time or as a daemon.

```bash
netweaver monitor [OPTIONS]
```

**Options:**
- `--realtime` - Launch interactive TUI dashboard
- `--interface <IF>` - Specific interface to monitor
- `--daemon` - Run as background service
- `--log <FILE>` - Log file path (daemon mode)
- `--protocol <PROTO>` - Filter by protocol (tcp/udp/icmp/all)

**Examples:**
```bash
sudo netweaver monitor --realtime
sudo netweaver monitor --interface wlan0 --realtime
sudo netweaver monitor --daemon --log /var/log/netweaver.log
```

### `report` - Report Generation

Generate comprehensive network analysis reports.

```bash
netweaver report --export <FILE> [OPTIONS]
```

**Options:**
- `--export <FILE>` - Output file path (required)
- `--format <FMT>` - Report format: json, yaml, html (auto-detected from extension)
- `--history` - Include historical data
- `--graphs` - Include visualizations (HTML only)

**Examples:**
```bash
netweaver report --export daily-report.json
netweaver report --export analysis.html --format html --graphs
netweaver report --export report.yaml --history
```

### `inspect` - Packet Inspection

Capture and analyze network packets.

```bash
netweaver inspect [OPTIONS]
```

**Options:**
- `--interface <IF>` - Interface to capture from
- `--filter <BPF>` - Berkeley Packet Filter expression
- `--count <N>` - Number of packets to capture
- `--output <FILE>` - Save capture to .pcap file
- `--analyze` - Perform immediate analysis

**Examples:**
```bash
sudo netweaver inspect --interface eth0 --count 1000
sudo netweaver inspect --filter "port 443" --output https-traffic.pcap
sudo netweaver inspect --analyze
```

### `security` - Security Auditing

Run security checks and vulnerability scans.

```bash
netweaver security [OPTIONS]
```

**Options:**
- `--all` - Run all security checks
- `--arp-detect` - Detect ARP spoofing attempts
- `--vpn-test` - Test VPN connection integrity
- `--port-scan` - Scan for exposed ports
- `--mitm-detect` - Detect man-in-the-middle indicators

**Examples:**
```bash
sudo netweaver security --all
sudo netweaver security --arp-detect
netweaver security --vpn-test
```

## Architecture

NetWeaver uses a hybrid architecture that plays to the strengths of both C and Rust:

### C Core Layer (`c_core/`)
Handles all performance-critical operations:
- Raw socket creation and management
- Low-level packet crafting (ICMP, TCP, UDP)
- Zero-copy packet parsing
- Hardware-accelerated checksumming
- Buffer pool management for stable memory usage

The C layer is compiled with `-O3` optimization and exposes a clean API to Rust via FFI bindings generated by `bindgen`.

### Rust Engine Layer (`src/`)
Manages high-level logic and safety:
- Async/await runtime using Tokio for concurrent operations
- Command-line interface with clap
- Real-time TUI dashboard with ratatui
- Analytics and pattern recognition
- Safe data structures with Arc/Mutex for shared state
- Serialization to JSON/YAML

### Plugin System
NetWeaver supports runtime plugin loading for extensibility:
- Write plugins in Rust or C
- Hook into packet processing pipeline
- Add custom analyzers and exporters
- Simple API for rapid development

## Performance

NetWeaver is built for speed:

- **Network scanning**: 10,000+ hosts/minute on modern hardware
- **Packet capture**: Line-rate processing on gigabit links
- **Memory footprint**: ~15MB typical RSS with buffer pools
- **Zero-copy design**: Minimal allocations in hot paths
- **Async I/O**: epoll/kqueue for scalable concurrent operations

## Security Considerations

NetWeaver requires elevated privileges for certain operations:

- **Raw sockets**: Needed for ICMP, custom packet crafting
- **Network interface access**: Required for packet capture
- **System parameter modification**: Optimization features may change sysctl values

**Best practices:**
- Use capabilities instead of full root when possible
- Review optimization changes in dry-run mode first
- Run monitoring in unprivileged mode when packet capture isn't needed
- Keep NetWeaver updated for security patches

## Troubleshooting

### "Permission denied" errors

Many NetWeaver features require root or specific capabilities:

```bash
# Option 1: Run with sudo
sudo netweaver scan --lan

# Option 2: Grant capabilities (Linux only)
sudo setcap cap_net_raw,cap_net_admin=eip /path/to/netweaver
```

### Build failures

If you encounter build errors:

```bash
# Ensure you have the latest Rust toolchain
rustup update

# Install missing dependencies
# Ubuntu/Debian:
sudo apt install build-essential pkg-config libpcap-dev

# Clean and rebuild
cargo clean
cargo build --release
```

### High CPU usage during scans

This is normal for intensive network scanning. To reduce load:

```bash
# Reduce thread count
netweaver scan --lan --threads 50

# Scan fewer ports
netweaver scan --lan --ports 22,80,443
```

## Contributing

NetWeaver is open source and we welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help makes this tool better for everyone.

**Ways to contribute:**
- Report bugs or request features via GitHub Issues
- Submit pull requests with improvements
- Write plugins or extensions
- Improve documentation
- Share your use cases and workflows

## Roadmap

Upcoming features we're excited about:

- [ ] Distributed scanning for enterprise networks
- [ ] Built-in firewall manager with AI-optimized rules
- [ ] WebAssembly-based interactive dashboard
- [ ] Integration with Prometheus, Grafana, and other observability tools
- [ ] Native Android CLI port
- [ ] BGP route analysis and monitoring
- [ ] Custom protocol analyzers

## License

NetWeaver is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

## Acknowledgments

NetWeaver builds on the shoulders of giants:
- The Rust community for exceptional tooling and libraries
- Linux kernel developers for raw socket support
- libc maintainers for stable system interfaces
- All our contributors and users

## Support

- **Documentation**: You're reading it! Check command-specific help with `netweaver <command> --help`
- **Issues**: Report bugs at https://github.com/yourusername/netweaver/issues
- **Discussions**: Join the community at https://github.com/yourusername/netweaver/discussions

---

**Built with ❤️  by the NetWeaver team. Happy networking!**
