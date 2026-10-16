# K10 builder revision contract (CB01-CB12)

Status: AUTHOR-PROPOSED source contract and ordinary test only.

- Author: `claude-opus-5-5` HIGH, tools off.
- Nothing was executed, imported, hashed or reviewed, and no implementation is supplied.
- RED and GREEN are unobserved, and no approval exists.
- Native, operator, wallet, prover, VM, deploy, key and network authority are all 0.
- This contract does not touch any controller or operator patch.

## Dependency (STOP if incompatible)

`build_k10_inputs_r3.py` consumes `c2_issue_artifacts.prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id)` unchanged, as frozen in CI01-CI12.

That consumer contract is AUTHOR-PROPOSED. Its first ordinary RED was observed (1 pass, 10 missing implementation). Its implementation is pending, and it is not GREEN, reviewed or adopted.

This contract assumes the CI section 4 result: an exact 12-key dict, with `labels` an exact dict of the 8 listed pairs.

A source cross-check finds the host EnvV2 and no-ada Value encodings byte-compatible with the legacy `cbor_encode` for leaves of 64 bytes or less. If the implemented consumer differs, STOP: neither side is adapted.

## Gap closed

The R2 builder (24480 B, `6c1cde7f…`, unchanged) reads `k8_fixture.ak` and the constants claimant 55x28, outref 22x32#0 and TEST_EXPIRY.

r3 takes claimant, outref, VK/statement digests, VK, proof and token only from the consumer result. That result is computed over the public ProofExport and setup JSON, and the expiry comes from argv. The legacy `frozen_positive_cbor` is K6 regression only.

## Clauses (EARS; author position FOR on all)

- **CB01** The builder shall be a new file `build_k10_inputs_r3.py` beside the unchanged `build_k10_inputs.py`.
  - It shall import only `hashlib json os re stat sys`, `build_k10_inputs as legacy`, `c2_issue_artifacts` and `host_codec`, and shall never mutate `sys.path`.
  - It shall expose exactly `parse_expiry, read_public_file, parse_public_json, check_setup_link, legacy_env_from_host, plan_outputs, main`.
  - It shall call `c2_issue_artifacts.prepare_c2_issue` by attribute at call time.
- **CB02** When `main(argv)` is called, the builder shall refuse `C2B-ARGV` unless:
  - `len(argv) == 6`, in the order `prog b9-applied.json proof-export.json setup-record.json expiry-ms output-dir`;
  - `parse_expiry` admits argv[4]: an exact str that fully matches ASCII `0|[1-9][0-9]{0,19}` with value <= 2**64-1.

  There is no default, clock, environment or TEST_EXPIRY fallback.
- **CB03** When ARGV passes, the builder shall refuse `C2B-OUTDIR`, before any read or creation, unless:
  - the output basename fully matches `[a-z0-9][a-z0-9._-]{0,99}`;
  - the path does not lexist;
  - its parent is an existing directory.
- **CB04** When OUTDIR passes, the builder shall refuse `C2B-READ` unless each input:
  - opens with `O_RDONLY|O_NOFOLLOW|O_NONBLOCK|O_CLOEXEC`;
  - is a regular file according to fstat;
  - yields at most `cap` bytes when at most `cap+1` bytes are read. The caps are b9 16 MiB, export 1 MiB and setup 64 KiB.
- **CB05** When `plan_outputs(b9_raw, export_raw, setup_raw, expiry)` runs, it shall:
  - first refuse `C2B-ARGV` unless expiry is an exact int in 0..2**64-1;
  - then refuse `C2B-JSON`, for the export and then the setup, unless `parse_public_json` admits it.

  `parse_public_json` admits only:
  - exact bytes no longer than the cap;
  - strict UTF-8 without a BOM;
  - one JSON value followed only by whitespace, with a top-level object;
  - no duplicate key at any depth;
  - no NaN or Infinity, and no fraction or exponent;
  - integer literals of at most 20 characters;
  - no lone surrogate in any key or string;
  - nesting of at most 16.

  RecursionError and decode errors map to the refusal.
- **CB06** When JSON passes, the builder shall refuse `C2B-LINK` unless export `setupRecordSha256` is an exact 64-lowercase-hex str equal to the SHA-256 of the exact setup file bytes.
- **CB07** When LINK passes, the builder shall refuse `C2B-POLICY` unless SHA-256(b9 bytes) == `legacy.PIN_B9_APPLIED` and the unchanged `legacy.extract_compiled` succeeds, and it shall use the recomputed hash as policy_id.
  - The pin is the integrity of the unchanged compiled C2 wrapper, not program admission.
  - No legacy fixture or context constant is used for the new context.
