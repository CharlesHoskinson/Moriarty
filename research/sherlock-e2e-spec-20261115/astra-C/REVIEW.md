# Astra-C phase 2 adversarial review

Reviewed on 2026-11-15. The eleven capability files contain 107 EARS requirements. This review also covers the two requested C bus records and compares them with Astra-C DRAFT.md sections 1, 2 and 5.

The review used source reads only. No build, test, prover, network request, git command or transaction was run. No wallet or key-material file was opened. A cited historical run is evidence of that run, not a new verification result.

## Source paths

- ROOT = /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec.
- WT = /home/hoskinson/Projects/Moriarty-wt.
- MAIN = /home/hoskinson/Projects/Moriarty.
- S = ROOT/openspec/changes/sherlock-e2e-prototype-20261115/specs. A capability name below means S/sherlock-e2e-NAME/spec.md.
- FINAL = ROOT/openspec/changes/sherlock-processor-final-20261111.
- COMPLETE = WT/spec-audit/openspec/changes/sherlock-completion-20261115.
- GOV = WT/t22-governance/openspec/changes/sherlock-governance.
- T22 = WT/t22-governance/research/sherlock-t22-governance-20261114.
- T20 = WT/t20-batched/research/sherlock-t20-batched-20261114.
- LDG = WT/t20-batched/openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-ledger-contract/spec.md.
- T17 = WT/t17-preview/research/sherlock-t17-preview-20261114.
- RUNNER = WT/t17-preview/tools/sherlock-sdk/t10a-ledger-run.mjs.
- ARGS = WT/t23-args/tools/t23-suite/accept_args.py.
- ARGTEST = WT/t23-args/tools/t23-suite/test_accept_args.py.
- MAP = WT/t23-args/research/sherlock-t23-args-20261115/MAP.md.
- LOOP = MAIN/research/sherlock-tested-design-20261113/LOOP.md.
- LOG = MAIN/llm-wiki/wiki/log.md.
- DDD and MADR = ROOT/research/sherlock-e2e-spec-20261115/bus/ddd-C.md and decisions-C.md.
- DRAFT = ROOT/research/sherlock-e2e-spec-20261115/astra-C/DRAFT.md.

For missing coverage, a status-line reference or a DDD invariant is not treated as a replacement for an executable requirement and its verification mapping. The fix can explicitly adopt the existing requirement by ID; it need not copy that requirement under a new ID.

## Findings

### F-01 — Wrap acceptance constraints are absent

- Class: MISSED
- Severity: HIGH
- IDs: WRP-001, WRP-005; LW-003, LW-004, LW-005, LW-006, LW-014
- Capabilities: wrap
- Wrong: Producing a proof with 1,026 public inputs does not require native log replay, canonical values, bounded reductions, zero padding, reservation coverage, layout binding or the DAG commitment check. The wrap also lacks the keygen shape/query-vector and log-capacity refusal requirements. These are C's wrap checks, including T4's full PATH replay.
- Source: FINAL/specs/sherlock-l-and-wrap/spec.md:26,37,48,55,123; LOOP:21; DRAFT:67,73,85,123. The reviewed wrap requirements are at wrap:4,60.
- Fix: Explicitly retain these LW IDs with their mutant and keygen-refusal checks. State lane 0, lane 1 and unused-lane constraints. Keep the final-key checks open until measured; a count of public inputs is insufficient.

### F-02 — The slot-15 rule omits chunk writes and ordinary-call write refusal

- Class: MISSED
- Severity: HIGH
- IDs: WRP-007; ANC-013, ANC-016; D-072
- Capabilities: wrap
- Wrong: WRP-007 pins only the anchor read. A chunk write can target another slot without violating its sentence. No C requirement rejects the anchor-write template in an ordinary call.
- Source: wrap:88; COMPLETE/specs/sherlock-anchor-chunks/spec.md:4 pins both read-key lane 6 and chunk-write-key lane 15 to slot 15; :37 requires ordinary-call refusal. LOOP:32 records the NT-1 defect.
- Fix: Reuse ANC-013 in full and retain ANC-016. Add final-key controls for a changed chunk write key and an ordinary call with anchor writes. Map D-072 to both read and write constraints.

### F-03 — The dependency gate rejects the existing stock verifier graph

