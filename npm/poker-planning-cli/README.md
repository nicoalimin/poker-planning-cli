# Poker Planning CLI

A terminal-based Poker Planning tool for agile teams, featuring a real-time TUI client and server.

![Poker Planning CLI Demo](https://raw.githubusercontent.com/nicoalimin/poker-planning-cli/main/intro.gif)

## Installation

```bash
npm install -g poker-planning-cli
```

Or run directly with npx:

```bash
npx poker-client
```

## Usage

### Client

Connect to a poker planning server:

```bash
poker-client
```

The client will connect to `localhost:8888` by default. Use environment variables to configure:

- `POKER_SERVER_HOST` - Server hostname (default: `localhost`)
- `POKER_SERVER_PORT` - Server port (default: `8888`)

### Server

Run your own poker planning server:

```bash
poker-server
```

## Features

- **Real-time Multiplayer**: See other players move and vote effectively instantly.
- **TUI Interface**: Fast, keyboard-centric interface (Arrow keys to move, Space to confirm).
- **Vote Privacy**: Votes are hidden until the Scrum Master reveals them.

## Supported Platforms

- macOS (Apple Silicon / ARM64)
- Linux (x64)
- Windows (x64)

## License

MIT
