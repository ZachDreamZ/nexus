# Contributing

Thanks for your interest in contributing to nexus!

## Getting Started

1. Fork the repo
2. Clone your fork
3. Run `cargo build` to verify it compiles
4. Run `cargo test` to verify tests pass

## Development

```bash
cargo build --release
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## PR Workflow

1. Create a feature branch from `master`
2. Make your changes
3. Ensure CI passes (build, test, clippy)
4. Open a PR against `master`

## Code Style

- Follow standard Rust formatting (`rustfmt`)
- Keep dependencies minimal (zero deps preferred)
- Add tests for new functionality
- Keep functions small and focused
