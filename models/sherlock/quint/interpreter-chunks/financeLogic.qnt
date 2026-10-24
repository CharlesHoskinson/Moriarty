module financeLogic {
 import financeTypes.* from "./financeTypes"
 // Model-only source-boundary fixture, separate from A/B semantic events.
 pure val initialFinance:Finance={committed:{reserve:100,liability:40,grossDebit:0,netReceipt:0,residualDuty:40,dutyPresent:true,unrelatedBalance:77,effects:List()},tentative:{reserveDelta:0,liabilityDelta:0,effects:List()},pending:NoPending,laterConsumedWork:0,phase:"initial"}
 pure def paidPrefix(f:Finance):Finance={...f,committed:{...f.committed,reserve:f.committed.reserve-30-2,liability:f.committed.liability-30,grossDebit:f.committed.grossDebit+30+2,netReceipt:f.committed.netReceipt+30,residualDuty:f.committed.residualDuty-30,effects:f.committed.effects.concat(List("pay30","fee2","discharge30"))},phase:"committed"}
 pure def nextTentative(f:Finance):Finance={...f,tentative:{reserveDelta:-6,liabilityDelta:0,effects:List("pay5","fee1")},pending:AtSite(9),laterConsumedWork:f.laterConsumedWork+2,phase:"tentative"}
 pure def failTentative(f:Finance):Finance={...f,tentative:{reserveDelta:0,liabilityDelta:0,effects:List()},pending:NoPending,phase:"failed"}
 pure def financeInvariant(f:Finance):bool=and{f.committed.unrelatedBalance==77,f.committed.dutyPresent,f.committed.residualDuty==f.committed.liability,f.laterConsumedWork>=0,if(f.phase=="initial")f.committed.reserve==100 else and{f.committed.reserve==68,f.committed.liability==10,f.committed.grossDebit==32,f.committed.netReceipt==30,f.committed.effects==List("pay30","fee2","discharge30")}}
}
