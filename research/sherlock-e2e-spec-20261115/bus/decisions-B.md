# Slice B decision drafts (for architecture/decisions/)

Sender: Fable-B. MADR shape of the profile template. IDs D-030 to D-051 from the reserved range D-030 to D-069. Each record binds only through the EARS IDs and checks in its Confirmation. Authority uses the three labels of the brief. Hard rules are D-001 to D-015 and D-018 (Fable-A); slice B sends only confirmation additions for them, at the end of this file.

<a id="d-030"></a>
# D-030: STARK layers run WHIR in unique decoding at five frozen parameter sets

- Status: accepted
- Legacy IDs: OD-4, OD-19, OD-24 option E, OD-18

## Context and Problem Statement

Every layer on the chain of a call needs at least 100 bits of proven soundness (D-003) while no BLS12-381 circuit exceeds k = 21 (D-001) and the ledger verifier stays unchanged (D-005). The STARK parameters decide proof size, wrap rows and what can be proved from theorems as stated.

## Decision Drivers

- D-003 (100 bits, no unproved conjecture)
- D-001 (wrap rows at k = 21)
- D-005 (ledger-readable wrap key)

## Considered Options

- Unique decoding on every layer; app, leaf-aggregation, internal, `L` and `L_Fr` sets as in `frozen.rs` (chosen)
- List decoding with two OOD samples (OD-1 to OD-3): fewer queries, needs bounds that are not proven as stated
- A G4 wrap directly over `L` (OD-24 options A, C): ledger 9 refuses the key (M7-0)

## Decision Outcome

