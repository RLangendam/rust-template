# Rust Template

A high-quality, production-ready Rust project template with comprehensive tooling, testing, and development infrastructure.

## Features

- **Modern Rust Setup**: Edition 2021 with MSRV 1.74
- **Dual Build Targets**: Library (rlib + cdylib) and CLI binary
- **Development Shell**: Nix flake for reproducible development environments
- **Testing Infrastructure**: Unit tests, integration tests, and documentation tests
- **Code Quality**: Clippy linting, rustfmt formatting, and cargo audit
- **Coverage Reporting**: LLVM-based code coverage with HTML reports
- **Benchmarking**: Built-in performance benchmarking support
- **Task Automation**: Taskfile for common workflows
- **Security Focused**: Denies unsafe code by default with configurable linting rules

## Quick Start

### Prerequisites

- **With Nix** (recommended): `nix flake update && nix develop`
- **Without Nix**: Rust 1.74+, OpenSSL development libraries

### Building

```bash
cargo build                 # Debug build
cargo build --release      # Release build with optimizations
```

### Running

```bash
cargo run                  # Run the CLI binary
cargo run -- [ARGS]       # Run with arguments
```

### Testing

```bash
# Run all tests
task check

# Run specific test suites
cargo test --lib          # Library tests only
cargo test --test '*'     # Integration tests only
cargo test --doc          # Documentation tests
```

### Coverage

Generate HTML coverage report:

```bash
task cov
```

This runs `cargo llvm-cov --html` and opens the report in your browser.

## Project Structure

```
.
├── src/
│   ├── lib.rs           # Library entry point
│   └── main.rs          # CLI binary entry point
├── tests/
│   └── integration_test.rs
├── benches/
│   └── perf_bench.rs
├── Cargo.toml           # Project manifest and dependencies
├── Cargo.lock           # Locked dependency versions
├── flake.nix            # Nix development environment
├── rust-toolchain.toml  # Rust version specification
├── Taskfile.yml         # Task definitions
└── deny.toml            # Dependency audit configuration
```

## Development Workflow

### Using Nix

```bash
# Enter development environment
nix develop

# Within the environment, run tasks
task check              # Run all quality gates
task cov                # Generate coverage report
```

### Without Nix

Install the following tools manually:
- `cargo-nextest` - Fast parallel test runner
- `cargo-llvm-cov` - Code coverage
- `cargo-audit` - Security auditing
- `cargo-expand` - Macro expansion
- `go-task` - Task runner

Then use `task` commands as above.

## Available Commands

### Quality Assurance

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test

# Security audit
cargo audit

# Expand macros (for debugging)
cargo expand
```

### Performance

```bash
# Run benchmarks
cargo bench

# Profile with coverage
cargo llvm-cov --open
```

## Configuration

### Cargo.toml

- **MSRV**: Set in `rust-version` (currently 1.74)
- **Build Profile**: Supports both library (rlib, cdylib) and binary
- **Lints**: Configured with safety-first defaults (denies unsafe code)
- **Keywords**: Update for crates.io discoverability

### Nix Flake

The `flake.nix` provides:
- Pinned nixpkgs version for reproducibility
- Rust toolchain from `rust-toolchain.toml`
- Development tools: cargo-nextest, cargo-llvm-cov, cargo-audit, etc.
- OpenSSL environment configuration

### Taskfile

Defines automated workflows in `Taskfile.yml`:
- `task check` - Full quality gate suite
- `task cov` - Coverage report generation

## Publishing

To publish to crates.io:

1. Update version in `Cargo.toml`
2. Ensure `publish = true` is set
3. Run `cargo publish --dry-run` to verify
4. Run `cargo publish` to publish

## Dependencies

Check current dependencies:

```bash
cargo tree                    # Dependency graph
cargo outdated               # Check for updates
cargo update                 # Update dependencies
```

## Security

- Unsafe code is denied by default
- Run `cargo audit` regularly to check for vulnerabilities
- Dependencies are audited via `deny.toml`

## License

Dual-licensed under MIT or Apache-2.0. See LICENSE files for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and run `task check`
4. Push and create a pull request

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
