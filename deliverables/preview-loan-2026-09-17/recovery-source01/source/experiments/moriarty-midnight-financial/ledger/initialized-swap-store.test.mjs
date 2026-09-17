import test from 'node:test';
import assert from 'node:assert/strict';
import * as fs from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {createRequire} from 'node:module';
import {createHash} from 'node:crypto';
import {PINNED_NM} from './providers.mjs';
const require=createRequire(join(PINNED_NM,'../package.json'));
const {Level}=require('level');
const accountId='synthetic-recovery-account';
const key='!sp05-loan:'+createHash('sha256').update(accountId).digest('hex').slice(0,32)+'!__midnight_encryption_metadata__';
const metadata=JSON.stringify({version:1,salt:'ab'.repeat(32)});
const snapshot=d=>fs.readdirSync(d).sort().map(n=>[n,fs.readFileSync(join(d,n)).toString('hex')]);
async function fixture(t,extra){const base=fs.mkdtempSync(join(tmpdir(),'recovery-store-fixture-'));fs.chmodSync(base,0o700);t.after(()=>fs.rmSync(base,{recursive:true,force:true}));const sourceDirectory=join(base,'source'),inspectionDirectory=join(base,'copy');const db=new Level(sourceDirectory);await db.open();await db.put(key,metadata);if(extra)await db.put(...extra);await db.close();fs.chmodSync(sourceDirectory,0o700);for(const n of fs.readdirSync(sourceDirectory))fs.chmodSync(join(sourceDirectory,n),0o600);return{base,sourceDirectory,inspectionDirectory,accountId};}
async function inspect(args){const api=await import('./recover-store.mjs').catch(()=>({}));assert.equal(typeof api.inspectFailedLoanStore,'function','bounded store inspector must exist');return api.inspectFailedLoanStore(args);}
// Synthetic encrypted scratch entries only; no original private store reads.
async function preserve(args){const api=await import('./recover-store.mjs');assert.equal(typeof api.preserveInitializedSwapStore,'function','initialized-store byte snapshot must exist');return api.preserveInitializedSwapStore(args);}
const contractAddress='8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261';
async function initializedFixture(t,mutate){
 const f=await fixture(t);for(const n of fs.readdirSync(f.sourceDirectory))fs.unlinkSync(join(f.sourceDirectory,n));
 const {levelPrivateStateProvider}=await import(join(PINNED_NM,'@midnight-ntwrk/midnight-js-level-private-state-provider/dist/index.mjs'));
 const provider=levelPrivateStateProvider({midnightDbName:f.sourceDirectory,privateStateStoreName:'sp05-swap',accountId,privateStoragePasswordProvider:()=> 'N7!kR4$vP9@xT6#bQ2'});
 provider.setContractAddress(contractAddress);await provider.set('sp05-swap',{});await provider.setSigningKey(contractAddress,'cd'.repeat(32));
 if(mutate){const db=new Level(f.sourceDirectory);await db.open();try{await mutate(db);}finally{await db.close();}}
 for(const n of fs.readdirSync(f.sourceDirectory))fs.chmodSync(join(f.sourceDirectory,n),0o600);
 return {...f,snapshotDirectory:join(f.base,'snapshot')};
}
const accountHash=createHash('sha256').update(accountId).digest('hex').slice(0,32);
const stateKey='!sp05-swap:'+accountHash+'!'+contractAddress+':sp05-swap';
const signingMetadata='!signing-keys:'+accountHash+'!__midnight_encryption_metadata__';
test('initialized store accepts actual SDK v2 namespace through separate inspection, preserving original and raw snapshot',async t=>{const f=await initializedFixture(t),before=snapshot(f.sourceDirectory);const r=await preserve(f);assert.deepEqual(r,{status:'PRESERVED',sourceManifestSha256:r.sourceManifestSha256,snapshotManifestSha256:r.sourceManifestSha256});assert.match(r.sourceManifestSha256,/^[a-f0-9]{64}$/);assert.deepEqual(snapshot(f.sourceDirectory),before);assert.deepEqual(snapshot(f.snapshotDirectory),before);assert.ok(fs.existsSync(f.inspectionDirectory));assert.equal(JSON.stringify(r).includes(f.base),false);assert.equal(JSON.stringify(r).includes('cdcdcd'),false);});
test('initialized inspection rejects missing, extra, plaintext, v1, mismatched-salt and malformed metadata without modifying source or snapshot',async t=>{for(const mutate of [db=>db.del(signingMetadata),db=>db.del(stateKey),db=>db.put('unknown','private'),db=>db.put(stateKey,'{}'),async db=>{const b=Buffer.from(await db.get(stateKey),'base64');b[0]=1;await db.put(stateKey,b.toString('base64'));},async db=>{const b=Buffer.from(await db.get(stateKey),'base64');b[1]^=1;await db.put(stateKey,b.toString('base64'));},db=>db.put(signingMetadata,JSON.stringify({salt:'ab',version:1})),db=>db.put(signingMetadata,JSON.stringify({salt:'ab'.repeat(32),version:2}))]){const f=await initializedFixture(t,mutate),before=snapshot(f.sourceDirectory);await assert.rejects(preserve(f),/RECOVERY_STORE/);assert.deepEqual(snapshot(f.sourceDirectory),before);assert.deepEqual(snapshot(f.snapshotDirectory),before);assert.ok(fs.existsSync(f.inspectionDirectory));}});
test('initialized copies reject reuse, overlaps, symlinks and loose permissions',async t=>{const f=await initializedFixture(t);for(const change of [{snapshotDirectory:f.sourceDirectory},{inspectionDirectory:f.snapshotDirectory},{inspectionDirectory:join(f.sourceDirectory,'nested')}])await assert.rejects(preserve({...f,...change}),/RECOVERY_STORE/);fs.mkdirSync(f.inspectionDirectory,{mode:0o700});await assert.rejects(preserve(f),/RECOVERY_STORE/);assert.equal(fs.existsSync(f.snapshotDirectory),false);fs.rmdirSync(f.inspectionDirectory);fs.symlinkSync('CURRENT',join(f.sourceDirectory,'evil'));await assert.rejects(preserve(f),/RECOVERY_STORE/);fs.unlinkSync(join(f.sourceDirectory,'evil'));fs.chmodSync(join(f.sourceDirectory,'CURRENT'),0o644);await assert.rejects(preserve(f),/RECOVERY_STORE/);assert.equal(fs.existsSync(f.snapshotDirectory),false);});
test('source and raw snapshot mutations during inspection are detected and retained',async t=>{for(const target of ['sourceDirectory','snapshotDirectory']){const f=await initializedFixture(t),open=Level.prototype.open;const mock=t.mock.method(Level.prototype,'open',function(...args){if(this.location===f.inspectionDirectory)fs.writeFileSync(join(f[target],'CURRENT'),'fixture mutation\n');return open.apply(this,args);});try{await assert.rejects(preserve(f),new RegExp(target==='sourceDirectory'?'RECOVERY_STORE_SOURCE_CHANGED':'RECOVERY_STORE_SNAPSHOT_CHANGED'));assert.ok(fs.existsSync(f.inspectionDirectory));}finally{mock.mock.restore();}}});
