## ADDED Requirements

### Requirement: EX001 complete staging inventory

WHEN capture admission is supplied, the exporter SHALL validate all 78 cases and
1557 events with exact source/raw/receipt inventories and SHALL emit staging-only
output. The producer SHALL not grant its own final package admission.

#### Scenario: Missing native shard

- **WHEN** one required raw shard is missing
- **THEN** export fails before any complete success record

### Requirement: EX002 exact event identity and provenance

WHEN state k is read, the exporter SHALL require exact native metadata, the fixed
global ordinal, cursor and sequence k, and the original ordered selector shape.
It SHALL bind before_index to max(0,k-1) and after_index to k.

#### Scenario: Incorrect first swap ordinal

- **WHEN** shard032 uses ordinal31 even with matching file hashes
- **THEN** the structural export rejects the driver counter

### Requirement: EX003 bounded streaming and retained parser evidence

WHILE processing shards, the exporter SHALL retain only its previous/current raw
state window and current event, hash in chunks and write incrementally. Each
invocation SHALL parse the complete pinned aggregate once, under explicit
900-second and 4096-MiB Node heap bounds, and retain original parser evidence in
the exclusive directory supplied by `--parser-receipts`.

#### Scenario: Parser execution and original evidence

- **WHEN** the parser terminates, fails, times out or produces oversized output
- **THEN** its original arguments, stdout, stderr and terminal record remain
- **AND** parser evidence is not inserted into semantic source pins or a self-hashing admission

### Requirement: EX004 complete stream termination

WHEN the states array appears complete, the exporter SHALL exhaust and validate
strict document EOF, including late metadata, duplicate fields and trailing data.

#### Scenario: Trailing data after valid states

- **WHEN** valid states are followed by trailing JSON
- **THEN** export fails and any new partial case file remains unadmitted

### Requirement: EX005 complete computation and cancellation records

WHEN a retained call is cancellation, computations SHALL be empty and unchanged
predecessor, NoInput, empty effects and NoCoreProjection SHALL be retained.
Agreement events SHALL retain every actual tagged evaluation and extraction in
the exact request order. Structural validation SHALL not replace semantic replay.

#### Scenario: Omitted or reordered agreement computation

- **WHEN** a required computation is omitted or distinct requests are reordered
- **THEN** the structural validator rejects the record

### Requirement: EX006 independent sealing and admission stability

WHEN full root admission is supplied, sealing SHALL revalidate exact
source/raw/receipt/case pins and all linked events without rewriting case files.
The final validated admission SHALL canonically equal the original admission.

#### Scenario: Coherent admission replacement

- **WHEN** admission and referenced files are coherently replaced during export
- **THEN** original-versus-final admission comparison rejects the change

#### Scenario: Real event omitted from copied case

- **WHEN** one real event is omitted from a separate incremental copy of case000
- **THEN** case EOF count validation rejects it
- **AND** the original admitted case hash remains unchanged

### Requirement: EX007 fail-closed exclusive outputs

WHEN an existing target, missing source, changed runtime, unknown IR/domain,
resource limit or receipt mismatch is encountered, the exporter SHALL return
nonzero without replacing existing bytes. Ordinary manifests SHALL be bounded
at 16 MiB and individual native receipts at 64 MiB.

#### Scenario: Existing output or parser receipt directory

- **WHEN** the requested target already exists
- **THEN** exclusive creation fails without rewriting its bytes

#### Scenario: Resource failure

- **WHEN** a resource bound is exceeded
- **THEN** the failure is recorded as operational evidence, not a semantic mutant kill

Experimental plan `e221eaf` supplies complete implementation and test steps.
Unit tests precede native intake; all four actual-package tests and the final
no-exclusion module remain mandatory. Structural specification validation is
not evidence of exporter execution or complete A4 acceptance.
