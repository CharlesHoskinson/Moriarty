module exportMulBindings_test {
  import exportMulBindings.* from "./exportMulBindings"
  run actualFactorialOperandsTest = init.then(bind(3, 2))
    .expect(exported.inputs.nth(0).value == UIntValue(3) and exported.inputs.nth(1).value == UIntValue(2)
      and exported.output.value == UIntValue(6) and modelInv)
  run maximumTimesTwoIsErrorTest = init.then(bind(maxU128, 2))
    .expect(exported.output.ty == CheckedU128Result and exported.output.value == PrimitiveError(10) and modelInv)
  run maximumTimesOneIsSuccessTest = init.then(bind(maxU128, 1))
    .expect(exported.output.value == UIntValue(maxU128) and modelInv)
  run maximumTimesZeroIsSuccessTest = init.then(bind(maxU128, 0))
    .expect(exported.output.value == UIntValue(0) and modelInv)
  run reversedInputsRejectedEvenWhenMulCommutesTest = init.then(bind(3, 2)).expect({
    val swapped = {...exported, inputs: List(exported.inputs.nth(1), exported.inputs.nth(0))}
    not(valid(record, swapped))
  })
  run oldOverflowPlaceholderRejectedTest = init.then(bind(maxU128, 2)).expect({
    val placeholder = {...exported, output: {name: "result", ty: UInt128, path: NoPath, value: MissingValue}}
    not(valid(record, placeholder))
  })
  run wrongCapturedPathRejectedTest = init.then(bind(3, 2)).expect({
    val wrong = {...exported, inputs: List({...exported.inputs.nth(0), path: ReturnedRight}, exported.inputs.nth(1))}
    not(valid(record, wrong))
  })
  run wrongSuccessResultRejectedTest = init.then(bind(3, 2)).expect({
    val wrong = {...exported, output: {...exported.output, value: UIntValue(7)}}
    not(valid(record, wrong))
  })
}
