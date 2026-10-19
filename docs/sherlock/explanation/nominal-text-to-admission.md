# Nominal text reaches the admitted compiler

This page originally described the frozen six-file API candidate. Its first
normal Core compilation failed at the 30-second wall bound; all dependent tests
and capture remain unrun. The [public command candidate](nominal-public-command.md)
adds source integration in a separate checkout and has no successful execution
result. The historical API source cut and its receipts remain separate.

The bounded nominal parser previously returned an owned source program or
type-preparation result, while `compile_ext_host` required a caller-built Rust
`ExtProgram`. The candidate `compile_nominal_host` connects those existing
operations. A caller can supply source text and explicit parser/compiler inputs
and receive the same admitted HostOnly artifact available from the typed API.

The parser preserves the source program. Generic declarations, constructor
fields and recursive calls enter the existing owner unchanged. Compilation
creates tables and an unstepped genesis image; it does not compute the entry
value or replace recursion with a literal. The new WrappedCountdown source
returns `Parcel<UInt>` from a recursive function, so its table test distinguishes
this connection from scalar-only preparation. The complete author table
expectation was written before the adapter and any execution; it is not an
independent semantic oracle.

The compiler and three focused tests are **UNRUN** at this source cut. They
cover complete admitted table contents and genesis/report identity, plus exact
existing parser/admission error payload propagation. Fresh full-candidate review
and inspected ordinary execution remain required. Earlier typed generic-C6
results retain their own source identities.

The API returns the existing typed artifact, which retains no source name or
span metadata. The separate public command candidate adds the explicit nominal selector,
caller inputs and production publication path. It still needs a successful
normal compilation, selected-handler tests and retained full-artifact comparisons.
Neither source candidate establishes observed CLI behavior.

No K, Quint, Lean/Clean or circuit semantics change here. Those layers still
need their stated compiler/consumer correspondence evidence for generic source.
Witness/constraint/export plans, native proof and Midnight target qualification
retain their separate missing or unobserved status. Tutorial and CLI how-to
examples stay unchanged because this candidate has no observed CLI journey.
