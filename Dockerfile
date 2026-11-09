# Optional base image for Cloud Agent environment builds (Rust + Vulkan deps on Linux).
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates git build-essential pkg-config \
    libssl-dev libxkbcommon-dev libwayland-dev libx11-dev \
    && rm -rf /var/lib/apt/lists/*

USER ubuntu
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"
WORKDIR /workspace
