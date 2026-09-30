# Independent result audit: Moriarty beta code candidate v2

**Auditor:** Grok 4.6 (this session’s system identity: “Grok 4.6 released by xAI”).  
**Requested:** `grok-4.6`, high effort.  
**Returned receipt:** none. The host did not expose a separate provider identity, effort telemetry, or signed review receipt. This report records the requested routing and the observable session identity only.  
**Mode:** read-only whole-candidate audit. No edits, commits, config changes, campaign dispatch, credential/wallet access, peer-audit reads, or subagents.  
**Checkout:** `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` at `870f998b36ecda04622fa4274132e74902942d0b`.  
**Startup:** `moriarty-dev:develop` loaded; guarded `status --json` only.

Guarded status: capability `SP01.6 loan-swap-subset`; `pendingTransactions: []`. No Midnight notification lines. Status also reports unrelated SP01 accounting/history gaps; those do not block this read-only beta review.

Scope is unqualified local authoring/preparation. Local tests and this vote do not qualify native proof, authenticated provider, cryptographic signature, or ledger commit.

---

## Verdicts

**Spec-compliance: PASS with findings** against CONVERGENCE v2 / IMPLEMENTATION-PLAN for the frozen v2 package, as unqualified local S0 preparation.

The production path (public text APIs → frontend → closed scenario JSON → Source/6 emitter → existing Core/5 → CLI/LSP/MCP) matches the selected contract on economic identity, Qty/S127 bounds, source-fixed vs untrusted scenario, duplicate-key JSON, overpay placeholder with Core first-failure order, ordered effects/post/work/replay, origin closure + input digests, parser/formatter budgets, LSP stale-version and UTF-16 conversion, readonly MCP/stdio admission, packed CLI/library identity as tested in recorded distribution evidence, and horizon `Unsupported` with no artifact. Typed full-language horizon grammar is out of this candidate.

**Code quality: ACCEPTABLE** for a bounded beta. One library feeds CLI, LSP, and MCP. Schemas are frozen. Failures return closed statuses and null published effects. The implementation is extremely dense (`frontend.ts`, `bridge.ts`, `index.ts` inspect, `cli.ts`), which concentrates risk in review, not in the challenged financial behaviors.

No High defect. One Medium honesty issue on `inspect` of SpecifiedOnly-only programs. Remaining issues are Low.

---

## Candidate freeze

Manifest `wiki-llm/beta-language-2026-09-30/audits/code-candidate-v2.json`:

| Field | Observed |
| --- | --- |
| schema | `moriarty-beta-code-candidate/1` |
| version | `v2` |
| baseline | `870f998b36ecda04622fa4274132e74902942d0b` = `HEAD` |
| files | 58, all present |
| per-file SHA256 | all match working-tree bytes |
| claimed aggregate | `cbfa1e295887f6ebc509bc7c1e54022cffe36d4be5ef177eb389010c77895b7c` |

Aggregate matches SHA256 of path-sorted `sha256  path\n` lines (JSON file order is already path-sorted). Sorting the same lines lexicographically by hash produces a different digest (`441deb3a…`). Integrity holds under the sha256sum-by-path reading of the stated algorithm.

Unlisted generated package files (outside the freeze): `editor/vscode/moriarty-beta-0.1.0.vsix`, `editor/vscode/server/cli.js`, `editor/vscode/server/package.json`. `prepare.mjs` copies `dist/cli.js` into the extension server path.

Existing Source/6 and Core/5 under `experiments/moriarty-language/src/successor` are imported, not copied. `git diff` vs the freeze baseline on that tree is empty.

---

## Challenge results

### Financial identity and Qty bounds

Economic IDs (`domain`/`account`/`asset`/`obligation` `id` fields) are what Source/6 and scenario keys use. Declaration rename `Buyer` → `Purchaser` left Core identities unchanged. Duplicate domain IDs and in-domain account/asset IDs reject (`frontend.ts` `identity()`, ~298–307). Transport alphabet matches CONVERGENCE (`[A-Za-z][A-Za-z0-9_]{0,63}` plus Source/6 reserved exclusion).

Qty/scalars are checked at every arithmetic step against `0..2^128−1` (`range()`, `frontend.ts` 206). S0 value/fee/caps/floors/repay amount narrow to `2^127−1` (`qty(..., true)`, 136–139, 360–371). Independent probe: `UInt128 max + 1 Qty` → `BETA_UINT128_BOUND`; transfer value `2^127` → `BETA_S127_BOUND`. Gross debit is `value+fee` as UInt128 and is left to Core for cap/balance comparison.

Inspect qty tags still carry the **declaration** asset name (`USD`) while `intent.asset.claims.id` is the economic ID (`A`). Both are present on the S0 inspect path.

### Source-fixed vs untrusted JSON scenario

