# Astra-B phase 2 adversarial review

Verdict: REWORK. This review has 32 findings: 4 BLOCKER, 20 MAJOR and 8 MINOR.

Scope: the 13 B capability specs, the independent draft, the B reuse list, CLAIMS.md, ddd-B.md and decisions-B.md. The reviewed text includes LFP-010–LFP-012, UKS-009, GPH-008 and D-051. These are at S/LFP:135, S/UKS:116, S/GPH:102 and BUS/decisions-B.md:867.

No build, test, prover, network request or git command was run. The checks below are source reads. A source-check result does not certify that a test passes. Historical commit labels remain the source's labels; repository ancestry was not checked. No A or C capability spec was reviewed.

## Citation paths

All relative paths start at the current worktree. The following aliases expand to exact paths. A citation such as `OVM/crates/...:26` means that file and line under its listed root.

| Alias | Path |
|---|---|
| BRIEF | research/sherlock-e2e-spec-20261115/BRIEF.md |
| DRAFT | research/sherlock-e2e-spec-20261115/astra-B/DRAFT.md |
| BUS | research/sherlock-e2e-spec-20261115/bus |
| LOOP | /home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md |
| WIKI | /home/hoskinson/Projects/Moriarty/llm-wiki/wiki |
| COMP | /home/hoskinson/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115 |
| FINAL | openspec/changes/sherlock-processor-final-20261111 |
| OVM | /home/hoskinson/.cache/m7-wrap/src/openvm-rekey-chips |
| T10MM | /home/hoskinson/.cache/m7-wrap/src/openvm-t10mm |
| T10Q | /home/hoskinson/.cache/m7-wrap/src/openvm-t10q |
| FP3 | /home/hoskinson/.cache/m7-wrap/src/openvm-fp3 |
| NTF | /home/hoskinson/.cache/m7-wrap/src/openvm-ntfix |
| SLOT | /home/hoskinson/.cache/m7-wrap/src/openvm-t12-slot15 |
| T16OVM | /home/hoskinson/.cache/m7-wrap/src/openvm-t16 |
| T14OVM | /home/hoskinson/.cache/m7-wrap/src/openvm-t14 |
| T14BOVM | /home/hoskinson/.cache/m7-wrap/src/openvm-t14b |
| T11OVM | /home/hoskinson/.cache/m7-wrap/src/openvm-t11zk |
| T14 | /home/hoskinson/Projects/Moriarty-wt/t14b |
| T11 | /home/hoskinson/Projects/Moriarty-wt/t11-zk |
| ZKR | /home/hoskinson/Projects/Moriarty-wt/t11-zk/research/sherlock-t11-zk-20261115/RESULTS.md |
| SETUP | /home/hoskinson/Projects/Moriarty-wt/t16-elastic/research/sherlock-t16-elastic-20261114/SETUP-SPEC.md |
| T16R | /home/hoskinson/Projects/Moriarty-wt/t16-elastic/research/sherlock-t16-elastic-20261114/RESULTS-CDE.md |
| RUNBOOK | /home/hoskinson/Projects/Moriarty-wt/t12-gate/research/sherlock-tested-design-20261113/t12/RUNBOOK.md |
| LIST | /home/hoskinson/.cache/rekey-t11/LIST.md |

| Spec alias | Spec file |
|---|---|
| S/LFP | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-leaf-proving/spec.md |
| S/LTR | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-l-transcript/spec.md |
| S/UKS | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-universal-keys/spec.md |
| S/NAG | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-node-aggregation/spec.md |
| S/GPH | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-gp-hooks/spec.md |
| S/CLB | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-call-binding/spec.md |
| S/ELA | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-elastic-proving/spec.md |
| S/STM | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-setup-mode/spec.md |
| S/BXF | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-box-fleet/spec.md |
| S/GPV | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-gpu-proving/spec.md |
| S/PDM | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-prover-daemon/spec.md |
| S/ZKE | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-zero-knowledge/spec.md |
| S/SSG | openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-stark-security/spec.md |

## Findings

Counts use one primary spec per finding. Cross-references inside a finding do not create extra findings.

### R-01 — BLOCKER — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-zero-knowledge/spec.md`  
Requirement: **ZKE-004**

Claim: The transfer ban changes case A into a single-host rule. It prevents another owned box from proving the prepared run. It also conflicts with ELA-006 and the trust boundary in CLAIMS.md.

Evidence: S/ZKE:46 bans deploy.secret, run.secret, witnesses and traces between boxes. ZKR:50 explicitly permits these files inside the user's owned-device trust domain. ZKR:55 and WIKI/log.md:4113 admit case A on any box the user owns. BUS/CLAIMS.md:29 says 'no plain witness off the owned boxes', not 'off the source box'. BUS/ddd-B.md:50 repeats the wrong invariant.

Fix: Gate transfers by ownership and trust domain. Permit protected transfers inside that domain; refuse transfers to case B helpers. Keep file permissions, authenticated links and post-call cleanup. Point the transfer check to BXF-007, not BXF-006.

### R-02 — BLOCKER — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-node-aggregation/spec.md`  
Requirement: **NAG-006**

Claim: AFK-F6 and AFK-F9(b) are honest inputs that must be accepted after the fix. The requirement calls them forged and requires refusal. Its named tests check the opposite behavior.

Evidence: S/NAG:74 and :85. tests/quint-regressions/COUNTEREXAMPLES.md:56 says AFK-F6 is accepted after the fix; :61 says the honest one-child MERGE-WORK input is accepted and the unlinked variant is refused. tests/quint-regressions/manifest.json:214 and :235 set expect=accept. T10Q/crates/sherlock-leaf/tests/quint_regressions.rs:98 implements acceptance checks; :180 and :190 select those cases.

Fix: Split acceptance of nonzero reg_root and a linked one-child MERGE-WORK node from refusal of the forged variants. Retain the existing positive tests and the unlinked refusal at quint_regressions.rs:348. Do not change those tests to satisfy the current sentence.

