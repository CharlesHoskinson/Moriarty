module signed_transition_test {
  import signed_transition.* from "./signed_transition"
  action initialize(s: State): bool = state' = s
  run paymentTest = init.then(accept(request(state, payment))).expect(
    state.ledger.balances.get(("alice", 0)) == 28 and state.ledger.balances.get(("bob", 0)) == 80
    and state.ledger.balances.get(("treasury", 0)) == 2 and state.ledger.paid == 70
    and state.ledger.slots.get(0) == { present: true, amount: 30 }
    and state.ledger.slots.get(1) == { present: true, amount: 5 }
    and state.ledger.slots.get(2) == { present: true, amount: 0 }
    and state.effects.foldl(List(), (l, e) => l.append(e.operation.kind)) == List(Transfer, Fee, Discharge, StateWrite, Emit)
    and state.effects.foldl(List(), (l, e) => l.append(e.after)) == List(70, 2, 30, 70, 70)
    and state.effects.nth(2).before == 50 and state.effects.nth(3).before == 0
    and totals(state.source, state.ledger, state.policy, payment, 0) == { outgoing: 70, fees: 2, credit: 70, debit: 0, gross: 72, net: 68, closing: 35 }
    and acceptedPaymentWitness and safety)
  pure def caps(s: State, c: Caps): State = { ...s, authority: { ...s.authority, limits: s.authority.limits.set(0, c) } }
  pure val exact: Caps = { gross: 72, fee: 2, net: 68, duty: 35 }
  run exactCapsTest = initialize(caps(initial(1, 3), exact)).then(accept(request(state, payment))).expect(safety)
  run grossCapTest = initialize(caps(initial(1, 3), { ...exact, gross: 71 })).then(refuse(request(state, payment))).expect(state.last == GrossCap and safety)
  run feeCapTest = initialize(caps(initial(1, 3), { ...exact, fee: 1 })).then(refuse(request(state, payment))).expect(state.last == FeeCap and safety)
  run netFloorTest = initialize(caps(initial(1, 3), { ...exact, net: 69 })).then(refuse(request(state, payment))).expect(state.last == NetFloor and safety)
  run dutyCapTest = initialize(caps(initial(1, 3), { ...exact, duty: 34 })).then(refuse(request(state, payment))).expect(state.last == DutyCap and safety)
  run stateOnlyZeroFinanceTest = initialize(initial(0, 0))
    .then(accept(request(state, List(op(StateWrite, 9), op(Emit, 9)))))
    .expect(state.ledger.paid == 9 and state.source.assets.keys() == Set() and state.effects.length() == 2 and safety)
  run stateOnlyWithDutiesTest = init.then(accept(request(state, List(op(StateWrite, 9))))).expect(state.ledger.slots == state.previous.slots and safety)
  run selfTransferTest = init.then(accept(request(state, List({ ...op(Transfer, 10), recipient: "alice" }))))
    .expect(state.ledger.balances == state.previous.balances and selfTransferWitness and safety)
  run selfBeneficiaryTest = initialize({ ...initial(1, 0), policy: { ...policy, beneficiary: "alice" },
    authority: { ...initial(1, 0).authority, policy: { ...policy, beneficiary: "alice" } } })
    .then(accept(request(state, List({ ...op(Transfer, 10), recipient: "alice" }))))
    .expect(totals(state.source, state.ledger, state.policy, List({ ...op(Transfer, 10), recipient: "alice" }), 0).net == 0 and safety)
  run selfBeneficiaryFeeOnceTest = {
    val s = initial(1, 0)
    val p = { ...policy, beneficiary: "alice" }
    val t = totals(s.source, s.ledger, p, List({ ...op(Transfer, 10), recipient: "alice" }, op(Fee, 2)), 0)
    and { t.credit == 10, t.debit == 10, t.fees == 2, t.net == -2, not(checked(t)) }
  }
  run partialFullZeroDutyTest = init.then(accept(request(state, List(op(Discharge, 20)))))
    .expect(partialWitness and safety).then(accept(request(state, List(op(Discharge, 30)))))
    .expect(fullWitness and state.ledger.slots.get(0) == { present: true, amount: 0 } and safety)
    .then(accept(request(state, List(op(Discharge, 0)))))
    .expect(state.ledger.slots.get(1).amount == 5 and state.ledger.slots.get(2) == { present: true, amount: 0 } and safety)
  pure def absentZero(s: State): State = {
    val ledger = { ...s.ledger, slots: s.ledger.slots.set(2, absent) }
    { ...s, ledger: ledger, previous: ledger }
  }
  run absentDischargeZeroTest = initialize(absentZero(initial(1, 3))).then(refuse(request(state, List({ ...op(Discharge, 0), duty: 2 }))))
    .expect(state.last == Arithmetic and state.ledger.slots.get(2) == absent and safety)
  run absentAccrueZeroTest = initialize(absentZero(initial(1, 3))).then(accept(request(state, List({ ...op(Accrue, 0), duty: 2 }))))
    .expect(presentZeroWitness and safety)
  run missingSlotTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    opening: { ...state.ledger, slots: Map(0 -> state.ledger.slots.get(0), 1 -> state.ledger.slots.get(1)) } } })))
    .expect(state.last == OpeningMismatch and safety)
  run hiddenSlotTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    opening: { ...state.ledger, slots: state.ledger.slots.put(3, { present: true, amount: 1 }) } } })))
    .expect(state.last == OpeningMismatch and safety)
  run malformedAbsentTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    opening: { ...state.ledger, slots: state.ledger.slots.set(2, { present: false, amount: 1 }) } } })))
    .expect(state.last == OpeningMismatch and safety)
  run sourceMismatchTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    source: { ...state.source, identity: "other" } } }))).expect(state.last == SourceMismatch and safety)
  pure val domains = List({ ...domain, network: "other" }, { ...domain, anchor: "other" }, { ...domain, target: "other" }, { ...domain, context: "other" }, { ...domain, selectedAction: "other" })
  run domainsTest = init.then(5.reps(i => refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    authority: { ...state.authority, domain: domains.nth(i) } } }))))
    .expect(state.last == DomainMismatch and state.attempts == 5 and safety)
  pure val policies = List({ ...policy, owner: "other" }, { ...policy, beneficiary: "other" }, { ...policy, address: "other" })
  run independentPoliciesTest = init.then(3.reps(i => refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    authority: { ...state.authority, policy: policies.nth(i) } } }))))
    .expect(state.last == PolicyMismatch and state.attempts == 3 and safety)
  run authorityTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    authority: { ...state.authority, key: "other" } } })))
    .then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    authority: { ...state.authority, nonce: 2 } } })))
    .then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    authority: { ...state.authority, predecessor: "other" } } })))
    .expect(state.last == AuthorityMismatch and state.attempts == 3 and safety)
  run authorityCapsMismatchTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    authority: { ...state.authority, limits: Map(0 -> { ...exact, gross: 73 }) } } })))
    .expect(state.last == AuthorityMismatch and safety)
  run intervalEndpointsTest = init.then(accept({ ...request(state, List()), at: 10 })).then(accept({ ...request(state, List()), at: 20 })).expect(safety)
  run intervalOutsideTest = init.then(refuse({ ...request(state, List()), at: 9 })).then(refuse({ ...request(state, List()), at: 21 }))
    .then(refuse({ ...request(state, List()), at: -1 })).then(refuse({ ...request(state, List()), at: uintMax + 1 }))
    .expect(state.last == TimeMismatch and state.attempts == 4 and safety)
  run emptyIntervalTest = initialize({ ...initial(1, 3), authority: { ...initial(1, 3).authority, start: 20, end: 10 } })
    .then(refuse(request(state, List()))).expect(state.last == TimeMismatch and safety)
  run falseSignatureTest = init.then(refuse({ ...request(state, payment), signatureVerified: false })).expect(state.last == SignatureMismatch and safety)
  run wholeCandidateSignatureTest = init.then(refuse({ ...request(state, payment), statement: { ...request(state, payment).statement, operations: List() } }))
    .expect(state.last == SignatureMismatch and safety)
  run openingSignatureBindingTest = init.then(accept(request(state, List(op(StateWrite, 1)))))
    .then(refuse({ ...request(state, List()), signedStatement: request(initial(1, 3), List()).statement }))
    .expect(state.last == SignatureMismatch and safety)
  run hiddenPayerTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    source: { ...state.source, payers: Set("alice", "mallory") } } }))).expect(state.last == Admission and safety)
  run hiddenDebtorTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    source: { ...state.source, duties: state.source.duties.set(2, { ...duty(2), debtor: "mallory" }) } } })))
    .expect(state.last == Admission and safety)
  pure val assets = List({ ...asset(0), network: "other" }, { ...asset(0), issuer: "other" }, { ...asset(0), reference: 1 }, { ...asset(0), scale: 1 })
  run exactAssetOperationTest = init.then(4.reps(i => refuse(request(state, List({ ...op(Transfer, 1), asset: assets.nth(i) })))))
    .expect(state.last == Arithmetic and state.attempts == 4 and safety)
  run recipientAndPayerTest = init.then(refuse(request(state, List({ ...op(Transfer, 1), recipient: "unknown" }))))
    .then(refuse(request(state, List({ ...op(Fee, 1), from: "bob" }))))
    .expect(state.last == Arithmetic and safety)
  run underflowAtomicTest = init.then(refuse(request(state, List(op(StateWrite, 99), op(Discharge, 51)))))
    .expect(state.last == Arithmetic and state.ledger.paid == 0 and state.effects == List() and safety)
  pure def huge(s: State): State = {
    val ledger = { ...s.ledger, balances: s.ledger.balances.set(("alice", 0), uintMax).set(("bob", 0), 0), slots: s.ledger.slots.set(0, { present: true, amount: uintMax }) }
    { ...s, ledger: ledger, previous: ledger, authority: { ...s.authority, limits: Map(0 -> { gross: uintMax, fee: uintMax, net: 0, duty: uintMax }) } }
  }
  run exactUInt128MaximumTest = initialize(huge(initial(1, 1))).then(accept(request(state, List(op(Transfer, uintMax), op(Discharge, uintMax)))))
    .expect(state.ledger.balances.get(("bob", 0)) == uintMax and state.ledger.slots.get(0) == { present: true, amount: 0 } and safety)
  run dutyOverflowTest = initialize(huge(initial(1, 1))).then(refuse(request(state, List(op(Accrue, 1))))).expect(state.last == Arithmetic and safety)
  run amountOverflowTest = init.then(refuse(request(state, List(op(Transfer, uintMax + 1))))).then(refuse(request(state, List(op(Transfer, -1)))))
    .expect(state.last == Arithmetic and safety)
  run recipientOverflowTest = initialize({ ...huge(initial(1, 1)), ledger: { ...huge(initial(1, 1)).ledger, balances: huge(initial(1, 1)).ledger.balances.set(("bob", 0), uintMax) },
    previous: { ...huge(initial(1, 1)).ledger, balances: huge(initial(1, 1)).ledger.balances.set(("bob", 0), uintMax) } })
    .then(refuse(request(state, List(op(Transfer, 1))))).expect(state.last == Arithmetic and safety)
  run checkedSumsOverflowTest = initialize(huge(initial(1, 1))).then(refuse(request(state, List({ ...op(Transfer, uintMax), recipient: "alice" }, { ...op(Transfer, 1), recipient: "alice" }))))
    .expect(state.last == Arithmetic and safety)
  run closingDutySumOverflowTest = initialize({ ...huge(initial(1, 2)), ledger: { ...huge(initial(1, 2)).ledger, slots: huge(initial(1, 2)).ledger.slots.set(1, { present: true, amount: 1 }) },
    previous: { ...huge(initial(1, 2)).ledger, slots: huge(initial(1, 2)).ledger.slots.set(1, { present: true, amount: 1 }) } })
    .then(refuse(request(state, List()))).expect(state.last == Arithmetic and safety)
  run eachAccumulatorOverflowTest = {
    val t = { outgoing: 0, fees: 0, credit: 0, debit: 0, gross: 0, net: 0, closing: 0 }
    and { checked(t), not(checked({ ...t, outgoing: uintMax + 1 })), not(checked({ ...t, fees: uintMax + 1 })),
    not(checked({ ...t, credit: uintMax + 1 })), not(checked({ ...t, debit: uintMax + 1 })),
    not(checked({ ...t, gross: uintMax + 1 })), not(checked({ ...t, net: -1 })), not(checked({ ...t, closing: uintMax + 1 })) }
  }
  run sixteenAdmissionTest = initialize(initial(16, 16)).then(accept(request(state, List(op(StateWrite, 1)))))
    .expect(state.source.assets.keys().size() == 16 and state.ledger.slots.keys().size() == 16 and safety)
  run seventeenAssetsTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    source: { ...state.source, assets: initial(17, 3).source.assets } } }))).expect(state.last == Admission and safety)
  run seventeenDutiesTest = init.then(refuse(sign({ ...request(state, List()), statement: { ...request(state, List()).statement,
    source: { ...state.source, duties: initial(1, 17).source.duties } } }))).expect(state.last == Admission and safety)
  run sourceClosureTest = all {
    admitted(initial(0, 0).source, policy), admitted(initial(16, 16).source, policy),
    not(admitted(initial(17, 0).source, policy)), not(admitted(initial(1, 17).source, policy)),
    not(admitted({ ...initial(2, 0).source, assets: Map(0 -> asset(0), 1 -> asset(0)) }, policy)),
    not(admitted({ ...initial(1, 1).source, duties: Map(0 -> { ...duty(0), asset: asset(1) }) }, policy)),
  }
}
