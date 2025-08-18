#!/bin/bash
# scripts/setup-android.sh

set -e

echo "Setting up Android build environment for Ballistics Analyzer..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install Rust first."
    exit 1
fi

# Install Android targets for Rust
echo "Installing Android targets..."
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# Install cargo-ndk for easier Android builds
echo "Installing cargo-ndk..."
cargo install cargo-ndk

# Check if Android SDK is installed
if [ -z "$ANDROID_SDK_ROOT" ] && [ -z "$ANDROID_HOME" ]; then
    echo "Android SDK not found. Please install Android SDK and set ANDROID_SDK_ROOT or ANDROID_HOME"
    echo "You can download it from: https://developer.android.com/studio"
    exit 1
fi

ANDROID_SDK="${ANDROID_SDK_ROOT:-$ANDROID_HOME}"
echo "Using Android SDK at: $ANDROID_SDK"

# Install required SDK components
echo "Installing Android SDK components..."
$ANDROID_SDK/cmdline-tools/latest/bin/sdkmanager --install \
    "platform-tools" \
    "platforms;android-34" \
    "build-tools;34.0.0" \
    "ndk;26.1.10909125"

# Set up NDK path
export ANDROID_NDK_ROOT="$ANDROID_SDK/ndk/26.1.10909125"
echo "Android NDK set to: $ANDROID_NDK_ROOT"

# Create local.properties file for Android project
echo "Creating local.properties..."
cat > ballistics-mobile/android/local.properties << EOF
sdk.dir=$ANDROID_SDK
ndk.dir=$ANDROID_NDK_ROOT
EOF

echo "Android build environment setup complete!"
echo ""
echo "To build the APK, run: ./scripts/build-android.sh"