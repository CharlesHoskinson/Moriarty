# C2 consumer profile contract r2 (CP01-CP08)

Status: AUTHOR-PROPOSED contract and policy only (claude-opus-5-5, tools off).
- No r2 test source or implementation exists. Nothing was executed, imported, hashed or reviewed, and RED and GREEN are unobserved.
- Attempt 2 of this profile revision family. The terminal predecessor (S12, exit 124, no valid object) is preserved and contributes nothing.
- Native and operator authority are 0. builderReliancePermitted is false.

## 1. Basis

- v1 c2_issue_artifacts.py is adopted only as an unverified structural serializer under R01-R03 (S05). Its bytes, CI01-CI12, format /1, eight labels and test history are immutable.
- Rust sources read: relation.rs (RelationDescriptor, check_profile), export.rs (SetupRecord, parse_setup_record, check_export, cardano_export), lib.rs (DRIVER_ID, ADAPTER) and hexutil.rs. Fixtures read: S06 and S07.
- Correction: RelationDescriptor has 29 fields, not the 30 in the R01 wording. r2 closes the relation to those 29.
- Unread, so not promised:
  - encoding.rs (decode_vk, decode_proof, curve and subgroup rules);
  - claim.rs (parse_statement re-encode, check_statement_against, MAX_STATEMENT_BYTES);
  - input.rs parse_json limits, mori_r1cs and mori_core;
  - the host_codec.py body (r2 relies on the host APIs only as S01/S02 describe them);
  - the v1 test body (deliberately unread);
  - the CB r1 contract.

## 2. Clauses (EARS; author FOR on all)

- CP01: The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec and whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, applying the CI02 ARG checks unchanged; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format, eight labels and test history stay immutable.
- CP02: When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is exactly 976 lowercase hex; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256.
- CP03: When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys of relation.rs with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests as 64 lowercase hex and sourceSha256 as 64 lowercase hex or null; logicalWork as a canonical unsigned decimal of at most 39 digits and value at most 2**128-1; the ABI v2 statementTagHex and familyIdHex literals; headerLen 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex as even lowercase hex of at most 524288 characters, length-checked before scanning, whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.
- CP04: While admitting either record, the consumer shall accept an integer only as an exact int (never bool) within its declared range and a key or string value only as an exact str containing no code point in U+D800..U+DFFF, and shall otherwise refuse with the code of the stage that admits the enclosing object.
- CP05: When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe the whole admission only as a declared subset of Rust parse_setup_record and check_export, never as equivalence; statement re-encode, VK and proof curve or subgroup validity, the setupRecordSha256 file-byte link, parse_json limits and the Groth16 pairing shall remain labelled non-claims.
- CP06: When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 14-entry r2 label map; every r2-admitted input shall also be v1-admitted, and for such input envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex shall equal the v1 values.
- CP07: If any check fails, then the consumer shall raise exactly ValueError whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD, shall return no partial result, shall never mutate or retain either input on success or refusal, and shall perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.
- CP08: The r2 records shall keep builderReliancePermitted false until a new r2 implementation, an independent non-author audit, root-observed meaningful RED then GREEN of ordinary tests covering obligations T00-T17, and a new downstream CB contract that explicitly accepts format moriarty-c2-issue-artifacts/2 with its 14 labels all exist, and shall keep every native, operator, wallet, prover, VM and deploy flag false regardless; a CB r1 consumer expecting format /1 or eight labels shall refuse r2 output and shall not treat it as compatible.

No check of stage k runs before stage k-1 completes. Order within a stage is free, except for the length-before-scan rule in CP03. Relation-key faults that v1 reported as C2I-SETUP are reported as C2I-PROFILE.

## 3. Rust correspondence (declared subset)

- Mirrored by text:
  - SetupRecord and RelationDescriptor closure, the export.rs literals and DRIVER_ID;
  - check_profile, provingKeySha256 and provingKeyBytes;
  - VK length and digest;
  - the check_export identity, context, digest, proof-length, cardano and premises comparisons.
- Narrowed:
  - the sourceSha256 key is required (serde would read a missing Option as None);
  - bodyLen <= 261801 and networkId 0;
  - x1/x2 are compared as exact host strings;
  - the shape-bytes bound reuses the v1 524288-character figure.
- Type-only, as in Rust: relationFormat, logicalCostProfile, fieldModulusLeHex, action, relationVersion, constraints, witnessVariables.
- Not mirrored, so labelled: decode_vk/decode_proof, parse_statement re-encode, check_statement_against beyond CI07, the setupRecordSha256 byte link, parse_json limits and the Groth16 pairing.

## 4. Output r2

The output has the 12 CI10 keys and format moriarty-c2-issue-artifacts/2. labels is exactly:
- expiry CALLER_SUPPLIED_NOT_PROOF_BOUND
- context CALLER_DECLARED_UNAUTHENTICATED
- policyId CALLER_SUPPLIED_UNAUTHENTICATED
- trustProfile single-party-test-setup/1
- setupRecordLink DECLARED_NOT_BYTE_CHECKED
- groth16 NOT_VERIFIED
- vkCurveValidity NOT_CHECKED
- proofCurveValidity NOT_CHECKED
- statementReencode NOT_CHECKED
- relationProfile DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED
- driverId DECLARED_NAME_NOT_PROVENANCE
- rustAcceptance DECLARED_SUBSET_NOT_EQUIVALENT
- circuitVkCorrespondence OPEN
- ledger NO_CLAIM

