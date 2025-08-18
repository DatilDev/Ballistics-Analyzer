# 🎯 Ballistics Analyzer

[![Build Status](https://github.com/DatilDev/Ballistics-Analyzer/workflows/Build%20and%20Release/badge.svg)](https://github.com/DatilDev/Ballistics-Analyzer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%E2%9C%94-orange.svg)](https://www.rust-lang.org)
[![GitHub release](https://img.shields.io/github/release/DatilDev/Ballistics-Analyzer.svg)](https://github.com/DatilDev/Ballistics-Analyzer/releases)

Professional-grade ballistics calculation software with hardware integration, available as native applications for desktop and mobile platforms.

<div align="center">
  <img src="assets/screenshot-main.png" alt="Ballistics Analyzer Main Screen" width="600">
</div>

> **Note**: The Progressive Web App (PWA) version is temporarily unavailable while we upgrade the web assembly infrastructure. Desktop and mobile applications remain fully functional.

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

## 📦 Installation

### Desktop Applications
Download from [Latest Release](https://github.com/DatilDev/Ballistics-Analyzer/releases/latest):

| Platform | Download | Requirements |
|----------|----------|--------------|
| Windows | [ballistics-analyzer-windows.exe](https://github.com/DatilDev/Ballistics-Analyzer/releases/latest/download/ballistics-analyzer-windows.exe) | Windows 10+ |
| macOS | [ballistics-analyzer-macos](https://github.com/DatilDev/Ballistics-Analyzer/releases/latest/download/ballistics-analyzer-macos) | macOS 11+ |
| Linux | [ballistics-analyzer-linux](https://github.com/DatilDev/Ballistics-Analyzer/releases/latest/download/ballistics-analyzer-linux) | GTK3, glibc 2.31+ |
| Arch Linux | [ballistics-analyzer.pkg.tar.zst](https://github.com/DatilDev/Ballistics-Analyzer/releases/latest/download/ballistics-analyzer.pkg.tar.zst) | Via pacman |

### Mobile Apps
- **Android**: APK coming soon (development build available)
- **iOS**: TestFlight beta available on request

## 🚀 Quick Start Guide

### First Time Setup
1. **Launch the app** from desktop or mobile
2. **Create identity** (optional):
   - Generate new Nostr keys, or
   - Import existing nsec/hex key, or
   - Skip for local-only usage
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
cargo build --release -p ballistics-desktop

# Run desktop app
cargo run --release -p ballistics-desktop

Platform-Specific Builds
Windows
# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc -p ballistics-desktop
# Output: target/release/ballistics-analyzer.exe

macOS
# Build for macOS (Intel)
cargo build --release --target x86_64-apple-darwin -p ballistics-desktop

# Build for macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin -p ballistics-desktop

# Create universal binary
lipo -create target/x86_64-apple-darwin/release/ballistics-analyzer \
             target/aarch64-apple-darwin/release/ballistics-analyzer \
     -output ballistics-analyzer-universal
Linux
# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu -p ballistics-desktop

# Create AppImage (optional)
./scripts/create-appimage.sh

Android (Development)

# Install Android tools
rustup target add aarch64-linux-android
cargo install cargo-ndk

# Build libraries
cd ballistics-mobile
cargo ndk -t arm64-v8a -t armeabi-v7a build --release

📡 Supported Hardware
Rangefinders (Bluetooth LE)
BrandModelsFeaturesSig SauerKILO2200BDX, KILO2400ABS, KILO3000BDXRange, angle, temperatureLeicaRangemaster CRF 2800, CRF 3500Range, angle, pressureVortexFury HD 5000AB, Razor HD 4000GBRange, angle, bearingATNABL 1000, ABL 1500Range, angle
Weather Meters
BrandModelsFeaturesKestrel5700 Elite, DROP D3Full environmental suiteWeatherFlowWEATHERmeterWind, temp, pressure
🤝 Contributing
We welcome contributions! See CONTRIBUTING.md for guidelines.
Development Setup

# Install development tools
cargo install cargo-watch cargo-audit cargo-tarpaulin

# Run with auto-reload
cargo watch -x 'run -p ballistics-desktop'

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check

# Security audit
cargo audit

Areas for Contribution

🐛 Bug fixes and testing
📚 Documentation improvements
🌍 Translations (i18n support)
🎨 UI/UX enhancements
📡 Additional hardware support
🧮 Advanced ballistics models
📱 Mobile app development

📊 Technical Architecture
Core Stack

Language: Rust (performance & safety)
GUI Framework: egui (immediate mode)
Database: SQLite (desktop) / JSON (mobile)
Networking: Nostr protocol for sharing

Performance

60 FPS rendering on all platforms
< 100ms calculation time for 1000-yard trajectory
< 50MB RAM usage (typical)
Offline-first with background sync

Project Structure
Ballistics-Analyzer/
├── ballistics_core/         # Core calculation library
├── ballistics-desktop/      # Desktop application
├── ballistics-mobile/       # Mobile application
├── scripts/                 # Build scripts
└── assets/                  # Icons and resources

🗺️ Roadmap
Version 1.0 ✅ (Current)

 Core ballistics engine
 Desktop applications
 Mobile framework
 Bluetooth hardware integration
 Nostr authentication
 Profile management

Version 1.1 (Q2 2025)

 6DOF calculations
 Doppler radar support
 Multi-zone wind profiles
 Reloading database
 Export to Applied Ballistics format

Version 1.2 (Q3 2025)

 Progressive Web App (PWA) re-launch
 WebAssembly optimization
 Offline-first web experience

Version 2.0 (Q4 2025)

 AR trajectory overlay
 AI shot correction
 Team synchronization
 Competition mode
 Voice commands

📚 Documentation

User Guide - Complete usage instructions
API Reference - Developer documentation
Hardware Guide - Device setup instructions
Ballistics Theory - Mathematical models explained
Troubleshooting - Common issues and solutions

📄 License
This project is licensed under the MIT License - see LICENSE file for details.
🙏 Acknowledgments

Nostr Protocol - Decentralized networking
egui - Immediate mode GUI
Rust Community - Language and ecosystem
JBM Ballistics - Ballistics research
All open source contributors

📞 Support

Bug Reports: GitHub Issues
Discussions: GitHub Discussions
Security: Security Policy
Email: support@datildev.com

📈 Stats

https://img.shields.io/github/forks/DatilDev/Ballistics-Analyzer?style=social

⚠️ Safety Disclaimer
This software is for educational and sporting purposes only. Always verify calculations with real-world testing. Follow all firearm safety rules and local regulations. The developers assume no liability for the use or misuse of this software.
Made with ❤️ by the Ballistics Community
Documentation • Releases • Issues
</div>
```
Key changes made:

Removed PWA/web app sections from the intro
Added a note explaining PWA is temporarily unavailable
Removed web app installation instructions
Removed PWA-specific features from the features list
Updated build instructions to focus on desktop and mobile only
Added PWA re-launch to the roadmap (Version 1.2)
Removed references to web-based functionality
Updated build commands to use -p ballistics-desktop flag
