# NPM Publishing Guide

This guide explains how to publish the Poker Planning CLI to npm.

## Package Structure

The npm distribution uses platform-specific optional dependencies (similar to esbuild, turbo):

```
poker-planning-cli           # Main package with bin wrappers
├── @poker-planning/client-darwin-arm64  # macOS ARM64 client binary
├── @poker-planning/client-linux-x64     # Linux x64 client binary
├── @poker-planning/client-win32-x64     # Windows x64 client binary
├── @poker-planning/server-darwin-arm64  # macOS ARM64 server binary
├── @poker-planning/server-linux-x64     # Linux x64 server binary
└── @poker-planning/server-win32-x64     # Windows x64 server binary
```

When a user runs `npm install poker-planning-cli`, npm automatically downloads only the platform-specific packages matching their OS/architecture.

## Prerequisites

1. **npm account**: You need an npm account and be logged in
   ```bash
   npm login
   ```

2. **npm organization**: Create the `@poker-planning` organization on npm if it doesn't exist
   - Go to https://www.npmjs.com/org/create
   - Create organization named `poker-planning`

3. **Built binaries**: Run `make all` to build binaries for all platforms

## Publishing

### First Time Setup

1. Build all binaries:
   ```bash
   make all
   ```

2. Do a dry run to verify everything is correct:
   ```bash
   make npm-publish-dry
   ```

3. Publish for real:
   ```bash
   make npm-publish
   ```

### Version Bumping

To release a new version:

1. Bump the version across all packages:
   ```bash
   ./npm/bump-version.sh 1.2.0
   ```

2. Build and publish:
   ```bash
   make all
   make npm-publish
   ```

## Manual Publishing

If you need to publish manually:

```bash
# 1. Copy binaries to npm packages
cd npm
./publish.sh --dry-run  # Test first
./publish.sh            # Publish for real
```

## Testing Locally

Before publishing, you can test the packages locally:

```bash
# In the npm/poker-planning-cli directory
npm pack

# Install the tarball globally
npm install -g poker-planning-cli-1.1.0.tgz

# Test the commands
poker-client --help
poker-server --help
```

## Troubleshooting

### "Package already exists"

If you see this error, the version already exists on npm. Bump the version using `bump-version.sh`.

### "Missing binary for platform"

Run `make all` to build binaries for all platforms, then `make npm-prepare` to copy them.

### Binary not executable

The publish script automatically sets the executable bit on Unix binaries. If you're having issues on Windows, ensure you're using the `.exe` file.
