FROM mcr.microsoft.com/devcontainers/rust:bookworm

RUN export DEBIAN_FRONTEND=noninteractive

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes \
    curl \
    build-essential \
    cmake \
    g++-aarch64-linux-gnu \
    git \
    pkg-config \
    libdbus-1-dev:arm64 \
    libudev-dev:arm64 \
    libxkbcommon-dev:arm64 \
    libfontconfig1-dev:arm64

RUN apt-get install -y git

# custom for this container
RUN apt-get install -y \
    cmake \
    libasound2-dev \
    libfreetype6-dev \
    pkg-config \
    libfontconfig1-dev

# websocat for testing websocket connection
RUN curl -L -o /usr/local/bin/websocat https://github.com/vi/websocat/releases/download/v4.0.0-alpha2/websocat.x86_64-unknown-linux-musl && \
    chmod +x /usr/local/bin/websocat

# Use the current user (with the same UID/GID as the host user)
ARG USERNAME=user
ARG USER_UID=1001
ARG USER_GID=$USER_UID

# Create the user and group
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

# Set up workspace directory
WORKDIR /workspace

# Change ownership of the workspace
RUN chown -R $USERNAME:$USERNAME /workspace

# Switch to non-root user
USER $USERNAME

# Set environment variables
ENV RUST_BACKTRACE=1
ENV CARGO_INCREMENTAL=1
