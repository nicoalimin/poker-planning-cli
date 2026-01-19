#!/bin/bash

set -e

# Script to bump version across all npm packages

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.2.0"
    exit 1
fi

NEW_VERSION="$1"

echo "📦 Bumping version to $NEW_VERSION"

# All package directories
PACKAGES=(
    "poker-planning-cli"
    "poker-planning-client-darwin-arm64"
    "poker-planning-client-linux-x64"
    "poker-planning-client-win32-x64"
    "poker-planning-server-darwin-arm64"
    "poker-planning-server-linux-x64"
    "poker-planning-server-win32-x64"
)

for pkg in "${PACKAGES[@]}"; do
    PKG_JSON="$SCRIPT_DIR/$pkg/package.json"
    if [ -f "$PKG_JSON" ]; then
        # Update version using node
        node -e "
            const fs = require('fs');
            const pkg = JSON.parse(fs.readFileSync('$PKG_JSON', 'utf8'));
            pkg.version = '$NEW_VERSION';
            fs.writeFileSync('$PKG_JSON', JSON.stringify(pkg, null, 2) + '\n');
        "
        echo "  ✓ $pkg"
    fi
done

# Update optionalDependencies versions in main package
MAIN_PKG="$SCRIPT_DIR/poker-planning-cli/package.json"
node -e "
    const fs = require('fs');
    const pkg = JSON.parse(fs.readFileSync('$MAIN_PKG', 'utf8'));
    for (const dep of Object.keys(pkg.optionalDependencies || {})) {
        pkg.optionalDependencies[dep] = '$NEW_VERSION';
    }
    fs.writeFileSync('$MAIN_PKG', JSON.stringify(pkg, null, 2) + '\n');
"
echo "  ✓ Updated optionalDependencies versions"

echo ""
echo "✅ Version bumped to $NEW_VERSION"
echo ""
echo "Next steps:"
echo "  1. Build binaries: make all"
echo "  2. Test locally: npm pack (in each package dir)"
echo "  3. Publish: make npm-publish"
