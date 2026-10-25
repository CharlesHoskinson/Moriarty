# U014–U016 suspension and continuation models

`python3 models/sherlock/quint/push-q1/verify.py` reproduces all checks with
Quint 0.32.0's default Rust evaluator, two evaluator threads and a 60-second
per-command timeout. It writes `verification.log`, `verification-results.json`
(including source SHA-256 pins) and an expected counterexample ITF trace here.
Nothing outside `push-q1` is written. No Rust build, holdout, ledger or native
proof check is run by this package.

## Continuation model

`suspensionResume.qnt` models one pure machine at the await in
`10 + inner(3)`, with bottom-to-top `ReturnOuter`, `AddRight(10)`, `ReturnInner`
frames. Checkpoint fields retain control phase, lexical bindings, reachable
scalar data, capacities, work counters, guard captures, site, epoch,
observation identity and the empty financial prefix.

The exact /2 suspension costs are entry `1`, accept `1 + 5 = 6` for the guard
`v == n + 1`. Fuel exhaustion precedes DATA capacity exhaustion, which precedes
environment capacity exhaustion, matching `owner_suspend::await_enter`.
False/error observations change only the host report. Successful resume adds
one DATA row and binding, clears captures and retains all caller frames.
Return frames carry their saved lexical environments, and addition reads its
saved left operand from the retained frame. An uncut ghost execution stays equal to the execution crossing structural
archive boundaries. The model reaches return value 4 before the pending
addition and publishes 14 after it. Local repeated resume is deterministic;
consumption is untracked.

The suffix collapses owner microsteps into body evaluation, callee return,
addition, caller return and final publication. Its unit charges are **abstract**,
not estimates of Rust helper/reference/logical work. Only the entry and resume
charges are claimed exact. Final publication costs zero. The initial prefix
before the await is omitted. The three-cut limit bounds sampling, not the
implementation. A failed resume API call remains enabled in terminal states
because callers may still attempt that API; it preserves the terminal image.
Every major action has a witness; `cutBeforeAddition` records that the cut itself
occurred at that phase. The deterministic scenarios also force this boundary.

The 30 tests cover pending frames, successful return, false/error identity,
repeated attempts, wrong CK/program/site/label/layout, resource boundaries and
precedence, zero-fuel publication, and cuts. Lossy checkpoint mutations are
checked against independently specified frame/environment/work expectations.

## Imported-checkpoint disagreement

`resumeAdmission.qnt` separates the design's R0 behavior (`contractCheck`) from
the implementation's validation order (`sourceCheck`, the default step).
Its four tests record the disagreement and machine-produced-input agreement.
`acceptedIsAffordable` holds for the source projection. `matchesContract`
intentionally fails for the source projection and holds with
`--step contractCheck`.

The source projection is from the owning agent's `Moriarty-wt-factory-u` tree:

- `compile_artifact/suspension2.rs::resume_pure` rebuilds the image, then
  returns `not_ready` on a false/error guard before calling `resume_accept`.
- `shared/owner_resume.rs::resume_profile2_checkpoint` delegates to the
  /2 owner's step invariants. `shared/owner_runtime.rs` checks SI01–SI24 and
  Waiting/Await agreement (SI27), without the site-specific SI29 resource test.
- `shared/owner_suspend.rs::resume_accept` performs that resource test after
  the guard, returning an error for an unaffordable accept.
- Design §4 says R0 must reject a malformed checkpoint violating SI29 before
  guard evaluation.

Thus an imported Waiting checkpoint with insufficient remaining fuel, DATA
rows or environment cells returns `not_ready` if the guard is false, or a late
`run_failure` if it is true, rather than R0 refusal. The checked-in ITF gives a
source-order counterexample; it is **not an executed Rust exploit**. All other
admission, structure, capture, checksum and observation checks are assumed
successful in this small projection. Work conservation can remain satisfied
on such an input by moving fuel to consumed work and the reference cursor;
checksums are public and do not establish machine reachability. The entry rule
protects machine-produced Waiting checkpoints. FINDING messages were sent to
`sherlock-fable` and `sherlock-u2`.

## Correspondence and limits

| Contract | Code/spec reference | Model |
| --- | --- | --- |
| U014 complete pending callee | suspension-and-finance §§3.4, 6.1–6.2; owner_suspend; complete HostSnapshot | Image, restore, waitingCanResume |
| U015 atomic resume and retained caller | suspension2::resume_pure; owner_suspend::resume_accept; design §5 | accept, notReady, atomicAccept, continuationResult |
| U016 transparent technical cut | local owner_resume::resume_technical_checkpoint; sherlock_technical_chunks_positive | cut, checkpointTransparency, cutBeforeAddition |
| Imported Waiting SI29 | design §4 R0 versus owner_runtime / resume_pure order | resumeAdmission source/contract actions |

Codec bytes, lane widths, native C6 constraints, digest equality with the stock
verifier, authentication, on-chain single use, arbitrary programs, physical
renumbering, heap sharing and financial transitions are outside these models.
`restore` is a structural projection, not a proof of the Rust archive codec.
Observation identity is symbolic; the `value` stored as identity is adequate
only for this one-UInt, fixed-site example. Guard errors abstract a different
admitted arithmetic guard; the concrete small `n+1` guard cannot error.
Simulation is sampled, not exhaustive, and no Rust-to-Quint refinement is
claimed. Update the source correspondence and rerun before relying on these
models after a suspension, checkpoint or charging change.

During development two modeling issues were corrected: inserting a new binding
requires `Map.put`, and chaining two disabled `.fail()` actions loses the test
state in Quint. A separate tool failure was reduced to Quint's JSON reviver
converting a record field named `reference` to BigInt; the field is now `uncut`.
The final checks use the default Rust evaluator without a tool patch.
