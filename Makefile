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
	mkdir -p $(DIST_DIR)/$(TARGET_MACOS_ARM)
	mkdir -p $(DIST_DIR)/$(TARGET_WINDOWS)
	mkdir -p $(DIST_DIR)/$(TARGET_LINUX)

# Build for macOS ARM (aarch64-apple-darwin)
.PHONY: build-macos-arm
build-macos-arm: $(DIST_DIR)
	cargo build --release --target $(TARGET_MACOS_ARM)
	cp target/$(TARGET_MACOS_ARM)/release/client $(DIST_DIR)/$(TARGET_MACOS_ARM)/$(CLIENT_BIN)
	cp target/$(TARGET_MACOS_ARM)/release/server $(DIST_DIR)/$(TARGET_MACOS_ARM)/$(SERVER_BIN)

# Build for Windows (x86_64-pc-windows-gnu)
.PHONY: build-windows
build-windows: $(DIST_DIR)
	cargo build --release --target $(TARGET_WINDOWS)
	cp target/$(TARGET_WINDOWS)/release/client.exe $(DIST_DIR)/$(TARGET_WINDOWS)/$(CLIENT_BIN).exe
	cp target/$(TARGET_WINDOWS)/release/server.exe $(DIST_DIR)/$(TARGET_WINDOWS)/$(SERVER_BIN).exe

# Build for Linux (x86_64-unknown-linux-musl)
.PHONY: build-linux
build-linux: $(DIST_DIR)
	cargo build --release --target $(TARGET_LINUX)
	cp target/$(TARGET_LINUX)/release/client $(DIST_DIR)/$(TARGET_LINUX)/$(CLIENT_BIN)
	cp target/$(TARGET_LINUX)/release/server $(DIST_DIR)/$(TARGET_LINUX)/$(SERVER_BIN)

# Build only client for all targets
.PHONY: client-all
client-all: $(DIST_DIR)
	cargo build --release --package client --target $(TARGET_MACOS_ARM)
	cargo build --release --package client --target $(TARGET_WINDOWS)
	cargo build --release --package client --target $(TARGET_LINUX)
	cp target/$(TARGET_MACOS_ARM)/release/client $(DIST_DIR)/$(TARGET_MACOS_ARM)/$(CLIENT_BIN)
	cp target/$(TARGET_WINDOWS)/release/client.exe $(DIST_DIR)/$(TARGET_WINDOWS)/$(CLIENT_BIN).exe
	cp target/$(TARGET_LINUX)/release/client $(DIST_DIR)/$(TARGET_LINUX)/$(CLIENT_BIN)

# Build only server for all targets
.PHONY: server-all
server-all: $(DIST_DIR)
	cargo build --release --package server --target $(TARGET_MACOS_ARM)
	cargo build --release --package server --target $(TARGET_WINDOWS)
	cargo build --release --package server --target $(TARGET_LINUX)
	cp target/$(TARGET_MACOS_ARM)/release/server $(DIST_DIR)/$(TARGET_MACOS_ARM)/$(SERVER_BIN)
	cp target/$(TARGET_WINDOWS)/release/server.exe $(DIST_DIR)/$(TARGET_WINDOWS)/$(SERVER_BIN).exe
	cp target/$(TARGET_LINUX)/release/server $(DIST_DIR)/$(TARGET_LINUX)/$(SERVER_BIN)

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -rf $(DIST_DIR)

# Clean only distribution directory
.PHONY: clean-dist
clean-dist:
	rm -rf $(DIST_DIR)

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
	@echo "  help           Show this help message"
