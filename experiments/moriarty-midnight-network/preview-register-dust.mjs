// Register only this receipt's existing NIGHT coins for DUST; no contract action.
// Must run with tsx after the read-only observer has stopped.
import fs from 'node:fs/promises';
import path from 'node:path';
import {createRequire} from 'node:module';
import {fileURLToPath, pathToFileURL} from 'node:url';

process.umask(0o077);
if (process.argv.slice(2).some(a=>a!=='--submit')) throw Error('Usage: preview-register-dust.mjs [--submit]');
const submit=process.argv.includes('--submit');
const repo=fileURLToPath(new URL('../../',import.meta.url));
const scaffold=path.join(repo,'experiments/moriarty-midnight-network/hello-world');
const require=createRequire(path.join(scaffold,'package.json'));
globalThis.WebSocket=require('ws').WebSocket;
const {firstValueFrom,filter,timeout}=require('rxjs');
const {createWallet,persistWalletState,unshieldedToken}=await import(pathToFileURL(path.join(scaffold,'src/wallet.ts')).href);
const evidence=process.env.MIDNIGHT_PREVIEW_EVIDENCE_DIRECTORY;
if (!evidence || !path.isAbsolute(evidence)) throw Error('Select an absolute Preview evidence directory');
const receipt=JSON.parse(await fs.readFile(path.join(evidence,'wallet-public.json'),'utf8'));
if(receipt.network!=='preview'||receipt.configuration.node!=='https://rpc.preview.midnight.network'||!path.isAbsolute(receipt.stateDirectory)) throw Error('Expected Preview receipt');
const stateDir=receipt.stateDirectory;
const seedStat=await fs.lstat(receipt.seedFile);
if(!seedStat.isFile()||seedStat.isSymbolicLink()||seedStat.uid!==process.getuid()||(seedStat.mode&0o077))throw Error('Private seed permissions invalid');
const run=new Date().toISOString().replace(/[:.]/g,'-');
const log=path.join(evidence,`dust-registration-${run}.ndjson`);
const emit=async item=>{const line=JSON.stringify({at:new Date().toISOString(),...item},(_,v)=>typeof v==='bigint'?v.toString():v);console.log(line);await fs.appendFile(log,line+'\n',{mode:0o600});};
const rpc=await fetch(receipt.configuration.node,{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({jsonrpc:'2.0',id:1,method:'system_chain',params:[]}),signal:AbortSignal.timeout(15000)}).then(r=>r.json());
if(rpc.result!=='Midnight Preview')throw Error('Wrong chain');
const ctx=await createWallet({network:'preview',networkConfig:{...receipt.configuration,composeServices:['proof-server']},seed:(await fs.readFile(receipt.seedFile,'utf8')).trim(),cwd:stateDir});
const deadline=setTimeout(()=>{console.error('Registration helper exceeded 300-second bound');process.exit(124);},300000);
try {
  if(ctx.unshieldedKeystore.getBech32Address().asString()!==receipt.address)throw Error('Wallet identity mismatch');
  const state=await firstValueFrom(ctx.wallet.state().pipe(filter(s=>s.isSynced),timeout({first:90000})));
  // SDK UtxoWithMeta stores the token identifier at utxo.type (SyncSchema.d.ts).
  const coins=state.unshielded.availableCoins.filter(c=>c.utxo.type===unshieldedToken().raw&&!c.meta?.registeredForDustGeneration);
  await emit({event:'ready',address:receipt.address,restored:ctx.restored,isSynced:state.isSynced,nightSmallestUnit:state.unshielded.balances[unshieldedToken().raw]??0n,dustBalance:state.dust.balance(new Date()),unregisteredCoins:coins.length,submit});
  if(submit&&coins.length>0){
    // Exclusive snapshot backup precedes the wallet mutation.
    const backup=path.join(stateDir,'before-dust-registration-'+run);
    await fs.mkdir(backup,{mode:0o700});
    for(const kind of ['shielded','unshielded','dust'])await fs.copyFile(path.join(stateDir,'.midnight-wallet-state/preview',kind+'.json'),path.join(backup,kind+'.json'),1);
    const recipe=await ctx.wallet.registerNightUtxosForDustGeneration(coins,ctx.unshieldedKeystore.getPublicKey(),payload=>ctx.unshieldedKeystore.signData(payload));
    const finalized=await ctx.wallet.finalizeRecipe(recipe);
    const estimatedFee=await ctx.wallet.calculateTransactionFee(finalized);
    const txFile=path.join(stateDir,'dust-registration-'+run+'.tx');
    await fs.writeFile(txFile,finalized.serialize(),{flag:'wx',mode:0o600});
    await emit({event:'finalized-before-submission',transactionHash:finalized.transactionHash(),identifiers:finalized.identifiers(),estimatedFee,privateTransactionFile:txFile,scope:'Register existing NIGHT UTXOs for wallet DUST generation; no contract call or deployment'});
    // Submit over HTTP: the SDK's WebSocket reconnect/disconnect path has failed
    // before acceptance. Keep the same signed SDK recipe and observe finality
    // through the wallet/indexer, independently of the transport response.
    const {ApiPromise,HttpProvider}=require('@polkadot/api');
    const api=await ApiPromise.create({provider:new HttpProvider(receipt.configuration.node),noInitWarn:true});
    try {
      const extrinsic=api.tx.midnight.sendMnTransaction('0x'+Buffer.from(finalized.serialize()).toString('hex'));
      const extrinsicHash=await extrinsic.send();
      await emit({event:'submitted',transport:'http',extrinsicHash:extrinsicHash.toHex(),transactionHash:finalized.transactionHash(),identifiers:finalized.identifiers()});
    } finally {await api.disconnect();}
  }
  if(submit){
    const after=await firstValueFrom(ctx.wallet.state().pipe(filter(s=>s.isSynced&&s.dust.balance(new Date())>0n),timeout({first:120000})));
    await emit({event:'dust-available',isSynced:after.isSynced,dustBalance:after.dust.balance(new Date()),nightSmallestUnit:after.unshielded.balances[unshieldedToken().raw]??0n,availableDustCoins:after.dust.availableCoins.length});
  }
}catch(error){
  const causes=[];
  for(let cause=error,depth=0;cause&&depth<5;cause=cause.cause,depth++) {
    const message=String(cause.message??cause._tag??cause.name??'Unknown error').replace(/(?:0x)?[a-fA-F0-9]{64,}/g,'[hex redacted]').slice(0,1000);
    causes.push({name:cause._tag??cause.name??'Error',message});
  }
  await emit({event:'failure',causes});process.exitCode=1;
}
finally{await persistWalletState('preview',ctx,stateDir);await ctx.wallet.stop();clearTimeout(deadline);}
process.exit(process.exitCode??0);
