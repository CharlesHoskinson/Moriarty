module authorization_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import authorization_harness.* from "./authorization_harness"

  run prepareAfterResolveTest = init.then(prepareAlice).expect(aliceChecked)
  run prepareBeforeResolveTest = assert(canPrepareSigning(initialSigning, aliceBefore, Alice))
  run wrongSignerCannotCheckTest = assert(not(canPrepareSigning(initialSigning, aliceAfter, Mallory)))
  run cannotSignWithoutCheckTest = assert(not(canSign(initialSigning, aliceAfter, Alice, 0)))
}
