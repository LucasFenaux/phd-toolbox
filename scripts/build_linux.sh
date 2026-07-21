#!/bin/bash
set -e

echo "==========================================="
echo "Building WebAppLauncher for Linux..."
echo "==========================================="

echo "[INFO] Note: Cross-compiling for Linux from macOS is complex due to WebKit2GTK dependencies."
echo "This script uses the official Tauri Docker image to build the Linux version cleanly."
echo "Make sure Docker is installed and running."
echo "-------------------------------------------"
sleep 2

npm install

# Use the official Tauri Docker image to build for Linux
# It mounts the current directory into the container and runs the build command
docker run --rm -v "$PWD":/app -w /app ghcr.io/tauri-apps/tauri:ubuntu-22.04 bash -c "
  npm install && 
  npm run tauri build
"

echo "==========================================="
echo "✅ Linux build complete!"
echo "Find your .AppImage or .deb in: src-tauri/target/release/bundle/"
echo "==========================================="
