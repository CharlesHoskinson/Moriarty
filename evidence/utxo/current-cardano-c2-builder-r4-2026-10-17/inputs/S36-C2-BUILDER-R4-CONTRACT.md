# K10 builder R4 contract (CB401-CB412), dependency-reconciled revision, CONDITIONAL

Status: AUTHOR-PROPOSED replacement source and dependency contract (claude-opus-5-5 HIGH, tools off). It reconciles the earlier R4 candidate with the corrected CP r2 revision 4 candidate and changes nothing else. No test source or implementation is supplied. Nothing was executed, imported, hashed, verified, reviewed or adopted. Authority is 0. sourceReady, builderReliancePermitted and every flag are false.

## 0. History, condition, scope
- Immutable history: the R3 prefix and seam, PREFIX-CONTRACT, CB01-CB12, C2-BUILDER-API, test_c2_builder.py, the reported R3 RED and the failed exit-124 tail attempt. R4 edits, imports, aliases, monkeypatches or bridges none of them. R3 RED and the 11-test CI v1 GREEN are not R4 evidence.
- Preserved, unedited and unadopted: the earlier R4 candidate in current-cardano-c2-builder-r4-contract-candidate-20261016 (contract 21493 B, 9ca6e095...; API 9512 B, 13551a95...) and its ROOT-DEPENDENCY-NOTICE.json (6240c044...), which root keeps with this output. These two files replace that candidate as whole documents, not as patches.
- CONDITIONAL on the corrected CP r2 revision 4 candidate (C2-PROFILE-R2-CONTRACT.md 30809 B, 6e2c3989...; C2-PROFILE-R2-API.json 8563 B, 0f592eea...), the exact N01-N11 repair of the revision 3 candidate in current-cardano-c2-profile-policy-corrected-candidate-20261016, pinned by the root manifest and POLICY-ADOPTION.json. A fresh non-author Opus source-policy review found it SOURCE_POLICY_READY with CP01-CP08 AGREE (CP04 and CP06 the changed literals), and root adopted that exact policy solely as the basis for separately reviewed r2 test authoring; no r2 implementation, test execution, runtime result or builder reliance is adopted.
  - Frozen from that identity (API `dependency`): format /2, the 12 result keys, the 16-entry label map, the 11 C2I stages and the 7 consumer host APIs.
  - Attempt 2 (14 labels, rustAcceptance DECLARED_SUBSET_NOT_EQUIVALENT) was rejected on CP04/CP05 and is superseded. R4 consumes nothing from it.
  - R4 never declares CP adopted for its own reliance. The source-policy review is complete and its required corrections are incorporated in revision 4; the r2 implementation, its test-source review, a fresh non-author audit and GREEN remain open. A CP byte change, or an r2 implementation that differs from these literals, voids R4 until explicit downstream reconciliation and fresh review. Final R4 adoption stays conditional on those steps, on the separate CP08 r2 reliance decision and on fresh votes on CB401-CB412; this revision closes none of them.
- Rust acceptance of an r2-admitted input is NOT_ESTABLISHED. R4 states no subset, equivalence or conditional-inclusion relation between r2 admission, its own gates and Rust acceptance. CP's separate source-construction property (an r2-admitted input is v1-admitted, with four equal byte fields) is only sampled by CP tests T02 and T17 when authored; T16 samples strictness only and gives no sample of it. It does not establish Rust acceptance, and no R4 check relies on it.
- v1 c2_issue_artifacts.py (15231 B, 23c3f99d...) stays prohibited for builder reliance and is never imported. CB r1 is incompatible with /2: r2 output is not routed to it, and R4 imposes no new obligation on CB r1.
- Supported inputs: moriarty-k2-groth16-proof/1 and moriarty-k2-groth16-setup/1 as r2 admits them. Future strand-producer and Web export versions are separate; no new strand format and no full Web is claimed.
- POSIX only, Linux envelope only. R4 needs os.O_NOFOLLOW, O_NONBLOCK, O_CLOEXEC, O_NOCTTY, O_DIRECTORY, and dir_fd support for open, mkdir, stat, unlink, rmdir and rename. If any is missing, that is STOP, never a refusal. CPython behaviour is cited from knowledge.
- Layout:
  1. A new R4 prefix: imports, constants, and the CB402-CB407 functions as new source with a fresh review. R3 text may be reused only where it satisfies these clauses.
  2. The R4 seam.
  3. The tail.
  Root pins each phase.

