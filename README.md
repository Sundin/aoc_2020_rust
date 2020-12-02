# Advent of Code 2020 (Rust version)

Install Rust and Cargo:

    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Add the following line to your `~/.bashrc` or `~/.zshrc`:

    export PATH="$HOME/.cargo/bin:$PATH"

Run project:

    cargo run <day>

Run unit tests:

    cargo test -- --nocapture

Just compile:

    cargo build

Check code without producing an executable:

    cargo check

Building for release:

    cargo build --release

Open bundled documentation:

    cargo doc --open
