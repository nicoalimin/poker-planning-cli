# Poker Planning CLI

![Poker Planning CLI Demo](intro.gif)

A terminal-based Poker Planning tool for agile teams, featuring a real-time TUI client and server.

## Installation & Connection

To join a session, you need to connect to the game server and run the client.

### 1. Connect to Server (AWS SSM)
You must forward the server port (8888) to your local machine using AWS Systems Manager.
Run the following command in a terminal (keep it open):

```bash
aws ssm start-session \
    --target i-0498121b79469ca5b \
    --document-name AWS-StartPortForwardingSessionToRemoteHost \
    --parameters '{"portNumber":["8888"],"localPortNumber":["8888"]}' \
    --region ap-southeast-1
```

If you want to run the Jira plugin, you must forward the HTTP port (8887) to your local machine using AWS Systems Manager.
Run the following command in a terminal (keep it open):

```bash
aws ssm start-session \
    --target i-0498121b79469ca5b \
    --document-name AWS-StartPortForwardingSessionToRemoteHost \
    --parameters '{"portNumber":["8887"],"localPortNumber":["8887"]}' \
    --region ap-southeast-1
```

*Note: Requires AWS CLI and Session Manager plugin.*

### 2. Install & Run Client

**Option A: Run directly with npx (no install needed)**

```bash
npx poker-client
```

**Option B: Install globally via npm**

```bash
npm install -g poker-client
poker-client
```

**Option C: Install via shell script**

```bash
curl -fsSL https://raw.githubusercontent.com/nicoalimin/poker-planning-cli/main/install.sh | bash
./client
```

**Option D: Manual download**

Download the binary for your platform from [Releases](https://github.com/nicoalimin/poker-planning-cli/releases):

| Platform | Binary |
|----------|--------|
| macOS (Apple Silicon) | `poker-client-macos-arm64` |
| Linux (x64) | `poker-client-linux-x64` |
| Windows (x64) | `poker-client-windows-x64.exe` |

Then make it executable (macOS/Linux only): `chmod +x poker-client-*`

## Running Your Own Server

To host your own poker planning server:

```bash
npx poker-server
```

Or install globally:

```bash
npm install -g poker-server
poker-server
```

The server listens on port 8888 by default.

## Features
- **Real-time Multiplayer**: See other players move and vote effectively instantly.
- **TUI Interface**: Fast, keyboard-centric interface (Arrow keys to move, Space to confirm).
- **Vote Privacy**: Votes are hidden until the Scrum Master reveals them.
