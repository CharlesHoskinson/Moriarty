I'll start by reading the required project and skill files, then run guarded status and independently audit the frozen v5 candidate.Using the Moriarty develop skill for a read-only independent audit. Next I’ll take guarded status, then verify the frozen v5 candidate hashes.Guarded status is clean (no pending txs). I’ll verify the v5 hashes, then read the plan, production sources, and the named evidence files.Hashes match the frozen aggregate. Next I’ll read production sources, Core/Source bindings, and the named evidence files.Production sources are in hand. I’ll read Core/Source6, tests, editor assets, and the v5 evidence next, focusing on the named discriminators.Editor evidence files are present now. I’ll read those receipts and run a few focused discriminators on the production path.## Audit identity

**Requested:** `grok-4.6` at high effort, independent read-only whole-candidate review of frozen **code-candidate-v5**.  
**Observable returned identity:** Grok 4.6 (session system identity: “Grok 4.6 released by xAI”).  
**Effort:** this session followed the high-effort whole-candidate instruction. This host provided no separate machine receipt for `reasoning_effort` or a provider completion id. None is fabricated.

**Checkout:** `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`  
**Startup:** `moriarty-dev:develop` loaded; guarded `status --json` only.  
**Status observation:** capability `SP01.6 loan-swap-subset`; `pendingTransactions: []`. No Midnight notification lines. This audit did not dispatch campaigns.

**Candidate freeze (verified):**  
`wiki-llm/beta-language-2026-09-30/audits/code-candidate-v5.json`  
- 62 files, all present, all SHA256 match  
- aggregate SHA256 `3b981e68930f87655256f96f2d364201314a44369fe84193c8d69d3aa1561b80`  
- baseline `870f998b36ecda04622fa4274132e74902942d0b` equals this worktree HEAD  
- `experiments/moriarty-language/src/successor/{mil4-s0-core-v5.ts,mil4-s0-source-v6.ts,financial-agreement-source-v6-frontend.ts}` have empty `git diff` against that baseline

Local tests and this vote **do not** qualify native proof, authenticated provider signatures, cryptographic signatures, or ledger commit. The beta remains **unqualified local preparation**.

---

## Verdict

| Axis | Verdict |
| --- | --- |
| **Spec-compliance (CONVERGENCE v2 + v5 authoring repairs, S0/Core path)** | **Pass with findings.** Local transfer/repay lowering, scenario closure, Core first-failure order, and SpecifiedOnly refusal match the selected contract. One **medium** authoring-contract miss: stage `retained_duties` arrays. |
| **Code quality** | **Pass with findings.** Bounded text APIs, honest unqualified labeling, and production-path tests are coherent. Two **low** public-shape nits. |
| **Publication vs CONVERGENCE “final bounded authoring repairs”** | Repair the medium finding before treating v5 as closed against that paragraph. S0 finance can stay on original Core. |

**Scoped product statement:** this candidate is usable as a standalone authoring beta and as **PreparedUnqualified** local S0 preparation for source-fixed transfer/repay plus untrusted scenario text. Horizon families are structural authoring only. Authentication, native proof, financial correspondence, and atomic ledger acceptance remain open.

---

## Evidence actually observed

| Receipt | Observation |
| --- | --- |
| Hash manifest | 62/62 match; aggregate matches parent expectation |
| `/home/charl/research/moriarty-beta-2026-09-30/beta-tests-code-v5.txt` | `tests 79`, `pass 79`, including packed-install, protocol, inspect/origin, and v5 authoring discriminators |
| `typecheck-code-v5.txt` | `tsc --noEmit` completed with no compiler diagnostics |
| `regression-final.txt` | existing language suite `tests 938` / `pass 938` |
| `nvim-final-v5.txt` | exact smoke line: `Neovim Moriarty stdio LSP smoke passed: initialize/open/definition/symbols/format/change/diagnostics/shutdown` |
| `vsix-final-v5.txt` | `moriarty-beta-0.1.0.vsix`, 326 files, 500.44 KB; tree has extension, grammar, snippets, `server/cli.js`, languageclient deps; `.gitignore` / `.npmignore` / `.vscodeignore` / `prepare.mjs` absent from the VSIX tree |
| `vsix-protocol-final-v5.txt` | protocol file `tests 13` / `pass 13` |
| Focused live probes (this audit) | SpecifiedOnly human label; `scenarioValidation: NotAppliedUnsupported`; stage duty-array reject; CLI `BETA_BOM` / `BETA_UTF8` |

