// Fin-route R0 import and host charge, scoped to the work-counter premise.
// All program/plan/policy/site/capture/snapshot checks are fixed successful
// premises. Public canonical envelope/JSON is not a validity proof of counters.
// No forged on-chain proof or Rust execution is claimed. Requires TS backend.
module financialImport {
  pure val MAX = 340282366920938463463374607431768211455
  type Phase = Supplied | Accepted | Refused | Charged | ArithmeticOverflow
  type State = {initial: int, remaining: int, consumed: int, canonical: bool, phase: Phase}
  var checkpoint: State
  pure def u128(n: int): bool = n >= 0 and n <= MAX
  pure def countersValid(s: State): bool = s.remaining + s.consumed == s.initial
  // decode_body canonicalizes the FinImage2, then resume's R1/R2/R6/R7
  // inspect outcome/identity/logical-plan/snapshot, but not this counter triple.
  pure def decoderAccepts(s: State): bool =
    s.canonical and u128(s.initial) and u128(s.remaining) and u128(s.consumed)
  action initCase(initial: int, remaining: int, consumed: int, canonical: bool): bool =
    checkpoint' = {initial: initial, remaining: remaining, consumed: consumed,
      canonical: canonical, phase: Supplied}
  action init: bool = {
    nondet initial = Set(0, 1, MAX).oneOf()
    nondet remaining = Set(0, 1, MAX).oneOf()
    nondet consumed = Set(0, 1, MAX).oneOf()
    nondet canonical = Set(false, true).oneOf()
    initCase(initial, remaining, consumed, canonical)
  }
  action importStep: bool = all {
    checkpoint.phase == Supplied,
    checkpoint' = {...checkpoint, phase: if (decoderAccepts(checkpoint)) Accepted else Refused},
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
  val currentCodeInv = and {
    u128(checkpoint.initial), u128(checkpoint.remaining), u128(checkpoint.consumed),
    imported implies checkpoint.canonical,
    (checkpoint.phase == Refused) implies not(checkpoint.canonical),
  }
  // Design SI-03 requirement, intentionally false for this source port.
  val acceptedWorkInv = imported implies countersValid(checkpoint)
}
