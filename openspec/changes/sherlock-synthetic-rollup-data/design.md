# Design: FSYN, the synthetic Fenrir rollup

Status: DRAFT / PROPOSED, 2026-10-21. Specified only. Labels follow the repository evidence discipline: **source fact** (a pinned file says it), **repository observation**, **inference**, **plan choice** (this package decides it) and **open question**.

## 1. Inputs and sources of truth

| Input | Role | Pin |
|---|---|---|
| Five module specs: VALIDITY (rev 8), DA-PROOF (rev 3), CELESTIA-AUTH (rev 3), LIVENESS-EXIT (rev 7), LIFECYCLE (rev 5) | source of truth for rules, statements and properties | design-cut `module_specs` |
| `INTERFACES.md`, `synthesis/SECURITY-MODEL.md`, `synthesis/CATALOGUE.md`, `synthesis/EXIT-DESIGN-NOTE.md` | interface contracts, adversary table, goals G1 to G6, exit construction | design-cut `interfaces_and_security` |
| `OWNER-QUESTIONS.md` | rulings Q1 to Q17 | design-cut `owner_rulings` |
| 51 Quint files in `models/` (12 composed models including `exit_core.qnt`, 39 `if_IF-*` contracts) | executable oracles | design-cut `quint_oracles` |
| `liv_params.planning.json`, `liv_params_check.py`, `gen_da_table.py` | parameter gate and DA table | design-cut `parameter_gate` |
| Lean package `FenrirRollup`, 47 source files including `L19_Exit.lean` and `L19_Fenc.lean` (no `sorry` or `axiom`, source fact `THEOREMS.md`) | proved bounds used as assertions | design-cut `lean_bounds` |
| FRTF v3 (`MANIFEST.json`, `PIN-SET.json`, `params-v3.json`, `workloads.json`, handoff), `rollup_test_format.qnt`, `envelope.rs` | S0 format, golden vectors, invalid cases | design-cut `rollup_test_format_v3` |
| Fenrir `sprint5-rollup-test-format` (RTF-001 to RTF-030) | format ownership and versioning rules | design-cut `rollup_test_format_spec` |
| `ROLLUP-ARCHITECTURE.md` revision 20 and `scripts/build-rev20.py` | final design, generated from the module specs, interfaces, security model and contracts; E-RB gates | design-cut `architecture` |
| `audits/ROLLUP-ARCHITECTURE.rev19.md` | cost model RX-52 to RX-57, RX-71, RX-94 (revision 20 keeps RX ids only as citations) | design-cut `cost_model_history` |

Revision state (repository observation, 2026-10-21): the design is final. `ROLLUP-ARCHITECTURE.md` revision 20 (sha256 `e933a56d…7e20366`) is generated from the module specs, `INTERFACES.md`, the security model and the executable contracts by `scripts/build-rev20.py`, and `build-rev20.py --check` passed at pin time. FSYN takes rules, statements and properties from the module specs, which revision 20 restates. Revision 20 cites the RX cost figures without restating them, so the cost model and unit costs come from the pinned revision 19 file, labelled history. `stmt_da` is under `DOM_DA7` and `stmt_dm` under `DOM_DM5`.

Rulings that shape the data (source facts, OWNER-QUESTIONS):

- Q1: 100 TPS settled at launch. The 5,000 TPS figure is ordering capacity.
- Q2: full in-circuit user-proof verification (`L_upv`).
- Q8 = B: settlement needs a Celestia inclusion proof for each range (`stmt_da` beside `stmt`).
- Q9 = A: one hand-written decider D.
- Q10 = A: the 100 TPS target stands, and the fleet is sized from E-RB-2 and E-RB-8. The settled rate is what the fleet carries; the governor meters the rest as lag.
- Q12 = A: DA miss reason 4 on an expired light-client store.
- Q13 = C: a funded light-client advancer duty.
- Q14: the contract maintenance authority is unsatisfiable.
- Q15: sealed for life. The only in-place rotation keeps every key and resets an expired light-client store. A change of any key or roster is a new deployment, and owners leave the old one by forced exit, claim or escape. No seat is replaced in place; a migration starts at 3 permanent seat losses (LIFECYCLE R6).
- Q16 = YES: a claim or escape to an unshielded Midnight address reveals its own amount and destination on Midnight, unlinkable to the user's L2 history.
- Q17 = A: a shielded exit hides its destination and its link to L2 history; its asset and amount are public, like any escrow movement on Ledger 9.

Exit construction (source fact, `EXIT-DESIGN-NOTE.md`, LIVENESS-EXIT revision 7): one exit key `xk = (family, era, nf)` that every spend, escape and import consumes; one exit statement XS shared by the forced `WITHDRAW` and the escape, with destination commitment `xcm = H(DOM_XCM, deployment_id, dest, blind)`; payout by opening `xcm`; one hold check, at execution, keyed by `ht = H(DOM_HT, nf)`; at most one hold per capability; `exit_roots` stored in `settled_record[b]`.

## 2. Architecture

```text
design-cut.json ──► extract ──► constants.json, layouts.json, rules.json
                                   │
seed, scale, profile, mix ──► schedule ──► semantic events ──► reference replay
                                                      │              │
                                                      ▼              ▼
                                               artefact emitters   Quint replay tests
                                                      │              │
                                                      ▼              ▼
                                               fixtures + MANIFEST ◄─ oracle gate
                                                      │
                                                      ▼
                                       catalog /3 cases ──► SBENCH /2 consumer
```

Components (plan choice):

