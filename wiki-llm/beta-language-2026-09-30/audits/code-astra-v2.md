# Independent whole-package implementation/result audit — code v2

Date: 2026-09-30. Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.

**Compliance verdict: PASS WITH MINOR FINDINGS for the bounded authoring/local S0 beta scope. Code-quality verdict: PASS WITH MINOR FINDINGS. No remaining critical, high, or medium defect was identified in this review.** The two low findings below remain open. This is sufficient for the authorized independent developer trials, not financial, proof, provider, or ledger acceptance.

Requested reviewer: fresh independent `gpt-6-astra`, medium effort. Observed identity: this independently delegated reviewer context; the host does not expose a separate provider-returned model/effort attestation to this context. The requested identity is recorded without inventing an attestation. No peer audit, prior repair report, or prior verdict was consulted. The parent subsequently confirmed the intended manifest ordering, and requested that both independently found minor issues remain recorded without editing the candidate.

## Frozen input and startup

Read checkout `AGENTS.md` and `plugins/moriarty-dev/skills/develop/SKILL.md`, then ran the guarded `status --json`. It reported unresolved historical campaign admission/accounting gaps and no pending transactions. This read-only audit does not dispatch that campaign or override its stops.

Manifest: `audits/code-candidate-v2.json`, baseline `870f998b36ecda04622fa4274132e74902942d0b`. Verified all 58 individual SHA-256 values before review and again before writing this report. With entries sorted by path, the concatenation of `sha256 + "  " + path + "\n"`, encoded as UTF-8, hashes to:

`cbfa1e295887f6ebc509bc7c1e54022cffe36d4be5ef177eb389010c77895b7c`

This matches the supplied digest. Literal sorting of complete hash-prefixed lines produces a different aggregate; see L02. `git diff <baseline> -- experiments/moriarty-language/src` was empty. Existing Source/6 and Core/5 production sources are unchanged. No product edits or commits were made by this reviewer; this audit is the sole requested output.

## Remaining findings

### L01 — Low: incoming editor positions disagree with outgoing positions for CR-only files

Locations: `packages/moriarty-beta/src/servers.ts:47`, especially lines 49–50; compare `packages/moriarty-beta/src/frontend.ts:404` and its CR handling. The server's inbound `offset` scans only LF. Its outgoing `byteToPosition` treats CR, LF, and CRLF as line breaks. Consequently a valid, accepted CR-only source receives symbols on lines that cannot be navigated through the same server.

Reproduced in an actual `node packages/moriarty-beta/src/cli.ts lsp` subprocess, using Content-Length frames:

```js
const text = 'profile "moriarty-beta/1";\ragreement Demo {\r const amount = 1;\r const total = amount + 1;\r}';
```

After `initialize` and `didOpen` at version 1, diagnostics are empty. `documentSymbol` reports `amount` on line 2 and `total` on line 3. `textDocument/definition` at `{line:3,character:16}` returns error `-32602`, `Editor line out of range`, rather than the `amount` definition. Hover/completion share this inbound conversion. LF and CRLF paths exercised by existing tests are unaffected. This is a minor editor consistency defect, not financial acceptance or stale Core execution.

Recommended repair: use one line-break policy in both directions and add a CR-only navigation discriminator. Also make an explicit choice for CR termination of line comments in the frontend lexer and lexical editor scanner, rather than leaving newline handling inconsistent.

### L02 — Low: aggregate algorithm description omits its sort key

Location: `wiki-llm/beta-language-2026-09-30/audits/code-candidate-v2.json:5` (`aggregate_algorithm`). The declaration says `SHA256 of sorted UTF8 sha256-two-spaces-path-LF lines`. Sorting the completed lines lexically yields:

`441deb3a9fb9415c5c152236dcbbe6cf647c81e193b454e334c7f75df72c0504`

Sorting entries by path before constructing lines yields the recorded `cbfa...` digest. The parent confirmed this latter algorithm is intended. Every file hash matches, so this is a reproducibility wording defect, not evidence of changed content. Specify `entries sorted lexicographically by path; SHA256 of their UTF8 sha256-two-spaces-path-LF lines` in a successor manifest/receipt.

## Production inspection and scope conclusions

Inspected all package production TypeScript modules, build/package/type configuration, lockfile metadata, editor client and preparation scripts, syntax/snippet assets, AI guidance, all eight family examples, starter, every test source/fixture, and both CONVERGENCE and IMPLEMENTATION-PLAN. Read the existing Source/6 parser/lowerer and Core/5 preparation path relevant to the bridge, rather than inferring behavior from passing tests.

