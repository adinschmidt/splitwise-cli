# splitwise-cli Plan

## Goal

Build a Rust CLI for Splitwise that starts with full API coverage, good scripting support, and a reusable core client layer.

The immediate target is feature parity with the current MCP in `/Users/adin/mcps/splitwise-mcp`, but with a CLI-native interface, stable exit codes, shell completion, and ergonomic output.

## Source of Truth

- Current implementation: `/Users/adin/mcps/splitwise-mcp/src/index.ts`
- API path index: `/Users/adin/mcps/splitwise-mcp/spec/paths/index.yaml`
- Path specs: `/Users/adin/mcps/splitwise-mcp/spec/paths/`

The Rust CLI should use the same API coverage and semantic behavior as the MCP, especially for endpoints where HTTP 200 does not imply success.

## Product Decisions

- Language: Rust
- Runtime model: compiled CLI, not a runtime YAML-driven adapter
- Scope for v1: all current Splitwise endpoints covered
- Request modeling: typed where practical, raw JSON escape hatch everywhere it matters
- Response modeling: preserve raw JSON output; only add human-friendly formatting on top

## Recommended Architecture

Use one Cargo package with a reusable library and a thin binary:

```text
splitwise-cli/
  Cargo.toml
  src/
    main.rs
    lib.rs
    cli.rs
    config.rs
    auth.rs
    client.rs
    error.rs
    operations.rs
    output.rs
    commands/
      users.rs
      groups.rs
      friends.rs
      expenses.rs
      comments.rs
      notifications.rs
      reference.rs
  tests/
```

### Module Responsibilities

- `main.rs`: startup, error handling, dispatch
- `lib.rs`: exports reusable client and operation types
- `cli.rs`: Clap command tree, global flags, completion support
- `config.rs`: env/config loading and precedence rules
- `auth.rs`: bearer token resolution
- `client.rs`: `reqwest` wrapper, request building, response parsing
- `error.rs`: typed error model
- `operations.rs`: metadata for supported API operations and success semantics
- `output.rs`: `table|json|yaml` formatting
- `commands/*`: domain-level argument mapping and command handlers

## Command Shape

Use domain-first subcommands instead of exposing raw endpoint names:

- `splitwise users me`
- `splitwise users get <id>`
- `splitwise users update <id> ...`
- `splitwise groups list|get|create|delete|undelete`
- `splitwise groups add-user|remove-user`
- `splitwise friends list|get|create|create-many|delete`
- `splitwise expenses list|get|create|update|delete|undelete`
- `splitwise comments list|create|delete`
- `splitwise notifications list`
- `splitwise categories list`
- `splitwise currencies list`

For awkward write payloads, support both:

- ergonomic flags for common cases
- `--body @file.json` or `--body '{"key":"value"}'` for full control

This is important for endpoints like `create_group`, `create_friends`, `create_expense`, and `update_expense`, where payloads can become irregular or flattened.

## Core Types

```rust
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub output: OutputFormat,
}

pub struct SplitwiseClient {
    http: reqwest::Client,
    config: Config,
}

pub struct OperationSpec {
    pub name: &'static str,
    pub method: http::Method,
    pub path: &'static str,
    pub success_rule: SuccessRule,
}

pub enum SuccessRule {
    HttpOkOnly,
    FieldTrue(&'static str),
    ErrorsObjectEmpty(&'static str),
}
```

The key point is that the CLI must understand Splitwise's soft-failure responses.

Examples:

- `delete_friend`, `delete_group`, `undelete_group`, `delete_expense`, `undelete_expense`, `remove_user_from_group`: fail when `success != true`
- `create_expense`, `update_expense`, `add_user_to_group`: fail when `errors` is non-empty

## Config and Auth

Keep compatibility with the existing MCP environment variables:

- `SPLITWISE_API_KEY`
- `SPLITWISE_ACCESS_TOKEN`
- `SPLITWISE_OAUTH_ACCESS_TOKEN`
- `SPLITWISE_BEARER_TOKEN`
- `SPLITWISE_BASE_URL`

Recommended precedence:

1. CLI flag such as `--token`
2. Environment variables
3. Config file, for example `~/.config/splitwise/config.toml`

Useful global flags:

- `--json`
- `--yaml`
- `--output table|json|yaml`
- `--base-url`
- `--token`
- `--verbose`

## Output Behavior

Default behavior should be interactive and readable.

- Lists: table output by default
- Single resources: concise pretty output by default
- `--json`: exact parsed API response
- Errors: concise stderr summary plus non-zero exit status

Exit code policy:

- `0`: success
- `1`: transport/auth/validation failure
- `2`: Splitwise semantic failure returned in a 200 response

## Dependencies

Recommended crates:

- `clap`
- `tokio`
- `reqwest` with `json` and `rustls-tls`
- `serde`
- `serde_json`
- `thiserror`
- `toml`
- `directories` or `dirs`
- `comfy-table` or `tabled`

Recommended test crates:

- `wiremock` or `httpmock`
- `insta`

## Implementation Phases

### Phase 1: Skeleton

- Initialize Cargo package
- Add CLI parsing
- Add config/auth resolution
- Add shared error model

### Phase 2: HTTP Core

- Build `SplitwiseClient`
- Support path params, query params, JSON bodies, and parsed JSON responses
- Add reusable semantic success evaluation

### Phase 3: Read-Only Commands

Implement first:

- `users me`
- `users get`
- `groups list|get`
- `friends list|get`
- `expenses list|get`
- `comments list`
- `notifications list`
- `categories list`
- `currencies list`

This gets a lot of utility with relatively low risk.

### Phase 4: Mutation Commands With Raw Body Support

Implement:

- `users update`
- `groups create|delete|undelete|add-user|remove-user`
- `friends create|create-many|delete`
- `expenses create|update|delete|undelete`
- `comments create|delete`

For v1, every mutation should work even if the typed flags are minimal, as long as `--body` supports the full payload shape.

### Phase 5: Ergonomic Builders

Add higher-level flag-driven builders for the most common writes:

- `expenses create`
- `groups create`
- `friends create`
- `comments create`

The raw JSON path remains available for edge cases and API parity.

### Phase 6: Packaging and DX

- Add shell completions
- Add `--help` examples
- Add release build instructions
- Add `cargo install --path .` flow
- Consider GitHub Releases for prebuilt binaries

## Testing Plan

- Unit tests for config precedence
- Unit tests for `SuccessRule`
- Integration tests for HTTP request construction
- Integration tests for semantic failure detection on 200 responses
- Snapshot tests for CLI help text and output
- Coverage test asserting all paths in `spec/paths/index.yaml` are represented

## Non-Goals for V1

- Do not load the YAML specs at runtime
- Do not fully generate Rust request/response models from the OpenAPI
- Do not optimize for a TUI or interactive wizard flow yet
- Do not hide raw JSON output from scripting users

## Why This Shape

The current MCP works well because it is generic and thin. The CLI should not copy that literally.

For a CLI, the better split is:

- curated commands for usability
- reusable generic client underneath
- raw JSON escape hatch for complete coverage

That gives a good v1 without turning the project into a large codegen effort.
