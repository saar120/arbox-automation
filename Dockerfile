# Use the official Rust image from Docker Hub as the base image
FROM rust:1.77.2 as builder

# Create a new directory for your application
WORKDIR /usr/src/myapp

# Copy your source tree
COPY ./ ./

# Build your application in release mode
RUN cargo build --release

# Use Debian Bookworm as the runtime container base
FROM debian:bookworm

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/myapp/target/release/ /usr/local/bin/

# Copy the users.json file
COPY --from=builder /usr/src/myapp/users.json /

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/*

# Install any runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

# Set the CMD to your app \
    CMD ["arbox_automation"]