`experiments-typecheck-final.txt` and `experiments-typecheck-final-v5.txt` are empty (0 bytes). Package typecheck is evidenced; experiments-wide `tsc` is not.

---

## Findings

### M1 — Medium — stage `retained_duties` arrays fail the v5 hint contract

**Contract:** CONVERGENCE final repairs (lines 143–144) and GETTING-STARTED.md lines 177–178: retained-duty hints are text **or** arrays of text; stage `signed_floor` is text or a Qty on that stage domain.

**Code:** `packages/moriarty-beta/src/frontend.ts`

- Intent path (line 360) already allows array-of-string or string.
- Stage path folds `retained_duties` into the string-only loop at **lines 336–338**:

```336:338:packages/moriarty-beta/src/frontend.ts
    for (const k of 'strike_units strike unit source finality provenance source_hash duty_preservation scope evidence trigger paired_claim duty status relation failure rounding pending timeout recovery authority completion retained_duties'.split(' ')) if (f[k]) {
      if (k === 'authority' && f[k].tag === 'entity') entity(f[k], 'grant'); else string(f[k]);
    }
```

**Repro (observed this session):**

```text
profile "moriarty-beta/1"; agreement Domains {
  domain H = {id:"Home",chain:"example",network:"test"};
  policy P = {};
  stage S = { domain: H, retained_duties: ["pending"] };
  intent I = { operation: governance.execute(policy: P) };
}
```

- `analyze` → `AuthoringRejected`, `BETA_TYPE`, `Expected string`
- Same program with `retained_duties: "pending"` → `AuthoringChecked`
- Intent-level `retained_duties: ["pending"]` → `AuthoringChecked`

This is **nominal authoring**. It does not move Core caps, effects, or authority.

**Repair:** In `horizonDeclaration`, handle `retained_duties` the same way as horizon intents (array of strings **or** string) **before** the generic string loop, and remove `retained_duties` from that loop. Add a frontend test next to `share identities and duty hints have explicit authoring shapes`. Keep financial truth open.

---

### L1 — Low — CLI `LocalError` JSON omits `qualification`

**Code:** `packages/moriarty-beta/src/cli.ts` **lines 118–120**

Library `expand`/`simulate` failures include `qualification: 'local-stipulation-only'` (`bridge.ts` 11–14, 87). CLI file-boundary refusals do not.

**Repro (observed):** UTF-8 BOM and invalid UTF-8 `mori check FILE --json` return:

```json
{"status":"FormationRejected","diagnostics":[{"code":"BETA_BOM","message":"..."}],"publishedEffects":null,"publishedPost":null}
```

Same shape for `BETA_UTF8`. `published*` null is honest. The v4/v5 refusal field `qualification` is missing on this path (`BETA_INIT_EXISTS` and case-schema errors use the same catch).

**Repair:** Add `qualification: 'local-stipulation-only'` to that `LocalError` JSON object. Keep exit code 1 and refuse `fmt --write` on the original bytes.

---

### L2 — Low — init repay source omits the trailing newline present in the shipped example

**Files:** `src/starter.ts` line 7 (`repaymentSource` length 915) vs `examples/local/repay/repayment.mori` (916 bytes, extra final `\n`). Scenario JSON and `mori.tests.json` match the starter objects.

**Effect:** `sourceHash` differs for otherwise identical programs. Init tests still pass.

**Repair:** End `repaymentSource` (and, for consistency, `starterSource`) with a newline so init bytes match the shipped example.

---

No **high** defect was reproduced on the S0 production path (identity, Qty bounds, duplicate JSON keys, overpay placeholder, ordered effects/post/work/replay, origin closure, parser/format budgets, LSP stale/UTF-16/unavailable generation, MCP/stdio bounds, packed CLI/API, horizon `Unsupported`).

---

## Challenge results (production path)

**Financial identity and Qty bounds.** Economic IDs (`[A-Za-z][A-Za-z0-9_]{0,63}`, Source/6 reserved exclusion) are emitted; declaration names are navigation only. Duplicate domain / in-domain account / in-domain asset IDs reject (`BETA_DUPLICATE_ID`). Qty spelling, whitespace/comment separation, scale padding without rounding, UInt128 expression range, and S127 narrowing on S0 caps/amounts are in `frontend.ts` 106–111, 136–140, 211–219, 377–388. Renaming `Buyer`→`Purchaser` leaves Core identities unchanged (`bridge.test.mjs`).

