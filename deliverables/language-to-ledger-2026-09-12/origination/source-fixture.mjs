// Independent source consumer fixture. No production implementation is imported.
export const declarations = `
unit Cash;
asset Cash: Asset;
record TransferFields { id: Text; from: Text; to: Text; settlementAsset: Text; transferAmount: Amount<Cash>; }
record RepayFields { allocationId: Text; transferId: Text; obligationId: Text; payer: Text; nominalAmount: Quantity<Units<Cash,1>,0>; }
record ConversionFields { mantissa: UInt128; scale: UInt128; rounding: Text; }
record AccrualTermsFields { numerator: UInt128; denominator: UInt128; rounding: Text; periodSeconds: UInt64; firstPeriodStart: UInt64; }
record OriginateFields { obligationId: Text; transferId: Text; originationId: Text; debtor: Text; creditor: Text; nominalAmount: Quantity<Units<Cash,1>,0>; denomination: Text; settlementAsset: Text; conversion: Record<ConversionFields>; allocationRule: Text; accrualTerms: Record<AccrualTermsFields>; nominalLiabilityCap: Quantity<Units<Cash,1>,0>; }
record AccrueFields { accrualId: Text; obligationId: Text; periodIndex: UInt64; observedTime: UInt64; }
operation Transfer: TransferFields;
operation Repay: RepayFields;
operation Originate: OriginateFields;
operation Accrue: AccrueFields;
state phase: UInt128;`;
export const originationPair = (transfer='transferId',origin='originationId',loan='"Loan1"') => `
emit Transfer { id: ${transfer}, from: "Lender", to: "Borrower", settlementAsset: "Cash", transferAmount: amount<Cash>(100) };
emit Originate { obligationId: ${loan}, transferId: ${transfer}, originationId: ${origin}, debtor: "Borrower", creditor: "Lender", nominalAmount: nominal, denomination: "Cash", settlementAsset: "Cash", conversion: record<ConversionFields>{mantissa: 1, scale: 0, rounding: "none"}, allocationRule: "AccrualFirst", accrualTerms: record<AccrualTermsFields>{numerator: 1, denominator: 10, rounding: "floor", periodSeconds: u64(60), firstPeriodStart: u64(1000)}, nominalLiabilityCap: quantity<Units<Cash,1>,0>(110) };`;
export const source = `profile "moriarty-financial-agreement-source/5";
agreement IndependentOrigin {
${declarations}
action originate(transferId: Text, originationId: Text) {
 let nominal = quantity<Units<Cash,1>,0>(100);
 next.phase = pre.phase + 1;
 ${originationPair()}
 ensures post.phase == pre.phase + 1;
 ensures post_outstanding<Cash>("Loan1") == nominal;
 ensures post_balance<Cash>("Borrower") == amount<Cash>(110);
}
action accrue(accrualId: Text, periodIndex: UInt64, observedTime: UInt64) {
 next.phase = pre.phase + 1;
 emit Accrue { accrualId: accrualId, obligationId: "Loan1", periodIndex: periodIndex, observedTime: observedTime };
 ensures post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(110);
 ensures outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(100);
}
}`;
// Straight-line constructor counts derived before the new evaluator exists:
// originate prefix: Let2 + Next4 + Transfer8 + Originate22 =36;
// suffix: ordinary6 + Quantity5 + Amount6 =17; kernel2; total55.
// accrue prefix: Next4 + Accrue6 =10; suffix5+5=10; kernel1; total21.
export const twoOrigins = source.replace(originationPair(),originationPair()+originationPair('"D2"','"O2"','"Loan2"')).replace('post_balance<Cash>("Borrower") == amount<Cash>(110)','post_balance<Cash>("Borrower") == amount<Cash>(210)');
// Second pair adds30 prefix reductions and two kernel actions: total87.