1. **Extractor** (`models/sherlock/rollup-synth/extract.py`). Reads the cut, checks every pin and writes `constants.json`, `layouts.json` and `rules.json`. Constants come from `quint compile --target json` output of the oracle models (pure `val` definitions), `liv_params.planning.json` and `params-v3.json`. Field orders and domain tags come from the statement definitions in the module specs through a pinned table parser; each extracted row cites file, line and the cut hash of that file. A constant that exists in more than one source must agree; a disagreement stops extraction with exit 1 and names both sources.
2. **Schedule** (`fsyn/src/schedule.rs`). Produces the simulated clock, header heights, header record counts and payload sizes, and Celestia block heights.
3. **Semantic events** (`fsyn/src/events.rs`). Produces transactions and records per leaf step kind, forced entries, deposits, escapes, exits and claims, DA misses, committee actions and light-client appends.
4. **Reference replay** (`fsyn/src/replay.rs`). Applies events to a reference state (the 23-field `S5` for production shape, the 12-field state for S0) and computes every digest. This is internal consistency only (SSRD-015).
5. **Artefact emitters** (`fsyn/src/emit/`). Write FRTF v3 envelopes, production-shape envelopes, statement preimages and digests, Celestia blobs and shares, NMT range proofs, light-client headers and `stmt_lc` preimages, forced queue entries and claim leaves.
6. **Oracle gate** (`models/sherlock/rollup-synth/oracle.py`). Runs forward and reverse replay against the Quint models (section 5), runs `rtf verify` on S0 output, runs `gen_da_table.py --check` and `liv_params_check.py`, and evaluates property tests.
7. **Catalog bridge** (`models/sherlock/rollup-synth/catalog.py`). Emits `sherlock-benchmark-workloads/3` cases and keeps every /2 case unchanged.

Language split (plan choice): the Rust package `models/sherlock/rollup-synth/fsyn/` is standalone with its own `Cargo.lock`, outside the root workspace (`members = ["crates/*"]`). It pins the Fenrir Ledger-9 crate set of RTF-018 (`ledger-9.1.0.0-rc.6`, midnight-proofs 0.8.2, midnight-circuits 7.2.4 Fenrir-hardened, midnight-zk-stdlib 2.3.5) for Poseidon over the BLS12-381 scalar field and canonical field encoding, `sha2 =0.10.9` for `TH`, and a pinned Ed25519 crate for synthetic Celestia votes. The root workspace keeps its ledger pin `04c9c5d9`. Python orchestration uses the standard library only, like the existing workload generator.

Trust boundaries:

- Every key FSYN creates is public test data derived from the seed: synthetic spender keys, synthetic consent keys, 16 synthetic committee seats and the synthetic Celestia validator set. FSYN reads no wallet, seed phrase, private state or credential, and no real address enters a fixture (carries forward the October 19 security-owner clarification of the stagenet experiment).
- Generated Compact, ZKIR or proofs are out of scope. FSYN emits data and statements only.
- External inputs (Quint, Lean, Fenrir binaries) run from pinned paths; FSYN treats their output as untrusted until the oracle gate checks it.

Failure handling: exit 0 when every check passes; exit 1 when a check fails (the failure record names artefact, check, expected and observed values); exit 2 for a stale cut, a missing input or a malformed file. A failed artefact stays in `failed/` with its record and never enters a manifest.

## 3. Scale ladder

| Scale | Content | Counts per batch | Batches | Purpose |
|---|---|---|---|---|
| S0 | FRTF v3, seed `0x5354524157` | 16 tx, 50 headers, 12-field state | minimal, b1, b2, b4, b6 | byte-identical reproduction; first recursion workloads |
| S1 | production rules at small size | 64 tx, 50 headers, every step kind, 2 TierC, 1 forced of each kind, 1 deposit, 1 DA pointer per header | 1 to 6 | rule coverage at a size every oracle explores |
| S2 | E-RB-8 header | one header at `RECORDS_BYTES_MAX` (3.5 MiB, 7,615 shares, Lean `Da.Budget.share_counts`) and one at the 3 MB planning mix (6,225 shares) | 1 header each | worst-case DA leaf and decider inputs |
| S3 | production batch | 50 headers at 600 tx a header = 30,000 tx (RX-56); planning mix about 3 MB a header; 38,950 `L_inc` leaves | 1 | E-RB-8 throughput run, fleet sizing |
| S4 | sustained | S3 batches at an offered load of r TPS, r in {1, 5, 10, 25, 50, 75, 100, 107} | at least 12 a point | settled-TPS curve; 12 batches is the governor's rate window (RX-53) |
| S5 | adversarial | S0 or S1 base plus one mutation | per case | adversarial corpora (section 7.4) |

The 107 TPS point is the RX-72 standing-reservation ceiling (642 admitted records a header). It probes the governor above the 100 TPS target.

## 4. Generator

### 4.1 Determinism and random streams

Plan choice: each stream is a SHA-256 counter stream `TH("moriarty/fsyn/rng/v1", seed, stream_name, counter)`, where `TH` is the FRTF tagged hash. Stream names are `schedule`, `mix`, `amounts`, `committee`, `celestia`, `forced`, `escape`, `mutation`. A change to one concern changes only its stream (SSRD-004). Proof randomness is outside FSYN: provers use their own entropy, so proof bytes are not expected to be identical between runs (repository observation, stagenet experiment). E-RB-2 seeds the transcript RNG on the prover side.

### 4.2 Header schedule

- Header period 6 s, `H_B` = 50 headers a batch, batch period 300 s, 288 batches a day (source facts, RX-55, RX-56).
- Records a header: offered load r TPS gives 6r records a header, capped at 642 (RX-72). In FROZEN windows the cap is 560 (source fact).
- Payload bytes: `Records` uses 31-byte words (VALIDITY records codec). The generator sizes each record from its kind and pads the header to the mix target. Shares follow Celestia's sparse-share rule: 478 payload bytes in the first share and 482 in each later share (Lean `share_counts`). A header above `MAX_HEADER_SHARES` = 8,192 is refused (SSRD-008).
- Header fields: `pz_hdr_digest`, `fdac_digest` and the range-signature message follow VALIDITY section 2.3 field order from `layouts.json`.

### 4.3 Transaction mix

Three presets (plan choice). Each preset is a vector over leaf step kinds and outcome codes. The realized counts are written per batch.

