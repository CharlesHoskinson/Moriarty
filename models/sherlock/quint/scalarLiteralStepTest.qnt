module scalarLiteralStepTest {
  import kProfileMachine.* from "./kProfileMachine"

  // Hand-derived complete normal states, frozen before emitter implementation.
  // Exact fixture: scalarLiteralStep.expected.json, SHA256
  // 358e34422f4e2487832a5afd5768cb4849a0f0247052a3163a0a204a6ada7990.
  // No evaluator or circuit output generated these expectations. Tests UNRUN.
  // K-A publication is free; the older Quint-A terminal tick is not this profile.

  pure val uint7Program: Program = {
    expressions: Map(0 -> Literal(UInt(7))), functions: List(), entry: 0,
  }
  pure val uint7Before: Snapshot = {
    program: uint7Program, control: Eval(0), environment: Map(), frames: List(),
    stackLimit: 0, remaining: 1, initial: 1, outcome: Pending,
  }
  pure val uint7Returned: Snapshot = {
    program: uint7Program, control: Return(UInt(7)), environment: Map(), frames: List(),
    stackLimit: 0, remaining: 0, initial: 1, outcome: Pending,
  }
  pure val uint7Published: Snapshot = {
    program: uint7Program, control: EmptyControl, environment: Map(), frames: List(),
    stackLimit: 0, remaining: 0, initial: 1, outcome: Done(UInt(7)),
  }

  run uint7LiteralAndFreePublicationTest = initProgram(uint7Program, 1, 0)
    .expect(machine == uint7Before and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == uint7Returned and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == uint7Published and lastRule == 80 and lastCharge == 0)

  pure val falseProgram: Program = {
    expressions: Map(0 -> Literal(BoolValue(false))), functions: List(), entry: 0,
  }
  pure val falseBefore: Snapshot = {
    program: falseProgram, control: Eval(0), environment: Map(), frames: List(),
    stackLimit: 4, remaining: 3, initial: 3, outcome: Pending,
  }
  pure val falseReturned: Snapshot = {
    program: falseProgram, control: Return(BoolValue(false)), environment: Map(), frames: List(),
    stackLimit: 4, remaining: 2, initial: 3, outcome: Pending,
  }
  pure val falsePublished: Snapshot = {
    program: falseProgram, control: EmptyControl, environment: Map(), frames: List(),
    stackLimit: 4, remaining: 2, initial: 3, outcome: Done(BoolValue(false)),
  }

  run falseLiteralAndFreePublicationTest = initProgram(falseProgram, 3, 4)
    .expect(machine == falseBefore and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == falseReturned and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == falsePublished and lastRule == 80 and lastCharge == 0)
}
