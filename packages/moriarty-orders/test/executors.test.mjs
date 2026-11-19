import assert from 'node:assert/strict';
import {test} from 'node:test';
import {mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {buildAcceptance} from '../src/build-acceptance.mjs';
import {signOrder} from '../src/authorization.mjs';

test('separate keys authorize exclusive primary and fallback windows',async()=>{
  const out = mkdtempSync(join(tmpdir(),'order-clients-'));
  try {
    const build = buildAcceptance(out,{policy:'executors',skipZk:true});
    const rt = await import(pathToFileURL(build.runtimePath));
    const compiled = await import(pathToFileURL(join(out,'contract/index.js')));
    const {signExecutor} = await import('../src/executors.mjs');
    const bytes = n=>new Uint8Array(32).fill(n);
    const account = n=>({bytes:bytes(n)});
    const address = rt.sampleContractAddress();
    const self = {bytes:rt.encodeContractAddress(address)};
    const terms = {account:account(1),recipient:account(2),executor:account(3),
      orderId:bytes(4),network:bytes(5),assetA:bytes(6),assetB:bytes(7),program:Buffer.from(build.programHash,'hex'),
      input:100n,minimum:200n,fee:0n,expiry:1400n,epoch:0n};
    const keys = [19n,23n,31n,41n].map(k=>rt.ecMulGenerator(k));
    const contract = new compiled.Contract({});
    function context(id,state,time,balance=100n) {
      const ctx = rt.createCircuitContext({circuitId:id,contractAddress:address,coinPublicKeyOrZswapState:'00'.repeat(32),contractState:state,privateState:{},time});
      const q = ctx.callContext.currentQueryContext;
      q.block = {...q.block,secondsSinceEpoch:BigInt(time),lastBlockTime:BigInt(time),secondsSinceEpochErr:0,
        balance:new Map([[{tag:'unshielded',raw:Buffer.from(terms.assetA).toString('hex')},balance]])};
      return ctx;
    }
    const init = (...trusted)=>contract.initialState(rt.createConstructorContext({},'00'.repeat(32)),terms,keys[0],...trusted);
    await assert.rejects(()=>init(keys[1],keys[1]),/DISTINCT/);
    const initial = (await init(keys[1],keys[2])).currentContractState;
    const funded = (await contract.provableCircuits.fund(context('fund',initial,1000,0n))).context.callContext.currentQueryContext.state;
    const ownerSig = signOrder(rt,compiled.pureCircuits,{terms,contract:self,owner:keys[0],action:0n,revision:0n},19n);
    const fill = async(role,secret,time)=>{
      const clientSig = signExecutor(rt,compiled.pureCircuits,{terms,contract:self,revision:0n},role,secret);
      return contract.provableCircuits.fill(context('fill',funded,time),0n,100n,200n,0n,BigInt(time),ownerSig,role==='fallback',clientSig);
    };
    await fill('primary',23n,1000);
    await fill('fallback',31n,1100);
    await assert.rejects(()=>fill('fallback',31n,1099),/FALLBACK_WINDOW/);
    await assert.rejects(()=>fill('primary',23n,1100),/PRIMARY_WINDOW/);
    await assert.rejects(()=>fill('primary',41n,1000),/CLIENT_SIGNATURE/);
    await assert.rejects(()=>fill('fallback',23n,1100),/CLIENT_SIGNATURE/);
  } finally { rmSync(out,{recursive:true,force:true}); }
});