| Kind | `strawman` (S0) | `planning` (S1, S3, S4) | `cap` (stress) |
|---|---|---|---|
| MAIN transfers with user proof | FRTF v3 kinds 0 to 3 | remainder of the header | remainder |
| covenant call (TierC) | 2 a batch (FRTF covenant pick) | 256 a batch | 1,024 a batch (`TIERC_MAX`, 5 TierC levels) |
| FORCED (WITHDRAW, TRANSFER, FENC_RELEASE) | none | 64 a batch | 512 a batch (`FORCED_MAX`), `FORCED_ROOM(0)` = 11 a header |
| DEPOSIT | 1 a batch | 64 a batch | 512 a batch (`DEP_MAX`), `DEP_ROOM` = 11 a header |
| withdrawal and claim (`FWDR2` leaf) | 1 a batch | 1 percent of MAIN | `EXPORT_MAX` = 1,024 exports |
| SYS, FENC records | none | 4 a batch | `FENC_FORCED_BUDGET` = 512 |
| IMPORT (escape import) | none | only in FX12 scenarios | `IMPORT_ROOM` = 82 a header, `IMPORT_PER_BATCH` = 4,096 |
| LATE | none | 0 | 1 percent of MAIN |
| CLOSE | per FRTF step shape | one per header | one per header |

Exit kinds (section 4.4) are drawn from the `mix` stream with a shielded share parameter (planning 50 percent, plan choice).

Forced outcomes cover the eight active codes of VALIDITY and LIVENESS-EXIT: APPLIED, NULLIFIER_SPENT, COMMITMENT_ALREADY_PRESENT, EXPIRED, CAPABILITY_CONSUMED, ALREADY_LIFTED, FENC_ENCUMBERED, FORCED_INVALID. The pinned exit design note withdraws `FENC_STALE` and `fenc_seq_ref`; neither is an active generated outcome or field. The `planning` values are plan choices, not Fenrir figures; open question OQ-1 asks for a target mix. Caps come from `constants.json`.

### 4.4 Lifecycle events

- **DA misses.** Reason 2 (`laterStored`), reason 3 (`dmAccepted` with a `stmt_dm` for MISS_A or MISS_B and layout cases A1 to A5), reason 4 (pointer above `lc_top` on an expired store, Q12). Reason 1 never anchors and appears only in adversarial cases. A lower miss supersedes a pending one; `reorgFreeze` clears a miss whose block left the chain.
- **Exit paths and related actions.** Note-spend routes share the routed exit key `xk`; each action follows its pinned state effect, rather than consuming a new note key at every stage. An applied `FENC_RELEASE` lifts the hold and consumes the capability, leaving the note unspent so a later withdrawal can apply. IMPORT records an already escaped `xk` in the routed nullifier state without another payout; `claim` marks the withdrawal paid and does not spend the note again. FSYN emits each row with its route-specific public transcript:

| Kind | Route | Statement | Public on Midnight | Hold check |
|---|---|---|---|---|
| L2 spend | ordinary MAIN record | user proof (`L_upv`) | nullifier, commitments, ciphertexts | at execution in the leaf |
| forced `TRANSFER` | `forced_enqueue_v2`, drained by a leaf | FS1 in zero knowledge | `xk`, output commitments, ciphertexts, expiry; no amount, asset or address | at drain only |
| forced `WITHDRAW` | `forced_enqueue_v2`, drained, then `claim` | XS (FS1) | `xk`, `asset`, `amount`, `xcm`, `refund_dest` | at drain only |
| `FENC_RELEASE` | `forced_enqueue_v2` | FS1 | `xk`, capability | lifts the hold when `APPLIED` |
| escape | FX12 `escape` | XS under `escapeKeyAt(esc_gen)` | `xk`, `asset`, `amount`, `xcm` | in the escape proof, against `exit_roots.fenc_root` pinned at `openEscape` |
| import | IMPORT step after FX12 | consumes the escaped `xk` | escape-import root | none (consumes `xk` exactly) |
| unshielded payout | `claim` or native escape | opening of `xcm` in clear | destination and amount (Q16) | none |
| shielded payout | `claim` or native escape | opening of `xcm` as private input; shielded output | asset and amount only (Q17) | none |
| foreign-backed escape | `FWDR v2` foreign exit leaf | XS | `xk`, `asset`, `amount`, `xcm` | as escape |

- **Claims.** Withdrawal leaves are `H(DOM_FWDR2, deployment_id, batch_no, wd_index, asset, amount, xcm)` under `wd_roots[batch_no]`, verified under `claimKeyAt(settled_record[batch_no].g)`. The leaf carries `xcm`, never a destination.
- **Holds.** A hold is keyed by `ht = H(DOM_HT, nf)` in `fenc_root`. The consent binding `hb = H(DOM_HB, cap_id, ht, r)` is a hiding commitment created only by the owner's own spend (C1, C2, P-VAL-30) and opened only at the hold insert. A capability carries at most one hold. The hold is decided once, at execution, from `H(DOM_HT, nf)` and public data; an enqueue-time hold check does not exist. Hold scenarios cover: unheld exit applies; held exit refused as `FENC_ENCUMBERED`; hold inserted after enqueue and caught at drain; release then exit.
- **Escape.** A settlement stall longer than `T_FREEZE` = 604,800 s opens FX12; `esc_gen` and `b = batch_no` are fixed at `openEscape`; `closeEscapeWindow` after `ESC_OPEN_MIN` = 259,200 s; escapes read `settled_record[b].exit_roots`; deposits refund to the recorded destination. Time is simulated, so a seven-day stall is a schedule, not a wait.
- **Disclosure transcript.** Each fixture writes `public.json`, the list of values exposed by its pinned route to an observer of Midnight and Celestia, and `private.json`, the openings. Forced TRANSFER publishes no note asset, amount or destination; its allowed fields remain those of the pinned FS1 interface, including fee/bond refund metadata. WITHDRAW, escape and payout expose asset and amount under Q17. A shielded payout hides its destination; an unshielded payout reveals its own destination under Q16. The oracle gate checks the route-specific field sets and the model predicates `shieldedPrivate` and `unlinkable`; agreement with these abstract predicates is not a cryptographic privacy proof.
- **Modes.** COMMITTEE, PUBLISHED_FHDR (after 24 h without settlement) and FORCED_ONLY. FORCED_ONLY stays off at launch (Q3), so FSYN emits it only for oracle scenarios that the models cover, tagged `launch=off`.

