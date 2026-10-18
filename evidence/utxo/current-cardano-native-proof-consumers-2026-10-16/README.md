# Native proof consumers: K8 and K9 local results

This capsule preserves two reviewed local Aiken consumer results using the same genuine current-Rust invoice-partial Groth16 proof. **K8** observed the unchanged `pvc2/groth16.verify_digest` accepting the genuine proof and refusing three pairing tamper cases. **K9** observed the unchanged `pvc2/claim.issue` local typed function accepting one coherent issuance fixture and refusing six negative fixtures. These are adopted local-function predicates, not ledger transactions, universal correctness certificates or seventeen different cryptographic theorems. This document transcribes the existing qualified adoptions; it supplies no new security judgment or audit approval.

The underlying [proof capsule](../current-rust-groth16-first-proof-2026-10-16/README.md) was published at `8b4fab`; the [public VK-record capsule](../current-rust-groth16-public-verifier-record-2026-10-16/PUBLICATION.json) at `8360447` supplies the [public verification record](../current-rust-groth16-public-verifier-record-2026-10-16/receipts/invoice-partial.setup.json) and [export-source documentation](../current-rust-groth16-public-verifier-record-2026-10-16/source/README.md). The reused fixture has a 488-byte VK, 192-byte proof and statement digest `c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc`. K8 and K9 contain identical [fixture source bytes](k8/actual/staged/lib/pvc2/k8_fixture.ak). Neither run generated a new proof, setup, proving key or RNG sample. This capsule makes no new latest-graph claim.

**K8: qualified acceptance, preserved controller failure.** The [fresh actual review](k8/reviews/native-actual/REPORT.md) (`85e4248f…`) and [root adoption](k8/root/K8-ACTUAL-ADOPTION.json) (`99541a43…`) adopt only the direct verifier predicate. [Raw telemetry](k8/actual/run/logs/step2-k8-tests.stdout.log) reports ten passing unit rows: nine normal `fail_immediately` rows and one plain-`fail` malformed-length row with `succeed_eventually`.

| K8 rows | Observed meaning |
|---|---|
| Genuine proof | `verify_digest` returned True |
| N1 re-derived body digest | Returned False through the pairing equation |
| N2 re-derived context digest | Returned False through the pairing equation |
| N3 proof A/C swap | Returned False through the pairing equation |
| Five normal support checks | Three digest/scalar checks, byte layout, and distinctness/digest bindings |
| Short VK, plain `fail` annotation | Optional malformed-length test passed in its failure mode; not another cryptographic False |

The source-fixed [ten tests](k8/actual/staged/lib/pvc2/k8_genuine_proof_test.ak) were not changed after execution. Both Aiken calls exited 0, but the [original runtime result](k8/actual/run/receipts/RESULT.json) remains **FAILED**, and the [outer capture](k8/actual/capture/RESULT.json) remains exit 1. Its sole terminal error was an incorrect metadata expectation: the operator wanted `succeed_immediately` for the plain-`fail` short-VK test, while actual telemetry correctly reported `succeed_eventually`. The [mismatch reproduction](k8/actual/METADATA-MISMATCH-REPRODUCTION.json) records that distinction. Earlier [R1](history/k8-source-r1/REPORT.md) and [R2](history/k8-source-r2/REPORT.md) source reviews endorsed the mistaken mapping and remain unchanged.

**The later K8 correction closes metadata only.** The pinned-version [parser source](aiken-v1.1.24-source/aiken-lang/src/parser/definition/test_like.rs), [plain-fail snapshot](aiken-v1.1.24-source/aiken-lang/src/parser/definition/snapshots/def_test_fail.snap), and [JSON telemetry source](aiken-v1.1.24-source/aiken-project/src/telemetry/json.rs) distinguish normal → `fail_immediately`, plain `fail` → `succeed_eventually`, and `fail once` → `succeed_immediately`. The [ordinary captured-data validator](k8/telemetry-correction/source/validate_capture.py) checked the three existing pinned public files and produced [VALIDATION.json](k8/telemetry-correction/actual/VALIDATION.json), SHA-256 `1db9d384d23da9bca96e8c2f14e7e8ae87a66587313ae27293725f2ed285696a`. The [ordinary result](k8/telemetry-correction/root/RESULT.json) records exit 0, one attempt and zero retries. Its [source review](k8/telemetry-correction/reviews/source/REPORT.md) and [fresh actual review](k8/telemetry-correction/reviews/actual/REPORT.md) (`677e7ee7…`) remain separately scoped.

