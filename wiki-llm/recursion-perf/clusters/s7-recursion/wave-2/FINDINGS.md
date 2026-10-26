# s7-recursion wave 2 — make C1 checkable

**ESTIMATE — Verdict:** Advance C1 to the bounded E1 implementation described in [OPENSPEC-DRAFT.md](OPENSPEC-DRAFT.md), after C0. C1 directly constrains A102→A80 and their link. It needs **no in-circuit proof verifier, accumulator, subgroup check for recursive proof points, or pairing gadget**. The recorded 316,833-row gadget and its k19 floor therefore **do not apply to C1**. The ledger still checks the new outer PLONK/KZG proof over BLS12-381 with Blake2b and the official SRS. This conclusion follows from the separation of relation constraints and host verification, not from a new benchmark. Sources: [local implementation audit](../../../raw/s7-recursion/wave-2/local-c1-audit.md), [pinned stdlib](../../../raw/s7-recursion/wave-2/stdlib-2.3.5-public.md), [private inventory summary](../../../raw/s7-recursion/wave-2/private-inventory-summary.md).

**OPEN QUESTION — The fit and the speedup remain unproven.** Exact A102/A80 row counts are absent from the recorded inventory. The current A3 fixture also has different capacities from the historical k13/k12 report. The provisional C1 budget is **20,466 advice rows**, with tables separately constrained; it is not a synthesis result. At unchanged width and linear domain scaling, strict >10× requires **h < 7.0968% at k15** or **h < 4% at k16**. C0 must establish the matched baseline and E1 must measure the candidate. No current matched complete-chain result exists in the inspected records.

## Evidence and the two different fusions

**SOURCE — Evidence anchors:** [R1: local audit](../../../raw/s7-recursion/wave-2/local-c1-audit.md) records source hashes at Moriarty HEAD `865ad2dcfa1fc2e0e8c40ad0809b16d55764fbdd`; [R2: private inventory summary](../../../raw/s7-recursion/wave-2/private-inventory-summary.md) records exact numbers and private checkout HEAD `843c672dbe6c4fd2fafa5848f413c12db201e364`, without copying its text. [R3: Astra-2](../../../raw/s7-recursion/astra2-prior.md), [Loki-05](../../../raw/s7-recursion/loki05-prior.md), [Loki-07](../../../raw/s7-recursion/loki07-prior.md) and [s7 wave 1](../wave-1/FINDINGS.md) are prior studies, not independent replications. The two requested Loki files were located in `loki/nodes/`, not `loki/10x/`.

| Design | What the relation establishes | Pairing / transcript boundary | Geometry implication |
|---|---|---|---|
| SOURCE: current children → P → D | P prepares two child proofs; D verifies P and discharges its carried obligation. | In-circuit residue pairing in D; host Blake2b for D's outer proof. [R1], [R2]. | SOURCE: P and D k20 in their recorded workloads. |
| ESTIMATE: s8 rank 5, parent/node + D fusion | The last parent directly verifies child proofs and decides their accumulator in one proof. | Still needs an in-circuit decision of those child KZG obligations; the existing design retains the residue gadget. [R1], [R2], [s8 wave 1](../../s8-decider-settle/wave-1/FINDINGS.md). | ESTIMATE: 316,833-row gadget remains; k19 floor for that implementation. Four-child N plus pairing is approximately 958095+316833=1274928 advice rows before glue: k21, not k15. Tables/reconfiguration can change this sum. |
| ESTIMATE: **C1 direct fixed-pair terminal** | One witness supplies both transition images; original transition equations, linkage, ordered statement and call opening are constrained directly. | No child or parent proof exists inside this relation. In-circuit Poseidon remains. Outer Blake2b and native KZG pairing remain outside the circuit. [R1], [pinned stdlib](../../../raw/s7-recursion/wave-2/stdlib-2.3.5-public.md). | ESTIMATE: k15 target, k16 fallback. Neither the recursive BLS table extent nor the pairing rows are inherited automatically. Use only the stock chips actually required by the direct relation. |

