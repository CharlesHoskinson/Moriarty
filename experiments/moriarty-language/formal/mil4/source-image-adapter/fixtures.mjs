/** Local source fixtures. Claims and authenticated blocks are presentation only. */
export function transfer(overrides = {}) {
  const p = { agreement: 'AgreementA', domain: 'D', asset: 'A', scale: '0',
    sourceHash: 'claim-source', policyDigest: 'claim-policy', key: 'key1',
    value: '10', fee: '1', balance: '100', postHead: 'h1', ...overrides };
  const gross = (BigInt(p.value) + BigInt(p.fee)).toString();
  return `profile "moriarty-financial-agreement-source/6";
agreement ${p.agreement} {
 domain ${p.domain}; settlement ${p.asset} scale ${p.scale};
 selected TransferLiteralFee source_hash "${p.sourceHash}" digest "${p.policyDigest}";
 intent {
  signer Owner key "${p.key}"; nonce "n1"; pre_head "h0"; valid 0..10;
  gross_cap ${gross}; fee_cap ${p.fee}; net_floor ${p.value}; failure success_only;
  signed_action transfer from Owner to Recipient fee_to Fee value ${p.value} fee ${p.fee};
  observations empty; disclosures empty; retained_effects empty; retained_duties empty;
  delegation none; recovery none;
 }
 authenticated {
  head "h0"; predecessor "genesis"; round 1;
  balance Owner ${p.balance}; balance Recipient 0; balance Fee 0;
  allowance Owner remaining 100 spent 0; replay unused; work_remaining 10; work_spent 0;
 }
 submit transfer from Owner to Recipient fee_to Fee value ${p.value} fee ${p.fee};
 effects { debit Owner ${gross}; credit Recipient ${p.value};
  ${p.fee === '0' ? '' : `credit Fee ${p.fee};`}
  use_allowance Owner ${gross}; use_replay "n1"; advance_head "h0" "${p.postHead}";
 }
 post_head "${p.postHead}";
}`;
}

export function repay(overrides = {}) {
  const p = { agreement: 'AgreementA', domain: 'D', asset: 'A', scale: '0',
    sourceHash: 'claim-source', policyDigest: 'claim-policy', amount: '30',
    principal: '1000', accrued: '10', outstanding: '1010', postPrincipal: '980',
    postOutstanding: '980', postStatus: 'outstanding', ...overrides };
  return `profile "moriarty-financial-agreement-source/6";
agreement ${p.agreement} {
 domain ${p.domain}; settlement ${p.asset} scale ${p.scale};
 selected RepayAccrualFirst source_hash "${p.sourceHash}" digest "${p.policyDigest}";
 intent {
  signer Payer key "key1"; nonce "n1"; pre_head "h0"; valid 0..10;
  gross_cap ${p.amount}; fee_cap 0; net_floor 0; failure success_only;
  signed_action repay obligation Loan payer Payer amount ${p.amount} conversion identity;
  observations empty; disclosures empty; retained_effects empty; retained_duties empty;
  delegation none; recovery none;
 }
 authenticated {
  head "h0"; predecessor "genesis"; round 1;
  balance Payer 2000; balance Creditor 0;
  allowance Payer remaining 2000 spent 0;
  obligation Loan { debtor Payer; creditor Creditor; asset ${p.asset};
   principal ${p.principal}; accrued ${p.accrued}; outstanding ${p.outstanding}; status outstanding; }
  replay unused; work_remaining 10; work_spent 0;
 }
 submit repay obligation Loan payer Payer amount ${p.amount} conversion identity;
 effects { debit Payer ${p.amount}; credit Creditor ${p.amount};
  set_obligation Loan principal ${p.postPrincipal} accrued 0 outstanding ${p.postOutstanding} status ${p.postStatus};
  use_allowance Payer ${p.amount}; use_replay "n1"; advance_head "h0" "h1";
 }
 post_head "h1";
}`;
}