The [companion closure](k8/telemetry-correction/root/ORDINARY-TELEMETRY-CORRECTION-CLOSURE.json), SHA-256 `ca14d5877d57b810b4bec49824d0566bd512304fa622189d60237b8e00283929`, closes `ordinaryTelemetryCorrectionClosed`. The older `99541a43…` adoption's pending flag remains an immutable captured-time fact. This later closure does not relabel the native FAILED result, repeat a native prefix, refresh the cryptographic predicate, change counters or authorize a retry.

**K9: seven normal local claim-consumer cases.** The [fresh actual review](k9/reviews/native-actual/REPORT.md) (`36111bc4…`) and [root adoption](k9/root/K9-ACTUAL-ADOPTION.json) (`76ccfd08…`) adopt `nativeC2ConsumerPrerequisiteClosed` for the local typed function only. [Telemetry](k9/actual/run/logs/step2-k9-tests.stdout.log) reports seven unit passes, all normal `fail_immediately` rows: one positive `claim.issue(...)` and six explicit `!claim.issue(...)` negative assertions in the [test source](k9/actual/staged/lib/pvc2/k9_genuine_c2_consumer_test.ak).

| K9 fixture | Adopted local result |
|---|---|
| Genuine proof with coherent issuance fixture | True |
| N1 full envelope rebuilt with re-derived body digest | False through pairing |
| N2 full envelope rebuilt with re-derived context digest | False through pairing |
| N3 A/C-swapped proof | False through pairing |
| Minted quantity 2 | False at quantity guard |
| Mismatched envelope token name | False at name guard |
| Wrong VK digest | False at VK-digest guard |

The [builder](k9/actual/staged/lib/pvc2/k9_consumer_builder.ak) rebuilds the envelope, token name, mint and output coherently for N1/N2; the actual audit attributes their refusals to the pairing path. The [runtime result](k9/actual/run/receipts/RESULT.json) records both Aiken calls exiting 0; the [outer capture](k9/actual/capture/RESULT.json) is also exit 0. Its `COMPLETED_AWAITING_FRESH_ACTUAL_AUDIT` status is preserved as a pre-review snapshot; the later root adoption supplies the qualified actual acceptance. The [earlier K9 source review](history/k9-source/REPORT.md) is source readiness, not runtime evidence. The root adoption also corrects compressed review wording: only the six negative cases use `!claim.issue`, and all 20 UTxO families remain distinct from ACTUS coverage.

**Measured costs stay local.** Host wall observations were approximately 6.013 seconds for K8 and 6.933 seconds for K9. Positive-test Aiken VM measurements were memory 22,033 / CPU 2,142,357,433 for K8 and memory 200,075 / CPU 2,238,951,754 for K9; all other per-row values are preserved in telemetry. These interpreter units are not qualified ledger ExUnits, a Preprod cost model, transaction budget fit or CPU seconds. Waited-child timing does not measure transitive native CPU, which remains unmeasured. Each experiment consumed one attempt, two Aiken launch commands and zero retries. Compiler release identity is manifest-declared pinned metadata; no binary-from-source proof is claimed. Tool binaries and standard-library caches are not included.