### R-03 — BLOCKER — Decision without authority

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-gp-hooks/spec.md`  
Requirement: **GPH-008; missing unconditional collection-form completion**

Claim: D-049 removes the L collection form from T25 and postpones batching to a second key era. GPH-008 only applies if that form exists. This makes a hard-rule item optional. The recorded lead pick has no authority to waive the hard rule.

Evidence: S/GPH:102 and :113. BUS/decisions-B.md:802 chooses acceptRun with T8c in a later era; :807 leaves LPC-003 open; :809 labels this a Fable-B lead pick; :814 says operator batching is not demonstrated. BRIEF:54 requires batched writes with one root per checkpoint and owner authority for hard-rule changes. LOOP:101 preserves that authority. COMP/specs/sherlock-loop-completion/spec.md:26 requires the L check over SET on real proofs. BUS/CLAIMS.md:4 assigns batching to BAT.

Fix: Make the collection-form proof and its real-proof refusal checks a required B-to-BAT dependency of the complete prototype. Add it to the rekey inputs. Record interim acceptRun work as partial delivery. A complete-prototype waiver needs a cited owner ruling.

### R-04 — BLOCKER — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-stark-security/spec.md`  
Requirement: **SSG-006; UKS-007**

Claim: The gate record and qualification transition omit G5. A key set can reach qualified through these new requirements without a theorem-to-configuration mapping for that set.

Evidence: S/SSG:74 triggers the record on G1 to G4 and FP-8. S/UKS:88 uses SSG-006 as the prototype boundary. BUS/ddd-B.md:68 makes GatePassed change the status to qualified. FINAL/specs/sherlock-security-gate/spec.md:4 requires G1 to G4 with G5; :79 defines the mapping for each frozen set; :86 prohibits a freeze without it. BUS/design-B.md:97 and :105 reuse those IDs.

Fix: Include G5 in the gate predicate and record. Bind its mapping to the actual T25 set. Add a negative scenario: G1–G4 and FP-8 pass, G5 is absent or names old parameters, and the manifest remains prototype.

### R-05 — MAJOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-box-fleet/spec.md`  
Requirement: **BXF-001**

Claim: A hostname allowlist is not the case A ownership rule. It rejects a fourth owned box and accepts one of the named boxes when it proves for a user who does not own it.

Evidence: S/BXF:4. WIKI/log.md:4113 permits any box the user owns. ZKR:51 gives odin proving for a third party as case B. LOOP:96 is the factory roster. BUS/ddd-B.md:50 turns that roster into a proving invariant.

Fix: Separate the factory roster from the public prover's trust-domain policy. Admit a configured owned box; refuse an unowned box regardless of its hostname. Add both cases. Keep muninn, huginn and odin as factory fixtures.

### R-06 — MAJOR — Untestable or weak EARS

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-leaf-proving/spec.md`  
Requirement: **LFP-002**

Claim: The verification compares N1 pre-hashes, while the second scenario requires one leaf key to verify two programs. The cited probe creates a leaf key per app and never checks that scenario.

Evidence: S/LFP:20 and :30. OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:2024 makes each app key; :2030 makes each leaf key; :2034–:2040 compares N1 pre-hashes only. FINAL/specs/sherlock-shard-plan/spec.md:29 distinguishes per-program leaf-layer keys from universal internal keys. BUS/ddd-B.md:9 and :43 use both meanings.

Fix: Define 'universal leaf key' by layer and serialized key identity. Align LFP-002, RSL-001 and DDD with SPL-004. If identical leaf verifying keys are intended, add a direct key-byte comparison and verify both proofs with one supplied key. Keep the N1 probe as a separate check.

### R-07 — MAJOR — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-universal-keys/spec.md`  
Requirement: **UKS-001; UKS-004**

Claim: One digest for every complete key manifest conflicts with the manifest's per-program contents. UKS-004 includes every key file and key_root, while reused SPL-004 permits per-program keys.

Evidence: S/UKS:6 requires one manifest sha256 for all calls; :46 defines its contents. FINAL/specs/sherlock-shard-plan/spec.md:29 commits app, leaf-layer and internal-for-leaf keys per program. BUS/design-B.md:105 reuses SPL-004. BUS/ddd-B.md:42–:43 separates UniversalKeySet and ProgramKeys.

Fix: Define separate universal-suite and per-program manifests, or define an immutable catalog with a precise extension policy. Compare the universal suite digest across programs. Check each program's key_root and membership separately. Do not force a new program to change the universal-suite identity.

### R-08 — MINOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-leaf-proving/spec.md`  
Requirement: **LFP-003**

Claim: The claimed missing mixed-hash refusal test already exists. The LIST citation points to secp256k1 instead of Keccak.

Evidence: S/LFP:39 and :48. OVM/crates/sherlock-leaf/tests/rsl_keccak.rs:150 defines rsl_sha_and_keccak_in_one_program_refused. LIST:34 records the mixed-hash refusal; LIST:35 is the secp256k1 row. OVM/crates/sherlock-leaf/src/rsl/trace.rs:49 holds MAX_APP_AIRS; :48 is its comment.

Fix: Use the existing test name and correct source lines. Report whether its run evidence exists separately from whether the test exists. Do not request a duplicate test.

### R-09 — MAJOR — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-leaf-proving/spec.md`  
Requirement: **LFP-003**

Claim: The requirement makes an implementation AIR limit a permanent refusal of the whole program. It names no route that preserves both admitted hash kits.

Evidence: S/LFP:37 and :44. OVM/crates/sherlock-leaf/src/rsl/trace.rs:114 rejects a combined SHA/Keccak program. COMP/specs/sherlock-rsl-chipset/spec.md:37 and :70 require proof support for the two kits. FINAL/specs/sherlock-general-purpose/spec.md:50 permits no narrowing constant without an owner decision and an unbounded path. BRIEF:54 keeps GP-1–GP-4 intact.

Fix: Specify refusal of an oversized app shape, then require a supported split, chip-set route or other complete proving path for the program. Keep the present combined-hash panic as an open implementation gap. Cite an owner ruling if whole-program exclusion is intended.

### R-10 — MAJOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-node-aggregation/spec.md`  
Requirement: **NAG-005**

Claim: The status presents 5,417 columns as the current minimum for STACKS and VALUE DIGEST. The supplied rekey tree already records the T10q reduction and a lower derived total.

