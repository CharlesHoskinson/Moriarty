// FAL-LEDGER-VERIFICATION-1 (MID-002, SLC-001, SLC-004): the verifier key of each SherlockLedger entry.
//   T10/T17 three-entry contract (acceptCollection, acceptRun, openClaim): all three are write entries, wrap key on each.
//   SLC-001 four-entry contract (acceptCollection, acceptRun, rootAccepted, runAccepted): the wrap key on the write
//   entries; each read entry keeps the compiled native key of the entry (compactc keys/<entry>.verifier, SLC-004).
// The entries come from the compiled build (keys/*.verifier), so the runner installs and checks what compactc built.
import {readdirSync} from 'node:fs';

export const T10_ENTRIES = ['acceptCollection', 'acceptRun', 'openClaim'];
export const SLC_ENTRIES = ['acceptCollection', 'acceptRun', 'rootAccepted', 'runAccepted'];
export const WRITE_ENTRIES = ['acceptCollection', 'acceptRun', 'openClaim'];
export const READ_ENTRIES = ['rootAccepted', 'runAccepted'];

export const compiledEntries = keysDir => readdirSync(keysDir).filter(f => f.endsWith('.verifier')).map(f => f.slice(0, -9)).sort();

/** entry -> {kind: 'wrap' | 'native', bytes}. `nativeKey(entry)` reads the compiled key of a read entry. */
export function keyPlan(entries, wrapVk, nativeKey) {
  const set = [...entries].sort().join();
  if (set !== T10_ENTRIES.join() && set !== SLC_ENTRIES.join())
    throw new Error(`entries ${set}: neither the T10 contract (${T10_ENTRIES}) nor SLC-001 (${SLC_ENTRIES})`);
  const plan = {};
  for (const e of [...entries].sort())
    plan[e] = READ_ENTRIES.includes(e) ? {kind: 'native', bytes: nativeKey(e)} : {kind: 'wrap', bytes: wrapVk};
  return plan;
}

/** Each installed key, read back with `installedOf(entry)`, against the plan. */
export function checkInstalled(plan, installedOf) {
  const rows = {};
  for (const [e, p] of Object.entries(plan)) {
    const k = Buffer.from(installedOf(e));
    rows[e] = {kind: p.kind, equals: k.equals(Buffer.from(p.bytes)), bytes: k.length};
  }
  return {ok: Object.values(rows).every(r => r.equals), rows};
}
