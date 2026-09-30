I'll start with the required startup reads and a guarded status check, then independently audit the frozen v6 candidate against production bytes.Using the Moriarty develop skill for a read-only independent audit. Next I’ll take guarded status, then freeze and hash-check the v6 candidate.Status is unblocked for this read-only review: no pending transactions. Next I’ll freeze the v6 manifest, verify hashes, and map the production package.Manifest lists 62 files with the expected aggregate. I’ll hash-check production bytes next and confirm v6 differs from v5 only in the two documentation files.Hashes match the frozen v6 aggregate; only the two documentation files differ from v5. I’ll now read the production sources, contract docs, and parent evidence independently.Source is compact enough to read in full. I’ll load the production path, Core/Source adapters, and the v5 evidence receipts next.I’ll inspect Core’s first-failure order, the tests that pin those judgments, and the parent v5 evidence next.# Independent code/result audit — Moriarty beta `code-candidate-v6`

## Auditor identity (requested vs observable)

- **Requested:** `grok-4.6`, high effort, fresh independent full-candidate audit.
- **Observable in this session:** the runtime identifies this agent as **Grok 4.6** released by xAI. No separate host provider identity, effort receipt, or canonical model ticket was returned. This report does not invent one.
- This review is **read-only**. Guarded `status --json` was the only plugin CLI invocation. No `next`/`run`, no campaign, no wallet/credential access, no file edits, no subagents, no peer/prior audit or repair files.

Guarded status: capability `SP01.6 loan-swap-subset`; `pendingTransactions: []`. That loan-swap stop is outside this beta-language candidate.

## Frozen candidate

| Item | Observed |
| --- | --- |
| Manifest | `wiki-llm/beta-language-2026-09-30/audits/code-candidate-v6.json` |
| Version / baseline | `v6` / `870f998b36ecda04622fa4274132e74902942d0b` |
| File count | 62/62 present |
| Per-file SHA-256 | all match |
| Aggregate algorithm | lexicographic `sha256  path\n` |
| Aggregate | `ea4f68158d893d6fa71d3048c3d164ac54912397604b134e6f52ff08a9738da6` (matches expected; recomputed from manifest entries and from disk) |
| v5 → v6 byte delta | **only** `packages/moriarty-beta/GETTING-STARTED.md` and `wiki-llm/beta-language-2026-09-30/CONVERGENCE.md` |
| Source/6 + Core/5 | `git diff 870f998b -- experiments/moriarty-language/src/successor` empty |

Production code/tests/package/editor/AI bytes are the v5 implementation. v6 adds documentation of (1) missing `qualification` never meaning qualification, (2) byte-exact `sourceHash` including trailing newlines, (3) intent `retained_duties` text or string arrays vs stage text only. Those three statements match current production.

Generated `packages/moriarty-beta/editor/vscode/server/{cli.js,package.json}` are outside the 62-file freeze (copied by `prepare.mjs` from `dist/cli.js`). That is consistent with `.gitignore` and the packed-install equality check.

---

## Verdicts

**Spec-compliance (bounded beta authoring + unqualified local S0): pass, with one Low SpecifiedOnly nominal-cap tightness gap.**  
The production path implements CONVERGENCE v2 + IMPLEMENTATION-PLAN for the in-scope beta: economic IDs, exact Qty, closed duplicate-safe scenario JSON, total overpay proposal with Core-first failure, origin closure, finite budgets, LSP/MCP bounds, honest Unsupported, inspect signed terms, conditional S0 inspection scope. This candidate is **unqualified local preparation**. It does not authenticate, prove, sign, or settle.

**Code quality: pass for this freeze.**  
One text frontend feeds unchanged Source/6 and Core/5; CLI, library, LSP, and MCP call the same APIs. Failures publish no partial Source/6. Public interfaces take text. Editor/AI assets stay inert and scoped. Density is high; the S0 financial path is still the original Core judgment order.

Local package tests, parent reproductions, and this auditor’s focused probes **do not** qualify native authentication, a cryptographic signature, a proof, or a ledger commit.

---

## Finding

### L1 — Low — horizon intent caps without a declared/bridge unit skip asset matching