Evidence: S/NAG:71. OVM/crates/sherlock-lfr/src/frozen.rs:166–:168 records 4,988 measured columns without the GP block and fv_state, then 5,121 derived columns with the estimated STACKS and measured value-set additions. :169 keeps the bound at 5,400.

Fix: Report 5,284 and 5,417 as older figures. State 4,988 MEASURED and 5,121 DERIVED with the STACKS estimate. Keep final integrated width open. Add an actual 5,401-column refusal check; the positive d3work run does not test that boundary.

### R-11 — MAJOR — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-call-binding/spec.md`  
Requirement: **CLB-004**

Claim: The unconditional read-and-write sentence requires an anchor write for ordinary calls. CLB-006 and reused ANC-016 forbid that write.

Evidence: S/CLB:46 and :74. SLOT/crates/sherlock-lfr/src/iface.rs:123 defines the read prologue; :129 defines the chunk-only write template; :1555–:1557 says an honest run reads and a chunk writes. COMP/specs/sherlock-anchor-chunks/spec.md:37 refuses ordinary anchor-write templates.

Fix: Require every call to read slot 15 and only chunk-mode calls to write slot 15. Add an ordinary-call positive case with no anchor write. Retain refusal of an ordinary anchor-write template.

### R-12 — MINOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-call-binding/spec.md`  
Requirement: **CLB-002; CLB-003; CLB-005; CLB-006**

Claim: Four requirements put the chunk tests in sherlock-lfr/tests. The available source files are in sherlock-leaf/tests. The FP-3 line numbers also name older contents without a usable current-tree location.

Evidence: S/CLB:29, :43, :62 and :85. FP3/crates/sherlock-leaf/tests/t13_t10q_chunk.rs:759 is nt1_residual_shifted_write_is_fp3; :781 is fp3_k_is_the_true_slice_count; :854 is fp3_bindings_refuse. NTF/crates/sherlock-leaf/tests/t13_t10q_chunk.rs:663 and :670 are NT2 and NT3. The cited sherlock-lfr/tests path is absent in these clones.

Fix: Correct the crate and current-tree lines. Keep historical commit evidence distinct from dirty-tree evidence. Name the NT1 test directly in CLB-006 Verification instead of 'the test above'.

### R-13 — MINOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-l-transcript/spec.md`  
Requirement: **LTR-007; LTR-005; NAG-010; UKS-002**

Claim: Several navigation references name the wrong implementation or requirement. They cannot serve as the stated evidence.

Evidence: S/LTR:99 cites OVM t7_chain.rs:1563 for pathc, but that line defines t10; pathc starts at :1775. S/NAG:141 cites T10MM t7_chain.rs:1563 as the chain-record writer, but it constructs a ProvingContext. S/UKS:29 names stmt.rs:291 UKeys::keys_d, but it is index. S/LTR:71 calls SSG-005 a G3 requirement; S/SSG:60 is a circuit-change-list requirement.

Fix: Replace each reference with the actual function or output writer. Point transcript closure to SQG-014 and the corrected SSG-006. Keep branch-specific line numbers; do not reuse one clone's line numbers for another.

### R-14 — MAJOR — Untestable or weak EARS

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-l-transcript/spec.md`  
Requirement: **LTR-004**

Claim: The requirement promises refusal when Tape::expose is missing. Its verification runs only repaired, positive cases. A machine that accepts every tape can pass those cases.

Evidence: S/LTR:46 and :48. OVM/crates/sherlock-lfr/src/tape.rs:327 defines expose, not a negative acceptance test. LOOP:20 records three runs passing after expose was added. BRIEF:11 requires a check of the requirement.

Fix: Add a named negative test that omits expose from an otherwise valid tape and checks the exact refusal boundary. Pair it with the same tape after expose. Keep the existing three runs as positive integration evidence.

### R-15 — MAJOR — Untestable or weak EARS

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-l-transcript/spec.md`  
Requirement: **LTR-006**

Claim: The scenario compares the hash of the JSON record with the hash of the moved proof file. These are different objects. A schema check also cannot detect a corrupt proof transfer.

Evidence: S/LTR:74 stores the proof's hash in a record; :81 instead compares the record hash with the file hash. BUS/ddd-B.md:65 defines the event as proof path plus sha256. S/BXF:88 compares a file with the digest carried in its bus message.

Fix: Compare SHA256(proof bytes) with the proof_sha256 field. Add corruption and wrong-path cases. Verify that the event's public words and key pre-hash match the verified proof, rather than accepting schema validity alone.

### R-16 — MAJOR — Untestable or weak EARS

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-universal-keys/spec.md`  
Requirement: **UKS-008**

Claim: A stable manifest digest does not prove that key files stayed unchanged. The EARS sentence also drops ST-007's ban on making or selecting keys and changing proof identity in memory.

Evidence: S/UKS:102 and :104. SETUP:15 prohibits making, changing or selecting a key, shape, plan_d or proof value. T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:672 calls keys() during setup. The cited ST-007 is broader than file mutation.

Fix: Retain ST-007's full scope and resolve setup's key construction as an explicit gap or amendment. Rehash each file before and after, detect additions and deletions, and compare proof-identity inputs. Include a mutation that changes a key file while leaving the manifest text untouched.

### R-17 — MINOR — Untestable or weak EARS

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-setup-mode/spec.md`  
Requirement: **STM-003**

Claim: The EARS sentence leaves the process budget undefined. The only named unit test checks JSON round trips, not the probe's minimum-of-two rule. The scenario title states the lower-cgroup case, but its GIVEN clause does not.

Evidence: S/STM:32–:39. SETUP:11 defines min(MemAvailable, cgroup headroom). T16OVM/crates/sherlock-leaf/src/sched.rs:177 implements the minimum. T16OVM/crates/sherlock-leaf/src/profile.rs:650–:655 checks serialization and a wrong format.

Fix: Put the minimum rule in the requirement. Give explicit available-RAM and cgroup values in both orders, plus an unlimited-cgroup case. Verify the probe calculation separately from JSON serialization.

### R-18 — MAJOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-setup-mode/spec.md`  
Requirement: **STM-004**

