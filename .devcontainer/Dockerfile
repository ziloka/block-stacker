FROM rust:1.62.1-bullseye

# Add wasm support
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli wasm-server-runner basic-http-server

# Install bevy debian/ubuntu dependencies
# https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN apt-get install -y neofetch zip

# Firefox
# RUN apt-get install -y firefox-esr

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
# ARG VARIANT="bullseye"
# FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>