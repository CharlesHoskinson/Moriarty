// Finite normal QA of selected aux/2 relations. Source only; all execution UNRUN.
// Full 16-byte words, finite sampled inputs; no SDK/field/dispatch correspondence.
module arithmeticFuel {
  type Op = AddOp | SubOp | MulOp | LtOp
  type ArithOutcome = UIntOk(List[int]) | BoolOk(bool) | Err10
  type ArithInput = {op: Op, left: List[int], right: List[int]}
  type Ripple = {digits: List[int], links: List[int]}
  // links include fixed endpoint c0 (and c32 for Mul), not extra ABI lanes.
  type ArithWitness = {low: List[int], high: List[int], carry: List[int],
    highSum: int, highZero: int, outcome: ArithOutcome}
  type ControlClass = RetControl | ErrorControl | OtherControl
  type FuelInput = {remaining: List[int], eventEnabled: bool,
    control: ControlClass, depth: int}
  type FuelRegime = Disabled | DoneFree | RejectedFree | ExhaustedFree | Charged
  type FuelWitness = {zeroBits: List[int], zeroPrefix: List[int],
    borrow: List[int], post: List[int], charge: int, regime: FuelRegime}
  type Observation = ArithObservation({input: ArithInput, witness: ArithWitness})
    | FuelObservation({input: FuelInput, witness: FuelWitness})
  var observed: Observation

