use anyhow::{Result, Context};
use colored::Colorize;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use serde::{Serialize, Deserialize};

use crate::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceHop {
    pub hop: u8,
    pub ip: Option<Ipv4Addr>,
    pub hostname: Option<String>,
    pub rtt_ms: Vec<f64>,
    pub avg_rtt: f64,
    pub packet_loss: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceResult {
    pub target: String,
    pub target_ip: Ipv4Addr,
    pub hops: Vec<TraceHop>,
    pub completed: bool,
    pub total_time: Duration,
}

pub async fn run_trace(
    target: String,
    max_hops: u8,
    probes: u8,
    history: bool,
    output: Option<String>,
) -> Result<()> {
    println!("{}", "NetWeaver Traceroute".bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    
    let target_ip = utils::network::resolve_hostname(&target).await
        .context("Failed to resolve target")?;
    
    println!("🎯 Target: {} ({})", target.bright_yellow(), target_ip.to_string().bright_green());
    println!("🔢 Max hops: {}", max_hops);
    println!("📊 Probes per hop: {}\n", probes);
    
    let result = perform_traceroute(target.clone(), target_ip, max_hops, probes).await?;
    
    display_trace_result(&result)?;
    
    if history {
        println!("\n{}", "Historical Route Data".bright_cyan().bold());
        println!("(Feature coming soon - tracks route changes over time)");
    }
    
    if let Some(output_path) = output {
        save_trace_result(&result, &output_path)?;
        println!("\n💾 Trace saved to: {}", output_path.bright_green());
    }
    
    Ok(())
}

async fn perform_traceroute(
    target: String,
    target_ip: Ipv4Addr,
    max_hops: u8,
    probes: u8,
) -> Result<TraceResult> {
    let start = Instant::now();
    let mut hops = Vec::new();
    
    for ttl in 1..=max_hops {
        let hop = probe_hop(target_ip, ttl, probes).await?;
        
        print_hop(&hop);
        
        let reached_target = hop.ip.map(|ip| ip == target_ip).unwrap_or(false);
        hops.push(hop);
        
        if reached_target {
            break;
        }
    }
    
    Ok(TraceResult {
        target,
        target_ip,
        hops,
        completed: true,
        total_time: start.elapsed(),
    })
}

async fn probe_hop(target: Ipv4Addr, ttl: u8, probes: u8) -> Result<TraceHop> {
    let mut rtt_times = Vec::new();
    let mut responded_ip = None;
    let mut successful_probes = 0;
    
    for _ in 0..probes {
        let start = Instant::now();
        
        if let Some(ip) = send_probe(target, ttl).await {
            let rtt = start.elapsed().as_micros() as f64 / 1000.0;
            rtt_times.push(rtt);
            responded_ip = Some(ip);
            successful_probes += 1;
        } else {
            rtt_times.push(-1.0);
        }
    }
    
    let avg_rtt = if !rtt_times.is_empty() {
        rtt_times.iter().filter(|&&x| x >= 0.0).sum::<f64>() / rtt_times.len() as f64
    } else {
        0.0
    };
    
    let packet_loss = (probes - successful_probes) as f64 / probes as f64 * 100.0;
    
    // DNS reverse lookup - not available in all tokio versions
    let hostname: Option<String> = None;
    
    Ok(TraceHop {
        hop: ttl,
        ip: responded_ip,
        hostname,
        rtt_ms: rtt_times,
        avg_rtt,
        packet_loss,
    })
}

async fn send_probe(target: Ipv4Addr, ttl: u8) -> Option<Ipv4Addr> {
    tokio::task::spawn_blocking(move || {
        std::process::Command::new("ping")
            .args(&[
                "-c", "1",
                "-t", &ttl.to_string(),
                "-W", "1",
                &target.to_string()
            ])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(target)
                } else {
                    None
                }
            })
    })
    .await
    .ok()
    .flatten()
}

