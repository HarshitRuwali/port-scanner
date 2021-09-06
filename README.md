# port-scanner
Port Scanner made using Rust

Build:

`cargo build --release`

Usage:

Scan for first 1000 ports:
    `port-scanner -t thread_count ip`

Scan for all open ports:
    `port-scanner -t thread_count ip -p-`

- Binary:
    `target/release/port-scanner -t 100 192.168.1.1`

- dev:
    `cargo run -- -t 100 192.168.1.1`

