## ADDED Requirements

### Requirement: Item 1.2 source repair scope and exact live labels
After both fresh specification approvals, Grok SHALL repair the existing `records.py` binding, candidate, review, acceptance, campaign-stage and evidence-applicability consumers and the receipt interpretation in their existing CLI/store callers. Source tests SHALL use isolated fixtures only. This scope SHALL NOT create a live action, adopt canonical history, apply a resource amendment or dispatch a process. Missing concrete live records SHALL remain unavailable while source validation is implemented.

For this live path the exact values SHALL be: binding and campaign status `admitted-i2-source-only`; review scope and campaign scope `I2 live executor source correctness only`; source-acceptance status `complete-i2-source-only`; stage `i2`; owner `MC02`; evidence profile `finalized-financial-settlement`; candidate `digestAlgorithm` `SHA256 compact sorted-key JSON of ownedFiles mapping`. Digest computation SHALL use UTF-8 compact sorted-key JSON of the exact owned-file hash mapping, as the existing candidate contract specifies. Values SHALL match exactly, without prefix/substring or case-folded label acceptance. Source acceptance SHALL NOT make stage I2 complete or establish financial settlement.

#### Scenario: Current exact live-source candidate
- **WHEN** isolated current records contain the exact live labels, all current owned/input hashes, both matching source reviews, valid source acceptance and existing I2 prerequisites
- **THEN** the repaired consumers accept only those source/currentness predicates; absent live resources/history still prevent execution.

#### Scenario: Design or near-miss labels
- **WHEN** a live record uses a historical design status/scope/profile, adds a suffix to a design/live label, supplies a digest algorithm merely containing SHA256, or changes label case
- **THEN** live admission rejects it. Authentic historical design records retain their original checks and accepted scope; no new live permission follows.

### Requirement: Current source reviews and source-only acceptance
For the live campaign only, existing field `review` SHALL reference the Opus receipt and `sourceReview` SHALL reference the Astra receipt. Each SHALL be a contained current file, conform to existing review fields and carry verdict `APPROVED`, the exact live review scope and exact candidate digest. Reviewer/model identity SHALL establish fresh non-author `claude-opus-5` and `gpt-6-astra` medium respectively; unavailable, substituted, self-authored, stale or blocking receipts SHALL reject. Requested and returned identities SHALL remain retained. The new current binding's hash-verified inputs SHALL include these immutable receipt files; candidate source inputs SHALL not include later reviews or mutable controls. Freeze source/candidate first, then reviews, then the current binding, avoiding self-reference.

The current acceptance SHALL use the existing acceptance fields with status `complete-i2-source-only`, exact candidate hash, `sourceCandidate` equal to the current candidate manifest path and `review` equal to the campaign's Opus path. The current binding SHALL also commit that immutable acceptance file. Both campaign review paths are checked even though acceptance retains the existing singular `review` field. Exact current live source acceptance SHALL use direct owned/input equality, not require or manufacture a historical publication transformation. Historical design acceptance continues through its original publication-bridge checks.

Source/specification/resource receipts SHALL not be stored as actual-result evidence. Receipt ingestion SHALL classify the exact new live-source scope as source review; it SHALL preserve task 1.1a's universal rule that an approval clears only explicitly named same-lineage findings. Actual financial-result acceptance is not implemented by these source statuses.

#### Scenario: Both bound source receipts
- **WHEN** Opus and Astra independently approve the same exact source candidate and all source-acceptance references agree
- **THEN** source validation passes without closing I2, altering historical reviewers or clearing unnamed findings.

#### Scenario: One receipt or false result promotion
- **WHEN** a receipt is absent/hash-mismatched, identifies the wrong provider/scope, or source approval is presented as actual ledger success
- **THEN** the respective review/result predicate fails; specification or source approval never supplies missing actual-result evidence.

