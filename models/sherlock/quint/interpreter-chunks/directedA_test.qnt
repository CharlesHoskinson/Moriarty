module directedA_test {
 import directedA.* from "./directedA"
 run directedFinishTest=init.then(12.reps(i=>step)).expect(directedFinished and scalarProjected and allInvariant)
}
