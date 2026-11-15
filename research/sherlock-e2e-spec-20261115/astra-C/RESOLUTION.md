# Astra-C review: resolution by Fable-C (2026-11-15)

REVIEW.md has 36 findings (19 HIGH, 16 MEDIUM, 1 LOW). The lead of slice C decides each one. ACCEPT means the fix is in the specs or bus files now. PARTIAL means the defect is accepted and the proposed fix is changed, with the reason. No finding waits for another party. After the fixes: 121 EARS in 11 specs; `openspec validate sherlock-e2e-prototype-20261115 --strict` is valid; `ears_lint.py --require-ids --strict` gives 0 defects and 0 warnings.

| Finding | Verdict | What changed, and why |
|---|---|---|
| F-01 wrap acceptance constraints absent | ACCEPT | New WRP-012: the final-key qualification record needs a pass for LW-003 to LW-006 and LW-014. The LW IDs stay normative (reuse rule), so the fix is a gate, not a copy. |
| F-02 slot 15 omits the chunk write and the ordinary-call refusal | ACCEPT | WRP-007 now pins the read key (lane 6) and the chunk write key (lane 15), cites ANC-013 and ANC-016, and names the two final-key controls. D-072 updated. Checked against the ANC-013 text on spec-audit. |
| F-03 dependency gate rejects the stock graph | ACCEPT | LVR-002 names the four ledger crates and the revision; a `[patch.crates-io]` entry that selects that revision passes; published crates are out of the gate. Checked against `ledger-check/Cargo.toml:14-23`. |
| F-04 node-version history not recorded | ACCEPT | LVR-008 now states the first-connect record only, status PARTIAL; the version-change record is a named task for STL-006. The code keeps `st.chain ??= chain`. |
| F-05 native read-key deploy reported as built | ACCEPT | SLC-004 is PARTIAL; the evidence says the test is a runtime test and that the T17 runner installs the wrap key on every entry; a four-entry deploy path with readback is MISSING. |
| F-06 builder interface wrong | ACCEPT | SLC-006 rewritten: input is the private `sherlock-accept-bundle/1` on the box, output is `StatementPublics`; the runner file has four fields. ddd-C.md context map corrected. |
| F-07 `test_enc1` does not test the Mycroft input | ACCEPT | SLC-008: the salt comes from the prover; verification cites `MycroftCall.*`; the CSPRNG test is MISSING. D-071 confirmation corrected (16 is the whole file). |
| F-08 reuse sentence resets a stateful sequence | ACCEPT | SLC-011 applies once, at the start of a program's replay; scenario is a two-call sequence. D-084 updated. Checked against `parta.py:48-60`. |
| F-09 no readiness gate | ACCEPT | New SLC-012 (no live settle of an unbound statement) and PVW-013 (the final deploy needs the gate record of the exact key). Slice B still produces the evidence. |
| F-10 T23 Part A and per-call comparison lack coverage | PARTIAL | The defect is real: references in status text are easy to lose. The fix is not new EARS: STL-004 to STL-006 already state these checks, and proposal.md forbids a restated requirement. `bus/adopted-C.md` now lists the adopted IDs with their check state for COVERAGE.md, and SLC-010 says the dry run is not a live pass. |
| F-11 3,600 s TTL not built | ACCEPT | BAT-006 is the default when the user gives no TTL, status PARTIAL. D-074 says default, not enforced. |
| F-12 epoch close ignores an era end | ACCEPT | New BAT-010 (early close before `end_eff`) and BAT-011 (no run starts that cannot land in its era), both TO BUILD, with the budgets as parameters under D-077. |
| F-13 collection contract rules missing | PARTIAL | LDG-001 to LDG-006 are adopted in `bus/adopted-C.md`, not copied. New BAT-012: the T15 record shows a collection receipt on the final keys or lists batched collection settlement as open. I do not make a final-key collection run a condition of the prototype: the `L` collection form has no code and no row (T8c), and adding it to the critical path is not a slice C call. The gap has an owner lane. |
| F-14 registry batching omitted | ACCEPT | LDG-008 adopted; the missing update entry and the open N-8 history rule are in the gap table and in D-073 and D-075. No new rule is written. |
| F-15 foreign kind separation and once-only use omitted | ACCEPT | LDG-004 and LDG-007 adopted with the two compiled-contract tests named in D-073; the seventh DIGEST tag is handed to slice B as a key-changing dependency (BAT-012, gap table). |
| F-16 readback gate incomplete | ACCEPT | EGV-002 checks `{[], 1, 0}` and every entry key, and blocks announcement, attestation and calls. |
| F-17 unsupported hash for the key-set identity | ACCEPT | EGV-004 uses `keyset_d`; the SHA-256 choice is removed. D-077 picks the encoding (LEAD PICK for T15): the key manifest of slice B (UKS), not the wrap VK hash alone. |
| F-18 fallback selected without its premise | ACCEPT | This was my error. D-078 is rewritten: the rekey waits for an era row in the list or a recorded failure of premise A2 or A3. EGV-005 is the fail-closed gate on the rekey runner; EGV-006 is the fallback, in force only on a recorded failure. T22 tasks 2 and 3 move before T25. |
| F-19 governance verbs have no EARS | PARTIAL | GOV-001 to GOV-011 exist and stay normative; they are adopted in `bus/adopted-C.md` with the unmodelled verbs named. D-076 no longer reads as if 13 invariants cover every verb. No copy of the verbs is added. |
| F-20 consumer rules absent | PARTIAL | CRR-001 to CRR-007 adopted the same way. Retry identity now has a LEAD PICK for T15 in D-077: a retry of one logical run reuses its stored salt, so the public words stay equal; a new salt is a new run (agrees with ZK-006 and D-071). Lane: slice B (ZKE). |
| F-21 proof-version migration missing | ACCEPT | New EGV-010 from T22 O11, TO BUILD, LEAD PICK for T15 under D-077, with the trust downgrade stated. |
| F-22 authority record asserts an exclusion | ACCEPT | D-076 quotes the group confirmation, names what row T22 listed at that time, and marks the scope over O1 to O13 UNCONFIRMED; numeric values have no explicit confirmation. |
| F-23 future reviews reported as done | ACCEPT | EGV-008 and HLP-006 are TO BUILD with "MISSING until T15". |
| F-24 control verdicts overlap | ACCEPT | PVW-007 narrowed; the four results are exclusive and ordered; new PVW-012 for an included control. D-081 updated. Checked against `t10a-ledger-run.mjs:556-573`. |
| F-25 validium requirement drops most of T19 | PARTIAL | PVW-011 keeps the two ledger refusals and now states that LPC-009 holds the whole test, with the A and B dependencies. No copy of LPC-009. |
| F-26 wallet coverage too narrow | ACCEPT | WAL-002 covers derived private keys and child processes (PARTIAL: the code scrubs two values); new WAL-009 (no overwrite) and WAL-010 (directory 0700 and file 0600 checked before the phrase is read, TO BUILD on the runner's read path), both read from code only. |
| F-27 factory operation not separated from the product | ACCEPT | WAL-003, WAL-007 and HLP-007 are worded for "the pinned wallet" and "the machine that holds the wallet"; muninn and OP-PV-WALLET appear only as the factory rehearsal. New SCL-010 (the SDK runs from the user's own configuration), TO BUILD. D-080 and D-085 updated. |
| F-28 syntax validation used as semantic proof | ACCEPT | OBS-010 claims structural validation only; the semantic audit is named separately (AUDIT.md of 2026-11-15, LPC-017). |
| F-29 T21 optional and views missing | ACCEPT | OBS-012 is ubiquitous and names stages, source and per-box schedule in live and replay modes; PGUI IDs stay normative. |
| F-30 failed steps get markers | ACCEPT | FRC-001 triggers on exit status 0. |
| F-31 four attempts became four retries | ACCEPT | FRC-004: at most 4 attempts in total. Checked against `t10a-ledger-run.mjs:763-776`. |
| F-32 no contract for ambiguous submission and corrupt artifacts | ACCEPT | New FRC-013 (resolve an unknown inclusion state before another submission) and FRC-014 (hash, key and genesis check before reuse); FRC-006 retitled to match its sentence. |
| F-33 durability and freeze reduced to a clean worktree | PARTIAL | FRC-010 cites the T0 patch round trip and drops the wrong LPC-020 reference; new FRC-015 for T1 ancestry and suites. The freeze presentation and sign-off stay LPC-014 (adopted), not a new ID. |
| F-34 a substituted box artifact passes | ACCEPT | New HLP-008 (sha256 and call-vector binding before the proof is verified); HLP-002 scenario now matches its trigger. |
| F-35 context map reverses governance | ACCEPT | ddd-C.md: Governance is upstream; primary path and fallback are separate in the row. |
| F-36 public-input requirement reissued | ACCEPT | WRP-005 is withdrawn; LW-003 is the ID; WRP-010 and WRP-012 cite it. The number WRP-005 is not reused. |

Counts: 30 ACCEPT, 6 PARTIAL (F-10, F-13, F-19, F-20, F-25, F-33), 0 rejected. Every PARTIAL keeps the reuse rule of proposal.md: an existing ID is adopted by reference, with its check state in `bus/adopted-C.md`, in place of a restated requirement.

Astra's numeric cross-checks (REVIEW.md, last table) found no wrong figure in the specs. Its qualifications are in the status text: T10m and T17 (a) figures are records on the older keys, not final-key results.

After the review the lead also compared the ID list of DRAFT.md section 2 with the specs and `bus/adopted-C.md`. Added to the adopted table: DG-001 to DG-003, DG-010, LW-008 to LW-011, DCD-004, CLM, FREG. WRP-002 to WRP-004 now say that DG-001 to DG-003 state the limits and that the WRP IDs add only the refusals of the wrap tool. Astra's own new IDs (COPS-001 to COPS-017, CSDK-001 to CSDK-006) map to BAT-003, BAT-004, OBS, FRC, WAL, HLP, SCL and SLC-005, SLC-010 to SLC-012.

Side effects of the Astra lane: `git status --short` shows only the change folder and this research folder. The phase logs hold one wallet path string, from a code line of `preview-session.mjs`; no wallet file was opened. The logs (`phase1.log`, `phase2.log`) are not for the commit.
