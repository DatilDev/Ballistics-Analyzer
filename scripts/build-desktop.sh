# Location: scripts/build-desktop.sh
#!/bin/bash
echo "Building Desktop Application..."
cd ballistics-desktop
cargo build --release
echo "Desktop build complete: target/release/ballistics-analyzer"