### 4.5 Committee

- 16 seats, threshold t = 12, f = 4 (T1). Lean `committee_arith` gives `2t - n = 8 > f`.
- Byzantine behaviours for at most 4 seats a run: sign anything, equivocate on `(anchor_epoch, h)`, withhold shares and data, delay range signatures up to `F_LAG_SIGN_MAX` = 120 s.
- Colluding-12 behaviours appear only in adversarial cases (section 7.4).
- Seat churn under Q15: no in-place replacement. Permanent losses accumulate per run; at 3 losses the generator switches the run to a migration scenario.
- Sealed-for-life rotation. The only in-place rotation is key-preserving (LIFECYCLE R4): `propose` accepts only `keys[gen]` itself, `activate` needs `TL >= 14 d`, phase NORMAL and the drain of every pre-cutoff withdrawal, and its one effect beyond the generation step is a fresh quorum-signed reset header for an expired store (Q12, `DOM_LCRESET2`). A key or roster change is a migration: a new deployment with a new `relation_id`, while owners of the old deployment leave by forced exit, claim or escape. Migration scenarios generate both deployments and the wind-down exits of every old note.

### 4.6 Celestia light client

- Celestia heights at 6 s, 14,400 a day. Header `time` stored as floor(ns / 10^6) ms (CELESTIA-AUTH).
- Synthetic validator set: 100 validators (plan choice), at most 105 distinct signers per append (DLC: 7 signature leaves of 15 Ed25519 each). Voting power and churn come from the `celestia` stream. Churn rate is a sweep parameter; full churn forces skip length 1.
- Appends: skip length s in [1, `GAP_MAX` = 256]; thresholds `3 x signed_trusted > total_trusted` and `3 x signed_target > 2 x total_target`; trust period `LC_TRUST_PERIOD` = 518,400 s.
- Pointer gaps: each pointer `c` has a cover `B` with `pred(B) < c <= B` and `B - c + 1 <= GAP_MAX` (Lean `cover_exists`, `cover_unique`). The generator emits gaps of every size class: 1, 2, 255, 256, and an over-gap case of 257 that only adversarial corpora carry.
- Statements: `stmt_lc` preimage of CELESTIA-AUTH O1 with `DOM_LC1`, reset headers under `DOM_LCRESET2`.
- Blobs: namespace, share layout and NMT range proofs (8 shares a leaf, one range proof a leaf). NMT golden vectors come from Celestia's public nmt test vectors at a pinned commit (task 2.8).

## 5. Profiles over one schema

The core record set is shared. Each profile is a view of it (SSRD-011).

**Profile A** (source fact, HANDOFF section 2 and RTF-011 to RTF-013). The STF is the Sherlock program `sherlock-stf/1`. Each transaction is one A0 step proof of a bounded step sequence: pre-closes on step 0, apply, post-closes, at most 50 closes a step. A0 exposes the 16-field segment; A1 folds four children at arity 4 with a carried 12-field accumulator; A2 is the top node with wrapper checks; D is the recl9 decider re-keyed with `w_vk` = A2 VK and `rel = relation_digest = H(DOM_REL3, 1, stf_program_fp, vk_fp(A0), vk_fp(A1), vk_fp(A2))`. Shapes:

| Scale | Profile A tree |
|---|---|
| S0 minimal | 2 A0, A2 at arity 2, D (local verify only) |
| S0 b1 | 16 A0, 4 A1, A2, D |
| S1 | 64 A0, 16 A1, 4 A1, A2, D |
| S3 (projection input) | 30,000 A0; arity-4 levels to 1 (4^7 < 30,000 <= 4^8, so 8 node levels), D |

**Profile B** (source facts, VALIDITY section 4 and RX-56). State spine of `L_state` leaves (B = 16 steps, k = 19), 6 node levels; UPV subtree of `L_upv` leaves (4 user-proof slots, k = 20), 7 levels; TierC subtree of `TierCLeaf` leaves (one decided `SherlockProof v1` against `arbiter_vks[(interpreter_version, bounds_class, decided)]`, merged accumulator), 5 levels; then J1, W = J2 and D. Nodes are arity 4 with real children before pads. Inclusion tree INC1: `L_inc` leaves (k = 19, 8 shares), 8 arity-4 levels, walk slots (`L_wk`, at most 50 slots of at most 256 headers), J3 and `DAD`. Miss path: `DMD`.

FRTF v3 Profile B differs from production in one place (source fact): its node has arity 3 (`[main leaf, TierCLeaf, TierCLeaf]`). S0 keeps arity 3 for byte identity. S1 and larger use the production arity 4.

Production-shape envelope (plan choice): FSYN writes `frtf-x/1`, a draft superset of FRTF v3 under tag `moriarty/fsyn-frtf-x-draft/v1`. It adds the 23-field `S5` state, the 17-field `stmt` with `ext_digest`, the `bc_digest` fields, per-header `fdac_digest` and DA pointer tuples, and the inclusion-tree plan. Fenrir owns the format (ID-08, ID-134), so `frtf-x/1` is a proposal to the `sprint5-rollup-test-format` owner and retires when Fenrir freezes FRTF v4 (OQ-2).

## 6. Fidelity

### 6.1 Artefact map

