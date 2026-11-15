# Astra-B review: resolution by Fable-B (2026-11-15)

Input: `astra-B/REVIEW.md` (32 findings: 4 BLOCKER, 20 MAJOR, 8 MINOR; verdict REWORK) and the phase 1 draft `astra-B/DRAFT.md`. Every finding is decided here by the slice lead, with the reason and the change. Result: 30 accepted in full, 2 accepted in part (R-09, R-30), 0 rejected. After the changes the slice has 127 EARS in 13 specs (126 after the review; UKS-010 was added on the integrator ruling of 10:28 about the era-check row); `openspec validate sherlock-e2e-prototype-20261115 --strict` passes and the EARS linter passes with `--require-ids --strict` (0 defects, 0 warnings).

## Findings

| Id | Sev | Decision | Reason | Change |
|---|---|---|---|---|
| R-01 | BLOCKER | ACCEPT | ZK-001 and the owner line of 2026-11-15 09:05 put every owned box inside the trust domain. My text banned moves between owned boxes and so contradicted ELA-006. | ZKE-004 now refuses a transfer only when the destination is outside the owned-box roster; a positive scenario (odin to huginn) is added. ddd-B invariant and D-010 additions follow. |
| R-02 | BLOCKER | ACCEPT | COUNTEREXAMPLES.md:56 and :61 and the manifest rows (`expect = accept`) show that AFK-F6 and the linked one-child node are honest inputs. Requiring their refusal would break honest shards. | NAG-006 keeps the 6 refusal tests (MW-F01 x3, AFK-F7, GP2-F4.2, unlinked one-child). New NAG-011 requires acceptance of the nonzero `reg_root` and the linked one-child node. |
| R-03 | BLOCKER | ACCEPT | A lead cannot waive a hard-rule item, and the "Where" form made the collection form optional. The order of work is a lead matter; the obligation is not. | GPH-008 is now unconditional ("When the node root is a SET root ...") and its status reads required for the complete prototype. D-049 is rewritten as an order decision (T25 first, T8c as a recorded second rekey before T15) that waives nothing; the option "drop the form" is listed as not chosen and owner-gated. |
| R-04 | BLOCKER | ACCEPT | SQG-001 requires G1 to G4 with G5. My gate record could mark a key set `qualified` without G5. | SSG-006 and UKS-007 name G1 to G5 and FP-8. A negative scenario (G5 names old parameters, status stays `prototype`) is added. ddd-B and D-003 additions follow. |
| R-05 | MAJOR | ACCEPT | Case A is "any box the user owns". Three fixed host names are the factory fixtures, not the rule. | BXF-001 is the owned-box roster that the user writes; scenarios for a host outside the roster (refused) and a fourth owned box (admitted). |
| R-06 | MAJOR | ACCEPT | SPL-004 makes app and leaf-layer keys per program. The probe compares N1 pre-hashes and never verifies two programs with one leaf key. | LFP-002 states the N1 pre-hash property; the second scenario uses the internal-recursive key. LFP-008 and the scope text drop "universal leaf key" as a key; ddd-B defines the LOOP term as the pre-hash property. |
| R-07 | MAJOR | ACCEPT | A manifest with `key_root` and every key file changes with each program, so "one manifest digest for all calls" cannot hold. | UKS-004 is the universal key manifest (universal key files, `keys_d`, list digest, commits). UKS-001 compares that digest and checks `key_root` per program apart. |
| R-08 | MINOR | ACCEPT | The test `rsl_sha_and_keccak_in_one_program_refused` exists (`tests/rsl_keccak.rs:150`); the constant is at `trace.rs:49`; the LIST row is 34. | LFP-003 verification names the existing test; citations corrected; no duplicate test asked. |
| R-09 | MAJOR | ACCEPT IN PART | The AIR bound is real and its refusal is correct behaviour, so the If-then requirement stays. Astra is right that it must not read as a permanent program limit. The limit itself is slice A's record D-024 (LEAD PICK for T15, temporary only), so I do not write a second decision or invent a route here. | LFP-003 refuses an oversized app shape (not "the program"); the status records the GP-001 gap, cites D-024 and names the open route (second bucket or split) with owner lane rekey-chips. D-008 additions list it as an open narrowing. |
| R-10 | MAJOR | ACCEPT | `frozen.rs:166` to `:168` on `rekey-chips` records 4,988 MEASURED and 5,121 DERIVED after T10q; 5,417 is superseded. | NAG-005 evidence and status corrected; a 5,401-column refusal check added; the complete-kernel width stays OPEN as DERIVED. D-012 additions follow. |
| R-11 | MAJOR | ACCEPT | "Read and write the anchor" for every call contradicts CLB-006 and ANC-016. | CLB-004: slot 15 for the anchor read of every call and the anchor write of a chunk call; the scenario is an ordinary call that writes no anchor. |
| R-12 | MINOR | ACCEPT | The chunk tests are in `crates/sherlock-leaf/tests/`. | CLB-002, CLB-003, CLB-005, CLB-006 paths corrected; committed lines (`:639`, `:651`) and dirty-tree lines (`:759`, `:781`, `:854`) are named apart; CLB-006 names its test. |
| R-13 | MINOR | ACCEPT | Four navigation references were wrong. | LTR-007 cites `fn pathc` at `t7_chain.rs:1775`; NAG-010 drops the wrong writer line; UKS-002 cites `stmt.rs:90` only; LTR-005 points to SQG-014 and SSG-006. |
| R-14 | MAJOR | ACCEPT | Three positive runs do not test a refusal. | LTR-004 verification adds a named negative test (a valid tape without `Tape::expose`, paired with the same tape after the call); status marks the gap. |
| R-15 | MAJOR | ACCEPT | The scenario compared the record digest with the file digest. | LTR-006 has the field `proof_sha256` over the proof bytes; the verification compares public words and key pre-hash with the verified proof; a corrupted-transfer scenario is added. |
| R-16 | MAJOR | ACCEPT | ST-007 covers keys, shapes, `plan_d` and proof values, not only files; setup calls `keys()` (`t16.rs:672`). | UKS-008 carries ST-007 in full. Verification rehashes each key file and detects added and deleted files. The in-memory `keys()` call is recorded as a gap that needs a ruling or a load from disk. |
| R-17 | MINOR | ACCEPT | The minimum rule was missing and the test named was a JSON round trip. | STM-003 states the lower of `MemAvailable` and cgroup headroom; scenarios with explicit values; a probe unit test is asked. |
| R-18 | MAJOR | ACCEPT | The code loops device kinds with one GPU engine; a calibration entry has no device index. | STM-004 status: BUILT for one-GPU boxes, OPEN for each GPU of a multi-GPU box. |
| R-19 | MAJOR | ACCEPT | "Carries SC-005 and SC-007" did not state their responses; the host cap on a discrete GPU was lost. | STM-011 states device bytes and host bytes; new STM-017 (least time and tie order, test `cx_f8_...`) and STM-018 (lower budget never faster, test `cx_f9_...`). |
| R-20 | MAJOR | ACCEPT | PF-008 was hidden in a vague sentence. The draft's response is better. | STM-015 states the three policy outcomes of PF-007. New STM-019 states the early-wrap admission rule and the tail formula, with a one-byte-short refusal test. |
| R-21 | MAJOR | ACCEPT | A new run draws a new salt, so "one program and input" does not fix the claim. | ELA-001 and ELA-005 hold for one prepared run of one deployment; a scenario for two fresh runs (claims differ) is added; ddd-B defines "prepared run". |
| R-22 | MAJOR | ACCEPT | Scale-down and CPU fallback must not promise a proof when nothing fits (STM-012). | ELA-003 and ELA-004 carry the precondition "While at least one leaf shape fits"; a refusal scenario is added. |
| R-23 | MINOR | ACCEPT | huginn and odin picked the same schedule (T16R:60). | ELA-005 scenario requires a valid schedule per box and equal claims, not different schedules. |
| R-24 | MAJOR | ACCEPT | The cited function is the app prover; RESULTS-CDE.md:52 says the leaf layer proves on the CPU; the stock verifier cannot show the device. | GPV-001 is the app proof on the GPU with a backend record. New GPV-007 (leaf layer on the GPU) is OPEN. |
| R-25 | MINOR | ACCEPT | Unset means enabled; `0` or `off` means CPU. | GPV-004 and GPV-005 triggers state the values; an off scenario is added. |
| R-26 | MAJOR | ACCEPT | Reusing PRD-002 while choosing no pool left two texts in conflict; `slot_reuse` needs no pool. | D-040 now amends the pool obligation of PRD-002 for the prototype (LEAD PICK for T15). PDM-005 cites it. New PDM-009 requires cross-request isolation without condition, verified by `slot_reuse` and a socket-level check. |
| R-27 | MAJOR | ACCEPT | A count floor passes when a refusal case is replaced by a copy of a positive case. | GPH-007 requires a GP case map (cases and expected outcomes per GP-001 to GP-004); the count floors stay as a smoke check. |
| R-28 | MINOR | ACCEPT | The owner rule has a SIGKILL step after 30 s. | New BXF-013 with a recheck of pid, parent and role before SIGKILL. |
| R-29 | MINOR | ACCEPT | The factory rule "only through muninn" was not stated. | New BXF-014, marked as a factory-fleet rule apart from BXF-001. |
| R-30 | MAJOR | ACCEPT IN PART | ANC-017, ANC-018 and ANC-019 state bindings ("SHALL bind", "SHALL compute"). CLB-002, CLB-003 and CLB-005 are the If-then refusals that test them; the ears skill (rule 7) asks for a separate requirement for each unwanted behaviour. So I keep the IDs. Astra is right that the relation was not stated. | Each of CLB-002, CLB-003, CLB-005, CLB-006 now says "refusal side of ANC-0nn, which stays the normative text for the binding"; design-B.md states the rule. No ANC text is superseded. |
| R-31 | MAJOR | ACCEPT | `eq_zk` proves twice on one GPU object; a cross-device difference alone passes with a fixed stream per device. | ZKE-007 requires two proofs on each backend and pairwise unequal byte strings over one statement; a CPU leg is asked as new work. |
| R-32 | MAJOR | ACCEPT | `HEIGHTS` is an ordered AIR height vector per shape, not a list of shard heights; chip set 3 has no named vector. | LFP-005 requires the ordered AIR height vector of the admitted shape; a swapped-height scenario; the chip set 3 shape is a named gap. D-018 additions follow. |

