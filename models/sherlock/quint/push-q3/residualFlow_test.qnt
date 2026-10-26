module residualFlow_test {
  import residualFlow.* from "./residualFlow"
  run missingBlockIdIsRefusedTest = init.then(refuse({block: Missing, rule: 102}))
    .expect(validation.rejected == 1 and validation.accepted == List() and modelInv)
  run unknownBlockIdIsRefusedTest = init.then(refuse({block: Named(99), rule: 102}))
    .expect(validation.rejected == 1 and modelInv)
  run wrongRuleInNamedBlockIsRefusedTest = init.then(refuse({block: Named(0), rule: 80}))
    .expect(validation.rejected == 1 and modelInv)
  run knownSuccessorIsAcceptedTest = init.then(publish({block: Named(0), rule: 102}))
    .then(publish({block: Named(1), rule: 80})).expect(usedSuccessor and modelInv)
  run illegalEdgeIsRefusedTest = init.then(publish({block: Named(0), rule: 102}))
    .then(refuse({block: Named(2), rule: 84})).expect(validation.accepted.length() == 1 and modelInv)
  run genericExitDoesNotNeedExplicitEdgeTest = init.then(publish({block: Named(3), rule: 153}))
    .then(publish({block: Named(2), rule: 84})).expect(exitedGeneric and modelInv)
  run sameBlockIsAcceptedTest = init.then(publish({block: Named(0), rule: 102}))
    .then(publish({block: Named(0), rule: 102})).expect(validation.accepted.length() == 2 and modelInv)
  run enteringGenericStillNeedsAnEdgeTest = init.then(publish({block: Named(1), rule: 80}))
    .then(refuse({block: Named(3), rule: 153})).expect(validation.rejected == 1 and modelInv)
}