| Artefact | Interface operation | Statement | Oracle module (cut file) |
|---|---|---|---|
| header range, preimages, BIP340 aggregate | V1 `anchorRange`, V8 `signRange` | range-signature message v5 | `validity.qnt` (`validity_core`), `if_IF-VAL-*` |
| batch statement | V3 `acceptValidity`, V4 `settleValidity` | `stmt`, `ext_digest`, `bc_digest` | `validity.qnt`, `if_IF-LIV-VAL-2` (`batchConstants`) |
| escape import and claim | V4 step 6, `claim` | `S5_post[9] = esc_acc_at[esc_k_post]`; `FWDR2` leaf | `val_import_lock.qnt`, `liveness_exit.qnt`, `exit_core.qnt` |
| exit statement and payout | `forced_enqueue_v2` (WITHDRAW), `escape`, `claim` | XS over `settled_record[b].exit_roots`; `xcm` opening | `exit_core.qnt` (QEX) |
| holds | hold insert under C1 and C2, drain, `FENC_RELEASE` | `ht = H(DOM_HT, nf)` in `fenc_root`; `hb` binding | `exit_core.qnt`, `if_IF-VAL-LIV-4` (forced outcome function) |
| disclosure transcript | every exit route | `public.json` against `private.json` | `exit_core.qnt` (`shieldedPrivate`, `unlinkable`) |
| ANCHORED exposure | signing at ANCHORED | `exposed <= cap` | `val_exposure.qnt` |
| inclusion proof per range | `acceptDa`, `daLock` | `stmt_da` (`DOM_DA7`, `da_acc`, `wk_acc`) | `da_proof.qnt` (43 table runs), `da_accept.qnt`, `if_IF-DA-VAL-*` |
| DA miss | `acceptDaMiss`, `dmAccepted`, `admitDaMiss` | `stmt_dm` (`DOM_DM5`) | `da_accept.qnt`, `liveness_exit.qnt`, `if_IF-DA-LIV-1` |
| light-client append | O1 `acceptLC`, O2 `advanceCelestiaLC`, O10 `resetStore` | `stmt_lc` (`DOM_LC1`), `DOM_LCRESET2` | `cel_auth.qnt` (`cel_fixed`, `cel_duty`), `cel_dlc_votes.qnt`, `if_IF-CEL-*` |
| pointer cover | O6 `coverBase` | `IF-CEL-DA-1` | `if_IF-CEL-DA-1`, `cel_auth.qnt` (`coverage`) |
| forced queue, deposits, cursors | `forced_enqueue_v2`, `deposit`, `refundDeposit` | `forced_to`, `dep_to` in `bc_digest` | `liveness_exit.qnt`, `if_IF-LIV-VAL-2` |
| FX12 escape | `openEscape`, `escape`, `closeEscapeWindow` | exit relation v2 (XS) | `liveness_exit.qnt`, `exit_core.qnt` |
| key-preserving rotation, migration | `propose`, `activate`, new deployment | `relation_id` | `lifecycle.qnt` (`lifecycle_sealed`), `life_gen.qnt`, `life_freeze.qnt` |
| FRTF v3 envelope | Profile A and B batch | FRTF `stmt`, `relation_digest` | `rollup_test_format.qnt`, `rtf verify` |

### 6.2 Differential replay

- **Forward** (SSRD-013). For each scenario, FSYN emits `replay/<case>.qnt`. It imports the oracle module and states one `run` that chains the scenario's actions with concrete arguments, each followed by `assert(allInv)` (or `contract`, `consumerHolds`, `consumerOk`, `post` for interface files). Command: `quint test replay/<case>.qnt --main <case>`. Abstract model values (for example hash identities) map through a per-model abstraction table in `rules.json`; the table is part of the cut-bound extraction.
- **Reverse** (SSRD-014). For each oracle, `quint run --main <module> --max-samples N --n-traces N --max-steps M --out-itf traces/<model>.itf.json` samples traces (N and M follow the module evidence: 2,000 by 20 for interface contracts, 5,000 by 30 for LIFECYCLE). FSYN reads each trace, rebuilds the events through its own rules and compares every observable state variable. A trace FSYN cannot express is a coverage gap and is reported, not skipped.
- **FRTF** (S0). `rtf generate --out DIR` output and FSYN output must be byte-identical, and `rtf verify --dir DIR` must pass on FSYN output.
- **DA table and parameters.** `python3 models/gen_da_table.py --check` and `python3 models/liv_params_check.py` run against the FSYN `constants.json` values.

### 6.3 Property tests and Lean bounds

Each property test samples generated scenarios and checks the property on the reference replay and on the Quint replay. Lean bounds become assertions on generated data.

| Property | Check on generated data | Lean assertion |
|---|---|---|
| P-VAL-4 | at most one content per `(anchor_epoch, h)` with at most 4 Byzantine seats | `committee_arith`: `2t - n = 8 > f`; `inter_card` |
| P-VAL-9, P-VAL-11 | settled `post` equals replay of the anchored headers in order; node segment equals sequential composition of real children | L6 segment theorems |
| P-VAL-10, P-VAL-13 | fold binding at equal length; real leaves branch on `bc` | `fold_bind`, L7 |
| P-VAL-15, P-VAL-16 | settle once, in order; accept keys in `(batch_no, batch_no + W_PIPE]`, `W_PIPE` = 8 | none |
| P-VAL-24 | `exposed <= cap` per signing domain | none |
| P-VAL-26 | `K_ANC_MAX x 6 s >= 1.5 (T_prove + T_incl + T_final)` at `K_ANC_MAX` = 96 | `planning_valid_iff` |
| P-VAL-29 | escape-import lock | none |
| P-DA-2, P-DA-3 | an honest range yields `stmt_da`; a range without its data yields none | `Da.chain_binding`, `join_binds_roots` |
| P-DA-5 to P-DA-8, P-DA-17, P-DA-18 | pad, tuple and position substitutions refused | none (Quint `da_proof.qnt`) |
| P-DA-9, P-DA-10 | no false miss; every true miss provable | `wclose_sound` |
| P-DA-14 | leaf rows below 2^19 | `seg_leaf_fits_k19`: 377,000 and 477,000 rows; `unit_subtree_depth`: 1,024 <= 4^5; `slot_subtree_depth`: 43 <= 4^3 |
| P-CEL-3, P-CEL-4 | gap and coverage of every append | `gap_append`, `cover_exists`, `cover_unique` |
| P-CEL-6, P-CEL-7 | Tendermint thresholds; walk authenticates the data root | L02 `Tendermint.*`, `walk_sound` |
| P-CEL-10, P-CEL-14 | retention of covers; front-run append harmless | none |
| P-CEL-13 | a pointer d above `lc_top` covered in `ceil(d / s)` appends | none |
| P-CEL-16 | `GAP_MAX x 6 s < LC_TRUST_PERIOD`; I16: 518,400 + 39,600 + 14,400 + 14,400 = 586,800 <= 604,800 | `planning_valid_iff`, `planning_gate` |
| P-LIV-3 | no non-FORCED_ONLY batch settles without `stmt_da` | none |
| P-LIV-12 | forced drain within `floor(q / cap) + ceil(r / budget) + 1` windows | `drain_bound` |
| P-LIV-15, P-LIV-26 | one outcome per `WITHDRAW`; claims recompute the `FWDR2` leaf | none |
| P-LIV-11, P-LIV-17 | one exit per key across spend, escape and import; the due prefix, `fenc_stop` and the hold decision are stable under appends | `Exit.one_exit_per_key`, `dueCount_append_stable`, `fencStop_append_stable`, `held_append_stable` |
| P-LIV-30 | an escape pays exactly the proven note to its owner's committed destination, once, and never for a held note | none (`exit_core.qnt` `payoutMatches`, `holdRespected`) |
| exit invariants | no double pay, honest exit never refused, no stranded withdrawal, one hold per capability, a held capability stays live, binding by owner only, shielded destination private, unlinkable | `exit_core.qnt` `allInv` (`noDoublePay`, `honestNotRefused`, `noStranded`, `oneCapOneHold`, `heldCapLive`, `byOwnerOnly`, `shieldedPrivate`, `unlinkable`) |
| P-LIFE-14, P-LIFE-19 | reset only on an expired store; maintenance update refused | L12 `Life.*` |
| P-LIFE-18 | rotation batch drains 655 x 50 = 32,750 entries | `rotation_batch_cap`, `forced_rooms` |

