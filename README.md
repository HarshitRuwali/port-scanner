# port-scanner
Port Scanner made using Rust

Build:

`cargo build --release`

Usage:

`port-scanner -t thread_count ip`

- Binary:
    `target/release/port-scanner -t 100 192.168.1.1`

- dev:
    `cargo run -- -t 100 192.168.1.1`