- Class: WRONG
- Severity: MEDIUM
- IDs: LVR-001, LVR-002
- Capabilities: ledger-verifier
- Wrong: The scenario and proposed gate require every midnight-* package to come from the ledger git revision. The lockfile has registry dependencies such as midnight-circuits and midnight-curves. The negative scenario also rejects any patch entry, although the cited manifest uses patch entries to select the unchanged pinned ledger crates.
- Source: ledger-verifier:11,20,23; ROOT/tools/sherlock-wrap/ledger-check/Cargo.toml:19 through :23; its Cargo.lock:1508 through :1535.
- Fix: Define the set of ledger-owned crates that must resolve to the exact stock revision. Permit source-selection patches to that revision. Reject modified source, local overrides and wrong revisions. Do not reject required published cryptography crates by name prefix.

### F-04 — The claimed node-version history is not recorded

- Class: WRONG
- Severity: MEDIUM
- IDs: LVR-008
- Capabilities: ledger-verifier
- Wrong: The scenario requires both the old and new node versions and a change report. The cited code initializes st.chain only when absent and checks only the genesis hash. A later node version is not added to that saved record. BUILT does not cover the scenario.
- Source: ledger-verifier:102 through :113; RUNNER:225 through :229, especially st.chain ??= chain. COMPLETE/specs/sherlock-suite-settlement/spec.md:64 through :68 requires the Part A rerun and hard-fork comparison split.
- Fix: Record each observed node/runtime version with its height and time, preserve the prior observation, and test a same-genesis version change. Mark that behavior TO BUILD until evidence exists.

### F-05 — Native read-key deployment is reported as built from a runtime test

- Class: STATUS
- Severity: HIGH
- IDs: SLC-004
- Capabilities: ledger-contract
- Wrong: The requirement concerns the deploy runner. Its test constructs an in-memory contract state and tests callee-key resolution. The cited Preview runner installs the same wrap key on every entry and checks that same condition at readback. The runtime test does not establish the required deployment behavior.
- Source: ledger-contract:46,48,57; WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:75,191; RUNNER:203,646 through :649; T17/RESULTS.md:17 lists the old three-entry deployment.
- Fix: Mark PARTIAL. Require the final four-entry deploy path to install wrap keys on writes and compiled native keys on reads, then read each back and run a real consumer call.

### F-06 — The argument-builder interface and output are wrong

- Class: WRONG
- Severity: HIGH
- IDs: SLC-006; DDD StatementPublics
- Capabilities: ledger-contract
- Wrong: StatementPublics contains four public digests. The implemented builder takes a private sherlock-accept-bundle/1 with program, input and outcome preimages or packed operands. The three contract arguments are also not the complete CLI file: ledger_args_bytes emits c_p, input_pub, outcome_pub and anchor. The current DDD edge assigns private derivation to an interface that cannot perform it.
- Source: ledger-contract:74,85; DDD:23; MAP:43,49 through :51; ARGS:287 through :301,321 through :328.
- Fix: Name the private box-local bundle as the builder input, and StatementPublics as its public result. Distinguish the three acceptRun arguments from the four-field runner file. Keep the private derivation on the owned proving machine.

### F-07 — test_enc1 does not verify the claimed Mycroft input construction

- Class: UNTESTABLE
- Severity: MEDIUM
- IDs: SLC-008; D-071
- Capabilities: ledger-contract
- Wrong: The Verification line says test_enc1 verifies vd(tape), zero fuel/stack and CSPRNG salt. That test uses VAL_NONE, fixed salt 1..8, fuel 2^64, stack 2^16 and Rejected(7). The Mycroft adapter accepts salt as an argument; it does not generate it. BUILT and D-071's observed confirmation overstate the evidence. D-071 also attaches the historical whole-suite count of 16 passes to MycroftCall.* alone.
- Source: ledger-contract:102,104,113; MADR:85; ARGTEST:59 through :76; ARGS:347 through :358; MAP:73 assigns salt generation to the prover.
- Fix: Assign fresh salt generation to the prover, retain the adapter's zero defaults, cite the Mycroft adapter tests for tape mapping, and mark the production CSPRNG test MISSING. Keep the proof-binding gap separate.

### F-08 — The reuse sentence can reset a stateful scenario before every call

- Class: WRONG
- Severity: MEDIUM
- IDs: SLC-011; D-084
- Capabilities: ledger-contract
- Wrong: The sentence orders a fresh deployment for a suite call whenever state differs from deployment state. It omits the scenario's condition that the call needs deployment state. Applied to a sequence, the first state change forces a reset before the next call. The cited Part A code decides reuse once per program before replay.
- Source: ledger-contract:144 versus :150; WT/t23-suite/tools/t23-suite/parta.py:48 through :67, especially :56; LOOP:45; MADR:645 names suites that depend on earlier calls as a reconsideration.
- Fix: Apply the reuse gate at the start of the baseline replay, or when the call's declared pre-state requires deployment state. Preserve state between dependent calls. Test a two-call counter sequence.

