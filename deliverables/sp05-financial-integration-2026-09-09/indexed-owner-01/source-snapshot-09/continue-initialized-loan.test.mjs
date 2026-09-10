import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import {PINNED_NM} from './providers.mjs';
const api=await import('./continue-initialized-loan.mjs').catch(()=>({}));
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')));
const root=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-recovery-03/',import.meta.url);
const raw=readFileSync(new URL('run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin',root));
const stateBytes=readFileSync(new URL('indexed-initialize-state.bin',root));
const state=()=>ledger.ContractState.deserialize(stateBytes);
const address='ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713';
const submitted='006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46';
test('actual initialize bytes bind complete history input identities and corrected minted origin',()=>{
 assert.equal(typeof api.inspectInitializedLoanBytes,'function');const b=api.inspectInitializedLoanBytes(raw,ledger);
 assert.equal(b.txId,submitted);assert.equal(b.contractAddress,address);assert.equal(b.identifiers.length,2);
 assert.deepEqual(b.mintedOutput,{segment:21861,section:'guaranteed',outputNo:0,intentHash:'4d343d21f150af922dc483b319b87b069a2b2cb8e3f3f64f4bad4485dbf61603',owner:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',type:'e92df6339320f55209d4586ce039b6ef05a7be960999fca913006146cb72cdef',value:'20000000000'});
 const tx=ledger.Transaction.deserialize('signature','proof','binding',raw);const nullifiers=[...tx.intents.values()].flatMap(i=>i.dustActions?.spends??[]).map(s=>s.oldNullifier);
 assert.ok(nullifiers.length>0);assert.deepEqual(b.oldDustNullifiers,nullifiers);assert.deepEqual(b.spentUnshieldedInputs,[]);
 assert.ok(Object.isFrozen(b)&&Object.isFrozen(b.mintedOutput)&&Object.isFrozen(b.oldDustNullifiers)&&Object.isFrozen(b.identifiers));assert.equal(b.proofVerified,false);assert.equal(b.ledgerAccepted,false);
});
test('altered native bytes and the previous deployment fail closed',()=>{
 assert.equal(typeof api.inspectInitializedLoanBytes,'function');const changed=Buffer.from(raw);changed[changed.length-1]^=1;
 for(const value of [changed,new Uint8Array(),null,readFileSync(new URL('../local-execution-04/run-public/public-transactions/0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c.bin',root))])assert.throws(()=>api.inspectInitializedLoanBytes(value,ledger),/INITIALIZED_NATIVE_HASH/);
});
test('actual initialized state matches exactly; authority, balances and extra operations reject',()=>{
 assert.equal(typeof api.assertInitializedLoanState,'function');assert.equal(api.assertInitializedLoanState(state(),ledger),true);
 for(const mutate of [s=>s.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(s.maintenanceAuthority.committee,1,1n),s=>s.balance=new Map([[{tag:'unshielded',raw:'22'.repeat(32)},1n]]),s=>s.setOperation('unexpected',s.operation('initialize'))]){const s=state();mutate(s);assert.throws(()=>api.assertInitializedLoanState(s,ledger),/INITIALIZED_STATE_MISMATCH/);}
 assert.throws(()=>api.assertInitializedLoanState(null,ledger),/INITIALIZED_STATE_TYPE/);
});
function privateFixture(){
 const calls=[],key='11'.repeat(32),publicState=state();let active=false;
 const provider={
  setContractAddress(a){calls.push('address');assert.equal(a,address);},
  async get(id){calls.push('get');assert.equal(id,'sp05-loan');active=true;await Promise.resolve();active=false;return {};},
  async getSigningKey(a){assert.equal(active,false,'reads must be sequential for Level handle');calls.push('key');assert.equal(a,address);return key;},
 };
 for(const name of ['set','setSigningKey','clear','remove'])Object.defineProperty(provider,name,{get(){throw Error('PRIVATE_WRITE_FORBIDDEN');}});
 // Only signature-key derivation is synthetic. Actual retained authority/state are used.
 const pinnedAuthority=publicState.maintenanceAuthority.committee[0];const sdk={...ledger,signatureVerifyingKey:k=>{assert.equal(k,key);return pinnedAuthority;}};
 return {calls,key,provider,options:{provider,signingKey:key,contractState:publicState,ledger:sdk}};
}
test('existing private state verification only scopes and sequentially reads, returns no secrets or authority token',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPrivate,'function');const f=privateFixture(),result=await api.verifyInitializedLoanPrivate(f.options);
 assert.deepEqual(f.calls,['address','get','key']);assert.equal(result.status,'PRIVATE_STATE_CHECKED');assert.equal(result.contractAddress,address);assert.equal(result.txId,submitted);assert.equal(result.dispatchAuthorized,false);assert.ok(Object.isFrozen(result));assert.ok(!JSON.stringify(result).includes(f.key));
});
for(const entry of [null,undefined,[],{extra:true},Object.create(null)])test('missing or nonempty private state rejects without key read or writes',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPrivate,'function');const f=privateFixture();f.provider.get=async()=>{f.calls.push('get');return entry;};
 await assert.rejects(api.verifyInitializedLoanPrivate(f.options),/INITIALIZED_PRIVATE_STATE/);assert.deepEqual(f.calls,['address','get']);
});
test('private state accessor or hidden value is rejected without getter execution',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPrivate,'function');for(const entry of [Object.defineProperty({},'private',{get(){throw Error('GETTER_FORBIDDEN');}}),Object.assign({},{[Symbol('hidden')]:true})]){const f=privateFixture();f.provider.get=async()=>entry;await assert.rejects(api.verifyInitializedLoanPrivate(f.options),/INITIALIZED_PRIVATE_STATE/);}
});
test('stored signing-key mismatch and read failure stop with closed errors',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPrivate,'function');const f=privateFixture();f.provider.getSigningKey=async()=> '22'.repeat(32);await assert.rejects(api.verifyInitializedLoanPrivate(f.options),/INITIALIZED_PRIVATE_KEY/);
 const g=privateFixture();g.provider.get=async()=>{throw Error('PRIVATE_ERROR_CANARY');};await assert.rejects(api.verifyInitializedLoanPrivate(g.options),e=>{assert.equal(e.message,'INITIALIZED_PRIVATE_READ');assert.ok(!JSON.stringify(e).includes('PRIVATE_ERROR_CANARY'));return true;});
});
test('wrong real signing identity or wrong state prevents any provider access',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPrivate,'function');const f=privateFixture();f.options.ledger=ledger;await assert.rejects(api.verifyInitializedLoanPrivate(f.options),/INITIALIZED_SIGNING_AUTHORITY/);assert.deepEqual(f.calls,[]);
 const g=privateFixture();g.options.contractState.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(g.options.contractState.maintenanceAuthority.committee,1,1n);await assert.rejects(api.verifyInitializedLoanPrivate(g.options),/INITIALIZED_STATE_MISMATCH/);assert.deepEqual(g.calls,[]);
});