Leaves and nodes are STARKs over BabyBear with `EF` of degree 4; `L` is BabyBear with Midnight Poseidon over Fr; `L_Fr` is a STARK over BLS12-381 Fr with no extension. WHIR, `k_whir` 4, unique decoding in every round. Sets: APP 1/193-88-81-81, LEAF_AGG 2/118-84-81, INTERNAL 3/97-82-81, `L` and `L_Fr` 3/98-83-82 (openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:81` to `:214`, `:234`).

Authority: OWNER (owner decisions 2026-11-12, `research/sherlock-final-design-20261111/DECISIONS.md:18`, `:33`, `:35`).

### Consequences

- Good: Every term rests on a proven result; one wrap of 1,685,845 rows at k = 21 (MEASURED, T10m).
- Bad: Unique decoding costs queries: larger STARK proofs and the added `L_Fr` stage (105.0 s MEASURED before the GPU work).

## Confirmation

LFP-001, NAG-001, LTR-001, LTR-002, SSG-002 by gate V.10 and the dependency check (observed for V.10 on the T10m keys; planned for T25). SSG-001 and RKY-005 by `fp8_d3` (observed on 8 chain keys, weakest 101.0110; planned for the R_SL app keys). SQG-002, SQG-003.

## More Information

### Reconsideration

A wrap-diet point that moves the wrap under the 85 percent gate at another setting (LOOP O3); a proven list-decoding bound; a change of the ledger verifier.

<a id="d-031"></a>
# D-031: OpenVM stark-backend with its CUDA prover is the STARK engine

- Status: accepted
- Legacy IDs: owner 2026-11-11; SPL-007; OD-11

## Context and Problem Statement

Leaves, nodes and `L` need one prover with a GPU path and a recursion circuit whose key does not depend on the child program.

## Decision Drivers

- D-004 (one universal key set: the recursion circuit reads the child DAG as a cached trace)
- GPV-001 to GPV-005 (GPU paths)

## Considered Options

- OpenVM stark-backend v2.1.0 with the CUDA prover (chosen)
- Plonky3 0.9.0-rc.2 with Plonky3-recursion (second choice)

## Decision Outcome

OpenVM stark-backend with `openvm-cuda-backend`; local changes are kept as exported patch series (LOOP T0).

Authority: OWNER ("openvm seems to be the winner", TERMINOLOGY.md, STARK engine row; OD-11: no second engine).

### Consequences

- Good: One engine for five layers and both devices.
- Bad: The prototype carries 14 patch series against upstream (9 openvm, 5 stark-backend).

## Confirmation

GPV-001, GPV-003, GPV-004 by gates `sg6` and `lfr-gpu` (observed on the T10m keys); GPV-002 and GPV-005 (planned).

## More Information

### Reconsideration

An upstream release that breaks the cached-trace recursion; a GPU path that cannot pass G3.

<a id="d-032"></a>
# D-032: One rekey from one list regenerates L, L_Fr and the wrap key

- Status: accepted
- Legacy IDs: T25; RKY-001; the single rekey list

## Context and Problem Statement

Nine or more changes alter a universal key (NT-1 to NT-3, typed MEM_BUS, R_SL chip set 3 and kits, T11 blinders, FP-3, slot 15, range checks). Each keygen invalidates every gate, profile and deployed contract.

## Decision Drivers

- D-004
- cost of a keygen and of the gate reruns (UKS-006)
- Preview redeploy after T12 (LOOP lead decision)

## Considered Options

- One rekey from `~/.cache/rekey-t11/LIST.md` after every input row is committed (chosen)
- A rekey per change: each one reruns the T10m, T10q and T8b gates and stales every box profile

## Decision Outcome

Row T25 runs once. A change that alters a key joins the list first (RKY-002). The runner writes a key manifest (UKS-004) and reruns every gate (UKS-006).

Authority: LEAD PICK confirmed (owner 2026-11-15 ~09:10, "the single rekey list", `~/Projects/Moriarty/llm-wiki/wiki/log.md:4107`).

### Consequences

- Good: One gate rerun, one profile rerun per box, one redeploy.
- Bad: Every late finding that touches a key delays T25; the list has no row yet for FP-3, slot 15, the range checks, LFP-010 to LFP-012, UKS-003 and the era-check words of EGV-005 (UKS-010).

## Confirmation

RKY-001, RKY-002, UKS-004, UKS-005, UKS-006, UKS-009, UKS-010, STM-008 (all planned).

## More Information

### Reconsideration

A defect found after T25 that changes a key: a second rekey is recorded as such (RKY-001 scenario).

<a id="d-033"></a>
# D-033: Every AIR is required and prog_d with key_root replaces the KeyBind chain

- Status: accepted
- Legacy IDs: O10 (FP8 picks 1 to 7), T3, T7

## Context and Problem Statement

A recursion layer keyed with optional AIRs lets a prover omit a proof-shape AIR. The KeyBind public-value chain bound the keys but narrowed GP-3 and GP-4.

## Decision Drivers

- D-008 (GP never narrowed)
- D-004

## Considered Options

- All AIRs required at every layer; `prog_d` in the leaf statement; `key_root` opened in N1 (chosen)
- Keep the KeyBind chain: narrows GP-3 and GP-4

## Decision Outcome

Every recursion layer is keyed with all AIRs required. N1 opens `prog_d = P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` and checks each child key by inclusion in `key_root`.

Authority: OWNER (2026-11-13: "go with your recommendations on O1 to O12"; O10 withdraws the chain; LOOP rulings table).

### Consequences

- Good: Mixed chip sets and FV units pass under one `key_root`.
- Bad: Each program pays one Merkle path per child key in N1.

## Confirmation

NAG-002 by gate V.9 (observed, 70 cases); NAG-003, NAG-004, LFP-006, LFP-007 by `check-chain.sh cases` (observed on the T10m keys); LPC-018 (planned).

## More Information

### Reconsideration

A proof shape that needs an optional AIR.

<a id="d-034"></a>
# D-034: Path C checks the child DAG key in L and L_Fr under the same L key

- Status: proposed
- Legacy IDs: T7c; I-17 path C; lead ruling 2026-11-14

## Context and Problem Statement

A one-leaf call has no universal node to check the leaf key against `key_root`.

## Decision Drivers

- D-004 (one `L` key for both paths)
- LTR-003 (`L` has 2,000 columns)

## Considered Options

- `L` proves the inclusion of the DAG leaf with one more public value `X`, and `L_Fr` equates `X` with the cached-trace commitment (chosen)
- A second `L` key for path C: breaks D-004
- Always add one internal node: slower one-leaf calls

## Decision Outcome

As built in T7c (openvm 3607e95): one universal `L` and `L_Fr` key serves path C and the full tree; the DAG leaf sits at `KEYDAG_IDX`.

Authority: LEAD PICK for T15 (lead ruling 2026-11-14, ARCHITECTURE.md section 1.6; no owner line names it).

### Consequences

- Good: One-leaf Compact calls skip the node stage.
- Bad: `L` is at 1,995 of 2,000 columns; the DAG leaf index collides with the planned R_SL index (D-046).

## Confirmation

LTR-007 by gates `pathc` and `pathc-d3` (observed, 25 of 25); UKS-003 (planned).

## More Information

### Reconsideration

`L` over 2,000 columns after T25; the owner rejects the pick at T15.

<a id="d-035"></a>
# D-035: T8b picks: keys_d with 7 rows and the MERGE-WORK node modes

- Status: accepted
- Legacy IDs: T8b picks; N-7; N-8

## Context and Problem Statement

GP-4 needs work units that merge in any shape under universal keys.

## Decision Drivers

- D-008
- D-012 (5,400 columns)

## Considered Options

- `keys_d` with 7 node rows (rows 4 to 6 are NW, NW2 and N3 over NW2), chip set 2 per-program leaf key, constant `vd` form, kernel step words as columns (chosen)
- A separate key set for work units: breaks D-004

## Decision Outcome

As built in T8b and merged in T10m (openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:88`).