- **File:** `packages/moriarty-beta/src/frontend.ts:357-367`
- **Contract:** CONVERGENCE “Provided cap quantities use the declared asset; bridge can infer its local amount asset when omitted. AMM swap net_floor uses the explicit output asset.” GETTING-STARTED lines 174–175 same rule.
- **Behavior:** `boundAsset` is `f.asset` or bridge `amount`’s asset. `net_floor` on `amm.swap_exact_input` uses `output_asset`. If `asset` is omitted on a non-bridge intent, `qty(cap, undefined)` accepts any nominal unit for `gross_cap` / `fee_cap`.

**Repro** (observed this session against current `src`):

```text
intent I = { operation: amm.swap_exact_input(..., fee_cap: 0.00 USD, net_floor: 2.00 GOLD),
             fee_cap: 1.00 GOLD };
```

without `asset:` → `AuthoringChecked`.  
Same `fee_cap: 1.00 GOLD` with `asset: USD` → `BETA_ASSET_MISMATCH`.  
`net_floor: 1.00 USD` on that swap (output GOLD) → `BETA_ASSET_MISMATCH` even without `asset:`.

**Repair:** when `gross_cap`/`fee_cap` are present and `boundAsset` is missing, reject (`BETA_MISSING_FIELD` / `BETA_ASSET_MISMATCH`), or infer the operation’s local amount unit (swap input for fee_cap). Keep AMM `net_floor` on the output asset and bridge inference on local `amount`.

**Scope:** SpecifiedOnly authoring. No S0 lowering, no Core judgment, no financial qualification. S0 transfer/repay still require a declared asset and S127-narrowed caps.

No High or Medium production defects were reproduced on the S0 path, JSON transport, inspect projection, origin closure, CLI encoding, or stdio bounds.

---

## Challenge results (production path)

