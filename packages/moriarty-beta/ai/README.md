# Project-scoped AI adapters

These files are inert project guidance. Copy the relevant adapter into a project
only when that project's owner requests adoption. Keep the full `AGENTS.md`
guidance available alongside it. No provider activation, model training or vendor
understanding has been tested or claimed.

The MCP process is `mori mcp` (or `node /absolute/path/dist/cli.js mcp`). Its stdio
transport is newline-delimited JSON-RPC. Initialize the connection, then send
`notifications/initialized` before `tools/list` or `tools/call`. Its tools are
`check`, `inspect`, `expand` and `preview`. Each accepts only bounded source text;
expand/preview additionally accept an action name and bounded scenario JSON text.
Tool calls have closed argument schemas. No arbitrary paths, ASTs, shells, hosted
accounts, signing, proving, deployment or transactions are exposed.

For a client configured explicitly by a project owner, the process entry is:

```json
{"command":"node","args":["/absolute/path/to/dist/cli.js","mcp"]}
```

This is an example process descriptor, not a global client configuration. Provider
schemas differ; consult the relevant client before adopting it. No configuration
has been changed by shipping this pack. Node >=24 is required.

`examples/valid.mori` demonstrates unsigned scalar authoring. `invalid.mori`
contains noncanonical spelling and an unknown reference. Neither is a financial
settlement example. The CLI starter provides the actual local S0 example.
