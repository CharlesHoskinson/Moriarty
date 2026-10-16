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