**Source-fixed vs untrusted scenario.** `expand` always `analyze(source)` then parses scenario text with `parseBoundedJson`. Scenario cannot add intent fields. Domain/asset mismatches use `/domain` and `/asset` (`bridge.ts` 52–54). Obligation mismatch uses `/obligation` (60). Missing `/allowance` pointer observed in tests.

**Duplicate decoded JSON keys / closed schema.** `json.ts` 48–55 rejects decoded duplicates (including `"\\u0068ead"`) before `JSON.parse`. Scenario `record()` rejects unknown and missing keys. `__proto__` is not a legal Moriarty identifier (must start `[A-Za-z]`), so inspect prototype-key injection via source records does not arise.

**Overpay and Core order.** Automatic repay with `n > outstanding` emits unchanged principal/accrued/outstanding plus debit/credit/admin lines (`bridge.ts` 194–201). `proposalNote` is set only when that automatic path runs (`over && !candidate_effects`). Core/5 still judges Stage → Intent → Effect → Authority → History → Failure (`mil4-s0-core-v5.ts` 180–334). Overpay hits `S0_EFFECT_RANGE` at line 281 **before** history. Expiry+overpay hits `S0_INTENT_SCOPE` first. Caps/authority are not pre-checked in the beta. Original Core files are unmodified.

**Ordered effects / post / work / replay.** Transfer: Debit(gross), Credit(value), optional fee Credit, UseAllowance, UseReplay, AdvanceHead. Zero fee keeps three snapshot cells and five effects. Replay in Core is the tuple key; starter expectations use `["Midnight","Owner","n1"]`. Work 10→9 / 0→1. Independent Source/6 oracles in `tests/fixtures.mjs` correspond through `parseAndLowerSource6` and `prepareSource6S0Unqualified`.

**Origin graph.** `definitionOrigin` depends on declaration range and value range; uses depend on definitions; `rangeOrigin` includes contained uses (`bridge.ts` 125–146). Repair tests walk `intent.signed_action` / caps / effects to defining consts, accounts, and assets. Origin exhaustion (`BETA_ORIGIN_BOUND`) publishes no `source6`/`fieldMap`/`origins`. Input digests are SHA256 of the exact strings.

**Parser / formatter totality and budgets.** First diagnostic only; incomplete/trailing input rejected. Source 65536, tokens 8192 including EOF, depth 64, declarations 256, fields 64, strings 1024, identifiers 64, edges 32768. Format is comment/spelling-preserving, delimiter-aware (call commas stay compact; record commas newline), idempotent on valid source, and re-`analyze`s output. Generated Source/6 byte bound 65536; 8192 generated tokens are enforced by unchanged Source/6 `TOKEN_BOUND` and mapped to `FormationRejected`.

**LSP stale-version, UTF-16, unavailable generation.** `positionEncoding: 'utf-16'`. `byteToPosition` counts UTF-16 units and treats CRLF as one break (`frontend.ts` 421–430). Stale `version <= doc.version` is ignored. Oversized Full change sets `unavailable`, empty text, `BETA_RESOURCE`, and does not keep the previous successful analysis (`servers.ts` 81–89). Protocol tests cover CR/LF/CRLF navigation and emoji diagnostic columns.

**Readonly MCP / stdio bounds.** Tools: `check`, `inspect`, `expand`, `preview`; closed text schemas; `readOnlyHint: true`; no path/shell/AST. Header 8192, body 262144 (pre-allocation), response 524288, 32 docs, aggregate 1 MiB. Framing violations close the process. One synchronous dispatch per frame.

**Packed CLI / API / IDE server.** `distribution.test.mjs` packs, installs with `PATH` empty, runs init/check/fmt/inspect/expand/simulate/test, imports the library, typechecks a consumer against packed `.d.ts`, and asserts `editor/vscode/server/cli.js` bytes equal `dist/cli.js`. `pack-final-v5.txt` shows GETTING-STARTED.md in the tarball (54 files).

**Horizon unsupported.** SpecifiedOnly actions return `Unsupported`, `BETA_PROFILE_UNSUPPORTED`, `publishedEffects/Post: null`, `scenarioValidation: NotAppliedUnsupported` **before** scenario schema (`bridge.ts` 87–92). Live probe: invalid JSON scenario still gets `NotAppliedUnsupported`. Eight family examples are AuthoringChecked and execution-unsupported (`examples.test.mjs`). Typed full-language horizon grammar is **not** implemented (separate document).

