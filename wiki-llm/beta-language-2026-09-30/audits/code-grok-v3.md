# Moriarty beta code-candidate v3 independent audit

## Reviewer identity (requested vs observable)

| Item | Value |
| --- | --- |
| Requested | `grok-4.6`, high effort |
| Observable identity | System prompt names this process **Grok 4.6** (xAI) |
| Effort receipt | High effort was requested. No separate host/provider identity or effort receipt is available in this session |
| Fabricated receipt | None. This is not a cryptographic signature, authenticated provider attestation, native proof, or ledger commit |

Startup: `AGENTS.md` and `plugins/moriarty-dev/skills/develop/SKILL.md` were applied. Guarded `status --json` was run. No campaign dispatch, no file edits, no subagents, no peer code-audit or repair-conclusion files.

## Frozen candidate

- Manifest: `wiki-llm/beta-language-2026-09-30/audits/code-candidate-v3.json`
- Schema `moriarty-beta-code-candidate/1`, version `v3`
- Baseline / `HEAD`: `870f998b36ecda04622fa4274132e74902942d0b`
- Files: 58/58 present, 0 hash mismatches
- Aggregate SHA-256 (lexicographic `sha256  path` LF lines): `c66b683b47aec11f3670f93510a73f706ee37369bb848164559edf122287ade9` **matches**
- Source/6 and Core/5 under `experiments/moriarty-language/src/successor/{financial-agreement-source-v6-frontend,mil4-s0-core-v5,mil4-s0-source-v6}.ts`: **0 bytes** of diff vs baseline

Selected contract: `CONVERGENCE.md` plus `IMPLEMENTATION-PLAN.md` / `BETA-DESIGN.md` as amended. Typed full-language horizon grammar is **out of this candidate**.

## Verdict

**Spec-compliance: pass for frozen beta/1 local authoring and unqualified S0 preparation.**  
**Code quality: pass** for a bounded, fail-closed, single-library CLI/LSP/MCP path.

No High or Medium production-path defect was demonstrated against the frozen contract. Local tests and this review **do not** qualify native proof, authenticated provider, cryptographic signature, or ledger commit. Status `PreparedUnqualified` remains an untrusted local stipulation.

Two Low contract-surface notes are recorded below. They do not break the challenged financial or Core-order predicates.

---

## Challenge results (production path, not tests-only)

### Financial identity and Qty bounds

Economic IDs are taken from fields, not declaration names, and must match Source/6 `[A-Za-z][A-Za-z0-9_]{0,63}` plus the same reserved-word set as Source/6 (`frontend.ts:21,141`; `bridge.ts:96–106,163–164`). Duplicate domain IDs, in-domain account IDs, and in-domain asset IDs (including conflicting scale/representation) reject (`frontend.ts:298–308`). Expression arithmetic is checked `0..2^128−1`; S0 value/fee/cap/floor/amount narrow to `2^127−1` (`frontend.ts:18–19,136–139,360–371`). Zero action amount is authoring-legal; a focused run of `value: 0.00 USD` was `AuthoringChecked` then Core `intent/S0_INTENT_SCOPE`. Caps are not pre-compared to value+fee. Declaration rename `Buyer`→`Purchaser` is tested to leave Core identities unchanged.

### Source-fixed request vs untrusted scenario JSON

Transfer/repay endpoints, amounts, nonce, caps, and signed action come from source. Scenario supplies head/predecessor/round/ordered balances/allowance/replay/work/post_head (and repay obligation cells). Domain/asset/signer/account order must match source (`bridge.ts:49–77`). Extra/missing fields reject. `candidate_effects` is the explicit hostile vector and cannot change `signed_action`.

### Duplicate decoded JSON keys / closed schema

`parseBoundedJson` walks text, decodes keys with `JSON.parse` on each key slice, rejects duplicates, then parses (`json.ts:19–79`). Fixture `'"head":"h0","\\u0068ead":"evil"'` is `BETA_JSON_DUPLICATE`. Integers must be canonical decimal **strings** (`bridge.ts:29–31`). Non-string JSON input throws `BETA_JSON_TEXT`. No public AST/object scenario API.

### Overpay proposal and Core order

Automatic overpay emits **unchanged** principal/accrued/outstanding plus requested debit/credit/admin lines (`bridge.ts:193–201`). Core still does Intent (round/caps) before `n > outstanding` → `effect/S0_EFFECT_RANGE` before effect-vector compare and before history (`mil4-s0-core-v5.ts:220–323`). Recorded cases: overpay → `S0_EFFECT_RANGE`; overpay+expired → `intent/S0_INTENT_SCOPE`; overpay+stale head → `S0_EFFECT_RANGE` (Effect before History). Placeholder is never `PreparedUnqualified`.

### Ordered effects, post, work, replay

Independent Source/6 oracles are compared field-for-field (`tests/bridge.test.mjs`). Transfer effects: Debit, Credit, Credit, UseAllowance, UseReplay, AdvanceHead; post balances `8990/1000/10`; work `9/1`; consumed replay `["Midnight","Owner","n1"]` (Core wrapping of source nonce). Zero fee: three snapshot cells, no fee credit. Premises/bindings length 4 on unqualified success.

