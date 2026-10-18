# C2 consumer profile contract r2, revision 3 (CP01-CP08)

Status: AUTHOR-PROPOSED text correction (claude-opus-5-5, tools off). This is attempt 3 of the profile family.
- Attempt 1 (contract plus tests) timed out and is preserved.
- Attempt 2 (VAL/current-cardano-c2-consumer-profile-policy-candidate-20261016/, immutable) was REJECTED by VAL/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/MODEL-REVIEW.json (report SHA-256 eaa31ec5d9404c0a7cfc87c12a84f3a19459d7e0883d6a2ad7cc400f17eee8df): CP04 and CP05 DISAGREE, findings F01-F13.
- This revision is a finding-driven correction in a new root, not a retry and not a provider fallback.

No r2 test or implementation exists. Nothing was executed, imported, hashed or compiled. Author FOR is not adoption: fresh non-author exact clause votes are required before any test is authored. Every authority, runtime, reliance and readiness flag is false.

## 1. Source manifest

Roots:
- RUST = /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust
- HOST = /home/charl/Moriarty/.worktrees/utxo/experiments/cardano-preprod-host
- VAL = /home/charl/moriarty-eutxo-validation-2026-10-12
- PRE = VAL/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs

| Path | Bytes, SHA-256 |
|---|---|
| HOST/host_codec.py | 23252, 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459 |
| VAL/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py (v1) | 15231, 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76 |
| PRE/K6-PUBLIC-PROOF.json | 5800, 930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b |
| PRE/K2-PUBLIC-SETUP-RECORD.json | 3926 declared only by C2-BUILDER-API.json (root records the observed count before test authoring); 86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b |
| RUST/k2/src/{relation,export,lib,hexutil,claim,encoding,input}.rs; RUST/r1cs/source/mori-r1cs/src/abi2.rs; RUST/moriarty/crates/mori-core/src/json.rs; v1 contract and test in VAL/current-cardano-proofexport-c2-consumer-source-candidate-20261016/; VAL/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json | from the root source manifest of this step; not supplied inline and not computed by the author |

The root manifest pins this file and C2-PROFILE-R2-API.json. Neither file pins the other.

## 2. Clauses (EARS; author FOR on all)

- CP01: The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec, whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, which applies the CI02 ARG checks unchanged, and which calls build_env_data, build_c2_mint_data, build_value_data, encode_data, parse_statement_header, statement_digest_hex and statement_public_inputs only as attributes of the unchanged pinned host_codec (23252 B, SHA-256 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459), never copying or re-implementing them; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format /1, eight labels and test history stay immutable.
- CP02: When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is an exact str of length 976, length-checked before its lowercase-hex scan; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256; it shall decode no VK point, so the decode_vk IC-prefix, flag, field-range, curve, subgroup, identity and canonical-encoding checks are not performed.
- CP03: When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests are 64 lowercase hex and sourceSha256 is present as 64 lowercase hex or null; logicalWork is an exact str of 1..39 characters, each in ASCII 0-9, with no leading zero unless it is 0, all checked before any int conversion, and value at most 2**128-1; statementTagHex and familyIdHex equal the ABI v2 literals; headerLen is 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex is an exact str of even length at most 524288, length-checked before any scan, then lowercase hex whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.
- CP04: While admitting the setup record, its relation and the export, the consumer shall accept a key only as an exact str naming a field of the closed schema, an integer only as an exact int (never bool) within its kind range, and a str-kind value only as an exact str containing no code point in U+D800..U+DFFF, found by an allocation-free code-point comparison scan (never str.encode) that runs after any length check of that field; otherwise it shall refuse with C2I-SETUP for a setup top-level member, C2I-PROFILE for any relation key or member, and C2I-EXPORT for any export key or member at any depth, except that statementHex, proofHex, x1 and x2 are not scanned at EXPORT and are left to the length-first checks of STATEMENT, PROOF and DIGEST.
- CP05: When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe its admission only as the selected checks of contract section 3, stating that it performs no decode_vk or decode_proof point check (IC prefix, flags, field range, curve, subgroup, identity, canonical encoding), no setupRecordSha256 file-byte check, no parse_json byte, UTF-8, duplicate-key, trailing-data, depth or node limit, no parse_statement re-encode and no Groth16 pairing, that Rust acceptance of an r2-admitted input is not established, and never asserting any subset, equivalence or conditional-inclusion relation between r2 admission and Rust acceptance.
- CP06: When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 16-entry r2 label map of C2-PROFILE-R2-API.json, and shall retain every v1 check at equal or stricter strength, so that by source construction every r2-admitted input is v1-admitted with envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex equal to the v1 values, a property that tests T02, T16 and T17 only sample.
- CP07: If any check fails, then the consumer shall raise exactly ValueError, never a subclass such as UnicodeEncodeError, whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD; it shall rewrap a host ValueError with the current stage code, let non-ValueError environmental errors such as MemoryError propagate unconverted, return no partial result, never mutate either input, keep no input dict or list reachable from module state or from the result (immutable str and int leaves may be shared), and perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.
- CP08: The r2 records shall keep builderReliancePermitted false and every native, operator, wallet, prover, VM, deploy, signing and network flag false; builderReliancePermitted may change only by a separate root source-adoption decision taken after a reviewed r2 implementation, a fresh non-author audit, root-observed meaningful RED then GREEN of the reviewed T00-T17 test and a reviewed downstream contract that explicitly consumes format moriarty-c2-issue-artifacts/2 with the 16 labels, and none of these prerequisites grants that permission automatically; r2 output is not CB-r1-compatible and shall not be routed to CB r1, and this contract imposes no obligation on CB r1.

