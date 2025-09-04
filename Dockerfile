# Build stage
FROM rust:1.87-alpine as builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

# Copy workspace files for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY cms/Cargo.toml ./cms/
COPY cms/src ./cms/src
COPY cms/migrations ./cms/migrations

# Build the CMS application
RUN cargo build --release --bin llacms

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    sqlite \
    curl

# Create app user
RUN adduser -D -u 1001 appuser

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/llacms /usr/local/bin/llacms

# Copy migrations
COPY --from=builder /app/cms/migrations ./migrations

# Create necessary directories
RUN mkdir -p /app/kv_storage && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3001

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/healthz || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV SERVER_PORT=3001
ENV DATABASE_URL=/app/cms.db
ENV ENVIRONMENT=production

# Run the application
CMD ["/usr/local/bin/llacms"]