### F-09 — No mandatory readiness gate prevents settlement of an unbound statement

- Class: MISSED
- Severity: HIGH
- IDs: SLC-007 through SLC-010, PVW-009, HLP-003; STL-001, STL-002, RKY-003 through RKY-005
- Capabilities: ledger-contract, preview-deployment
- Wrong: Gaps appear in evidence text, but no requirement refuses live settlement until program identity, INIT/input, outcome/effect binding and the private bundle are complete. Checking the wrap's validity and comparing host-computed words does not prove those bindings. The final deploy trigger also lacks an explicit receipt/manifest gate tying the installed key to the new-key row, VK-size, 100-bit and zero-knowledge qualification results.
- Source: MAP:81 through :90 explicitly keeps live settle off; LOOP:47 requires rerunning the gates on final keys; /home/hoskinson/.cache/lead/CHECKLIST-20261115.md:12 through :14 retains these dependencies; DRAFT:739,967,968. Reviewed PVW-009 is at preview-deployment:116.
- Fix: Adopt STL-001/STL-002 and the RKY qualification references as fail-closed prerequisites. Have C verify the completed gate record for the exact deployed key and reject missing or mismatched evidence. B still owns producing the keys and proving evidence.

### F-10 — T23 Part A and per-call settlement comparison lack binding coverage

- Class: MISSED
- Severity: HIGH
- IDs: SLC-010, BAT-004, FRC-012; STL-004, STL-005, STL-006
- Capabilities: ledger-contract
- Wrong: Counts, refused-call handling and restart skipping do not require the native 33-program rerun, the hard-fork split, the successful-call runs-map readback, or the comparison of each settled outcome with the native transcript. References to STL IDs in status text leave these C-owned actions without their own verification mapping.
- Source: LOOP:45; COMPLETE/specs/sherlock-suite-settlement/spec.md:40,52,64; DRAFT:255 through :274.
- Fix: Explicitly adopt STL-004 through STL-006. Require tx/hash/block and runs-map readback per included call, zero mismatches for completion, blocked reasons, and the Part A version-aware report. Do not treat the zero-mismatch dry run as a live pass.

### F-11 — Exact 3,600-second TTL is not built for the general runner

- Class: STATUS
- Severity: MEDIUM
- IDs: BAT-006; D-074
- Capabilities: batched-settlement
- Wrong: BAT-006 says every Preview intent has a 3,600-second TTL and reports BUILT. The runner defaults to 14,400 seconds online. t17.sh supplies 3,600 only when the environment variable is absent and permits an override. The requirement and its status exceed the observed T17 configuration.
- Source: batched-settlement:74,79 through :85; RUNNER:192 through :194; T17/t17.sh:19.
- Fix: Mark PARTIAL. State whether 3,600 is the selected default or an enforced value, preserve D-074's proposed authority, and test direct runner invocation plus an explicit override.

### F-12 — Epoch closure and intent admission ignore an announced era end

- Class: MISSED
- Severity: HIGH
- IDs: BAT-001, BAT-006, EGV-008; ERT-009
- Capabilities: batched-settlement, governance
- Wrong: BAT-001 waits for the time or capacity limit even when the resulting collection cannot land before the era ends. No requirement stops starting a run too late or clamps its expiry to the era boundary.
- Source: batched-settlement:4; T22/OPEN-QUESTIONS.md:70 requires admission against end_eff and expiry before it; :86 requires early epoch close before activation; DRAFT:821.
- Fix: Add an era-aware admission/early-close rule with parameterized proving and inclusion budgets. Preserve the normal 3,600-second/capacity limit as an upper bound. Test a near-boundary epoch and an intent that would expire after end_eff.

### F-13 — The collection contract's state and acceptance rules are missing

- Class: MISSED
- Severity: HIGH
- IDs: SLC-001, BAT-001, BAT-007, BAT-009; LDG-001 through LDG-006
- Capabilities: ledger-contract, batched-settlement
- Wrong: Listing four entries, routing jobs and publishing paths does not specify collection verification against [reg_pub, claims_root], rejection of duplicate/unaccepted roots, the current-registry check, no per-claim state, or zero writes for an empty epoch. BAT-009 prevents an unsupported submission but supplies no positive final-collection acceptance gate.
- Source: LDG:4,20,27,38,45; T20/RESULTS.md:70 states L-over-SET proving is absent; LOOP:42; DRAFT:318 through :357.
- Fix: Explicitly retain the amended LDG requirements and their negative tests. Mark real collection proof-to-ledger verification MISSING. Require a successful final-key batched run for prototype completion, with empty-epoch and bounded-state checks.

