FROM mcr.microsoft.com/vscode/devcontainers/rust:buster

ENV DEBIAN_FRONTEND noninteractive

# Add wasm support
RUN rustup target add wasm32-unknown-unknown

# Install macroquad & egui debian/ubuntu dependencies
# https://github.com/not-fl3/macroquad#linux
RUN apt-get update && \
    apt-get install -y wget pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev && \
    wget https://github.com/caddyserver/caddy/releases/download/v2.7.4/caddy_2.7.4_linux_amd64.deb && \
    dpkg -i caddy_2.7.4_linux_amd64.deb
    
