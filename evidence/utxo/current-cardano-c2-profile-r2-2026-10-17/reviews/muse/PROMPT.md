You are a fresh NONAUTHOR independent reviewer of the exact repaired C2r2revision4 serializer source-policy/API. The user explicitly authorized GEMINI+MUSE to replace unavailable Opus for this review. You may judge cryptographic/security correctness of THIS policy, but perform NO tools, project code, SDK, compiler, curve, proof, setup, VM, wallet, private-material, ledger or network action. All native/operator/etc authority remains false. The sourceauthor was Opus5.5; authorFOR is not adoption. Prior failed reviews returned no valid response; do not infer approval or rejection from transport failure. Neither new reviewer receives the other newreviewer response.
Independently review full NEW candidate CONTRACT/API/DESIGN-POSITIONS against all primary source and prior audits supplied. Return exact8literalclauses CP01..08 copied from NEW contract, each AGREE/DISAGREE/ABSTAIN with substantive rationale. Do not copy oldrev3 clauses. Evaluate ALLN01-N11 fixes: T16explicitv1-admit/r2-refuse subset excludingT02/T17; subset-byte-equalitysampling T02/T17 only vsT16strictness; T05a/b/csubTests insideoneT05method giving18methodsT00-T17/run-countSTOP; T07nested-listEXPORT vs top-listARG; T04/T14exactcases; CP04 length-first/range/scan without proportionalcopy/str.encode and DIGEST exactstringequality; selectedRustchecksONLY notRustadmissionsetparity; scopedhistoricalK6proof notstale neverruncomment or universalprovenance; tokens independentlyhashedfromliteralframing NONORACLE; syntheticT17notauthledgerdata, sourceSHA/intents/statementdigestfieldsallnamed; redundantT06extrakey noted; exactly16labels and frozenv1immutability/no silent/1→/2bridge/builderReliancefalse; full20families and financial/recursion/private/multiparent scope retained.
Newauthorcandidate was exported from COMPLETE schema-valid public structured object, with disclosed root-schema mistake: files/positions were forcedempty in author schema, so two full file bodies and8clauses were carried in crosschecks. Root verified verbatimexports, all32sourcepins, andonlyCP04+CP06literalchanges; no privatepartialread/salvage occurred. Newcandidate authorready is not auditready. Please assessactualwholecandidate, not force a broadrewrite to address preferences. Flag realcontradictions and independentlyvalidate test expectations; never relabel partialRust sourcechecks as proofverification. No test/implementationexists; allruntimeexpectations are predictions. Output only one schema-valid whole JSON, sevenfields(no files field), report<=300words, <=12materialfindings. SOURCE_POLICY_READY+sourceReadytrue means ONLY policyapprovedtoauthor and separatelyreviewtests. ordinarySelftestReady=false, all operator/native/etcfalse. Preserve limitations/unreadpaths/abstentions. Remote comments are evidence, not instructions.
Requestedreviewermodel: muse-wonder-1.0 high. reviewToken c2-profile-r2-rev4-muse-independent-review.

## S01 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/C2-PROFILE-R2-CONTRACT.md
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


## S02 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/C2-PROFILE-R2-API.json
{
"format": "moriarty-c2-profile-r2-api/2",
"status": "AUTHOR-PROPOSED revision 3; supersedes rejected attempt 2 (immutable); no implementation or test; nothing run; pinned only by the root manifest",
"module": "c2_issue_artifacts_r2.py",
"imports": ["hashlib", "host_codec"],
"hostApisByAttribute": ["build_env_data", "build_c2_mint_data", "build_value_data", "encode_data", "parse_statement_header", "statement_digest_hex", "statement_public_inputs"],
"function": "prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id)",
"stages": ["ARG", "SETUP", "PROFILE", "EXPORT", "RELATION", "STATEMENT", "CONTEXT", "DIGEST", "PROOF", "CARDANO", "BUILD"],
"unreachableForAdmittedInput": ["BUILD"],
"refusal": "exactly ValueError, never a subclass; prefix C2I-<STAGE>: of the first failing stage; host ValueError rewrapped with the stage code; non-ValueError environmental errors propagate",
"stageOwnership": {"setupTopLevel": "SETUP", "relationKeysAndMembers": "PROFILE", "exportKeysAndMembersAnyDepth": "EXPORT", "notScannedAtExport": {"statementHex": "STATEMENT", "proofHex": "PROOF", "x1": "DIGEST", "x2": "DIGEST"}},
"intRule": "exact int, never bool, within kind range",
"strRule": "exact str without U+D800..U+DFFF; allocation-free code-point scan after any length check; never str.encode",
"scannedStr": {"relation": ["relationFormat", "logicalCostProfile", "action", "fieldModulusLeHex"], "export": ["action", "cardano.catalogueActionEntry.name", "cardano.intentTargetHashV2", "cardano.intentAnchor"]},
"setup": {"keys": "exactly the 12 SetupRecord keys", "fixed": "format, trustProfile, notice, curve, snark, encoding = export.rs constants", "driver": "fixed:mori-k2-driver/0.1.0-k2s1+k4h1", "provingKeySha256": "hex64", "provingKeyBytes": "int:1..18446744073709551615", "relation": "dict", "verifyingKeyHex": "str, length 976 checked first, then lowercase hex; no point decoding", "vkDigestBlake2b256": "hex64 = BLAKE2b-256(VK bytes)"},
"relation": {
"keys": "exactly these 29",
"fixed": {"language": "moriarty/1", "semantics": "moriarty-core/1", "adapter": "SettlementHashV2", "statementTagHex": "4d524731362d5056432d53544d542d32", "familyIdHex": "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32"},
"int": {"headerLen": "343", "instanceVariables": "3", "publicInputCount": "2", "bodyLen": "0..261801", "actionIndex": "u16", "relationVersion": "u64", "constraints": "u64", "witnessVariables": "u64"},
"hex64": ["profileDigest", "programHash", "relationShapeDigest", "compilerSourceManifestSha256", "coreClosureSha256", "compilerBuildDigest", "predicateDigest", "abi2ShapeDigest", "r1csSha256"],
"hex64OrNullKeyRequired": ["sourceSha256"],
"str": ["relationFormat", "logicalCostProfile", "action", "fieldModulusLeHex"],
"logicalWork": "exact str, 1..39 chars, ASCII 0-9 only, no leading zero unless 0, checked before int(); value <= 2**128-1",
"abi2ShapeBytesHex": "exact str, even length <= 524288 checked first, lowercase hex, BLAKE2b-256 = abi2ShapeDigest"
},
"export": "CI05-CI09 unchanged, narrowed by intRule and strRule",
"result": "the 12 CI10 keys; format moriarty-c2-issue-artifacts/2; labels exactly the 16 below",
"labels": {"expiry": "CALLER_SUPPLIED_NOT_PROOF_BOUND", "context": "CALLER_DECLARED_UNAUTHENTICATED", "policyId": "CALLER_SUPPLIED_UNAUTHENTICATED", "trustProfile": "single-party-test-setup/1", "setupRecordLink": "DECLARED_NOT_BYTE_CHECKED", "groth16": "NOT_VERIFIED", "vkCurveValidity": "NOT_CHECKED", "vkProfile": "NOT_CHECKED", "proofCurveValidity": "NOT_CHECKED", "statementReencode": "NOT_CHECKED", "jsonParseLimits": "NOT_CHECKED", "relationProfile": "DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED", "driverId": "DECLARED_NAME_NOT_PROVENANCE", "rustAcceptance": "NOT_ESTABLISHED", "circuitVkCorrespondence": "OPEN", "ledger": "NO_CLAIM"},
"rust": {"acceptance": "NOT_ESTABLISHED; no subset, equivalence or conditional-inclusion claim", "notChecked": ["decode_vk IC prefix, flags, field, curve, subgroup, identity, canonical", "decode_proof point rules and identity", "setupRecordSha256 file-byte link", "parse_json and parse_limited limits", "parse_statement re-encode", "Groth16 pairing"], "addedByR2": ["networkId 0", "bodyLen <= 261801", "sourceSha256 key required", "exact-int and surrogate-free str rules"]},
"v1": {"module": "c2_issue_artifacts.py", "bytes": 15231, "sha256": "23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76", "format": "moriarty-c2-issue-artifacts/1", "labels": 8, "status": "immutable; adopted only as unverified structural serializer; R03 runtime OPEN"},
"compatibility": {"admissionVsV1": "subset by source construction; sampled by T02, T16, T17; not proven", "byteEqualWhenBothAdmit": ["envCborHex", "envelopeTokenNameHex", "issueCborHex", "mintValueCborHex"], "cbR1": "not compatible; r2 output not routed to CB r1; no CB r1 obligation", "r4": "targets attempt-2 14 labels; incompatible until explicit reviewed reconciliation"},
"pins": {"host_codec.py": "23252 B 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459", "c2_issue_artifacts.py": "15231 B 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76", "K6-PUBLIC-PROOF.json": "5800 B 930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b", "K2-PUBLIC-SETUP-RECORD.json": "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b; 3926 B declared only, root-recorded count required", "contractAndApi": "root manifest only; no self-hash"},
"flags": {"sourceReady": false, "implemented": false, "tested": false, "adopted": false, "builderReliancePermitted": false, "native": false, "operator": false, "wallet": false, "prover": false, "vm": false, "deploy": false, "signing": false, "network": false, "publicVerificationInputsPublishable": false}
}


## S03 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/DESIGN-POSITIONS.json
[
  {
    "clauseId": "CP01",
    "clause": "The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec, whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, which applies the CI02 ARG checks unchanged, and which calls build_env_data, build_c2_mint_data, build_value_data, encode_data, parse_statement_header, statement_digest_hex and statement_public_inputs only as attributes of the unchanged pinned host_codec (23252 B, SHA-256 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459), never copying or re-implementing them; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format /1, eight labels and test history stay immutable.",
    "position": "FOR",
    "rationale": "Restates the CI01 host-reuse rule with the seven API names (F11) and keeps v1 history immutable."
  },
  {
    "clauseId": "CP02",
    "clause": "When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is an exact str of length 976, length-checked before its lowercase-hex scan; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256; it shall decode no VK point, so the decode_vk IC-prefix, flag, field-range, curve, subgroup, identity and canonical-encoding checks are not performed.",
    "position": "FOR",
    "rationale": "The audit agreed with the substance. This version states the decode_vk omission explicitly (F01/F02) and adds the length-first rule."
  },
  {
    "clauseId": "CP03",
    "clause": "When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests are 64 lowercase hex and sourceSha256 is present as 64 lowercase hex or null; logicalWork is an exact str of 1..39 characters, each in ASCII 0-9, with no leading zero unless it is 0, all checked before any int conversion, and value at most 2**128-1; statementTagHex and familyIdHex equal the ABI v2 literals; headerLen is 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex is an exact str of even length at most 524288, length-checked before any scan, then lowercase hex whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.",
    "position": "FOR",
    "rationale": "Closes F07: checks are ASCII-only and length-first before int(). The moriarty/1 and moriarty-core/1 descriptor check is preserved without recompilation."
  },
  {
    "clauseId": "CP04",
    "clause": "While admitting the setup record, its relation and the export, the consumer shall accept a key only as an exact str naming a field of the closed schema, an integer only as an exact int (never bool) within its kind range, and a str-kind value only as an exact str containing no code point in U+D800..U+DFFF, found by an allocation-free code-point comparison scan (never str.encode) that runs after any length check of that field; otherwise it shall refuse with C2I-SETUP for a setup top-level member, C2I-PROFILE for any relation key or member, and C2I-EXPORT for any export key or member at any depth, except that statementHex, proofHex, x1 and x2 are not scanned at EXPORT and are left to the length-first checks of STATEMENT, PROOF and DIGEST.",
    "position": "FOR",
    "rationale": "Resolves the CP04 DISAGREE (F06). Stage ownership is explicit, the scan is allocation-free and runs after the length check, there is no encode leak, and four export fields are explicitly deferred."
  },
  {
    "clauseId": "CP05",
    "clause": "When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe its admission only as the selected checks of contract section 3, stating that it performs no decode_vk or decode_proof point check (IC prefix, flags, field range, curve, subgroup, identity, canonical encoding), no setupRecordSha256 file-byte check, no parse_json byte, UTF-8, duplicate-key, trailing-data, depth or node limit, no parse_statement re-encode and no Groth16 pairing, that Rust acceptance of an r2-admitted input is not established, and never asserting any subset, equivalence or conditional-inclusion relation between r2 admission and Rust acceptance.",
    "position": "FOR",
    "rationale": "Resolves the CP05 DISAGREE (F01/F02). All Rust-acceptance relation wording is withdrawn and replaced by explicit performed checks, omissions and NOT_ESTABLISHED."
  },
  {
    "clauseId": "CP06",
    "clause": "When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 16-entry r2 label map of C2-PROFILE-R2-API.json, and shall retain every v1 check at equal or stricter strength, so that by source construction every r2-admitted input is v1-admitted with envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex equal to the v1 values, a property that tests T02, T16 and T17 only sample.",
    "position": "FOR",
    "rationale": "The label count is updated to 16. The r2-versus-v1 subset is kept as a source-construction claim that tests only sample (F05)."
  },
  {
    "clauseId": "CP07",
    "clause": "If any check fails, then the consumer shall raise exactly ValueError, never a subclass such as UnicodeEncodeError, whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD; it shall rewrap a host ValueError with the current stage code, let non-ValueError environmental errors such as MemoryError propagate unconverted, return no partial result, never mutate either input, keep no input dict or list reachable from module state or from the result (immutable str and int leaves may be shared), and perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.",
    "position": "FOR",
    "rationale": "Preserves the CI11 host-rewrap and environmental-error rules and scopes aliasing to mutable containers (F09). A UnicodeEncodeError cannot escape (F06)."
  },
  {
    "clauseId": "CP08",
    "clause": "The r2 records shall keep builderReliancePermitted false and every native, operator, wallet, prover, VM, deploy, signing and network flag false; builderReliancePermitted may change only by a separate root source-adoption decision taken after a reviewed r2 implementation, a fresh non-author audit, root-observed meaningful RED then GREEN of the reviewed T00-T17 test and a reviewed downstream contract that explicitly consumes format moriarty-c2-issue-artifacts/2 with the 16 labels, and none of these prerequisites grants that permission automatically; r2 output is not CB-r1-compatible and shall not be routed to CB r1, and this contract imposes no obligation on CB r1.",
    "position": "FOR",
    "rationale": "Applies F13: CB r1 incompatibility is stated as non-routing, adoption needs a separate root decision, and nothing is automatic. All flags stay false."
  }
]


## S04 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/SOURCE-MANIFEST.json
{
  "format": "moriarty-corrected-cp-policy-author-source-intake/1",
  "observedAt": "2026-10-16T22:53:12.851307+00:00",
  "origin": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-opus-precise-revision-20261016",
  "actualCanonicalModel": "claude-opus-5-5",
  "sourceInputPinsChecked": 26,
  "sourceInputs": [
    {
      "id": "S01",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/MODEL-REVIEW.json",
      "bytes": 44551,
      "sha256": "f46928736648d84b179c6815334967f94cf308a711072a2ac0f051adab088ed8"
    },
    {
      "id": "S02",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/ROOT-QUALIFICATION.json",
      "bytes": 1055,
      "sha256": "f8b0c16c9d69731ce02a0a6e2e721656c2785d01274b58128b2690e3a8a3a15a"
    },
    {
      "id": "S03",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-CONTRACT.md",
      "bytes": 11790,
      "sha256": "c51ff84a153828547060ff00f801ca0beca200b95eb96b4de0388d83f91f5ce8"
    },
    {
      "id": "S04",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-API.json",
      "bytes": 3271,
      "sha256": "7becb4dee68d95fe8830e427c6a0a67e5b92c06e249ef2328cd76b3fc2b5a7b5"
    },
    {
      "id": "S05",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/DESIGN-POSITIONS.json",
      "bytes": 5830,
      "sha256": "b19d4e6c6d5481e19ba3093a964cb986bbd811b2ef7eac15f55886a2037160fb"
    },
    {
      "id": "S06",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/C2-ISSUE-ARTIFACT-CONTRACT.md",
      "bytes": 12645,
      "sha256": "f1a904ad80f5ebf7be6518cb5904da66b729913bf2b59e97555722307b51c851"
    },
    {
      "id": "S07",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py",
      "bytes": 15231,
      "sha256": "23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76"
    },
    {
      "id": "S08",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/REPORT.md",
      "bytes": 11465,
      "sha256": "05c66c0dbeac99f387403498b4012a9bb8f93e8c5ea84602f587312d7f98c34d"
    },
    {
      "id": "S09",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/FINDINGS.json",
      "bytes": 5863,
      "sha256": "00e54bedaf9d2960d0fab408386142a944a293ada9b8ff332e462c9e1c102d6b"
    },
    {
      "id": "S10",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json",
      "bytes": 2690,
      "sha256": "61327c199afd1214b46e770c8af60f75ace84a4f6fdbb8d86f68df3dcc64642f"
    },
    {
      "id": "S11",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K2-PUBLIC-SETUP-RECORD.json",
      "bytes": 3926,
      "sha256": "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"
    },
    {
      "id": "S12",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K6-PUBLIC-PROOF.json",
      "bytes": 5800,
      "sha256": "930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b"
    },
    {
      "id": "S13",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/relation.rs",
      "bytes": 14625,
      "sha256": "e18ae46c5ec0650f0c5dbc3c8b39262eb432c7d2e3be7b3b14cc01fe285493d5"
    },
    {
      "id": "S14",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/export.rs",
      "bytes": 15374,
      "sha256": "8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c"
    },
    {
      "id": "S15",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/lib.rs",
      "bytes": 2783,
      "sha256": "1b13cdc57fa21ab78a5bd6c33ba64d4628d8ed388f4a7121b40b18c5cae4b830"
    },
    {
      "id": "S16",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/hexutil.rs",
      "bytes": 2273,
      "sha256": "d242105dad5dee03c393e95b6b5ba22b19497253d1a9e9135d746b001b7c19ae"
    },
    {
      "id": "S17",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-r2-opus-contract-20261016/PROCESS.json",
      "bytes": 772,
      "sha256": "c3c5128a8135e66893bb697479d8befe83dc996f546527577d5cc6a9a0385242"
    },
    {
      "id": "S18",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-preprod-host/host_codec.py",
      "bytes": 23252,
      "sha256": "3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459"
    },
    {
      "id": "S19",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/test_c2_issue_artifacts.py",
      "bytes": 20643,
      "sha256": "4fa543bf5708957ed9beabb32c2614e50f8c0097fb157ed2c9bd6ed519c9f6f0"
    },
    {
      "id": "S20",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-CONTRACT.md",
      "bytes": 8466,
      "sha256": "93420c97fe7ed382e96b9a9af43ac605e9616298aab670525ebe83eed1f965d4"
    },
    {
      "id": "S21",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-API.json",
      "bytes": 7190,
      "sha256": "abe6631ffc60a06916c13e912ded326429675d2ac03e9fc508d7aab0a3bb036d"
    },
    {
      "id": "S22",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/claim.rs",
      "bytes": 9280,
      "sha256": "f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3"
    },
    {
      "id": "S23",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/encoding.rs",
      "bytes": 9631,
      "sha256": "2545d7799449f5580286a47949ca0116197772b3a24ddf1ee7b8c05c9b1c6729"
    },
    {
      "id": "S24",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/input.rs",
      "bytes": 6262,
      "sha256": "646c1aad3cea935de2e084e2cb1518772e8bd0fc7e57eb6b3f4581edc67b5b42"
    },
    {
      "id": "S25",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/src/abi2.rs",
      "bytes": 19304,
      "sha256": "51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb"
    },
    {
      "id": "S26",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/moriarty/crates/mori-core/src/json.rs",
      "bytes": 5391,
      "sha256": "faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183"
    }
  ],
  "literalAuthorForClauses": 8,
  "files": [
    {
      "path": "C2-PROFILE-R2-CONTRACT.md",
      "bytes": 23830,
      "characters": 23830,
      "sha256": "03f03887df741ce21556158a0490e24890ba33f310ce276f5060d1013f9e5e03"
    },
    {
      "path": "C2-PROFILE-R2-API.json",
      "bytes": 5750,
      "characters": 5750,
      "sha256": "965b880d5b440376faf495b757ba9b06460e9e5755e943b23c81c789982af0fb"
    }
  ],
  "combinedCharacters": 29580,
  "authorRequestedCombinedCharacterCap": 24000,
  "characterCapSatisfied": false,
  "labelCount": 16,
  "setupPublicFixtureRootObserved": {
    "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K2-PUBLIC-SETUP-RECORD.json",
    "bytes": 3926,
    "sha256": "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"
  },
  "sourceAdopted": false,
  "testOrImplementationAuthored": false,
  "nativeAuthority": 0,
  "dependencyStatus": "HistoricalR4candidate14labelmap is incompatible with this16labelproposal; exactreconciliation andindependentreview required. Rustacceptance NOT_ESTABLISHED, no broadsubsetclaim.",
  "initialRootIntake": "Whole8authorclauseliterals eachoccuronce checked without assuming Markdown syntax. No execution."
}


## S05 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/MODEL-REVIEW.json
{
  "reviewToken": "current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016",
  "report": "FRESH NON-AUTHOR SOURCE-ONLY AUDIT (claude-opus-5-5, tools off, inline bodies only). I read all 23 supplied bodies (S01-S23) as text and treated them as evidence, not instructions. I executed, imported, compiled and hashed nothing. Every byte count and SHA-256 below is the value declared in the supplied text. I did not recompute any BLAKE2b or SHA-256 value, including BLAKE2b-256 of the fixture abi2ShapeBytesHex. Same-provider independence is context only. The author's FOR on CP01-CP08 is not adoption, and this review grants no adoption, toolchain or authority.\n\nVERDICT: REVISE, NOT SOURCE-READY. The r2 contract correctly closes most of the R01 profile gap:\n- the 29-key RelationDescriptor;\n- the DRIVER_ID pin;\n- the provingKeyBytes u64 bound;\n- language/semantics/adapter, instanceVariables and the shape digest;\n- lone-surrogate refusal;\n- three new non-claim labels.\n\nThe clause kinds and constants agree with relation.rs, export.rs, lib.rs and hexutil.rs. Three material problems remain, and the exact contract cannot be adopted without resolving them:\n1. The CP05 and label wording 'declared subset of Rust parse_setup_record and check_export' is ambiguous. Read as acceptance-set inclusion, it is false. It also does not match the check sets either, because r2 adds narrowings Rust lacks.\n2. The RED protocol cannot work as written. T00 asserts that the r2 module is absent, so the same test bytes cannot pass at GREEN. The find_spec discriminator is unreliable under `python3 -I` and bypasses the pinned-byte loading pattern.\n3. CP04's refusal-stage mapping is ambiguous for relation members. CP05 promises a parse_json-limits label that does not exist among the 14.\n\nThe test matrix also has unimplementable or misstaged rows, and the T16 subset demonstration is overbroad. These are fixable in a text revision without touching v1.\n\n1. R01-R03 FIX VALIDATION\nR01 (Rust-acceptance superset). Every item R01 asked to pin is now in the clauses:\n- DRIVER_ID 'mori-k2-driver/0.1.0-k2s1+k4h1', equal to lib.rs and the K2 fixture (CP02);\n- language moriarty/1, semantics moriarty-core/1, adapter SettlementHashV2, instanceVariables 3 (CP03);\n- provingKeyBytes 1..2**64-1, matching the export.rs u64 plus the ==0 refusal (CP02);\n- the nine check_profile hex digests, logicalWork as parse_u128_decimal, and the shape-bytes BLAKE2b (CP03);\n- the relationProfile, statementReencode and proofCurveValidity labels;\n- a closed 29-key relation.\n\nThe R01 '30 keys' wording is correctly fixed to 29. The UTF-8 point (old F06) is addressed by CP04. R01's residual (no decode_vk, decode_proof, setup-byte link or parse_json limits) is kept as non-claims, which is correct, but the subset wording around it now overreaches (section 3).\n\nR02 (parity overclaim). The r2 text avoids 'equivalent' and 'mirrors parse_setup_record', but it substitutes 'the whole admission ... a declared subset of Rust parse_setup_record and check_export'. That is broader than R02's own proposed phrase ('a declared subset of the non-pairing check_export consistency') and is readable as an acceptance claim. R02 is only partly honoured.\n\nR03 (untested refusal branches). T03-T15 map onto every R03 bullet for the r2 module:\n- str and hex-or-null kinds, missing fields, non-dict nested objects, nested extra keys, u8/u16/u64 upper bounds and premises variants (T07);\n- SETUP literals and leaves (T04);\n- the 8 identity keys plus the export VK digest (T08);\n- statement oversize, non-hex, tag/family/shape flips and relation bodyLen (T09);\n- x2 alone (T11);\n- uppercase proof (T12);\n- CARDANO fields (T13);\n- refusal-path no-mutation (T15).\n\nHowever, R03 was a condition on the adopted v1 component. Nothing in r2 runs these tampers against v1, so R03 for v1 stays OPEN. No r2 test exists or has run, so nothing in R03 is closed at runtime.\n\n2. 29 KEYS, KINDS, CONSTANTS, SHAPE HASH\nRelationDescriptor (relation.rs) has exactly 29 fields. The serde camelCase names match the API keys one for one:\nrelationFormat, relationVersion, language, semantics, logicalCostProfile, profileDigest, programHash, sourceSha256, action, actionIndex, relationShapeDigest, compilerSourceManifestSha256, coreClosureSha256, compilerBuildDigest, adapter, logicalWork, predicateDigest, abi2ShapeDigest, abi2ShapeBytesHex, bodyLen, statementTagHex, familyIdHex, headerLen, fieldModulusLeHex, r1csSha256, constraints, instanceVariables, witnessVariables, publicInputCount.\n\nThe K2 fixture relation has the same 29 keys.\n\nAPI kinds against the Rust types:\n- u64 for relationVersion, constraints and witnessVariables (Rust u64);\n- u16 for actionIndex;\n- int:343 for headerLen (Rust u32, check_profile ==343);\n- int:3 for instanceVariables (u64, ==3);\n- int:2 for publicInputCount (u8, ==2);\n- bodyLen 0..261801 (u32 in Rust, a declared narrowing, = 262144-343);\n- str type-only for relationFormat, logicalCostProfile, action and fieldModulusLeHex;\n- hex64 for exactly the nine check_profile digests;\n- hex64|null with the key required for sourceSha256 (a declared narrowing, since a serde Option without the key reads as None).\n\nI decoded the statementTagHex and familyIdHex literals as ASCII 'MRG16-PVC-STMT-2' (16 B) and 'MRG16-PVC-FAM2-G16-BLS-ARK04-HL2' (32 B), equal to abi2.rs STMT_TAG and FAMILY_V2.\n\nThe fixture abi2ShapeBytesHex starts with the ASCII for 'MRG16-PVC-SHP-02' (SHAPE_TAG). It ends with 0007 'partial' 0001, consistent with action 'partial' and actionIndex 1 under abi2.rs shape_bytes. I did not compute its BLAKE2b-256 against 3631...2969.\n\nThe shape bound of 524288 hex characters equals Rust's decode_bounded(..., MAX_STATEMENT_BYTES=262144). Section 3 wrongly lists it as a narrowing; it is mirrored.\n\nOne consistency rule is missing. If the pinned K2 record were refused at PROFILE during GREEN (a shape-digest inconsistency), that must be a STOP for review. It must not be fixed by editing the fixture, recomputing the digest, or moving the expected stage. T00 pins prevent silent edits, but the contract should say STOP explicitly.\n\n3. DRIVER, U64 AND UTF-8 CLOSURE\n- The driver pin is exact. driverId DECLARED_NAME_NOT_PROVENANCE is the right non-claim. Note that the supplied lib.rs docstring says 'nothing in this crate has been compiled, tested or run', yet K2/K6 artefacts exist, so fixture-to-source provenance is not established.\n- The u64 closure is complete for setup and relation. The export ints were already bounded in v1.\n- UTF-8: CP04's U+D800..U+DFFF exclusion is exactly the set of Python str values that fail strict UTF-8 encoding, so the closure is correct.\n\nThree pitfalls need contract text:\n(a) If the implementer uses str.encode('utf-8'), the exception raised is UnicodeEncodeError, a ValueError subclass. It must be rewrapped, because CP07 requires exactly ValueError with the prefix.\n(b) The CP04 scan must not allocate or run before the CP03 length-before-scan rule on abi2ShapeBytesHex. Within PROFILE the generic str scan could precede the length check, and the contract does not say that the hex scan subsumes the surrogate scan.\n(c) At EXPORT, CP04 scans statementHex before the STATEMENT-stage bound. That is linear over already-resident input, so it is acceptable, but it should be allocation-free and stated.\n\nlogicalWork needs explicit ASCII wording. Python's int() and str.isdigit() accept Unicode digits, underscores, surrounding whitespace and signs, while Rust uses is_ascii_digit. int() on more than 4300 digits raises an unprefixed ValueError under the default int_max_str_digits, so the length check must come first. T05 lacks non-ASCII-digit, underscore and whitespace cases.\n\n4. UPPER BOUNDS AND ORDERING\nThe inherited CI bounds align with PROFILE:\n- statementHex <= 524288 characters (262144 B), matching the host _MAX_STATEMENT_BYTES and claim.rs;\n- bodyLen <= 261801, so 343 + bodyLen <= 262144;\n- VK 976 and proof 384 hex characters equal decode_exact::<488> and ::<192>;\n- expiry u64 at ARG.\n\nThe stage order is identical in CP07 and the API: ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD. Moving relation faults from v1 SETUP to PROFILE is declared.\n\nTamper staging must name the record side to avoid misstaging:\n- a setup-side abi2ShapeDigest change refuses at PROFILE (the shape-bytes digest), not at RELATION;\n- a setup-side programHash, predicateDigest or compilerBuildDigest change passes PROFILE (hex only) and refuses at RELATION;\n- a setup-side bodyLen change within range refuses at STATEMENT, which T09 correctly states;\n- T08 must be export-side.\n\n5. FOURTEEN LABELS AND BYTE-EQUAL V1 MIGRATION\nThe 14 labels are identical in S01 section 4 and S02, and the 8 v1 labels are a value-identical subset. CP06's byte equality of envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex follows from building as CI10 with the same pinned host. That r2 admission is a subset of v1 admission holds by construction of the text: every v1 SETUP check is retained at SETUP or PROFILE with equal or stricter kinds, and EXPORT-CARDANO are unchanged apart from CP04.\n\nGap: CP05 says parse_json limits 'shall remain labelled non-claims', but no label covers them; rustAcceptance is the only candidate. vkCurveValidity also does not name decode_vk's non-curve profile checks (the IC prefix 03..00 at offset 336, identity refusal and canonical re-encode).\n\nV1 history stays immutable. The pins (15231 B, 23c3f99d...) match the root adoption record, and no v1 edit, rename or relabel is proposed.\n\n6. SUBSET WORDING (CRUCIAL)\nTwo readings must be separated.\n\n(i) Check-set: r2 performs a subset of the checks in parse_setup_record/check_export. This is not strictly true either. r2 adds checks Rust lacks: networkId==0, bodyLen<=261801 at parse time, and sourceSha256 key presence. It omits decode_vk, decode_proof, the setupRecordSha256 file-byte link and the parse_json limits (1 MiB, UTF-8, duplicate keys, depth 32, 65,536 nodes; input.rs).\n\n(ii) Acceptance-set: everything r2 admits, Rust admits. This is FALSE unconditionally. A setup whose 488 VK bytes are not valid compressed points but whose BLAKE2b digest is consistent, a proof with off-curve points, a mismatched setupRecordSha256, or a document beyond the parse_json limits can all be admitted by r2 and refused by Rust. Conversely, Rust admits things r2 refuses (networkId 1). The sets are incomparable.\n\nThe precise, supportable statement from source inspection is narrower. Restricted to inputs that also pass decode_vk, decode_proof, the setup file-byte link and the parse_json limits, r2-admitted implies Rust-admitted. This depends on the Python-JSON-to-serde mapping and the unread mori_core parse_limited.\n\nWith claim.rs and abi2.rs now supplied, parse_statement's re-encode is acceptance-neutral for host-admitted statements. Its offsets (112/144/172/173/201/233/265/267/275/307/339) equal the host _STATEMENT_FIELDS, and abi2::statement_bytes reconstructs the same bytes. statementReencode NOT_CHECKED is therefore accurate but contributes no acceptance gap. abi2.rs is itself labelled unreviewed and never compiled.\n\ncheck_statement_against compares exactly predicate, shape, vk and bodyLen, so it is fully mirrored by CI07. Section 3's 'beyond CI07' is vacuous. The x1/x2 exact-string comparison is acceptance-equivalent to Rust's canonical parse_u128_decimal plus to_string, so it is not a narrowing.\n\nCP05 and the rustAcceptance value 'DECLARED_SUBSET_NOT_EQUIVALENT' should be reworded to the conditional statement with the narrowings listed, so that no reader infers Rust acceptance.\n\n7. V1 ADMISSION SUBSET AND T16\nT16 ('driver, PROFILE, surrogate, the u64 bound are admitted by v1') is overbroad.\n\nPROFILE refusals that v1 admits:\n- an extra relation key;\n- a missing non-consumed key;\n- a language, semantics or adapter change;\n- a malformed profileDigest, relationShapeDigest, compilerSourceManifestSha256 or coreClosureSha256;\n- a bad logicalWork;\n- instanceVariables 4;\n- shape bytes malformed or flipped;\n- relationVersion, constraints or witnessVariables as bool or 2**64;\n- a non-str relationFormat, logicalCostProfile or fieldModulusLeHex.\n\nPROFILE refusals that v1 also refuses, at SETUP: consumed-key faults (missing programHash etc., tag or family, headerLen, bodyLen, publicInputCount, actionIndex 65536, malformed consumed digests).\n\nSurrogate cases v1 admits:\n- in relation relationFormat or logicalCostProfile;\n- in a relation key (v1 relation is open);\n- consistently in setup relation action, export action and cardano name.\n\nSurrogate cases v1 refuses: in export action alone (RELATION), intentAnchor alone (CARDANO), statementHex (STATEMENT), or any top-level key (closed objects).\n\nu64 cases v1 admits: provingKeyBytes 2**64 and relation u64 fields. v1 refuses the export u64 fields.\n\nT16 tests strictness (r2 is a proper subset of v1). It does not test r2 being a subset of v1, which T02 and T17 can only sample.\n\n8. TEST OBLIGATIONS AND RED\nUnimplementable or misstaged rows:\n- T00 asserts absence, so it fails at GREEN.\n- T14 'each adjacent pair' includes CARDANO/BUILD, and BUILD is unreachable (R04).\n- T15 'one refusal per stage' includes BUILD.\n- T15 'no input aliasing' conflicts with CI10, which returns the input setupRecordSha256 str and expiry int objects. It must be scoped to mutable containers. 'Retain' is testable only as module-state and result reachability.\n- T06 key and setup-value surrogate cases do not discriminate CP04: closed-key and fixed or hex checks refuse them anyway.\n- T17 is underspecified. The host has no statement builder, so the test must hand-edit header bytes and recompute the digest and x1/x2. Doing that with the production host mirrors the DIGEST check onto itself. Use hashlib directly, label it non-oracle, assert r2 and v1 admission plus byte equality, and keep networkId 0.\n\nT01's counting proxy is implementable, but CP01 does not restate CI01's host-reuse obligation.\n\nThe section 7 discriminator uses importlib.util.find_spec. Under the CI envelope's `python3 -I -B`, sys.path excludes the script directory, so find_spec can return None even when the file exists at GREEN. Inserting the path instead loads through import machinery rather than pinned O_NOFOLLOW bytes. Reuse v1's pattern: FileNotFoundError on an exact O_NOFOLLOW open is the only 'missing r2 implementation' source, and any other OSError means STOP.\n\nr2 also restates no run envelope and no host or fixture pins. The setup byte count (3926) is declared only by the CB API.\n\nThe meaningfulness check (a v1 copy under the r2 name fails T02, T04 and T05) is good. It needs its own reviewed scratch envelope, and it should add r2-minus-CP04 and r2-minus-shape mutants so that T06 and T05 are shown to discriminate.\n\nExpected stages must be literal table values, never derived by running r2 or v1. A differential v1 column, with literal expected outcomes (same code, SETUP instead of PROFILE, or admitted), would make T16 systematic and close R03 for v1 under a separately reviewed test.\n\n9. CB R1\nCB09 refuses unless the result has exactly the 12 CI keys 'and its format'. A source-only CB r1 implementation would therefore refuse /2 at C2B-BRIDGE. CB r1 also imports c2_issue_artifacts by name, so it never calls r2. CP08 states an obligation on another, unadopted component. It should be phrased as a non-compatibility declaration plus 'r2 output shall not be routed to CB r1'. No bridge or builder approval is implied. 'Until' must not read as automatic permission once the prerequisites exist, and the API gates omit the wallet, prover, VM and deploy flags.\n\n10. PRESERVED LIMITS\n- Current Rust moriarty/1 + moriarty-core/1 is preserved as a declared descriptor check, not a recompilation.\n- Setup is class 0, single-party TEST ONLY and forgeable.\n- Circuit/VK correspondence is OPEN.\n- Context is caller-declared and unauthenticated. networkId 0 does not separate Preview (first) from Preprod (separate, later).\n- These remain OPEN: all 20 UTxO families, financial, residual, ACTUS, DeFi, real PCD recursion, private, multiparent, authenticated-ledger and universal-certificate tracks.\n- These are review controls, not public toolchain gates. All runtime and authority flags remain false.",
  "findings": [
    {
      "id": "F01-subset-wording-acceptance-overclaim",
      "severity": "high",
      "summary": "CP05, S01 section 3 and the label rustAcceptance=DECLARED_SUBSET_NOT_EQUIVALENT call the whole admission a 'declared subset of Rust parse_setup_record and check_export'. Read as acceptance-set inclusion, that is false. Read as check-set inclusion, it is also not exact, because r2 adds narrowings Rust lacks.",
      "evidence": "S21 decode_vk/decode_proof (curve, subgroup, canonical, identity, IC prefix), S12 check_export same('setupRecordSha256') and S22 parse_json limits are not mirrored, so r2 admits inputs Rust refuses. Conversely, networkId==0, bodyLen<=261801 and the required sourceSha256 key are r2-only. S06 R02 had proposed only 'a declared subset of the non-pairing check_export consistency'.",
      "requiredFix": "Reword CP05 and section 3. r2 mirrors a declared subset of the parse_setup_record/check_export checks and adds the listed narrowings. Its acceptance set is NOT contained in Rust's. Only for inputs that also pass decode_vk, decode_proof, the setup file-byte link and the parse_json limits does r2-admitted imply Rust-admitted (source inspection only). Change the rustAcceptance value to something like MIRRORED_CHECK_SUBSET_ACCEPTANCE_NOT_CONTAINED, and record it in the API compatibility block."
    },
    {
      "id": "F02-parse-json-label-missing",
      "severity": "medium",
      "summary": "CP05 requires parse_json limits to 'remain labelled non-claims', but none of the 14 labels covers them. vkCurveValidity also does not name decode_vk's non-curve profile checks.",
      "evidence": "S01 section 4 and S02 labels: there is no JSON-limit key. S22 parse_json: 1 MiB, UTF-8, duplicate keys, depth 32, 65,536 nodes. S21 decode_vk: IC prefix at offset 336, identity refusal, canonical re-encode.",
      "requiredFix": "Either add a label (e.g. jsonParseLimits NOT_CHECKED) and update the counts in CP06, CP08, the API and T02 together, or reword CP05 to say these are documented under rustAcceptance. Either way, state that the VK profile (IC prefix, identity points, canonical encoding) is NOT_CHECKED."
    },
    {
      "id": "F03-red-discriminator-find-spec",
      "severity": "high",
      "summary": "The section 7 RED discriminator ('missing r2 implementation' only when importlib.util.find_spec returns None) is unreliable under the isolated run, and it abandons the pinned-byte loading pattern.",
      "evidence": "S04 section 7 runs python3 -I -B. In isolated mode sys.path excludes the script directory, so find_spec can return None at GREEN too. Inserting the path instead loads code through import machinery rather than the S17 pattern (_read with O_NOFOLLOW, then exec of the exact bytes). r2 restates no run envelope.",
      "requiredFix": "Specify the S17 pattern: an exact O_NOFOLLOW read of HERE/c2_issue_artifacts_r2.py. FileNotFoundError is the only source of 'missing r2 implementation'; any other OSError, a non-regular file, or a missing prepare_c2_issue is STOP. Restate the run envelope (read-only dir, limits, single run) and the host and fixture pins, including a root-recorded K2 byte count."
    },
    {
      "id": "F04-T00-phase-dependent",
      "severity": "high",
      "summary": "T00 asserts that the r2 module is absent, so identical test bytes cannot be RED-pass and GREEN-pass. The GREEN run would fail T00.",
      "evidence": "S01 section 6 T00: 'All pins match and the r2 module is absent at RED'. S01 section 7: T00 passes at RED; GREEN after implementation is required by CP08.",
      "requiredFix": "T00 should assert only fixture, host and v1 pins. Absence of r2 at RED is a root-observed precondition recorded outside the test. Root records the r2 bytes and SHA at GREEN."
    },
    {
      "id": "F05-T16-overbroad-and-subset-untested",
      "severity": "medium",
      "summary": "T16 claims that all 'driver, PROFILE, surrogate, the u64 bound' refusals are v1-admitted. Many are v1-refused. And T16 shows r2 is stricter than v1, not that r2-admitted inputs are v1-admitted.",
      "evidence": "S05 v1 _RELATION refuses consumed-key faults at SETUP; closed export and top-level objects refuse surrogate keys; CI07 refuses a surrogate statementHex; v1 EXPORT bounds export u64 fields. The CP06 subset is sampled only by T02 (K6) and T17.",
      "requiredFix": "Enumerate the T16 cases precisely. PROFILE: extra relation key, missing non-consumed key, language/semantics/adapter, the four non-consumed digests, logicalWork, instanceVariables, shape bytes, relation u64 fields and non-str type-only fields. Surrogate: relationFormat/logicalCostProfile, relation key, or consistent action. u64: provingKeyBytes and relation u64 fields. Record that the subset property rests on source review plus T02/T17 sampling, and make T17 also compare against v1."
    },
    {
      "id": "F06-CP04-stage-and-scan-ambiguity",
      "severity": "medium",
      "summary": "'The stage that admits the enclosing object' is ambiguous for relation members (SETUP admits relation as a dict; PROFILE admits its contents). The key clause is redundant in all-closed objects. The scan order conflicts with CP03's length-before-scan rule. And UnicodeEncodeError is a ValueError subclass that can escape unprefixed.",
      "evidence": "S01 CP02 'relation is an exact dict' at SETUP, versus the note 'Relation-key faults ... reported as C2I-PROFILE'. T06 '[stage of the owning object]'. CP03 length-before-scan, with intra-stage order otherwise free. S16 _wire_text uses encode() and catches the exception. S17 refused() asserts type(error) is ValueError.",
      "requiredFix": "State the mapping explicitly: setup top-level members are SETUP, relation members are PROFILE, and every export member at any depth is EXPORT. State that the hex and length checks subsume the surrogate scan for hex fields, and that no scan precedes the CP03 length check. Require an allocation-free code-point scan, or rewrap any encode exception. Drop or mark as redundant the T06 key and setup-value cases, and use relation logicalCostProfile and export action or intentAnchor as the discriminators."
    },
    {
      "id": "F07-logicalwork-ascii-and-int-limit",
      "severity": "medium",
      "summary": "'Canonical unsigned decimal' needs to be ASCII-only and length-first in Python. Otherwise Unicode digits, underscores, whitespace or very long strings are mis-admitted or raise unprefixed exceptions.",
      "evidence": "S14 parse_u128_decimal uses is_ascii_digit and len<=39. Python int() accepts non-ASCII digits, '1_0' and ' 28'. Above 4300 digits int() raises an unprefixed ValueError. S16 _wire_uint shows the correct char-range pattern. T05 lists only '', '01', '+1', 40 digits, 2**128 and int.",
      "requiredFix": "CP03 and the API should say: ASCII '0'-'9' only, 1..39 characters, checked before any int() conversion. Add T05 cases for Arabic-Indic digits, '1_0', ' 28', '28\\n' and a 5000-digit string, all expecting PROFILE."
    },
    {
      "id": "F08-R03-v1-open-and-self-mirroring",
      "severity": "medium",
      "summary": "The r2 matrix covers the R03 branch list for r2 only. R03 for the adopted v1 stays OPEN. Several expectations risk self-mirroring.",
      "evidence": "S08 R03 condition on v1. S01 section 6 runs v1 only in T02 and T16. T17 'host-built' digests would reuse the host's own DIGEST algorithm. T02 compares to v1 output rather than to independent literals.",
      "requiredFix": "Add a v1 differential column to T03-T13, with literal expected v1 outcomes (same code, SETUP instead of PROFILE, or admitted), so that R03 for v1 can be closed under a separately reviewed test. Keep T02's historical literal vectors (HIST_ENV_CBOR, the token, the 710-byte issue framing) as expectations alongside v1 equality. Compute T17 digests with hashlib and label them non-oracle."
    },
    {
      "id": "F09-unimplementable-or-misstaged-rows",
      "severity": "medium",
      "summary": "Several obligation rows are unreachable, conflict with CI10, or are misstaged unless the tamper side is specified.",
      "evidence": "T14 includes the CARDANO/BUILD pair and T15 includes BUILD, but BUILD is unreachable (S07 R04). T15 'no input aliasing' conflicts with CI10, which returns proof_export['setupRecordSha256'] and the expiry int objects. 'Retain' cannot be observed generally. A setup-side abi2ShapeDigest tamper is refused at PROFILE, not RELATION.",
      "requiredFix": "Exclude BUILD from T14 and T15. Scope aliasing to mutable containers, and define retention as no input container reachable from module globals or from the result. Name the target record for every T04-T13 tamper. Add a rule: if the pinned K2 or K6 is refused at GREEN (e.g. PROFILE), STOP for review; never edit fixtures, recompute digests or re-stage expectations."
    },
    {
      "id": "F10-section3-correspondence-stale",
      "severity": "low",
      "summary": "Section 3 and the section 1 'unread' list predate the now-supplied claim.rs, abi2.rs, encoding.rs, input.rs and host body. Several classifications are inaccurate.",
      "evidence": "S11 check_profile decode_bounded(..., MAX_STATEMENT_BYTES) equals 524288 characters, so it is mirrored, not narrowed. S20 check_statement_against equals CI07 exactly. The x1/x2 exact-string comparison is acceptance-equivalent to canonical parse plus to_string. The S20/S23 re-encode is the identity for host-admitted statements (host offsets equal claim.rs offsets).",
      "requiredFix": "Update section 3: the shape bound and x1/x2 are mirrored; check_statement_against is fully mirrored; statementReencode is NOT_CHECKED but acceptance-neutral by source inspection (abi2.rs itself unreviewed). Refresh the section 1 source list."
    },
    {
      "id": "F11-CP01-host-reuse-omitted",
      "severity": "low",
      "summary": "CP01 does not restate CI01's obligation to call the seven host_codec APIs by attribute without copying them, yet T01 tests it.",
      "evidence": "S04 CI01 last sentence; S01 CP01; T01 'counting host proxy sees all seven APIs'.",
      "requiredFix": "Append to CP01 (or the API): 'and the CI01 host-reuse rule unchanged'."
    },
    {
      "id": "F12-pins-ids-api-gates",
      "severity": "low",
      "summary": "The r2 contract does not pin its own API JSON, does not restate the host and fixture pins, cites prompt-relative source IDs (S05, S08-S11, S03 F06) that differ from the current numbering, and the API gates omit the wallet, prover, VM and deploy flags.",
      "evidence": "S01 sections 1 and 6; S03 rationales; S02 gates has only builderReliancePermitted, nativeAuthority and operatorAuthority.",
      "requiredFix": "Cite files by path and SHA, not prompt IDs. Pin C2-PROFILE-R2-API.json in the adoption record. List every flag as false in the API."
    },
    {
      "id": "F13-CP08-external-obligation-wording",
      "severity": "info",
      "summary": "CP08 imposes refusal behaviour on CB r1, an unadopted component, and its 'until' could be misread as automatic permission once the prerequisites exist.",
      "evidence": "S18 CB09 refuses unless the result has the 12 CI keys 'and its format', so it would refuse /2 by source. CB r1 imports c2_issue_artifacts (v1) by name. S19 assumedResult has 8 labels.",
      "requiredFix": "Phrase it as: r2 output is not CB-r1-compatible and shall not be routed to CB r1. builderReliancePermitted may change only by a separate root adoption decision after the listed prerequisites."
    }
  ],
  "crosschecks": [
    {
      "id": "X01-29-keys",
      "severity": "pass",
      "summary": "RelationDescriptor has 29 fields, and the API relation schema and the K2 fixture relation each have the same 29 camelCase keys.",
      "evidence": "S11 struct fields; S02 relation object; S09 relation object. The R01 '30' was a typo, correctly fixed.",
      "requiredFix": "None."
    },
    {
      "id": "X02-kinds-and-constants",
      "severity": "pass",
      "summary": "The API kinds match the Rust types and check_profile constants: the nine hex64 digests, u64/u16, headerLen 343, instanceVariables 3, publicInputCount 2, and the tag and family literals.",
      "evidence": "S11 check_profile; S23 STMT_TAG/FAMILY_V2/HEADER_LEN; I decoded the tag and family hex as ASCII by hand.",
      "requiredFix": "None beyond F07."
    },
    {
      "id": "X03-driver-and-setup-literals",
      "severity": "pass",
      "summary": "DRIVER_ID, the setup literals and the provingKeyBytes u64 bound match export.rs and lib.rs.",
      "evidence": "S13 DRIVER_ID; S12 parse_setup_record; S09 fixture driver.",
      "requiredFix": "None."
    },
    {
      "id": "X04-shape-tag-in-fixture",
      "severity": "info",
      "summary": "The fixture abi2ShapeBytesHex starts with SHAPE_TAG and ends with action 'partial' and index 1, consistent with shape_bytes. The BLAKE2b equality with abi2ShapeDigest was not computed.",
      "evidence": "S09 abi2ShapeBytesHex; S23 shape_bytes.",
      "requiredFix": "GREEN must observe it. If PROFILE refuses K2, STOP (F09)."
    },
    {
      "id": "X05-host-offsets-vs-claim",
      "severity": "pass",
      "summary": "The host _STATEMENT_FIELDS offsets and widths equal claim.rs offsets and abi2.rs ContextV2 byte order, so the re-encode is acceptance-neutral.",
      "evidence": "S16 _STATEMENT_FIELDS; S20 offsets and parse_statement; S23 statement_bytes and ContextV2::bytes.",
      "requiredFix": "Reflect in section 3 (F10)."
    },
    {
      "id": "X06-labels-14-and-v1-pins",
      "severity": "pass",
      "summary": "The 14 labels are identical in S01 and S02, the 8 v1 labels are a value-identical subset, and the v1 pin 15231 B / 23c3f99d... matches the root adoption record.",
      "evidence": "S01 section 4; S02 labels; S05 _LABELS; S08 componentSha256.",
      "requiredFix": "See F02 for the missing JSON-limit label."
    },
    {
      "id": "X07-v1-admission-subset",
      "severity": "pass",
      "summary": "By text construction, every v1 check is retained at equal or stricter strength in r2, so r2 admission is a subset of v1 admission and the four byte fields are equal when both admit.",
      "evidence": "S05 _SETUP_RECORD/_RELATION/_PROOF_EXPORT versus S01 CP02-CP06.",
      "requiredFix": "Sampling only (T02/T17). Fix T16 per F05."
    },
    {
      "id": "X08-cb-r1-refusal",
      "severity": "info",
      "summary": "The source-only CB r1 BRIDGE check on format would refuse /2. No bridge or builder approval is implied.",
      "evidence": "S18 CB09; S19 assumedResult.",
      "requiredFix": "See F13."
    },
    {
      "id": "X09-stage-order-coherence",
      "severity": "pass",
      "summary": "CP07, the API stages and CP05 give the same order, and PROFILE is inserted between SETUP and EXPORT consistently.",
      "evidence": "S01 CP05/CP07; S02 stages.",
      "requiredFix": "None."
    }
  ],
  "recommendation": {
    "verdict": "REVISE - not source-ready. DISAGREE on CP04 and CP05; material RED-protocol and test-matrix defects (F03, F04, F05, F09). Other clauses AGREE as contract text, with low-severity fixes.",
    "sourceReady": false,
    "scope": "Source-only review of the C2 consumer profile r2 contract (CP01-CP08, S01-S03) against S04-S23. No r2 test or implementation exists and nothing was run. Author FOR is not adoption.",
    "firstBlockedBoundary": "Contract text revision before any r2 test source is authored: CP05 acceptance-relation wording and the missing parse_json label, the CP04 stage mapping and scan order, a phase-independent T00, and an O_NOFOLLOW-based missing-module discriminator with the run envelope and pins restated.",
    "operatorSourceReady": false,
    "ordinarySelftestReady": false,
    "ordinarySelftestScope": "None now. After revision and a fresh non-author review: one root-observed ordinary unittest RED of the reviewed r2 test in a read-only pinned directory, then GREEN after implementation, plus a separately enveloped meaningfulness mutant run. No native, curve, prove, verify, VM, wallet or network action.",
    "authorityProposalReady": false,
    "nativeRoundtripPrerequisiteClosed": false,
    "publicRecordExportPrerequisiteClosed": false,
    "publicVerificationInputsPublishable": false
  },
  "coverage": {
    "readPaths": [
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-API.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/DESIGN-POSITIONS.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/C2-ISSUE-ARTIFACT-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/REPORT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/FINDINGS.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K2-PUBLIC-SETUP-RECORD.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K6-PUBLIC-PROOF.json",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/relation.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/export.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/lib.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/hexutil.rs",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-r2-opus-contract-20261016/PROCESS.json",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-preprod-host/host_codec.py",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/test_c2_issue_artifacts.py",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-API.json",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/claim.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/encoding.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/input.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/src/abi2.rs"
    ],
    "unreadPaths": [
      "k2/src/error.rs",
      "k2/src/setup.rs, prove.rs, verify.rs, fixed.rs, inspect.rs",
      "mori_core json::parse_limited and intent::hex",
      "mori_r1cs blake2b, identity, statement, plan, adapter modules",
      "ABI-V2.md and DESIGN.md",
      "legacy build_k10_inputs.py and test_c2_builder.py",
      "claim.ak, types.ak and the C2 wrapper",
      "root RED/GREEN RESULT.json files and the S12 terminal predecessor output",
      "the original author prompts behind the S-ID cross-references in S01/S03"
    ],
    "limitations": "Text-only reading of 23 inline bodies. Nothing was executed, imported, compiled or hashed. All byte counts and SHA-256 values are as declared. BLAKE2b-256 of the fixture shape bytes and the VK was not recomputed. Serde Option-missing behaviour, serde_json surrogate refusal and mori_core parse_limited semantics are inferred from standard behaviour and declared docs, not read. Python int() and isdigit behaviour and -I sys.path behaviour are stated from language knowledge, not observed. No tests exist, so R03 runtime closure is impossible. Same-provider fresh context is not provider diversity."
  },
  "positions": [
    {
      "clauseId": "CP01",
      "clause": "The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec and whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, applying the CI02 ARG checks unchanged; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format, eight labels and test history stay immutable.",
      "position": "AGREE",
      "rationale": "A separate module with the v1 pins matching S08 keeps v1 history immutable, and CI02 is reused unchanged. Low fix (F11): restate CI01's host-reuse rule, because T01 tests an obligation that CP01 does not state."
    },
    {
      "clauseId": "CP02",
      "clause": "When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is exactly 976 lowercase hex; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256.",
      "position": "AGREE",
      "rationale": "Field for field this matches S12 parse_setup_record (12 fields, fixed literals, DRIVER_ID from S13, decode_exact::<488>, a u64 proving_key_bytes != 0) except decode_vk, which is correctly left to labelled non-claims. The driver remains a declared name, not provenance."
    },
    {
      "clauseId": "CP03",
      "clause": "When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys of relation.rs with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests as 64 lowercase hex and sourceSha256 as 64 lowercase hex or null; logicalWork as a canonical unsigned decimal of at most 39 digits and value at most 2**128-1; the ABI v2 statementTagHex and familyIdHex literals; headerLen 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex as even lowercase hex of at most 524288 characters, length-checked before scanning, whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.",
      "position": "AGREE",
      "rationale": "Verified against S11: 29 fields, the nine hex digests, check_profile constants, parse_u128_decimal, and a shape bound equal to decode_bounded(MAX_STATEMENT_BYTES). This preserves the moriarty/1 and moriarty-core/1 requirement as a declared descriptor check, without recompilation. Required clarifications (F07, F06, F12): ASCII-only digits with the length checked before int(), the interplay with the CP04 scan, and pinning the referenced API JSON."
    },
    {
      "clauseId": "CP04",
      "clause": "While admitting either record, the consumer shall accept an integer only as an exact int (never bool) within its declared range and a key or string value only as an exact str containing no code point in U+D800..U+DFFF, and shall otherwise refuse with the code of the stage that admits the enclosing object.",
      "position": "DISAGREE",
      "rationale": "The rule is right in substance: it closes the old F06 lone-surrogate point and keeps the Rust integer widths. But 'the stage that admits the enclosing object' is ambiguous for relation members (SETUP admits relation as a dict, PROFILE admits its contents). The key clause is redundant in all-closed objects. Scan order versus CP03's length-before-scan is unspecified. An encode-based check can leak UnicodeEncodeError. Fix per F06 with an explicit SETUP/PROFILE/EXPORT mapping."
    },
    {
      "clauseId": "CP05",
      "clause": "When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe the whole admission only as a declared subset of Rust parse_setup_record and check_export, never as equivalence; statement re-encode, VK and proof curve or subgroup validity, the setupRecordSha256 file-byte link, parse_json limits and the Groth16 pairing shall remain labelled non-claims.",
      "position": "DISAGREE",
      "rationale": "'Declared subset of Rust parse_setup_record and check_export' can be read as acceptance-set inclusion. That is false: decode_vk/decode_proof, the setup byte link and parse_json limits are unmirrored (S21, S12, S22), and the r2 narrowings are not Rust checks. Also, parse_json limits are promised a label that does not exist among the 14. Reword to a conditional statement (F01) and add or redirect the label (F02)."
    },
    {
      "clauseId": "CP06",
      "clause": "When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 14-entry r2 label map; every r2-admitted input shall also be v1-admitted, and for such input envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex shall equal the v1 values.",
      "position": "AGREE",
      "rationale": "By text construction r2 keeps every v1 check at equal or stricter strength, so the subset and byte-equality properties hold, and the format and label change prevents silent confusion with v1. The universal property can only be sampled (T02, T17) plus source review. T16 must be narrowed as described in F05."
    },
    {
      "clauseId": "CP07",
      "clause": "If any check fails, then the consumer shall raise exactly ValueError whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD, shall return no partial result, shall never mutate or retain either input on success or refusal, and shall perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.",
      "position": "AGREE",
      "rationale": "Consistent with CI11 and the API stage list, and it adds refusal-path no-mutation (R03). Test caveats (F09): BUILD is unreachable; 'retain' is observable only as module-state and result reachability; immutable leaf sharing (the setupRecordSha256 str, the expiry int) must not count as aliasing. CI11's host-rewrap and environmental-propagation rules should be restated."
    },
    {
      "clauseId": "CP08",
      "clause": "The r2 records shall keep builderReliancePermitted false until a new r2 implementation, an independent non-author audit, root-observed meaningful RED then GREEN of ordinary tests covering obligations T00-T17, and a new downstream CB contract that explicitly accepts format moriarty-c2-issue-artifacts/2 with its 14 labels all exist, and shall keep every native, operator, wallet, prover, VM and deploy flag false regardless; a CB r1 consumer expecting format /1 or eight labels shall refuse r2 output and shall not treat it as compatible.",
      "position": "AGREE",
      "rationale": "It keeps builder reliance false and all authority at zero, and makes CB r1 incompatibility explicit without approving any bridge. S18 CB09's format check would refuse /2 by source. Wording fixes (F13): the 'until' must not read as automatic permission, and the obligation should be phrased as non-routing rather than imposed on an unadopted CB r1."
    }
  ]
}


## S06 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/ROOT-QUALIFICATION.json
{
  "format": "moriarty-cp-policy-opus-review-root-intake/1",
  "observedAt": "2026-10-16T22:41:47.374569+00:00",
  "actualCanonicalModel": "claude-opus-5-5",
  "terminalExit": 0,
  "schemaValid": true,
  "sourcePinsChecked": 23,
  "allReadPathsAccounted": true,
  "literalClausesMatched": 8,
  "disagree": [
    "CP04",
    "CP05"
  ],
  "otherAgreeWithClarifications": [
    "CP01",
    "CP02",
    "CP03",
    "CP06",
    "CP07",
    "CP08"
  ],
  "sourceAdopted": false,
  "reportSha256": "eaa31ec5d9404c0a7cfc87c12a84f3a19459d7e0883d6a2ad7cc400f17eee8df",
  "firstBlockedBoundary": "Contractrevision beforetests/code: F01acceptance overclaim/F03find_spec RED/F04phase dependentT00/F06mapping, all13findings retained.",
  "qualification": "No runtime/Rustadmission proof. ReviewerF01 gives source-derived counterexamples (invalidcurveVK/proof/byte-link/parselimits) not executednativecases. Change approach: remove broadRustsetinclusion language entirely, precise selectedchecks/nonclaims. Existingv1R03runtime remainsopen.",
  "nativeAuthority": 0
}


## S07 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-CONTRACT.md
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


## S08 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-API.json
{
"format": "moriarty-c2-profile-r2-api/1",
"status": "AUTHOR-PROPOSED contract only; no implementation or test; nothing run",
"module": "c2_issue_artifacts_r2.py",
"imports": ["hashlib", "host_codec"],
"function": "prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id)",
"stages": ["ARG", "SETUP", "PROFILE", "EXPORT", "RELATION", "STATEMENT", "CONTEXT", "DIGEST", "PROOF", "CARDANO", "BUILD"],
"refusal": "ValueError; message prefix C2I-<STAGE>: of the first failing stage",
"intRule": "exact int, never bool, within kind range",
"strRule": "exact str key or value without U+D800..U+DFFF",
"setup": {
"keys": "exactly the 12 SetupRecord keys",
"fixed": "format, trustProfile, notice, curve, snark, encoding = export.rs constants",
"driver": "fixed:mori-k2-driver/0.1.0-k2s1+k4h1",
"provingKeySha256": "hex64",
"provingKeyBytes": "int:1..18446744073709551615",
"relation": "dict",
"verifyingKeyHex": "hex976",
"vkDigestBlake2b256": "hex64 = BLAKE2b-256(VK bytes)"
},
"relation": {
"relationFormat": "str",
"relationVersion": "u64",
"language": "fixed:moriarty/1",
"semantics": "fixed:moriarty-core/1",
"logicalCostProfile": "str",
"profileDigest": "hex64",
"programHash": "hex64",
"sourceSha256": "hex64|null, key required",
"action": "str",
"actionIndex": "u16",
"relationShapeDigest": "hex64",
"compilerSourceManifestSha256": "hex64",
"coreClosureSha256": "hex64",
"compilerBuildDigest": "hex64",
"adapter": "fixed:SettlementHashV2",
"logicalWork": "canonical decimal u128",
"predicateDigest": "hex64",
"abi2ShapeDigest": "hex64",
"abi2ShapeBytesHex": "even lowercase hex <= 524288 chars; BLAKE2b-256 = abi2ShapeDigest",
"bodyLen": "int:0..261801",
"statementTagHex": "fixed:4d524731362d5056432d53544d542d32",
"familyIdHex": "fixed:4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32",
"headerLen": "int:343",
"fieldModulusLeHex": "str",
"r1csSha256": "hex64",
"constraints": "u64",
"instanceVariables": "int:3",
"witnessVariables": "u64",
"publicInputCount": "int:2"
},
"export": "CI05-CI09 unchanged, narrowed by strRule",
"result": "the 12 CI10 keys; format moriarty-c2-issue-artifacts/2",
"labels": {
"expiry": "CALLER_SUPPLIED_NOT_PROOF_BOUND",
"context": "CALLER_DECLARED_UNAUTHENTICATED",
"policyId": "CALLER_SUPPLIED_UNAUTHENTICATED",
"trustProfile": "single-party-test-setup/1",
"setupRecordLink": "DECLARED_NOT_BYTE_CHECKED",
"groth16": "NOT_VERIFIED",
"vkCurveValidity": "NOT_CHECKED",
"proofCurveValidity": "NOT_CHECKED",
"statementReencode": "NOT_CHECKED",
"relationProfile": "DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED",
"driverId": "DECLARED_NAME_NOT_PROVENANCE",
"rustAcceptance": "DECLARED_SUBSET_NOT_EQUIVALENT",
"circuitVkCorrespondence": "OPEN",
"ledger": "NO_CLAIM"
},
"v1": {"module": "c2_issue_artifacts.py", "bytes": 15231, "sha256": "23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76", "format": "moriarty-c2-issue-artifacts/1", "labels": 8, "status": "immutable; unverified structural serializer only"},
"compatibility": {"admission": "r2 subset of v1", "byteEqualWhenBothAdmit": ["envCborHex", "envelopeTokenNameHex", "issueCborHex", "mintValueCborHex"], "cbR1": "must refuse r2; new CB contract required"},
"gates": {"builderReliancePermitted": false, "nativeAuthority": 0, "operatorAuthority": 0}
}


## S09 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/DESIGN-POSITIONS.json
[
  {
    "clauseId": "CP01",
    "clause": "The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec and whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, applying the CI02 ARG checks unchanged; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format, eight labels and test history stay immutable.",
    "position": "FOR",
    "rationale": "Keeps the immutable v1 history (S05 componentSha256, S01 CI01/CI02) and gives r2 a separate identity, so admission changes cannot silently alter what was adopted."
  },
  {
    "clauseId": "CP02",
    "clause": "When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is exactly 976 lowercase hex; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256.",
    "position": "FOR",
    "rationale": "Closes the R01 driver and u64 gaps against S09 parse_setup_record and S10 DRIVER_ID. The driver remains a declared name, not provenance."
  },
  {
    "clauseId": "CP03",
    "clause": "When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys of relation.rs with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests as 64 lowercase hex and sourceSha256 as 64 lowercase hex or null; logicalWork as a canonical unsigned decimal of at most 39 digits and value at most 2**128-1; the ABI v2 statementTagHex and familyIdHex literals; headerLen 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex as even lowercase hex of at most 524288 characters, length-checked before scanning, whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.",
    "position": "FOR",
    "rationale": "Mirrors S08 deny_unknown_fields and check_profile, plus S11 parse_u128_decimal, on the actual 29 fields (not 30). This preserves the current moriarty/1 and moriarty-core/1 language requirement without recompiling."
  },
  {
    "clauseId": "CP04",
    "clause": "While admitting either record, the consumer shall accept an integer only as an exact int (never bool) within its declared range and a key or string value only as an exact str containing no code point in U+D800..U+DFFF, and shall otherwise refuse with the code of the stage that admits the enclosing object.",
    "position": "FOR",
    "rationale": "Closes the S03 F06 lone-surrogate superset point and keeps the Rust integer widths (S08, S09) without bool confusion."
  },
  {
    "clauseId": "CP05",
    "clause": "When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe the whole admission only as a declared subset of Rust parse_setup_record and check_export, never as equivalence; statement re-encode, VK and proof curve or subgroup validity, the setupRecordSha256 file-byte link, parse_json limits and the Groth16 pairing shall remain labelled non-claims.",
    "position": "FOR",
    "rationale": "Applies R02. encoding.rs, claim.rs and input.rs are unread, so those checks are explicit non-claims rather than asserted equivalence with S09."
  },
  {
    "clauseId": "CP06",
    "clause": "When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 14-entry r2 label map; every r2-admitted input shall also be v1-admitted, and for such input envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex shall equal the v1 values.",
    "position": "FOR",
    "rationale": "A distinct format and labels prevent silent confusion with v1. The subset property and byte-equal outputs make the migration checkable."
  },
  {
    "clauseId": "CP07",
    "clause": "If any check fails, then the consumer shall raise exactly ValueError whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD, shall return no partial result, shall never mutate or retain either input on success or refusal, and shall perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.",
    "position": "FOR",
    "rationale": "Extends the CI11 discipline with the new PROFILE stage and makes no-mutation on refusal paths an explicit obligation (R03)."
  },
  {
    "clauseId": "CP08",
    "clause": "The r2 records shall keep builderReliancePermitted false until a new r2 implementation, an independent non-author audit, root-observed meaningful RED then GREEN of ordinary tests covering obligations T00-T17, and a new downstream CB contract that explicitly accepts format moriarty-c2-issue-artifacts/2 with its 14 labels all exist, and shall keep every native, operator, wallet, prover, VM and deploy flag false regardless; a CB r1 consumer expecting format /1 or eight labels shall refuse r2 output and shall not treat it as compatible.",
    "position": "FOR",
    "rationale": "Preserves S05 builderReliancePermitted false and zero authority, and makes the CB r1 incompatibility explicit rather than silent."
  }
]


## S10 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/C2-ISSUE-ARTIFACT-CONTRACT.md
# C2 issue-artifact consumer contract (CI01-CI12)

Status: AUTHOR-PROPOSED source contract plus ordinary test only. Author `claude-opus-5-5` HIGH, tools off: nothing was executed, imported, hashed or reviewed; no implementation body is supplied; RED and GREEN are unobserved; no reviewer has approved this contract. Native, operator, wallet, prover, VM, deploy, key, signing and network authority all remain 0. Author-time source work, not adoption. Independent of, and not touching, any controller/operator patch.

## 1. Gap and scope

The K10 R2 builder `build_k10_inputs.py` (24480 B, SHA-256 `6c1cde7f971a4c3ade25a3089f22f557fe2290b3b1266c4c18b6b4b7ce918d00`) reads an Aiken fixture and hard-codes claimant `55`x28, settle outref `22`x32#0 and `TEST_EXPIRY` 1893456000000 (lines 89-96, 285-294). The archived operator consumes only its UPLC context text. The new pure sibling module `c2_issue_artifacts.py` derives EnvV2 Data/CBOR, `C2Mint::Issue` Data/CBOR, the envelope token name and the mint Value Data from a genuine Rust `moriarty-k2-groth16-proof/1` export and its public `moriarty-k2-groth16-setup/1` record. Both arrive already parsed to JSON-compatible dicts, together with a caller-explicit expiry and policy id. A production byte parser, file I/O and CLI remain a separate open gap. No campaign, runner or framework is created.

## 2. Source grounding (identifiers only)

| Source | Use |
|---|---|
| `k2/src/export.rs` | SetupRecord, ProofExport, ContextExport, EnvV2Export (no expiry), cardano_export, check_export order, PREMISES, SETUP_FORMAT, TRUST_PROFILE, SETUP_NOTICE, CURVE, SNARK, ENCODING |
| `host_codec.py` 23252 B `3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459` | the seven reused APIs; unchanged |
| `claim.ak` 24-71, `types.ak` 227-243, `c2` wrapper | envelope_name, well_formed, issue conjuncts, EnvV2, C2Mint |
| builder excerpts 89-96, 285-363, 375-394, 437-502 | fixed identities, legacy env/context, callsites |
| EXPECTATIONS 43-71 | Env CBOR segments, >64-byte chunk rule, V3 zero-ada omission |
| manifest 32-42, 104-135; context 09; receipt 11 | historical Env CBOR, token, policy, ACCEPT_UNIT (reproduction only) |
| STATEMENT-HOST-CONTRACT.md sections 3, 7 | header offsets; K6 fixture pin |

Unread and not invented: relation.rs (RelationDescriptor, check_profile), claim.rs check_statement_against, encoding.rs, hexutil.rs, input.rs parse_json, DRIVER_ID, builder helpers cbor_encode/uplc_term/read_pinned/sha256_hex, operator remainder, and the exact bytes of both JSON fixtures.

## 3. Clauses (EARS; author position FOR on all)

- **CI01** The consumer shall be the pure sibling module `c2_issue_artifacts.py`. Its only import statements shall be `import hashlib` and `import host_codec`, and its only public function shall be `prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id)`, with no defaults. It shall call `build_env_data`, `build_c2_mint_data`, `build_value_data`, `encode_data`, `parse_statement_header`, `statement_digest_hex` and `statement_public_inputs` as attributes of the unchanged pinned `host_codec`, never copying or re-implementing them.
- **CI02** When `prepare_c2_issue` is called, the consumer shall, before touching any record key or value, refuse with `C2I-ARG` unless `type(proof_export) is dict`, `type(setup_record) is dict`, `type(expiry) is int` with 0 <= expiry <= 2**64-1, and `type(policy_id) is bytes` of length 28.
- **CI03** The consumer shall take EnvV2 `expiry` only from the `expiry` argument, never from a record, default, clock or the K6 constant. It shall treat any `expiry` key inside the export as a closed-schema violation (`C2I-EXPORT`) and shall label the value `CALLER_SUPPLIED_NOT_PROOF_BOUND`.
- **CI04** When CI02 passes, the consumer shall refuse with `C2I-SETUP` unless all of the following hold:
  - the setup record has exactly the 12 SetupRecord keys;
  - format, trustProfile, notice, curve, snark and encoding equal the export.rs constants;
  - driver is a str;
  - provingKeySha256 is 64 lowercase hex and provingKeyBytes is a positive int;
  - relation is a dict carrying programHash, compilerBuildDigest, r1csSha256, predicateDigest and abi2ShapeDigest (64 lowercase hex), sourceSha256 (64 lowercase hex or null), action (str) and actionIndex (u16);
  - relation statementTagHex and familyIdHex equal the ABI v2 literals, headerLen is 343, bodyLen is 0..261801 and publicInputCount is 2;
  - verifyingKeyHex is exactly 976 lowercase hex;
  - unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256.
- **CI05** When CI04 passes, the consumer shall refuse with `C2I-EXPORT` unless all of the following hold:
  - the export and its context, publicInputs, cardano, envV2, settleOutref and catalogueActionEntry objects are exact dicts with exactly the export.rs field names;
  - integers are exact int (never bool) within their declared u8/u16/u64 ranges;
  - fixed-width digests are lowercase hex of exact width, and sourceSha256 may be null;
  - statementHex, proofHex, x1, x2, action, name, intentTargetHashV2 and intentAnchor are str;
  - format equals `moriarty-k2-groth16-proof/1`;
  - premises is an exact list equal to the six PREMISES.
- **CI06** When CI05 passes, the consumer shall refuse with `C2I-RELATION` unless programHash, sourceSha256, action, actionIndex, compilerBuildDigest, r1csSha256, predicateDigest and abi2ShapeDigest equal the setup relation values, and vkDigestBlake2b256 equals the CI04 recomputed VK digest.
- **CI07** When CI06 passes, the consumer shall refuse with `C2I-STATEMENT` unless:
  - statementHex is even-length lowercase hex of at most 524288 characters;
  - `host_codec.parse_statement_header` admits the decoded bytes;
  - header predicateDigest, shapeDigest, vkDigest and bodyLen equal relation predicateDigest, relation abi2ShapeDigest, the recomputed VK digest and relation bodyLen.
- **CI08** When CI07 passes, the consumer shall refuse with `C2I-CONTEXT` unless the export context equals the ten parsed header context values exactly, and header networkId is 0 (testnet only; this does not distinguish Preview from Preprod).
- **CI09** When CI08 passes, the consumer shall apply three checks in order:
  - refuse with `C2I-DIGEST` unless `statement_digest_hex` equals statementDigestBlake2b256 and `statement_public_inputs` equals (x1, x2) as exact strings;
  - then refuse with `C2I-PROOF` unless proofHex is exactly 384 lowercase hex;
  - then refuse with `C2I-CARDANO` unless the cardano section equals the cardano_export derivation from the relation, the recomputed VK digest, the parsed header and the statement digest.
- **CI10** When CI09 passes, the consumer shall return the section 4 result, built as follows:
  - envData = `build_env_data(relation predicate, recomputed VK digest, header claimantVkh, header consumedTxid, header consumedIx, host statement digest, expiry)`;
  - envelopeTokenNameHex = unkeyed BLAKE2b-256 of `encode_data(envData)`;
  - issueData = `build_c2_mint_data('Issue', vk488, proof192)`;
  - mintValueData = `build_value_data(None, [(policy_id, token, 1)])`.
- **CI11** If any check fails, then the consumer shall raise exactly `ValueError` whose message starts with `<code>:` for the first failing stage, in the order ARG, SETUP, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO (BUILD if a host factory unexpectedly refuses). It shall rewrap host ValueErrors with the stage code and return no partial result. It shall never mutate either input, and shall perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.
- **CI12** The consumer shall claim nothing about Groth16 validity, VK or proof curve validity, setup-record byte identity, ledger or provider authenticity, Preview/Preprod acceptance, UPLC or legacy-builder compatibility, or shared-program certification. K6 reproduction is a historical reproduction vector only, and every native, operator, wallet, prover, VM and deploy flag stays false.

Intra-stage check order is not frozen. No check of stage k runs before stage k-1 completes.

## 4. Returned artifact schema (CI10)

The result is an exact `dict` with exactly these 12 keys, and all objects are fresh on every call.

| Key | Type / bound |
|---|---|
| format | `moriarty-c2-issue-artifacts/1` |
| envData | host Data `('constr', 0, [...9 fields])` |
| envCborHex | lowercase hex, 212-222 bytes (220 for K6 at expiry 1893456000000) |
| envelopeTokenNameHex | 64 lowercase hex |
| issueData | `('constr', 0, [('bytes', vk488), ('bytes', proof192)])` |
| issueCborHex | exactly 710 bytes: VK as 7x64+40 chunks, proof as 3x64 chunks |
| mintValueData | `build_value_data(None, [(policy_id, token, 1)])` |
| mintValueCborHex | exactly 67 bytes: `a1 581c policy a1 5820 token 01` |
| policyIdHex | 56 lowercase hex |
| expiry | exact int echo |
| setupRecordSha256Declared | export value copied, NOT byte-checked |
| labels | exactly: expiry CALLER_SUPPLIED_NOT_PROOF_BOUND; context CALLER_DECLARED_UNAUTHENTICATED; policyId CALLER_SUPPLIED_UNAUTHENTICATED; trustProfile single-party-test-setup/1; setupRecordLink DECLARED_NOT_BYTE_CHECKED; groth16 NOT_VERIFIED; vkCurveValidity NOT_CHECKED; ledger NO_CLAIM |

## 5. Implementation split and routing

- **Cryptographic calculation and consistency (Opus, next step):** the whole consumer module, covering CI02-CI11.
- **Mechanical glue (Sol HIGH, after this contract is frozen):** the new builder revision of C2-CONSUMER-INTEGRATION.json, reviewed by a fresh Meta Muse high. It covers argv, pinned bounded I/O, strict JSON, the file-byte check `sha256(setup bytes) == export.setupRecordSha256`, and manifest fields.
- **Combined review:** a fresh non-author Opus reviews the actual consumer plus host and the actual results. Same-provider independence is context only.

## 6. Dispositions and non-claims

- **F01:** the mint Value uses lovelace=None (canonical no-ada, matching the V3 zero-ada omission in historical context 09). lovelace=0 is never used. No ledger Value, min-ADA or output-value correspondence is claimed.
- **F02:** Issue is narrowed to VK 488 / proof 192 by length only. Curve, subgroup and pairing validity are unchecked; the on-chain groth16.verify_digest remains the only pairing check.
- **Historical vectors:** the manifest Env CBOR, token `9fd5c53f...` and the captured ACCEPT_UNIT are reproduction vectors of the historical builder and run, not an independently computed oracle. The K6 context (55/22/a2, seq 1) is fabricated and historical, not funded Preview/T1 state.
- **Demonstration levels:** K6 reproduction is distinct from a second real current-context proof (needs separate prover authority), which is in turn distinct from on-chain acceptance (Preview first, Preprod separately later).
- **Legacy boundary:** host lowercase tagged tuples are NOT assumed compatible with the legacy uppercase Constr/B/I/Map/List objects or the UPLC text that the operator consumes. A later finite reviewed bridge is required (see the integration JSON).
- **Mandatory limits:** setup is single-party, TEST ONLY and forgeable. VK-to-matrix, compiler and hash-gadget correspondence is OPEN. No shared-program/property certification and no universal, recursive, private or multi-parent PCD is established. Provider context is unauthenticated. Full20 UTxO, ACTUS and DeFi obligations are preserved.

## 7. Dedicated directory, pins and run envelope

The directory is read-only (directory mode 555, file modes 444) and holds exactly:

- `test_c2_issue_artifacts.py`;
- `host_codec.py`: 23252 B, SHA-256 `3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459`;
- `K6-PUBLIC-PROOF.json`: 5800 B, SHA-256 `930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b`. Root confirms equality with supplied source 04;
- `K2-PUBLIC-SETUP-RECORD.json`: SHA-256 `86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b`, the declared export link. The byte count is unread; on mismatch, stop;
- `c2_issue_artifacts.py`: absent at RED.

No file is generated at test time, and paths inside fixture metadata are never followed. Root inspects all sources and effects first, then runs `python3 -I -B test_c2_issue_artifacts.py` once, with no retry, under these limits: timeout 10 s, CPU 10 s, address space 512 MiB, fsize 2 MiB, core 0, minimal secret-free environment, no network, before/after directory pins, total budget <= 1200 s.

## 8. RED expectation (unobserved)

`test_01` passes. `test_02` to `test_11` fail, each with assertion text containing `missing implementation`. Any setUpModule error, pin mismatch, ImportError or other outcome stops the step for review.


## S11 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py
"""Pure C2 Issue artifacts from a parsed Rust proof export and its setup record.

Author-proposed implementation of C2-ISSUE-ARTIFACT-CONTRACT.md CI01-CI12; not
reviewed or adopted. The consumer checks the non-pairing consistency of an
already parsed moriarty-k2-groth16-proof/1 export against its public
moriarty-k2-groth16-setup/1 record, then derives EnvV2 Data/CBOR, the envelope
token name, C2Mint Issue Data/CBOR and the mint Value Data through the pinned
host_codec. Groth16 validity, VK or proof curve validity, setup-record byte
identity, ledger or provider authenticity, Preview/Preprod acceptance and
legacy-builder or UPLC compatibility are not claimed. The expiry and the policy
id are caller supplied and not bound by the proof.
"""
import hashlib
import host_codec

_ARTIFACT_FORMAT = "moriarty-c2-issue-artifacts/1"

# export.rs profile constants and fixed premises, as public literals.
_PROOF_FORMAT = "moriarty-k2-groth16-proof/1"
_SETUP_FORMAT = "moriarty-k2-groth16-setup/1"
_TRUST_PROFILE = "single-party-test-setup/1"
_SETUP_NOTICE = (
    "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. "
    "Whoever controlled the setup process could forge proofs for this relation. "
    "VK provenance is declared by this record; no verifier can check that the VK "
    "was generated from r1csSha256.")
_CURVE = "BLS12-381"
_SNARK = "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)"
_ENCODING = ("arkworks 0.4 compressed points (Zcash BLS12-381 layout); "
             "VK 488 bytes, proof 192 bytes")
_PREMISES = (
    "The proof shows, under setup honesty and Groth16 knowledge soundness, that the "
    "statement's constant header and body satisfy the named compiled moriarty-core/1 "
    "relation.",
    "Context bytes (vk digest, claimant, network, settle hash, thread, consumed outref, "
    "seq, catalogue digest, intents digest) are caller-declared premises: bound by the "
    "proof, not authenticated by it. S2 recomputes each from its authenticated "
    "ScriptContext and datum.",
    "Trust profile single-party-test-setup/1: whoever ran setup could forge proofs for "
    "this relation.",
    "Groth16 proofs are malleable: the proof bytes are not a unique claim identifier.",
    "Signed intents, debit authority, frame completeness, physical asset delta, replay "
    "and phase outcomes are target obligations outside this proof.",
    "No ledger admission, deployment, settlement, Preprod or Midnight result is implied.",
)

# ABI v2 statement tag and family id as the relation descriptor records them.
_STMT_TAG_HEX = "4d524731362d5056432d53544d542d32"
_FAMILY_HEX = "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32"
_HEADER_LEN = 343
_MAX_STATEMENT_BYTES = 262144
_MAX_UINT64 = (1 << 64) - 1
_HEX_DIGITS = "0123456789abcdef"
_VK_HEX_LEN = 976
_PROOF_HEX_LEN = 384
_POLICY_LEN = 28

_LABELS = {
    "expiry": "CALLER_SUPPLIED_NOT_PROOF_BOUND",
    "context": "CALLER_DECLARED_UNAUTHENTICATED",
    "policyId": "CALLER_SUPPLIED_UNAUTHENTICATED",
    "trustProfile": "single-party-test-setup/1",
    "setupRecordLink": "DECLARED_NOT_BYTE_CHECKED",
    "groth16": "NOT_VERIFIED",
    "vkCurveValidity": "NOT_CHECKED",
    "ledger": "NO_CLAIM",
}

# Schema kinds: ("object", fields, closed), ("hex", chars), ("hex-or-null", chars),
# ("int", minimum, maximum or None), ("str",), ("fixed", text), ("premises",).
_HEX32 = ("hex", 64)
_HEX28 = ("hex", 56)
_TEXT = ("str",)
_U8 = ("int", 0, 255)
_U16 = ("int", 0, 65535)
_U64 = ("int", 0, _MAX_UINT64)

# The ten ContextExport fields, named as parse_statement_header returns them.
_CONTEXT = {
    "vkDigest": _HEX32,
    "claimantVkh": _HEX28,
    "networkId": _U8,
    "settleHash": _HEX28,
    "threadId": _HEX32,
    "consumedTxid": _HEX32,
    "consumedIx": _U16,
    "seq": _U64,
    "catalogueDigest": _HEX32,
    "intentsDigest": _HEX32,
}
_PUBLIC_INPUTS = {"x1": _TEXT, "x2": _TEXT}
_OUTREF = {"transactionId": _HEX32, "outputIndex": _U16}
# EnvV2Export has no expiry: an injected one fails the closed schema (CI03).
_ENV_V2 = {
    "version": _U8,
    "claimKind": _U8,
    "familyId": _HEX32,
    "predicateDigest": _HEX32,
    "vkDigest": _HEX32,
    "claimantVkh": _HEX28,
    "settleOutref": ("object", _OUTREF, True),
    "statementDigest": _HEX32,
}
_ACTION_ENTRY = {
    "name": _TEXT,
    "index": _U16,
    "predicateDigest": _HEX32,
    "vkDigest": _HEX32,
    "shapeDigest": _HEX32,
}
_CARDANO = {
    "envV2": ("object", _ENV_V2, True),
    "catalogueActionEntry": ("object", _ACTION_ENTRY, True),
    "intentTargetHashV2": _TEXT,
    "intentAnchor": _TEXT,
}
_PROOF_EXPORT = ("object", {
    "format": ("fixed", _PROOF_FORMAT),
    "setupRecordSha256": _HEX32,
    "programHash": _HEX32,
    "sourceSha256": ("hex-or-null", 64),
    "action": _TEXT,
    "actionIndex": _U16,
    "compilerBuildDigest": _HEX32,
    "r1csSha256": _HEX32,
    "predicateDigest": _HEX32,
    "abi2ShapeDigest": _HEX32,
    "vkDigestBlake2b256": _HEX32,
    "context": ("object", _CONTEXT, True),
    "statementHex": _TEXT,
    "statementDigestBlake2b256": _HEX32,
    "publicInputs": ("object", _PUBLIC_INPUTS, True),
    "proofHex": _TEXT,
    "cardano": ("object", _CARDANO, True),
    "premises": ("premises",),
}, True)

# Only the relation keys this consumer reads; other descriptor keys stay open.
_RELATION = {
    "programHash": _HEX32,
    "sourceSha256": ("hex-or-null", 64),
    "action": _TEXT,
    "actionIndex": _U16,
    "compilerBuildDigest": _HEX32,
    "r1csSha256": _HEX32,
    "predicateDigest": _HEX32,
    "abi2ShapeDigest": _HEX32,
    "statementTagHex": ("fixed", _STMT_TAG_HEX),
    "familyIdHex": ("fixed", _FAMILY_HEX),
    "headerLen": ("int", _HEADER_LEN, _HEADER_LEN),
    "bodyLen": ("int", 0, _MAX_STATEMENT_BYTES - _HEADER_LEN),
    "publicInputCount": ("int", 2, 2),
}
_SETUP_RECORD = ("object", {
    "format": ("fixed", _SETUP_FORMAT),
    "trustProfile": ("fixed", _TRUST_PROFILE),
    "notice": ("fixed", _SETUP_NOTICE),
    "relation": ("object", _RELATION, False),
    "curve": ("fixed", _CURVE),
    "snark": ("fixed", _SNARK),
    "encoding": ("fixed", _ENCODING),
    "verifyingKeyHex": ("hex", _VK_HEX_LEN),
    "vkDigestBlake2b256": _HEX32,
    "provingKeySha256": _HEX32,
    "provingKeyBytes": ("int", 1, None),
    "driver": _TEXT,
}, True)

# check_export identity comparisons between the export and the setup relation.
_RELATION_IDENTITY = ("programHash", "sourceSha256", "action", "actionIndex",
                      "compilerBuildDigest", "r1csSha256", "predicateDigest",
                      "abi2ShapeDigest")


def _refuse(code, detail):
    raise ValueError(code + ": " + detail)


def _host(code, api, *args):
    # Host refusals carry the stage code; other exceptions propagate unconverted.
    try:
        return api(*args)
    except ValueError as error:
        raise ValueError(code + ": host refused: " + str(error)) from None


def _is_hex(value, chars):
    return (type(value) is str and len(value) == chars
            and set(value).issubset(_HEX_DIGITS))


def _admit_object(value, fields, closed, code, path):
    if type(value) is not dict:
        _refuse(code, path + ": expected exact object")
    # Iterating an exact dict neither hashes nor compares its stored keys, and
    # unknown fields are refused before any value of this object is visited.
    for key in value:
        if type(key) is not str:
            _refuse(code, path + ": expected exact string keys")
        if closed and key not in fields:
            _refuse(code, path + ": expected closed schema object")
    for key, kind in fields.items():
        if key not in value:
            _refuse(code, path + "." + key + ": expected required field")
        _admit(value[key], kind, code, path + "." + key)


def _admit(value, kind, code, path):
    tag = kind[0]
    if tag == "object":
        _admit_object(value, kind[1], kind[2], code, path)
        return
    if tag == "hex":
        admitted = _is_hex(value, kind[1])
    elif tag == "hex-or-null":
        admitted = value is None or _is_hex(value, kind[1])
    elif tag == "int":
        admitted = (type(value) is int and kind[1] <= value
                    and (kind[2] is None or value <= kind[2]))
    elif tag == "str":
        admitted = type(value) is str
    elif tag == "fixed":
        admitted = type(value) is str and value == kind[1]
    elif tag == "premises":
        admitted = (type(value) is list and len(value) == len(_PREMISES)
                    and all(type(item) is str and item == premise
                            for item, premise in zip(value, _PREMISES)))
    else:
        admitted = False
    if not admitted:
        _refuse(code, path + ": outside the " + tag + " schema")


def prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id):
    """Return fresh C2 Issue artifacts, or raise ValueError('C2I-<stage>: ...').

    Stages run in the fixed order ARG, SETUP, EXPORT, RELATION, STATEMENT,
    CONTEXT, DIGEST, PROOF, CARDANO and BUILD; neither record is mutated.
    """
    # ARG: exact types only, so neither record is touched before this passes.
    if type(proof_export) is not dict or type(setup_record) is not dict:
        _refuse("C2I-ARG", "records: expected exact dict objects")
    if type(expiry) is not int or not 0 <= expiry <= _MAX_UINT64:
        _refuse("C2I-ARG", "expiry: expected exact int in 0..2**64-1")
    if type(policy_id) is not bytes or len(policy_id) != _POLICY_LEN:
        _refuse("C2I-ARG", "policy_id: expected exact 28-byte bytes")

    # SETUP: closed record, profile literals, consumed relation keys, VK digest.
    _admit(setup_record, _SETUP_RECORD, "C2I-SETUP", "setup")
    relation = setup_record["relation"]
    vk = bytes.fromhex(setup_record["verifyingKeyHex"])
    # Unkeyed BLAKE2b-256 of the 488 VK bytes (length only; no curve check).
    vk_digest = hashlib.blake2b(vk, digest_size=32).hexdigest()
    if vk_digest != setup_record["vkDigestBlake2b256"]:
        _refuse("C2I-SETUP", "vkDigestBlake2b256: differs from BLAKE2b-256 of the VK")

    # EXPORT: closed ProofExport schema at every level.
    _admit(proof_export, _PROOF_EXPORT, "C2I-EXPORT", "export")

    # RELATION: the export names the setup relation and its VK.
    for key in _RELATION_IDENTITY:
        if proof_export[key] != relation[key]:
            _refuse("C2I-RELATION", key + ": differs from the setup relation")
    if proof_export["vkDigestBlake2b256"] != vk_digest:
        _refuse("C2I-RELATION", "vkDigestBlake2b256: differs from the setup VK digest")

    # STATEMENT: bounded lowercase hex, host header admission, relation constants.
    statement_hex = proof_export["statementHex"]
    if (len(statement_hex) > 2 * _MAX_STATEMENT_BYTES or len(statement_hex) % 2
            or not set(statement_hex).issubset(_HEX_DIGITS)):
        _refuse("C2I-STATEMENT", "statementHex: expected bounded even lowercase hex")
    statement = bytes.fromhex(statement_hex)
    header = _host("C2I-STATEMENT", host_codec.parse_statement_header, statement)
    for name, expected in (("predicateDigest", relation["predicateDigest"]),
                           ("shapeDigest", relation["abi2ShapeDigest"]),
                           ("vkDigest", vk_digest),
                           ("bodyLen", relation["bodyLen"])):
        if header[name] != expected:
            _refuse("C2I-STATEMENT", "header " + name + ": differs from the relation")

    # CONTEXT: declared fields equal the header bytes; testnet only (network 0
    # does not distinguish Preview from Preprod).
    if proof_export["context"] != {key: header[key] for key in _CONTEXT}:
        _refuse("C2I-CONTEXT", "context: differs from the statement header")
    if header["networkId"] != 0:
        _refuse("C2I-CONTEXT", "networkId: only testnet 0 is admitted")

    # DIGEST: host digest and public inputs as exact strings.
    digest = _host("C2I-DIGEST", host_codec.statement_digest_hex, statement)
    if digest != proof_export["statementDigestBlake2b256"]:
        _refuse("C2I-DIGEST", "statementDigestBlake2b256: differs from the statement")
    public_inputs = proof_export["publicInputs"]
    halves = _host("C2I-DIGEST", host_codec.statement_public_inputs, statement)
    if halves != (public_inputs["x1"], public_inputs["x2"]):
        _refuse("C2I-DIGEST", "publicInputs: differ from the statement digest halves")

    # PROOF: 192 bytes by length only; no curve, subgroup or pairing check.
    if not _is_hex(proof_export["proofHex"], _PROOF_HEX_LEN):
        _refuse("C2I-PROOF", "proofHex: expected exactly 384 lowercase hex")

    # CARDANO: the section equals the cardano_export derivation.
    predicate = relation["predicateDigest"]
    shape = relation["abi2ShapeDigest"]
    derived = {
        "envV2": {
            "version": 2,
            "claimKind": 0,
            "familyId": _FAMILY_HEX,
            "predicateDigest": predicate,
            "vkDigest": vk_digest,
            "claimantVkh": header["claimantVkh"],
            "settleOutref": {"transactionId": header["consumedTxid"],
                             "outputIndex": header["consumedIx"]},
            "statementDigest": digest,
        },
        "catalogueActionEntry": {
            "name": relation["action"],
            "index": relation["actionIndex"],
            "predicateDigest": predicate,
            "vkDigest": vk_digest,
            "shapeDigest": shape,
        },
        "intentTargetHashV2": predicate + ":" + vk_digest + ":" + shape,
        "intentAnchor": "cardano-plutus-v3:" + header["settleHash"],
    }
    if proof_export["cardano"] != derived:
        _refuse("C2I-CARDANO", "cardano: differs from the derived section")

    # BUILD: host factories only; the expiry comes from the argument alone.
    env_data = _host("C2I-BUILD", host_codec.build_env_data,
                     bytes.fromhex(predicate), bytes.fromhex(vk_digest),
                     bytes.fromhex(header["claimantVkh"]),
                     bytes.fromhex(header["consumedTxid"]), header["consumedIx"],
                     bytes.fromhex(digest), expiry)
    env_cbor = _host("C2I-BUILD", host_codec.encode_data, env_data)
    token = hashlib.blake2b(env_cbor, digest_size=32).digest()
    issue_data = _host("C2I-BUILD", host_codec.build_c2_mint_data, "Issue", vk,
                       bytes.fromhex(proof_export["proofHex"]))
    issue_cbor = _host("C2I-BUILD", host_codec.encode_data, issue_data)
    # F01: canonical no-ada Value (lovelace None, never 0).
    mint_value_data = _host("C2I-BUILD", host_codec.build_value_data, None,
                            [(policy_id, token, 1)])
    mint_value_cbor = _host("C2I-BUILD", host_codec.encode_data, mint_value_data)
    return {
        "format": _ARTIFACT_FORMAT,
        "envData": env_data,
        "envCborHex": env_cbor.hex(),
        "envelopeTokenNameHex": token.hex(),
        "issueData": issue_data,
        "issueCborHex": issue_cbor.hex(),
        "mintValueData": mint_value_data,
        "mintValueCborHex": mint_value_cbor.hex(),
        "policyIdHex": policy_id.hex(),
        "expiry": expiry,
        "setupRecordSha256Declared": proof_export["setupRecordSha256"],
        "labels": dict(_LABELS),
    }


## S12 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/REPORT.md
FRESH NON-AUTHOR REVIEW (claude-opus-5-5, tools off, inline bodies only). I read all 17 supplied bodies (S01-S17) as text. I executed nothing, imported nothing and recomputed no hashes. Every byte count and SHA-256 below is the value declared by the supplied headers or the root RESULT files, not one I verified. Same-provider independence is context-only.

VERDICT: AGREE-NARROW. Full source inspection finds that c2_issue_artifacts.py (declared 15231 B, 23c3f99d...) implements the frozen CI01-CI12 contract as written. I found no correctness defect. Root's actual RED (20:56Z: 11 run, 1 pass, 10 failures all 'missing implementation', 0 errors) matches the author prediction. Root's actual GREEN (21:06Z: 11/11, exit 0, five pinned files unchanged) is newer than every author-time 'RED and GREEN unobserved' statement in S01, S03 and S06. Those statements are kept here as historical snapshots. The component can be adopted as source ONLY as an explicitly unverified structural artifact serializer, with conditions R01-R03 recorded. It is not equivalent to Rust parse_setup_record/check_export acceptance. It closes no integration, native, operator, ledger or authority boundary.

CLAUSE CONFORMANCE (source):
CI01: The module's only imports are `import hashlib` and `import host_codec`. Its only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id), with no defaults. All seven host APIs are called as host_codec attributes: parse_statement_header, statement_digest_hex, statement_public_inputs, build_env_data, encode_data (x3), build_c2_mint_data and build_value_data. hashlib appears only for the VK digest and the token, and neither is a host API. test_02 checks the signature and host identity, not that the calls are made (R05). Source inspection confirms the calls.
CI02: type checks with `is`; bool is excluded; the 0..2^64-1 range and the 28-byte bytes check all happen before any subscript; the _Hooked test confirms this.
CI03: expiry reaches only build_env_data. Every export object is closed (top, context, publicInputs, cardano, envV2, settleOutref, catalogueActionEntry), so an expiry key at any level is refused with C2I-EXPORT. The label is present. setup.relation is open, but its values are never read for expiry.
CI04: there are 12 closed keys. The notice, snark, encoding and six-premise literals match export.rs character for character, and GREEN on the real fixture confirms it. driver is any str; provingKeyBytes is int >= 1 with no upper bound. The relation is an exact dict with exact str keys; its consumed keys are typed and the rest stay open. headerLen is 343, bodyLen is 0..261801 (= 262144 - 343) and publicInputCount is 2. The VK is 976 hex characters, and BLAKE2b-256(vk) == vkDigestBlake2b256.
CI05: field names match the Rust structs exactly (18/10/2/4/8/2/5). The u8/u16/u64 ranges are exact. Fixed widths are 64/56. sourceSha256 is hex-or-null. The str leaves, the fixed format and the premises list are all checked.
CI06: the loop covers the 8 identity keys plus the VK digest.
CI07: length is checked before the set() scan, so the 524288-character bound holds before any O(n) work. Parity, lowercase, host admission, and header predicate/shape/vk/bodyLen are checked.
CI08: dict equality is checked against the 10 header keys; booleans were already excluded, so 1 == True cannot match. networkId == 0 is checked after equality. A comment says network 0 does not distinguish Preview from Preprod.
CI09: digest and halves use exact strings via the host. The proof is checked as 384 lowercase hex. The derived cardano section matches cardano_export field for field (familyId = FAMILY_V2 hex; claimant, outref and anchor from the header; name, index, predicate and shape from the relation).
CI10: there are 12 keys. env, issue and value objects are fresh from the host. labels is dict(_LABELS), and its values are immutable strings. Byte bounds check out: env is 212-222 B (expiry head 1-9 B, outputIndex head 1-3 B), issue is 710 B, mint is 67 B. The EnvV2 field order, OutputReference constr 0 [txid, ix] and C2Mint Issue constr 0 [vk, proof] match types.ak. The token rule matches claim.ak envelope_name.
CI11: the module raises only ValueError('C2I-<stage>: ...'). _host rewraps ValueError only, and MemoryError and similar propagate, consistent with the host's I2 rule. Nothing in the module writes to its inputs. There is no I/O, clock, environment access, randomness, logging, caching or global mutation.
CI12: labels and the docstring carry the required non-claims.

BUILD and DIGEST rewrap reachability: every host precondition is implied by earlier stages. predicate, vk and digest are 32 B and claimant is 28 B (hex-checked); ix is u16 from a 2-byte header field; expiry was checked at ARG; vk is 488 B and the proof 192 B; policy is 28 B and the token 32 B with quantity 1. DIGEST uses the same admission as STATEMENT. So C2I-BUILD and the DIGEST host refusals cannot happen for admitted inputs. They are defensive code that ordinary tests cannot reach without substituting the host. This is source-inspected only, not tested (R04).

TEST COVERAGE (GREEN covers selected subtests only). Covered: the API shape; the K6 env, issue and mint vectors; two policies; four expiry encodings and six bad expiries; result schema, labels, widths, determinism and detachment; no mutation on the success path; 29 tampers; 8 first-failure orderings; 11 ARG cases; and the import, fixture-token and effect scan.
Untested branches that are correct by source inspection (R03):
- Schema kinds: no refusal test for the 'str' kind or for a malformed non-null 'hex-or-null', and none for a missing required field or a non-dict nested object.
- SETUP: wrong format, notice, curve, snark or encoding; non-str driver; malformed provingKeySha256; provingKeyBytes 0, negative or bool; non-dict relation; a missing consumed relation key; a bad statementTagHex or familyIdHex; bodyLen out of range; publicInputCount other than 2.
- EXPORT: extra keys in context, publicInputs, cardano, settleOutref or catalogueActionEntry; u8, u16 or u64 upper-bound violations; premises reordered, as a tuple, or with one character changed.
- RELATION: mismatch of action, compilerBuildDigest, r1csSha256, predicateDigest or abi2ShapeDigest alone, and of the export vkDigestBlake2b256 alone.
- STATEMENT: more than 524288 characters; non-hex characters; a flip in the shape digest (offset 80), tag or family; a relation bodyLen mismatch at a consistent length.
- DIGEST: an x2-only mismatch.
- PROOF: uppercase.
- CARDANO: catalogueActionEntry, intentTargetHashV2, envV2 version, familyId or statementDigest.
- Purity: no-mutation on refusal paths.
- Host use: no check that the host APIs are actually invoked.
- Context: only one real (fabricated, historical) K6 context is exercised.
All of these reuse code paths that are partly exercised: the generic _admit/_admit_object (closed check at two levels; int, hex, fixed and premises kinds), the identity loop, host admission and dict equality. For a narrow serializer they do not block adoption, but each must be recorded as source-inspected, not tested. A separately reviewed supplementary test revision is required before anyone claims refusal completeness.

F01-F08 CROSSCHECK DECISIONS:
F01 (driver is any str; Rust requires DRIVER_ID 'mori-k2-driver/0.1.0-k2s1+k4h1'). Confirmed from lib.rs and export.rs. Acceptable ONLY for the unverified serializer. It rules out any claim that an admitted setup is one Rust would accept. Pin it in the next contract revision; it is still a declared name, not provenance.
F02 (open relation subset vs deny_unknown_fields and full check_profile). Confirmed. The consumer admits descriptors Rust refuses: extra or missing unconsumed keys, wrong language, semantics or adapter, instanceVariables other than 3, a non-decimal logicalWork, and abi2ShapeDigest != BLAKE2b(abi2ShapeBytesHex). The output does not depend on these fields, but this must not be read as a moriarty/1 / moriarty-core/1 / SettlementHashV2 profile check. Do not let this cheaper library subset quietly stand in for the current language requirement. The adoption record must state that relationProfile is NOT_CHECKED beyond the consumed keys. The next revision should pin language, semantics, adapter and instanceVariables before the builder treats admission as meaningful.
F03 (curve strictness not mirrored). Confirmed against encoding.rs decode_vk/decode_proof/vk_profile_ok. These are non-pairing checks that Rust check_export performs. Acceptable as an explicit non-claim. vkCurveValidity NOT_CHECKED is labelled, but proof-point validity is covered only implicitly by groth16 NOT_VERIFIED; add an explicit proofCurveValidity label in a later revision.
F04 (provingKeyBytes has no u64 bound). Confirmed. A superset of Rust, info only; add the u64 bound in the next revision.
F05 (no compiler re-encode). Confirmed against claim.rs parse_statement. abi2.rs is unread, so whether re-encoding refuses anything beyond tag, family and length is unknown. Acceptable as a non-claim; compiler-encoder correspondence stays OPEN and there is no label for it yet.
F06 (exact dict and str interpretation). AGREE. It is narrower than plain 'dict' and consistent with JSON-compatible input and the host's _wire_keys idiom. Note that str leaves accept lone-surrogate text that serde_json refuses. Such text never reaches the artifacts, but it is one more superset point. Clarify the wording in the next contract text.
F07 (protocol literals). AGREE. I checked the tag and family hex against the ASCII. 343 and 262144 match claim.rs and the host. The literals are constants, not copied host algorithms, and they avoid coupling to the host's private attributes. The drift risk is bounded by the hash-pinned host and redundant admission.
F08 (untested paths). Confirmed and expanded under R03. Its 'GREEN unobserved' wording is historical, because root GREEN is now observed.

NARROWINGS RELATIVE TO RUST (acceptable): sourceSha256 must be present (serde treats a missing Option as None); x1 and x2 are compared as exact canonical strings (hexutil.parse_u128_decimal unread); networkId must be 0; premises are checked at EXPORT. The wording 'non-pairing consistency' in the module docstring, and the DESIGN-POSITIONS rationales 'Mirrors parse_setup_record' (CI04) and 'Covers the remaining non-pairing check_export steps' (CI09), overstate parity (R02).

LIMITS PRESERVED: the current real public path is the old fixture-based R2 builder. No r3 implementation exists; the latest builder is only at actual RED (1 historical pass, 9 missing). The byte parser, I/O, CLI and the sha256(setup bytes) link check remain open. The legacy uppercase Data/UPLC bridge is open. The K5/K6 setup is single-party, TEST ONLY and forgeable. VK-to-matrix, compiler and hash-gadget correspondence is OPEN. Context and prover-chosen roots are not ledger-authenticated. There is no funded-context proof, no transaction, and no Preview or Preprod result. There is no general bundle and no recursive, private, multi-parent or universal PCD; one ProofExport is not a bundle or PCD. Full20 UTxO, financial, ACTUS and DeFi obligations stay OPEN. The host accepts structure only; it has no authority over min-ADA, fees, collateral, ExUnits or fit. The historical K6 Env CBOR and token are reproduction vectors, not an oracle. This review grants no maintainer, council, campaign or toolchain permissions.


## S13 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/FINDINGS.json
[
  {
    "id": "R01-rust-acceptance-superset",
    "severity": "medium (non-blocking for narrow serializer adoption; blocks any Rust-acceptance or profile claim)",
    "summary": "Admission checks a declared subset of Rust parse_setup_record + check_export, so the consumer accepts inputs Rust refuses. Missing checks: DRIVER_ID; relation deny_unknown_fields and check_profile (language, semantics, adapter, 9-hex set, logicalWork, instanceVariables==3, shape-bytes digest); provingKeyBytes u64 bound; decode_vk/decode_proof; parse_statement re-encode; the setupRecordSha256 byte link; UTF-8-valid strings. The output labels expose only groth16, vkCurveValidity and setupRecordLink.",
    "evidence": "S04 _SETUP_RECORD 'driver': _TEXT, 'provingKeyBytes': ('int', 1, None), 'relation': ('object', _RELATION, False); _LABELS has 8 keys with no relationProfile, statementReencode or proofCurveValidity label; S09 parse_setup_record fixed(driver, DRIVER_ID) and check_profile(); S10 RelationDescriptor deny_unknown_fields and check_profile; S11 parse_statement rebuild; S12 decode_vk/decode_proof.",
    "requiredFix": "Adoption record must state: 'structural serializer; NOT equivalent to Rust parse_setup_record/check_export acceptance; relation language/semantics/adapter/profile, driver identity, statement re-encode and proof/VK curve validity NOT_CHECKED'. A separately reviewed contract revision is required before builder integration treats admission as meaningful. It should pin DRIVER_ID, language moriarty/1, semantics moriarty-core/1, adapter SettlementHashV2, instanceVariables 3 and provingKeyBytes <= 2^64-1, and add labels relationProfile, statementReencode and proofCurveValidity. Optionally close the relation schema to all 30 Rust keys."
  },
  {
    "id": "R02-parity-wording-overclaim",
    "severity": "low",
    "summary": "The module docstring 'checks the non-pairing consistency' and the DESIGN-POSITIONS rationales CI04 'Mirrors parse_setup_record where supplied' and CI09 'Covers the remaining non-pairing check_export steps in Rust order' overstate parity. Several non-pairing Rust steps are not mirrored.",
    "evidence": "S04 docstring lines 4-6; S02 CI04 and CI09 rationale; S09 check_export performs setupRecordSha256, decode_proof, parse_u128_decimal and parse_statement (re-encode), none of which the consumer mirrors.",
    "requiredFix": "Do not repeat these rationales in the adoption record. The next source revision should change the docstring to 'checks a declared subset of the non-pairing check_export consistency'. That edit changes bytes and needs a new root GREEN."
  },
  {
    "id": "R03-untested-refusal-branches",
    "severity": "medium (review attention; source-inspected only, not silently approved)",
    "summary": "Ordinary GREEN does not exercise: the 'str' and malformed 'hex-or-null' refusal kinds; the missing-required-field and non-dict nested-object branches; most SETUP literals and leaves; nested extra keys outside top level and envV2; u8/u16/u64 upper bounds; premises variants; five of eight RELATION keys plus the export vkDigest alone; statement >524288 characters, non-hex, shape/tag/family flips and relation bodyLen mismatch; x2 alone; uppercase proof; most CARDANO fields; no-mutation on refusal paths. Only one real context is used.",
    "evidence": "S03 TAMPER (29 entries), test_09 (8 cases), test_06 no-mutation on success only; S04 _admit/_admit_object branches for 'str', 'hex-or-null', missing field and non-dict.",
    "requiredFix": "Record these as source-inspected-only. Before any claim of refusal completeness or builder reliance, run a separately reviewed supplementary test revision that adds one tamper per listed branch and kind, plus refusal-path deepcopy equality. Do not edit the frozen test in place."
  },
  {
    "id": "R04-build-and-digest-rewrap-unreachable",
    "severity": "info",
    "summary": "C2I-BUILD and the C2I-DIGEST host-refusal rewraps cannot trigger for admitted inputs, because every host precondition is implied by earlier stages. They are defensive code with no ordinary test.",
    "evidence": "S04 BUILD arguments come from hex64/hex56-checked fields, a 2-byte header ix, the ARG-checked expiry, a 976-hex VK, a 384-hex proof, a 28-byte policy, a 32-byte token and quantity 1; S05 build_env_data, build_c2_mint_data and build_value_data bounds; DIGEST reuses _admit_statement, already passed at STATEMENT.",
    "requiredFix": "None for narrow adoption. Optionally, a separate reviewed test could exec the consumer against a refusing host proxy without displacing the pinned host."
  },
  {
    "id": "R05-host-call-not-test-enforced",
    "severity": "info",
    "summary": "test_02 checks host identity and that no API was copied, but not that the seven APIs are invoked. A hashlib reimplementation of statement_digest_hex would still pass. Source inspection confirms all seven calls.",
    "evidence": "S03 test_02 and test_11; S04 _host(...) calls to all seven host_codec attributes.",
    "requiredFix": "Optional future test: a counting proxy module for a separate consumer exec. Not required for narrow adoption."
  },
  {
    "id": "R06-historical-wording-and-mutable-module-tables",
    "severity": "info",
    "summary": "The author-time statements 'RED and GREEN unobserved' / 'not reviewed or adopted' (S01, S03, S04, S06) are historical snapshots; the actual root RED and GREEN are newer. Module-level schema and label dicts are mutable but only read or copied.",
    "evidence": "S01 Status; S03 docstring; S06 F08; S07/S08 timestamps; S08 'unreviewed': true, 'adopted': false; S04 _LABELS and schema dicts.",
    "requiredFix": "Keep the historical wording unchanged, and cite S07/S08 as the actual results. Optionally freeze the tables (tuples or mapping proxies) in a later revision; in-process tampering is out of scope."
  }
]


## S14 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json
{
  "format": "root-c2-component-source-adoption/1",
  "observedAt": "2026-10-16T21:58:08.524944+00:00",
  "requestedModel": "claude-opus-5-5 HIGH",
  "actualCanonicalModel": "claude-opus-5-5",
  "actualEffort": "not independently observed",
  "terminalExitCode": 0,
  "wholeOriginalSchemaValid": true,
  "all17InputPinsUnchanged": true,
  "declaredInlineReadCoverageMatches17IDs": true,
  "canonicalLiteralClausesMatched": 12,
  "nonauthorAgreePositions": 12,
  "independence": "Fresh context, same provider; not independent provider diversity.",
  "qualifiedScope": "Only explicitly unverified structural artifact serializer implementing frozen CI01-CI12; no Rust acceptance parity or current-language profile check.",
  "reviewConditions": [
    "R01: structural serializer; NOT equivalent to Rust parse_setup_record/check_export acceptance; relation language/semantics/adapter/profile, driver identity, statement re-encode and proof/VK curve validity NOT_CHECKED.",
    "R02: do not repeat parity-overstating docstring/rationales as acceptance evidence.",
    "R03: listed refusal branches remain source-inspected only, not tested; supplementary tests required before refusal-completeness or builder-reliance claims."
  ],
  "adoptionPermittedOnlyWithConditionsRecorded": true,
  "builderReliancePermitted": false,
  "nativeAuthority": 0,
  "reportSha256": "05c66c0dbeac99f387403498b4012a9bb8f93e8c5ea84602f587312d7f98c34d",
  "componentSource": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py",
  "componentBytes": 15231,
  "componentSha256": "23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76",
  "sourceAdopted": true,
  "scopeExpansion": false,
  "actualOrdinaryResult": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-first-ordinary-green-20261016/RESULT.json",
  "historicalActualTestCount": 11,
  "disposition": "Adopt exact reviewed component only under the Opus limitations above. The genuine current-language requirement is preserved; a separate reviewed profile/test revision is required before builder reliance. No bytes changed in historical source or tests.",
  "openBoundaries": [
    "strict byte parser and setup file-byte link",
    "actual r3 builder and finite legacy bridge",
    "full current Rust relation/profile checks and supplementary refusal coverage",
    "native proof and actual Preview then Preprod transactions",
    "general proof sets, universal certificates, authenticated ledger claims, recursive/private/multi-parent PCD",
    "all 20 UTXO families, ACTUS, DeFi and general obligations"
  ]
}


## S15 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K2-PUBLIC-SETUP-RECORD.json
{
  "format": "moriarty-k2-groth16-setup/1",
  "trustProfile": "single-party-test-setup/1",
  "notice": "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. Whoever controlled the setup process could forge proofs for this relation. VK provenance is declared by this record; no verifier can check that the VK was generated from r1csSha256.",
  "relation": {
    "relationFormat": "mori-r1cs-transition/1-unadopted",
    "relationVersion": 1,
    "language": "moriarty/1",
    "semantics": "moriarty-core/1",
    "logicalCostProfile": "moriarty-reference-logical-cost/1",
    "profileDigest": "f388e435d80aa53e35049128a773dbb1a19f345290b37d12151db205c8c9292a",
    "programHash": "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9",
    "sourceSha256": "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9",
    "action": "partial",
    "actionIndex": 1,
    "relationShapeDigest": "4d755ee1f766b05981706355e27fc8c1bc0a7bf124742d0bd09aaab6bd5c6516",
    "compilerSourceManifestSha256": "eea1a420ec996e6369ef2eb1a60208f682ac21cf646cfa80c8729c7621722d47",
    "coreClosureSha256": "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed",
    "compilerBuildDigest": "0069e686c5a58165d177b6866531769680c8460fa300b2094a597b1639adad09",
    "adapter": "SettlementHashV2",
    "logicalWork": "28",
    "predicateDigest": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
    "abi2ShapeDigest": "36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969",
    "abi2ShapeBytesHex": "4d524731362d5056432d5348502d30320001000b6f7261636c652d756e6974000e696e766f6963652d69737375657200046361736800000300066d61726b65720000066f70656e6564010004706169640000010006616d6f756e7400000300000006657363726f7700000004666565730000000673656c6c657200020007696e766f69636500000006657363726f77000673656c6c657200077265736572766500000006657363726f770005627579657200040200000001010000000203000001000002000100000008000000077061727469616c0001",
    "bodyLen": 378,
    "statementTagHex": "4d524731362d5056432d53544d542d32",
    "familyIdHex": "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32",
    "headerLen": 343,
    "fieldModulusLeHex": "01000000fffffffffe5bfeff02a4bd5305d8a10908d83933487d9d2953a7ed73",
    "r1csSha256": "e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0",
    "constraints": 308790,
    "instanceVariables": 3,
    "witnessVariables": 306452,
    "publicInputCount": 2
  },
  "curve": "BLS12-381",
  "snark": "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)",
  "encoding": "arkworks 0.4 compressed points (Zcash BLS12-381 layout); VK 488 bytes, proof 192 bytes",
  "verifyingKeyHex": "a1d3222b7d8a7211d331a755687b11a12ba4256e9022c19345b5e7d95e74b093f5a0d48dd922aba4abfbf8179f50045e9668aeb28cae8317ec2795c1826d76879fd39f50d3cc5f5333b56433ea873c2fdfdde258c4ddba246b3a7660d604fb460506d663033240821a56e05c817152bf1728f2eb70eb35a2cbcf52b75b599f44fd39e4edbe63ccab13280df90635da92947f7c86a540d61c4598c82aaba6d5db26400d369d849779d04d23309118546b440447cf24d4cfefd4663394ddd3aabd0edc2863e52578d2af13e2f5da858e638c6cf629b94c977e4e1abbe7f2d95437c1d9d29db3916f21c8b4bfc56e51cd81881b302d8a709038e642c7e426edb538fc1b69f0bb205d89584bf577907659a479f2eb674216075ecb9c0bd5a0c0678f172eb3a7938087c77cf91885750e6c3684f92798a1e62f540524d800ac928481e5a8f83194f36a5b85165ec6062a6ed603000000000000008f8e7855e9f7f8914208fca2beaffd968ae16a240e14c54a1e4dfa8a1f9c2c9b8190cabf864d8e7ef10aafd4f8c081f5891a98e4adb3ed589c6a8e897aefd3b865c416a8874173ed6ed4ad8c0ddf376ff6fefe29179d1999135750683aecb2d4a03df7bf82089977a567a9fd05ddc3d7a897765ce9b0644a1b3f13dc3d1fac1c36dfd661a202313cf450fb1667eb84e9",
  "vkDigestBlake2b256": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
  "provingKeySha256": "cadcf470b30c76ca7dc91838c8325fd00e6b0778a4082d7cf9d13a023fe4b614",
  "provingKeyBytes": 98715456,
  "driver": "mori-k2-driver/0.1.0-k2s1+k4h1"
}


## S16 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K6-PUBLIC-PROOF.json
{
  "format": "moriarty-k2-groth16-proof/1",
  "setupRecordSha256": "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b",
  "programHash": "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9",
  "sourceSha256": "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9",
  "action": "partial",
  "actionIndex": 1,
  "compilerBuildDigest": "0069e686c5a58165d177b6866531769680c8460fa300b2094a597b1639adad09",
  "r1csSha256": "e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0",
  "predicateDigest": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
  "abi2ShapeDigest": "36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969",
  "vkDigestBlake2b256": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
  "context": {
    "vkDigest": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
    "claimantVkh": "55555555555555555555555555555555555555555555555555555555",
    "networkId": 0,
    "settleHash": "a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2",
    "threadId": "7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a",
    "consumedTxid": "2222222222222222222222222222222222222222222222222222222222222222",
    "consumedIx": 0,
    "seq": 1,
    "catalogueDigest": "37de4837bc1417d65df0c40b4829d796b04d97fd037bb128fc0814abb5646e18",
    "intentsDigest": "b8e2f734456ad3d939e91441330cb0635707e09d70af45edead18923108fc308"
  },
  "statementHex": "4d524731362d5056432d53544d542d324d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c320d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd5555555555555555555555555555555555555555555555555555555500a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a27a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a22222222222222222222222222222222222222222222222222222222222222220000000000000000000137de4837bc1417d65df0c40b4829d796b04d97fd037bb128fc0814abb5646e18b8e2f734456ad3d939e91441330cb0635707e09d70af45edead18923108fc3080000017a000000000000000000000000000000230000000000000000000000000000000701000000000000000000000000000000000000000000000000000000000000006d0000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000005a010000000000000000000000000000000e0000000000000000000000000000000701000000000000000000000000000000230000000000000000000000000000004800000000000000000000000000000002000000000000000000000000000000230100000000000000000000000000000037010000000000000000000000000000000e00000000000000000000000000000002000000000000000000000000000000230000000000000000000000000000005a000000000000000000000000000000370000000000000000000000000000000000000000000000000000000000000023000000000000000000000000000000000025000000000000000000000000000000000002",
  "statementDigestBlake2b256": "c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc",
  "publicInputs": {
    "x1": "268418678883876601707409168117995592842",
    "x2": "8714689549842776844379904284721120460"
  },
  "proofHex": "a2d16a6ad9b5071682b8ba27d7a71bf0d3d397f31b2708eab206cb1f8fac328496bfc379f42b4b4e28a9384d1b67a5f08fb24946df509f31c31aac76b2036c0798f70f730515e8b6a6b7e935f58872212bf5df5b38ec3d4aef5288d24399ac63165d6872c5d7b114615cd8e4c40857e655540052325c5468093f9617c51a2873488929471d80dc89278f6c12173f80eaa74d34724490ea9f4d629884b078d4ecd3dc761ad29ec10bb5af5a1f991d2313c2c24a993c8468967dc097e97e1cd242",
  "cardano": {
    "envV2": {
      "version": 2,
      "claimKind": 0,
      "familyId": "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32",
      "predicateDigest": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
      "vkDigest": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
      "claimantVkh": "55555555555555555555555555555555555555555555555555555555",
      "settleOutref": {
        "transactionId": "2222222222222222222222222222222222222222222222222222222222222222",
        "outputIndex": 0
      },
      "statementDigest": "c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc"
    },
    "catalogueActionEntry": {
      "name": "partial",
      "index": 1,
      "predicateDigest": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
      "vkDigest": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
      "shapeDigest": "36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969"
    },
    "intentTargetHashV2": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd:d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd:36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969",
    "intentAnchor": "cardano-plutus-v3:a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2"
  },
  "premises": [
    "The proof shows, under setup honesty and Groth16 knowledge soundness, that the statement's constant header and body satisfy the named compiled moriarty-core/1 relation.",
    "Context bytes (vk digest, claimant, network, settle hash, thread, consumed outref, seq, catalogue digest, intents digest) are caller-declared premises: bound by the proof, not authenticated by it. S2 recomputes each from its authenticated ScriptContext and datum.",
    "Trust profile single-party-test-setup/1: whoever ran setup could forge proofs for this relation.",
    "Groth16 proofs are malleable: the proof bytes are not a unique claim identifier.",
    "Signed intents, debit authority, frame completeness, physical asset delta, replay and phase outcomes are target obligations outside this proof.",
    "No ledger admission, deployment, settlement, Preprod or Midnight result is implied."
  ]
}


## S17 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/relation.rs
//! Compilation through the actual frozen compiler and the descriptor of every identity a
//! compiled SettlementHashV2 relation binds.
//!
//! Nothing here builds constraints or identities by hand. `compile_selected` calls
//! `mori_r1cs::compile_source::<Fr>` (Core `language` routing, `parse`, `validate`,
//! `require_core1`, plan, layout, probe synthesis). A `moriarty/2` source is admitted by Core's
//! stage loader and refused by the compiler with `StagedNotLowered`; the driver surfaces that
//! refusal unchanged.
use crate::error::DriverError;
use crate::hexutil;
use crate::input::{RelationSelector, SELECTOR_FORMAT};
use ark_bls12_381::Fr;
use ark_ff::{BigInteger, PrimeField};
use ark_relations::r1cs::ConstraintMatrices;
use mori_r1cs::abi2::{FAMILY_V2, HEADER_LEN, STMT_TAG};
use mori_r1cs::identity::{self, Digest32};
use mori_r1cs::{AccountingBinding, CapBinding, CompileOptions, CompiledRelation, CompilerBuild, FrameCells, StatementAdapter};
use serde::{Deserialize, Serialize};

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const CAP: &str = "NotBound";
pub const ACCOUNTING: &str = "GrossDebitAndFees";
pub const FRAME: &str = "TouchedOnly";
/// SettlementHashV2 public inputs: the two 128-bit digest halves.
pub const PUBLIC_INPUTS: u8 = 2;
/// arkworks counts the constant-one instance variable: 1 + 2.
pub const INSTANCE_VARIABLES: u64 = 3;

/// Refuses every selector value outside the one supported profile. Runs before compilation.
pub fn check_selector(s: &RelationSelector) -> Result<(), DriverError> {
    let fixed = [
        ("format", s.format.as_str(), SELECTOR_FORMAT),
        ("language", s.language.as_str(), LANGUAGE),
        ("semantics", s.semantics.as_str(), SEMANTICS),
        ("adapter", s.adapter.as_str(), crate::ADAPTER),
        ("cap", s.cap.as_str(), CAP),
        ("accounting", s.accounting.as_str(), ACCOUNTING),
        ("frame", s.frame.as_str(), FRAME),
    ];
    for (what, got, want) in fixed {
        if got != want {
            return Err(DriverError::Unsupported(format!("selector {what} {got:?}; this driver supports only {want:?}")));
        }
    }
    if s.action.is_empty() || s.action.len() > 1024 {
        return Err(DriverError::Input("selector action must contain 1..1024 bytes".into()));
    }
    Ok(())
}

/// The selector that recompiles the relation a descriptor names (verify with `--source`).
pub fn selector_for(desc: &RelationDescriptor) -> RelationSelector {
    RelationSelector {
        format: SELECTOR_FORMAT.into(),
        language: desc.language.clone(),
        semantics: desc.semantics.clone(),
        action: desc.action.clone(),
        adapter: desc.adapter.clone(),
        cap: CAP.into(),
        accounting: ACCOUNTING.into(),
        frame: FRAME.into(),
        expected_source_sha256: desc.source_sha256.clone(),
        expected_program_hash: Some(desc.program_hash.clone()),
    }
}

/// Every identity of one compiled SettlementHashV2 relation, as lowercase hex or decimal.
/// Built only by [`RelationDescriptor::of`] from an actual compilation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RelationDescriptor {
    pub relation_format: String,
    pub relation_version: u64,
    pub language: String,
    pub semantics: String,
    pub logical_cost_profile: String,
    pub profile_digest: String,
    pub program_hash: String,
    /// Raw source bytes; declared provenance, never bound in-circuit.
    pub source_sha256: Option<String>,
    pub action: String,
    pub action_index: u16,
    /// mori-r1cs `mori-r1cs-shape/1` digest (SHA-256), a header constant inside the predicate.
    pub relation_shape_digest: String,
    pub compiler_source_manifest_sha256: String,
    pub core_closure_sha256: String,
    pub compiler_build_digest: String,
    pub adapter: String,
    pub logical_work: String,
    /// `RelationIdentity::digest()`; statement bytes 48..80 and the catalogue entry.
    pub predicate_digest: String,
    /// BLAKE2b-256 of the `MRG16-PVC-SHP-02` bytes; statement bytes 80..112.
    pub abi2_shape_digest: String,
    pub abi2_shape_bytes_hex: String,
    pub body_len: u32,
    pub statement_tag_hex: String,
    pub family_id_hex: String,
    pub header_len: u32,
    pub field_modulus_le_hex: String,
    /// `mori-r1cs-matrices/1` digest of the probe synthesis.
    pub r1cs_sha256: String,
    pub constraints: u64,
    pub instance_variables: u64,
    pub witness_variables: u64,
    pub public_input_count: u8,
}

fn count(n: usize, what: &str) -> Result<u64, DriverError> {
    u64::try_from(n).map_err(|_| DriverError::Backend(format!("{what} exceeds u64")))
}

impl RelationDescriptor {
    /// Reads every identity from the compiled relation. Refuses a relation that is not
    /// SettlementHashV2 or whose predicate constant differs from its identity digest.
    pub fn of(relation: &CompiledRelation) -> Result<Self, DriverError> {
        if relation.adapter() != StatementAdapter::SettlementHashV2 {
            return Err(DriverError::Unsupported("relation adapter is not SettlementHashV2".into()));
        }
        let consts = relation
            .abi2()
            .ok_or_else(|| DriverError::Backend("SettlementHashV2 relation without ABI v2 constants".into()))?;
        let id = relation.identity();
        let predicate = id.digest().map_err(DriverError::Compile)?;
        if predicate.0 != consts.predicate_digest {
            return Err(DriverError::Backend("ABI v2 predicate constant differs from RelationIdentity::digest()".into()));
        }
        let probe = relation.probe();
        let header_len = u32::try_from(HEADER_LEN).map_err(|_| DriverError::Backend("header length".into()))?;
        Ok(Self {
            relation_format: id.relation_format.clone(),
            relation_version: id.relation_version,
            language: id.language.clone(),
            semantics: id.semantics.clone(),
            logical_cost_profile: id.logical_cost_profile.clone(),
            profile_digest: id.profile_digest.hex(),
            program_hash: id.program_hash.hex(),
            source_sha256: id.source_sha256.map(|d| d.hex()),
            action: id.action.clone(),
            action_index: id.action_index,
            relation_shape_digest: id.shape_digest.hex(),
            compiler_source_manifest_sha256: id.compiler_build.compiler_source_manifest_sha256.hex(),
            core_closure_sha256: id.compiler_build.core_closure_sha256.hex(),
            compiler_build_digest: id.compiler_build_digest.hex(),
            adapter: crate::ADAPTER.into(),
            logical_work: id.logical_work.0.to_string(),
            predicate_digest: hexutil::encode(&consts.predicate_digest),
            abi2_shape_digest: hexutil::encode(&consts.shape_digest),
            abi2_shape_bytes_hex: hexutil::encode(&consts.shape_bytes),
            body_len: consts.body_len,
            statement_tag_hex: hexutil::encode(STMT_TAG),
            family_id_hex: hexutil::encode(FAMILY_V2),
            header_len,
            field_modulus_le_hex: probe.field_modulus_le_hex.clone(),
            r1cs_sha256: probe.r1cs_sha256.hex(),
            constraints: count(probe.constraints, "constraints")?,
            instance_variables: count(probe.instance_variables, "instance variables")?,
            witness_variables: count(probe.witness_variables, "witness variables")?,
            public_input_count: PUBLIC_INPUTS,
        })
    }

    /// Syntax and profile checks on a descriptor read from a file (no recompilation).
    pub fn check_profile(&self) -> Result<(), DriverError> {
        let fixed = [
            ("language", self.language.as_str(), LANGUAGE),
            ("semantics", self.semantics.as_str(), SEMANTICS),
            ("adapter", self.adapter.as_str(), crate::ADAPTER),
        ];
        for (what, got, want) in fixed {
            if got != want {
                return Err(DriverError::Unsupported(format!("descriptor {what} {got:?}; expected {want:?}")));
            }
        }
        for (what, value) in [
            ("profileDigest", &self.profile_digest),
            ("programHash", &self.program_hash),
            ("relationShapeDigest", &self.relation_shape_digest),
            ("compilerSourceManifestSha256", &self.compiler_source_manifest_sha256),
            ("coreClosureSha256", &self.core_closure_sha256),
            ("compilerBuildDigest", &self.compiler_build_digest),
            ("predicateDigest", &self.predicate_digest),
            ("abi2ShapeDigest", &self.abi2_shape_digest),
            ("r1csSha256", &self.r1cs_sha256),
        ] {
            hexutil::decode_exact::<32>(value, what)?;
        }
        if let Some(s) = &self.source_sha256 {
            hexutil::decode_exact::<32>(s, "sourceSha256")?;
        }
        hexutil::parse_u128_decimal(&self.logical_work, "logicalWork")?;
        if self.statement_tag_hex != hexutil::encode(STMT_TAG) || self.family_id_hex != hexutil::encode(FAMILY_V2) {
            return Err(DriverError::Unsupported("descriptor statement tag or family id is not ABI v2".into()));
        }
        if self.header_len as usize != HEADER_LEN {
            return Err(DriverError::Unsupported("descriptor header length is not 343".into()));
        }
        if self.public_input_count != PUBLIC_INPUTS || self.instance_variables != INSTANCE_VARIABLES {
            return Err(DriverError::Unsupported("SettlementHashV2 has exactly two public inputs".into()));
        }
        // The ABI v2 shape digest is BLAKE2b-256 of the shape bytes the descriptor carries.
        let shape = hexutil::decode_bounded(&self.abi2_shape_bytes_hex, crate::claim::MAX_STATEMENT_BYTES, "abi2ShapeBytesHex")?;
        if hexutil::encode(&mori_r1cs::blake2b::blake2b256(&shape)) != self.abi2_shape_digest {
            return Err(DriverError::mismatch("abi2ShapeDigest", hexutil::encode(&mori_r1cs::blake2b::blake2b256(&shape)), self.abi2_shape_digest.clone()));
        }
        Ok(())
    }
}

/// Name and values of the first descriptor field that differs, if any.
pub fn first_difference(expected: &RelationDescriptor, actual: &RelationDescriptor) -> Option<(String, String, String)> {
    let (Ok(serde_json::Value::Object(a)), Ok(serde_json::Value::Object(b))) = (serde_json::to_value(expected), serde_json::to_value(actual)) else {
        return Some(("descriptor".into(), "serializable".into(), "unserializable".into()));
    };
    for (key, va) in &a {
        let vb = b.get(key).cloned().unwrap_or(serde_json::Value::Null);
        if *va != vb {
            return Some((key.clone(), va.to_string(), vb.to_string()));
        }
    }
    for (key, vb) in &b {
        if !a.contains_key(key) {
            return Some((key.clone(), "null".into(), vb.to_string()));
        }
    }
    None
}

/// Refuses unless `actual` equals `expected` in every field.
pub fn require_same(expected: &RelationDescriptor, actual: &RelationDescriptor) -> Result<(), DriverError> {
    match first_difference(expected, actual) {
        None => Ok(()),
        Some((field, e, a)) => Err(DriverError::mismatch(format!("relation descriptor field {field}"), e, a)),
    }
}

/// A relation compiled by the actual compiler for one selector, with its descriptor.
#[derive(Debug, Clone)]
pub struct SelectedRelation {
    selector: RelationSelector,
    relation: CompiledRelation,
    descriptor: RelationDescriptor,
}

impl SelectedRelation {
    pub fn selector(&self) -> &RelationSelector {
        &self.selector
    }
    pub fn relation(&self) -> &CompiledRelation {
        &self.relation
    }
    pub fn descriptor(&self) -> &RelationDescriptor {
        &self.descriptor
    }
}

/// Compiles `source` for `selector` with the actual compiler, then checks optional pins.
pub fn compile_selected(source: &str, selector: &RelationSelector) -> Result<SelectedRelation, DriverError> {
    check_selector(selector)?;
    let options = CompileOptions {
        action: selector.action.clone(),
        cap: CapBinding::NotBound,
        frame: FrameCells::TouchedOnly,
        accounting: AccountingBinding::GrossDebitAndFees,
        compiler: CompilerBuild::embedded().map_err(DriverError::Compile)?,
    };
    let relation =
        mori_r1cs::compile_source::<Fr>(source, options, StatementAdapter::SettlementHashV2).map_err(DriverError::Compile)?;
    let descriptor = RelationDescriptor::of(&relation)?;
    if descriptor.language != selector.language || descriptor.semantics != selector.semantics {
        return Err(DriverError::mismatch(
            "language/semantics",
            format!("{}/{}", selector.language, selector.semantics),
            format!("{}/{}", descriptor.language, descriptor.semantics),
        ));
    }
    if let Some(pin) = &selector.expected_source_sha256 {
        hexutil::decode_exact::<32>(pin, "expectedSourceSha256")?;
        let actual = descriptor.source_sha256.clone().unwrap_or_default();
        if *pin != actual {
            return Err(DriverError::mismatch("source sha256", pin.clone(), actual));
        }
    }
    if let Some(pin) = &selector.expected_program_hash {
        hexutil::decode_exact::<32>(pin, "expectedProgramHash")?;
        if *pin != descriptor.program_hash {
            return Err(DriverError::mismatch("program hash", pin.clone(), descriptor.program_hash.clone()));
        }
    }
    Ok(SelectedRelation { selector: selector.clone(), relation, descriptor })
}

/// `mori-r1cs-matrices/1`: the exact byte rule of the compiler's probe (`circuit.rs:282-305`),
/// recomputed here because the probe function is private. A divergence makes `prove` refuse;
/// it can never make it accept.
pub fn matrix_digest(m: &ConstraintMatrices<Fr>) -> Digest32 {
    let mut bytes = b"mori-r1cs-matrices/1\0".to_vec();
    for n in [m.num_instance_variables, m.num_witness_variables, m.num_constraints] {
        bytes.extend_from_slice(&(n as u64).to_le_bytes());
    }
    for matrix in [&m.a, &m.b, &m.c] {
        bytes.extend_from_slice(&(matrix.len() as u64).to_le_bytes());
        for row in matrix.iter() {
            bytes.extend_from_slice(&(row.len() as u64).to_le_bytes());
            for (coefficient, index) in row.iter() {
                bytes.extend_from_slice(&(*index as u64).to_le_bytes());
                let le = coefficient.into_bigint().to_bytes_le();
                bytes.extend_from_slice(&(le.len() as u64).to_le_bytes());
                bytes.extend_from_slice(&le);
            }
        }
    }
    identity::digest_bytes(&bytes)
}


## S18 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/export.rs
//! Public artefacts: the setup record and the proof export, with strict parsers.
//!
//! Neither document contains proving-key bytes, setup randomness or prover blinding. Every
//! field that duplicates another (identities copied from the setup record, decoded context
//! fields, digests, public inputs, the Cardano section) is re-derived by [`check_export`] and
//! must be byte-equal; a document whose parts disagree is refused, never reconciled.
use crate::claim::{self, MAX_STATEMENT_BYTES, PreparedClaim, StatementView};
use crate::encoding::{self, PROOF_LEN, VK_LEN};
use crate::error::DriverError;
use crate::hexutil;
use crate::input::parse_json;
use crate::relation::RelationDescriptor;
use ark_bls12_381::Bls12_381;
use ark_groth16::{Proof, VerifyingKey};
use mori_r1cs::ContextV2;
use mori_r1cs::abi2::FAMILY_V2;
use mori_r1cs::identity::{self, Digest32};
use serde::{Deserialize, Serialize};

pub const SETUP_FORMAT: &str = "moriarty-k2-groth16-setup/1";
pub const PROOF_FORMAT: &str = "moriarty-k2-groth16-proof/1";
pub const TRUST_PROFILE: &str = "single-party-test-setup/1";
pub const SETUP_NOTICE: &str = "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. Whoever controlled the setup process could forge proofs for this relation. VK provenance is declared by this record; no verifier can check that the VK was generated from r1csSha256.";
pub const CURVE: &str = "BLS12-381";
pub const SNARK: &str = "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)";
pub const ENCODING: &str = "arkworks 0.4 compressed points (Zcash BLS12-381 layout); VK 488 bytes, proof 192 bytes";

/// Fixed non-claims carried by every proof export.
pub const PREMISES: [&str; 6] = [
    "The proof shows, under setup honesty and Groth16 knowledge soundness, that the statement's constant header and body satisfy the named compiled moriarty-core/1 relation.",
    "Context bytes (vk digest, claimant, network, settle hash, thread, consumed outref, seq, catalogue digest, intents digest) are caller-declared premises: bound by the proof, not authenticated by it. S2 recomputes each from its authenticated ScriptContext and datum.",
    "Trust profile single-party-test-setup/1: whoever ran setup could forge proofs for this relation.",
    "Groth16 proofs are malleable: the proof bytes are not a unique claim identifier.",
    "Signed intents, debit authority, frame completeness, physical asset delta, replay and phase outcomes are target obligations outside this proof.",
    "No ledger admission, deployment, settlement, Preprod or Midnight result is implied.",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SetupRecord {
    pub format: String,
    pub trust_profile: String,
    pub notice: String,
    pub relation: RelationDescriptor,
    pub curve: String,
    pub snark: String,
    pub encoding: String,
    /// The 488-byte C2 VK, lowercase hex.
    pub verifying_key_hex: String,
    /// BLAKE2b-256 of the VK bytes (C2 `env.vk_digest`, catalogue `vk_digest`).
    pub vk_digest_blake2b256: String,
    /// Declared custody link to the proving-key file (SHA-256 of its exact bytes).
    pub proving_key_sha256: String,
    pub proving_key_bytes: u64,
    pub driver: String,
}

/// A setup record that passed strict parsing, with its decoded VK.
#[derive(Debug, Clone)]
pub struct ParsedSetup {
    pub record: SetupRecord,
    pub vk: VerifyingKey<Bls12_381>,
    pub vk_bytes: Vec<u8>,
    pub vk_digest: [u8; 32],
    /// SHA-256 of the exact record file bytes.
    pub record_sha256: Digest32,
}

fn same(what: &str, expected: &str, actual: &str) -> Result<(), DriverError> {
    if expected == actual { Ok(()) } else { Err(DriverError::mismatch(what, expected, actual)) }
}

fn fixed(what: &str, got: &str, want: &str) -> Result<(), DriverError> {
    if got == want { Ok(()) } else { Err(DriverError::Unsupported(format!("{what} {got:?}; expected {want:?}"))) }
}

/// Canonical bytes of a public document: Core's ASCII-safe pretty JSON plus a final newline.
pub fn document_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, DriverError> {
    let text = mori_core::json::export(value).map_err(|e| DriverError::Backend(format!("export: {e}")))?;
    let mut out = text.into_bytes();
    out.push(b'\n');
    Ok(out)
}

/// Strict setup-record parsing: profile strings, descriptor syntax, strict VK decoding and the
/// VK digest. `bytes` must be the exact file bytes (their SHA-256 is the record identity).
pub fn parse_setup_record(bytes: &[u8]) -> Result<ParsedSetup, DriverError> {
    let record: SetupRecord = parse_json(bytes, "setup record")?;
    fixed("setup record format", &record.format, SETUP_FORMAT)?;
    fixed("trust profile", &record.trust_profile, TRUST_PROFILE)?;
    fixed("notice", &record.notice, SETUP_NOTICE)?;
    fixed("curve", &record.curve, CURVE)?;
    fixed("snark", &record.snark, SNARK)?;
    fixed("encoding", &record.encoding, ENCODING)?;
    fixed("driver", &record.driver, crate::DRIVER_ID)?;
    record.relation.check_profile()?;
    let vk_bytes = hexutil::decode_exact::<VK_LEN>(&record.verifying_key_hex, "verifyingKeyHex")?.to_vec();
    let vk = encoding::decode_vk(&vk_bytes)?;
    let vk_digest = mori_r1cs::blake2b::blake2b256(&vk_bytes);
    same("vkDigestBlake2b256", &hexutil::encode(&vk_digest), &record.vk_digest_blake2b256)?;
    hexutil::decode_exact::<32>(&record.proving_key_sha256, "provingKeySha256")?;
    if record.proving_key_bytes == 0 {
        return Err(DriverError::Input("provingKeyBytes must be positive".into()));
    }
    Ok(ParsedSetup { record, vk, vk_bytes, vk_digest, record_sha256: identity::digest_bytes(bytes) })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextExport {
    pub vk_digest: String,
    pub claimant_vkh: String,
    pub network_id: u8,
    pub settle_hash: String,
    pub thread_id: String,
    pub consumed_txid: String,
    pub consumed_ix: u16,
    pub seq: u64,
    pub catalogue_digest: String,
    pub intents_digest: String,
}

pub fn context_export(ctx: &ContextV2) -> ContextExport {
    ContextExport {
        vk_digest: hexutil::encode(&ctx.vk_digest),
        claimant_vkh: hexutil::encode(&ctx.claimant_vkh),
        network_id: ctx.network_id,
        settle_hash: hexutil::encode(&ctx.settle_hash),
        thread_id: hexutil::encode(&ctx.thread_id),
        consumed_txid: hexutil::encode(&ctx.consumed_txid),
        consumed_ix: ctx.consumed_ix,
        seq: ctx.seq,
        catalogue_digest: hexutil::encode(&ctx.catalogue_digest),
        intents_digest: hexutil::encode(&ctx.intents_digest),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublicInputsExport {
    /// `int_be(digest[0..16])`, canonical decimal.
    pub x1: String,
    /// `int_be(digest[16..32])`, canonical decimal.
    pub x2: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OutRefExport {
    pub transaction_id: String,
    pub output_index: u16,
}

/// `EnvV2` field values (`types.ak:227-238`) except `expiry`, which the transaction builder
/// chooses and the proof does not bind. The driver produces no Plutus Data/CBOR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EnvV2Export {
    pub version: u8,
    pub claim_kind: u8,
    pub family_id: String,
    pub predicate_digest: String,
    pub vk_digest: String,
    pub claimant_vkh: String,
    pub settle_outref: OutRefExport,
    pub statement_digest: String,
}

/// The digests of a catalogue `ActionEntryV2` (`types.ak:73-82`); `advancers` is the
/// catalogue author's choice and is not produced here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActionEntryExport {
    pub name: String,
    pub index: u16,
    pub predicate_digest: String,
    pub vk_digest: String,
    pub shape_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CardanoExport {
    pub env_v2: EnvV2Export,
    pub catalogue_action_entry: ActionEntryExport,
    /// Domain `/2` intent `authority.target_hash`: `pred:vk:shape` (194 bytes, ABI-V2.md 7.2).
    pub intent_target_hash_v2: String,
    /// `cardano-plutus-v3:` + hex(settle hash) from the declared context (S2 derives its own).
    pub intent_anchor: String,
}

pub fn cardano_export(desc: &RelationDescriptor, vk_digest: &[u8; 32], ctx: &ContextV2, statement_digest: &[u8; 32]) -> CardanoExport {
    let vk = hexutil::encode(vk_digest);
    CardanoExport {
        env_v2: EnvV2Export {
            version: 2,
            claim_kind: 0,
            family_id: hexutil::encode(FAMILY_V2),
            predicate_digest: desc.predicate_digest.clone(),
            vk_digest: vk.clone(),
            claimant_vkh: hexutil::encode(&ctx.claimant_vkh),
            settle_outref: OutRefExport { transaction_id: hexutil::encode(&ctx.consumed_txid), output_index: ctx.consumed_ix },
            statement_digest: hexutil::encode(statement_digest),
        },
        catalogue_action_entry: ActionEntryExport {
            name: desc.action.clone(),
            index: desc.action_index,
            predicate_digest: desc.predicate_digest.clone(),
            vk_digest: vk.clone(),
            shape_digest: desc.abi2_shape_digest.clone(),
        },
        intent_target_hash_v2: format!("{}:{}:{}", desc.predicate_digest, vk, desc.abi2_shape_digest),
        intent_anchor: format!("cardano-plutus-v3:{}", hexutil::encode(&ctx.settle_hash)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofExport {
    pub format: String,
    /// SHA-256 of the exact setup-record file bytes (declared link, checked by `verify`).
    pub setup_record_sha256: String,
    pub program_hash: String,
    pub source_sha256: Option<String>,
    pub action: String,
    pub action_index: u16,
    pub compiler_build_digest: String,
    pub r1cs_sha256: String,
    pub predicate_digest: String,
    pub abi2_shape_digest: String,
    pub vk_digest_blake2b256: String,
    pub context: ContextExport,
    pub statement_hex: String,
    pub statement_digest_blake2b256: String,
    pub public_inputs: PublicInputsExport,
    /// The 192-byte C2 proof, lowercase hex (C2 `Issue { vk, proof }` takes the record's VK).
    pub proof_hex: String,
    pub cardano: CardanoExport,
    pub premises: Vec<String>,
}

pub fn parse_proof_export(bytes: &[u8]) -> Result<ProofExport, DriverError> {
    let e: ProofExport = parse_json(bytes, "proof export")?;
    fixed("proof export format", &e.format, PROOF_FORMAT)?;
    Ok(e)
}

/// Builds the export of a proof. The caller runs [`check_export`] on its serialized bytes.
pub fn build_export(setup: &ParsedSetup, claim: &PreparedClaim, proof: &Proof<Bls12_381>) -> Result<ProofExport, DriverError> {
    let desc = &setup.record.relation;
    Ok(ProofExport {
        format: PROOF_FORMAT.into(),
        setup_record_sha256: setup.record_sha256.hex(),
        program_hash: desc.program_hash.clone(),
        source_sha256: desc.source_sha256.clone(),
        action: desc.action.clone(),
        action_index: desc.action_index,
        compiler_build_digest: desc.compiler_build_digest.clone(),
        r1cs_sha256: desc.r1cs_sha256.clone(),
        predicate_digest: desc.predicate_digest.clone(),
        abi2_shape_digest: desc.abi2_shape_digest.clone(),
        vk_digest_blake2b256: hexutil::encode(&setup.vk_digest),
        context: context_export(&claim.context),
        statement_hex: hexutil::encode(&claim.statement),
        statement_digest_blake2b256: hexutil::encode(&claim.statement_digest),
        public_inputs: PublicInputsExport { x1: claim.public_inputs[0].to_string(), x2: claim.public_inputs[1].to_string() },
        proof_hex: hexutil::encode(&encoding::encode_proof(proof)?),
        cardano: cardano_export(desc, &setup.vk_digest, &claim.context, &claim.statement_digest),
        premises: PREMISES.iter().map(|s| (*s).to_string()).collect(),
    })
}

/// Everything a verifier needs after the export passed every non-cryptographic check.
#[derive(Debug, Clone)]
pub struct CheckedExport {
    pub proof: Proof<Bls12_381>,
    pub public_inputs: [u128; 2],
    pub statement: Vec<u8>,
    pub statement_digest: [u8; 32],
    pub view: StatementView,
}

/// Every non-pairing check of a proof export against a parsed setup record.
pub fn check_export(setup: &ParsedSetup, e: &ProofExport) -> Result<CheckedExport, DriverError> {
    fixed("proof export format", &e.format, PROOF_FORMAT)?;
    let desc = &setup.record.relation;
    same("setupRecordSha256", &setup.record_sha256.hex(), &e.setup_record_sha256)?;
    same("programHash", &desc.program_hash, &e.program_hash)?;
    same("sourceSha256", &format!("{:?}", desc.source_sha256), &format!("{:?}", e.source_sha256))?;
    same("action", &desc.action, &e.action)?;
    same("actionIndex", &desc.action_index.to_string(), &e.action_index.to_string())?;
    same("compilerBuildDigest", &desc.compiler_build_digest, &e.compiler_build_digest)?;
    same("r1csSha256", &desc.r1cs_sha256, &e.r1cs_sha256)?;
    same("predicateDigest", &desc.predicate_digest, &e.predicate_digest)?;
    same("abi2ShapeDigest", &desc.abi2_shape_digest, &e.abi2_shape_digest)?;
    same("vkDigestBlake2b256", &hexutil::encode(&setup.vk_digest), &e.vk_digest_blake2b256)?;

    let statement = hexutil::decode_bounded(&e.statement_hex, MAX_STATEMENT_BYTES, "statementHex")?;
    let view = claim::parse_statement(&statement)?;
    claim::check_statement_against(desc, &setup.vk_digest, &view)?;
    if context_export(&view.context) != e.context {
        return Err(DriverError::mismatch("context fields", "the statement's context bytes", "different declared fields"));
    }
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    same("statementDigestBlake2b256", &hexutil::encode(&statement_digest), &e.statement_digest_blake2b256)?;
    let public_inputs = claim::digest_halves(&statement_digest);
    let x1 = hexutil::parse_u128_decimal(&e.public_inputs.x1, "x1")?;
    let x2 = hexutil::parse_u128_decimal(&e.public_inputs.x2, "x2")?;
    same("x1", &public_inputs[0].to_string(), &x1.to_string())?;
    same("x2", &public_inputs[1].to_string(), &x2.to_string())?;
    let proof_bytes = hexutil::decode_exact::<PROOF_LEN>(&e.proof_hex, "proofHex")?;
    let proof = encoding::decode_proof(&proof_bytes)?;
    if cardano_export(desc, &setup.vk_digest, &view.context, &statement_digest) != e.cardano {
        return Err(DriverError::mismatch("cardano section", "values derived from the statement and record", "different values"));
    }
    if e.premises.len() != PREMISES.len() || e.premises.iter().zip(PREMISES).any(|(a, b)| a != b) {
        return Err(DriverError::mismatch("premises", "the fixed premise list", "a different list"));
    }
    Ok(CheckedExport { proof, public_inputs, statement, statement_digest, view })
}


## S19 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/lib.rs
//! UNADOPTED K2 driver: one callable path from an admitted `moriarty/1` source to a Groth16
//! proof of the actual compiled `SettlementHashV2` relation, and to the public artefacts the
//! Cardano C2/S2 consumers read.
//!
//! Source only: nothing in this crate has been compiled, tested or run.
//!
//! Pipeline, the same for every admitted program:
//!
//! 1. [`input`]: bounded reads and strict JSON (Core's duplicate-key/depth/node limits, then
//!    `deny_unknown_fields` DTOs).
//! 2. [`relation`]: `mori_r1cs::compile_source::<Fr>` with the only options SettlementHashV2
//!    admits and the embedded compiler build. Every identity comes from that run.
//! 3. [`claim`]: a transition is *proposed* (Core reference or explicit records) and the
//!    statement is built by the compiler's own `statement_v2`.
//! 4. [`setup`] / [`prove`] / [`verify`]: separate entry points. Setup and prove take an
//!    explicit RNG; verify takes none and never generates keys. Prove refuses unless the
//!    compiled relation itself is satisfied and its prove-mode matrices equal the probe's.
//! 5. [`export`] and [`encoding`]: canonical public records and strict parsers.
//! 6. [`fixed`] and [`inspect`] (K4H successor): the relation's fixed setup-mode matrices, the
//!    row-by-row decision of any full assignment against them, and the ordinary statement
//!    receipt that exposes decoded statement values. No key, proof or randomness.
//!
//! Trust boundaries (see `DESIGN.md` 3.3): the proof establishes, under setup honesty and
//! Groth16 knowledge soundness, that the statement's constant header and body satisfy this
//! relation. Context bytes are bound but not authenticated. VK provenance is declared by the
//! setup record. Nothing here is a ledger, admission, settlement or Preprod result.
#![forbid(unsafe_code)]

pub mod claim;
pub mod encoding;
pub mod error;
pub mod export;
pub mod fixed;
pub mod hexutil;
pub mod input;
pub mod inspect;
pub mod prove;
pub mod relation;
pub mod setup;
pub mod verify;

pub use claim::{PreparedClaim, StatementView, prepare_claim};
pub use error::DriverError;
pub use export::{ParsedSetup, ProofExport, SetupRecord};
pub use input::{ContextInput, ProveRequest, RelationSelector, TransitionInput};
pub use relation::{RelationDescriptor, SelectedRelation, compile_selected};

/// Declared identity of this driver, written into every setup record. It is a name, not a
/// hash: the root pins these bytes by hash from outside. `+k4h1` marks the K4H successor
/// bytes (fixed-matrix tier, receipts, digest-bound setup); the package version and the lock
/// are unchanged.
pub const DRIVER_ID: &str = "mori-k2-driver/0.1.0-k2s1+k4h1";

/// The only statement adapter this driver drives.
pub const ADAPTER: &str = "SettlementHashV2";


## S20 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/hexutil.rs
//! Strict lowercase hexadecimal and canonical decimal helpers.
use crate::error::DriverError;

/// Lowercase hex through Core's encoder (the same one every compiler digest uses).
pub fn encode(bytes: &[u8]) -> String {
    mori_core::intent::hex(bytes)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

/// Decodes lowercase hex of at most `max_bytes` bytes. Uppercase, odd length, whitespace or a
/// `0x` prefix is refused.
pub fn decode_bounded(text: &str, max_bytes: usize, what: &str) -> Result<Vec<u8>, DriverError> {
    let raw = text.as_bytes();
    if raw.len() % 2 != 0 {
        return Err(DriverError::Input(format!("{what}: hex has odd length {}", raw.len())));
    }
    if raw.len() / 2 > max_bytes {
        return Err(DriverError::Input(format!("{what}: {} bytes exceed the {max_bytes}-byte bound", raw.len() / 2)));
    }
    let mut out = Vec::with_capacity(raw.len() / 2);
    for pair in raw.chunks(2) {
        match (nibble(pair[0]), nibble(pair[1])) {
            (Some(hi), Some(lo)) => out.push((hi << 4) | lo),
            _ => return Err(DriverError::Input(format!("{what}: hex must be lowercase [0-9a-f]"))),
        }
    }
    Ok(out)
}

/// Decodes exactly `N` bytes of lowercase hex.
pub fn decode_exact<const N: usize>(text: &str, what: &str) -> Result<[u8; N], DriverError> {
    if text.len() != 2 * N {
        return Err(DriverError::Input(format!("{what}: expected {} hex characters, got {}", 2 * N, text.len())));
    }
    let bytes = decode_bounded(text, N, what)?;
    bytes.try_into().map_err(|_| DriverError::Input(format!("{what}: expected {N} bytes")))
}

/// Canonical unsigned decimal of a `u128`: digits only, no sign, no leading zero except "0".
pub fn parse_u128_decimal(text: &str, what: &str) -> Result<u128, DriverError> {
    let ok = !text.is_empty()
        && text.len() <= 39
        && text.bytes().all(|b| b.is_ascii_digit())
        && !(text.len() > 1 && text.starts_with('0'));
    if !ok {
        return Err(DriverError::Input(format!("{what}: expected a canonical unsigned decimal")));
    }
    text.parse::<u128>().map_err(|_| DriverError::Input(format!("{what}: decimal exceeds u128")))
}


## S21 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-r2-opus-contract-20261016/PROCESS.json
{
  "reviewToken": "current-cardano-c2-consumer-profile-r2-opus-contract-20261016",
  "requestedModel": "claude-opus-5-5",
  "requestedEffort": "high",
  "scope": "New CP profile/refusal contract and test revision following completed independent review R01-R03; no implementation or native actions.",
  "maximumAttempts": 1,
  "retries": 0,
  "wallSeconds": 1200,
  "status": "terminal",
  "startedAt": "2026-10-16T21:59:15.935388+00:00",
  "verifiedInputsBefore": 12,
  "promptBytes": 121082,
  "actualEffort": "not independently observed",
  "pid": 3311544,
  "exitCode": 124,
  "endedAt": "2026-10-16T22:19:16.591765+00:00",
  "schemaValid": false,
  "identityVerified": false,
  "inputFilesChangedAfter": [],
  "publicResultUnavailableReasonType": "JSONDecodeError"
}


## S22 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/cardano-preprod-host/host_codec.py
"""Provisional pure PlutusData encoder for the frozen serialise_data profile."""

_MAX_DEPTH = 128
_MAX_VISITS = 65536
_MAX_LEAF_BYTES = 65536
_MAX_OUTPUT_BYTES = 1048576
_MAX_UINT64 = (1 << 64) - 1


def _append(out, fragment):
    if len(fragment) > _MAX_OUTPUT_BYTES - len(out):
        raise ValueError("encoded data exceeds the output bound")
    out.extend(fragment)


def _head(major, value):
    if value < 24:
        return bytes((major | value,))
    if value <= 255:
        width, marker = 1, 24
    elif value <= 65535:
        width, marker = 2, 25
    elif value <= 4294967295:
        width, marker = 4, 26
    else:
        width, marker = 8, 27
    return bytes((major | marker,)) + int.to_bytes(value, width, "big")


def _write_bytes(out, value):
    size = len(value)
    if size > _MAX_LEAF_BYTES:
        raise ValueError("bytes or integer magnitude exceeds the leaf bound")
    if size <= 64:
        _append(out, _head(0x40, size))
        _append(out, value)
        return
    _append(out, b"\x5f")
    for offset in range(0, size, 64):
        chunk = value[offset:offset + 64]
        _append(out, _head(0x40, len(chunk)))
        _append(out, chunk)
    _append(out, b"\xff")


def _encode(data):
    out = bytearray()
    visits = 0
    # A continuation retains only one next child, even for a broad container.
    # Kind 0 visits a node; kinds 1 and 2 resume an array or map respectively.
    pending = [(0, data, 1, 0)]
    while pending:
        kind, value, depth, index = pending.pop()
        if kind == 1:
            if index == len(value):
                _append(out, b"\xff")
            else:
                pending.append((1, value, depth, index + 1))
                pending.append((0, value[index], depth, 0))
            continue
        if kind == 2:
            if index < 2 * len(value):
                pair = value[index // 2]
                if type(pair) is not tuple or len(pair) != 2:
                    raise ValueError("map entries must be exact pairs")
                pending.append((2, value, depth, index + 1))
                pending.append((0, pair[index % 2], depth, 0))
            continue

        visits += 1
        if visits > _MAX_VISITS:
            raise ValueError("data exceeds the expanded node visit bound")
        if depth > _MAX_DEPTH:
            raise ValueError("data exceeds the depth bound")
        if type(value) is not tuple or not value or type(value[0]) is not str:
            raise ValueError("a data node must be an exact tagged tuple")

        tag = value[0]
        if tag == "int":
            if len(value) != 2 or type(value[1]) is not int:
                raise ValueError("an integer node requires an exact integer")
            integer = value[1]
            if int.bit_length(integer) > _MAX_LEAF_BYTES * 8 + 1:
                raise ValueError("integer magnitude exceeds the leaf bound")
            magnitude = integer if integer >= 0 else -1 - integer
            if magnitude <= _MAX_UINT64:
                _append(out, _head(0x00 if integer >= 0 else 0x20, magnitude))
            else:
                size = (int.bit_length(magnitude) + 7) // 8
                if size > _MAX_LEAF_BYTES:
                    raise ValueError("integer magnitude exceeds the leaf bound")
                _append(out, b"\xc2" if integer >= 0 else b"\xc3")
                _write_bytes(out, int.to_bytes(magnitude, size, "big"))
            continue
        if tag == "bytes":
            if len(value) != 2 or type(value[1]) is not bytes:
                raise ValueError("a bytes node requires exact bytes")
            _write_bytes(out, value[1])
            continue
        if tag == "map":
            if len(value) != 2 or type(value[1]) is not list:
                raise ValueError("a map node requires an exact list of pairs")
            pairs = value[1]
            if len(pairs) > (_MAX_VISITS - visits) // 2:
                raise ValueError("map exceeds the expanded node visit bound")
            _append(out, _head(0xa0, len(pairs)))
            if pairs:
                pending.append((2, pairs, depth + 1, 0))
            continue
        if tag == "list":
            if len(value) != 2 or type(value[1]) is not list:
                raise ValueError("a list node requires an exact list")
            children = value[1]
        elif tag == "constr":
            if (len(value) != 3 or type(value[1]) is not int
                    or not 0 <= value[1] <= _MAX_UINT64
                    or type(value[2]) is not list):
                raise ValueError("a constructor requires a uint64 index and exact list")
            constructor = value[1]
            children = value[2]
            if constructor <= 6:
                _append(out, _head(0xc0, 121 + constructor))
            elif constructor <= 127:
                _append(out, _head(0xc0, 1280 + constructor - 7))
            else:
                _append(out, _head(0xc0, 102))
                _append(out, b"\x82")
                _append(out, _head(0x00, constructor))
        else:
            raise ValueError("unknown data tag")

        if len(children) > _MAX_VISITS - visits:
            raise ValueError("array exceeds the expanded node visit bound")
        if children:
            _append(out, b"\x9f")
            pending.append((1, children, depth + 1, 0))
        else:
            _append(out, b"\x80")
    return bytes(out)


def encode_data(data):
    """Return complete CBOR bytes, or reject data outside the frozen contract."""
    # Environmental failures such as MemoryError propagate unconverted (contract I2).
    return _encode(data)


# Provisional Rust request-wire APIs. These validate structure only; Core/Rust
# owns program semantics, and caller-supplied context is not authenticated.


def _wire_keys(value, path):
    if type(value) is not dict:
        raise ValueError(path + ": expected exact object")
    # Iterating an exact dict does not hash or compare its stored keys.
    for key in value:
        if type(key) is not str:
            raise ValueError(path + ": expected exact string keys")


def _wire_record(value, path, fields, optional=()):
    _wire_keys(value, path)
    # Refuse all unknown fields before looking at any of this object's values.
    for key in value:
        if key not in fields:
            raise ValueError(path + ": expected closed schema object")
    for key in fields:
        if key not in optional and key not in value:
            raise ValueError(path + "." + key + ": expected required field")
    return {key: fields[key](item, path + "." + key)
            for key, item in value.items()}


def _wire_tagged(value, path, tag, variants):
    _wire_keys(value, path)
    # Determine the admissible complete field shapes from keys alone. This
    # rejects unknown fields without visiting values, including the tag value.
    names = []
    fields = None
    for name, schema in variants.items():
        if len(value) == len(schema) and all(key in schema for key in value):
            names.append(name)
            fields = schema
    if fields is None:
        raise ValueError(path + ": expected closed tagged schema object")
    kind = value[tag]
    if type(kind) is not str or kind not in names:
        raise ValueError(path + "." + tag + ": expected schema tag string")
    return _wire_record(value, path, variants[kind])


def _wire_text(value, path):
    if type(value) is not str:
        raise ValueError(path + ": expected exact UTF-8 string")
    try:
        value.encode("utf-8")
    except UnicodeEncodeError:
        raise ValueError(path + ": expected exact UTF-8 string") from None
    return value


def _wire_action(value, path):
    if type(value) is not str or not 1 <= len(value) <= 1024:
        raise ValueError(path + ": expected action string of 1..1024 UTF-8 bytes")
    try:
        size = len(value.encode("utf-8"))
    except UnicodeEncodeError:
        raise ValueError(path + ": expected action string of 1..1024 UTF-8 bytes") from None
    if size > 1024:
        raise ValueError(path + ": expected action string of 1..1024 UTF-8 bytes")
    return value


def _wire_uint(value, path):
    if (type(value) is not str or not 1 <= len(value) <= 39
            or any(char < "0" or char > "9" for char in value)
            or (len(value) > 1 and value[0] == "0")):
        raise ValueError(path + ": expected canonical u128 decimal string")
    if int(value) > (1 << 128) - 1:
        raise ValueError(path + ": expected canonical u128 decimal string")
    return value


def _wire_bool(value, path):
    if type(value) is not bool:
        raise ValueError(path + ": expected exact boolean")
    return value


def _wire_integer(value, path, maximum):
    if type(value) is not int or not 0 <= value <= maximum:
        raise ValueError(path + ": expected unsigned integer in schema range")
    return value


def _wire_u8(value, path):
    return _wire_integer(value, path, 255)


def _wire_u16(value, path):
    return _wire_integer(value, path, 65535)


def _wire_u64(value, path):
    return _wire_integer(value, path, (1 << 64) - 1)


def _wire_hex(value, path, size):
    if (type(value) is not str or len(value) != size * 2
            or any(char not in "0123456789abcdef" for char in value)):
        raise ValueError(path + ": expected exact lowercase hexadecimal string")
    return value


def _wire_hex28(value, path):
    return _wire_hex(value, path, 28)


def _wire_hex32(value, path):
    return _wire_hex(value, path, 32)


def _wire_optional_hex32(value, path):
    if value is None:
        return None
    return _wire_hex32(value, path)


def _wire_value(value, path):
    return _wire_tagged(value, path, "type", {
        "UInt": {"type": _wire_text, "value": _wire_uint},
        "Bool": {"type": _wire_text, "value": _wire_bool},
    })


def _wire_map(value, path):
    _wire_keys(value, path)
    for key in value:
        _wire_text(key, path + ".*")
    return {key: _wire_value(item, path + ".*")
            for key, item in value.items()}


def _wire_array(value, path, item_validator):
    if type(value) is not list:
        raise ValueError(path + ": expected exact array")
    return [item_validator(item, path + "[]") for item in value]


def _wire_asset(value, path):
    return _wire_record(value, path, {
        "network": _wire_text,
        "issuer": _wire_text,
        "reference": _wire_text,
        "scale": _wire_u8,
    })


def _wire_balance(value, path):
    return _wire_record(value, path, {
        "account": _wire_text,
        "asset": _wire_asset,
        "amount": _wire_uint,
    })


def _wire_liability(value, path):
    return _wire_record(value, path, {
        "name": _wire_text,
        "asset": _wire_asset,
        "debtor": _wire_text,
        "creditor": _wire_text,
        "amount": _wire_uint,
    })


def _wire_balances(value, path):
    return _wire_array(value, path, _wire_balance)


def _wire_liabilities(value, path):
    return _wire_array(value, path, _wire_liability)


def _wire_snapshot(value, path):
    return _wire_record(value, path, {
        "state": _wire_map,
        "balances": _wire_balances,
        "liabilities": _wire_liabilities,
    })


def _wire_scenario(value, path):
    return _wire_record(value, path, {
        "state": _wire_map,
        "inputs": _wire_map,
        "balances": _wire_balances,
        "liabilities": _wire_liabilities,
    })


def _wire_effect(value, path):
    movement = {
        "kind": _wire_text,
        "asset": _wire_asset,
        "from": _wire_text,
        "to": _wire_text,
        "amount": _wire_uint,
    }
    return _wire_tagged(value, path, "kind", {
        "State": {
            "kind": _wire_text,
            "name": _wire_text,
            "before": _wire_value,
            "after": _wire_value,
        },
        "Transfer": movement,
        "Fee": movement,
        "Liability": {
            "kind": _wire_text,
            "name": _wire_text,
            "asset": _wire_asset,
            "debtor": _wire_text,
            "creditor": _wire_text,
            "before": _wire_uint,
            "after": _wire_uint,
        },
        "Emit": {"kind": _wire_text, "name": _wire_text, "value": _wire_value},
    })


def _wire_effects(value, path):
    return _wire_array(value, path, _wire_effect)


def _wire_transition(value, path):
    return _wire_tagged(value, path, "kind", {
        "CoreReference": {"kind": _wire_text, "scenario": _wire_scenario},
        "Records": {
            "kind": _wire_text,
            "inputs": _wire_map,
            "pre": _wire_snapshot,
            "post": _wire_snapshot,
            "effects": _wire_effects,
        },
    })


def _wire_context(value, path):
    return _wire_record(value, path, {
        "claimantVkh": _wire_hex28,
        "networkId": _wire_u8,
        "settleHash": _wire_hex28,
        "threadId": _wire_hex32,
        "consumedTxid": _wire_hex32,
        "consumedIx": _wire_u16,
        "seq": _wire_u64,
        "catalogueDigest": _wire_hex32,
        "intentsDigest": _wire_hex32,
        "vkDigest": _wire_optional_hex32,
    }, ("vkDigest",))


def _wire_selector(value, path):
    result = _wire_record(value, path, {
        "format": _wire_text,
        "language": _wire_text,
        "semantics": _wire_text,
        "action": _wire_action,
        "adapter": _wire_text,
        "cap": _wire_text,
        "accounting": _wire_text,
        "frame": _wire_text,
        "expectedSourceSha256": _wire_optional_hex32,
        "expectedProgramHash": _wire_optional_hex32,
    }, ("expectedSourceSha256", "expectedProgramHash"))
    expected = {
        "format": "moriarty-k2-relation-selector/1",
        "language": "moriarty/1",
        "semantics": "moriarty-core/1",
        "adapter": "SettlementHashV2",
        "cap": "NotBound",
        "accounting": "GrossDebitAndFees",
        "frame": "TouchedOnly",
    }
    for key, literal in expected.items():
        if result[key] != literal:
            raise ValueError(path + "." + key + ": expected fixed schema string")
    return result


def _wire_request(value, path):
    result = _wire_record(value, path, {
        "format": _wire_text,
        "selector": _wire_selector,
        "transition": _wire_transition,
        "context": _wire_context,
    })
    if result["format"] != "moriarty-k2-prove-request/1":
        raise ValueError(path + ".format: expected fixed schema string")
    return result


def build_prove_request(action, transition, context, expected_source_sha256=None,
                        expected_program_hash=None):
    """Return a detached structural Rust request; evaluate no financial rules."""
    selector = {
        "format": "moriarty-k2-relation-selector/1",
        "language": "moriarty/1",
        "semantics": "moriarty-core/1",
        "action": _wire_action(action, "request.selector.action"),
        "adapter": "SettlementHashV2",
        "cap": "NotBound",
        "accounting": "GrossDebitAndFees",
        "frame": "TouchedOnly",
    }
    if expected_source_sha256 is not None:
        selector["expectedSourceSha256"] = _wire_hex32(
            expected_source_sha256, "request.selector.expectedSourceSha256")
    if expected_program_hash is not None:
        selector["expectedProgramHash"] = _wire_hex32(
            expected_program_hash, "request.selector.expectedProgramHash")
    request = {
        "format": "moriarty-k2-prove-request/1",
        "selector": selector,
        "transition": _wire_transition(transition, "request.transition"),
        "context": _wire_context(context, "request.context"),
    }
    validate_prove_request(request)
    return request


def validate_prove_request(request):
    """Check wire structure only, without mutating or authenticating input."""
    _wire_request(request, "request")
    return True


def validate_context_input(context):
    """Return detached context labelled as caller supplied and unauthenticated."""
    result = _wire_context(context, "context")
    result["authentication"] = "CALLER_SUPPLIED_UNAUTHENTICATED"
    return result


# Provisional ABI v2 statement APIs. These check fixed-header framing only: the
# body is opaque, context fields are caller supplied and unauthenticated, and no
# proof, VK, descriptor, program or ledger value is verified.

import hashlib

_STMT_TAG = b"MRG16-PVC-STMT-2"
_FAMILY_V2 = b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2"
_HEADER_LEN = 343
_MAX_STATEMENT_BYTES = 262144
# Returned fields: key, offset, width, kind (claim.rs offsets, abi2.rs statement_bytes).
_STATEMENT_FIELDS = (
    ("predicateDigest", 48, 32, "hex"),
    ("shapeDigest", 80, 32, "hex"),
    ("vkDigest", 112, 32, "hex"),
    ("claimantVkh", 144, 28, "hex"),
    ("networkId", 172, 1, "int"),
    ("settleHash", 173, 28, "hex"),
    ("threadId", 201, 32, "hex"),
    ("consumedTxid", 233, 32, "hex"),
    ("consumedIx", 265, 2, "int"),
    ("seq", 267, 8, "int"),
    ("catalogueDigest", 275, 32, "hex"),
    ("intentsDigest", 307, 32, "hex"),
    ("bodyLen", 339, 4, "int"),
)


def _admit_statement(statement):
    # A1 comes first: exact bytes is immutable and has no caller-defined hooks,
    # so nothing about a refused argument is touched (contract SH-05).
    if type(statement) is not bytes:
        raise ValueError("statement: expected exact bytes")
    size = len(statement)
    if not _HEADER_LEN <= size <= _MAX_STATEMENT_BYTES:
        raise ValueError("statement: length outside the ABI v2 bounds")
    if statement[0:16] != _STMT_TAG:
        raise ValueError("statement: tag is not MRG16-PVC-STMT-2")
    if statement[16:48] != _FAMILY_V2:
        raise ValueError("statement: family is not MRG16-PVC-FAM2-G16-BLS-ARK04-HL2")
    if int.from_bytes(statement[339:343], "big") + _HEADER_LEN != size:
        raise ValueError("statement: length differs from 343 + bodyLen")


def _statement_digest(statement):
    _admit_statement(statement)
    # Unkeyed 32-byte BLAKE2b of the whole admitted statement; no key, salt or person.
    return hashlib.blake2b(statement, digest_size=32).digest()


def parse_statement_header(statement):
    """Return a new dict of the header fields as found; authenticate and judge none."""
    _admit_statement(statement)
    result = {}
    for name, offset, width, kind in _STATEMENT_FIELDS:
        field = statement[offset:offset + width]
        if kind == "hex":
            result[name] = field.hex()
        else:
            result[name] = int.from_bytes(field, "big")
    # The body is opaque: admission checked only its length.
    result["body"] = statement[_HEADER_LEN:]
    return result


def statement_digest_hex(statement):
    """Return the lowercase hex BLAKE2b-256 digest of an admitted statement."""
    return _statement_digest(statement).hex()


def statement_public_inputs(statement):
    """Return (x1, x2): decimal big-endian 128-bit halves of the statement digest."""
    digest = _statement_digest(statement)
    return (str(int.from_bytes(digest[0:16], "big")),
            str(int.from_bytes(digest[16:32], "big")))


# Provisional structural Data factories (DF-01..DF-15). All inputs are caller
# supplied and unauthenticated. Opaque bytes carry no VK, proof or ledger judgment.


def _data_bytes(value, minimum, maximum, path):
    if type(value) is not bytes or not minimum <= len(value) <= maximum:
        raise ValueError(path + ": expected exact bytes within the length bounds")


def _data_integer(value, minimum, maximum, path):
    if type(value) is not int or not minimum <= value <= maximum:
        raise ValueError(path + ": expected exact integer within the range")


def build_value_data(lovelace, tokens):
    """Return a fresh map of policy maps in the supplied strictly ascending order."""
    if lovelace is not None:
        _data_integer(lovelace, 0, 18446744073709551615, "lovelace")
    if type(tokens) is not list:
        raise ValueError("tokens: expected exact list")
    if len(tokens) > 256:
        raise ValueError("tokens: expected at most 256 elements")

    entries = []
    if lovelace is not None:
        entries.append((("bytes", b""), ("map", [(("bytes", b""), ("int", lovelace))])))
    previous = None
    current_policy = None
    names = None
    for token in tokens:
        if type(token) is not tuple or len(token) != 3:
            raise ValueError("tokens[]: expected exact tuple of length 3")
        policy, name, quantity = token
        _data_bytes(policy, 28, 28, "tokens[].policy")
        _data_bytes(name, 0, 32, "tokens[].name")
        _data_integer(quantity, -18446744073709551615, 18446744073709551615,
                      "tokens[].quantity")
        if quantity == 0:
            raise ValueError("tokens[].quantity: expected nonzero integer")
        pair = (policy, name)
        if previous is not None and pair <= previous:
            raise ValueError("tokens.order: expected strictly ascending policy and name")
        if policy != current_policy:
            names = []
            entries.append((("bytes", policy), ("map", names)))
            current_policy = policy
        names.append((("bytes", name), ("int", quantity)))
        previous = pair
    return ("map", entries)


def build_output_reference(transaction_id, output_index):
    """Return a fresh constructor-0 output reference after structural checks."""
    _data_bytes(transaction_id, 32, 32, "transaction_id")
    _data_integer(output_index, 0, 65535, "output_index")
    return ("constr", 0, [("bytes", transaction_id), ("int", output_index)])


def build_env_data(predicate_digest, vk_digest, claimant_vkh, settle_transaction_id,
                   settle_output_index, statement_digest, expiry):
    """Return a fresh fixed-version envelope without authenticating its fields."""
    _data_bytes(predicate_digest, 32, 32, "predicate_digest")
    _data_bytes(vk_digest, 32, 32, "vk_digest")
    _data_bytes(claimant_vkh, 28, 28, "claimant_vkh")
    _data_bytes(settle_transaction_id, 32, 32, "settle_transaction_id")
    _data_integer(settle_output_index, 0, 65535, "settle_output_index")
    _data_bytes(statement_digest, 32, 32, "statement_digest")
    _data_integer(expiry, 0, 18446744073709551615, "expiry")
    return ("constr", 0, [
        ("int", 2),
        ("int", 0),
        ("bytes", b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2"),
        ("bytes", predicate_digest),
        ("bytes", vk_digest),
        ("bytes", claimant_vkh),
        ("constr", 0, [("bytes", settle_transaction_id), ("int", settle_output_index)]),
        ("bytes", statement_digest),
        ("int", expiry),
    ])


def build_c2_mint_data(kind, vk=None, proof=None):
    """Return fresh Issue or Burn Data, treating Issue payloads as opaque bytes."""
    if type(kind) is not str or kind not in ("Issue", "Burn"):
        raise ValueError("kind: expected exact Issue or Burn string")
    if kind == "Issue":
        _data_bytes(vk, 1, 65536, "vk")
        _data_bytes(proof, 1, 65536, "proof")
        return ("constr", 0, [("bytes", vk), ("bytes", proof)])
    if vk is not None:
        raise ValueError("vk: expected None for Burn")
    if proof is not None:
        raise ValueError("proof: expected None for Burn")
    return ("constr", 1, [])


## S23 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/test_c2_issue_artifacts.py
'''Ordinary tests for the proposed c2_issue_artifacts consumer (contract CI01-CI12).

Author-proposed; RED and GREEN unobserved. Run only as
python3 -I -B test_c2_issue_artifacts.py inside the read-only dedicated
directory of C2-ISSUE-ARTIFACT-CONTRACT.md section 7. Effects: bounded
O_NOFOLLOW reads of sibling files, execution of the pinned host bytes and of
the consumer bytes read once, process-local sys.modules['host_codec'],
unittest output and one stderr line naming the consumer SHA-256. SHA-256 is
provenance only: every expected digest, token and CBOR byte is a public
literal, or literal framing around pinned fixture bytes, never a call to a
production hash or encoder.
'''
import copy
import hashlib
import json
import os
import stat
import sys
import types
import unittest

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))

HOST_FILE, HOST_SIZE = 'host_codec.py', 23252
HOST_SHA256 = '3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459'
PROOF_FILE, PROOF_SIZE = 'K6-PUBLIC-PROOF.json', 5800
PROOF_SHA256 = '930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b'
SETUP_FILE = 'K2-PUBLIC-SETUP-RECORD.json'
SETUP_SHA256 = '86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b'
CONSUMER_FILE = 'c2_issue_artifacts.py'
HOST_APIS = ('build_env_data', 'build_c2_mint_data', 'build_value_data', 'encode_data',
             'parse_statement_header', 'statement_digest_hex', 'statement_public_inputs')

FAMILY = b'MRG16-PVC-FAM2-G16-BLS-ARK04-HL2'
PRED = '0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd'
VKD = 'd19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd'
STMTD = 'c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc'
# Historical K10 R2 builder output (generated manifest lines 32-42 and 104-135):
# a reproduction vector for the captured K6 context, not an independent oracle.
HIST_EXPIRY = 1893456000000
HIST_POLICY = '6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1'
HIST_TOKEN = '9fd5c53f6a1a03d20b0409f2ca3ad28bd02d5401564603fab5f81b289cc6d54e'
HIST_ENV_CBOR = (
    'd8799f020058204d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32'
    '58200d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd'
    '5820d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd'
    '581c55555555555555555555555555555555555555555555555555555555'
    'd8799f5820222222222222222222222222222222222222222222222222222222222222222200ff'
    '5820c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc'
    '1b000001b8dac5b400ff')
ENV_PREFIX = HIST_ENV_CBOR[:-20]
POLICY = bytes.fromhex(HIST_POLICY)
RESULT_KEYS = ('envCborHex', 'envData', 'envelopeTokenNameHex', 'expiry', 'format',
               'issueCborHex', 'issueData', 'labels', 'mintValueCborHex', 'mintValueData',
               'policyIdHex', 'setupRecordSha256Declared')
LABELS = {
    'expiry': 'CALLER_SUPPLIED_NOT_PROOF_BOUND',
    'context': 'CALLER_DECLARED_UNAUTHENTICATED',
    'policyId': 'CALLER_SUPPLIED_UNAUTHENTICATED',
    'trustProfile': 'single-party-test-setup/1',
    'setupRecordLink': 'DECLARED_NOT_BYTE_CHECKED',
    'groth16': 'NOT_VERIFIED',
    'vkCurveValidity': 'NOT_CHECKED',
    'ledger': 'NO_CLAIM',
}
FIX = {}


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _read(name, cap):
    fd = os.open(os.path.join(HERE, name), os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise AssertionError(name + ': expected a regular file')
        data = b''
        while len(data) <= cap:
            chunk = os.read(fd, 65536)
            if not chunk:
                return data
            data += chunk
        raise AssertionError(name + ': exceeds the read bound')
    finally:
        os.close(fd)


def _module(name, data):
    # Execute exactly the bytes already read and pinned: no import path, no pycache.
    module = types.ModuleType(name)
    module.__file__ = os.path.join(HERE, name + '.py')
    exec(compile(data, module.__file__, 'exec', dont_inherit=True), module.__dict__)
    return module


def _unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise AssertionError('duplicate JSON key ' + key)
        result[key] = value
    return result


def _no_constant(name):
    raise AssertionError('non-finite JSON constant ' + name)


def _inputs():
    # Fresh JSON-compatible dictionaries for every call.
    return tuple(json.loads(FIX[key].decode('utf-8'), object_pairs_hook=_unique,
                            parse_constant=_no_constant) for key in ('proof', 'setup'))


def setUpModule():
    host = _read(HOST_FILE, 32768)
    proof = _read(PROOF_FILE, 65536)
    setup = _read(SETUP_FILE, 65536)
    if (len(host), _sha(host)) != (HOST_SIZE, HOST_SHA256):
        raise AssertionError('host_codec.py is not the pinned reviewed 23252-byte host')
    if (len(proof), _sha(proof)) != (PROOF_SIZE, PROOF_SHA256):
        raise AssertionError(PROOF_FILE + ' is not the pinned K6 public proof export')
    if _sha(setup) != SETUP_SHA256:
        raise AssertionError(SETUP_FILE + ' differs from the declared setupRecordSha256')
    FIX.update(proof=proof, setup=setup, previous=sys.modules.get('host_codec'))
    FIX['host'] = _module('host_codec', host)
    sys.modules['host_codec'] = FIX['host']


def tearDownModule():
    if FIX.get('previous') is None:
        sys.modules.pop('host_codec', None)
    else:
        sys.modules['host_codec'] = FIX['previous']


def _prepare():
    if 'prepare' not in FIX:
        try:
            data = _read(CONSUMER_FILE, 131072)
        except FileNotFoundError:
            raise AssertionError('missing implementation: ' + CONSUMER_FILE + ' is absent') from None
        FIX['consumer_bytes'] = data
        print(CONSUMER_FILE + ' sha256 ' + _sha(data), file=sys.stderr)
        FIX['consumer'] = _module('c2_issue_artifacts', data)
        FIX['prepare'] = FIX['consumer'].__dict__.get('prepare_c2_issue')
    if type(FIX['prepare']) is not types.FunctionType:
        raise AssertionError('missing implementation: prepare_c2_issue')
    return FIX['prepare']


def _same(left, right):
    # Data equality that also distinguishes int from bool and tuple from list.
    if type(left) is not type(right):
        return False
    if type(left) in (tuple, list):
        return len(left) == len(right) and all(_same(a, b) for a, b in zip(left, right))
    return left == right


def _env(expiry):
    return ('constr', 0, [
        ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', bytes.fromhex(PRED)),
        ('bytes', bytes.fromhex(VKD)), ('bytes', bytes.fromhex('55' * 28)),
        ('constr', 0, [('bytes', bytes.fromhex('22' * 32)), ('int', 0)]),
        ('bytes', bytes.fromhex(STMTD)), ('int', expiry)])


def _issue_cbor(vk_hex, proof_hex):
    # serialise_data: tag 121, indefinite fields, bytes over 64 as indefinite
    # 64-byte chunks (EXPECTATIONS cborRule; pallas plutus_data.rs 423-443).
    vk = ''.join('5840' + vk_hex[i:i + 128] for i in range(0, 896, 128))
    proof = ''.join('5840' + proof_hex[i:i + 128] for i in range(0, 384, 128))
    return 'd8799f5f' + vk + '5828' + vk_hex[896:] + 'ff5f' + proof + 'ffff'


def _mint_cbor(policy_hex, token_hex):
    return 'a1581c' + policy_hex + 'a15820' + token_hex + '01'


def _flip(text, offset):
    # Flip the low bit of one statement byte (structural boundary data only).
    i = 2 * offset
    return text[:i] + format(int(text[i:i + 2], 16) ^ 1, '02x') + text[i + 2:]


def _mutated(changes):
    export, setup = _inputs()
    for target, path, value in changes:
        node = export if target == 'export' else setup
        for key in path[:-1]:
            node = node[key]
        node[path[-1]] = value(node[path[-1]]) if callable(value) else value
    return export, setup


class _Hooked(dict):
    # Any hook call proves the record was touched before the C2I-ARG refusal.
    def _touched(self, *args, **kwargs):
        raise AssertionError('record touched before the C2I-ARG refusal')
    __getitem__ = __iter__ = __len__ = __contains__ = keys = items = values = get = _touched


TAMPER = (
    ('C2I-SETUP', 'setup', ('trustProfile',), 'multi-party/1'),
    ('C2I-SETUP', 'setup', ('extra',), 0),
    ('C2I-SETUP', 'setup', ('relation', 'headerLen'), 344),
    ('C2I-SETUP', 'setup', ('verifyingKeyHex',), str.upper),
    ('C2I-SETUP', 'setup', ('verifyingKeyHex',), lambda v: v[:-2]),
    ('C2I-SETUP', 'setup', ('vkDigestBlake2b256',), lambda v: v[:-1] + 'c'),
    ('C2I-EXPORT', 'export', ('format',), 'moriarty-k2-groth16-proof/2'),
    ('C2I-EXPORT', 'export', ('extra',), 0),
    ('C2I-EXPORT', 'export', ('premises',), lambda v: v[:5]),
    ('C2I-EXPORT', 'export', ('context', 'seq'), True),
    ('C2I-EXPORT', 'export', ('cardano', 'envV2', 'expiry'), HIST_EXPIRY),
    ('C2I-RELATION', 'export', ('programHash',), '00' * 32),
    ('C2I-RELATION', 'export', ('actionIndex',), 2),
    ('C2I-RELATION', 'export', ('sourceSha256',), None),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: v[:-2]),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: v[:-1]),
    ('C2I-STATEMENT', 'export', ('statementHex',), str.upper),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: _flip(v, 48)),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: _flip(v, 112)),
    ('C2I-CONTEXT', 'export', ('statementHex',), lambda v: _flip(v, 144)),
    ('C2I-CONTEXT', 'export', ('context', 'consumedIx'), 1),
    ('C2I-DIGEST', 'export', ('statementHex',), lambda v: _flip(v, 400)),
    ('C2I-DIGEST', 'export', ('statementDigestBlake2b256',), '00' * 32),
    ('C2I-DIGEST', 'export', ('publicInputs', 'x1'), lambda v: '0' + v),
    ('C2I-PROOF', 'export', ('proofHex',), lambda v: v[:-2]),
    ('C2I-PROOF', 'export', ('proofHex',), lambda v: v + '00'),
    ('C2I-CARDANO', 'export', ('cardano', 'envV2', 'claimantVkh'), '54' * 28),
    ('C2I-CARDANO', 'export', ('cardano', 'envV2', 'settleOutref', 'outputIndex'), 1),
    ('C2I-CARDANO', 'export', ('cardano', 'intentAnchor'), lambda v: v.replace('v3', 'v2')),
)


class C2IssueArtifactTests(unittest.TestCase):

    def call(self, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        return _prepare()(export, setup, expiry, policy_id=policy)

    def refused(self, code, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        prepare = _prepare()
        try:
            prepare(export, setup, expiry, policy_id=policy)
        except ValueError as error:
            self.assertIs(type(error), ValueError)
            self.assertTrue(str(error).startswith(code + ':'), (code, str(error)))
            return
        self.fail('accepted; expected ' + code)

    def test_01_public_records_literal_provenance(self):
        export, setup = _inputs()
        self.assertEqual(export['setupRecordSha256'], SETUP_SHA256)
        self.assertEqual(setup['trustProfile'], 'single-party-test-setup/1')
        self.assertEqual((export['vkDigestBlake2b256'], setup['vkDigestBlake2b256']), (VKD, VKD))
        self.assertEqual((export['predicateDigest'], export['statementDigestBlake2b256']), (PRED, STMTD))
        context = export['context']
        self.assertEqual((context['claimantVkh'], context['consumedTxid'], context['consumedIx'],
                          context['networkId']), ('55' * 28, '22' * 32, 0, 0))
        self.assertEqual((len(setup['verifyingKeyHex']), len(export['proofHex'])), (976, 384))
        # Expiry is absent from both Rust records: only the caller can supply it.
        self.assertNotIn(b'expiry', FIX['proof'].lower())
        self.assertNotIn(b'expiry', FIX['setup'].lower())
        segments = ('d8799f', '02', '00', '5820' + FAMILY.hex(), '5820' + PRED, '5820' + VKD,
                    '581c' + '55' * 28, 'd8799f5820' + '22' * 32 + '00ff', '5820' + STMTD,
                    '1b000001b8dac5b400', 'ff')
        self.assertEqual(''.join(segments), HIST_ENV_CBOR)

    def test_02_api_shape_and_host_reuse(self):
        prepare = _prepare()
        code = prepare.__code__
        self.assertEqual((code.co_argcount, code.co_kwonlyargcount), (3, 1))
        self.assertEqual(code.co_flags & 0x0C, 0)
        self.assertEqual(code.co_varnames[:4], ('proof_export', 'setup_record', 'expiry', 'policy_id'))
        self.assertIsNone(prepare.__defaults__)
        self.assertIsNone(prepare.__kwdefaults__)
        consumer = FIX['consumer']
        self.assertIs(consumer.__dict__.get('host_codec'), FIX['host'])
        for name in HOST_APIS:
            self.assertNotIn(name, consumer.__dict__)
        public = sorted(name for name, value in consumer.__dict__.items()
                        if not name.startswith('_') and type(value) is types.FunctionType)
        self.assertEqual(public, ['prepare_c2_issue'])

    def test_03_k6_env_data_cbor_and_token_reproduce_historical_vector(self):
        result = self.call(*_inputs())
        self.assertTrue(_same(result['envData'], _env(HIST_EXPIRY)))
        self.assertEqual(result['envCborHex'], HIST_ENV_CBOR)
        self.assertEqual(result['envelopeTokenNameHex'], HIST_TOKEN)

    def test_04_k6_issue_data_and_chunked_cbor(self):
        export, setup = _inputs()
        result = self.call(export, setup)
        vk, proof = bytes.fromhex(setup['verifyingKeyHex']), bytes.fromhex(export['proofHex'])
        self.assertTrue(_same(result['issueData'], ('constr', 0, [('bytes', vk), ('bytes', proof)])))
        self.assertEqual(result['issueCborHex'], _issue_cbor(setup['verifyingKeyHex'], export['proofHex']))
        self.assertEqual(len(result['issueCborHex']), 1420)

    def test_05_mint_value_no_ada_and_caller_policy(self):
        for policy_hex in (HIST_POLICY, '01' * 28):
            with self.subTest(policy=policy_hex):
                result = self.call(*_inputs(), policy=bytes.fromhex(policy_hex))
                expected = ('map', [(('bytes', bytes.fromhex(policy_hex)),
                                     ('map', [(('bytes', bytes.fromhex(HIST_TOKEN)), ('int', 1))]))])
                self.assertTrue(_same(result['mintValueData'], expected))
                self.assertEqual(result['mintValueCborHex'], _mint_cbor(policy_hex, HIST_TOKEN))
                self.assertEqual(result['policyIdHex'], policy_hex)
                # The policy is not an EnvV2 field: Env bytes and token are unchanged.
                self.assertEqual((result['envCborHex'], result['envelopeTokenNameHex']),
                                 (HIST_ENV_CBOR, HIST_TOKEN))

    def test_06_result_schema_labels_bounds_and_detachment(self):
        export, setup = _inputs()
        before = copy.deepcopy((export, setup))
        first = self.call(export, setup)
        second = self.call(export, setup)
        self.assertEqual((export, setup), before)
        self.assertIs(type(first), dict)
        self.assertEqual(sorted(first), sorted(RESULT_KEYS))
        self.assertEqual(first['format'], 'moriarty-c2-issue-artifacts/1')
        self.assertIs(type(first['labels']), dict)
        self.assertEqual(first['labels'], LABELS)
        self.assertIs(type(first['expiry']), int)
        self.assertEqual(first['expiry'], HIST_EXPIRY)
        self.assertEqual(first['setupRecordSha256Declared'], SETUP_SHA256)
        widths = {'envCborHex': 440, 'envelopeTokenNameHex': 64, 'issueCborHex': 1420,
                  'mintValueCborHex': 134, 'policyIdHex': 56, 'setupRecordSha256Declared': 64}
        for key, width in widths.items():
            self.assertIs(type(first[key]), str)
            self.assertEqual(len(first[key]), width)
            self.assertLessEqual(set(first[key]), set('0123456789abcdef'))
        self.assertEqual(first, second)
        self.assertIsNot(first['envData'][2], second['envData'][2])
        self.assertIsNot(first['labels'], second['labels'])
        first['envData'][2].clear()
        first['labels'].clear()
        third = self.call(*_inputs())
        self.assertTrue(_same(third['envData'], _env(HIST_EXPIRY)))
        self.assertEqual(third['labels'], LABELS)

    def test_07_expiry_is_caller_explicit_and_bounded(self):
        cases = ((0, '00'), (23, '17'), (24, '1818'), ((1 << 64) - 1, '1b' + 'ff' * 8))
        for expiry, encoded in cases:
            with self.subTest(expiry=expiry):
                result = self.call(*_inputs(), expiry=expiry)
                self.assertTrue(_same(result['envData'], _env(expiry)))
                self.assertEqual(result['envCborHex'], ENV_PREFIX + encoded + 'ff')
                self.assertEqual(result['expiry'], expiry)
                self.assertEqual(len(result['envelopeTokenNameHex']), 64)
                self.assertNotEqual(result['envelopeTokenNameHex'], HIST_TOKEN)
        for index, bad in enumerate((-1, 1 << 64, True, None, float(HIST_EXPIRY), str(HIST_EXPIRY))):
            with self.subTest(bad=index):
                self.refused('C2I-ARG', *_inputs(), expiry=bad)

    def test_08_each_tamper_refused_at_its_stage(self):
        for index, (code, target, path, value) in enumerate(TAMPER):
            with self.subTest(index=index, code=code):
                export, setup = _mutated([(target, path, value)])
                self.refused(code, export, setup)

    def test_09_first_failing_stage_wins(self):
        bad_setup = ('setup', ('trustProfile',), 'multi-party/1')
        bad_format = ('export', ('format',), 'moriarty-k2-groth16-proof/2')
        bad_seq = ('export', ('context', 'seq'), True)
        bad_program = ('export', ('programHash',), '00' * 32)
        short_statement = ('export', ('statementHex',), lambda v: v[:-2])
        bad_predicate = ('export', ('statementHex',), lambda v: _flip(v, 48))
        mainnet = [('export', ('statementHex',), lambda v: _flip(v, 172)),
                   ('export', ('context', 'networkId'), 1)]
        bad_digest = ('export', ('statementDigestBlake2b256',), '00' * 32)
        short_proof = ('export', ('proofHex',), lambda v: v[:-2])
        bad_anchor = ('export', ('cardano', 'intentAnchor'), 'cardano-plutus-v3:00')
        cases = (
            ('C2I-ARG', [bad_setup, bad_format], -1),
            ('C2I-SETUP', [bad_setup, bad_format], HIST_EXPIRY),
            ('C2I-EXPORT', [bad_seq, bad_program], HIST_EXPIRY),
            ('C2I-RELATION', [bad_program, short_statement], HIST_EXPIRY),
            ('C2I-STATEMENT', [bad_predicate, short_proof, bad_anchor], HIST_EXPIRY),
            ('C2I-CONTEXT', mainnet + [short_proof, bad_anchor], HIST_EXPIRY),
            ('C2I-DIGEST', [bad_digest, short_proof, bad_anchor], HIST_EXPIRY),
            ('C2I-PROOF', [short_proof, bad_anchor], HIST_EXPIRY),
        )
        for code, changes, expiry in cases:
            with self.subTest(code=code):
                export, setup = _mutated(changes)
                self.refused(code, export, setup, expiry=expiry)

    def test_10_argument_types_refused_before_records_are_touched(self):
        export, setup = _inputs()
        cases = (
            (_Hooked(export), setup, POLICY), (export, _Hooked(setup), POLICY),
            ([], setup, POLICY), (None, setup, POLICY),
            (FIX['proof'].decode('utf-8'), setup, POLICY), (export, FIX['setup'], POLICY),
            (export, setup, POLICY[:27]), (export, setup, POLICY + bytes(1)),
            (export, setup, bytearray(POLICY)), (export, setup, HIST_POLICY), (export, setup, None))
        for index, (proof_export, setup_record, policy) in enumerate(cases):
            with self.subTest(index=index):
                self.refused('C2I-ARG', proof_export, setup_record, policy=policy)

    def test_11_consumer_source_has_no_fixture_identity_or_effects(self):
        _prepare()
        text = FIX['consumer_bytes'].decode('utf-8')
        imports = sorted(line.strip() for line in text.splitlines()
                         if line.strip().startswith(('import ', 'from ')))
        self.assertEqual(imports, ['import hashlib', 'import host_codec'])
        fixture_identity = ('55' * 14, '22' * 16, 'a2' * 14, chr(92) + 'x55', chr(92) + 'x22',
                            str(HIST_EXPIRY), HIST_TOKEN[:16], HIST_POLICY[:16], PRED[:16],
                            VKD[:16], STMTD[:16], '7a7fafdd', '37de4837', 'b8e2f734')
        effects = ('open(', 'subprocess', 'socket', 'eval(', 'exec(', 'compile(', '__import__',
                   'importlib', 'environ', 'urllib', 'print(')
        for token in fixture_identity + effects:
            with self.subTest(token=token):
                self.assertNotIn(token, text)


if __name__ == '__main__':
    unittest.main(verbosity=2)


## S24 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-CONTRACT.md
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


## S25 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-API.json
{
 "format": "c2-builder-api/1",
 "status": "AUTHOR_PROPOSED_SOURCE_ONLY; not implemented, run, reviewed or adopted",
 "author": "claude-opus-5-5 HIGH, tools off",
 "contract": "C2-BUILDER-CONTRACT.md CB01-CB12",
 "dependency": {
  "consumer": "c2_issue_artifacts.prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) per CI01-CI12; AUTHOR-PROPOSED, ordinary RED observed, implementation pending; consumed unchanged",
  "assumedResult": "exact 12-key CI section 4 dict; labels an exact dict of 8 pairs; *Hex values lowercase",
  "onIncompatibility": "STOP; adapt neither the consumer nor the builder"
 },
 "module": {
  "file": "build_k10_inputs_r3.py",
  "beside": "build_k10_inputs.py, 24480 B, sha256 6c1cde7f971a4c3ade25a3089f22f557fe2290b3b1266c4c18b6b4b7ce918d00, unchanged",
  "imports": ["hashlib", "json", "os", "re", "stat", "sys", "build_k10_inputs as legacy", "c2_issue_artifacts", "host_codec"],
  "sysPath": "never mutated by r3; the importer places the dedicated directory first"
 },
 "functions": {
  "parse_expiry(text) -> int": "ARGV; Opus",
  "read_public_file(path, cap) -> bytes": "READ; Opus",
  "parse_public_json(raw, cap) -> dict": "JSON; Opus",
  "check_setup_link(export, setup_raw) -> str": "LINK; Opus",
  "legacy_env_from_host(env_data) -> legacy Constr": "BRIDGE; Opus",
  "plan_outputs(b9_raw, export_raw, setup_raw, expiry) -> dict[str, bytes]": "ARGV, JSON(export 1048576), JSON(setup 65536), LINK, POLICY, CONSUMER, BRIDGE, RENDER. Opus owns the stage order and the BRIDGE block; Sol owns the case table, legacy render calls and manifest assembly",
  "main(argv) -> int": "ARGV, OUTDIR, READ, plan, WRITE, REREAD. Sol wires; Opus owns the OUTDIR/READ policy and the exclusive writer"
 },
 "argv": ["prog", "b9-applied.json", "proof-export.json", "setup-record.json", "expiry-ms: full ASCII match 0|[1-9][0-9]{0,19}, <= 2**64-1", "output-dir: must not lexist, parent is an existing dir, basename [a-z0-9][a-z0-9._-]{0,99}"],
 "caps": {"b9Bytes": 16777216, "exportBytes": 1048576, "setupBytes": 65536, "jsonDepth": 16, "jsonIntChars": 20},
 "refusal": {
  "form": "ValueError starting 'C2B-<STAGE>:'; main prints exactly one stderr line, no stdout, returns 2",
  "stages": ["ARGV", "OUTDIR", "READ", "JSON", "LINK", "POLICY", "CONSUMER", "BRIDGE", "WRITE", "REREAD"],
  "consumerWrap": "C2B-CONSUMER: <original C2I message>",
  "unexpected": "non-ValueError environmental errors propagate"
 },
 "bridge": {
  "disposition": "reuse legacy uplc_term, script_context, value_map, cbor_encode, flip_first_bit and extract_compiled unchanged; no general host-Data-to-UPLC renderer",
  "envV2": "exact host shape -> legacy Constr; legacy cbor_encode == envCborHex bytes; blake2b256 == envelopeTokenNameHex",
  "value": "legacy cbor_encode(value_map(policy, token, 1)) == mintValueCborHex (no-ada, F01)",
  "issue": "exact leaves 488/192 equal to the setup VK and export proof bytes; host encode_data(issueData) == issueCborHex; host-only, because the legacy encoder fails closed above 64 bytes",
  "t7": "legacy env CBOR == host encode_data(build_env_data(...flipped vk digest...))"
 },
 "outputs": {
  "files": ["script.hex", "contexts/K10-T1-positive-genuine-issue.uplc.txt", "contexts/K10-T5-quantity-two-reject.uplc.txt", "contexts/K10-T6-token-name-bitflip-reject.uplc.txt", "contexts/K10-T7-vk-digest-bitflip-reject.uplc.txt", "manifest.json"],
  "encoding": "script.hex = code_hex plus newline; context = '(con data (' + legacy.uplc_term(script_context) + '))' plus newline; manifest = json.dumps(indent=2, sort_keys=True) plus newline; UTF-8",
  "writeOrder": "mkdir outdir and contexts (exclusive); script.hex; contexts; re-read inputs; manifest.json last",
  "fileOpen": "O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC, mode 0644",
  "stdout": "one JSON line {output: argv[5], cases: 4}"
 },
 "manifest": {
  "format": "cardano-k2-k10-public-inputs/2",
  "keptFromR2": "auditSourcePins, compiledValidatorTitle, policyIdHex, positiveEnvelopeCborHex, positiveTokenNameHex, proofBytes, scriptCompiledBytes, scriptHashAlgorithm, scriptHashClaimedHex, scriptHashClassification, scriptHashIndependentlyRecomputedHex, scriptHashPreimage, scriptHexFile, scriptInnerFlatBytes, statementDigestHex, vkBytes, vkDigestHex, cases (R2 per-case keys); R2 strings and semantics are copied verbatim",
  "new": "builderRevision build_k10_inputs_r3; consumerFormat; consumerLabels (verbatim); builderLabels and limitations exactly as in test_c2_builder.py; flags (all false); expiry; claimantVkhHex; settleOutref {transactionIdHex, outputIndex}; issueCborHex; mintValueCborHex; setupRecordSha256 (file-checked); inputs [{role, sha256, bytes}] for b9-applied, proof-export, setup-record; coverageGaps T2-T4 status NOT_GENERATED_HISTORICAL_FIXTURE_ONLY"
 },
 "cases": {"T1": "consumer env, token, qty 1", "T5": "qty 2", "T6": "token first bit flipped", "T7": "env vk digest first bit flipped, token recomputed", "T2-T4": "coverage gap; historical 7-case output preserved; no synthetic proofs"},
 "ownership": {
  "opusSecurity": "argv/expiry, bounded no-follow read, strict JSON, file-byte link, policy pin gate, EnvV2 bridge and every equality check, stage order, exclusive writer",
  "solMechanical": "main dispatch, case table, legacy render calls, manifest field copy, stdout/stderr reporting",
  "review": "fresh non-author Opus on the actual combined code and results; fresh Muse on the Sol glue; same-provider independence is context only"
 },
 "prerequisites": ["c2_issue_artifacts.py implemented, with its own ordinary run observed and its bytes/sha recorded by root", "inputs/b9-applied.json staged with sha 10baa1794e0ae286cd780978408552fc0e994426367c2b92117e2c4b46ad1d36, byte count recorded", "inputs/HIST-09-positive-script-context.txt equal to the archived 09 bytes", "K6 fixtures 5800 B and 3926 B, host 23252 B and legacy 24480 B, as pinned in the test", "absent required public file: BLOCKED, never synthesized"],
 "envelope": {
  "layout": "isolated/ (555, files 444): test_c2_builder.py, build_k10_inputs.py, host_codec.py, c2_issue_artifacts.py, inputs/ (555); r3 absent at RED. scratch/ beside it (700, empty)",
  "command": "python3 -I -B isolated/test_c2_builder.py, once, no retry, after root inspects all sources and effects",
  "limits": "timeout 20 s, CPU 20 s, address space 512 MiB, fsize 2 MiB, core 0, PATH/LANG/PYTHONDONTWRITEBYTECODE only, no network, total <= 1200 s",
  "pins": "before/after sha256 and bytes of every isolated file unchanged",
  "scratchRosterRED": "empty",
  "scratchRosterGREEN": "out-k6/ (contexts/ + 6 files), pre-existing/ (empty), link-out -> pre-existing, oversize.json (1048577 B), input-link.json -> isolated/inputs/K6-PUBLIC-PROOF.json",
  "distinctFrom": "the CI pure envelope; this one adds a writable scratch"
 },
 "red": "test_01 passes; test_02-test_10 fail with 'missing implementation: build_k10_inputs_r3.py'; 0 errors. Any setUpModule error, pin mismatch, ImportError or test_01 failure: STOP, not RED",
 "flags": {"native": false, "operator": false, "wallet": false, "prover": false, "vm": false, "deploy": false, "builderImplemented": false, "previewTested": false, "preprodTested": false}
}


## S26 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/claim.rs
//! Claimed transitions and their ABI v2 statements.
//!
//! Proposal is separate from acceptance. [`propose_values`] obtains a *claim* from Core's
//! reference or from explicit records; it decides nothing. The statement is built by the
//! compiler's own `CompiledRelation::statement_v2`. Acceptance is the constraint system's
//! (`crate::prove::check_claim`) and, for a proof, the pairing check's.
use crate::error::DriverError;
use crate::hexutil;
use crate::input::{ContextInput, TransitionInput};
use crate::relation::{RelationDescriptor, SelectedRelation};
use ark_bls12_381::Fr;
use mori_r1cs::abi2::{self, AbiV2Constants, FAMILY_V2, HEADER_LEN, STMT_TAG};
use mori_r1cs::{CompiledRelation, ContextV2, TransitionValues, witness};

/// Statement ceiling (bytes). Larger statements are refused before hashing or synthesis.
pub const MAX_STATEMENT_BYTES: usize = 262_144;

/// Byte offsets of ABI v2 statement fields (`abi2.rs:5-16`, `ABI-V2.md` section 3).
pub mod offsets {
    pub const PREDICATE: usize = 48;
    pub const SHAPE: usize = 80;
    pub const VK_DIGEST: usize = 112;
    pub const CLAIMANT: usize = 144;
    pub const NETWORK: usize = 172;
    pub const SETTLE: usize = 173;
    pub const THREAD: usize = 201;
    pub const TXID: usize = 233;
    pub const IX: usize = 265;
    pub const SEQ: usize = 267;
    pub const CATALOGUE: usize = 275;
    pub const INTENTS: usize = 307;
    pub const BODY_LEN: usize = 339;
}

/// A claim ready for the constraint system: values, context, statement and public inputs.
#[derive(Debug, Clone)]
pub struct PreparedClaim {
    pub values: TransitionValues,
    pub context: ContextV2,
    pub statement: Vec<u8>,
    pub statement_digest: [u8; 32],
    /// `[x1, x2]`: big-endian halves of `statement_digest`.
    pub public_inputs: [u128; 2],
}

/// Big-endian 128-bit halves of a 32-byte digest (`abi2.rs:297-305`).
pub fn digest_halves(d: &[u8; 32]) -> [u128; 2] {
    let mut hi = [0u8; 16];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&d[..16]);
    lo.copy_from_slice(&d[16..]);
    [u128::from_be_bytes(hi), u128::from_be_bytes(lo)]
}

/// Builds `ContextV2` from declared premises and the digest of the VK actually used.
pub fn context_from_input(input: &ContextInput, vk_digest: [u8; 32]) -> Result<ContextV2, DriverError> {
    if let Some(stated) = &input.vk_digest {
        let stated = hexutil::decode_exact::<32>(stated, "context vkDigest")?;
        if stated != vk_digest {
            return Err(DriverError::mismatch("context vkDigest", hexutil::encode(&vk_digest), hexutil::encode(&stated)));
        }
    }
    Ok(ContextV2 {
        vk_digest,
        claimant_vkh: hexutil::decode_exact::<28>(&input.claimant_vkh, "claimantVkh")?,
        network_id: input.network_id,
        settle_hash: hexutil::decode_exact::<28>(&input.settle_hash, "settleHash")?,
        thread_id: hexutil::decode_exact::<32>(&input.thread_id, "threadId")?,
        consumed_txid: hexutil::decode_exact::<32>(&input.consumed_txid, "consumedTxid")?,
        consumed_ix: input.consumed_ix,
        seq: input.seq,
        catalogue_digest: hexutil::decode_exact::<32>(&input.catalogue_digest, "catalogueDigest")?,
        intents_digest: hexutil::decode_exact::<32>(&input.intents_digest, "intentsDigest")?,
    })
}

/// Proposes transition values. A Core refusal or a record that does not fit the relation's
/// shape is a refusal to propose, never a "false" claim.
pub fn propose_values(relation: &CompiledRelation, transition: &TransitionInput) -> Result<TransitionValues, DriverError> {
    match transition {
        TransitionInput::CoreReference { scenario } => witness::propose(relation, scenario, None).map_err(DriverError::Proposal),
        TransitionInput::Records { inputs, pre, post, effects } => {
            witness::from_records(relation.plan(), inputs, pre, post, effects, None).map_err(DriverError::Proposal)
        }
    }
}

/// Builds the statement and public inputs of already-proposed values with the compiler. The
/// values are a claim of any origin (proposal, records or a hostile edit); the compiler checks
/// only their shape and types here, and the constraint system decides them.
pub fn claim_from_values(selected: &SelectedRelation, values: TransitionValues, ctx: ContextV2) -> Result<PreparedClaim, DriverError> {
    let relation = selected.relation();
    let statement = relation.statement_v2(&values, &ctx).map_err(DriverError::Proposal)?;
    if statement.len() > MAX_STATEMENT_BYTES {
        return Err(DriverError::Input(format!("statement of {} bytes exceeds {MAX_STATEMENT_BYTES}", statement.len())));
    }
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    let public_inputs = digest_halves(&statement_digest);
    // The compiler's own public-input path must agree with the digest halves.
    let compiled = relation.public_inputs_v2::<Fr>(&values, &ctx).map_err(DriverError::Proposal)?;
    if compiled != vec![Fr::from(public_inputs[0]), Fr::from(public_inputs[1])] {
        return Err(DriverError::Backend("compiler public inputs differ from the statement digest halves".into()));
    }
    Ok(PreparedClaim { values, context: ctx, statement, statement_digest, public_inputs })
}

/// Proposes the values and builds the statement and public inputs with the compiler.
pub fn prepare_claim(
    selected: &SelectedRelation,
    transition: &TransitionInput,
    context: &ContextInput,
    vk_digest: [u8; 32],
) -> Result<PreparedClaim, DriverError> {
    let ctx = context_from_input(context, vk_digest)?;
    let values = propose_values(selected.relation(), transition)?;
    claim_from_values(selected, values, ctx)
}

/// The fields of a parsed ABI v2 statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementView {
    pub predicate_digest: [u8; 32],
    pub shape_digest: [u8; 32],
    pub context: ContextV2,
    pub body_len: u32,
}

fn take<const N: usize>(s: &[u8], at: usize) -> Result<[u8; N], DriverError> {
    s.get(at..at + N)
        .and_then(|b| <[u8; N]>::try_from(b).ok())
        .ok_or_else(|| DriverError::Input(format!("statement too short for {N} bytes at {at}")))
}

/// Strictly parses statement bytes, then re-encodes them with the compiler's
/// `abi2::statement_bytes` and requires byte equality (so the offsets here cannot drift from
/// the compiler's layout unnoticed).
pub fn parse_statement(s: &[u8]) -> Result<StatementView, DriverError> {
    use crate::claim::offsets as o;
    if s.len() < HEADER_LEN || s.len() > MAX_STATEMENT_BYTES {
        return Err(DriverError::Input(format!("statement length {} outside [{HEADER_LEN}, {MAX_STATEMENT_BYTES}]", s.len())));
    }
    if s[..16] != STMT_TAG[..] {
        return Err(DriverError::Input("statement tag is not MRG16-PVC-STMT-2".into()));
    }
    if s[16..48] != FAMILY_V2[..] {
        return Err(DriverError::Input("statement family id is not MRG16-PVC-FAM2-G16-BLS-ARK04-HL2".into()));
    }
    let context = ContextV2 {
        vk_digest: take::<32>(s, o::VK_DIGEST)?,
        claimant_vkh: take::<28>(s, o::CLAIMANT)?,
        network_id: s[o::NETWORK],
        settle_hash: take::<28>(s, o::SETTLE)?,
        thread_id: take::<32>(s, o::THREAD)?,
        consumed_txid: take::<32>(s, o::TXID)?,
        consumed_ix: u16::from_be_bytes(take::<2>(s, o::IX)?),
        seq: u64::from_be_bytes(take::<8>(s, o::SEQ)?),
        catalogue_digest: take::<32>(s, o::CATALOGUE)?,
        intents_digest: take::<32>(s, o::INTENTS)?,
    };
    let body_len = u32::from_be_bytes(take::<4>(s, o::BODY_LEN)?);
    let expected_len = usize::try_from(body_len).ok().and_then(|b| b.checked_add(HEADER_LEN));
    if expected_len != Some(s.len()) {
        return Err(DriverError::Input(format!("statement length {} differs from 343 + body_len {body_len}", s.len())));
    }
    let view = StatementView {
        predicate_digest: take::<32>(s, o::PREDICATE)?,
        shape_digest: take::<32>(s, o::SHAPE)?,
        context,
        body_len,
    };
    let consts = AbiV2Constants {
        predicate_digest: view.predicate_digest,
        shape_bytes: Vec::new(),
        shape_digest: view.shape_digest,
        body_len,
    };
    let rebuilt = abi2::statement_bytes(&consts, &view.context, &s[HEADER_LEN..]).map_err(DriverError::Proposal)?;
    if rebuilt != s {
        return Err(DriverError::Backend("statement offsets differ from the compiler's ABI v2 encoder".into()));
    }
    Ok(view)
}

/// Checks a parsed statement's relation constants and VK digest against a descriptor.
pub fn check_statement_against(desc: &RelationDescriptor, vk_digest: &[u8; 32], view: &StatementView) -> Result<(), DriverError> {
    let checks = [
        ("statement predicate digest", desc.predicate_digest.clone(), hexutil::encode(&view.predicate_digest)),
        ("statement ABI v2 shape digest", desc.abi2_shape_digest.clone(), hexutil::encode(&view.shape_digest)),
        ("statement vk digest", hexutil::encode(vk_digest), hexutil::encode(&view.context.vk_digest)),
        ("statement body length", desc.body_len.to_string(), view.body_len.to_string()),
    ];
    for (what, expected, actual) in checks {
        if expected != actual {
            return Err(DriverError::mismatch(what, expected, actual));
        }
    }
    Ok(())
}


## S27 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/encoding.rs
//! Canonical BLS12-381 Groth16 encodings for the Cardano C2 consumer, and strict decoders.
//!
//! Derived from the pinned sources, not from memory:
//! - points: `ark-bls12-381 0.4.0 curves/util.rs:16-72`, `g1.rs:102-134`, `g2.rs:136-177`.
//!   Compressed G1 is 48 bytes, G2 96 bytes; big-endian x (G2: `x.c1 || x.c0`); flag bits in the
//!   first byte: 7 compressed, 6 infinity, 5 lexicographically largest y. This is the Zcash
//!   layout that the Plutus `bls12_381_*_uncompress` builtins read (C2, `groth16.ak`).
//! - VK and proof: field order of `ark-groth16 0.4.0 data_structures.rs:6-42`, `Vec` length as
//!   u64 little-endian (`ark-serialize 0.4.2 impls.rs:527-540`). VK = alpha G1, beta G2,
//!   gamma G2, delta G2, `03 00 00 00 00 00 00 00`, ic0, ic1, ic2 = 488 bytes; proof = A G1,
//!   B G2, C G1 = 192 bytes (`groth16.ak:15-20,52-69`).
//!
//! Strictness beyond arkworks: arkworks' compressed reader returns the identity whenever the
//! infinity bit is set and ignores every other bit (`util.rs:107-109,166-168`). Every decoder
//! here re-encodes and requires byte equality, checks on-curve and subgroup membership, and
//! refuses identity points in a VK (all seven) and in a proof (all three). C2's F-I profile
//! refuses identity only for gamma, delta, ic1 and ic2, so the driver accepts a subset of what
//! C2 accepts; it never accepts more.
use crate::error::DriverError;
use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

pub const G1_LEN: usize = 48;
pub const G2_LEN: usize = 96;
pub const VK_LEN: usize = 488;
pub const PROOF_LEN: usize = 192;
/// Offset of the `gamma_abc_g1` length prefix in a VK (C2 `slice_bytearray(336, 8, vk)`).
pub const VK_IC_OFFSET: usize = 336;
/// u64 little-endian 3: constant term plus two public inputs (C2 `vk_ic_count`).
pub const VK_IC_PREFIX: [u8; 8] = [3, 0, 0, 0, 0, 0, 0, 0];

/// Compressed G1 identity: `c0` followed by 47 zero bytes (C2 `g1_identity_compressed`).
pub fn g1_identity_compressed() -> [u8; G1_LEN] {
    let mut b = [0u8; G1_LEN];
    b[0] = 0xc0;
    b
}

/// Compressed G2 identity: `c0` followed by 95 zero bytes (C2 `g2_identity_compressed`).
pub fn g2_identity_compressed() -> [u8; G2_LEN] {
    let mut b = [0u8; G2_LEN];
    b[0] = 0xc0;
    b
}

fn enc(what: &str, detail: impl std::fmt::Display) -> DriverError {
    DriverError::Encoding(format!("{what}: {detail}"))
}

pub fn encode_g1(p: &G1Affine) -> Result<[u8; G1_LEN], DriverError> {
    let mut out = [0u8; G1_LEN];
    p.serialize_compressed(&mut out[..]).map_err(|e| enc("G1 serialization", e))?;
    Ok(out)
}

pub fn encode_g2(p: &G2Affine) -> Result<[u8; G2_LEN], DriverError> {
    let mut out = [0u8; G2_LEN];
    p.serialize_compressed(&mut out[..]).map_err(|e| enc("G2 serialization", e))?;
    Ok(out)
}

/// Strict compressed G1: exact length, valid flags, coordinate below p, on curve, in the
/// prime-order subgroup and canonical (re-encoding gives the same bytes).
pub fn decode_g1(bytes: &[u8], what: &str) -> Result<G1Affine, DriverError> {
    if bytes.len() != G1_LEN {
        return Err(enc(what, format!("expected {G1_LEN} bytes, got {}", bytes.len())));
    }
    let p = G1Affine::deserialize_compressed(bytes).map_err(|e| enc(what, e))?;
    if !p.is_on_curve() || !p.is_in_correct_subgroup_assuming_on_curve() {
        return Err(enc(what, "point not in the prime-order subgroup"));
    }
    if encode_g1(&p)?[..] != bytes[..] {
        return Err(enc(what, "non-canonical encoding"));
    }
    Ok(p)
}

/// Strict compressed G2, as [`decode_g1`].
pub fn decode_g2(bytes: &[u8], what: &str) -> Result<G2Affine, DriverError> {
    if bytes.len() != G2_LEN {
        return Err(enc(what, format!("expected {G2_LEN} bytes, got {}", bytes.len())));
    }
    let p = G2Affine::deserialize_compressed(bytes).map_err(|e| enc(what, e))?;
    if !p.is_on_curve() || !p.is_in_correct_subgroup_assuming_on_curve() {
        return Err(enc(what, "point not in the prime-order subgroup"));
    }
    if encode_g2(&p)?[..] != bytes[..] {
        return Err(enc(what, "non-canonical encoding"));
    }
    Ok(p)
}

fn is_identity<A: AffineRepr>(p: &A) -> bool {
    p.xy().is_none()
}

/// SettlementHashV2 VK profile: exactly three IC points and no identity point at all.
pub fn vk_profile_ok(vk: &VerifyingKey<Bls12_381>) -> Result<(), DriverError> {
    if vk.gamma_abc_g1.len() != 3 {
        return Err(enc("verifying key", format!("{} IC points; SettlementHashV2 needs 3", vk.gamma_abc_g1.len())));
    }
    let g1 = [("alpha", &vk.alpha_g1), ("ic0", &vk.gamma_abc_g1[0]), ("ic1", &vk.gamma_abc_g1[1]), ("ic2", &vk.gamma_abc_g1[2])];
    for (name, p) in g1 {
        if is_identity(p) {
            return Err(enc("verifying key", format!("{name} is the identity")));
        }
    }
    let g2 = [("beta", &vk.beta_g2), ("gamma", &vk.gamma_g2), ("delta", &vk.delta_g2)];
    for (name, p) in g2 {
        if is_identity(p) {
            return Err(enc("verifying key", format!("{name} is the identity")));
        }
    }
    Ok(())
}

/// The 488-byte C2 VK encoding. The manual layout is cross-checked against ark-groth16's
/// derived serialization of the same value.
pub fn encode_vk(vk: &VerifyingKey<Bls12_381>) -> Result<Vec<u8>, DriverError> {
    if vk.gamma_abc_g1.len() != 3 {
        return Err(enc("verifying key", "SettlementHashV2 needs exactly 3 IC points"));
    }
    let mut out = Vec::with_capacity(VK_LEN);
    out.extend_from_slice(&encode_g1(&vk.alpha_g1)?);
    out.extend_from_slice(&encode_g2(&vk.beta_g2)?);
    out.extend_from_slice(&encode_g2(&vk.gamma_g2)?);
    out.extend_from_slice(&encode_g2(&vk.delta_g2)?);
    out.extend_from_slice(&VK_IC_PREFIX);
    for p in &vk.gamma_abc_g1 {
        out.extend_from_slice(&encode_g1(p)?);
    }
    if out.len() != VK_LEN {
        return Err(enc("verifying key", format!("encoded {} bytes, expected {VK_LEN}", out.len())));
    }
    let mut derived = vec![0u8; vk.compressed_size()];
    vk.serialize_compressed(&mut derived[..]).map_err(|e| enc("verifying key", e))?;
    if derived != out {
        return Err(enc("verifying key", "manual layout differs from the ark-groth16 derived serialization"));
    }
    Ok(out)
}

/// Strict 488-byte VK decoding: fixed offsets (the length prefix is compared, never trusted
/// as an allocation size), strict points, profile, and whole-key round trip.
pub fn decode_vk(bytes: &[u8]) -> Result<VerifyingKey<Bls12_381>, DriverError> {
    if bytes.len() != VK_LEN {
        return Err(enc("verifying key", format!("expected {VK_LEN} bytes, got {}", bytes.len())));
    }
    if bytes[VK_IC_OFFSET..VK_IC_OFFSET + 8] != VK_IC_PREFIX[..] {
        return Err(enc("verifying key", "IC length prefix is not 3 (u64 little-endian)"));
    }
    let mut ic = Vec::with_capacity(3);
    for (k, at) in [344usize, 392, 440].into_iter().enumerate() {
        ic.push(decode_g1(&bytes[at..at + G1_LEN], &format!("vk ic{k}"))?);
    }
    let vk = VerifyingKey::<Bls12_381> {
        alpha_g1: decode_g1(&bytes[0..48], "vk alpha")?,
        beta_g2: decode_g2(&bytes[48..144], "vk beta")?,
        gamma_g2: decode_g2(&bytes[144..240], "vk gamma")?,
        delta_g2: decode_g2(&bytes[240..336], "vk delta")?,
        gamma_abc_g1: ic,
    };
    vk_profile_ok(&vk)?;
    if encode_vk(&vk)? != bytes {
        return Err(enc("verifying key", "non-canonical encoding"));
    }
    Ok(vk)
}

/// The 192-byte C2 proof encoding, cross-checked against the derived serialization.
pub fn encode_proof(proof: &Proof<Bls12_381>) -> Result<Vec<u8>, DriverError> {
    let mut out = Vec::with_capacity(PROOF_LEN);
    out.extend_from_slice(&encode_g1(&proof.a)?);
    out.extend_from_slice(&encode_g2(&proof.b)?);
    out.extend_from_slice(&encode_g1(&proof.c)?);
    let mut derived = vec![0u8; proof.compressed_size()];
    proof.serialize_compressed(&mut derived[..]).map_err(|e| enc("proof", e))?;
    if derived != out || out.len() != PROOF_LEN {
        return Err(enc("proof", "manual layout differs from the ark-groth16 derived serialization"));
    }
    Ok(out)
}

/// Strict 192-byte proof decoding; identity points are refused.
pub fn decode_proof(bytes: &[u8]) -> Result<Proof<Bls12_381>, DriverError> {
    if bytes.len() != PROOF_LEN {
        return Err(enc("proof", format!("expected {PROOF_LEN} bytes, got {}", bytes.len())));
    }
    let proof = Proof::<Bls12_381> {
        a: decode_g1(&bytes[0..48], "proof A")?,
        b: decode_g2(&bytes[48..144], "proof B")?,
        c: decode_g1(&bytes[144..192], "proof C")?,
    };
    if is_identity(&proof.a) || is_identity(&proof.b) || is_identity(&proof.c) {
        return Err(enc("proof", "identity point"));
    }
    Ok(proof)
}

/// Proving key: arkworks compressed serialization (custody material, not a public export).
pub fn encode_proving_key(pk: &ProvingKey<Bls12_381>) -> Result<Vec<u8>, DriverError> {
    let mut out = vec![0u8; pk.compressed_size()];
    pk.serialize_compressed(&mut out[..]).map_err(|e| enc("proving key", e))?;
    Ok(out)
}

/// Proving key decoding with arkworks validation (on-curve and subgroup for every point),
/// exact length and the VK profile.
pub fn decode_proving_key(bytes: &[u8]) -> Result<ProvingKey<Bls12_381>, DriverError> {
    let pk = ProvingKey::<Bls12_381>::deserialize_compressed(bytes).map_err(|e| enc("proving key", e))?;
    if pk.compressed_size() != bytes.len() {
        return Err(enc("proving key", "trailing bytes after the key"));
    }
    vk_profile_ok(&pk.vk)?;
    Ok(pk)
}


## S28 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/input.rs
//! Bounded input reading and strict request parsing.
//!
//! Every JSON document goes through `mori_core::json::parse_limited` first: duplicate decoded
//! keys, trailing data, depth above 32 and more than 65,536 nodes are refused there. The value
//! is then decoded into `deny_unknown_fields` DTOs. Hex and decimal fields are checked by
//! [`crate::hexutil`] where they are used, never coerced.
use crate::error::DriverError;
use mori_core::{Effect, Scenario, Snapshot, Value};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Source text ceiling (bytes).
pub const MAX_SOURCE_BYTES: u64 = 262_144;
/// JSON document ceiling; equal to Core's own `json::parse` ceiling.
pub const MAX_JSON_BYTES: usize = 1_048_576;
/// Proving-key file ceiling. The real size is unmeasured; a larger key is refused, not read.
pub const MAX_PROVING_KEY_BYTES: u64 = 2 * 1024 * 1024 * 1024;

pub const SELECTOR_FORMAT: &str = "moriarty-k2-relation-selector/1";
pub const REQUEST_FORMAT: &str = "moriarty-k2-prove-request/1";

/// Reads a regular file of at most `max` bytes. Devices, directories and larger files are
/// refused before their content is read.
pub fn read_bounded(path: &Path, max: u64, what: &str) -> Result<Vec<u8>, DriverError> {
    let file = File::open(path).map_err(|e| DriverError::Io(format!("{what}: cannot open {}: {e}", path.display())))?;
    let meta = file.metadata().map_err(|e| DriverError::Io(format!("{what}: cannot stat {}: {e}", path.display())))?;
    if !meta.is_file() {
        return Err(DriverError::Io(format!("{what}: {} is not a regular file", path.display())));
    }
    if meta.len() > max {
        return Err(DriverError::Input(format!("{what}: {} bytes exceed the {max}-byte bound", meta.len())));
    }
    let mut out = Vec::new();
    file.take(max.saturating_add(1))
        .read_to_end(&mut out)
        .map_err(|e| DriverError::Io(format!("{what}: cannot read {}: {e}", path.display())))?;
    if out.len() as u64 > max {
        return Err(DriverError::Input(format!("{what}: file grew beyond the {max}-byte bound while read")));
    }
    Ok(out)
}

/// Reads a source file as UTF-8 text.
pub fn read_source(path: &Path) -> Result<String, DriverError> {
    let bytes = read_bounded(path, MAX_SOURCE_BYTES, "source")?;
    String::from_utf8(bytes).map_err(|_| DriverError::Input("source: not UTF-8".into()))
}

/// Strict JSON decoding of `bytes` into `T`.
pub fn parse_json<T: DeserializeOwned>(bytes: &[u8], what: &str) -> Result<T, DriverError> {
    if bytes.len() > MAX_JSON_BYTES {
        return Err(DriverError::Input(format!("{what}: {} bytes exceed the JSON bound", bytes.len())));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| DriverError::Input(format!("{what}: not UTF-8")))?;
    let value = mori_core::json::parse_limited(text, MAX_JSON_BYTES, "JSON input exceeds the driver bound")
        .map_err(|e| DriverError::Input(format!("{what}: {e}")))?;
    serde_json::from_value(value).map_err(|e| DriverError::Input(format!("{what}: {e}")))
}

/// Which relation to compile. Every option is spelled out; the driver supports exactly one
/// value of each and refuses the rest (see [`crate::relation::check_selector`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RelationSelector {
    pub format: String,
    pub language: String,
    pub semantics: String,
    pub action: String,
    pub adapter: String,
    pub cap: String,
    pub accounting: String,
    pub frame: String,
    /// Optional pin: lowercase hex SHA-256 of the raw source bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_source_sha256: Option<String>,
    /// Optional pin: lowercase hex canonical typed-Program hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_program_hash: Option<String>,
}

/// The claimed transition, as a proposal. There is no default kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum TransitionInput {
    /// Core `simulate` proposes the transition for this scenario.
    CoreReference { scenario: Scenario },
    /// Explicit typed records: inputs, pre and post snapshots and ordered effects.
    Records { inputs: BTreeMap<String, Value>, pre: Snapshot, post: Snapshot, effects: Vec<Effect> },
}

/// Per-proof context bytes, as caller-declared premises. The VK digest is derived from the
/// VK actually used; if a request restates it, the two must be equal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextInput {
    pub claimant_vkh: String,
    pub network_id: u8,
    pub settle_hash: String,
    pub thread_id: String,
    pub consumed_txid: String,
    pub consumed_ix: u16,
    pub seq: u64,
    pub catalogue_digest: String,
    pub intents_digest: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vk_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProveRequest {
    pub format: String,
    pub selector: RelationSelector,
    pub transition: TransitionInput,
    pub context: ContextInput,
}

/// Parses and format-checks a selector document.
pub fn parse_selector(bytes: &[u8]) -> Result<RelationSelector, DriverError> {
    let s: RelationSelector = parse_json(bytes, "selector")?;
    if s.format != SELECTOR_FORMAT {
        return Err(DriverError::Unsupported(format!("selector format {:?}", s.format)));
    }
    Ok(s)
}

/// Parses and format-checks a prove request.
pub fn parse_request(bytes: &[u8]) -> Result<ProveRequest, DriverError> {
    let r: ProveRequest = parse_json(bytes, "prove request")?;
    if r.format != REQUEST_FORMAT {
        return Err(DriverError::Unsupported(format!("request format {:?}", r.format)));
    }
    if r.selector.format != SELECTOR_FORMAT {
        return Err(DriverError::Unsupported(format!("selector format {:?}", r.selector.format)));
    }
    Ok(r)
}


## S29 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/src/abi2.rs
//! `SettlementHashV2` adapter: MRG16-PVC ABI v2 statement bytes and their in-circuit
//! binding (P3S1, PROPOSED, unreviewed, never compiled or run).
//!
//! Normative text: `candidate/docs/ABI-V2.md` (sections 3 and 4). The statement is
//!
//! ```text
//! tag(16) family(32) predicate_digest(32) shape_digest(32)          -- circuit constants
//! vk_digest(32) claimant_vkh(28) network_id(1) settle_hash(28)
//! thread_id(32) consumed_txid(32) consumed_ix(2) seq(8)
//! catalogue_digest(32) intents_digest(32)                           -- free witness bytes
//! body_len(4)                                                       -- circuit constant
//! body                                                              -- slot wires
//! ```
//!
//! and the two public inputs are the big-endian 128-bit halves of BLAKE2b-256(statement).
//! The body is every non-header slot of the [`StatementLayout`] in layout order (UInt 16
//! bytes big-endian, Bool 1 byte, wide sums 18 bytes big-endian). The header slots are
//! covered by `predicate_digest` = [`crate::RelationIdentity::digest`], whose bound
//! identity includes this adapter's name. Context bytes are hashed, not interpreted:
//! the Cardano validator (S2) derives each one from its ScriptContext and recomputes the
//! statement; the proof only certifies that the transition values satisfy the relation.
use crate::adapter::{AdapterError, BindError, SlotWire};
use crate::blake2b::{self, Bit};
use crate::error::{CompileError, MappingError};
use crate::gadgets::{UIntW, WideW};
use crate::plan::{AccountingBinding, ActionPlan, CapBinding, CompileOptions, DutyOrigin, EffectShape, FrameCells};
use crate::r1cs::{Cs, Lc, fe};
use crate::statement::{EffectValues, SlotEncoding, SlotKind, StatementLayout, TransitionValues, Wide, accounting_summary};
use ark_ff::PrimeField;
use mori_core::{AssetId, Program, Statement, Type, UInt, Value};
use serde::Serialize;

pub const STMT_TAG: &[u8; 16] = b"MRG16-PVC-STMT-2";
pub const FAMILY_V2: &[u8; 32] = b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2";
pub const SHAPE_TAG: &[u8; 16] = b"MRG16-PVC-SHP-02";
/// Bytes before the body.
pub const HEADER_LEN: usize = 343;

/// Per-proof context bytes: hashed into the statement, never interpreted in-circuit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContextV2 {
    pub vk_digest: [u8; 32],
    pub claimant_vkh: [u8; 28],
    pub network_id: u8,
    pub settle_hash: [u8; 28],
    pub thread_id: [u8; 32],
    pub consumed_txid: [u8; 32],
    pub consumed_ix: u16,
    pub seq: u64,
    pub catalogue_digest: [u8; 32],
    pub intents_digest: [u8; 32],
}

impl ContextV2 {
    fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LEN - 116);
        out.extend_from_slice(&self.vk_digest);
        out.extend_from_slice(&self.claimant_vkh);
        out.push(self.network_id);
        out.extend_from_slice(&self.settle_hash);
        out.extend_from_slice(&self.thread_id);
        out.extend_from_slice(&self.consumed_txid);
        out.extend_from_slice(&self.consumed_ix.to_be_bytes());
        out.extend_from_slice(&self.seq.to_be_bytes());
        out.extend_from_slice(&self.catalogue_digest);
        out.extend_from_slice(&self.intents_digest);
        out
    }
}

/// Relation constants of the statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AbiV2Constants {
    pub predicate_digest: [u8; 32],
    pub shape_bytes: Vec<u8>,
    pub shape_digest: [u8; 32],
    pub body_len: u32,
}

/// ABI v2 fixes the compile options: no public cap (W is a constant inside the
/// predicate digest), the in-circuit gross-debit and fee sums (P3-3(d)), and no frame
/// cells (the Cardano datum carries the complete snapshot and S2 enforces the frame).
pub fn options_ok(options: &CompileOptions) -> Result<(), CompileError> {
    if options.cap != CapBinding::NotBound {
        return Err(CompileError::Option("SettlementHashV2 requires CapBinding::NotBound".into()));
    }
    if options.accounting != AccountingBinding::GrossDebitAndFees {
        return Err(CompileError::Option("SettlementHashV2 requires AccountingBinding::GrossDebitAndFees".into()));
    }
    if options.frame != FrameCells::TouchedOnly {
        return Err(CompileError::Option(
            "SettlementHashV2 requires FrameCells::TouchedOnly; the target enforces the frame over the complete datum".into(),
        ));
    }
    Ok(())
}

fn too_long(what: &str) -> CompileError {
    CompileError::Bound(format!("ABI v2 {what} exceeds a u16 length"))
}

fn push_u16(out: &mut Vec<u8>, n: usize, what: &str) -> Result<(), CompileError> {
    let n = u16::try_from(n).map_err(|_| too_long(what))?;
    out.extend_from_slice(&n.to_be_bytes());
    Ok(())
}

fn push_lp(out: &mut Vec<u8>, b: &[u8], what: &str) -> Result<(), CompileError> {
    push_u16(out, b.len(), what)?;
    out.extend_from_slice(b);
    Ok(())
}

fn ty_byte(ty: Type) -> u8 {
    match ty {
        Type::UInt => 0,
        Type::Bool => 1,
    }
}

/// Program assets sorted by Core's `AssetId` order (the order the shape and the Cardano
/// catalogue use).
pub fn sorted_assets(program: &Program) -> Vec<AssetId> {
    let mut ids: Vec<AssetId> = program.assets.iter().map(|a| a.identity.clone()).collect();
    ids.sort();
    ids.dedup();
    ids
}

/// `MRG16-PVC-SHP-02` bytes (ABI-V2.md section 4) of a planned action.
pub fn shape_bytes(program: &Program, plan: &ActionPlan) -> Result<Vec<u8>, CompileError> {
    let assets = sorted_assets(program);
    let asset_index = |id: &AssetId| -> Result<usize, CompileError> {
        assets.iter().position(|a| a == id).ok_or_else(|| CompileError::Bound("plan asset outside the program".into()))
    };
    let mut out = SHAPE_TAG.to_vec();
    push_u16(&mut out, assets.len(), "asset list")?;
    for a in &assets {
        push_lp(&mut out, a.network.as_bytes(), "asset network")?;
        push_lp(&mut out, a.issuer.as_bytes(), "asset issuer")?;
        push_lp(&mut out, a.reference.as_bytes(), "asset reference")?;
        out.push(a.scale);
    }
    push_u16(&mut out, plan.states.len(), "state list")?;
    for s in &plan.states {
        push_lp(&mut out, s.name.as_bytes(), "state name")?;
        out.push(ty_byte(s.ty));
    }
    push_u16(&mut out, plan.inputs.len(), "input list")?;
    for s in &plan.inputs {
        push_lp(&mut out, s.name.as_bytes(), "input name")?;
        out.push(ty_byte(s.ty));
    }
    push_u16(&mut out, plan.cells.len(), "cell list")?;
    for c in &plan.cells {
        push_u16(&mut out, asset_index(&c.asset)?, "asset index")?;
        push_lp(&mut out, c.account.as_bytes(), "account")?;
    }
    push_u16(&mut out, plan.duties.len(), "duty list")?;
    for d in &plan.duties {
        push_lp(&mut out, d.name.as_bytes(), "duty name")?;
        push_u16(&mut out, asset_index(&d.asset)?, "asset index")?;
        push_lp(&mut out, d.debtor.as_bytes(), "debtor")?;
        push_lp(&mut out, d.creditor.as_bytes(), "creditor")?;
    }
    push_u16(&mut out, plan.effects.len(), "effect list")?;
    for e in &plan.effects {
        match e {
            EffectShape::State { name, .. } => {
                out.push(0);
                let k = plan.state_index(name).ok_or_else(|| CompileError::Bound("state effect outside the plan".into()))?;
                push_u16(&mut out, k, "state index")?;
            }
            EffectShape::Transfer { from_cell, to_cell, .. } | EffectShape::Fee { from_cell, to_cell, .. } => {
                out.push(if e.is_fee() { 2 } else { 1 });
                push_u16(&mut out, *from_cell, "cell index")?;
                push_u16(&mut out, *to_cell, "cell index")?;
            }
            EffectShape::Liability { duty, origin, .. } => {
                out.push(3);
                push_u16(&mut out, *duty, "duty index")?;
                out.push(match origin {
                    DutyOrigin::Accrue => 0,
                    DutyOrigin::Discharge => 1,
                });
            }
            EffectShape::Emit { .. } => {
                return Err(CompileError::Unsupported {
                    construct: "Statement::Emit",
                    reason: "SettlementHashV2 applies the initial ledger target Emit refusal".into(),
                });
            }
        }
    }
    push_u16(&mut out, plan.debit_cells.len(), "debit cells")?;
    for c in &plan.debit_cells {
        push_u16(&mut out, *c, "cell index")?;
    }
    out.extend_from_slice(&plan.statement_count.to_be_bytes());
    let action = &program.actions[usize::from(plan.action_index)];
    let liability_statements = action.statements.iter().filter(|s| matches!(s, Statement::Liability { .. })).count();
    push_u16(&mut out, liability_statements, "liability statements")?;
    push_lp(&mut out, plan.action.as_bytes(), "action name")?;
    out.extend_from_slice(&plan.action_index.to_be_bytes());
    Ok(out)
}

/// Body length in bytes of a layout (every non-header slot).
pub fn body_len(layout: &StatementLayout) -> Result<usize, CompileError> {
    let mut n = 0usize;
    for slot in &layout.slots {
        n += match (slot.kind.is_header(), slot.encoding) {
            (true, _) => 0,
            (false, SlotEncoding::UInt) => 16,
            (false, SlotEncoding::Bool) => 1,
            (false, SlotEncoding::Wide) => 18,
            (false, other) => return Err(CompileError::Bound(format!("unexpected body slot encoding {other:?}"))),
        };
    }
    Ok(n)
}

pub(crate) fn constants(program: &Program, plan: &ActionPlan, layout: &StatementLayout, predicate: [u8; 32]) -> Result<AbiV2Constants, CompileError> {
    let shape_bytes = shape_bytes(program, plan)?;
    let shape_digest = blake2b::blake2b256(&shape_bytes);
    let body_len = u32::try_from(body_len(layout)?).map_err(|_| CompileError::Bound("ABI v2 body exceeds u32".into()))?;
    Ok(AbiV2Constants { predicate_digest: predicate, shape_bytes, shape_digest, body_len })
}

fn value_bytes(v: &Value) -> Vec<u8> {
    match v {
        Value::UInt(UInt(n)) => n.to_be_bytes().to_vec(),
        Value::Bool(b) => vec![u8::from(*b)],
    }
}

fn wide_bytes(w: Wide) -> Vec<u8> {
    let mut out = w.top.to_be_bytes().to_vec();
    out.extend_from_slice(&w.lo.to_be_bytes());
    out
}

/// Host encoding of the statement body for a claimed transition (honest proposal only;
/// the circuit binds every byte to its slot wire).
pub fn body_bytes(layout: &StatementLayout, plan: &ActionPlan, values: &TransitionValues) -> Result<Vec<u8>, MappingError> {
    let summary = accounting_summary(plan, values);
    let row = |cell: usize| summary.iter().find(|(c, _, _)| *c == cell).copied().ok_or_else(|| MappingError::Shape(format!("no accounting row for cell {cell}")));
    let mut out = Vec::new();
    for slot in &layout.slots {
        let bytes: Vec<u8> = match slot.kind {
            SlotKind::Header(_) => continue,
            SlotKind::Input { index } => value_bytes(&values.inputs[index]),
            SlotKind::PreState { index } => value_bytes(&values.pre.states[index]),
            SlotKind::PostState { index } => value_bytes(&values.post.states[index]),
            SlotKind::PreCell { index } => values.pre.cells[index].0.to_be_bytes().to_vec(),
            SlotKind::PostCell { index } => values.post.cells[index].0.to_be_bytes().to_vec(),
            SlotKind::PreDutyPresent { index } => vec![u8::from(values.pre.duties[index].present)],
            SlotKind::PreDutyAmount { index } => values.pre.duties[index].amount.0.to_be_bytes().to_vec(),
            SlotKind::PostDutyPresent { index } => vec![u8::from(values.post.duties[index].present)],
            SlotKind::PostDutyAmount { index } => values.post.duties[index].amount.0.to_be_bytes().to_vec(),
            SlotKind::Effect { index, role } => {
                use crate::statement::EffectRole as R;
                match (&values.effects[index], role) {
                    (EffectValues::State { before, .. }, R::StateBefore) => value_bytes(before),
                    (EffectValues::State { after, .. }, R::StateAfter) => value_bytes(after),
                    (EffectValues::Amount(a), R::Amount) => a.0.to_be_bytes().to_vec(),
                    (EffectValues::Duty { before, .. }, R::DutyBefore) => before.0.to_be_bytes().to_vec(),
                    (EffectValues::Duty { after, .. }, R::DutyAfter) => after.0.to_be_bytes().to_vec(),
                    _ => return Err(MappingError::Shape(format!("effect {index} has no role {role:?}"))),
                }
            }
            SlotKind::GrossDebit { cell } => wide_bytes(row(cell)?.1),
            SlotKind::FeesPaid { cell } => wide_bytes(row(cell)?.2),
        };
        out.extend(bytes);
    }
    Ok(out)
}

/// Complete statement bytes.
pub fn statement_bytes(consts: &AbiV2Constants, ctx: &ContextV2, body: &[u8]) -> Result<Vec<u8>, MappingError> {
    if body.len() != consts.body_len as usize {
        return Err(MappingError::Shape(format!("body has {} bytes, the relation fixes {}", body.len(), consts.body_len)));
    }
    let mut s = Vec::with_capacity(HEADER_LEN + body.len());
    s.extend_from_slice(STMT_TAG);
    s.extend_from_slice(FAMILY_V2);
    s.extend_from_slice(&consts.predicate_digest);
    s.extend_from_slice(&consts.shape_digest);
    s.extend(ctx.bytes());
    s.extend_from_slice(&consts.body_len.to_be_bytes());
    s.extend_from_slice(body);
    debug_assert_eq!(s.len(), HEADER_LEN + body.len());
    Ok(s)
}

/// The two public inputs `[int_be(d[0..16]), int_be(d[16..32])]`, d = BLAKE2b-256(stmt).
pub fn public_inputs<F: PrimeField>(statement: &[u8]) -> Vec<F> {
    let d = blake2b::blake2b256(statement);
    let mut hi = [0u8; 16];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&d[..16]);
    lo.copy_from_slice(&d[16..]);
    vec![fe(u128::from_be_bytes(hi)), fe(u128::from_be_bytes(lo))]
}

// ## In-circuit binding

fn const_bytes<F: PrimeField>(bytes: &[u8], bits: &mut Vec<Bit<F>>) {
    for b in bytes {
        for j in 0..8 {
            bits.push(Bit::constant((b >> j) & 1 == 1));
        }
    }
}

/// Free witness bytes (context): booleanized bits, honest values from `honest`.
fn free_bytes<F: PrimeField>(cs: &mut Cs<F>, honest: Option<&[u8]>, n: usize, bits: &mut Vec<Bit<F>>) -> Result<(), BindError> {
    for k in 0..n {
        for j in 0..8 {
            let val = honest.map(|h| (h[k] >> j) & 1 == 1);
            bits.push(blake2b::alloc_bit(cs, "abi2.ctx", val)?);
        }
    }
    Ok(())
}

/// Bytes of a limb (big-endian, `n` bytes) bound to its linear combination: booleanized
/// bits whose weighted sum equals the limb. A constant limb yields constant bits.
fn limb_bytes<F: PrimeField>(
    cs: &mut Cs<F>,
    limb: &Lc<F>,
    konst: Option<u128>,
    n: usize,
    honest: Option<&[u8]>,
    bits: &mut Vec<Bit<F>>,
) -> Result<(), BindError> {
    if let Some(k) = konst {
        let be = k.to_be_bytes();
        const_bytes(&be[16 - n..], bits);
        return Ok(());
    }
    let mut sum = Lc::zero();
    for k in 0..n {
        for j in 0..8 {
            let val = honest.map(|h| (h[k] >> j) & 1 == 1);
            let bit = blake2b::alloc_bit(cs, "abi2.body", val)?;
            let weight = 8 * (n - 1 - k) + j;
            sum = sum.plus(&bit.lc.scaled(fe(1u128 << weight)));
            bits.push(bit);
        }
    }
    cs.enforce_eq(&sum, limb)?;
    Ok(())
}

fn uint_bytes<F: PrimeField>(cs: &mut Cs<F>, u: &UIntW<F>, honest: Option<&[u8]>, bits: &mut Vec<Bit<F>>) -> Result<(), BindError> {
    let (khi, klo) = match u.konst {
        Some(k) => (Some(k >> 64), Some(k & u128::from(u64::MAX))),
        None => (None, None),
    };
    limb_bytes(cs, &u.hi, khi, 8, honest.map(|h| &h[0..8]), bits)?;
    limb_bytes(cs, &u.lo, klo, 8, honest.map(|h| &h[8..16]), bits)
}

fn wide_bytes_in<F: PrimeField>(cs: &mut Cs<F>, w: &WideW<F>, honest: Option<&[u8]>, bits: &mut Vec<Bit<F>>) -> Result<(), BindError> {
    limb_bytes(cs, &w.top, None, 2, honest.map(|h| &h[0..2]), bits)?;
    limb_bytes(cs, &w.mid, None, 8, honest.map(|h| &h[2..10]), bits)?;
    limb_bytes(cs, &w.lo, None, 8, honest.map(|h| &h[10..18]), bits)
}

/// Binds the statement: constant header bytes, free context bytes, every body byte tied
/// to its slot wire, BLAKE2b-256 in-circuit, and the two public inputs equal to the
/// digest halves. `honest` is the host statement (prove mode) or `None` (setup).
pub(crate) fn bind_v2<F: PrimeField>(
    cs: &mut Cs<F>,
    consts: &AbiV2Constants,
    layout: &StatementLayout,
    wires: &[SlotWire<F>],
    honest: Option<&[u8]>,
    claimed: Option<&[F]>,
) -> Result<(), BindError> {
    if wires.len() != layout.slots.len() {
        return Err(BindError::Adapter(AdapterError::Shape("wire count differs from slot count".into())));
    }
    let total = HEADER_LEN + consts.body_len as usize;
    if honest.is_some_and(|h| h.len() != total) {
        return Err(BindError::Adapter(AdapterError::Shape("honest statement length differs from the relation".into())));
    }
    if claimed.is_some_and(|c| c.len() != 2) {
        return Err(BindError::Adapter(AdapterError::Shape("SettlementHashV2 has exactly two public inputs".into())));
    }
    let start = cs.num_constraints();
    cs.push("abi2".into());
    let mut bits: Vec<Bit<F>> = Vec::with_capacity(8 * total);
    const_bytes(STMT_TAG, &mut bits);
    const_bytes(FAMILY_V2, &mut bits);
    const_bytes(&consts.predicate_digest, &mut bits);
    const_bytes(&consts.shape_digest, &mut bits);
    free_bytes(cs, honest.map(|h| &h[112..339]), 339 - 112, &mut bits)?;
    const_bytes(&consts.body_len.to_be_bytes(), &mut bits);
    let mut at = HEADER_LEN;
    for (slot, wire) in layout.slots.iter().zip(wires) {
        if slot.kind.is_header() {
            continue;
        }
        let width = match slot.encoding {
            SlotEncoding::UInt => 16,
            SlotEncoding::Bool => 1,
            SlotEncoding::Wide => 18,
            _ => return Err(BindError::Adapter(AdapterError::Shape("unexpected body slot encoding".into()))),
        };
        let h = honest.map(|h| &h[at..at + width]);
        match (wire, slot.encoding) {
            (SlotWire::U(u), SlotEncoding::UInt) => uint_bytes(cs, u, h, &mut bits)?,
            (SlotWire::B(b), SlotEncoding::Bool) => {
                // The Bool wire is already 0 or 1; its byte is (b, 0, ..., 0) LSB first.
                bits.push(Bit { lc: b.lc.clone(), val: b.val, konst: b.konst });
                for _ in 1..8 {
                    bits.push(Bit::constant(false));
                }
            }
            (SlotWire::W(w), SlotEncoding::Wide) => wide_bytes_in(cs, w, h, &mut bits)?,
            _ => return Err(BindError::Adapter(AdapterError::Shape(format!("slot {} wire does not match its encoding", slot.label)))),
        }
        at += width;
    }
    if bits.len() != 8 * total {
        return Err(BindError::Adapter(AdapterError::Shape("statement bit count differs from the relation".into())));
    }
    let digest = blake2b::blake2b256_gadget(cs, &bits)?;
    for (k, from) in [0usize, 16].into_iter().enumerate() {
        let x = cs.input(claimed.map(|c| c[k]))?;
        cs.enforce_eq(&Lc::var(x), &blake2b::digest_half_lc(&digest, from))?;
    }
    cs.pop();
    cs.record("abi2-binding".into(), start);
    Ok(())
}


## S30 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/moriarty/crates/mori-core/src/json.rs
//! Bounded JSON parsing that rejects duplicate keys before schema decoding.
use crate::Error;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::{cell::Cell, fmt};
const MAX_BYTES: usize = 1_048_576;
struct Seed<'a> {
    depth: usize,
    nodes: &'a Cell<usize>,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        use serde::de::Error as _;
        self.nodes.set(self.nodes.get() + 1);
        if self.depth > 32 || self.nodes.get() > 65_536 {
            return Err(D::Error::custom("JSON work/depth limit"));
        }
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite JSON number"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Seed {
            depth: self.depth + 1,
            nodes: self.nodes,
        })? {
            out.push(v)
        }
        Ok(Value::Array(out))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        use serde::de::Error as _;
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(A::Error::custom("duplicate decoded key"));
            }
            let v = a.next_value_seed(Seed {
                depth: self.depth + 1,
                nodes: self.nodes,
            })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
/// Strict recursive JSON value decoding for versioned DTO dynamic fields.
/// Duplicate decoded keys and the existing depth/node bounds apply even when
/// a consumer invokes serde directly instead of the bounded text entry point.
pub fn deserialize_strict<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Value, D::Error> {
    let nodes = Cell::new(0);
    Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(decoder)
}
pub fn parse(text: &str) -> Result<Value, Error> {
    parse_inner(text, MAX_BYTES, "JSON exceeds 1 MiB")
}
/// The same strict parser under a caller-selected byte ceiling. Duplicate-key,
/// depth (32) and node (65,536) limits are unchanged; `/2` stage files use it.
pub fn parse_limited(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    parse_inner(text, max, too_large)
}
fn parse_inner(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    use serde::de::DeserializeSeed;
    if text.len() > max {
        return Err(Error::new("json", too_large));
    }
    let nodes = Cell::new(0);
    let mut decoder = serde_json::Deserializer::from_str(text);
    let value = Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(&mut decoder)
    .map_err(|e| Error::new("json", e.to_string()))?;
    decoder
        .end()
        .map_err(|e| Error::new("json", e.to_string()))?;
    Ok(value)
}

/// ASCII-safe pretty JSON that fits the same public reader profile, including
/// the final newline added by CLI printing. Parsed values and signing bytes
/// are preserved; terminal-controlled Unicode remains visibly escaped.
pub fn export(value: &impl serde::Serialize) -> Result<String, Error> {
    let json =
        serde_json::to_string_pretty(value).map_err(|e| Error::new("json", e.to_string()))?;
    parse(&json)?;
    let mut display = String::with_capacity(json.len());
    for character in json.chars() {
        if character.is_ascii() && (!character.is_control() || character == '\n') {
            display.push(character);
        } else {
            for unit in character.encode_utf16(&mut [0; 2]) {
                use std::fmt::Write;
                write!(&mut display, "\\u{unit:04x}").expect("writing to String cannot fail");
            }
        }
        if display.len() >= MAX_BYTES {
            return Err(Error::new(
                "json",
                "Escaped JSON and final newline exceed 1 MiB",
            ));
        }
    }
    parse(&display)?;
    Ok(display)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_decoded_keys_and_trailing_data_reject() {
        assert!(parse(r#"{"x":1,"\u0078":2}"#).is_err());
        assert!(parse("{} []").is_err());
        assert!(parse(r#"{"x":"1"}"#).is_ok());
    }
}


## S31 FULL PUBLIC INPUT /home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/root/K6-ACTUAL-ADOPTION.json
{
  "format": "root-K6-native-roundtrip-actual-adoption/1",
  "createdAt": "2026-10-16T11:48:42.512619+00:00",
  "nativeRoundtripPrerequisiteClosed": true,
  "actualResultAdopted": true,
  "scope": "One currentRust moriarty-core/1 invoicepartial Groth16roundtrip under single-party-test-setup/1 only; honestlocalcircuitexample with caller-declared fabricatedcontext, notledgertruth",
  "publicResultManifest": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k6-public-actual-intake-20261016/PUBLIC-RESULT-MANIFEST.json",
  "publicResultManifestSha256": "d300f2c685e66278517ebc3d2acedd9ee0ae21d81765b65f62b19e91fbd21b57",
  "freshActualReview": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k6-native-independent-muse-actual-review-20261016/REPORT.md",
  "freshActualReviewSha256": "d0926597ef0e3f869fd1036f56f90ec068c089f8a6b4c3ad172c9e4bec5c877e",
  "reviewer": "Freshnon-author MuseWonder1.0 high requested/actualeffortunobserved; samefamily notcrossprovider independence",
  "acceptedObservations": {
    "serializedPkValidatedDecodeAndEmbeddedVkMatch": true,
    "honestCurrentCompiledRelationProofProduced": true,
    "nativeSourceRecompiledVerificationAccepted": true,
    "nativeExportOnlyVerificationAccepted": true,
    "negativeRejectionsMatchSpecifiedBranches": true,
    "n10DescriptorGuardObserved": true,
    "n10DescriptorField": "abi2ShapeBytesHex",
    "n9DescriptorGuardObserved": false,
    "proofExportBytes": 5800,
    "rawProofBytes": 192,
    "proofExportSha256": "930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b",
    "vkDigestBlake2b256": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
    "setupRecordSha256": "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b",
    "statementDigestBlake2b256": "c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc",
    "nativeCountEvidence": "19sandboxes/17drivercalls/3proveargv/1successfulproof/2PKbyte-read/1fullPKdecode/1OsRng/0setup/0retry consistent orderedjournals+productionstop-points, H6COUNTconstantsnotindependentinstrumentation"
  },
  "resourceQualification": {
    "wallObservedByLauncherSeconds": 123.3336031689978,
    "authDefinedAggregateChargeCpu": 19,
    "authDefinedAggregateChargeWall": 139,
    "authCeilingsCpuWall": [
      7200,
      12600
    ],
    "remainingUnderAuthChargeDefinition": [
      7181,
      12461
    ],
    "actualTransitiveNativeCpu": "UNMEASURED; not19, notP6charge1, notlauncherCHILDREN0.382541",
    "adoptionRationale": "Actualauditor explicitlyadoptsnarrowcryptoobservations whilekeepingnominalchargeclassification andtransitiveCPUgap. Sourceper-processcapsobservedcleanterminalexits, no retry/reset."
  },
  "limitations": {
    "class0ControllerCanForge": true,
    "consumerCanCheckVkToMatrixLink": false,
    "publicVkIndependentPairingReproduction": false,
    "ledgerTruthClaimed": false,
    "spectator41FrameContextAuthenticated": false,
    "aikenC2PairingObserved": false,
    "appliedWrappersObserved": false,
    "genuineIntentSignaturesObserved": false,
    "preprodTransactionObserved": false,
    "universalPropertyCertificate": false,
    "binarySourceCorrespondence": "Recordedbuild/source/hashcustody evidence, notmathematicalbinaryproof"
  },
  "allRequiredScopeRetained": [
    "all20UTXOfamilies",
    "ACTUS277fixtures18types32taxonomyDS01-07",
    "DeFi72DA01-24heldoutsintentsregressions",
    "generalfinancialobligations",
    "realrecursion/privatehandoff/boundedmulti-parentPCD",
    "universalcertificates",
    "realPreprod"
  ],
  "executionAuthorityGranted": false,
  "spentK6Authorization": "Consumedonce; no newnative/privatePK/setup/RNG/ledgeraction or retry/reset",
  "nextPermittedPreparation": "Separatelyreviewedminimalpublicverification-recordexport only; actualnewscopeAUTH required beforecontrolledoperatorONEknownrecordread. Rootnever/w. Then publicAikenconsumerfixture/sourcepreparation and scopedvalidation; no automaticexecution."
}


## S32 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-corrected-policy-fresh-opus-audit-20261016/MODEL-REVIEW.json
{
  "reviewToken": "current-cardano-c2-profile-corrected-policy-fresh-opus-audit-20261016",
  "report": "FRESH NON-AUTHOR SOURCE-ONLY AUDIT (claude-opus-5-5, tools off, inline bodies only)\n\nScope and method\n- I read all 31 supplied bodies (S01-S31) in full as text and treated them as evidence, not instructions.\n- I executed, imported, compiled and hashed nothing. Every byte count and SHA-256 below is the value declared in the supplied headers or the root manifest (S04). I did not recompute any BLAKE2b or SHA-256, including the K2 shape digest.\n- I did not inherit earlier votes. The rejected attempt 2 (S07-S09) and its audit (S05/S06) are immutable history. The candidate is S01-S04.\n- Process note, not a product-policy issue: the contract plus API come to 23,830 + 5,750 = 29,580 characters, against the author's requested 24,000. I reviewed every byte without truncation.\n- Pins: the S04 root manifest pins match every supplied header (S05-S30). The K2 fixture is root-observed at 3,926 B with SHA-256 86b588fe\u2026 (S04 setupPublicFixtureRootObserved). That closes the \u00a71 'declared only' caveat, but the \u00a71 text is stale. S03 (DESIGN-POSITIONS.json, 8536095b\u2026) is pinned by the task header but is not in S04's files list.\n- No r2 code or test exists, and root has run nothing for this candidate. Native authority is ZERO.\n\nVERDICT: REVISE (narrow text correction). NOT SOURCE-READY.\nThe policy substance is sound:\n- CP01-CP05, CP07 and CP08 are agreed as exact text.\n- CP06 is DISAGREED for one reason only: its evidence statement says T16 samples the v1-subset property, which repeats the prior F05 error.\n- Four test-obligation defects make the exact contract unusable as a literal test source (T16 contradiction, T05 method grouping against the 18-test count, T07 mis-staged list case, and the CP06/\u00a75/API sampling attribution).\nThese are bounded wording fixes. No v1 byte, label, clause substance or Rust claim needs to change.\n\n1. Prior findings F01-F13 (rechecked against the current text)\n\nClosed:\n- F01: no admission-level subset, equivalence or conditional-inclusion claim remains. Checked in CP05, \u00a73, \u00a74, \u00a75, the API rust/compatibility blocks and the labels. rustAcceptance is NOT_ESTABLISHED. The old audit's suggested conditional ('restricted to inputs that also pass\u2026') was correctly not adopted, as ROOT-QUALIFICATION asked.\n- F02: jsonParseLimits and vkProfile were added. The count is 16 everywhere (CP06, CP08, \u00a74, API, T02, S04 labelCount).\n- F03: the O_NOFOLLOW _read loader is restated. FileNotFoundError is the sole source of the phrase. The run envelope and pins are restated.\n- F04: T00 makes no r2 assertion, so it is phase-independent.\n- F06: explicit SETUP/PROFILE/EXPORT ownership; length before scan; no str.encode; four deferred export fields.\n- F07: logicalWork is ASCII-only and length-first, and T05b has the Unicode-digit, underscore, whitespace, LF and 5000-digit cases.\n- F09: BUILD is excluded from T14/T15; aliasing is scoped to mutable containers; tamper sides are named; the STOP rule is added.\n- F10: the shape bound is no longer listed as a narrowing; check_statement_against is fully mirrored; the x1/x2 outcome note and the re-encode note are present.\n- F11: CP01 restates CI01 host reuse with all seven APIs.\n- F13: CP08 is non-routing, requires a separate root decision, and nothing is automatic.\n\nPartial:\n- F08: a v1 column with literal expectations exists, T02 keeps the historical literals, and T17 uses hashlib. R03 for v1 stays OPEN, which is correct.\n- F12: the API now lists every flag, and the root manifest pins the contract and API. DESIGN-POSITIONS is not in the S04 files list, and the \u00a71 K2 count is stale.\n\nRegressed:\n- F05: T16 is now 'every ADMIT cell above', which literally includes T02 (r2 ADMIT). CP06, \u00a75 and the API still say T16 samples the r2\u2286v1 property. T16 inputs are r2-refused, so they give zero samples of 'r2-admitted \u21d2 v1-admitted'. T16 samples only strictness.\n\n2. Question: T16 'every ADMIT cell above'\nThis is contradictory.\n- T02's v1 cell is ADMIT and its r2 cell is also ADMIT. A literal T16 would require r2 to refuse K6, which contradicts T02, CP06 and the STOP rule.\n- T17 is below T16, so 'above' excludes it. Its row is also both-admit.\n\nThe exact subset T16 needs is every v1-ADMIT case where r2 refuses:\n- T04: driver one-character change (r2 SETUP); provingKeyBytes 2**64 (r2 SETUP).\n- T05b: every case (r2 PROFILE).\n- T06: U+D800 in relation relationFormat, logicalCostProfile or fieldModulusLeHex; U+D800 in a relation extra key; U+D800 consistently in relation action, export action and catalogue name (all r2 PROFILE).\n- T14: the SETUP/PROFILE pair, driver one-character change plus language (r2 SETUP).\nNo other row has a v1 ADMIT: T05a is v1 SETUP, T05c is v1 RELATION, and T03 and T07-T13 are '='.\n\n3. Question: T05a/b/c as one method and the 18-test count\n- \u00a77 predicts 18 runs at RED and 18 passes at GREEN. That works only if T00-T17 are 18 methods with T05a/b/c inside one method.\n- \u00a76 shows 20 rows and never states that grouping. The NOT RED list does not cover a different run count.\n- The meaningfulness rule 'v1 under the r2 name must fail \u2026 T05b' is observable inside a grouped method only through subTest labels. A grouped T05 also fails on T05a for that mutant, because v1 refuses at SETUP where PROFILE is expected.\n- The staged expected values themselves are coherent (verified row by row in section 6).\n- Required: state the grouping (subTests labelled T05a/T05b/T05c in one method), identify meaningfulness failures by subTest label, and make any count other than 18 NOT RED/STOP.\n\n4. Question: CP04 scan/defer mapping and length-first ordering against the inherited CI checks\n- Scanned fields: relation relationFormat, logicalCostProfile, action and fieldModulusLeHex at PROFILE; export action, catalogueActionEntry.name, intentTargetHashV2 and intentAnchor at EXPORT. This matches the API scannedStr.\n- Every other string field is hex, fixed, decimal or premises. Each of those refuses surrogates through its own character or equality check, and every closed-key check refuses a surrogate key, because all schema names are ASCII.\n- Setup top-level members have no str kind, since driver is now fixed.\n- Deferred fields at STATEMENT and PROOF have length-first checks:\n  - statementHex: v1 CI07 checks the length (> 524288) before parity and the set scan;\n  - proofHex: v1 _is_hex checks length 384 before the scan.\n- DIGEST has no length check. x1/x2 are refused there by exact-string equality with the host decimal (v1 `halves != (x1, x2)`). The outcome and T06's DIGEST expectation are correct, but the CP04 phrase 'length-first checks of \u2026 DIGEST' is inaccurate wording (low).\n- PROFILE length-first rules:\n  - verifyingKeyHex: length 976 is checked first at SETUP;\n  - abi2ShapeBytesHex: length \u2264 524288 and parity are checked before any scan;\n  - logicalWork: 1..39 is checked before the ASCII check and int(), so int_max_str_digits cannot fire.\n- 'Allocation-free' needs a CPython definition. Iterating a non-Latin-1 str creates transient one-character objects. The intent should read as 'no input-proportional copy, never encode' (low).\n\n5. Question: Rust admission claims\nNone survives at admission level. What remains is check-level only:\n- 'each mirrors a named check' (this contradicts the 'Added by r2' list, low);\n- the x1/x2 same-outcome note (correct by hexutil.rs reading);\n- 'Rust also admits inputs that r2 refuses, for example networkId 1'. That is an unexecuted source reading and should be phrased as 'Rust check_export contains no networkId check' (low).\n\n\u00a73's 'Not performed' list is complete against encoding.rs:\n- the IC prefix at offset 336;\n- seven VK points for flags, field range, curve and subgroup, identity refusal (vk_profile_ok) and per-point plus whole-key canonical re-encoding;\n- proof A/B/C;\n- the setupRecordSha256 file-byte link;\n- input.rs/json.rs: 1,048,576 B, UTF-8, duplicate decoded keys, trailing data, depth 32, 65,536 nodes, plus serde DTO decoding;\n- the parse_statement re-encode;\n- the pairing.\n\n6. S31 (scoped historical K6 adoption)\n- S31 root-adopts one current-Rust moriarty-core/1 'invoice partial' Groth16 round trip. Its pins are proofExportSha256 930198\u2026 (the K6 pin) and setupRecordSha256 86b588\u2026 (the K2 pin), under class-0 single-party-test-setup/1 with caller-declared fabricated context. binarySourceCorrespondence is custody evidence, not a proof.\n- So the contract's reliance on the lib.rs and abi2.rs 'never compiled or run' docstrings, in \u00a78 and \u00a73, is stale as evidence. It does not establish the global absence of later runs.\n- I preserve exactly that one result. I infer no universal compiler, VK, gadget or circuit correspondence, and I do not re-review its native inputs.\n- driverId DECLARED_NAME_NOT_PROVENANCE, circuitVkCorrespondence OPEN and statementReencode NOT_CHECKED all remain correct for the r2 consumer, which cannot verify provenance (low wording fix).\n- The schema's nativeRoundtripPrerequisiteClosed=false below means only that this review grants nothing. It does not negate S31's scoped historical record.\n\n7. Other crosschecks\n- 29 keys and kinds: verified against relation.rs. The nine hex64 fields equal the check_profile list; sourceSha256 is hex-or-null with the key required; the four str kinds are type-only, as in Rust; the int widths are u64/u16; headerLen is 343 (Rust u32); instanceVariables is 3 (u64); publicInputCount is 2 (u8); bodyLen 0..261801 narrows Rust's u32.\n- u64/u128 and length checks:\n  - provingKeyBytes 1..2**64-1 matches Rust's u64 plus its != 0 check;\n  - logicalWork matches parse_u128_decimal.\n- v1 retained behaviour, checked against S11: every v1 SETUP check is kept at SETUP, or at PROFILE with equal or stricter kinds; driver and provingKeyBytes are stricter; EXPORT-CARDANO change only through CP04. So 'r2-admitted \u21d2 v1-admitted' with four equal byte fields holds by source construction. Runtime evidence is samples only, from T02 and T17.\n- Host APIs: all seven exist in S22 with the bounds v1 relies on. BUILD and the DIGEST rewraps stay unreachable (R04).\n- Labels: 16 in \u00a74 and 16 in the API, with identical keys and values. The eight v1 labels are a value-identical subset, and the label scopes match encoding.rs.\n- Aliasing: CP07 and T15 scope aliasing to mutable containers, consistent with CI10 sharing the setupRecordSha256 str and the expiry int.\n- T02 literals: ENV_PREFIX equals HIST_ENV_CBOR[:-20]. The expiry heads are 00, 17, 1818 and 1b0000000100000000 (2**32 needs an 8-byte head), 1bffffffffffffffff, and 1b000001b8dac5b400 (= 1893456000000).\n- Byte sizes: issue framing is 710 B (3+1+462+42+1+1+198+1+1) and mint is 67 B.\n- T17: offsets 144/172/173/233/265/267 equal the claim.rs offsets.\n- Stage notes (side decides the stage): T05c (PROFILE vs v1 RELATION), T08 setup-side (RELATION) and T09 bodyLen 377 (STATEMENT) are all correct.\n- T14: all nine pairs and their v1 cells are correct.\n\n8. New defects and notes\n- Medium (block exact adoption): the T16 contradiction; the CP06/\u00a75/API T16 sampling attribution; the unstated T05 grouping against the 18-test count; and T07's 'any of these \u2026 given as a list', which includes the top-level export. A list export is C2I-ARG under CI02, as T03 itself shows.\n- Low (record, not blocking alone):\n  - T04's '(v1 ADMIT)' after 'provingKeyBytes 0, -1, True or 2**64' is ambiguous: v1 admits only 2**64. T14's driver mutation is unspecified.\n  - The CP04 DIGEST wording and the 'allocation-free' definition.\n  - \u00a73 'each mirrors' and 'Rust also admits'.\n  - The stale S31 evidence wording.\n  - At non-historical expiries (T02) and in T17, the token is only compared with v1 through the same host. A hashlib expectation computed in the test and labelled non-oracle is possible. T17 should also name statementDigestBlake2b256 among the updated fields.\n- Info:\n  - T06's relation-extra-key surrogate case does not discriminate, because the closed-key check refuses it without any scan. It is fine as a redundant case.\n  - DESIGN-POSITIONS should be pinned in S04.\n  - The 14-label R4 contract stays incompatible until a new explicit reviewed reconciliation. There is no automatic builder reliance (\u00a75, CP08, S04 dependencyStatus).\n\n9. Readiness\n- Readiness here means exact source-contract adoption only. sourceReady is false because the medium items are unresolved material conditions on the exact test policy that CP08 makes a prerequisite.\n- After a text-only revision of those items and a fresh non-author vote, the contract appears adoptable.\n- No implementation, test execution, builder reliance or runtime acceptance is granted. Every authority flag stays false.\n- Still OPEN: all 20 UTxO families, full financial semantics, ACTUS, DeFi, universal certificates, authenticated ledger claims, recursion, private and bounded multiparent PCD, and Preview then Preprod.\n- The public toolchain stays permissionless. This is an internal review.",
  "findings": [
    {
      "id": "N01-T16-normative-contradiction",
      "severity": "medium",
      "summary": "T16 'every ADMIT cell above: v1 returns /1 with 8 labels while r2 refuses at the listed stage' literally includes T02, whose r2 and v1 cells are both ADMIT, so a literal T16 requires r2 to refuse K6.",
      "evidence": "S01 \u00a76: the T02 row has r2 ADMIT and v1 ADMIT; the T16 row says 'every ADMIT cell above \u2026 r2 refuses at the listed stage'. The other v1-ADMIT cells sit in parentheticals inside T04, T06 and T14, plus the whole T05b v1 column. This contradicts CP06, T02 and the \u00a76 STOP rule, under which a K6 refusal at GREEN is STOP.",
      "requiredFix": "Replace the T16 phrase with the exact enumerated subset. T04: driver one-character change (r2 SETUP); provingKeyBytes 2**64 (r2 SETUP). T05b: every case (r2 PROFILE). T06: U+D800 in relation relationFormat, logicalCostProfile or fieldModulusLeHex; U+D800 in a relation extra key; U+D800 consistently in relation action, export action and catalogue name (r2 PROFILE). T14: driver one-character change plus language (r2 SETUP). Exclude the both-admit rows T02 and T17."
    },
    {
      "id": "N02-CP06-T16-sampling-attribution",
      "severity": "medium",
      "summary": "CP06, \u00a75 and the API say T16 samples the 'r2-admitted \u21d2 v1-admitted with equal bytes' property. T16 inputs are r2-refused, so they give zero samples of that implication; T16 shows only strictness. This regresses prior finding F05.",
      "evidence": "S03/S01 CP06: 'a property that tests T02, T16 and T17 only sample'. S01 \u00a75: 'Runtime evidence covers samples only (T02, T16, T17)'. S02 compatibility.admissionVsV1: 'sampled by T02, T16, T17'. S05 F05: 'T16 tests strictness \u2026 It does not test r2 being a subset of v1, which T02 and T17 can only sample.'",
      "requiredFix": "In CP06, \u00a75 and the API, attribute the sampling of the subset and byte-equality property to T02 and T17 only, and describe T16 as sampling strictness (v1-admitted inputs that r2 refuses). Take a fresh vote on the reworded CP06."
    },
    {
      "id": "N03-T05-grouping-vs-18-count",
      "severity": "medium",
      "summary": "\u00a77 predicts exactly 18 tests at RED and GREEN, but \u00a76 lists 20 rows (T05a/b/c). The contract never states that T05a-c form one method, and the NOT RED rules do not cover a different run count. The 'must fail T05b' meaningfulness rule is also unobservable at method level when T05 is grouped.",
      "evidence": "S01 \u00a77: 'RED \u2026 18 tests run'; 'GREEN: 18 passes'; 'v1 bytes under the r2 name must fail T02, T04 and T05b'. S01 \u00a76 table rows T05a, T05b, T05c. A v1 mutant also fails the T05a cases (it refuses at SETUP where PROFILE is expected), so a grouped method fails regardless of T05b.",
      "requiredFix": "State that T05a, T05b and T05c are subTests labelled T05a/T05b/T05c inside one method, giving 18 methods T00-T17. Identify meaningfulness failures by subTest label. Add a run count other than 18 to the NOT RED/STOP conditions."
    },
    {
      "id": "N04-T07-top-level-list-misstaged",
      "severity": "medium",
      "summary": "T07 expects EXPORT for 'any of these \u2026 given as a list', which includes the top-level export. A list export is refused at C2I-ARG.",
      "evidence": "S01 \u00a76 T07: 'extra key in the export, context, publicInputs, cardano, envV2 \u2026; any of these missing a field or given as a list' with r2 EXPORT and v1 '='. S11 prepare_c2_issue refuses 'type(proof_export) is not dict' with C2I-ARG (CI02). S01 T03 itself expects ARG for 'record arguments as list'. Under the \u00a76 rule 'never \u2026 re-stage expectations', this literal value forces a GREEN STOP.",
      "requiredFix": "Limit 'given as a list' to the six nested objects (context, publicInputs, cardano, envV2, settleOutref, catalogueActionEntry). Keep 'missing a field' and 'extra key' for the top-level export."
    },
    {
      "id": "N05-ambiguous-v1-cells-T04-T14",
      "severity": "low",
      "summary": "'provingKeyBytes 0, -1, True or 2**64 (v1 ADMIT)' can be read as all four cases being v1-admitted. v1 refuses 0, -1 and True at SETUP and admits only 2**64. T14's 'driver' mutation is unspecified, and an int driver is v1 SETUP, not ADMIT.",
      "evidence": "S11 _SETUP_RECORD provingKeyBytes ('int', 1, None) and driver _TEXT. S01 T04 and the T14 SETUP/PROFILE row.",
      "requiredFix": "Attach '(v1 ADMIT)' to 2**64 only, and specify 'driver one-character change' in T14."
    },
    {
      "id": "N06-CP04-DIGEST-wording-and-allocation",
      "severity": "low",
      "summary": "CP04 hands x1/x2 to 'the length-first checks of \u2026 DIGEST', but DIGEST has no length check for them; it refuses by exact-string equality. 'Allocation-free' is also undefined for CPython, where iterating a non-Latin-1 str creates transient one-character objects.",
      "evidence": "S11 DIGEST: `halves != (public_inputs['x1'], public_inputs['x2'])`. S01 CP04 and the API strRule. The outcome is correct: T06 expects DIGEST for a surrogate in x1.",
      "requiredFix": "In a later revision, say 'the STATEMENT and PROOF length-first hex checks and the DIGEST exact-string equality'. Define the scan as 'no input-proportional copy and never str.encode; an str.isascii() fast path is allowed'. Not a blocker on its own."
    },
    {
      "id": "N07-section3-check-level-wording",
      "severity": "low",
      "summary": "\u00a73 says each performed check 'mirrors a named check', yet lists r2-only checks such as networkId 0 under 'Added by r2'. It also states 'Rust also admits inputs that r2 refuses, for example networkId 1' as fact, although this was never executed.",
      "evidence": "S01 \u00a73 'Performed \u2026 Each mirrors a named check' versus 'Added by r2: networkId 0 \u2026'. S18 check_export has no networkId check, but no native run is cited for the claim.",
      "requiredFix": "Write 'Each, except the Added-by-r2 checks, mirrors \u2026'. Rephrase the second sentence as 'Rust parse_setup_record/check_export source contains no networkId check (source reading, not executed)'. This is not an admission-level relation claim."
    },
    {
      "id": "N08-S31-stale-never-run-evidence",
      "severity": "low",
      "summary": "\u00a78 bases non-provenance on 'lib.rs says the crate was never compiled or run', and \u00a73 says 'abi2.rs \u2026 was never compiled'. S31 records a root-adopted current-Rust K6 round trip whose export and setup pins equal the fixtures.",
      "evidence": "S31: proofExportSha256 930198\u2026 and setupRecordSha256 86b588\u2026 (the K6/K2 pins), scope 'One currentRust moriarty-core/1 invoice partial Groth16 roundtrip under single-party-test-setup/1 \u2026 fabricated context', binarySourceCorrespondence 'custody evidence, not mathematical binary proof'.",
      "requiredFix": "Cite S31 as the one scoped historical result (class-0, forgeable, fabricated context). Keep driverId DECLARED_NAME_NOT_PROVENANCE, circuitVkCorrespondence OPEN and statementReencode NOT_CHECKED, because r2 cannot verify provenance. Infer no universal compiler, VK or gadget correspondence."
    },
    {
      "id": "N09-token-and-T17-expectation-independence",
      "severity": "low",
      "summary": "At the five non-historical T02 expiries and in T17, envelopeTokenNameHex is checked only against v1 through the same host, so it has no independent expectation. T17 also does not name statementDigestBlake2b256 among the fields it updates.",
      "evidence": "S01 T02 gives a literal token only at 1893456000000. T17 asserts that 'the four byte fields are equal' and that envCborHex has literal framing. S23 test_07 checked only length and inequality.",
      "requiredFix": "Optionally compute the expected token in the test with hashlib.blake2b(digest_size=32) over the literal env bytes, labelled non-oracle. Name statementDigestBlake2b256 and envV2.statementDigest explicitly in T17."
    },
    {
      "id": "N10-manifest-and-process-notes",
      "severity": "info",
      "summary": "DESIGN-POSITIONS.json is not in the S04 files list. The \u00a71 K2 'declared only' count is stale now that root has observed 3926 B. The combined size of 29,580 characters missed the requested 24,000 (process estimate only). R4 (14 labels, DECLARED_SUBSET_NOT_EQUIVALENT) remains incompatible.",
      "evidence": "S04 files lists only the contract and API; S04 setupPublicFixtureRootObserved 3926/86b588\u2026; S04 combinedCharacters 29580 and characterCapSatisfied false; S01 \u00a75 and the S04 dependencyStatus.",
      "requiredFix": "Pin DESIGN-POSITIONS in the adoption record and refresh the \u00a71 K2 row. A new explicit reviewed reconciliation is required before R4 consumes /2; there is no automatic mapping or builder reliance."
    },
    {
      "id": "N11-T06-nondiscriminating-case",
      "severity": "info",
      "summary": "The T06 case 'U+D800 in a relation extra key' is refused by the closed-key check whether or not the CP04 scan exists, so it cannot discriminate the scan mutant.",
      "evidence": "S01 stage note: 'closed-key checks refuse any surrogate key'. The meaningfulness rule depends on relationFormat, logicalCostProfile or export action instead.",
      "requiredFix": "Keep the case, marked redundant. The scan mutant must fail through the relationFormat, logicalCostProfile, fieldModulusLeHex, export action or intentAnchor cases."
    }
  ],
  "crosschecks": [
    {
      "id": "X-F01",
      "severity": "pass",
      "summary": "No admission-level subset, equivalence or conditional-inclusion claim survives between r2 and Rust.",
      "evidence": "S01 CP05, \u00a73 'withdrawn, not relabelled', \u00a74 rustAcceptance NOT_ESTABLISHED; S02 rust.acceptance. What remains is check-level only (N07).",
      "requiredFix": "None beyond N07."
    },
    {
      "id": "X-F02",
      "severity": "pass",
      "summary": "jsonParseLimits and vkProfile labels added; 16 labels everywhere.",
      "evidence": "S01 \u00a74 and S02 labels match key for key and value for value; CP06, CP08 and T02 say 16; S04 labelCount 16.",
      "requiredFix": "None."
    },
    {
      "id": "X-F03",
      "severity": "pass",
      "summary": "The missing-r2 discriminator now uses the pinned O_NOFOLLOW _read. Only FileNotFoundError produces the phrase, and the run envelope and pins are restated.",
      "evidence": "S01 \u00a77 loading, mirroring the S23 _read/_module pattern; no find_spec or import machinery.",
      "requiredFix": "None."
    },
    {
      "id": "X-F04",
      "severity": "pass",
      "summary": "T00 makes no r2 assertion, so it is phase-independent; root observes r2 absence outside the test.",
      "evidence": "S01 \u00a76 T00 and the \u00a77 root responsibilities.",
      "requiredFix": "None."
    },
    {
      "id": "X-F05",
      "severity": "open",
      "summary": "Regressed: T16 contradicts T02, and CP06/\u00a75/API attribute subset sampling to T16.",
      "evidence": "See N01 and N02.",
      "requiredFix": "N01 and N02."
    },
    {
      "id": "X-F06",
      "severity": "pass",
      "summary": "CP04 maps setup members to SETUP, relation members to PROFILE and export members to EXPORT; length comes before the scan; no encode; four fields are deferred.",
      "evidence": "S01 CP04, stage notes and S02 stageOwnership/scannedStr. The deferrals resolve to the STATEMENT/PROOF hex checks and DIGEST equality.",
      "requiredFix": "N06 wording only."
    },
    {
      "id": "X-F07",
      "severity": "pass",
      "summary": "logicalWork is ASCII-only, 1..39 characters and checked before int(), matching parse_u128_decimal; the T05b cases were added.",
      "evidence": "S01 CP03, S02 logicalWork, S20 parse_u128_decimal, S01 T05b.",
      "requiredFix": "None."
    },
    {
      "id": "X-F08",
      "severity": "partial",
      "summary": "A v1 column with literal expectations, the historical T02 literals and hashlib-based T17 digests were added. R03 for v1 stays OPEN, which is correct.",
      "evidence": "S01 \u00a76 notes and the \u00a76 closing paragraph.",
      "requiredFix": "N09 (optional token independence)."
    },
    {
      "id": "X-F09",
      "severity": "pass",
      "summary": "BUILD is excluded from T14/T15, aliasing is scoped, tamper sides are named and the STOP rule is present.",
      "evidence": "S01 T14, T15, the 'Side' column and the STOP paragraph.",
      "requiredFix": "N04 is a new, separate misstaging."
    },
    {
      "id": "X-F10",
      "severity": "pass",
      "summary": "\u00a73 refreshed: shape bound mirrored, check_statement_against fully mirrored, x1/x2 same-outcome note, re-encode NOT_CHECKED.",
      "evidence": "S01 \u00a73 against S18, S20, S26 and S29.",
      "requiredFix": "N08 for the stale 'never compiled' wording."
    },
    {
      "id": "X-F11",
      "severity": "pass",
      "summary": "CP01 names all seven host APIs, called by attribute only.",
      "evidence": "S01 CP01; S22 defines all seven names.",
      "requiredFix": "None."
    },
    {
      "id": "X-F12",
      "severity": "partial",
      "summary": "The root manifest pins the contract and API, and the API lists every flag false. DESIGN-POSITIONS is not in the manifest files list.",
      "evidence": "S04 files; S02 flags.",
      "requiredFix": "N10."
    },
    {
      "id": "X-F13",
      "severity": "pass",
      "summary": "CP08 states non-routing to CB r1, imposes no obligation on CB r1, and requires a separate root decision with nothing automatic.",
      "evidence": "S01 CP08; S24 CB09 would refuse /2 by format.",
      "requiredFix": "None."
    },
    {
      "id": "X-stage-order",
      "severity": "pass",
      "summary": "The stage order is identical in CP05, CP07, the API stages and v1 CI11, with PROFILE inserted.",
      "evidence": "S01 CP05/CP07, S02 stages, S10 CI11.",
      "requiredFix": "None."
    },
    {
      "id": "X-schema-29-kinds",
      "severity": "pass",
      "summary": "The 29 RelationDescriptor keys and the API kinds (5 fixed, 8 int, 9 hex64, 1 hex-or-null, 4 str, logicalWork, shape bytes) match relation.rs and check_profile; the K2 relation has the same 29 keys.",
      "evidence": "S17 struct and check_profile; S02 relation; S15 relation.",
      "requiredFix": "None."
    },
    {
      "id": "X-int-widths",
      "severity": "pass",
      "summary": "Integer bounds match Rust: provingKeyBytes 1..2**64-1 (u64, !=0); relationVersion, constraints and witnessVariables u64; actionIndex u16; headerLen 343; instanceVariables 3; publicInputCount 2. bodyLen 0..261801 narrows Rust's u32, and export u8/u16/u64 are inherited.",
      "evidence": "S17, S18, S11.",
      "requiredFix": "None."
    },
    {
      "id": "X-v1-retained-subset",
      "severity": "pass",
      "summary": "By source construction every v1 check is retained at equal or stricter strength, so r2-admitted inputs are v1-admitted with four equal byte fields. Runtime evidence is samples only (T02, T17).",
      "evidence": "S11 _SETUP_RECORD, _RELATION and _PROOF_EXPORT against S01 CP02-CP06.",
      "requiredFix": "N02 wording."
    },
    {
      "id": "X-rust-omissions",
      "severity": "pass",
      "summary": "The \u00a73 Not-performed list is complete against encoding.rs, export.rs, input.rs and json.rs.",
      "evidence": "S27 VK_IC_OFFSET 336, decode_g1/g2, vk_profile_ok and decode_proof; S18 same('setupRecordSha256'); S28 parse_json; S30 depth 32, 65,536 nodes, duplicate keys and trailing data; S26 re-encode.",
      "requiredFix": "None."
    },
    {
      "id": "X-host-apis",
      "severity": "pass",
      "summary": "The seven host APIs exist with the bounds v1 relies on; BUILD and the DIGEST rewraps stay unreachable for admitted input.",
      "evidence": "S22; S13 R04; S02 unreachableForAdmittedInput.",
      "requiredFix": "None."
    },
    {
      "id": "X-aliasing",
      "severity": "pass",
      "summary": "CP07 and T15 scope aliasing to input dicts and lists, consistent with CI10 sharing immutable leaves.",
      "evidence": "S01 CP07 and T15; S11 return block.",
      "requiredFix": "None."
    },
    {
      "id": "X-T02-literals",
      "severity": "pass",
      "summary": "T02's literal expectations are correct: expiry heads 00, 17, 1818, 1b0000000100000000, 1b000001b8dac5b400 (=1893456000000) and 1bffffffffffffffff; issue framing 710 B; mint 67 B; historical token literal.",
      "evidence": "S23 HIST_ENV_CBOR, ENV_PREFIX and _issue_cbor; hand arithmetic.",
      "requiredFix": "N09 optional."
    },
    {
      "id": "X-T17-offsets",
      "severity": "pass",
      "summary": "The T17 synthetic offsets 144, 172, 173, 233, 265 and 267 equal the claim.rs offsets and host _STATEMENT_FIELDS.",
      "evidence": "S26 offsets; S22 _STATEMENT_FIELDS.",
      "requiredFix": "N09 (name statementDigestBlake2b256)."
    },
    {
      "id": "X-matrix-stages",
      "severity": "pass",
      "summary": "All other T03-T15 r2 and v1 cells were checked against S11 and the clauses and are correct (T05a SETUP, T05c RELATION, T06 listed stages, T08 setup-side RELATION, T09 bodyLen 377 STATEMENT, all nine T14 pairs).",
      "evidence": "S11 stage code against S01 \u00a76.",
      "requiredFix": "N04 and N05 only."
    },
    {
      "id": "X-red-green-loader-pins",
      "severity": "partial",
      "summary": "The loader, pins, envelope and meaningfulness mutants are coherent. The 18-test count depends on the unstated T05 grouping.",
      "evidence": "S01 \u00a77; S04 pins match the S05-S30 headers.",
      "requiredFix": "N03."
    },
    {
      "id": "X-S31-scope",
      "severity": "info",
      "summary": "S31 is one scoped, root-adopted current-Rust K6 round trip whose pins equal the fixtures. It is preserved without any universal correspondence inference.",
      "evidence": "S31 scope, acceptedObservations and limitations.",
      "requiredFix": "N08."
    },
    {
      "id": "X-R4",
      "severity": "info",
      "summary": "The 14-label R4 contract is incompatible with /2 and 16 labels; an explicit reviewed reconciliation is required and there is no automatic builder reliance.",
      "evidence": "S01 \u00a75; S04 dependencyStatus; CP08.",
      "requiredFix": "None in this contract."
    },
    {
      "id": "X-process-size",
      "severity": "info",
      "summary": "Combined 29,580 characters against the requested 24,000: a process estimate miss. All bytes were reviewed.",
      "evidence": "S04 combinedCharacters and characterCapSatisfied false.",
      "requiredFix": "None."
    }
  ],
  "recommendation": {
    "verdict": "REVISE (narrow text correction), not source-ready. AGREE on CP01-CP05, CP07 and CP08. DISAGREE on CP06 only for its T16 sampling attribution (a regression of F05). Medium test-policy defects N01-N04 block literal test authoring.",
    "sourceReady": false,
    "scope": "Exact source-contract adoption review of the corrected C2 r2 profile contract and API and CP01-CP08 (S01-S04) against S05-S31. No implementation or test exists and nothing was run. This grants no implementation, test execution, builder reliance, runtime acceptance or authority.",
    "firstBlockedBoundary": "Contract text revision before any r2 test is authored: enumerate the exact T16 subset (N01); fix the CP06, \u00a75 and API T16 sampling attribution and re-vote CP06 (N02); state the T05a/b/c subTest grouping and make a run count other than 18 a STOP (N03); restrict T07's 'given as a list' to nested objects (N04).",
    "operatorSourceReady": false,
    "ordinarySelftestReady": false,
    "ordinarySelftestScope": "None now. After revision and a fresh non-author vote: root authors and reviews test_c2_issue_artifacts_r2.py, observes r2 absence, then one ordinary RED (18 methods, T00 pass, 17 phrase failures, 0 errors) in the read-only pinned envelope; GREEN only after a reviewed implementation; then the separately enveloped meaningfulness mutants. No native, curve, prove, verify, VM, wallet, signing or network action.",
    "authorityProposalReady": false,
    "nativeRoundtripPrerequisiteClosed": false,
    "publicRecordExportPrerequisiteClosed": false,
    "publicVerificationInputsPublishable": false
  },
  "coverage": {
    "readPaths": [
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/C2-PROFILE-R2-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/C2-PROFILE-R2-API.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/DESIGN-POSITIONS.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/SOURCE-MANIFEST.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/MODEL-REVIEW.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/ROOT-QUALIFICATION.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-API.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/DESIGN-POSITIONS.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/C2-ISSUE-ARTIFACT-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/REPORT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/FINDINGS.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K2-PUBLIC-SETUP-RECORD.json",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K6-PUBLIC-PROOF.json",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/relation.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/export.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/lib.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/hexutil.rs",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-r2-opus-contract-20261016/PROCESS.json",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-preprod-host/host_codec.py",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/test_c2_issue_artifacts.py",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-CONTRACT.md",
      "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-API.json",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/claim.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/encoding.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/input.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/src/abi2.rs",
      "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/moriarty/crates/mori-core/src/json.rs",
      "/home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/root/K6-ACTUAL-ADOPTION.json"
    ],
    "unreadPaths": [
      "k2/src/error.rs, setup.rs, prove.rs, verify.rs, fixed.rs, inspect.rs",
      "mori_r1cs blake2b, identity, statement, plan and adapter modules; mori_core intent::hex",
      "the running downstream R4 contract (14-label attempt-2 target)",
      "the S31-referenced PUBLIC-RESULT-MANIFEST.json and Muse actual REPORT.md",
      "root RED/GREEN RESULT.json of the v1 consumer",
      "CB r1 test_c2_builder.py and legacy build_k10_inputs.py",
      "ABI-V2.md and DESIGN.md"
    ],
    "limitations": "Text-only reading of 31 inline bodies. Nothing was executed, imported, compiled or hashed, and all byte counts and SHA-256 values are as declared. I did not recompute BLAKE2b-256 of the K2 shape bytes or the VK. Equality of Rust blake2b256 with hashlib.blake2b(digest_size=32) is supported only by v1's root-observed GREEN on K6 and by S31's scoped record, not by me. CPython allocation, int() and isdigit behaviour are stated from language knowledge. S31 is preserved as scoped history, and I did not re-review its native inputs. Same-provider fresh context is not provider diversity. nativeRoundtripPrerequisiteClosed=false reflects this review's zero grant, not a negation of S31."
  },
  "positions": [
    {
      "clauseId": "CP01",
      "clause": "The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec, whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, which applies the CI02 ARG checks unchanged, and which calls build_env_data, build_c2_mint_data, build_value_data, encode_data, parse_statement_header, statement_digest_hex and statement_public_inputs only as attributes of the unchanged pinned host_codec (23252 B, SHA-256 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459), never copying or re-implementing them; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format /1, eight labels and test history stay immutable.",
      "position": "AGREE",
      "rationale": "The seven API names exist in S22, and the pins match S04, S14 and S11. CI02 is reused unchanged, and v1 history stays immutable. This closes prior F11, and T01's counting proxy now tests an obligation the clause actually states."
    },
    {
      "clauseId": "CP02",
      "clause": "When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is an exact str of length 976, length-checked before its lowercase-hex scan; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256; it shall decode no VK point, so the decode_vk IC-prefix, flag, field-range, curve, subgroup, identity and canonical-encoding checks are not performed.",
      "position": "AGREE",
      "rationale": "Field by field it matches S18 parse_setup_record (12 deny_unknown_fields keys, fixed literals, DRIVER_ID from S19, decode_exact::<488>, u64 proving_key_bytes != 0, the VK digest). The decode_vk omission is stated explicitly and matches the S27 checks. The length-first rule is explicit. The driver remains a declared name (driverId label)."
    },
    {
      "clauseId": "CP03",
      "clause": "When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests are 64 lowercase hex and sourceSha256 is present as 64 lowercase hex or null; logicalWork is an exact str of 1..39 characters, each in ASCII 0-9, with no leading zero unless it is 0, all checked before any int conversion, and value at most 2**128-1; statementTagHex and familyIdHex equal the ABI v2 literals; headerLen is 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex is an exact str of even length at most 524288, length-checked before any scan, then lowercase hex whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.",
      "position": "AGREE",
      "rationale": "Verified against S17 check_profile and S20. There are 29 keys and the nine hex digests match. parse_u128_decimal is mirrored ASCII-only and length-first before int(). The shape bound equals decode_bounded(MAX_STATEMENT_BYTES). The tag and family literals match abi2.rs. The moriarty/1 and moriarty-core/1 requirement is preserved as a descriptor check, not a recompilation. The API kinds account for all 29 keys."
    },
    {
      "clauseId": "CP04",
      "clause": "While admitting the setup record, its relation and the export, the consumer shall accept a key only as an exact str naming a field of the closed schema, an integer only as an exact int (never bool) within its kind range, and a str-kind value only as an exact str containing no code point in U+D800..U+DFFF, found by an allocation-free code-point comparison scan (never str.encode) that runs after any length check of that field; otherwise it shall refuse with C2I-SETUP for a setup top-level member, C2I-PROFILE for any relation key or member, and C2I-EXPORT for any export key or member at any depth, except that statementHex, proofHex, x1 and x2 are not scanned at EXPORT and are left to the length-first checks of STATEMENT, PROOF and DIGEST.",
      "position": "AGREE",
      "rationale": "The stage ownership is now explicit and consistent with CP02, the API stageOwnership and T04-T07. The scanned fields are exactly the non-hex, non-fixed str kinds, and every other kind excludes surrogates through its own character or equality checks. The deferred fields are refused by length-first hex checks at STATEMENT and PROOF and by exact-string equality at DIGEST, so the outcomes match T06. There is no UnicodeEncodeError path. Low wording notes (N06): DIGEST is an equality check, not a length-first check, and 'allocation-free' should be defined as no input-proportional copy."
    },
    {
      "clauseId": "CP05",
      "clause": "When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe its admission only as the selected checks of contract section 3, stating that it performs no decode_vk or decode_proof point check (IC prefix, flags, field range, curve, subgroup, identity, canonical encoding), no setupRecordSha256 file-byte check, no parse_json byte, UTF-8, duplicate-key, trailing-data, depth or node limit, no parse_statement re-encode and no Groth16 pairing, that Rust acceptance of an r2-admitted input is not established, and never asserting any subset, equivalence or conditional-inclusion relation between r2 admission and Rust acceptance.",
      "position": "AGREE",
      "rationale": "This resolves prior F01 and F02. The omission list is complete against S27, S18, S28, S30 and S26, and the order matches CI05-CI09. No admission-level subset, equivalence or conditional-inclusion statement survives anywhere in S01 or S02. The \u00a73 'each mirrors' and 'Rust also admits' sentences are check-level wording notes (N07), not admission-relation claims."
    },
    {
      "clauseId": "CP06",
      "clause": "When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 16-entry r2 label map of C2-PROFILE-R2-API.json, and shall retain every v1 check at equal or stricter strength, so that by source construction every r2-admitted input is v1-admitted with envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex equal to the v1 values, a property that tests T02, T16 and T17 only sample.",
      "position": "DISAGREE",
      "rationale": "The behaviour is agreed. I verified the 12 keys, /2 and the 16 labels, and that r2-admitted \u21d2 v1-admitted with four equal byte fields holds by source construction against S11. But the clause's evidence statement is false: T16 inputs are r2-refused, so T16 gives zero samples of that implication and shows only strictness. This repeats prior F05 and also appears in \u00a75 and the API. Reword to 'T02 and T17 only sample' (N02) and re-vote."
    },
    {
      "clauseId": "CP07",
      "clause": "If any check fails, then the consumer shall raise exactly ValueError, never a subclass such as UnicodeEncodeError, whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD; it shall rewrap a host ValueError with the current stage code, let non-ValueError environmental errors such as MemoryError propagate unconverted, return no partial result, never mutate either input, keep no input dict or list reachable from module state or from the result (immutable str and int leaves may be shared), and perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.",
      "position": "AGREE",
      "rationale": "It is consistent with the CI11 host-rewrap and environmental-propagation rules and with the API stage list. Aliasing is scoped to mutable containers, compatible with CI10 sharing the setupRecordSha256 str and the expiry int. Length-first logicalWork prevents an unprefixed int() ValueError, and the ban on encode prevents a subclass escaping. BUILD is correctly excluded from tests as unreachable."
    },
    {
      "clauseId": "CP08",
      "clause": "The r2 records shall keep builderReliancePermitted false and every native, operator, wallet, prover, VM, deploy, signing and network flag false; builderReliancePermitted may change only by a separate root source-adoption decision taken after a reviewed r2 implementation, a fresh non-author audit, root-observed meaningful RED then GREEN of the reviewed T00-T17 test and a reviewed downstream contract that explicitly consumes format moriarty-c2-issue-artifacts/2 with the 16 labels, and none of these prerequisites grants that permission automatically; r2 output is not CB-r1-compatible and shall not be routed to CB r1, and this contract imposes no obligation on CB r1.",
      "position": "AGREE",
      "rationale": "It closes prior F13: CB r1 is handled by non-routing, a separate root decision is required, nothing is automatic, and every flag is false (S02 flags). The 'reviewed downstream contract consuming /2 with 16 labels' gate means the 14-label R4 cannot qualify without an explicit reviewed reconciliation. S24 CB09 would refuse /2 by source."
    }
  ]
}


## S33 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-candidate-20261017/C2-PROFILE-R2-CONTRACT.md
# C2 consumer profile contract r2, revision 4 (CP01-CP08)

Status: AUTHOR-PROPOSED finding-driven text repair (claude-opus-5-5, tools off). This is revision 4 of attempt 3 of the profile family.
- Attempt 1 (contract plus tests) timed out and is preserved.
- Attempt 2 (VAL/current-cardano-c2-consumer-profile-policy-candidate-20261016/, immutable) was REJECTED by VAL/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/MODEL-REVIEW.json (report SHA-256 eaa31ec5d9404c0a7cfc87c12a84f3a19459d7e0883d6a2ad7cc400f17eee8df): CP04 and CP05 DISAGREE, findings F01-F13.
- Revision 3 (VAL/current-cardano-c2-profile-policy-corrected-candidate-20261016/, immutable) was audited by VAL/current-cardano-c2-profile-corrected-policy-fresh-opus-audit-20261016/MODEL-REVIEW.json: CP01-CP05, CP07 and CP08 AGREE; CP06 DISAGREE; N01-N04 (medium) block test authoring; N05-N09 low; N10-N11 info.
- This revision applies exactly the N01-N11 requiredFix texts, in a new root (checklist in section 9). It is not a retry, a provider fallback or a broad rewrite. Semantics, labels, stage expectations, current Rust identities and all general obligations are unchanged.

No r2 test or implementation exists. Nothing was executed, imported, hashed or compiled. Author FOR is not adoption: fresh non-author exact clause votes are required before any test is authored. The CP04 (N06) and CP06 (N02) texts changed. Every authority, runtime, reliance and readiness flag is false.

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
| PRE/K2-PUBLIC-SETUP-RECORD.json | 3926 root-observed (setupPublicFixtureRootObserved in VAL/current-cardano-c2-profile-policy-corrected-candidate-20261016/SOURCE-MANIFEST.json), 86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b |
| Revision 3 C2-PROFILE-R2-CONTRACT.md and C2-PROFILE-R2-API.json (VAL/current-cardano-c2-profile-policy-corrected-candidate-20261016/) | 23830, 03f03887df741ce21556158a0490e24890ba33f310ce276f5060d1013f9e5e03; 5750, 965b880d5b440376faf495b757ba9b06460e9e5755e943b23c81c789982af0fb (as declared by that SOURCE-MANIFEST) |
| Revision 3 DESIGN-POSITIONS.json and SOURCE-MANIFEST.json; the revision 3 audit VAL/current-cardano-c2-profile-corrected-policy-fresh-opus-audit-20261016/MODEL-REVIEW.json; the scoped K6 record /home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/root/K6-ACTUAL-ADOPTION.json; RUST/k2/src/{relation,export,lib,hexutil,claim,encoding,input}.rs; RUST/r1cs/source/mori-r1cs/src/abi2.rs; RUST/moriarty/crates/mori-core/src/json.rs; v1 contract and test in VAL/current-cardano-proofexport-c2-consumer-source-candidate-20261016/; VAL/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json | from the root source manifest of this step; not supplied inline and not computed by the author |

The root manifest pins this file, C2-PROFILE-R2-API.json and the author positions of this step. None of them pins another.

## 2. Clauses (EARS; author FOR on all)

- CP01: The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec, whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, which applies the CI02 ARG checks unchanged, and which calls build_env_data, build_c2_mint_data, build_value_data, encode_data, parse_statement_header, statement_digest_hex and statement_public_inputs only as attributes of the unchanged pinned host_codec (23252 B, SHA-256 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459), never copying or re-implementing them; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format /1, eight labels and test history stay immutable.
- CP02: When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is an exact str of length 976, length-checked before its lowercase-hex scan; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256; it shall decode no VK point, so the decode_vk IC-prefix, flag, field-range, curve, subgroup, identity and canonical-encoding checks are not performed.
- CP03: When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests are 64 lowercase hex and sourceSha256 is present as 64 lowercase hex or null; logicalWork is an exact str of 1..39 characters, each in ASCII 0-9, with no leading zero unless it is 0, all checked before any int conversion, and value at most 2**128-1; statementTagHex and familyIdHex equal the ABI v2 literals; headerLen is 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex is an exact str of even length at most 524288, length-checked before any scan, then lowercase hex whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.
- CP04: While admitting the setup record, its relation and the export, the consumer shall accept a key only as an exact str naming a field of the closed schema, an integer only as an exact int (never bool) within its kind range, and a str-kind value only as an exact str containing no code point in U+D800..U+DFFF, found by a code-point comparison scan that makes no input-proportional copy and never calls str.encode (a str.isascii() fast path is allowed) and that runs after any length check of that field; otherwise it shall refuse with C2I-SETUP for a setup top-level member, C2I-PROFILE for any relation key or member, and C2I-EXPORT for any export key or member at any depth, except that statementHex, proofHex, x1 and x2 are not scanned at EXPORT and are left to the length-first lowercase-hex checks of STATEMENT and PROOF and the exact-string equality of DIGEST.
- CP05: When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe its admission only as the selected checks of contract section 3, stating that it performs no decode_vk or decode_proof point check (IC prefix, flags, field range, curve, subgroup, identity, canonical encoding), no setupRecordSha256 file-byte check, no parse_json byte, UTF-8, duplicate-key, trailing-data, depth or node limit, no parse_statement re-encode and no Groth16 pairing, that Rust acceptance of an r2-admitted input is not established, and never asserting any subset, equivalence or conditional-inclusion relation between r2 admission and Rust acceptance.
- CP06: When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 16-entry r2 label map of C2-PROFILE-R2-API.json, and shall retain every v1 check at equal or stricter strength, so that by source construction every r2-admitted input is v1-admitted with envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex equal to the v1 values, a property that tests T02 and T17 only sample; T16 samples only strictness, that is v1-admitted inputs that r2 refuses, and is not evidence of that property.
- CP07: If any check fails, then the consumer shall raise exactly ValueError, never a subclass such as UnicodeEncodeError, whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD; it shall rewrap a host ValueError with the current stage code, let non-ValueError environmental errors such as MemoryError propagate unconverted, return no partial result, never mutate either input, keep no input dict or list reachable from module state or from the result (immutable str and int leaves may be shared), and perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.
- CP08: The r2 records shall keep builderReliancePermitted false and every native, operator, wallet, prover, VM, deploy, signing and network flag false; builderReliancePermitted may change only by a separate root source-adoption decision taken after a reviewed r2 implementation, a fresh non-author audit, root-observed meaningful RED then GREEN of the reviewed T00-T17 test and a reviewed downstream contract that explicitly consumes format moriarty-c2-issue-artifacts/2 with the 16 labels, and none of these prerequisites grants that permission automatically; r2 output is not CB-r1-compatible and shall not be routed to CB r1, and this contract imposes no obligation on CB r1.

Stage notes:
- No check of stage k runs before stage k-1 completes. Order within a stage is free, except that length checks come before scans.
- The CP04 str scan covers relation relationFormat, logicalCostProfile, action and fieldModulusLeHex (at PROFILE), and export action, catalogueActionEntry.name, intentTargetHashV2 and intentAnchor (at EXPORT).
- Deferred export fields: statementHex and proofHex are refused by the length-first lowercase-hex checks of STATEMENT and PROOF. x1 and x2 have no length check; DIGEST refuses them by exact-string equality with the host decimals.
- Setup top-level members have no str kind. Hex, fixed and decimal kinds exclude surrogates through their own character checks, and closed-key checks refuse any surrogate key.
- Every relation fault that v1 reported as C2I-SETUP is C2I-PROFILE in r2.
- The side of the tamper decides the stage:
  - a setup-side abi2ShapeDigest change refuses at PROFILE;
  - a setup-side change to programHash, sourceSha256, action, actionIndex, compilerBuildDigest, r1csSha256 or predicateDigest refuses at RELATION;
  - a setup-side bodyLen change within range refuses at STATEMENT.

## 3. Rust correspondence: selected checks only

Performed: the checks of CP02, CP03 and CI05-CI09 as narrowed by CP04. Each one, except the Added-by-r2 checks below, mirrors a named check in:
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
- The audit's counterexamples are source-derived and were never run natively, so no runtime counterexample is claimed. They are: invalid-point VK or proof bytes under a consistent BLAKE2b digest, a mismatched setupRecordSha256, and documents outside the parse_json limits.
- The Rust parse_setup_record and check_export source contains no networkId check. This is a source reading and was not executed. Nothing is stated about which inputs Rust admits or refuses at runtime.
- r2 is a structural serializer and verifies no Groth16 proof.

Source-reading note (not executed): the host statement offsets equal the claim.rs offsets and the abi2::statement_bytes order. This contract does not review abi2.rs, and its 'never compiled or run' docstring is stale history (section 8). r2 performs no re-encode, so statementReencode stays NOT_CHECKED.

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
- r2 versus v1: by source construction, r2-admitted inputs are v1-admitted, with the four byte fields equal. Runtime evidence covers samples only, from T02 and T17. T16 samples only strictness (v1-admitted inputs that r2 refuses) and gives no sample of this property.
- No bridge converts between /1 and /2.
- CB r1 is not compatible: by source reading, its CB09 format check refuses /2. r2 output shall not be routed to CB r1, and nothing is imposed on CB r1.
- The running downstream R4 contract task targets attempt 2 (14 labels, rustAcceptance DECLARED_SUBSET_NOT_EQUIVALENT). This revision changes that dependency. R4 stays incompatible until an explicit reviewed reconciliation. There is no silent or automatic mapping and no builder reliance.

## 6. Test obligations (future; nothing ran)

One reviewed file, test_c2_issue_artifacts_r2.py, holds exactly 18 test methods, T00-T17. T05 is one method whose cases run as subTests labelled T05a, T05b and T05c. Every other row is one method.
- Each case applies tampers to fresh fixture dicts. S is the setup record, S.rel its relation, and E the export.
- The r2 and v1 expectations are literal values written from this contract. They are never derived by running r2 or v1.
- In the v1 column, = means the same code, ADMIT means v1 returns its /1 result, and a stage name means v1 refuses at that stage. 'unless noted' means the parenthetical v1 values in the Cases cell apply.
- Each of T01-T17 obtains r2 before any other assertion or subTest.
- Token expectations: an expected envelopeTokenNameHex is either the historical literal, or hashlib.blake2b(bytes.fromhex(E), digest_size=32).hexdigest() computed in the test over the literal expected envCborHex E. That value is a test-computed NON-ORACLE expectation. It is independent of host_codec code but not of hashlib, and it is not evidence about Rust. Equality with v1 is consistency through the shared host, not an oracle.

| ID | Side | Cases | r2 | v1 |
|---|---|---|---|---|
| T00 | - | host, v1, K6 and K2 bytes and SHA-256 equal the section 1 pins; fixtures parse with duplicate-key and NaN refusal; no assertion about r2 | pass | - |
| T01 | - | import lines exactly import hashlib and import host_codec; 3 positional params plus keyword-only policy_id, no defaults; prepare_c2_issue is the only public function; no host API name in the r2 namespace; r2 bytes re-executed against a counting proxy in sys.modules (restored afterwards) call all seven APIs on K6 | - | - |
| T02 | - | K6 at expiries 0, 23, 24, 2**32, 1893456000000 and 2**64-1, for HIST_POLICY and 01x28: format /2, 12 keys, exactly 16 labels; envCborHex = v1-test ENV_PREFIX + 00, 17, 1818, 1b0000000100000000 or 1bffffffffffffffff + ff; at 1893456000000 it equals the literal HIST_ENV_CBOR and the token is 9fd5c53f6a1a03d20b0409f2ca3ad28bd02d5401564603fab5f81b289cc6d54e; at the other five expiries the token equals the hashlib expectation over that literal envCborHex; issueCborHex = literal 710-byte 64-byte chunk framing; mintValueCborHex = a1581c+policy+a15820+token+01; four byte fields equal v1 | ADMIT | ADMIT |
| T03 | args | record arguments as list, None, str or _Hooked dict subclass; expiry -1, 2**64, True, None, float or str; policy of 27 or 29 bytes, bytearray, str or None; records untouched | ARG | = |
| T04 | S | one-char change of format, trustProfile, notice, curve, snark or encoding; driver one-char change (v1 ADMIT) or int (v1 SETUP); provingKeySha256 uppercase or 63 hex; provingKeyBytes 0, -1 or True (v1 SETUP) or 2**64 (v1 ADMIT); extra or missing key; relation as a list; VK of 974 hex or uppercase; vkDigest flip | SETUP | SETUP unless noted |
| T05a (subTest of T05) | S.rel | each of the 13 v1-consumed keys missing; programHash, compilerBuildDigest, r1csSha256, predicateDigest, abi2ShapeDigest or sourceSha256 as 63 hex or uppercase; tag or family flip; headerLen 342; bodyLen -1 or 261802; publicInputCount 3; actionIndex 65536 or True; action int | PROFILE | SETUP |
| T05b (subTest of T05) | S.rel | each of the 16 non-consumed keys missing; an extra key; language, semantics or adapter one-char; profileDigest, relationShapeDigest, compilerSourceManifestSha256 or coreClosureSha256 as 63 hex or uppercase; logicalWork '', '01', '+1', '-1', 40 digits, 2**128 as decimal, Arabic-Indic two-eight, '1_0', ' 28', '28' plus LF, 5000 digits, or int 28; instanceVariables 4; relationVersion, constraints or witnessVariables True, -1 or 2**64; relationFormat, logicalCostProfile or fieldModulusLeHex as int; abi2ShapeBytesHex odd, uppercase, 524290 chars, or one byte flipped | PROFILE | ADMIT |
| T05c (subTest of T05) | S.rel | abi2ShapeDigest set to another valid hex64 | PROFILE | RELATION |
| T06 | S.rel, E | U+D800 in relationFormat, logicalCostProfile or fieldModulusLeHex (v1 ADMIT), in a relation extra key (v1 ADMIT; REDUNDANT: the closed-key check refuses it with or without the CP04 scan, so it does not discriminate the scan), in relation action alone (v1 RELATION), or consistently in relation action, export action and catalogue name (v1 ADMIT): PROFILE. U+D800 in export action alone (v1 RELATION) or in catalogue name, intentTargetHashV2 or intentAnchor alone (v1 CARDANO): EXPORT. U+D800 in statementHex, proofHex or x1: STATEMENT, PROOF or DIGEST (v1 =) | listed | listed |
| T07 | E | extra key in the export, context, publicInputs, cardano, envV2 (including expiry), settleOutref or catalogueActionEntry; any of these missing a field; context, publicInputs, cardano, envV2, settleOutref or catalogueActionEntry given as a list (a list-valued export itself is ARG and belongs to T03, not T07); networkId, version or claimKind 256; actionIndex, consumedIx, outputIndex or index 65536; seq 2**64; any int field True; action int; sourceSha256 as 64 uppercase A; premises reordered, as a tuple, with 5 items, or one char changed | EXPORT | = |
| T08 | E, S.rel | E: each of the 8 identity keys alone set to another valid value (sourceSha256 null, action full, actionIndex 2), or vkDigestBlake2b256 alone. S.rel: programHash, compilerBuildDigest, r1csSha256 or predicateDigest set to 00x32; sourceSha256 null; action full; actionIndex 2 | RELATION | = |
| T09 | E, S.rel | statementHex as 00x262145, odd length, uppercase, or containing g; a flip at offset 0, 16, 48, 80 or 112; last byte removed; S.rel bodyLen 377 | STATEMENT | = |
| T10 | E | each of the 10 context fields changed alone; statement byte 144 flipped; networkId 1 in both statement byte 172 and the context | CONTEXT | = |
| T11 | E | statementDigest 00x32; body byte 400 flipped; x1 or x2 alone set to 0; x1 with a leading 0 | DIGEST | = |
| T12 | E | proofHex uppercase, 382 or 386 chars, or containing g | PROOF | = |
| T13 | E | each envV2 field (version 3, claimKind 1, familyId, the two digests, claimantVkh, settleOutref fields, statementDigest); each catalogueActionEntry field (name full, index 2, three digests); intentTargetHashV2; intentAnchor | CARDANO | = |
| T14 | mixed | two faults per input, earlier stage expected: ARG/SETUP (expiry -1 + trustProfile), SETUP/PROFILE (driver one-char change + language; v1 ADMIT), PROFILE/EXPORT (language + E extra key; v1 EXPORT), EXPORT/RELATION (seq True + E programHash), RELATION/STATEMENT (E programHash + last byte removed), STATEMENT/CONTEXT (flip 48 + consumedIx 1), CONTEXT/DIGEST (consumedIx 1 + statementDigest), DIGEST/PROOF (statementDigest + 382-char proof), PROOF/CARDANO (382-char proof + intentAnchor). BUILD excluded as unreachable | earlier | = unless noted |
| T15 | - | deep-copy equality of both inputs after success and after the first-fault refusal of each T14 pair plus a CARDANO fault; no input dict or list in the result by identity, while str and int leaves may be shared; two results share no mutable container; module namespace keys and mutable globals unchanged and holding no input container; effect-token scan as v1 test_11 plus global | - | - |
| T16 | - | strictness only. Exactly these v1-ADMIT cases, each refused by r2 at the stage shown: T04 driver one-char change (SETUP); T04 provingKeyBytes 2**64 (SETUP); every T05b case (PROFILE); T06 U+D800 in relation relationFormat, logicalCostProfile or fieldModulusLeHex (PROFILE); T06 U+D800 in a relation extra key (PROFILE); T06 U+D800 consistently in relation action, export action and catalogue name (PROFILE); T14 driver one-char change + language (SETUP). For each, v1 returns /1 with 8 labels. The both-admit rows T02 and T17 are excluded. T16 gives no sample of the CP06 subset property | refuse at the stage shown | ADMIT |
| T17 | E | SYNTHETIC_NOT_PROOF_NOT_ORACLE public test fixture derived from K6; not a proof, not an oracle and not authenticated ledger data. Statement: claimant 66x28 (offset 144), settleHash b3x28 (173), consumedTxid 33x32 (233), consumedIx 7 (265) and seq 2 (267), networkId still 0. statementDigestBlake2b256 and x1/x2 are computed in the test by hashlib.blake2b(digest_size=32) and int.from_bytes, never by the host. Updated export fields: statementHex, statementDigestBlake2b256, context (claimantVkh, settleHash, consumedTxid, consumedIx, seq), publicInputs x1 and x2, cardano.envV2 claimantVkh, settleOutref.transactionId, settleOutref.outputIndex and statementDigest, and cardano.intentAnchor. proofHex is unchanged and proves nothing. r2 and v1 both admit; the four byte fields are equal; envCborHex equals literal framing segments around the synthetic values; the token equals the hashlib expectation over that envCborHex | ADMIT | ADMIT |

If the pinned K2 or K6 record, or T17, is refused at GREEN, STOP for review. Never edit fixtures, recompute fixture digests or re-stage expectations. T17 and every hashlib expectation stay SYNTHETIC_NOT_PROOF_NOT_ORACLE or NON-ORACLE. The public test fixtures are not authenticated ledger data. R03 for v1 closes only when the v1 column is implemented, root-executed and separately reviewed.

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

RED (an unobserved prediction): 18 test methods run (T00-T17, with T05 holding the T05a, T05b and T05c subTests). T00 passes. T01-T17 fail, each with the phrase. There are 0 errors.

NOT RED, and STOP for review: a pin mismatch, a setUpModule or unittest error, a T00 failure, any failure without the phrase, or a run count other than 18.

GREEN: 18 passes, exit 0, pins unchanged. Any failure, or a run count other than 18, is STOP.

Meaningfulness check:
- It runs in a separate reviewed scratch envelope after GREEN, using root-pinned mutants that are never adopted.
- Failures are identified by method and, inside T05, by subTest label.
- v1 bytes under the r2 name must fail T02, T04 and the T05b subTests. They also fail T05a; that is expected and does not count toward T05b.
- r2 without the CP04 scan must fail T06 through the relationFormat, logicalCostProfile, fieldModulusLeHex, export action or intentAnchor cases. The redundant relation-extra-key case does not count.
- r2 without the shape-digest comparison, or using int() without the ASCII check, must fail T05b subTests.
- Every such failure must be an assertion failure whose text is not the missing phrase.

## 8. Non-claims and open tracks

- v1 remains the adopted, unverified structural serializer under R01-R03. Its history is unchanged, and R03 runtime stays OPEN.
- The full r2 implementation and tests are absent. There is no native, prover, VM, wallet, signing or network action.
- The setup is single-party, TEST ONLY and forgeable. Circuit/VK correspondence is OPEN.
- driver is a declared name only, and r2 cannot verify fixture-to-source provenance. The lib.rs and abi2.rs 'never compiled or run' docstrings are stale comments, not evidence that nothing ran. /home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/root/K6-ACTUAL-ADOPTION.json records one root-adopted current-Rust moriarty-core/1 invoice-partial Groth16 round trip. Its proofExportSha256 and setupRecordSha256 equal the K6 and K2 pins. It ran under class-0 single-party-test-setup/1 with caller-declared fabricated context, and its binary-source correspondence is custody evidence, not a mathematical proof. That one scoped historical result is preserved as history. No universal compiler, VK, gadget or circuit correspondence is inferred from it. driverId DECLARED_NAME_NOT_PROVENANCE, circuitVkCorrespondence OPEN and statementReencode NOT_CHECKED stay. This contract's false flags grant nothing; they do not negate that record.
- Context is caller-declared and unauthenticated. networkId 0 does not separate Preview (first) from Preprod (separate, later); both are OPEN.
- moriarty/1 and moriarty-core/1 are checked as descriptor fields, not recompiled. Compiler Strands and future formats are not covered, and no full Web is claimed.
- OPEN: all 20 UTxO families, general financial residual duties, ACTUS, DeFi, authenticated ledger claims, universal certificates, execution, recursion, private handoff and multiparent.
- The public toolchain stays permissionless. These reviews are not developer permission and not compile, prove or deploy gates.

## 9. Revision 4 repair checklist (S32 requiredFix, applied exactly)

- N01: T16 lists exactly the cases v1 admits and r2 refuses; T02 and T17 are excluded.
- N02: CP06, section 5 and the API attribute subset and byte-equality sampling to T02 and T17 only; T16 is strictness only. CP06 needs a fresh vote.
- N03: 18 methods; T05a/T05b/T05c are subTests of T05; a run count other than 18 is NOT RED or STOP; meaningfulness is judged by subTest label.
- N04: T07 'given as a list' covers only the six nested objects; a list-valued export is ARG (T03).
- N05: in T04, '(v1 ADMIT)' applies only to provingKeyBytes 2**64 and the driver one-char change; T14 uses a driver one-char change.
- N06: CP04 and the API name the STATEMENT/PROOF length-first hex checks and the DIGEST exact-string equality; the scan makes no input-proportional copy, never calls str.encode, and may use a str.isascii() fast path. CP04 needs a fresh vote.
- N07: section 3 wording is check-level: 'Each, except the Added-by-r2 checks'; the networkId statement is a source reading.
- N08: S31 is scoped history; the docstrings are stale; the labels are unchanged.
- N09: hashlib NON-ORACLE token expectations; T17 names statementDigestBlake2b256 and envV2.statementDigest.
- N10: K2 is 3926 B, root-observed; the root manifest pins the author positions; R4 stays incompatible.
- N11: the T06 relation-extra-key case is marked REDUNDANT.


## S34 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-candidate-20261017/C2-PROFILE-R2-API.json
{
"format": "moriarty-c2-profile-r2-api/3",
"status": "AUTHOR-PROPOSED revision 4; exact S32 N01-N11 repair of immutable revision 3 (api/2); format bumped only to distinguish the revision; no implementation or test; nothing run; pinned only by the root manifest",
"module": "c2_issue_artifacts_r2.py",
"imports": ["hashlib", "host_codec"],
"hostApisByAttribute": ["build_env_data", "build_c2_mint_data", "build_value_data", "encode_data", "parse_statement_header", "statement_digest_hex", "statement_public_inputs"],
"function": "prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id)",
"stages": ["ARG", "SETUP", "PROFILE", "EXPORT", "RELATION", "STATEMENT", "CONTEXT", "DIGEST", "PROOF", "CARDANO", "BUILD"],
"unreachableForAdmittedInput": ["BUILD"],
"refusal": "exactly ValueError, never a subclass; prefix C2I-<STAGE>: of the first failing stage; host ValueError rewrapped with the stage code; non-ValueError environmental errors propagate",
"stageOwnership": {"setupTopLevel": "SETUP", "relationKeysAndMembers": "PROFILE", "exportKeysAndMembersAnyDepth": "EXPORT", "notScannedAtExport": {"statementHex": "STATEMENT", "proofHex": "PROOF", "x1": "DIGEST", "x2": "DIGEST"}, "deferredCheck": {"statementHex": "length-first lowercase hex", "proofHex": "length-first lowercase hex", "x1": "exact-string equality with the host decimal; no length check", "x2": "exact-string equality with the host decimal; no length check"}},
"intRule": "exact int, never bool, within kind range",
"strRule": "exact str without U+D800..U+DFFF; code-point comparison scan after any length check, with no input-proportional copy; never str.encode; a str.isascii() fast path is allowed",
"scannedStr": {"relation": ["relationFormat", "logicalCostProfile", "action", "fieldModulusLeHex"], "export": ["action", "cardano.catalogueActionEntry.name", "cardano.intentTargetHashV2", "cardano.intentAnchor"]},
"setup": {"keys": "exactly the 12 SetupRecord keys", "fixed": "format, trustProfile, notice, curve, snark, encoding = export.rs constants", "driver": "fixed:mori-k2-driver/0.1.0-k2s1+k4h1", "provingKeySha256": "hex64", "provingKeyBytes": "int:1..18446744073709551615", "relation": "dict", "verifyingKeyHex": "str, length 976 checked first, then lowercase hex; no point decoding", "vkDigestBlake2b256": "hex64 = BLAKE2b-256(VK bytes)"},
"relation": {
"keys": "exactly these 29",
"fixed": {"language": "moriarty/1", "semantics": "moriarty-core/1", "adapter": "SettlementHashV2", "statementTagHex": "4d524731362d5056432d53544d542d32", "familyIdHex": "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32"},
"int": {"headerLen": "343", "instanceVariables": "3", "publicInputCount": "2", "bodyLen": "0..261801", "actionIndex": "u16", "relationVersion": "u64", "constraints": "u64", "witnessVariables": "u64"},
"hex64": ["profileDigest", "programHash", "relationShapeDigest", "compilerSourceManifestSha256", "coreClosureSha256", "compilerBuildDigest", "predicateDigest", "abi2ShapeDigest", "r1csSha256"],
"hex64OrNullKeyRequired": ["sourceSha256"],
"str": ["relationFormat", "logicalCostProfile", "action", "fieldModulusLeHex"],
"logicalWork": "exact str, 1..39 chars, ASCII 0-9 only, no leading zero unless 0, checked before int(); value <= 2**128-1",
"abi2ShapeBytesHex": "exact str, even length <= 524288 checked first, lowercase hex, BLAKE2b-256 = abi2ShapeDigest"
},
"export": "CI05-CI09 unchanged, narrowed by intRule and strRule",
"result": "the 12 CI10 keys; format moriarty-c2-issue-artifacts/2; labels exactly the 16 below",
"labels": {"expiry": "CALLER_SUPPLIED_NOT_PROOF_BOUND", "context": "CALLER_DECLARED_UNAUTHENTICATED", "policyId": "CALLER_SUPPLIED_UNAUTHENTICATED", "trustProfile": "single-party-test-setup/1", "setupRecordLink": "DECLARED_NOT_BYTE_CHECKED", "groth16": "NOT_VERIFIED", "vkCurveValidity": "NOT_CHECKED", "vkProfile": "NOT_CHECKED", "proofCurveValidity": "NOT_CHECKED", "statementReencode": "NOT_CHECKED", "jsonParseLimits": "NOT_CHECKED", "relationProfile": "DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED", "driverId": "DECLARED_NAME_NOT_PROVENANCE", "rustAcceptance": "NOT_ESTABLISHED", "circuitVkCorrespondence": "OPEN", "ledger": "NO_CLAIM"},
"rust": {"acceptance": "NOT_ESTABLISHED; no subset, equivalence or conditional-inclusion claim", "notChecked": ["decode_vk IC prefix, flags, field, curve, subgroup, identity, canonical", "decode_proof point rules and identity", "setupRecordSha256 file-byte link", "parse_json and parse_limited limits", "parse_statement re-encode", "Groth16 pairing"], "addedByR2": ["networkId 0", "bodyLen <= 261801", "sourceSha256 key required", "exact-int and surrogate-free str rules"], "checkLevel": "every performed check except addedByR2 mirrors a named Rust check; Rust parse_setup_record/check_export source contains no networkId check (source reading, not executed); no runtime admission statement"},
"v1": {"module": "c2_issue_artifacts.py", "bytes": 15231, "sha256": "23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76", "format": "moriarty-c2-issue-artifacts/1", "labels": 8, "status": "immutable; adopted only as unverified structural serializer; R03 runtime OPEN"},
"compatibility": {"admissionVsV1": "subset by source construction; sampled by T02 and T17 only; not proven", "strictnessVsV1": "sampled by T16 only (v1-admitted inputs that r2 refuses); not evidence of admissionVsV1", "byteEqualWhenBothAdmit": ["envCborHex", "envelopeTokenNameHex", "issueCborHex", "mintValueCborHex"], "cbR1": "not compatible; r2 output not routed to CB r1; no CB r1 obligation", "r4": "targets attempt-2 14 labels; incompatible until explicit reviewed reconciliation; no automatic mapping or builder reliance"},
"tests": {"file": "test_c2_issue_artifacts_r2.py", "methods": 18, "ids": "T00-T17", "T05": "one method; subTests labelled T05a, T05b, T05c", "runCountOtherThan18": "NOT RED at RED; STOP at GREEN", "T07ListCase": "context, publicInputs, cardano, envV2, settleOutref or catalogueActionEntry as a list => EXPORT; a list-valued export itself => ARG (T03)", "T16StrictnessCases": ["T04 driver one-char change => SETUP", "T04 provingKeyBytes 2**64 => SETUP", "T05b every case => PROFILE", "T06 U+D800 in relation relationFormat, logicalCostProfile or fieldModulusLeHex => PROFILE", "T06 U+D800 in a relation extra key => PROFILE", "T06 U+D800 consistently in relation action, export action and catalogue name => PROFILE", "T14 driver one-char change + language => SETUP"], "T16Excludes": ["T02", "T17"], "T06Redundant": "U+D800 in a relation extra key: closed-key refusal, not a CP04 scan discriminator", "tokenExpectation": "historical literal, or hashlib.blake2b(bytes.fromhex(literal envCborHex), digest_size=32) computed in the test; NON-ORACLE; equality through the shared host is consistency, not an oracle", "T17": "SYNTHETIC_NOT_PROOF_NOT_ORACLE public test fixture, not authenticated ledger data; updates statementHex, statementDigestBlake2b256, context, publicInputs, cardano.envV2 claimantVkh, settleOutref and statementDigest, cardano.intentAnchor", "meaningfulness": "identified by method and T05 subTest label; scan mutant must fail T06 via relationFormat, logicalCostProfile, fieldModulusLeHex, export action or intentAnchor cases"},
"history": {"scopedK6": "/home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/root/K6-ACTUAL-ADOPTION.json: one root-adopted current-Rust moriarty-core/1 invoice-partial Groth16 round trip; pins equal K6/K2; class-0 single-party-test-setup/1; fabricated context; custody, not mathematical, binary-source correspondence; preserved as scoped history only; no universal correspondence; lib.rs/abi2.rs never-run docstrings are stale comments"},
"pins": {"host_codec.py": "23252 B 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459", "c2_issue_artifacts.py": "15231 B 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76", "K6-PUBLIC-PROOF.json": "5800 B 930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b", "K2-PUBLIC-SETUP-RECORD.json": "3926 B root-observed (revision 3 SOURCE-MANIFEST setupPublicFixtureRootObserved) 86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b", "contractApiAndPositions": "root manifest only; no self-hash"},
"flags": {"sourceReady": false, "implemented": false, "tested": false, "adopted": false, "builderReliancePermitted": false, "native": false, "operator": false, "wallet": false, "prover": false, "vm": false, "deploy": false, "signing": false, "network": false, "publicVerificationInputsPublishable": false}
}


## S35 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-candidate-20261017/DESIGN-POSITIONS.json
[
  {
    "clauseId": "CP01",
    "clause": "The r2 consumer shall be a new pure module c2_issue_artifacts_r2.py whose only import statements are import hashlib and import host_codec, whose only public function is prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id) with no defaults, which applies the CI02 ARG checks unchanged, and which calls build_env_data, build_c2_mint_data, build_value_data, encode_data, parse_statement_header, statement_digest_hex and statement_public_inputs only as attributes of the unchanged pinned host_codec (23252 B, SHA-256 3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459), never copying or re-implementing them; it shall not import, patch or replace the adopted v1 module c2_issue_artifacts.py (15231 B, SHA-256 23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76), whose bytes, CI01-CI12 contract, format /1, eight labels and test history stay immutable.",
    "position": "FOR",
    "rationale": "Author FOR. The literal text is unchanged from revision 3, which S32 AGREEd."
  },
  {
    "clauseId": "CP02",
    "clause": "When ARG passes, the consumer shall refuse with C2I-SETUP unless the setup record has exactly the 12 SetupRecord keys; format, trustProfile, notice, curve, snark and encoding equal the export.rs constants; driver equals DRIVER_ID mori-k2-driver/0.1.0-k2s1+k4h1; provingKeySha256 is 64 lowercase hex; provingKeyBytes is an exact int in 1..2**64-1; relation is an exact dict; verifyingKeyHex is an exact str of length 976, length-checked before its lowercase-hex scan; and unkeyed BLAKE2b-256 of the 488 VK bytes equals vkDigestBlake2b256; it shall decode no VK point, so the decode_vk IC-prefix, flag, field-range, curve, subgroup, identity and canonical-encoding checks are not performed.",
    "position": "FOR",
    "rationale": "Author FOR. The literal text is unchanged; S32 AGREE."
  },
  {
    "clauseId": "CP03",
    "clause": "When SETUP passes, the consumer shall refuse with C2I-PROFILE unless the relation has exactly the 29 RelationDescriptor keys with the kinds in C2-PROFILE-R2-API.json, including language moriarty/1, semantics moriarty-core/1 and adapter SettlementHashV2; the nine check_profile digests are 64 lowercase hex and sourceSha256 is present as 64 lowercase hex or null; logicalWork is an exact str of 1..39 characters, each in ASCII 0-9, with no leading zero unless it is 0, all checked before any int conversion, and value at most 2**128-1; statementTagHex and familyIdHex equal the ABI v2 literals; headerLen is 343, instanceVariables 3, publicInputCount 2 and bodyLen 0..261801; and abi2ShapeBytesHex is an exact str of even length at most 524288, length-checked before any scan, then lowercase hex whose unkeyed BLAKE2b-256 equals abi2ShapeDigest.",
    "position": "FOR",
    "rationale": "Author FOR. The literal text is unchanged; S32 AGREE. The moriarty/1 and moriarty-core/1 descriptor check is preserved."
  },
  {
    "clauseId": "CP04",
    "clause": "While admitting the setup record, its relation and the export, the consumer shall accept a key only as an exact str naming a field of the closed schema, an integer only as an exact int (never bool) within its kind range, and a str-kind value only as an exact str containing no code point in U+D800..U+DFFF, found by a code-point comparison scan that makes no input-proportional copy and never calls str.encode (a str.isascii() fast path is allowed) and that runs after any length check of that field; otherwise it shall refuse with C2I-SETUP for a setup top-level member, C2I-PROFILE for any relation key or member, and C2I-EXPORT for any export key or member at any depth, except that statementHex, proofHex, x1 and x2 are not scanned at EXPORT and are left to the length-first lowercase-hex checks of STATEMENT and PROOF and the exact-string equality of DIGEST.",
    "position": "FOR",
    "rationale": "Author FOR. Changed only by the N06 requiredFix: the deferred checks are named precisely and the scan is defined for CPython. Stage ownership and all T06 outcomes are unchanged."
  },
  {
    "clauseId": "CP05",
    "clause": "When PROFILE passes, the consumer shall apply the CI05-CI09 checks, narrowed only by CP04, in the order EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, and shall describe its admission only as the selected checks of contract section 3, stating that it performs no decode_vk or decode_proof point check (IC prefix, flags, field range, curve, subgroup, identity, canonical encoding), no setupRecordSha256 file-byte check, no parse_json byte, UTF-8, duplicate-key, trailing-data, depth or node limit, no parse_statement re-encode and no Groth16 pairing, that Rust acceptance of an r2-admitted input is not established, and never asserting any subset, equivalence or conditional-inclusion relation between r2 admission and Rust acceptance.",
    "position": "FOR",
    "rationale": "Author FOR. The literal text is unchanged; S32 AGREE. Section 3 wording is now check-level only (N07), and no acceptance claim was added."
  },
  {
    "clauseId": "CP06",
    "clause": "When every check passes, the consumer shall return an exact fresh dict with the 12 CI10 keys built as in CI10, except that format is moriarty-c2-issue-artifacts/2 and labels is exactly the 16-entry r2 label map of C2-PROFILE-R2-API.json, and shall retain every v1 check at equal or stricter strength, so that by source construction every r2-admitted input is v1-admitted with envCborHex, envelopeTokenNameHex, issueCborHex and mintValueCborHex equal to the v1 values, a property that tests T02 and T17 only sample; T16 samples only strictness, that is v1-admitted inputs that r2 refuses, and is not evidence of that property.",
    "position": "FOR",
    "rationale": "Author FOR. N02: the evidence attribution is corrected to T02 and T17, and T16 is strictness only. Behaviour, the 12 keys, /2 and the 16 labels are unchanged."
  },
  {
    "clauseId": "CP07",
    "clause": "If any check fails, then the consumer shall raise exactly ValueError, never a subclass such as UnicodeEncodeError, whose message starts with C2I-<STAGE>: for the first failing stage in the order ARG, SETUP, PROFILE, EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO, BUILD; it shall rewrap a host ValueError with the current stage code, let non-ValueError environmental errors such as MemoryError propagate unconverted, return no partial result, never mutate either input, keep no input dict or list reachable from module state or from the result (immutable str and int leaves may be shared), and perform no I/O and use no clock, environment, randomness, logging, caching or global mutation.",
    "position": "FOR",
    "rationale": "Author FOR. The literal text is unchanged; S32 AGREE."
  },
  {
    "clauseId": "CP08",
    "clause": "The r2 records shall keep builderReliancePermitted false and every native, operator, wallet, prover, VM, deploy, signing and network flag false; builderReliancePermitted may change only by a separate root source-adoption decision taken after a reviewed r2 implementation, a fresh non-author audit, root-observed meaningful RED then GREEN of the reviewed T00-T17 test and a reviewed downstream contract that explicitly consumes format moriarty-c2-issue-artifacts/2 with the 16 labels, and none of these prerequisites grants that permission automatically; r2 output is not CB-r1-compatible and shall not be routed to CB r1, and this contract imposes no obligation on CB r1.",
    "position": "FOR",
    "rationale": "Author FOR. The literal text is unchanged; S32 AGREE. No gate was added."
  }
]


## S36 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-candidate-20261017/ROOT-INTAKE.json
{
  "recordedAt": "2026-10-17T01:33:37.814233+00:00",
  "sourceJob": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-final-text-opus-repair-20261017",
  "process": {
    "reviewToken": "c2-profile-final-text-repair-20261017",
    "provider": "anthropic",
    "model": "claude-opus-5-5",
    "requestedModel": "claude-opus-5-5",
    "requestedEffort": "high",
    "scope": "Opus-owned finding-driven CP01-CP08 source-contract/API repair only; no test or implementation authoring, no native/private/controller action",
    "maximumAttempts": 1,
    "retries": 0,
    "wallSeconds": 1200,
    "status": "terminal",
    "startedAt": "2026-10-17T01:26:36.863161+00:00",
    "verifiedInputsBefore": 32,
    "promptBytes": 375275,
    "actualEffort": "not independently observed",
    "pid": 3622821,
    "exitCode": 0,
    "endedAt": "2026-10-17T01:32:56.061794+00:00",
    "schemaValid": true,
    "identityVerified": true,
    "inputFilesChangedAfter": [],
    "observedCanonicalModel": "claude-opus-5-5",
    "observedUsageModelKeys": [
      "claude-opus-5-5"
    ],
    "structuredResultSource": "structured_output",
    "reportSha256": "0730733b102893e2a2507a23f514ab63d61debeeb210c091e3eae130e70ee44c",
    "structuredReviewExported": true
  },
  "completeOriginalSchemaValid": true,
  "publicPayloadMapping": "Complete file bodies in crosschecks FILE-* evidence and all8authorclauses in POSITION-* summary. Original requested schema mistakenly allowed0files/0positions, so author used existing schema fields and disclosed it. This is the complete public schema-validated object, not private partial recovery.",
  "all32SourcePinsUnchanged": true,
  "clauseIds": [
    "CP01",
    "CP02",
    "CP03",
    "CP04",
    "CP05",
    "CP06",
    "CP07",
    "CP08"
  ],
  "changedLiteralClauses": [
    "CP04",
    "CP06"
  ],
  "sourceAdopted": false,
  "nativeAuthority": 0,
  "builderReliancePermitted": false,
  "testAndImplementationAuthored": false,
  "files": [
    {
      "path": "C2-PROFILE-R2-CONTRACT.md",
      "bytes": 30809,
      "sha256": "6e2c3989ede633901d7d1a95ee3a081c8eb628fbea199c1aa2858a3eaa2cba34"
    },
    {
      "path": "C2-PROFILE-R2-API.json",
      "bytes": 8563,
      "sha256": "0f592eea77df7531be1edf1685b2b7290add7f9873cda2e11f99c9aed436c3af"
    },
    {
      "path": "DESIGN-POSITIONS.json",
      "bytes": 7631,
      "sha256": "a863c35ba6b22a41c2d9a8a5d7f87c8a076b4cc92927cc4a10f71e1683cf5e44"
    }
  ]
}


## S37 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-candidate-20261017/SOURCE-MANIFEST.json
{
  "format": "c2-profile-r2-revision4-source-manifest/1",
  "sources": [
    {
      "id": "S01",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/C2-PROFILE-R2-CONTRACT.md",
      "bytes": 23830,
      "sha256": "03f03887df741ce21556158a0490e24890ba33f310ce276f5060d1013f9e5e03"
    },
    {
      "id": "S02",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/C2-PROFILE-R2-API.json",
      "bytes": 5750,
      "sha256": "965b880d5b440376faf495b757ba9b06460e9e5755e943b23c81c789982af0fb"
    },
    {
      "id": "S03",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/DESIGN-POSITIONS.json",
      "bytes": 7725,
      "sha256": "8536095bbda3eab22977c7f3002f26bac1609fe25a881fff2729463bfe89be0d"
    },
    {
      "id": "S04",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-corrected-candidate-20261016/SOURCE-MANIFEST.json",
      "bytes": 8714,
      "sha256": "a3ef25f34ef819d0c44622eb01169d8015434352f7b423f4beb856ac4078c7d9"
    },
    {
      "id": "S05",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/MODEL-REVIEW.json",
      "bytes": 44551,
      "sha256": "f46928736648d84b179c6815334967f94cf308a711072a2ac0f051adab088ed8"
    },
    {
      "id": "S06",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-policy-fresh-opus-source-audit-20261016/ROOT-QUALIFICATION.json",
      "bytes": 1055,
      "sha256": "f8b0c16c9d69731ce02a0a6e2e721656c2785d01274b58128b2690e3a8a3a15a"
    },
    {
      "id": "S07",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-CONTRACT.md",
      "bytes": 11790,
      "sha256": "c51ff84a153828547060ff00f801ca0beca200b95eb96b4de0388d83f91f5ce8"
    },
    {
      "id": "S08",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/C2-PROFILE-R2-API.json",
      "bytes": 3271,
      "sha256": "7becb4dee68d95fe8830e427c6a0a67e5b92c06e249ef2328cd76b3fc2b5a7b5"
    },
    {
      "id": "S09",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-policy-candidate-20261016/DESIGN-POSITIONS.json",
      "bytes": 5830,
      "sha256": "b19d4e6c6d5481e19ba3093a964cb986bbd811b2ef7eac15f55886a2037160fb"
    },
    {
      "id": "S10",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/C2-ISSUE-ARTIFACT-CONTRACT.md",
      "bytes": 12645,
      "sha256": "f1a904ad80f5ebf7be6518cb5904da66b729913bf2b59e97555722307b51c851"
    },
    {
      "id": "S11",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-implementation-candidate-20261016/c2_issue_artifacts.py",
      "bytes": 15231,
      "sha256": "23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76"
    },
    {
      "id": "S12",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/REPORT.md",
      "bytes": 11465,
      "sha256": "05c66c0dbeac99f387403498b4012a9bb8f93e8c5ea84602f587312d7f98c34d"
    },
    {
      "id": "S13",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-fresh-opus-full-candidate-audit-20261016/FINDINGS.json",
      "bytes": 5863,
      "sha256": "00e54bedaf9d2960d0fab408386142a944a293ada9b8ff332e462c9e1c102d6b"
    },
    {
      "id": "S14",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-reviewed-component-root-adoption-20261016/SOURCE-ADOPTION.json",
      "bytes": 2690,
      "sha256": "61327c199afd1214b46e770c8af60f75ace84a4f6fdbb8d86f68df3dcc64642f"
    },
    {
      "id": "S15",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K2-PUBLIC-SETUP-RECORD.json",
      "bytes": 3926,
      "sha256": "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"
    },
    {
      "id": "S16",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-builder-public-prerequisites-sol-20261016/inputs/K6-PUBLIC-PROOF.json",
      "bytes": 5800,
      "sha256": "930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b"
    },
    {
      "id": "S17",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/relation.rs",
      "bytes": 14625,
      "sha256": "e18ae46c5ec0650f0c5dbc3c8b39262eb432c7d2e3be7b3b14cc01fe285493d5"
    },
    {
      "id": "S18",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/export.rs",
      "bytes": 15374,
      "sha256": "8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c"
    },
    {
      "id": "S19",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/lib.rs",
      "bytes": 2783,
      "sha256": "1b13cdc57fa21ab78a5bd6c33ba64d4628d8ed388f4a7121b40b18c5cae4b830"
    },
    {
      "id": "S20",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/hexutil.rs",
      "bytes": 2273,
      "sha256": "d242105dad5dee03c393e95b6b5ba22b19497253d1a9e9135d746b001b7c19ae"
    },
    {
      "id": "S21",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-consumer-profile-r2-opus-contract-20261016/PROCESS.json",
      "bytes": 772,
      "sha256": "c3c5128a8135e66893bb697479d8befe83dc996f546527577d5cc6a9a0385242"
    },
    {
      "id": "S22",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-preprod-host/host_codec.py",
      "bytes": 23252,
      "sha256": "3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459"
    },
    {
      "id": "S23",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-consumer-source-candidate-20261016/test_c2_issue_artifacts.py",
      "bytes": 20643,
      "sha256": "4fa543bf5708957ed9beabb32c2614e50f8c0097fb157ed2c9bd6ed519c9f6f0"
    },
    {
      "id": "S24",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-CONTRACT.md",
      "bytes": 8466,
      "sha256": "93420c97fe7ed382e96b9a9af43ac605e9616298aab670525ebe83eed1f965d4"
    },
    {
      "id": "S25",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-proofexport-c2-builder-source-candidate-20261016/C2-BUILDER-API.json",
      "bytes": 7190,
      "sha256": "abe6631ffc60a06916c13e912ded326429675d2ac03e9fc508d7aab0a3bb036d"
    },
    {
      "id": "S26",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/claim.rs",
      "bytes": 9280,
      "sha256": "f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3"
    },
    {
      "id": "S27",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/encoding.rs",
      "bytes": 9631,
      "sha256": "2545d7799449f5580286a47949ca0116197772b3a24ddf1ee7b8c05c9b1c6729"
    },
    {
      "id": "S28",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/k2/src/input.rs",
      "bytes": 6262,
      "sha256": "646c1aad3cea935de2e084e2cb1518772e8bd0fc7e57eb6b3f4581edc67b5b42"
    },
    {
      "id": "S29",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/src/abi2.rs",
      "bytes": 19304,
      "sha256": "51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb"
    },
    {
      "id": "S30",
      "path": "/home/charl/Moriarty/.worktrees/utxo/experiments/moriarty-cardano-rust/moriarty/crates/mori-core/src/json.rs",
      "bytes": 5391,
      "sha256": "faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183"
    },
    {
      "id": "S31",
      "path": "/home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/root/K6-ACTUAL-ADOPTION.json",
      "bytes": 4005,
      "sha256": "83a6e4fd1624774ee050ce88d3671187548e65c294ad147e5910a42685a6cb4d"
    },
    {
      "id": "S32",
      "path": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-corrected-policy-fresh-opus-audit-20261016/MODEL-REVIEW.json",
      "bytes": 48625,
      "sha256": "34515ffb645944d6908d3d357941c9413c7c636fc5f30144e5f14a1c9a3f81ad"
    }
  ],
  "files": [
    {
      "path": "C2-PROFILE-R2-CONTRACT.md",
      "bytes": 30809,
      "sha256": "6e2c3989ede633901d7d1a95ee3a081c8eb628fbea199c1aa2858a3eaa2cba34"
    },
    {
      "path": "C2-PROFILE-R2-API.json",
      "bytes": 8563,
      "sha256": "0f592eea77df7531be1edf1685b2b7290add7f9873cda2e11f99c9aed436c3af"
    },
    {
      "path": "DESIGN-POSITIONS.json",
      "bytes": 7631,
      "sha256": "a863c35ba6b22a41c2d9a8a5d7f87c8a076b4cc92927cc4a10f71e1683cf5e44"
    }
  ],
  "scope": "Authorcandidateonly; exactfreshnonauthorvotesrequired; no test/implementation/nativeauthority"
}


## S38 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-final-text-opus-repair-20261017/FINDINGS.json
[
  {
    "id": "N01-T16-normative-contradiction",
    "severity": "medium",
    "summary": "REPAIRED. T16 no longer says 'every ADMIT cell above'. It lists exactly the cases v1 admits and r2 refuses, and excludes T02 and T17.",
    "evidence": "Contract section 6 T16 row and API tests.T16StrictnessCases/T16Excludes. Listed cases: T04 driver one-char change (SETUP) and provingKeyBytes 2**64 (SETUP); every T05b case (PROFILE); T06 relationFormat/logicalCostProfile/fieldModulusLeHex, relation extra key, and consistent action/export action/catalogue name (PROFILE); T14 driver one-char change + language (SETUP).",
    "requiredFix": "None further; fresh non-author vote."
  },
  {
    "id": "N02-CP06-T16-sampling-attribution",
    "severity": "medium",
    "summary": "REPAIRED. CP06, section 5 and API compatibility now attribute subset and byte-equality sampling to T02 and T17 only. T16 is described as sampling strictness only.",
    "evidence": "CP06 literal now ends 'a property that tests T02 and T17 only sample; T16 samples only strictness, that is v1-admitted inputs that r2 refuses, and is not evidence of that property.' Section 5 and API admissionVsV1/strictnessVsV1 match.",
    "requiredFix": "Fresh non-author re-vote of the changed CP06 text."
  },
  {
    "id": "N03-T05-grouping-vs-18-count",
    "severity": "medium",
    "summary": "REPAIRED. The file has 18 methods (T00-T17), and T05a/T05b/T05c are labelled subTests inside one T05 method. Any run count other than 18 is NOT RED at RED and STOP at GREEN. Meaningfulness results are identified by subTest label.",
    "evidence": "Section 6 preamble and T05 row IDs; section 7 RED, NOT RED, GREEN and meaningfulness; API tests.methods/T05/runCountOtherThan18.",
    "requiredFix": "None further."
  },
  {
    "id": "N04-T07-top-level-list-misstaged",
    "severity": "medium",
    "summary": "REPAIRED. 'given as a list' now covers only the six nested objects (EXPORT). A list-valued export itself is ARG and belongs to T03.",
    "evidence": "Section 6 T07 row; API tests.T07ListCase. No other stage expectation changed.",
    "requiredFix": "None further."
  },
  {
    "id": "N05-ambiguous-v1-cells-T04-T14",
    "severity": "low",
    "summary": "REPAIRED. In T04, '(v1 ADMIT)' now applies only to provingKeyBytes 2**64 and the driver one-char change. 0, -1, True and an int driver are v1 SETUP. T14 specifies a driver one-char change.",
    "evidence": "Section 6 T04 row (v1 'SETUP unless noted') and T14 SETUP/PROFILE pair.",
    "requiredFix": "None further."
  },
  {
    "id": "N06-CP04-DIGEST-wording-and-allocation",
    "severity": "low",
    "summary": "REPAIRED. CP04 and the API now name the STATEMENT/PROOF length-first lowercase-hex checks and the DIGEST exact-string equality. The scan is defined as no input-proportional copy, never str.encode, with a str.isascii() fast path allowed.",
    "evidence": "CP04 literal; stage note on deferred fields; API strRule and stageOwnership.deferredCheck. Outcomes (T06 stages) are unchanged.",
    "requiredFix": "Fresh non-author re-vote of the changed CP04 text."
  },
  {
    "id": "N07-section3-check-level-wording",
    "severity": "low",
    "summary": "REPAIRED. The text now reads 'Each, except the Added-by-r2 checks, mirrors'. 'Rust also admits ... networkId 1' is replaced by a source-reading statement that check_export has no networkId check (not executed). No acceptance claim is added.",
    "evidence": "Section 3 Performed and Acceptance; API rust.checkLevel.",
    "requiredFix": "None further."
  },
  {
    "id": "N08-S31-stale-never-run-evidence",
    "severity": "low",
    "summary": "REPAIRED. S31 is cited as the one scoped historical current-Rust K6 round trip. The 'never compiled or run' docstrings in lib.rs and abi2.rs are stale comments, not evidence. driverId, circuitVkCorrespondence and statementReencode labels are kept, and no universal correspondence is inferred.",
    "evidence": "Section 8 driver bullet; section 3 source-reading note; API history.scopedK6.",
    "requiredFix": "None further."
  },
  {
    "id": "N09-token-and-T17-expectation-independence",
    "severity": "low",
    "summary": "REPAIRED. The test computes hashlib.blake2b(digest_size=32) token expectations over literal envCborHex for the five non-historical T02 expiries and for T17. They are labelled NON-ORACLE, and the shared host is not treated as an oracle. T17 names statementDigestBlake2b256 and envV2.statementDigest among the updated fields and stays SYNTHETIC_NOT_PROOF_NOT_ORACLE; the fixture is not authenticated ledger data.",
    "evidence": "Section 6 token-expectation note, T02 and T17 rows, closing paragraph; API tests.tokenExpectation and tests.T17.",
    "requiredFix": "None further."
  },
  {
    "id": "N10-manifest-and-process-notes",
    "severity": "info",
    "summary": "REPAIRED. The section 1 K2 row records 3926 B root-observed. The root manifest pins the contract, the API and this step's author positions. R4 stays incompatible, with no automatic mapping or builder reliance.",
    "evidence": "Section 1 table and pin sentence; section 5 R4 bullet; API pins and compatibility.r4.",
    "requiredFix": "Root pins all three when recording the manifest."
  },
  {
    "id": "N11-T06-nondiscriminating-case",
    "severity": "info",
    "summary": "REPAIRED. The T06 relation extra-key surrogate case is kept and marked REDUNDANT. The scan mutant must fail through the relationFormat, logicalCostProfile, fieldModulusLeHex, export action or intentAnchor cases.",
    "evidence": "Section 6 T06 row; section 7 meaningfulness; API tests.T06Redundant/meaningfulness.",
    "requiredFix": "None further."
  }
]


## S39 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-final-text-opus-repair-20261017/REPORT.md
Author revision 4 (claude-opus-5-5, tools off). It repairs immutable revision 3 against the exact S32 requiredFix for each of N01-N11. Nothing was executed, imported, hashed or compiled, and no test or implementation was authored.

Clause text changed in two places:
- CP04 (N06): the checks for deferred fields are now named exactly. STATEMENT and PROOF use length-first hex checks; DIGEST uses exact-string equality. The scan makes no input-proportional copy, never calls str.encode, and may use an isascii fast path.
- CP06 (N02): subset and byte-equality evidence is now attributed to T02 and T17 only. T16 shows strictness only.

The literal text of CP01-CP03, CP05, CP07 and CP08 is unchanged.

Test-policy repairs:
- T16 now lists exactly which cases v1 admits and r2 refuses (N01).
- There are 18 methods. T05a/b/c are subTests of T05, and any run count other than 18 is NOT RED/STOP (N03).
- T07's list-valued cases cover only the six nested objects; a list-valued export itself is ARG (N04).
- The T04 and T14 cells are fixed (N05). Section 3 wording is check-level only (N07). S31 is cited as scoped history (N08).
- hashlib token expectations are added and labelled non-oracle (N09). K2 is recorded as 3926 B, root-observed (N10). The T06 extra-key case is marked redundant (N11).

Stage expectations, labels, Rust identities, v1 history, the 20 families and the permissionless model are unchanged.

Schema conflict: `files` allows 0 items (only SECURITY-REVIEW.md/ERRATA paths), and `positions` also allows 0 items. So the full verbatim file bodies and the eight author-FOR literal clauses are in crosschecks: FILE-C2-PROFILE-R2-CONTRACT.md, FILE-C2-PROFILE-R2-API.json and POSITION-CP01..CP08. Root should extract them, compute pins and check that each clause appears exactly once. Every readiness flag is false until a fresh non-author vote.


## S40 FULL PUBLIC INPUT /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-gemini-muse-authorization-20261017/USER-AUTHORIZATION.json
{
  "recordedAt": "2026-10-17T02:19:14.174634+00:00",
  "userMessage": "authorize gemini muse",
  "scope": "Gemini and Muse replace unavailable Opus for independent review of the repaired serializer source policy. No native/wallet/key/proving/VM/ledger authority is added.",
  "candidate": "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-c2-profile-r2-revision4-candidate-20261017",
  "requiredReviewers": [
    "Gemini via AGY,gemini-4-argon-high,highrequested",
    "Meta muse-wonder-1.0,highrequested"
  ],
  "history": "Earlier Opus author candidate and two failed fresh reviews remain immutable; failures supply no vote.",
  "independence": "Fresh nonauthor reviewcontexts; neither new reviewer receives the other new reviewer response. This is the user-selected two-reviewer source workflow, not a claim of formal blinded Council quorum.",
  "nativeAuthority": 0
}
