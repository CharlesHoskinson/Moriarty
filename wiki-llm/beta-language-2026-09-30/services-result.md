# Beta stdio, editor and AI service result

Observation date: 2026-09-30T18:18:03.662028+00:00. Author implementation result; independent whole-candidate review belongs to the root integration task.

## Delivered behavior

Repository observation: `servers.ts` exports `runLsp()` and `runMcp()` and consumes
only the shared source-text APIs. No source AST, filesystem path, shell, network,
signing, proof, or financial qualification API is exposed. The root CLI routes
`mori lsp` and `mori mcp` to these services. This is authoring and local preparation.

LSP capabilities are Full synchronization, UTF-16 diagnostics and editor ranges,
lexical/context-limited completion, declaration/action hover and definition,
document symbols and strict formatting. Successful analyses are invalidated by
new Full source generations, including over-budget source. Stale versions and
duplicate opens cannot replace current documents. Incomplete source may receive
lexical completion; it never enters lowering through the LSP. Hover/navigation and
symbols require a current strict analysis. Semantic tokens/debugging are absent.

MCP uses actual newline-delimited JSON-RPC stdio, initialization and the closed
`check`, `inspect`, `expand`, `preview` tool schemas. Expand and preview invoke the
same library and preserve source/scenario digests, local stipulation labeling,
Core outcomes, required external premises and unverified bindings. Tool errors do
not create a prepared artifact. `PreparedUnqualified` does not close proof or ledger
gates. No client/global configuration was changed.

## Finite transport admission

Repository observation: LSP headers <=8192 bytes, incoming bodies <=262144 bytes,
and individual serialized responses <=524288 bytes. The LSP decoder validates
Content-Length before allocating a body, rejects duplicate/malformed length,
invalid UTF-8/JSON and truncated input, and closes framing violations. MCP line
admission is likewise <=262144 bytes. Analysis is synchronous per frame; there is
no promise queue. A stalled output consumer closes the connection once bounded
pending output exceeds twice the maximum response budget.

Source documents are <=65536 UTF-8 bytes, at most32 open URIs and <=1048576 aggregate
source bytes. URI and version fields are validated. A new oversized Full edit
replaces the prior successful analysis with a rejected generation; valid later
versions can recover. Every diagnostic/source range uses the shared
`byteToPosition` UTF-16 conversion. No filesystem lookup is derived from a URI.

## Test-first and observed corrections

The initial new service test run had 0/3 passing groups because the protocol and
server modules did not yet exist. Subsequent behavioral red assertions reproduced
(1) hover highlighting the declaration rather than the actual reference,
(2) a newer oversized Full edit leaving prior symbols available, and
(3) invalid JSON-RPC envelopes failing to receive Invalid Request, and
(4) record field keys being mistaken for identifier references. Navigation now uses
the shared frontend reference spans and declaration/action name spans. These were
repaired and verified. Completion excludes comments and strings.

## Reproduced commands and results

From the repository root:

```sh
node --test packages/moriarty-beta/tests/protocol.test.mjs
```

Experiment observation: 11 test groups passed, 0 failed. They cover byte-split
Unicode framing, pre-body length/header rejection, response limits, real subprocess
LSP initialize/open/update/close/format/hover/definition/symbols/completion,
monotone versions, rejected-generation recovery, CRLF/astral UTF-16 locations,
32-document and independent aggregate-byte admission, actual MCP initialize/list/
check/inspect/expand/preview, closed bounded arguments, S0 candidate state and open
premises, real process closure on framing violations, Invalid Request recovery, and distinguishing actual references from record keys.

From `packages/moriarty-beta`:

```sh
npm run typecheck
npm run build
MORIARTY_BETA_PROTOCOL_CLI="$PWD/dist/cli.js" node --test tests/protocol.test.mjs
MORIARTY_BETA_CLI="$PWD/dist/cli.js" nvim --headless -u NONE -l editor/nvim/smoke.lua
node editor/vscode/prepare.mjs
```

Experiment observation: strict TypeScript checking passed. The same11 protocol
groups passed through the actual bundled CLI routes. Installed Neovim **0.11.6**
passed actual stdio initialize/open/UTF-16 negotiation/document symbols/definition/
format/update/diagnostics/shutdown. The test used a temporary buffer and no user
configuration. Exact Neovim result:

```text
Neovim Moriarty stdio LSP smoke passed: initialize/open/definition/symbols/format/change/diagnostics/shutdown
```

VSIX preparation and packaging:

```sh
cd editor/vscode
npm install --omit=dev --ignore-scripts
npm exec --yes --package @vscode/vsce@3.6.2 -- vsce package --allow-missing-repository
```

Experiment observation: `vscode-languageclient`9.0.1 installed from the committed
extension lock; npm reported8 installed packages and no audit findings. The pinned
packager produced `moriarty-beta-0.1.0.vsix`: **328 ZIP entries;509161 bytes**.
Manifest, language configuration, grammar and snippets parsed as JSON; extension
JavaScript passed `node --check`. ZIP validation confirmed the manifest/main,
client dependency, lexical grammar/snippets and ESM bundled server. Packaged server
bytes matched `dist/cli.js`. After extraction to `/tmp/mori-vsix-final-client-xl_1o39n`,
the same11 test groups passed with `MORIARTY_BETA_PROTOCOL_CLI` pointing to that
extracted server. The server therefore ran outside the checkout; that observation
is distinct from VS Code activation.

The packager warned about166 JavaScript files among328 archive entries and
recommended bundling the extension client. Its dependency files are currently
included; the CLI server is already bundled. No activation claim follows from this
package. Final root changes must rebuild, rerun `prepare.mjs` and repackage before
using these particular binary digests as final-candidate evidence.

## Asset inventory and limits

- VS Code: TextMate grammar, snippets, brackets/comments configuration, actual
  `vscode-languageclient` stdio extension, standalone server preparation, README,
  lockfile and package metadata. Real VS Code activation is **unperformed**.
- Neovim: explicit project-local client helper and reproduced headless smoke.
  Other editor client activation is **unperformed**.
- AI: inert project instructions, opt-in skill, Copilot/Claude/Codex/Cursor adapter
  files and valid/invalid scalar source examples. Vendor plugin/model activation,
  training and understanding are **unperformed**. The examples demonstrate
  authoring, not financial settlement.
- Existing Source/6/Core/5 semantics, native/authentication/ledger qualification,
  external acceptance gates and formal correspondence remain outside this result.
  No external financial action was submitted.

## Recorded digests

| Observed path | SHA256 |
| --- | --- |
| `packages/moriarty-beta/src/servers.ts` | `16442c6a55b39a76e909e6d7aedadcf03ae107f2c7bd1f47957bb4d22c02daab` |
| `packages/moriarty-beta/src/protocol.ts` | `dec81573439fac840a91a8bb8ed43b6f98bfa0239fc42945140c47b357a3c458` |
| `packages/moriarty-beta/tests/protocol.test.mjs` | `0f1e5416e8f0a167fa544bc0d79e5756613c26c2f3b741b70ec7d25eb2cabdd2` |
| `packages/moriarty-beta/editor/vscode/moriarty-beta-0.1.0.vsix` | `d58c0d950677d0f8ab129d5ad01fc7c2ece295accf7a8f8af3fc746eae47177f` |
| `packages/moriarty-beta/editor/vscode/server/cli.js` | `cd53003791a86659225ccb2b9791c9e304e361a3da931fe291391405f5526f69` |
