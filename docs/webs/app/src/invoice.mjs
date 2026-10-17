/** Shared arithmetic for the worksheet and invoice role statements. */
export function invoiceAmounts(amount, openingCash) {
  if (!Number.isInteger(amount) || amount < 1 || amount > 99) throw new RangeError('Partial payment must be 1–99 CASH.');
  if (![110,112].includes(openingCash)) throw new RangeError('Unknown invoice example.');
  const fee=2;
  const debit=amount+fee;
  const invoice=100-amount;
  const reserve=10;
  const duties=invoice+reserve;
  const cash=openingCash-debit;
  return {amount, openingCash, fee, debit, invoice, reserve, invoiceDuty:invoice, reserveDuty:reserve, duties, cash, gap:duties-cash};
}
