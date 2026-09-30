I'll start with the required startup docs and a guarded status check, then independently audit the frozen v4 candidate against the production package.Using `moriarty-dev:develop` for a read-only result audit. Next I’ll take guarded status, then verify the frozen v4 manifest hashes before inspecting the production package.Status is clear of pending transactions. I’ll verify the frozen v4 hashes, then read the production package and plan independently.Hashes match the frozen aggregate. Next I’ll read the production sources, Core/Source6 path, and v4 evidence independently.The test receipt is 75/75. I’ll now inspect the production frontend, bridge, CLI, and servers rather than relying on that receipt.I’ve got a likely BOM-decoder mismatch and a few refusal-shape questions. Next I’ll confirm those against tests, Core order, and the editor/I’ll inspect the BOM decoder, origin closure, inspect contract, and editor/evidence next — those are the highest-risk v4 claims.I’ll run focused discriminators on BOM handling, IDE/CLI byte identity, and successor immutability, then finish the remaining production files.# Moriarty beta code candidate v4 — independent result audit

**Requested identity/effort:** `grok-4.6`, high effort.  
**Observable identity:** this session is Grok 4.6 (xAI). The host returned no separate signed provider/effort receipt, and none is fabricated here.  
**Role:** read-only result auditor. No file edits, commits, config changes, campaign dispatch, wallet/credential access, subagents, or peer/prior audit reads.

**Startup:** `moriarty-dev:develop` loaded. Guarded `status --json` from `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`:

- capability `SP01.6 loan-swap-subset`
- `pendingTransactions: []`
- blocked on operational-history / stale SP01 admission inputs / missing current-accounting  
This review is independent of that blocked campaign.

---

## Verdict

**Spec-compliance: PASS** for CONVERGENCE v2 / IMPLEMENTATION-PLAN bounded beta authoring and **unqualified local S0 preparation**. The frozen 62-file candidate keeps Source/6 and Core/5 first-failure order, economic IDs, exact Qty/S127/UInt128 bounds, closed duplicate-safe scenario JSON, total overpay proposal with Core deciding rejection, origin closure to transitive definitions, inspect signed terms, and explicit `Unsupported` for horizon execution.

**Code-quality: PASS with Low findings.** Production path is coherent (library, CLI, packed CLI bytes, VSIX `server/cli.js`, stdio LSP/MCP). Four Low issues below do not overturn the spec pass.

This candidate is local preparation only. Authentication, native proof, financial correspondence, and ledger commit remain open. Package tests and model votes do not qualify those gates.

---

## Candidate freeze

| Item | Value |
| --- | --- |
| Manifest | `wiki-llm/beta-language-2026-09-30/audits/code-candidate-v4.json` |
| Schema / version | `moriarty-beta-code-candidate/1` / `v4` |
| Baseline | `870f998b36ecda04622fa4274132e74902942d0b` (matches `HEAD`) |
| Files | 62; all hashes match; **0 mismatches, 0 missing** |
| Aggregate SHA-256 | `f4725cd8316876f89b7b54f55dc671ac28aa57e78598eb588fbb9578c87f6b9f` |
| Successor | `experiments/moriarty-language/src/successor` git diff empty vs baseline |

Current built CLI bytes (not in the 62-file freeze) match each other: `packages/moriarty-beta/dist/cli.js` = `editor/vscode/server/cli.js` = VSIX `extension/server/cli.js`, SHA-256 `31990e8d96d862bfbbd59f9c780f2f3006b684ac56a9ad1b49048c73334457b9`.

---

## Challenge results (production path)

Independent reading of `packages/moriarty-beta/src/{frontend,bridge,json,index,cli,servers,protocol,starter}.ts` plus unchanged Core/Source6, with focused discriminators on BOM/inspect/identity. Recorded receipts were treated as command logs, not ledger evidence.

### Financial identity and Qty bounds

Economic IDs are field `id` values with Source/6 alphabet and reserved-word exclusion (`frontend.ts` `transport()`, `identity()`). Declaration names are navigation only. Duplicate domain / in-domain account / in-domain asset IDs reject, including scale/representation conflicts.

