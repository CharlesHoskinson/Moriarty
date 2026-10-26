// Rule-153 UInt multiplication projection of 32fcbce3e export_plans.rs.
// Actual u128 bound, finite sampled operands; assumes a valid reference record.
// KDisplay strings are abstract constructors, not a JSON/CLI execution model.
module exportMulBindings {
  type Payload = UIntValue(int) | PrimitiveError(int) | MissingValue
  type Ty = UInt128 | CheckedU128Result
  type Path = CapturedLeft | ReturnedRight | PrimitiveResult | NoPath
  type Binding = {name: str, ty: Ty, path: Path, value: Payload}
  type Record = {left: int, right: int, after: Payload}
  type Export = {inputs: List[Binding], output: Binding}
  var record: Record
  var exported: Export
  var observed: bool
  pure val maxU128 = 340282366920938463463374607431768211455
  pure val operands = Set(0, 1, 2, 3, maxU128)
  pure def reference(left: int, right: int): Record = {
    val product = left * right
    {left: left, right: right,
      after: if (product <= maxU128) UIntValue(product) else PrimitiveError(10)}
  }
  pure def input(name: str, path: Path, value: int): Binding =
    {name: name, ty: UInt128, path: path, value: UIntValue(value)}
  pure def describe(r: Record): Export = {
    inputs: List(input("operand_left", CapturedLeft, r.left),
      input("operand_right", ReturnedRight, r.right)),
    output: {name: "result", ty: match r.after {
        | UIntValue(_) => UInt128 | _ => CheckedU128Result
      }, path: PrimitiveResult, value: r.after},
  }
  pure def valid(r: Record, e: Export): bool = and {
    e.inputs == List(input("operand_left", CapturedLeft, r.left),
      input("operand_right", ReturnedRight, r.right)),
    e.output.name == "result",
    e.output.path == PrimitiveResult,
    e.output.value == r.after,
    match r.after {
      | UIntValue(n) => e.output.ty == UInt128 and n == r.left * r.right and n <= maxU128
      | PrimitiveError(code) => e.output.ty == CheckedU128Result and code == 10 and r.left * r.right > maxU128
      | MissingValue => false
    },
  }
  action init = all {
    record' = reference(0, 0), exported' = describe(reference(0, 0)), observed' = false,
  }
  action bind(left: int, right: int): bool = all {
    record' = reference(left, right), exported' = describe(reference(left, right)), observed' = true,
  }
  action step = {
    nondet left = operands.oneOf()
    nondet right = operands.oneOf()
    bind(left, right)
  }
  val modelInv = valid(record, exported)
  val success = observed and exported.output.ty == UInt128
  val overflow = observed and exported.output.value == PrimitiveError(10)
  val unequalOperands = observed and record.left != record.right
}
