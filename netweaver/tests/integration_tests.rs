// Integration tests for NetWeaver
// Tests the complete workflow of the network intelligence framework

use netweaver_lib::ffi;
use std::net::Ipv4Addr;

#[test]
fn test_nw_init_and_cleanup() {
    unsafe {
        let result = ffi::nw_init();
        assert_eq!(result, ffi::nw_error_t_NW_SUCCESS);
        ffi::nw_cleanup();
    }
}

#[test]
fn test_checksum_calculation() {
    let test_data: [u8; 8] = [0x45, 0x00, 0x00, 0x3c, 0x1c, 0x46, 0x40, 0x00];
    
    unsafe {
        let checksum = ffi::nw_checksum(
            test_data.as_ptr() as *const std::ffi::c_void,
            test_data.len(),
        );
        
        // Checksum should be non-zero for this data
        assert_ne!(checksum, 0);
    }
}

#[test]
fn test_ip_string_conversion() {
    let test_ip = "192.168.1.100";
    let expected: u32 = (192 << 24) | (168 << 16) | (1 << 8) | 100;
    
    unsafe {
        let ip_str = std::ffi::CString::new(test_ip).unwrap();
        let ip_int = ffi::nw_ip_str_to_int(ip_str.as_ptr());
        assert_eq!(ip_int, expected);
        
        // Convert back
        let mut buffer = [0u8; 16];
        ffi::nw_ip_int_to_str(ip_int, buffer.as_mut_ptr() as *mut i8, buffer.len());
        
        let result_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8)
            .to_str()
            .unwrap();
        assert_eq!(result_str, test_ip);
    }
}

#[test]
fn test_icmp_packet_crafting() {
    let mut packet: ffi::nw_packet_t = unsafe { std::mem::zeroed() };
    let dst_ip: u32 = (8 << 24) | (8 << 16) | (8 << 8) | 8; // 8.8.8.8
    let id: u16 = 1234;
    let seq: u16 = 1;
    
    unsafe {
        let result = ffi::nw_packet_craft_icmp_echo(&mut packet, dst_ip, id, seq);
        assert_eq!(result, ffi::nw_error_t_NW_SUCCESS);
        assert!(packet.length > 0);
        assert_eq!(packet.protocol, ffi::nw_protocol_t_NW_PROTO_ICMP as u8);
        assert_eq!(packet.dst_ip, dst_ip);
    }
}

#[test]
fn test_tcp_syn_packet_crafting() {
    let mut packet: ffi::nw_packet_t = unsafe { std::mem::zeroed() };
    let src_ip: u32 = (192 << 24) | (168 << 16) | (1 << 8) | 100;
    let dst_ip: u32 = (8 << 24) | (8 << 16) | (8 << 8) | 8;
    let src_port: u16 = 54321;
    let dst_port: u16 = 443;
    
    unsafe {
        let result = ffi::nw_packet_craft_tcp_syn(
            &mut packet,
            src_ip,
            dst_ip,
            src_port,
            dst_port,
        );
        assert_eq!(result, ffi::nw_error_t_NW_SUCCESS);
        assert!(packet.length > 0);
        assert_eq!(packet.protocol, ffi::nw_protocol_t_NW_PROTO_TCP as u8);
        assert_eq!(packet.src_ip, src_ip);
        assert_eq!(packet.dst_ip, dst_ip);
        assert_eq!(packet.src_port, src_port);
        assert_eq!(packet.dst_port, dst_port);
    }
}

#[test]
fn test_udp_packet_crafting() {
    let mut packet: ffi::nw_packet_t = unsafe { std::mem::zeroed() };
    let src_ip: u32 = (192 << 24) | (168 << 16) | (1 << 8) | 100;
    let dst_ip: u32 = (8 << 24) | (8 << 16) | (8 << 8) | 8;
    let src_port: u16 = 54321;
    let dst_port: u16 = 53;
    let payload = b"test payload";
    
    unsafe {
        let result = ffi::nw_packet_craft_udp(
            &mut packet,
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            payload.as_ptr(),
            payload.len(),
        );
        assert_eq!(result, ffi::nw_error_t_NW_SUCCESS);
        assert!(packet.length > 0);
        assert_eq!(packet.protocol, ffi::nw_protocol_t_NW_PROTO_UDP as u8);
    }
}