Authority: OWNER (owner rulings 2026-11-14, T15 picks confirmed: "T8b picks (keys_d 7 rows, chip set 2 per-program leaf key, constant vd form, kernel step words as columns, N-7, N-8)").

### Consequences

- Good: gp2 48 of 48 and x7 22 of 22 on the stock verifier.
- Bad: A change of the row count rekeys `L` and `L_Fr`.

## Confirmation

UKS-002, GPH-002, GPH-005 by gates `gp2` and `x7` (observed on the T10m keys); GPH-007 (planned floor).

## More Information

### Reconsideration

STACKS and VALUE DIGEST need another row or more than 5,400 columns (D-012).

<a id="d-036"></a>
# D-036: X14 work units use a 17-bit range checker and a 3,300 column bound

- Status: accepted
- Legacy IDs: O1, O2

## Context and Problem Statement

The FV work units of GP-3 do not fit the 16-bit RANGE table without 3 constraints per row.

## Decision Drivers

- D-008 (GP-3)
- SQG-002 (shape bounds)

## Considered Options

- 17-bit range in the work units only; 3,300 columns for FV work units (chosen)
- The 16-bit table everywhere: 3 constraints per row

## Decision Outcome

As ruled.

Authority: OWNER (O1, O2; 2026-11-13).

### Consequences

- Good: FV units fit their bound (add unit 3,138 MEASURED).
- Bad: Two range widths in the design.

## Confirmation

GPH-003 by the FV unit case and `pathc-d3` on `fv_vk.bitcode` (observed on the T10m keys).

## More Information

### Reconsideration

A work unit over 3,300 columns.

<a id="d-037"></a>
# D-037: Option A (the L_Fr stage at log blowup 3) for the prototype key freeze

- Status: accepted
- Legacy IDs: O3; E4

## Context and Problem Statement

E4 offered option A (measured end to end) and option B (log blowup 2, 12.4 s less STARK time, wrap over the 85 percent gate).

## Decision Drivers

- D-001
- measured call time on huginn

## Considered Options

- A (chosen)
- B: wrap of 1,927,450 rows, over the gate, no key

## Decision Outcome

A for the T12 prototype freeze; B is revisited with the wrap-diet settings after the prototype.

Authority: LEAD PICK confirmed (owner 2026-11-14 12:13 "you decide"; LOOP lead decisions).

### Consequences

- Good: Keeps 96,728 rows under the gate.
- Bad: 12.4 s of warm STARK time left on the table.

## Confirmation

LPC-005 timing on the T25 keys (planned); RKY-003 (planned, slice C).

## More Information

### Reconsideration

Wrap rows under the gate with a real key at option B.

<a id="d-038"></a>
# D-038: FP-3: L binds call lanes to guest effects through a forward chain