Claim: BUILT and MEASURED overstates calibration on each GPU. The cited implementation has CPU and GPU device kinds, with one GPU engine; it does not iterate GPU indices.

Evidence: S/STM:46 and :57. T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:689–:706 builds [Cpu, Gpu] and loops those kinds. T16OVM/crates/sherlock-leaf/src/harness.rs:149–:154 has one static GPU engine. SETUP:12 requires each GPU. Its single-GPU box records do not establish multi-GPU calibration.

Fix: Mark per-GPU calibration as open. Add a device index to calibration records and selection. Add a two-GPU check that proves which device executed each sample. Preserve the existing three box measurements as single-GPU evidence.

### R-19 — MAJOR — Missed component

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-setup-mode/spec.md`  
Requirement: **STM-010; STM-011; missing SC-005 and SC-007 obligations**

Claim: The claimed migration loses the least-time selection and tie order, the lower-budget monotonicity rule, and the host-memory cap for discrete GPU concurrency. Saying 'carries' these IDs does not state their responses.

Evidence: S/STM:130–:155 claims SC-002/007 and SC-004/005. BUS/design-B.md:85 says STM is their OpenSpec home; BUS/decisions-B.md:847 makes STM normative. SETUP:34–:37 states all three missing responses. T16OVM/crates/sherlock-leaf/src/profile.rs:293–:298 applies the host cap; :324–:326 orders picks; :623 and :632 name the tie and budget regressions.

Fix: Retain the original IDs as normative imports or add exact mapped requirements. Test discrete GPUs with abundant VRAM but little host RAM, reversed calibration order at a tie, and a sequence of lower budgets. State the fixed-shape concurrency condition separately from a change of shape.

### R-20 — MAJOR — Better in your draft

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-setup-mode/spec.md`  
Requirement: **STM-015; missing PF-008 response**

Claim: STM-015 names memory minus headroom but omits the runtime early-wrap admission rule and the tail calculation. A scheduler that always picks full overlap satisfies its vague response.

Evidence: S/STM:200 and :211 claim PF-007 and PF-008. SETUP:26 defines the three policy outcomes; :27 defines the residual tail and runtime admission. T16OVM/crates/sherlock-leaf/src/profile.rs:446 and :457 implement these separate operations. DRAFT:680 says: 'When early wrap execution is requested, the device guard shall reserve STARK extra bytes, the profile-derived wrap tail and the margin.'

Fix: Adopt the draft's PF-008 response and retain the full PF-007 decision table. Specify tail = wrap peak minus resident cosets without eviction, or the whole peak with eviction. Require refusal when current free memory is below STARK extra + tail + margin. Bind both tests to those responses.

### R-21 — MAJOR — Better in your draft

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-elastic-proving/spec.md`  
Requirement: **ELA-001; ELA-005**

Claim: Program and input alone do not define a repeatable claim. New runs draw new salt. The requirement must hold deployment and prepared-run identity fixed across schedules.

Evidence: S/ELA:4 and :60. ZKR:17–:21 defines deployment, run and proof randomness separately. T11OVM/crates/sherlock-leaf/src/blind.rs:180–:183 draws a fresh salt on prepare; :187 loads an existing run. DRAFT:527 says: 'GIVEN the same deployment and run salt, WHEN partition, concurrency or host changes, THEN claim_pub remains equal.' DRAFT:602 requires different salts and claims for two new runs.

Fix: Use the same prepared run, deployment, input and salt in equality checks. Add a separate fresh-run check. Carry an opaque run identity through the cross-box interfaces without publishing the salt.

### R-22 — MAJOR — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-elastic-proving/spec.md`  
Requirement: **ELA-003; ELA-004**

Claim: The requirements demand a proof after any budget reduction or loss of the GPU, even when no CPU leaf fits. STM-012 requires refusal in that state.

Evidence: S/ELA:32 and :46. S/STM:158 requires no-fit refusal. SETUP:36 states that rule. T16OVM/crates/sherlock-leaf/src/profile.rs:307–:309 rejects zero concurrency.

Fix: Condition successful scale-down and CPU fallback on at least one valid fitting shape. Otherwise require STM-012 refusal. Add zero-budget and CPU-too-small cases beside the successful scale-down cases.

### R-23 — MINOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-elastic-proving/spec.md`  
Requirement: **ELA-005**

Claim: The scenario requires different boxes to choose different schedules. Hardware differences do not force different schedules.

Evidence: S/ELA:67. T16R:60–:61 records the same shape 1, cut [4,1], GPU and concurrency 2 on huginn and odin. SETUP:31 and :35 choose from profile costs and limits, not from a rule that box schedules must differ.

Fix: Require a valid schedule for each box and equal claims for the same prepared run. Use controlled profile inputs when a test needs different picks. Accept identical picks on different hardware.

### R-24 — MAJOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-gpu-proving/spec.md`  
Requirement: **GPV-001**

Claim: The status says app and leaf-layer GPU proving are built and measured, but the cited function proves the app only. A stock verifier accepts a CPU proof too, so the scenario cannot detect CPU fallback.

Evidence: S/GPV:4, :11 and :15. T16OVM/crates/sherlock-leaf/src/harness.rs:145 calls the function an app proof; :153 selects frozen::APP. T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:706 selects GPU for the app; :711 runs the leaf layer separately. T16R:52 explicitly states that the leaf layer still proves on CPU.

Fix: Split the app and leaf-layer paths and their status. Require an instrumented backend/device record or a device-call assertion in each GPU test. Add the missing GPU leaf-layer implementation/check as an open task. Preserve stock verification as the correctness check.

### R-25 — MINOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-gpu-proving/spec.md`  
Requirement: **GPV-004; GPV-005**

Claim: The trigger 'is set' includes the value off. The implementation uses off to request CPU proving and enables GPU paths by default in the relevant build.

Evidence: S/GPV:46 and :60. T14OVM/crates/sherlock-wrap-export/Cargo.toml:52–:57 documents both off switches. T14OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:109 also treats T7_NODE_GPU=0 as disabled and absence as enabled; :893 handles T14_L_GPU=off.