## 1. Reconciliation map
Only the rows below changed. These are unchanged:
- signatures and exact builtin types;
- path and JSON handling;
- stages and refusal mapping;
- setup byte pinning;
- the finite legacy bridge and host T7 encoding;
- the 488-byte VK and 192-byte proof;
- the renderer;
- atomic manifest publication and cleanup;
- the named residuals.

| Item | Earlier R4 | This revision |
|---|---|---|
| CP basis | attempt 2 | revision 4 candidate (N01-N11 repair of revision 3), source-policy reviewed and adopted for test authoring only |
| consumerLabels | 14; rustAcceptance DECLARED_SUBSET_NOT_EQUIVALENT | 16; adds vkProfile and jsonParseLimits NOT_CHECKED; rustAcceptance NOT_ESTABLISHED |
| result keys, format, stages, host APIs | 12, /2, 11, 7 | unchanged |
| CB409 | 14-entry map | 16-entry map; the 8- and 14-entry maps are refused |
| CB412 | flags false until CP08 and R4 evidence | adds VK profile, Rust parse_json limits and any Rust-acceptance relation as non-claims; reliance only by a separate root decision |
| builderLabels.consumerProfile | CP_R2_CANDIDATE_DECLARED_SUBSET_NOT_ADOPTED | CP_R2_REV4_CANDIDATE_SELECTED_CHECKS_NOT_ADOPTED |
| limitations[3] | "declared subset of Rust acceptance" | selected checks; Rust acceptance not established |
| manifest | 33 keys; 9 builderLabels, 4 limitations, 10 flags | same counts; only nested values change |
| R00, R11, R13, RED | 14 labels; r2 pinned | 16 labels; reviewed and root-adopted r2; label-map mutations |
| API format | c2-builder-r4-api/1 | c2-builder-r4-api/2 |

## 2. Clauses (EARS; author FOR on all)
CB409 and CB412 are reworded and need fresh independent votes. The other ten are text-identical to the earlier candidate, which had author votes only.

