# Candidate B fourth-draft intake

Disposition: preserve as planning input, not adopted for implementation. This is
not a negative architecture finding or a justified stop for B; the concrete design
still needs completion. Root read the complete 221-line fourth draft.

Proposal SHA-256: b20a0345971dd4c0e8e528d9eacb1851855dffd9d7f747d5385567b8bc821181.
Archived at `evidence/s02-model-comparison/candidate-b-plan-intake/proposal-fourth-draft.md`.

Improvements: the draft names the empty-swap timeout separately, distinguishes
Core-error and native-diagnostic attempt statuses, adds now to discharge APIs,
states finite history/account bounds, preserves actual-balance refund templates,
and retains the later common-authority integration boundary. Bridge controls are
unresolved rather than silently not applicable.

Remaining concrete issues before implementation:

1. The purported literal total graph is prose, not exact per-node fields. It says
   unused obligations have no successor, but BObligation.successor is a required
   BContinuation. It relies on a phase-node set absent from BGraph's carrier.
   Define coherent carriers and every node's prerequisites, exclusion set,
   templates, phase, successor and reference tag.
2. The prose still uses undefined RejectedAttemptB(error) after replacing that
   constructor with RejectedCoreAttemptB and RejectedNativeAttemptB. The rejection
   API accepts only BError and lacks request/input/time; it cannot record the
   specified complete Core-rejection request without a corrected interface.
3. negativeMatchedDepositDiagnosticB is only named, not defined as a literal
   graph/workload. No BWorkload or graph table supplies its negative deposit.
   Successful positive templates and a rejecting negative fixture must be distinct.
4. The proposed single input-number domain omits choice value two, needed for
   the shared choice-bounds diagnostic. Use distinct finite choice and deposit
   domains and reconcile them with the global successful-value/time statement.
5. Rank-decreasing prerequisites and successor edges are not coherently defined:
   successor is currently a phase, not a node. Provide an explicit rank/table
   interpretation instead of asserting the DAG check is executable.
6. The complete raw observation and reduction/warning formulas for each native
   transition still need exact definitions. Transfer templates alone cannot
   establish frozen Core continuation, time, choice, warning or reduction fields.

The root should resolve these issues in a concrete type-and-table contract before
delegating B logic. Do not repeat a common-design Council vote or reinterpret
missing B implementation as evidence against that architecture. A, C, D and the
full XML work remain in scope; no model-checking or selection claim follows here.
