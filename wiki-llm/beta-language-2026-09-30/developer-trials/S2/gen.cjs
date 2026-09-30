const fs=require("fs");
const A="USDCanonical";
const eff=(pay,ob,nonce,st)=>[
 {kind:"Debit",account:"Alice",asset:A,amount:pay},
 {kind:"Credit",account:"Bob",asset:A,amount:pay},
 {kind:"SetObligation",id:"Loan1",principal:ob[0],accrued:ob[1],outstanding:ob[2],status:st},
 {kind:"UseAllowance",owner:"Alice",amount:pay},
 {kind:"UseReplay",key:JSON.stringify(["Midnight","Alice",nonce])},
 {kind:"AdvanceHead",predecessor:"h0",successor:"h1"}];
const post=(alice,bob,spent,ob,nonce,st)=>({core:"moriarty-core/5",domain:"Midnight",asset:A,head:"h1",round:"1",workRemaining:"9",workSpent:"1",
 balances:[{account:"Alice",amount:alice},{account:"Bob",amount:bob}],
 allowances:[{owner:"Alice",remaining:alice,spent}],
 obligations:[{id:"Loan1",debtor:"Alice",creditor:"Bob",asset:A,principal:ob[0],accrued:ob[1],outstanding:ob[2],status:st}],
 consumedReplay:[JSON.stringify(["Midnight","Alice",nonce])]});
// hand arithmetic: loan 50000 principal + 1250 accrued = 51250; Alice 80000, Bob 5000
const good=[
 ["partial 100.00 clears accrued then principal","pay_partial","open",{status:"PreparedUnqualified",effects:eff("10000",["41250","0","41250"],"rp1","Outstanding"),post:post("70000","15000","10000",["41250","0","41250"],"rp1","Outstanding")}],
 ["interest-only 5.00 leaves 7.50 accrued","pay_interest_only","open",{status:"PreparedUnqualified",effects:eff("500",["50000","750","50750"],"rp3","Outstanding"),post:post("79500","5500","500",["50000","750","50750"],"rp3","Outstanding")}],
 ["full 512.50 settles","pay_full","open",{status:"PreparedUnqualified",effects:eff("51250",["0","0","0"],"rp2","Settled"),post:post("28750","56250","51250",["0","0","0"],"rp2","Settled")}],
 ["partial at last valid round 10","pay_partial","last_round",{status:"PreparedUnqualified",effects:eff("10000",["41250","0","41250"],"rp1","Outstanding")}],
 ["partial at first valid round 0","pay_partial","round0",{status:"PreparedUnqualified"}],
 ["expired round 11 rejected","pay_partial","expired",{status:"CoreRejected",code:"S0_INTENT_SCOPE"}],
 ["full payoff with insufficient funds","pay_full","short_funds",{status:"CoreRejected",code:"S0_EFFECT_RANGE"}],
];
const w=(dir,cases)=>fs.writeFileSync(dir+"/mori.tests.json",JSON.stringify({profile:"moriarty-beta-tests/1",cases:cases.map(c=>({name:c[0],source:c[1],action:c[2],scenario:"scenarios/"+c[3]+".json",expect:c[4]}))},null,1));
w(".",good.map(g=>[g[0],"loan.mori",g[1],g[2],g[3]]));
const bad=[
 ["overpay by one cent 512.51","overpay_by_one_cent.mori","pay_full","open",{status:"CoreRejected",code:"S0_EFFECT_RANGE"}],
 ["gross_cap below amount","cap_below_amount.mori","pay_full","open",{status:"CoreRejected",code:"S0_INTENT_SCOPE"}],
 ["nonzero fee_cap on repay","nonzero_fee_cap.mori","pay_partial","open",{status:"CoreRejected",code:"S0_INTENT_SCOPE"}],
 ["wrong signer","wrong_signer.mori","pay_partial","open",{status:"AuthoringRejected",code:"BETA_SIGNER"}],
 ["bare amount without asset","bare_amount.mori","pay_partial","open",{status:"AuthoringRejected",code:"BETA_DECIMAL_SCALAR"}],
 ["three decimals on scale 2","too_precise.mori","pay_full","open",{status:"AuthoringRejected",code:"BETA_PRECISION"}],
];
w("invalid",bad);
