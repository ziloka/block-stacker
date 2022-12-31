FROM rust:1.66.1-bullseye

# Add wasm support
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli wasm-server-runner basic-http-server

# Install macroquad debian/ubuntu dependencies
# https://github.com/not-fl3/macroquad#linux
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

RUN apt-get install -y neofetch zip