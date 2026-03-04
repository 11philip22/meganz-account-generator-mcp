# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-03-04

### Changed

- Tool `mega/generate` response now returns both `content` (human-readable summary) and `structuredContent` (unescaped JSON with accounts)
- Upgrade meganz-account-generator usage to builder API (AccountGenerator::builder().proxy().build())
- Migrated from custom MCP implementation to the official [modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk) (rmcp crate)
- Server now uses rmcp's server-as-struct pattern: config (`proxy_url`) is passed via `MegaServer::new(proxy_url)` and accessible in tool handlers via `&self`
- Removed `mcp-core` dependency; added `rmcp` and `schemars`
- Tool `mega/generate` now uses `#[tool]` macro with typed `GenerateArgs` (JsonSchema)
- Replaced custom protocol types and handlers with rmcp's `ServerHandler`, `ToolRouter`, `tool_handler`

### Removed

- Custom `src/protocol/` module (McpRequest, McpResponse, McpErrorBody)
- `AppState` and global/static config workarounds
- Env var `set_var` hack for proxy URL; config flows from CLI into server constructor