### F-14 — On-ledger foreign registry batching is omitted

- Class: MISSED
- Severity: HIGH
- IDs: LDG-008; D-073, D-075
- Capabilities: batched-settlement
- Wrong: None of BAT-001 through BAT-009 covers registry epochs, one root update for many key changes, zero writes without changes, or registered-key membership. D-075 states later-collection revocation behavior without exposing the missing update implementation and unresolved history/grace rule.
- Source: LDG:59 through :63; T20/RESULTS.md:143 says reg_root is sealed with no update entry and history/revocation remains N-8 work; :155 names the existing model; LOOP:42; DRAFT:365.
- Fix: Adopt LDG-008 with a MISSING contract/update test and an explicit history-rule gap. Preserve collection finality. Do not turn the model's current-root rule into evidence of a deployed revocation mechanism.

### F-15 — Foreign claim kind separation and once-only consumption are omitted

- Class: MISSED
- Severity: HIGH
- IDs: LDG-004, LDG-007; D-073
- Capabilities: ledger-contract
- Wrong: BAT-007 has a run-membership scenario, but no requirement defines the foreign leaf, rejects run/foreign substitution, or prevents using the same claim through runs and multiple collection roots. These are C's on-ledger consumption obligations.
- Source: LDG:27,52; WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:162,202,214,227,236; T20/RESULTS.md:95,97; DRAFT:359.
- Fix: Reuse LDG-004/007. Map the kind-separation and cross-source replay controls, including the zero-padded slot control. Hand the seventh DIGEST tag to B as a key-changing dependency; do not claim that circuit delta exists.

### F-16 — The sealed-deployment readback gate is incomplete

- Class: MISSED
- Severity: HIGH
- IDs: EGV-002; ERT-002, ELD-002
- Capabilities: governance
- Wrong: EGV-002 checks committee and threshold only. It omits counter == 0 and every installed entry key. EGV-001's requested constructor values do not establish what was actually deployed.
- Source: governance:18; GOV/specs/sherlock-era-table/spec.md:11; GOV/specs/sherlock-era-ledger/spec.md:11; T22/REPORT.md:8,9.
- Fix: Adopt the complete ERT-002 readback for the era table and every program ledger. Compare entry keys to the expected manifest, including native read keys. Block announcement, attestation and use after any mismatch.

### F-17 — Era key-set identity is replaced with an unsupported hash choice

- Class: AUTHORITY
- Severity: MEDIUM
- IDs: EGV-004; ERT-003
- Capabilities: governance
- Wrong: EGV-004 selects SHA-256 of the wrap VK as the table's key-set identity. The cited requirement and model concern a universal key-set digest, not that encoding. No record establishes that the wrap-VK hash uniquely commits to the complete governed key set or resolves per-program ledger identity.
- Source: governance:46,48; GOV/specs/sherlock-era-table/spec.md:22,43; T22/REPORT.md:12,15; T22/OPEN-QUESTIONS.md:18.
- Fix: Use the existing keyset_d term and define its authenticated manifest/encoding as an explicit decision. If a wrap-VK hash is chosen, label it LEAD PICK for T15 and state the required binding proof/check. Keep the uniqueness test's encoding coverage MISSING.

### F-18 — The fallback is selected without its required failed premise

- Class: AUTHORITY
- Severity: HIGH
- IDs: EGV-005, EGV-006; D-078; ELD-003, ELD-007
- Capabilities: governance
- Wrong: D-078 selects zero era calls because the rekey list lacks the change and A1–A3 are untested. ELD-007 permits fallback where A2 or A3 fails. Absence and untested status are not failure. The source instead requires the added words to be bound before T12. The draft changes that condition and defers the ledger check to a later era.
- Source: MADR:361 through :386; governance:60,71; GOV/specs/sherlock-era-ledger/spec.md:18,50; T22/REPORT.md:10,58; T22/OPEN-QUESTIONS.md:13; LOG:4107.
- Fix: Restore the conditional primary path. Reconcile the era-tail dependency with the single rekey list before keygen. Enable the fallback only with the specified premise result or an explicit superseding ruling, and retain its stricter first-use rule and missing verification.

### F-19 — Governance verbs and replay protection have no EARS coverage

