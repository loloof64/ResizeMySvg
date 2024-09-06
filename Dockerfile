FROM ubuntu:20.04

# Avoid interactions
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    cmake \
    libgtk-3-dev \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m docker_user
USER docker_user
WORKDIR /home/docker_user

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/docker_user/.cargo/bin:${PATH}"

WORKDIR /app

# Copy project files into container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Generate a script in order to copy executable
RUN echo '#!/bin/bash\n\
cp /app/target/release/resize_my_svg /output/ 2>/dev/null || \
sudo cp /app/target/release/resize_my_svg /output/' > /home/docker_user/copy_executable.sh && \
    chmod +x /home/docker_user/copy_executable.sh

# Install sudo
USER root
RUN apt-get update && apt-get install -y sudo && \
    echo "docker_user ALL=(ALL) NOPASSWD: /bin/cp" >> /etc/sudoers
USER docker_user

# Set up volume for access from host
VOLUME /output