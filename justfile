default:
    @just --list

build:
    cargo build --workspace

check:
    cargo check --workspace

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

docs:
    cargo doc --workspace --open

run:
    cargo run -p starblox-tui

test:
    cargo test --workspace

test-core:
    cargo test -p starblox-core

test-tui:
    cargo test -p starblox-tui
