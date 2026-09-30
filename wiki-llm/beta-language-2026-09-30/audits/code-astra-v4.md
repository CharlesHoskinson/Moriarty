# Independent whole-candidate code and result review: v4

Verdict: **changes requested** for one medium defect in bounded inspection. No critical or high finding. This verdict applies only to the authoring/local-preparation candidate below; it does not qualify financial execution or the separate typed horizon.

## Identity, scope and evidence

- Requested review seat: GPT-6 Astra, medium. This fresh host task exposes no independent provider/effort attestation; this report does not invent one.
- Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.
- Candidate: `code-candidate-v4.json`, 62 files, aggregate SHA256 `f4725cd8316876f89b7b54f55dc671ac28aa57e78598eb588fbb9578c87f6b9f`. Every file hash and the prescribed sorted-line aggregate were verified before review; all 62 still matched at the final integrity check.
- Read the repository AGENTS instructions and installed `moriarty-dev:develop` skill; refreshed guarded status. Status reports unresolved historical campaign admission/accounting, with no pending transaction notifications. This read-only review performs no campaign dispatch.
- Reviewed the entire production frontend, bridge, JSON transport, CLI, inspection API, protocol/services, starter, build/distribution configuration, tests, editor and AI assets, examples, README/GETTING-STARTED, CONVERGENCE and IMPLEMENTATION-PLAN. Inspected the reused Source/6 wrapper and Core/5 judgment path. `git diff` against manifest baseline `870f998b36ecda04622fa4274132e74902942d0b` was empty for `experiments/moriarty-language/src/successor`.
- No peer/prior audit reports or repair conclusions were used to establish this finding or verdict.
- Current supplied evidence was inspected: `beta-tests-code-v4.txt` reports 75/75 passing, zero failures; `typecheck-code-v4.txt` records the strict `tsc --noEmit` run without diagnostics; `regression-final.txt` reports 938/938 passing for unchanged language/Core. These are observed recorded results, not a claim that this reviewer reran those suites. One additional focused in-memory probe was run below. No broad repeated tests were needed.

## All severity findings

### M1 — Valid shared constants overflow the inspection call stack before bounded refusal

**Severity:** medium. **Location:** `packages/moriarty-beta/src/index.ts:19`, particularly recursive array projection at line 25 and record/call projection at line 27; exception propagation at line 57.

Repository observation: the parser limits nesting within each declaration, but declaration references preserve a shared value DAG. Therefore a series of individually shallow declarations can compose a much deeper resolved value. `inspect` recursively projects arrays and records. Its 8192-work/524288-byte checks do not bound recursion depth, and its handler only converts `LocalError` to `InspectionRejected`.

Reproduce from the checkout root, without writing any file:

```sh
node --input-type=module <<'JS'
import {check, inspect} from './packages/moriarty-beta/src/index.ts';
let ds = 'const v0 = "claim";';
for (let i = 1; i <= 80; i++)
  ds += `const v${i} = ${'['.repeat(30)}v${i-1}${']'.repeat(30)};`;
const src = `profile "moriarty-beta/1"; agreement Deep {
  policy P = {}; ${ds}
  intent I = { operation: governance.execute(policy: P), retained_effects: v80 };
  action go uses I;
}`;
console.log(check(src).status);
try { console.log(inspect(src).status); }
catch (e) { console.log(e.name, e.message); }
JS
```

Experiment observation: the equivalent compact source is 6239 bytes; `check` returns `AuthoringChecked`, while `inspect` throws `RangeError: Maximum call stack size exceeded`. The formatted reproduction above has the same declaration/value structure. The composed value has 2400 nested arrays although no declaration violates its local parser depth limit.

Impact: an admitted source bypasses the promised structured inspection refusal. The direct API throws; the CLI's generic exception path reports an unstructured command error; MCP maps it to an internal service error instead of a bounded inspection result. This does not produce financial acceptance, mutate state or change Core judgments.