| Challenge | Result |
| --- | --- |
| Financial identity vs declaration names | Source/6 and scenario keys use `id` fields. Renaming `Buyer`→`Purchaser` leaves Core identities unchanged (`bridge.test.mjs`; `bridge.ts` `economicId`). |
| Exact Qty bounds | Expressions `0..2^128−1`; S0 caps/amounts `0..2^127−1` (`frontend.ts` `qty(..., narrow)`, `range`). Zero amount is authoring-legal; Core Intent rejects it. |
| Source fixed vs untrusted scenario | `expand`/`simulate` take source+scenario **text**. Intent/operation come from analyzed source. Scenario cannot add intent fields. Unknown/missing scenario keys are `FormationRejected`. |
| Duplicate decoded JSON keys / closed schema | `parseBoundedJson` rejects decoded duplicates (`"head"` vs `"\u0068ead"`) before `JSON.parse`. Closed required/optional sets in `bridge.ts` `record()`. |
| Overpay proposal + Core order | Automatic overpay emits **unchanged** principal/accrued/outstanding plus debit/credit/admin lines; `proposalNote` only when automatic (`bridge.ts:199`). Core `prepareMil4S0`: Stage → Intent (window/caps) → Effect `S0_EFFECT_RANGE` for `n>outstanding` **before** `sameEffects` → Authority → History → Failure. Expired+overpay is Intent first; stale+overpay remains Effect (`mil4-s0-core-v5.ts:220-322`, `bridge.test.mjs:41-46`). Core bytes unchanged. |
| Ordered effects, post, work, replay | Transfer: Debit gross, Credit value, optional fee Credit, UseAllowance, UseReplay tuple, AdvanceHead. Work −1/+1. Replay `["Midnight","Owner","n1"]`. Starter/repay fixtures assert the full vector and post. |
| Origin graph / input digests | SHA-256 of actual source/scenario text. Origins tagged source/scenario/derived/generated, cap 2048. Closure from `intent.signed_action` reaches `const original`, `account Buyer`, and `domain Preview` (tests + this session). Bound exhaustion returns `FormationRejected` with no `source6`. |
| Parser/formatter totality / budgets | Source 65536 / 8192 tokens / depth 64 / 256 decls / 64 fields / 1024-byte strings / 64-char ids / 32768 edges. Format preserves comments/spelling, compact named calls via delimiter stack, re-analyzes, rejects >65536 formatted bytes. |
| LSP stale version + UTF-16 | `positionEncoding: 'utf-16'`. Full sync only. `version <= doc.version` ignored. Newer Full always re-analyzes; over-budget generation stores empty text + `BETA_RESOURCE` and cannot reuse the prior success (`servers.ts:81-89`). |
| Readonly MCP / stdio bounds | Header 8192 / body 262144 / response 524288; admission before body buffer; framing errors close. MCP NDJSON; tools `check/inspect/expand/preview`; closed text args; `readOnlyHint: true`; no paths/shell/signing. |
| Packed CLI/API vs IDE server | `distribution.test.mjs` packs, installs with `PATH` empty, checks `editor/vscode/server/cli.js` byte-identical to packed `dist/cli.js`, then CLI+library+strict `.d.ts`. Parent v5 receipt: 79/79. |
| Horizon unsupported | `support!=='LocalS0'` returns `Unsupported` **before** scenario parse, `scenarioValidation: NotAppliedUnsupported`, `qualification: local-stipulation-only`, null published effects. Confirmed with `{}`, `{`, invalid JSON, duplicate keys. |
| Inspect per-intent signed terms | Transfer/repay intents expose caps, rounds, operation args, empty arrays, `authenticated: false`. Native Claude MCP inspect JSON matches this shape. |
| Origin links to transitive definitions | Expand origin walk reaches declaration text of referenced consts, accounts, and domains. Inspect deliberately does **not** expand declaration DAGs. |
| S0 inspect scope | SpecifiedOnly: empty premises/bindings, `financialRelations: Open`. Mixed: top-level `ConditionalOnLocalS0Preparation` / `s0Scope: LocalS0ActionsOnly`; only LocalS0 actions carry S0 premises. Actionless: Open, empty premises; intent terms still present. |
| JSON-safe span metadata | `check` returns byte `{start,end}` on diagnostics, declarations, `agreementSpan`, `references`, `fieldUses`; declaration **values** stripped. Native MCP `check` content is JSON-parseable. |
| Human `check` vs `--json` | Default: `AuthoringChecked`, `name: support`, SpecifiedOnly suffix ` (execution unsupported; financial relations open)`, and LocalS0 line `Local preparation remains PreparedUnqualified.` `--json` is the machine object. Observed this session. |
| CR / LF / CRLF | Lexer, `byteToPosition`, LSP `offset`, and format treat the three endings as line breaks. Comments stop before the break. Protocol tests cover navigation/completion. |
| Overflow generation policy | Proposal builder does not pre-check balances/caps. Receiver overflow is Core `S0_EFFECT_RANGE`. S0 amount+fee fits UInt128. Unrepresentable candidate integers are formation errors. Bound failures omit `source6`. |
| Automatic-only `proposalNote` | Set only for automatic overpay; explicit `candidate_effects` clears it (`repair.test.mjs:88-92`). |
| Fatal CLI UTF-8 / BOM | `cli.ts:19` refuses BOM (`BETA_BOM`) then fatal UTF-8. `fmt --write` does not rewrite those bytes. |
| `InspectionRejected` / `authoringStatus` | Depth/work/byte projection failure keeps original authoring status, empty intents/identities/actions, `BETA_INSPECTION_BOUND`. Shared DAG valid at `check` can still be `InspectionRejected`. |
| Bounded case mismatch | RFC6901 pointer, up to four first diffs, containers summarized (`cli.ts:44-53`). Example `/post/balances/0/amount`. |
| Hand-derived repay template | `starter.ts` comments independent derivation; init `--template repay` + shipped `examples/local/repay` complete effects/post. |
| Compact calls | Formatter keeps `transfer(from: Buyer, to: Seller, ...)` on one line; record commas stay newline-separated. |
| Const hover | Qty hover `Qty<USD>: 1000 atoms`; identity-claim text omitted (`protocol.test.mjs:185-193`). |
| Scenario domain/asset pointers | `BETA_SCENARIO_IDENTITY` with `/domain` or `/asset`; diagnostic message includes pointer; span is the action (documented). |
| `BETA_INIT_EXISTS` | Existing directory → `FormationRejected` / `BETA_INIT_EXISTS`, **no** `qualification` field. |
| Qualification on refusals | expand/simulate/test/init carry `local-stipulation-only`. Authoring check/inspect and CLI formation refusals omit it; GETTING-STARTED v6 states a missing field never means qualification. |
| Inspect depth 64 | `term(..., depth>64)` throws before recursive explosion (`index.ts:19-20`). |
| Bridge local custody | Owner/amount share local domain; only `source`/`destination` may be foreign (`frontend.ts:289-297`, `348-355`). |
| Optional horizon headers/rounds | Provided domain/asset/signer/rounds must agree with operation domain. |
| Caps unit | Declared asset or bridge amount; AMM `net_floor` uses output. Residual hole: L1. |
| Share-class duplicate IDs | `share_class:domain:id` identity key (`frontend.ts:328`). |
| `retained_duties` | Horizon intent: text or `string[]`. Stage: text only. Array on stage → `BETA_TYPE` (this session). S0 still requires empty arrays. |
| Stage `signed_floor` | Text or Qty on the stage domain. |
| Raw BOM prefix | CLI file BOM refused; API strings are already decoded. |
| Unified local coverage | Per-action `financialRelations` / `localPreparation` on check and inspect. |
| VSIX ignore | `.vscodeignore` drops `prepare.mjs`, `*.vsix`, lockfile, `.gitignore`, `.npmignore`. Pack listing includes `server/cli.js` + languageclient; omits those ignore-listed files. |

