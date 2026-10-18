# c2_issue_artifacts_r2.py implementation notes (author: claude-opus-5-5, high; tools Read/Write/Edit only)

Status: this is an author-proposed implementation. It has not been reviewed or adopted, and it has never been executed. I did not run, import, compile, hash or test anything. Every statement about runtime behaviour is a prediction made by reading the source. Author FOR is not adoption. Root inspection and a fresh non-author full-source Opus 5.5 audit are still required before GREEN. builderReliancePermitted stays false, and every native, operator, wallet, prover, VM, deploy, signing and network flag stays false.

## Sources read (public packet only; no private or provenance paths were followed)
- S33 (C2 profile r2 contract, rev 4) and S34 (API), both read in full.
- S35, S42, S44 and S45 (adopted policy context).
- S11, the pinned v1 consumer, unchanged: 15231 B, sha256 23c3f99d….
- S22, the pinned host_codec: 23252 B, sha256 3c18539e….
- The full reviewed test test_c2_issue_artifacts_r2.py: 58800 B, sha256 b9c19211….
- TEST-SOURCE-REVIEW.json, ROOT-RED-DECISION.json and ROOT-RED-RESULT.json.
- The fixtures S15 (K2 setup record) and S16 (K6 proof export).

Only the root records establish the RED result, and its scope is the missing-implementation RED only. In S33 and S34, "absent/unexecuted" is preserved history.

## Clause traceability
- **CP01**
  - The only import lines are `import hashlib` and `import host_codec`.
  - The single public callable is `prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id)`, with no defaults. Every helper and constant starts with `_`.
  - The ARG block is the v1 CI02 text, unchanged.
  - Each of the 7 host APIs is called in place as `host_codec.<api>(…)` at its call site:
    - `parse_statement_header` at STATEMENT;
    - `statement_digest_hex` and `statement_public_inputs` at DIGEST;
    - `build_env_data`, `encode_data` (3 calls), `build_c2_mint_data` and `build_value_data` at BUILD.
  - No host API is bound to another name, passed around as a value, copied, patched or re-implemented. Unlike v1's `_host(code, api, …)`, no function object is handed to a helper.
  - v1 is not imported or referenced. The premises, notice and ABI literals are restated as contract/export.rs constants.
- **CP02 (SETUP)**
  - `_SETUP_RECORD` is a closed schema of 12 keys.
  - format, trustProfile, notice, curve, snark and encoding are fixed literals.
  - driver is fixed to `mori-k2-driver/0.1.0-k2s1+k4h1`.
  - provingKeySha256 must be hex64. provingKeyBytes must be an int in 1..2**64-1.
  - relation must be an exact dict. Its members are left to PROFILE.
  - verifyingKeyHex is checked through `_is_hex(…, 976)`: length first, then the lowercase scan.
  - The unkeyed `hashlib.blake2b(digest_size=32)` digest of the VK bytes must equal vkDigestBlake2b256.
  - No VK point is decoded.
- **CP03 (PROFILE)**
  - `_RELATION` is a closed schema of 29 keys with the S34 kinds:
    - 5 fixed: language, semantics, adapter, statementTagHex and familyIdHex;
    - 8 int: headerLen 343, instanceVariables 3, publicInputCount 2, bodyLen 0..261801, actionIndex u16, and relationVersion, constraints and witnessVariables u64;
    - 9 hex64;
    - sourceSha256 as a required key holding hex64 or null;
    - 4 str;
    - logicalWork;
    - abi2ShapeBytesHex.
  - `_is_decimal` checks, in order: length 1..39, ASCII 0-9 by code-point comparison, no leading zero, and only then `int()` with the bound 2**128-1.
  - `_is_shape_hex` checks length ≤524288 and evenness first, then the lowercase scan. After all members are admitted, the BLAKE2b-256 of the shape bytes must equal abi2ShapeDigest.
- **CP04**
  - Every key must be an exact str that names a schema field (`_names_field` after `type(key) is str`). This check runs before any value is visited.
  - Ints must be `type(v) is int` and within the kind's range, so bool is refused.
  - Str-kind values go through `_is_text`:
    - an exact str check;
    - a `str.isascii` fast path;
    - otherwise a per-character `ord` comparison against 0xD800..0xDFFF, with no copy and no str→bytes conversion.
  - The scan covers relation relationFormat, logicalCostProfile, action and fieldModulusLeHex, and export action, catalogueActionEntry.name, intentTargetHashV2 and intentAnchor.
  - statementHex, proofHex, x1 and x2 use the `deferred` kind: an exact str check only at EXPORT. They are refused later by:
    - the STATEMENT length-first hex check;
    - the PROOF `_is_hex(…, 384)`;
    - the DIGEST exact-string equality.
  - Stage ownership comes from the code argument: setup members → C2I-SETUP, relation members → C2I-PROFILE, export members at any depth → C2I-EXPORT.
- **CP05**
  - The stages after PROFILE are EXPORT, RELATION, STATEMENT, CONTEXT, DIGEST, PROOF and CARDANO. Each is the v1 check with only the CP04 narrowing.
  - The module docstring describes admission as the selected checks of §3, lists every check that is not performed, and states that Rust acceptance is not established. It asserts no subset, equivalence or inclusion relation.
