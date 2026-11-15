# Spec audit of the lead checklist, 2026-11-15

Owner order 2026-11-15 (owner AFK): "Audit this checklist to confirm all work items. Make sure the openspec is accurate and we have EARS for each item."

## Inputs and method

- Checklist: `~/.cache/lead/CHECKLIST-20261115.md` (sections A to F). Each compound bullet is split into atomic items: 38 items.
- Rows: `research/sherlock-tested-design-20261113/LOOP.md` on main 0265fb3af (cited as `LOOP:NN`). The lead staged rows T24 (keccak and secp256k1 kits) and T25 (the single rekey) in the main working copy at 09:11 (wiki 09:11). They are not committed. This audit maps A3, A4 to T24 and B1, B2 to T25.
- Repos: Moriarty lane branches, the openvm lane trees in `~/.cache/m7-wrap/src/openvm-*`, the Mycroft worktrees, `~/.cache/rekey-t11/LIST.md`, the box bus `~/.cache/box-bus/all.log`, and the wiki log `llm-wiki/wiki/log.md` (cited as `wiki HH:MM`).
- Time of the evidence: 2026-11-15 09:15 MST, with a refresh at 09:40 for the rows that moved during the audit (A1, A3, A4a, A7b, D1, M6, T24, T25).

## Base branch

This branch `spec-audit` starts at `t10m` 3dc9001a9, not at main. The governing change `sherlock-processor-final-20261111` exists only on `tested` and its descendants. Main holds an older `sherlock-modular-leaf-20261111` and no `mycroft/` tree. The OpenSpec tree of `t10m` is byte-equal to `tested`. Commit 1 adds new files only (this file and the new change), so it applies to main without conflict. Commit 2 holds the in-place fixes.

## Results

- Items confirmed real: 38 of 38. Status claims checked: 38. Status wrong or stale in the checklist or its records: 4 (A8b, A8c, E1 wording, D1). See column "Status verified".
- Items missing from the checklist: 15 (section "Missing from the checklist").
- Spec requirements fixed in place: 15 (section "OpenSpec fixes").
- New change `sherlock-completion-20261115`: 9 specs, 62 EARS requirements, each with a scenario and a verification line. `openspec validate --strict` passes for it, for `sherlock-processor-final-20261111` and for `sherlock-modular-leaf-20261111`. The EARS linter of the `ears` skill passes with `--require-ids --strict` (0 defects, 0 warnings).
- `dw validate` and `dw ears` do not apply: the repo keeps no drwatt `requirements.json` for Sherlock. The statements use upper-case `SHALL` for OpenSpec, which the drwatt grammar rejects.

## Checklist items