- Status: proposed
- Legacy IDs: FP-3; ANC-018; ANC-019; NT-1 residual

## Context and Problem Statement

After NT-1 to NT-3 a shifted anchor write still passes, because `L` does not bind the call-vector lanes to what the guest wrote, and `k` is not tied to the slice count.

## Decision Drivers

- D-011 (K_MAX = 2)
- D-013 (lock form)
- D-021 (slot 15)

## Considered Options

- Bind each effect lane to the guest effect word and chain the slices forward, `C_j = vd(Cons(rec_j, C_(j+1)))` (chosen)
- Check the lanes in the wrap: more wrap rows, against D-001 headroom

## Decision Outcome

The form of openvm `fp3` (d2abc38 plus the untracked `fp3.rs`); it joins the single rekey.

Authority: LEAD PICK for T15 (no owner line names FP-3; the lock operand extension was owner-confirmed 2026-11-15).

### Consequences

- Good: Closes `nt1_residual_shifted_write_is_fp3`; unblocks the t10q push.
- Bad: Adds `L` constraints with 5 columns of headroom (LTR-003); not committed; one test passes without showing a refusal (CLB-003).

## Confirmation

CLB-002, CLB-003, ANC-018, ANC-019 by `nt1_residual_shifted_write_is_fp3`, `fp3_bindings_refuse`, `fp3_k_is_the_true_slice_count` (planned; before-log fails, after-log passes 12 of 12).

## More Information

### Reconsideration

`L` over 2,000 columns; a Carbon Quint lane finds a forgery.

<a id="d-039"></a>
# D-039: Blinder lifetimes and the test-seed guard

- Status: accepted
- Legacy IDs: T11-1, T11-4

## Context and Problem Statement

The chain driver sets `r_P = 1000 + i`; seeded paths exist for tests.

## Decision Drivers

- D-002

## Considered Options

- `r_P` once per deployment, salt per run, blinders per proof, every seeded path behind the cargo feature `test-seed` with exit code 4 otherwise (chosen)
- Fresh `r_P` per call: changes `C_P`, which the deployed contract refuses

## Decision Outcome

As T11-1 and T11-4.

Authority: LEAD PICK confirmed (owner 2026-11-15 ~09:10, `~/Projects/Moriarty/llm-wiki/wiki/log.md:4107`).

### Consequences

- Good: Seeded proofs cannot leave a production build.
- Bad: `deploy.secret` becomes state that a box must keep (ZKE-005).

## Confirmation

ZK-012, ZK-013, ZK-014, ZKE-001, ZKE-002, ZKE-005, ZKE-006 (unit tests observed in lane logs; files untracked; the rest planned).

## More Information

### Reconsideration

A design that rotates `C_P`.

<a id="d-040"></a>
# D-040: Blinders are drawn per proof; no slot pool is built for the prototype

- Status: proposed
- Legacy IDs: T11-2; PRD-002; ZK-016; OD-13

## Context and Problem Statement

