# Independent pilot compilation view plan review

**Spec verdict: PASS. Quality verdict: PASS. No remaining planning blocker.**

Reviewed plan: `docs/superpowers/plans/2026-09-06-candidate-a-pilot-compilation-view.md`

Plan SHA256: `0621f1e3bcf002ec7d59f8ae90c38c0572a3c849dbc1a88bc3fd85c03954ab54`

Scope: independent source/command/evidence review of this six-stage pilot plan.
The view remains specified-only at review. No compiler, Quint test, simulation,
checker or solver was invoked. This is not Council or evidence admission.

Repository observation: independent read-only import traversal confirms exactly
the enumerated 24 modules from both pilot roots and the prefix-test root. All
relative imports stay inside that closure. `consumption.qnt:5` is the sole
`AuthorityKey` definition; neither adapter supplies a competing local binding.
The two single-name imports point to the same copied consumption definition,
already in the closure, and add no new dependency version. The view preserves
all module names and existing routes: the factored pilot and joint module still
import the original adapter, while factored boundary/swap/fixtures use the
factored adapter. Both adapter copies require the proposed correction.

Repository observation: `prepare.py` checks the exact closure, baseline pins,
24 copied files, byte equality after deleting the two import lines, and exact
relative destination lists. `record.py` preserves original typecheck, quantified
22-prefix test, two 100-sample/eight-witness runs, and sequential paired compile
gates with their existing budgets. Both complete Python blocks parse as valid
Python. Static reconstruction of the unchanged helper's source set confirms
the old compile receipt contains exactly its 69 automatic repository source
pins. Filtering those pins out of `extra` therefore removes duplicate archive
entries without removing separately required repository evidence.

Inference: these checks establish a narrow, reviewable source/reference identity
predicate. They do not prove compiler correctness or guarantee that this remedy
is sufficient for the full pilot. In particular, the actual key's Domain and
Principal structure is richer than the toy control. Different alias failures,
timeouts, unexpected diagnostics or failed size predicates correctly stop the
series without a further correction or automatic retry.

Repository observation: actual dispatch and historical runtime bootstrap remain
distinct; complete view/manifest/recorder/plan pins, baseline receipts, predecessor
stage originals, raw child results and outer terminal records are retained.
Compilation success and a failed size gate remain separate outcomes. Generated
declaration presence checks are explicitly preliminary: root must inspect full
binding expressions, pilot state/history/guards and paired size measurements
before a separately reviewed solver gate.

Resolved review findings:

- The original handoff prose promised output on every failure although missing
  pins or unreadable archives can terminate its command. Final CV004 explicitly
  requires preserving that actual failure and partial files, identifying absent
  reports/indexes, and transferring inventory ownership to root. No fabricated
  outputs, handoff rerun or native rerun is authorized.
- Final constraints enumerate all nine fresh cache paths: six native stages
  and three preparation/freeze/handoff paths.

Root's concrete script/view review and exclusive freeze still precede execution.
Original models, existing evidence and runtime stay immutable. This review
authorizes no actual-model modification, native evidence admission, H1 claim,
full A5 completion, or Council conclusion.