Stage notes:
- No check of stage k runs before stage k-1 completes. Order within a stage is free, except that length checks come before scans.
- The CP04 str scan covers relation relationFormat, logicalCostProfile, action and fieldModulusLeHex (at PROFILE), and export action, catalogueActionEntry.name, intentTargetHashV2 and intentAnchor (at EXPORT).
- Setup top-level members have no str kind. Hex, fixed and decimal kinds exclude surrogates through their own character checks, and closed-key checks refuse any surrogate key.
- Every relation fault that v1 reported as C2I-SETUP is C2I-PROFILE in r2.
- The side of the tamper decides the stage:
  - a setup-side abi2ShapeDigest change refuses at PROFILE;
  - a setup-side change to programHash, sourceSha256, action, actionIndex, compilerBuildDigest, r1csSha256 or predicateDigest refuses at RELATION;
  - a setup-side bodyLen change within range refuses at STATEMENT.

## 3. Rust correspondence: selected checks only

Performed: the checks of CP02, CP03 and CI05-CI09 as narrowed by CP04. Each mirrors a named check in:
- parse_setup_record and check_profile;
- check_export, with decode_bounded and decode_exact for length and lowercase hex only;
- parse_statement framing (length, tag, family, 343+bodyLen) through the host;
- check_statement_against (all four comparisons) and cardano_export.

x1 and x2 are compared as exact strings. By source reading of hexutil.rs this gives the same outcome as parse_u128_decimal followed by to_string.

Not performed:
- decode_vk:
  - the IC prefix 0300000000000000 at offset 336;
  - flags, field range, on-curve and prime-order subgroup for all seven points;
  - identity refusal for all seven points;
  - per-point and whole-key canonical re-encoding.
- decode_proof: the same point rules and identity refusal for A, B and C.
- setupRecordSha256 against the SHA-256 of the exact setup file bytes. r2 sees no bytes and checks only hex64.
- input.rs parse_json and mori-core parse_limited (1048576 bytes, UTF-8, duplicate decoded keys, trailing data, depth 32, 65536 nodes), and serde DTO decoding.
- The parse_statement re-encode.
- The Groth16 pairing.

Added by r2: networkId 0; bodyLen <= 261801 at PROFILE; sourceSha256 key required; the Python exact-int and surrogate-free str rules.

Acceptance:
- Rust acceptance of any r2-admitted input is NOT established.
- Attempt 2's 'declared subset of Rust parse_setup_record and check_export', and every subset, equivalence or conditional-inclusion statement between r2 admission and Rust acceptance, is withdrawn, not relabelled.
- The audit's counterexamples are source-derived and were never executed natively, so no runtime counterexample is claimed. They are: invalid-point VK or proof bytes under a consistent BLAKE2b digest, a mismatched setupRecordSha256, and documents outside the parse_json limits.
- Rust also admits inputs that r2 refuses, for example networkId 1.
- r2 is a structural serializer and verifies no Groth16 proof.

Source-reading note (not executed): the host statement offsets equal the claim.rs offsets and the abi2::statement_bytes order. abi2.rs is unreviewed and was never compiled, so statementReencode stays NOT_CHECKED.

## 4. Output labels (16)