OD-13 (owner decision 2026-11-12, `research/sherlock-final-design-20261111/DECISIONS.md:27`) reads "required, local socket, one-use slots"; PRD-002 and ZK-016 describe a pool of one-use blinding slots. The daemon as built draws fresh `OsRng` blinders for each proof and has no pool (Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/serve.rs:114`).

## Decision Drivers

- D-002
- PDM-005

## Considered Options

- Per-proof draw inside the daemon, no pool (chosen)
- Build the pool: precomputed blinding work saves time per call, adds state that can be reused or copied

## Decision Outcome

The prototype keeps the per-proof draw. This record amends the pool obligation of PRD-002 for the prototype: no pool is built. One-use holds because no blinder exists before the proof and none is stored after it. Cross-request isolation is required without condition (PDM-009). The one-use and refusal rules of PRD-002 and ZK-016 stay in force for any pool built later.

Authority: LEAD PICK for T15 (Fable-B, this change). The pick departs from the letter of OD-13 ("one-use slots") and keeps its purpose (no blinder is used twice). OD-13 is a design decision, not a hard rule, and design stays open until the end-to-end prototype (D-015), so the lead may propose it; the owner confirms or reverses it at T15. T11-2 "merge t14b" is owner-confirmed and merged the daemon without a pool.

### Consequences

- Good: No reusable secret state in the daemon.
- Bad: No precomputed blinding; PRD-002 text and the code differ until T15.

## Confirmation

PDM-005 and ZK-015 by `eq_zk` through the daemon (planned); PDM-009 by `slot_reuse` (planned).

## More Information

### Reconsideration

A measured wrap speedup that needs precomputed slots.

<a id="d-041"></a>
# D-041: No PCS masking while every STARK layer runs on owned boxes

- Status: accepted
- Legacy IDs: T11-3

## Context and Problem Statement

The CPU and CUDA STARK backends have no PCS masking (LOOP T11), so STARK proofs are not zero knowledge.

## Decision Drivers

- D-002
- D-010

## Considered Options

- No masking; the wrap is the only hiding layer; STARK proofs never leave the owned boxes (chosen)
- Add masking to both backends: key changes, GPU work, needed only for case B

## Decision Outcome

T11-3 as worded: "no PCS masking while every STARK layer stays on devices the user owns".

Authority: LEAD PICK confirmed (owner 2026-11-15 ~09:10; restated 09:05 "T11-3 stands as worded", `log.md:4113`).

### Consequences

- Good: No verifying key changes for zero knowledge.
- Bad: Case B needs a new design (Veritee); every STARK proof and checkpoint is a secret (ZKE-003, ZKE-004).

## Confirmation

ZKE-003, ZKE-004, ZK-001, ZK-018, BXF-001 (privacy scan observed in T17 (a); enforcement planned).

## More Information

### Reconsideration

Case B is ordered; a STARK proof must be shown to a third party.

<a id="d-042"></a>
# D-042: Box roles: muninn leads and checks, huginn and odin take the heavy loads

- Status: accepted
- Legacy IDs: owner 2026-11-15 (odin); LOOP compute boxes rule; heavy-slot lead decision

## Context and Problem Statement

Three owned boxes of different hardware prove; results must be comparable and trusted.

## Decision Drivers

- D-010
- LPC-005

## Considered Options

- muninn = lead, wiki, wallet, ledger-check authority; huginn and odin = heavy jobs; per-box baselines; heavy-slot 3 slots and 64 GB on muninn (chosen)
- Free routing with cross-box timing claims: not comparable (muninn GPU clock cap 754 of 3,003 MHz)

## Decision Outcome

As the LOOP compute boxes rule and the Finish plan table.

Authority: OWNER (2026-11-15: "add odin to our workflow for heavy loads, make sure we include it in our loop to finish sherlock"; LOOP compute boxes rule). The 64 GB heavy-slot budget and the kept muninn clock cap are lead decisions under the owner's "you decide" of 2026-11-14 12:13; they are parameters inside this record, not separate authority.

### Consequences

- Good: Idle boxes get work the same tick; speed claims stay honest.
- Bad: muninn is a single point for checks and file moves.

## Confirmation

BXF-001 to BXF-004, BXF-007 to BXF-011 (rules observed in use; scripts planned).

## More Information

### Reconsideration

A fourth owned box; muninn stops being the check authority.

<a id="d-043"></a>
# D-043: Orphan GPU-lock holders are ended under a narrow rule

- Status: accepted
- Legacy IDs: owner 2026-11-15 box-orphan-lock rule

## Context and Problem Statement

An orphan sampler held `gpu.lock` on huginn for 7 h 40 min.

## Decision Drivers

- BXF-004

## Considered Options

- Box agents may end a sampler or monitor of an ended job after 10 minutes without progress, bus log first (chosen)
- Wait for the owner: idle GPU overnight

## Decision Outcome

As the owner rule (`~/sherlock/tooling/ORPHAN-LOCK-RULE.md` on huginn).

Authority: OWNER (2026-11-15).

### Consequences

- Good: No lost night.
- Bad: A wrong match could end a live process; provers and `wrapd` are excluded (BXF-006).

## Confirmation

BXF-005, BXF-006 by the bus log (observed rule; no script).

## More Information

### Reconsideration

A kill of a live prover.

<a id="d-044"></a>
# D-044: A row is DONE only with a Quint lane verdict and a counterexample for every finding

- Status: accepted
- Legacy IDs: owner 2026-11-14 (two rules); T13

## Context and Problem Statement

Model findings and code drift apart unless each finding is built on the real code.

## Decision Drivers

- D-003

## Considered Options

- Carbon Quint lane per row, lead reruns, counterexample on real code, regression test per finding (chosen)
- Model-only QA

## Decision Outcome

As the LOOP loop rules.

Authority: OWNER (2026-11-14).

### Consequences

- Good: 18 FIXED and 22 GUARD HOLDS rows are backed by tests.
- Bad: Lanes take over 1.5 h each; 8 rows NOT ATTEMPTED and 1 CONFIRMED without fix remain.

## Confirmation

SSG-003, SSG-004, SSG-005, NAG-006, NAG-007, LPC-004, LPC-016 (planned scripts; record observed).

## More Information

### Reconsideration

The owner changes the QA rule.

<a id="d-045"></a>
# D-045: Stages hand over named records; ProofBundle is the root of a run

- Status: proposed
- Legacy IDs: new; Fable-B

## Context and Problem Statement

The chain driver, the T16 runner and the T23 runner each write their own JSON. Slices A and C need stable names.

## Decision Drivers

- EUF hand-offs (slice A)
- WRP input (slice C)
- BXF-007

## Considered Options

- Named records `LeafProofsReady`, `ProofBundle`, `LfrProofReady`, `BoxResult` and the key manifest, each with paths and sha256 (chosen)
- Keep ad hoc records per runner

## Decision Outcome

The five records are the published language of the proving slice. They carry paths and digests (field `proof_sha256` over the proof bytes), never witness data. The key manifest is the universal key manifest; per-program keys are checked through `key_root`.

Authority: LEAD PICK for T15 (Fable-B with Fable-A and Fable-C on the bus; no owner line).

### Consequences

- Good: One schema check per hand-off.
- Bad: Runner work in t23-suite before the T23 chain.

## Confirmation

LFP-009, NAG-010, LTR-006, LTR-008, UKS-004 (planned).

## More Information

### Reconsideration

A serve API in the daemon replaces file hand-offs.

<a id="d-046"></a>
# D-046: R_SL key leaves get an index that differs from the DAG leaf

- Status: proposed
- Legacy IDs: new; LIST "key_root menu id 3"

## Context and Problem Statement

`KEYDAG_IDX = 3` is the path C DAG leaf (openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:363`); the rekey list plans menu id 3 for R_SL chip set 3.

