# 🎯 Ballistics Analyzer

[![Build Status](https://github.com/DatilDev/Ballistics-Analyzer/workflows/Build%20and%20Release/badge.svg)](https://github.com/DatilDev/Ballistics-Analyzer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PWA Ready](https://img.shields.io/badge/PWA-Ready-brightgreen.svg)](https://datildev.github.io/Ballistics-Analyzer/)
[![Rust](https://img.shields.io/badge/rust-%E2%9C%94-orange.svg)](https://www.rust-lang.org)
[![GitHub release](https://img.shields.io/github/release/DatilDev/Ballistics-Analyzer.svg)](https://github.com/DatilDev/Ballistics-Analyzer/releases)

Professional-grade ballistics calculation software with hardware integration, available as a Progressive Web App and native applications for all platforms.

<div align="center">
  <img src="assets/screenshot-main.png" alt="Ballistics Analyzer Main Screen" width="600">
</div>

## 🌐 Try It Now

### **[Launch Ballistics Analyzer PWA →](https://datildev.github.io/Ballistics-Analyzer/)**

Works instantly in any modern browser - install it as an app for offline access!

## ✨ Features

### 🎯 Core Ballistics Engine
- **Advanced Trajectory Modeling** - Modified point mass calculations with atmospheric corrections
- **Multiple Drag Models** - Support for G1, G7, and custom drag coefficients
- **Environmental Corrections** - Temperature, pressure, humidity, altitude, and Coriolis effect
- **Wind Modeling** - Multi-zone wind with gradient calculations
- **Scope Adjustments** - MOA and MIL corrections for elevation and windage
- **Zero Calculations** - Automatic sight-in adjustments for any distance

### 🔫 Firearm Management
- **Firearm Profiles** - Store unlimited rifle, pistol, and shotgun configurations
- **Ammunition Library** - Pre-loaded factory ammo data from major manufacturers
- **Custom Loads** - Add your own handload data with powder specifications
- **Barrel Twist Calculator** - Stability calculations based on Miller formula
- **Profile Sharing** - Export/import profiles via JSON or Nostr protocol

### 📡 Hardware Integration
- **Bluetooth Rangefinders** - Direct connection to Sig KILO, Leica CRF, Vortex Fury
- **Weather Meters** - Auto-import from Kestrel 5700, WeatherFlow devices
- **Real-time Updates** - Live environmental data feeds into calculations
- **Auto-apply** - Seamless data integration with manual override options
- **Hardware Status** - Connection monitoring and battery indicators

### 🔐 Privacy & Security
- **Local-First Architecture** - All data stored encrypted on your device
- **Nostr Authentication** - Decentralized identity with no central servers
- **Zero Tracking** - No analytics, telemetry, or user monitoring
- **End-to-End Encryption** - Secure sharing via cryptographic keys
- **Open Source** - Complete transparency and auditability

### 📱 Progressive Web App
- **Install Anywhere** - Works on Windows, macOS, Linux, Android, iOS
- **Offline Mode** - Full functionality without internet connection
- **Auto Updates** - Seamless updates when connected
- **Native Features** - Camera access, Bluetooth, push notifications
- **Small Size** - < 5MB initial download, efficient caching

## 📦 Installation

### Primary Platforms

#### Arch Linux
```bash
# From AUR (when available)
yay -S ballistics-analyzer

# From source
git clone https://github.com/DatilDev/Ballistics-Analyzer.git
cd Ballistics-Analyzer
make build-arch
sudo make install

# Download APK from releases
# Or build from source:
export ANDROID_HOME=/path/to/sdk
make build-android
# APK will be in ballistics-mobile/android/app/build/outputs/apk/

## 🚀 Quick Start Guide

### First Time Setup
1. **Launch the app** from web or desktop
2. **Create identity**:
   - Generate new Nostr keys, or
   - Import existing nsec/hex key, or
   - Connect via NIP-07 extension
3. **Create firearm profile**:
   - Enter rifle/pistol specifications
   - Set sight height and zero distance
4. **Connect hardware** (optional):
   - Pair Bluetooth rangefinder
   - Connect weather meter

### Basic Calculation
1. **Enter projectile data**:
   - Caliber and bullet weight
   - Muzzle velocity
   - Ballistic coefficient (G1/G7)
2. **Set environmental conditions**:
   - Temperature, pressure, altitude
   - Wind speed and direction
3. **Calculate trajectory**:
   - View drop/drift table
   - See MOA/MIL adjustments
   - Export or share results

### Advanced Features
- **Photo attachments** - Document shots with images
- **Notes system** - Add detailed observations
- **History tracking** - Review past calculations
- **Batch calculations** - Multiple ranges at once
- **Trajectory graphs** - Visual representation
- **Print reports** - Formatted range cards

## 🛠️ Building from Source

### Prerequisites
- [Rust](https://rustup.rs/) 1.75+
- [Node.js](https://nodejs.org/) 18+ (for PWA tools)
- Platform-specific requirements:
  - **Linux**: `libgtk-3-dev libssl-dev pkg-config`
  - **Windows**: Visual Studio Build Tools 2019+
  - **macOS**: Xcode Command Line Tools

### Clone and Build

```bash
# Clone repository
git clone https://github.com/DatilDev/Ballistics-Analyzer.git
cd Ballistics-Analyzer

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build desktop application
cargo build --release

# Run desktop app
cargo run --release

# Build PWA (requires wasm-pack)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack build --target web --out-dir pkg --release

# Serve PWA locally
cd dist && python3 -m http.server 8000
# Open http://localhost:8000
```

### Platform-Specific Builds

#### Windows
```powershell
# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc
# Output: target/release/ballistics-analyzer.exe
```

#### macOS
```bash
# Build for macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# Build for macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create target/x86_64-apple-darwin/release/ballistics-analyzer \
             target/aarch64-apple-darwin/release/ballistics-analyzer \
     -output ballistics-analyzer-universal
```

#### Linux
```bash
# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Create AppImage (optional)
./create-appimage.sh

# Build Arch package
makepkg -si
```

#### Android (Development)
```bash
# Install Android tools
rustup target add aarch64-linux-android
cargo install cargo-ndk

# Build libraries
cargo ndk -t arm64-v8a -t armeabi-v7a build --release
```

## 📡 Supported Hardware

### Rangefinders (Bluetooth LE)
| Brand | Models | Features |
|-------|--------|----------|
| Sig Sauer | KILO2200BDX, KILO2400ABS, KILO3000BDX | Range, angle, temperature |
| Leica | Rangemaster CRF 2800, CRF 3500 | Range, angle, pressure |
| Vortex | Fury HD 5000AB, Razor HD 4000GB | Range, angle, bearing |
| ATN | ABL 1000, ABL 1500 | Range, angle |

### Weather Meters
| Brand | Models | Features |
|-------|--------|----------|
| Kestrel | 5700 Elite, DROP D3 | Full environmental suite |
| WeatherFlow | WEATHERmeter | Wind, temp, pressure |
| Bluetooth Stations | Generic BLE weather | Basic environmental |

### Connection Guide
1. Enable Bluetooth on device
2. Open Hardware panel (📡 icon)
3. Click "Connect" for device type
4. Select from discovered devices
5. Authorize pairing if prompted
6. Data auto-populates in calculations

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup
```bash
# Install development tools
cargo install cargo-watch cargo-audit cargo-tarpaulin

# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check

# Security audit
cargo audit
```

### Areas for Contribution
- 🐛 Bug fixes and testing
- 📚 Documentation improvements
- 🌍 Translations (i18n support)
- 🎨 UI/UX enhancements
- 📡 Additional hardware support
- 🧮 Advanced ballistics models
- 📱 Mobile app development

## 📊 Technical Architecture

### Core Stack
- **Language**: Rust (performance & safety)
- **GUI Framework**: egui (immediate mode)
- **Web Target**: WebAssembly (wasm-bindgen)
- **Async Runtime**: tokio (desktop) / wasm-bindgen-futures (web)
- **Database**: SQLite (desktop) / IndexedDB (web)
- **Networking**: Nostr protocol for sharing

### Performance
- 60 FPS rendering on all platforms
- < 100ms calculation time for 1000-yard trajectory
- < 50MB RAM usage (typical)
- < 5MB initial PWA download
- Offline-first with background sync

### Project Structure
```
Ballistics-Analyzer/
├── src/
│   ├── main.rs              # Desktop entry point
│   ├── lib.rs               # Library root
│   ├── ballistics.rs        # Core calculations
│   ├── hardware.rs          # Device integration
│   ├── auth.rs              # Nostr authentication
│   ├── storage.rs           # Data persistence
│   └── ui.rs                # Interface components
├── assets/                  # Icons and resources
├── .github/workflows/       # CI/CD pipelines
├── Cargo.toml              # Rust dependencies
├── index.html              # PWA entry
├── manifest.json           # PWA manifest
└── sw.js                   # Service worker
```

## 🗺️ Roadmap

### Version 1.0 ✅ (Current)
- [x] Core ballistics engine
- [x] PWA with offline support
- [x] Desktop applications
- [x] Bluetooth hardware integration
- [x] Nostr authentication
- [x] Profile management

### Version 1.1 (UNK)
- [ ] 6DOF calculations
- [ ] Doppler radar support
- [ ] Multi-zone wind profiles
- [ ] Reloading database
- [ ] Export to Applied Ballistics format
- [ ] Backup/restore via Nostr relays

### Version 2.0 (UNK)
- [ ] AR trajectory overlay
- [ ] AI shot correction
- [ ] Team synchronization
- [ ] Competition mode
- [ ] Voice commands
- [ ] Watch app support

## 📚 Documentation

- [User Guide](docs/USER_GUIDE.md) - Complete usage instructions
- [API Reference](docs/API.md) - Developer documentation
- [Hardware Guide](docs/HARDWARE.md) - Device setup instructions
- [Ballistics Theory](docs/THEORY.md) - Mathematical models explained
- [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2025 Ballistics Analyzer Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction...
```

## 🙏 Acknowledgments

- [Nostr Protocol](https://nostr.com) - Decentralized networking
- [egui](https://github.com/emilk/egui) - Immediate mode GUI
- [Rust Community](https://rust-lang.org) - Language and ecosystem
- [JBM Ballistics](http://www.jbmballistics.com/) - Ballistics research
- All open source contributors

## 📞 Support

- **Bug Reports**: [GitHub Issues](https://github.com/DatilDev/Ballistics-Analyzer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/DatilDev/Ballistics-Analyzer/discussions)
- **Security**: [Security Policy](SECURITY.md)
- **Email**: support@datildev.com
- **Nostr**: `#ballisticsanalyzer`

## 🏆 Sponsors

This project is maintained by volunteers. Support development:

- ⭐ Star this repository
- 🐛 Report bugs and test features
- 💡 Suggest improvements
- 🤝 Contribute code
- ☕ [Buy us a coffee](https://ko-fi.com/ballisticsanalyzer)
- ⚡ [Lightning tips](lightning:ballisticsanalyzer@getalby.com)

## 📈 Stats

![GitHub stars](https://img.shields.io/github/stars/DatilDev/Ballistics-Analyzer?style=social)
![GitHub forks](https://img.shields.io/github/forks/DatilDev/Ballistics-Analyzer?style=social)
![GitHub watchers](https://img.shields.io/github/watchers/DatilDev/Ballistics-Analyzer?style=social)

---

<div align="center">

**⚠️ Safety Disclaimer**

This software is for educational and sporting purposes only. Always verify calculations with real-world testing. Follow all firearm safety rules and local regulations. The developers assume no liability for the use or misuse of this software.

**Made with ❤️ by the Ballistics Community**

[Website](https://datildev.github.io/Ballistics-Analyzer/) • [Documentation](docs/) • [Releases](https://github.com/DatilDev/Ballistics-Analyzer/releases) • [Issues](https://github.com/DatilDev/Ballistics-Analyzer/issues)

</div>