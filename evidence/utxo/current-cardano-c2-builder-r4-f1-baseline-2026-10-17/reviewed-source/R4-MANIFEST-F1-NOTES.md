# R4 manifest F1 test-source revision (Parts A and B): author notes

## Identity and authority
- Requested: claude-opus-5-5 HIGH, with claude-opus-4-8 HIGH allowed as a fallback by the user.
  Actual: claude-opus-5-5 (Opus 5.5). The harness sets the effort level and I have not verified
  it myself. Nothing is relabelled.
- This is author work done in one source-only attempt. It is not a review vote and not an
  adoption. The provider is the same as for the prefix author and the Part A/B/C authors, so it
  adds no provider diversity.
- Tools used: Read, Write and Edit only. Nothing was run, imported, compiled, AST-parsed, hashed,
  linted or tested, and no shell, Git, network, provider or agent was used. Native authority is
  0. No build_k10_inputs_r4.py exists or was read, and none was written.

## What changed
- `R4-R01-R08.py.part` (Part A):
  - The class docstring attribution now names this revision.
  - `test_r03` gains F1 stale-field checks.
  - R01, R02 and R04-R08 are byte-identical to the original.
- `R4-R09-R13.py.part` (Part B):
  - Two header comment lines record this revision.
  - `test_r13` gains the full manifest-value oracle.
  - R09-R12 are byte-identical to the original.
- Unchanged:
  - the prefix and Part C;
  - the 18 method names and the method count;
  - the first `self.r4()` call in every method;
  - the class names, wrappers, labels (16), keys (33), refusal stages and their order;
  - the R11 edits (22) and the R12 mutations (7).
- No import, footer, helper, global rebinding, monkeypatch, source-format bridge, v1 consumer or
  host feature was added.

## The two F1 findings and how they close
- Both audits found that R13 checked only that the 33 keys were present. These fields had no
  value check:
  - auditSourcePins;
  - the five scriptHash* fields;
  - vkBytes and proofBytes;
  - vkDigestHex and statementDigestHex;
  - issueCborHex;
  - the four legacy fields: compiledValidatorTitle, scriptHexFile, scriptCompiledBytes and
    scriptInnerFlatBytes.
- Audit B also noted that the prefix literal SCRIPT_HASH_STRINGS was defined but never used.
- R13 now builds an independent `expected` manifest covering exactly the 33 keys. It then
  asserts:
  - (a) `set(expected) == MANIFEST_KEYS`;
  - (b) one subTest equality for each key;
  - (c) exact `int` types for expiry, vkBytes, proofBytes, scriptCompiledBytes and
    scriptInnerFlatBytes, and `is False` for every flag. This matters because `0 == False` and
    `488.0 == 488` would pass a plain equality, and R02's canonical-form check would not catch
    them either;
  - (d) `outputs["manifest.json"] == canonical_json(expected)`, a byte-exact manifest golden.
- R03 is the only fixture that varies the consumer, so it is where stale (hardcoded K6) values
  can be detected. It now asserts:
  - statementDigestHex equals the edited host digest;
  - vkDigestHex equals the export VK digest, which this fixture leaves unchanged;
  - mintValueCborHex is built over the edited token;
  - `cases` equals the four CB410 rows built from the edited fields with the prefix k6_rows
    recipe;
  - the K6 statement digest is absent from every output except script.hex. This is a fifth
    stale literal; the original four are kept.

## Oracle derivations (independent of R4)
- **auditSourcePins and the scriptHash\* strings.** The expected values are the prefix
  `AUDIT_SOURCE_PINS` and `SCRIPT_HASH_STRINGS`. R4-API manifest.keptFromR2 says these are
  copied verbatim from build_k10_inputs.py main().
  - I compared the three strings with main() lines 512-525 character by character, by reading.
  - R13 also anchors `legacy.AUDIT_SOURCE_PINS`, which R00 checks as well.
- **scriptHashClaimedHex and scriptHashIndependentlyRecomputedHex.** The expected value is
  POLICY for both.
  - In legacy main() these are `TOOL_DECLARED_C2_HASH` (anchored equal to POLICY) and the
    recomputed hash.
  - R00 already proves that `extract_compiled(B9)` returns POLICY and that
    blake2b-224(0x03 || code) equals POLICY.
- **vkBytes and proofBytes.** The expected values are 488 and 192. Sources: contract section 1,
  legacy main() `len(vk)`/`len(proof)`, and an anchor on the K2/K6 byte lengths.
- **vkDigestHex.** The expected value is K6 vkDigestBlake2b256. It is anchored to BLAKE2b-256 of
  the K2 VK (computed by the test with hashlib) and to K2 vkDigestBlake2b256. In legacy main()
  this field is `vk_d.hex()`.
- **statementDigestHex.** The expected value is K6 statementDigestBlake2b256, anchored to the
  pinned `host_codec.statement_digest_hex` of the K6 statement. In R03 it is the edited digest.
  In legacy main() this field is `stmt_d.hex()`.
- **issueCborHex.** CB409 defines issueData as ('constr', 0, [('bytes', vk), ('bytes', proof)])
  and says it is host-encoded only.
  - The expected bytes are assembled inline: d8 79 9f; then for each of VK (488 B) and proof
    (192 B): 5f, the 64-byte chunks each as 58 len + bytes, ff; then a closing ff.
  - This follows host_codec `_write_bytes`/`_head`, and the legacy docstring's citation of Pallas
    BoundedBytes, which uses indefinite 64-byte chunks above 64 bytes.
  - It is cross-checked against `host_codec.encode_data(host_codec.build_c2_mint_data("Issue",
    vk, proof))`. host_codec is a pinned dependency, not code under test.
  - The VK has 7 chunks of 64 bytes plus 1 of 40 (head 58 28). The proof has 3 chunks of 64.
