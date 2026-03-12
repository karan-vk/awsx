#!/bin/bash
set -e

# Configuration
REPO="karan-vijayakumar/awsp" # User should update this to their actual repo name
INSTALL_DIR="/usr/local/bin"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$OS" in
  linux*)  PLATFORM="linux" ;;
  darwin*) PLATFORM="macos" ;;
  *)       echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64)  SUFFIX="amd64" ;;
  arm64|aarch64) SUFFIX="arm64" ;;
  *)       echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

ASSET_NAME="awsp-${PLATFORM}-${SUFFIX}.tar.gz"

echo "Downloading awsp for ${PLATFORM}/${ARCH}..."

# Get latest release from GitHub API
LATEST_RELEASE=$(curl -s https://api.github.com/repos/${REPO}/releases/latest | grep "tag_name" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE" ]; then
    echo "No releases found for ${REPO}. Please ensure you have created a release tag."
    exit 1
fi

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_RELEASE}/${ASSET_NAME}"

curl -L -o "/tmp/${ASSET_NAME}" "${DOWNLOAD_URL}"

echo "Installing to ${INSTALL_DIR}..."
tar -xzf "/tmp/${ASSET_NAME}" -C /tmp
sudo mv /tmp/awsp "${INSTALL_DIR}/awsp"
sudo chmod +x "${INSTALL_DIR}/awsp"

echo "Successfully installed awsp to ${INSTALL_DIR}/awsp"
echo "Don't forget to add the shell hook to your config!"
echo "Run 'awsp' to learn how."
