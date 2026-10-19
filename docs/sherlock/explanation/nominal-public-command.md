# Connecting nominal source to the public compiler

The nominal parser and admitted compiler previously required separate Rust
calls. This candidate joins them to the existing `mori compile` selected
handler through an explicit nominal source selector. The old Core and draft
configuration contracts remain distinct. Nominal input supplies the additional
owner capacities and exactly the parser extents actually consumed.

The command serializes the real admitted program with the production serializer.
It retains recursive references, generic declarations, ordered constructor
fields and all program tables for the later interpreter/witness/native consumer.
A target descriptor and a source digest establish neither native support nor
semantic correspondence. The existing missing witness/constraint plans and
unsupported export remain visible rather than becoming a literal-only bypass.

The output states that generic expression body typechecking has not occurred.
The bounded parser checks its grammar and explicit extents; owner admission
performs its existing AD-0a/AD-1/AD-2/AD-3/AD-6 admission preconditions,
Pass B finance-syntax refusal, B/T declaration, C/P expression/MatchPlan and
V table checks, grouped under `admit` errors. Genesis refusal remains
`admit-run`. These operations do not supply the missing whole-language typing
or compiler/consumer correspondence proof.

Implementation is source-only and every new subject check is unrun. K, Quint,
Lean/Clean and handwritten circuits are unchanged; their generic compiler
correspondence and target obligations remain separate. No original requirement,
B04 or release exit is closed. The
[command reference](../reference/nominal-compile-command.md) is prospective.
Existing tutorials and executed how-to examples are unchanged
because no new observed public journey exists.