Required repair: bound projection depth before descending, or use an explicitly bounded iterative projection and serialization strategy. Preserve `AuthoringChecked` and return `InspectionRejected` with `BETA_INSPECTION_BOUND`, empty identities/actions/intents, and no partial scope. A conservative depth limit such as 64 must cover arrays and record/call branches and keep serialization safely bounded. Add this deep shared-chain regression alongside the existing branching shared-DAG test, and verify the direct API plus the CLI/MCP result boundary. Catching arbitrary `RangeError` after overflowing the stack is insufficient as the primary resource guard.

## Specification and code assessment

The finite grammar uses exact BigInt arithmetic, per-operation UInt128 checks and S127 assignment checks for S0 fields. Nominal account/asset/domain handling, prior-only references, economic-ID resolution, closed call schemas and unknown-field rejection match the scoped contract. Horizon operations remain recognized structural authoring with `SpecifiedOnly` execution; they do not receive financial relation approval.

The bridge re-analyzes source text and accepts duplicate-safe bounded scenario JSON. Source intent remains fixed. Scenario cells are exact and ordered; arbitrary candidate effects remain proposals through Source/6 formation and the unchanged Core. Overpayment emits the documented unchanged debt placeholder only for automatic construction, without suppressing the original Intent/Effect order. Core owns full ordered effects, post-state, work, allowance, replay and head checks. Rejections retain null published effects/post. Four external premises and four unverified bindings remain visible.

Origin roots distinguish source, scenario, derived and generated information, retain use/definition dependencies and both input digests, and bound the origin graph/output. Scenario messages identify JSON pointers rather than falsely presenting action spans as scenario positions. The inspected current tests cover direct Source/6 comparison, hostile candidate omission, overpay/expiry, receiver overflow, stale/replayed heads, shared origins and bounded graph refusal.

The transfer and repayment expectations are literal fixtures rather than generated evaluator output. Independently checking the repayment arithmetic gives 3000 paid as 1000 accrued plus 2000 principal, leaving principal/outstanding 98000 and accrued zero; payer/creditor become 197000/3000, allowance remaining/spent 197000/3000, work 9/1, and replay/head advance explicitly. The exact ordered six-effect vector agrees with those expectations.

CLI UTF-8 decoding is fatal and preserves BOM visibility for explicit rejection before formatting can write. The test runner bounds manifest JSON, closes case schemas, resolves contained paths, compares exact effects/post, escapes pointer components, and emits bounded first differences. Hostile expected JSON is constrained before comparison by the same duplicate/depth/node/string limits; missing/container values receive summaries rather than recursively dumping them. `InspectionRejected` is separately documented from source rejection, but M1 still defeats that boundary for composed depth.

LSP uses UTF-16 conversion, Full synchronization, increasing versions and bounded current documents. Oversized admitted Full generations invalidate prior results. Hover/definition resolve actual source reference spans, and quantity constants display exact atoms. Formatter handling follows the active delimiter for comma layout, retains comments, reparses its result and has idempotence/financial-expansion checks. MCP exposes only bounded text check/inspect/expand/preview tools, with no paths, shell, signing or sending; framing and stdout buffering have explicit limits.

Build configuration bundles the reused S0 path into standalone CLI/API outputs and emits declaration imports. The current passing distribution test exercises a packed installation outside the checkout, CLI/API use without PATH tools, public declarations, and exact bundled editor-server bytes. Editor configuration is project-local; VSIX packaging is distinct from real activation. AI adapters are inert guidance and make no provider qualification claim. Additive check metadata is documented rather than presented as a permanently closed output schema.

Code quality is generally coherent around shared services and explicit boundary checks. Dense one-line functions increase inspection effort, but no separate severity finding is justified solely by style. The concrete weakness is that parser syntax depth, graph work and serialization/projection depth were treated as interchangeable in inspection; M1 demonstrates why they need separate enforcement.

## Scoped verdict and abstentions

Do not approve frozen v4 until M1 is repaired and the changed bytes receive current verification/review. The passing recorded suites support the exercised behaviors but do not negate this independent counterexample.

Final v4 Neovim/VSIX rechecks were pending at handoff and were not used as current approval evidence. This reviewer did not independently perform real VS Code activation, provider activation or the six requested developer trials, and does not certify their completion from package tests. The separate entire typed horizon is outside this code manifest and remains specified-only. No signature authentication, provider evidence, native proof, financial correspondence, ledger settlement or atomic acceptance is established by this audit.
