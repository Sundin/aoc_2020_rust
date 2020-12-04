# Advent of Code 2020 (Rust version)

## Setup

Install Rust and Cargo:

    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Add the following line to your `~/.bashrc` or `~/.zshrc`:

    export PATH="$HOME/.cargo/bin:$PATH"

## Run project

Run project:

    cargo run <day>

Run unit tests:

    cargo test
    
Run unit tests with log output:

    cargo test -- --nocapture

## Useful commands

Just compile:

    cargo build

Check code without producing an executable:

    cargo check

Building for release:

    cargo build --release

Open bundled documentation:

    cargo doc --open
