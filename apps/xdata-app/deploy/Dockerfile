FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user (using UID 10000 to avoid conflicts)
RUN useradd -m -u 10000 appuser

WORKDIR /app

# Copy the pre-built binary from host
COPY target/debug/xdata-app /app/xdata-app

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port (adjust if your app uses a specific port)
EXPOSE 8080

# Run the application
CMD ["/app/xdata-app"]
