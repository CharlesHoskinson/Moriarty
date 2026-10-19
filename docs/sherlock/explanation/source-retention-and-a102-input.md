# Why the checked source is retained beside the B pair

The existing draft compiler already builds an A-shaped Arc<Program>, checks it,
and copies the same syntax into the actual B owner. Its public report exposed
the prepared B tables/genesis but discarded that source Arc. Retaining the actual
pair and syntax together lets an explicit A reference consumer use the same checked
source without reverse-decoding B tables or pretending a B microstate is an A state.

The report is private behind an immutable accessor so callers cannot replace it
with a report from another source while keeping the original pair. Both public
compiler functions share the same private preparation/projection implementation;
this prevents budget/error/digest paths from drifting. The new getter introduces
no new guard policy or admitted A capability. A callers' ability to construct a
public Program remains separate from its suitability for unmetered host diagnostics.

The finite LiteralSeven observation joins real source compilation, B genesis
content, the same retained Arc, and an actual A102 input producer. It compares every
padded input lane against values specified before execution by an implementation-
exposed author. Independent Opus source review re-derived the literal/table shape
and retained uncertainty about hash-derived lanes until the actual consumer run.
Passing that check demonstrates agreement for this input; it does not turn a hash,
a matched golden or a review into semantic correspondence or source authentication.

B preparation uses fuel64/stack8. The chosen A transition uses fuel1/stack0 and stops
at Return(Constant0), Pending. It neither steps B nor performs A80 free publication.
A102 and the composed A102-to-publication relation are distinct carriers. Schema/
relation/context IDs bind exact W3 source bytes, including its parent module; old
keys and proof receipts cannot be borrowed from another source cut.

[The source/results package](../../results/sherlock-real-source-a102-input-20261019/README.md)
retains raw objections, source-author output, final nonauthor opinion, qualifications,
commands/warnings and actual results. Native proofs, ledger/network selection and
settlement, A/B refinement, full StepInv/G4/projectA/GAP1/F4/H1/general39 and original
Sherlock acceptance remain separate work. No requirement status is changed here.
