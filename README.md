# 🔨 Forge

[![CI](https://github.com/nhudson/forge/actions/workflows/ci.yml/badge.svg)](https://github.com/nhudson/forge/actions/workflows/ci.yml)
[![Release](https://github.com/nhudson/forge/actions/workflows/release.yml/badge.svg)](https://github.com/nhudson/forge/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast, reliable Rust-based CLI tool for converting PFX/P12 certificate files to PEM format. Forge provides a modern, user-friendly interface with beautiful output formatting and comprehensive error handling.

## ✨ Features

- 🔐 **Secure Conversion**: Convert PFX/P12 files to PEM format with OpenSSL
- 🔑 **Password Support**: Handle password-protected certificate files
- 🔗 **Certificate Chains**: Extract and save complete certificate chains
- 📁 **Flexible Output**: Customizable output directories and filenames
- 🎨 **Beautiful CLI**: Colorized output with progress indicators and formatted tables
- ⚡ **Fast & Reliable**: Built in Rust for performance and safety
<!-- - 🧪 **Well Tested**: Comprehensive test suite with integration tests -->

## 📦 Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/nhudson/forge/releases):

> **Note for Windows users**: Pre-built Windows binaries include OpenSSL statically linked, so no additional OpenSSL installation is required.

```bash
# Linux (x86_64) - replace {version} with the latest version (e.g., 0.1.0)
curl -L https://github.com/nhudson/forge/releases/latest/download/forge-linux-x86_64-{version}.tar.gz | tar xz
sudo mv forge /usr/local/bin/

# macOS (Intel) - replace {version} with the latest version
curl -L https://github.com/nhudson/forge/releases/latest/download/forge-macos-x86_64-{version}.tar.gz | tar xz
sudo mv forge /usr/local/bin/

# macOS (Apple Silicon) - replace {version} with the latest version
curl -L https://github.com/nhudson/forge/releases/latest/download/forge-macos-aarch64-{version}.tar.gz | tar xz
sudo mv forge /usr/local/bin/

# Windows
# Download forge-windows-x86_64-{version}.zip from the releases page and extract forge.exe
# No additional OpenSSL installation required!
```

Or use these one-liners that automatically fetch the latest version:

```bash
# Linux (x86_64)
curl -L https://github.com/nhudson/forge/releases/latest/download/forge-linux-x86_64-$(curl -s https://api.github.com/repos/nhudson/forge/releases/latest | grep tag_name | cut -d '"' -f 4 | sed 's/v//').tar.gz | tar xz
sudo mv forge /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/nhudson/forge/releases/latest/download/forge-macos-x86_64-$(curl -s https://api.github.com/repos/nhudson/forge/releases/latest | grep tag_name | cut -d '"' -f 4 | sed 's/v//').tar.gz | tar xz
sudo mv forge /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/nhudson/forge/releases/latest/download/forge-macos-aarch64-$(curl -s https://api.github.com/repos/nhudson/forge/releases/latest | grep tag_name | cut -d '"' -f 4 | sed 's/v//').tar.gz | tar xz
sudo mv forge /usr/local/bin/
```

### Docker

```bash
# Run directly
docker run --rm -v $(pwd):/workspace ghcr.io/nhudson/forge:latest --pfx /workspace/cert.pfx --out /workspace/output

# Pull the image
docker pull ghcr.io/nhudson/forge:latest
```

### Cargo Install

If you have Rust installed, you can install forge directly from crates.io or from the Git repository:

```bash
# Install from crates.io
cargo install forge-pfx

# Install from Git repository (latest development version)
cargo install --git https://github.com/nhudson/forge

# Install a specific version from Git
cargo install --git https://github.com/nhudson/forge --tag {version}
```

### From Source

Requires Rust 1.85+ and OpenSSL development libraries:

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev

# Install dependencies (macOS)
brew install openssl pkg-config

# Install dependencies (Windows)
# Option 1: Use vcpkg (recommended)
vcpkg install openssl:x64-windows-static

# Option 2: Download and install OpenSSL from https://slproweb.com/products/Win32OpenSSL.html
# Then set environment variables:
# set OPENSSL_DIR=C:\Program Files\OpenSSL-Win64

# Build and install
cargo install --git https://github.com/nhudson/forge
```

> **Windows Note**: When building from source on Windows, you'll need OpenSSL development libraries. The vcpkg method is recommended as it provides static libraries that don't require runtime dependencies.

## 🚀 Usage

### Basic Usage

```bash
# Convert a PFX file (no password)
forge --pfx certificate.pfx

# Convert with password
forge --pfx certificate.pfx --password mypassword

# Specify output directory
forge --pfx certificate.pfx --out ./certificates/
```

### Advanced Options

```bash
# Create combined PEM file (private key + certificate)
forge --pfx certificate.pfx --combined

# Extract certificate chain
forge --pfx certificate.pfx --chain

# Custom filenames
forge --pfx certificate.pfx \
  --key-file my-private-key.pem \
  --cert-file my-certificate.pem \
  --combined-file my-combined.pem

# Verbose output with detailed information
forge --pfx certificate.pfx --verbose
```

### Complete Example

```bash
forge --pfx certificate.pfx \
  --password "mypassword" \
  --out ./output/ \
  --combined \
  --chain \
  --verbose
```

This will:
- Convert `certificate.pfx` using password "mypassword"
- Save all files to `./output/` directory
- Create a combined PEM file with private key and certificate
- Extract the complete certificate chain
- Show detailed progress and certificate information

## 📋 Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--pfx` | Path to the PFX/P12 certificate file | **Required** |
| `--password` | Password for the PFX file | Empty string |
| `--out` | Output directory for PEM files | Current directory |
| `--combined` | Create combined PEM file (key + cert) | `false` |
| `--chain` | Extract complete certificate chain | `false` |
| `--key-file` | Custom private key filename | `private_key.pem` |
| `--cert-file` | Custom certificate filename | `certificate.pem` |
| `--combined-file` | Custom combined file filename | `certificate_with_key.pem` |
| `--verbose` | Enable verbose output | `false` |

## 📁 Output Files

Forge generates the following files based on your options:

- `private_key.pem` - Private key in PKCS#8 PEM format
- `certificate.pem` - Main certificate in PEM format
- `certificate_with_key.pem` - Combined file (if `--combined` is used)
- `certificate_chain.pem` - Complete chain (if `--chain` is used)
- `chain_cert_N.pem` - Individual chain certificates (if `--chain` is used)

## 🔧 Development

### Prerequisites

- Rust 1.85+
- OpenSSL development libraries
- Git

### Building

```bash
git clone https://github.com/nhudson/forge.git
cd forge

# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with example
cargo run -- --pfx examples/test.pfx --verbose
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests

# Run with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Security audit
cargo audit
```

## 🐳 Docker Development

```bash
# Build Docker image
docker build -t forge .

# Run in container
docker run --rm -v $(pwd)/examples:/workspace forge --pfx /workspace/test.pfx
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [OpenSSL](https://www.openssl.org/)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Beautiful terminal output with [colored](https://github.com/colored-rs/colored) and [tabled](https://github.com/zhiburt/tabled)

## 📞 Support

- 🐛 [Report bugs](https://github.com/nhudson/forge/issues)
- 💡 [Request features](https://github.com/nhudson/forge/issues)
<!-- - 📖 [Documentation](https://github.com/nhudson/forge/wiki) -->

---