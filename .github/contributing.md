# Contributing to Open Payments Rust

Thank you for your interest in contributing to the Open Payments Rust client library! This document provides guidelines and information for contributors.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

- Use the [GitHub issue tracker](https://github.com/interledger/open-payments-rust/issues)
- Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md)
- Include detailed steps to reproduce the bug
- Provide your environment details (OS, Rust version, etc.)
- Include any error messages or logs

### Suggesting Enhancements

- Use the [GitHub issue tracker](https://github.com/interledger/open-payments-rust/issues)
- Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md)
- Describe the enhancement clearly
- Explain why this enhancement would be useful
- Consider the impact on existing functionality

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run clippy (`cargo clippy --all-targets --all-features -- -D warnings`)
7. Format your code (`cargo fmt --all`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to the branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request using the [PR template](.github/PULL_REQUEST_TEMPLATE.md)

## Development Setup

### Prerequisites

- Rust 1.43.0 or later
- Git

### Building

```bash
git clone https://github.com/interledger/open-payments-rust.git
cd open-payments-rust
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with snippets feature
cargo test --features snippets

# Run doc tests
cargo test --doc

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

### Documentation

```bash
# Generate documentation
cargo doc --no-deps

# Generate documentation with snippets
cargo doc --features snippets --no-deps

# Open documentation in browser
cargo doc --no-deps --open
```

## Code Style

- Follow Rust conventions and idioms
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Write comprehensive tests
- Add documentation for public APIs
- Use meaningful variable and function names
- Keep functions small and focused
- Add doc comments for all public items

## Testing Guidelines

- Write unit tests for all new functionality
- Ensure integration tests cover the main use cases
- Test both success and error scenarios
- Use descriptive test names
- Mock external dependencies when appropriate

## Documentation Guidelines

- Add doc comments for all public APIs
- Include usage examples in doc comments
- Update README.md when adding new features
- Keep documentation up to date with code changes

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense
- Keep the first line under 50 characters
- Add more details in the body if needed
- Reference issues when applicable

## Release Process

1. Update version in `Cargo.toml`
3. Create a git tag
4. Push the tag to trigger release workflow

## Questions?

If you have questions about contributing, please:
- Open an issue using the [question template](.github/ISSUE_TEMPLATE/question.md)
- Reach out to the maintainers at tech@interledger.org
- Join our [community discussions](https://github.com/interledger/open-payments-rust/discussions)

## Getting Help

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Open Payments Documentation](https://openpayments.dev/)
- [Interledger Documentation](https://interledger.org/developers/) 