## Decision Drivers

- D-004
- D-034

## Considered Options

- Give chip set 3 the next free leaf position and keep the DAG leaf (chosen)
- Move the DAG leaf: changes path C, which is tested

## Decision Outcome

The rekey places R_SL key leaves at a position other than the DAG leaf.

Authority: LEAD PICK for T15 (Fable-B).

### Consequences

- Good: Path C and R_SL work together, which every one-leaf Compact call needs.
- Bad: One more rekey list row.

## Confirmation

UKS-003 (planned).

## More Information

### Reconsideration

A key tree layout change at T25.

<a id="d-047"></a>
# D-047: The Compact adapter class is compact-zkir3-kzg-bls12381-blake2b/1

- Status: accepted
- Legacy IDs: O8

## Context and Problem Statement

GP-3 needs a named class for native Compact proofs.

## Decision Drivers

- D-008

## Considered Options

- `compact-zkir3-kzg-bls12381-blake2b/1` (chosen)

## Decision Outcome

The name as picked in T8; no code holds it yet.

Authority: LEAD PICK confirmed (O8 "lead proposes in T8"; AFK picks confirmed, LOOP lead decisions).

### Consequences

- Good: Stable registry key.
- Bad: None until the X8 verifier exists.

## Confirmation

GPH-004 (planned).

## More Information

### Reconsideration

A ZKIR or transcript change in Compact.

<a id="d-048"></a>
# D-048: The prover is a resident daemon on a local socket

- Status: accepted
- Legacy IDs: OD-13; T14b

## Context and Problem Statement

Key and SRS loading dominate a cold call (huginn 113.01 s per process against 71.59 s with the daemon).

