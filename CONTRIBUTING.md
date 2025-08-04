# Contributing to Ballistics Analyzer

Thank you for your interest in contributing! We welcome all contributions that improve the project.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/ballistics-analyzer.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Test thoroughly
6. Commit: `git commit -m "Add your feature"`
7. Push: `git push origin feature/your-feature`
8. Create a Pull Request

## Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-watch
cargo install wasm-pack
cargo install cargo-audit

# Install Node.js dependencies (for PWA)
npm install

# Run development server
cargo watch -x run

# Run tests
cargo test