**ESTIMATE — s8's slowdown is a useful control, not a refutation of C1.** Historical N+D proving is `867.279750+586.381690=1453.661440 s`. A same-width fused k21 proxy costs `867.279750×2×21/20=1821.287475 s`, **25.29% slower**. It still processes proofs. C1 instead replaces the proof-verification workload with the application equations. Removing the BLS in-circuit chip does not change the outer proof's BLS12-381 curve. [R2], [model-results.json](model-results.json).

## Auditable k15 / k16 row budget

**SOURCE — What is actually recorded:** A102 k13, A80 k12, 253 instance lanes each, program width P=86, state width S=81, and 3,440-byte child proofs. Exact advice/table/column/degree counts are **not recorded**. The seven unusable rows are from the historical Fenrir model configuration, not a measured C1 configuration. [R1], [R2].

**ESTIMATE — The table below decomposes Astra-2's 8,192-row glue allowance into engineering reservations.** Neither the child capacities nor the glue reservations are measured C1 rows. They are budgets to falsify by component synthesis at both k values. Shared assignment, compiler changes, range widths and chip configuration can change every nonzero entry. Do not subtract rows for shared cells until measured.

| Component | Evidence and accounting | k15 planned advice rows | k16 planned advice rows |
|---|---|---:|---:|
| A102 semantic fragment | SOURCE: k13 only [R1]. ESTIMATE: capacity allowance `8192−7`; preserve its original equations. | 8,185 | 8,185 |
| A80 semantic fragment | SOURCE: k12 only [R1]. ESTIMATE: capacity allowance `4096−7`. | 4,089 | 4,089 |
| Link, context, enables, envelopes, heights | SOURCE: shared program/middle state, two context pairs, (3,1)/(0,0), 64-bit endpoints [R1]. ESTIMATE: reservation. | 1,024 | 1,024 |
| Program/state commitments and ordered trace | SOURCE: one program, three states and 21-field trace [R1]. ESTIMATE: reservation. | 4,096 | 4,096 |
| Statement digest | SOURCE: Poseidon over `SHK_STMT, instance_id, batch_no` plus 16 fields = 19 inputs [R1]. ESTIMATE: reservation. | 1,024 | 1,024 |
| Communication opening, 15-slot template and glue reserve | SOURCE: Poseidon(comm_rand, stmt); bind slots 0/1/10 and all constants [R1]. ESTIMATE: reservation, including assignment/layout overhead. | 2,048 | 2,048 |
| Outer Blake2b transcript | SOURCE: host transcript parameter, not a relation gadget [pinned stdlib](../../../raw/s7-recursion/wave-2/stdlib-2.3.5-public.md), [R1]. ESTIMATE: zero circuit rows; **nonzero host time remains**. | 0 | 0 |
| Recursive verifier / pairing | ESTIMATE: absent by C1 construction; zero rows. External ledger pairing remains. | 0 | 0 |
| **Total** | ESTIMATE: `8185+4089+8192`. | **20,466** | **20,466** |
| Usable capacity | ESTIMATE: `2^k−u`, provisionally u=7. | 32,761 | 65,529 |
| Remaining advice headroom | ESTIMATE: capacity minus budget. | **12,295** | **45,063** |
| Total possible glue if child allowances hold | ESTIMATE: capacity minus 12,274. | 20,487 | 53,255 |
| Actual table extent, columns, degree | OPEN QUESTION: separately resynthesize; no transfer of N's million-row table. | Unknown | Unknown |

**ESTIMATE — A hash audit makes the reserve checkable:** for the historical P=86/S=81 fixture the input lengths are `87 + 3×82 + 21 + 19 + 2 = 375` field elements across seven hashes. This is an input count, **not 375 constraints or rows**. `keys_id` is derived from pinned old child VK identities and shape, then assigned as a constant; it is not another witness SHA-256 circuit. Use the [pinned Poseidon implementation](../../../raw/s7-recursion/wave-2/poseidon-7.2.4-public.md) when measuring padding/permutations. Changing P/S requires recounting. [R1].

**ESTIMATE — Fit rule:** `max(advice_extent, table_extent, other constrained row extents)+u ≤ 2^k`. Do not sum advice and table extents as if they occupied disjoint rows. With glue fixed at 8,192, the child allowance may expand by at most `(32761−8192)/12274=2.0017×` at k15 or `(65529−8192)/12274=4.6714×` at k16. These margins are not proof that the selected stock architecture fits; column/degree growth can erase the time advantage even when rows fit. [model-results.json](model-results.json).

