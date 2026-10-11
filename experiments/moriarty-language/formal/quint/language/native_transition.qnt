// Local proof verification QA. Hash collision resistance, source-to-constraint
// correspondence and real native cryptography are explicit external premises.
// No ledger acceptance, signature authority, history or liability support.
module native_transition {
  type Claim = { source: str, program: str, target: str, scenario: str, execution: str, actionId: str, circuitK: int }
  type Candidate = {
    metadata: Claim, proofClaim: Claim, relation: str, nativeValid: bool,
    pinnedParameters: bool, canonicalProof: bool, liabilities: int, cap: int,
  }
  type State = { accepts: int, rejects: int, receipts: List[Claim], safe: bool }
  var state: State
  pure val expected: Claim = {
    source: "source-bytes-A", program: "program-A", target: "target-A", scenario: "scenario-A",
    execution: "complete-local-frame-A", actionId: "pay", circuitK: 10,
  }
  // Relation/key identity derives from recompiled source and target.
  // Bundle-supplied verifier keys are outside the admitted schema.
  pure val expectedRelation = "derived-relation-A"
  pure val good: Candidate = {
    metadata: expected, proofClaim: expected, relation: expectedRelation,
    nativeValid: true, pinnedParameters: true, canonicalProof: true, liabilities: 0, cap: 10,
  }
  pure def canVerify(c: Candidate): bool = and {
    c.metadata == expected, c.proofClaim == expected, c.relation == expectedRelation,
    c.nativeValid, c.pinnedParameters, c.canonicalProof, c.liabilities == 0,
    c.cap >= 1, c.cap <= 19, expected.circuitK <= c.cap,
  }
  action init = state' = { accepts: 0, rejects: 0, receipts: List(), safe: true }
  action verify(c: Candidate): bool = all {
    canVerify(c), state' = { ...state, accepts: state.accepts + 1,
      receipts: state.receipts.append(c.metadata), safe: state.safe and c.proofClaim == expected },
  }
  action refuse(c: Candidate): bool = all { not(canVerify(c)), state' = { ...state, rejects: state.rejects + 1 } }
  pure val badClaims = List(
    { ...expected, source: "changed-whitespace" }, { ...expected, program: "other-program" },
    { ...expected, target: "other-anchor" }, { ...expected, scenario: "other-untouched-balance" },
    { ...expected, execution: "other-fee-or-ordered-effect" }, { ...expected, actionId: "other-action" },
    { ...expected, circuitK: 9 }
  )
  action step = {
    nondet kind = 0.to(5).oneOf()
    nondet index = 0.to(6).oneOf()
    val c = if (kind == 0) good
      else if (kind == 1) { ...good, metadata: badClaims.nth(index) }
      else if (kind == 2) { ...good, proofClaim: badClaims.nth(index) }
      else if (kind == 3) { ...good, relation: "attacker-key" }
      else if (kind == 4) { ...good, nativeValid: false }
      else { ...good, liabilities: 1 }
    any { verify(c), refuse(c) }
  }
  val safety = state.safe and state.accepts == state.receipts.length()
    and state.receipts.foldl(true, (ok, c) => ok and c == expected)
  val verifiedWitness = state.accepts > 0
  val refusedWitness = state.rejects > 0
}
