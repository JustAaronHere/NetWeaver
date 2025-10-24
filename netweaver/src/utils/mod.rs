use std::net::{IpAddr, Ipv4Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};

pub mod ip;
pub mod network;

pub fn get_timestamp_us() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

pub fn format_bandwidth(bytes_per_sec: f64) -> String {
    const UNITS: &[&str] = &["B/s", "KB/s", "MB/s", "GB/s"];
    let mut value = bytes_per_sec;
    let mut unit_idx = 0;
    
    while value >= 1024.0 && unit_idx < UNITS.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }
    
    format!("{:.2} {}", value, UNITS[unit_idx])
}

pub fn format_latency(us: f64) -> String {
    if us < 1000.0 {
        format!("{:.2} μs", us)
    } else if us < 1_000_000.0 {
        format!("{:.2} ms", us / 1000.0)
    } else {
        format!("{:.2} s", us / 1_000_000.0)
    }
}

pub fn parse_cidr(cidr: &str) -> Result<(Ipv4Addr, u8)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid CIDR format");
    }
    
    let ip: Ipv4Addr = parts[0].parse().context("Invalid IP address")?;
    let prefix: u8 = parts[1].parse().context("Invalid prefix length")?;
    
    if prefix > 32 {
        anyhow::bail!("Prefix length must be <= 32");
    }
    
    Ok((ip, prefix))
}

pub fn cidr_to_range(ip: Ipv4Addr, prefix: u8) -> Vec<Ipv4Addr> {
    let ip_num = u32::from(ip);
    let mask = !0u32 << (32 - prefix);
    let network = ip_num & mask;
    let broadcast = network | !mask;
    
    (network + 1..broadcast)
        .map(Ipv4Addr::from)
        .collect()
}

pub fn get_local_ip() -> Result<Ipv4Addr> {
    local_ip_address::local_ip()
        .context("Failed to get local IP")
        .and_then(|ip| match ip {
            IpAddr::V4(ipv4) => Ok(ipv4),
            _ => anyhow::bail!("Only IPv4 supported"),
        })
}

pub fn is_privileged() -> bool {
    unsafe { libc::geteuid() == 0 }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    pub fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }
    
    pub fn to_string(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
    
    pub fn vendor(&self) -> &'static str {
        match (self.0[0], self.0[1], self.0[2]) {
            (0x00, 0x50, 0x56) => "VMware",
            (0x00, 0x0c, 0x29) => "VMware",
            (0x08, 0x00, 0x27) => "VirtualBox",
            (0x52, 0x54, 0x00) => "QEMU/KVM",
            (0x00, 0x1c, 0x42) => "Parallels",
            (0xdc, 0xa6, 0x32) => "Raspberry Pi",
            (0xb8, 0x27, 0xeb) => "Raspberry Pi",
            (0xf0, 0x18, 0x98) => "Apple",
            (0x00, 0x1b, 0x63) => "Apple",
            _ => "Unknown",
        }
    }
}
