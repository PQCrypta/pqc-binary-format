# Contributing to PQC Binary Format

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful and professional. We're all here to advance post-quantum cryptography.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/PQCrypta/pqcrypta-community/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Code samples if applicable
   - Rust version and OS

### Suggesting Features

1. Open an issue with the "enhancement" label
2. Describe the feature and its use case
3. Explain why it would be valuable to users

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run clippy (`cargo clippy`)
7. Format code (`cargo fmt`)
8. Commit with clear messages
9. Push to your fork
10. Open a Pull Request

### Code Standards

- Follow Rust API guidelines
- Write documentation for public APIs
- Add examples for new features
- Maintain test coverage above 80%
- Keep functions focused and small
- Use meaningful variable names

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Check documentation
cargo doc --open
```

### Commit Messages

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit first line to 72 characters
- Reference issues and PRs liberally

Example:
```
Add support for custom algorithm identifiers

- Extend Algorithm enum with Custom variant
- Update binary format validation
- Add tests for custom algorithms

Fixes #123
```

## Project Structure

```
pqc-binary-format/
├── src/
│   ├── lib.rs          # Main library entry
│   ├── algorithm.rs    # Algorithm identifiers
│   ├── error.rs        # Error types
│   ├── format.rs       # Core format implementation
│   └── metadata.rs     # Metadata structures
├── examples/           # Usage examples
├── benches/            # Performance benchmarks
├── tests/              # Integration tests
└── docs/               # Additional documentation
```

## Areas Needing Help

### High Priority
- **Python bindings** using PyO3
- **JavaScript/WebAssembly** port
- **C FFI** for cross-language compatibility
- **Fuzzing** with cargo-fuzz
- **Documentation** improvements

### Medium Priority
- **Go bindings** using cgo
- **CLI tool** for format inspection
- **Performance optimizations**
- **Additional examples**

### Research
- **IETF RFC draft** for standardization
- **Security audit** by third parties
- **Formal verification** of checksum algorithm

## Getting Help

- **GitHub Discussions**: Ask questions
- **Issues**: Report bugs
- **Documentation**: Read the docs first

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in documentation

## License

By contributing, you agree that your contributions will be licensed under both the MIT and Apache 2.0 licenses.

---

Thank you for contributing to post-quantum cryptography! 🔐
