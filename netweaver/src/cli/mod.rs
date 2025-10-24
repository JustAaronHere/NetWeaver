use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{scanner, diagnostics, optimizer, monitor, security};

#[derive(Parser)]
#[command(name = "netweaver")]
#[command(author = "NetWeaver Contributors")]
#[command(version = "0.1.0")]
#[command(about = "Hybrid C + Rust network intelligence framework", long_about = None)]
#[command(after_help = "From packets to perfection — meet the network toolkit that finally does it all.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Scan and discover network devices")]
    Scan {
        #[arg(long, help = "Scan local area network")]
        lan: bool,

        #[arg(short, long, help = "Target IP or CIDR range")]
        target: Option<String>,

        #[arg(short, long, default_value = "100", help = "Number of concurrent threads")]
        threads: usize,

        #[arg(long, help = "Scan specific ports (comma-separated)")]
        ports: Option<String>,

        #[arg(short, long, help = "Export results to file (JSON/YAML)")]
        output: Option<String>,

        #[arg(long, help = "Generate network topology visualization")]
        topology: bool,
    },

    #[command(about = "Trace route to target with advanced analytics")]
    Trace {
        #[arg(short, long, help = "Target hostname or IP")]
        target: String,

        #[arg(short, long, default_value = "30", help = "Maximum TTL hops")]
        max_hops: u8,

        #[arg(short, long, default_value = "3", help = "Number of probes per hop")]
        probes: u8,

        #[arg(long, help = "Show historical route data")]
        history: bool,

        #[arg(short, long, help = "Export trace data")]
        output: Option<String>,
    },

    #[command(about = "Optimize network performance")]
    Optimize {
        #[arg(long, help = "Enable turbo mode with auto-tuning")]
        turbo: bool,

        #[arg(long, help = "Benchmark and select fastest DNS resolver")]
        dns: bool,

        #[arg(long, help = "Optimize MTU settings")]
        mtu: bool,

        #[arg(long, help = "Tune TCP window parameters")]
        tcp: bool,

        #[arg(long, help = "Run all optimizations")]
        all: bool,

        #[arg(short, long, help = "Dry run - show recommendations only")]
        dry_run: bool,
    },

    #[command(about = "Monitor network in real-time")]
    Monitor {
        #[arg(long, help = "Enable real-time TUI dashboard")]
        realtime: bool,

        #[arg(short, long, help = "Interface to monitor")]
        interface: Option<String>,

        #[arg(long, help = "Run as background daemon")]
        daemon: bool,

        #[arg(short, long, help = "Log file path")]
        log: Option<String>,

        #[arg(long, help = "Monitor specific protocol (tcp/udp/icmp/all)")]
        protocol: Option<String>,
    },

    #[command(about = "Generate network analysis report")]
    Report {
        #[arg(short, long, help = "Export report to file")]
        export: String,

        #[arg(short, long, help = "Report format (json/yaml/html)")]
        format: Option<String>,

        #[arg(long, help = "Include historical data")]
        history: bool,

        #[arg(long, help = "Include graphs and visualizations")]
        graphs: bool,
    },

    #[command(about = "Deep packet inspection and diagnostics")]
    Inspect {
        #[arg(short, long, help = "Interface to capture from")]
        interface: Option<String>,

        #[arg(short, long, help = "Capture filter (BPF syntax)")]
        filter: Option<String>,

        #[arg(short, long, help = "Number of packets to capture")]
        count: Option<usize>,

        #[arg(short, long, help = "Save capture to file (.pcap)")]
        output: Option<String>,

        #[arg(long, help = "Analyze captured packets")]
        analyze: bool,
    },

    #[command(about = "Security auditing and monitoring")]
    Security {
        #[arg(long, help = "Detect ARP spoofing attempts")]
        arp_detect: bool,

        #[arg(long, help = "Test VPN connection integrity")]
        vpn_test: bool,

        #[arg(long, help = "Scan for open ports on local network")]
        port_scan: bool,

        #[arg(long, help = "Monitor for MITM attempts")]
        mitm_detect: bool,

        #[arg(long, help = "Run all security checks")]
        all: bool,
    },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { lan, target, threads, ports, output, topology } => {
            scanner::run_scan(lan, target, threads, ports, output, topology).await?;
        }
        Commands::Trace { target, max_hops, probes, history, output } => {
            diagnostics::run_trace(target, max_hops, probes, history, output).await?;
        }
        Commands::Optimize { turbo, dns, mtu, tcp, all, dry_run } => {
            optimizer::run_optimize(turbo, dns, mtu, tcp, all, dry_run).await?;
        }
        Commands::Monitor { realtime, interface, daemon, log, protocol } => {
            monitor::run_monitor(realtime, interface, daemon, log, protocol).await?;
        }
        Commands::Report { export, format, history, graphs } => {
            monitor::generate_report(export, format, history, graphs).await?;
        }
        Commands::Inspect { interface, filter, count, output, analyze } => {
            diagnostics::run_inspect(interface, filter, count, output, analyze).await?;
        }
        Commands::Security { arp_detect, vpn_test, port_scan, mitm_detect, all } => {
            security::run_security_audit(arp_detect, vpn_test, port_scan, mitm_detect, all).await?;
        }
    }

    Ok(())
}
