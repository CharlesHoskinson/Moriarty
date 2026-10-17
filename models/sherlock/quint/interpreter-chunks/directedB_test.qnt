module directedB_test {
 import directedB.* from "./directedB"
 run directedFinishTest=init.then(11.reps(i=>step)).expect(directedFinished and noNext and allInvariant)
}
