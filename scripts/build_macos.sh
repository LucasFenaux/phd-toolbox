#!/bin/bash
set -e

echo "==========================================="
echo "Building WebAppLauncher for macOS..."
echo "==========================================="

# Install frontend dependencies
npm install

# Build the macOS binary
# Note: To build a universal binary (Intel + Apple Silicon), you can use:
# npm run tauri build -- --target universal-apple-darwin
npm run tauri build

echo "==========================================="
echo "✅ macOS build complete!"
echo "Find your .dmg and .app bundle in: src-tauri/target/release/bundle/macos/"
echo "==========================================="
