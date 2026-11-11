// Beat ids, titles and narration of scene 8, copied from STORYBOARD.md (backticks as <code>).
export const BEAT_TEXT = [
  {
    "id": "check-at-each-node",
    "title": "Check at each node",
    "narration": "Every Midnight node does this on-chain work itself. The stored claim reaches network validation as the result of the transaction's contract state transition. Midnight nodes on ledger 9.1.0.1 each verify the transaction independently. Each uses the contract's verifier key and checks the bound public values and contract operations against ledger state. Each node rejects an invalid proof or a replay on its own checks."
  },
  {
    "id": "keep-verification-fixed",
    "title": "Fixed verification cost",
    "narration": "Each node uses the same verifier key for every program. It checks the final decider proof. It does not execute the witness tape, check each leaf or run the Sherlock chip. The proof remains about 6.4 KB. The cost of checking that proof does not depend on the length of the run. Longer execution adds prover work and leaves the node's run-verification work unchanged."
  },
  {
    "id": "accept-the-run-fact",
    "title": "Accepted run fact",
    "narration": "After acceptance, the claim is a fact on chain: program <code>prog_d</code> on input <code>input_d</code> produced <code>outcome_d</code> in <code>n_steps</code> steps. Any contract or later proof can rely on that statement. A contract identifies it through <code>claim_d</code> and the stored record. A later proof checks the sealed claim and any required value opening before using its result."
  },
  {
    "id": "join-complete-runs",
    "title": "Join of complete runs",
    "narration": "Collections combine sealed runs with JOIN, the relation that joins claim sets. It computes <code>set_root = H(SHK_SET_JOIN, l, r)</code>, where <code>H</code> is Poseidon and <code>l</code> and <code>r</code> are the authenticated child roots. The domain tag identifies a set join. Recomputing this digest binds both children in order and rejects a substituted child. Membership openings recover each included claim from the resulting root."
  },
  {
    "id": "bind-the-proof-graph",
    "title": "Proof graph binding",
    "narration": "Runs join in any tree or DAG, a directed graph without cycles that permits shared dependencies. <code>shape_d</code> commits to the proof graph through relation types, enabled child positions, key selections, child shapes and authenticated work descriptors. Each node derives these fields from its checked children and work. A proof cannot claim another graph. The shape contains no physical state commitment or address."
  },
  {
    "id": "settle-the-collection",
    "title": "Collection settlement",
    "narration": "A collection settles as <code>[reg_root, set_root, shape_d]</code>. <code>reg_root</code> commits to admitted verifier keys and their interpretation. <code>set_root</code> commits to the authenticated claims and their dependency coverage. <code>shape_d</code> commits to the graph that combines them. Verification binds these values and requires the registry root to match contract state. A stale registry fails. The final proof size and verification cost stay independent of graph depth and claim count."
  },
  {
    "id": "preserve-claim-identity",
    "title": "Claim identity preserved",
    "narration": "Different groupings of the same claims can produce different collection roots and shape digests. These commitments describe the collection. Each member keeps its <code>claim_d</code>. Acceptance, replay refusal and consumption use claim identity. A shared claim can feed several branches, and a second path grants no second settlement. Collection verification creates no repeated per-claim effect keyed by collection shape."
  },
  {
    "id": "follow-the-whole-path",
    "title": "The whole path",
    "narration": "Moriarty source becomes the ROM program image with <code>prog_d</code> and <code>input_d</code>. Execution hands over the witness tape and its outcome frame. Leaf proving turns slices into slabs with their public pins. Nodes join the slabs into a root slab carrying its accumulator ring. The decider hands over a seal and <code>[prog_d, input_d, outcome_d, n_steps]</code>. The transaction carries them in an envelope. The contract produces the stored claim drawer. Midnight nodes accept the claim, with the same public values bound throughout."
  }
];
