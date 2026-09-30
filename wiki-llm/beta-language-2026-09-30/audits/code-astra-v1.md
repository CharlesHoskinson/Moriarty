# Independent beta implementation audit — Astra seat, v1

2026-09-30. **Specification compliance: changes required. Implementation quality: changes required.** Three medium findings reproduced; no high finding established. This is an independent implementation/result review, not a PL design vote or financial acceptance receipt.

Requested reviewer: `gpt-6-astra`, medium. The parent supplied that routing and the host assigned this fresh agent task. This agent has no separate returned provider/model/effort attestation, so requested routing must not be treated as a verified returned identity. No peer implementation audit was read. No implementation files were edited and no commit was made.

## Candidate and inspected scope

Candidate manifest: `audits/code-candidate-v1.json`, declared aggregate SHA-256 `fa4f67cd6860449f5c1f304d6f649944629f770113030a6e2d9f95cc6590db6c`; baseline `870f998b36ecda04622fa4274132e74902942d0b`. Recomputed every one of its 57 file digests: zero mismatches. This verifies the individual manifest entries; the aggregate digest was not independently reconstructed.

Loaded the Moriarty development skill, repository AGENTS and guarded status. Status had no pending transactions; existing campaign admission is blocked by stale inputs/missing operational evidence. This read-only audit does not resolve those stops.

Reviewed the complete beta frontend, JSON parser, bridge, inspection API, CLI/starter, transports, LSP/MCP handlers, tests and independent fixtures, package/build declarations, editor integration/assets, examples, and inert AI instructions. Inspected package lock metadata. Read BETA-DESIGN, CONVERGENCE and IMPLEMENTATION-PLAN. Read the actual Source/6 parser/lowering, Source/6 wrapper and Core/5 implementation; a baseline diff for those three files was empty. The separate typed full-language horizon under concurrent finalization is outside this audit.

Evidence inspected: `/home/charl/research/moriarty-beta-2026-09-30/beta-tests-code-candidate.txt` reports 54/54 passing, including clean packed installation and actual stdio subprocess tests; `regression-preaudit.txt` reports 938/938 passing. These are supplied run records, not suites rerun by this reviewer. Focused probes below were executed by this reviewer against source modules with Node. Typecheck success was reported in the handoff, but its separate raw receipt was not inspected. No independent Neovim, VS Code activation, live provider, proof or ledger execution was performed.

## M1 — Automatic proposal applies Core opaque-string rules before Core

**Severity: medium.** `packages/moriarty-beta/src/bridge.ts:24`, consumed at `:191` and `:192`.

The source frontend accepts a JSON-escaped control character in nonce/pre-head. Source/6 also represents that string. But the automatic candidate effects pass their strings through the scenario `str()` helper, which rejects control characters. Consequently an accepted beta input fails formation with a scenario diagnostic although the corresponding representable Source/6 input reaches Core's Intent judgment. This violates CONVERGENCE's specified formation/Core boundary and first-failure comparison domain. It also points to a nonexistent candidate-effects JSON field when the scenario did not supply candidate effects.

Reproduced from repository root:

```js
import {transferSource, transferScenario, explicitTransfer}
  from './packages/moriarty-beta/tests/fixtures.mjs';
import {simulate} from './packages/moriarty-beta/src/bridge.ts';
import {prepareSource6S0Unqualified}
  from './experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts';
const beta = transferSource.replace('nonce: "n1"', 'nonce: "n\\n"');
const old = explicitTransfer.replaceAll('"n1"', '"n\\n"');
console.log(simulate(beta, 'pay', JSON.stringify(transferScenario)));
console.log(prepareSource6S0Unqualified(old));
```

Observed: beta `FormationRejected / BETA_SCENARIO_STRING`, message pointer `/candidate_effects/4`; direct Source/6 `CoreRejected / intent / S0_INTENT_SCOPE`, with null published effects/post and diagnosticWork 1. `analyze(beta)` separately returned `AuthoringChecked`.

Repair: distinguish transport representability from Core opaque-string acceptance and preserve representable strings through proposal emission. Test nonce, pre-head and corresponding scenario head/effect strings, including competing earlier failures. Merely moving this restriction to authoring would narrow the agreed comparison domain and needs an explicit contract disposition.

## M2 — inspect does not expose the signed request or operative bounds

**Severity: medium.** `packages/moriarty-beta/src/index.ts:16` and `:18`.

`inspect(starterSource)` returns economic identity claims and action names/support, but no operation arguments, signer binding, validity window, nonce/head, source/policy claims, or actual gross cap/fee cap/net floor. Its `bounds` contains only the two global integer maxima. The README advertises operative bounds; the implementation interface and MCP description promise signed scope. CLI and MCP both route to this same incomplete implementation.

Reproduced by importing `inspect` from `src/index.ts` and `starterSource` from `src/starter.ts`, then printing `inspect(starterSource)`. The payment's gross cap 1010 atoms, fee cap 10 atoms, net floor 1000 atoms and validity 0..10 are absent. Actions contain only name, intent declaration name, spans and `LocalS0` support.

Repair: return a bounded per-action signed-scope summary with resolved nominal/economic references and exact values, while retaining unverified labels. Include tests for transfer and repay and exercise the summary through actual CLI/MCP paths. Avoid serializing an expanded declaration DAG.

## M3 — Origin dependency closure loses referenced definitions

**Severity: medium.** `packages/moriarty-beta/src/bridge.ts:120`–`:127`.

The `resolved-reference-use-and-definition` rule records the supplied value span and matching reference use spans, but never follows those references to their defining value/declaration spans. Operation origins therefore stop at names inside the operation; derived arithmetic can lose the literals that determine emitted money. This fails the agreed preservation of bounded derived dependencies. Source hashing preserves input identity but does not supply the missing field-to-definition mapping.

Reproduced by replacing the starter price declaration with:

```mori
const original: Qty<USD> = 5.00 USD;
const price: Qty<USD> = original * 2;
```

Expand the payment and recursively follow `dependencies` from the `intent.signed_action` field-map origins. Its source spans are exactly the operation call and the uses `Buyer`, `Seller`, `Treasury`, `price`, `fee`; neither price's expression nor original's defining literal is reachable. Following `intent.net_floor` yields `original * 2`, `original`, and repeated `price` uses, but no `5.00 USD` definition. These outputs were printed in the focused probe.

Repair: memoize bounded origin nodes for declaration/value definitions and connect use sites to those definitions recursively without expanding the value DAG. Assert that operation, cap/floor and automatic effect roots reach the relevant original numeric literals and identity declarations. Retain the existing origin limit and atomic failure on budget exhaustion.

## Other reviewed properties and limits

Repository observations: the bridge invokes the actual strict Source/6 parser and actual Core/5 wrapper; it does not manufacture authenticated or ledger-qualified results. Repayment overpayment uses the documented placeholder, and normal transfer/repay results preserve complete Core effects/post-state and external premises. Explicit malformed effects remain formation failures. Source economic IDs and nominal quantities are checked; scenarios reject decoded duplicate keys and unexpected fields. Public MCP operations take text and expose no filesystem, signing or shell execution. Horizon operations remain unsupported for execution. Transport, document and response budgets are enforced at the reviewed service boundaries; package paths bundle the actual S0 implementation. Editor/provider activation limitations are stated honestly.

Inference: these controls and the passing supplied tests support a useful bounded authoring beta, but do not establish full specification compliance while M1–M3 remain. No conclusion here closes authentication, native proof, formal financial correspondence, or atomic ledger acceptance. Re-freeze and obtain fresh review of repaired candidate bytes and focused results before reporting approval.