`expand`/`simulate` take source text, action name, and scenario **text**, then re-`analyze` (`bridge.ts` 84–106). Scenario domain/asset must match source economic IDs; extra fields such as `gross_cap` reject (`BETA_SCENARIO_SCHEMA`). Object/getter inputs return `FormationRejected` / `BETA_JSON_TEXT` without invoking getters. Candidate effects may replace the proposal shape; Core still compares them to the intent-derived expected vector.

### Duplicate decoded JSON keys and closed schema

`parseBoundedJson` walks decoded keys with a `Set` before `JSON.parse` (`json.ts` 49–54, 79). Probe: `{"head":"h0","\u0068ead":"evil"}` → `BETA_JSON_DUPLICATE`; expand publishes no `source6`. Required/optional scenario keys are closed (`bridge.ts` 50). Integers are canonical decimal strings with UInt128/S127 maxima. Trailing JSON material rejects.

### Overpay proposal and Core order

Overpay 31 vs outstanding 30: expand is total; Source/6 keeps `set_obligation … principal 20 accrued 10 outstanding 30` and still emits `debit Payer 31` plus admin lines; `proposalNote` is set (`bridge.ts` 193–201). Simulate → `CoreRejected` / judgment `effect` / `S0_EFFECT_RANGE` / `publishedPost` and `publishedEffects` null.

Same overpay with `round: 11` → judgment `intent` / `S0_INTENT_SCOPE` (Intent before Effect). Overpay with stale head → `effect` / `S0_EFFECT_RANGE` (Effect range before History), matching unchanged Core (`mil4-s0-core-v5.ts` 220–322).

### Ordered effects, post, work, replay

Independent transfer simulate:

| Slot | Observed |
| --- | --- |
| effects | Debit Owner A 1010, Credit Recipient 1000, Credit Fee 10, UseAllowance Owner 1010, UseReplay `["Midnight","Owner","n1"]`, AdvanceHead h0→h1 |
| balances | 8990 / 1000 / 10 |
| allowance | remaining 8990, spent 1010 |
| work | remaining 9, spent 1 |
| replay | consumed composed key |
| qualification | `local-stipulation-only` |
| premises | four required; four unverified bindings |

Zero fee: five effects (no fee Credit), three snapshot cells, Fee balance `0`. Source/6 lowering composes UseReplay from domain+signer+nonce (`financial-agreement-source-v6-frontend.ts` 347–351).

### Origin graph and input digests

Origins are `generated` / `source` / `derived` / `scenario`. Agreement origin is the identifier span `Invoice`. `intent.signed_action` closure on `original * 2` reached both `original = 5.00 USD` and `price: Qty<USD> = original * 2` plus the USD declaration. `sourceHash` is SHA-256 of exact source bytes. Bound failure (`BETA_ORIGIN_BOUND`) returns no `source6`/`origins`/`fieldMap`.

### Parser, formatter, budgets

Lexer/parser enforce source 65536, tokens 8192 including EOF, depth 64, declarations 256, fields 64, strings 1024, identifiers 64, edges 32768. Format uses original token spelling/comments, re-analyzes, and rejects invalid/incomplete source. Generated Source/6 is byte-capped at 65536 and then admitted by the existing Source/6 65536/8192 token parser.

### LSP stale-version and UTF-16

`positionEncoding: 'utf-16'`. Full sync only. `version <= doc.version` is ignored (`servers.ts` 81–82). Duplicate open does not replace. Over-budget Full replaces the stored analysis with a `BETA_RESOURCE` generation. `byteToPosition` (`frontend.ts` 404–413) is the shared UTF-16 helper. Hover/definition use frontend reference spans; record keys do not navigate. Recorded protocol tests and Neovim 0.11.6 smoke are in `repair-beta-tests.txt` / `services-result.md`; this auditor did not re-run those processes.

### Readonly MCP / stdio bounds

MCP tools are `check` / `inspect` / `expand` / `preview` with `readOnlyHint`, closed text fields, no path/shell/network. Framing: header 8192, body 262144, response 524288; violations close stdio. One synchronous dispatch per admitted frame.

### Packed CLI/API and IDE server

`package.json` exports library `dist/index.js` (no CLI side effects) and bin `dist/cli.js`. `distribution.test.mjs` packs, installs with `PATH` empty, checks CLI/API/types, and requires `editor/vscode/server/cli.js` bytes to equal `dist/cli.js`. Recorded run: 63/63 including that pack test. This auditor did not re-run `npm pack`.

### Horizon unsupported statuses

Closed 24-name registry. Examples check as `SpecifiedOnly` / `localPreparation: Unsupported`. Expand/simulate of `examples/lending.mori` → `Unsupported` / `BETA_PROFILE_UNSUPPORTED`, no `source6`, `publishedEffects`/`publishedPost` null. Stage/policy authoring strings are free-form; typed lifecycle status enums are the excluded horizon document.

### Inspect signed terms

