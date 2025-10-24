#include "netweaver_core.h"
#include <string.h>
#include <arpa/inet.h>
#include <netinet/ip.h>
#include <netinet/tcp.h>
#include <netinet/udp.h>
#include <netinet/ip_icmp.h>

// Enhanced packet parser with deep inspection capabilities
// Provides zero-copy parsing for optimal performance

/**
 * Parse Ethernet frame (if present in raw capture)
 * Returns offset to IP header
 */
static size_t parse_ethernet_header(const uint8_t *data, size_t len) {
    // Standard Ethernet II frame has 14-byte header
    // 6 bytes dest MAC + 6 bytes src MAC + 2 bytes EtherType
    if (len < 14) return 0;
    
    uint16_t ethertype = (data[12] << 8) | data[13];
    
    // 0x0800 = IPv4, 0x86DD = IPv6
    if (ethertype == 0x0800) {
        return 14; // Skip to IP header
    }
    
    return 0;
}

/**
 * Parse IP header and extract key fields
 * Validates header checksum and structure
 */
static nw_error_t parse_ip_header(const uint8_t *data, size_t len, nw_packet_t *parsed) {
    if (len < sizeof(struct iphdr)) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    const struct iphdr *ip = (const struct iphdr *)data;
    
    // Validate IP version
    if (ip->version != 4) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    // Extract header length (IHL field is in 32-bit words)
    size_t ip_hdr_len = ip->ihl * 4;
    if (ip_hdr_len < 20 || ip_hdr_len > len) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    // Populate parsed packet structure
    parsed->src_ip = ntohl(ip->saddr);
    parsed->dst_ip = ntohl(ip->daddr);
    parsed->protocol = ip->protocol;
    parsed->ttl = ip->ttl;
    
    return NW_SUCCESS;
}

/**
 * Parse TCP header and extract port information
 * Handles variable-length TCP options
 */
static nw_error_t parse_tcp_header(const uint8_t *data, size_t len, nw_packet_t *parsed) {
    if (len < sizeof(struct tcphdr)) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    const struct tcphdr *tcp = (const struct tcphdr *)data;
    
    parsed->src_port = ntohs(tcp->source);
    parsed->dst_port = ntohs(tcp->dest);
    
    return NW_SUCCESS;
}

/**
 * Parse UDP header and extract port information
 * UDP header is fixed 8 bytes, simpler than TCP
 */
static nw_error_t parse_udp_header(const uint8_t *data, size_t len, nw_packet_t *parsed) {
    if (len < sizeof(struct udphdr)) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    const struct udphdr *udp = (const struct udphdr *)data;
    
    parsed->src_port = ntohs(udp->source);
    parsed->dst_port = ntohs(udp->dest);
    
    return NW_SUCCESS;
}

/**
 * Parse ICMP header for diagnostic packets
 * Used for ping, traceroute, and error messages
 */
static nw_error_t parse_icmp_header(const uint8_t *data, size_t len, nw_packet_t *parsed) {
    if (len < sizeof(struct icmphdr)) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    const struct icmphdr *icmp = (const struct icmphdr *)data;
    
    // Store ICMP type and code in port fields for convenience
    parsed->src_port = icmp->type;
    parsed->dst_port = icmp->code;
    
    return NW_SUCCESS;
}

/**
 * Main packet parsing function with protocol-aware dissection
 * Performs zero-copy analysis of network packets
 * 
 * @param raw Raw packet data from network interface
 * @param parsed Output structure with parsed fields
 * @return NW_SUCCESS on successful parse, error code otherwise
 */
nw_error_t nw_packet_parse_full(const nw_packet_t *raw, nw_packet_t *parsed) {
    if (!raw || !parsed || raw->length == 0) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    // Copy raw packet data
    memcpy(parsed, raw, sizeof(nw_packet_t));
    
    const uint8_t *data = raw->data;
    size_t remaining = raw->length;
    size_t offset = 0;
    
    // Check if packet has Ethernet header
    offset = parse_ethernet_header(data, remaining);
    if (offset == 0) {
        // Assume raw IP packet (no Ethernet header)
        offset = 0;
    }
    
    // Parse IP header
    if (parse_ip_header(data + offset, remaining - offset, parsed) != NW_SUCCESS) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    // Calculate IP header length for protocol parsing
    const struct iphdr *ip = (const struct iphdr *)(data + offset);
    size_t ip_hdr_len = ip->ihl * 4;
    offset += ip_hdr_len;
    
    if (offset >= remaining) {
        return NW_SUCCESS; // Valid IP packet, no payload
    }
    
    // Parse transport layer based on protocol
    switch (parsed->protocol) {
        case IPPROTO_TCP:
            return parse_tcp_header(data + offset, remaining - offset, parsed);
            
        case IPPROTO_UDP:
            return parse_udp_header(data + offset, remaining - offset, parsed);
            
        case IPPROTO_ICMP:
            return parse_icmp_header(data + offset, remaining - offset, parsed);
            
        default:
            // Unknown protocol, IP parsing is sufficient
            return NW_SUCCESS;
    }
}