**Inspect signed terms and S0 scope.** `inspect` projects per-intent terms (caps, `valid`, operation args, economic claims, `authenticated: false`) without expanding declaration DAGs (`index.ts` 10–18, 36–40). Depth 64 / work 8192 / 524288 bytes → `InspectionRejected` + `authoringStatus`.  
Conditional coverage (live tests + code 46–54):

| Program | `coverage.financialRelations` | S0 premises/bindings | `s0SignedFieldMax` |
| --- | --- | --- | --- |
| Pure SpecifiedOnly | `Open` | empty | absent |
| Actionless (intent present, no action) | `Open` | empty | absent |
| Mixed | `ConditionalOnLocalS0Preparation` | present; `localS0Actions` names only LocalS0 | present; per-action horizon rows omit S0 bounds |

**Public JSON-safe spans.** `Span` is UTF-8 `{start,end}`. `check --json` keeps byte spans and strips declaration values. Human `check` (no `--json`) prints 1-based `line:column` via `byteToPosition` and, for SpecifiedOnly, ` (execution unsupported; financial relations open)` (observed on `examples/amm.mori`). GETTING-STARTED documents additive `references`/`fieldUses`.

**CR/LF/CRLF.** Lexer terminates `//` on CR or LF; tests cover all three EOLs for parse, format, LSP positions, and comment completion.

**CLI UTF-8/BOM.** File read refuses BOM then fatal UTF-8 (`cli.ts` 19). `fmt --write` does not rewrite those bytes (cli tests). Library APIs take already-decoded text; U+FEFF in a string is `BETA_CHARACTER`.

**Other v4/v5 items present in production.** `BETA_INIT_EXISTS`; bounded case mismatches (`/post/balances/0/amount`); hand-derived repay template (3000 → accrued 1000 then principal 2000, post principal 98000); const hover `Qty<USD>: 1000 atoms`; bridge local custody/amount with only `source`/`destination` foreign (`frontend.ts` 288–297, 349–359); cap units from declared asset or inferred bridge amount; AMM `net_floor` uses output asset (365–367); share-class IDs unique per domain (328); stage `signed_floor` text or local Qty (329).

---

## Code-quality notes (non-blocking)

The architecture matches the plan: one frontend, text-only public APIs, bridge always re-analyzes, Core/Source6 imported and unchanged, stdio servers call the same library. Budgets are enforced at lexer, parser, JSON, origin, inspect, protocol, and LSP admission. Unqualified premises/bindings appear on successful local preparation. AI adapters are inert project files. Snippets/grammar are lexical.

`frontend.ts` (456 lines) is dense; behavior is covered by discriminators rather than comments. `skipLibCheck` is on; package `tsc` includes `src/**/*.ts` only.

---

## Abstentions

- **VS Code GUI activation and any provider/plugin activation** — unperformed; VSIX/nvim receipts are stdio/package observations only.
- **Native proof, authenticated provider, cryptographic signature, ledger commit, K/Quint correspondence** — open; tests and this audit do not close them.
- **Typed full-language horizon grammar** — excluded from this code candidate; eight family files are SpecifiedOnly sketches.
- **Experiments-wide typecheck** — v5 receipts are empty; only package `tsc --noEmit` was observed.
- **Exact nvim/VSIX command lines and binary digests** — `nvim-final-v5.txt` is the smoke print; `vsix-protocol-final-v5.txt` is the node:test summary. This audit did not re-run Neovim or vsce.
- **TextMate `$` vs CR-only highlighting** — parser/LSP handle CR; grammar highlighting under CR-only files was not visually verified.
- **Peer audits / repair write-ups** — unread, as instructed.

---

## Qualification boundary (required discrimination)

Static checks in this package are **nominal authoring** (names, Qty types, closed schemas, UTF-8 spans) plus **unqualified local Core/5 evaluation** of representable Source/6.  
`PreparedUnqualified` still lists four external premises (canonical intent signature, snapshot-to-head, head extension, atomic ledger compare-and-consume) and four unverified bindings (agreement-id, selected-program, asset-scale, authenticated-predecessor).  
Horizon header/cap/share-class/duty-hint checks do **not** establish conservation, authority, evidence, or a valid financial floor.