Fix: State the supported enable/disable values and defaults. Add off and unset cases. Do not use environment-variable presence as the GPU selection predicate.

### R-26 — MAJOR — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-prover-daemon/spec.md`  
Requirement: **PDM-005**

Claim: The reuse list keeps mandatory PRD-002, while D-040 chooses no pool. PDM-005 also makes slot_reuse conditional on building a pool, although the existing test checks cross-request state in a Resident and needs no pool.

Evidence: S/PDM:62 and :71. BUS/design-B.md:77 imports PRD-002. FINAL/specs/sherlock-prover-daemon/spec.md:11 mandates a pool. BUS/decisions-B.md:441 chooses per-proof draws, but :448 leaves the requirement conflict until T15. T11/tools/sherlock-wrap-gpu/tests/t11_zk.rs:62–:87 compares X after Y with fresh X on Resident objects. T14/tools/sherlock-wrap-gpu/src/serve.rs:117 draws OsRng at prove time.

Fix: Explicitly amend or supersede the pool implementation obligation under the recorded lead pick. Keep cross-request isolation unconditional for both daemon and STARK worker. Run the existing Resident reuse check, and add a socket-level check if the requirement claims the served path.

### R-27 — MAJOR — Better in your draft

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-gp-hooks/spec.md`  
Requirement: **GPH-007**

Claim: A case-count floor does not enforce the no-narrowing rule. Replacing each removed negative or DAG case with a duplicate positive case preserves both counts.

Evidence: S/GPH:88 and :90. FINAL/specs/sherlock-general-purpose/spec.md:50 prohibits semantic narrowing. DRAFT:237 says: 'If a proof gate fails, then the project shall retain the full GP-1 through GP-4 acceptance requirements.' DRAFT:240 asks for a coverage review that detects deleted outcomes, foreign-proof paths, DAG cases and length cases.

Fix: Keep the count floor as a smoke check. Add a named requirement-to-case map with expected outcomes and coverage of GP-1–GP-4. Fail when a required case, refusal, foreign adapter path or unbounded route is removed or weakened.

### R-28 — MINOR — Missed component

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-box-fleet/spec.md`  
Requirement: **BXF-005**

Claim: The orphan rule stops at SIGTERM. It drops the required SIGKILL fallback for a sampler that keeps the lock.

Evidence: S/BXF:60 and :67. WIKI/log.md:4117 states log first, SIGTERM then SIGKILL, under the narrow ended-job and missing-parent conditions. S/BXF:74 preserves a live prover or wrapd.

Fix: Add bounded reinspection after SIGTERM and SIGKILL only if the same eligible orphan still holds the lock. Recheck identity and eligibility before escalation. Keep the live-prover exclusion.

### R-29 — MINOR — Missed component

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-box-fleet/spec.md`  
Requirement: **BXF-007; missing factory transfer route**

Claim: The digest requirement does not enforce the factory rule that files move through muninn. A direct odin-to-huginn transfer with a matching hash satisfies BXF-007.

Evidence: S/BXF:88 and :99. LOOP:96 explicitly says files move only through muninn with path + sha256. BUS/CLAIMS.md:26 assigns file moves to BXF.

Fix: Add a factory-only routing requirement and a direct-transfer refusal case. Keep that maintainer topology separate from the public owned-device trust-domain rule in R-05.

### R-30 — MAJOR — Duplicate or conflict

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-call-binding/spec.md`  
Requirement: **CLB-002 vs ANC-018; CLB-003 vs ANC-019; CLB-005 vs ANC-017**

Claim: The spec creates new IDs for obligations that its reuse list says it does not restate. This gives the same binding two normative homes without a supersession rule.

Evidence: S/CLB:18, :32 and :60. COMP/specs/sherlock-anchor-chunks/spec.md:59 binds guest effect words, :70 binds the true slice count, and :48 binds the lock operands. BUS/design-B.md:19 imports ANC-013–ANC-019 without restatement. BRIEF:46 requires reuse or explicit supersession.

Fix: Keep ANC-017/018/019 as the normative requirements and attach the added negative scenarios and evidence to those IDs. If the new IDs are needed, state exactly which old text they supersede and why. Do not duplicate the obligation while claiming reference-only reuse.