- **CB08** When POLICY passes, the builder shall call `prepare_c2_issue(export, setup, expiry, policy_id=policy)` once and rewrap any ValueError as `C2B-CONSUMER: <message>`.
- **CB09** When CONSUMER passes, the builder shall refuse `C2B-BRIDGE` unless all of the following hold:
  - the result has exactly the 12 CI keys and its format;
  - `legacy_env_from_host` admits the exact EnvV2 shape (exact tuple, list, str, bytes and int types; widths 32/32/32/28/32/32; family literal; version 2; kind 0; ix u16; expiry u64);
  - `legacy.cbor_encode(env) == envCborHex` bytes, and its BLAKE2b-256 == `envelopeTokenNameHex`;
  - `legacy.cbor_encode(legacy.value_map(policy, token, 1)) == mintValueCborHex`;
  - issueData is exactly `('constr', 0, [('bytes', vk488), ('bytes', proof192)])`, with vk == setup VK bytes and proof == export proof bytes;
  - `host_codec.encode_data(issueData) == issueCborHex`;
  - the expiry and policyIdHex echoes are equal;
  - the T7 legacy env CBOR == `host_codec.encode_data(build_env_data(...))`.
- **CB10** When BRIDGE passes, `plan_outputs` shall return a fresh dict of exactly 6 relpath-to-bytes entries for script.hex, manifest.json and the contexts of T1, T5, T6 and T7.
  - Contexts shall be rendered only by the unchanged `legacy.script_context` and `uplc_term`.
  - T2-T4 shall be recorded as coverageGaps.
  - No I/O, clock, randomness or environment access is performed.
- **CB11** When planning succeeds, `main` shall:
  - create the output directory and `contexts/` exclusively;
  - write each file `O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC` with mode 0644;
  - re-read all inputs and refuse `C2B-REREAD` on any change;
  - write manifest.json last, print one JSON line and return 0.

  Any refusal shall print exactly one stderr line `C2B-<STAGE>: <reason>` with no stdout and return 2. An OSError while creating outputs is `C2B-WRITE`, and a missing manifest marks an incomplete output.
- **CB12** The builder and manifest shall claim nothing about Groth16, VK/proof curve validity, the ledger or provider, fees, min-ADA, collateral, ExUnits, fit, Preview/Preprod or operator compatibility. They shall label:
  - the context FABRICATED_LOCAL_VM_TEST (zero fee, empty inputs, zero txid, everything interval);
  - the expiry as not proof-bound;
  - the setup as single-party TEST ONLY and forgeable.

  All flags stay false.

## Bridge disposition

The legacy renderer and encoder are REUSED UNCHANGED. The only new code is the finite EnvV2 translator plus the byte-equality checks on every call. No general host-Data-to-UPLC renderer is written.

Issue CBOR is not legacy-encodable, because the legacy encoder fails closed above 64 bytes. It is therefore checked only against the host and the file leaves; independent chunked checking stays open.

## Ownership and review

- **Opus (security; implemented after freeze):** parse_expiry, read_public_file, parse_public_json, check_setup_link, legacy_env_from_host, the BRIDGE block, stage order, and the exclusive writer.
- **Sol (mechanical only):** argv dispatch, the case table, legacy render calls, manifest assembly per the API, and reporting.
- **Reviews:** a fresh non-author Opus review of the combined actual code and results; a fresh Muse review of the Sol glue. Same-provider independence is context only.

## Prerequisites, envelope, RED

See C2-BUILDER-API.json.

The consumer must exist and its own ordinary run must have been observed. b9-applied.json and HIST-09 must be staged and pinned.

RED (unobserved): test_01 passes, and test_02 to test_10 each fail with `missing implementation`. Any setUpModule error, pin mismatch, ImportError or test_01 failure means STOP, never RED.

## Non-claims

- The setup is single-party, TEST ONLY and forgeable.
- VK-to-matrix, compiler and hash-gadget correspondence is OPEN.
- F01 None means canonical no-ada; 0 is non-canonical by design.
- Redeemer chunking above 64 bytes is structural only.
- Real transaction assembly is separate. Preview comes first and Preprod later.
- Full20 UTxO, ACTUS, DeFi, recursion, private, multi-parent and universal obligations remain open.
