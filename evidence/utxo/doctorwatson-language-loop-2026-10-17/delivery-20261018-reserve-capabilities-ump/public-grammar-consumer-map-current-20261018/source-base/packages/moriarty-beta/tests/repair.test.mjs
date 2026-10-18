import test from 'node:test';import assert from 'node:assert/strict';
import {inspect,check} from '../src/index.ts';
import {expand,simulate} from '../src/bridge.ts';
import {transferSource,transferScenario} from './fixtures.mjs';
test('representable invalid nonce reaches original Core Intent judgment',()=>{
 const source=transferSource.replace('nonce: "n1"','nonce: "n\\n"');
 const r=simulate(source,'pay',JSON.stringify(transferScenario));
 assert.equal(r.status,'CoreRejected',JSON.stringify(r));assert.equal(r.result.rejection.judgment,'intent');assert.equal(r.result.rejection.code,'S0_INTENT_SCOPE');
});
test('inspect exposes actual request, signed caps and validity',()=>{
 const r=inspect(transferSource);assert.ok(r.intents?.length>0);
 const i=r.intents.find(x=>x.name==='Payment');assert.equal(i.terms.gross_cap.atoms,'1010');assert.equal(i.terms.operation.name,'transfer');assert.equal(i.terms.valid.args.from.value,'0');
});
test('operation origins include every defining constant in dependency closure',()=>{
 const source=transferSource.replace('const price: Qty<USD> = 10.00 USD;','const original = 5.00 USD; const price: Qty<USD> = original * 2;');
 const r=expand(source,'pay',JSON.stringify(transferScenario));assert.equal(r.status,'Expanded');
 const row=r.fieldMap.find(x=>x.field==='intent.signed_action');const visited=new Set();const spans=[];
 function visit(id){if(visited.has(id))return;visited.add(id);const o=r.origins.find(x=>x.id===id);if(o.span)spans.push(Buffer.from(source).subarray(o.span.start,o.span.end).toString());for(const dep of o.dependencies??[])visit(dep);}
 for(const id of row.origins)visit(id);
 assert.ok(spans.some(x=>x.includes('original = 5.00 USD')),JSON.stringify(spans));
 assert.ok(spans.some(x=>x.includes('price: Qty<USD> = original * 2')),JSON.stringify(spans));
});

test('escaped transport strings preserve direct Core first failure and unpublished outputs',async()=>{
 const {explicitTransfer}=await import('./fixtures.mjs');
 const {prepareSource6S0Unqualified}=await import('../../../experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts');
 const cases=[
  [transferSource.replace('nonce: "n1"','nonce: "n\\n"'),{...transferScenario,head:'stale',work_remaining:'0'},explicitTransfer.replaceAll('"n1"','"n\\n"').replace('\n head "h0";','\n head "stale";').replace('work_remaining 10;','work_remaining 0;')],
  [transferSource.replace('pre_head: "h0"','pre_head: "h\\t"'),{...transferScenario,work_remaining:'0'},explicitTransfer.replace('pre_head "h0";','pre_head "h\\t";').replace('advance_head "h0"','advance_head "h\\t"').replace('work_remaining 10;','work_remaining 0;')],
  [transferSource,{...transferScenario,head:'h\n',round:'11'},explicitTransfer.replace('\n head "h0";','\n head "h\\n";').replace('round 1;','round 11;')],
  [transferSource,{...transferScenario,post_head:'h\u007f',round:'11'},explicitTransfer.replaceAll('"h1"','"h\\u007f"').replace('round 1;','round 11;')],
  [transferSource,{...transferScenario,predecessor:'g\n'},explicitTransfer.replace('"genesis"','"g\\n"')],
 ];
 for(const [source,scenario,oracle] of cases){const r=simulate(source,'pay',JSON.stringify(scenario));const expected=prepareSource6S0Unqualified(oracle);assert.equal(r.status,expected.status,JSON.stringify(r));assert.deepEqual(r.result,expected);if(r.status==='CoreRejected'){assert.equal(r.result.rejection.publishedPost,null);assert.equal(r.result.rejection.publishedEffects,null);}}
 const effects=[{kind:'Debit',account:'Owner',asset:'A',amount:'1010'},{kind:'Credit',account:'Recipient',asset:'A',amount:'1000'},{kind:'Credit',account:'Fee',asset:'A',amount:'10'},{kind:'UseAllowance',owner:'Owner',amount:'1010'},{kind:'UseReplay',key:'bad\n'},{kind:'AdvanceHead',predecessor:'h0',successor:'h1'}];
 for(const round of ['1','11']){const r=simulate(transferSource,'pay',JSON.stringify({...transferScenario,round,candidate_effects:effects}));const expected=prepareSource6S0Unqualified(explicitTransfer.replace('use_replay "n1"','use_replay "bad\\n"').replace('round 1;',`round ${round};`));assert.deepEqual(r.result,expected);}
});

