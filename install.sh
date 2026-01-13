#!/bin/bash

set -e

REPO="nicoalimin/poker-planning-cli"
VERSION="v1.0.0"
BINARY_NAME="client"

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Darwin)
        case "$ARCH" in
            arm64)
                ASSET_NAME="poker-client-macos-arm64"
                ;;
            x86_64)
                echo "Error: macOS x86_64 is not supported. Only ARM64 (Apple Silicon) is available."
                exit 1
                ;;
            *)
                echo "Error: Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            x86_64)
                ASSET_NAME="poker-client-linux-x64"
                ;;
            *)
                echo "Error: Unsupported architecture: $ARCH. Only x86_64 is available for Linux."
                exit 1
                ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*)
        ASSET_NAME="poker-client-windows-x64.exe"
        BINARY_NAME="client.exe"
        ;;
    *)
        echo "Error: Unsupported operating system: $OS"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"

if [ -f "$BINARY_NAME" ]; then
    echo "Client binary already exists, skipping download."
else
    echo "Detected: $OS $ARCH"
    echo "Downloading Poker Planning Client ($ASSET_NAME)..."
    curl -L -o "$BINARY_NAME" "$DOWNLOAD_URL"
fi

echo "Making executable..."
chmod +x "$BINARY_NAME"

echo "Done! Run ./$BINARY_NAME to start."