test('minted continuation gate requires exactly one full available output and none pending',()=>{
 assert.equal(typeof api.assertInitializedLoanMintedOutput,'function');const binding=api.inspectInitializedLoanBytes(raw,ledger),m=binding.mintedOutput;
 const coin=()=>({utxo:{intentHash:m.intentHash,outputNo:m.outputNo,owner:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9',type:m.type,value:BigInt(m.value)},meta:{}});
 const synced=()=>({unshielded:{availableCoins:[coin()],pendingCoins:[]}});
 assert.equal(api.assertInitializedLoanMintedOutput({synced:synced(),binding}),true);
 for(const mutate of [s=>s.unshielded.availableCoins=[],s=>s.unshielded.pendingCoins=[coin()],s=>s.unshielded.availableCoins.push(coin()),s=>s.unshielded.availableCoins[0].utxo.owner='aa'.repeat(32),s=>s.unshielded.availableCoins[0].utxo.type='aa'.repeat(32),s=>s.unshielded.availableCoins[0].utxo.value=20000000000,s=>s.unshielded.availableCoins[0].utxo.value=1n,s=>s.unshielded.availableCoins[0].utxo.outputNo=1,s=>s.unshielded.availableCoins[0]=coin().utxo]){const s=synced();mutate(s);assert.throws(()=>api.assertInitializedLoanMintedOutput({synced:s,binding}),/INITIALIZED_WALLET/);}
 assert.throws(()=>api.assertInitializedLoanMintedOutput({synced:synced(),binding:{...binding,mintedOutput:{...m,value:'1'}}}),/INITIALIZED_WALLET_BINDING/);
});

test('actual wallet sync decoder keeps indexed Bech32m owner and the continuation gate matches its native identity',async()=>{
 const {createRequire}=await import('node:module');const require=createRequire('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/package.json');const {Schema}=require('effect');
 const {WalletSyncUpdateSchema}=await import('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-unshielded-wallet/dist/v1/SyncSchema.js');
 const binding=api.inspectInitializedLoanBytes(raw,ledger),m=binding.mintedOutput,owner='mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9';
 // Actual initialized output; reconstructed API encoding and inert update metadata.
 const wire={type:'UnshieldedTransaction',transaction:{id:35,hash:binding.transactionHash,type:'RegularTransaction',protocolVersion:1000000,block:{timestamp:1789027704001},transactionResult:{status:'SUCCESS',segments:null}},createdUtxos:[{value:m.value,owner,tokenType:m.type,intentHash:m.intentHash,outputIndex:m.outputNo,ctime:1789027704,registeredForDustGeneration:false}],spentUtxos:[]};
 const coin=Schema.decodeUnknownSync(WalletSyncUpdateSchema)(wire).createdUtxos[0];assert.equal(coin.utxo.owner,owner);assert.equal(coin.utxo.value,20000000000n);
 const synced={unshielded:{availableCoins:[coin],pendingCoins:[]}};assert.equal(api.assertInitializedLoanMintedOutput({synced,binding}),true);
 const {MidnightBech32m,UnshieldedAddress}=await import('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-address-format/dist/index.js');
 for(const bad of [m.owner,owner.toUpperCase(),MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(m.owner,'hex'))).toString(),MidnightBech32m.encode('undeployed',new UnshieldedAddress(Buffer.from('aa'.repeat(32),'hex'))).toString()]){
  const changed={unshielded:{availableCoins:[{...coin,utxo:{...coin.utxo,owner:bad}}],pendingCoins:[]}};
  assert.throws(()=>api.assertInitializedLoanMintedOutput({synced:changed,binding}),/INITIALIZED_WALLET_OUTPUT/);
 }
});