Properties held as assumptions in SECURITY-MODEL section 6 (for example P-VAL-27, A-DA-2) get data, not a pass verdict: FSYN supplies the inputs that turn them into evidence (RX-91 mutation floor, G-DA, G-DM, E-RB-8) and records them as open.

## 7. Tests

### 7.1 Golden vectors

| Vector | Value | Source |
|---|---|---|
| `relation_digest` | `8f4ff297…379543` | `params-v3.json` |
| FRTF b1 envelope digests | A `4946d258…488e27d`, B `c6110b79…3097cf` | `workloads.json` |
| FRTF final posts | b1 `f3b33453…`, b6 `ae8cf44f…` | `workloads.json` |
| shares at the planning mix | 3,000,000 B gives 6,225 shares | Lean `Da.Budget.share_counts` |
| shares at `RECORDS_BYTES_MAX` | 3.5 MiB gives 7,615 shares | Lean `Da.Budget.share_counts` |
| shares at the cap | 478 + 8,191 x 482 B gives 8,192 shares | Lean `Da.Budget.share_counts` |
| `L_inc` leaves a batch | 38,950 at the planning mix; 51,200 at the cap | RX-54 |
| I16 sum | 586,800 <= 604,800 | SECURITY-MODEL section 7 |
| rotation batch | 32,750 entries | Lean `rotation_batch_cap` |
| FRTF sizes | A full batch 3,140 B; minimal 1,320 B; B full batch 10,116 B | FRTF v3 bytes |

Each vector file names its source path, line or theorem and the cut hash of that source.

### 7.2 Property tests

Property tests use a seeded shrinking runner (`proptest` in the Rust package; Python `unittest` for the catalog bridge). Each failing case shrinks to a minimal seed and event list and is saved as a regression fixture.

### 7.3 Mutation suite

- **Generator mutants.** Each rule in `rules.json` has a registered mutant (for example gap 257 admitted, a pad before a real child, a forced entry above `FORCED_ROOM`, a stale light-client base, a `stmt_da` over the wrong `k`). Each mutant runs through the oracle gate and must be killed by a named check.
- **Oracle mutants.** The existing Quint mutants are reused as teeth checks: `validity_mut_notruncate`, `validity_mut_keepsptr`, `validity_mut_prune`, `validity_mut_overwrite`, `val_import_lock_mutprefix`, `val_exposure_mutnocap`, `val_exposure_mutunset`, `lifecycle_mut_signer`, `lifecycle_mut_seal`, `noGapMutantBreaksGapTest`, the eleven exit mutants of `exit_core.qnt` (`MutEscKeyHash`, `MutUnboundHoldLookup`, `MutStopFqCount`, `MutRootsAbsent`, `MutShieldedPublic`, `MutHoldByCm`, `MutQueueTimeHold`, `MutLeafBatch`, `MutPublicBinding`, `MutPublicNfBinding`, `MutMultiHold`), and FRTF `rtf_m1` to `rtf_m8`. A generated scenario set must make each of them fail at least one replay, which shows the scenarios reach the guarded states.
- **Codec fuzzing.** The FRTF harness `rollup_envelope_decode` runs 300 s on every FSYN format change (RTF-030), and `frtf-x/1` gets its own bolero harness.

### 7.4 Adversarial corpora

One family for each of the 14 adversary classes of SECURITY-MODEL section 2, plus two attackers named by the exit design note: a designated party that inserts holds and an observer that reads every public field. Every case records base fixture, mutation, expected failing layer, reason code, Quint invariant and killing mutant.