The output has the 12 CI10 keys and format moriarty-c2-issue-artifacts/2. labels is exactly:
- expiry CALLER_SUPPLIED_NOT_PROOF_BOUND
- context CALLER_DECLARED_UNAUTHENTICATED
- policyId CALLER_SUPPLIED_UNAUTHENTICATED
- trustProfile single-party-test-setup/1
- setupRecordLink DECLARED_NOT_BYTE_CHECKED
- groth16 NOT_VERIFIED
- vkCurveValidity NOT_CHECKED
- vkProfile NOT_CHECKED
- proofCurveValidity NOT_CHECKED
- statementReencode NOT_CHECKED
- jsonParseLimits NOT_CHECKED
- relationProfile DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED
- driverId DECLARED_NAME_NOT_PROVENANCE
- rustAcceptance NOT_ESTABLISHED
- circuitVkCorrespondence OPEN
- ledger NO_CLAIM

Scope of the NOT_CHECKED labels:
- vkCurveValidity: flags, field range, curve and subgroup of the seven VK points.
- vkProfile: the IC prefix, identity refusal and canonical re-encoding.
- proofCurveValidity: the point rules and identity refusal for A, B and C.
- jsonParseLimits: the section 3 parse_json limits.

The 8 v1 labels are a value-identical subset. Attempt 2's 14-label map is superseded, not renamed.

## 5. Migration and compatibility

- v1 is never edited, renamed or relabelled, and its outputs gain no profile claim.
- r2 versus v1: by source construction, r2-admitted inputs are v1-admitted, with the four byte fields equal. Runtime evidence covers samples only (T02, T16, T17).
- No bridge converts between /1 and /2.
- CB r1 is not compatible: by source reading, its CB09 format check refuses /2. r2 output shall not be routed to CB r1, and nothing is imposed on CB r1.
- The running downstream R4 contract task targets attempt 2 (14 labels, rustAcceptance DECLARED_SUBSET_NOT_EQUIVALENT). This revision changes that dependency. R4 is incompatible until an explicit reviewed reconciliation; there is no silent mapping.

## 6. Test obligations (future; nothing ran)

One reviewed file, test_c2_issue_artifacts_r2.py, holds the tests.
- Each case applies tampers to fresh fixture dicts. S is the setup record, S.rel its relation, and E the export.
- The r2 and v1 expectations are literal values written from this contract. They are never derived by running r2 or v1.
- In the v1 column, = means the same code, ADMIT means v1 returns its /1 result, and a stage name means v1 refuses at that stage.
- Each of T01-T17 obtains r2 before any other assertion.

