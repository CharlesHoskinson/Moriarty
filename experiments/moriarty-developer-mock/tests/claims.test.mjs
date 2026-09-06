import test from 'node:test';
import assert from 'node:assert/strict';
import * as m from '../src/language/claims.ts';

test('canonical encoding is byte-exact and bounded rather than normalizing ambiguous input', () => {
  assert.equal(typeof m.canonical, 'function');
  assert.equal(m.canonical({z:['2',true],a:'1'}), '{"a":"1","z":["2",true]}');
  assert.deepEqual(m.decodeCanonical('{"a":"1"}'), {a:'1'});
  for (const text of ['{"a":"1","a":"2"}', '{ "a":"1"}', '{"a":1}', '{"z":"2","a":"1"}']) assert.throws(() => m.decodeCanonical(text));
  const cycle = {}; cycle.x = cycle;
  for (const value of [NaN,1n,()=>1,{x:undefined},cycle,{['x'.repeat(65)]:'1'},'x'.repeat(4097)]) assert.throws(()=>m.canonical(value));
  let deep='x';for(let i=0;i<18;i++)deep=[deep];assert.throws(()=>m.canonical(deep));
});

test('canonical arrays reject symbols, custom prototypes and accessors without executing code', () => {
  const symbolArray=['safe'];symbolArray[Symbol('hidden')]='value';
  const customArray=['safe'];Object.setPrototypeOf(customArray,{custom:true});
  let calls=0;const accessorArray=[];Object.defineProperty(accessorArray,'0',{enumerable:true,get(){calls++;return 'unsafe';}});accessorArray.length=1;
  for(const value of [symbolArray,customArray,accessorArray]) assert.throws(()=>m.canonical(value));
  assert.equal(calls,0);
});

test('SHA-256 commitment has a fixed byte-level vector', async () => {
  assert.equal(typeof m.hashCanonical,'function');
  // SHA256 of canonical empty object, the two bytes 0x7b 0x7d.
  assert.equal(await m.hashCanonical({}), '44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a');
});

// Independent synthetic ledger/policy, not evaluator output or financial formula.
function inputs() {
 const before={instance:'pool',revision:'0',remaining:'8',values:{reserveA:'100',reserveB:'200',traderA:'10',traderB:'0',providerA:'0',providerB:'0',closed:'0'}};
 const after={...before,revision:'1',remaining:'7',values:{...before.values,reserveA:'102',reserveB:'197',traderA:'8',traderB:'3'}};
 const effects=[{kind:'Transfer',fields:{asset:'demo:A',from:'trader',to:'pool',amount:'2'}},{kind:'Transfer',fields:{asset:'demo:B',from:'pool',to:'trader',amount:'3'}}];
 const policy={profile:'swap',accounting:{reserveA:{asset:'demo:A',account:'pool'},reserveB:{asset:'demo:B',account:'pool'},traderA:{asset:'demo:A',account:'trader'},traderB:{asset:'demo:B',account:'trader'},providerA:{asset:'demo:A',account:'provider'},providerB:{asset:'demo:B',account:'provider'}},transfers:[{asset:'demo:A',from:'trader',to:'pool',maxAmount:'2'},{asset:'demo:B',from:'pool',to:'trader',maxAmount:'200'}],minimumCredits:[{asset:'demo:B',to:'trader',minAmount:'3'}],fees:[],dues:[],allowedWrites:['reserveA','reserveB','traderA','traderB']};
 const result={outcome:'evaluated',before,after,effects,writes:['reserveA','reserveB','traderA','traderB'],steps:'4'};
 return {before,action:{name:'swap',args:{amountIn:'2'}},result,policy,domain:{network:'local',deployment:'r2-demo',semantics:'moriarty-r2/1',verifierProfile:'midnight-native-pcd/1'}};
}
async function prepare() {
 assert.equal(typeof m.prepareLocalPlan,'function');
 const i=inputs(); const p=await m.prepareLocalPlan({description:'synthetic test program'},i.before,i.action,i.result,i.policy,i.domain);
 assert.equal(p.outcome,'prepared',p.message);return p.plan;
}

