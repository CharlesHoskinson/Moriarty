// Read-only Preview wallet observation. Run with the installed hello-world tsx.
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
process.umask(0o077);
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
const log = path.join(evidence, `diagnose-${run}.ndjson`);
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

const dustParams = p => Object.fromEntries(['nightDustRatio','generationDecayRate','dustGracePeriodSeconds','timeToCapSeconds'].map(k=>[k,p[k]]));
try {
 const state = await Promise.race([ctx.wallet.waitForSyncedState(), new Promise((_, reject) => setTimeout(() => reject(Error('Sync timeout')), Number(process.env.MIDNIGHT_SYNC_TIMEOUT_MS ?? 240000)))]);
 const d=state.dust;
 const expiredCopy=d.state.state.processTtls(new Date(Date.now()+Number(d.state.state.params.dustGracePeriodSeconds)*1000+1000));
 await emit({event:'read-only-expired-copy', liveUtxos:d.state.state.utxos.length, copiedUtxos:expiredCopy.utxos.map(c=>({ctime:c.ctime,seq:c.seq})), copiedBalance:expiredCopy.walletBalance(new Date())});
 await emit({event:'synced-diagnostics', restored:ctx.restored, isSynced:state.isSynced, nightSmallestUnit:state.unshielded.balances[unshieldedToken().raw], nightCoins:state.unshielded.availableCoins.map(c=>({value:c.value,registered:c.meta?.registeredForDustGeneration})), dust:{syncTime:d.state.state.syncTime, progress:d.progress, balance:d.balance(new Date()), available:d.availableCoins.map(c=>({generatedNow:c.generatedNow,ctime:c.token.ctime,seq:c.token.seq})), pending:d.pendingCoins.map(c=>({generatedNow:c.generatedNow,ctime:c.token.ctime,seq:c.token.seq})), params:dustParams(d.state.state.params)}});
 const response=await fetch(receipt.configuration.indexer,{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({query:'{block {height hash timestamp ledgerParameters}}'}),signal:AbortSignal.timeout(20000)});
 const result=await response.json();
 const ledger=require('@midnight-ntwrk/ledger-v8');
 const block=result.data?.block;
 const params=block?.ledgerParameters ? ledger.LedgerParameters.deserialize(Buffer.from(block.ledgerParameters,'hex')) : null;
 await emit({event:'indexer-tip',httpStatus:response.status,errors:result.errors,block:block ? {height:block.height,hash:block.hash,timestamp:block.timestamp,dust:params ? dustParams(params.dust) : null}:null});
} catch(error) {await emit({event:'failure',message:error.message});process.exitCode=1;} finally {await persistWalletState('preview',ctx,stateDir);await ctx.wallet.stop();}
process.exit(process.exitCode??0);
