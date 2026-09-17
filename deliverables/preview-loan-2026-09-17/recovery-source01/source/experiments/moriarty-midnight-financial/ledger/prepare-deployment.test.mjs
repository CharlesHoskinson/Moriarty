import test from 'node:test';
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
const require=createRequire('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/package.json');
const ledger=await import(pathToFileURL(require.resolve('@midnight-ntwrk/ledger-v8')).href);
const module=await import('./prepare-deployment.mjs').catch(()=>({}));

function fixture() {
  const deployment=new ledger.ContractDeploy(new ledger.ContractState());
  const tx=ledger.Transaction.fromParts('undeployed',undefined,undefined,ledger.Intent.new(new Date('2030-01-01')).addDeploy(deployment));
  const options={compiledContract:{},privateStateId:'sp05-loan',initialPrivateState:{},args:[new Uint8Array(32),new Uint8Array(32).fill(1),{bytes:new Uint8Array(32).fill(2)},{bytes:new Uint8Array(32).fill(3)},new Uint8Array(32).fill(4),new Uint8Array(32).fill(5)]};
  const seen=[],key='private-signing-key-canary';
  const sdk={
    createUnprovenDeployTx:async(_providers,opts)=>{seen.push('prepare');assert.equal(opts.signingKey,key);return {public:{contractAddress:deployment.address,initialContractState:new ledger.ContractState()},private:{unprovenTx:tx,signingKey:key,initialPrivateState:{}}};},
    submitTx:async(_providers,{unprovenTx})=>{seen.push('submit');assert.deepEqual(unprovenTx.serialize(),tx.serialize());return {status:'SucceedEntirely',txId:'observed-deploy-id'};},
    submitCallTx:async()=>{seen.push('call');return 'actual-call-adapter';},
  };
  const providers={privateStateProvider:{setContractAddress:a=>{assert.equal(a,deployment.address);seen.push('address');},set:async(id,state)=>{assert.equal(id,options.privateStateId);assert.deepEqual(state,{});seen.push('state');},setSigningKey:async(a,k)=>{assert.equal(a,deployment.address);assert.equal(k,key);seen.push('key');}}};
  return {options,seen,key,sdk,providers,tx,deployment};
}
async function prepare(f) {
  assert.equal(typeof module.prepareFinancialDeployment,'function');
  return module.prepareFinancialDeployment({deploymentOptions:f.options,publicWalletProvider:{},zkConfigProvider:{},signingKey:f.key,ledger,sdk:f.sdk});
}

test('preparation fixes the contract address before funding and submits the same transaction once',async()=>{
  const f=fixture(),p=await prepare(f);
  assert.deepEqual(f.seen,['prepare']);
  assert.equal(p.public.contractAddress,f.deployment.address);
  assert.equal(p.public.scope,'prepared-only');
  assert.ok(!JSON.stringify(p.public).includes(f.key));
  const result=await p.driverSdk.deployContract(f.providers,f.options);
  assert.deepEqual(f.seen,['prepare','submit','address','state','key']);
  assert.deepEqual(result,{deployTxData:{public:{contractAddress:f.deployment.address,txId:'observed-deploy-id'}}});
  await assert.rejects(p.driverSdk.deployContract(f.providers,f.options),/ALREADY_CONSUMED/);
  assert.equal(await p.driverSdk.submitCallTx(),'actual-call-adapter');
});

test('prepared deployment rejects changed constructor or private state before submission',async()=>{
  for(const mutate of [o=>{o.args[0][0]=9;},o=>{o.initialPrivateState.extra='private';},o=>{o.privateStateId='sp05-swap';},o=>{o.compiledContract={};}]) {
    const f=fixture(),p=await prepare(f);mutate(f.options);
    await assert.rejects(p.driverSdk.deployContract(f.providers,f.options),/PREPARED_DEPLOYMENT_OPTIONS_CHANGED/);
    assert.deepEqual(f.seen,['prepare']);
  }
});

test('submission failure is single-use and stores no successful deployment state',async()=>{
  const f=fixture();f.sdk.submitTx=async()=>{f.seen.push('submit');return {status:'FailFallible',txId:'failed-id'};};
  const p=await prepare(f);
  await assert.rejects(p.driverSdk.deployContract(f.providers,f.options),/DEPLOYMENT_NOT_SUCCEEDED/);
  await assert.rejects(p.driverSdk.deployContract(f.providers,f.options),/ALREADY_CONSUMED/);
  assert.deepEqual(f.seen,['prepare','submit']);
});

test('preparation verifies native deployment identity and retains an immutable byte snapshot',async()=>{
  const f=fixture(),original=f.sdk.createUnprovenDeployTx;
  f.sdk.createUnprovenDeployTx=async(...args)=>{const d=await original(...args);d.public.contractAddress='ff'.repeat(32);return d;};
  await assert.rejects(prepare(f),/PREPARED_DEPLOYMENT_IDENTITY/);
  const g=fixture(),raw=Buffer.from(g.tx.serialize());
  g.sdk.submitTx=async(_providers,{unprovenTx})=>{assert.deepEqual(Buffer.from(unprovenTx.serialize()),raw);return {status:'SucceedEntirely',txId:'observed-deploy-id'};};
  const p=await prepare(g);
  g.tx.intents=new Map();
  g.sdk.submitTx=async()=>{throw Error('MUTATED_ADAPTER');};
  const result=await p.driverSdk.deployContract(g.providers,g.options);
  assert.equal(result.deployTxData.public.contractAddress,g.deployment.address);
});
