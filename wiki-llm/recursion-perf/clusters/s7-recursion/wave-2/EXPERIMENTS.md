# First experiments — C1 after C0

**SOURCE — This wave ran only the arithmetic in [model.py](model.py).** No prover experiment below has run. Existing controls and factory CLI flags were checked in [local-c1-audit.md](../../../raw/s7-recursion/wave-2/local-c1-audit.md). The `sherlock-fused-e1` binary and its flags below are **proposed interfaces to implement**, not available commands. [OPENSPEC-DRAFT.md](OPENSPEC-DRAFT.md) assigns their outputs and requirements.

**ESTIMATE — Common protocol:** implement and build in the Moriarty repository after C0. Use its pinned lockfile/toolchain. Record the actual CPU affinity, 20 available cores, 90 GB RAM, GPU/driver and CUDA versions. Run one heavy job at a time through DoctorWatson. Start CPU-only, with RAYON=4 and one test thread; make RAYON=20 a separate cohort. Use measured caps and a 1,800-second initial wall limit. Run preflight before allocation. For a failure, inspect its trace, state one hypothesis, and retry with `--hypothesis`. Shrink the reproducer after two failures of the same kind. Do not repeatedly launch full proving after a deterministic mismatch.

**ESTIMATE — Manifest prerequisite:** implement `qa/sherlock/e1/manifest.json` as a public, validated manifest. Include C0 receipt ID and hashes; source/lockfile/features; exact fixture/capacities; ordered input digest; original transition/child-key/context identities; baseline P/D key and relation digests; official SRS path/hash/derivation; compiled call template hash; and machine/thread/cache settings. Do not include witnesses, private call openings, blinding or wallet material. Both sides use the same complete semantic input. The historical P=86/S=81 k13/k12 fixture and current A3 fixture must be identified separately. If their geometry differs, revise the budget instead of substituting inputs silently.

## E1 — Refute or establish the direct row budget

**OPEN QUESTION — Hypothesis:** preserving the original equations and complete binding fits k15, or at worst k16, without any recursive verifier or pairing gadget.

1. **ESTIMATE:** add tests before refactoring the original emitters. Expose shared private operands while preserving their original transition equations. Force both enables to one. Keep standalone child proof paths as independent reference controls.
2. **ESTIMATE:** compose program/pre/mid/post cells, contexts, envelopes, heights and all 16 statement fields. Add the 19-input statement hash, two-input communication opening and exact 15-slot call template. Use a stock architecture containing only required chips. Exclude child proof bytes, child proof verification and accumulator fields from the direct witness type.
3. **ESTIMATE:** implement `census` with explicit k=15 and k=16, without automatic size promotion. Measure the fragments alone and cumulative whole-circuit extents after each component. Record advice extent, each table extent, instance/fixed extents, actual unusable rows, columns, lookups, permutation width, gate degree, quotient extension, architecture and hash permutation counts. Marginal extents are attribution aids; the combined synthesis is authoritative.
4. **ESTIMATE:** run the proposed command from the repository root:

   ```bash
   RAYON_NUM_THREADS=4 scripts/doctorwatson heavy \
     --id S7E1-GEOMETRY --budget s7-e1-geometry --max-wall 1800 -- \
     cargo run --release --locked -p mori-recursion --features sherlock-ledger \
     --bin sherlock-fused-e1 -- census \
     --manifest qa/sherlock/e1/manifest.json --k 15,16 \
     --output target/s7-e1/geometry
   ```

5. **ESTIMATE:** compare measured component extents against 8,185/4,089 child allowances and 1,024/4,096/1,024/2,048 glue reservations. Report deviations, including the lookup configuration at each k. Confirm outer Blake2b has no circuit gadget while retaining its host timer. Do not add the historical recursive million-row table to this circuit.

**ESTIMATE — Deciding measurement:** pass geometry only when the actual maximum row extent plus unusable rows fits the explicit domain. Record k15 and k16 independently. The 20,466-row budget is a planning target, not a required exact count. k15 failure may advance k16; k16 failure stops this E1 implementation scope with its trace and best hypothesis. A fit with increased columns/degree proceeds to timing without a promised gain. Do not silently add k17 or remove semantic checks.

## E2 — Equivalence, stock ledger acceptance and migration refusal tests

**OPEN QUESTION — Hypothesis:** the direct relation proves the same bounded transitions and public statement meaning, and a newly registered stock verifier can accept its outer proof.

