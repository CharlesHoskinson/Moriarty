# Developer mock implementation plan

**Goal:** deliver a runnable local developer workspace for the approved loan,
swap and proof-workflow design, with honest simulated evidence.

**Architecture:** TypeScript pure scenario/workflow functions, a browser UI and
a loopback-only static server. No runtime framework or remote services. The mock
is a separate experiment; it does not implement the full DSL or a proof backend.

**Location:** `experiments/moriarty-developer-mock/` in the isolated
`moriarty-developer-mock` worktree. User's “begin” approves the preceding design
and this first mock implementation. Original source and experimental freezes
remain intact.

## Constraints

- Distinct `SimulatedEvidence`; real verification always returns unavailable.
- Imported expected ACTUS results stay labeled imported, separate from edited
  first-period arithmetic previews. Currency is not an actual settlement token.
- BigInt calculations and bounded decimal parsing; no financial Number rounding.
- Changing inputs invalidates all prepared simulated authorization/proof state.
- No wallet, chain, prover, telemetry, CDN or external API connection.
- Local exports retain mock labels. Recovery cannot restore a real certificate.
- Implement the scenario table from the approved interface: loan/calendar/option,
  swap rounding/slippage, mandate, observations, proof/domain failures, duplicate
  consumption, composition, cancellation/partial-fill and recovery demonstrations.

## PCD report changes applied during this sprint

Follow [R0–R6](../../research/2026-09-06-pcd-report-integration.md#revised-sprint-sequence-and-exit-evidence).
Finish this compatible mock; add a proposed typed-claims panel with required
IntentEffects, ContractInvariant, TransitionValidity and HistoryCompliance,
all real checks unavailable. Do not implement a cryptographic codec or claim
proof support through simulated evidence.

After the mock: shared ACTUS/DeFi semantic slice and canonical claim/intent
bindings; the Midnight-native IVC boundary; private cross-party split/join PCD;
checked contract certificates and package expansion. A native Rust proof is a
separate gate from ledger acceptance. Optional acceleration comes last and
preserves mandatory acceptance. No generic zkVM or foreign recursion detour is
authorized by the report. Proof experiments still need their bounded run record.

Recovery in this mock restores configuration and clears staged evidence. Its
simulated consumption ledger is tab-local; a reload starts a new demo ledger.
Input edits must preserve consumption within the tab. Only an explicitly named
reset may clear it. This is not durable transaction-submission recovery.

## Work

1. Add source-pinned fixture extracts and Node tests for exact swap output,
   slippage/rounding rejection, rational loan preview, bounds, staged evidence,
   changed-input invalidation, duplicate simulated submission and unavailable
   real verification. Watch missing behavior fail, then implement
   `src/model.ts`. Tests use Node's TypeScript stripping, no test framework.
2. Add `index.html`, `styles.css`, `src/app.ts`: responsive workspace, example
   selection, editable bounded inputs, imported event details, generated source
   preview, diagnostics, property status, explicit demo workflow and JSON export.
   Use DOM text rendering for imported/user values. Static fixture JSON comes
   only from the pinned local corpus.
3. Add build/typecheck and a loopback server, README launch instructions and
   browser checks. Validate desktop/mobile rendering, keyboard-accessible
   controls, scenario changes, proof/ledger distinction, export labels and
   local-only network use. Use the existing Playwright installation.
4. Review the whole mock for spec/quality, fix material findings, record scoped
   results, update the current roadmap and checkpoint, commit and integrate the
   completed experiment. Keep unrelated dirty files outside commits.

## Acceptance commands

From the experiment: `npm test`, `npm run build`, `npm start`.
Expected swap: 19,743 output for reserves 1,000,000/2,000,000 and input 10,000;
minimum 19,744 rejects. Expected loan first period: exact rational 2480/73 for
principal 5,000, rate 0.08 and 31/365, displayed separately from the imported
33.972602739726. A full demo can complete only as simulated evidence; real
verification stays unavailable. Browser smoke covers the rendered workflow,
exports and responsive layout. No native verification campaign is authorized.


## Recorded outcome

The first local interaction mock is implemented and exercised; see
[README and limitations](../../../experiments/moriarty-developer-mock/README.md)
and [validation](../../../evidence/moriarty-developer-mock-2026-09-06/README.md).
Independent review corrections cover changed-input evidence, consumption history
and ineffective controls. Imported timelines are selectable, not regenerated;
event-order mutation, complete effect checking, actual successor fills, lending
health and durable submission recovery remain R2+ semantic work. The mock does
not establish completion of the full SDK or of every future scenario predicate.