## 5. Compatibility and migration

- v1 is never edited, renamed or relabelled, and its outputs gain no profile claim. r2 is a new file with new pins.
- r2 admits a subset of what v1 admits. When both admit, the four byte fields are equal; only format and labels differ.
- No bridge converts between /1 and /2.
- CB r1 is not compatible and must refuse r2. A new reviewed CB contract must accept /2, the 14 labels and C2I-PROFILE first.

## 6. Test obligation matrix (future; nothing ran)

Fixtures: the pinned host_codec.py, K6-PUBLIC-PROOF.json, K2-PUBLIC-SETUP-RECORD.json and the v1 module. Each case applies one tamper to a deep copy. The expected refusal stage is in brackets.

| ID | Obligation |
|---|---|
| T00 | All pins match and the r2 module is absent at RED. A failure here is STOP, not RED. |
| T01 | Imports are exactly hashlib and host_codec; the signature has keyword-only policy_id and no defaults; a counting host proxy sees all seven APIs. |
| T02 | K6 succeeds with format /2, the 12 keys and exactly the 14 labels. The four byte fields equal v1 at expiries 0, 23, 24, 2**32, 1893456000000 and 2**64-1, for two policies. |
| T03 | CI02 cases: bool, dict subclass, bytearray, a 27- or 29-byte policy; the records stay untouched [ARG]. |
| T04 | A one-character change to each literal; driver changed or non-str; provingKeySha256 uppercase or 63 hex; provingKeyBytes 0, -1, True or 2**64; an extra or missing key; relation not a dict; VK of 974 hex or uppercase; a VK digest flip [SETUP]. |
| T05 | Each of the 29 keys missing; an extra key; language, semantics or adapter changed; each digest malformed, plus sourceSha256; logicalWork '', '01', '+1', 40 digits, 2**128 or an int; instanceVariables 4; publicInputCount 3; headerLen 342; bodyLen -1 or 261802; a tag or family flip; a u64 field as bool or 2**64; actionIndex 65536; a str field non-str; shape bytes odd, uppercase, 524290 characters, or one byte flipped [PROFILE]. |
| T06 | A lone surrogate in a setup value, a relation value, an export value and a key [stage of the owning object]. |
| T07 | An extra key in each nested object, including expiry in envV2; a missing field; a non-dict nested object; u8/u16/u64 upper bound + 1; bool ints; a str kind refused; malformed non-null hex-or-null; premises reordered, as a tuple, with 5 items, or one character changed [EXPORT]. |
| T08 | Each of the 8 identity keys alone; the export vkDigestBlake2b256 alone [RELATION]. |
| T09 | 524290 characters; odd length; uppercase; non-hex; a tag, family, predicate, shape (offset 80) or vk flip; relation bodyLen changed within range [STATEMENT]. |
| T10 | Each context field alone; a consistent networkId 1 [CONTEXT]. |
| T11 | A digest flip; x1 alone; x2 alone; x1 with a leading zero [DIGEST]. |
| T12 | proofHex uppercase, 382 or 386 characters [PROOF]. |
| T13 | Each envV2 and catalogueActionEntry field; intentTargetHashV2; intentAnchor [CARDANO]. |
| T14 | For each adjacent stage pair, a two-fault input reports the earlier stage. |
| T15 | Deep-copy equality after success and after one refusal per stage; no input aliasing; fresh results; an effect scan. |
| T16 | The r2-only refusals (driver, PROFILE, surrogate, the u64 bound) are admitted by v1. |
| T17 | One host-built synthetic context, labelled as synthetic input, not a proof. |

The BUILD and DIGEST host rewraps are unreachable for admitted inputs (R04), so they carry no ordinary obligation.

## 7. RED discriminator

At RED the directory holds the pinned fixtures and v1, but no c2_issue_artifacts_r2.py.
- T00 passes.
- Every other test fails with assertion text containing 'missing r2 implementation'. That text is raised only when importlib.util.find_spec returns None for the r2 module.
- NOT RED, and STOP for review: a pin mismatch, a setUpModule error, a unittest error, an ImportError or SyntaxError from a file that is present, or a T00 failure. Fixture failures are never counted as RED.
- Meaningfulness check: a v1 copy placed under the r2 name must fail T02, T04 and T05 with different text.

## 8. Non-claims

- The setup is single-party, TEST ONLY and forgeable. Circuit/VK correspondence is OPEN.
- Context is caller-declared. networkId 0 does not separate Preview (first) from Preprod.
- The moriarty/1 and moriarty-core/1 profiles are preserved. Singletons and Source/5 establish no Web.
- Certificates, executions and ledger claims stay distinct tracks. Recursion, private handoff, bounded multiparent, all20 UTxO, financial duties, ACTUS and DeFi stay OPEN.
- Tooling stays permissionless. These controls are not compile, prove or deploy gates.
