module financeTypes {
  type FinancialSnapshot = {reserve:int,liability:int,grossDebit:int,netReceipt:int,residualDuty:int,dutyPresent:bool,unrelatedBalance:int,effects:List[str]}
  type Tentative = {reserveDelta:int,liabilityDelta:int,effects:List[str]}
  type PendingObservation = NoPending | AtSite(int)
  type Finance = {committed:FinancialSnapshot,tentative:Tentative,pending:PendingObservation,laterConsumedWork:int,phase:str}
}