- Class: MISSED
- Severity: HIGH
- IDs: EGV-007; GOV-001 through GOV-011; D-076
- Capabilities: governance
- Wrong: Refusing an in-place key change does not specify distinct-current-roster quorum, delayed announcement, permissionless activation, lazy activation before soundness_end, irreversible retirement, cancellation, delayed rotation, attestation or signed-message replay protection. D-076's verb list and the DDD aggregate do not supply their scenarios or verification mapping.
- Source: GOV/specs/sherlock-governance-verbs/spec.md:4,11,18,25,32,39,50,57,64,71,78; T22/REPORT.md:53 expressly excludes attestation, replay and cancel_rotation from the model.
- Fix: Adopt the existing GOV IDs with explicit coverage and open checks. Preserve parameterization. Do not represent the 13 modeled invariants as coverage of the unmodeled verbs.

### F-20 — Consumer authentication and cross-era replay rules are absent

- Class: MISSED
- Severity: HIGH
- IDs: EGV-006; CRR-001 through CRR-007
- Capabilities: governance
- Wrong: Calling active(era_id) alone does not authenticate the supplied ledger or lineage. It also does not specify the stable nullifier, profile A versus profile B, or consumer-only writes. A fresh salt on a retry can change input_pub and outcome_pub, so the same-logical-run boundary must be defined for cross-era replay tests.
- Source: GOV/specs/sherlock-consumer-read/spec.md:4,11,18,32,39,46; T22/OPEN-QUESTIONS.md:16,20,22; MAP:73; DRAFT:977.
- Fix: Adopt the CRR IDs and the attested-lineage dependency. Require rejection of a forged ledger address and duplicate logical use across eras. Specify retry/salt identity at the A/B/C interface without inventing a new owner ruling.

### F-21 — Proof-version migration is missing

- Class: MISSED
- Severity: MEDIUM
- IDs: D-076; T22 O11
- Capabilities: governance
- Wrong: A ledger hard fork that stops verifying the old proof version is only a reconsideration concern. No requirement or gap covers migration while the old version still works, predecessor linkage, or the explicitly marked snapshot trust downgrade.
- Source: T22/OPEN-QUESTIONS.md:21; DRAFT:827; governance contains no corresponding requirement.
- Fix: Add the migration path and its source authority, with a MISSING migration drill. Keep the trust downgrade visible and do not imply that a sealed contract can acquire a new verifier key.

### F-22 — The authority record asserts an exclusion the confirmation does not state

- Class: AUTHORITY
- Severity: MEDIUM
- IDs: D-076, D-077
- Capabilities: governance
- Wrong: D-076 says O1–O13 are not included in the owner's confirmation. The later log confirms “T22 governance picks” as a group and does not state that exclusion. The earlier open-questions report cannot establish the scope of the later confirmation. Exact numeric P1–P6 values remain a distinct evidence question.
- Source: MADR:291; T22/OPEN-QUESTIONS.md:3,27; LOG:4107; DRAFT:905,908 and the authority-conflict table.
- Fix: Record the group confirmation exactly. Distinguish confirmed architecture picks from numeric values without explicit confirmation. Mark any unresolved scope UNCONFIRMED; do not assert that the owner excluded all O1–O13.

### F-23 — Future reviews are reported as completed verification

- Class: STATUS
- Severity: MEDIUM
- IDs: EGV-008, HLP-006; D-077
- Capabilities: governance, helper-machines
- Wrong: EGV-008 reports BUILT and says the T15 record review exists, while D-077 calls that review planned. HLP-006 similarly labels its T15 review as existing. An accepted policy is not an observed future review.
- Source: governance:104,113; helper-machines:76,85; MADR:342; COMPLETE/specs/sherlock-loop-completion/spec.md:146 through :155 and LOOP:48 show T15 waits.
- Fix: Separate “policy specified” from implementation and verification status. Mark the T15 review MISSING/planned, and attach the actual freeze record only when it exists.

### F-24 — Control verdict rules overlap

- Class: WRONG
- Severity: MEDIUM
- IDs: PVW-007, PVW-008; D-081
- Capabilities: preview-deployment
- Wrong: A timeout has no node JSON-RPC error. PVW-007 therefore requires no-node-verdict while PVW-008 requires not-included-timeout. A wrongly included control also has no error, but should retain included as the observed result while failing qualification.
- Source: preview-deployment:88,102; RUNNER:560,572 explicitly distinguishes included, timeout, node refusal and transport failure.
- Fix: Specify mutually exclusive outcomes in that order. Narrow PVW-007 to a failed submission without either an inclusion result, an inclusion timeout or a node refusal. Keep the qualification verdict separate from the observed submission result.

### F-25 — The validium requirement drops most of T19's ledger acceptance test

