# NetWeaver

> **From packets to perfection** — The network intelligence framework that finally does it all.

NetWeaver is a high-performance, open-source network toolkit built for people who actually care about how their networks behave. It merges the raw performance of C with the reliability and concurrency of Rust, creating a hybrid system that’s both fast and safe.

It’s designed for developers, system administrators, and security professionals who want deep visibility into their networks — without juggling a dozen separate tools.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20BSD-lightgrey.svg)]()

---

## What Makes NetWeaver Different

Most network tools force a tradeoff: power or usability. NetWeaver gives you both. It’s a single binary that does it all — from mapping and diagnostics to optimization and security auditing. The hybrid C/Rust core means you get low-level control without sacrificing safety or concurrency.

**No GUI. No unnecessary layers. Just a fast, intelligent command-line tool that gets things done.**

---

## Features

### Network Discovery & Mapping

* Multi-threaded LAN scanning with adaptive scheduling tuned to network conditions
* Real-time device detection, MAC vendor resolution, and OS fingerprinting
* Port scanning with service detection across both standard and custom ranges
* Topology visualization in clean ASCII or exportable formats
* JSON and YAML exports for easy dashboard or monitoring integration

### Smart Diagnostics

* Advanced traceroute with multi-probe hop analysis
* Real-time packet capture and deep inspection
* Latency trend analysis and anomaly detection
* Bandwidth measurement and performance tracking over time
* Historical route comparison for detecting path shifts

### Performance Optimization

* **Turbo Mode**: Automated network tuning that learns from your activity
* DNS benchmarking for selecting the fastest resolver per region
* MTU calibration to minimize fragmentation and boost throughput
* TCP tuning with dynamic window scaling and congestion control optimization
* Full dry-run mode to preview recommended changes before applying

### Real-Time Monitoring

* Terminal dashboard powered by ratatui, showing live network stats
* Protocol-based filtering (TCP, UDP, ICMP, or full view)
* Connection tracking with state visualization
* Daemon mode for continuous background monitoring
* Exportable logs and reports in JSON, YAML, or HTML formats

### Security Auditing

* ARP spoofing and MITM detection
* VPN leak and integrity testing
* Local port exposure scan
* SSL/TLS certificate verification
* Detailed reports with remediation advice

---

## Installation

### Prerequisites

**System Requirements**

* Linux (kernel 3.10+), macOS (10.13+), or BSD
* Rust 1.70 or newer
* GCC or Clang for compiling C components
* libpcap development headers (for packet capture)

**Install Tools and Dependencies**

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libpcap-dev

# Fedora/RHEL/CentOS
sudo dnf install gcc make pkg-config libpcap-devel

# macOS (Homebrew)
brew install libpcap
```

### Build from Source

```bash
git clone https://github.com/yourusername/netweaver.git
cd netweaver
cargo build --release

# Optional: install globally
sudo cp target/release/netweaver /usr/local/bin/
```

Some features require elevated privileges. To grant them safely:

```bash
sudo setcap cap_net_raw,cap_net_admin=eip /usr/local/bin/netweaver
```

---

## Quick Start

### Scan Your Local Network

```bash
netweaver scan --lan --threads 100
```

This scans your local subnet, detects devices, and identifies open ports with latency data.

Scan specific ports or ranges:

```bash
netweaver scan --target 192.168.1.0/24 --ports 22,80,443,3306,5432
netweaver scan --target 10.0.0.0/24 --ports 1-1024,8000-9000
```

Generate a topology map:

```bash
netweaver scan --lan --topology --output network.json
```

### Traceroute Intelligence

```bash
netweaver trace --target github.com
```

Displays detailed hop-by-hop data, latency averages, and packet loss metrics.

Export trace data:

```bash
netweaver trace --target 8.8.8.8 --max-hops 20 --probes 5 --output trace.json
```

### Optimize Your Network

```bash
netweaver optimize --all --dry-run
sudo netweaver optimize --all
sudo netweaver optimize --dns
sudo netweaver optimize --turbo
```

Turbo Mode continuously learns and adjusts parameters for your network profile.

### Monitor in Real Time

```bash
sudo netweaver monitor --realtime
sudo netweaver monitor --interface eth0 --protocol tcp
sudo netweaver monitor --daemon --log /var/log/netweaver.log
```

### Generate Reports

```bash
netweaver report --export report.html --format html --graphs
netweaver report --export analysis.yaml --history
```

### Deep Packet Inspection

```bash
sudo netweaver inspect --interface eth0 --count 1000
sudo netweaver inspect --filter "tcp port 443" --output capture.pcap
sudo netweaver inspect --analyze
```

### Security Checks

```bash
sudo netweaver security --all
sudo netweaver security --arp-detect
netweaver security --vpn-test
```

---

## Architecture

### C Core Layer

Handles everything performance-sensitive:

* Raw sockets, custom packet crafting, and fast parsing
* Hardware-assisted checksum calculations
* Efficient buffer management and zero-copy design
* Compiled with `-O3` optimizations, exposed via FFI bindings

### Rust Engine Layer

Responsible for:

* Async runtime using Tokio
* Command-line handling with clap
* TUI dashboard with ratatui
* Data serialization, analytics, and reporting
* Safe concurrency via Arc/Mutex primitives

### Plugin System

Developers can extend NetWeaver by writing plugins in either Rust or C.

* Integrate with the packet pipeline
* Build custom analyzers, exporters, or monitors
* Simple API for extension and experimentation

---

## Performance

* Scans over 10,000 hosts per minute on modern systems
* Processes packets at line rate on gigabit links
* Maintains a small memory footprint (~15MB RSS)
* Uses async I/O and zero-copy pipelines for stability under load

---

## Security Considerations

Some operations require elevated privileges:

* Raw socket access
* Packet capture
* Network parameter adjustments

**Recommendations**

* Use `setcap` instead of full root privileges
* Run in dry-run mode before applying changes
* Keep the binary updated to the latest version

---

## Troubleshooting

### Permission Errors

```bash
sudo netweaver scan --lan
sudo setcap cap_net_raw,cap_net_admin=eip /path/to/netweaver
```

### Build Issues

```bash
rustup update
sudo apt install build-essential pkg-config libpcap-dev
cargo clean && cargo build --release
```

### High CPU Usage

Reduce threads or scope:

```bash
netweaver scan --lan --threads 50
netweaver scan --ports 22,80,443
```

---

## Contributing

Contributions are welcome — whether you’re fixing a bug, optimizing code, or improving docs.

**Ways to contribute**

* Report issues or suggest features
* Submit PRs for code, docs, or performance improvements
* Build and share custom plugins
* Help test across platforms

---

## Roadmap

* Distributed network scanning
* AI-driven firewall management
* WebAssembly-based live dashboard
* Integration with Prometheus and Grafana
* Android command-line port
* BGP route analysis
* Custom protocol analyzers

---

## License

NetWeaver is dual-licensed under:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

You may use either.

---

## Acknowledgments

NetWeaver stands on the work of:

* The Rust community for their exceptional ecosystem
* The Linux kernel developers for raw socket and epoll support
* libc maintainers for their foundational stability
* Every contributor who made this project stronger

---

## Support

* **Docs**: Use `netweaver <command> --help` for command details
* **Issues**: Report problems or suggestions at
  [https://github.com/JustAaronHere/netweaver/issues](https://github.com/JustAaronHere/netweaver/issues)

---

**Built with care by the Alacran team.**
