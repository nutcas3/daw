# Justfile for My DAW
# Install just: cargo install just
# Usage: just <command>

# Default recipe (runs when you just type 'just')
default:
    @just --list

# Build in debug mode
build:
    cargo build

# Build in release mode (optimized)
release:
    cargo build --release

# Run the DAW in debug mode
run:
    RUST_LOG=info cargo run

# Run in release mode
run-release:
    RUST_LOG=info cargo run --release

# Run with verbose logging
debug:
    RUST_LOG=debug cargo run

# Run tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Check code without building
check:
    cargo check --workspace

# Run clippy (linter)
lint:
    cargo clippy --workspace -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Clean build artifacts
clean:
    cargo clean

# Generate documentation
doc:
    cargo doc --no-deps --workspace --open

# Watch for changes and rebuild
watch:
    cargo watch -x run

# Profile with flamegraph (requires cargo-flamegraph)
flamegraph:
    cargo flamegraph

# Run all checks (lint, test, format)
ci: fmt-check lint test
    @echo "✅ All checks passed!"

# Install development tools
install-tools:
    cargo install cargo-watch
    cargo install cargo-flamegraph
    cargo install just

# Update dependencies
update:
    cargo update

# Show dependency tree
tree:
    cargo tree

# Benchmark (when benchmarks are added)
bench:
    cargo bench