## Decision Drivers

- LPC-005
- D-002

## Considered Options

- Resident daemon, Unix socket mode 0600, same user only (chosen)
- One process per call

## Decision Outcome

`wrapd serve` and the STARK worker as built on `t14b`.

Authority: OWNER (OD-13, 2026-11-12).

### Consequences

- Good: Warm calls.
- Bad: A long-lived process that holds keys and device memory.

## Confirmation

PDM-001 to PDM-004 (observed timing on the T10m keys), PDM-006 to PDM-008 (planned).

## More Information

### Reconsideration

A serve API that crosses machines (case B).

<a id="d-049"></a>
# D-049: T25 runs first; the L collection form follows as a recorded second rekey

- Status: proposed
- Legacy IDs: T8c; BAT-009; LPC-003; LOOP Finish plan

## Context and Problem Statement

`acceptCollection` needs `L` over a SET root (GPH-008), which is not built and not in the rekey list. The LOOP Finish plan sets the critical path as T11, FP-3, T12-SLOT15, T24 and the R_SL leaf key into T25, with no T8c input. The batched-write hard rule (D-009) is owned by slice C; T20 fix 1 keeps `acceptRun` for a user's own run and uses `acceptCollection` for operator jobs.

## Decision Drivers

- D-009 (batched writes; not changed by this record)
- D-032 (one rekey from one list)
- LPC-003 (real-proof run of `L` over a SET root)

## Considered Options

- T25 as planned; T8c lands after it as a second rekey that RKY-001 records; the prototype is not complete until GPH-008 passes (chosen)
- Hold T25 for T8c: every row behind T25 (T12 gate, T23) waits on unbuilt work
- Drop the collection form from the prototype: a waiver of a hard-rule item, which needs the owner; not chosen and not proposed

## Decision Outcome

Order only. T25 does not wait for T8c. GPH-008 is a required dependency of BAT for the complete prototype and stays OPEN until its real-proof run. If T8c is committed before the other T25 inputs, it joins the list and no second rekey is needed.

Authority: LEAD PICK for T15 (the order follows the LOOP Finish plan critical path; Fable-B). This record waives no hard rule; Astra-B review R-03 found that the first draft read as a waiver.

### Consequences

- Good: the critical path stays unblocked and the open item is visible.
- Bad: a second rekey, with its gate reruns and profile reruns, is likely.

## Confirmation

GPH-008 and LPC-003 by the real-proof run with refusal cases (planned); BAT-009 (slice C) refuses `acceptCollection` until then (planned).

## More Information

### Reconsideration

The owner orders `acceptCollection` in the first key set; T8c lands before T25.

<a id="d-050"></a>
# D-050: STM is the OpenSpec home of the setup-mode requirements

- Status: proposed
- Legacy IDs: ST, PF, SC of SETUP-SPEC.md; T23 profile path pick

## Context and Problem Statement

The setup-mode EARS live in a research file of branch `t16-elastic`; several hold two obligations in one sentence.

## Decision Drivers

- D-014

## Considered Options

- STM-001 to STM-019 carry ST, PF and SC by id, one obligation each (chosen)
- Copy the 29 statements unchanged: fails the EARS linter

## Decision Outcome

STM is normative for this change; SETUP-SPEC.md is the source. Box profile path `~/sherlock/setup/profile.json`.

Authority: LEAD PICK for T15 (the mapping of ST, PF and SC into STM, Fable-B). The box profile path inside it was confirmed by the owner on 2026-11-15 ~09:10 (`log.md:4107`).

### Consequences

- Good: One home, linted.
- Bad: Two texts until `t16-elastic` merges.

## Confirmation

STM-001 to STM-019 (observed by `profile::tests` and the T16 records where the status reads BUILT; the rest planned).

## More Information

### Reconsideration

The T16 merge rewrites SETUP-SPEC.md.

<a id="d-051"></a>
# D-051: The R_SL statement bindings join the single rekey

- Status: proposed
- Legacy IDs: ERS-006, ERS-007, ERS-012 (slice A findings)

## Context and Problem Statement

