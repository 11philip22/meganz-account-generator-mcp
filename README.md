<h1 align="center">MEGA.nz Account Generator MCP</h1>

<p align="center">
  A minimal Rust MCP server for generating MEGA.nz accounts over stdio.
</p>

<p align="center">
  <a href="https://crates.io/crates/meganz-account-generator-mcp"><img src="https://img.shields.io/crates/v/meganz-account-generator-mcp?style=for-the-badge&logo=rust&logoColor=white&label=crate" alt="Crates.io version"></a>
  <a href="https://github.com/11philip22/meganz-account-generator-mcp"><img src="https://img.shields.io/badge/MCP-stdio-0EA5E9?style=for-the-badge" alt="MCP stdio transport"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-22C55E?style=for-the-badge" alt="MIT License"></a>
</p>

<p align="center">
  <a href="#features">Features</a> &middot;
  <a href="#installation">Installation</a> &middot;
  <a href="#quick-start">Quick Start</a> &middot;
  <a href="#tool-reference">Tool Reference</a> &middot;
  <a href="#configuration">Configuration</a> &middot;
  <a href="#development">Development</a>
</p>

---

## Features

- Exposes a single MCP tool, `mega/generate`, over stdio.
- Generates one or more MEGA.nz accounts using temporary email addresses.
- Returns machine-readable account data in `structuredContent`.
- Supports optional proxy routing through a CLI flag or environment variable.
- Enforces a small per-request account limit to keep calls predictable.

> [!IMPORTANT]
> Use this project only for legitimate automation, testing, or research that you are authorized to perform. Third-party services may apply rate limits, account policies, or terms of service restrictions.

## Installation

Install the published binary with Cargo:

```bash
cargo install meganz-account-generator-mcp
```

Or run it directly from a local checkout:

```bash
cargo run
```

The server communicates through MCP over stdio, so it is meant to be launched by an MCP client or driven with newline-delimited JSON-RPC messages.

## Quick Start

### MCP client config

Use the installed binary as a stdio MCP server:

```json
{
  "mcpServers": {
    "meganz-account-generator": {
      "command": "meganz-account-generator-mcp"
    }
  }
}
```

Then call the `mega/generate` tool from your MCP client.

### Manual JSON-RPC flow

Start the server:

```bash
meganz-account-generator-mcp
```

Send these messages one line at a time:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","clientInfo":{"name":"manual-test","version":"1.0.0"},"capabilities":{}}}
```

```json
{"jsonrpc":"2.0","method":"notifications/initialized"}
```

```json
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mega/generate","arguments":{"count":1,"password":"MySecurePassword123!"}}}
```

Example response:

```json
{
  "content": [
    {
      "type": "text",
      "text": "Generated 1 account(s)."
    }
  ],
  "structuredContent": {
    "accounts": [
      {
        "email": "account@example.com",
        "password": "MySecurePassword123!",
        "name": "Example Name"
      }
    ]
  },
  "isError": false
}
```

## Tool Reference

### `mega/generate`

Generates MEGA.nz accounts using temporary email addresses.

| Argument | Type | Default | Notes |
| --- | --- | --- | --- |
| `count` | number | `1` | Must be between `1` and `5`. |
| `password` | string | `Mcp!Passw0rd2026` | Must not be empty. Passing an explicit password is recommended. |

Example:

```json
{
  "name": "mega/generate",
  "arguments": {
    "count": 3,
    "password": "MySecurePassword123!"
  }
}
```

The generated accounts are returned in `structuredContent.accounts`.

## Configuration

Route MEGA requests through a proxy with either the CLI flag:

```bash
meganz-account-generator-mcp --proxy-url http://127.0.0.1:8080
```

Or the environment variable:

```bash
MEGA_PROXY_URL=http://127.0.0.1:8080 meganz-account-generator-mcp
```

For local development with Cargo:

```bash
MEGA_PROXY_URL=http://127.0.0.1:8080 cargo run
```

## Development

```bash
cargo check
cargo run
```

Project layout:

```text
src/main.rs                CLI parsing and tracing setup
src/app.rs                 MCP server and tool registration
src/handlers/generate.rs   Account generation handler
```

Use `RUST_LOG=debug` when you need more runtime detail from the server.
