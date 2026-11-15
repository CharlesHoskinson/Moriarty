# Slice C decision drafts (for architecture/decisions/)

Sender: Fable-C. MADR shape of the profile template. IDs D-070 to D-085 from the reserved range. Each record binds only through the EARS IDs and checks in its Confirmation.

<a id="d-070"></a>
# D-070: One SherlockLedger for each program

- Status: accepted
- Legacy IDs: N-11; MAP pick 3

## Context and Problem Statement

`L_Fr` requires the anchor operand to equal the run's `C_P` in mode ORD (openvm `t10mm@5b7ada8:crates/sherlock-lfr/src/lverify.rs:1836-1878`). A ledger therefore accepts only runs of the program whose `c_p` is its constructor anchor. T17 R4 confirmed it on Preview.

## Decision Drivers

- SLC-005 (anchor = `c_p`)
- hard rule: ledger 9.1.0.1 verifier unchanged
- ZK-002: plain `prog_d` never on chain

## Considered Options

- One ledger for each program, anchor = `c_p` (chosen)
- One ledger for each era with a map of program anchors: changes the prologue (the anchor read becomes a map lookup) and the wrap decoder

## Decision Outcome

One SherlockLedger for each (contract, circuit) R_SL program, with a fresh `r_P` for each deployment kept on the prover box.