---

## Parent v5 evidence (code unchanged in v6)

Observed receipts, not re-executed here except focused discriminators:

| Receipt | Observation |
| --- | --- |
| `beta-tests-code-v5.txt` | 79/79 package tests |
| `typecheck-code-v5.txt` | `tsc --noEmit` silent success |
| `regression-final.txt` | 938/938 existing language tests |
| `experiments-typecheck-v5.receipt.json` | `tsc -p experiments/moriarty-language` exit 0; capture file empty because tsc is silent |
| `nvim-final-v5.txt` | headless Neovim stdio smoke passed |
| `vsix-protocol-final-v5.txt` | 13/13 protocol tests |
| `vsix-final-v5.txt` | packaged VSIX listing; `server/cli.js` present |
| `artifact-hashes-v5.json` | v5 tarball/VSIX/cli hashes (v6 GETTING-STARTED would change a **fresh** pack hash) |
| `claude-mcp-activation-v5.json` | native Claude MCP `connected`; `check` + `inspect` on starter source; no global edits |
| Six parent v5 reproductions | 65 original cases + 6 starter `literal fee payment` cases + 4-case S1 negative-control with one expected `TestsFailed` mismatch |

`check` can succeed while Core rejects a scenario (fee over cap, stale head). That is the documented split.

---

## Focused probes this session (no broad re-run)

All passed: SpecifiedOnly `scenarioValidation: NotAppliedUnsupported` even on malformed JSON; human SpecifiedOnly label; JSON `check` key set + JSON-safe spans; stage duties text vs array; intent duties text or array; shipped repay `sourceHash` differs from init template by trailing newline; `BETA_INIT_EXISTS` without `qualification`; origin closure to `domain Preview`; AMM cap hole in L1.

---

## Abstentions

1. **Typed full-language horizon document** is out of this code candidate. Eight family examples are SpecifiedOnly authoring. Proposed stage/evidence/duty grammar is not executable beta syntax.
2. **VS Code GUI activation** and **non-Claude / adapter training / other-provider activation** were not observed. Neovim headless, VSIX pack+stdio protocol, and one native Claude MCP check/inspect session were observed.
3. **Native authentication, signatures, ZK proofs, financial ledger settlement, and K/Quint correspondence** remain open. Four premises and four unverified bindings stay on every `PreparedUnqualified` result. `authenticated: false` on inspect claims.
4. **MCP NDJSON CRLF:** decoder completes a frame on `0x0A` and keeps a preceding `0x0D` in the JSON body. Unix Claude stdio used `\n`. Windows `\r\n` MCP clients are untested.
5. **Peer/prior audits and repair write-ups** were not read.
6. **No model vote, package test, or local Core run** is treated as ledger, proof, or signature evidence.

---

## Scoped conclusion

`code-candidate-v6` is the v5 production implementation plus two documentation files. Hashes match the freeze. Unchanged Source/6 and Core/5 still decide S0 money in Stage→Intent→Effect→Authority→History→Failure order. The beta is honest about `PreparedUnqualified` and `SpecifiedOnly`.

**Accept this freeze as unqualified local authoring/preparation**, with L1 recorded as a SpecifiedOnly nominal-cap tightness item. Do not treat this audit, the 79 tests, the 938 regressions, or the Claude MCP probe as financial acceptance.