- **CB401** The builder shall be a new file build_k10_inputs_r4.py, consisting of a new R4 prefix ending in the line `# === C2B-R4-APPEND-SEAM: append tail below; prefix above is immutable ===` and a tail, whose only import statements are import hashlib, import json, import os, import re, import stat, import sys, import build_k10_inputs as legacy, import c2_issue_artifacts_r2 and import host_codec, which exposes exactly parse_expiry, read_public_file, parse_public_json, check_setup_link, legacy_env_from_host, plan_outputs and main, which calls c2_issue_artifacts_r2.prepare_c2_issue and host_codec APIs by module attribute at call time, and which never imports or binds the identifier c2_issue_artifacts, mutates sys.path or sys.modules, uses importlib, rebinds an imported module or attribute, or bridges format /2 to /1.
- **CB402** Where any public function is called, the builder shall promise behaviour only for exact builtin arguments (str, bytes, int that is not bool, an exact list argv, and dicts returned by parse_public_json or first proven to have only exact str keys), shall invoke no caller-defined hook (subclass, PathLike, __fspath__, __index__, __eq__ or __hash__) before refusing, and shall raise every refusal as a ValueError with one exact str argument `C2B-<STAGE>: <reason>` on one line, where reason is a fixed literal optionally carrying a module constant, an errno class name and number, or, for CONSUMER only, a conforming C2I message, so that no path, argv text or other caller free text is echoed.
- **CB403** When main(argv) is called, the builder shall refuse C2B-ARGV before any filesystem access unless type(argv) is list, len(argv) == 6 in the order prog b9-applied.json proof-export.json setup-record.json expiry-ms output-dir, every element is an exact str, argv[1], argv[2], argv[3] and argv[5] are each non-empty, NUL-free and accepted by os.fsencode (UnicodeEncodeError caught without echo), and parse_expiry admits argv[4] as an exact str fully matching ASCII (?:0|[1-9][0-9]{0,19}) with value <= 2**64-1, with no default, clock, environment or TEST_EXPIRY fallback.
- **CB404** When ARGV passes, the builder shall refuse C2B-OUTDIR, creating nothing, unless os.path.split(argv[5]) gives a basename fully matching ASCII [a-z0-9][a-z0-9._-]{0,99}, the parent ("." when empty) opens with O_RDONLY|O_DIRECTORY|O_CLOEXEC as a directory fd that main keeps as the pinned parent until it returns, and os.stat(basename, dir_fd=parent, follow_symlinks=False) fails with ENOENT, treating only the final component as no-follow and parent-path symlinks as the caller's trust.
- **CB405** When OUTDIR passes, main shall read argv[1], argv[2] and argv[3] in that order with read_public_file and the caps 16777216, 1048576 and 65536, and read_public_file(path, cap) shall refuse C2B-READ unless path is an exact non-empty NUL-free fsencode-encodable str, cap is one of those three constants, a no-follow lstat reports S_ISREG, os.open(os.fsencode(path), O_RDONLY|O_NOFOLLOW|O_NONBLOCK|O_CLOEXEC|O_NOCTTY) succeeds, fstat of that fd reports S_ISREG, and os.read chunks of at most 65536 bytes until EOF or cap+1 total bytes give at most cap bytes, closing the fd on every path, mapping only the listed lstat/open errno classes to C2B-READ and letting every other OSError (including any from fstat, read or close), MemoryError, OverflowError and KeyboardInterrupt propagate unconverted after a best-effort close.
- **CB406** When plan_outputs(b9_raw, export_raw, setup_raw, expiry) runs, it shall first refuse C2B-ARGV unless type(expiry) is int and 0 <= expiry <= 2**64-1, then refuse C2B-JSON for the export (cap 1048576) and then the setup (cap 65536) unless parse_public_json admits exact bytes no longer than cap, without a UTF-8 BOM, decoding as strict UTF-8 to exactly one top-level object with only space, tab, LF or CR before and after it, no raw control character in a string, no duplicate key after unescaping at any depth, no NaN, Infinity, fraction or exponent, every integer literal fully matching ASCII (?:0|-?[1-9][0-9]*) in at most 20 characters counting the sign (so -0 is refused), no lone surrogate in any key or string, and at most 16 nesting levels counting the top-level object as 1, enforced by a lexical pre-scan before decoding and a post-walk, with RecursionError and decode errors mapped to C2B-JSON and no JSON canonicality claimed beyond exact-byte SHA-256 identity.
- **CB407** When JSON passes, plan_outputs shall refuse C2B-LINK unless every key of the export is an exact str and export setupRecordSha256 is an exact str fully matching ASCII [0-9a-f]{64} equal to hashlib.sha256(setup_raw).hexdigest(), and shall then refuse C2B-POLICY unless b9_raw is exact bytes of at most 16777216 bytes whose SHA-256 equals legacy.PIN_B9_APPLIED before legacy.extract_compiled is called, the unchanged extract_compiled returns (code_hex, policy_hex) without ValueError, policy_hex is 56 lowercase hex, and hashlib.blake2b(bytes([3]) + bytes.fromhex(code_hex), digest_size=28).hexdigest() == policy_hex, using those 28 bytes as policy_id; the pin is wrapper integrity, not program admission.
- **CB408** When POLICY passes, plan_outputs shall call c2_issue_artifacts_r2.prepare_c2_issue(export, setup, expiry, policy_id=policy) exactly once and, if it raises ValueError, shall refuse with `C2B-CONSUMER: ` followed by the message when the exception has exactly one exact str argument of at most 1024 characters, without CR or LF, beginning `C2I-<STAGE>: ` for a STAGE in ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO or BUILD, and otherwise with `C2B-CONSUMER: nonconforming consumer refusal` without echo, letting every non-ValueError exception propagate.
- **CB409** When CONSUMER returns, plan_outputs shall refuse C2B-BRIDGE unless the result is an exact dict with exactly the 12 API resultKeys; format is exactly moriarty-c2-issue-artifacts/2; labels is an exact dict equal to the frozen 16-entry r2 label map of API dependency.labels, so that the v1 8-entry map, the superseded 14-entry map and any missing, extra or changed entry are refused; policyIdHex == policy.hex(), expiry is an exact int equal to the argument, and setupRecordSha256Declared equals the LINK digest; every *Hex value is an exact even-length lowercase hex str; envData equals host_codec.build_env_data over predicateDigest, vkDigestBlake2b256, context claimantVkh, consumedTxid, consumedIx and statementDigestBlake2b256 of the admitted export and expiry, legacy_env_from_host admits it, and legacy.cbor_encode of the translation, host_codec.encode_data(envData) and bytes.fromhex(envCborHex) are equal, with BLAKE2b-256 of them equal to envelopeTokenNameHex; mintValueData equals host_codec.build_value_data(None, [(policy, token, 1)]) and legacy.cbor_encode(legacy.value_map(policy, token, 1)), host_codec.encode_data(mintValueData) and bytes.fromhex(mintValueCborHex) are equal; issueData is exactly ('constr', 0, [('bytes', vk), ('bytes', proof)]) with exact types, vk the 488 setup VK bytes and proof the 192 export proof bytes, equal to host_codec.build_c2_mint_data('Issue', vk, proof), and host_codec.encode_data(issueData) == bytes.fromhex(issueCborHex), host-only because the legacy encoder fails closed above 64 bytes; and the T7 legacy env CBOR equals host_codec.encode_data of build_env_data with legacy.flip_first_bit(vk digest).
- **CB410** When BRIDGE passes, plan_outputs shall construct exactly the rows T1 (consumer env, consumer token, quantity 1), T5 (quantity 2), T6 (legacy.flip_first_bit of the token) and T7 (env with flipped vk digest and its recomputed token), all with the genuine VK and proof, compute each row's envelopeCborHex, tokenNameHex, quantity, envVkDigestHex, envStatementDigestHex and proofHexSha256 itself, assemble every other manifest field from the frozen API literals and the checked values, call the verbatim Sol helper _render_prepared_outputs only after all of this, and return its fresh dict of exactly the 6 API relpaths to bytes, with T2-T4 only as coverageGaps and no I/O, clock, randomness or environment access.
- **CB411** When planning succeeds, main shall create the output with os.mkdir(basename, 0o755, dir_fd=parent) and contexts with os.mkdir("contexts", 0o755, dir_fd=out), pinning each by an O_RDONLY|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC fd, write script.hex and then the T1, T5, T6 and T7 contexts each through os.open(name, O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC|O_NOCTTY, 0o644, dir_fd=pinned) with a complete os.write loop and close, re-read the three inputs with read_public_file and the same caps refusing C2B-REREAD on any refusal or byte difference, write manifest.json.tmp the same way and publish it last by os.rename to manifest.json within the pinned output fd, print exactly one JSON line {"output": argv[5], "cases": 4} and return 0, mapping every OSError while creating, writing, closing or publishing to C2B-WRITE; after the first mkdir, any refusal or exception shall first trigger cleanup of exactly the entries this call created (unlink files, rmdir contexts, rmdir output, all by pinned dir fd, OSError ignored), and every refusal at any stage shall print exactly one stderr line, no stdout, and return 2, while non-refusal exceptions propagate.
- **CB412** The builder and manifest shall claim nothing about Groth16, VK or proof curve validity, VK profile, statement re-encode, Rust parse_json limits, circuit/VK correspondence, Rust acceptance of any admitted input (including any subset, equivalence or conditional-inclusion relation), the ledger or provider, fees, min-ADA, collateral, ExUnits, fit, Preview, Preprod or operator compatibility, and shall infer none of these from the r2 descriptor checks or the builder's own gates; shall carry the frozen builderLabels, limitations and the 16 r2 consumerLabels, label the context FABRICATED_LOCAL_VM_TEST, the expiry caller-supplied and not proof-bound, the declared context unauthenticated and the setup single-party TEST ONLY and forgeable; and shall keep every flag and builderReliancePermitted false, which may change only by a separate root source-adoption decision taken after the CP08 root decision for the reviewed r2, a fresh review of this contract, a reviewed R4 implementation, a fresh non-author Opus audit with a Muse review of the Sol glue, and root-observed meaningful RED then GREEN all exist, none of which grants it automatically.

