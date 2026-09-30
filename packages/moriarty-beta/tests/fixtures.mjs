export const transferSource = `profile "moriarty-beta/1";
agreement Invoice {
 domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
 account Buyer = { domain: Preview, id: "Owner" };
 account Seller = { domain: Preview, id: "Recipient" };
 account Treasury = { domain: Preview, id: "Fee" };
 asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical", symbol: "USD" };
 const price: Qty<USD> = 10.00 USD;
 const fee = 0.10 USD;
 intent Payment = {
  domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "n1", pre_head: "h0",
  valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: price + fee, fee_cap: fee, net_floor: price,
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee),
  source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly,
  observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None,
 };
 action pay uses Payment;
}`;
export const repaySource = `profile "moriarty-beta/1";
agreement LoanAgreement {
 domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
 account Payer = { domain: Preview, id: "Payer" };
 account Creditor = { domain: Preview, id: "Creditor" };
 asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical" };
 obligation Debt = { domain: Preview, asset: USD, id: "Loan" };
 const amount = 30.00 USD;
 intent Repayment = { domain: Preview, asset: USD, signer: Payer, key: "key1", nonce: "n1", pre_head: "h0",
 valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: amount, fee_cap: 0.00 USD, net_floor: 0.00 USD,
 operation: repay(obligation: Debt, payer: Payer, amount: amount), source_hash: "src1", policy_digest: "policy1",
 failure: SuccessOnly, observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None };
 action repay_loan uses Repayment;
}`;
export const transferScenario = {profile:'moriarty-local-scenario/1',kind:'local-stipulation',domain:'Midnight',asset:'A',head:'h0',predecessor:'genesis',round:'1',balances:[{account:'Owner',amount:'10000'},{account:'Recipient',amount:'0'},{account:'Fee',amount:'0'}],allowance:{owner:'Owner',remaining:'10000',spent:'0'},replay:'unused',work_remaining:'10',work_spent:'0',post_head:'h1'};
export const repayScenario = {...transferScenario,balances:[{account:'Payer',amount:'200000'},{account:'Creditor',amount:'0'}],allowance:{owner:'Payer',remaining:'200000',spent:'0'},obligation:{id:'Loan',debtor:'Payer',creditor:'Creditor',asset:'A',principal:'100000',accrued:'1000',outstanding:'101000',status:'Outstanding'}};
// Independent explicit Source/6 oracle; this does not import the beta elaborator.
export const explicitTransfer = `profile "moriarty-financial-agreement-source/6";
agreement Invoice {
 domain Midnight;
 settlement A scale 2;
 selected TransferLiteralFee source_hash "src1" digest "policy1";
 intent {
 signer Owner key "key1";
 nonce "n1";
 pre_head "h0";
 valid 0..10;
 gross_cap 1010;
 fee_cap 10;
 net_floor 1000;
 failure success_only;
 signed_action transfer from Owner to Recipient fee_to Fee value 1000 fee 10;
 observations empty;
 disclosures empty;
 retained_effects empty;
 retained_duties empty;
 delegation none;
 recovery none;
 }
 authenticated {
 head "h0";
 predecessor "genesis";
 round 1;
 balance Owner 10000;
 balance Recipient 0;
 balance Fee 0;
 allowance Owner remaining 10000 spent 0;
 replay unused;
 work_remaining 10;
 work_spent 0;
 }
 submit transfer from Owner to Recipient fee_to Fee value 1000 fee 10;
 effects {
 debit Owner 1010;
 credit Recipient 1000;
 credit Fee 10;
 use_allowance Owner 1010;
 use_replay "n1";
 advance_head "h0" "h1";
 }
 post_head "h1";
}`;
export const explicitRepay = `profile "moriarty-financial-agreement-source/6";
agreement LoanAgreement {
 domain Midnight;
 settlement A scale 2;
 selected RepayAccrualFirst source_hash "src1" digest "policy1";
 intent {
 signer Payer key "key1";
 nonce "n1";
 pre_head "h0";
 valid 0..10;
 gross_cap 3000;
 fee_cap 0;
 net_floor 0;
 failure success_only;
 signed_action repay obligation Loan payer Payer amount 3000 conversion identity;
 observations empty;
 disclosures empty;
 retained_effects empty;
 retained_duties empty;
 delegation none;
 recovery none;
 }
 authenticated {
 head "h0";
 predecessor "genesis";
 round 1;
 balance Payer 200000;
 balance Creditor 0;
 allowance Payer remaining 200000 spent 0;
 obligation Loan {
 debtor Payer;
 creditor Creditor;
 asset A;
 principal 100000;
 accrued 1000;
 outstanding 101000;
 status outstanding;
 }
 replay unused;
 work_remaining 10;
 work_spent 0;
 }
 submit repay obligation Loan payer Payer amount 3000 conversion identity;
 effects {
 debit Payer 3000;
 credit Creditor 3000;
 set_obligation Loan principal 98000 accrued 0 outstanding 98000 status outstanding;
 use_allowance Payer 3000;
 use_replay "n1";
 advance_head "h0" "h1";
 }
 post_head "h1";
}`;