## From row fit to the 10× test

**ESTIMATE — Wave-1 model:** `R=T1/T0=h+(1−h)q`, where q is 1/32 at k15 or 1/16 at k16 and h is the unchanged time share. Solving `R<0.1` gives `(0.1−q)/(1−q)`: **7.096774% / 4%**. Equality gives exactly 10×. More generally use `R=h+(1−h)αq+δ`, with α capturing width/degree/backend cost and δ new overhead normalized by T0. At k15/h=6%, α must be below 1.3617 when δ=0. At k16/h=4%, there is no positive overhead margin. These are screens; the two original k20 proofs and child work must be phase-weighted in a complete-chain model after C0.

| Geometry / assumed h | ESTIMATE gain | ESTIMATE A3 check-wall proxy, 315.880–382.278 s |
|---|---:|---:|
| k15, 2% | 19.75× | 15.99–19.35 s |
| k15, 6% | 11.19× | 28.23–34.17 s |
| k15, 10% | 7.80× | 40.47–48.98 s |
| k16, 2% | 12.31× | 25.67–31.06 s |
| k16, 4% | 10.00× | 31.59–38.23 s |
| k16, 6% | 8.42× | 37.51–45.40 s |

**SOURCE — Those A3 values are check intervals including modeling/key work and verification, not isolated warm proving times.** An older receipt reports 882.589555 s with different lineage. A4 success at 324.739 s and later failure at 2.201 s cannot be summed with unrelated A3 runs to manufacture a baseline. [R1], [R3]. **ESTIMATE:** E1 reports `T0/T1` from identical logical inputs, scheduling and cache boundaries. Include witness creation, all necessary proofs, packaging and independent local ledger verification. Report stagenet latency separately as `(T0+S0)/(T1+S1)`. If settlement is unchanged, a proof-only 10× gain gives less than 10× finalization acceleration.

## Ranked implementation decisions

**ESTIMATE — Cost and gain ranges below are conditional planning scenarios, not measured confidence intervals.** All candidates can fail or regress. Hardware is 20 cores, 90 GB RAM, RTX 5090 32 GB, CUDA 13.x; CPU qualification comes first.

