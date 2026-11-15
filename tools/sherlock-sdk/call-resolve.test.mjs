// FAL-NETOPS-4 (FRC-013, FRC-003, FRC-005): the inclusion state of a submitted honest call is resolved from the
// node and the indexer, and an included call that the indexer has not shown is never marked refused.
// Replay of the network-operations oracle trace `NetworkOps.pin_resolve`
// (ops toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 2, o 14 1]: included = true, savedRefused = false), and the fault
// test with a stub indexer that lags 90 s. No network, no SDK: node --test call-resolve.test.mjs
import assert from 'node:assert/strict';
import test from 'node:test';
import {resolveInclusion, UNKNOWN_VERDICTS} from './call-resolve.mjs';

// A fake clock: sleep advances it; the stub indexer shows the call `lagS` seconds after the start.
const clock = () => {
  let t = 0;
  return {now: () => t, sleep: async ms => { t += ms; }};
};
const indexerLagging = (c, lagS) => async () => c.now() >= lagS * 1000;
const block = {blockHash: '0xf0c07f3af59030233d1cd51091c77253b6aa33e9c980fbc7f1760f16b4296beb', blockHeight: '115187'};

test('FAL-NETOPS-4 pin_resolve: a timed-out call that the node has in a block resolves as included, not refused', async () => {
  // o 13 2: the submit timed out (PVW-008), so the state is unknown; o 14 1: the node and the indexer resolve it.
  for (const verdict of ['not-included-timeout', 'no-node-verdict']) assert.ok(UNKNOWN_VERDICTS.includes(verdict), verdict);
  const c = clock();
  const r = await resolveInclusion({verdict: 'not-included-timeout', ...block},
    {indexed: indexerLagging(c, 30), nodeHasBlock: async () => true, ttlExpired: () => false, sleep: c.sleep});
  assert.equal(r.state, 'indexed');      // included = true
  assert.equal(r.refused, null);         // savedRefused = false
});

test('FAL-NETOPS-4 fault: the indexer lags 90 s after inclusion; the call stays saved and unrefused', async () => {
  const c = clock();
  const r = await resolveInclusion({verdict: 'included', ...block},
    {indexed: indexerLagging(c, 90), nodeHasBlock: async b => b.blockHash === block.blockHash, ttlExpired: () => false, sleep: c.sleep});
  assert.equal(r.refused, null, 'an included call is never marked refused');
  assert.equal(r.state, 'included, not indexed', 'the run stops; a rerun reads the call back');
  assert.ok(c.now() >= 60_000, 'the indexer was given the 60 s wait first');
});

test('FAL-NETOPS-4: a sending call with no node record after its TTL is not included (FRC-003 reuses it), never refused', async () => {
  const c = clock();
  const r = await resolveInclusion({verdict: 'sending'},
    {indexed: async () => false, nodeHasBlock: async () => false, ttlExpired: () => true, sleep: c.sleep});
  assert.deepEqual(r, {state: 'not included', refused: null});
  const p = await resolveInclusion({verdict: 'sending'},
    {indexed: async () => false, nodeHasBlock: async () => false, ttlExpired: () => false, sleep: clock().sleep});
  assert.deepEqual(p, {state: 'pending', refused: null}, 'inside the TTL the state stays unknown: no new submission');
});

test('FAL-NETOPS-4: a block the node no longer has is not taken as inclusion', async () => {
  const r = await resolveInclusion({verdict: 'included', ...block},
    {indexed: async () => false, nodeHasBlock: async () => false, ttlExpired: () => false, sleep: clock().sleep});
  assert.deepEqual(r, {state: 'pending', refused: null});
});