- Class: MISSED
- Severity: HIGH
- IDs: PVW-011; LPC-009
- Capabilities: preview-deployment
- Wrong: Refusing a wrong program or stale root does not require opening the hidden input/outcome commitments, accepting the correct new root, preserving state for an invalid batch, chaining multiple batches, or checking equal final proof size across batch counts.
- Source: preview-deployment:144; LOOP:41; DRAFT:282,983.
- Fix: Explicitly adopt LPC-009 and map C's state-root acceptance/readback and refusal checks. Name A's statement opening and B's GP-2/GP-4 proof as dependencies. Keep the live Preview run MISSING.

### F-26 — Wallet preservation and secret coverage are narrower than the hard boundary

- Class: MISSED
- Severity: HIGH
- IDs: WAL-001, WAL-002, WAL-008; D-080
- Capabilities: wallet
- Wrong: Reading a mode-0600 file does not require private parent-directory permissions or preservation of an existing wallet. Replacing the phrase and master seed does not cover derived signing keys and other private wallet material in logs/error records. The repository prohibition does not close that logging gap.
- Source: wallet:4,18,102; WT/preview-deploy/tools/sherlock-sdk/preview-session.mjs:35 through :50 uses a 0700 directory and exclusive creation; WT/preview-deploy/tools/sherlock-sdk/preview-session.test.mjs:23,65 tests no overwrite. ROOT/research/sherlock-e2e-spec-20261115/BRIEF.md:20 says wallet handling is never logged; supplied AGENTS.md requires preserving existing wallets and keys.
- Fix: Require no overwrite/regeneration of an existing wallet, permission checks on secret files and directories, and no private wallet material on any output surface. Use synthetic marker secrets in error, child-process and receipt tests; do not inspect real secrets.

### F-27 — Factory-only operation is not separated from the public developer path

- Class: MISSED
- Severity: MEDIUM
- IDs: SCL-001; D-080, D-085
- Capabilities: sdk-cli
- Wrong: The SDK is defined as research commands behind the maintainer runbook, while WAL-003/007 and HLP-007 impose OP-PV-WALLET and muninn. No developer workflow or scope rule says how a user compiles, proves and deploys with their own machine and wallet without the factory.
- Source: sdk-cli:4; wallet:32,88; helper-machines:88; MADR:669 through :683; supplied AGENTS.md “Product boundary”; DRAFT:833.
- Fix: Scope the pinned wallet and muninn rules to the authorized factory rehearsal. Require a clean-environment owned-machine/user-wallet workflow for the public SDK. A seven-command interface is acceptable; a factory dependency is not.

### F-28 — Strict syntax validation is used as proof of semantic consistency

- Class: UNTESTABLE
- Severity: MEDIUM
- IDs: OBS-010
- Capabilities: observability
- Wrong: The EARS sentence runs strict OpenSpec validation, but the scenario additionally promises that no architecture row contradicts a closure test. The named validation and wiki lint do not demonstrate that semantic relation. The historical T2 record contains both checks as separate completion conditions.
- Source: observability:130 through :141; LOOP:19; DRAFT:986 through :1006 lists concrete semantic conflicts that syntax validation alone does not resolve.
- Fix: Separate structural validation from a source-to-closure evidence audit. Name the audit artifact/check for semantic consistency or mark it MISSING. Do not label the second result BUILT from the first command.

### F-29 — T21 is optional and its source/elasticity views are missing

- Class: AUTHORITY
- Severity: MEDIUM
- IDs: OBS-012; LPC-011
- Capabilities: observability
- Wrong: “Where the prover GUI is installed” permits satisfying the requirement by omitting the GUI. The owner requested a working page with code/source views and a view of elastic proof flow; showing five stages in order does not cover that scope.
- Source: observability:158,165; LOOP:43; DRAFT:815.
- Fix: Require delivery of the T21 page and its live/replay modes. Explicitly adopt LPC-011 and the applicable existing GUI IDs for source navigation and elasticity, keeping B's bridge dependency separate.

### F-30 — Failed steps would receive success markers

- Class: WRONG
- Severity: MEDIUM
- IDs: FRC-001
- Capabilities: recovery
- Wrong: A failed step also “finishes.” FRC-001 requires writing its .ok marker, which allows the next run to skip failed work. The code writes call.ok only after exit status 0.
- Source: recovery:4; T17/t17.sh:26,91,114.
- Fix: Trigger the marker only after successful completion and required durable readback. Require no success marker after failure/interruption. Test a failure between submission and readback.

### F-31 — Four attempts became four retries