| Rank / idea | Mechanism | Evidence (raw paths) | Bounded expected gain vs recorded baseline, arithmetic | Engineering cost | Risk to ledger-9 acceptance | Experiment on this host |
|---|---|---|---|---|---|---|
| 1. C1 at k15 | ESTIMATE: compose the two transition witnesses; remove recursive proof machinery. | SOURCE: [R1], [R3], [pinned stdlib](../../../raw/s7-recursion/wave-2/stdlib-2.3.5-public.md). | ESTIMATE: 11.19–19.75× A3 sizing proxy for h=.06–.02, `1/(h+(1−h)/32)`. No measured chain gain. | ESTIMATE: 2–4 engineer-weeks for fixed fixture, excluding C0 and live migration lead time. | OPEN QUESTION: semantic fragment equivalence and new registered key; medium integration risk. No ledger protocol change planned. | ESTIMATE: E1 component census, E2 adversarial/ledger qualification, E3 five paired CPU runs; GPU only as a separately timed variant. |
| 2. C1 at k16 fallback | ESTIMATE: use extra capacity for faithful binding/fragment integration, without dropping checks. | SOURCE: [R1], [R3]; ESTIMATE: budget above. | ESTIMATE: 8.42–12.31× at h=.06–.02, `1/(h+(1−h)/16)`; 10× only at h≤.04 under this model. | ESTIMATE: same key family workflow; 1–3 extra days to qualify a second geometry. | OPEN QUESTION: same semantic/registration risks; fitting k16 is not a 10× pass. | ESTIMATE: E1 measures both k values; E3 chooses complete measured latency, not lower nominal k alone. |
| 3. Parent/node + D fusion control | ESTIMATE: retain child proof verification, remove the separate parent proof. | SOURCE: [R2], [local decider](../../../raw/s7-recursion/wave-2/local-c1-audit.md). | ESTIMATE: N+D proxy 1.676× if fused cost=N; 0.798× if N×2.1 (25.29% slower). This is recl9, not measured Sherlock. | ESTIMATE: 2–4 weeks; outside E1. | OPEN QUESTION: new key, all child obligations and pairing remain; domain risk. | ESTIMATE: geometry-only rejection screen if C1 witnesses unavailable; no expensive control proof needed for E1. |
| 4. More accumulation/deferred pairing | SOURCE: existing parent already prepares, accumulates once and defers. | SOURCE: [R1], [accumulation paper](../../../raw/s7-recursion/wave-2/accumulation-paper.md), [KZG code](../../../raw/s7-recursion/wave-2/kzg-accumulation-code.md). | ESTIMATE: **1.00× credited** for repeating an existing transformation. Unmeasured duplicate removal is a separate hypothesis. | ESTIMATE: <1 day to confirm call counts in C0. | SOURCE: deleting the final decision while retaining opaque proofs leaves an unchecked obligation. | ESTIMATE: C0 call-count audit; do not add this as a second optimization to C1. |
| 5. Folding / tree redesign | ESTIMATE: future many-step or opaque-proof workload, not needed for two available transitions. | SOURCE: [Nova](../../../raw/s7-recursion/wave-2/nova-current.md), [R2], [Loki-07](../../../raw/s7-recursion/loki07-prior.md). | ESTIMATE: **zero gain credited** to E1. For historical recl9, folding+compression+wrapper must total ≤161.729 s to beat 1617.292 s by 10×; a 586.382 s wrapper alone caps gain below 2.758×. | ESTIMATE: 8–16+ weeks for a complete bridge; separate proposal. | SOURCE: Nova lists Pallas/Vesta, BN254/Grumpkin and secp/secq, not BLS12-381; its output is not ledger-9 PLONK. OPEN QUESTION: full wrapper cost. | ESTIMATE: synthesize final wrapper before folding benchmarks. Actual 16 tx means four leaves, not sixteen proof leaves. No paper benchmark is transferred. |

## What a new registered key costs

**SOURCE — New circuit, new key:** the stock reader reconstructs the constraint system from serialized architecture and k. The current A4 test enforces four separate deployment pins: address, terminal VK hash, parent VK hash and relation hash. [R1], [pinned stdlib](../../../raw/s7-recursion/wave-2/stdlib-2.3.5-public.md). The [official SRS catalog](../../../raw/s7-recursion/wave-2/official-srs-catalog.md) matches the local k20 file hash; [pinned parameter source](../../../raw/s7-recursion/wave-2/params-0.8.3-source.md) truncates monomial powers and recomputes Lagrange powers for smaller k. **ESTIMATE:** authenticate that official source, derive the supported k15/k16 parameters and record provenance; a new trusted-setup ceremony is unnecessary. Cold downsize/VK/PK costs remain to be measured.

**SOURCE — Smaller official files already have published pins:** k15 is **6,291,844 bytes**, SHA-256 `724c7c3d779148bb113c7ee9c034b2f27db16e6bdf315fde90105a9bad00b1de`; k16 is **12,583,300 bytes**, SHA-256 `09c877216d6589b370263e18af40a030a901b41a7a7c37ef58c9901db41f05c6`. [Official catalog](../../../raw/s7-recursion/wave-2/official-srs-catalog.md). **ESTIMATE:** a verified catalog file can avoid repeatedly loading/downsizing the 201,326,980-byte k20 file; this is setup/I/O work, not an extra proven steady-state speedup. No SRS file was downloaded in this wave.