  // Finite MODEL-ONLY sampling domain. Word width/base are unchanged from aux/2.
  pure val z16 = List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val one16 = List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val max16 = List(255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255)
  pure val high120 = List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1)
  pure val high127 = List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128)
  pure val words: Set[List[int]] = Set(z16, one16, max16, high120, high127,
    List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
    List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0))
  pure val fuelWords: Set[List[int]] = Set(z16, one16, max16, high120,
    List(0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0),
    List(5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
    List(3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0))
  pure val ops: Set[Op] = Set(AddOp, SubOp, MulOp, LtOp)
  pure val controls: Set[ControlClass] = Set(RetControl, ErrorControl, OtherControl)
  pure val depths: Set[int] = Set(0,1)
  pure def bit(wi: bool): int = if (wi) 1 else 0
  pure def wordOf(wi: List[int]): int =
    range(0, wi.length()).foldl(0, (acc, ix) => acc + wi.nth(ix) * (256 ^ ix))
  pure def sumBytes(wi: List[int]): int = wi.foldl(0, (acc, digit) => acc + digit)
  pure def byteWord(wi: List[int]): bool = wi.length() == 16 and
    wi.indices().forall(ix => wi.nth(ix) >= 0 and wi.nth(ix) < 256)
  pure def bitsList(wi: List[int]): bool =
    wi.indices().forall(ix => wi.nth(ix) == 0 or wi.nth(ix) == 1)

  // Constructor arithmetic uses local recurrence; global integer equations below
  // are a separate oracle. Literal tests provide separately hand-derived chains.
  pure def rippleAdd(aa: List[int], bb: List[int]): Ripple =
    range(0,16).foldl({digits: List(), links: List(0)}, (acc, ix) => {
      val total = aa.nth(ix) + bb.nth(ix) + acc.links.nth(ix)
      {digits: acc.digits.append(total % 256), links: acc.links.append(total / 256)}
    })
  pure def rippleSub(aa: List[int], bb: List[int]): Ripple =
    range(0,16).foldl({digits: List(), links: List(0)}, (acc, ix) => {
      val diff = aa.nth(ix) - bb.nth(ix) - acc.links.nth(ix)
      val br = bit(diff < 0)
      {digits: acc.digits.append(diff + 256 * br), links: acc.links.append(br)}
    })
  pure def columnSum(aa: List[int], bb: List[int], ix: int): int =
    range(0,16).foldl(0, (acc, jx) => acc +
      (if (ix - jx >= 0 and ix - jx < 16) aa.nth(jx) * bb.nth(ix-jx) else 0))
  pure def product(aa: List[int], bb: List[int]): Ripple =
    range(0,32).foldl({digits: List(), links: List(0)}, (acc, ix) => {
      val total = columnSum(aa,bb,ix) + acc.links.nth(ix)
      {digits: acc.digits.append(total % 256), links: acc.links.append(total / 256)}
    })
  pure def arithmeticWitness(wi: ArithInput): ArithWitness = {
    val aw = if (wi.op == AddOp) rippleAdd(wi.left,wi.right)
      else if (wi.op == MulOp) product(wi.left,wi.right)
      else rippleSub(wi.left,wi.right)
    val packedR = aw.digits.slice(0,16)
    val bb = if (wi.op == MulOp) aw.digits.slice(16,32) else z16
    val total = sumBytes(bb)
    val expectedOutcome = if (wi.op == LtOp) BoolOk(aw.links.nth(16) == 1)
      else if (wi.op == MulOp)
        (if (total == 0) UIntOk(packedR) else Err10)
      else if (aw.links.nth(16) == 0) UIntOk(packedR) else Err10
    {low: packedR, high: bb, carry: aw.links, highSum: total,
      highZero: if (wi.op == MulOp) bit(total == 0) else 0, outcome: expectedOutcome}
  }

  // eventEnabled abstracts Pending AND a present Control row. Terminal control
  // is examined BEFORE fuel exhaustion. Rule identifiers never select charge.
  pure def fuelWitness(fi: FuelInput): FuelWitness = {
    val evt = bit(fi.eventEnabled)
    val zeros = range(0,16).foldl(List(), (acc, ix) =>
      acc.append(evt * bit(fi.remaining.nth(ix) == 0)))
    val prefix = range(0,16).foldl(List(evt), (acc, ix) =>
      acc.append(acc.nth(ix) * zeros.nth(ix)))
    val terminal = fi.control == ErrorControl or (fi.control == RetControl and fi.depth == 0)
    val chargeValue = evt * bit(not(terminal)) * (1 - prefix.nth(16))
    val borrows = range(0,16).foldl(List(chargeValue), (acc, ix) =>
      acc.append(acc.nth(ix) * zeros.nth(ix)))
    val postWord = range(0,16).foldl(List(), (acc, ix) =>
      acc.append(fi.remaining.nth(ix) - borrows.nth(ix) + 256 * borrows.nth(ix+1)))
    val regimeValue = if (not(fi.eventEnabled)) Disabled
      else if (fi.control == RetControl and fi.depth == 0) DoneFree
      else if (fi.control == ErrorControl) RejectedFree
      else if (prefix.nth(16) == 1) ExhaustedFree else Charged
    {zeroBits: zeros, zeroPrefix: prefix, borrow: borrows, post: postWord,
      charge: chargeValue, regime: regimeValue}
  }

  pure def outcomeRelation(wi: ArithInput, aw: ArithWitness): bool = {
    val packedA = wordOf(wi.left)
    val packedB = wordOf(wi.right)
    val productValue = if (wi.op == AddOp) packedA + packedB
      else if (wi.op == SubOp) packedA - packedB else packedA * packedB
    if (wi.op == LtOp) aw.outcome == BoolOk(packedA < packedB)
    else if (productValue < 0 or productValue >= 256^16) aw.outcome == Err10
    else aw.outcome == UIntOk(aw.low) and wordOf(aw.low) == productValue
  }
  pure def arithRelation(wi: ArithInput, aw: ArithWitness): bool = {
    val arithShape = and {
      byteWord(wi.left), byteWord(wi.right), byteWord(aw.low), byteWord(aw.high),
      aw.carry.length() == (if (wi.op == MulOp) 33 else 17),
      aw.carry.nth(0) == 0,
      if (wi.op == MulOp) and {
        aw.carry.indices().forall(ix => aw.carry.nth(ix) >= 0 and aw.carry.nth(ix) < 4096),
        aw.carry.nth(32) == 0, aw.highSum == sumBytes(aw.high),
        aw.highZero == bit(aw.highSum == 0)
      } else and {bitsList(aw.carry), aw.high == z16, aw.highSum == 0, aw.highZero == 0}
    }
    val localEquations = if (wi.op == MulOp)
      range(0,32).indices().forall(ix =>
        columnSum(wi.left,wi.right,ix) + aw.carry.nth(ix) ==
        (if (ix < 16) aw.low.nth(ix) else aw.high.nth(ix-16)) + 256 * aw.carry.nth(ix+1))
      else range(0,16).indices().forall(ix => if (wi.op == AddOp)
        wi.left.nth(ix) + wi.right.nth(ix) + aw.carry.nth(ix) == aw.low.nth(ix) + 256 * aw.carry.nth(ix+1)
        else wi.left.nth(ix) + 256 * aw.carry.nth(ix+1) == wi.right.nth(ix) + aw.carry.nth(ix) + aw.low.nth(ix))
    val productValue = wordOf(aw.low) + 256^16 * wordOf(aw.high)
    and {
      arithShape, localEquations, outcomeRelation(wi,aw),
      if (wi.op == MulOp) productValue == wordOf(wi.left) * wordOf(wi.right)
      else if (wi.op == AddOp) wordOf(aw.low) + 256^16 * aw.carry.nth(16) == wordOf(wi.left) + wordOf(wi.right)
      else wordOf(aw.low) == wordOf(wi.left) - wordOf(wi.right) + 256^16 * aw.carry.nth(16)
    }
  }
  pure def fuelRelation(fi: FuelInput, fw: FuelWitness): bool = {
    val evt = bit(fi.eventEnabled)
    val terminal = fi.control == ErrorControl or (fi.control == RetControl and fi.depth == 0)
    // Whole-word oracle is independent of prefix/borrow construction.
    val chargeValue = if (fi.eventEnabled and not(terminal) and wordOf(fi.remaining) > 0) 1 else 0
    val regimeValue = if (not(fi.eventEnabled)) Disabled
      else if (fi.control == RetControl and fi.depth == 0) DoneFree
      else if (fi.control == ErrorControl) RejectedFree
      else if (wordOf(fi.remaining) == 0) ExhaustedFree else Charged
    and {
      byteWord(fi.remaining), fi.depth >= 0, byteWord(fw.post),
      fw.zeroBits.length() == 16, fw.zeroPrefix.length() == 17, fw.borrow.length() == 17,
      bitsList(fw.zeroBits), bitsList(fw.zeroPrefix), bitsList(fw.borrow),
      fw.zeroPrefix.nth(0) == evt, fw.borrow.nth(0) == chargeValue,
      fw.borrow.nth(16) == 0, fw.charge == chargeValue, fw.regime == regimeValue,
      range(0,16).indices().forall(ix => and {
        fw.zeroBits.nth(ix) == evt * bit(fi.remaining.nth(ix) == 0),
        fw.zeroPrefix.nth(ix+1) == fw.zeroPrefix.nth(ix) * fw.zeroBits.nth(ix),
        fw.borrow.nth(ix+1) == fw.borrow.nth(ix) * fw.zeroBits.nth(ix),
        fw.post.nth(ix) == fi.remaining.nth(ix) - fw.borrow.nth(ix) + 256 * fw.borrow.nth(ix+1)
      }),
      wordOf(fw.post) == wordOf(fi.remaining) - chargeValue,
      if (chargeValue == 0) fw.post == fi.remaining else true
    }
  }

  action observeArith(wi: ArithInput): bool =
    observed' = ArithObservation({input: wi, witness: arithmeticWitness(wi)})
  action observeFuel(fi: FuelInput): bool =
    observed' = FuelObservation({input: fi, witness: fuelWitness(fi)})
  action init = observeArith({op: AddOp, left: z16, right: z16})
  action arithmeticStep = {
    nondet selectedOp = ops.oneOf()
    nondet selectedLeft = words.oneOf()
    nondet selectedRight = words.oneOf()
    observeArith({op: selectedOp, left: selectedLeft, right: selectedRight})
  }
  action fuelStep = {
    nondet selectedFuel = fuelWords.oneOf()
    nondet selectedControl = controls.oneOf()
    nondet selectedDepth = depths.oneOf()
    nondet enabledEvent = Set(false,true).oneOf()
    observeFuel({remaining: selectedFuel, eventEnabled: enabledEvent,
      control: selectedControl, depth: selectedDepth})
  }
  action step = any {arithmeticStep, fuelStep}
  val modelInv = match observed {
    | ArithObservation(arithmeticCase) => arithRelation(arithmeticCase.input, arithmeticCase.witness)
    | FuelObservation(fuelCase) => fuelRelation(fuelCase.input, fuelCase.witness)
  }
}
