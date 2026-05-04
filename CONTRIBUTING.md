# Contributing to My Project

Thank you for your interest in contributing! This guide will help you get started.

## Setup

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### With Nix (Recommended)

```bash
nix flake update
nix develop
```

This automatically provides all necessary tools including:
- Rust toolchain with clippy, rustfmt, and miri
- Cargo extensions (nextest, llvm-cov, audit, expand, etc.)
- Package management tools

### Without Nix

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add clippy rustfmt llvm-tools-preview miri
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/my-awesome-feature
```

### 2. Make Your Changes

```bash
# Edit code
vim src/lib.rs

# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings
```

### 3. Test Unsafe Code (if applicable)

If your code contains `unsafe` blocks:

```bash
# Run Miri to detect undefined behavior
task miri

# Run with strict checking (experimental)
task miri:strict
```

### 4. Run Full Quality Gates

```bash
# Run all checks at once
task check

# Or individually:
task fmt      # Format code
task lint     # Run clippy
task test     # Run tests
task cov      # Generate coverage report
```

### 5. Push & Create Pull Request

```bash
git add .
git commit -m "feat: describe your changes"
git push origin feature/my-awesome-feature
```

Create a pull request on GitHub. Our CI will automatically:
- Check formatting (`cargo fmt`)
- Run linting (`cargo clippy`)
- Execute tests on multiple platforms (Linux, macOS, Windows)
- Test with stable and nightly Rust
- Run Miri for undefined behavior detection
- Audit dependencies for vulnerabilities
- Generate coverage reports

## Coding Standards

### Documentation

All public items must have documentation:

```rust
/// Adds two numbers together.
///
/// # Examples
/// ```
/// assert_eq!(my_project::add(2, 2), 4);
/// ```
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Avoid `.unwrap()` and `.expect()` in library code
- Document errors in the `# Errors` section of rustdoc

```rust
/// Reads configuration from a file.
///
/// # Errors
///
/// Returns an error if the file does not exist or is invalid.
pub fn read_config(path: &str) -> Result<Config, ConfigError> {
    // ...
}
```

### Unsafe Code

Unsafe code is restricted. If necessary:
- Document with a `# Safety` section explaining invariants
- Add a test with Miri to detect undefined behavior
- Minimize the scope of the `unsafe` block

```rust
/// # Safety
///
/// The caller must ensure that `ptr` is valid and properly aligned.
pub unsafe fn from_raw(ptr: *const u8) -> Self {
    // ...
}
```

### Testing

- Write unit tests in the same module
- Write integration tests in `tests/`
- Aim for >80% code coverage
- Test edge cases and error paths

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_basic() {
        assert_eq!(add(1, 1), 2);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 5), 5);
    }

    #[test]
    fn test_add_overflow() {
        // Test overflow behavior
    }
}
```

## Code Style

- Follow Rust conventions (see [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/))
- Use `rustfmt` for formatting (enforced in CI)
- Use meaningful variable and function names
- Add comments for complex logic

### Examples of Good Code

```rust
// ✅ Clear naming and structure
fn process_user_input(input: &str) -> Result<Data, ParseError> {
    let trimmed = input.trim();
    validate_input(trimmed)?;
    parse_data(trimmed)
}

// ❌ Unclear abbreviations
fn proc_ui(s: &str) -> Result<D, E> {
    // ...
}
```

## Commit Messages

Write clear, descriptive commit messages:

```
feat: add user authentication

- Implement JWT token validation
- Add login endpoint
- Update tests

Fixes #123
```

Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `refactor:` for code refactoring
- `test:` for adding/updating tests
- `chore:` for maintenance

## Performance Considerations

When appropriate, add benchmarks:

```bash
task bench
```

Benchmarks help ensure performance improvements don't regress.

## Questions?

- Check existing issues and discussions
- Read the README for project overview
- Review SECURITY.md for safety guidelines
- Open a discussion or issue on GitHub

## Code of Conduct

Please be respectful and constructive. We value:
- Inclusive language
- Constructive feedback
- Patience with newcomers
- Collaboration over confrontation

## Legal

By contributing, you agree that your contributions will be licensed under the project's license (MIT OR Apache-2.0).
