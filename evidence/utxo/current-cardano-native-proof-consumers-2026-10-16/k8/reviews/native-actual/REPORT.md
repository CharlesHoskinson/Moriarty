# Independent actual review - K2-K8 native consumer (fresh, non-author)

## Verdict

**Top finding: the scoped native Aiken consumer prerequisite CAN be adopted from the existing raw evidence, with full qualification, while the wrapper status remains FAILED and is not relabelled.** The single terminal failure is a second parser-class defect in the operator (and in both prior fresh reviews, R1 5c72 and R2 a563): it expected `succeed_immediately` for the plain-`fail` short-VK test, but the actual Aiken AST parser maps plain `fail` to `SucceedEventually`. No rerun, no expectation rewrite of this run, no new retry AUTH, and no re-spending of the consumed AUTH follows from this review.

## Exact accepted predicate

`scopedNativeAikenConsumerPrerequisiteClosed` is adopted **only** as: the unchanged `pvc2/groth16.verify_digest` (groth16.ak:52-91, sha 1cc3da60...), compiled and run by the pinned data-only Aiken v1.1.24 binary in the consumed one-attempt offline bwrap run, returned Bool **True** for the public 488-byte VK / 192-byte proof / 32-byte statement digest (`c9ef8ea0...`, BE128 halves x1/x2), and Bool **False** for the N1 re-derived body digest, N2 re-derived context digest, and N3 A/C-swapped proof. Source-fixed expectations (test header, k8_genuine_proof_test.ak:6-22) and source bytes did not change after execution.

Outcome accounting (honest, not 10 security theorems): **10/10 raw unit rows pass** in the structured JSON (summary 10/10/0, unit 10/property 0). Of these, the **9 normal rows** (`on_failure: fail_immediately`) carry the scoped evidence: 4 pairing outcomes (1 genuine pass, 3 actual tamper rejections via `!verify_digest` Bool assertions, test.ak:35-45) plus 5 support checks (3 scalar BE128, byte-profile 488/192 layout, distinctness/digest bindings). The **10th raw unit**, `k8_short_vk_is_refused`, is the optional malformed-length case: its unit passes with `succeed_eventually`, it is **not** a cryptographic False, and it is classified separately.

## The parser defect, reproduced from actual definitions

`test_like.rs:27-30`: optional `Token::Once` -> `SucceedImmediately`, absent once -> `SucceedEventually`; snapshot `def_test_fail.snap:51` confirms plain `fail` -> `SucceedEventually`. The test source is `test k8_short_vk_is_refused() fail {` (no `once`, test.ak:111), so the actual `succeed_eventually` (step2 stdout:109) is parser-correct. Operator `operator.py:368` demanded `succeed_immediately`; R1/R2 endorsed that mapping (R2 REPORT.md:7). Original reviews and this counterexample are retained - no fabricated agreement. NC-C9 and NC-C13 are therefore DISAGREEd **as written**; any fix is an ordinary future source/result validator over the captured JSON (`fail` -> succeed_eventually, `fail once` -> succeed_immediately), with no native prefix repeat unless a fresh scope explicitly requires unobserved behaviour.

## Run integrity as observed

One attempt, zero retries, exactly 2 Aiken launch attempts, both exit 0, in bwrap offline; outer exit 1, wall 6.013s; step1 stderr `Summary 0 errors, 0 warnings`, step2 stderr `Summary 10 checks, 0 errors, 0 warnings`. All 14 host pins and staged 5 AK + toml/lock + 38 stdlib + packages.toml + tool postchecks held (RESULT host/staged unchanged true); M5/AUTH unchanged apart from the consumed run. No PK setup, proof generation, RNG, key/wallet, signing, network, ledger, or native-extra step. Compiler identity is manifest-declared pin-hash metadata only (binary body not mirrored, not rebuilt from source, no `--version`).

## Qualifications retained

Execution units (positive mem 22033 / cpu 2142357433; negatives mem 22534 / cpu 2142513482) are pinned Aiken interpreter measurements, **not** qualified network ledger ExUnits or tx fit. Waited-children CPU 0.234848s is not transitive/native CPU. K6 single-party forgeability, fabricated/context-2222/555-type premises, a2-fabricated material and unproved VK-matrix link are retained; context is not authenticated ledger truth. This local-function closure does not qualify the whole 34-file package, any C2 wrapper, or 20 test patterns. **No** actual C2 claim.issue, compiled/applied B9 wrapper/hash, FX1, ledger ExUnits, or Preprod evidence exists yet. All 20 UTxO families, ACTUS/DeFi/general obligations, real recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod remain open, with no compatibility percentage. New K9/C2 source may proceed as separate source preparation only, with no implicit effect permission.

## First blocked boundary

Existing-data ordinary parser repair plus a fresh audit of the repaired validator wording - routine, no user ask and no native execution. After that, the actual next boundary is a bounded C2 consumer authority of its own. If the scoped evidence above were judged insufficient, the correct outcome would be ABSTAIN on that predicate - still with no automatic rerun.

## Independence

Fresh independent muse-wonder-1.0 (HIGH) reviewer in a new context, under the indexed explicit user Gemini/Muse fallback confirmation (USER-GEMINI-MUSE-FALLBACK-CONFIRMATION-20261016T1227.json) after the original Opus author timed out. Same-family independence, not cross-provider. I did not author the candidate, operator, either prior source review, or any executed result, and I executed nothing: read-only mirror review only, no shell/code execution, network, setup, RNG, proof, key, wallet, signing, ledger, or nested agents. No provider identity, authority, or approval is invented.
