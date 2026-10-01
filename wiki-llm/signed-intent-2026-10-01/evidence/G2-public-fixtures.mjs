import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {starterScenario,repaymentScenario} from '/home/charl/Moriarty/.worktrees/moriarty-beta-20260930/packages/moriarty-beta/src/starter.ts';
const root='/home/charl/Moriarty/.worktrees/moriarty-beta-20260930';
const binary='/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto';
const helper='/home/charl/research/moriarty-crypto-2026-09-30/target/debug/examples/intent-fixture';
const vectors=JSON.parse(readFileSync(root+'/experiments/midnight-crypto/fixtures/intent-vectors.json','utf8'));
const receipts=[];
for(const [name,fixture,scheme,framing] of [['transfer-schnorr-raw','transfer-fee','schnorr_bip340','raw'],['transfer-ecdsa-wallet','transfer-fee','ecdsa_secp256k1_sha256','midnight-sign-data'],['repay-ecdsa-raw','repay-partial','ecdsa_secp256k1_sha256','raw']]){
 const f=vectors.fixtures.find(x=>x.name===fixture),repay=f.scenario==='repayment',scenario=repay?repaymentScenario:starterScenario,e=f.expected_economics;
 const draft={...f.draft,sourceSha256:createHash('sha256').update(f.source).digest('hex')};
 const artifact=JSON.parse(execFileSync(helper,[scheme,framing],{input:JSON.stringify(draft),encoding:'utf8'}));
 const dir=root+'/packages/moriarty-beta/examples/signed-intent/'+name;mkdirSync(dir,{recursive:true});
 writeFileSync(dir+'/program.mori',f.source);writeFileSync(dir+'/scenario.json',JSON.stringify(scenario,null,2)+'\n');writeFileSync(dir+'/signature.json',JSON.stringify(artifact,null,2)+'\n');
 const payer=repay?'Payer':'Owner',recipient=repay?'Creditor':'Recipient',replay=JSON.stringify(['Midnight',payer,'n1']);
 const effects=[{kind:'Debit',account:payer,asset:'A',amount:e.debit},{kind:'Credit',account:recipient,asset:'A',amount:repay?e.debit:e.credits.Recipient}];
 let obligations=[];
 if(repay){const after=e.obligationAfter,o={id:'Loan',debtor:'Payer',creditor:'Creditor',asset:'A',principal:after.principal,accrued:after.accrued,outstanding:(BigInt(after.principal)+BigInt(after.accrued)).toString(),status:after.status};obligations=[o];effects.push({kind:'SetObligation',id:o.id,principal:o.principal,accrued:o.accrued,outstanding:o.outstanding,status:o.status});}else effects.push({kind:'Credit',account:'Fee',asset:'A',amount:e.credits.Fee});
 effects.push({kind:'UseAllowance',owner:payer,amount:e.debit},{kind:'UseReplay',key:replay},{kind:'AdvanceHead',predecessor:'h0',successor:'h1'});
 const post={core:'moriarty-core/5',domain:'Midnight',asset:'A',head:'h1',round:'1',workRemaining:'9',workSpent:'1',balances:[{account:payer,amount:e.payerAfter},{account:recipient,amount:repay?e.debit:e.credits.Recipient},...repay?[]:[{account:'Fee',amount:e.credits.Fee}]],allowances:[{owner:payer,remaining:e.payerAfter,spent:e.debit}],obligations,consumedReplay:[replay]};
 writeFileSync(dir+'/mori.tests.json',JSON.stringify({profile:'moriarty-beta-tests/1',cases:[{name:'independent-complete-economics',source:'program.mori',action:f.draft.actionName,scenario:'scenario.json',expect:{status:'PreparedUnqualified',effects,post}}]},null,2)+'\n');
 const args=['verify-intent',dir+'/program.mori','--action',f.draft.actionName,'--scenario',dir+'/scenario.json','--signature',dir+'/signature.json','--crypto-binary',binary];
 const verified=JSON.parse(execFileSync(process.execPath,[root+'/packages/moriarty-beta/dist/cli.js',...args],{encoding:'utf8'}));assert.equal(verified.status,'SignedPreparedUnqualified');assert.equal(verified.signature.signature_valid,true);assert.deepEqual(verified.local.result.candidate.effects,effects);assert.deepEqual(verified.local.result.candidate.candidatePost,post);
 const tested=JSON.parse(execFileSync(process.execPath,[root+'/packages/moriarty-beta/dist/cli.js','test',dir],{encoding:'utf8'}));assert.equal(tested.status,'TestsPassed');
 receipts.push({name,fixture,scheme,framing,status:verified.status,signature_valid:true,ledger_accepted:verified.ledger_accepted,source_sha256:artifact.statement.sourceSha256,frame_sha256:verified.signature.frame_sha256,independent_effects_and_post_match:true,test_status:tested.status});
}
writeFileSync('/home/charl/research/moriarty-signed-intent-2026-10-01/G2-public-fixtures-receipt.json',JSON.stringify({status:'PublicFixturesVerified',binary,helper,receipts,private_key_persisted:false},null,2)+'\n');console.log(JSON.stringify(receipts));