test('inspection summarizes transfer and repay terms with economic claims and no declaration DAG',async()=>{
 const {repaySource}=await import('./fixtures.mjs');
 const p=inspect(transferSource).intents.find(i=>i.name==='Payment').terms;
 assert.equal(p.operation.args.value.atoms,'1000');assert.equal(p.operation.args.fee.atoms,'10');assert.equal(p.operation.args.from.claims.id,'Owner');assert.equal(p.operation.args.from.claims.domain.id,'Midnight');
 assert.equal(p.asset.claims.scale,'2');assert.equal(p.asset.claims.representation,'canonical');assert.equal(p.signer.claims.id,'Owner');assert.equal(p.key.value,'key1');assert.equal(p.nonce.value,'n1');assert.equal(p.pre_head.value,'h0');assert.equal(p.source_hash.value,'src1');assert.equal(p.policy_digest.value,'policy1');assert.equal(p.fee_cap.atoms,'10');assert.equal(p.net_floor.atoms,'1000');assert.equal(p.valid.args.to.value,'10');assert.equal(p.failure.name,'SuccessOnly');assert.deepEqual(p.observations.items,[]);assert.equal(p.delegation.name,'None');
 const r=inspect(repaySource).intents.find(i=>i.name==='Repayment').terms;assert.equal(r.operation.name,'repay');assert.equal(r.operation.args.amount.atoms,'3000');assert.equal(r.operation.args.obligation.claims.id,'Loan');assert.equal(r.operation.args.obligation.claims.asset.id,'A');assert.equal(r.operation.args.payer.claims.id,'Payer');assert.equal(r.gross_cap.atoms,'3000');assert.equal(r.fee_cap.atoms,'0');assert.equal(r.net_floor.atoms,'0');assert.equal(r.operation.args.obligation.authenticated,false);
 const shared=transferSource.replace('const price: Qty<USD> = 10.00 USD;','const original = 5.00 USD; const price: Qty<USD> = original * 2;');assert.equal(inspect(shared).intents[0].terms.operation.args.value.atoms,'1000');assert.ok(Buffer.byteLength(JSON.stringify(inspect(shared)))<524288);
});

