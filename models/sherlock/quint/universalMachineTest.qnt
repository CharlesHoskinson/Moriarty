module universalMachineTest {
  import universalMachine.* from "./universalMachine"

  // Literals were frozen independently in expected-normal.json before this source.
  run factorial3Test = initCase(FactorialCase)
    .expect(not(recursiveWitness))
    .then(3.reps(i => tick))
    .expect(machine.frames.length() == 3 and machine.arg == 0 and allInv)
    .then(5.reps(i => tick))
    .expect(recursiveWitness and machine.result == ResultValue(6) and allInv)

  run listSum123Test = initCase(ListCase)
    .expect(not(listWitness))
    .then(3.reps(i => tick))
    .expect(machine.frames.length() == 3 and machine.cursor == 0 and allInv)
    .then(5.reps(i => tick))
    .expect(listWitness and machine.result == ResultValue(6) and allInv)

  run treeSum45Test = initCase(TreeCase)
    .expect(not(treeWitness))
    .then(3.reps(i => tick))
    .expect(machine.cursor == 5 and machine.control == Evaluate and allInv)
    .then(3.reps(i => tick))
    .expect(treeWitness and machine.result == ResultValue(9) and allInv)

  run yieldPendingReturnTest = initCase(YieldCase)
    .expect(not(waitingWitness))
    .then(5.reps(i => tick))
    .expect(and {
      waitingWitness, machine.acc == 1, machine.pending == PendingReturn(1),
      machine.frames.length() == 3, machine.checkpoint.consumed == 5, allInv,
    })
    .then(observe(false))
    .expect(and {
      machine.phase == Waiting, machine.observations == 1,
      machine.acceptedEvents == 0, machine.remaining == 27,
      machine.frames == machine.checkpoint.frames,
      machine.checkpoint.consumed == 5, machine.boundary == 1, allInv,
    })
    .then(observe(true))
    .expect(and {
      machine.phase == Running, machine.pending == NoPending,
      machine.frames.length() == 3, machine.remaining == 27,
      machine.checkpoint.consumed == 5, machine.boundary == 1, allInv,
    })
    .then(4.reps(i => tick))
    .expect(and {
      resumedWitness, machine.result == ResultValue(6),
      machine.checkpoint.consumed == 5, machine.checkpoint.acc == 1,
      machine.checkpoint.frames.length() == 3, machine.boundary == 1, allInv,
    })

  run exhaustionAfterProgressTest = initCase(ExhaustCase)
    .expect(not(exhaustionWitness))
    .then(2.reps(i => tick))
    .expect(machine.phase == Running and machine.remaining == 0 and allInv)
    .then(tick)
    .expect(and {
      exhaustionWitness, machine.consumed == 2, machine.attempted == 3,
      machine.frames.length() == 2, machine.boundary == 0, allInv,
    })

  // Ordinary mathematical result outside this instance's scalar domain.
  run normalScalarBoundTest = initCase(BoundCase)
    .expect(not(rejectionWitness))
    .then(9.reps(i => tick))
    .expect(and {
      rejectionWitness, machine.result == NoResult, machine.acc == 6,
      machine.consumed == 9, machine.attempted == 9, allInv,
    })

  run terminalStutterTest = initCase(FactorialCase)
    .then(8.reps(i => tick))
    .expect(machine.phase == Halted and machine.consumed == 8)
    .then(terminalStutter)
    .then(terminalStutter)
    .expect(and {
      machine.phase == Halted, machine.result == ResultValue(6),
      machine.consumed == 8, machine.attempted == 8,
      machine.frames.length() == 0, machine.boundary == 0, allInv,
    })

  run concreteBoundsTest = initCase(TreeCase)
    .expect(and {
      MAX_SCALAR == 15, MAX_FRAMES == 8, HEAP_IDS.size() == 8,
      machine.total == 32, machine.heap.keys() == HEAP_IDS, allInv,
    })
}
