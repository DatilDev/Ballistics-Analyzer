# IronSights - Android & Arch Linux Edition

This branch contains the Android and Arch Linux specific builds only.

## Supported Platforms
- **Arch Linux** (x86_64, aarch64)
- **Android** (API 21+, ARM64, ARMv7)

## Building

### Arch Linux
```bash
git checkout feature/android-arch-only
makepkg -si
# or
make build-arch

### Arch Linux

git checkout feature/android-arch-only
export ANDROID_HOME=/path/to/sdk
make build-android