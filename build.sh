#!/bin/bash
# Install Rust if not present
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install dioxus-cli
cargo install dioxus-cli@0.6.1

# Build project
dx build --release --platform web