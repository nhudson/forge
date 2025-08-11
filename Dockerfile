# Build stage
FROM rust:1.89-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:12.11-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false forge

# Copy the binary from builder stage
COPY --from=builder /app/target/release/forge /usr/local/bin/forge

# Set ownership and permissions
RUN chown forge:forge /usr/local/bin/forge && \
    chmod +x /usr/local/bin/forge

# Create workspace directory
WORKDIR /workspace
RUN chown forge:forge /workspace

# Switch to non-root user
USER forge

# Set the entrypoint
ENTRYPOINT ["forge"]

# Default command (show help)
CMD ["--help"]

# Metadata
LABEL org.opencontainers.image.title="Forge"
LABEL org.opencontainers.image.description="A fast, reliable CLI tool for converting PFX/P12 certificate files to PEM format"
LABEL org.opencontainers.image.vendor="Nick Hudson"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/nhudson/forge" 