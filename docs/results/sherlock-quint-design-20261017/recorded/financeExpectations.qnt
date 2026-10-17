module financeExpectations {
 import financeTypes.* from "./financeTypes"
 // Literals from Astra bounded contract and Root's explicit discriminator choices,
 // written before subject wrapper logic. No advance/producer/post constructor used.
 pure val financeInitial:Finance={committed:{reserve:100,liability:40,grossDebit:0,netReceipt:0,residualDuty:40,dutyPresent:true,unrelatedBalance:77,effects:List()},tentative:{reserveDelta:0,liabilityDelta:0,effects:List()},pending:NoPending,laterConsumedWork:0,phase:"initial"}
 pure val financePaid:Finance={committed:{reserve:68,liability:10,grossDebit:32,netReceipt:30,residualDuty:10,dutyPresent:true,unrelatedBalance:77,effects:List("pay30","fee2","discharge30")},tentative:{reserveDelta:0,liabilityDelta:0,effects:List()},pending:NoPending,laterConsumedWork:0,phase:"committed"}
 pure val financeTentative:Finance={committed:{reserve:68,liability:10,grossDebit:32,netReceipt:30,residualDuty:10,dutyPresent:true,unrelatedBalance:77,effects:List("pay30","fee2","discharge30")},tentative:{reserveDelta:-6,liabilityDelta:0,effects:List("pay5","fee1")},pending:AtSite(9),laterConsumedWork:2,phase:"tentative"}
 pure val financeFailed:Finance={committed:{reserve:68,liability:10,grossDebit:32,netReceipt:30,residualDuty:10,dutyPresent:true,unrelatedBalance:77,effects:List("pay30","fee2","discharge30")},tentative:{reserveDelta:0,liabilityDelta:0,effects:List()},pending:NoPending,laterConsumedWork:2,phase:"failed"}
}
