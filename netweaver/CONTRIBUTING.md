# Contributing to NetWeaver

First off, thank you for considering contributing to NetWeaver! It's people like you who make this tool better for everyone in the networking community.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include as many details as possible:

- **Description**: Clear and concise description of the bug
- **Steps to reproduce**: Exact commands and environment needed to trigger the issue
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: OS, kernel version, Rust version, NetWeaver version
- **Logs**: Relevant error messages or stack traces

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use case**: Why would this enhancement be useful?
- **Proposed solution**: How should this work?
- **Alternatives**: What alternatives have you considered?
- **Examples**: Mock commands or API designs if applicable

### Pull Requests

1. Fork the repository and create your branch from `main`
2. Make your changes following our coding standards
3. Add tests if you've added code that should be tested
4. Ensure the test suite passes: `cargo test`
5. Run the formatter: `cargo fmt`
6. Run clippy: `cargo clippy`
7. Update documentation if needed
8. Submit a pull request!

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/netweaver.git
cd netweaver

# Build the project
cargo build

# Run tests
cargo test

# Try your changes
cargo run -- scan --lan
```

## Coding Standards

### Rust Code
- Follow standard Rust conventions
- Use `rustfmt` for formatting
- Address all `clippy` warnings
- Write descriptive variable names
- Add comments for complex logic
- Keep functions focused and small

### C Code
- Follow Linux kernel style guidelines
- Use consistent indentation (4 spaces)
- Always check return values
- Free allocated memory
- Document non-obvious code

### Commit Messages
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit first line to 72 characters
- Reference issues and PRs when relevant

Example:
```
Add DNS leak detection to VPN testing

Implements comprehensive DNS leak testing by querying multiple
DNS providers and checking for location inconsistencies.

Closes #123
```

## Project Structure

```
netweaver/
├── src/              # Rust source code
│   ├── cli/          # Command-line interface
│   ├── scanner/      # Network scanning
│   ├── diagnostics/  # Trace and inspection
│   ├── optimizer/    # Performance tuning
│   ├── monitor/      # Real-time monitoring
│   ├── security/     # Security auditing
│   ├── analytics/    # Data analysis
│   └── utils/        # Shared utilities
├── c_core/           # C implementation
│   ├── include/      # Header files
│   └── src/          # C source files
├── tests/            # Integration tests
└── docs/             # Documentation
```

## Testing

- Write unit tests for new functionality
- Add integration tests for complete features
- Test on multiple platforms when possible
- Include edge cases in your tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Documentation

- Document public APIs with doc comments
- Update README.md for user-facing changes
- Add examples for new features
- Keep documentation clear and concise

## Questions?

Feel free to ask questions in:
- GitHub Discussions
- Issue comments
- Pull request comments

Thank you for contributing to NetWeaver! 🎉