Expression values are checked at every `+ - *` against `0..2^128-1`. S0 caps, fees, floors, transfer value/fee, repay amount use S127 (`qty(..., true)`). Horizon amounts stay UInt128. Frontend accepts zero amounts; Core Intent rejects them. Caps, stale head, window, and balance sufficiency stay in Core.

Renaming `Buyer` → `Purchaser` leaves Core identities (`Owner` / `A` / `Midnight`) unchanged (`bridge.test.mjs`).

### Source-fixed vs untrusted scenario JSON

Public APIs take source/scenario **text**. `expand`/`simulate` re-analyze source, then parse scenario with `parseBoundedJson`. No AST/object scenario getter path. `candidate_effects` is a hostile proposal override; signed intent fields still come from source. Scenario `domain`/`asset` must equal the selected action’s economic IDs; mismatch is `BETA_SCENARIO_IDENTITY` with pointers `/domain` and `/asset` (discriminator confirmed).

### Duplicate decoded JSON keys / closed schema

`json.ts` walks objects, JSON-decodes each key, and rejects duplicates **before** `JSON.parse`. Escaped `"\u0068ead"` vs `"head"` is `BETA_JSON_DUPLICATE`. Unknown/missing scenario fields, extra balance cells, and open records reject as `FormationRejected`. Integers are canonical decimal strings; S0 obligation fields are S127.

### Overpay proposal and Core order

Automatic repay with amount > outstanding emits **unchanged** principal/accrued/outstanding plus debit/credit (`bridge.ts` 194–201). `proposalNote` is set only for that automatic path. Core still runs Stage → Intent → Effect → Authority → History → Failure:

| Case | First Core judgment |
| --- | --- |
| Overpay | Effect `S0_EFFECT_RANGE` |
| Overpay + expired round | Intent `S0_INTENT_SCOPE` |
| Overpay + stale head | Effect (before History) |
| Missing fee credit + stale | Effect `S0_EFFECT_MISMATCH` |
| Receiver UInt128 overflow | Effect |
| Low allowance / zero work | Authority |
| Stale / consumed replay | History |

Caps and authority are not moved earlier than Core’s first-failure order. Rejection publishes `publishedEffects: null` / `publishedPost: null`.

### Ordered effects, post, work, replay

Transfer fixture: Debit 1010, Credit 1000, Credit 10, UseAllowance 1010, UseReplay `["Midnight","Owner","n1"]`, AdvanceHead `h0→h1`; balances 8990/1000/10; work 10→9 / 0→1; replay consumed. Zero fee keeps three snapshot cells and omits the fee Credit. Repay 3000 on 100000+1000: accrued then principal, post principal/outstanding 98000, payer 197000. Starter, `init --template repay`, and `examples/local/repay` carry the same hand-derived complete expect vectors.

### Origin graph / input digests

Every generated line has origins. Source uses memoize by span; definition origins depend on declaration span **and** value span, so reference closure reaches nested consts (`original * 2`, 180-deep sharing stays linear). Origin exhaustion (`BETA_ORIGIN_BOUND`) returns `FormationRejected` with no `source6`/`origins`/`fieldMap`. SHA-256 digests are of the actual input strings.

### Parser / formatter / budgets

Lexer/parser enforce CONVERGENCE budgets (64KiB, 8192 tokens incl. EOF, depth 64, 256 decls, 64 fields, 1024-byte strings, 64-char ids, 32768 edges). Generated Source/6 is byte-capped in the bridge and token/byte-capped again by unchanged `parseAndLowerSource6`. Format rejects invalid source, preserves comments/spelling, keeps named calls compact via delimiter stack, re-analyzes, and is idempotent. Format emits `\n` (CR/CRLF sources change bytes/hashes; that is documented).

### LSP stale-version and UTF-16

