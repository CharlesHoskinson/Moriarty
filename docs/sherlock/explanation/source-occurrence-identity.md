# Why identical expressions keep different source identities

Sherlock distinguishes a source location from the value computed there. In
`down(1) + down(1)`, the two calls have equal text and may return equal values,
but they occupy different child positions in the entry expression. Keeping
those locations distinct lets a trace identify which call actually ran.

A prepared program also retains its original function-table order. A recursive
call refers to that table; preparation does not copy the callee body once per
possible call or choose a recursion depth. The runtime follows the reference
when it reaches the call, subject to the selected execution bounds. Unused
functions remain part of the prepared program rather than disappearing from
its identity.

Three identities answer different questions:

- A source occurrence identifies an expression's owning entry or function and
  its ordered child path. Identical expressions at different paths are distinct.
- A prepared expression ID identifies a row in this program image. Its source
  occurrence supplies the connection back to the program.
- An expanded runtime step ID identifies a step in a particular run. Repeated
  execution of one recursive call site keeps its source occurrence while
  producing separate runtime steps.

The dedicated draft public API returns the unstepped preparation together with
its completed run. The caller can inspect the prepared tables and capture
`StepEventC` values through the sink passed to `run_draft_host`. Checking only
the final numeric answer would miss a compiler that merged two source sites,
reordered the function table, or lost an event's source mapping.

The original [UIREQ-027 contract](../../../openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md#requirement-uireq-027-indexed-program-image-provenance)
requires those distinctions. The [draft compiler reference](../reference/draft-source-compiler.md)
describes the supported source and runtime interfaces. This explanation states
the contract; acceptance of a concrete implementation requires its independently
specified tables and actual events. Source provenance alone establishes no
constraint, proof or network result.
