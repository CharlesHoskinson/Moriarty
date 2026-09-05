# Candidate A strict streaming JSON

## ADDED Requirements

These prospective EARS/OpenSpec requirements refine CT-007–CT-009. Exact code
plan: experimental commit22cdec6,
`docs/superpowers/plans/2026-09-05-candidate-a-streaming-json.md`,
SHA256`a75dbb9283027d2937e663653289f3761a3813f82603cd99532042b0de571d95`.
A shared transport parser does not make the semantic oracles shared; that
lexical-layer dependency must nevertheless be disclosed. Runtime acceptance is open.

### Requirement: ST001: Bounded selected array

When a caller selects a top-level array field, the reader SHALL yield each element independently, retaining no preceding element.

#### Scenario: ST001: Bounded selected array

**WHEN** the caller consumes the first item of a long stream, **THEN** the reader has not read the full document and no later item is required in memory.

### Requirement: ST002: Strict object fields

When duplicate top-level or nested object keys occur, the reader SHALL reject them, including keys whose JSON escapes decode to the same string.

#### Scenario: ST002: Strict object fields

**WHEN** `"x"` and `"\u0078"` occur in the same object, **THEN** duplicate-key failure occurs regardless of chunk boundaries.

### Requirement: ST003: Strict UTF-8 and JSON syntax

The reader SHALL reject malformed/truncated UTF-8, BOM-prefixed input, incomplete containers/strings, illegal controls, invalid escapes, invalid numbers, nonfinite values, trailing commas and trailing non-whitespace data.

#### Scenario: ST003: Strict UTF-8 and JSON syntax

**WHEN** invalid UTF-8 is split between chunks or trailing garbage follows the root object, **THEN** completion is not emitted.

### Requirement: ST004: Numeric type preservation

For valid JSON, integers SHALL remain Python int, decimal/exponent numbers float, and booleans bool; float overflow SHALL be rejected.

#### Scenario: ST004: Numeric type preservation

**WHEN** an item contains `0,1.0,true,1e2`, **THEN** decoded types are int,float,bool,float; leading-zero integers and Infinity/NaN are rejected. This utility does not apply authority-specific bigint or finite-domain rules.

### Requirement: ST005: Complete document judgment

The reader SHALL require exactly one selected field, whose value is an array, and emit its end event only after the entire root object and trailing whitespace are consumed. Other top-level fields may precede or follow the selected array and SHALL be yielded in source order.

#### Scenario: ST005: Complete document judgment

**WHEN** a duplicate field or malformed suffix occurs after valid array items, **THEN** an exception occurs and no end event indicates success.

### Requirement: ST006: Incremental output and hashing

The writer SHALL serialize array elements incrementally with allow_nan=False and UTF-8, preserve item ordering, reject duplicate/reserved top-level fields, and handle short binary writes. Hashing SHALL read fixed positive chunks and return SHA256 plus byte count.

#### Scenario: ST006: Incremental output and hashing

**WHEN** the sink accepts only three bytes per write, **THEN** the resulting document still parses identically and its digest equals the reference bytes.

### Requirement: ST007: Explicit resource limits

When a single value exceeds the configured character limit, the reader SHALL raise ResourceLimit rather than pretend the data is semantically invalid.

#### Scenario: ST007: Explicit resource limits

**WHEN** a small configured limit is exceeded, **THEN** ResourceLimit is distinguishable from ordinary JSON failure and cannot count as a semantic mutant kill. The default limit is64Mi characters per value, configurable by the caller; changing this limit never permits case omission.

### Requirement: ST008: Transport-only independence

The utility SHALL import only Python standard-library modules.

#### Scenario: ST008: Transport-only independence

**WHEN** its AST imports are inspected, **THEN** there are no `scripts`, `moriarty`, producer, checker or Quint imports. Both consumers SHALL perform their own schema/semantic judgments after decoding.

### Requirement: ST009: Partial work is not acceptance

A writer error may leave a partial staging stream; the caller SHALL not publish it as an admitted artifact. A reader consumer SHALL exhaust through the end event before declaring complete success.

#### Scenario: ST009: Partial work is not acceptance

**WHEN** a caller has observed one valid item followed by malformed JSON, **THEN** it cannot report a complete valid document.
