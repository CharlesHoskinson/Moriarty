// FIRST INCREMENT, UNEXECUTED: initialization + one successful prefix transition.
// Independent inputs: frozen ORACLE-CONTINUATIONS.json, escrow/reserve opening
// and prefix_waiting rows. No candidate compiler/evaluator import or invocation.
module continuation_prefix {
  type Specimen = Escrow | Reserve
  type NormalEffect =
    | Transfer(int)
    | StateChange({ before: int, after: int })
    | LiabilityChange({ before: int, after: int })

  // One invocation has one cohesive public financial boundary. Static names,
  // asset identity, capture types and source ordinals are in ADAPTER.md.
  type FinancialFrame = {
    specimen: Specimen,
    inputValue: int,
    balances: str -> int,
    programState: str -> int,
    dutyPresent: bool,
    dutyAmount: int,
    pendingSite: int,
    captureValues: List[int],
    logicalAttempted: int,
    logicalCommitted: int,
    effects: List[NormalEffect],
  }

  var boundary: FinancialFrame

  pure def initialFrame(specimen: Specimen): FinancialFrame = {
    specimen: specimen,
    inputValue: if (specimen == Escrow) 10 else 2,
    balances: if (specimen == Escrow)
      Map("alice" -> 100, "bob" -> 0, "escrow" -> 0, "observer" -> 42)
    else
      Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
    programState: if (specimen == Escrow)
      Map("marker" -> 77)
    else
      Map("allocated" -> 0, "basis" -> 40, "marker" -> 77),
    dutyPresent: specimen == Reserve,
    dutyAmount: if (specimen == Escrow) 0 else 5,
    pendingSite: 0,
    captureValues: List(),
    logicalAttempted: 0,
    logicalCommitted: 0,
    effects: List(),
  }

  pure def canStart(s: FinancialFrame): bool = s.pendingSite == 0

  pure def applyStart(s: FinancialFrame): FinancialFrame =
    if (s.specimen == Escrow) {
      ...s,
      balances: s.balances
        .set("alice", s.balances.get("alice") - s.inputValue)
        .set("escrow", s.balances.get("escrow") + s.inputValue),
      dutyPresent: true,
      dutyAmount: s.dutyAmount + s.inputValue,
      pendingSite: 1,
      captureValues: List(s.inputValue),
      logicalAttempted: s.logicalAttempted + 5,
      logicalCommitted: s.logicalCommitted + 5,
      effects: List(
        Transfer(s.inputValue),
        LiabilityChange({ before: s.dutyAmount, after: s.dutyAmount + s.inputValue })
      ),
    } else {
      // held is captured from the initial basis, before basis is decremented.
      val held = s.programState.get("basis") + s.inputValue
      {
        ...s,
        programState: s.programState.set("basis", s.programState.get("basis") - 5),
        dutyPresent: true,
        dutyAmount: s.dutyAmount + held,
        pendingSite: 1,
        captureValues: List(s.inputValue, held),
        logicalAttempted: s.logicalAttempted + 11,
        logicalCommitted: s.logicalCommitted + 11,
        effects: List(
          StateChange({ before: s.programState.get("basis"), after: s.programState.get("basis") - 5 }),
          LiabilityChange({ before: s.dutyAmount, after: s.dutyAmount + held })
        ),
      }
    }

  action initCase(specimen: Specimen): bool = boundary' = initialFrame(specimen)

  action init: bool = {
    nondet specimen = Set(Escrow, Reserve).oneOf()
    initCase(specimen)
  }

  action start: bool = all {
    canStart(boundary),
    boundary' = applyStart(boundary),
  }

  action step: bool = start

  // Positive reachability target, false at initialization. No invariant is
  // proposed in this nonsecurity first increment.
  val firstBoundaryReached: bool = boundary.pendingSite == 1
}
