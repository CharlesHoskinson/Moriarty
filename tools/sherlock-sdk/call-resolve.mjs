// FRC-013 (FAL-NETOPS-4): the inclusion state of a submitted honest call is resolved from the indexer and the node
// before the ledger runner builds another submission of the same claim. The resolution never marks the saved call
// refused: only a node refusal does that (FRC-005, at the submit). Network-operations oracle `doResolve`.
//   indexed                 the indexer shows the call (readback follows)
//   included, not indexed   the node reported InBlock and still has that block at that height: keep the call saved,
//                           stop the run; a rerun reads it back
//   not included            no node record and the intent TTL is over: FRC-003 then reuses the saved call while its
//                           TTL lasts, or proves again (FAL-NETOPS-2, LEAD PICK)
//   pending                 otherwise: the state stays unknown, nothing is submitted, a rerun resolves it
// Submit verdicts with an unknown inclusion state: `sending` (the runner stopped during the submit), `included`
// (InBlock, not yet indexed), `not-included-timeout` (PVW-008) and `no-node-verdict` (PVW-007).
export const UNKNOWN_VERDICTS = ['sending', 'included', 'not-included-timeout', 'no-node-verdict'];

export async function resolveInclusion(sub, {indexed, nodeHasBlock, ttlExpired, sleep, waitMs = 60_000, stepMs = 2000}) {
  if (await indexed()) return {state: 'indexed', refused: null};
  for (let k = 0; k < waitMs / stepMs; k++) { await sleep(stepMs); if (await indexed()) return {state: 'indexed', refused: null}; }
  if (sub.blockHash && sub.blockHeight !== undefined && await nodeHasBlock({blockHash: sub.blockHash, blockHeight: sub.blockHeight}))
    return {state: 'included, not indexed', refused: null};
  if (ttlExpired()) return {state: 'not included', refused: null};
  return {state: 'pending', refused: null};
}

// The node lookup over its HTTP JSON-RPC: the block of the InBlock event is still the node's block at that height.
export const nodeHasBlockRpc = url => async ({blockHash, blockHeight}) => {
  const r = await fetch(url, {method: 'POST', headers: {'content-type': 'application/json'},
    body: JSON.stringify({jsonrpc: '2.0', id: 1, method: 'chain_getBlockHash', params: [Number(blockHeight)]})});
  const j = await r.json();
  return typeof j.result === 'string' && j.result.toLowerCase() === String(blockHash).toLowerCase();
};