#[test]
fn test_buffer_pool() {
    let mut pool: ffi::nw_buffer_pool_t = unsafe { std::mem::zeroed() };
    let buffer_size: usize = 1024;
    let pool_size: usize = 10;
    
    unsafe {
        // Initialize buffer pool
        let result = ffi::nw_buffer_pool_init(&mut pool, buffer_size, pool_size);
        assert_eq!(result, ffi::nw_error_t_NW_SUCCESS);
        
        // Acquire buffers
        let buf1 = ffi::nw_buffer_pool_acquire(&mut pool);
        assert!(!buf1.is_null());
        
        let buf2 = ffi::nw_buffer_pool_acquire(&mut pool);
        assert!(!buf2.is_null());
        
        // Buffers should be different
        assert_ne!(buf1, buf2);
        
        // Release buffers
        ffi::nw_buffer_pool_release(&mut pool, buf1);
        ffi::nw_buffer_pool_release(&mut pool, buf2);
        
        // Should be able to acquire again
        let buf3 = ffi::nw_buffer_pool_acquire(&mut pool);
        assert!(!buf3.is_null());
        
        ffi::nw_buffer_pool_release(&mut pool, buf3);
        ffi::nw_buffer_pool_destroy(&mut pool);
    }
}

#[test]
fn test_timestamp() {
    unsafe {
        let ts1 = ffi::nw_timestamp_us();
        std::thread::sleep(std::time::Duration::from_micros(100));
        let ts2 = ffi::nw_timestamp_us();
        
        // Second timestamp should be greater
        assert!(ts2 > ts1);
        
        // Difference should be roughly 100 microseconds (with some tolerance)
        let diff = ts2 - ts1;
        assert!(diff >= 100);
        assert!(diff < 10000); // Should be less than 10ms
    }
}

#[test]
fn test_packet_validation() {
    let mut packet: ffi::nw_packet_t = unsafe { std::mem::zeroed() };
    let dst_ip: u32 = (8 << 24) | (8 << 16) | (8 << 8) | 8;
    
    unsafe {
        // Create a valid ICMP packet
        let result = ffi::nw_packet_craft_icmp_echo(&mut packet, dst_ip, 1, 1);
        assert_eq!(result, ffi::nw_error_t_NW_SUCCESS);
        
        // Validate the packet
        let is_valid = ffi::nw_packet_validate(&packet);
        assert!(is_valid);
        
        // Test with invalid packet (zero length)
        let mut invalid_packet: ffi::nw_packet_t = unsafe { std::mem::zeroed() };
        invalid_packet.length = 0;
        let is_valid = ffi::nw_packet_validate(&invalid_packet);
        assert!(!is_valid);
    }
}

mod utils_tests {
    use netweaver_lib::utils;
    use std::net::Ipv4Addr;
    
    #[test]
    fn test_cidr_parsing() {
        let result = utils::parse_cidr("192.168.1.0/24");
        assert!(result.is_ok());
        
        let (ip, prefix) = result.unwrap();
        assert_eq!(ip, "192.168.1.0".parse::<Ipv4Addr>().unwrap());
        assert_eq!(prefix, 24);
    }
    
    #[test]
    fn test_cidr_to_range() {
        let ip = "192.168.1.0".parse::<Ipv4Addr>().unwrap();
        let range = utils::cidr_to_range(ip, 30);
        
        // /30 gives us 4 IPs total, but we exclude network and broadcast
        // So we should get 2 usable IPs
        assert_eq!(range.len(), 2);
        assert_eq!(range[0], "192.168.1.1".parse::<Ipv4Addr>().unwrap());
        assert_eq!(range[1], "192.168.1.2".parse::<Ipv4Addr>().unwrap());
    }
    
