# Dockerfile - Multi-stage build for Ballistics Analyzer
# Supports building for multiple platforms

# Stage 1: Rust Builder
FROM rust:1.75-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libx11-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libwayland-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY ballistics_core ./ballistics_core
COPY ballistics-desktop ./ballistics-desktop

# Build the application
RUN cargo build --release -p ballistics-desktop

# Stage 2: Runtime Image
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    libwebkit2gtk-4.0-37 \
    libappindicator3-1 \
    librsvg2-2 \
    libx11-6 \
    libxcb1 \
    libxkbcommon0 \
    libgl1 \
    libwayland-client0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 ballistics && \
    mkdir -p /home/ballistics/.local/share/ballistics && \
    chown -R ballistics:ballistics /home/ballistics

# Copy binary from builder
COPY --from=builder /build/target/release/ballistics-analyzer /usr/local/bin/
COPY --from=builder /build/ballistics-desktop/assets /usr/local/share/ballistics/assets

# Copy documentation
COPY README.md LICENSE PRIVACY_POLICY.md /usr/local/share/doc/ballistics/

# Set permissions
RUN chmod +x /usr/local/bin/ballistics-analyzer

# Switch to non-root user
USER ballistics
WORKDIR /home/ballistics

# Environment variables for GUI
ENV DISPLAY=:0
ENV XDG_RUNTIME_DIR=/tmp/runtime-ballistics
ENV NO_AT_BRIDGE=1

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ballistics-analyzer --version || exit 1

# Entry point
ENTRYPOINT ["ballistics-analyzer"]