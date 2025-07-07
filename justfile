default:
    @just --list

build:
    cargo build --workspace

check:
    cargo check --workspace

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo +nightly fmt --all

sort:
    cargo sort --workspace

docs:
    cargo doc --workspace --open

run:
    cargo run -p starblox

test:
    cargo test -p starblox

test-core:
    cargo test -p starblox-core

tests:
    cargo test --workspace