| Adversary class | Cases (examples) | Expected outcome | Bounding properties |
|---|---|---|---|
| Settler | arbitrary arguments, wrong `bases`, front-run settle, FORCED_ONLY before `snap_time(h_last)` | refused; no state change | P-VAL-14, P-VAL-15, P-DA-1 |
| Prover | swapped child proof, pad abuse, tuple or position substitution, replay across generations; FRTF `inner_proof_tampered`, `inner_vk_mismatched` | refused at the named layer | P-VAL-10 to P-VAL-13, P-DA-5 to P-DA-8 |
| Up to f = 4 Byzantine seats | equivocation, withheld shares, signatures on anything | no quorum at t = 12 | P-VAL-3, P-VAL-4 |
| Colluding 12 of 16 | two contents a height, wrong pointer height, foreign namespace, unposted blob, `snap_time` at `SNAP_TOL`, stall, reset header not on Celestia | no invalid settlement; no settlement without data; no freeze | P-VAL-9, P-DA-2, P-DA-15, G6 |
| Rotation quorum | rotation before `TL`, a key set other than `keys[gen]`, activation outside NORMAL or before the pre-cutoff drain | refused; key changes only by migration (Q15) | P-LIFE-2, P-LIFE-4 to P-LIFE-6, P-LIFE-14 |
| CMA holder | maintenance update | refused (`MutSignerCMA`, `MutSatisfiableSeal` kill the bypass) | P-LIFE-17, P-LIFE-19 |
| Light-client advancer | gap 257, out-of-order append, stale base, front-run append, stop advancing | refused or reason 4 | P-CEL-1 to P-CEL-6, P-CEL-14 |
| Activation caller | stale or altered reset fields | refused | P-CEL-12, P-LIFE-14 |
| Users | queue spam, double `WITHDRAW`, refused claim, refund to another destination, escape then spend of the same `xk`, escape then import replayed, claim under another batch number, substituted `xcm`; FRTF eight transaction cases | one outcome per key; recomputed leaf | P-LIV-11, P-LIV-15, P-LIV-17, P-LIV-26, P-LIV-30 |
| Watchers | early `openEscape`, racing calls | refused before `T_FREEZE` | P-LIV-1, P-LIV-19 |
| Designated party (exit note) | hold without owner consent, binding from a public nullifier, one capability over two notes, hold keyed by `cm`, hold inserted after enqueue | refused, or caught by the execution-time check | P-VAL-30, `exit_core.qnt` `byOwnerOnly`, `oneCapOneHold`, `holdRespected` |
| Observer (exit note) | link a shielded exit to its L2 history, read a shielded destination, read a binding at note creation | no link; destination absent from `public.json`; amounts public (Q17) | `exit_core.qnt` `shieldedPrivate`, `unlinkable` |
| Celestia below 1/3 | withheld blob, reorder in block, evicted PFB | miss provable; walk authenticates | P-CEL-7, P-DA-9, P-DA-10 |
| Celestia at 1/3 or more | false header accepted by the store | validity holds; residual R6 recorded | none (out of model, T3) |
| Midnight Ledger 9 | delayed inclusion, reorg of unfinal blocks | `reorgFreeze` drops dependent misses | P-LIV-9 |
| Time | stall past `T_FREEZE`, store expiry, retention expiry, timelock | escape opens; reason 4; drain completes | P-LIV-27, P-LIFE-4, P-LIFE-6 |

The 11 FRTF v2 cases and 8 FRTF v3 cases are imported unchanged with their layers (`invalid-v3/v2-case-layers.json`).

### 7.5 Replayable fixtures

- Fixture id: `fid = TH("moriarty/fsyn/fixture/v1", cut_digest, generator_version, seed, profile, scale, mix)`.
- Each release has `MANIFEST.json` (path, bytes, SHA-256 for every file), `FIXTURE.json` (fid, cut digest, generator version, seed, scale, profile, mix, realized counts, regeneration command) and the oracle gate record.
- `fsyn verify --fixture DIR` regenerates into a temporary directory and compares every byte.

## 8. Benchmarks (switch on with recursion)

All benchmark samples use the SBENCH /2 sample schema and the SRPOC-005 metric set: actual k, 2^k capacity, used and lookup rows and columns where exposed, proof, PK and VK bytes, keygen, witness, prove and local verify wall, CPU and peak RSS, threads, host, toolchain and crate pins, SRS hash (2^20 SRS `1cc62978…`), cold or warm cache. Every sample adds the FSYN fid and scale. Failures and timeouts are censored samples, kept.

| Benchmark | Workload | Measurement | Comparison basis (source) |
|---|---|---|---|
| prover throughput per leaf type | 1,000 seeded inputs a key from S1 and S3 | proofs a host-hour and core-s a proof for A0, A1, A2, `L_state`, `L_upv`, TierCLeaf, node4, J1, J2, D, `L_inc`, `L_wk`, J3, DAD, DMD, DLC leaves | RX-52: node4 958,095 rows, 867.3 s on 6 threads, 14.99 GB; D 586.4 s on 6 threads, 181.4 s on 12; 4-tx leaf k = 17, 34.8 to 49.4 s |
| tree depth and latency | S0 b1, S1, S3 tree plans | critical path from the last record to the D proof; per-level wall | RX-55: 11 k = 20 proofs in series; 53 min dedicated 12-thread CPU, 12 to 24 min GPU at 8x |
| fleet sizing (E-RB-8, E-RB-2) | S2, S3 | core-s a batch for the validity tree V and the inclusion tree I; fleet factor (V + I) / V; hosts at 100 TPS = (V + I) / (300 s x 64 cores) | RX-54: V 24.6 to 57.9 M; I 45.0 to 101.3 M; factor 2.83 to 2.75; CPU 3,610 to 8,330 hosts |
| E-RB-2 inputs | 1,000 seeded inputs for every production key | FSYN supplies inputs and expected public instances; the prover side compares CPU and GPU bytes | RX-54 criterion 1 |
| Ledger 9 decider cost | S0 b1 to b6 (Profile A), S1 (Profile B) | D prove wall, CPU, RSS; `acceptValidity`, `settleValidity`, `acceptDa`, `acceptLC`, `advanceCelestiaLC` transaction bytes and DUST | 2026-10-15 Stagenet run: `acceptValidity` 0.231 DUST, 10,147 B; `settleValidity` 0.233 DUST, 8,545 B |
| DA inclusion proof cost | S2 headers, S3 batch | `L_inc`, node, walk, J3, DAD core-s; DAD and DMD over the 1,024-leaf tree of one header | RX-71, E-RB-8 |
| settled-TPS curve | S4 at r in {1, 5, 10, 25, 50, 75, 100, 107} | settled TPS, lag in headers against `SETTLE_LAG_ADMIT`, tail p50 and p99 | Q1 target 100; RX-53 thresholds |