| ID | Side | Cases | r2 | v1 |
|---|---|---|---|---|
| T00 | - | host, v1, K6 and K2 bytes and SHA-256 equal the section 1 pins; fixtures parse with duplicate-key and NaN refusal; no assertion about r2 | pass | - |
| T01 | - | import lines exactly import hashlib and import host_codec; 3 positional params plus keyword-only policy_id, no defaults; prepare_c2_issue is the only public function; no host API name in the r2 namespace; r2 bytes re-executed against a counting proxy in sys.modules (restored afterwards) call all seven APIs on K6 | - | - |
| T02 | - | K6 at expiries 0, 23, 24, 2**32, 1893456000000 and 2**64-1, for HIST_POLICY and 01x28: format /2, 12 keys, exactly 16 labels; envCborHex = v1-test ENV_PREFIX + 00, 17, 1818, 1b0000000100000000 or 1bffffffffffffffff + ff; at 1893456000000 it equals the literal HIST_ENV_CBOR and the token is 9fd5c53f6a1a03d20b0409f2ca3ad28bd02d5401564603fab5f81b289cc6d54e; issueCborHex = literal 710-byte 64-byte chunk framing; mintValueCborHex = a1581c+policy+a15820+token+01; four byte fields equal v1 | ADMIT | ADMIT |
| T03 | args | record arguments as list, None, str or _Hooked dict subclass; expiry -1, 2**64, True, None, float or str; policy of 27 or 29 bytes, bytearray, str or None; records untouched | ARG | = |
| T04 | S | one-char change of format, trustProfile, notice, curve, snark or encoding; driver one-char (v1 ADMIT) or int; provingKeySha256 uppercase or 63 hex; provingKeyBytes 0, -1, True or 2**64 (v1 ADMIT); extra or missing key; relation as a list; VK of 974 hex or uppercase; vkDigest flip | SETUP | SETUP |
| T05a | S.rel | each of the 13 v1-consumed keys missing; programHash, compilerBuildDigest, r1csSha256, predicateDigest, abi2ShapeDigest or sourceSha256 as 63 hex or uppercase; tag or family flip; headerLen 342; bodyLen -1 or 261802; publicInputCount 3; actionIndex 65536 or True; action int | PROFILE | SETUP |
| T05b | S.rel | each of the 16 non-consumed keys missing; an extra key; language, semantics or adapter one-char; profileDigest, relationShapeDigest, compilerSourceManifestSha256 or coreClosureSha256 as 63 hex or uppercase; logicalWork '', '01', '+1', '-1', 40 digits, 2**128 as decimal, Arabic-Indic two-eight, '1_0', ' 28', '28' plus LF, 5000 digits, or int 28; instanceVariables 4; relationVersion, constraints or witnessVariables True, -1 or 2**64; relationFormat, logicalCostProfile or fieldModulusLeHex as int; abi2ShapeBytesHex odd, uppercase, 524290 chars, or one byte flipped | PROFILE | ADMIT |
| T05c | S.rel | abi2ShapeDigest set to another valid hex64 | PROFILE | RELATION |
| T06 | S.rel, E | U+D800 in relationFormat, logicalCostProfile or fieldModulusLeHex (v1 ADMIT), in a relation extra key (v1 ADMIT), in relation action alone (v1 RELATION), or consistently in relation action, export action and catalogue name (v1 ADMIT): PROFILE. U+D800 in export action alone (v1 RELATION) or in catalogue name, intentTargetHashV2 or intentAnchor alone (v1 CARDANO): EXPORT. U+D800 in statementHex, proofHex or x1: STATEMENT, PROOF or DIGEST (v1 =) | listed | listed |
| T07 | E | extra key in the export, context, publicInputs, cardano, envV2 (including expiry), settleOutref or catalogueActionEntry; any of these missing a field or given as a list; networkId, version or claimKind 256; actionIndex, consumedIx, outputIndex or index 65536; seq 2**64; any int field True; action int; sourceSha256 as 64 uppercase A; premises reordered, as a tuple, with 5 items, or one char changed | EXPORT | = |
| T08 | E, S.rel | E: each of the 8 identity keys alone set to another valid value (sourceSha256 null, action full, actionIndex 2), or vkDigestBlake2b256 alone. S.rel: programHash, compilerBuildDigest, r1csSha256 or predicateDigest set to 00x32; sourceSha256 null; action full; actionIndex 2 | RELATION | = |
| T09 | E, S.rel | statementHex as 00x262145, odd length, uppercase, or containing g; a flip at offset 0, 16, 48, 80 or 112; last byte removed; S.rel bodyLen 377 | STATEMENT | = |
| T10 | E | each of the 10 context fields changed alone; statement byte 144 flipped; networkId 1 in both statement byte 172 and the context | CONTEXT | = |
| T11 | E | statementDigest 00x32; body byte 400 flipped; x1 or x2 alone set to 0; x1 with a leading 0 | DIGEST | = |
| T12 | E | proofHex uppercase, 382 or 386 chars, or containing g | PROOF | = |
| T13 | E | each envV2 field (version 3, claimKind 1, familyId, the two digests, claimantVkh, settleOutref fields, statementDigest); each catalogueActionEntry field (name full, index 2, three digests); intentTargetHashV2; intentAnchor | CARDANO | = |
| T14 | mixed | two faults per input, earlier stage expected: ARG/SETUP (expiry -1 + trustProfile), SETUP/PROFILE (driver + language; v1 ADMIT), PROFILE/EXPORT (language + E extra key; v1 EXPORT), EXPORT/RELATION (seq True + E programHash), RELATION/STATEMENT (E programHash + last byte removed), STATEMENT/CONTEXT (flip 48 + consumedIx 1), CONTEXT/DIGEST (consumedIx 1 + statementDigest), DIGEST/PROOF (statementDigest + 382-char proof), PROOF/CARDANO (382-char proof + intentAnchor). BUILD excluded as unreachable | earlier | = unless noted |
| T15 | - | deep-copy equality of both inputs after success and after the first-fault refusal of each T14 pair plus a CARDANO fault; no input dict or list in the result by identity, while str and int leaves may be shared; two results share no mutable container; module namespace keys and mutable globals unchanged and holding no input container; effect-token scan as v1 test_11 plus global | - | - |
| T16 | - | every ADMIT cell above: v1 returns /1 with 8 labels while r2 refuses at the listed stage | refuse | ADMIT |
| T17 | E | SYNTHETIC_NOT_PROOF_NOT_ORACLE: K6 statement with claimant 66x28 (offset 144), settleHash b3x28 (173), consumedTxid 33x32 (233), consumedIx 7 (265) and seq 2 (267), networkId still 0; digest and x1/x2 computed by hashlib.blake2b(digest_size=32) and int.from_bytes in the test, never by the host; export context, statement fields, publicInputs, envV2 and intentAnchor updated; proofHex unchanged and proves nothing; r2 and v1 both admit, the four byte fields are equal, and envCborHex equals literal framing segments around the synthetic values | ADMIT | ADMIT |

