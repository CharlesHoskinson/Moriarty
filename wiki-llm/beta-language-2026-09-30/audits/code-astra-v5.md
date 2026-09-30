# Independent actual-code and result audit: beta v5

Requested reviewer: GPT-6 Astra, medium effort, fresh independent host-routed seat. Observed identity: this delegated Codex session; no independent provider identity or effort attestation is available. This report does not substitute for the separately selected provider audit.

Candidate: `code-candidate-v5.json`, 62 files, aggregate SHA256 `3b981e68930f87655256f96f2d364201314a44369fe84193c8d69d3aa1561b80`. I recomputed every listed file hash and the sorted aggregate; all matched. Baseline `870f998b36ecda04622fa4274132e74902942d0b`.

I loaded the development skill and AGENTS instructions and ran status before inspection. Status retains the operational-history/resource stops and reports no financial transaction evidence. This read-only review does not clear those stops. No peer code audit, prior code audit, or repair conclusion was consulted.

## All severity findings

**Low / P3 — retained-duty hint documentation leaves the stage/intent distinction ambiguous.** `packages/moriarty-beta/src/frontend.ts:336` includes `retained_duties` in the generic text-only declaration-field loop; line337 calls `string` for the stage field. The intent-specific implementation at line360 accepts strings and string arrays. `packages/moriarty-beta/GETTING-STARTED.md:177` and `wiki-llm/beta-language-2026-09-30/CONVERGENCE.md:143` promise text or arrays of text without restricting that promise to intents.

Reproduce from the checkout with the public source API:

```js
import { check } from './packages/moriarty-beta/src/index.ts';
console.log(check(`profile "moriarty-beta/1";
agreement Demo {
 domain H = { id: "Home", chain: "example", network: "test" };
 stage S = { domain: H, retained_duties: ["pending"] };
}`));
```

Observed: `AuthoringRejected`, `BETA_TYPE`, `Expected string`. Changing the array to `"pending"` yields `AuthoringChecked`. This is an authoring compatibility/documentation defect, with no financial execution exposure: stage hints are unqualified data and do not execute. The parent subsequently clarified the intended scope: arrays apply to intent hints; existing stage retained duties remain text-only. Repair the two published contracts to state this distinction explicitly. No new stage-array feature is needed. The v5 bytes audited here still contain the ambiguity.

No critical, high, or medium defect was identified in this review. The low finding above is the complete finding set; evidence and exclusions below are scope limits, not additional severity findings.

## Inspected scope and observations

I inspected all nine production source files, all nine test/fixture files, build and package/type configuration, both lockfiles, editor extension/server preparation/grammar/snippet/Neovim assets, AI adapter and example files, README and GETTING-STARTED, all eight horizon examples and the local repayment source/scenario/full expectation fixture, and CONVERGENCE/IMPLEMENTATION-PLAN. Package metadata and lockfiles have pinned matching tool/client versions. I also read the complete unchanged `financial-agreement-source-v6-frontend.ts`, `mil4-s0-source-v6.ts`, and `mil4-s0-core-v5.ts`. Their diff against the stated baseline is empty.

The following are repository observations, supplemented by the specific execution evidence below:

- Arithmetic evaluates canonical decimal strings with BigInt, nominal quantity assets and intermediate UInt128 checks. Local S0 monetary assignments narrow to S127; the gross sum remains an unsigned proposal. Domain/account/asset/obligation identity aliases reject, and source declaration renaming does not alter economic IDs. Share-class aliases also reject within a domain.
- Closed S0 intent terms are fixed in source. Scenarios accept text only and reject duplicate decoded keys, missing/unknown fields, incorrect cell ordering and accounting/identity violations. Neither scenario claims nor computed hashes authenticate data. Hostile effect overrides remain proposals subjected to Source/6 shape and Core ordered comparison.
- The actual simulation caller expands and parses Source/6, then invokes the unchanged S0 wrapper. Core keeps Stage → Intent → Effect → Authority → History → Failure order. Valid economic requests publish complete ordered debit/credit/debt/allowance/replay/head effects and exact post-state; rejections retain null published outputs. Work changes by one only in the prepared candidate. Automatic overpayment emits the documented unchanged debt placeholder and lets Core decide the first failure; explicit overrides suppress that automatic-proposal note.
- Generated Source/6 origins distinguish source byte spans, scenario pointers, derived rules and generated rules. Reference-use and definition dependencies are memoized; actual input digests are separate from signed source hash/policy claims. Output/edge/origin limits refuse partial expansions.
- The inspection projection has an effective depth guard in addition to work/output limits. I independently generated 80 prior declarations, each adding 30 nested arrays around its predecessor, and placed the last value in a horizon intent. Source remains `AuthoringChecked`; inspection returns `InspectionRejected`, original `authoringStatus: AuthoringChecked`, `BETA_INSPECTION_BOUND`, and empty identities/actions/intents. No stack overflow or partial signed scope was observed.
- Bridge owner and amount must share their local domain; only named foreign source/destination roles bypass the local-domain comparison. Supplied horizon domain/asset/signer and rounds are compared with local operation domains. Cap nominal units use explicit asset or inferred bridge amount, while AMM swap net floor uses output asset. These checks are authoring checks, not conservation/authority proofs.
- Inspection reports action-specific local S0 premises and leaves horizon financial relations open. Empty/partial policies and prose duty hints do not become authority. The separately proposed typed horizon is not accepted as beta grammar. An independent invalid-scenario probe returns `Unsupported`, `scenarioValidation: NotAppliedUnsupported`, null effects/post, and an honest nonvalidation diagnostic.
- CLI files are decoded with fatal UTF-8 and explicit BOM rejection before formatting writes. Formatting rechecks bounded replacement text; invalid source receives no edit. Case runner paths are contained after realpath resolution; complete effects/post assertions compare ordered vectors and exact records.
- LSP and MCP use the same library. Stdio input is bounded before body allocation; output and retained document bounds are explicit. LSP tracks monotonic document versions, removes results for over-budget new Full generations, and converts byte spans to UTF-16, including CR/LF/CRLF. MCP exposes only four closed text tools with no path, shell, signing, proving or sending operation. Client activation claims stay separate from protocol tests.
- Packaging bundles the unchanged financial caller path and emits public declarations. The distribution test installs a real tarball outside the checkout, runs CLI/API with empty PATH, typechecks an importing client, and checks the bundled editor CLI is byte-identical to the installed CLI. This verifies the tested artifact route, not registry publication or real VS Code activation.

## Actual result evidence and limits

I read `/home/charl/research/moriarty-beta-2026-09-30/beta-tests-code-v5.txt`: **79 tests, 79 pass, 0 fail**, including the real outside-checkout packed distribution test and real stdio subprocess tests. The complete test implementations were inspected rather than treating the pass count as financial evidence. `typecheck-code-v5.txt` records `tsc --noEmit` with no diagnostics. The recorded unchanged-core suite in `regression-final.txt` has **938 tests, 938 pass, 0 fail**. I verified the three inspected Source/6/Core/5 files are unchanged; I did not rerun the broad historical suite.

Focused probes in this audit independently reproduced the stage-hint mismatch, effective deep-DAG bounded refusal, and Unsupported/nonvalidated invalid scenario behavior. They changed no source, configuration or fixtures. No broad optional testing, wallet access, network proof run or financial transaction was performed.

The six exact developer seats are a distinct user deliverable, with separate `developer-trials/S1`, `S2`, `S3`, `G1`, `G2`, `G3` artifact directories. I did not use their severity feedback as peer approval or read their report conclusions. Final packed trial reruns, final Neovim and VSIX rerun reconciliation were still assigned to the parent at this audit boundary; this report does not assert their completion. The passing v5 distribution test does establish its own packed CLI/API/type-import and editor-server consistency checks.

## Scoped verdict

The inspected v5 code and recorded automated results support the bounded authoring and unqualified local S0 behavior described above, with one low-severity stage/intent hint documentation ambiguity to resolve. There is no approval beyond this tested scope. In particular this report does not establish real VS Code/provider activation, completion of all six final developer trial reruns, acceptance of the proposed typed horizon grammar, signatures, authenticated snapshots, native proofs, formal K/Quint correspondence, or atomic financial ledger acceptance. The four external S0 premises, four unverified bindings, existing financial gates and first-failure semantics remain unchanged.
