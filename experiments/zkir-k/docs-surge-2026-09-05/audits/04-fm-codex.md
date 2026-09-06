The report could not be saved because the filesystem is read-only. Audit content follows.

# Audit of 04-values-and-encoding.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared constructors, encodings, decoding guards and strict decoding with `zkir-values.k`, `zkir-syntax.k` and `zkir-ext.k`.
- Checked Rust encoder and decoder implementations in both specified `_build` trees and midnight-circuits 7.2.4.
- Traced input loading, transcript consumption, public-input construction and output commitments through the VM and constraint rules.
- Verified all named unit checks and the recorded `42/42` result; independently recomputed foreign encodings of zero and one.
- Evaluated the actual error-classification function on relevant messages.
- Confirmed one top-level title, no em-dashes and no placeholders.
- Attempted the chapter’s unit command from the repository root. It exited before running tests because uv could not create a cache lock on the read-only filesystem. Tests were not reproduced during this audit.

## Findings

### F1 blocking Extension Bytes decoding hides a panic/error mismatch

Claim: Chapter lines 27 and 89 describe the extension’s byte representation and decoding as matching the crate, without identifying the `Bytes<32>` exception.

Evidence: Repository observation: `experiments/zkir-k/tools/zkir_kast.py:37` maps `Bytes<32>` to `Bytes32`; lines 73–75 select that mapping before extension-specific handling. `experiments/zkir-k/semantics/zkir-values.k:165` returns `decPanic` for malformed Bytes32 chunks, and line 236 preserves that result through strict decoding. Conversely, `repos/_build/midnight-zkir-2ffe2d1/zkir/src/ir_instructions/encode.rs:181` routes every Bytes length through `decode_bytes`; lines 156–157 return `None` for nonzero unused bytes, producing an ordinary error at line 230.

Inference: For raw elements `[0, 256]` and type `Bytes<32>`, K produces a panic while the extension crate returns an error. This follows directly from the source; execution was not reproduced.

Fix: State that K retains the base Bytes32 panic behavior for length 32, while the extension crate returns an ordinary decoding error. Add the discrepancy to Notes for maintainers and qualify the extension decoding description.

### F2 blocking Non-canonical input does not unconditionally establish a violated commitment

Claim: Chapter line 113 says “a non-canonical input encoding is accepted off-circuit and violated in circuit.”

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-values.k:238` rejects successful non-canonical decodings when strict decoding is enabled. For commitments, `experiments/zkir-k/semantics/zkir-constraints.k:106` returns `holds` when the computed hash equals the supplied commitment; lines 107–108 return `violated` only when those integers differ. The predicate is hash inequality, not merely unequal input encodings.

The recorded example at `evidence/zkir-k-divergence-tests-2026-09-05b.txt:10` establishes `commGate:violated` for `f05_noncanonical_foreign_limbs`, not for every non-canonical encoding.

Fix: Replace the conclusion with: “On the base surface, some non-canonical encodings decode successfully. With commitments enabled, the raw and re-encoded streams can produce different commitments; `commGate` returns `violated` when its computed hash differs from the supplied commitment. The recorded `f05_noncanonical_foreign_limbs` case exhibits this behavior.”

### F3 major Encoding-length guarantee needs a valid-value precondition

Claim: Chapter line 31 states that `encodeValue(Value)` returns a list whose length equals `encodedLen(typeOf(V))`.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-values.k:28` permits any `Bytes` payload syntactically. Line 137 returns `.List` when that payload is not 32 bytes. Thus `encodeValue(bytes32(.Bytes))` has length zero, while `typeOf` returns `bytes32()` at line 43 and `encodedLen(bytes32())` is two in `experiments/zkir-k/semantics/zkir-syntax.k:49`.

Fix: Restrict the length statement to valid runtime values. Explain that K’s payload sorts do not themselves enforce the table’s range and length invariants, and mention the malformed Bytes32 fallback.

### F4 minor Panic status is presented as a harness error class

Claim: The Harness class column at chapter line 128 says “status `panic` on both sides.”

Evidence: Repository observation: `experiments/zkir-k/tools/diff_test.py:141–145` compares error classes even when both statuses are `panic`. Lines 92–97 return a fallback class based on the first 40 message characters. Evaluating that function gives:

- K message: `other:assertion failed: Bytes32 low element us`
- Rust panic prefix: `other:thread 'main' (2444202) panicked at zkir`

The chapter acknowledges this limitation only in the removable maintainer note at line 165.

Fix: Put “distinct fallback classes; matching panic statuses alone do not yield agreement” in the table. Retain the limitation in the main harness discussion.

### F5 minor Maintainer note misstates the common brief

Claim: Chapter line 164 says the common brief expects the working checkout to be at `92e8bdd3`.

Evidence: Repository observation: `experiments/zkir-k/docs-surge-2026-09-05/briefs/COMMON.md:45–50` explicitly directs readers to the `_build` copies and warns that working checkouts may be at other commits. The sentence also introduces drafting context into the chapter.

Fix: Delete “Expected the checkout to be at 92e8bdd3 as the common brief states.” Retain the factual distinction between the working checkout and reference sources.

## Coverage

- Value sort, thirteen constructors, payloads, `typeOf`, `typeName`: covered; clarify invariant scope under F3.
- Exact encoding layouts, limb parameters and shifts: covered.
- Decoding guards, error/panic distinction and strict re-encoding: partly; correct F1.
- Typed inputs, memory, `<pi>` and output encoding: covered; qualify F2.
- Exact error messages and harness classification: partly; correct F4.
- Per-type cross-check table: covered; absence of dedicated unit checks is explicitly identified.