module liability_transition_test {
  import liability_transition.* from "./liability_transition"
  run orderedProgressTest = init.then(accept(request(state, List(op(0, Accrue, 4, 0), op(0, Discharge, 3, 1), op(0, Accrue, 2, 2), op(0, Discharge, 5, 3)))))
    .expect(state.slots.get(0) == { presence: 1, amount: 8 } and state.effects.length() == 4
      and state.effects.foldl(List(), (l, e) => l.append(e.before.amount)) == List(10, 14, 11, 13)
      and state.effects.foldl(List(), (l, e) => l.append(e.after.amount)) == List(14, 11, 13, 8) and safety)

  action initialize(s: State): bool = state' = s
  run zeroLifecycleTest = init.then(initialize({ ...state, slots: state.slots.set(0, absent), previous: state.previous.set(0, absent) }))
    .then(accept(request(state, List(op(0, Accrue, 0, 0), op(0, Discharge, 0, 1), op(0, Accrue, 2, 2), op(0, Discharge, 2, 3)))))
    .expect(state.slots.get(0) == { presence: 1, amount: 0 } and state.effects.length() == 4
      and state.effects.foldl(List(), (l, e) => l.append(e.before.presence)) == List(0, 1, 1, 1)
      and state.effects.foldl(List(), (l, e) => l.append(e.after.amount)) == List(0, 0, 2, 0) and safety)
  run absentDischargeZeroTest = init.then(refuse(request(state, List(op(1, Discharge, 0, 0)))))
    .expect(state.refused == 1 and state.slots.get(1) == absent and state.effects == List() and safety)
  run fullDischargeTest = init.then(accept(request(state, List(op(0, Discharge, 10, 0)))))
    .expect(state.slots.get(0) == { presence: 1, amount: 0 } and fullWitness and safety)
  run presentZeroDischargeTest = init.then(accept(request(state, List(op(1, Accrue, 0, 0)))))
    .then(accept(request(state, List(op(1, Discharge, 0, 0)))))
    .expect(state.slots.get(1) == { presence: 1, amount: 0 } and safety)
  run partialWithUntouchedZeroTest = init.then(initialize({ ...state, slots: state.slots.set(1, { presence: 1, amount: 0 }), previous: state.previous.set(1, { presence: 1, amount: 0 }) }))
    .then(accept(request(state, List(op(0, Discharge, 7, 2)))))
    .expect(state.slots.get(0).amount == 3 and state.slots.get(1) == { presence: 1, amount: 0 } and partialWitness and safety)
  run underflowAtomicTest = init.then(accept(request(state, List(op(0, Accrue, 1, 0)))))
    .then(refuse(request(state, List(op(0, Discharge, 12, 0), op(0, Accrue, 12, 1)))))
    .expect(state.slots.get(0).amount == 11 and state.effects == state.previousEffects and safety)
  run maximumOverflowTest = init.then(initialize({ ...state, slots: state.slots.set(0, { presence: 1, amount: uintMax }), previous: state.previous.set(0, { presence: 1, amount: uintMax }) }))
    .then(refuse(request(state, List(op(0, Accrue, 1, 0), op(0, Discharge, 1, 1)))))
    .expect(state.slots.get(0).amount == uintMax and state.effects == List() and safety)
  run intermediateUnderflowTest = init.then(initialize({ ...state, slots: state.slots.set(0, { presence: 1, amount: 0 }), previous: state.previous.set(0, { presence: 1, amount: 0 }) }))
    .then(refuse(request(state, List(op(0, Discharge, 1, 0), op(0, Accrue, 1, 1)))))
    .expect(state.slots.get(0).amount == 0 and safety)
  run maximumLegalTest = init.then(initialize({ ...state, slots: state.slots.set(0, { presence: 1, amount: uintMax - 1 }), previous: state.previous.set(0, { presence: 1, amount: uintMax - 1 }) }))
    .then(accept(request(state, List(op(0, Accrue, 1, 0)))))
    .expect(state.slots.get(0).amount == uintMax and safety)
  pure val mutations = List(
    { ...identity(1), network: "other" }, { ...identity(1), issuer: "other" },
    { ...identity(1), reference: "other" }, { ...identity(1), scale: 1 },
    { ...identity(1), debtor: "other" }, { ...identity(1), creditor: "other" })
  run untouchedIdentitySubstitutionTest = init.then(6.reps(i => {
    val r = request(state, List())
    refuse({ ...r, frame: List({ identity: identity(0), slot: state.slots.get(0) }, { identity: mutations.nth(i), slot: state.slots.get(1) }) })
  })).expect(state.refused == 6 and state.accepted == 0 and safety)
  run sourceIdentitySubstitutionTest = init.then(6.reps(i => {
    val r = request(state, List())
    refuse({ ...r, declarations: List(identity(0), mutations.nth(i)) })
  })).expect(state.refused == 6 and safety)
  run operationIdentitySubstitutionTest = init.then(refuse(request(state, List({ ...op(0, Discharge, 1, 0), identity: { ...identity(0), debtor: "other" } }))))
    .expect(state.refused == 1 and safety)
  run hiddenDutyTest = init.then(refuse({ ...request(state, List()), frame: request(state, List()).frame.append({ identity: identity(2), slot: { presence: 1, amount: 1 } }) }))
    .expect(state.refused == 1 and safety)
  run duplicateScenarioTest = init.then(refuse({ ...request(state, List()), frame: request(state, List()).frame.append(request(state, List()).frame.head()) }))
    .expect(state.refused == 1 and safety)
  run missingFrameTest = init.then(refuse({ ...request(state, List()), frame: List({ identity: identity(0), slot: state.slots.get(0) }) }))
    .expect(state.refused == 1 and safety)
  pure val malformed = List({ presence: 0, amount: 1 }, { presence: 2, amount: 0 }, { presence: 1, amount: -1 }, { presence: 1, amount: uintMax + 1 })
  run malformedFrameTest = init.then(4.reps(i => refuse({ ...request(state, List()), frame: List({ identity: identity(0), slot: malformed.nth(i) }, { identity: identity(1), slot: state.slots.get(1) }) })))
    .expect(state.refused == 4 and safety)
  run identicalZeroEffectOrderTest = init.then(refuse({ ...request(state, List(op(0, Accrue, 0, 3), op(0, Accrue, 0, 5))), descriptorSteps: List(5, 3) }))
    .expect(state.refused == 1 and safety)
  run sourceStepOrderTest = init.then(refuse(request(state, List(op(0, Accrue, 0, 5), op(0, Accrue, 0, 3)))))
    .expect(state.refused == 1 and safety)
  run duplicateSourceAndCapTest = all {
    sourceAdmitted(List()), sourceAdmitted(List(identity(0), identity(0))),
    not(sourceAdmitted(List(identity(1), mutations.nth(0)))),
    sourceAdmitted(0.to(15).fold(List(), (l, n) => l.append(identity(n)))),
    not(sourceAdmitted(0.to(16).fold(List(), (l, n) => l.append(identity(n))))),
  }
  run sixteenOccupiedZerosTest = initialize({ ...initial(16), slots: 0.to(15).mapBy(_ => { presence: 1, amount: 0 }), previous: 0.to(15).mapBy(_ => { presence: 1, amount: 0 }) })
    .then(accept(request(state, List(op(0, Discharge, 0, 0)))))
    .then(refuse({ ...request(state, List()), declarations: request(state, List()).declarations.append(identity(16)) }))
    .expect(state.registry.keys().size() == 16 and state.slots.keys().forall(n => state.slots.get(n).presence == 1) and safety)
  run noDutyOperationPreservesFrameTest = init.then(accept(request(state, List())))
    .expect(state.slots == state.previous and state.effects == List() and safety)
  run zeroCreationWitnessTest = init.then(accept(request(state, List(op(1, Accrue, 0, 0)))))
    .expect(zeroCreationWitness and safety)
  run emptyRegistryStageTest = initialize(initial(0)).then(accept(request(state, List())))
    .expect(state.registry.keys() == Set() and state.slots.keys() == Set() and state.accepted == 1 and safety)
  run invalidDeltaTest = init.then(refuse(request(state, List(op(0, Accrue, -1, 0)))))
    .then(refuse(request(state, List(op(0, Accrue, uintMax + 1, 0)))))
    .expect(state.refused == 2 and safety)
}