## 3. main stages, effects, exceptions (Opus owns order and mapping)
| # | Stage | Effects | On failure |
|---|---|---|---|
| 1 | ARGV | none | C2B-ARGV |
| 2 | OUTDIR | read-only parent dir fd; stat | API errno.outdirRefusal: C2B-OUTDIR; else propagate |
| 3 | READ b9, export, setup | bounded reads | errno.readRefusal or policy: C2B-READ; else propagate |
| 4 | plan: ARGV-int, JSON, JSON, LINK, POLICY, CONSUMER, BRIDGE, RENDER | pure | C2B-<stage>. In POLICY, an extract ValueError is C2B-POLICY. In BRIDGE, host and legacy ValueErrors are C2B-BRIDGE. Other exceptions propagate |
| 5 | WRITE: mkdir out and contexts, script.hex, T1, T5, T6, T7 | exclusive creates | every OSError: C2B-WRITE, cleanup |
| 6 | REREAD | 3 reads | refusal or byte mismatch: C2B-REREAD, cleanup. Environmental: cleanup, then re-raise |
| 7 | WRITE manifest.json.tmp, then rename to manifest.json | create, rename | OSError: C2B-WRITE, cleanup |
| 8 | close dir fds; stdout; return 0 | stdout | A close OSError is C2B-WRITE with cleanup. A print error propagates and the output stays complete |

