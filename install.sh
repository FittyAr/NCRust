#!/bin/sh
set -e

# Pairee Linux Installer
# Installs Pairee statically built binary and copies assets to the user's config directories.

REPO="FittyAr/Pairee"
INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/pairee"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "${BLUE}Pairee Installer for Linux${NC}"
echo "=============================="

# 1. Architecture Check
OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" != "Linux" ]; then
    echo "${RED}Error: This script only supports Linux.${NC}"
    exit 1
fi

if [ "$ARCH" != "x86_64" ]; then
    echo "${RED}Error: Currently only x86_64 architecture is supported via installer.${NC}"
    exit 1
fi

# 2. Dependency Check
for cmd in curl tar; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "${RED}Error: Required command '$cmd' is not installed.${NC}"
        exit 1
    fi
done

# 3. Check for Existing Installation
if [ -f "$INSTALL_DIR/pairee" ] || [ -d "$CONFIG_DIR" ]; then
    # Colors for warning/options
    YELLOW='\033[1;33m'
    echo "${YELLOW}Warning: Pairee is already installed.${NC}"
    
    if [ -c /dev/tty ]; then
        printf "Do you want to overwrite and update the binary? [y/N]: "
        read -r OVERWRITE < /dev/tty || OVERWRITE="n"
        case "$OVERWRITE" in
            [yY][eE][sS]|[yY])
                echo "Proceeding with update..."
                ;;
            *)
                echo "Installation cancelled."
                exit 0
                ;;
        esac

        if [ -d "$CONFIG_DIR" ]; then
            printf "Do you want to clear old configurations, themes, and history settings? [y/N]: "
            read -r CLEAR_CONFIG < /dev/tty || CLEAR_CONFIG="n"
            case "$CLEAR_CONFIG" in
                [yY][eE][sS]|[yY])
                    echo "Clearing old settings in $CONFIG_DIR..."
                    rm -rf "$CONFIG_DIR"
                    ;;
                *)
                    echo "Keeping existing settings."
                    ;;
            esac
        fi
    else
        echo "Non-interactive shell detected. Overwriting existing installation..."
    fi
fi

# 4. Retrieve Latest Version
echo "Fetching latest version info..."
VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | head -n 1 | cut -d '"' -f 4)

if [ -z "$VERSION" ]; then
    echo "${RED}Error: Could not retrieve latest release version from GitHub API.${NC}"
    exit 1
fi
echo "Latest version found: ${GREEN}${VERSION}${NC}"

# 5. Create paths
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR/lang"
mkdir -p "$CONFIG_DIR/help"

# 6. Download and Extract
TEMP_DIR=$(mktemp -d)
TARBALL="pairee-${VERSION}-x86_64-unknown-linux-musl.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${TARBALL}"

echo "Downloading ${TARBALL}..."
curl -L "$DOWNLOAD_URL" -o "${TEMP_DIR}/${TARBALL}"

echo "Extracting archive..."
tar -xzf "${TEMP_DIR}/${TARBALL}" -C "$TEMP_DIR"

# 7. Install assets and binary
echo "Installing files..."
PKG_FOLDER="${TEMP_DIR}/pairee-${VERSION}-x86_64-unknown-linux-musl"

# Copy binary
cp "${PKG_FOLDER}/pairee" "$INSTALL_DIR/pairee"
chmod +x "$INSTALL_DIR/pairee"

# Copy translations and help markdown
cp -r "${PKG_FOLDER}/lang/"* "$CONFIG_DIR/lang/"
cp -r "${PKG_FOLDER}/help/"* "$CONFIG_DIR/help/"

# Clean up
rm -rf "$TEMP_DIR"

echo "=============================="
echo "${GREEN}Pairee version ${VERSION} installed successfully!${NC}"
echo "Binary location: ${BLUE}${INSTALL_DIR}/pairee${NC}"
echo "Config location: ${BLUE}${CONFIG_DIR}/${NC}"
echo ""

# 8. PATH verification
case :$PATH: in
    *:"$INSTALL_DIR":*) ;;
    *)
        echo "${BLUE}Note: '${INSTALL_DIR}' is not in your PATH.${NC}"
        echo "Please add it to your shell configuration (e.g. ~/.bashrc or ~/.zshrc):"
        echo "  ${GREEN}export PATH=\"\$PATH:\$HOME/.local/bin\"${NC}"
        echo ""
        ;;
esac

echo "Run Pairee by typing: ${GREEN}pairee${NC}"
