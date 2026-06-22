# Installation

## Prerequisites

- **Rust** 1.75+ ([install](https://rustup.rs))
- **Git** 2.x

## Build from Source

```bash
# Clone the repository
git clone https://github.com/tianhaikuo123/sqlrustgo.git
cd sqlrustgo

# Build
cargo build --release

# Run tests
cargo test

# Start REPL
cargo run --bin sqlrustgo
```

## Verify Installation

```bash
cargo test --all-features
```

All 73+ tests should pass.
