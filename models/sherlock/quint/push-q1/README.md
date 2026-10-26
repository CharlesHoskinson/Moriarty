# U014–U016 suspension and continuation models

`python3 models/sherlock/quint/push-q1/verify.py` reproduces all checks with
Quint 0.32.0's default Rust evaluator for the continuation/admission models,
two evaluator threads and a 60-second per-command timeout. The epoch model
uses `--backend typescript` for exact UInt64 boundary literals. It writes `verification.log`, `verification-results.json`
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

## Imported epoch overflow

`resumeEpoch.qnt` models the unchecked `header.resume_epoch + 1` in both
`resume_pure` and `resume_fin`, separately from checkpoint/guard validation.
`Header::get` accepts every UInt64 value without a reachability check. For an
otherwise valid imported checkpoint whose epoch is `18446744073709551615`, a
true guard can cause a panic with overflow checks, or an accepted successor
whose epoch wraps to zero without them. The model explicitly represents both
Rust overflow modes; Quint arithmetic itself does not overflow. Six tests
exercise zero, max-minus-one, max, false guard and a candidate checked increment.

`sourceBehavior` is green and witnesses both failure modes. `safeHeader`
intentionally fails under the default `sourceAccept` step. The candidate
`--step checkedAccept` refuses an overflowing increment without changing the
checkpoint and satisfies `safeHeader` in sampled runs. This candidate is a
proposed remedy, **not a claim that the current design pins an overflow refusal
stage**. The accepted successor's `epoch = predecessor + 1` is specified in §6.2;
the current note does not define the maximum-epoch edge case. The source-order
trace `resumeEpoch-counterexample.itf.json` is not a Rust execution. The finding
was sent to `sherlock-fable` and `sherlock-u2`.

The epoch model requires `--backend typescript`: Quint 0.32.0's Rust evaluator
rejects literals outside the signed Int64 range with QNT600. The ordinary
continuation/admission models still run on the default Rust evaluator. The
verification runner records the backend on each command. All simulation runs
use depth 30; runs with `contractCheck`, `checkedAccept` or `scopedStep` explicitly select a
non-default step. The imported-input models finish after one API decision;
short traces there are intentional, rather than protocol deadlocks.

## Await lowering, post-resume SI-20 / ScopeF, and site numbering

`awaitLowering.qnt` projects `draft_source.rs`, `owner_suspend.rs` and
`owner_runtime.rs` across three U014–U016 lowering and continuation boundaries:

1. **Post-resume `SI-20` (`reg.controlOrigin`) violation, U016 technical-cut refusal at `InnerReturn`, and await binder env leak**:
   - `draft_source::Parser::await_expression` (`draft_source.rs:826–846`) lowers
     `await L(x: T) when G; E` to an `ExpressionRowC::Let` row (`site.expr`)
     wrapping continuation body `E` (`site.body != site.expr`).
   - `owner_suspend::await_enter` (`owner_suspend.rs:416`) intercepts
     `ControlC::Eval { expr: site.expr }` before `KMachineEvalLet`, so no
     `ScopeF` frame is pushed.
   - `owner_suspend::resume_accept` (`owner_suspend.rs:444–491`) extends
     `img.env_root` with the binder cell(s) and sets
     `img.control = ControlC::Eval { expr: site.body }` and
     `img.control_origin = site.body` without pushing
     `ScopeF { saved_env: caller_env, origin: site.expr }`.
   - When `site.body` returns (`ControlC::Return`) while `img.frames` is
     non-empty (`ContextReturnF` such as `InnerReturn` in `10 + inner(3)`, or
     `ContextRightF` in `10 + (await ...)`), `si20_return_ok`
     (`owner_runtime.rs:4311–4396`) checks whether `img.control_origin` is in
     the `If`-branch closure of `expectedChild(top) = site.expr`. Because
     `si20_return_ok` only closes across `ExpressionRowC::If` branches and not
     `ExpressionRowC::Let.body`, `si20_return_ok` returns `false` and
     `owner_check_step_inv` (`owner_runtime.rs:4898–4900`) fails with `SI20`.
   - Consequently, taking a U016 technical checkpoint (`resume_profile2_checkpoint`
     or `resume_technical_checkpoint`) at `InnerReturn` (`AtContextReturn` under
     `ContextReturnF`) is refused with `ResumeError::StepInvariant(["SI20 ..."])`.
   - Furthermore, when the enclosing frame does not restore `saved_env` on return
     (`ContextRightF`, `UnaryF`, or top-level `ContextTopEntry`), omitting `ScopeF`
     leaves the await binder cell in `img.env_root` after the await expression
     returns (`binderLeakedAfterReturn`). Pushing `ScopeF` on `resume_accept`
     (`--step scopedStep`) satisfies `SI-20`, accepts technical cuts at
     `InnerReturn`, and restores `CallerEnv` via `KMachineReturnScope`.
2. **Sequential `await` post-order site numbering reversal**:
   - In `draft_source::Parser::await_expression` (`draft_source.rs:823–854`),
     `let body = self.expression(0)?;` recursively parses `body` before
     `self.awaits.push(...)`, and `draft_source.rs:556–559` assigns 1-based
     `site` numbers in `self.awaits` push order.
   - In a sequential chain `await first(a: UInt) when G1; await second(b: UInt) when G2; E`,
     `second` receives `site = 1` and `first` receives `site = 2`
     (`reversedSequentialSites`), contradicting source-order site numbering
     (`draft_source.rs:29,217–220`; Design §3.2).
3. **Draft `/2` surface admission gaps**:
   - `draft_source.rs:681,809` rejects `yield L;` and zero-binder
     `await L() when G; E`, and `draft_source.rs:1050,1145` records ordinary
     `Expr::Let` locals with `let_sort = None` in `AwaitScopes::walk` and
     rejects them in `guard_type` (`surfaceRejectedYieldOrLetLocal`).

The 7 tests in `awaitLowering_test.qnt` cover `InnerReturn` `SI-20` violation
and technical-cut refusal, `ReturnF` masking of `env_root` after pop, `RightF`
binder env leak, the `scopedStep` remedy under both `ReturnF` and `RightF`,
sequential site reversal, and surface `/2` admission. `awaitLowering-counterexample.itf.json`
records the 4-state `AtAwaitEval -> AtWaiting -> AtBodyEval -> AtContextReturn`
trace violating `si20AndCutSafety`.

## Correspondence and limits

| Contract | Code/spec reference | Model |
| --- | --- | --- |
| U014 complete pending callee | suspension-and-finance §§3.4, 6.1–6.2; owner_suspend; complete HostSnapshot | Image, restore, waitingCanResume |
| U015 atomic resume and retained caller | suspension2::resume_pure; owner_suspend::resume_accept; design §5 | accept, notReady, atomicAccept, continuationResult |
| U016 transparent technical cut | local owner_resume::resume_technical_checkpoint; sherlock_technical_chunks_positive | cut, checkpointTransparency, cutBeforeAddition |
| Imported Waiting SI29 | design §4 R0 versus owner_runtime / resume_pure order | resumeAdmission source/contract actions |
| Imported epoch overflow | Header::get; suspension2::resume_pure; suspension2_fin::resume_fin | resumeEpoch sourceAccept / checkedAccept |
| Await lowering, SI-20 & ScopeF | draft_source::await_expression; owner_suspend::resume_accept; owner_runtime::si20_return_ok | awaitLowering sourceStep / scopedStep |

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
The continuation/admission checks use the default Rust evaluator; the epoch
checks use TypeScript. Neither evaluator is patched.