**Trust and target boundaries remain.** The adoptions retain forgeable Class-0 single-party test setup, no trusted ceremony and an unproved VK-to-matrix link. The selected `a2…` policy is not an actual compiled C2 script hash. Expiry `1893456000000` (2030-01-01 UTC) is builder-chosen and unbound. Outref `2222…`, claimant `5555…` and settlement `a2…` are fabricated caller premises, not authenticated ledger facts. The K9 placeholder does not fully validate fees, inputs, minADA or value conservation; its C2 function does not authenticate claimant, outref, predicate, expiry, signatures or unspentness. These are the preserved actual auditors' limitations.

No compiled proof-applied C2/B9 wrapper execution with genuine script identity, live claim-token mint, settlement, actual intent signature, wallet operation or real Preprod transaction is established. The next recorded boundary remains the compiled applied wrapper and genuine hash, FX1, authenticated intent/context integration, concrete transaction economics, qualified target costs and actual Preprod. Source readiness, internal authority, actual local results and ledger acceptance remain separate. All 20 UTxO families; ACTUS's 277 fixtures, 18 types, 32 taxonomy dispositions and DS01–07; DeFi's 72 rows, DA01–24, held-outs, intents and regressions; general financial obligations; real recursion, private handoff and bounded multi-parent PCD; and universal certificates remain open. No whole-compatibility percentage follows.

**Inspecting the public record.** [INVENTORY.json](INVENTORY.json) records each copied payload's original public path, publication path, byte length and SHA-256. The 110 copied payloads total 749,145 bytes:

| Group | Payloads | Contents |
|---|---:|---|
| K8 actual intake | 27 | 25 pinned run/source/capture files, intake manifest, mismatch reproduction |
| K9 actual intake | 31 | 30 pinned run/source/capture files and intake manifest |
| Native root adoptions | 2 | Qualified K8 and K9 local acceptance |
| Four review bundles | 32 | Native K8/K9 and ordinary-correction source/actual reports plus seven public companion records each |
| K8 ordinary correction | 12 | Five source/manifest files, validation result and six root records |
| Aiken source data | 3 | Parser, snapshot and telemetry serialization definitions |
| Historical source reports | 3 | Unchanged K8 R1/R2 and K9 source reviews |

The two [K8 project descriptor](k8/actual/staged/aiken.toml) and [K9 project descriptor](k9/actual/staged/aiken.toml) snapshots are separate exact projects. No combined project was built or tested. The capsule does not bundle a compiler binary or standard library, so these snapshots alone are not a complete offline native build kit. Copied source headers and records saying “source-only”, “pending” or “unrun” retain their authoring-time meaning; this README and the linked later adoptions identify scoped supersession. Original links and absolute paths inside immutable records are provenance; the inventory locates the copied public files.

For ordinary reinspection of the fixed captures, the archived validator reads only its three pinned public inputs using Python's standard library. From the capsule root, with an initially absent output directory beside the capsule:

```sh
python3 -I k8/telemetry-correction/source/validate_capture.py \
  --source k8/actual/staged/lib/pvc2/k8_genuine_proof_test.ak \
  --telemetry k8/actual/run/logs/step2-k8-tests.stdout.log \
  --original-result k8/actual/run/receipts/RESULT.json \
  --output-dir ../k8-captured-validation-fresh
```

This rechecks captured metadata; it does not run Aiken or verify a new proof. Input path strings enter the derived record, so a relocated output need not have the archived VALIDATION.json hash. No such reproduction was run while packaging. Spent native operators are archived experiment evidence with hardcoded paths, not native retry instructions. Internal AUTH and reviewer records govern the recorded experiment only; public developers do not need Foreman, campaign or maintainer admission to compile, prove or deploy supported programs.

The fresh non-author source/actual reviewers used Muse Wonder 1.0 under the user's explicit fallback after the original Opus author timed out. Their independence is same-family freshness, not cross-provider independence; requested high effort is not an invented observed provider metric. [STAGING-MANIFEST.json](STAGING-MANIFEST.json) distinguishes complete byte checking from partial semantic read coverage. This staging candidate still requires fresh independent packaging review before publication.