| # | Item | LOOP row | Status verified | OpenSpec location | EARS IDs | Gap |
|---|---|---|---|---|---|---|
| A1 | R_SL leaf key universal for every call (odin r10 on 6f773b2) | T11, T23 (LOOP:33, 45) | yes, DONE at 09:40: LIST.md records the odin r10 probe on 6f773b2 MEASURED, `n1_pre_hash_equals_universal_n1` true for a05, t02, s01 (6 AIRs), d04 and z02 (9 AIRs); r8 had FALSE for a05 (wiki 08:46) | completion `sherlock-rsl-chipset`; processor-final CF-005 (fixed) | RSL-001, RSL-002 | no wiki log entry yet; keccak programs have 8 app AIRs, so RSL-002 needs a recheck with the T24 kits |
| A2 | R_SL app-level prove of all 142 suite calls | T23 (LOOP:45) | yes, open: only execution is MEASURED (odin r5, 142/142 exec equal, LIST.md rekey-chips table); no prove run | `sherlock-rsl-chipset` | RSL-003 | the checklist word "prove" must not be read as done from the exec result |
| A3 | keccak kit (1 call, s02) | T24, T23 | yes, BUILT at 09:40: openvm `rekey-chips` bbf6ad3, c9601df, 699539c (KECCAK_REQ plus p3-keccak-air, test `rsl_keccak_wrong_request_key_refused`); no MEASURED run on file; row T24 | `sherlock-rsl-chipset`; CF-005 | RSL-004 | no LIST.md row |
| A4a | secp256k1 kit (2 calls), R_SL side | T24, T23 | yes, BUILT at 09:40: openvm `rekey-chips` c281a5f (`tests/rsl_secp.rs` against k256 0.13.4); no MEASURED run on file; row T24 | `sherlock-rsl-chipset`; CF-005 | RSL-005 | no LIST.md row |
| A4b | Mycroft frontend lowering of secp256k1 | T18 (d), T23 | yes: `mycroft/compiler/src/zkir.rs:117` refuses every input type other than `Scalar<BLS12-381>` (`FeRefusal::InputType`) | completion `sherlock-compact-frontend`; Mycroft MYC-BLT-007 (stale, note M1 below) | CF-013, CF-014 | MYC-BLT-007 still routes secp256k1 and keccak to a tier 3 kit |
| A5a | T11 blinders: `r_P` per deployment, salt per run | T11 (LOOP:33) | yes, open: T11 RESULTS section 1 (untracked in `~/Projects/Moriarty-wt/t11-zk/research/sherlock-t11-zk-20261115/`); `blind.rs` untracked in openvm `t11-zk`; pick T11-1 owner-confirmed (wiki 09:04) | processor-final ZK-003 (fixed); completion `sherlock-zero-knowledge` | ZK-012, ZK-013 | nothing committed |
| A5b | test-seed guards | T11 | yes, open: section 4 RESULTS-PENDING; `zkguard.rs` untracked; pick T11-4 owner-confirmed | ZK-003 (fixed) | ZK-014 | |
| A5c | EQ-ZK | T11 | yes, open: section 3 RESULTS-PENDING | ZK-003 scenario EQ-ZK | ZK-015 | |
| A5d | slot reuse | T11 | yes, open: section 5 RESULTS-PENDING; T11-2 merged t14b (Moriarty 31e25cc0c) | `sherlock-prover-daemon` PRD-002 | ZK-016 | |
| A5e | GPU half on odin | T11 | yes, open: odin queue "T11 GPU half" behind r10 (bus 16:11Z); section 7 RESULTS-PENDING | ZK-003 | ZK-017 | |
| A5f | ZK-L2 done (71c4c001b) | T11 | yes: Moriarty `t11-zk` 71c4c001b (`ZkPadding.lean`, 0 sorry, axioms within the three); not merged into `t10m` | ZK-010 (fixed: was "open") | ZK-019 | merge pending |
| A6 | FP-3: `L` binds call lanes to effects, forward chain, `k` tied to slice count | T10q (LOOP:32) | yes, open: openvm `fp3` d2abc38 adds the failing tests; `crates/sherlock-lfr/src/fp3.rs` untracked; the t10q push waits (wiki 06:58) | completion `sherlock-anchor-chunks` | ANC-018, ANC-019 | no LIST.md row although FP-3 changes the `L` key |
| A7a | T12-SLOT15: anchor 0 -> 15 in `decode.rs`, `iface.rs`, SherlockLedger | T12 (LOOP:34) | yes, open: openvm `t12-slot15` has `iface.rs` dirty (ANCHOR_SLOT = 15, test `t12_anchor_slot_15`); Moriarty `t12-slot15` 40a86bea5 holds only the part 1 commits; pick owner-confirmed (wiki 09:04) | processor-final ANC-001 (fixed); completion ANC-013, ANC-014 | ANC-013, ANC-014 | no commit, no LIST.md row |
| A7b | Mycroft MYC-SEAL-021 refusal test | T12, T18 | yes, open: MYC-SEAL-021 in `mycroft-factory` `mycroft/spec/specs/seal/spec.md:155` lists the test as missing; at 09:40 a commit "MYC-REF-025 refuses a contract that uses anchor slot 15" runs its T0 hook in worktree `t12-slot15-mycroft` | completion ANC-015; Mycroft MYC-SEAL-021 | ANC-015 | Mycroft text says the rekey is OWNER; the owner confirmed it 09:04 (note M2) |
| A8a | DONE: NT-1..NT-3 (t10q-ntfix) | T10q | yes: openvm 8780973, 6a00df3, e3b407d; Moriarty 74158f3c1, bb9dc9ae0 (wiki 06:58, before 3/3 fail, after 9/9) | ANC-002 (fixed to the `x_j` form with `k`, `d_in`, `d_out`) | ANC-016, ANC-017 | not merged into `t10m`; LOOP:32 still reads "running" |
| A8b | DONE: typed MEM_BUS C-A2c/PK1 | T11 rekey | partly: code commit openvm `rekey-chips` fe3601c and test fix 4437a74 exist; the keys are not regenerated, so DONE means code only | LW-018 (fixed, scenario); completion LW-020, LW-021 | LW-020, LW-021 | no wiki log entry records it |
| A8c | DONE: C3.1 `checked_sub` | T10q | yes: Moriarty `t10q-ntfix` 95990a6d3 (wiki 07:31) | completion `sherlock-l-and-wrap` | LW-022 | LIST.md row still says "NOT committed" |
| B1 | Rekey: regenerate `L`, `L_Fr`, wrap keys once from LIST.md | T25 | yes, open | completion `sherlock-rekey` | RKY-001, RKY-002 | at 09:40 LIST.md has rows for keccak, secp256k1 and jubjub; it lacks rows for FP-3, T12-SLOT15 and the range checks |
| B2 | Re-measure wrap rows (k <= 21), VK bytes (<= 50,000), 100-bit security | T25 | yes, open | `sherlock-rekey`; DG-002, DG-003, SQG-004 | RKY-003, RKY-004, RKY-005 | |
| C1 | Build G3 legs (CPU prover, GPU prover, stark-backend verifier) | T12 | yes, open: RUNBOOK on `t12-gate` 90f9b4df9 says the legs are not built | SQG-008; completion `sherlock-security-gate` | SQG-014 | |
| C2 | Run G3, G4, G1/G2, FP-8, ledger 9.1.0.1 on the new keys | T12 | yes, open | SQG-006 to SQG-011, FPR-019 | SQG-015, SQG-016 | RUNBOOK lines 77 and 87 hold UNVERIFIED steps |
| D1 | T23: statement -> acceptRun args map (lane t23-args) | T23 | yes, open; at 09:40 the commit of MAP.md runs its T0 hook in worktree `t23-args` | completion `sherlock-suite-settlement` | STL-001 | not committed at 09:40 |
| D2 | Full chain per call app -> leaf -> N1..N3 -> L -> L_Fr -> wrap | T23 | yes, open: waits on B and C | `sherlock-suite-settlement` | STL-002 | |
| D3 | Redeploy SherlockLedger (slot 15) on Preview | T23, T17 | yes, open; LOOP:72 puts the redeploy after T12 | `sherlock-suite-settlement` | STL-003, ANC-014 | |
| D4 | Settle 208 calls, compare with native | T23 | yes, open | `sherlock-suite-settlement` | STL-004, STL-005 | |
| D5 | Part A native rerun (unit t23-partA) | T23 | yes, running: 3 of 33 programs, 4 calls word-equal (t23-suite 16836929b, wiki 08:30) | `sherlock-suite-settlement` | STL-006 | |
| D6 | Dry run DONE | T23 | yes: t23-suite daa1ca745 (0 mismatch, 62 refused-as-native, 145 blocked, 1 pure) | `sherlock-suite-settlement` | STL-005 (verification) | |
| E1 | T8b GP hooks | T8b (LOOP:27) | yes, PARTIAL (openvm 89bee35, Moriarty b1ff56ba1); the open items need row T8c, which LOOP.md does not have | modular-leaf GP specs; completion `sherlock-loop-completion` | LPC-001 | see M3 |
| E2 | T13 Quint lanes since openvm c4bd312 | T13 (LOOP:35) | yes, open: batch 2 prompts ready, e5-wrap 70e728635 merge into tested pending | `sherlock-loop-completion` | LPC-004 | |
| E3 | T14/T14b final timing on new keys (odin s-series `lfr_prove` hill-climb) | T14, T14b (LOOP:36, 38) | yes, open: w3b KEEP on T10m keys (wiki 09:04); odin s-series prep only (bus h77) | GPW-007, PRD-001; `sherlock-loop-completion` | LPC-005 | |
| E4 | T16 (b) run lengths | T16 (LOOP:37) | yes, PARTIAL (two run lengths open) | SPL-011 (fixed) | LPC-006 | |
| E5 | T19 validium (waits T17 b) | T19 (LOOP:41) | yes, waits | `sherlock-loop-completion` | LPC-009 | |
| E6 | T21 prover GUI beyond stub | T21 (LOOP:43) | yes, running; PGUI requirements (165 IDs) in `prover-gui/requirements/` on branch `prover-gui` | `sherlock-loop-completion` | LPC-011 | |
| E7 | T22 Carbon QA verdict | T22 (LOOP:44) | yes, open: no verdict under `qa/gemini-quint` on any branch; spec `sherlock-governance` (34 EARS, ELD and others) on `t22-governance` fe7207819 | `t22-governance` change; `sherlock-loop-completion` | LPC-012 | |
| E8 | Mycroft KRS gates (mutations, compare, gates.sh) | T18 (d) (LOOP:40) | yes, PARTIAL: gates PASS MEASURED (wiki 06:30, 58/62 mutants killed); deliverables staged, not committed in `mycroft-factory` | Mycroft spec; `sherlock-loop-completion` | LPC-013 | |
| F1 | T15 interface freeze | T15 (LOOP:46) | yes, waits | `sherlock-loop-completion` | LPC-014 | |
| F2 | Push t10q after FP-3 | T10q (LOOP:32, 79) | yes, held on FP-3 (wiki 06:58) | `sherlock-loop-completion` | LPC-015 | |
| F3 | Other pushes after T15 | AFK rule (LOOP:79), O11 lead decision (LOOP:73) | yes | `sherlock-loop-completion` | LPC-015 | LOOP:73 says "after T12", the checklist says "after T15"; the AFK rule holds every push except t10q, so no push happens before the owner returns |

