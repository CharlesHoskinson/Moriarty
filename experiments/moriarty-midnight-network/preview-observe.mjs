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
const stateDir = '/home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907';
await fs.mkdir(stateDir, { recursive: true, mode: 0o700 });
const run = new Date().toISOString().replace(/[:.]/g, '-');
const log = path.join(evidence, `observe-${run}.ndjson`);
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
await emit({ event: 'started', network: 'preview', address: receipt.address, restored: ctx.restored, durationSeconds: 180 });
let last, latest;
const subscription = ctx.wallet.state().subscribe({
  next(state) {
    latest = state;
    const now = Date.now();
    if (last && now - last < 15_000) return;
    last = now;
    const progress = {};
    for (const name of ['shielded', 'unshielded', 'dust']) {
      const value = state[name]?.state?.progress ?? state[name]?.progress;
      progress[name] = value ? Object.fromEntries(Object.entries(value).filter(([, v]) => ['string', 'number', 'bigint', 'boolean'].includes(typeof v))) : null;
    }
    void emit({ event: 'progress', isSynced: state.isSynced, nightSmallestUnit: (state.unshielded.balances[unshieldedToken().raw] ?? 0n).toString(), progress });
  },
  error(error) { void emit({ event: 'stream-error', message: error.message }); },
});
let finish;
const done = new Promise(resolve => { finish = resolve; });
const timer = setTimeout(finish, 180_000);
process.once('SIGTERM', finish);
process.once('SIGINT', finish);
await done;
clearTimeout(timer);
subscription.unsubscribe();
await persistWalletState('preview', ctx, stateDir);
await ctx.wallet.stop();
await emit({ event: 'stopped', isSynced: latest?.isSynced ?? false, nightSmallestUnit: latest ? (latest.unshielded.balances[unshieldedToken().raw] ?? 0n).toString() : null, stateDirectory: stateDir, persistence: 'attempted; inspect child files before claiming full restore', transactionsSubmitted: 0 });
process.exit(0);