The R_SL app proof does not constrain its FP-1 public values, and R_SL has no `prog_d` or `input_d` binding. No recursion proof authenticates a Compact outcome today.

## Decision Drivers

- D-004
- D-032
- T23 done-when

## Considered Options

- Add the three bindings to R_SL AIR 0 and the leaf before T25 (chosen)
- Settle T23 without them: every Compact claim is unauthenticated

## Decision Outcome

LFP-010, LFP-011 and LFP-012 are inputs of T25, as rows of the rekey list.

Authority: LEAD PICK for T15 (Fable-B and Fable-A).

### Consequences

- Good: T23 claims become sound.
- Bad: More work before T25; the 6-or-9 bucket and the leaf pre-hash must be re-probed.

## Confirmation

LFP-010, LFP-011, LFP-012, ERS-006, ERS-007, ERS-012 (planned).

## More Information

### Reconsideration

A different binding point (for example in N1).

# Confirmation additions for the hard-rule records

| Record | Slice B EARS IDs | Check |
|---|---|---|
| D-001 No circuit above k = 21 | SSG-002 (the STARK crates hold no PLONK or KZG circuit, so the wrap is the one circuit with a k) | dependency check in the T25 gate table (planned) |
| D-002 Zero knowledge end to end | ZKE-001 to ZKE-007, PDM-005, BXF-001 | T11 unit tests (observed in lane logs, untracked), `eq_zk` (planned), privacy scan (observed T17 (a)) |
| D-003 Proven 100-bit security | SSG-001, SSG-003, SSG-004, SSG-006 (G1 to G5 and FP-8), SSG-007, LTR-003, NAG-005 | `fp8_d3` (observed, 8 chain keys, weakest 101.0110), T12 gate (planned) |
| D-004 One universal key set | UKS-001 to UKS-010, LFP-002, LFP-008, LTR-007, ELA-002, GPH-001, PDM-006 | rsl probe (observed, 5 calls), `pathc` (observed), T16 runs (observed), T25 manifest (planned) |
| D-007 One wrap over small-field STARKs | LFP-001, NAG-001, LTR-001, LTR-002, SSG-002 | gate V.10 (observed on the T10m keys) |
| D-008 GP-1 to GP-4 never narrowed | GPH-001 to GPH-008, LFP-003 (open narrowing: both hash kits in one program, D-024) | `gp2` 48 of 48, `x7` 22 of 22 (observed); GP case map (planned) |
| D-010 Case A only | BXF-001 (owned-box roster), ZKE-004 (no secret or witness to a host outside the roster), ELA-006, UKS-005 | roster and destination tests (planned) |
| D-011 K_MAX = 2 | CLB-007 | `chunkCap.qnt` with mutant `cap_four` (observed on `t20-batched`); Rust planner test (planned) |
| D-012 MERGE-WORK 5,400 columns | NAG-005, NAG-006, NAG-011 | `check_vk_bounds` (`frozen.rs:434`); 4,988 MEASURED and 5,121 DERIVED with STACKS and the value set (openvm `rekey-chips` dcf1b4c `frozen.rs:166`); 5,401-column refusal check and LPC-002 (planned) |
| D-013 The x_j lock form | CLB-005, CLB-006 | `t10q_nt2_equal_run_tag_splice_refused`, `t10q_nt3_k_change_refused`, `t10q_nt1_ordinary_call_anchor_write_refused` (observed on `t10q-ntfix`, 9 of 9) |
| D-014 Setup mode replaces F-3 | STM-001 to STM-019, ELA-001 to ELA-007, UKS-008 | `profile::tests` (observed), T16 records (observed), `rg` check for the menu code (planned, STM-014) |
| D-015 Design open until the end-to-end prototype | UKS-007, SSG-006 | manifest status field (planned) |
| D-018 Every proof is a fixed-size shard | LFP-005 (ordered AIR height vector per shape), LFP-008 | gate V.10 field `fixed` (observed, chip sets 0 to 2); swapped-height case, chip set 3 shape, LPC-007 (planned) |
| D-021 Anchor slot 15 (slice A record) | CLB-004, CLB-008 | `t12_anchor_slot_15` (observed on `t12-slot15`); constant cross-check (planned) |
