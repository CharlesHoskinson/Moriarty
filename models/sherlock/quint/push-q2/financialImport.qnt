// Fin-route R0/R3-R5 import and host charge (sherlock2.rs 907e5fc7,
// suspension2_fin.rs 2852e423), scoped to work counters, resume_epoch and
// R0-vs-R3 refusal ordering. All program/plan/policy/site/capture/snapshot
// checks are fixed successful premises, and the guard is fixed true after R3.
// Public canonical envelope/JSON is not a validity proof of counters.
// Requires TS backend.
module financialImport {
  pure val MAX = 340282366920938463463374607431768211455
  type Phase = Supplied | Accepted | RefusedR0 | RefusedR3 | Charged | ArithmeticOverflow
  type State = {
    initial: int,
    remaining: int,
    consumed: int,
    canonical: bool,
    epochMax: bool,
    obsValid: bool,
    phase: Phase,
  }
  var checkpoint: State
  pure def u128(n: int): bool = n >= 0 and n <= MAX
  pure def countersValid(s: State): bool = s.remaining + s.consumed == s.initial
  // suspension2_fin.rs:217-230 checks canonical FinImage2 and resume_epoch == u64::MAX
  // at R0, then runs admit_observation (R3-R5) at :272 BEFORE sherlock2.rs:460
  // checks remaining_work == 0 ("R0", SI-29). Neither checks remaining + consumed == initial.
  pure def outerR0Accepts(s: State): bool =
    s.canonical and not(s.epochMax) and u128(s.initial) and u128(s.remaining) and u128(s.consumed)
  action initFullCase(
    initial: int,
    remaining: int,
    consumed: int,
    canonical: bool,
    epochMax: bool,
    obsValid: bool
  ): bool =
    checkpoint' = {
      initial: initial,
      remaining: remaining,
      consumed: consumed,
      canonical: canonical,
      epochMax: epochMax,
      obsValid: obsValid,
      phase: Supplied,
    }
  action initCase(initial: int, remaining: int, consumed: int, canonical: bool): bool =
    initFullCase(initial, remaining, consumed, canonical, false, true)
  action init: bool = {
    nondet initial = Set(0, 1, MAX).oneOf()
    nondet remaining = Set(0, 1, MAX).oneOf()
    nondet consumed = Set(0, 1, MAX).oneOf()
    nondet canonical = Set(false, true).oneOf()
    nondet epochMax = Set(false, true).oneOf()
    nondet obsValid = Set(false, true).oneOf()
    initFullCase(initial, remaining, consumed, canonical, epochMax, obsValid)
  }
  action importStep: bool = all {
    checkpoint.phase == Supplied,
    checkpoint' = {
      ...checkpoint,
      phase: if (not(outerR0Accepts(checkpoint)))
        RefusedR0
      else if (not(checkpoint.obsValid))
        RefusedR3
      else if (checkpoint.remaining == 0)
        RefusedR0
      else
        Accepted,
    },
  }
  action chargeStep: bool = all {
    checkpoint.phase == Accepted, checkpoint.remaining > 0,
    // Debug builds fail at consumed+=1 when MAX. Recording this separately
    // avoids inventing a released successor for an overflowing host operation.
    checkpoint' = if (checkpoint.consumed == MAX)
      {...checkpoint, phase: ArithmeticOverflow}
    else {...checkpoint, remaining: checkpoint.remaining - 1,
      consumed: checkpoint.consumed + 1, phase: Charged},
  }
  action observeStep: bool = all {checkpoint.phase != Supplied, checkpoint' = checkpoint}
  action step: bool = any {importStep, chargeStep, observeStep}
  val imported = Set(Accepted, Charged, ArithmeticOverflow).contains(checkpoint.phase)
  val overflowReached = checkpoint.phase == ArithmeticOverflow
  val malformedAccepted = imported and not(countersValid(checkpoint))
  val epochMaxRefused = checkpoint.epochMax and checkpoint.phase == RefusedR0
  val r0LateAfterR3Observed =
    checkpoint.canonical and not(checkpoint.epochMax) and checkpoint.remaining == 0
      and not(checkpoint.obsValid) and checkpoint.phase == RefusedR3
  val currentCodeInv = and {
    u128(checkpoint.initial), u128(checkpoint.remaining), u128(checkpoint.consumed),
    imported implies (checkpoint.canonical and not(checkpoint.epochMax) and checkpoint.obsValid),
    (checkpoint.phase == Accepted) implies checkpoint.remaining > 0,
    (checkpoint.phase == RefusedR3) implies (outerR0Accepts(checkpoint) and not(checkpoint.obsValid)),
  }
  // Design SI-03 and §4 R0-first ordering requirements, intentionally false for current Rust.
  val acceptedWorkInv = imported implies countersValid(checkpoint)
  val r0FirstOrderingInv =
    (checkpoint.canonical and checkpoint.remaining == 0 and checkpoint.phase != Supplied)
      implies (checkpoint.phase == RefusedR0)
}
