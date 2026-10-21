requires "../source/public-nominal-source09-inputs.k"
module SHERLOCK-PUBLIC-NOMINAL-SOURCE09-LAWS
 imports SHERLOCK-PUBLIC-NOMINAL-SOURCE09-INPUTS
 syntax Bool ::= npLawMapQueue() [function]
 rule npLawMapQueue() => npProjectedValue(npRunReference(npMapQueueProgram(),npMapQueueTypes(),npMapQueueCtors(),512)) ==K npNominal(npNominalType("Envelope",npType(npNominalType("Queue",npType(npUIntType(),npNoTypes())),npType(npBoolType(),npNoTypes()))),"Wrap",npField("z_items",npNominal(npNominalType("Queue",npType(npUIntType(),npNoTypes())),"Push",npField("z_item",npUInt(5),npField("a_tail",npNominal(npNominalType("Queue",npType(npUIntType(),npNoTypes())),"Push",npField("z_item",npUInt(10),npField("a_tail",npNominal(npNominalType("Queue",npType(npUIntType(),npNoTypes())),"Empty",npNoFields()),npNoFields()))),npNoFields()))),npField("a_marker",npBool(true),npNoFields())))
 syntax Bool ::= npLawMirrorArbor() [function]
 rule npLawMirrorArbor() => npProjectedValue(npRunReference(npMirrorArborProgram(),npMirrorArborTypes(),npMirrorArborCtors(),512)) ==K npNominal(npNominalType("Stamp",npType(npNominalType("Arbor",npType(npBoolType(),npNoTypes())),npType(npUIntType(),npNoTypes()))),"Seal",npField("z_tree",npNominal(npNominalType("Arbor",npType(npBoolType(),npNoTypes())),"Fork",npField("z_left",npNominal(npNominalType("Arbor",npType(npBoolType(),npNoTypes())),"Fork",npField("z_left",npNominal(npNominalType("Arbor",npType(npBoolType(),npNoTypes())),"Tip",npField("z_value",npBool(false),npNoFields())),npField("a_right",npNominal(npNominalType("Arbor",npType(npBoolType(),npNoTypes())),"Tip",npField("z_value",npBool(true),npNoFields())),npNoFields()))),npField("a_right",npNominal(npNominalType("Arbor",npType(npBoolType(),npNoTypes())),"Tip",npField("z_value",npBool(false),npNoFields())),npNoFields()))),npField("a_code",npUInt(41),npNoFields())))
 syntax Bool ::= npLawHeldOutNotFalse() [function]
 rule npLawHeldOutNotFalse() => npProjectedValue(npRunReference(npHeldOutNotProgram(),npNoTypeRows(),npNoCtorRows(),512)) ==K npBool(true)
 syntax Bool ::= npLawNotCompleteSavedState() [function]
 rule npLawNotCompleteSavedState() =>
  ngStep(ngState(npHeldOutNotProgram(),ngReturn(ngBool(false)),(99 |-> ngBool(true)),
   ngFrame(ngNotArgument(17,1017,(99 |-> ngUInt(8))),
    ngFrame(ngArmScope(3,1003,0,.Map),ngNoFrames())),
   ngEvent(ngLiteralEvaluated(18,1018,ngBool(false)),ngNoEvents())))
  ==K ngState(npHeldOutNotProgram(),ngReturn(ngBool(true)),(99 |-> ngUInt(8)),
   ngFrame(ngArmScope(3,1003,0,.Map),ngNoFrames()),
   ngEvent(ngLiteralEvaluated(18,1018,ngBool(false)),
    ngEvent(ngNotReturned(17,1017,ngBool(false),ngBool(true),(99 |-> ngUInt(8))),ngNoEvents())))
endmodule