`positionEncoding: utf-16`. `byteToPosition` counts UTF-16 units and treats CRLF as one break. Full sync only; stale `version <= doc.version` is ignored. Over-budget Full change stores `unavailable`, publishes `BETA_RESOURCE`, and drops prior analysis. Duplicate URI open is refused. 32 docs and 1MiB aggregate are enforced. Hover/definition ignore record keys; const Qty hover reports atoms (`Qty<USD>: 1000 atoms`).

### Readonly MCP / stdio bounds

MCP tools are `check`/`inspect`/`expand`/`preview`, closed string args, `readOnlyHint`, no paths/shell/signing. Header 8192 / body 262144 / response 524288; admission before body allocation; framing violations close stdio. One synchronous dispatch per frame.

### Packed CLI / API / IDE server

`distribution.test.mjs` packs, installs outside the checkout, `PATH:''`, runs CLI + `import {check,simulate}` + declaration types. Current `dist/cli.js`, vscode `server/cli.js`, and VSIX server bytes are identical. Extension launches `node server/cli.js lsp`.

### Horizon unsupported statuses

Eight family examples `AuthoringChecked` + `SpecifiedOnly`; `expand`/`simulate` return `Unsupported` / `BETA_PROFILE_UNSUPPORTED` / null published effects. Typed full-language horizon grammar is **not** in this candidate.

### Inspect signed terms and S0 scope

Inspect walks each intent’s fields through `term()` (evaluated signed quantities, shallow entity claims, `authenticated: false`). Declaration DAG is not cloned.

| Program | Top-level financialRelations | S0 premises / `s0SignedFieldMax` | Intents |
| --- | --- | --- | --- |
| LocalS0 action | `ConditionalOnLocalS0Preparation` | present, scoped `LocalS0ActionsOnly` | full terms |
| SpecifiedOnly only | `Open` | empty / absent | full terms |
| Mixed | conditional; only LocalS0 actions carry S0 bounds | present | both intents |
| Actionless (intent, no action) | `Open` | empty / absent | full terms still exposed |
| Bound blow-up | `InspectionRejected` + `authoringStatus` | empty identities/actions/intents | none |

### JSON-safe span metadata

`check()` is JSON-stringifiable, strips expanded values, keeps UTF-8 `{start,end}` on diagnostics, `agreementSpan`, `references`, `fieldUses`. LSP converts those bytes through `byteToPosition`. Additive keys are documented as non-closed.

### Human `check` vs `--json`

Default stdout is `AuthoringChecked` / diagnostics at 1-based `line:character` / `name: LocalS0|SpecifiedOnly` / LocalS0 reminder. `--json` is the machine object. Invalid source is non-JSON with a `BETA_` code.

### CR / LF / CRLF

Parser, `byteToPosition`, and LSP `offset()` treat all three. Line comments stop at CR or LF. Protocol/LSP tests cover navigation and comment completion for each EOL. Formatter rewrites to LF.

### Overflow generation / automatic `proposalNote`

Unrepresentable scenario counters → `BETA_SCENARIO_ACCOUNTING`. Automatic overpay does not compute negative principal. Receiver overflow stays Core Effect. Generated/serialized bound failure publishes no artifact. `proposalNote` is null when `candidate_effects` is supplied, even if those effects settle the loan.

### v4 CLI/authoring items

Focused discriminator on Node v24.21.0: invalid UTF-8 → `BETA_UTF8`; UTF-8 BOM → `BETA_BOM`; `fmt --write` leaves bytes unchanged. `BETA_INIT_EXISTS` on existing init dir. Case mismatch reports RFC6901 pointer plus expected/actual (`/post/balances/0/amount`, `8991` vs `8990`). GETTING-STARTED documents scenario schema, repay example, UTF-8/BOM, inspect refusal, and unqualified premises.

---

## Findings

### L1 — Low — BOM check depends on Node’s `ignoreBOM` emitting U+FEFF

**Where:** `packages/moriarty-beta/src/cli.ts:19`

**Behavior:** `new TextDecoder('utf-8', {fatal:true, ignoreBOM:true})` then `text.startsWith('\uFEFF')`. On Node v24.21.0, `ignoreBOM:true` **leaves** U+FEFF (charCode 65279), so `BETA_BOM` fires and `fmt --write` is skipped. WHATWG/MDN describe `ignoreBOM:true` as omitting the BOM.

