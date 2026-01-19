# Poker Planning CLI - Cross-compilation Makefile

# Binary names
CLIENT_BIN = poker-client
SERVER_BIN = poker-server

# Target architectures
TARGET_MACOS_ARM = aarch64-apple-darwin
TARGET_WINDOWS = x86_64-pc-windows-gnu
TARGET_LINUX = x86_64-unknown-linux-musl

# Output directory
DIST_DIR = dist
# Linux Builder
LINUX_BUILDER = docker run --platform linux/amd64 --rm -v "$(shell pwd)":/home/rust/src -e CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=/usr/local/musl/bin/x86_64-unknown-linux-musl-gcc messense/rust-musl-cross:x86_64-musl


# Build all targets
.PHONY: all
all: build-macos-arm build-windows build-linux

# Install required targets
.PHONY: setup
setup:
	rustup target add $(TARGET_MACOS_ARM)
	rustup target add $(TARGET_WINDOWS)
	rustup target add $(TARGET_LINUX)

# Create distribution directory
$(DIST_DIR):
	mkdir -p $(DIST_DIR)

# Build for macOS ARM (aarch64-apple-darwin)
.PHONY: build-macos-arm
build-macos-arm: $(DIST_DIR)
	cargo build --release --target $(TARGET_MACOS_ARM)
	cp target/$(TARGET_MACOS_ARM)/release/client $(DIST_DIR)/$(CLIENT_BIN)-macos-arm64
	cp target/$(TARGET_MACOS_ARM)/release/server $(DIST_DIR)/$(SERVER_BIN)-macos-arm64

# Build for Windows (x86_64-pc-windows-gnu)
.PHONY: build-windows
build-windows: $(DIST_DIR)
	cargo build --release --target $(TARGET_WINDOWS)
	cp target/$(TARGET_WINDOWS)/release/client.exe $(DIST_DIR)/$(CLIENT_BIN)-windows-x64.exe
	cp target/$(TARGET_WINDOWS)/release/server.exe $(DIST_DIR)/$(SERVER_BIN)-windows-x64.exe

# Build for Linux (x86_64-unknown-linux-musl)
.PHONY: build-linux
build-linux: $(DIST_DIR)
	$(LINUX_BUILDER) cargo build --release --target $(TARGET_LINUX)
	cp target/$(TARGET_LINUX)/release/client $(DIST_DIR)/$(CLIENT_BIN)-linux-x64
	cp target/$(TARGET_LINUX)/release/server $(DIST_DIR)/$(SERVER_BIN)-linux-x64

# Build only client for all targets
.PHONY: client-all
client-all: $(DIST_DIR)
	cargo build --release --package client --target $(TARGET_MACOS_ARM)
	cargo build --release --package client --target $(TARGET_WINDOWS)
	$(LINUX_BUILDER) cargo build --release --package client --target $(TARGET_LINUX)
	cp target/$(TARGET_MACOS_ARM)/release/client $(DIST_DIR)/$(CLIENT_BIN)-macos-arm64
	cp target/$(TARGET_WINDOWS)/release/client.exe $(DIST_DIR)/$(CLIENT_BIN)-windows-x64.exe
	cp target/$(TARGET_LINUX)/release/client $(DIST_DIR)/$(CLIENT_BIN)-linux-x64

# Build only server for all targets
.PHONY: server-all
server-all: $(DIST_DIR)
	cargo build --release --package server --target $(TARGET_MACOS_ARM)
	cargo build --release --package server --target $(TARGET_WINDOWS)
	$(LINUX_BUILDER) cargo build --release --package server --target $(TARGET_LINUX)
	cp target/$(TARGET_MACOS_ARM)/release/server $(DIST_DIR)/$(SERVER_BIN)-macos-arm64
	cp target/$(TARGET_WINDOWS)/release/server.exe $(DIST_DIR)/$(SERVER_BIN)-windows-x64.exe
	cp target/$(TARGET_LINUX)/release/server $(DIST_DIR)/$(SERVER_BIN)-linux-x64

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -rf $(DIST_DIR)

# Clean only distribution directory
.PHONY: clean-dist
clean-dist:
	rm -rf $(DIST_DIR)

# NPM Publishing targets
NPM_DIR = npm

# Prepare npm packages (copy binaries)
.PHONY: npm-prepare
npm-prepare: all
	@echo "Copying binaries to npm packages..."
	cp $(DIST_DIR)/$(CLIENT_BIN)-macos-arm64 $(NPM_DIR)/poker-planning-client-darwin-arm64/poker-client
	cp $(DIST_DIR)/$(CLIENT_BIN)-linux-x64 $(NPM_DIR)/poker-planning-client-linux-x64/poker-client
	cp $(DIST_DIR)/$(CLIENT_BIN)-windows-x64.exe $(NPM_DIR)/poker-planning-client-win32-x64/poker-client.exe
	cp $(DIST_DIR)/$(SERVER_BIN)-macos-arm64 $(NPM_DIR)/poker-planning-server-darwin-arm64/poker-server
	cp $(DIST_DIR)/$(SERVER_BIN)-linux-x64 $(NPM_DIR)/poker-planning-server-linux-x64/poker-server
	cp $(DIST_DIR)/$(SERVER_BIN)-windows-x64.exe $(NPM_DIR)/poker-planning-server-win32-x64/poker-server.exe
	chmod +x $(NPM_DIR)/poker-planning-client-darwin-arm64/poker-client
	chmod +x $(NPM_DIR)/poker-planning-client-linux-x64/poker-client
	chmod +x $(NPM_DIR)/poker-planning-server-darwin-arm64/poker-server
	chmod +x $(NPM_DIR)/poker-planning-server-linux-x64/poker-server
	@echo "Done!"

# Publish to npm (dry run)
.PHONY: npm-publish-dry
npm-publish-dry: npm-prepare
	./$(NPM_DIR)/publish.sh --dry-run

# Publish to npm
.PHONY: npm-publish
npm-publish: npm-prepare
	./$(NPM_DIR)/publish.sh

# Show help
.PHONY: help
help:
	@echo "Poker Planning CLI - Build Targets"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all            Build for all platforms (default)"
	@echo "  setup          Install required Rust targets"
	@echo "  build-macos-arm Build for macOS ARM (aarch64-apple-darwin)"
	@echo "  build-windows  Build for Windows (x86_64-pc-windows-gnu)"
	@echo "  build-linux    Build for Linux (x86_64-unknown-linux-musl)"
	@echo "  client-all     Build only client for all platforms"
	@echo "  server-all     Build only server for all platforms"
	@echo "  clean          Clean all build artifacts"
	@echo "  clean-dist     Clean only distribution directory"
	@echo "  npm-prepare    Copy binaries to npm packages"
	@echo "  npm-publish-dry Dry run npm publish"
	@echo "  npm-publish    Publish to npm registry"
	@echo "  help           Show this help message"
