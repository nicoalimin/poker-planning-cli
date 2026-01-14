# Jira Poker Planning Chrome Extension

A Chrome extension that integrates with the Poker Planning CLI server, adding voting controls directly to Jira issues on `smma-sqe.atlassian.net`.

## Features

- **Start Voting**: Starts a poker planning vote session, automatically capturing the current Jira issue number
- **Reveal Votes**: Ends voting and displays statistics (average, median, min/max, individual votes)
- **Live Status**: Real-time updates showing connected players and voting progress via Server-Sent Events

## Prerequisites

The Poker Planning CLI server must be running. It provides:
- TCP server on port 8888 (for CLI clients)
- HTTP API on port 8887 (for this Chrome extension)

## Installation

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode" in the top right corner
3. Click "Load unpacked"
4. Select this `chrome-extension` folder
5. The extension is now installed and active

## Usage

1. Start the poker planning server (`cargo run` in the server directory)
2. Navigate to any issue on `https://smma-sqe.atlassian.net`
3. You'll see the Poker Planning widget appear next to the issue breadcrumb
4. Click "Start Vote" to begin a voting session (issue number is captured automatically)
5. Team members vote using the CLI client
6. Click "Reveal" to see the results with statistics

## API Endpoints

The extension communicates with the server via these HTTP endpoints:

### POST /api/start-voting
Starts a new voting session.

**Request Body:**
```json
{
  "issue_number": "PROJ-123"  // Optional, auto-captured from Jira
}
```

**Response:**
```json
{
  "success": true,
  "message": "Voting started"
}
```

### POST /api/reveal
Ends voting and returns results.

**Response:**
```json
{
  "success": true,
  "issue_number": "PROJ-123",
  "votes": [
    { "player_name": "Alice", "vote": 5 },
    { "player_name": "Bob", "vote": 8 }
  ],
  "statistics": {
    "total_voters": 2,
    "votes_cast": 2,
    "average": 6.5,
    "median": 6.5,
    "min": 5,
    "max": 8,
    "mode": 5
  }
}
```

### GET /api/status (SSE)
Server-Sent Events stream for real-time status updates.

**Event Data:**
```json
{
  "phase": "voting",
  "issue_number": "PROJ-123",
  "connected_players": [
    { "name": "Alice", "has_voted": true },
    { "name": "Bob", "has_voted": false }
  ],
  "votes_cast": 1,
  "total_players": 2
}
```

## Files

- `manifest.json` - Extension configuration (Manifest V3)
- `content.js` - Content script with UI and server communication

## Development

To modify the extension:
1. Edit the files in this folder
2. Go to `chrome://extensions/`
3. Click the refresh icon on the extension card
4. Reload the Jira page to see changes
