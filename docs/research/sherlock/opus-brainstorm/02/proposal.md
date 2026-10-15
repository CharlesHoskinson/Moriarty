# Opus lane 02: universal step/recursion interface on Midnight IVC (ledger9 custom decider, no VerifyProof)

**Status:** independent Opus 5.5 brainstorm; nothing adopted or approved. Native authority 0; no K/Quint/native/SDK/proof execution; Fenrir source neither read nor sent. Labels: **[F]** source fact, **[I]** inference, **[L]** limitation.

## 1. Source facts used

- **[F]** `IvcCircuit<T>` proves one step. Its public instance is `(selfVk repr, T::format_public_input(state), accumulator)`. The witness is `(prev_state, prev_acc, prev_proof, transition_witness)`. See `ivc-circuit.txt` lines 23–33, 94–106 and 154–164.
- **[F]** `prev_proof` is verified under the *self* VK, a public input; "`vk` never needs to be checked against the IVC instance" (lines 43–50) because canonical `setup` supplies it, not because the circuit checks it.
- **[F]** If `prev_state` equals `genesis(ctx)` (fixed constants), the prior accumulator is scaled by 0 (lines 214–225). `ctx` feeds `T::new` and the genesis constants (lines 176, 217), so changing `ctx` changes circuit and VK.
- **[F]** `IvcState::decider` is *off-circuit*, called by `IvcVerifier::verify` (lane-06 `midnight-ivc.md` lines 103–116). N is hidden unless state carries a counter (lines 18–20).
- **[Premise, L]** The user's Fenrir ledger9 custom-decider route accepts this recursion without `VerifyProof`/ledger10; its internals are unread here.

## 2. Candidate architecture

**[I] One reusable interpreter circuit per *semantics configuration*, with code as data.** The `ctx` holds only the semantics identifiers: opcode set version, field and u128 encoding, the bound tuple `(maxFrameDepth, maxAccessPath, microstepsPerStep, maxEffectsPerStep)`, and hash domain tags. Changing semantics produces a new VK, and that is intentional: **VK identity = interpreter semantics identity.** Program identity never goes into `ctx`. A per-program `ctx` would mean per-program keys, which amounts to generated semantics.

**Assigned state (constant size):**
`(semId, phase, programRoot, entryId, inputCommit, pc, envRoot, heapRoot, frameRoot, effectAcc, fuel, stepCtr, status)`.
The native state also carries the preimages: the code table, heap nodes and frames. `effectAcc` is a hash chain of ordered financial effects and residual duty.

**Self-recursive closure and program loading.** Genesis is one fixed state per `ctx`, so it cannot name a program. **[I]** Genesis is `phase=EMPTY` with domain-tagged zero roots; the only legal transition from it is `LOAD(programRoot, entryId, inputCommit, fuel0)` → `phase=RUN`, after which those fields and `semId` are copy-constrained. Code identity binds late but irrevocably.

**Terminal states.** `HALT_OK` and `HALT_ERR` are absorbing: their step is a constrained no-op that only increments `stepCtr`. Without this, a halted state could be resumed after the endpoint has read it.

**Where checks must live.** Checks that the native preimage matches its summary are typically put in `IvcState::decider`. Here they **must** move into `circuit_transition`, as per-step authenticated openings of code, heap and frame nodes. The ledger9 endpoint enforces only what its custom decider circuit enforces. It does not run Rust `decider` hooks. **[I, strongest security finding of this lane]**

**Endpoint adapter.** `StepStatement → Ledger9Statement` is a named pure projection exposing `semId`, `programRoot`, `entryId`, `inputCommit`, `status`, `effectAcc`, remaining `fuel` and `stepCtr`. The expected self-VK representation is an adapter/contract **constant**, never prover-chosen; the custom decider checks the accumulator invariant against it.

**Generic code and data.** Code is a Merkle-committed instruction table from deterministic lowering (host work). Recursive ADTs are content-addressed immutable nodes `(tag, arity, fields…)` in `heapRoot`: acyclic by hash construction, sharing allowed. Frames are a hash-linked stack `(retPc, dest, envRoot, next)`. One step = fetch, decode, one bounded opcode gadget, fuel/counter update, effect append.

