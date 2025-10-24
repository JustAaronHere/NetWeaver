#include "netweaver_core.h"
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>
#include <sys/time.h>
#include <time.h>

typedef struct __attribute__((packed)) {
    uint8_t type;
    uint8_t code;
    uint16_t checksum;
    uint16_t id;
    uint16_t sequence;
} icmp_header_t;

typedef struct __attribute__((packed)) {
    uint16_t src_port;
    uint16_t dst_port;
    uint32_t seq_num;
    uint32_t ack_num;
    uint8_t data_offset;
    uint8_t flags;
    uint16_t window;
    uint16_t checksum;
    uint16_t urgent_ptr;
} tcp_header_t;

typedef struct __attribute__((packed)) {
    uint16_t src_port;
    uint16_t dst_port;
    uint16_t length;
    uint16_t checksum;
} udp_header_t;

typedef struct __attribute__((packed)) {
    uint8_t version_ihl;
    uint8_t tos;
    uint16_t total_length;
    uint16_t id;
    uint16_t flags_fragment;
    uint8_t ttl;
    uint8_t protocol;
    uint16_t checksum;
    uint32_t src_ip;
    uint32_t dst_ip;
} ip_header_t;

uint16_t nw_checksum(const void *data, size_t len) {
    const uint16_t *buf = (const uint16_t *)data;
    uint32_t sum = 0;
    
    while (len > 1) {
        sum += *buf++;
        len -= 2;
    }
    
    if (len == 1) {
        sum += *(uint8_t *)buf;
    }
    
    sum = (sum >> 16) + (sum & 0xFFFF);
    sum += (sum >> 16);
    
    return (uint16_t)~sum;
}

uint64_t nw_timestamp_us(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return (uint64_t)tv.tv_sec * 1000000 + tv.tv_usec;
}

nw_error_t nw_packet_craft_icmp_echo(nw_packet_t *packet, uint32_t dst_ip, uint16_t id, uint16_t seq) {
    if (!packet) return NW_ERROR_INVALID_PARAM;
    
    memset(packet, 0, sizeof(nw_packet_t));
    
    ip_header_t *ip = (ip_header_t *)packet->data;
    icmp_header_t *icmp = (icmp_header_t *)(packet->data + sizeof(ip_header_t));
    
    ip->version_ihl = 0x45;
    ip->tos = 0;
    ip->total_length = htons(sizeof(ip_header_t) + sizeof(icmp_header_t));
    ip->id = htons(rand() & 0xFFFF);
    ip->flags_fragment = 0;
    ip->ttl = 64;
    ip->protocol = NW_PROTO_ICMP;
    ip->dst_ip = htonl(dst_ip);
    ip->checksum = 0;
    ip->checksum = nw_checksum(ip, sizeof(ip_header_t));
    
    icmp->type = 8;
    icmp->code = 0;
    icmp->id = htons(id);
    icmp->sequence = htons(seq);
    icmp->checksum = 0;
    icmp->checksum = nw_checksum(icmp, sizeof(icmp_header_t));
    
    packet->length = sizeof(ip_header_t) + sizeof(icmp_header_t);
    packet->dst_ip = dst_ip;
    packet->protocol = NW_PROTO_ICMP;
    packet->timestamp_us = nw_timestamp_us();
    
    return NW_SUCCESS;
}

nw_error_t nw_packet_craft_tcp_syn(nw_packet_t *packet, uint32_t src_ip, uint32_t dst_ip,
                                     uint16_t src_port, uint16_t dst_port) {
    if (!packet) return NW_ERROR_INVALID_PARAM;
    
    memset(packet, 0, sizeof(nw_packet_t));
    
    ip_header_t *ip = (ip_header_t *)packet->data;
    tcp_header_t *tcp = (tcp_header_t *)(packet->data + sizeof(ip_header_t));
    
    ip->version_ihl = 0x45;
    ip->tos = 0;
    ip->total_length = htons(sizeof(ip_header_t) + sizeof(tcp_header_t));
    ip->id = htons(rand() & 0xFFFF);
    ip->flags_fragment = htons(0x4000);
    ip->ttl = 64;
    ip->protocol = NW_PROTO_TCP;
    ip->src_ip = htonl(src_ip);
    ip->dst_ip = htonl(dst_ip);
    ip->checksum = 0;
    ip->checksum = nw_checksum(ip, sizeof(ip_header_t));
    
    tcp->src_port = htons(src_port);
    tcp->dst_port = htons(dst_port);
    tcp->seq_num = htonl(rand());
    tcp->ack_num = 0;
    tcp->data_offset = 0x50;
    tcp->flags = 0x02;
    tcp->window = htons(65535);
    tcp->checksum = 0;
    tcp->urgent_ptr = 0;
    
    packet->length = sizeof(ip_header_t) + sizeof(tcp_header_t);
    packet->src_ip = src_ip;
    packet->dst_ip = dst_ip;
    packet->src_port = src_port;
    packet->dst_port = dst_port;
    packet->protocol = NW_PROTO_TCP;
    packet->timestamp_us = nw_timestamp_us();
    
    return NW_SUCCESS;
}