- No mkdir, write-open, unlink or rename happens before stage 5.
- main catches only a ValueError whose single str argument starts with C2B-. Any other exception propagates, after cleanup once stage 5 has begun.
- Cleanup removes only the names this call created, through pinned fds. If cleanup is incomplete, there is still no manifest.json, which marks the output incomplete.
- No fsync or durability is claimed.
- Residual: a same-uid writer to the parent directory or to the created directories can race. The scratch directory is 700.

## 4. Decisions on fresh prefix findings F01-F07
- **F01.** CB403 pre-refuses and CB405 refuses. The fsencoded bytes are opened, and the input is never echoed.
- **F02.** API `errno`: path properties are refusals. Resource and I/O failures propagate, so they never masquerade as input verdicts.
- **F03.**
  - Final-component-only by choice: paths are chosen by the operator and the envelope is read-only.
  - O_NOCTTY is added.
  - An advisory no-follow lstat avoids opening devices and FIFOs unless someone races it. The fstat on the fd stays authoritative.
  - A raced device open can run driver open-time effects before fstat refuses it. That is an accepted, recorded residual. O_NONBLOCK prevents blocking.
- **F04.** cap is one of three constants. Each os.read requests at most 64 KiB. OverflowError is unreachable for those caps and is listed as propagating.
- **F05.** Decided in CB406.
- **F06.** Refusal reasons carry only constants and errno. Consumer text is bounded and checked for conformance.
- **F07.** Decided in CB402. check_setup_link confirms exact str keys before any lookup.

## 5. legacy_env_from_host (BRIDGE, Opus)
- Admits only an exact tuple ('constr', 0, list) whose list holds exactly 9 nodes, in order:
  1. ('int', 2)
  2. ('int', 0)
  3. ('bytes', legacy.FAMILY_V2)
  4. ('bytes', 32-byte predicate digest)
  5. ('bytes', 32-byte vk digest)
  6. ('bytes', 28-byte claimant)
  7. ('constr', 0, [('bytes', 32-byte txid), ('int', 0..65535)])
  8. ('bytes', 32-byte statement digest)
  9. ('int', 0..2**64-1)
- Every node must be an exact tuple, list, str, bytes or int; bool is refused.
- It returns a fresh legacy.Constr/I/B tree, never mutates its input, and otherwise refuses C2B-BRIDGE.