Corrections to the checklist status:

- A8b: "DONE" is code only. The keys change at the rekey.
- A8c: LIST.md still marks C3.1 as not committed. Commit 95990a6d3 exists.
- E1: the T8b remainder belongs to a row T8c that LOOP.md does not have.
- D1: lane t23-args had no commit at 09:15 (one in flight at 09:40).

## Missing from the checklist

| # | Item | Source | EARS IDs |
|---|---|---|---|
| M3 | Row T8c: STACKS AIR and VALUE DIGEST inside MERGE-WORK at 5,400 columns (at least 5,417 measured by Carbon F-5), the X8 in-circuit Compact verifier, `L` over SET. LOOP.md names T8c in row T8b but has no row | LOOP:27, 71 | LPC-002, LPC-003 |
| M4 | T16 (c) BLOCKED (leaf proves 5 rules, needs the rekey), (d) muninn runs and the two-host join | LOOP:37 | LPC-007 |
| M5 | T17 (b) on T10q keys and (c) on T11 and T12 keys with a huginn prover hook | LOOP:39 | LPC-008 |
| M6 | jubjub kit end to end: `rsl_jubjub` failed on 9364edb; openvm 637ca2e (09:40) adds HashToCurve, EcMulGenerator and the s01 ec call (tests `rsl_jubjub_htc_constants`, `rsl_jubjub_s01_ec_call`), no MEASURED run on file | wiki 08:14, LIST.md | RSL-006 |
| M7 | SHA-256 kit (built and MEASURED, part of the rekey) | LIST.md | RSL-007 |
| M8 | Canonical ALU and Poseidon input range checks: in the rekey list of wiki 06:24 and 06:50, absent from LIST.md | wiki 06:24, 06:50 | RSL-008 |
| M9 | Merge `t20-batched` 5dc1be2ee after T10q; its ANC-012 still reads "LEAD pick for T15 confirmation" for `K_MAX = 2`, which the owner accepted (wiki 2026-11-14 19:35, LOOP:90) | LOOP:42 | LPC-010 |
| M10 | Merge of `t10q-ntfix`, `t11-zk` (ZK-L2) and the FP-3 work into `t10m`, and the LOOP:32 status of T10q | LOOP:32 | ANC-016, ANC-017, ZK-019 |
| M11 | T13: merge e5-wrap 70e728635 into tested | LOOP:35 | LPC-004 |
| M12 | T18 open items: premise S1, tasks 3b.1 to 3b.3, `lake build --wfail`, commit of the T18 (d) report | LOOP:40 | LPC-013, LPC-019 |
| M13 | T3 open items: parent constraints that never fired, sdk-node 3 of 144 failing without `SHERLOCK_SDK_ROOT` | LOOP:20 | LPC-018 |
| M14 | Lead verdicts missing for DONE or PARTIAL rows: T10q-ntfix, T11, T12 part 1, T17 (a), T18 (c) and (d), T22, T23 | LOOP:81 | LPC-016 |
| M15 | FINAL-DESIGN.md 1.6 and ARCHITECTURE.md keep the `P2(DOM_LOCK)` lock form that the owner replaced | LOOP:90 | LPC-017 |
| M16 | Rows T24 and T25 are staged in the main working copy of LOOP.md (wiki 09:11) but not committed. T25 lists no canonical range checks (M8) | box bus, wiki 09:11 | LPC-020 |
| M17 | Case B (helper hosts) as a future Veritee-based design: refused until then | wiki 09:05 | ZK-018 |

