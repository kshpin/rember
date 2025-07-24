FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

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