## 6. Renderer, ownership, review
- **Opus owns** all security logic in CB401-CB411: the policies, every check, hash and equality, the rows and negative mutations, the manifest values, the writer and cleanup, and the stage and exception mapping.
- **Sol renderer** (`_render_prepared_outputs`) may be appended verbatim below the seam and called only after Opus discharges every precondition in the Sol INTEGRATION-CONTRACT.md:
  - an exact 4-row sequence of exact dicts;
  - translated legacy env trees;
  - row metadata computed by Opus;
  - exact builtin, finite, acyclic data;
  - unchanged legacy and json modules.
  The renderer's "opaque" consumer values are the frozen format /2 and 16-label literals that BRIDGE has already checked.
- **Sol** may write only the argv dispatch skeleton and the stdout/stderr printing as specified.
- **Reviews:** a fresh non-author Opus review of the combined actual source and results, and a fresh Muse review of the Sol glue. Same-provider independence is context only.

## 7. Future ordinary obligations (proposed; no test code; nothing ran)
Every case uses deep copies and independent literal expectations, never output from R4. I/O is bounded, writes go to scratch only, the expected stage is asserted, and input non-mutation is checked. Instrumentation is limited to attribute wrappers on c2_issue_artifacts_r2, host_codec and legacy that either delegate or inject one labelled fault, restored in finally. Patching os, builtins, json or R4 itself is never allowed.

| ID | Obligation |
|---|---|
| R00 | Prerequisites, which must pass before a meaningful RED:<br>- the exact reviewed and root-adopted c2_issue_artifacts_r2.py, with bytes and SHA-256 equal to the root adoption record, plus the adopted CP contract and API pins;<br>- root pins for host_codec (23252 B, 3c18539e...), legacy (24480 B, 6c1cde7f...), K6 (5800 B, 93019863...), K2 (3926 B, 86b588fe...), b9 (10baa179...) and HIST-09;<br>- v1 absent and scratch empty;<br>- r2 on K6 returns /2, the 12 keys and exactly the 16 labels, compared with literals copied from the API;<br>- HIST-09 equals the independent formatter and the legacy render.<br>A missing, unadopted or mismatched r2, or a missing or malformed fixture, is STOP. |
| R01 | AST check: exactly the 9 imports; no binding of c2_issue_artifacts, importlib, sys.modules or sys.path; exactly 7 public names; one R4 seam. After import, v1 is not in sys.modules. |
| R02 | K6 golden: 6-file roster, T1 == HIST-09, script.hex, canonical manifest, envelope, token, policy and mint literals; deterministic. |
| R03 | Fields come from the consumer (edited-statement negative fixture, labelled as such); stale constants are absent. |
| R04 | parse_expiry vectors; the plan gate with True, -1, 2**64, "1" and 1.0; main with a non-list, arity 5 or 7, a non-str element, NUL, and "\ud800" (not echoed). No filesystem effect. |
| R05 | OUTDIR: an existing output, a pre-existing directory, a final symlink, a bad name, an absent parent, a trailing slash and ".". Nothing is created. |
| R06 | READ: cap+1 refused and exactly cap admitted; symlink, absent, directory and FIFO (no blocking); cap 0, 7, True and 2**63; bytes, PathLike and str-subclass paths; "\ud800". No paths in messages. |
| R07 | JSON: the R3 test_05 vectors; leading whitespace admitted; raw control character, -0, -01 and 21 characters refused; a 20-character signed literal passes JSON and is refused at CONSUMER; depth 16 passes JSON and depth 17 is refused. |
| R08 | LINK: the R3 vectors; a non-str key; a hostile key that counts __eq__ and __hash__ calls sees zero calls. |
| R09 | POLICY: b9 plus b" ", bytearray, over-cap. A counter shows extract_compiled is not called on a pin mismatch. |
| R10 | CONSUMER: C2I-EXPORT, C2I-SETUP and C2I-PROFILE are kept verbatim; LINK comes before CONSUMER; nonconforming messages become the literal. |
| R11 | BRIDGE: the 13 R3 test_08 edits applied to r2, plus format /1; labels replaced by the v1 8-entry map or the superseded 14-entry map; rustAcceptance set to DECLARED_SUBSET_NOT_EQUIVALENT; vkProfile or jsonParseLimits removed; a 17th label; a changed setupRecordSha256Declared; and envData built for another expiry. |
| R12 | legacy_env_from_host matches an independent literal tree, refuses the shape and type mutations, and does not mutate its input. |
| R13 | The T5, T6 and T7 contexts match the independent formatter. The manifest has the 33 keys, the 16 consumerLabels, the 9 builderLabels, the 4 limitations and the 10 false flags as literals, plus gaps and inputs. No manifest string contains 'subset' in any letter case. |
| R14 | CLI success: exact tree, mode 0o644 & ~umask, one stdout line, empty stderr, no .tmp file left, inputs unchanged. |
| R15 | CLI refusals: one stderr line, exit 2, no stdout, scratch unchanged; precedence ARGV > OUTDIR > READ. |
| R16 | A 0555 parent gives C2B-WRITE with nothing created. A consumer wrapper edits a scratch input copy, giving C2B-REREAD with the output removed. Must run as an unprivileged uid; uid 0 is STOP. |
| R17 | Counters show the consumer reaching all 7 host APIs and the builder's 4 host calls; parsed dicts unmutated; no file outside scratch touched. |

