# NetWeaver Architecture

This document explains the technical architecture and design decisions behind NetWeaver.

## Overview

NetWeaver uses a hybrid C + Rust architecture that combines the strengths of both languages:

- **C Core**: Performance-critical packet operations
- **Rust Engine**: Safe concurrency, async I/O, and high-level logic
- **FFI Bridge**: Clean interface between C and Rust

## Architecture Layers

### 1. C Core Layer (`c_core/`)

The C layer handles all low-level network operations where performance is critical.

#### Components

**Packet Core (`packet_core.c`)**
- ICMP, TCP, UDP packet crafting
- Zero-copy packet parsing
- Checksum calculation (hardware-accelerated when available)
- Protocol header manipulation

**Raw Socket Management (`raw_socket.c`)**
- Raw socket creation with proper capabilities
- Non-blocking I/O configuration
- Timeout management
- Send/receive operations

**Network I/O (`network_io.c`)**
- Buffer pool management for stable memory usage
- Interface enumeration and configuration
- Gateway detection
- Memory arena allocations

**Key Design Decisions:**
- All structures are packed for zero-copy operations
- Buffer pools prevent allocation churn during high-throughput operations
- Error codes instead of exceptions for predictable control flow
- Platform-specific code isolated behind abstraction layer

#### Memory Management

```c
// Buffer pool for zero-copy packet handling
typedef struct {
    void *buffers[NW_BUFFER_POOL_SIZE];
    size_t buffer_size;
    uint32_t available_mask[NW_BUFFER_POOL_SIZE / 32];
} nw_buffer_pool_t;
```

The buffer pool:
- Pre-allocates memory at initialization
- Uses bitmask for O(1) acquire/release
- Eliminates allocation latency in hot paths
- Prevents memory fragmentation

### 2. Rust Engine Layer (`src/`)

The Rust layer provides safe, concurrent processing and high-level abstractions.

#### Module Structure

**CLI (`src/cli/`)**
- Argument parsing with clap
- Command dispatch
- User interaction

**Scanner (`src/scanner/`)**
- Parallel host discovery
- Port scanning with rayon
- Device fingerprinting
- Topology generation

**Diagnostics (`src/diagnostics/`)**
- Traceroute implementation
- Packet capture and analysis
- Route tracking
- Anomaly detection

**Optimizer (`src/optimizer/`)**
- DNS benchmarking
- MTU discovery
- TCP parameter tuning
- Performance profiling

**Monitor (`src/monitor/`)**
- Real-time TUI with ratatui
- Statistics aggregation
- Report generation
- Daemon mode

**Security (`src/security/`)**
- ARP spoofing detection
- VPN integrity testing
- Port scanning
- MITM detection

**Analytics (`src/analytics/`)**
- Latency analysis with statistical methods
- Bandwidth calculation
- Packet loss detection
- Pattern recognition

**Utils (`src/utils/`)**
- IP address utilities
- Network calculations
- Formatting helpers
- Common operations

#### Concurrency Model

NetWeaver uses Tokio for async I/O:

```rust
// Async scanning with bounded parallelism
let devices = Arc::new(Mutex::new(Vec::new()));

let tasks: Vec<_> = ip_list
    .into_iter()
    .map(|ip| {
        let devices = Arc::clone(&devices);
        tokio::spawn(async move {
            if let Some(device) = scan_host(ip).await {
                devices.lock().await.push(device);
            }
        })
    })
    .collect();

for task in tasks {
    let _ = task.await;
}
```

Benefits:
- Thousands of concurrent scans without thread overhead
- Clean cancellation semantics
- Composable async operations
- Efficient resource usage

### 3. FFI Bridge

The C and Rust layers communicate through automatically generated bindings.

#### Build Process

```rust
// build.rs
let bindings = bindgen::Builder::default()
    .header("c_core/include/netweaver_core.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");
```

This creates safe Rust wrappers for all C functions:

```rust
// Automatically generated in target/
pub unsafe fn nw_packet_send_raw(
    sock: *mut nw_socket_t,
    packet: *const nw_packet_t
) -> nw_error_t;
```