### R-31 — MAJOR — Untestable or weak EARS

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-zero-knowledge/spec.md`  
Requirement: **ZKE-007**

Claim: The named eq_zk test makes two GPU Resident proofs. It does not implement the CPU-versus-GPU comparison in this requirement. Different bytes across devices alone also permit one fixed blinder stream per device.

Evidence: S/ZKE:88 and :90. T11/tools/sherlock-wrap-gpu/tests/t11_zk.rs:37 installs the GPU path; :42–:44 calls the same Resident twice. Its :53–:56 checks distinct bytes and aligned blocks. S/ZKE:99 cites a historical CPU/GPU result but gives no reusable CPU leg for the proposed verification.

Fix: Name separate CPU and GPU entry points. Require repeated proofs on each backend and compare one identical public statement. Keep existing EQ-ZK and ledger verification on each output. Add a test that rejects fixed per-device randomness.

### R-32 — MAJOR — Wrong claim against code or ruling

Spec: `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-leaf-proving/spec.md`  
Requirement: **LFP-005**

Claim: HEIGHTS is an ordered AIR-height vector for an interpreted slice shard, not a menu of whole-shard heights. Requiring a height from that list does not preserve the fixed shape and does not cover R_SL.

Evidence: S/LFP:65 and :76. OVM/crates/sherlock-leaf/src/leaf.rs:33–:35 defines seven heights in AIR order; :631–:635 defines a different vector for chip set 2. OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:2038–:2039 records separate R_SL app and leaf shapes. LOOP:102 requires fixed-size shards.

Fix: Require the full ordered trace-height vector and AIR identity for each admitted shape and chip set. Name the R_SL shape source separately. Test a swapped AIR height, an oversized AIR and a valid final padded shard. Keep shape equality under increased run length.

## Hard-rule coverage

Each rule below comes from BRIEF:54. This table checks B's part of the rule. A C-owned item is linked through CLAIMS.md; this review makes no claim that its C spec is correct.

| Rule | B requirement or reused ID | Review result and source |
|---|---|---|
| Never k=22; all circuits k<=21 | SSG-002; UKS-006; CPR-001 on the wrap boundary | B is STARK-only; the circuit gate is WRP (BUS/CLAIMS.md:1). S/SSG:18 checks dependencies, not k. The hard ceiling remains in FINAL/specs/sherlock-general-purpose/spec.md:22 under CPR-001, referenced at S/SSG:29. |
| Zero knowledge end to end | ZK-001–ZK-019; ZKE-001–ZKE-007; PDM-005 | Covered by IDs, with an incorrect trust boundary and weak device verification: R-01, R-26, R-31. Reuse list BUS/design-B.md:113. |
| Seeded proofs test-only | ZK-014; ZKE-001; ZKE-002 | Explicit at S/ZKE:4 and :18. T11OVM/crates/sherlock-leaf/src/blind.rs:70 guards the seed feature. Production integration remains OPEN at S/ZKE:15. |
| No plain witness to helpers | ZK-018; ZKE-004; BXF-001 | Stated with the wrong meaning: R-01 and R-05. Owned machines are case A, not helpers: ZKR:50–:55. |
| Proven 100-bit security | SQG-001–SQG-016; RKY-005; SSG-001; SSG-006 | G5 conflict blocks qualification: R-04. The 101-bit margin still exists in reused SQG-004 at FINAL/specs/sherlock-security-gate/spec.md:25. |
| One universal key set | CPR-003; SPL-004/005/009; RSL-001; UKS; ELA-002 | Coverage exists, but the leaf-key and manifest meanings conflict: R-06 and R-07. Reuse list BUS/design-B.md:105. |
| Unchanged ledger 9.1.0.1 verifier | UKS-006 handoff; ZKE-007 ledger check | Ledger ownership is LVR and WRP (BUS/CLAIMS.md:1–:2); B sends LfrProofReady (S/LTR:74). R-15 affects that handoff. |
| Wrap VK<=50,000 bytes | T25 boundary; RKY-004 belongs to C | BUS/CLAIMS.md:20 explicitly excludes wrap row/VK checks from B. No duplicate B requirement is needed. The actual wrap key must be the one named by UKS-004, not an older key. |
| One BLS12-381 wrap; small-field leaves and nodes | LFP-001; NAG-001; LTR-001/002; SSG-002 | Field types are supported by OVM/crates/sherlock-leaf/src/harness.rs:26 and OVM/crates/sherlock-lfr/src/fixture.rs:26–:28. L_Fr is a separate Fr STARK, not another wrap circuit (S/LTR:18). |
| GP-1–GP-4 never narrowed | GP-001–GP-004; CPR-005; LPC-001–LPC-003; GPH | R-09 and R-27 identify narrowing and a weak guard. R-03 leaves required composition/collection work outside the prototype. BUS/design-B.md:35 imports the complete GP obligations. |
| Batched writes; one root per checkpoint | GPH-008; LPC-003; LTR-006 handoff to BAT | R-03 blocks acceptance. A conditional collection-form requirement cannot bind the hard rule. BUS/CLAIMS.md:4 assigns checkpoint batching to C. |
| Case A now; case B deferred to Veritee | ZK-018; BXF-001; ZKE-004 | R-01 and R-05 require correction. WIKI/log.md:4112–:4113 gives the owner ruling and scope. |
| K_MAX=2 | CLB-007; ANC-012 | Explicit at S/CLB:88. The status at :99 records the code gap. LOOP:106 confirms the bound. |
| MERGE-WORK=5,400 | NAG-005; LPC-002; SQG-013 | Bound is explicit and present in T10MM/crates/sherlock-lfr/src/frozen.rs:166. R-10 corrects the width evidence; R-02 corrects behavior within the bound. |
| Canonical x_j lock | CLB-005; ANC-017 | Formula is explicit at S/CLB:60 and LIST:5. Correct source paths and duplicate IDs: R-12 and R-30. |
| F-3 replaced by setup mode | STM; SPL-011; ST/PF/SC mapping | Coverage is incomplete: R-16–R-20. STM-014 correctly records the surviving menu-code gap at S/STM:197. |
| Parameters, keys and key handling open until full prototype; hard-rule changes owner-gated | UKS-007; SPL-009; SQG-011; decision records | LOOP:101 defines the distinction. R-03 rejects a lead-only hard-rule waiver. R-04 prevents an incomplete security gate from qualifying keys. |

## Requested LOOP-row coverage

All 15 requested rows have named coverage through direct requirements or the reuse lists. A coverage entry is not a completion claim. The remaining omissions are responses within capabilities, not unnamed top-level capabilities.

| LOOP row and source | B coverage | Review result |
|---|---|---|
| T3, LOOP:20 | NAG-002, LTR-004, LPC-018 | Parent-constraint gap is retained; expose refusal lacks a negative check (R-14). |
| T6, LOOP:23 | LFP-001/005/006; NAG-008; FLG-001–009; LW | Fixed-height meaning is wrong (R-32). TAG_NODE is covered by reused FLG-002, FINAL/specs/sherlock-field-log/spec.md:11; it is not an uncovered component. |
| T7, LOOP:24 | LFP-006/007; NAG-003/004/008; GPH-003 | Named program, outcome and key checks exist. R_SL binding gaps now have LFP-010/011/012, S/LFP:135–:174. |
| T7c, LOOP:25 | LTR-007; UKS-003; PDM-007; PRD-004 | Path C coverage exists; source navigation is wrong (R-13). |
| T8, LOOP:26 | GPH-001–007; GP/CLM/CONT/FREG/FVR reuse | No-narrowing verification is weak (R-27). |
| T8b, LOOP:27 | LPC-001–003; NAG-005; GPH-002/004/005/008 | The collection-form deferral is a blocker (R-03); width evidence is stale (R-10). |
| T10q, LOOP:32 | NAG-006/007; CLB-002/003/005/006; UKS-009 | Honest-input fixes are reversed (R-02); chunk test paths are wrong (R-12). |
| T11, LOOP:33 | ZK reuse; ZKE; PDM-005 | Later lifetimes govern the stale per-proof r_P row (ZKR:21). Trust and state-reuse checks need R-01/R-21/R-26/R-31. |
| T12, LOOP:34 | SQG reuse; SSG; CLB-004/008; ANC-013 | G5 is missing from the new qualification predicate (R-04); ordinary/chunk anchor modes conflict (R-11). |
| T13, LOOP:35 | SSG-003/004/005; LPC-004/016 | Named coverage exists. NAG-006 must retain honest-input acceptance as a valid counterexample outcome (R-02). |
| T14, LOOP:36 | GPV; PDM; LPC-005 | GPU path claims and selection triggers need R-24/R-25. |
| T14b, LOOP:38 | PDM; PRD; GPW-007 | Pool requirements need an explicit amendment; reuse isolation stays required (R-26). |
| T16, LOOP:37 | ELA; STM; BXF; LPC-006/007 | R-05, R-16–R-25 and R-28/R-29 cover the lost or wrong responses. Two-run-length and scaling work stays open under reused LPC-006/007. |
| T24 leaf side, LOOP:46 | RSL-004/005/006/007; LFP-002/003 | Kit IDs are reused in BUS/design-B.md:61. Whole-program mixed-kit refusal needs a complete route (R-09). |
| T25, LOOP:47 | RKY-001/002/005; UKS-001–009; SSG | Integrated-tree guard is now explicit (S/UKS:116). Universal identity, setup isolation and qualification need R-07/R-16/R-04. |

## Citation audit

The following 52 checks cover four source locations per spec. Each location was opened. “Supports” means the source supports the limited statement shown; it does not mean the requirement or its current execution status passes.

| Check | Spec / requirement | Cited location checked | Result |
|---|---|---|---|
| C-01 | LFP-001 | OVM/crates/sherlock-leaf/src/harness.rs:26 | Supports BabyBearPoseidon2Config. |
| C-02 | LFP-003 | OVM/crates/sherlock-leaf/src/rsl/trace.rs:48 | Comment at 48; constant at 49; refusal at 114/137. R-08/R-09. |
| C-03 | LFP-003 | OVM/crates/sherlock-leaf/tests/rsl.rs:191 | Test exists; checks 6 through 9 AIRs and more than 100 built cases. |
| C-04 | LFP-002 | OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:2040 | Checks N1 pre-hash equality only. R-06. |
| C-05 | LTR-001 | OVM/crates/sherlock-lfr/src/fixture.rs:26 | Supports LSC field/hash configuration. |
| C-06 | LTR-002 | OVM/crates/sherlock-lfr/tests/i10_gate.rs:83 | Named test exists; ignored heavy test requiring a fixture. |
| C-07 | LTR-004 | OVM/crates/sherlock-lfr/src/tape.rs:327 | Defines expose; does not supply the missing-expose negative test. R-14. |
| C-08 | LTR-007 | OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:1563 | Wrong function: t10. pathc starts at 1775. R-13. |
| C-09 | UKS-002 | T10MM/crates/sherlock-lfr/src/stmt.rs:88 | Comment describes seven rows; UROWS=7 at 90. |
| C-10 | UKS-002 | T10MM/crates/sherlock-lfr/src/stmt.rs:291 | Function is index, not keys_d. R-13. |
| C-11 | UKS-003 | OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:363 | KEYDAG_IDX=3; leaf count assertion at 450 supports the collision check. |
| C-12 | UKS-008 | SETUP:15 | Broader than file mutation: keys, shapes, plan_d and proof values. R-16. |
| C-13 | NAG-001 | OVM/crates/sherlock-lfr/src/fixture.rs:28 | Supports BabyBear Poseidon2 node configuration. |
| C-14 | NAG-005 | T10MM/crates/sherlock-lfr/src/frozen.rs:166 | Bound is 5,400; check at 434. Current rekey-tree width update is at OVM frozen.rs:166–:168. R-10. |
| C-15 | NAG-006 | T10Q/crates/sherlock-leaf/tests/quint_regressions.rs:180 | Named test exists and selects an expected-accept manifest row. R-02. |
| C-16 | NAG-007 | T10Q/crates/sherlock-leaf/tests/quint_regressions.rs:201 | Named test exists; ignore at 200 explicitly names the open defect. |
| C-17 | GPH-002 | T10MM/crates/sherlock-lfr/src/gp.rs:1–:26 | Mode descriptions exist; they include SEAL, dependency and JOIN semantics. |
| C-18 | GPH-002 | T10MM/crates/sherlock-wrap-export/examples/t8_gp.rs:564 | gp2 function exists. |
| C-19 | GPH-005 | T10MM/crates/sherlock-wrap-export/examples/t8_gp.rs:863 | x7 function exists. |
| C-20 | GPH-006 | T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:235 | pos_plan_schedules comment and cases exist; old sched::check at 246 remains. |
| C-21 | CLB-001 | T10MM/crates/sherlock-lfr/src/iface.rs:42 | LANE_PAIRS=5. |
| C-22 | CLB-001 | T10MM/crates/sherlock-lfr/src/iface.rs:1455 | layout_full_is_12166 exists and asserts 12,166. |
| C-23 | CLB-004 | SLOT/crates/sherlock-lfr/src/iface.rs:112 | ANCHOR_SLOT=15; ordinary read and chunk write are distinct. R-11. |
| C-24 | CLB-005 | NTF/crates/sherlock-leaf/tests/t13_t10q_chunk.rs:663 | NT2 test exists here; spec cites sherlock-lfr/tests. R-12. |
| C-25 | ELA-002 | SETUP:43 | Same app, leaf, N1/N2/N3 and L keys across schedules are required. |
| C-26 | ELA-006 | T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:762 | Join entry exists; manifest and leaf checks precede nodes. |
| C-27 | ELA-006 | T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:797 | Named stock-L-verifier case exists; leaf verification is at 781. |
| C-28 | ELA-006 | T16R:66 | Records five cross-host cases on odin with huginn; muninn remains not run in that record. |
| C-29 | STM-003 | T16OVM/crates/sherlock-leaf/src/sched.rs:172 | Probe takes min(cgroup headroom, MemAvailable) at 177. R-17. |
| C-30 | STM-006 | T16OVM/crates/sherlock-leaf/src/profile.rs:79 | Writes the named profile fields. |
| C-31 | STM-004 | T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:690 | Adds one GPU device kind, not a loop per GPU index. R-18. |
| C-32 | STM-015 | T16OVM/crates/sherlock-leaf/src/profile.rs:446 | wrap_tail subtracts resident cosets without eviction. Runtime guard follows at 457. R-20. |
| C-33 | BXF-002 | /home/hoskinson/.local/bin/heavy-slot:27 | Default SLOTS=3, with environment override. |
| C-34 | BXF-003 | /home/hoskinson/.local/bin/heavy-slot:115 | Over-budget shared requests fail; exclusive requests are separate. |
| C-35 | BXF-008 | /home/hoskinson/.local/bin/box-relay:22 | Forwards to the named box or to:all; timeline path at 7. |
| C-36 | BXF-002 | /home/hoskinson/.local/bin/heavy-slot:28 | Default BUDGET=64, with environment override; supports the script-side correction. |
| C-37 | GPV-001 | T16OVM/crates/sherlock-leaf/src/harness.rs:148 | CUDA function is the APP prover and has one static GPU engine. R-24. |
| C-38 | GPV-003 | T14OVM/crates/sherlock-lfr/tests/e4_gpu_hash.rs:49 | e4_gpu_poseidon_equals_cpu exists and requires the GPU path. |
| C-39 | GPV-003 | T14OVM/crates/sherlock-lfr/tests/e4_gpu_frac.rs:36 | Fractional-sumcheck comparison test exists; compares output items at 51–54. |
| C-40 | GPV-004 | T14OVM/crates/sherlock-wrap-export/Cargo.toml:51–:58 | CUDA feature and explicit off switches exist. R-25. |
| C-41 | PDM-002 | T14/tools/sherlock-wrap-gpu/src/serve.rs:210 | Sets socket mode 0600. |
| C-42 | PDM-003 | T14/tools/sherlock-wrap-gpu/src/serve.rs:137–:201 | Four operation arms plus unknown-op error. |
| C-43 | PDM-004 | T14BOVM/crates/sherlock-wrap-export/examples/common/t14.rs:30–:48 | Documents resident STARK worker and opens its socket. |
| C-44 | PDM-005 | T14/tools/sherlock-wrap-gpu/src/serve.rs:114–:117 | prove calls OsRng; does not establish a slot pool. R-26. |
| C-45 | ZKE-001 | T11OVM/crates/sherlock-leaf/src/blind.rs:26 | REFUSED_EXIT=4; feature guard at 70. |
| C-46 | ZKE-004 | T11OVM/crates/sherlock-leaf/src/blind.rs:180–:187 | Prepares/loads secrets; supplies no cross-box transfer prohibition. R-01. |
| C-47 | ZKE-005 | T11OVM/crates/sherlock-leaf/src/blind.rs:187 | Loads both secret files and propagates load errors. |
| C-48 | ZKE-007 | T11/tools/sherlock-wrap-gpu/tests/t11_zk.rs:36 | EQ-ZK uses one GPU Resident twice, not CPU versus GPU. R-31. |
| C-49 | SSG-001 | T10MM/crates/sherlock-wrap-export/examples/fp8_d3.rs:9 | Named calculator entry exists; header states 100/101-bit rules at 7–8. |
| C-50 | SSG-001 | T10MM/crates/sherlock-wrap-export/examples/fp8_d3.rs:84–:87 | Implements conservative minimum two stacked columns. |
| C-51 | SSG-006 | RUNBOOK:61–:63 | CPU prover, GPU prover and verifier transcript legs are marked not built. |
| C-52 | SSG-003 | tests/quint-regressions/COUNTEREXAMPLES.md:75 | GP2-C2.1 is CONFIRMED and conditional on an unreachable pre-state under chip sets 0–2. |

Additional source reads support the findings: ZKR:50–:55, WIKI/log.md:4107/:4113/:4117, SETUP:26–:37, FINAL security gate :4/:79/:86, the full B reuse list and the B decision outcomes. These reads are not counted in the 52-check minimum.

## Verdict by spec

Counts below assign each finding to its primary spec only.

| Spec | BLOCKER | MAJOR | MINOR | Findings | Verdict |
|---|---:|---:|---:|---|---|
| sherlock-e2e-leaf-proving (LFP) | 0 | 3 | 1 | R-06, R-08, R-09, R-32 | REWORK |
| sherlock-e2e-l-transcript (LTR) | 0 | 2 | 1 | R-13, R-14, R-15 | REWORK |
| sherlock-e2e-universal-keys (UKS) | 0 | 2 | 0 | R-07, R-16 | REWORK |
| sherlock-e2e-node-aggregation (NAG) | 1 | 1 | 0 | R-02, R-10 | REWORK |
| sherlock-e2e-gp-hooks (GPH) | 1 | 1 | 0 | R-03, R-27 | REWORK |
| sherlock-e2e-call-binding (CLB) | 0 | 2 | 1 | R-11, R-12, R-30 | REWORK |
| sherlock-e2e-elastic-proving (ELA) | 0 | 2 | 1 | R-21, R-22, R-23 | REWORK |
| sherlock-e2e-setup-mode (STM) | 0 | 3 | 1 | R-17, R-18, R-19, R-20 | REWORK |
| sherlock-e2e-box-fleet (BXF) | 0 | 1 | 2 | R-05, R-28, R-29 | REWORK |
| sherlock-e2e-gpu-proving (GPV) | 0 | 1 | 1 | R-24, R-25 | REWORK |
| sherlock-e2e-prover-daemon (PDM) | 0 | 1 | 0 | R-26 | REWORK |
| sherlock-e2e-zero-knowledge (ZKE) | 1 | 1 | 0 | R-01, R-31 | REWORK |
| sherlock-e2e-stark-security (SSG) | 1 | 0 | 0 | R-04 | REWORK |
| Total | 4 | 20 | 8 | 32 findings | REWORK |

Verdict: REWORK. Resolve the four blockers before this slice can define a complete working prototype.