## 3. Reusable circuits vs. generated semantics: tradeoffs

- **A. Universal step, code as data (recommended candidate).** Identity: VK = semantics, `programRoot` = code. Pros: one audit surface and one VK; upgrading a program needs no keygen. Cons: every step pays for every opcode branch, plus Merkle openings.
- **B. Small opcode family with a selector.** Identity: k VKs, with the selector pinned. Pro: cheaper steps. Con: needs a custom wrapper, because the stock `IvcCircuit` recurses only on its own VK. Recursion across VKs needs an allow-list, which adds attack surface.
- **C. One generated circuit per program.** Identity: VK = program. Pro: fastest. Cons: keygen for every program, the compiler joins the trusted base, and little reuse.

**Counterargument [I]:** A gives up concrete cost for audit simplicity, and no cost has been measured. B's VK allow-list is the most likely place for a VK-confusion bug. If B is pursued, the allow-list must be a constant root committed in `ctx`.

## 4. Draft K/Quint correspondence notes

K cells: `<sem> <phase> <code> <pc> <env> <heap> <frames> <effects> <fuel> <ctr> <status>`; `LOAD` only from `EMPTY`; halt rules absorbing. Quint (bounded): lifecycle, fuel monotonicity, frame depth, effect prefix order; commitments are opaque IDs.

**[L]** K proves rule-level semantics only. Quint samples the lifecycle. Neither proves circuit correspondence, hash binding, accumulator soundness or ledger acceptance.

## 5. Owned EARS clauses (Opus draft)

- **UISEC-BACKEND-001** (code-identity immutability): When the step's pre-state has `phase=RUN`, the step circuit shall constrain `programRoot`, `entryId`, `inputCommit` and `semId` in the post-state to equal those in the pre-state.
  - *Accept:* any witnessed difference makes the circuit unsatisfiable.
- **UISEC-BACKEND-002** (single load from genesis): The step circuit shall permit `LOAD` iff the pre-state equals `genesis(ctx)`.
  - *Accept:* `LOAD` from any non-`EMPTY` state is unsatisfiable. No non-`LOAD` opcode is satisfiable from `EMPTY`.
- **UISEC-BACKEND-003** (in-circuit binding, no off-circuit decider reliance): The interpreter shall enforce every consistency between native preimages and assigned summaries inside `circuit_transition`.
  - *Accept:* `IvcState::decider` for the interpreter checks nothing beyond the accumulator. A review traces each consistency check to a constraint.
- **UISEC-BACKEND-004** (pinned self-VK): When the ledger9 custom decider evaluates an interpreter statement, it shall compare the self-VK representation against a constant fixed for the admitted `semId`.
  - *Accept:* a statement carrying any other VK representation is rejected. Nothing derived from the prover selects the VK.
- **UISEC-BACKEND-005** (absorbing termination): While `status∈{HALT_OK, HALT_ERR}`, the step circuit shall constrain every field except `stepCtr` to be unchanged.
- **UISEC-BACKEND-006** (fuel and progress): For each non-absorbing step, the circuit shall enforce `fuel' = fuel − cost(op) ≥ 0` and `stepCtr' = stepCtr+1`, using range-checked arithmetic. If fuel would go negative, the step shall set `HALT_ERR`.
- **UISEC-BACKEND-007** (injective public input): The `format_public_input`/`as_public_input` encoding shall be domain-separated and injective over the assigned tuple. The u128 and tag limbs shall be range-checked.
  - *Accept:* no two distinct tuples encode to the same vector. Genesis is unique.
- **UISEC-BACKEND-008** (projection fidelity, no VerifyProof dependency): The endpoint adapter shall be a total, documented projection. It shall need neither `VerifyProof` nor ledger10.
  - *Accept:* the adapter reads only `HALT_OK` statements. It exposes `effectAcc` exactly, so residual duty cannot be dropped.

## 6. Open items

Open items for designated review: the Fenrir custom-decider internals, whether the ledger9 statement is wide enough for the projected fields, the costs of A and B, and whether `stepCtr` should be public.
