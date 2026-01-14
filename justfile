# Default recipe
default:
    @just --list

# Build the project
build:
    cargo build

# Run the project
run:
    cargo run

# Run tests
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Run clippy
clippy:
    cargo clippy -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Lint: format check + clippy
lint: fmt clippy

# Check: build + test + clippy
check: build test clippy