## Items taken from the phase 1 draft (before the review)

| Draft item | Decision | Change |
|---|---|---|
| G-03 rekey-chips is not the extended-lock tree | ACCEPT | New UKS-009: the keygen tree is a descendant of every commit of the rekey list. |
| G-07 mixed-hash refusal against GP-001 | ACCEPT | LFP-003 status (see R-09). |
| G-08 "differ byte for byte" reads as every byte differs | ACCEPT | ZKE-007 uses "byte strings are not equal". ZK-015 is reused text of another change; the wording fix for it is listed as a gap for the spec-audit lane. |
| G-17 "GPU leaf" hides CPU work | ACCEPT | ELA-007 records the device of each stage; GPV-001 and GPV-007 split app and leaf layer. |
| G-01, G-02, G-04 to G-06, G-09 to G-16, G-18 | already covered | SSG-006 and SQG-014 to SQG-016 (G-01); UKS-009, RKY-002 (G-02); CLB-002, CLB-003 (G-04); BXF-001, ZKE-004 (G-05); CLB-004, CLB-008 (G-06); ZKE-001 (G-09); NAG-005 (G-10); GPH-002 to GPH-008 (G-11); STM-014 (G-12); ELA-001 to ELA-006, LFP-008 (G-13); LFP-002 (G-14); CLB-007 (G-15); BXF-002 (G-16); D-030 (G-18). |

