# Candidate A export and model-checking intake

Classification: repository observations and preliminary hypotheses, 2026-09-05.
This is not an adopted A4/A5 implementation plan or acceptance result.
Frozen semantic source: d14cfea98a1c9213e5ef5f12c1a088f4e966083d.
Lifecycle plans:82d2c0b1d35ecf2f954603e4b8d54b248bda2663.
A2/A3 implementation and review remain dependencies of full acceptance.

## A4: root source inspection

Root read the complete existing exporter and correspondence checker. Preserve
their schema1 complete-inventory route and tests. A versioned authority export
must address these observed gaps rather than claim existing correspondence is
already integrated:

- scripts/export_s02_candidate_a_cases.py recognizes only swapTrace,
  installmentTrace and diagnosticCases. It expects one ITF variable and old
  trace carrier fields; new authority harness state is not that format.
- scripts/check_s02_candidate_a_correspondence.py independently decodes the
  complete sixteen-node programs and six-account/five-choice states, executes
  pinned Python Core, compares all result/projection fields and exact effect
  order, and validates raw provenance. It does not check signing/nonce/parent
  histories. Existing Python Core cannot establish those properties.
- The old complete-inventory check covers every record in the declared raw
  input set. A4 additionally needs a separately frozen mandatory scenario/profile
  inventory, so removing a whole mandatory trace and its binding cannot hide it.
  Keep rejection/negative cases as well as successful terminal cases.
- candidate_a_authority_adapter.qnt computes actual raw Core results but
  retains CoreProjected in its observation. New exports must capture the actual
  raw result at execution, not manufacture it from literal expected fixtures.
  Extra observation instrumentation requires explicit source/trace binding and
  independent review; no adapter or common change is authorized by this note.
- Cancellation has NoCoreProjection and identity candidate/empty effects.
  Check it against actual policy, attempt and consumption history separately.
  A rejected workload ending is not necessarily financial termination.

Proposed ownership split for later review: structural exporter owner distinct
from independent Python agreement/authority checker owner. Freeze schema,
enumerated mandatory inventory and concrete tests before edits. No allow-subset
acceptance. Archive baseline/mutant/report triples required by XML A4.
No A4 implementation or trace-export command was run during this intake.

## A5: prior failure and independent research

The existing offline manifest records a93,413,860-byte flattened input and two
terminal InlinePass heap failures at4/8GiB. Neither run checked states.
Generated input was retained only at a local temporary path, not archived.
The committed logs and hashes preserve that limit. No further heap-only retry.

Native nonauthor a0_final_review inspected actual source and proposed:

1. Compute raw transaction/pre-reduction/effects together to avoid duplicate
   evaluator expansion across candidate_a_harness and candidate_a_projection.
   Preserve the public extractor's complete supplied-result check, diagnostic
   distinctions and deposit position between pre/post payment effects.
2. Replace whole-history semantic replay with an incremental recurrence only
   after establishing equality with the original traceSafety, including final
   reconstructed state and terminal/enabled behavior. Retain complete history;
   a stored trusted valid flag alone is not equivalent.
3. If sharing disappears during inlining, expose bounded evaluator microsteps
   behind one atomic publication. Keep all authority/financial state unchanged
   during internal steps; preserve the projected original choices and adverse
   interleavings. This carries larger preservation obligations.

Measure generated expression size before asserting improvement. Local val
bindings alone may still inline repeatedly. Start with an actual two-profile
Alice funding slice and rebound forged-successor rejection, then expand to all
mandatory bounded workloads. A slice cannot discharge full A5.

Compare original/factored results over the complete existing semantic corpus and
mutation controls, adapter/boundary tests, and final A2/A3 inventories. Include
whole program, accounts, optional choices, request/time, raw errors/warnings/
reductions, ordered effects, every attempt/evidence, signing/registry/parent
state and terminal behavior. History/microstep changes additionally need an
explicit state/trace relation; corpus agreement alone is not general equivalence.

Use isolated experimental modules, leaving originals frozen. Before checking,
declare domains, properties, fairness, depth, deadlock/terminal treatment and
resource budget. Reuse direct offline compound .qnt.json input without a listener.
Require actual terminal results; OOM/timeout is an unresolved obligation, not an
architecture counterexample. No factoring was implemented or checker launched.