test('claim commitments have an acyclic dependency graph and no enclosing roots in ClaimSpec', async () => {
 const p=await prepare(); const known=new Set();
 for (const claim of p.claims) {
   assert.ok(claim.spec.dependencies.every(d=>known.has(d)));
   assert.ok(!('intentDigest' in claim.spec));assert.ok(!('manifestRoot' in claim.spec));
   assert.equal(claim.id, await m.hashCanonical({domain:'MORIARTY-CLAIM-v1',spec:claim.spec}));known.add(claim.id);
 }
 assert.equal(p.claims.length,4);
 assert.equal(p.manifestRoot,await m.hashCanonical({domain:'MORIARTY-MANIFEST-v1',claims:p.claims}));
});

test('real local Ed25519 signatures verify but do not fulfill mandatory contract/PCD claims', async () => {
 const plan=await prepare();const keys=await m.generateLocalKey();const signed=await m.signLocalPlan(plan,keys);
 const checked=await m.verifySignedPlan(signed,plan,signed.publicKey);
 assert.equal(checked.outcome,'checked',checked.message);
 const required=await m.verifyRequiredClaims(signed,plan,signed.publicKey);
 assert.equal(required.outcome,'unavailable');assert.equal(required.missing.length,4);
 assert.ok(!JSON.stringify(signed).includes('privateKey'));
 const other=await m.generateLocalKey();const otherSigned=await m.signLocalPlan(plan,other);
 assert.equal((await m.verifySignedPlan(signed,plan,otherSigned.publicKey)).outcome,'rejected');
 const forged=structuredClone(signed);forged.signature='00'.repeat(64);
 assert.equal((await m.verifySignedPlan(forged,plan,signed.publicKey)).outcome,'rejected');
});

test('signed domain/program/state/effect/policy/mandatory/verifier bindings reject substitution', async () => {
 const plan=await prepare();const signed=await m.signLocalPlan(plan,await m.generateLocalKey());
 const mutations=[
   x=>x.plan.domain.network='elsewhere',x=>x.plan.txCore.programHash='00'.repeat(32),
   x=>x.plan.txCore.after.values.traderB='4',x=>x.plan.txCore.effects[1].fields.to='Mallory',
   x=>x.plan.policy.transfers[0].maxAmount='999',x=>x.plan.claims.pop(),
   x=>x.plan.claims[0].spec.mode='optional',x=>x.plan.claims[0].spec.verifierProfile='approve-all',
   x=>x.plan.claims[0].spec.type='UnknownMandatory',x=>x.boundClaims.pop(),
   x=>x.boundClaims[0].intentDigest='00'.repeat(32),x=>x.boundClaims[0].evidence={kind:'SimulatedEvidence'}
 ];
 for(const mutate of mutations){const bad=structuredClone(signed);mutate(bad);assert.equal((await m.verifyRequiredClaims(bad,plan,signed.publicKey)).outcome,'rejected');}
});

test('preparation cannot sign a hidden or unauthorized effect just because execution was evaluated', async () => {
 assert.equal(typeof m.prepareLocalPlan,'function');
 const i=inputs();i.result.effects.push({kind:'Approval',fields:{spender:'Mallory'}});
 const p=await m.prepareLocalPlan({},i.before,i.action,i.result,i.policy,i.domain);
 assert.equal(p.outcome,'rejected');
});

test('preparation snapshots every caller input before its first asynchronous digest yields', async () => {
 const i=inputs();const program={description:'original program'};
 const expected={program:structuredClone(program),before:structuredClone(i.before),action:structuredClone(i.action),result:structuredClone(i.result),policy:structuredClone(i.policy),domain:structuredClone(i.domain)};
 const pending=m.prepareLocalPlan(program,i.before,i.action,i.result,i.policy,i.domain);
 program.description='mutated program';i.before.instance='mutated';i.action.name='mutated';
 i.result.after.values.traderB='999';i.result.effects[0].fields.to='Mallory';i.policy.transfers[0].maxAmount='999';i.domain.network='mutated';
 const prepared=await pending;assert.equal(prepared.outcome,'prepared',prepared.message);
 assert.equal(prepared.plan.txCore.programHash,await m.hashCanonical(expected.program));
 assert.deepEqual(prepared.plan.txCore.before,expected.before);assert.deepEqual(prepared.plan.txCore.action,expected.action);
 assert.deepEqual(prepared.plan.txCore.after,expected.result.after);assert.deepEqual(prepared.plan.txCore.effects,expected.result.effects);
 assert.deepEqual(prepared.plan.policy,expected.policy);assert.deepEqual(prepared.plan.domain,expected.domain);
});
