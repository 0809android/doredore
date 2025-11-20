# Contributing to doredore

Thank you for your interest in contributing to doredore! 🎉

## Getting Started

### Prerequisites

- Rust 1.87 or later
- Python 3.8 or later (for Python bindings)
- Node.js 14 or later (for Node.js bindings)
- Ruby 2.7 or later (for Ruby bindings)
- Docker (optional, for containerized development)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/doredore.git
cd doredore

# Build the Rust core
cargo build

# Build Python bindings
cd doredore-py
maturin develop

# Build Node.js bindings (requires Rust 1.91)
cd ../doredore-js
npm install
npm run build

# Build Ruby bindings
cd ../doredore-rb
cargo build --release

# Build REST API server
cd ../doredore-server
cargo build --release
```

## Project Structure

```
doredore/
├── doredore-core/      # Rust core library
├── doredore-py/        # Python bindings (PyO3)
├── doredore-js/        # Node.js bindings (NAPI-rs)
├── doredore-rb/        # Ruby bindings (FFI)
├── doredore-server/    # REST API server (Axum)
└── examples/               # Example code
```

## How to Contribute

### Reporting Bugs

- Use the GitHub issue tracker
- Include detailed reproduction steps
- Provide system information (OS, Rust version, etc.)
- Include error messages and logs

### Suggesting Features

- Open a GitHub issue with the "enhancement" label
- Describe the feature and use case
- Explain why it would be useful

### Submitting Pull Requests

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```

3. **Make your changes**
   - Write clear, concise commit messages
   - Follow the existing code style
   - Add tests if applicable
   - Update documentation

4. **Test your changes**
   ```bash
   # Run Rust tests
   cargo test

   # Run Python tests
   cd doredore-py
   python test_simple.py

   # Run server tests
   cd ../doredore-server
   cargo test
   ```

5. **Submit the pull request**
   - Describe what the PR does
   - Reference any related issues
   - Wait for review

## Code Style

### Rust
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write documentation comments for public APIs

### Python
- Follow [PEP 8](https://pep8.org/)
- Use type hints
- Write docstrings for public functions

### JavaScript/TypeScript
- Use 2 spaces for indentation
- Follow [Standard JS](https://standardjs.com/) style
- Provide TypeScript type definitions

### Ruby
- Follow [Ruby Style Guide](https://rubystyle.guide/)
- Use 2 spaces for indentation
- Write RDoc comments

## Testing

### Running Tests

```bash
# Rust core tests
cargo test --package doredore-core

# Python integration tests
cd doredore-py
python test_simple.py
python examples/python/basic.py

# Server tests
cd ../doredore-server
cargo test

# Ruby tests (manual for now)
cd ../doredore-rb
ruby examples/ruby/basic.rb
```

### Writing Tests

- Write unit tests for new functions
- Write integration tests for new features
- Ensure all tests pass before submitting PR

## Documentation

- Update README.md if adding new features
- Add usage examples for new functionality
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/)
- Write clear API documentation

## Release Process

1. Update version in all `Cargo.toml` files
2. Update version in `pyproject.toml`
3. Update version in `package.json`
4. Update `CHANGELOG.md`
5. Create a git tag
6. Build and publish packages

## Community

- Be respectful and constructive
- Help others in issues and discussions
- Share your use cases and experiences

## License

By contributing to doredore, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue or reach out to the maintainers!

---

**Thank you for contributing!** 🙏