If the pinned K2 or K6 record, or T17, is refused at GREEN, STOP for review. Never edit fixtures, recompute digests or re-stage expectations. R03 for v1 closes only when the v1 column is implemented, root-executed and separately reviewed.

## 7. Envelope, loading, RED and GREEN

Directory: mode 555, files 444. It holds exactly test_c2_issue_artifacts_r2.py, host_codec.py, c2_issue_artifacts.py (v1), K6-PUBLIC-PROOF.json and K2-PUBLIC-SETUP-RECORD.json. c2_issue_artifacts_r2.py is present at GREEN only.

Root responsibilities, outside the test:
- observe r2 absence before RED;
- record the test file pins and, at GREEN, the r2 bytes and SHA-256.

Run:
- Root inspects sources and effects first, then runs python3 -I -B test_c2_issue_artifacts_r2.py once, with no retry.
- Limits: timeout 10 s, CPU 10 s, address space 512 MiB, fsize 2 MiB, core 0, minimal secret-free environment, no network, before/after pins, total <= 1200 s.
- Test effects: bounded reads, exec of pinned bytes, a process-local sys.modules entry for host_codec, unittest output, and one stderr line with the r2 SHA.

Loading reuses the pinned v1 test pattern:
- _read opens with os.open O_RDONLY|O_NOFOLLOW|O_CLOEXEC, checks for a regular file with fstat, and reads within a bound.
- _module execs compile(bytes, path, exec, dont_inherit=True) into a fresh ModuleType.
- setUpModule checks pins, then installs the host as sys.modules host_codec and loads v1 from its exact bytes. tearDownModule restores sys.modules.
- r2 is read by the same _read with cap 131072. Only FileNotFoundError from that exact open raises AssertionError 'missing r2 implementation: c2_issue_artifacts_r2.py is absent'.
- find_spec, import machinery and ImportError are never used.
- Any other OSError, a non-regular file, oversize, an exec exception or a missing prepare_c2_issue raises text without that phrase.

RED (an unobserved prediction): 18 tests run. T00 passes. T01-T17 fail, each with the phrase. There are 0 errors.

NOT RED, and STOP for review: a pin mismatch, a setUpModule or unittest error, a T00 failure, or any failure without the phrase.

GREEN: 18 passes, exit 0, pins unchanged. Any failure is STOP.

Meaningfulness check:
- It runs in a separate reviewed scratch envelope after GREEN, using root-pinned mutants that are never adopted.
- v1 bytes under the r2 name must fail T02, T04 and T05b.
- r2 without the CP04 scan must fail T06.
- r2 without the shape-digest comparison, or using int() without the ASCII check, must fail T05b.
- Every such failure must be an assertion failure whose text is not the missing phrase.

## 8. Non-claims and open tracks

- v1 remains the adopted, unverified structural serializer under R01-R03. Its history is unchanged, and R03 runtime stays OPEN.
- The full r2 implementation and tests are absent. There is no native, prover, VM, wallet, signing or network action.
- The setup is single-party, TEST ONLY and forgeable. Circuit/VK correspondence is OPEN.
- driver is a declared name only. Fixture-to-source provenance is not established: lib.rs says the crate was never compiled or run.
- Context is caller-declared and unauthenticated. networkId 0 does not separate Preview (first) from Preprod (separate, later); both are OPEN.
- moriarty/1 and moriarty-core/1 are checked as descriptor fields, not recompiled. Compiler Strands and future formats are not covered, and no full Web is claimed.
- OPEN: all 20 UTxO families, general financial residual duties, ACTUS, DeFi, authenticated ledger claims, universal certificates, execution, recursion, private handoff and multiparent.
- The public toolchain stays permissionless. These reviews are not developer permission and not compile, prove or deploy gates.
