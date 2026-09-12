# Design: PCD ledger-anchored acceptance

Dependencies: MC01 for the language profile, MC02 for integrated financial effects, and the Midnight dependency tracker in `openspec/sprints/pcd-integration.json`.

## Inputs

- Decision report `deliverables/pcd-midnight-native-2026-09-11/REPORT.md`, digest-pinned in `pcd-integration.json`.
- PCD roadmap `openspec/PCD-ROADMAP-2026-09-11.md`, digest-pinned the same way.
- Reproduced measurements `evidence/pcd-midnight-native-2026-09-11/MEASUREMENTS.md`.

## Outputs

Specified-only planning records. Implementation paths are fixed at `native-path-freeze` and in each sprint execution packet.

## Interfaces

| Interface | Definition | Owner |
|---|---|---|
| Verification seam | Ledger `well_formed` checks each contract-call proof against the operation key read from `ContractState.operations` | MC04 |
| Step relation | One circuit per entry point: authorization, transition validity, effect projection, intent refinement and per-step invariant | MC04 |
| Head | `(instanceId, headId, revision, stateCommit, lifecycle, Π_P, netTag)`; every write follows a read of the same head in one transcript section | MC04 |
| Deploy audit | Recomputes address, operation set, keys, `Uninit` state (with the recorded predecessor for a successor) and authority: committee `[]`, threshold ≥ 1, counter 0 | MC04 |
| Program digest Π_P | Core, bounds, property-certificate hash, claim sets, toolchain versions and a successor allowlist of program digests | MC05 |
| Intent digest v2 | Exact-head and outcome modes | MC05 |
| Entry points | `Initialize`, `Step`, `Split`, `Join`, `Terminate`, optional `Pause`; cross-contract `Release`, `JoinFrom`, `Migrate`, `ImportFrom`, `Reclaim` | MC05, MC06 |
| Certificate entry point | `VerifyProof` and `InnerProof` with guard constant 1; ledger-side accumulator pairing on `ledger-10` | MC03 |

## Trust boundaries

- The ledger is trusted to verify proofs against stored keys, to check `Popeq` reads at application and to apply one intent atomically.
- The deploy audit is trusted to reject a deployment whose keys or authority differ from a reproducible build. Relying parties run it on counterparties.
- Oracle truth, custody and witness availability stay external assumptions.
- Claimed cross-contract calls are atomic in one direction only.
- `netTag` separates networks. It does not separate byte-identical replicated deployments.

## Semantic guarantees and feasibility

Semantic guarantees come from the step relation, head discipline and induction over ledger acceptance. Feasibility comes from experiments E1–E5 and the bounds freeze. A failed experiment stops its stage. It never relaxes an acceptance requirement.

## Failure handling

- E1 fails: stop Stages 2–6 until a reviewed consumption-set design replaces head discipline.
- E2 fails: revisit relation decomposition within k ≤ 17.
- `ledger-10` changes formats: pin the released formats and rerun E3.
- Recursion slips: certificate stages record `blocked`. The core and the Preview gate stand.
- The certificate resource amendment is refused: certificate stages stay blocked.