The RX figures in the comparison column come from the pinned revision 19 file (`cost_model_history`); revision 20 cites them by id. They are comparison bases, never inputs to a projection.

Settled-TPS method (SSRD-023): a measured series runs S4 on the allocated host set; a projected series multiplies measured unit costs by the generated tree plan and a stated fleet size. Each point carries `measured` or `projected`. Settled TPS = settled transactions / elapsed time over at least 12 batches. No point is filled from the modeled RX figures.

Report format: `fsyn-bench-report/1` JSON (cut digest, fid list, host inventory, crate and SRS pins, per-benchmark sample references, measured and projected series) beside the SBENCH samples and raw logs. Reproduction: one command per benchmark, recorded in the report.

## 9. Phasing and gates

| Phase | Runs | Builds | Gate (all must hold) |
|---|---|---|---|
| P0 foundation | now | design cut check, extractor, random streams, S0 generator, catalog /3 skeleton | G0: SSRD-001 to SSRD-005; S0 byte-identical to `rtf generate`; `rtf verify` passes on FSYN output; golden `relation_digest` matches |
| P1 production shape and oracles | now | S1 to S3, fidelity map, forward and reverse replay, golden vectors, property tests | G1: every forward replay passes; reverse replay agrees on every sampled trace; every section 6.3 property passes; `gen_da_table.py --check` and `liv_params_check.py` pass; Lean assertions hold |
| P2 adversarial, mutation, fixtures | now | 14 adversary families, mutant registry, fixture release `fsyn-fixtures/1`, cut check in DoctorWatson | G2: every mutant killed; every adversary class has a case; fixtures regenerate byte-identically on two hosts; handoff to the Fenrir format owner filed |
| P3 Profile A recursion | trigger: A0, A1, A2 and re-keyed D keyed at k <= 20 on the pinned crates, and a recursive proof validated per SRPOC-002 | S0 minimal, b1, b2, b4, b6 benchmarks; tree latency | G3: SBENCH `initial-deployed-baseline` and `full-rollup-series` qualified; every FRTF invalid case rejected at its layer |
| P4 Profile B leaves | trigger: keys for `L_state`, `L_upv`, TierCLeaf, node4, J1, J2, D, `L_inc`, `L_wk`, J3, DAD, DMD, DLC at k <= 20 | per-leaf throughput, S2 and S3 runs, fleet sizing, DA and decider cost | G4: E-RB-8 report against 45.0 to 101.3 M core-s; fleet table recut from measured costs |
| P5 sustained | trigger: G4 and an allocated host set or a GPU backend candidate | settled-TPS curve, E-RB-2 input sets, 24 h run inputs | G5: curve published with labelled series; E-RB-2 criterion 1 inputs delivered |

P0 to P2 need no proof, key or network. P3 is the switch that recursion turns on. Benchmark runs follow the existing native owner, resource limits and the Fenrir rule that milestone benchmarks run at sprint exits.

## 10. Interfaces with Fenrir

- **Locations.** Code: `models/sherlock/rollup-synth/`. Small fixtures (S0, S1, S5): `models/sherlock/rollup-synth/fixtures/<fid>/`, committed. Large fixtures (S2 to S4): regenerated into `$FSYN_CACHE/<fid>/` (default `~/.cache/moriarty-fsyn/`); only `MANIFEST.json` and `FIXTURE.json` are committed. FRTF v3 is read from its pinned path and is not copied.
- **Versioning.** Generator `fsyn/MAJOR.MINOR`: MAJOR changes any emitted byte for an unchanged cut; MINOR adds a scale, preset or case. Catalog cases cite the fid. FRTF follows Fenrir's versions (v3, v3.1 at the FX42 freeze, next version for DL-24 scoped witnesses). `frtf-x/1` stays a draft tag until Fenrir adopts a production-shape version.
- **Sync.** `fsyn cut check` also runs `scripts/build-rev20.py --check`, so revision 20 and its module sources stay one record. The check runs in the ordinary DoctorWatson corpus check. A changed pinned file makes every dependent fixture stale (SSRD-024). `fsyn cut refresh` writes a new cut and a diff of changed constants, layouts and rules; regeneration follows; fixtures whose bytes change get a new fid. FSYN never edits a Fenrir file. Rule questions go to the module owners (RS-1 to RS-5) and format questions to the `sprint5-rollup-test-format` owner.
- **One source of truth.** Constants and rules come from the executable models and the module specs through the extractor. A golden vector that disagrees with its source is a design defect report to Fenrir, not a local fix.

## 11. Semantic guarantees and operational feasibility

Semantic guarantees FSYN supplies: deterministic bytes from a pinned cut; agreement with the Quint oracles on every emitted scenario; Lean bounds checked on every generated artefact; FRTF v3 byte identity. FSYN does not establish proof soundness, circuit fit, prover cost or ledger acceptance. Those are measured in P3 to P5 on the actual circuits.

Operational feasibility to confirm in P1 (inference from sizes): S3 holds 30,000 transactions and 50 headers of about 3 MB, so about 150 MB of blob payload and about 39,000 inclusion leaves a batch. Generation of S3 runs on one host. Quint replay of S3 is infeasible at full size, so S3 replays per header and per range through the interface contracts, and S1 carries the composed-model replay.

## 12. Open questions

- OQ-1: target transaction mix for the `planning` preset (TierC share, forced and deposit rates, LATE rate).
- OQ-2: does Fenrir adopt `frtf-x/1` as FRTF v4, or does Moriarty keep it as a local superset?
- OQ-3: does Profile A run at S3 as a benchmark (30,000 A0 proofs a batch), or only as a projection input?
- OQ-4: storage for large fixtures beyond the local cache (release assets or none).
- OQ-5: host set for P5 and the reference host for E-RB-2 and E-RB-8.
- OQ-6: do DLC fixtures include captured public Celestia headers beside the synthetic validator set?
