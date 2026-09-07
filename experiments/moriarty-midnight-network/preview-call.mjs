// One instrumented call on the existing Preview contract; --submit is required.
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
process.umask(0o077);
if (!process.argv.includes('--submit')) throw Error('Explicit --submit required');
const repo = path.resolve(new URL('../..', import.meta.url).pathname);
const runtime = path.join(repo, '.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world');
const require = createRequire(path.join(runtime, 'package.json'));
const { WebSocket } = require('ws');
globalThis.WebSocket = WebSocket;
const { createWallet, persistWalletState, unshieldedToken } = await import(pathToFileURL(path.join(runtime, 'src/wallet.ts')).href);
const evidence = path.join(repo, 'evidence/midnight-preview-2026-09-07');
const receipt = JSON.parse(await fs.readFile(path.join(evidence, 'wallet-public.json'), 'utf8'));
if (receipt.network !== 'preview') throw Error('Expected dedicated Preview receipt');
const stateDir = process.env.MIDNIGHT_PREVIEW_STATE_DIR ?? '/home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907';
await fs.mkdir(stateDir, { recursive: true, mode: 0o700 });
const run = new Date().toISOString().replace(/[:.]/g, '-');
const log = path.join(evidence, `call-${run}.ndjson`);
const emit = async (item) => {
  const line = JSON.stringify({ at: new Date().toISOString(), ...item }, (_, value) => typeof value === 'bigint' ? value.toString() : value);
  console.log(line);
  await fs.appendFile(log, line + '\n', { mode: 0o600 });
};
const seed = (await fs.readFile(receipt.seedFile, 'utf8')).trim();
const ctx = await createWallet({ network: 'preview', networkConfig: { ...receipt.configuration, composeServices: ['proof-server'] }, seed, cwd: stateDir });
if (ctx.unshieldedKeystore.getBech32Address().asString() !== receipt.address) {
  await ctx.wallet.stop();
  throw Error('Wallet derivation differs from faucet address');
}


const address='ddc676b1bfb36f29665caf17b4ae5016595af74c3dc4165e638b6af77ef40b4f';
const message='Moriarty Preview settlement test';
const dependency = name => import(pathToFileURL(require.resolve(name).replace(/\.cjs$/, '.mjs')).href);
const { findDeployedContract, submitCallTx } = await dependency('@midnight-ntwrk/midnight-js-contracts');
const { httpClientProofProvider } = await dependency('@midnight-ntwrk/midnight-js-http-client-proof-provider');
const { indexerPublicDataProvider } = await dependency('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const { levelPrivateStateProvider } = await dependency('@midnight-ntwrk/midnight-js-level-private-state-provider');
const { NodeZkConfigProvider } = await dependency('@midnight-ntwrk/midnight-js-node-zk-config-provider');
const { CompiledContract } = await dependency('@midnight-ntwrk/midnight-js-protocol/compact-js');
const zkPath=path.join(runtime,'contracts/managed/hello-world');
const contract=await import(pathToFileURL(path.join(zkPath,'contract/index.js')).href);
const compiledContract=CompiledContract.make('hello-world',contract.Contract).pipe(CompiledContract.withVacantWitnesses,CompiledContract.withCompiledFileAssets(zkPath));
const summarize=state=>({isSynced:state.isSynced,dustSyncTime:state.dust.state.state.syncTime,dustCommitmentRoot:state.dust.state.state.commitmentTreeRoot(),dustGenerationRoot:state.dust.state.state.generatingTreeRoot(),balance:state.dust.balance(new Date()),available:state.dust.availableCoins.map(c=>({ctime:c.token.ctime,seq:c.token.seq,generatedNow:c.generatedNow})),pending:state.dust.pendingCoins.length});
const timeout=setTimeout(()=>{console.error('Call experiment timeout; inspect retained transaction before any retry');process.exit(124);},180000);
let latest;
const sub=ctx.wallet.state().subscribe(s=>{latest=s;});
try {
 const initial=await ctx.wallet.waitForSyncedState();
 await emit({event:'initial',stateDirectory:stateDir,restored:ctx.restored,...summarize(initial)});
 if(initial.dust.availableCoins.length===0)throw Error('No available DUST: do not submit');
 const walletProvider={
  getCoinPublicKey:()=>ctx.shieldedSecretKeys.coinPublicKey,
  getEncryptionPublicKey:()=>ctx.shieldedSecretKeys.encryptionPublicKey,
  async balanceTx(tx,ttl) {
   const before=await ctx.wallet.waitForSyncedState();
   await emit({event:'before-balance',...summarize(before)});
   await persistWalletState('preview',ctx,stateDir);
   await fs.copyFile(path.join(stateDir,'.midnight-wallet-state/preview/dust.json'),path.join(stateDir,`before-call-${run}-dust.json`),1);
   const recipe=await ctx.wallet.balanceUnboundTransaction(tx,{shieldedSecretKeys:ctx.shieldedSecretKeys,dustSecretKey:ctx.dustSecretKey},{ttl:ttl??new Date(Date.now()+30*60*1000)});
   const finalized=await ctx.wallet.finalizeRecipe(recipe);
   const privateFile=path.join(stateDir,`call-${run}.tx`);
   await fs.writeFile(privateFile,finalized.serialize(),{flag:'wx',mode:0o600});
   await emit({event:'finalized-before-submission',txHash:finalized.transactionHash(),identifiers:finalized.identifiers(),dustActions:[...(finalized.intents?.entries()??[])].filter(([,i])=>i.dustActions).map(([segment,i])=>({segment,ctime:i.dustActions.ctime,spends:i.dustActions.spends.length})),privateTransactionFile:privateFile});
   return finalized;
  },
  async submitTx(tx) {
   try {return await ctx.wallet.submitTransaction(tx);}
   catch(error) {await emit({event:'submission-rejected',message:error.message,...(latest?summarize(latest):{})});throw error;}
  }
 };
 const providers={privateStateProvider:levelPrivateStateProvider({privateStateStoreName:'hello-world-state',accountId:receipt.address,privateStoragePasswordProvider:()=> 'Local-Devnet-Development-Placeholder-1'}),publicDataProvider:indexerPublicDataProvider(receipt.configuration.indexer,receipt.configuration.indexerWS),zkConfigProvider:new NodeZkConfigProvider(zkPath),proofProvider:httpClientProofProvider(receipt.configuration.proofServer,new NodeZkConfigProvider(zkPath)),walletProvider,midnightProvider:walletProvider};
 await findDeployedContract(providers,{compiledContract,contractAddress:address,privateStateId:'helloWorldPrivateState',initialPrivateState:{}});
 const result=await submitCallTx(providers,{compiledContract,contractAddress:address,privateStateId:'helloWorldPrivateState',circuitId:'storeMessage',args:[message]});
 await emit({event:'call-result',txId:result.public.txId,blockHeight:result.public.blockHeight});
 const state=await providers.publicDataProvider.queryContractState(address);
 const readBack=Buffer.from(contract.ledger(state.data).message).toString();
 await emit({event:'readback',readBack,pass:readBack===message});
 if(readBack!==message)throw Error('Message differs');
} catch(error) {await emit({event:'failure',message:error.message,cause:error.cause?.message});process.exitCode=1;}
finally {clearTimeout(timeout);sub.unsubscribe();await persistWalletState('preview',ctx,stateDir);await ctx.wallet.stop();}
process.exit(process.exitCode??0);
