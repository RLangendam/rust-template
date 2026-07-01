# Rust Template

A high-quality, production-ready Rust project template with comprehensive tooling, testing, and development infrastructure.

## Features

- **Modern Rust Setup**: Edition 2021 with MSRV 1.75
- **Dual Build Targets**: Library (rlib + cdylib) and CLI binary
- **Development Shell**: Nix flake for reproducible development environments
- **Testing Infrastructure**: Unit tests, integration tests, and documentation tests
- **Code Quality**: Clippy linting, rustfmt formatting, and cargo audit
- **Coverage Reporting**: LLVM-based code coverage with HTML reports
- **Benchmarking**: Built-in performance benchmarking support
- **Task Automation**: Taskfile for common workflows
- **Security Focused**: Denies unsafe code by default with configurable linting rules
- **Fast Linking**: Mold linker for significantly faster compilation
- **Development Tools**: Cargo-watch for file watching, Bacon for background checking

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

### Examples

The project includes example programs to demonstrate library usage:

```bash
# List all available examples
cargo run --example

# Run the basic usage example
cargo run --example basic_usage

# Build all examples
cargo build --examples

# Build a specific example
cargo build --example basic_usage
```

The `basic_usage` example demonstrates:
- How to use the library's `add` function
- Compile-time evaluation with const functions
- Basic Rust project structure

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
├── examples/
│   └── basic_usage.rs   # Example demonstrating library usage
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

### Developer Tools Added

This template now includes extra developer-quality tools and integrations:

- **Pre-commit hooks**: A `.pre-commit-config.yaml` file is included. Install with:

```bash
pip install pre-commit
pre-commit install
```

- **Unused dependencies**: Use `task udeps` to run `cargo udeps` and detect unused dependencies.
- **Unsafe usage scan**: Use `task geiger` to run `cargo geiger` and get a report of `unsafe` blocks.

See `Taskfile.yml` for the exact commands. These are available inside the Nix development shell so contributors get reproducible tooling.
```

### Without Nix

Install the following tools manually:
- `cargo-nextest` - Fast parallel test runner
- `cargo-llvm-cov` - Code coverage
- `cargo-audit` - Security auditing
- `cargo-expand` - Macro expansion
- `go-task` - Task runner

Then use `task` commands as above.

## Development Tools

This template includes several development tools for improved productivity:

### Mold Linker

[Mold](https://github.com/rui314/mold) is a modern linker that significantly speeds up compilation:

- **Faster linking**: 5-10x faster than default linker
- **Automatic configuration**: Enabled via `RUSTFLAGS` in the Nix environment
- **Cross-platform**: Works on Linux, macOS, and Windows

### Cargo Watch

[`cargo-watch`](https://github.com/watchexec/cargo-watch) automatically runs commands when files change:

```bash
# Run tests on file changes
task watch

# Run checks on file changes
task watch:check
```

### Bacon

[`bacon`](https://github.com/Canop/bacon) provides background code checking with a nice TUI:

```bash
# Start background checking
task bacon
```

Features:
- Real-time feedback as you code
- Clear error display with context
- Fast incremental checking

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