### Origin graph / input digests

Expand returns SHA-256 of exact source and scenario text. Origins are `source | scenario | derived | generated` with memoized declaration-definition and use→definition links (`bridge.ts:117–153`). Focused/recorded tests show `original = 5.00 USD` and `price: Qty<USD> = original * 2` in the signed-action closure, plus account/asset declarations. Origin exhaustion (`BETA_ORIGIN_BOUND`) returns `FormationRejected` with `publishedPost/publishedEffects` null and no `source6`/`origins`/`fieldMap`. Generated `selected` includes a `generated` origin (`bridge.ts:154,165`).

### Parser / formatter totality and budgets

Source 65536 bytes, 8192 tokens including trivia+EOF, 8192 nodes, depth 64, 256 declarations, 64 fields, 1024-byte strings, 32768 edges (`frontend.ts:70–176,375–378`). Invalid source: one diagnostic, `format` returns `text: null`. Formatter preserves raw spellings/comments, re-analyzes, bounds output, LF-normalizes trivia. DAG values are reused, not cloned (`frontend.ts:158`; frontend tests).

### LSP stale version and UTF-16

`positionEncoding: 'utf-16'`. Full sync only. `version <= current` is ignored and does not republish (`servers.ts:82`). Over-budget Full source replaces analysis with empty unavailable generation and `BETA_RESOURCE` (`servers.ts:85–89`). `byteToPosition` is the shared UTF-16 converter (`frontend.ts:404–413`). Duplicate URI open does not replace. Protocol tests in the v3 69/69 log cover split frames, stale versions, UTF-16 diagnostics, CR/LF/CRLF navigation.

### Read-only MCP / stdio bounds

Header 8192, body 262144 **before** body alloc, response 524288, 32 docs, aggregate 1 MiB (`protocol.ts:2–35`; `servers.ts:77–79`). MCP tools are closed text schemas with `readOnlyHint: true`; extra keys, paths, and AST objects are `-32602`. Framing violations close stdio. Dispatch is synchronous per frame.

### Packed CLI / library / IDE server

Recorded v3 `distribution.test.mjs` packed outside checkout with `PATH` empty: CLI init/check/fmt/inspect/expand/simulate/test, ESM import, and strict `tsc` of published types. Inspect atoms `1010`/`1000`. This session: `packages/moriarty-beta/dist/cli.js` **equals** `editor/vscode/server/cli.js` (`94dbd70fe5585b2d04495fd2e9f57523bd45175697c9fb04202f8cf94245a6f7`). VSIX activation was not performed.

### Horizon unsupported statuses

24 dotted operations in the frozen catalog (`frontend.ts:32–54`; frontend catalog test). Horizon actions are `SpecifiedOnly`. Examples expand/simulate to `Unsupported` / `BETA_PROFILE_UNSUPPORTED` / `publishedEffects: null`. Typed lifecycle grammar is **not** implemented here.

### Inspect signed terms and conditional S0 scope

Inspect walks each intent’s fields into JSON-safe terms (caps, validity, operation args, economic claim ids, `authenticated: false`) (`index.ts:35–53`). Focused run:

| Program | `coverage.financialRelations` | S0 bounds/premises | Intent terms |
| --- | --- | --- | --- |
| SpecifiedOnly only | `Open` | absent | present (`governance.queue`) |
| Actionless S0 intent | `Open` | absent | present (`value.atoms === "1000"`) |
| Mixed LocalS0 + horizon | `ConditionalOnLocalS0Preparation` | only on LocalS0 actions | both intents |

Qty terms still name the **declaration** (`USD`); economic id `A` lives on identities/claims. That matches “declaration names for reference, economic IDs from fields.”

### Public JSON-safe span metadata vs human `check`

`Span` is `{start,end}` UTF-8 bytes. `check()` strips declaration **values** and `JSON.stringify`s (`frontend.ts:398–402`). Human `mori check` is **not** JSON: 1-based UTF-16 line/column via `byteToPosition`, action support, and `Local preparation remains PreparedUnqualified.` (`cli.ts:12–17`). `--json` is the machine object. Focused dump of `check(transferSource)` keys:

`status, sourceHash, diagnostics, agreement, agreementSpan, declarations, actions, references, fieldUses, operationSchemas, evidence, openGates`

`references` / `fieldUses` are extra relative to the original Analysis summary in `IMPLEMENTATION-PLAN.md`. They are still JSON-safe spans, not expanded values. See Low finding 1.

### CR / LF / CRLF

Line comments end on CR or LF (`frontend.ts:85`). Frontend tests accept CR, LF, and CRLF without swallowing the next declaration; formatter is idempotent after LF normalization. LSP tests use the same positions for all three.

### Unavailable overflow generation; automatic-only `proposalNote`

