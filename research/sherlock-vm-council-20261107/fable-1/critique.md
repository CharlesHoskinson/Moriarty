# Critique: fable-1 (chair) on the five other positions

Written 2026-11-07 ~21:15Z after reading all six `position.md` files plus `astra-1/assurance-proposed.md`, `astra-2/security-clauses.md`, `astra-3/proposed-text.md`. Paths relative to `/home/hoskinson/Projects/Moriarty` at `8fd34ed79`.

## Where the six positions already agree (no issue to open)

1. Keep the explicit bounded machine, the native leaf -> node -> decider route with the terminal pairing, the ledger-9 custom-decider contract operation without `VerifyProof`, UISEC-001..012 / ZR / MNR as distinct obligations, and the factory predicate loop. All six.
2. Re-pin the workspace to the ledger 9.1.0.1 line (`a2b3012`), `midnight-circuits 7.3.0`, `midnight-zk-stdlib 2.4.0`, `midnight-zkir 3.2.0`, Compact 0.36.0, Midnight.js 5, proof server 9.0.x; regenerate every key; stagenet (`rpc.stagenet.shielded.tools`, specVersion 2000000, ledger `=1.0.0`) is an rc-era chain, Preview (specVersion 2001100, reset 2026-11-07) is the mainnet rehearsal. fable-2 §M6, fable-3 §6, astra-3 observations, astra-1/astra-2 ledger sections, midnight-scout `MIDNIGHT-READY`.
3. The dispatch factorial run is a test-cost problem and a nonproving `ProofPreimage::check`; do not raise `--max-wall` first. All six.
4. A8 passes on any `ContractCall` at the pinned address and fails today only because `2391d62f…` has a deploy and no call; bind acceptance to a candidate manifest (A9/A10 and statement readback). astra-1, astra-3, fable-2 I6, fable-3 I2.
5. Mutation accounting: `qa_mutation::run_once` maps synthesis errors to `Rejected`, `run_suite` counts panics as rejected, `assert_sound` does not require zero panics (`crates/mori-backend/src/qa_mutation.rs:458-486,640-650`). astra-1 found it, astra-2 and astra-3 verified it. Accept as a requirement.
6. Evidence staleness: `status.json` keeps pass flags across heads (`models/doctorwatson/factory/status.py:34-47,77-112`); add dependency fingerprints / a target-pins check. astra-1 ASR-002, astra-3, fable-3 I5/I8.
7. The six-lane producer vs seven-lane consumer mismatch (`crates/mori-backend/src/sherlock_c6_child.rs:817-834` vs `crates/mori-recursion/src/ivc.rs:191-201`). astra-2 found it, astra-1 and astra-3 verified it. I concede: my position cites the five-lane statement from `docs/sherlock/stagenet-gp-recursion.md:113`; the consumer of record already carries `steps`, so the statement to freeze is `[binding, prog_d, pre_d, post_d, rule, h_pre, steps]`, which is exactly what my S-step leaf needs.
8. Stand-ins, lost-host receipts, `checkStepInv`, wrapper exit codes and status prose are not evidence. All six.

## astra-1 (assurance)

Agree: the typed assurance record per result (class, fingerprints, profile, bounds, command, hashes, assumptions); completeness and soundness as stated proof obligations per rule, not consequences of a finite suite; ASR-003/007 (negative-evidence accounting; manifest-bound receipts). The `ToyLeaf` finding (`crates/mori-recursion/tests/sherlock_ivc.rs:68-72`, binding and steps unconstrained) is correct and strengthens the case for the versioned leaf contract.

Disagree, mildly: M1 "use the existing literal program and the smallest recursive example to connect source ... leaf proof, recursive parent, decider" reuses the A102/A80 two-child `parent.rs`. That parent cannot take a C6 leaf without a reviewed C6 slot (`docs/sherlock/design/c6-recursion-adapter.md:82-93`), and the GP node `ivc.rs` already exists with real keys. Building the small complete path on the IVC node, not the literal parent, saves one throwaway integration. Evidence: `docs/sherlock/stagenet-gp-recursion.md:190-196` (node and decider measured).

Concede: ASR-001/002/012 belong in `assurance.md` as ubiquitous requirements; my position had only the evidence-class list.

## astra-2 (security)

Agree: SND-001 (one versioned leaf statement, producer and consumer checked together); SND-002 (StepInv preserved internally, canonical decode only at external boundaries, as `security-candidate-b.md:A4` already says); SND-003 (bounded `steps`/height arithmetic; `u64` in Rust is not an in-circuit range proof, `ivc.rs:116,308`); SND-004/005/006. The admin-publish challenge (Return payload/padding lanes copied into Halted, `c6_admin_publish_step.rs:88`, `c6_step_kit.rs:521`) is a legitimate standalone hostile case; it must go into the hostile-witness list for M2 even though no full-run exploit is shown.

Disagree: none on substance. One precision: SND-001 says bind "exact step interval"; the brief's `h_pre`/`steps` pair gives that, provided `steps` is defined (see ISSUE 5).