nw_error_t nw_packet_craft_udp(nw_packet_t *packet, uint32_t src_ip, uint32_t dst_ip,
                                uint16_t src_port, uint16_t dst_port, 
                                const uint8_t *payload, size_t payload_len) {
    if (!packet || (payload_len > 0 && !payload)) return NW_ERROR_INVALID_PARAM;
    
    memset(packet, 0, sizeof(nw_packet_t));
    
    ip_header_t *ip = (ip_header_t *)packet->data;
    udp_header_t *udp = (udp_header_t *)(packet->data + sizeof(ip_header_t));
    uint8_t *data = packet->data + sizeof(ip_header_t) + sizeof(udp_header_t);
    
    if (payload_len > 0) {
        memcpy(data, payload, payload_len);
    }
    
    ip->version_ihl = 0x45;
    ip->tos = 0;
    ip->total_length = htons(sizeof(ip_header_t) + sizeof(udp_header_t) + payload_len);
    ip->id = htons(rand() & 0xFFFF);
    ip->flags_fragment = 0;
    ip->ttl = 64;
    ip->protocol = NW_PROTO_UDP;
    ip->src_ip = htonl(src_ip);
    ip->dst_ip = htonl(dst_ip);
    ip->checksum = 0;
    ip->checksum = nw_checksum(ip, sizeof(ip_header_t));
    
    udp->src_port = htons(src_port);
    udp->dst_port = htons(dst_port);
    udp->length = htons(sizeof(udp_header_t) + payload_len);
    udp->checksum = 0;
    
    packet->length = sizeof(ip_header_t) + sizeof(udp_header_t) + payload_len;
    packet->src_ip = src_ip;
    packet->dst_ip = dst_ip;
    packet->src_port = src_port;
    packet->dst_port = dst_port;
    packet->protocol = NW_PROTO_UDP;
    packet->timestamp_us = nw_timestamp_us();
    
    return NW_SUCCESS;
}

nw_error_t nw_packet_parse(const nw_packet_t *raw, nw_packet_t *parsed) {
    if (!raw || !parsed || raw->length < sizeof(ip_header_t)) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    memcpy(parsed, raw, sizeof(nw_packet_t));
    
    const ip_header_t *ip = (const ip_header_t *)raw->data;
    parsed->src_ip = ntohl(ip->src_ip);
    parsed->dst_ip = ntohl(ip->dst_ip);
    parsed->protocol = ip->protocol;
    parsed->ttl = ip->ttl;
    
    size_t ip_header_len = (ip->version_ihl & 0x0F) * 4;
    
    if (ip->protocol == NW_PROTO_TCP && raw->length >= ip_header_len + sizeof(tcp_header_t)) {
        const tcp_header_t *tcp = (const tcp_header_t *)(raw->data + ip_header_len);
        parsed->src_port = ntohs(tcp->src_port);
        parsed->dst_port = ntohs(tcp->dst_port);
    } else if (ip->protocol == NW_PROTO_UDP && raw->length >= ip_header_len + sizeof(udp_header_t)) {
        const udp_header_t *udp = (const udp_header_t *)(raw->data + ip_header_len);
        parsed->src_port = ntohs(udp->src_port);
        parsed->dst_port = ntohs(udp->dst_port);
    }
    
    return NW_SUCCESS;
}

bool nw_packet_is_icmp_reply(const nw_packet_t *packet, uint16_t expected_id) {
    if (!packet || packet->protocol != NW_PROTO_ICMP) return false;
    
    const ip_header_t *ip = (const ip_header_t *)packet->data;
    size_t ip_header_len = (ip->version_ihl & 0x0F) * 4;
    
    if (packet->length < ip_header_len + sizeof(icmp_header_t)) return false;
    
    const icmp_header_t *icmp = (const icmp_header_t *)(packet->data + ip_header_len);
    return icmp->type == 0 && ntohs(icmp->id) == expected_id;
}

bool nw_packet_is_tcp_syn_ack(const nw_packet_t *packet) {
    if (!packet || packet->protocol != NW_PROTO_TCP) return false;
    
    const ip_header_t *ip = (const ip_header_t *)packet->data;
    size_t ip_header_len = (ip->version_ihl & 0x0F) * 4;
    
    if (packet->length < ip_header_len + sizeof(tcp_header_t)) return false;
    
    const tcp_header_t *tcp = (const tcp_header_t *)(packet->data + ip_header_len);
    return (tcp->flags & 0x12) == 0x12;
}

uint32_t nw_ip_str_to_int(const char *ip_str) {
    struct in_addr addr;
    if (inet_pton(AF_INET, ip_str, &addr) <= 0) {
        return 0;
    }
    return ntohl(addr.s_addr);
}

void nw_ip_int_to_str(uint32_t ip, char *buffer, size_t buffer_len) {
    struct in_addr addr;
    addr.s_addr = htonl(ip);
    inet_ntop(AF_INET, &addr, buffer, buffer_len);
}