| Operational item | Cost / acceptance work |
|---|---|
| ESTIMATE: new key family | Generate and authenticate relation/VK/PK, public template, source-semantic pins and SRS derivation. Change cache identities; never load the old D key for C1. |
| ESTIMATE: deployment | Budget a fresh stagenet deployment/registration of `acceptValidity` with the new verifier, plus independent state/key readback. An in-place key update is not assumed. Current [ContractOperation docs](../../../raw/s7-recursion/c1-contract-operation.md) describe versioned verifier keys, but are v8.1.3 documentation, not proof of the pinned ledger-9 migration API. |
| ESTIMATE: fixture and client re-pin | Update `crates/mori-recursion/tests/fixtures/stagenet-verifier.json` only from successful readback. Introduce an explicit verifier-family schema: recursive-parent versus direct-fixed-pair. Preserve recursive parent pins in the old variant; require semantic-source/shape pins in the new variant. Update strict fixture deserialization, caller address/key routing and the A4-equivalent check together. |
| SOURCE: A4-class mismatch risk | [R1] records a historical A4 hypothesis about archived parent `afc4ec3a…` versus current `801467c8…`, and a separate corrected-template run. These show what must be diagnosed; they do not establish the cause of the latest missing trace. Wrong parent family, relation bytes, template constant, address, stock architecture or cached key must fail before proving where possible. |
| ESTIMATE: state continuity | Preserve existing wallets, keys and contract state. A fresh address does not inherit predecessor root, height, instance namespace, accepted statements or replay history. Specify the new deployment's initial state and prevent routing/replaying old statements across deployments. Do not claim atomic migration without its protocol. |
| OPEN QUESTION: operational price | Actual DUST, wallet synchronization, fee-proof count, deployment/finality latency and maintenance authority requirements are unmeasured. Report them separately from steady-state C1 proving. Plan 2–5 additional engineer-days for adapter/re-pin/readback work, excluding network waiting and any state migration protocol. |

## Verified, Consumed and rejection obligations

**ESTIMATE — Required semantics:** C1 proves the two transition equations, not possession of original child proof bytes. Its manifest must explicitly identify this relation family. `Verified` requires independent stock ledger verification of the final proof and exact typed public statement/key provenance. `Consumed` records exactly the ordered A102 then A80 coverage for one aggregate. It does not mean a chain spend has finalized. No deferred accumulator may survive the direct-family result. The anchor still checks predecessor root/height, instance/program/context pins and replay rules before settlement. [R1], [Loki-05](../../../raw/s7-recursion/loki05-prior.md).

**ESTIMATE — Equivalence boundary:** constrain both enables to one, original transition equations, shared program, A102.post=A80.pre, pinned contexts/envelopes, 64-bit heights, count/work, all 16 statement fields and their ordered trace, instance/batch and communication opening. Mutations must reach constraints, not merely host refusals. Preserve the existing scope of the two binding lanes: committed values, not newly proved SHA-256 source claims. A different binding lane with a consistently different valid statement is not automatically a false transition. Claim no full-39 interpreter coverage or new wallet-witness availability. [R1], [R3].

## What changes the verdict

- **OPEN QUESTION — E1 actual geometry:** k>16, oversized tables or expensive width/degree expansion refutes the current sizing case. A measured k15/16 fit supports implementation but does not prove 10×.
- **OPEN QUESTION — Equivalence:** any missing original equation, unconstrained communication opening, invalid-transition acceptance or unavailable witness blocks promotion regardless of speed.
- **OPEN QUESTION — C0/E3 timing:** h above the thresholds, material δ, or matched complete-chain gain below 10 refutes the tenfold claim. Report a smaller valid gain without dropping obligations.
- **OPEN QUESTION — Ledger qualification:** stock reader failure or inability to register/read back the exact new key blocks deployment. Local verification alone cannot establish installed-key compatibility or settlement.

**SOURCE — Work completed in this wave:** eight leads in [PLAN.md](PLAN.md); public Scrapling fetches and both requested AgentBus helpers; direct source/inventory reads; [model.py](model.py) arithmetic. The upstream `stdlib/src/lib.rs` fetch returned 404; pinned docs.rs source replaced it. One parameter-source URL redirected to a crate page; the `/crate/.../source/` URL supplied the actual code. AgentBus wait connections failed intermittently; the Nova source was fetched directly. No synthesis, proving, installation, implementation edit, deployment, commit or push occurred.

[R1]: ../../../raw/s7-recursion/wave-2/local-c1-audit.md
[R2]: ../../../raw/s7-recursion/wave-2/private-inventory-summary.md
[R3]: ../../../raw/s7-recursion/astra2-prior.md
