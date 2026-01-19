# poker-client

Terminal-based Poker Planning client for agile teams.

![Poker Planning CLI Demo](https://raw.githubusercontent.com/nicoalimin/poker-planning-cli/main/intro.gif)

## Installation

```bash
npm install -g poker-client
```

Or run directly with npx:

```bash
npx poker-client
```

## Usage

```bash
poker-client
```

The client will connect to `localhost:8888` by default. Use environment variables to configure:

- `POKER_SERVER_HOST` - Server hostname (default: `localhost`)
- `POKER_SERVER_PORT` - Server port (default: `8888`)

## Features

- **Real-time Multiplayer**: See other players move and vote effectively instantly.
- **TUI Interface**: Fast, keyboard-centric interface (Arrow keys to move, Space to confirm).
- **Vote Privacy**: Votes are hidden until the Scrum Master reveals them.

## Supported Platforms

- macOS (Apple Silicon / ARM64)
- Linux (x64)
- Windows (x64)

## See Also

- [poker-server](https://www.npmjs.com/package/poker-server) - Run your own poker planning server

## License

MIT