- **The four API-unrestated fields.**
  - The expected values are `c2.c2.mint` (anchored to `legacy.C2_TITLES[0]`), `script.hex` (API
    outputs.files[0]), 2770 (`legacy.COMPILED_BYTES_LEN`) and 2767 (`legacy.INNER_FLAT_LEN`).
  - All four are taken from the pinned legacy main() manifest. The R4 API does not restate them,
    so this needs an owner decision (OD1).
- **claimantVkhHex and settleOutref.** For K6 these are k6_fields() (0x55 x 28; 0x22 x 32 with
  index 0). The key names transactionIdHex and outputIndex are the original R03 literals, taken
  from API manifest.other.
- **Everything else.** These are the existing prefix literals and formatters (R2_LABELS,
  BUILDER_LABELS, LIMITATIONS, FLAGS, expected_inputs, expected_gaps, expected_case, env_cbor,
  MINT_VALUE_CBOR_HEX, PINS).

## RED behaviour
- The predicted shape is unchanged: 18 tests run, 1 pass (R00), 17 MISSING failures, 0 errors.
- Every new line comes after the leading `self.r4()` and cannot be reached at RED.
- The earlier RED observation was of the old bytes (c9375097...). It is not evidence for these
  bytes, and its fresh actual-result audit is still pending.
- A fresh combined-source, effects and R00 audit, and any new ordinary RED decision, must follow
  these changed bytes.
- Stub meaningfulness is unchanged: R03 and R13 call plan_outputs first.

## Wrappers and effects
- No wrapper was added.
- R13's new calls are direct and pure:
  - legacy constants;
  - `host_codec.statement_digest_hex`, `build_c2_mint_data` and `encode_data`.
- R13 makes no filesystem access. These calls happen outside R17's counters, so they do not
  change R17's run-minus-baseline arithmetic.
- R03's additions are pure prefix formatters.
- os, builtins, json and R4 are never patched.

## Remaining gaps (disclosed, not filled)
- **Computed facts are predictions.** The hash and CBOR anchors, and the byte-exact manifest
  equality, were derived by reading and nothing was computed. If an anchor were wrong, R13 would
  fail at GREEN without any R4 output being involved.
- **Values that never vary.** vkDigestHex, vkBytes, proofBytes, issueCborHex and proofHexSha256
  are only checked against the genuine values. Varying them would need a new VK or proof, which
  is excluded. The edited-statement fixture stays an explicitly invalid-proof negative control.
- **inputs[] on a varied export** is not asserted, because the API does not define sha256/bytes
  for a non-K6 export (OD2).
- **No execution or mutation testing**, so the kill power of each new check is argued from
  source only.
- **No inferred claims.** Nothing about Groth16, curves, VK profile, statement re-encode, Rust
  acceptance, the ledger, fees, ExUnits, fit or the network is asserted. Full UTxO and crypto
  coverage are not inferred.
- **Open tracks.** All 20 UTxO families, full finance, ACTUS, DeFi, PCD recursion, private
  handoff, bounded multiparent and real Preprod remain OPEN.

## Owner decisions needed
- **OD1.** Confirm or restate in R4-API.json the four legacy-derived values: c2.c2.mint,
  script.hex, 2770 and 2767.
- **OD2.** Decide what inputs[] means for a non-K6 export.
- Neither decision requires a change to the prefix or Part C.

## Files read (whole) and not read
- **Read in full:**
  - MANIFEST.json
  - SOURCE-AUDIT-A.json and SOURCE-AUDIT-B.json, including their F1 findings and citations
  - R4-CONTRACT.md and R4-API.json
  - ROOT-CP08-SOURCE-ADOPTION.json, ROOT-PUBLIC-FIXTURE-PINS.json, ROOT-OBSERVED-RED-RESULT.json
    and ROOT-LITERAL-ASSEMBLY.json
  - USER-BOTH-OPUS-RULE.md
  - R4-TEST-PREFIX.py.part and R4-TEST-INTERFACE.json
  - ORIGINAL-R4-R01-R08.py.part with its CONTRACT and NOTES
  - ORIGINAL-R4-R09-R13.py.part with its CONTRACT and NOTES
  - UNCHANGED-R4-R14-R17.py.part
  - build_k10_inputs.py, c2_issue_artifacts_r2.py and host_codec.py
  - K6-PUBLIC-PROOF.json and K2-PUBLIC-SETUP-RECORD.json
- **Not in the packet, so not read:**
  - the B9 body (opaque, pin only)
  - the HIST-09 body
  - C2-PROFILE-R2 contract and API
  - DOWNSTREAM-R4-CONDITIONAL-ADOPTION.json
  - the Part C contract and notes, and the prefix notes
  - the historical R3 test
  - the SOL renderer and integration contract
  - PROPOSED-RED-ENVELOPE.json and ROOT-MEANINGFULNESS-ACCEPTANCE.json
- No path outside the packet and output directories was opened.

## Escaping
- The source keeps the original Python escapes unchanged.
- Metadata describes escaped values by code point, for example lone high surrogate U+D800, NULL
  U+0000 and LINE FEED U+000A.
