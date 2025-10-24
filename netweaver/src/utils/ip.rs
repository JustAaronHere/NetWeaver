use std::net::Ipv4Addr;

pub fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
}

pub fn u32_to_ipv4(ip: u32) -> Ipv4Addr {
    Ipv4Addr::from(ip)
}

pub fn is_private(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    matches!(octets[0], 10 | 172 | 192)
}

pub fn is_loopback(ip: Ipv4Addr) -> bool {
    ip.is_loopback()
}

pub fn is_multicast(ip: Ipv4Addr) -> bool {
    ip.is_multicast()
}
