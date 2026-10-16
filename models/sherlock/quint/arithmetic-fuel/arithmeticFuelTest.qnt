// Whole literal expectations frozen before implementation; every test UNRUN.
module arithmeticFuelTest {
  import arithmeticFuel.* from "./arithmeticFuel"
  // 255 + 1 carries into byte 1
  pure val arithInput1: ArithInput = {op: AddOp, left: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), right: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)}
  pure val arithExpected1: ArithWitness = {low: List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), highSum: 0, highZero: 0, outcome: UIntOk(List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0))}
  run arithmeticNormal1Test = observeArith(arithInput1)
    .expect(observed == ArithObservation({input: arithInput1, witness: arithExpected1}) and modelInv)
  // max + 1 overflows after all 16 bytes
  pure val arithInput2: ArithInput = {op: AddOp, left: List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255), right: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)}
  pure val arithExpected2: ArithWitness = {low: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), highSum: 0, highZero: 0, outcome: Err10}
  run arithmeticNormal2Test = observeArith(arithInput2)
    .expect(observed == ArithObservation({input: arithInput2, witness: arithExpected2}) and modelInv)
  // 0 - 1 underflows with max remainder
  pure val arithInput3: ArithInput = {op: SubOp, left: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), right: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)}
  pure val arithExpected3: ArithWitness = {low: List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), highSum: 0, highZero: 0, outcome: Err10}
  run arithmeticNormal3Test = observeArith(arithInput3)
    .expect(observed == ArithObservation({input: arithInput3, witness: arithExpected3}) and modelInv)
  // 256 - 1 borrows through byte 0
  pure val arithInput4: ArithInput = {op: SubOp, left: List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0), right: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)}
  pure val arithExpected4: ArithWitness = {low: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), highSum: 0, highZero: 0, outcome: UIntOk(List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0))}
  run arithmeticNormal4Test = observeArith(arithInput4)
    .expect(observed == ArithObservation({input: arithInput4, witness: arithExpected4}) and modelInv)
  // high byte dominates: 2^120 < 255 is false
  pure val arithInput5: ArithInput = {op: LtOp, left: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1), right: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)}
  pure val arithExpected5: ArithWitness = {low: List(1,255,255,255,255,255,255,255,255,255,255,255,255,255,255,0), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0), highSum: 0, highZero: 0, outcome: BoolOk(false)}
  run arithmeticNormal5Test = observeArith(arithInput5)
    .expect(observed == ArithObservation({input: arithInput5, witness: arithExpected5}) and modelInv)
  // reverse high-byte comparison is true
  pure val arithInput6: ArithInput = {op: LtOp, left: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), right: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1)}
  pure val arithExpected6: ArithWitness = {low: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1), highSum: 0, highZero: 0, outcome: BoolOk(true)}
  run arithmeticNormal6Test = observeArith(arithInput6)
    .expect(observed == ArithObservation({input: arithInput6, witness: arithExpected6}) and modelInv)
  // 255 squared has two nonzero low bytes
  pure val arithInput7: ArithInput = {op: MulOp, left: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), right: List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)}
  pure val arithExpected7: ArithWitness = {low: List(1,254,0,0,0,0,0,0,0,0,0,0,0,0,0,0), high: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), carry: List(0,254,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), highSum: 0, highZero: 1, outcome: UIntOk(List(1,254,0,0,0,0,0,0,0,0,0,0,0,0,0,0))}
  run arithmeticNormal7Test = observeArith(arithInput7)
    .expect(observed == ArithObservation({input: arithInput7, witness: arithExpected7}) and modelInv)
  // max squared complete 32 bytes and carry chain
  pure val arithInput8: ArithInput = {op: MulOp, left: List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255), right: List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255)}
  pure val arithExpected8: ArithWitness = {low: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), high: List(254,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255), carry: List(0,254,509,764,1019,1274,1529,1784,2039,2294,2549,2804,3059,3314,3569,3824,4079,3825,3570,3315,3060,2805,2550,2295,2040,1785,1530,1275,1020,765,510,255,0), highSum: 4079, highZero: 0, outcome: Err10}
  run arithmeticNormal8Test = observeArith(arithInput8)
    .expect(observed == ArithObservation({input: arithInput8, witness: arithExpected8}) and modelInv)
  // F1 A102 charged fuel 1 to 0
  pure val fuelInput1: FuelInput = {remaining: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: OtherControl, depth: 0}
  pure val fuelExpected1: FuelWitness = {zeroBits: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), borrow: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 1, regime: Charged}
  run fuelNormal1Test = observeFuel(fuelInput1)
    .expect(observed == FuelObservation({input: fuelInput1, witness: fuelExpected1}) and modelInv)
  // F2 A111 fuel 65536 borrows across two zero bytes
  pure val fuelInput2: FuelInput = {remaining: List(0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: OtherControl, depth: 0}
  pure val fuelExpected2: FuelWitness = {zeroBits: List(1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0), borrow: List(1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(255,255,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 1, regime: Charged}
  run fuelNormal2Test = observeFuel(fuelInput2)
    .expect(observed == FuelObservation({input: fuelInput2, witness: fuelExpected2}) and modelInv)
  // F3 A153 semantic Err10 still charged, long borrow
  pure val fuelInput3: FuelInput = {remaining: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1), eventEnabled: true, control: RetControl, depth: 1}
  pure val fuelExpected3: FuelWitness = {zeroBits: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0), zeroPrefix: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0), borrow: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0), post: List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,0), charge: 1, regime: Charged}
  run fuelNormal3Test = observeFuel(fuelInput3)
    .expect(observed == FuelObservation({input: fuelInput3, witness: fuelExpected3}) and modelInv)
  // F4 A135 max fuel decrements
  pure val fuelInput4: FuelInput = {remaining: List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255), eventEnabled: true, control: OtherControl, depth: 1}
  pure val fuelExpected4: FuelWitness = {zeroBits: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), zeroPrefix: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), borrow: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(254,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255), charge: 1, regime: Charged}
  run fuelNormal4Test = observeFuel(fuelInput4)
    .expect(observed == FuelObservation({input: fuelInput4, witness: fuelExpected4}) and modelInv)
  // F5a A080 Return empty stack wins over zero fuel
  pure val fuelInput5: FuelInput = {remaining: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: RetControl, depth: 0}
  pure val fuelExpected5: FuelWitness = {zeroBits: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), borrow: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 0, regime: DoneFree}
  run fuelNormal5Test = observeFuel(fuelInput5)
    .expect(observed == FuelObservation({input: fuelInput5, witness: fuelExpected5}) and modelInv)
  // F5b A084 nonterminal zero fuel exhausts
  pure val fuelInput6: FuelInput = {remaining: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: OtherControl, depth: 0}
  pure val fuelExpected6: FuelWitness = {zeroBits: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), borrow: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 0, regime: ExhaustedFree}
  run fuelNormal6Test = observeFuel(fuelInput6)
    .expect(observed == FuelObservation({input: fuelInput6, witness: fuelExpected6}) and modelInv)
  // F6 A082 Error with fuel preserves it
  pure val fuelInput7: FuelInput = {remaining: List(5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: ErrorControl, depth: 1}
  pure val fuelExpected7: FuelWitness = {zeroBits: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), borrow: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 0, regime: RejectedFree}
  run fuelNormal7Test = observeFuel(fuelInput7)
    .expect(observed == FuelObservation({input: fuelInput7, witness: fuelExpected7}) and modelInv)
  // F7 A082 Error wins over zero fuel
  pure val fuelInput8: FuelInput = {remaining: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: ErrorControl, depth: 1}
  pure val fuelExpected8: FuelWitness = {zeroBits: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), borrow: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 0, regime: RejectedFree}
  run fuelNormal8Test = observeFuel(fuelInput8)
    .expect(observed == FuelObservation({input: fuelInput8, witness: fuelExpected8}) and modelInv)
  // E1 E-FUEL A108 charged independently of omitted inventory
  pure val fuelInput9: FuelInput = {remaining: List(5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: OtherControl, depth: 0}
  pure val fuelExpected9: FuelWitness = {zeroBits: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), borrow: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 1, regime: Charged}
  run fuelNormal9Test = observeFuel(fuelInput9)
    .expect(observed == FuelObservation({input: fuelInput9, witness: fuelExpected9}) and modelInv)
  // A225 primitive failure is charged before Error control
  pure val fuelInput10: FuelInput = {remaining: List(3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), eventEnabled: true, control: OtherControl, depth: 0}
  pure val fuelExpected10: FuelWitness = {zeroBits: List(0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1), zeroPrefix: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), borrow: List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), post: List(2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0), charge: 1, regime: Charged}
  run fuelNormal10Test = observeFuel(fuelInput10)
    .expect(observed == FuelObservation({input: fuelInput10, witness: fuelExpected10}) and modelInv)
}