Generator does not wrap negative remaining debt: overpay keeps stipulated obligation cells. Receiver UInt128 overflow is a representable credit; Core `S0_EFFECT_RANGE`. Scenario remaining+spent sums are checked against UInt128 (`bridge.ts:75`). Unrepresentable candidate integers fail `uint()` / Source/6 (`SOURCE6_RANGE` / `TOKEN_BOUND` at 8192). `proposalNote` is set only when automatic overpay **and** no `candidate_effects` (`bridge.ts:198–202`); candidate override has `proposalNote: null` and can emit a different `set_obligation`. Confirmed in a focused expand.

---

## Findings

### 1 — Low: public `check()` JSON includes internal origin maps

- **File:** `packages/moriarty-beta/src/frontend.ts:398–402`
- **What:** `check` spreads full `Analysis`, so machine JSON includes `references` and `fieldUses` (and `agreementSpan`) in addition to the documented summary (status, hashes, diagnostics, declarations without values, actions, schemas, gates). Spans are JSON-safe UTF-8 byte offsets; values are omitted. This is extra surface, not a financial-identity failure.
- **Repro:** `JSON.stringify(check(transferSource))` contains `references` and `fieldUses`. Human `mori check` does not.
- **Repair:** Return a closed summary: `{status, sourceHash, diagnostics, agreement, declarations: {kind,name,span,nameSpan}, actions+coverage, operationSchemas, evidence, openGates}`. Keep reference/field-use tables internal to expand/LSP.

### 2 — Low: inspection work bound reuses `AuthoringRejected`

- **File:** `packages/moriarty-beta/src/index.ts:56–58` (bound throw at `index.ts:9`, `54`)
- **What:** An `AuthoringChecked` program whose inspect DAG walk exceeds 8192 work or 524288 bytes returns `status: 'AuthoringRejected'`, `BETA_INSPECTION_BOUND`, empty `identities/actions/intents`. Fail-closed and tested (`repair.test.mjs` nested `retained_effects`). Clients that treat `AuthoringRejected` as parse failure will misread a valid program.
- **Repro:** 20-level doubled array const referenced from a SpecifiedOnly intent `retained_effects` (existing test).
- **Repair:** Distinct status (for example `InspectionRejected`) or keep `AuthoringChecked` with only the bound diagnostic and empty signed scope.

No High or Medium items.

---

## Abstentions

- **Typed full-language horizon mockup / requirements matrix:** excluded from this code candidate; not treated as implemented.
- **VS Code extension activation and vendor AI-provider activation:** unperformed, as stated.
- **Neovim 0.11.6 and VSIX stdio in `services-result.md`:** that note hashes `servers.ts` as `16442c6a…` and `protocol.test.mjs` as `0f1e5416…`. Frozen v3 is `a9d07e76…` and `490f0d79…`. Those editor observations are **not** v3-byte evidence. v3 in-package protocol tests **are** in `beta-tests-code-v3.txt` (69/69). This session did not re-run Neovim or `vsce package`.
- **Fresh `npm pack` in this session:** not re-run; relied on recorded v3 distribution test plus current `dist/cli.js` ≡ `editor/vscode/server/cli.js`.
- **Existing 938/938 language regressions and package typecheck:** recorded in `regression-preaudit.txt` and `typecheck-code-v3.txt`; not re-executed here.
- **Native proof, K/Quint correspondence, authenticated snapshot/head, atomic ledger consume, Midnight settlement:** open. `PreparedUnqualified` is local preparation only.
- **Peer code audits / repair writeups:** not read.

---

## Evidence used

- Independent SHA-256 of all 58 manifest paths and aggregate.
- Production: `packages/moriarty-beta/src/{index,frontend,bridge,json,protocol,servers,cli,starter}.ts`, `build.mjs`, `package.json`, editor/AI/examples assets, Core/Source6 modules (unchanged).
- Recorded: `/home/charl/research/moriarty-beta-2026-09-30/beta-tests-code-v3.txt` (69/69), `typecheck-code-v3.txt`, `regression-preaudit.txt` (938/938), `wiki-llm/.../services-result.md` (editor evidence, hash-stale vs v3 servers).
- Focused discriminators this session: `__proto__` field (lexical `BETA_CHARACTER` on `_`), `check` key set, inspect qty/declaration vs economic id, zero-value Core Intent, actionless inspect scope, overpay `proposalNote`, JSON.stringify of check/inspect.

**Qualification line for any later packet:** local authoring and unqualified S0 preparation on candidate v3 `c66b683b47aec11f3670f93510a73f706ee37369bb848164559edf122287ade9`. Not financial ledger acceptance.

## Parent transport receipt

CLI returned end_turn/exit0, session `01a0f3ce-ff97-7103-a74d-786acf70bd33`, modelUsage key `grok-4.6-build`. Requested `grok-4.6`, effort `high`; no independent returned effort attestation. The manifest hashes entries sorted by path before constructing their sha256-two-spaces-path-LF lines. Current v3 editor/regression evidence was produced after audit launch and will be linked in final result, superseding older services-result byte scope.
