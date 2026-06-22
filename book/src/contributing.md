# Contributing

We welcome contributions! Here's how to get started.

## Development Workflow

1. **Fork** the repository
2. **Create** a feature branch: `feature/your-feature`
3. **Make changes** and add tests
4. **Run checks**:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test --all-features
   cargo audit
   ```
5. **Commit** with [Conventional Commits](https://www.conventionalcommits.org/)
6. **Push** and create a Pull Request

## Code Style

- Follow Rust standard conventions (`rustfmt`)
- Write doc comments (`///`) for public APIs
- Add tests for new functionality
- Keep Clippy warnings at zero

## Pre-Release Checks

Run the automated gate check script:

```bash
bash scripts/pre-release.sh
```

See [RELEASE_CHECKLIST.md](../RELEASE_CHECKLIST.md) for the full checklist.
