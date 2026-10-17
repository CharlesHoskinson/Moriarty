module directedRefusal_test {
 import directedRefusal.* from "./directedRefusal"
 run directedFinishTest=init.then(2.reps(i=>step)).expect(directedFinished and projectionRefused and allInvariant)
}
