PUBLIC DERIVATIVE — private Clean source detail omitted.
Original combined report: 86ddd5fbd5a9e76bab788179ef4a1af8bd8153c8a75f11edbe6ef76d60d00525 (9766 bytes).
Entire original line53 is omitted under Root disposition; K/Quint text and scoped verdicts are retained byteexact. This is an omission derivative, not a new review/verdict. Full original remains privately retained.

# Fresh Independent Non-Author Formal Math/Source/API Review — Overall Report (`K source09`, `Quint source10`, `Clean source12`)

## 1. Reviewer Identity, Routing & Non-Execution Attestation
- **Role**: Fresh independent non-author formal math/source/API reviewer (acting under the standing October 16 Opus quota fallback to Gemini 4 Argon at high effort via AGY).
- **Requested Identity / Effort**: Gemini 4 Argon (High) via AGY (`IdentityrequestedHIGH/backendunattested`: model selection requested at High effort in AGY session; underlying provider backend runtime remains unattested by host cryptographic receipt).
- **Execution Discipline**: Read-only tools only (`read_file`), plus writing only the four scoped review reports (`K-REPORT.md`, `QUINT-REPORT.md`, `CLEAN-REPORT.md`, `REPORT.md`) in `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/formal-public-nominal-counterparts-fresh-gemini4-source-review01/`. Zero model, compiler, kernel, `rustc`, Cargo, K, Quint, Lean, SDK, native, proof, provider, network, or Git commands were executed. No private Fenrir payloads were read or copied.
- **Independence & Non-Vote Qualification**: Each lane's expected return trees were derived from the two `.mori` source fixtures (`map-queue.mori`, `mirror-arbor.mori`) by that lane's author prior to writing the candidate model/proofs, without reading new Rust test assertions or actual compiler outputs. Same-author pre-code expectations are independent of the Rust producer outputs, not a second independent reviewer vote (`Eachauthornotavote`).

---

## 2. Independent Derivation of the Two `.mori` Fixtures Across All Three Lanes

1. **`map-queue.mori`** (`638` bytes, SHA256 `b8b30eb7b66280065092d4e8590c8c798b640e64b3c748153a997107f8c8010d`):
   - **Types & Declaration Field Order**:
     - `Queue<T> { Empty; Push(z_item: T, a_tail: Queue<T>); }` — `Empty: []`, `Push: ["z_item", "a_tail"]` (deliberately opposite lexical order).
     - `Envelope<A, B> { Wrap(z_items: A, a_marker: B); }` — `Wrap: ["z_items", "a_marker"]` (deliberately opposite lexical order).
   - **Semantics**:
     - Entry calls `pack(Push(3, Push(8, Empty())))`, which evaluates `Wrap(bump(xs), true)` in declaration field order.
     - `bump` recursively maps `Push(item, tail) => Push(item + 2, bump(tail))` and `Empty() => Empty()`, yielding `Queue<UInt>::Push(z_item: 5, a_tail: Queue<UInt>::Push(z_item: 10, a_tail: Queue<UInt>::Empty()))`.
     - Complete typed return: `Envelope<Queue<UInt>, Bool>::Wrap(z_items: Push(z_item: 5, a_tail: Push(z_item: 10, a_tail: Empty())), a_marker: true)`.
   - **Overflow Conditionality**:
     - Addition `item + 2` requires `0 <= item` and `item + 2 < 2^128` in `UInt128` (K `ngFinishAdd` normal domain and Clean `UIntReady`, where `(2^128 - 2) + 2 = 2^128` is proven outside `UIntReady`), and `0 <= item + 2 <= 255` in Quint's finite `MAX_SCALAR = 255` profile.

2. **`mirror-arbor.mori`** (`735` bytes, SHA256 `504fd3f85c354b778f4f5c4d49cd75155c21a62d087c24530f9a2fa51e93bfa9`):
   - **Types & Declaration Field Order**:
     - `Arbor<T> { Tip(z_value: T); Fork(z_left: Arbor<T>, a_right: Arbor<T>); }` — `Tip: ["z_value"]`, `Fork: ["z_left", "a_right"]`.
     - `Stamp<A, B> { Seal(z_tree: A, a_code: B); }` — `Seal: ["z_tree", "a_code"]`.
   - **Semantics**:
     - Entry calls `stamp(Fork(Tip(true), Fork(Tip(false), Tip(true))))`, which evaluates `Seal(reflect(t), 41)` in declaration field order.
     - `reflect` maps `Tip(value) => Tip(not(value))` and `Fork(left, right) => Fork(reflect(right), reflect(left))`.
     - Evaluating `reflect(right)` first on `Fork(Tip(false), Tip(true))` yields `Fork(z_left: Tip(z_value: false), a_right: Tip(z_value: true))`; evaluating `reflect(left)` on `Tip(true)` yields `Tip(z_value: false)`.
     - Complete typed return: `Stamp<Arbor<Bool>, UInt>::Seal(z_tree: Arbor<Bool>::Fork(z_left: Arbor<Bool>::Fork(z_left: Arbor<Bool>::Tip(z_value: false), a_right: Arbor<Bool>::Tip(z_value: true)), a_right: Arbor<Bool>::Tip(z_value: false)), a_code: 41)`.

---

## 3. Summary of Per-Lane Verdicts, First Feasible Checks & Mandatory Fixes

