module treeInterning_test {
  import treeInterning.* from "./treeInterning"
  action begin(a: int, b: int, reversed: bool): bool = interner' = start(a, b, reversed)
  action shapeOnlyTick = all {interner.position < 7, interner' = intern(interner, true)}
  run sevenOccurrencesBecomeFourPhysicalNodesTest = begin(4, 5, false)
    .then(tick).then(tick).then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(finished and interner.input.length() == 7 and interner.nodes.length() == 4
      and interner.handles.get(2) == interner.handles.get(5)
      and sums(interner.nodes).nth(interner.handles.get(6)) == 18 and modelInv)
  run equalLeavesAlsoShareTest = begin(4, 4, false)
    .then(tick).then(tick).then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(interner.nodes.length() == 3 and sums(interner.nodes).nth(interner.handles.get(6)) == 16 and modelInv)
  run reversedChildrenAreNotEqualNodesTest = begin(4, 5, true)
    .then(tick).then(tick).then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(interner.nodes.length() == 5 and interner.handles.get(2) != interner.handles.get(5) and modelInv)
  run payloadBlindLeafMergeIsDetectedTest = begin(0, 5, false)
    .then(shapeOnlyTick).then(shapeOnlyTick).expect(not(modelInv))
  run zeroFuelBoundaryStillPreservedTest = begin(4, 5, false).then(tick)
    .expect(interner.after.fuel == 20 and interner.after.frames == List(3, 3) and modelInv)
}
