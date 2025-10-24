/**
 * NetWeaver Core - Low-level C packet operations library
 * 
 * This module provides high-performance packet crafting, parsing, and I/O operations
 * for the NetWeaver network intelligence framework. Written in C for maximum performance
 * and direct hardware access, with safe FFI bindings for Rust integration.
 * 
 * Design principles:
 * - Zero-copy where possible to minimize latency
 * - Buffer pooling for memory efficiency
 * - Careful error handling with explicit error codes
 * - Thread-safe by design (no global mutable state)
 * 
 * Performance targets:
 * - Packet crafting: < 1 microsecond per packet
 * - Parsing: Line-rate on gigabit ethernet
 * - Checksum: Hardware-accelerated where available
 * 
 * @author NetWeaver Contributors
 * @version 0.1.0
 */

#ifndef NETWEAVER_CORE_H
#define NETWEAVER_CORE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

#define NW_MAX_PACKET_SIZE 65535
#define NW_BUFFER_POOL_SIZE 1024
#define NW_MAX_INTERFACES 32

typedef enum {
    NW_SUCCESS = 0,
    NW_ERROR_SOCKET = -1,
    NW_ERROR_PERMISSION = -2,
    NW_ERROR_INVALID_PARAM = -3,
    NW_ERROR_TIMEOUT = -4,
    NW_ERROR_BUFFER_FULL = -5,
    NW_ERROR_NOT_FOUND = -6
} nw_error_t;

typedef enum {
    NW_PROTO_ICMP = 1,
    NW_PROTO_TCP = 6,
    NW_PROTO_UDP = 17,
    NW_PROTO_RAW = 255
} nw_protocol_t;

typedef struct {
    uint8_t data[NW_MAX_PACKET_SIZE];
    size_t length;
    uint64_t timestamp_us;
    uint32_t src_ip;
    uint32_t dst_ip;
    uint16_t src_port;
    uint16_t dst_port;
    uint8_t protocol;
    uint8_t ttl;
} nw_packet_t;

typedef struct {
    int fd;
    int af;
    int type;
    int protocol;
    bool is_raw;
    bool is_nonblocking;
} nw_socket_t;

typedef struct {
    void *buffers[NW_BUFFER_POOL_SIZE];
    size_t buffer_size;
    uint32_t available_mask[NW_BUFFER_POOL_SIZE / 32];
    size_t pool_size;
} nw_buffer_pool_t;

typedef struct {
    uint32_t ip;
    uint8_t mac[6];
    char interface[32];
    uint16_t mtu;
    bool is_up;
} nw_interface_t;

nw_error_t nw_init(void);
void nw_cleanup(void);

nw_error_t nw_socket_create(nw_socket_t *sock, int af, int type, nw_protocol_t proto);
nw_error_t nw_socket_close(nw_socket_t *sock);
nw_error_t nw_socket_set_nonblocking(nw_socket_t *sock, bool enable);
nw_error_t nw_socket_bind(nw_socket_t *sock, uint32_t addr, uint16_t port);
nw_error_t nw_socket_set_timeout(nw_socket_t *sock, uint32_t timeout_ms);

nw_error_t nw_packet_send_raw(nw_socket_t *sock, const nw_packet_t *packet);
nw_error_t nw_packet_recv_raw(nw_socket_t *sock, nw_packet_t *packet, uint32_t timeout_ms);

nw_error_t nw_packet_craft_icmp_echo(nw_packet_t *packet, uint32_t dst_ip, uint16_t id, uint16_t seq);
nw_error_t nw_packet_craft_tcp_syn(nw_packet_t *packet, uint32_t src_ip, uint32_t dst_ip, 
                                     uint16_t src_port, uint16_t dst_port);
nw_error_t nw_packet_craft_udp(nw_packet_t *packet, uint32_t src_ip, uint32_t dst_ip,
                                uint16_t src_port, uint16_t dst_port, const uint8_t *payload, size_t payload_len);

nw_error_t nw_packet_parse(const nw_packet_t *raw, nw_packet_t *parsed);
nw_error_t nw_packet_parse_full(const nw_packet_t *raw, nw_packet_t *parsed);
bool nw_packet_is_icmp_reply(const nw_packet_t *packet, uint16_t expected_id);
bool nw_packet_is_tcp_syn_ack(const nw_packet_t *packet);
bool nw_packet_validate(const nw_packet_t *packet);
const uint8_t* nw_packet_get_payload(const nw_packet_t *packet, size_t *payload_len);
const char* nw_packet_classify_protocol(const nw_packet_t *packet);

nw_error_t nw_buffer_pool_init(nw_buffer_pool_t *pool, size_t buffer_size, size_t pool_size);
void nw_buffer_pool_destroy(nw_buffer_pool_t *pool);
void *nw_buffer_pool_acquire(nw_buffer_pool_t *pool);
void nw_buffer_pool_release(nw_buffer_pool_t *pool, void *buffer);

nw_error_t nw_get_interfaces(nw_interface_t *interfaces, size_t *count);
nw_error_t nw_get_default_gateway(uint32_t *gateway_ip);
uint16_t nw_checksum(const void *data, size_t len);

uint64_t nw_timestamp_us(void);
uint32_t nw_ip_str_to_int(const char *ip_str);
void nw_ip_int_to_str(uint32_t ip, char *buffer, size_t buffer_len);

#ifdef __cplusplus
}
#endif

#endif