- Class: WRONG
- Severity: MEDIUM
- IDs: FRC-004
- Capabilities: recovery
- Wrong: The requirement allows four retries after the first failure, or five attempts. The source sets four total attempts and stops after the fourth failure. The historical rerun needed zero retries, so it does not exercise the bound.
- Source: recovery:46,48; RUNNER:765 through :775; T17/RESULTS.md:42.
- Fix: Say “at most four total attempts, with 30 seconds and resync between attempts” for the recorded default. Mark the attempt-bound fault test MISSING.

### F-32 — Ambiguous submission and corrupt resume artifacts have no recovery contract

- Class: MISSED
- Severity: HIGH
- IDs: FRC-002, FRC-003, FRC-005, FRC-006, FRC-012
- Capabilities: recovery
- Wrong: The requirements cover a known refusal, a saved unsubmitted call and an existing readback. They do not define pending/sending/timeout resolution, indexer lag, expiry, or validation of saved proof/state artifacts before reuse. FRC-006's title promises no resubmit of an included call, but its sentence only blocks the refusals step. The implementation can mark an included-but-not-indexed call refused after 60 seconds.
- Source: recovery:18,32,60,74,158; RUNNER:618 through :635,676,683 through :726; DRAFT:789,809,979. The deploy branch resumes an indexed address but can take a new-deploy branch when that lookup is absent.
- Fix: Define an explicit pending/included/refused/expired state machine. Preserve submitted bytes and transaction identity, resolve ambiguous inclusion before a fresh submission, and define safe expiry recovery. Validate artifact hashes, key identity, chain identity and claim/state before reuse. Add crash/indexer-lag/corruption tests; do not describe this as built idempotency.

### F-33 — Durability, integration and the freeze are reduced to a clean worktree

- Class: MISSED
- Severity: HIGH
- IDs: FRC-010, FRC-011; LPC-014; T0, T1, T15
- Capabilities: recovery
- Wrong: git status cannot establish exported patch round trips, integration ancestry, or passing required suites on the integrated tree. No C requirement preserves T1's integration record or presents the interface freeze for owner sign-off. Holding pushes and listing open parameters do not perform the freeze.
- Source: recovery:130,132,144; LOOP:17,18,48; COMPLETE/specs/sherlock-loop-completion/spec.md:147,155. FRC-010 also cites LPC-020, which is about box-queue rows, not durability (:212 through :219).
- Fix: Retain separate T0 round-trip evidence, T1 integrated-tip/required-suite evidence and LPC-014 freeze presentation/sign-off. Correct the LPC-020 mapping. These are future verification requirements; this review does not run git or tests.

### F-34 — A valid substituted box artifact can pass the helper check

- Class: MISSED
- Severity: HIGH
- IDs: HLP-001, HLP-002; DDD BoxResult
- Capabilities: helper-machines
- Wrong: HLP-002's scenario starts with a SHA-256 mismatch, but its EARS trigger is verifier refusal. A substituted valid proof can verify and still differ from the box report or belong to another job. None of the helper requirements rejects mismatched transfer hashes and job/key/call identity before KEEP.
- Source: helper-machines:18,23 through :25; DDD:30 defines BoxResult with path and sha256; LOOP:96 requires transfers with path and sha256; DRAFT:803.
- Fix: Add a separate transport/provenance check before native verification. Bind the received artifacts to the requested job, key and public call vector. Test a corrupted file and a valid proof from another job.

### F-35 — The context map reverses governance's influence

- Class: WRONG
- Severity: LOW
- IDs: DDD context-map row Settlement/Governance
- Capabilities: governance
- Wrong: The table labels Settlement upstream and Governance downstream, while its own text says Governance is upstream. Under D-078 the table controls consumer acceptance, not the prototype ledger's continued acceptance, so “decides which ledgers accept” is also inaccurate for that proposed fallback.
- Source: DDD:20,27,33; MADR:383 through :386; EGV-005/006 at governance:60,74.
- Fix: Put Governance upstream of the consumers/settlement policy it constrains. Distinguish the primary inclusion-time ledger check from fallback consumer-use checks in the map.

### F-36 — The public-input requirement is reissued under a new ID

- Class: DUPLICATE
- Severity: MEDIUM
- IDs: WRP-005; LW-003
- Capabilities: wrap
- Wrong: WRP-005 restates the call-vector size already required by LW-003. It adds no acceptance behavior and does not identify an amendment. It also drops constraints from the original, as F-01 records. A second normative ID creates two places to maintain the same layout rule.
- Source: wrap:60; FINAL/specs/sherlock-l-and-wrap/spec.md:26. ROOT/research/sherlock-e2e-spec-20261115/BRIEF.md:46 requires reusing IDs rather than duplicating them.
- Fix: Keep LW-003 as the normative ID. Reference it from C's capability coverage and add the current scenario/evidence there. Retain a new C ID only for a distinct observable integration behavior, or state exactly what it supersedes.

