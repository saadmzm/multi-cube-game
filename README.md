# Simple Multiplayer Game

## Setup

- Install [Rust](https://www.rust-lang.org/tools/install)
- Clone this repository.
- Open VSCode.
- Change directory to this repository in the terminal.

## Usage

- Run the commands in different terminals.

### Server

- Start the server.

```bash
cargo run --release --bin server
```

- To close the server press `Ctrl+C`.

### Client

- Client can be run multiple times.

```bash
cargo run --release --bin client
```

- Clients will be in sync only if the server is running.
