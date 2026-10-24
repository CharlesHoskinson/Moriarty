# Sherlock production codec API: source and metadata observation

This delivery exposes the existing host preparation and typed observation codec
through `mori_core::sherlock::layout_a` for trusted diagnostic use only. Actual
production Core and the unchanged external consumer passed metadata-only
typechecking without `cfg(test)`: 57 Core sources plus one consumer, all four
original source afterimages consumed, 0 errors and 7 + 1 warnings. Children exited
0 in 1.922 and 0.066 seconds; aggregate 1.991 seconds. Separate local DoctorWatson
CI exited 0 with 230 ordinary corpus/source regressions in 10.502 seconds.

The [result summary](result.json) distinguishes the exact metadata-compiled
source identities from this delivery. After those observations, the owner-authored
module documentation and serializer comment were corrected to state the contract
below and qualify the public typed-row decoder. Executable source is unchanged.
No compiler or CI rerun followed those documentation changes. The other two
source afterimages and the external consumer remain byte-identical.

The public APIs are `prepare_a`, `encode_observed_a`, `decode_observed_a` and
`observed_state_bytes_a`, with their row, state and located refusal types.
Preparation, codec, validation, capacity and refusal bodies are retained.
Historical observation, auxiliary and arithmetic helpers retain their effective
private test-only scope.

## Trusted diagnostic contract

The following is the exact owner-authored module contract, with Rust doc-comment
markers removed:

```text
SDK-free host L4 preparation and typed observation codec. Trusted diagnostic use only.
PreparedA remains a mutable diagnostic record, not an authenticated capability.
No byte parser, transition acceptance or native ABI; no Machine-state importer.
decode_observed_a is a public typed-row decoder under the trust preconditions.

Unchecked preconditions: every `&PreparedA` is the unmodified value returned by
`prepare_a`, and every decoded `StateBlock` came from `encode_observed_a` over a
trusted Snapshot with the same PreparedA and capacities. Otherwise calls may panic
(direct table indexing), run without practical bound, abort on native stack overflow
when decoded recursive values are dropped or compared, or return Ok with program,
name or expression data never bound to the ProgramBlock. Err is a located refusal,
not an exhaustive validator. Do not pass constructed, mutated or untrusted records.

Capacities are caller-trusted with no ceiling. prepare_a, encode_observed_a and
observed_state_bytes_a allocate and zero-fill every table to capacity; a successful
reservation does not guarantee the host can commit it. Encoder lookups are superlinear.

DecodeMode::Step accepts noncanonical in-step rows (Constant-space references,
shared subtrees, non-least IDs). Neither mode's Ok is canonical admission,
reachability, StepInv, transition acceptance, F4/G4 correctness or native acceptance.
A decoded Snapshot is not Machine-produced and cannot be loaded into a Machine.
```

No Machine-state importer; `decode_observed_a` is a public typed-row decoder under
the trust preconditions. The contract does not accept hand-built or untrusted
StateBlocks in either mode. This diagnostic seam does not close native
producer/StepInv/F4/G4 or general untrusted-language acceptance. Full language,
formal, financial, native and target obligations remain open. Separate security
review and Root's adoption decision are not established by this ordinary record.

## Observation limits

The unchanged [external example](../../../../crates/mori-core/examples/sherlock_codec.rs)
uses real APIs to prepare its own trusted input and request canonical roundtrips
and state bytes. Only its metadata was compiled. No example function, roundtrip
or codec runtime test was executed, no executable was linked, and this operation
did not build the cfg(test) graph. The CI count is not a Rust codec test count.
`--emit=metadata,dep-info` produced compiler metadata and dependency files.

The initial dep-info literal-path assertion rejected two own-tree
`../fragments` spellings. Lexical normalization matched the frozen source files;
raw dep-info and compiler results were retained, with no compiler rerun. Source,
compiler, six selected extern and two procedural-macro pins were unchanged on
that readback. Cached lexer, transitive dependency and procedural-macro source
correspondence remains unestablished; compiler/sysroot/dependency trust is not
proved by this capture.

Canonical observation encoding uses Data-space rows; equal decoded snapshots do
not establish equality with a native literal fragment's Constant-space in-step
rows. These bytes establish no native ABI or Step witness compatibility. No
SDK/native, circuit/proof, wallet/transaction, stagenet submission or settlement
readback was performed by this metadata operation. Existing limits and parent
acceptance obligations are unchanged.
