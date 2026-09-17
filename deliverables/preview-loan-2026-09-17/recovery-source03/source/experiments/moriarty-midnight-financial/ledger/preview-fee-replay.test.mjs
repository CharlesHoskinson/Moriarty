// Public retained Preview transaction replay only. Capability commitments are
// rebound to synthetic test secrets; all financial state/effects/native bytes
// remain retained inputs. This does not re-establish finality or secret custody.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {createFinancialComparator} from './financial-comparison.mjs';
import {decodeNativeFinancialTransaction} from './receipt.mjs';
const INPUTS={
 "plan.json": "43b7f40ebd7955a2153687b4446edd1a8898eb35bfeb464b6f0a72851b608a66",
 "integration-result.json": "b61ad34873668fa03321d85721a1c386bf5440516d7d68ca25b5d917f6380d7d",
 "stage-deploy.json": "b3c96db4c363896b55ab1ce7a626fb8fe643d7a9c7fae9c9eb8f8d4c4696b9f4",
 "stage-initialize.json": "b7dd753b7ecfee30fdd415f701733af2345f7e7774ed9bd5c297d2c362e32949",
 "stage-accrue.json": "2667c194053a40201a847330d0c8518a78084189dbc8a1a129eeb384b0d0da7f",
 "stage-settle.json": "7071ad891e930c30c8c4f4e3db7d7434c3cd295029ebacb268e9b51456af130c",
 "public-transactions/b5d9212284d697ca05342b2a7196680b7149568815224ca1674ecb9a5004f7d2.bin": "b5d9212284d697ca05342b2a7196680b7149568815224ca1674ecb9a5004f7d2",
 "public-transactions/494925874aaa6a9b858b96ac20ac537c4c948ae2ddfdb2059581d8caec05fed7.bin": "494925874aaa6a9b858b96ac20ac537c4c948ae2ddfdb2059581d8caec05fed7",
 "public-transactions/8c18658d4256afa57bda708a5a620e701847e069ddd4f8af062bf1716adf48c2.bin": "8c18658d4256afa57bda708a5a620e701847e069ddd4f8af062bf1716adf48c2",
 "public-transactions/b58618c82f6fc0208d59fc54b9d2b8b94515f7b94485e328ea8bd75b5d5d865c.bin": "b58618c82f6fc0208d59fc54b9d2b8b94515f7b94485e328ea8bd75b5d5d865c"
};
const base=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/actual-run/',import.meta.url);
function read(file){const raw=readFileSync(new URL(file,base));assert.equal(createHash('sha256').update(raw).digest('hex'),INPUTS[file]);return raw;}
const plan=JSON.parse(read('plan.json')),retained=JSON.parse(read('integration-result.json'));
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs').href);
const runtime=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/compact-runtime/dist/index.js').href);
const bytes=s=>Uint8Array.from(Buffer.from(s,'hex'));
const roles={firstAddress:plan.roles.firstAddress,secondAddress:plan.roles.secondAddress,firstSecret:new Uint8Array(32).fill(17),secondSecret:new Uint8Array(32).fill(18)};
function state(stage){
 const p=JSON.parse(read('stage-'+stage+'.json')).publicState;
 const scalars=x=>typeof x==='string'?BigInt(x):Object.fromEntries(Object.entries(x).map(([k,v])=>[k,scalars(v)]));
 const s={...p,programDigest:bytes(p.programDigest),networkTag:bytes(p.networkTag),remaining:BigInt(p.remaining),revision:BigInt(p.revision),kernelState:scalars(p.kernelState),usdDomain:bytes(p.usdDomain),usdColor:bytes(p.usdColor),lastAccrue:scalars(p.lastAccrue),lastSettle:scalars(p.lastSettle)};
 for(const [role,secret] of [['borrower',roles.firstSecret],['lender',roles.secondSecret]]){
  s[role+'Address']={bytes:bytes(p[role+'Address'].bytes)};
  s[role+'Capability']=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[Uint8Array.from(Buffer.from(('moriarty:sp05:loan:'+role).padEnd(32,'\0'))),bytes(p.networkTag),bytes(p.programDigest),secret]);
 }
 return s;
}
test('retained Preview loan native bytes/effects remain within the actual admitted plan cap',async t=>{
 const cap=BigInt(plan.limits.dustFee),c=await createFinancialComparator({kind:'loan',roles,networkTag:plan.networkTag,expectedProtocolVersion:plan.expectedProtocolVersion,dustFeeCap:cap});let total=0n;
 for(const receipt of retained.driver.stages){
  const raw=read('public-transactions/'+receipt.transaction.transactionHash+'.bin');
  assert.deepEqual(decodeNativeFinancialTransaction(raw,ledger),receipt.transaction);
  total+=BigInt(receipt.transaction.dustFee);
  c.verifyStage(receipt.circuitId,{receipt,state:state(receipt.circuitId)});
 }
 assert.equal(c.finish().status,'PASS');assert.ok(total<=cap);
 t.diagnostic(JSON.stringify({nativeFeeSpeck:total.toString(),admittedCapSpeck:cap.toString(),indexerUnits:'unresolved',scope:'retained public financial replay with synthetic capability commitments'}));
});
