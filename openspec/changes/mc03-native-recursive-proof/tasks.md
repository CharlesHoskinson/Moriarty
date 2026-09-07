# Tasks: Reviewed native encoding and recursive proof

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Produce and independently verify a real two-step financial recursive proof under a reviewed resource contract.

**Dependencies:** MC01.

**Implementation root:** `experiments/moriarty-native-ivc-r3/revision-01/`.

**Interfaces:** NativeEpisode binds the original R2 financial episode, all public contexts, fixed authority scope, and the remaining lifecycle bound. ProofArtifact binds proof bytes, VK, SRS, backend pin, statement encoding, and final accumulator verification.

## 1. Review the encoding and resource decision

- [ ] 1.1 Compare direct limbs with checked commitment encoding. Inspect constraint costs without proving. Select a justified representation.
- [ ] 1.2 Obtain Fable and independent GPT-6 approval of relation equivalence, exact command, k, SRS, and limits.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement encoding and arithmetic controls

- [ ] 2.1 Write independent preimage, limb-boundary, arithmetic, context, and malformed-genesis tests before changing the relation.
- [ ] 2.2 Compare every original episode field with R2. Require native and in-circuit relations to agree.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Implement retained proof verification

- [ ] 3.1 Add a separate verifier process and proof/context mutations before launching the positive proof campaign.
- [ ] 3.2 Require fresh deserialization, expected VK/SRS identity, transcript exhaustion, and discharged accumulator obligations.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Run one reviewed proof campaign

- [ ] 4.1 Adapt the existing cgroup wrapper to the new immutable contract. Preserve old counters and failures.
- [ ] 4.2 Run exactly two positive steps and retained negative controls. Stop immediately on the first failed predicate.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --preflight-only
python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --execute
python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC03/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