**Repro:**

```js
const b = Buffer.concat([Buffer.from([0xef,0xbb,0xbf]), Buffer.from(validSource)]);
writeFileSync(file, b);
spawnSync(process.execPath, ['packages/moriarty-beta/src/cli.ts','check',file,'--json']);
// status 1, code BETA_BOM; file bytes unchanged
```

**Repair:** Test the raw prefix `EF BB BF` (and invalid UTF-8 via `fatal:true`) on the `Buffer` **before** decode. Do not rely on `ignoreBOM`.

### L2 — Low — some refusal objects omit `qualification`

**Where:** `packages/moriarty-beta/src/index.ts:58` (`InspectionRejected`); `packages/moriarty-beta/src/cli.ts:120` (CLI `LocalError` → `FormationRejected` for UTF-8/BOM/init/cases)

**Behavior:** `expand`/`simulate` failures, `Initialized`, and `TestsPassed`/`TestsFailed` include `qualification: 'local-stipulation-only'`. `InspectionRejected` and CLI encoding/init `FormationRejected` do not.

**Repro:** inspect the shared-array bound fixture; keys are `status, authoringStatus, sourceHash, diagnostics, agreement, identities, actions, intents`.

**Repair:** add `qualification: 'local-stipulation-only'` on those refusal objects.

### L3 — Low — check vs inspect coverage labels differ

**Where:** `packages/moriarty-beta/src/frontend.ts:401` (`DelegatedToCore`); `packages/moriarty-beta/src/index.ts:47` (`DelegatedToCoreDuringLocalPreparation`)

**Repair:** use one string on both surfaces.

### L4 — Low — VSIX ships extra non-runtime files

**Where:** packaged `moriarty-beta-0.1.0.vsix` (328 files, includes `extension/.gitignore`); `.vscodeignore` does not drop it.

**Repair:** ignore `.gitignore` (and similar) in `.vscodeignore`. Does not change stdio behavior.

No High or Medium defects on the frozen candidate.

---

## Abstentions

- **VS Code GUI / provider activation:** unperformed. Evidence is VSIX package listing, stdio protocol tests, and Neovim headless smoke only.
- **Typed full-language horizon document:** excluded from this code candidate; SpecifiedOnly catalog is authoring structure only.
- **Native proof, authenticated provider, cryptographic signature, ledger commit, K/Quint correspondence:** open. 75 package tests, 938 existing-language tests, and this vote do not close them.
- **`experiments-typecheck-final.txt`:** empty in the evidence dir; package `tsc --noEmit` receipt `typecheck-code-v4.txt` is present. Experiments-wide typecheck not re-established here.
- **Peer/prior audits and repair writeups:** unread by instruction.
- **Full 75 / 938 / pack / nvim / vsce re-execution:** not repeated. Focused discriminators plus source/Core reading were used; recorded receipts are cited as command logs.

---

## Recorded evidence cited (not re-run)

| Receipt | Observation |
| --- | --- |
| `beta-tests-code-v4.txt` | 75/75 package tests |
| `typecheck-code-v4.txt` | `tsc --noEmit` on the package |
| `regression-final.txt` | 938/938 existing language tests |
| `nvim-final-v4.txt` | Neovim stdio smoke: initialize/open/definition/symbols/format/change/diagnostics/shutdown |
| `vsix-final-v4.txt` | vsce package 328 files, 500.17 KB |
| `vsix-protocol-final-v4.txt` | 13/13 protocol tests |

---

## Scoped conclusion

Frozen candidate v4 aggregate `f4725cd8316876f89b7b54f55dc671ac28aa57e78598eb588fbb9578c87f6b9f` is **spec-compliant** for unqualified local beta authoring/preparation, with **four Low** quality/label/portability issues. Source/6 and Core/5 bytes are unchanged. Horizon execution stays `Unsupported`. Local tests and this audit do not qualify Preview settlement, signatures, or native proofs.
