#include "netweaver_core.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <ifaddrs.h>

nw_error_t nw_buffer_pool_init(nw_buffer_pool_t *pool, size_t buffer_size, size_t pool_size) {
    if (!pool || buffer_size == 0 || pool_size == 0 || pool_size > NW_BUFFER_POOL_SIZE) {
        return NW_ERROR_INVALID_PARAM;
    }
    
    memset(pool, 0, sizeof(nw_buffer_pool_t));
    pool->buffer_size = buffer_size;
    pool->pool_size = pool_size;
    
    for (size_t i = 0; i < pool_size; i++) {
        pool->buffers[i] = malloc(buffer_size);
        if (!pool->buffers[i]) {
            for (size_t j = 0; j < i; j++) {
                free(pool->buffers[j]);
            }
            return NW_ERROR_SOCKET;
        }
        pool->available_mask[i / 32] |= (1U << (i % 32));
    }
    
    return NW_SUCCESS;
}

void nw_buffer_pool_destroy(nw_buffer_pool_t *pool) {
    if (!pool) return;
    
    for (size_t i = 0; i < pool->pool_size; i++) {
        if (pool->buffers[i]) {
            free(pool->buffers[i]);
            pool->buffers[i] = NULL;
        }
    }
}

void *nw_buffer_pool_acquire(nw_buffer_pool_t *pool) {
    if (!pool) return NULL;
    
    for (size_t i = 0; i < pool->pool_size; i++) {
        uint32_t mask_idx = i / 32;
        uint32_t bit_idx = i % 32;
        
        if (pool->available_mask[mask_idx] & (1U << bit_idx)) {
            pool->available_mask[mask_idx] &= ~(1U << bit_idx);
            return pool->buffers[i];
        }
    }
    
    return NULL;
}

void nw_buffer_pool_release(nw_buffer_pool_t *pool, void *buffer) {
    if (!pool || !buffer) return;
    
    for (size_t i = 0; i < pool->pool_size; i++) {
        if (pool->buffers[i] == buffer) {
            uint32_t mask_idx = i / 32;
            uint32_t bit_idx = i % 32;
            pool->available_mask[mask_idx] |= (1U << bit_idx);
            return;
        }
    }
}

nw_error_t nw_get_interfaces(nw_interface_t *interfaces, size_t *count) {
    if (!interfaces || !count) return NW_ERROR_INVALID_PARAM;
    
    struct ifaddrs *ifaddr, *ifa;
    size_t idx = 0;
    
    if (getifaddrs(&ifaddr) == -1) {
        return NW_ERROR_SOCKET;
    }
    
    for (ifa = ifaddr; ifa != NULL && idx < *count; ifa = ifa->ifa_next) {
        if (ifa->ifa_addr == NULL || ifa->ifa_addr->sa_family != AF_INET) {
            continue;
        }
        
        struct sockaddr_in *addr = (struct sockaddr_in *)ifa->ifa_addr;
        interfaces[idx].ip = ntohl(addr->sin_addr.s_addr);
        strncpy(interfaces[idx].interface, ifa->ifa_name, sizeof(interfaces[idx].interface) - 1);
        
        int fd = socket(AF_INET, SOCK_DGRAM, 0);
        if (fd >= 0) {
            struct ifreq ifr;
            strncpy(ifr.ifr_name, ifa->ifa_name, IFNAMSIZ - 1);
            
            if (ioctl(fd, SIOCGIFHWADDR, &ifr) == 0) {
                memcpy(interfaces[idx].mac, ifr.ifr_hwaddr.sa_data, 6);
            }
            
            if (ioctl(fd, SIOCGIFMTU, &ifr) == 0) {
                interfaces[idx].mtu = ifr.ifr_mtu;
            }
            
            if (ioctl(fd, SIOCGIFFLAGS, &ifr) == 0) {
                interfaces[idx].is_up = (ifr.ifr_flags & IFF_UP) != 0;
            }
            
            close(fd);
        }
        
        idx++;
    }
    
    freeifaddrs(ifaddr);
    *count = idx;
    
    return NW_SUCCESS;
}

nw_error_t nw_get_default_gateway(uint32_t *gateway_ip) {
    if (!gateway_ip) return NW_ERROR_INVALID_PARAM;
    
    FILE *fp = fopen("/proc/net/route", "r");
    if (!fp) return NW_ERROR_NOT_FOUND;
    
    char line[256];
    // Skip header line
    if (!fgets(line, sizeof(line), fp)) {
        fclose(fp);
        return NW_ERROR_NOT_FOUND;
    }
    
    // Parse routing table entries to find default gateway
    while (fgets(line, sizeof(line), fp)) {
        char iface[32];
        uint32_t dest, gateway;
        
        if (sscanf(line, "%31s %x %x", iface, &dest, &gateway) == 3) {
            if (dest == 0) {
                *gateway_ip = ntohl(gateway);
                fclose(fp);
                return NW_SUCCESS;
            }
        }
    }
    
    fclose(fp);
    return NW_ERROR_NOT_FOUND;
}