## 8. RED and envelope
- **RED:** R00 passes, and each of R01-R17 fails only with `missing implementation: build_k10_inputs_r4.py`. That text is raised only when the path does not lexist. 0 errors.
- **STOP, never RED:**
  - r2 absent, not root-adopted, or failing its pin;
  - a fixture missing, malformed or failing its pin;
  - a setUpModule error;
  - ImportError or SyntaxError from a file that is present;
  - an R00 failure;
  - any unittest error.
- **Meaningfulness probe** (review-time, not RED): a stub R4 with the CB401 imports and 7 names that raise NotImplementedError must pass none of R02-R17.
- **Later GREEN envelope:**
  - Command: `python3 -I -B isolated/test_c2_builder_r4.py`, once, no retry, after root inspection.
  - Limits: CPU 10 s, wall 20 s, address space 512 MiB, fsize 2 MiB, core 0. Environment: PATH, LANG and PYTHONDONTWRITEBYTECODE only.
  - isolated/ is 555 and its files are 444. It holds the root-adopted r2 bytes and no v1. scratch/ is 700 and empty.
  - Before and after pins of every isolated file.
  - The envelope grants no native, network, key or VM action. rlimits are not a network sandbox, and no instrumented-network-isolation claim is made.

## 9. Adoption and reliance order (proposed)
1. The CP source-policy review is complete and every required correction is incorporated in revision 4; root has adopted that exact policy for test authoring only. Root still decides separately, at CP08, whether to adopt the r2 contract and source for reliance.
2. This contract is reconciled with the final CP literals and receives fresh non-author votes on all 12 clauses.
3. Tests are authored from section 7 and reviewed. R00 requires the root-adopted r2. Meaningful RED is observed.
4. Implementation, GREEN, a fresh non-author Opus audit and a Muse review of the Sol glue.
5. Separate root decisions: CP08 for r2 reliance, then R4 adoption and builderReliancePermitted.

r2 source adoption is not builder reliance: R4 RED needs only the former. No step grants reliance automatically.

## 10. Non-claims and open tracks
- The setup is single-party, TEST ONLY and forgeable. Circuit/VK correspondence is OPEN.
- r2 performs selected descriptor checks only, and Rust acceptance is NOT_ESTABLISHED. No curve, Groth16, VK-profile, statement re-encode or ledger verification is inferred from those checks or from CB406-CB409.
- The CB406 gate is the builder's own (depth 16, no node bound). It is not Rust parse_json or parse_limited, and the jsonParseLimits label stays NOT_CHECKED. The CB407 file-byte link holds for one builder run only and does not establish Rust acceptance.
- Still OPEN and required: all 20 UTxO families, current Rust moriarty/1 and core/1, financial and residual obligations, ACTUS, DeFi, universal certificates, authenticated ledger claims, recursion, private handoff and bounded multiparent PCD.
- Execution and ledger claims are distinct tracks. Preview acceptance comes first and Preprod acceptance later; both remain required and open.
- The public Moriarty toolchain is permissionless. Review controls are internal project process, not developer permissions.
- This is a public source and data builder only. It contains no operator content and does not replace any controller. The earlier controller refusal is out of scope: it is not retried, rephrased or rerouted.
