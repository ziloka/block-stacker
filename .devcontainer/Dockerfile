FROM rust:1.66.1-bullseye

ENV DEBIAN_FRONTEND noninteractive

# Add wasm support
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli wasm-server-runner basic-http-server

# Install macroquad & egui debian/ubuntu dependencies
# https://github.com/not-fl3/macroquad#linux
# https://github.com/emilk/egui#demo
RUN apt-get update \
    && apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev \
    && apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev

RUN apt-get install -y neofetch zip