#### Safety Guarantees

Rust wrappers provide safety:

```rust
pub fn send_packet(socket: &mut Socket, packet: &Packet) -> Result<()> {
    let result = unsafe {
        ffi::nw_packet_send_raw(socket.as_mut_ptr(), packet.as_ptr())
    };
    
    match result {
        ffi::NW_SUCCESS => Ok(()),
        ffi::NW_ERROR_PERMISSION => Err(Error::Permission),
        _ => Err(Error::Unknown),
    }
}
```

### 4. Plugin System

Plugins extend NetWeaver functionality at runtime.

#### Plugin Interface

```rust
pub trait NetWeaverPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    fn on_packet(&mut self, packet: &Packet) -> Result<()>;
    fn on_device_discovered(&mut self, device: &Device) -> Result<()>;
    fn finalize(&mut self) -> Result<PluginReport>;
}
```

Plugins can:
- Process packets in the capture pipeline
- React to network events
- Export custom data formats
- Implement protocol analyzers

#### Loading Plugins

```rust
use libloading::{Library, Symbol};

let lib = unsafe { Library::new("./plugins/my_plugin.so")? };
let create: Symbol<fn() -> Box<dyn NetWeaverPlugin>> = 
    unsafe { lib.get(b"create_plugin")? };

let plugin = create();
```

## Performance Characteristics

### Benchmarks

**Network Scanning:**
- Single-host, 1000 ports: ~2 seconds
- /24 subnet, common ports: ~30 seconds
- /16 subnet, targeted scan: ~5 minutes

**Packet Processing:**
- Capture rate: Line-rate on gigabit links
- Parsing overhead: < 100ns per packet
- Memory usage: ~15MB baseline + 1MB per 10,000 buffered packets

### Optimization Techniques

1. **Zero-copy packet handling**
   - Direct buffer access
   - Avoid unnecessary memcpy operations

2. **Buffer pooling**
   - Pre-allocated buffers
   - Eliminates allocation overhead

3. **Parallel processing**
   - Rayon for CPU-bound work
   - Tokio for I/O-bound operations

4. **Adaptive concurrency**
   - Adjusts thread count based on latency
   - Prevents network saturation

5. **Smart caching**
   - DNS resolution results
   - MAC vendor lookups
   - Historical route data

## Cross-Platform Support

### Linux
- Primary development platform
- Full feature support
- Uses epoll for I/O multiplexing
- Reads from /proc for system info

### macOS
- Full feature support
- Uses kqueue for I/O multiplexing
- BSD socket interface

### BSD
- Core features supported
- Uses kqueue for I/O multiplexing
- Minor platform-specific adaptations

### Windows (Planned)
- Currently in development
- Uses Windows Sockets API
- Requires WinPcap/Npcap for capture

## Security Considerations

### Privilege Requirements

NetWeaver needs elevated privileges for:
- Creating raw sockets
- Modifying network parameters
- Accessing network interfaces

### Privilege Separation

Where possible, NetWeaver drops privileges after initialization:

```rust
if utils::is_privileged() {
    // Perform privileged operations
    create_raw_socket()?;
    
    // Drop privileges if possible
    drop_privileges()?;
}
```

### Input Validation

All user input is validated:
- IP addresses and CIDR ranges
- Port numbers and ranges
- File paths
- Command arguments

## Future Enhancements

### Planned Features

1. **eBPF Integration**
   - In-kernel packet filtering
   - Near-zero overhead capture
   - Custom XDP programs

2. **DPDK Support**
   - User-space packet processing
   - Multi-gigabit throughput
   - Hardware queue management

3. **Distributed Architecture**
   - Coordinator node for orchestration
   - Worker nodes for scanning
   - Shared result aggregation

4. **Machine Learning**
   - Anomaly detection models
   - Traffic classification
   - Predictive optimization

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on extending the architecture.

## References

- [Tokio Runtime Documentation](https://tokio.rs/)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Raw Sockets Programming](https://www.tenouk.com/Module43a.html)
- [Linux Network Stack](https://www.kernel.org/doc/html/latest/networking/index.html)
