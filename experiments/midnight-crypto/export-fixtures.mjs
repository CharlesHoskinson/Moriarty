/** Export actual Moriarty caller observations; no private keys or wallet access. */
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {expand,simulate} from '../../packages/moriarty-beta/src/bridge.ts';
import {analyze} from '../../packages/moriarty-beta/src/frontend.ts';
import {parseAndLowerSource6} from '../moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts';
import {prepareMil4S0} from '../moriarty-language/src/successor/mil4-s0-core-v5.ts';
import {starterSource,starterScenario,repaymentSource,repaymentScenario} from '../../packages/moriarty-beta/src/starter.ts';
const hash=s=>createHash('sha256').update(s).digest('hex');
const stable=v=>Array.isArray(v)?v.map(stable):v&&typeof v==='object'?Object.fromEntries(Object.keys(v).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(k=>[k,stable(v[k])])):v;
const canonical=v=>JSON.stringify(stable(v));
const frame=v=>{const payload=Buffer.from(canonical(v));const length=Buffer.alloc(4);length.writeUInt32BE(payload.length);return Buffer.concat([Buffer.from('moriarty-midnight-auth-experiment/1\0'),length,payload]);};
const leafPaths=(v,p='')=>v&&typeof v==='object'&&Object.keys(v).length?Object.entries(v).flatMap(([k,x])=>leafPaths(x,p+'/'+k.replaceAll('~','~0').replaceAll('/','~1'))):[p];
const fixtures=[];
function emit(name,source,action,scenario,expected){
 const scenarioText=JSON.stringify(scenario);const expanded=expand(source,action,scenarioText);assert.equal(expanded.status,'Expanded');
 const lowered=parseAndLowerSource6(expanded.source6);const result=simulate(source,action,scenarioText);assert.equal(result.status,'PreparedUnqualified');
 const candidate=result.result.candidate;assert.equal(candidate.candidatePost.balances[0].amount,expected.payer);assert.equal(candidate.effects[0].amount,expected.debit);
 assert.equal(candidate.candidatePost.workRemaining,'9');assert.equal(candidate.candidatePost.workSpent,'1');assert.equal(candidate.candidatePost.allowances[0].spent,expected.debit);
 if(expected.principal!==undefined){const o=candidate.candidatePost.obligations[0];assert.equal(o.principal,expected.principal);assert.equal(o.accrued,expected.accrued);assert.equal(o.status,expected.status);}
 const repayment=expected.principal!==undefined;const payer=repayment?'Payer':'Owner';const recipient=repayment?'Creditor':'Recipient';const replay=JSON.stringify(['Midnight',payer,'n1']);
 const expectedEffects=[{kind:'Debit',account:payer,asset:'A',amount:expected.debit},{kind:'Credit',account:recipient,asset:'A',amount:repayment?expected.debit:'1000'}];
 const obligations=repayment?[{id:'Loan',debtor:'Payer',creditor:'Creditor',asset:'A',principal:expected.principal,accrued:expected.accrued,outstanding:(BigInt(expected.principal)+BigInt(expected.accrued)).toString(),status:expected.status}]:[];
 if(repayment){const o=obligations[0];expectedEffects.push({kind:'SetObligation',id:'Loan',principal:o.principal,accrued:o.accrued,outstanding:o.outstanding,status:o.status});}
 else if(expected.debit==='1010'){expectedEffects.push({kind:'Credit',account:'Fee',asset:'A',amount:'10'});}
 expectedEffects.push({kind:'UseAllowance',owner:payer,amount:expected.debit},{kind:'UseReplay',key:replay},{kind:'AdvanceHead',predecessor:'h0',successor:'h1'});
 const expectedPost={core:'moriarty-core/5',domain:'Midnight',asset:'A',head:'h1',round:'1',workRemaining:'9',workSpent:'1',balances:[{account:payer,amount:expected.payer},{account:recipient,amount:repayment?expected.debit:'1000'},...repayment?[]:[{account:'Fee',amount:expected.debit==='1010'?'10':'0'}]],allowances:[{owner:payer,remaining:expected.payer,spent:expected.debit}],obligations,consumedReplay:[replay]};
 assert.deepEqual(candidate.effects,expectedEffects);assert.deepEqual(candidate.candidatePost,expectedPost);
 const analysis=analyze(source);const domain=analysis.declarations.find(d=>d.kind==='domain').value;const asset=analysis.declarations.find(d=>d.kind==='asset').value;
 const valueText=v=>v.tag==='string'?v.value:v.tag==='scalar'?v.value:null;
 const identity={agreement_id:lowered.ast.programId,selected_action_id:lowered.ast.selected.actionId,asset_scale:lowered.ast.settlement.scale,asset_representation:valueText(asset.fields.representation),chain:valueText(domain.fields.chain),network:valueText(domain.fields.network),source_sha256:hash(source),source6_sha256:hash(expanded.source6),policy_digest:lowered.ast.selected.policyDigest};
 const intentStatement={profile:'moriarty-midnight-auth-experiment/1',kind:'OwnerIntent',identity,domain:lowered.ast.domain,settlement:lowered.ast.settlement,intent:lowered.ast.intent,selected:lowered.ast.selected,lowered_intent:lowered.intent,replay_tuple:[lowered.intent.domain,lowered.intent.signer,lowered.intent.nonce]};
 const statement={profile:'moriarty-midnight-auth-experiment/1',kind:'CandidateClaim',authorization:intentStatement,ast:lowered.ast,lowered:{intent:lowered.intent,state:lowered.state,effects:lowered.submittedEffects,post_head:lowered.proposedPostHead},candidate};
 const negatives=[];
 for(const [n,change,code] of [
  ['expired',x=>x.state.round='11','S0_INTENT_SCOPE'],
  ['consumed-replay',x=>x.state.consumedReplay=[JSON.stringify([x.intent.domain,x.intent.signer,x.intent.nonce])],'S0_HISTORY_REPLAY'],
  ['wrong-effects',x=>x.submittedEffects[0].amount='1','S0_EFFECT_MISMATCH'],
  ['no-work',x=>x.state.workRemaining='0','S0_AUTH_SCOPE'],
  ['over-cap',x=>x.intent.grossCap='0','S0_INTENT_SCOPE'],
 ]){
  const x=structuredClone(lowered);change(x);const out=prepareMil4S0(x.state,x.intent,x.submittedEffects,x.proposedPostHead);
  assert.equal(out.status,'Rejected',n);assert.equal(out.code,code,n);assert.equal(out.publishedPost,null);assert.equal(out.publishedEffects,null);
  negatives.push({name:n,statement:{...statement,kind:'InvalidCandidateClaim',lowered:{intent:x.intent,state:x.state,effects:x.submittedEffects,post_head:x.proposedPostHead}},core_result:out});
 }
 const bytes=frame(statement);fixtures.push({name,source,action,scenario,source6:expanded.source6,intent_statement:intentStatement,statement,golden_frame_hex:bytes.toString('hex'),golden_frame_sha256:hash(bytes),expected,expected_effects:expectedEffects,expected_post:expectedPost,mutation_leaf_paths:leafPaths(JSON.parse(JSON.stringify(statement))),negative_candidates:negatives});
}
emit('transfer-fee',starterSource,'pay',starterScenario,{payer:'8990',debit:'1010'});
emit('transfer-zero-fee',starterSource.replace('const fee = 0.10 USD;','const fee = 0.00 USD;'),'pay',starterScenario,{payer:'9000',debit:'1000'});
emit('repay-interest-only',repaymentSource.replace('30.00 USD','5.00 USD'),'repay_loan',repaymentScenario,{payer:'199500',debit:'500',principal:'100000',accrued:'500',status:'Outstanding'});
emit('repay-partial',repaymentSource,'repay_loan',repaymentScenario,{payer:'197000',debit:'3000',principal:'98000',accrued:'0',status:'Outstanding'});
emit('repay-full',repaymentSource.replace('30.00 USD','1010.00 USD'),'repay_loan',repaymentScenario,{payer:'99000',debit:'101000',principal:'0',accrued:'0',status:'Settled'});
const unicodeStatement={'\u{10000}':'astral','\ue000':'bmp',text:'é😀\n"\\',array:[null,true,'0']};
const unicodeFrame=frame(unicodeStatement);
writeFileSync(new URL('./fixtures/moriarty.json',import.meta.url),JSON.stringify({profile:'moriarty-midnight-fixtures/1',codec_vectors:[{statement:unicodeStatement,golden_frame_hex:unicodeFrame.toString('hex'),golden_frame_sha256:hash(unicodeFrame)}],fixtures},null,2)+'\n');
console.log(JSON.stringify({fixtures:fixtures.length,core_negative_cases:fixtures.reduce((n,f)=>n+f.negative_candidates.length,0),qualification:'PreparedUnqualified; ephemeral test signatures do not authenticate ledger provenance'}));