- **Frontend and money:** bounded lexer/parser, ASCII names, prior-only resolution, immutable shared declaration values, canonical numeric spelling, exact bigint calculations, nominal quantities, asset/domain identity checks, closed operation arguments, UInt128 intermediates and S127 S0 field narrowing are present. Duplicate economic identity checks prevent aliases from changing scale/representation for the same declared identity. No floating point conversion, rounding, or hidden monetary defaults occur in the reviewed path.
- **JSON and formation:** the raw scenario transport checks decoded duplicate keys before JSON.parse discards them, bounds depth/nodes/fields/strings, rejects caller object transport, and closes scenario, balances, allowance, obligation and candidate-effect records. Required ordered cells and counter/debt formation relations correspond to strict Source/6. Source/scenario inputs remain separate. Shape failures return no partial expansion or published effects/post.
- **Rejection ordering:** representable opaque claims, including escaped controls, reach unchanged Core judgments. Shape constraints are formation checks; the bridge does not pre-judge caps, current validity, economic balance sufficiency, allowance or replay. Core retains stage → intent → effect → authority → history → failure ordering. The total repay proposal retains syntactic debt values on overpayment, records an explicit invalid-proposal note and defers failure to Core. It cannot become a prepared post on that route. Source/6-unrepresentable shapes are outside the claimed exact Core comparison domain.
- **Complete effects/post:** transfer proposals contain gross debit, recipient credit, optional nonzero fee credit, allowance, replay and head in order; zero fee still requires three distinct snapshot cells. Repay applies accrued amount first, then principal, and supplies the complete obligation effect plus debit, credit and administrative effects. Core compares the entire ordered vector and constructs the full post, including allowance spent/remaining, replay consumption, head and work. The bridge returns the actual wrapper outcome with external premises and unverified bindings.
- **Inspection/provenance:** `inspect` exposes the signed S0 operation, endpoint/obligation claims, amounts/fees/caps/floors, validity, keys/nonce/head, selected claims and explicit empty/None terms. Shallow entity claims are marked unauthenticated; bounded traversal prevents exponential serialization of shared nested values. Expansion provides input digests, contiguous generated byte spans, exact field-use roots, prior declaration definitions and transitive dependency edges. Generated selection rules are separately tagged. Origin count and serialized expansion bounds reject with no partial artifact.
- **Horizon:** the 24 registered non-S0 operations and eight family examples receive `SpecifiedOnly`; expansion and simulation reject them. Optional descriptive horizon fields are not evidence of complete typed lifecycle or financial relations. The separately reviewed full-language horizon document is excluded from this audit. The package explicitly keeps those relations, authentication and execution open.
- **CLI/API/starter:** the public entry module has no CLI launch side effect. CLI argument handling, nonzero rejection status, no-overwrite initialization, contained finite case-file resolution, exact optional effects/post expectations and strict formatting use the shared implementation. No source-built AST eligibility flag is consumed. The starter supplies explicit local balances and counters and tests complete expected financial results.
- **Protocols and tools:** framing has header/body admission bounds, fatal UTF-8 decoding, fixed response size and output backpressure closure. LSP uses synchronous current-document analysis, Full changes, increasing versions, duplicate URI/count/aggregate admission checks, close invalidation and lexical navigation. Oversized admissible Full changes clear previous successful analysis. L01 is the remaining newline inconsistency. MCP exposes only four closed text tools; there is no shell, arbitrary file/path, signing, proving, deployment, network transaction or ledger operation. Initialization and readiness checks are present.
- **Distribution/editor/AI:** esbuild bundles existing local S0 dependencies; declaration generation includes their types; package exports and bin target the distribution. Prepack rebuilds and copies the exact CLI into the VS Code extension's ESM server folder. Independently compared current `dist/cli.js` and `editor/vscode/server/cli.js`: equal bytes. The VS Code client invokes Node with explicit argv and `shell:false`; Neovim uses the same stdio server. AI adapters are inert project guidance and accurately retain unqualified statuses. No actual VS Code activation or AI-provider understanding is established here.

## Actual result evidence and limitations

Read the current parent-run output `/home/charl/research/moriarty-beta-2026-09-30/beta-tests-code-v2.txt`: **63 tests, 63 pass, 0 fail**, including the packed install outside the checkout, CLI commands with empty PATH, inert public API import, TypeScript consumer import, and byte equality of bundled IDE/distributed CLI. Inspected the actual distribution test implementation and its assertions. This was the current parent execution, not a second independently run full suite. The earlier 938/938 baseline regression is parent-reported evidence and was not rerun here.

Independently executed **14 focused successful probes**: mutate every ordered effect in successful transfer and repay candidates (12 probes), each yielding `CoreRejected/S0_EFFECT_MISMATCH` with null published post; traverse both expansions' entire origin graphs and all field roots, verifying every dependency exists, no cycles, and contiguous field-map coverage through the complete Source/6 byte length (2 probes). Independently reproduced L01 in the real stdio process. Probed large valid shared-reference checker inputs up to 4,000 references/56,083 source bytes; they remained bounded in the observed results (413,222 serialized checker bytes at that size). These probes supplement production inspection and do not prove all bounded inputs correct.

No provider activation, actual VS Code activation, financial authentication, native proof generation, K/Quint correspondence, financial ledger settlement, or atomic ledger acceptance was performed or inferred. Local results remain **PreparedUnqualified**. The open canonical-intent-signature, snapshot-to-head, head-extension and atomic-ledger-compare-and-consume premises and the four unverified bindings remain material limitations. The verdict applies only to the exact 58-file frozen candidate and the scope above; changed bytes require current affected-result review.
