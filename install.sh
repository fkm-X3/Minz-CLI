#!/bin/sh
set -e

REPO="fkm-X3/Minz-CLI"
INSTALL_DIR="/usr/local/bin"
TMP_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

# Detect Operating System
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$OS" in
    linux*)   OS_TYPE="linux" ;;
    darwin*)  OS_TYPE="macos" ;;
    freebsd*) OS_TYPE="freebsd" ;;
    *)
        echo "Error: Unsupported OS '$OS'" >&2
        exit 1
        ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64|amd64) ARCH_TYPE="amd64" ;;
    aarch64|arm64) ARCH_TYPE="arm64" ;;
    i386|i686)    ARCH_TYPE="386" ;;
    *)
        echo "Error: Unsupported architecture '$ARCH'" >&2
        exit 1
        ;;
esac

echo "Detected System: $OS_TYPE ($ARCH_TYPE)"

# Fetch latest release info from GitHub API
echo "Fetching latest release details for $REPO..."
RELEASE_JSON=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest")

# Extract target download asset URL matching system profile
# Looks for binary pattern matching {os} and {arch} in the asset name
DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -i "browser_download_url" | grep -i "$OS_TYPE" | grep -i "$ARCH_TYPE" | head -n 1 | cut -d '"' -f 4)

# Fallback: If no OS/Arch-specific file, grab primary executable asset
if [ -z "$DOWNLOAD_URL" ]; then
    DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -i "browser_download_url" | head -n 1 | cut -d '"' -f 4)
fi

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Could not find a suitable release binary for $OS_TYPE-$ARCH_TYPE." >&2
    exit 1
fi

echo "Downloading from: $DOWNLOAD_URL"
OUTPUT_FILE="$TMP_DIR/minz"

# Download binary using single curl call
curl -fsSL "$DOWNLOAD_URL" -o "$OUTPUT_FILE"

# Extract if archive or directly make executable
if echo "$DOWNLOAD_URL" | grep -qE '\.tar\.gz$|\.tgz$'; then
    tar -xzf "$OUTPUT_FILE" -C "$TMP_DIR"
    BIN_PATH=$(find "$TMP_DIR" -type f -name "minz" -o -name "minz-cli" | head -n 1)
else
    BIN_PATH="$OUTPUT_FILE"
fi

chmod +x "$BIN_PATH"

# 6. Install to system directory
echo "Installing binary to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$BIN_PATH" "$INSTALL_DIR/minz"
else
    echo "Elevated permissions required to write to $INSTALL_DIR:"
    sudo mv "$BIN_PATH" "$INSTALL_DIR/minz"
fi

echo " Minz-CLI installed successfully! Run 'minz --help' to verify (THIS COMMAND DOESN'T EXIST YET)."