# Step 1: Build the Rust binary
FROM rust:1.79 as builder

# Install necessary libraries
RUN apt-get update && apt-get install -y --no-install-recommends \
  libpq-dev \
  libclang-dev \
  clang \
  llvm \
  llvm-dev \
  llvm-runtime \
  libavutil-dev \
  libavcodec-dev \
  libavformat-dev \
  libavdevice-dev \
  libavfilter-dev \
  pkg-config

# Configure PKG_CONFIG_PATH for FFmpeg
ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/local/lib/pkgconfig

# Set LIBCLANG_PATH for bindgen
ENV LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu/

# Debug: Find and print all libclang files
RUN find /usr -name "libclang.so*"

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy source files
COPY . .

# Build the project
RUN cargo build --release

# Step 2: Lightweight image to run the backend
FROM debian:buster-slim

# Install necessary libraries
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

# Set the working directory
WORKDIR /usr/src/app

# Copy the binary from the build stage
COPY --from=builder /usr/src/app/target/release/rust_to_gif_backend .

# Expose the port
EXPOSE 8000

# Start the server
CMD ["./rust_to_gif_backend"]
