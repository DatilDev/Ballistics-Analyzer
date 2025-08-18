# Location: scripts/build-wasm.sh
#!/bin/bash
echo "Building WASM/PWA..."
cd ballistics-wasm
wasm-pack build --target web --out-dir pkg --release
echo "WASM build complete: ballistics-wasm/pkg/"