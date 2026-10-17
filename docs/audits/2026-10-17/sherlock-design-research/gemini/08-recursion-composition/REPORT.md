# Domain 08 Review: Native Recursion, Execution Chunks & Composition Architecture

- **Model**: Requested `Gemini 4 Argon (High)` / Observed `Gemini 4 Argon (High)`
- **Baseline & Candidate**: Worktree `628615a94d8a74837a1af510293927bb74d5c2b5` (`docs/sherlock-audit-improvements-20261017`); Candidate 02 (`3f689d5fbe9503fb997c8f305331c58801e9db15e618080f2d52cc90068ad7bb`). `native_authority == 0`, `C4 OPEN`, `R2 UNSATISFIED`, `60460 HOLD`.
- **Coverage & Captures (2 web queries, 3 Scrapling captures; 5/5 network calls)**:
  - Local: [SHERLOCK-IMPLEMENTATION-PLAN.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/SHERLOCK-IMPLEMENTATION-PLAN.md), [SHERLOCK-REFERENCE-STATUS.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/SHERLOCK-REFERENCE-STATUS.md), [SHERLOCK-CORRECTNESS-LOOP.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/SHERLOCK-CORRECTNESS-LOOP.md), [MORIARTY-BACKEND-REQUIREMENTS.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/MORIARTY-BACKEND-REQUIREMENTS.md#L22-L60), [pcd-midnight-native-architecture.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/wiki/decisions/pcd-midnight-native-architecture.md#L15-L62), [interfaces.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md#L179-L234), [security-candidate-b.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md#L64-L99), [spec.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md#L97-L148), [08/proposal.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/research/sherlock/papers/08/proposal.md), and audit reports [01](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/01-implementation/REPORT.md), [02](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/02-evidence/REPORT.md), [03](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/03-blockers/REPORT.md), [08](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/08-doctorwatson-evidence/REPORT.md).
  - Primary captures in [sources.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/08-recursion-composition/sources.json): (1) Bünz, Chiesa, Mishra, Spooner (BCMS20), `https://eprint.iacr.org/2020/499` -> [capture-01-bcms20-eprint-2020-499.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/08-recursion-composition/capture-01-bcms20-eprint-2020-499.md) (HTTP 200; quotes: *"accumulation scheme"* and *"proof-carrying data"*); (2) O(1) Labs Pickles path `https://o1-labs.github.io/proof-systems/specs/pickles.html` -> [capture-02-o1labs-pickles-spec.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/08-recursion-composition/capture-02-o1labs-pickles-spec.md) (HTTP 404 under `--no-follow-redirects`, logged without retry); (3) OpenVM modular continuations repository `https://github.com/openvm-org/openvm` -> [capture-03-openvm-repo.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/08-recursion-composition/capture-03-openvm-repo.md) (HTTP 200).

## Concrete Current Design & Code Gaps

1. **Unbridged Step-Mode vs. Canonical Boundary Chaining**: In [`emit_literal_complete_execution`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-backend/src/sherlock_machine/literal_complete_execution.rs#L18-L25), `A102 -> A80` chains in-circuit in Step mode (`outcomeValue.space == 0`, `activeData == 0`). Meanwhile, [`project_a`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/sherlock/layout_a/observation.rs#L90-L95) is a private `#[cfg(test)]` host helper, [`layout_a`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/sherlock/layout_a/mod.rs#L21-L26) cannot reload a decoded `Snapshot` into `Machine`, and [`helper_machine`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/sherlock/helper_machine/mod.rs#L1-L26) supports only genesis and one uncut host segment. Candidate 02 requires `ChunkStore` boundary canonicalization (`projectA`) while mid-segment steps use append-only `Data` (`StepInv`), leaving unspecified how chunk $i$'s Step-mode output connects to chunk $i+1$'s canonical input without trusting an unverified host re-encoding.
2. **Host-Loop Grouping vs. Multi-Parent PCD Statement Schema**: [`advance_m`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/sherlock/reference_slice.rs#L63-L72) (`#![cfg(test)]`) and [`Machine::visit_steps`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/sherlock/reference_stream.rs#L38-L67) are host loops. [`StepInput`/`StepOutput`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md#L214-L234) define only 1-to-1 linear steps with no fan-in statement for the five required composition operators (`ZR09`, `MC06`) or lineage tracking to prevent double-counting diamond ancestors (`MNR07`).
3. **Conflation of Technical Proof Cuts and Financial Commits**: In Core/2 [`Continuation`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/stage/mod.rs#L107-L169), `logical.committed` and `boundary` advance only at source `Await` boundaries; pre-await failures discard tentative effects ([continuations.rs](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/tests/continuations.rs#L100-L117)). Sherlock's [`ReferenceState`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md#L66-L71) lacks a split between committed checkpoint state and tentative intra-action state carried across a `TechnicalCut`.
4. **Ledger-9 Settlement Envelope vs. Recursive Accumulator Split**: [`verify_sherlock_complete_call`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-ledger/src/sherlock_literal_complete.rs#L523-L545) binds one call via `communication_commitment` (`pi[1]`) and `ProofMarker::proof_verify`. Per `CLM-0948`/`CLM-0954`, Compact/ZKIRv3 has no `verifyProof` opcode; recursion uses custom-decider accumulation (`UIREQ-023`), which Candidate 02 does not yet separate from the ledger-9 `ContractOperation` opening in [`interfaces.md`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md).

## Ranked Architectural Ideas

### 1. Dual-Plane Continuation State: Technical Cut vs. Financial Commit (Rank 1 — Mandatory)
- **Evidence & Applicability**: OpenVM connector segments ([capture-03-openvm-repo.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/08-recursion-composition/capture-03-openvm-repo.md)) and RISC Zero `SystemState` ([08/proposal.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/research/sherlock/papers/08/proposal.md#L19-L26)) separate proving shards from guest commits. Pairing this with Core/2 [`StageOutcome`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/stage/mod.rs#L155-L169) prevents a prover-chosen `TechnicalCut` from mutating committed financial state or residual duties before an action reaches `Commit`/`Await`/`Yield`.
- **Tradeoffs**: Adds committed-anchor and tentative-delta fields to chunk boundary records, slightly widening boundary serialization in exchange for exact rollback preservation on mid-action error/exhaustion (`A82`/`A84`).
- **Proposed Spec Text** ([interfaces.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md)):
  ```text
  record ChunkContinuationState where
    programIdentity       : ByteVec 32
    profileId             : ProfileId
    committedAnchorDigest : ByteVec 32 -- advances only on Commit / Await / Yield
    tentativeStepDigest   : ByteVec 32 -- exact Step-mode image across TechnicalCut
    committedWork         : Nat
    tentativeWork         : Nat
    residualDutiesDigest  : ByteVec 32
    boundaryKind          : BoundaryKind -- TechnicalCut | SourceSuspension | Terminal
  ```
- **Falsifiable Test**: Cut an action into two `TechnicalCut` chunks before an `A82` Error or `A84` FuelExhaustion; verify the terminal state rolls back tentative transfers to `committedAnchorDigest` while charging `tentativeWork` against fuel (`UIREQ-026`, `ZR06`).

### 2. Two-Mode Boundary Connector (`StepJoin` vs. Verified `CanonicalCheckpoint`) (Rank 2 — Mandatory)
- **Evidence & Applicability**: Rather than invoking host-only `project_a` at every technical cut, distinguish intra-action `StepJoin` (which chains append-only `StepInv` images by exact physical digest equality `UISEC-009` and explicit allocation counters `activeEnv`/`activeData`) from `CanonicalCheckpoint` (which requires a verified projection witness `ProjectRelation(stepImg, canonicalImg)` only at `SourceSuspension`, `Terminal`, or cross-party handoff).
- **Tradeoffs**: Avoids in-circuit graph compaction on every `m`-step proof cut while preserving Opus `A4` (`F3`/`F4`/`F5`/`StepInv`) at all external boundaries.
- **Proposed Spec Text** ([security-candidate-b.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md)):
  ```text
  Contiguous TechnicalCut chunks within one action segment chain via StepJoin:
  chunk[i].postStepDigest == chunk[i+1].preStepDigest with monotone allocation
  counters under StepInv. External checkpoints (SourceSuspension, Terminal,
  CrossPartyHandoff) require CanonicalCheckpoint satisfying ProjectRelation.
  ```
- **Falsifiable Test**: Reject any `ChunkStore` resume that substitutes a canonicalized image into a `StepJoin` slot without `ProjectRelation`, or that mutates inactive rows above `activeData` (`UISEC-009`).

### 3. Bounded Fan-In PCD Compliance Node with Lineage Conservation (Rank 3 — Mandatory)
- **Evidence & Applicability**: BCMS20 ([capture-01-bcms20-eprint-2020-499.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/08-recursion-composition/capture-01-bcms20-eprint-2020-499.md)) defines PCD compliance over local data $z$, incoming messages $(z_{\text{in},1},\dots,z_{\text{in},k})$, and an accumulator verifier without requiring a sublinear SNARK verifier. Borrowing this structure for [`interfaces.md`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md) supports all five composition operators (`ZR09`, `MC06`) and diamond lineage accounting (`MNR07`).
- **Tradeoffs**: Fixed arity ($k \le 2$ parents per join node) requires binary-tree folding for wider fan-in, keeping circuit layout static.
- **Proposed Spec Text** ([interfaces.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md)):
  ```text
  enum CompositionOp := Seq | DisjointPar | SharedInterleave | AtomicSync | AsyncMsg
  record ComplianceStatement (k : Nat) where
    op           : CompositionOp
    parentActive : Vector Bool k
    parentClaims : Vector ChunkContinuationState k
    lineageRoot  : ByteVec 32 -- rejects duplicate ancestor conservation claims
    mergedClaim  : ChunkContinuationState
  ```
- **Falsifiable Test**: Construct a 2-parent diamond join sharing a common ancestor; assert `ComplianceStatement` rejects duplicate resource/credit conservation or mismatched `programIdentity`/`profileId` across branches (`ZR09`, `MNR07`).

### 4. Two-Tier Statement Envelope: Inner Accumulator vs. Ledger-9 `ContractOperation` (Rank 4 — Optional/Staged)
- **Evidence & Applicability**: Per `CLM-0948`/`CLM-0954`, Midnight `ContractOperation` proofs bind `pi[1] = communication_commitment` ([sherlock_literal_complete.rs](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-ledger/src/sherlock_literal_complete.rs#L523-L545)). Separating the inner custom-decider accumulator `(ComplianceStatement, accInstance)` from the outer ledger-9 call opening keeps `ZR08`/`ZR10` aligned without assuming `verifyProof` in Compact.
- **Tradeoffs**: Requires explicit versioned domain tags between inner accumulator instances and outer `Transcript(v2.3)` commitments.
- **Proposed Spec Text** ([interfaces.md](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md)):
  ```text
  record LedgerSettlementEnvelope where
    complianceDigest : ByteVec 32 -- H("sherlock-pcd/1", ComplianceStatement)
    accumulatorHash  : ByteVec 32 -- custom-decider accumulator commitment
    consumedStateRef : ByteVec 32 -- single-consume authenticated ledger state (ZR10)
  ```
- **Falsifiable Test**: Replay a previously settled `consumedStateRef` or mismatch `complianceDigest` in `SherlockCompleteCallOpening`; verify deterministic rejection (`ZR10`, `MNR03`).

## Mandatory vs. Optional & What to Reject/Defer

- **Mandatory before multi-chunk/native claims**: Ideas 1, 2, and 3; all 35 `UIREQ` requirements, full-39 event semantics, `B-T01..14`, `U0..U7`, `M/MC/SP/G` exits, `ZR01..16`, `MNR01..08`, and Opus `C4`/`R2`/`60460` security gates.
- **Optional / Staged**: Idea 4's outer `LedgerSettlementEnvelope` wiring can stage behind local `StepJoin` and `ComplianceStatement` closure.
- **Reject / Defer**:
  1. **Reject** claiming `advance_m` or `visit_steps` host loops establish recursive continuations or PCD (`UIREQ-022`, `ZR04`).
  2. **Reject** importing foreign zkVM proof backends (STARK/BabyBear/Goldilocks verifier circuits) or assuming an unshipped Compact `verifyProof` opcode (`CLM-0948`).
  3. **Reject** unverified host `project_a` re-encoding between proven chunks (`UISEC-009`).

## Limited Design Verdict

**Conditionally sound direction; incomplete chunk/composition interface contract.** Candidate 02's Option 3 (converging A/B around explicit profile adapters) is the right architectural choice, provided [`interfaces.md`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md) and [`security-candidate-b.md`](file:///home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md) incorporate the `ChunkContinuationState` dual-plane split (Idea 1), the `StepJoin` vs. `CanonicalCheckpoint` connector rule (Idea 2), and the `ComplianceStatement` multi-parent schema (Idea 3) prior to multi-chunk or recursive implementation claims.

