# Complete nominal Quint ITF wire adapter

Source-only proposal. The adapter and its two synthetic normal serialization
tests have not run. No model, provider, kernel, compiler, Git or workspace
mutation is performed by this tool. Root owns source review, execution and
actual independent comparisons.

The existing frozen K-A ITF adapter renders K syntax and erases some source table
identities. This separate module projects the nominal profile directly to the
independent declaration-only wire. It imports only the existing ITF decoder and
canonical JSON byte helper, not any model or evaluator. It does not read oracle
materializers, expected cells, derivations or private freeze files.

All17 Snapshot fields are retained. Every expression/function/layout row,
integer identity, typed variant payload, pending constructor field, evaluated
prefix, remaining expressions, saved environment, eventual return frame,
source PC, outcome, clock, allocation counter and observation log is structural
data. Environments and program tables use the declared numeric entry ordering;
program ExprIds are never expanded or evaluated. Immutable source occurrence
and recursive group metadata come only from safe input declarations.

Raw ITF and its canonical hash remain in each envelope. The CLI also retains the
exact supplied ITF file hash and safe-input file hash. The comparison prefix
removes only exact repetitions after a terminal outcome; raw ordinal mapping
and all removed stutter ordinals remain explicit. Pending repeats and changed
terminal states stay in the comparison prefix. This is an observed structural
normalization, not an enabledness test or proof that no further transition exists.

The manifest supplies the declaration-only terminalStepContract string so Root
can bind the exact common profile metadata without editing states. The tool
returns actual observations under `observation`; it never creates or updates
expected cells, automatically approves comparisons, or infers producer provenance
from successful decoding.

Two tests use synthetic normal ITF encoding examples to check every field,
preorder values, tuple-key layouts, numeric environment ordering, nested saved
environments/frames, pending-repeat retention and explicit terminal stutter
ordinals. They do not execute a model or assert that their invented before/after
carriers form a valid semantic edge. No hostile/security cases are authored here.

Before any correspondence claim, Root must separately review and execute this
adapter against actual selected nominal captures, then compare complete
`observation` objects against the separately frozen non-author oracle. Source
and metadata hashes establish identity only. Independent expected cells remain
withheld from the source author; no comparison result is claimed by this batch.

Proposed manifest shape:

```json
{
  "terminalStepContract": "exact declared common profile metadata",
  "runs": [
    {"caseId": "input case ID", "itf": "actual-capture.itf.json", "safeInput": "SAFE-COMMON-INPUTS.json"}
  ]
}
```

Existing dependency: models/sherlock/qa/quint_observations.py. Root must include
that file in any selected snapshot closure and add the new tool/tests to existing
DoctorWatson membership only if integrating them. No shared runner is changed
in this source lane.
