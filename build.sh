#!/bin/bash
set -e

cargo clean
cargo fmt
cargo check --workspace --all-targets
cargo check --workspace --all-targets --release
cargo clean
cargo clippy --workspace --all-targets --all-features
cargo build --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo build --workspace --all-targets --all-features --release
cargo test --workspace --all-targets --all-features --release
cargo doc --no-deps --workspace --all-features --release
cargo test --doc --release