## Changes after the review that Astra did not see

- UKS-010 (rekey list rows, with the era-check row of EGV-005), LFP-010 reworded to the digest form of the new ERS-012, D-040 authority text (the pick departs from the letter of OD-13 and goes to T15), D-042 and D-050 with one authority label each, citations to the spec audit changed to this branch (95643d85f).
- Checks run by the lead on text that came from the review: the `expect` values of `tests/quint-regressions/manifest.json` (MW-F01 x3, AFK-F7, GP2-F4.2 refuse; AFK-F6, AFK-F9 accept), `rsl/air.rs:199` to `:202`, `profile.rs:324` to `:326`, `t7_chain.rs:109` and `:893` on `openvm-t14`, `t16.rs:672`, `log.md:4117`. A scan of every requirement and decision ID in the 13 specs and the bus files found no dangling ID (ANC-012 and ELD-007 are on branches `t20-batched` and `t22-governance` and are cited as such).

## Open after the resolution

These are gaps of the system, recorded in the Status scenarios with an owner lane. They are not open review findings.

1. LFP-010 to LFP-012: the R_SL app proof does not bind its statement, `prog_d` or `input_d` (HIGH; rekey-chips; before T25).
2. UKS-003: `KEYDAG_IDX = 3` collides with the planned R_SL `key_root` id 3 (rekey-chips; before T25).
3. UKS-009, CLB-008: one integrated keygen tree with the extended lock, FP-3, slot 15 and T11 (T25).
4. CLB-002, CLB-003: FP-3 is not committed; one test passes without a refusal (fp3).
5. NAG-007: GP2-C2.1 is CONFIRMED with no fix (t10q).
6. GPH-001, LFP-003: GP-1 at the proving layer (5 + 2 of 120 interpreted rules; both hash kits in one program).
7. GPH-004, GPH-008, LPC-002, LPC-003: T8c (X8 verifier, `L` over SET, STACKS, VALUE DIGEST) has no LOOP row.
8. STM-014: the F-3 menu code is still in `sched.rs` and the part (b) runner calls it.
9. SSG-003 to SSG-006: the T12 gate has no run; G3 legs are not built; 8 findings NOT ATTEMPTED.
10. ZKE-001, ZKE-005, ZKE-006, PDM-009: the T11 code and tests are untracked and have no recorded run.
11. GPV-005, GPV-007: `L` and the leaf layer on the GPU.
12. BXF-001, BXF-007, BXF-014, ZKE-004: no script enforces the roster, the digest check or the route.