| Lane | Handoff Status | Source/API Verdict | Mandatory Source Fixes | Precise Feasible First Check (Existing Unchanged Wrappers/Caps) |
| :--- | :--- | :--- | :--- | :--- |
| **K (`source09`)** | `COMPLETE_GENERAL_SOURCE09_FROZEN_REVIEW_READY_UNRUN` | **APPROVE** | **None** | Single `F03.launch` compile `00-public-nominal-source09-compile` (`kompile .../tests/public-nominal-source09-laws.k --backend haskell --no-haskell-binary`, 30s wall / 3s sweep / 512 MiB fsize / 512 MiB JVM heap / 32 MiB stack / 2 CPUs, no prefix rebuild); inventory outputs and qualify Kore symbol labels before separate 4-law run. |
| **Quint (`source10`)** | `SOURCE10_GENERAL_NOMINAL_CASE_EXTENSION_UNTYPECHECKED_UNRUN_READY_REVIEW` | **APPROVE** | **None** | `01-newNominalSourceRuntimeFrontend` (`quint typecheck .../nominalPublicSourceRuntime.qnt`, 60s wall / 512 MiB Node heap / 4 MiB stream cap), followed after `01` PASS by `02-fourExactNominalSourceCaseTests` (`quint test .../nominalPublicSourceTest.qnt` on the 4 `q10*` tests), and optional `03-oneMirrorArborOperationalWitness` (`--max-steps 256 --verbosity 1`, 64 MiB ITF cap). |
| **Clean (`source12`)** | `SOURCE12_ACTUAL_RECURSIVE_SOURCE_CLAUSE_CANDIDATE_READY_FRESH_REVIEW_NO_KERNEL` | **APPROVE** | **None** | Single strict-leaf Lean kernel check `NominalRecursiveSourceCases` (`PROPOSED-ONE-ARGV.json`: `systemd-run` 4 GiB / 0 swap / 256 tasks / `timeout 200s` + `10s` / controller 220s / 16 MiB file & stream caps / `bwrap --unshare-net` with 15 RO-bound passed artifacts from `CanonicalTypeId`, `ConstructorFields`, `NominalValue`; 0 copies / 0 promotions / 0 prefix compiles; 5 final outputs in `OUTPUT-CONTRACTS.json`). |

---

## 4. Cross-Lane Findings & Explicit Open Premises

1. **No Contradictions Between Counterparts**:
   - All three counterparts independently derive the exact same typed return trees, generic type-argument lists (`Envelope<Queue<UInt>, Bool>`, `Queue<UInt>`, `Stamp<Arbor<Bool>, UInt>`, `Arbor<Bool>`), non-lexical declaration field orders (`z_item, a_tail`, `z_items, a_marker`, `z_left, a_right`, `z_tree, a_code`), `UInt128` (and Quint finite scalar) addition bounds, and recursive `Arbor::Fork` child swapping + `Bool` negation.
2. **Critical Technical Checks Confirmed**:
   - **K `source09`**: `ngNot` includes `rule ngOccurrence(ngFoundExpr(ngNot(O,_))) => O`, which is required when `ngNot` is a direct constructor field argument (`Tip(not(value))` in `MirrorArbor`) so `ngConstructorFields` can record the field expression occurrence on return. `npObserveHalt` preserves the full `NGState` alongside `NPValue`.
   - **Quint `source10`**: `BooleanNot` / `NotFrame` / `booleanNot` reuses `push`, `emit`, and `returnValue` without introducing a second evaluator. `q10SameTypeFieldSwapIsDistinguishedTest` proves that swapping the two `Arbor<Bool>` children of `Fork` remains valid under `validValue` (`[30, 30]`) yet is distinguished by structural `Value` equality.
[PUBLIC OMISSION: complete detailed private Clean bullet, original line53; see PUBLIC-EXCLUSIONS.json for byte/hash provenance.]
   - **Capture Wrapper Note**: For Quint `source10` and Clean `source12`, the referenced `capture-helper.FULL.py` / `run_tail_four.py` supplies the `capture(op, phase)` subprocess/stream-cap primitive; Root must bind each lane's stage acceptance (`PROSPECTIVE-VALIDATION.json` for Quint; `PROPOSED-ONE-ARGV.json` + `OUTPUT-CONTRACTS.json` for Clean) rather than calling `run_tail_four.py`'s Cargo-specific `main()`.
3. **Preserved Open Boundaries (Not Established by This Source Review)**:
   - Every new model and proof candidate in `source09`, `source10`, and `source12` remains **UNRUN**.
   - Local numeric IDs (`4, 5` in K; `10, 20, 30, 40` in Quint; `100, 101, 200, 201` in Clean) are declared local projections, **not** canonical Moriarty compiler `TypeIdC` allocations or a generic monomorphization proof.
   - Clean `source12` models standalone positive source-clause relations (`QueueBump`, `ArborReflect`) on `NominalValue.Value` / `HasType`; the existing Clean `NominalSemantics.Expr` machine still lacks Boolean `not`, and bridging `QueueBump` / `ArborReflect` to `NominalSemantics.Evaluation` remains **OPEN**.
   - None of these source counterparts establish owner-B work/fuel clocks, complete cross-layer event-prefix equality, compiler admission, handwritten/Clean circuit equivalence, native Halo2 proofs, or EARS requirement closure.

---

## 5. Scoped Reports Written
- [K-REPORT.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/formal-public-nominal-counterparts-fresh-gemini4-source-review01/K-REPORT.md)
- [QUINT-REPORT.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/formal-public-nominal-counterparts-fresh-gemini4-source-review01/QUINT-REPORT.md)
- [CLEAN-REPORT.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/formal-public-nominal-counterparts-fresh-gemini4-source-review01/CLEAN-REPORT.md)
- [REPORT.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/formal-public-nominal-counterparts-fresh-gemini4-source-review01/REPORT.md)