Concede: my I4 ("freeze config in leaf key identity") is weaker than SND-005; the relation/proof/key/SRS/profile tuple must be authenticated at the acceptance path, not only recorded.

## astra-3 (critic)

Agree: A8 is deployment-specific, not a soundness failure; cut replacement zkVM, simultaneous layout candidates, speculative ISA expansion and new workflow machinery; a dispatch receipt alone cannot close U023 (`qa/holdout/lib/uireq.py:1195-1237`, `native_inventory.rs:805-813` `c6_fragments_wired: false`); "define whether `steps` means physical transitions or charged work" is the right question (free terminal publication makes them differ, `design.md:25-35`).

Disagree: "A reduced reproducer cannot replace the complete acceptance run" for dispatch. The complete run is O(steps x supported rules) `ProofPreimage::check` calls over 46 resident circuits; nothing in UISEC-001 or UIREQ-023 requires exhaustive host checking, and a host check is not soundness evidence anyway (UISEC-012). I propose the acceptance coverage be: fired-rule acceptance on every step of P1-P3, exclusivity of every supported rule on a committed seeded sample, mutation refusal on 40 states, each under its own budget, plus the record of reduced scope that astra-3 asks for. See ISSUE 4.

Disagree, mildly: sequencing "requalify the existing recursive endpoint" (A8 on stagenet) before dispatch/inventory. Stagenet is rc-era and not in the support matrix; the A-chain keys change at the re-pin anyway. Re-earning A8 on stagenet before M0 buys a green row that the re-pin voids. See ISSUE 2.

Concede: "Keep existing Candidate-A endpoint requalification independent of new C6 integration" is right as a dependency statement; neither borrows the other's completion.

## fable-2 (recursion)

Agree: the three-relation shape; S machine steps per leaf as the first lever; node k=19 attempt as an improvement not a gate; C0 phase profile before any accelerator; bind `cfg_d` (I4; same as my I4); key manifest (I7); recapture the acceptRun template with 0.36.0 (I8); cut MC03, the two-child parent and C8/C10; stagenet for cheap iteration, Preview as rehearsal, mainnet last.

Disagree: C5 "cut the Fenrir B chain as a gate for the Sherlock VM" conflicts with fable-3 M5 (B rows return as preview rows) and with the brief's inputs. I side with fable-2 on gating (B4/B5/B8/B9 must not gate U023/C4) but with fable-3 on keeping B as predicates in the same factory. See ISSUE 7.

Disagree, mildly: the 2 h / 48 GB throughput budget for a 320-step program is an estimate placed as a milestone gate (G5 "met or shortfall recorded"). Acceptable only in that recorded-shortfall form; it must not become an Unwanted-behaviour requirement.

Concede: fable-2's ledger-9 table is more complete than mine (storage 3.0.0 `EncodingShape`, `AlignedValueChecked`, #838 single-DUST-spend). Adopt it for `design.md`.

## fable-3 (delivery)

Agree: `scripts/doctorwatson` is the only surface; `stop-rule` has been in STOP for 295 h and nothing consumed it; operations (mutating, owner-authorized, with precondition and postcondition predicates) must be distinct from predicates; F14 traceability check and F15 target-pins check as T0 predicates; receipts committed under `qa/sherlock/receipt-traces/<ID>/` or the pass does not count; rollback = abandon + re-pin + maintenance-authority key insert, never delete history.

Disagree: C3 "cut stagenet as the acceptance target" and "stop spending wallet, proof-server and agent time there". fable-2 wants it for cheap iteration. The question is cost: a stagenet call is ~15 GB and ~8 min and needs the owner's wallet; Preview needs a new wallet and faucet. I lean to fable-3 (one target, Preview) but it is an owner cost decision. ISSUE 3.

Disagree: Q1 "preview is the exit, mainnet a successor change". The brief asks this change's workflow to cover mainnet deployment and rollback. Proposal: the change specifies T4 and the mainnet operation with the exact additions fable-3 names (TIERS gains T4, `network:` field, `qa/tiers/t4.sh`, validate rule), marks them as tasks, and makes the change's *acceptance* exit the Preview qualification; the mainnet operation stays owner-authorized and blocked until `MORIARTY-COMPLETION-PROGRAM.md:269` is amended. ISSUE 1.

Concede: my I7 ("workflow.md compiles to predicates.yaml rows") is fable-3's design in fewer words; adopt fable-3's M0/M1 (re-arm the factory; land the change with `unimplemented.sh` predicates) as the first two milestones ahead of my M0 re-pin.

## What I now concede from my own position

- The leaf statement is seven lanes including `steps`, not five.
- My "universal leaf" must be measured before it is a decision; both U and F are specified with their key tables (fable-2 I3).
- M0 (factory re-arm, change lands) precedes the re-pin; the re-pin is M1.
- Assurance needs the ASR/SND clauses, not only an evidence-class list.