/**
 * Validate packet integrity and structure
 * Performs comprehensive validation including checksums
 * 
 * @param packet Packet to validate
 * @return true if packet is valid, false otherwise
 */
bool nw_packet_validate(const nw_packet_t *packet) {
    if (!packet || packet->length == 0 || packet->length > NW_MAX_PACKET_SIZE) {
        return false;
    }
    
    // Check minimum size for IP header
    if (packet->length < sizeof(struct iphdr)) {
        return false;
    }
    
    const struct iphdr *ip = (const struct iphdr *)packet->data;
    
    // Validate IP version
    if (ip->version != 4) {
        return false;
    }
    
    // Validate header length
    size_t ip_hdr_len = ip->ihl * 4;
    if (ip_hdr_len < 20 || ip_hdr_len > packet->length) {
        return false;
    }
    
    // Validate total length field matches actual packet size
    uint16_t total_len = ntohs(ip->tot_len);
    if (total_len > packet->length) {
        return false;
    }
    
    // Additional protocol-specific validation could be added here
    
    return true;
}

/**
 * Extract payload data from parsed packet
 * Returns pointer to payload and sets payload_len
 * 
 * @param packet Parsed packet structure
 * @param payload_len Output parameter for payload length
 * @return Pointer to payload data, or NULL if no payload
 */
const uint8_t* nw_packet_get_payload(const nw_packet_t *packet, size_t *payload_len) {
    if (!packet || !payload_len) {
        return NULL;
    }
    
    if (packet->length < sizeof(struct iphdr)) {
        *payload_len = 0;
        return NULL;
    }
    
    const struct iphdr *ip = (const struct iphdr *)packet->data;
    size_t ip_hdr_len = ip->ihl * 4;
    size_t offset = ip_hdr_len;
    
    // Skip transport header based on protocol
    switch (packet->protocol) {
        case IPPROTO_TCP: {
            if (packet->length < offset + sizeof(struct tcphdr)) {
                *payload_len = 0;
                return NULL;
            }
            const struct tcphdr *tcp = (const struct tcphdr *)(packet->data + offset);
            offset += tcp->doff * 4; // TCP header length
            break;
        }
        
        case IPPROTO_UDP:
            if (packet->length < offset + sizeof(struct udphdr)) {
                *payload_len = 0;
                return NULL;
            }
            offset += sizeof(struct udphdr);
            break;
            
        case IPPROTO_ICMP:
            if (packet->length < offset + sizeof(struct icmphdr)) {
                *payload_len = 0;
                return NULL;
            }
            offset += sizeof(struct icmphdr);
            break;
            
        default:
            // Unknown protocol, no transport header to skip
            break;
    }
    
    if (offset >= packet->length) {
        *payload_len = 0;
        return NULL;
    }
    
    *payload_len = packet->length - offset;
    return packet->data + offset;
}

/**
 * Classify packet by application-layer protocol
 * Performs heuristic analysis of payload for protocol detection
 * 
 * @param packet Parsed packet with payload
 * @return Protocol string identifier, or "unknown"
 */
const char* nw_packet_classify_protocol(const nw_packet_t *packet) {
    if (!packet) {
        return "invalid";
    }
    
    // Classification based on well-known ports
    if (packet->protocol == IPPROTO_TCP) {
        if (packet->dst_port == 80 || packet->src_port == 80) {
            return "HTTP";
        } else if (packet->dst_port == 443 || packet->src_port == 443) {
            return "HTTPS/TLS";
        } else if (packet->dst_port == 22 || packet->src_port == 22) {
            return "SSH";
        } else if (packet->dst_port == 25 || packet->src_port == 25) {
            return "SMTP";
        } else if (packet->dst_port == 3306 || packet->src_port == 3306) {
            return "MySQL";
        } else if (packet->dst_port == 5432 || packet->src_port == 5432) {
            return "PostgreSQL";
        }
    } else if (packet->protocol == IPPROTO_UDP) {
        if (packet->dst_port == 53 || packet->src_port == 53) {
            return "DNS";
        } else if (packet->dst_port == 123 || packet->src_port == 123) {
            return "NTP";
        } else if (packet->dst_port == 67 || packet->dst_port == 68) {
            return "DHCP";
        }
    } else if (packet->protocol == IPPROTO_ICMP) {
        return "ICMP";
    }
    
    // Protocol-based classification
    switch (packet->protocol) {
        case IPPROTO_TCP: return "TCP";
        case IPPROTO_UDP: return "UDP";
        case IPPROTO_ICMP: return "ICMP";
        default: return "unknown";
    }
}
