# Provisional S0 wire codec implementation plan

**Goal:** implement an isolated candidate `/3` authorization codec and targeted discriminators. This is authorized prototype work; no normative vote, migration or common-document change occurs.

**Architecture:** standalone Node ESM, fixed ordered tagged binary records, explicit transfer/repayment variants and a SHA-256 content digest. JSON fixtures use canonical decimal strings for integers. Only new files in this directory are in scope.

**Alternatives:** canonical JSON would need duplicate-key and whitespace rules; CBOR would require a dependency or additional parser. Fixed tags and widths provide a smaller dependency-free experiment and make hostile byte offsets explicit. This choice is provisional.

**Scope:** built-in Node crypto/assert/test/fs only; no wallet invocation, signing, signature verification, proof generation, ledger integration, or Source/Core/K/Quint consumer changes. The effect commitment is supplied and signed; equality to actual effects is an external consumer obligation. Caps are experimental choices, not observed network limits.

## Task 1: independently specified fixtures and failing tests

- [x] Write `SPEC.md` with exact header, tag/type order, operation variants, numeric/size caps, rejection codes and unknown external premises.
- [x] Write `fixtures.json` with transfer and existing-obligation repayment values independent of codec implementation.
- [x] Write `codec.test.mjs` to compare a separately assembled transfer byte vector and round trips, then mutate header/tags/lengths/numbers/empties and signed bindings.
- [x] Run `node --test --test-reporter=tap experiments/moriarty-language/formal/mil4/wire/codec.test.mjs`; record expected missing-codec failure. Dynamic import fallback lets the missing API fail as an assertion rather than hiding it as a loader error.

## Task 2: minimal codec and hostile decoding

- [x] Implement `codec.mjs` exports `encodeAuthorization(value): Buffer`, `decodeAuthorization(bytes): object`, `authorizationDigest(value): string`, and immutable public caps/constants. Errors expose stable `code`.
- [x] Re-run the same test command; observed 72/72 passing.
- [x] Check for acceptance defects; none found by this targeted corpus. Add a rule-reaching hostile case before any later repair.

## Task 3: reproducible candidate evidence

- [x] Write `record-results.mjs` to run the targeted test command, save complete TAP output, runtime/command/status/counts and SHA-256 artifact digests to `results.json` and `test-output.tap`.
- [x] Run `node experiments/moriarty-language/formal/mil4/wire/record-results.mjs` and inspect its output and recorded results; result status is recorded independently in results.json.
- [x] Write `RESULT.md` stating exact observed scope and unverified wallet/hash/proof/ledger/cap/consumer loci. Do not close W-D2 or alter other files.

No commit is made from this shared working tree: the parent will review and integrate the isolated artifact.

## Post-audit repair, 2026-09-30

- [x] Reproduce long-string guard and shared replay identity failures before repair: 102 tests, 96 passed, six failed; preserve `post-audit-red-output.tap`.
- [x] Apply string length guards before ID/decimal regex validation and byte allocation/BigInt conversion.
- [x] Require `TRUNCATED` for every transfer and repayment prefix; directly hash stored golden bytes; add nested key-order, wire validity, repayment hostile-byte and content-binding controls.
- [x] Give repayment a distinct nonce; regenerate expected bytes, SHA-256, length and offsets using the independent Python generator alone.
- [x] Correct original-vector provenance claims to post-audit reproduction; retain the frozen audit candidate as historical evidence.
- [x] Observe 102/102 current checks and both independent reference vectors passing; record actual runs. W-D2 and all named consumer loci remain open.

## Metadata and descriptor snapshot repair, 2026-09-30

- [x] Add twelve controls; reproduce ten failures before repair with 104/114 passing; preserve `metadata-snapshot-red-output.tap`.
- [x] Snapshot required root and operation descriptor values once; reuse the captured kind descriptor; encode and check validity/positive amount from those same values.
- [x] Check decoder intrinsic byte-view brand/type and metadata, guard actual size before bounded copy, and map incompatible/forged/proxied/detached views to `SHAPE`.
- [x] Preserve all 102 prior controls and both independently constructed golden vectors; observe 114/114 passing.
- [x] Refresh current result, independent reproduction receipt and artifact hashes. Frozen audits stay historical; W-D2 and consumer acceptance stay open.
