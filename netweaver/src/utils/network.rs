use std::net::Ipv4Addr;
use std::time::Duration;
use anyhow::Result;

pub async fn resolve_hostname(hostname: &str) -> Result<Ipv4Addr> {
    use hickory_resolver::TokioAsyncResolver;
    use hickory_resolver::config::*;
    
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );
    
    let response = resolver.ipv4_lookup(hostname).await?;
    response
        .iter()
        .next()
        .map(|ip| ip.0)
        .ok_or_else(|| anyhow::anyhow!("No IPv4 address found"))
}

pub fn is_port_in_range(port: u16, range: &str) -> bool {
    if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                return port >= start && port <= end;
            }
        }
    } else if let Ok(single_port) = range.parse::<u16>() {
        return port == single_port;
    }
    false
}

pub fn parse_port_list(port_str: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    
    for part in port_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (range[0].parse::<u16>(), range[1].parse::<u16>()) {
                    ports.extend(start..=end);
                }
            }
        } else if let Ok(port) = part.parse::<u16>() {
            ports.push(port);
        }
    }
    
    ports
}

pub const COMMON_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 3306, 3389, 5432, 5900, 8080, 8443,
];

pub fn calculate_adaptive_timeout(rtt_avg: Duration) -> Duration {
    rtt_avg.mul_f64(2.5).max(Duration::from_millis(100))
}