On S0 transfer/repay, `inspect` returns per-intent terms: nonce, key, pre_head, valid window, caps/floor, operation name, value/fee or repay amount, endpoint economic IDs, `authenticated: false`. CLI and MCP inspect use this same `index.ts` function.

---

## Findings

### M1 — Medium — `inspect` always attaches S0 premises

**Where:** `packages/moriarty-beta/src/index.ts` 40–44.

**What:** `inspect` always emits S0 `requiredPremises`, `unverifiedBindings`, `bounds.s0SignedFieldMax`, and `coverage.financialRelations: 'S0 only during local preparation'`, including for SpecifiedOnly-only programs.

**Repro:**

```js
import { inspect } from '@moriarty-lang/beta';
import { readFileSync } from 'node:fs';
inspect(readFileSync('packages/moriarty-beta/examples/lending.mori','utf8'));
```

Observed: `actions[].support === 'SpecifiedOnly'`, yet `requiredPremises` is the four S0 ledger premises. Expand of those actions is `Unsupported`.

**Repair:** Derive those fields from LocalS0 actions. If none exist, omit S0 premises/bindings (or emit an empty list) and set `financialRelations` to `Open`. Keep per-intent terms and per-action `support`.

### L1 — Low — `check()` leaks editor metadata

**Where:** `packages/moriarty-beta/src/frontend.ts` 398–402.

**What:** Public `check` strips declaration values but still spreads `references` and `fieldUses`. Plan text asked for a JSON-safe summary without expanded values; these span maps are extra internal editor data.

**Repair:** Return only the documented summary fields (status, diagnostics, agreement, stripped declarations/actions, `operationSchemas`, coverage, open gates).

### L2 — Low — CLI `--json` is a no-op

**Where:** `packages/moriarty-beta/src/cli.ts` 50, 75.

**What:** `check FILE [--json]` accepts `--json`; `check` always JSON-prints.

**Repair:** Honor `--json` vs text, or drop the flag from help and the allow-list.

### L3 — Low — over-budget LSP Full stores empty text

**Where:** `packages/moriarty-beta/src/servers.ts` 84–88.

**What:** An oversized Full change bumps version and invalidates analysis (required), and also replaces `doc.text` with `''`. The server document then diverges from the client while the version matches. Format of that generation returns `[]`, so it does not wipe the client, but later features run on empty text until a later valid Full arrives.

**Repair:** Keep the version bump and `BETA_RESOURCE` diagnostics; either retain the previous text as non-authoritative overflow, or return an LSP error after publishing the invalidation, with the stored text policy documented.

### L4 — Low — overpay `proposalNote` survives candidate override

**Where:** `packages/moriarty-beta/src/bridge.ts` 191–203.

**What:** `proposalNote` is set from the automatic overpay placeholder, then `candidate_effects` can replace the emitted lines. The note can describe a proposal that was not emitted.

**Repair:** Set `proposalNote` only when automatic effects are actually emitted.

### L5 — Low — frozen `IMPLEMENTATION-PLAN.md` still shows implementation unchecked

**Where:** `wiki-llm/beta-language-2026-09-30/IMPLEMENTATION-PLAN.md` 121–126.

**What:** The frozen plan still has Frontend/bridge/CLI/audit boxes unchecked while this candidate contains that implementation.

**Repair:** Mark completed engineering tasks, keep typed-horizon and publication boxes accurate.

---

## Abstentions

- **Typed full-language horizon document** — excluded from this candidate; free-form `stage.status` strings were not treated as that grammar.
- **VS Code / provider activation** — unperformed. VSIX packaging and extracted-server stdio are recorded in `services-result.md` only.
- **Re-run of 938 language regressions and full 63-test / `npm pack` / Neovim** — not repeated here. Recorded evidence: `regression-preaudit.txt` 938/938 pass; `repair-beta-tests.txt` 63/63 pass including packed PATH-empty CLI; Neovim 0.11.6 smoke line in `services-result.md`. This audit independently executed the S0/JSON/origin/inspect/horizon probes above.
- **Peer audits and `code-repair-result.md`** — unread by instruction.
- **Native proof, provider authentication, signatures, Preview/ledger settlement** — not claimed; results stay `PreparedUnqualified` / `local-stipulation-only`.
- **Successor byte identity vs a pre-beta historical commit** — successor is unchanged vs freeze baseline; last dedicated successor commit is older (`3eafa22e…`).

---

## Evidence this vote does not create

Local Core correspondence, author test counts, and this review are not financial ledger acceptance, K/Quint correspondence, or an authenticated provider result. Preview transaction IDs: none in this session.

## Parent transport receipt

The CLI returned `stopReason: end_turn`, process exit0, session `01a0f3b3-3b08-7e80-ae41-08a9544305b5`, modelUsage key `grok-4.6-build`. Requested model `grok-4.6`, CLI effort `high`; no independent returned effort attestation. This supplements the reviewer’s narrower in-session identity observation; it does not change its verdict. Sanitized structured receipt is retained alongside this report.