1. **ESTIMATE:** compare original fragments and direct relation on the exact pinned honest fixture. Compare all 16 semantic statement fields and ordered hashes. Use additional admissible witnesses when the pinned relation allows them. Record the fixture's limited coverage explicitly.
2. **ESTIMATE:** mutate each transition's equations and shared-cell wiring below host validation. Cover disabled enables, wrong context/rule/charge, changed program, pre/mid/post mismatch, reordered/omitted/duplicated transition, h_pre at overflow boundaries, wrong count/work/trace, instance/batch and bad communication opening. Label each outcome `constraint-rejected`, `public-statement-mismatch`, `host-refused` or `panic`; host refusal/panic alone is not evidence that constraints reject an adversarial witness. Do not assert that an unconstrained provenance digest is newly authenticated.
3. **ESTIMATE:** authenticate the official SRS before decoding. Derive smaller parameters using the pinned downsize path. Generate fresh C1 VK/PK and tagged verifier material. Never use `unsafe_setup` for acceptance evidence. Round-trip through the exact ledger-9 key reader, then verify using its embedded verifier parameters.
4. **ESTIMATE:** test every one of the 15 public slots, vector length, malformed/trailing key bytes, wrong VK/family, wrong transcript and proof-byte mutation. Require failure in the applicable reader or verifier. Obtain a valid SDK call vector through an in-memory adapter pipe. Test the binding input and communication opening against that vector; do not persist its private opening.
5. **ESTIMATE:** run the proposed commands:

   ```bash
   RAYON_NUM_THREADS=4 scripts/doctorwatson heavy \
     --id S7E1-EQUIV --budget s7-e1-equiv --max-wall 1800 -- \
     cargo run --release --locked -p mori-recursion --features sherlock-ledger \
     --bin sherlock-fused-e1 -- equivalence \
     --manifest qa/sherlock/e1/manifest.json \
     --output target/s7-e1/equivalence

   RAYON_NUM_THREADS=4 scripts/doctorwatson heavy \
     --id S7E1-LEDGER --budget s7-e1-ledger --max-wall 1800 -- \
     cargo run --release --locked -p mori-recursion --features sherlock-ledger \
     --bin sherlock-fused-e1 -- qualify \
     --manifest qa/sherlock/e1/manifest.json \
     --geometry target/s7-e1/geometry/selected.json \
     --output target/s7-e1/qualification
   ```

6. **ESTIMATE:** exercise an explicit deployment-family schema using local fixtures. Preserve legacy A4 parent/relation/VK checks for the recursive variant. The direct variant pins source semantics, C1 relation and C1 VK. Refuse stale or mixed manifests before synthesis/keygen. Corrupt one field at a time: address, VK, relation, old parent-family discriminator, source keys/context, template and SRS identity.

**ESTIMATE — Deciding measurement:** zero invalid-transition acceptances, equal honest semantic statements, exact tagged-key round-trip, and stock ledger-9 acceptance of the candidate proof are mandatory. A single missing constraint or reader mismatch stops qualification. Record offline acceptance as `candidate-key accepted`, never `installed-key accepted`.

**ESTIMATE — Later live qualification:** deployment is a separate operational task. Register the candidate verifier on a new stagenet deployment unless the pinned ledger's maintenance path is separately qualified. Read back the exact operation key and initialized anchor state. Re-pin `crates/mori-recursion/tests/fixtures/stagenet-verifier.json` and caller routing only from that readback. Use the same A4-strength exact-key test for the selected family. Test predecessor/height and replay refusals, then settle and independently read back effects. Preserve the old deployment and wallet. Record DUST, fee-proof count, synchronization and finality separately. This research session neither submits nor changes pins.

## E3 — Matched complete latency and the h threshold

**OPEN QUESTION — Hypothesis:** a qualified C1 reaches tenfold complete local proof-production acceleration under C0's fixed input and cache boundaries.

1. **ESTIMATE:** build both paths once, outside timing. Use release binaries and identical official SRS provenance. Report cold load/downsize/VK/PK separately. Keep both paths equally warm for the main comparison; include any per-request key reconstruction that remains in either implementation.
2. **ESTIMATE:** start the complete interval before fixture witness production. Baseline includes both child proofs, parent, D and required checks. Candidate includes direct witness construction, C1 proof and required checks. End after serialization and independent ledger verification. Use fresh randomness for every request. Do not time only the removed parent or reuse an already produced baseline child proof without charging it.
3. **ESTIMATE:** log monotonic phase durations for witness/synthesis, key load/setup, MSM, lookups, quotient/NTT, opening, Blake2b, serialization and verification. Also log total wall/CPU, RSS and, if applicable, VRAM/transfers. Do not sum overlapping timers into wall time. Include public artifact hashes only.
4. **ESTIMATE:** run one warm-up and five alternating matched pairs, with no concurrent heavy jobs:

   ```bash
   RAYON_NUM_THREADS=4 scripts/doctorwatson heavy \
     --id S7E1-BENCH --budget s7-e1-bench --max-wall 14400 -- \
     cargo run --release --locked -p mori-recursion --features sherlock-ledger \
     --bin sherlock-fused-e1 -- benchmark \
     --manifest qa/sherlock/e1/manifest.json \
     --qualification target/s7-e1/qualification/result.json \
     --pairs 5 --warmups 1 --order alternate --backend cpu \
     --output target/s7-e1/timing-r4
   ```

   **ESTIMATE:** the longer batch wall cap contains multiple sequential proofs; enforce the measured per-proof cap too. Repeat as a separate RAYON=20 cohort only if profiling motivates it. Test a CUDA 13.x/5090 backend only after CPU acceptance and measured memory admission. Do not multiply a GPU headline gain by the domain ratio. Emit the same typed proof artifacts and verify independently.

5. **ESTIMATE:** classify time that remains unchanged as h, scaled work as measured q/α, and new overhead as δ. Report `h+(1−h)αq+δ` against the measured total, including its residual error. Test the wave-1 screens h<7.0968% at k15 and h<4% at k16 without treating them as sufficient conditions. Report all five pair ratios, median ratio-of-medians, ranges and outliers without deleting inconvenient runs.

**ESTIMATE — Deciding measurement:** claim tenfold local acceleration only when `median(T_complete_C1) ≤ median(T_complete_baseline)/10` and E2 passes for all produced proofs. Report a valid smaller gain honestly. Never call this measured end-to-end stagenet acceleration without the separate matched submission/finality interval. If setup is amortized over m calls, publish `(setup0+m×T0)/(setup1+m×T1)` with measured setup and m. A different fixture, process scheduling, dependency version or cache regime creates a new cohort.
