#!/bin/bash
set -e

echo "==========================================="
echo "Building WebAppLauncher for Windows..."
echo "==========================================="

echo "[INFO] Note: Cross-compiling for Windows from macOS requires the MinGW-w64 toolchain and the Windows Rust target."
echo "If you haven't installed them, please run:"
echo "  brew install mingw-w64"
echo "  rustup target add x86_64-pc-windows-gnu"
echo "-------------------------------------------"
sleep 2

npm install

# Build for Windows using the GNU target
npm run tauri build -- --target x86_64-pc-windows-gnu

echo "==========================================="
echo "✅ Windows build complete!"
echo "Find your .exe installer in: src-tauri/target/x86_64-pc-windows-gnu/release/bundle/nsis/"
echo "==========================================="