    #[test]
    fn test_bandwidth_formatting() {
        assert_eq!(utils::format_bandwidth(100.0), "100.00 B/s");
        assert_eq!(utils::format_bandwidth(1024.0), "1.00 KB/s");
        assert_eq!(utils::format_bandwidth(1024.0 * 1024.0), "1.00 MB/s");
        assert_eq!(utils::format_bandwidth(1024.0 * 1024.0 * 1024.0), "1.00 GB/s");
    }
    
    #[test]
    fn test_mac_address() {
        let mac = utils::MacAddress::new([0x00, 0x50, 0x56, 0xc0, 0x00, 0x08]);
        assert_eq!(mac.to_string(), "00:50:56:c0:00:08");
        assert_eq!(mac.vendor(), "VMware");
        
        let mac2 = utils::MacAddress::new([0xf0, 0x18, 0x98, 0x12, 0x34, 0x56]);
        assert_eq!(mac2.vendor(), "Apple");
    }
}

mod analytics_tests {
    use netweaver_lib::analytics::{LatencyAnalyzer, BandwidthAnalyzer, PacketLossDetector};
    use std::time::Duration;
    
    #[test]
    fn test_latency_analyzer() {
        let mut analyzer = LatencyAnalyzer::new(100);
        
        // Add some samples
        analyzer.add_sample(10.0);
        analyzer.add_sample(15.0);
        analyzer.add_sample(12.0);
        analyzer.add_sample(20.0);
        
        assert_eq!(analyzer.average(), 14.25);
        assert_eq!(analyzer.min(), 10.0);
        assert_eq!(analyzer.max(), 20.0);
        
        // Jitter should be non-zero with varying samples
        assert!(analyzer.jitter() > 0.0);
    }
    
    #[test]
    fn test_latency_anomaly_detection() {
        let mut analyzer = LatencyAnalyzer::new(100);
        
        // Add consistent samples
        for _ in 0..10 {
            analyzer.add_sample(10.0);
        }
        
        // No anomaly with consistent data
        assert!(!analyzer.detect_anomaly(3.0));
        
        // Add a spike
        analyzer.add_sample(100.0);
        
        // Should detect anomaly
        assert!(analyzer.detect_anomaly(3.0));
    }
    
    #[test]
    fn test_bandwidth_analyzer() {
        let mut analyzer = BandwidthAnalyzer::new(Duration::from_secs(1));
        
        analyzer.add_measurement(1000);
        std::thread::sleep(Duration::from_millis(100));
        analyzer.add_measurement(2000);
        
        let bandwidth = analyzer.current_bandwidth();
        // Should have some measured bandwidth
        assert!(bandwidth > 0.0);
    }
    
    #[test]
    fn test_packet_loss_detector() {
        let mut detector = PacketLossDetector::new();
        
        detector.record_sent(100);
        detector.record_received(95);
        
        // 5% loss
        assert_eq!(detector.loss_rate(), 5.0);
        
        detector.reset();
        assert_eq!(detector.loss_rate(), 0.0);
    }
}

mod network_utils_tests {
    use netweaver_lib::utils::network;
    
    #[test]
    fn test_port_list_parsing() {
        let ports = network::parse_port_list("80,443,8080-8083");
        assert_eq!(ports.len(), 6);
        assert!(ports.contains(&80));
        assert!(ports.contains(&443));
        assert!(ports.contains(&8080));
        assert!(ports.contains(&8081));
        assert!(ports.contains(&8082));
        assert!(ports.contains(&8083));
    }
    
    #[test]
    fn test_port_in_range() {
        assert!(network::is_port_in_range(8080, "8080-8090"));
        assert!(network::is_port_in_range(8085, "8080-8090"));
        assert!(!network::is_port_in_range(9000, "8080-8090"));
        
        assert!(network::is_port_in_range(80, "80"));
        assert!(!network::is_port_in_range(443, "80"));
    }
    
    #[test]
    fn test_common_ports() {
        assert!(network::COMMON_PORTS.contains(&80));
        assert!(network::COMMON_PORTS.contains(&443));
        assert!(network::COMMON_PORTS.contains(&22));
        assert!(network::COMMON_PORTS.contains(&3306));
    }
}
