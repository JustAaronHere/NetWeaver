# NetWeaver Docker Image
# Multi-stage build for minimal final image

FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libpcap-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy source
COPY . .

# Build release binary
RUN cargo build --release

# Final minimal image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpcap0.8 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/netweaver /usr/local/bin/netweaver

# Create config directory
RUN mkdir -p /etc/netweaver

# Set capabilities (requires privileged mode)
# RUN setcap cap_net_raw,cap_net_admin=eip /usr/local/bin/netweaver

WORKDIR /work

ENTRYPOINT ["/usr/local/bin/netweaver"]
CMD ["--help"]
