# Jira Issue Helper Chrome Extension

A Chrome extension that adds custom functionality to Jira issues on `smma-sqe.atlassian.net`.

## Features

- Detects when you're viewing a Jira issue
- Adds a custom div next to the issue breadcrumb

## Installation

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode" in the top right corner
3. Click "Load unpacked"
4. Select this `chrome-extension` folder
5. The extension is now installed and active

## Usage

Navigate to any issue on `https://smma-sqe.atlassian.net` and you'll see the custom div appear next to the issue breadcrumb.

## Development

- `manifest.json` - Extension configuration
- `content.js` - Content script that runs on Jira pages

## How it works

The extension uses a MutationObserver to detect when Jira's dynamic content loads, then finds the breadcrumb container element and inserts a custom div next to it.
