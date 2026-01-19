#!/usr/bin/env node

// This script runs after npm install to verify the correct platform package was installed

const fs = require("fs");
const path = require("path");

const platform = process.platform;
const arch = process.arch;
const key = `${platform}-${arch}`;

const CLIENT_PACKAGES = {
  "darwin-arm64": "@poker-planning/client-darwin-arm64",
  "linux-x64": "@poker-planning/client-linux-x64",
  "win32-x64": "@poker-planning/client-win32-x64",
};

const SERVER_PACKAGES = {
  "darwin-arm64": "@poker-planning/server-darwin-arm64",
  "linux-x64": "@poker-planning/server-linux-x64",
  "win32-x64": "@poker-planning/server-win32-x64",
};

const clientPackage = CLIENT_PACKAGES[key];
const serverPackage = SERVER_PACKAGES[key];

if (!clientPackage || !serverPackage) {
  console.warn(`\n⚠️  poker-planning-cli: Unsupported platform ${key}`);
  console.warn(`   Supported platforms: ${Object.keys(CLIENT_PACKAGES).join(", ")}`);
  console.warn(`   The binaries may not work on your system.\n`);
  process.exit(0);
}

// Check if at least the client package is available
const binaryName = platform === "win32" ? "poker-client.exe" : "poker-client";
const possiblePaths = [
  path.join(__dirname, "..", "node_modules", clientPackage, binaryName),
  path.join(__dirname, "..", "..", clientPackage, binaryName),
  path.join(__dirname, "..", "..", "..", clientPackage, binaryName),
];

let found = false;
for (const binaryPath of possiblePaths) {
  if (fs.existsSync(binaryPath)) {
    found = true;
    break;
  }
}

if (!found) {
  console.warn(`\n⚠️  poker-planning-cli: Platform package ${clientPackage} not found.`);
  console.warn(`   This might be a network issue. Try reinstalling.\n`);
}