- **CP06**
  - The result is a fresh 12-key dict with format `/2` and `labels = dict(_LABELS)`: exactly 16 entries, rebuilt on every call from a tuple of pairs.
  - The v1 checks are retained at equal or stricter strength:
    - stricter: driver fixed, provingKeyBytes bounded, relation closed, and the scan on 8 str fields;
    - equivalent: the hex scans, `char in "0123456789abcdef"` per character, give the same accept set as v1's `set(v).issubset(…)`;
    - identical: the CONTEXT per-key comparison is equivalent to v1's dict equality over the same 10 closed keys, and the CARDANO derived-dict comparison and BUILD arguments are identical to v1.
  - The by-construction subset property is sampled only by T02 and T17, and is not proven.
- **CP07**
  - `_refuse` raises an exact `ValueError("C2I-<STAGE>: …")`.
  - Host refusals are caught with `except ValueError` only and re-raised through `_host_refusal(code, error)` as an exact ValueError with the stage code, `from None`. Anything other than ValueError (for example MemoryError) propagates.
  - No partial result is returned, and inputs are never written.
  - The result holds no input dict or list. The only input leaves it shares are the setupRecordSha256 str, the expiry int and the policy_id bytes.
  - BUILD is reachable only through a host refusal, which admitted inputs do not trigger.
- **CP08**
  - No flag, bridge or routing exists in the source. The docstring states that the output is not for CB r1 and not for builder reliance.

## Effects (full)
- At import, the only effects are binding `hashlib` and `host_codec` and defining immutable module constants: str, int and tuples of these. There are no dicts, lists or sets at module level.
- At call time, the only effects are:
  - local computation;
  - `hashlib.blake2b(digest_size=32)` three times (VK, shape bytes, env CBOR);
  - `bytes.fromhex` on admitted hex (the VK, shape, statement and BUILD arguments are length-bounded conversions, not part of the surrogate scan);
  - the 7 host attribute calls.
- There is no I/O, clock, process-setting read, randomness, logging, caching, exec/eval, dynamic import, class, closure, lambda, function attribute, default argument or module-state write, and no mutation of inputs or of host_codec.

## Source-inspection obligations (cannot be proved by GREEN; for root and the fresh auditor)
1. **L01 (no host alias):** confirm that `host_codec.` appears only directly before each of the 7 API names at the call site, and that no module- or function-level name is bound to a host API at import or at call time.
2. **L02 (no host patching):** confirm that no assignment, setattr or del targets `host_codec` or `hashlib`, either at import or at call time.
3. **L03 (fresh nested containers, no hidden state):**
   - Confirm that envData, issueData and mintValueData come from host factories that construct fresh tuples and lists on each call (S22 docstrings and code), and that labels is a new dict.
   - Confirm there is no first-use cache, non-container state, function attribute, closure or class state.
   - Confirm that module constants contain no mutable container.
4. **I02 / CP04:**
   - `.encode` does not appear anywhere in the source. The only "encode" substrings are `encode_data` and docstring prose.
   - `_is_text` makes no input-proportional copy: no `set()`, slicing, `join`, `replace`, `translate` or conversion to bytes.
   - Every hex, shape, decimal and statement check tests length before its scan.
   - The deferred fields are not scanned at EXPORT.
5. **I01:** confirm that no docstring or comment line begins with `import ` or `from ` (the T01 line scan).
6. **T15 tokens:** confirm the source contains none of the T15 EFFECT_TOKENS: fixture-identity prefixes, the expiry literal, the effect calls, and the words containing "environ" and "global". The prose was worded to avoid them. This is checked by reading, not by running.
7. Confirm by reading S22 that `str(error)` on a host ValueError and the `int()` on a value of at most 39 ASCII digits cannot raise.

## Limitations and preserved non-claims
- The labels stay honest:
  - rustAcceptance NOT_ESTABLISHED, circuitVkCorrespondence OPEN, groth16 NOT_VERIFIED and ledger NO_CLAIM;
  - vkCurveValidity, vkProfile, proofCurveValidity, statementReencode and jsonParseLimits NOT_CHECKED;
  - the expiry, context and policyId labels mark those values as caller supplied and unauthenticated;
  - driverId DECLARED_NAME_NOT_PROVENANCE and trustProfile single-party-test-setup/1 (TEST ONLY, forgeable).
- No point decoding, proof decoding, pairing or re-encode is performed, and there is no Rust subset or equivalence claim and no proof of compiled semantics. Serde mechanics are not reproduced: closed keys and integer widths are Python analogues (R4-L01).
- The correspondence between hashlib BLAKE2b and Rust blake2b256 is not established here.
- There is no /1→/2 bridge. v1 bytes, format /1 and the 8 labels are untouched.
- The output is not CB-r1-compatible and must not be routed to CB r1.
- R4 depends on attempt 2 (14 labels) and stays incompatible until there is an explicit reviewed reconciliation.
- builderReliancePermitted needs a separate root decision, taken after audit, GREEN and a reviewed downstream /2 consumer.
- All 20 UTxO families stay OPEN, as do the general financial, ACTUS, DeFi, recursion, PCD, private-handoff and multiparent tracks, authenticated ledger claims, and Preview and Preprod.
- No fixture identity, token or expiry is hardcoded. The only literals are contract and export.rs constants.
- If the pinned K2/K6 records or T17 are refused at GREEN, STOP for review. Fixtures and tests must not be edited.
