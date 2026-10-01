// Source-only preparation helper. Requires separately authorized execution.
// Builds official generated constructor/runtime artifacts; no proofs or ledger acceptance.
import assert from 'node:assert/strict';
import {readFileSync,mkdirSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import * as runtime from '../compiler-probe/runtime/node_modules/@midnight-ntwrk/compact-runtime/dist/index.js';
import {Contract,ledger} from '../kernel-prototype/output/contract/index.js';
const sha=b=>createHash('sha256').update(b).digest('hex');
const [projectionPath,expectedProjectionSha,retainedTracePath,expectedTraceSha,vkPath,expectedVkSha,newOutput]=process.argv.slice(2);
const prepareNoVk=vkPath==='PREPARE_NO_VK';
assert.ok((prepareNoVk?expectedVkSha==='NONE':vkPath && /^[a-f0-9]{64}$/.test(expectedVkSha)) && newOutput && /^[a-f0-9]{64}$/.test(expectedProjectionSha) && /^[a-f0-9]{64}$/.test(expectedTraceSha),'exact supervisor source pins required');
const pbytes=readFileSync(projectionPath),tbytes=readFileSync(retainedTracePath);
assert.equal(sha(pbytes),expectedProjectionSha);assert.equal(sha(tbytes),expectedTraceSha);
const p=JSON.parse(pbytes),retained=JSON.parse(tbytes);
const bytes=h=>Uint8Array.from(Buffer.from(h,'hex'));
const restore=v=>Array.isArray(v)?v.map(restore):v&&typeof v==='object'?Object.keys(v).length===1&&'bytes'in v?bytes(v.bytes):Object.keys(v).length===1&&'bigint'in v?BigInt(v.bigint):Object.fromEntries(Object.entries(v).map(([k,x])=>[k,restore(x)])):v;
// Native Rust serde_bytes accepts integer arrays. Decimal BigInts stay exact JSON numbers.
function nativeJson(v){
 if(typeof v==='bigint')return v.toString();
 if(v instanceof Uint8Array)return '['+[...v].join(',')+']';
 if(Array.isArray(v))return '['+v.map(nativeJson).join(',')+']';
 if(v&&typeof v==='object')return '{'+Object.entries(v).map(([k,x])=>JSON.stringify(k)+':'+nativeJson(x)).join(',')+'}';
 return JSON.stringify(v);
}
const msg=bytes(p.framed.signing_message_hex),sig={r:BigInt('0x'+p.artifact.signatureHex.slice(0,64)),s:BigInt('0x'+p.artifact.signatureHex.slice(64))};
const pk={x:BigInt(p.publicKey.x),y:BigInt(p.publicKey.y),identity:false};
const contract=new Contract({});
const init=await contract.initialState(runtime.createConstructorContext({},'00'.repeat(32)),pk,bytes('a1'.repeat(32)),{bytes:bytes('02'.repeat(32))},{bytes:bytes('03'.repeat(32))},10000n,10000n,10n,1n);
init.currentContractState.balance=new Map([[{tag:'unshielded',raw:'a1'.repeat(32)},10000n]]);
const operation=new runtime.ContractOperation();
if(!prepareNoVk){const vkbytes=readFileSync(vkPath);assert.equal(sha(vkbytes),expectedVkSha);operation.verifierKey=new Uint8Array(vkbytes);}
init.currentContractState.setOperation("pay",operation);
const initial=init.currentContractState.serialize();
const result=await contract.circuits.pay(runtime.createCircuitContext({circuitId:'pay',contractAddress:'04'.repeat(32),coinPublicKeyOrZswapState:'00'.repeat(32),contractState:init.currentContractState,privateState:{},time:1,parentBlockHash:'00'.repeat(32)}),msg,sig);
assert.equal(result.context.callProofDataTrace.length,1);
const trace=result.context.callProofDataTrace[0];assert.equal(trace.circuitId,'pay');
const normalized={input:trace.input,output:trace.output,publicTranscript:trace.publicTranscript,privateTranscriptOutputs:trace.privateTranscriptOutputs};
assert.equal(nativeJson(normalized),nativeJson(restore(retained)),'generated actual trace differs from retained frozen source');
assert.equal(trace.publicTranscript.length,293);assert.equal(trace.publicTranscript.filter(o=>'popeq'in o).length,47);
// Serialize the official constructor storage; post storage is the actual native generated outcome.
const expected=runtime.ContractState.deserialize(initial);
expected.data=result.context.callContext.currentQueryContext.state;
const observation=ledger(result.context.callContext.currentQueryContext.state);
assert.equal(observation.ownerBalance,8990n);assert.equal(observation.recipientBalance,1000n);assert.equal(observation.feeBalance,10n);
assert.equal(Buffer.from(init.currentContractState.serialize()).toString('hex'),Buffer.from(initial).toString('hex'));
mkdirSync(newOutput); // refuses an existing output tree
const exports=[['initial-contract.tagged',initial],['expected-contract.tagged',expected.serialize()],['runtime-native.json',Buffer.from(nativeJson(normalized))],['registered-pay-operation.tagged',operation.serialize()]];
const receipt={scope:'TRUSTED GENESIS export only; no authenticated mint/deploy/funding or ledger acceptance',projection_sha256:expectedProjectionSha,retained_trace_sha256:expectedTraceSha,registered_vk_sha256:prepareNoVk?null:expectedVkSha,prepare_no_vk:prepareNoVk,expected_contract_scope:"storage-only generated poststate reference; escrow remains constructor10000 and is checked as8990 separately after native application",ops:293,reads:47,artifacts:[]};
for(const [name,b]of exports){writeFileSync(newOutput+'/'+name,b,{flag:'wx'});receipt.artifacts.push({name,sha256:sha(b),bytes:b.length});}
writeFileSync(newOutput+'/export-receipt.json',JSON.stringify(receipt,null,2),{flag:'wx'});