## OpenSpec fixes

Fixed in place on this branch (commit 2). Owner words are quoted in a "Source" scenario of each requirement. Anything else reads LEAD PICK or built design.

| Requirement | Before | After | Basis |
|---|---|---|---|
| ANC-001 | anchor cell `(0)` | top-level slot 15, key constant, prover-chosen slot refused | T12-SLOT15, owner-confirmed (wiki 09:04) |
| ANC-002 | `A_j = PoseidonFr_4(SHK_LOCK, pack(P2(DOM_LOCK, r_P, prog_d, run_tag, j)))` | `A_j = PoseidonFr_4(SHK_LOCK, x_j)`, `x_j = sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, j)` | owner canon 2026-11-14 (LOOP:90), extension confirmed (wiki 09:04), openvm e3b407d `iface::lock_x` |
| HDM-001 | use codes 0 to 7 | plus 8 for the lock operand | openvm `U_LOCK = 8` |
| HDM-007 | `DOM_LOCK` listed with no rule | `DOM_LOCK` enters no lock value (scenario) | owner canon |
| SPL-002 | policy holds "the list of admitted padded shard counts" | no shard-count list, no schedule menu | owner ruling 2026-11-14 on F-3 (LOOP:89) |
| SPL-011 | schedule "from the admitted list" | schedule from the setup-mode profile `sherlock-profile/1`, no menu in `key_root` | same, SETUP-SPEC ST-007, SC-010 to SC-013 |
| ZK-001 | owned hosts only, helpers undefined | case A owned hosts, case B refused (ZK-018), no PCS masking in case A | owner 2026-11-15 (wiki 09:05), T11-3 |
| ZK-003 | "a build with the test feature" | lifetimes per deployment, run and proof; cargo feature `test-seed` | T11-1, T11-4, owner-confirmed |
| ZK-008 | leaf counts follow "the padding rule of the scheduling policy" | leaf counts stay on the user's machine | F-3 ruling |
| ZK-009 | helper undefined | helper = case B of ZK-001 | owner 2026-11-15 |
| ZK-010 | ZK-L2 open | ZK-L2 built at 71c4c001b on `t11-zk` | commit |
| CF-005 | `Field` on FIELD-EMU, `transientHash` as `SPONGE(n)`, `persistentHash` on `sha2`, jubjub as log entries or JUBJUB-EMU | R_SL chip set 3, 6 or 9 AIRs, Fr words, fixed-length sponge, R_SL_SHA256, jubjub, secp256k1 and keccak256 kits in the bucket | built design 6f773b2 (LEAD PICK), owner "yes to both" (wiki 09:06) |
| LW-018 | untyped MEM_BUS | typed MEM_BUS (scenario, LW-020, LW-021) | C-A2c in the owner-confirmed rekey list |
| SQG-013 | 5,400 columns "a T8 AFK pick for owner confirmation at T15" | owner-confirmed 5,400, with the lead condition for STACKS and VALUE DIGEST | LOOP:90 and LOOP:71 |
| SHP-008 | same as SQG-013 | same | same |