Authority: LEAD PICK confirmed (AFK pick N-11 confirmed by the lead under the owner's "you decide", LOOP.md:87; restated by MAP.md pick 3, which is LEAD PICK for T15).

### Consequences

- Good: the program binding sits in the circuit, with no new check.
- Bad: the T23 suite needs up to 106 deploys (BAT-004), and the era table must attest per-program ledgers (T22 O8).

## Confirmation

SLC-005 by T17 (a) R4 (observed, refused by the node); BAT-004 by the T23 suite table (planned); SLC-012 keeps live settle off until the leaf statement binds `prog_d`, `input_d` and `outcome_d` (planned).

## More Information

### Reconsideration

A deploy count that the Preview fee or DUST budget cannot carry; a ruling on T22 O8 that picks one ledger for each era.

---

<a id="d-071"></a>
# D-071: Mycroft call to acceptRun: outcome, input and refused calls

- Status: proposed
- Legacy IDs: MAP picks 1, 2, 4; FP3-1 (C)

## Context and Problem Statement

T23 settles Mycroft calls through `acceptRun`, whose three words hide the statement. The form of `outcome_d` and `input_d` for an R_SL call was not fixed.

## Decision Drivers

- STL-001, SLC-006 to SLC-010
- T23 compares effect words with the native transcript
- ZK-002, ZK-006

## Considered Options

- Outcome `Halted([comm] ++ eff)`, input `val_d = vd(tape)` with `fuel` 0 and `stack_limit` 0, refused calls not settled (chosen)
- Input from the public inputs only: two runs with different private witnesses share one `input_pub`
- Empty input `VAL_NONE`: binds no input
- Settle a refusal as `Rejected(code)`: needs a rekey that proves refusals as runs

## Decision Outcome

1. Outcome of a halting Mycroft call is `Halted([comm, E_0 .. E_(L_e-1)])`. 2. Input is `val_d = vd(tape)`, salt of 8 CSPRNG words per run, `fuel` 0, `stack_limit` 0. 3. A call that native refuses gets no `acceptRun`.

Authority: LEAD PICK for T15 (MAP.md section 4, branch `t23-args`, staged).

### Consequences

- Good: the settled effect words are word-equal to native, which is what T23 measures.
- Bad: outcomes 1 and 2 are unbound in the proof until the R_SL leaf statement opens `outcome_d` and an INIT opens `input_d` (MAP.md section 5); live settle stays off until then.

## Confirmation

Outcome 1: SLC-007 by tests `MycroftCall.*` (observed; the whole test file has 16 passing tests). Outcome 2: SLC-008 by `MycroftCall.*` for the tape mapping with zero fuel and stack (observed); the prover's CSPRNG salt has no production test (planned); `test_enc1` covers the digest form only. Outcome 3: SLC-010 by the T23 dry run (observed, 62 refused-as-native). Proof binding: planned, T25.

## More Information

### Reconsideration

The T25 leaf statement binds another form; a ruling that refusals must settle.

---

<a id="d-072"></a>
# D-072: Anchor in ledger slot 15, pinned by the wrap key

- Status: accepted
- Legacy IDs: T12-SLOT15; MYC-SEAL-021; X1 (a)

## Context and Problem Statement

A Mycroft contract with a user-writable field 0 could hold a forged `C_P'` in the anchor cell. A prover-chosen slot lets a prover run its own program on a victim contract (T12 PENDING-CX).

## Decision Drivers

- WRP-007, SLC-002, ANC-013, ANC-014, ANC-015
- stock `compactc` words stay byte-equal for all-Sherlock contracts

## Considered Options

- Slot 15 as a wrap key constant, inside the single rekey (chosen)
- Keep cell 0 with a compiler refusal only: the guard is outside the circuit

## Decision Outcome

The anchor cell moves from 0 to 15 in `decode.rs`, `iface.rs` and SherlockLedger inside the T25 rekey: the wrap pins both the anchor read key (lane 6) and the chunk write key (lane 15) to 15 (ANC-013) and refuses an ordinary call that carries the anchor-write template (ANC-016); the Mycroft compiler refuses a contract that writes slot 15.

Authority: LEAD PICK confirmed (owner 2026-11-15, llm-wiki `wiki/log.md:4098-4107`). Shared with slice B (CLB) for `iface.rs` and slice A for the compiler refusal.

### Consequences

- Good: the guard sits in the circuit.
- Bad: every deployed ledger and the wrap VK change; the T17 (a) ledger is not reusable.

## Confirmation

WRP-007 by `t12_anchor_slot` (observed for forged slots on `t12-gate@90f9b4df9`) and `t12_anchor_slot_15` (planned, uncommitted); SLC-002 by a compiled-contract check (planned).

## More Information

### Reconsideration

A Compact release that changes ledger slot layout.

---

<a id="d-073"></a>
# D-073: Batched settlement form: collection statement, claims tree, native reads

- Status: accepted
- Legacy IDs: T20 picks W-1, W-2, W-3; O9; FP3-3 (a)

## Context and Problem Statement

Owner rule 2026-11-14: no per-event Midnight write; one root per checkpoint. The T10 contract wrote one `openClaim` per used claim.

## Decision Drivers

- LDG-001 to LDG-008, BAT-001 to BAT-012, SLC-001, SLC-004
- hard rule: Midnight writes batched

## Considered Options

- `acceptCollection(reg_pub, claims_root)` once per epoch, claims used by membership with no ledger write, read entries with native keys, at-most-once at the consumer nullifier (chosen)
- Keep `openClaim`: one wrap proof (175.5 s) and one write per used claim

## Decision Outcome

Collection statement `[reg_pub, claims_root]`; claims tree depth 20; native-keyed read entries `rootAccepted` and `runAccepted`; I-21 at the consumer nullifier; tag `SHK_FCLAIM`; epoch 3,600 s; a TTL below 1,755 s admits no chunk. `acceptRun` stays for a user's own run.

Authority: OWNER for the rule (order 2026-11-14, hard-rule record D-009). LEAD PICK confirmed for the picks (T20 picks confirmed by the owner, LOOP.md:106).

### Consequences

- Good: at the ceiling of one prover, at most 24 transactions per day in place of 492 (T20 RESULTS, DERIVED).
- Bad: a consumer waits up to one epoch plus the collection proof; the leaf list needs off-chain availability, with no storage or retention rule yet; LW-009 and DCD-004 are amended to `[reg_pub, claims_root]` on `t20-batched`; MID-002 no longer holds for read entries (superseded by SLC-004); the `L` collection form, the claims tree proof and the seventh DIGEST tag `SHK_FCLAIM` are not built (BAT-009, BAT-012); the registry has no update entry (`reg_root` is sealed) and its history rule N-8 is open, so LDG-008 has a model and no contract.

## Confirmation

LDG and BAT by `t20-ledger-test.out` (observed, 9 tests) and Quint `epochSettle.qnt`, `claimsRoot.qnt`, `setOpening.qnt` (observed, mutants caught). Kind separation and once-only use (LDG-004, LDG-007): `t20-ledger.test.mjs` tests `W-3 foreign claim` and `W-3 kind separation` (observed, compiled contracts). Collection proving and a final-key batched run: planned (BAT-009, BAT-012, row T8c). Era-aware close: planned (BAT-010, BAT-011).

## More Information

### Reconsideration

A measured epoch wait that a consumer cannot accept; a data-availability failure in a run.

---

<a id="d-074"></a>
# D-074: K_MAX = 2 at every TTL, and a 3,600 s settlement TTL

- Status: proposed
- Legacy IDs: ANC-012; O6; T20 fix 4

## Context and Problem Statement

ANC-012 gives `K_MAX = min(floor(TTL / 1,755 s), 18)`, which is 2 at 3,600 s and 8 at the runner's default of 14,400 s (O6). The owner accepted the cap as `K_MAX = 2`, and the brief lists it as a hard rule.

## Decision Drivers

- BAT-005, BAT-006, SCL-007
- hard rule K_MAX = 2

## Considered Options

- Pin `K_MAX` = 2 for every TTL and settle with TTL 3,600 s (chosen)
- Keep the formula: the cap grows with the TTL, against the ruling
- Keep TTL 14,400 s (O6) with the pinned cap: longer exposure of a built intent with no gain

## Decision Outcome

The plan tool refuses more than 2 chunks at every TTL. 3,600 s is the default TTL of the settle runner, not an enforced value: a user may give another TTL inside the range of SCL-007. The runner default changes from 14,400 s to 3,600 s.

Authority: LEAD PICK for T15 for the TTL value and the reading of ANC-012; the cap itself is OWNER (llm-wiki `wiki/log.md:3502-3503`).

### Consequences

- Good: one cap, no silent growth.
- Bad: O6 (4 h TTL, ruled 2026-11-13) is replaced by the later T20 value; a proof slower than 3,600 s cannot settle.

## Confirmation

BAT-005 by a `sherlock_plan` test at TTL 14,400 s (planned); BAT-006 by `t17.sh:19` (observed in T17 (a) through `t17.sh` only); the runner default and a direct-start test are planned.

## More Information

### Reconsideration

A measured entry that needs a chunk; a wrap time that nears the TTL.

---

<a id="d-075"></a>
# D-075: acceptCollection is final

- Status: accepted
- Legacy IDs: Carbon afk-picks F-8, option A

## Context and Problem Statement

A registry revocation after a collection is accepted could be read as revoking the collection's claims.

## Decision Drivers

- LDG-003, LDG-008

## Considered Options

- `acceptCollection` is final (chosen)
- A registry recheck on every use: a cost on every consumer for a case revocation does not cover

## Decision Outcome

An accepted root stays accepted. Revocation acts on later collections only.

Authority: LEAD PICK confirmed (lead decision under the owner's "you decide", LOOP.md:86).

### Consequences

- Good: no per-use registry read.
- Bad: a claim of a key revoked later stays usable; a soundness break is handled by the era (`soundness_end`), not by revocation. No deployed revocation mechanism exists: the registry update entry and the history and grace rule (N-8) are open.

## Confirmation

LDG-003 by `t20-ledger-test.out` (observed).

## More Information

### Reconsideration

A use case that needs retroactive revocation.

---

<a id="d-076"></a>
# D-076: Governance: sealed contracts, eras in a sealed era table, Fenrir operator verbs

- Status: accepted
- Legacy IDs: T22 LEAD pick; HFC study

## Context and Problem Statement

Owner words: governance is "a set of keys rotated from the Fenrir operators"; "Could we just develop a functional version system with backwards compatibility like Cardano does with the hard fork combinator".

## Decision Drivers

- EGV-001 to EGV-010; GOV-001 to GOV-011, ERT-001 to ERT-009, ELD-001 to ELD-007, CRR-001 to CRR-007
- hard rules: one universal key set, verifier unchanged

## Considered Options

- Sealed-for-life contracts; a new key set is a new era; a sealed era table lists eras; Fenrir operators act by quorum verbs (chosen)
- Upgradable contracts with in-place verifier-key swaps: one key committee can replace the verifier

## Decision Outcome

Every Sherlock contract deploys with an empty maintenance committee and threshold 1 and passes a readback gate. Eras are composed by the era table. Verbs: announce, activate, `soundness_end`, cancel, roster rotation.

Authority: LEAD PICK confirmed (owner 2026-11-15, llm-wiki `wiki/log.md:4107`, a group confirmation of "T22 governance picks" with no item list). The picks of row T22 at that time were sealed-for-life contracts, eras in a sealed era registry and the Fenrir verbs (LOOP.md:44). Whether the confirmation also covers the recommended options of O1 to O13 is UNCONFIRMED; no numeric value of P1 to P6 has an explicit confirmation (D-077).

### Consequences

- Good: no party can change a deployed verifier.
- Bad: the compact-js deploy path installs a one-key committee, so a sealed deploy tool is needed, and the T17 (a) ledger is unsealed (EGV-003).

## Confirmation

Quint `eraTable.qnt`, 13 invariants, 11 of 11 mutants caught (observed, lead run; Carbon verdict planned, LPC-012). Not modelled: attestation (GOV-008), replay protection (GOV-010), `cancel_rotation` (GOV-011), the grid (ERT-009), the use-time fallback (CRR-006); `inv_endSetOnce` and `inv_noOverlap` have no mutant of their own. GOV-001 to GOV-011 stay the normative verb requirements (bus/adopted-C.md). EGV-001, EGV-002 by a sealed deploy tool (planned, T22 task 4).

## More Information

### Reconsideration

A Midnight hard fork that stops verifying the proof version (T22 O11); a failed premise A1 or A2.

---

<a id="d-077"></a>
# D-077: Governance open items take the recommended options until the owner rules

- Status: proposed
- Legacy IDs: T22 O1 to O13, P1 to P6; UC-9

## Context and Problem Statement

The owner confirmed the T22 picks as a group. OPEN-QUESTIONS.md states that no parameter value is a rule.

## Decision Drivers

- EGV-008

## Considered Options

- Carry the recommended options, each marked open, and state no value as a rule (chosen)
- Write the recommended values as requirements: invents rulings

## Decision Outcome

Specs name `TL`, `RTL`, `Q`, `n`, `GRID`, `SKEW` as parameters. Recommended: `TL` 7 days, `RTL = TL`, no overlap, `Q` 12 of `n` 16, `GRID` 3,600 s, `SKEW` the ledger block-time bound. O8: keep N-11 with attested per-program ledger roots. O11: a proof-version change is a new table and new ledgers with a predecessor link and a marked trust downgrade (EGV-010). The encoding of `keyset_d` is open (EGV-004). Retry identity (LEAD PICK for T15): a retry of one logical run reuses the run salt that the box holds in private state, so `input_pub` and `outcome_pub` stay equal and CRR-005 sees one run; a new salt is a new run. This agrees with ZK-006 (the salt is fixed for the run at INIT) and with D-071 outcome 2. Lane: slice B (ZKE) for the salt store. `keyset_d` (LEAD PICK for T15): the universal key-set identity of slice B (the key manifest of UKS), not a hash of the wrap VK alone. Lane: slice B (UKS).

Authority: LEAD PICK for T15.

### Consequences

- Good: no invented rule.
- Bad: the era table cannot deploy for real use until the owner rules.

## Confirmation

EGV-008 by review of the T15 record (planned); EGV-010, BAT-010, BAT-011 have no check yet (planned).

## More Information

### Reconsideration

An owner ruling on any item.

---

<a id="d-078"></a>
# D-078: The era check enters the single rekey, or a recorded premise failure selects the fallback

- Status: proposed
- Legacy IDs: ELD-003, ELD-007; CRR-006; T22 premises A1 to A3; T22 tasks 2 and 3

## Context and Problem Statement

The era check adds 43 transcript words to `acceptRun` and `acceptCollection` (85 and 91 lanes), and the wrap must bind them. T22 task 3 puts that binding before T12. The single rekey list of T25 has no era row, and premises A1 to A3 are untested. ELD-007 allows the consumer-side fallback only where premise A2 or A3 fails. An untested premise is not a failed premise.

## Decision Drivers

- EGV-005, EGV-006, EGV-009
- one rekey from one list (RKY-001, RKY-002; row T25 of the lead's Finish plan, not a hard rule of the brief)
- ELD-003, ELD-007 (T22 design; the era check on every accept is T22 item O3, whose owner confirmation is UNCONFIRMED, see D-076)

## Considered Options

- The rekey waits for one of two records: an era row in the list (primary path, ELD-003), or a failed premise result (fallback, ELD-007) (chosen)
- Start T25 without either and let consumers check the era: selects the fallback without its premise, and leaves a ledger that accepts after a `soundness_end`
- Add the era ledger in a second rekey after T25: breaks the single rekey

## Decision Outcome

The T25 rekey does not start while the list has no era row and the premise test has no failed result. The T22 premise test (tasks 2 and 3) moves onto the critical path before T25. If A1 to A3 pass, the era-check words join the T25 list. If A2 or A3 fails, the T20 entries stay and consumers check the era at first use.

Authority: LEAD PICK for T15 (Fable-C, 2026-11-15), reading T22 task 3 and RKY-001 together. It changes no ruling. It replaces the first draft of this record, which chose the fallback outright (Astra-C finding F-18).

### Consequences

- Good: the single rekey stays single, and the fallback keeps its stated condition.
- Bad: T22 tasks 2 and 3 now gate T25; the Finish plan critical path (LOOP.md:52) gains one input, and the rekey list gains one row or one recorded failure.

## Confirmation

EGV-009 by the T22 devnet premise test (planned). EGV-005 by a check in the rekey runner (planned). EGV-006 by a consumer test and a model of the use-time fallback (planned).

## More Information

### Reconsideration

An owner ruling that the prototype may run without the era check; a premise test that cannot run on ledger 9.1.0.1.

---

<a id="d-079"></a>
# D-079: Preview: deploys and calls with the existing wallet; Mainnet never without the owner

- Status: accepted
- Legacy IDs: owner 2026-11-14; O11

## Context and Problem Statement

The prototype must run on Midnight Preview. The loop is often AFK.

## Decision Drivers

- PVW-001, PVW-002, PVW-009, PVW-013; MID-006

## Considered Options

- Preview allowed with the pinned wallet and genesis; Mainnet and pushes owner-gated; final deploy after T12 on the T25 keys (chosen)

## Decision Outcome

The settle runner sends only to the local devnet and the pinned Preview genesis. The final prototype deploy waits for the T12 pass and for the gate record of the exact wrap key (PVW-013).

Authority: OWNER (2026-11-14, LOOP.md:95) for Preview and Mainnet; LEAD PICK confirmed for "after T12" (LOOP.md:88).

### Consequences

- Good: a wrong chain cannot receive a transaction.
- Bad: no negative test of the chain pin exists yet.

## Confirmation

PVW-001, PVW-002 by negative tests (planned); PVW-009 by T17 (c) receipts (planned).

## More Information

### Reconsideration

An owner order for Mainnet.

---

<a id="d-080"></a>
# D-080: Wallet handling: in-process phrase, pinned address, one lock, muninn only

- Status: proposed
- Legacy IDs: T17 fixes 6 and 8; Finish plan

## Context and Problem Statement

Several agents send from one Preview wallet. The phrase must never reach a log, a file, a box or the repository.

## Decision Drivers

- WAL-001 to WAL-009, HLP-007
- owner rule: wallet seeds and keys are never read, printed or copied

## Considered Options

- Phrase read in process from the mode-0600 file, scrubbed from every message, address pinned, `flock` around every wallet use, submit from muninn only (chosen)
- A wallet on each box: more copies of the phrase

## Decision Outcome

As chosen. Boxes return public words and proofs; the one machine that holds the wallet submits. In the factory rehearsal that machine is muninn and the pinned address is OP-PV-WALLET; a user outside the factory pins the user's own wallet and machines (SCL-010). An existing wallet is never overwritten (WAL-009). No private wallet material, derived keys included, reaches a log, a state file, an error record, a child process or a receipt (WAL-002).

Authority: LEAD PICK for T15 (built design of T17; "wallet stays here", LOOP.md:58).

### Consequences

- Good: one copy of the phrase, one sender at a time.
- Bad: muninn is a single point for settlement; a muninn outage stops settles.

## Confirmation

WAL-004 by T17 (a) with concurrent T18 deploys (observed); WAL-009 by `preview-session.test.mjs` (observed on branch `preview-deploy`); WAL-002, WAL-003, WAL-005 by marker-secret tests (planned; derived keys are not scrubbed today).

## More Information

### Reconsideration

A multi-user deployment, where each user holds an own wallet.

---

<a id="d-081"></a>
# D-081: A refusal counts only with a node error

- Status: proposed
- Legacy IDs: T17 fixes 8, 9

## Context and Problem Statement

In the first T17 run a network loss made all four controls fail in the wallet; a naive runner would have counted them as refusals.

## Decision Drivers

- PVW-007, PVW-008, PVW-012, OBS-002, FRC-004

## Considered Options

- Four exclusive results in this order: `included`, `not-included-timeout`, `refused-by-node` (a node JSON-RPC error), `no-node-verdict`; a control passes only as `refused-by-node` (chosen)
- Count any failed submit as a refusal: hides a dead network

## Decision Outcome

As chosen.

Authority: LEAD PICK for T15 (built in `t10a-ledger-run.mjs:564-573`).

### Consequences

- Good: a tamper test cannot pass by accident.
- Bad: a run on a bad network fails and must be rerun.

## Confirmation

PVW-007 by the T17 (a) first-run record (observed); a fault test (planned).

## More Information

### Reconsideration

A node release that changes its error codes.

---

<a id="d-082"></a>
# D-082: Helper machines: case A only; muninn checks every box wrap

- Status: accepted
- Legacy IDs: ZK-018; T11-3; OD-9

## Context and Problem Statement

Owner 2026-11-15: "we will start with case A and a future design will allow for outsources components using the Veritee model from the MPC Research thread".

## Decision Drivers

- HLP-001 to HLP-008, ZK-001, ZK-009, ZK-018
- hard rules: zero knowledge end to end; no plain witness to helpers

## Considered Options

- Case A only now; case B is a later OpenSpec change on Veritee seald Sessions (chosen)

## Decision Outcome

Boxes the user owns prove. A box wrap is kept only after the ledger 9.1.0.1 check on muninn. Private parts stay on the box.

Authority: OWNER (2026-11-15, llm-wiki `wiki/log.md:4111-4113`; muninn check: loop rule, LOOP.md:96).

### Consequences

- Good: no witness leaves the user's machines.
- Bad: no executable ownership guard exists (ZK-018 open).

## Confirmation

HLP-001 by the w3b record (observed, 24 of 24); HLP-005 by a configuration test (planned).

## More Information

### Reconsideration

An accepted Veritee Sessions design.

---

<a id="d-083"></a>
# D-083: Wrap row gate: 1,782,573 rows at k = 21, option A

- Status: accepted
- Legacy IDs: O3; T4; OD-24 option E

## Context and Problem Statement

The wrap must fit k = 21 with headroom for later checks.

## Decision Drivers

- WRP-002, WRP-003, RKY-003
- hard rules: never k = 22; one BLS12-381 wrap

## Considered Options

- Gate at 85 percent of 2^21 - 7 rows with option A (chosen)
- Option B (log blowup 2): 1,927,450 rows, over the gate, no key

## Decision Outcome

The wrap tool stops above 1,782,573 rows. Measured: 1,685,845 rows on T10m.

Authority: LEAD PICK confirmed: the owner ruled "go with your recommendations" on O3 (2026-11-13) and then "you decide" (2026-11-14); the lead chose option A (LOOP.md:84). The k = 21 limit itself is OWNER (D-001).

### Consequences

- Good: 96,728 rows of headroom.
- Bad: 12.4 s of warm STARK time that option B would save.

## Confirmation

WRP-003 by `tests/i5_whir.rs` and the T10m measurement (observed); RKY-003 on T25 (planned).

## More Information

### Reconsideration

Wrap rows under the gate with option B and a real key (wrap-diet).

---

<a id="d-084"></a>
# D-084: T23 instance reuse and box profile path

- Status: accepted
- Legacy IDs: T23 LEAD PICKs

## Context and Problem Statement

Suite calls expect the deploy-block state; boxes need one profile location.

## Decision Drivers

- SLC-011

## Considered Options

- At the start of a program's replay, reuse an instance only if its state equals the deploy-block state; state is kept between the calls of one replay; profile at `~/sherlock/setup/profile.json` (chosen)

## Decision Outcome

As chosen.

Authority: LEAD PICK confirmed (owner 2026-11-15, llm-wiki `wiki/log.md:4107`).

### Consequences

- Good: outcomes compare with native.
- Bad: more deploys.

## Confirmation

SLC-011 by the Part A plan of `parta.py` (observed: reuse is decided once per program at the start of the replay); a two-call sequence test (planned).

## More Information

### Reconsideration

A suite whose calls depend on earlier calls.

---

<a id="d-085"></a>
# D-085: The prototype SDK is the existing tool set behind one runbook

- Status: proposed
- Legacy IDs: none (new)

## Context and Problem Statement

ANC-012 names `sherlock plan`. No single `sherlock` binary exists; the settlement surface is seven tools.

## Decision Drivers

- SCL-001 to SCL-010

## Considered Options

- Keep the seven tools and one runbook for the prototype (chosen)
- Build one `sherlock` binary before T15: new code on the critical path with no gate that needs it

## Decision Outcome

The prototype ships the seven commands of SCL-001. One entry point is work after T15. The factory pins (OP-PV-WALLET, muninn, the box table) are rehearsal configuration, not product rules: SCL-010 requires the same commands to run from a user's own configuration.

Authority: LEAD PICK for T15 (Fable-C, 2026-11-15).

### Consequences

- Good: no new code before the freeze.
- Bad: a developer must follow a runbook; error schema and versioning are not stable.

## Confirmation

SCL-001 by an end-to-end script (planned); SCL-010 by a clean-environment run (planned).

## More Information

### Reconsideration

A public developer release.

---
