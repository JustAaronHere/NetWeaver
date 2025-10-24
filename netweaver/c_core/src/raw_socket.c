#include "netweaver_core.h"
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>

nw_error_t nw_init(void) {
    return NW_SUCCESS;
}

void nw_cleanup(void) {
}

nw_error_t nw_socket_create(nw_socket_t *sock, int af, int type, nw_protocol_t proto) {
    if (!sock) return NW_ERROR_INVALID_PARAM;
    
    memset(sock, 0, sizeof(nw_socket_t));
    
    sock->af = af;
    sock->type = type;
    sock->protocol = proto;
    sock->is_raw = (type == SOCK_RAW);
    
    sock->fd = socket(af, type, proto);
    if (sock->fd < 0) {
        return (errno == EACCES || errno == EPERM) ? NW_ERROR_PERMISSION : NW_ERROR_SOCKET;
    }
    
    if (sock->is_raw) {
        int on = 1;
        if (setsockopt(sock->fd, IPPROTO_IP, IP_HDRINCL, &on, sizeof(on)) < 0) {
            close(sock->fd);
            return NW_ERROR_SOCKET;
        }
    }
    
    return NW_SUCCESS;
}

nw_error_t nw_socket_close(nw_socket_t *sock) {
    if (!sock || sock->fd < 0) return NW_ERROR_INVALID_PARAM;
    
    close(sock->fd);
    sock->fd = -1;
    return NW_SUCCESS;
}

nw_error_t nw_socket_set_nonblocking(nw_socket_t *sock, bool enable) {
    if (!sock || sock->fd < 0) return NW_ERROR_INVALID_PARAM;
    
    int flags = fcntl(sock->fd, F_GETFL, 0);
    if (flags < 0) return NW_ERROR_SOCKET;
    
    if (enable) {
        flags |= O_NONBLOCK;
    } else {
        flags &= ~O_NONBLOCK;
    }
    
    if (fcntl(sock->fd, F_SETFL, flags) < 0) {
        return NW_ERROR_SOCKET;
    }
    
    sock->is_nonblocking = enable;
    return NW_SUCCESS;
}

nw_error_t nw_socket_bind(nw_socket_t *sock, uint32_t addr, uint16_t port) {
    if (!sock || sock->fd < 0) return NW_ERROR_INVALID_PARAM;
    
    struct sockaddr_in sa;
    memset(&sa, 0, sizeof(sa));
    sa.sin_family = AF_INET;
    sa.sin_addr.s_addr = htonl(addr);
    sa.sin_port = htons(port);
    
    if (bind(sock->fd, (struct sockaddr *)&sa, sizeof(sa)) < 0) {
        return NW_ERROR_SOCKET;
    }
    
    return NW_SUCCESS;
}

nw_error_t nw_socket_set_timeout(nw_socket_t *sock, uint32_t timeout_ms) {
    if (!sock || sock->fd < 0) return NW_ERROR_INVALID_PARAM;
    
    struct timeval tv;
    tv.tv_sec = timeout_ms / 1000;
    tv.tv_usec = (timeout_ms % 1000) * 1000;
    
    if (setsockopt(sock->fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)) < 0) {
        return NW_ERROR_SOCKET;
    }
    
    return NW_SUCCESS;
}

nw_error_t nw_packet_send_raw(nw_socket_t *sock, const nw_packet_t *packet) {
    if (!sock || sock->fd < 0 || !packet) return NW_ERROR_INVALID_PARAM;
    
    struct sockaddr_in dest;
    memset(&dest, 0, sizeof(dest));
    dest.sin_family = AF_INET;
    dest.sin_addr.s_addr = htonl(packet->dst_ip);
    
    ssize_t sent = sendto(sock->fd, packet->data, packet->length, 0,
                          (struct sockaddr *)&dest, sizeof(dest));
    
    if (sent < 0) {
        return NW_ERROR_SOCKET;
    }
    
    return NW_SUCCESS;
}

nw_error_t nw_packet_recv_raw(nw_socket_t *sock, nw_packet_t *packet, uint32_t timeout_ms) {
    if (!sock || sock->fd < 0 || !packet) return NW_ERROR_INVALID_PARAM;
    
    if (timeout_ms > 0) {
        nw_socket_set_timeout(sock, timeout_ms);
    }
    
    struct sockaddr_in src;
    socklen_t src_len = sizeof(src);
    
    ssize_t received = recvfrom(sock->fd, packet->data, NW_MAX_PACKET_SIZE, 0,
                                (struct sockaddr *)&src, &src_len);
    
    if (received < 0) {
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            return NW_ERROR_TIMEOUT;
        }
        return NW_ERROR_SOCKET;
    }
    
    packet->length = received;
    packet->timestamp_us = nw_timestamp_us();
    
    return NW_SUCCESS;
}