function closure(r,source,field){const lookup=new Map(r.origins.map(o=>[o.id,o]));const seen=new Set(),spans=[];const visit=id=>{if(seen.has(id))return;seen.add(id);const o=lookup.get(id);assert.ok(o,`missing origin ${id}`);if(o.span)spans.push(Buffer.from(source).subarray(o.span.start,o.span.end).toString());for(const dep of o.dependencies??[])visit(dep);};for(const id of r.fieldMap.find(f=>f.field===field).origins)visit(id);return spans;}
test('field roots preserve actual uses, arithmetic definitions and economic declarations',()=>{
 const source=transferSource.replace('const price: Qty<USD> = 10.00 USD;','const original: Qty<USD> = 5.00 USD; const intermediate = original * 2; const price: Qty<USD> = intermediate;');
 const r=expand(source,'pay',JSON.stringify(transferScenario));assert.equal(r.status,'Expanded',JSON.stringify(r));
 for(const field of ['intent.signed_action','intent.gross_cap','intent.net_floor','effects.0','effects.1']){const spans=closure(r,source,field);assert.ok(spans.includes('5.00 USD'),`${field}: ${JSON.stringify(spans)}`);assert.ok(spans.some(x=>x.startsWith('const original:')),field);assert.ok(spans.some(x=>x.startsWith('const intermediate')),field);assert.ok(spans.some(x=>x.startsWith('const price:')),field);assert.ok(spans.some(x=>x.startsWith('asset USD')),field);}
 for(const field of ['intent.signed_action','effects.0']){const spans=closure(r,source,field);for(const name of ['Buyer','Seller','Treasury'])assert.ok(spans.some(x=>x.startsWith(`account ${name}`)),`${field}:${name}`);}
 const root=r.fieldMap.find(f=>f.field==='intent.net_floor');assert.ok(root.origins.some(id=>{const o=r.origins.find(o=>o.id===id);return o.span&&Buffer.from(source).subarray(o.span.start,o.span.end).toString()==='price';})||closure(r,source,'intent.net_floor').includes('price'));
 // This use is distinct from the same shared Value's other references.
 const use=source.indexOf('net_floor: price')+'net_floor: '.length;const found=new Set();const walk=id=>{if(found.has(id))return;found.add(id);for(const dep of r.origins.find(o=>o.id===id).dependencies??[])walk(dep);};root.origins.forEach(walk);assert.ok(r.origins.some(o=>found.has(o.id)&&o.span?.start===use&&o.span?.end===use+5));
 assert.ok(r.origins.length<=2048);assert.ok(Buffer.byteLength(JSON.stringify(r))<=524288);
});
test('shared declaration origins stay linear and origin exhaustion publishes no artifact',()=>{
 const chain=Array.from({length:180},(_,i)=>`const shared${i} = ${i?`shared${i-1} + shared${i-1}`:'0.00 USD'};`).join('\n');
 const source=transferSource.replace('const price: Qty<USD> = 10.00 USD;',`${chain}\nconst price: Qty<USD> = shared179 + 10.00 USD;`);
 const r=expand(source,'pay',JSON.stringify(transferScenario));assert.equal(r.status,'Expanded',JSON.stringify(r));assert.ok(r.origins.length<180*8+200);assert.ok(closure(r,source,'intent.signed_action').includes('0.00 USD'));
 const many=Array.from({length:80},(_,i)=>`const ref${i} = 0;`).join('\n');const expression=Array.from({length:1800},(_,i)=>`ref${i%80}`).join(' + ');
 const bounded=transferSource.replace('const price: Qty<USD> = 10.00 USD;',`${many}\nconst price: Qty<USD> = atoms(asset: USD, value: ${expression} + 1000);`);
 const fail=expand(bounded,'pay',JSON.stringify(transferScenario));assert.equal(fail.status,'FormationRejected',fail.status);assert.equal(fail.diagnostics[0].code,'BETA_ORIGIN_BOUND');assert.equal(fail.publishedPost,null);assert.equal(fail.publishedEffects,null);for(const k of ['source6','origins','fieldMap'])assert.equal(Object.hasOwn(fail,k),false);
});

test('inspection bounds shared nested values before serialization and returns no partial signed scope',()=>{
 const declarations=Array.from({length:20},(_,i)=>`const shared${i} = [${i?`shared${i-1},shared${i-1}`:'"claim"'}];`).join('\n');
 const source=`profile "moriarty-beta/1"; agreement Bounded { policy P = {}; ${declarations} intent View = { operation: governance.execute(policy: P), retained_effects: shared19 }; action view uses View; }`;
 const r=inspect(source);assert.equal(r.status,'InspectionRejected');assert.equal(r.authoringStatus,'AuthoringChecked');assert.equal(r.diagnostics[0].code,'BETA_INSPECTION_BOUND');assert.deepEqual(r.intents,[]);assert.deepEqual(r.identities,[]);assert.deepEqual(r.actions,[]);
});

