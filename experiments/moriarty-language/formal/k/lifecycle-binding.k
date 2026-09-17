requires "lifecycle-kernel.k"
module MORIARTY-LIFECYCLE-BINDING
  imports MORIARTY-LIFECYCLE-KERNEL
  syntax EJSON ::= lcRecord(EJSON,String) [function, total]
                 | lcAsset(EJSON) [function, total] | lcUnit(EJSON) [function, total]
  rule lcRecord(S,K) => lxGetName(ejGet(S,"recordTypes"),ejGet(ejGet(S,"operations"),K))
  rule lcAsset(S) => ejAt(ejGet(lcRecord(S,"Transfer"),"transferAmount"),1)
  rule lcUnit(S) => ejAt(ejAt(ejAt(ejGet(lcRecord(S,"Repay"),"nominalAmount"),1),0),0)
  syntax String ::= lcBinding(EJSON) [function, total]

  rule lcBinding(S) => lxFirst(lxCheck(lcAsset(S) ==K lcUnit(S),"OPERATION_BINDING"),lxFirst(lxCheck(ejClosed(ejGet(S,"operations"),ejCons(ejString("Transfer"),ejCons(ejString("Repay"),ejCons(ejString("Originate"),ejCons(ejString("Accrue"),ejNil()))))),"OPERATION_BINDING"),lxFirst(lxCheck(lcRecord(S,"Transfer") ==K ejObject(ejCons(ejPair("from",lxType("Text")),ejCons(ejPair("id",lxType("Text")),ejCons(ejPair("settlementAsset",lxType("Text")),ejCons(ejPair("to",lxType("Text")),ejCons(ejPair("transferAmount",lxType("Amount",lcAsset(S))),ejNil())))))),"OPERATION_BINDING"),lxFirst(lxCheck(lcRecord(S,"Repay") ==K ejObject(ejCons(ejPair("allocationId",lxType("Text")),ejCons(ejPair("nominalAmount",lcQuantity(lcUnit(S))),ejCons(ejPair("obligationId",lxType("Text")),ejCons(ejPair("payer",lxType("Text")),ejCons(ejPair("transferId",lxType("Text")),ejNil())))))),"OPERATION_BINDING"),lxFirst(lxCheck(lcRecord(S,"Originate") ==K ejObject(ejCons(ejPair("accrualTerms",ejGet(lcRecord(S,"Originate"),"accrualTerms")),ejCons(ejPair("allocationRule",lxType("Text")),ejCons(ejPair("conversion",ejGet(lcRecord(S,"Originate"),"conversion")),ejCons(ejPair("creditor",lxType("Text")),ejCons(ejPair("debtor",lxType("Text")),ejCons(ejPair("denomination",lxType("Text")),ejCons(ejPair("nominalAmount",lcQuantity(lcUnit(S))),ejCons(ejPair("nominalLiabilityCap",lcQuantity(lcUnit(S))),ejCons(ejPair("obligationId",lxType("Text")),ejCons(ejPair("originationId",lxType("Text")),ejCons(ejPair("settlementAsset",lxType("Text")),ejCons(ejPair("transferId",lxType("Text")),ejNil()))))))))))))),"OPERATION_BINDING"),lxFirst(lxCheck(lcRecord(S,"Accrue") ==K ejObject(ejCons(ejPair("accrualId",lxType("Text")),ejCons(ejPair("obligationId",lxType("Text")),ejCons(ejPair("observedTime",lxType("UInt64")),ejCons(ejPair("periodIndex",lxType("UInt64")),ejNil()))))),"OPERATION_BINDING"),lxFirst(lxCheck(ejAt(ejGet(lcRecord(S,"Originate"),"conversion"),0) ==K ejString("Record"),"OPERATION_BINDING"),lxFirst(lxCheck(lxGetName(ejGet(S,"recordTypes"),ejAt(ejGet(lcRecord(S,"Originate"),"conversion"),1)) ==K ejObject(ejCons(ejPair("mantissa",lxType("UInt128")),ejCons(ejPair("rounding",lxType("Text")),ejCons(ejPair("scale",lxType("UInt128")),ejNil())))),"OPERATION_BINDING"),lxFirst(lxCheck(ejAt(ejGet(lcRecord(S,"Originate"),"accrualTerms"),0) ==K ejString("Record"),"OPERATION_BINDING"),lxFirst(lxCheck(lxGetName(ejGet(S,"recordTypes"),ejAt(ejGet(lcRecord(S,"Originate"),"accrualTerms"),1)) ==K ejObject(ejCons(ejPair("denominator",lxType("UInt128")),ejCons(ejPair("firstPeriodStart",lxType("UInt64")),ejCons(ejPair("numerator",lxType("UInt128")),ejCons(ejPair("periodSeconds",lxType("UInt64")),ejCons(ejPair("rounding",lxType("Text")),ejNil())))))),"OPERATION_BINDING"),""))))))))))
  syntax EJSON ::= lcMapDescriptor(EJSON) [function, total]
  rule lcMapDescriptor(D) => lcMapFields(lxText(ejGet(D,"operation")),ejGet(D,"fields"))
  syntax EJSON ::= lcMapFields(String,EJSON) [function, total]

  rule lcMapFields("Transfer",A) => ejObject(ejCons(ejPair("amount",ejGet(A,"transferAmount")),ejCons(ejPair("asset",ejGet(A,"settlementAsset")),ejCons(ejPair("from",ejGet(A,"from")),ejCons(ejPair("id",ejGet(A,"id")),ejCons(ejPair("kind",ejString("Transfer")),ejCons(ejPair("to",ejGet(A,"to")),ejNil())))))))
  rule lcMapFields(K,A) => ejPut(A,"kind",ejString(K)) requires K ==String "Repay" orBool K ==String "Originate" orBool K ==String "Accrue"
  rule lcMapFields(_,_) => ejMissing() [owise]
  syntax String ::= lcDescriptorCode(EJSON,EJSON) [function, total]
  rule lcDescriptorCode(D,S) => lxCheck(ejGet(ejGet(D,"fields"),"settlementAsset") ==K lcAsset(S),"SETTLEMENT_UNIT") requires ejGet(D,"operation") ==K ejString("Transfer")
  rule lcDescriptorCode(D,_) => lxCheck(lcUnsigned(ejGet(ejGet(D,"fields"),"nominalAmount"),lcS128()),"NOMINAL_RANGE") requires ejGet(D,"operation") ==K ejString("Repay")
  rule lcDescriptorCode(D,S) => lxFirst(lxCheck(lcUnsigned(ejGet(ejGet(D,"fields"),"nominalAmount"),lcS128()) andBool lcUnsigned(ejGet(ejGet(D,"fields"),"nominalLiabilityCap"),lcS128()),"NOMINAL_RANGE"),lxFirst(lxCheck(ejGet(ejGet(D,"fields"),"denomination") ==K lcUnit(S),"NOMINAL_UNIT"),lxCheck(ejGet(ejGet(D,"fields"),"settlementAsset") ==K lcAsset(S),"SETTLEMENT_UNIT"))) requires ejGet(D,"operation") ==K ejString("Originate")
  rule lcDescriptorCode(D,_) => "" requires ejGet(D,"operation") ==K ejString("Accrue")
  rule lcDescriptorCode(_,_) => "UNSUPPORTED_OPERATION" [owise]
  syntax String ::= lcActionCode(EJSON) [function, total]

  rule lcActionCode(A) => lxFirst(lxCheck(lcIdentifier(ejGet(A,"id")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"from")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"to")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"asset")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcUnsigned(ejGet(A,"amount"),lcU128()),"INVALID_AMOUNT"),""))))) requires ejGet(A,"kind") ==K ejString("Transfer")
  rule lcActionCode(A) => lxFirst(lxCheck(lcIdentifier(ejGet(A,"allocationId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"transferId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"obligationId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"payer")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcUnsigned(ejGet(A,"nominalAmount"),lcS128()),"INVALID_AMOUNT"),""))))) requires ejGet(A,"kind") ==K ejString("Repay")
  rule lcActionCode(A) => lxFirst(lxCheck(lcIdentifier(ejGet(A,"obligationId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"transferId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"originationId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"debtor")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"creditor")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcUnsigned(ejGet(A,"nominalAmount"),lcS128()),"INVALID_AMOUNT"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"denomination")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"settlementAsset")),"INVALID_IDENTIFIER"),lxFirst(lcRow("conversion",ejGet(A,"conversion")),lxFirst(lxCheck(ejGet(A,"allocationRule") ==K ejString("AccrualFirst") orBool ejGet(A,"allocationRule") ==K ejString("PrincipalFirst") orBool ejGet(A,"allocationRule") ==K ejString("ProRata"),"SCHEMA"),lxFirst(lcRow("terms",ejGet(A,"accrualTerms")),lxFirst(lxCheck(lcUnsigned(ejGet(A,"nominalLiabilityCap"),lcS128()),"INVALID_AMOUNT"),"")))))))))))) requires ejGet(A,"kind") ==K ejString("Originate")
  rule lcActionCode(A) => lxFirst(lxCheck(lcIdentifier(ejGet(A,"accrualId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcIdentifier(ejGet(A,"obligationId")),"INVALID_IDENTIFIER"),lxFirst(lxCheck(lcUnsigned(ejGet(A,"periodIndex"),lcU64()),"INVALID_AMOUNT"),lxFirst(lxCheck(lcUnsigned(ejGet(A,"observedTime"),lcU64()),"INVALID_AMOUNT"),"")))) requires ejGet(A,"kind") ==K ejString("Accrue")
  rule lcActionCode(_) => "UNKNOWN_ACTION" [owise]
  syntax String ::= lcNoFinancialFields(EJSON) [function, total] | lcNoFinancialFieldList(EJList) [function, total]
  rule lcNoFinancialFields(S) => lcNoFinancialFieldList(lcObjectPairs(ejGet(S,"fields")))
  rule lcNoFinancialFieldList(ejNil()) => ""
  rule lcNoFinancialFieldList(ejCons(ejPair(_,F),L)) => lxFirst(lxCheck(ejGet(F,"writeClass") =/=K ejString("financial"),"TYPE_FINANCIAL_WRITE"),lcNoFinancialFieldList(L))
  syntax String ::= lcNominalUnits(EJList,EJSON,EJSON) [function, total]
  rule lcNominalUnits(ejNil(),_,_) => ""
  rule lcNominalUnits(ejCons(A,L),S,U) => lxFirst(lxCheck(ejGet(lcFind(ejGet(S,"obligations"),"id",ejGet(A,"obligationId")),"denomination") ==K U,"NOMINAL_UNIT"),lcNominalUnits(L,S,U)) requires ejGet(A,"kind") ==K ejString("Repay") orBool ejGet(A,"kind") ==K ejString("Accrue")
  rule lcNominalUnits(ejCons(A,L),S,U) => lcNominalUnits(L,S,U) requires ejGet(A,"kind") =/=K ejString("Repay") andBool ejGet(A,"kind") =/=K ejString("Accrue")
  syntax EJList ::= lcObjectPairs(EJSON) [function, total]
  rule lcObjectPairs(ejObject(L)) => L
  rule lcObjectPairs(_) => ejNil() [owise]
endmodule