### Requirement: Digest-bound single-path SP01 prerequisite bridge
The new I2 campaign's existing `reconciliation` field SHALL be a contained path string. The current I2 binding's `inputs` SHALL commit the exact bytes at that path. That reconciliation file SHALL be a closed object containing `schema`, `path`, `originalPath`, `originalSha256`, `currentSha256`, `diffPath`, `diffSha256`, `opusDisposition`, `astraDisposition`; schema is exactly `moriarty.sp01-prerequisite-disposition/1`. Disposition entries SHALL each be a closed `{path,sha256}` reference to a contained immutable receipt. This is a narrowly scoped existing-record extension, not a registry or self-authorizing free-standing file.

`path` SHALL equal only `openspec/sprints/sp01-financial-contract-and-execution-admission.md`; `originalPath` SHALL equal `plugins/moriarty-dev/tests/fixtures/sp01-financial-contract-and-execution-admission.md`. Original SHA SHALL equal `fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172` and both historical binding/candidate commitments. Current SHA at this specification is `bc60ffef020b282954f5543db3b00f51f06d45dd113cd68d853967ff50acc23b`; changing it requires a new exact-diff disposition and current binding, not an edited historical hash. Diff bytes SHALL be UTF-8 output of Python `difflib.unified_diff` on each decoded UTF-8 file's `splitlines(keepends=True)`, with `fromfile=originalPath`, `tofile=path`, context `n=3`, `lineterm="\n"` and no timestamps; reviewers must see both complete files. Disposition receipts SHALL use existing review fields, verdict `APPROVED`, exact scope `SP01 RP01-MC02 non-substantive prerequisite disposition only`, the two required fresh non-author identities, and `candidateSha256` equal to SHA-256 of UTF-8 compact sorted-key JSON mapping exactly `originalSha256,currentSha256,diffSha256`. Findings SHALL explicitly approve non-substantive impact on accepted RP01-MC02. Source/specification approval alone is not a disposition.

`_entry_eligible`/`_stage_complete`/`_prerequisite_campaign` SHALL pass this authenticated bridge context to `_verify_binding` and `_verify_candidate` only while checking that prerequisite for the new I2 action. The original bytes substitute for that single path in both historical input maps; all other inputs remain checked against current bytes. Registered SP01 design actions remain stale against the edited current input. Missing original bytes, changed receipt/diff/current/fixture bytes, substantive changes or unreviewed disposition SHALL leave the prerequisite unmet until new reviewed design acceptance exists.

#### Scenario: Authenticated scoped prerequisite reuse
- **WHEN** the new I2 binding commits the exact bridge and both hash-bound dispositions approve the exact non-substantive diff, while original bytes and all other inputs satisfy historical commitments
- **THEN** only that RP01-MC02 prerequisite may be reused without modifying original records; the bridge itself grants no live authority.

#### Scenario: Bridge tampering or excess scope
- **WHEN** an unreferenced bridge, another substituted path, altered fixture/current/diff/receipt, missing reviewer, substantive diff or copied current hash is offered
- **THEN** prerequisite verification rejects and historical artifacts remain unchanged.

### Requirement: Existing I2 action/gates remain authoritative
The intended future registration is action `sp05-loan-execute-once`, requirement `SP05.3`, stage `i2`, owner `MC02`, exact live evidence profile and a capability whose reviewed history mapping includes retained loan execution. Existing `atomic-accept` and `rp01-mc02` prerequisites, stage campaign pointer, accepted semantic profile, actual current command and full runtime closure SHALL all match. The accepted semantic profile SHALL continue to derive from the hash-bound `moriarty-bounds/1` input, not be confused with the evidence profile. The old SP01 verifier and SP05 local utility actions SHALL not be repurposed as this live command.

Task 1.2 tests SHALL cover actual `load_snapshot`/CLI caller behavior with isolated stores and both source-positive and individually falsified negative controls, including unchanged authentic historical-design verification and stale historical SP01 action rejection. They SHALL retain the review04 constraints: canonical identity requires positive anchor match, worktree repository-key variance cannot produce zero failures, live all-kind primary/retry guards remain, and no production anchor or token is created by ordinary status/review/schema initialization. Concrete live registration and adoption are later reviewed operations.

#### Scenario: Source repair does not admit a live instance
- **WHEN** source tests pass but the actual executor, resource grant, canonical identity/history or pending-spend observations remain missing
- **THEN** implementation may be source-reviewed while the live action remains unavailable and no registry or master mutation occurs.