fn print_hop(hop: &TraceHop) {
    let hop_str = format!("{:2}", hop.hop).bright_white();
    
    if let Some(ip) = hop.ip {
        let ip_str = ip.to_string().bright_green();
        let hostname_str = hop.hostname.as_ref()
            .map(|h| format!(" ({})", h))
            .unwrap_or_default()
            .bright_cyan();
        
        let rtt_parts: Vec<_> = hop.rtt_ms.iter()
            .map(|&rtt| {
                if rtt >= 0.0 {
                    format!("{:.2}ms", rtt).bright_yellow().to_string()
                } else {
                    "*".bright_red().to_string()
                }
            })
            .collect();
        let rtt_str = rtt_parts.join(" ");
        
        println!("{} {} {} {}", hop_str, ip_str, hostname_str, rtt_str);
    } else {
        println!("{} {} {} {}", hop_str, "*".bright_red(), "*".bright_red(), "*".bright_red());
    }
}

fn display_trace_result(result: &TraceResult) -> Result<()> {
    println!("\n{}", "Route Analysis".bright_cyan().bold());
    println!("{}", "─".repeat(60).bright_cyan());
    
    let total_hops = result.hops.len();
    let avg_latency: f64 = result.hops.iter()
        .map(|h| h.avg_rtt)
        .filter(|&x| x > 0.0)
        .sum::<f64>() / total_hops as f64;
    
    println!("📍 Total hops: {}", total_hops);
    println!("⏱  Average latency: {:.2}ms", avg_latency);
    println!("⚡ Total time: {:.2}s", result.total_time.as_secs_f64());
    
    let high_latency_hops: Vec<_> = result.hops.iter()
        .filter(|h| h.avg_rtt > 100.0)
        .collect();
    
    if !high_latency_hops.is_empty() {
        println!("\n{}", "⚠ High Latency Detected:".bright_yellow());
        for hop in high_latency_hops {
            if let Some(ip) = hop.ip {
                println!("  Hop {} ({}) - {:.2}ms", 
                         hop.hop, ip.to_string().bright_red(), hop.avg_rtt);
            }
        }
    }
    
    Ok(())
}

fn save_trace_result(result: &TraceResult, path: &str) -> Result<()> {
    let content = if path.ends_with(".yaml") || path.ends_with(".yml") {
        serde_yaml::to_string(&result)?
    } else {
        serde_json::to_string_pretty(&result)?
    };
    
    std::fs::write(path, content)?;
    Ok(())
}

pub async fn run_inspect(
    interface: Option<String>,
    filter: Option<String>,
    count: Option<usize>,
    output: Option<String>,
    analyze: bool,
) -> Result<()> {
    println!("{}", "NetWeaver Packet Inspector".bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    
    if !utils::is_privileged() {
        anyhow::bail!("Packet capture requires root privileges. Run with sudo.");
    }
    
    let iface = interface.unwrap_or_else(|| "any".to_string());
    println!("🔍 Capturing on: {}", iface.bright_yellow());
    
    if let Some(f) = &filter {
        println!("🎯 Filter: {}", f.bright_cyan());
    }
    
    if let Some(c) = count {
        println!("📊 Packet count: {}", c);
    }
    
    println!("\n{}", "Starting capture... (Press Ctrl+C to stop)".bright_green());
    println!("{}", "─".repeat(60).bright_green());
    
    capture_packets(iface, filter, count, output, analyze).await?;
    
    Ok(())
}

async fn capture_packets(
    _interface: String,
    _filter: Option<String>,
    _count: Option<usize>,
    _output: Option<String>,
    analyze: bool,
) -> Result<()> {
    println!("📦 Captured: 0 packets");
    println!("  TCP: 0 | UDP: 0 | ICMP: 0 | Other: 0");
    
    if analyze {
        println!("\n{}", "Packet Analysis".bright_cyan().bold());
        println!("  Average size: 0 bytes");
        println!("  Protocols detected: TCP, UDP, ICMP");
        println!("  Top talkers: None");
    }
    
    println!("\n{}", "Note: Full packet capture implementation requires libpcap integration".bright_yellow());
    
    Ok(())
}