Text that other lanes own (not edited here):

- `t20-batched` ANC-012: change "LEAD pick for T15 confirmation" to "owner-accepted 2026-11-14" at the merge. `K_MAX = 2` matches the ruling.
- `t16-elastic` `SETUP-SPEC.md`: EARS ST, PF, SC exist in a research file, not in OpenSpec. SPL-011 now cites them. Move them into OpenSpec at the T16 merge.
- Mycroft lane (`mycroft-factory`, `mycroft/spec/specs/`; `mycroft/` is not on main or `t10m`):
  - M1: MYC-BLT-007 routes `keccak256` and secp256k1 to a tier 3 kit. The owner ordered R_SL kits inside the 6-or-9 AIR bucket (wiki 09:06).
  - M2: MYC-SEAL-021 and MYC-SEAL-005 say the slot-15 rekey is OWNER. The owner confirmed T12-SLOT15 on 2026-11-15 (wiki 09:04).
  - MYC-CHIP-001 lists rules `R_SL_FIELD` to `R_SL_HALT`. The built chip set has the AIRs WORD, ALU, row PUBLIC, RANGE and SHA x4 (openvm `rekey-chips` 5d77cfc, 6f773b2).
- `~/.cache/rekey-t11/LIST.md` (T11 lane): C3.1 is committed (95990a6d3). Add rows for FP-3, T12-SLOT15 and the range checks.

## Requirement ID scheme

IDs continue the module prefixes of `sherlock-processor-final-20261111` (ANC, CF, LW, ZK, SQG). The highest IDs on all lane tips were ZK-011, ANC-012 (on `t20-batched`), LW-019, CF-012 and SQG-013, so the new IDs start above them. New modules take new prefixes: RSL, RKY, STL, LPC. No branch tip uses these prefixes.