test('SpecifiedOnly inspection has open financial relations and no S0 obligations',()=>{
 const source='profile "moriarty-beta/1"; agreement Horizon { policy P = {}; intent Queue = { operation: governance.queue(policy: P, next_epoch: 2), nonce: "proposal" }; action queue uses Queue; }';
 const r=inspect(source);assert.equal(r.status,'AuthoringChecked');assert.equal(r.coverage.financialRelations,'Open');assert.deepEqual(r.requiredPremises,[]);assert.deepEqual(r.unverifiedBindings,[]);assert.equal(Object.hasOwn(r.bounds,'s0SignedFieldMax'),false);assert.deepEqual(r.coverage.localS0Actions,[]);
 const action=r.actions[0];assert.equal(action.support,'SpecifiedOnly');assert.equal(action.coverage.financialRelations,'Open');assert.equal(action.coverage.localPreparation,'Unsupported');assert.deepEqual(action.requiredPremises,[]);assert.deepEqual(action.unverifiedBindings,[]);assert.equal(Object.hasOwn(action.bounds,'s0SignedFieldMax'),false);
 assert.equal(r.intents[0].terms.operation.name,'governance.queue');assert.equal(r.intents[0].terms.operation.args.next_epoch.value,'2');assert.equal(r.intents[0].terms.nonce.value,'proposal');
 const declarationOnly=inspect(transferSource.replace('action pay uses Payment;',''));assert.deepEqual(declarationOnly.requiredPremises,[]);assert.equal(declarationOnly.coverage.financialRelations,'Open');assert.equal(declarationOnly.intents[0].terms.operation.args.value.atoms,'1000');
});
test('mixed inspection scopes conditional Core coverage and signed bounds to actual LocalS0 actions',()=>{
 const source=transferSource.replace('action pay uses Payment;','action pay uses Payment; policy P = {}; intent Queue = { operation: governance.queue(policy: P, next_epoch: 2) }; action queue uses Queue;');
 const r=inspect(source);assert.equal(r.status,'AuthoringChecked');assert.equal(r.coverage.financialRelations,'ConditionalOnLocalS0Preparation');assert.deepEqual(r.coverage.localS0Actions,['pay']);assert.equal(r.bounds.s0Scope,'LocalS0ActionsOnly');assert.equal(r.bounds.s0SignedFieldMax,((1n<<127n)-1n).toString());
 const s0=r.actions.find(a=>a.name==='pay');assert.equal(s0.support,'LocalS0');assert.equal(s0.coverage.financialRelations,'DelegatedToCoreDuringLocalPreparation');assert.equal(s0.coverage.localPreparation,'AvailableUnqualified');assert.deepEqual(s0.requiredPremises,r.requiredPremises);assert.deepEqual(s0.unverifiedBindings,r.unverifiedBindings);assert.equal(s0.requiredPremises.length,4);assert.equal(s0.bounds.s0SignedFieldMax,r.bounds.s0SignedFieldMax);
 const horizon=r.actions.find(a=>a.name==='queue');assert.equal(horizon.coverage.financialRelations,'Open');assert.deepEqual(horizon.requiredPremises,[]);assert.deepEqual(horizon.unverifiedBindings,[]);assert.equal(Object.hasOwn(horizon.bounds,'s0SignedFieldMax'),false);
 assert.equal(r.intents.find(i=>i.name==='Payment').terms.net_floor.atoms,'1000');assert.equal(r.intents.find(i=>i.name==='Queue').terms.operation.args.next_epoch.value,'2');
});
test('automatic overpayment note describes only effects actually emitted',async()=>{
 const {repaySource,repayScenario}=await import('./fixtures.mjs');const source=repaySource.replace('30.00 USD','0.31 USD');const scenario={...repayScenario,obligation:{...repayScenario.obligation,principal:'20',accrued:'10',outstanding:'30'}};
 const automatic=expand(source,'repay_loan',JSON.stringify(scenario));assert.equal(automatic.status,'Expanded');assert.match(automatic.proposalNote,/unchanged obligation/);assert.match(automatic.source6,/set_obligation Loan principal 20 accrued 10 outstanding 30/);
 const candidate_effects=[{kind:'Debit',account:'Payer',asset:'A',amount:'31'},{kind:'Credit',account:'Creditor',asset:'A',amount:'31'},{kind:'SetObligation',id:'Loan',principal:'0',accrued:'0',outstanding:'0',status:'Settled'},{kind:'UseAllowance',owner:'Payer',amount:'31'},{kind:'UseReplay',key:'n1'},{kind:'AdvanceHead',predecessor:'h0',successor:'h1'}];const overridden=expand(source,'repay_loan',JSON.stringify({...scenario,candidate_effects}));assert.equal(overridden.status,'Expanded');assert.equal(overridden.proposalNote,null);assert.match(overridden.source6,/set_obligation Loan principal 0 accrued 0 outstanding 0 status settled/);assert.ok(!overridden.source6.includes('principal 20 accrued 10 outstanding 30'));
 const result=simulate(source,'repay_loan',JSON.stringify({...scenario,candidate_effects}));assert.equal(result.status,'CoreRejected');assert.equal(result.result.rejection.judgment,'effect');assert.equal(result.result.rejection.code,'S0_EFFECT_RANGE');
});
test('root missing scenario fields use RFC6901 pointers',()=>{
 const s={...transferScenario};delete s.allowance;
 const r=simulate(transferSource,'pay',JSON.stringify(s));assert.equal(r.status,'FormationRejected');assert.match(r.diagnostics[0].message,/\(\/allowance\)/);assert.ok(!r.diagnostics[0].message.includes('//'));
});
test('format keeps named calls compact with unchanged financial elaboration',async()=>{
 const {format}=await import('../src/index.ts');const source=transferSource.replace('const price','// price comment\n const price');
 const f=format(source);assert.ok(f.text);assert.match(f.text,/transfer\(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee\)/);
 assert.match(f.text,/\/\/ price comment/);assert.equal(format(f.text).text,f.text);
 assert.equal(expand(f.text,'pay',JSON.stringify(transferScenario)).source6,expand(source,'pay',JSON.stringify(transferScenario)).source6);
});
test('inspection refuses deep shared value projections before stack overflow',()=>{
 let source='profile "moriarty-beta/1"; agreement Deep { policy P = {}; const v0 = [];';
 for(let i=1;i<=80;i++)source+=`const v${i} = ${'['.repeat(30)}v${i-1}${']'.repeat(30)};`;
 source+='intent Queue = { operation: governance.queue(policy: P,next_epoch: 2), retained_effects: v80 }; action queue uses Queue; }';
 assert.equal(check(source).status,'AuthoringChecked');const r=inspect(source);
 assert.equal(r.status,'InspectionRejected');assert.equal(r.authoringStatus,'AuthoringChecked');assert.equal(r.diagnostics[0].code,'BETA_INSPECTION_BOUND');assert.deepEqual(r.intents,[]);
});
const horizonPrefix='profile "moriarty-beta/1"; agreement Domains { domain H = {id:"Home",chain:"example",network:"test"}; domain F = {id:"Foreign",chain:"example",network:"test"}; account Owner = {domain:H,id:"Owner"}; account Holder = {domain:F,id:"Holder"}; asset USD = {domain:H,id:"Usd",scale:2,representation:"canonical"}; asset GOLD = {domain:H,id:"Gold",scale:2,representation:"canonical"}; asset WUSD = {domain:F,id:"Usd",scale:2,representation:"wrapped"};';
const escrow='bridge.escrow(owner: Owner,amount: 1.00 USD,destination: F,claim_id:"claim")';
function staticReject(body,code){const r=check(horizonPrefix+body+'}');assert.equal(r.status,'AuthoringRejected');assert.equal(r.diagnostics[0].code,code);}
test('bridge local custody domain is checked without forbidding a foreign counterpart',()=>{
 assert.equal(check(horizonPrefix+`intent I={operation:${escrow}};action a uses I;}`).status,'AuthoringChecked');
 staticReject('intent I={operation:bridge.escrow(owner:Owner,amount:1.00 WUSD,destination:F,claim_id:"claim")};','BETA_DOMAIN_MISMATCH');
 staticReject('intent I={domain:H,asset:WUSD,signer:Holder,operation:bridge.claim(owner:Holder,amount:1.00 WUSD,source:H,claim_id:"claim")};','BETA_DOMAIN_MISMATCH');
 assert.equal(check(horizonPrefix+'intent I={domain:F,asset:WUSD,signer:Holder,operation:bridge.claim(owner:Holder,amount:1.00 WUSD,source:H,claim_id:"claim")};}').status,'AuthoringChecked');
});
test('horizon monetary caps preserve explicit or bridge-inferred nominal assets',()=>{
 for(const unit of ['WUSD','GOLD'])staticReject(`intent I={operation:${escrow},fee_cap:1.00 ${unit}};`,'BETA_ASSET_MISMATCH');
 staticReject(`intent I={asset:GOLD,operation:${escrow}};`,'BETA_ASSET_MISMATCH');
 const swap='pool Pool={domain:H,id:"Pool",assets:[USD,GOLD]}; intent I={domain:H,asset:USD,signer:Owner,gross_cap:1.00 USD,fee_cap:0.00 USD,net_floor:2.00 GOLD,operation:amm.swap_exact_input(pool:Pool,owner:Owner,input:1.00 USD,output_asset:GOLD,fee_cap:0.00 USD,net_floor:2.00 GOLD)};';
 assert.equal(check(horizonPrefix+swap+'}').status,'AuthoringChecked');
});
test('share identities and duty hints have explicit authoring shapes',()=>{
 staticReject('share_class A={domain:H,id:"Shares",backing:USD};share_class B={domain:H,id:"Shares",backing:USD};','BETA_DUPLICATE_ID');
 staticReject(`intent I={operation:${escrow},retained_duties:4};`,'BETA_TYPE');
 staticReject('stage S={domain:H,signed_floor:true};','BETA_TYPE');
 staticReject('stage S={domain:H,signed_floor:1.00 WUSD};','BETA_DOMAIN_MISMATCH');
 assert.equal(check(horizonPrefix+`stage S={domain:H,signed_floor:1.00 USD};intent I={operation:${escrow},retained_duties:["pending"]};}`).status,'AuthoringChecked');
});