## Numeric and evidence cross-checks

The following figures were checked against the source records or code. They do not prove the final T25 integration.

| Figures in the reviewed material | Check and qualification |
|---|---|
| k <= 21; 1,782,573 row gate; 1,685,845 T10m rows; 96,728 headroom | ROOT/tools/sherlock-wrap/src/bin/wrap.rs:32,45 through :49 and LOOP:31. Subtraction gives 96,728. The 85% expression in D-083 rounds down. |
| VK <= 50,000; historical raw 1,325 and framed 1,353 bytes | LOOP:30 and T17/RESULTS.md:17. No final-key measurement follows from these records. |
| 1,024 transcript words; 1,026 public inputs; lanes 10/20/34/39 | FINAL/specs/sherlock-l-and-wrap/spec.md:26; MAP:25 through :28; ARGS:311 through :318. Lane 6/read and lane 15/write enforcement is F-02. |
| Seven public-input controls and three byte controls | ROOT/tools/sherlock-wrap/ledger-check/src/main.rs:51 through :71. The current checker supports the count; LOOP:29's older T10a record names only three public-input controls. |
| 24/24 box checks and CPU/GPU VK equality | LOG:4108 is the w3b record. This is historical evidence, not a final-key or per-job provenance test. |
| 3,600-second epoch, depth 20, 1,048,576 capacity, 492 jobs/day, <=24 tx/day, about 163 KB versus 3.34 MB | T20/RESULTS.md:33,45 through :56,87. 6,797 collection bytes are an ESTIMATE; the rates are DERIVED. Capacity-limited rates differ above approximately 291 claims/second. |
| 33 programs, 106 circuits, 208 calls, 62 native refusals, one pure call, 145 candidate accepts | LOOP:40,45 and 208 - 62 - 1 = 145. BAT-004 states a conditional plan, not completed settlement. Its <=106 deploy bound depends on the per-circuit deployment pick. |
| Four T20 entries; 42/48 lanes; 43 extra era words and 85/91 lanes | T22/REPORT.md:10,32; the latter are compile-only counts. Neither count establishes the final deployment or authorizes fallback. |
| 13 governance invariants; 11 mutants; Q=12/n=16; TL=7 days | T22/REPORT.md:35 through :53 and OPEN-QUESTIONS.md:53,82. The model coverage has named omissions; exact parameter values are recommendations, not inferred owner rulings. |
| Preview deploy 113,966; honest call 115,187; fee 1,896,654,628,571,429; 757.2 seconds | T17/RESULTS.md:16,19,21. These are T17(a), with its older three-entry contract and keys. |
| Proof server 9.0.1/port 6341; 600-second wait; 1,209,600-second strict TTL bound | T17/RESULTS.md:36,37; RUNNER:193,197. The runner's online default is 14,400 seconds; F-11 distinguishes the runbook override. |
| Five-minute saved-call threshold; four attempts; 30-second retry delay | RUNNER:676,765 through :775. F-31 corrects attempts versus retries. |
| Sixteen builder tests in MAP | MAP:54 is the historical whole-suite count. MADR:85 incorrectly attaches that count to MycroftCall.* alone; it must label the whole recorded suite. test_enc1 has the values described in F-07. |

No finding treats a missing test as a missing implementation by itself. STATUS findings identify evidence that cannot establish the behavior claimed. Proposed requirements are allowed to exceed current code; their status and authority must say so.

## Capability verdicts

Counts include each finding that names the capability. One finding can affect more than one capability. There are 36 distinct findings.

| Capability | Verdict | Findings |
|---|---|---:|
| sherlock-e2e-wrap | needs fixes | 3 |
| sherlock-e2e-ledger-verifier | needs fixes | 2 |
| sherlock-e2e-ledger-contract | needs fixes | 8 |
| sherlock-e2e-batched-settlement | needs fixes | 4 |
| sherlock-e2e-governance | needs fixes | 10 |
| sherlock-e2e-preview-deployment | needs fixes | 3 |
| sherlock-e2e-wallet | needs fixes | 1 |
| sherlock-e2e-sdk-cli | needs fixes | 1 |
| sherlock-e2e-observability | needs fixes | 2 |
| sherlock-e2e-recovery | needs fixes | 4 |
| sherlock-e2e-helper-machines | needs fixes | 2 |

