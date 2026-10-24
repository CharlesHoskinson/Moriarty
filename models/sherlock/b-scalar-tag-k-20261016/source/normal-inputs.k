requires "scalar-tag.k"

module SHERLOCK-B-SCALAR-TAG-NORMAL-INPUTS
  imports SHERLOCK-B-SCALAR-TAG-FUNCTIONS
  // Independently authored model inputs, not O tuples or QA expectations.
  // Values above 2^64 and at 2^128-1 are exact Int decimal literals.
  syntax BTValueTableRead ::= btNormalConstants() [function] | btNormalData() [function]
  rule btNormalConstants()
    => btValueTableRead(2, 2,
         btRowsCons(btValueRow(btUIntTag(), btUInt128(3), btOpaque("const0-rest")),
         btRowsCons(btValueRow(btUIntTag(), btUInt128(18446744073709551617),
                               btOpaque("const1-rest")), btRowsNil())))
  rule btNormalData()
    => btValueTableRead(2, 1,
         btRowsCons(btValueRow(btUIntTag(),
                               btUInt128(340282366920938463463374607431768211455),
                               btOpaque("data0-rest")),
         btRowsCons(btValueRow(btOtherTag("inactive"), btUInt128(0),
                               btOpaque("inactive-row-rest")), btRowsNil())))
  syntax BTScalarRest ::= btNormalRest() [function]
  rule btNormalRest()
    => btScalarRest(btUInt128(4), btNoResume(),
         btSomeRef(btValueRef(btConstSpace(), 0)), btNoSort(), btStagedNone(),
         btOpaque("complete-further-register-lane-tuple"))
  syntax BTFraming ::= btNormalFraming() [function]
  rule btNormalFraming()
    => btFraming(btOpaque("control-Return-right-value"), 7, btOpaque("env-root"),
         1, 8, btRunning(), btUInt128(30), btUInt128(4),
         btAlloc(1, 0, 0), btActiveRest(0, 0, 0), 0, btOpaque("result-absent"),
         btSegBase(0, 0, 0, 0, 0, 0, 0, btUInt128(0)),
         btOutsideTables(btOpaque("data-fields"), btOpaque("environment"),
           btOpaque("value-slots"), btOpaque("right-frame-stack"), btOpaque("empty-tasks")),
         btOpaque("admission-handle"), btOpaque("run-id"), btOpaque("program-rest"))
  syntax BTRequest ::= btConstInput() [function] | btDataInput() [function]
  rule btConstInput()
    => btRequest(btState(btParams(100), btNormalConstants(), btNormalData(),
         btScalarRegister(true, "Scalar", "RetRight", "Add", btFirst(),
           btValueRef(btConstSpace(), 1), btUInt128(0), btNormalRest()),
         btChargedCounters(btUInt128(20), btUInt128(10), btUInt128(6), btUInt128(5)),
         btNormalFraming()))
  rule btDataInput()
    => btRequest(btState(btParams(100), btNormalConstants(), btNormalData(),
         btScalarRegister(true, "Scalar", "RetRight", "Add", btFirst(),
           btValueRef(btDataSpace(), 0), btUInt128(0), btNormalRest()),
         btChargedCounters(btUInt128(20), btUInt128(10), btUInt128(6), btUInt128(5)),
         btNormalFraming()))
  // These terms name the single normal semantic step; no literal postimage or
  // expected success boolean is embedded, and no execution result is asserted.
  syntax BTApplied ::= btConstStepTerm() [function] | btDataStepTerm() [function]
  syntax BTApplied ::= btStepInput(BTRequest) [function]
  rule btStepInput(btRequest(Pre)) => btStep(Pre)
  rule btConstStepTerm() => btStepInput(btConstInput())
  rule btDataStepTerm() => btStepInput(btDataInput())
endmodule
