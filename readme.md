# meganz-account-generator-mcp

[![MCP transport: stdio](https://img.shields.io/badge/MCP%20transport-stdio-informational)](#overview)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20windows%20%7C%20macOS-blue)](#overview)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/11philip22/meganz-account-generator-mcp/pulls)

<img src="https://upload.wikimedia.org/wikipedia/commons/b/bc/MEGA_logo.png" alt="MEGA Logo" width="300">

A minimal MCP-style JSON-over-stdio server in Rust for generating MEGA.nz accounts via the [meganz-account-generator](https://crates.io/crates/meganz-account-generator) crate.

## Overview

This server reads newline-delimited JSON requests from `stdin` and writes exactly one newline-delimited JSON response to `stdout` per request.

Implemented methods:
- `server.info`
- `tools.list`
- `mega.generate`

## Protocol

### Request
```json
{
  "id": "string",
  "method": "string",
  "params": object | null
}
```

## Response
```json
{
  "id": "string",
  "result": object | null,
  "error": { "code": "string", "message": "string" } | null
}
```

## Methods

### `server.info`
Returns static server metadata.

### `tools.list`
Returns available tools.

Example result:
```json
{
  "tools": ["mega.generate"]
}
```

### `mega.generate`
Generates one or more MEGA accounts.

Parameters:

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `count` | number | No | Number of accounts to generate. Default `1`, minimum `1`, maximum from server state (default `5`). |
| `password` | string | No | Password to use for generated accounts. Defaults to the server state password. |

Example result:
```json
{
  "accounts": [
    {
      "email": "example@mega.nz",
      "password": "StrongPass123!",
      "name": "Example Name"
    }
  ]
}
```

## Run

```bash
cargo run
```

Optional proxy support:

```bash
MEGA_PROXY_URL=http://127.0.0.1:8080 cargo run
```

Then send JSON lines to stdin, for example:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","clientInfo":{"name":"debug","version":"0.1.0"},"capabilities":{}}}
{"id":"1","method":"server/info","params":null}
{"id":"2","method":"tools/list","params":null}
{"id":"3","method":"mega/generate","params":{"count":1,"password":"StrongPass123!"}}
```

## Notes

- `stdout` is reserved for JSON responses only.
- Operational logs/errors are written to `stderr`.
- Output is deterministic for metadata/tooling methods (`server.info`, `tools.list`).

## Contributing

PRs are welcome!  
Please run `cargo fmt` and `cargo clippy` before submitting.

If you’re changing behavior, please document it in the PR.

## Support

If this crate saves you time or helps your work, support is appreciated:

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/11philip22)

## License

This project is licensed under the MIT License; see the [license](license) file for details.
