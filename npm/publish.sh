#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$ROOT_DIR/dist"
NPM_DIR="$SCRIPT_DIR"

VERSION=$(node -p "require('$NPM_DIR/poker-planning-cli/package.json').version")

echo -e "${GREEN}📦 Publishing Poker Planning CLI v${VERSION}${NC}"
echo ""

# Check if dist directory exists
if [ ! -d "$DIST_DIR" ]; then
    echo -e "${RED}Error: dist directory not found. Run 'make all' first.${NC}"
    exit 1
fi

# Function to check if binary exists
check_binary() {
    if [ ! -f "$1" ]; then
        echo -e "${RED}Error: Binary not found: $1${NC}"
        echo -e "${YELLOW}Run 'make all' to build all binaries first.${NC}"
        exit 1
    fi
}

# Copy binaries to their respective packages
echo -e "${YELLOW}📋 Copying binaries to npm packages...${NC}"

# Client binaries
check_binary "$DIST_DIR/poker-client-macos-arm64"
cp "$DIST_DIR/poker-client-macos-arm64" "$NPM_DIR/poker-planning-client-darwin-arm64/poker-client"
chmod +x "$NPM_DIR/poker-planning-client-darwin-arm64/poker-client"
echo "  ✓ poker-planning-client-darwin-arm64"

check_binary "$DIST_DIR/poker-client-linux-x64"
cp "$DIST_DIR/poker-client-linux-x64" "$NPM_DIR/poker-planning-client-linux-x64/poker-client"
chmod +x "$NPM_DIR/poker-planning-client-linux-x64/poker-client"
echo "  ✓ poker-planning-client-linux-x64"

check_binary "$DIST_DIR/poker-client-windows-x64.exe"
cp "$DIST_DIR/poker-client-windows-x64.exe" "$NPM_DIR/poker-planning-client-win32-x64/poker-client.exe"
echo "  ✓ poker-planning-client-win32-x64"

# Server binaries
check_binary "$DIST_DIR/poker-server-macos-arm64"
cp "$DIST_DIR/poker-server-macos-arm64" "$NPM_DIR/poker-planning-server-darwin-arm64/poker-server"
chmod +x "$NPM_DIR/poker-planning-server-darwin-arm64/poker-server"
echo "  ✓ poker-planning-server-darwin-arm64"

check_binary "$DIST_DIR/poker-server-linux-x64"
cp "$DIST_DIR/poker-server-linux-x64" "$NPM_DIR/poker-planning-server-linux-x64/poker-server"
chmod +x "$NPM_DIR/poker-planning-server-linux-x64/poker-server"
echo "  ✓ poker-planning-server-linux-x64"

check_binary "$DIST_DIR/poker-server-windows-x64.exe"
cp "$DIST_DIR/poker-server-windows-x64.exe" "$NPM_DIR/poker-planning-server-win32-x64/poker-server.exe"
echo "  ✓ poker-planning-server-win32-x64"

echo ""

# Check if --dry-run flag is passed
DRY_RUN=""
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN="--dry-run"
    echo -e "${YELLOW}🧪 Dry run mode - packages will not be published${NC}"
    echo ""
fi

# Publish platform-specific packages first
echo -e "${YELLOW}📤 Publishing platform packages...${NC}"

PLATFORM_PACKAGES=(
    "poker-planning-client-darwin-arm64"
    "poker-planning-client-linux-x64"
    "poker-planning-client-win32-x64"
    "poker-planning-server-darwin-arm64"
    "poker-planning-server-linux-x64"
    "poker-planning-server-win32-x64"
)

for pkg in "${PLATFORM_PACKAGES[@]}"; do
    echo "  Publishing $pkg..."
    cd "$NPM_DIR/$pkg"
    npm publish --access public $DRY_RUN || {
        echo -e "${YELLOW}  ⚠️  Package may already exist at this version${NC}"
    }
done

# Publish main package
echo ""
echo -e "${YELLOW}📤 Publishing main package...${NC}"
cd "$NPM_DIR/poker-planning-cli"
npm publish --access public $DRY_RUN || {
    echo -e "${YELLOW}  ⚠️  Package may already exist at this version${NC}"
}

echo ""
echo -e "${GREEN}✅ Publishing complete!${NC}"
echo ""
echo -e "Users can now install with:"
echo -e "  ${GREEN}npm install -g poker-planning-cli${NC}"
echo ""
echo -e "Or run directly with npx:"
echo -e "  ${GREEN}npx poker-client${NC}"
