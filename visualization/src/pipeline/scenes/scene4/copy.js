// Beat copy of scene 4, taken from the storyboard text. Narration, titles and label text are final.
export const COPY = [
  {
    id: "leaf-arrival", title: "Leaf arrival", duration: 22,
    narration: "Leaf proofs arrive with their authenticated public values. The node treats each proof as a segment, a consecutive interval of the run. Its self-recursive circuit verifies leaf proofs and proofs of its own circuit under pinned keys: keys that the verifier fixes. The circuit size setting is <code>k = 20</code> throughout the tree.",
    labels: [
      { text: "Leaf proof", kind: null },
      { text: "Node proof", kind: null },
      { text: "Pinned keys", kind: null },
      { text: "<code>k = 20</code>", kind: null },
    ],
  },
  {
    id: "authenticate-children", title: "Authenticated children", duration: 24,
    narration: "Child verification binds each proof to the exact values it publishes. A key selector chooses the permitted proof type and its matching key together. A prover cannot attach altered boundaries to a valid proof or swap in a circuit that has weaker checks. Unused child slots and the initial carry use neutral encodings, which add no verification obligation.",
    labels: [
      { text: "Child proof", kind: null },
      { text: "Public values", kind: null },
      { text: "Key selector", kind: null },
      { text: "Neutral encoding", kind: null },
    ],
  },
  {
    id: "bound-the-heights", title: "Bounded heights", duration: 22,
    narration: "A leaf enters with <code>h_lo = h_pre</code> and <code>h_hi = h_pre + steps</code>. Heights count completed machine steps. The node checks that <code>steps</code> is positive and fits the leaf, and that both heights stay below 2<sup>64</sup>. The 64-bit counter cannot wrap, so no invalid interval passes as valid. Its range is not a circuit size.",
    labels: [
      { text: "<code>h_pre</code>", kind: null },
      { text: "<code>steps</code>", kind: null },
      { text: "<code>h_lo</code>", kind: null },
      { text: "<code>h_hi</code>", kind: null },
      { text: "64-bit counter", kind: null },
      { text: "Below 2<sup>64</sup>", kind: null },
    ],
  },
  {
    id: "match-the-program", title: "Program match", duration: 16,
    narration: "MERGE combines adjacent segments. It first requires the left child's <code>prog_d</code> to equal the right child's <code>prog_d</code>, so both proofs describe the same program. Execution spliced from different programs cannot form a segment that claims one program identity.",
    labels: [
      { text: "MERGE", kind: null },
      { text: "Left segment", kind: null },
      { text: "Right segment", kind: null },
      { text: "<code>prog_d</code>", kind: null },
    ],
  },
  {
    id: "match-the-state", title: "State match", duration: 20,
    narration: "MERGE requires <code>L.post_d == R.pre_d</code>: the state where the left segment ends equals the state where the right segment begins. The boundary digest binds the entire header and body. Registers, stacks or memory replaced between valid segments fail this check, so disconnected executions cannot pass as one run.",
    labels: [
      { text: "<code>L.post_d</code>", kind: null },
      { text: "<code>R.pre_d</code>", kind: null },
      { text: "Equal boundary", kind: null },
    ],
  },
  {
    id: "match-the-height", title: "Height match", duration: 19,
    narration: "MERGE also requires <code>L.h_hi == R.h_lo</code>: the left segment's ending height equals the right segment's starting height. State equality alone does not fix this position in the run. With height equality, no skipped or overlapping interval can leave a gap or repeated steps in the claimed coverage.",
    labels: [
      { text: "<code>L.h_hi</code>", kind: null },
      { text: "<code>R.h_lo</code>", kind: null },
      { text: "Equal height", kind: null },
    ],
  },
  {
    id: "retain-the-outer-boundaries", title: "Outer boundaries", duration: 22,
    narration: "The merged segment publishes <code>(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)</code>. Its first state and height come from the left child. Its last state and height come from the right child. The matching inner digests leave the output, because the node proof has already established their equality. The next merge needs only the outer boundaries.",
    labels: [
      { text: "<code>prog_d</code>", kind: "program" },
      { text: "<code>L.pre_d</code>", kind: "state" },
      { text: "<code>R.post_d</code>", kind: "state" },
      { text: "<code>L.h_lo</code>", kind: "neutral" },
      { text: "<code>R.h_hi</code>", kind: "neutral" },
    ],
  },
  {
    id: "fold-the-obligations", title: "Folded obligations", duration: 23,
    narration: "Verifying a child proof leaves a pairing check, a cryptographic equation that completes proof verification. The node defers that pairing and folds the new check and every carried child check into one accumulator, a fixed-size encoding of deferred checks. Dropping a child's accumulator discards its verification obligation, so the circuit binds every enabled carry into the fold.",
    labels: [
      { text: "Child checks", kind: null },
      { text: "Fold", kind: null },
      { text: "Accumulator", kind: null },
      { text: "Deferred pairing", kind: null },
    ],
  },
  {
    id: "fixed-node-linear-work", title: "Fixed node, linear work", duration: 22,
    narration: "The accumulator keeps the same size as more checks enter it. A node at any tree height has the same circuit size and the same per-node proving cost. Longer runs require more leaf proofs and more node proofs. Their numbers grow in proportion to the number of machine steps, so total proving work grows linearly.",
    labels: [
      { text: "Fixed node size", kind: null },
      { text: "Fixed accumulator size", kind: null },
      { text: "Machine steps", kind: null },
      { text: "Leaf proofs", kind: null },
      { text: "Node proofs", kind: null },
      { text: "Linear work", kind: null },
    ],
  },
  {
    id: "node-proving-cost", title: "Node proving cost", duration: 18,
    narration: "On the reference host, one node proof takes about 65 s at 20 threads and uses 17 GB of memory. That cost is per node proof: a node near the root verifies accumulated work through the same circuit as a node near the leaves.",
    labels: [
      { text: "Reference host", kind: null },
      { text: "About 65 s", kind: null },
      { text: "20 threads", kind: null },
      { text: "17 GB", kind: null },
    ],
  },
  {
    id: "parallel-merge-trees", title: "Parallel merge trees", duration: 20,
    narration: "Machines prove independent leaves and ready merges at the same time. Every merge preserves execution order from left to right, while the tree can group adjacent segments in any shape. Work proceeds whenever the required child proofs are ready. On the reference host, three provers in parallel give 1.73x throughput.",
    labels: [
      { text: "Parallel provers", kind: null },
      { text: "Ready children", kind: null },
      { text: "Left to right", kind: null },
      { text: "1.73x throughput", kind: null },
    ],
  },
  {
    id: "include-new-work", title: "New work in a node", duration: 27,
    narration: "MERGE-WORK combines prior segment proofs with new machine steps inside the same node. The new work uses the leaf's execution checks. Its program must equal <code>prog_d</code>, its starting state must equal the right child's <code>post_d</code>, and its starting height must equal that child's ending height. The output height includes exactly the new steps and stays below 2<sup>64</sup>. Detached work and uncounted execution cannot enter the segment.",
    labels: [
      { text: "MERGE-WORK", kind: null },
      { text: "Prior segments", kind: null },
      { text: "New steps", kind: null },
      { text: "<code>prog_d</code>", kind: null },
      { text: "<code>post_d</code>", kind: null },
      { text: "Below 2<sup>64</sup>", kind: null },
    ],
  },
  {
    id: "join-complete-runs", title: "Join of complete runs", duration: 25,
    narration: "JOIN combines sets of claims, statements of complete runs. It computes <code>set_root = H(SHK_SET_JOIN, l, r)</code>, where <code>l</code> and <code>r</code> are the child set roots and <code>H</code> is Poseidon. The tag <code>SHK_SET_JOIN</code> identifies this hash as a set join. The ordered hash binds both child roots. JOIN needs no execution boundary between independent runs, as MERGE does, and carries no claim counter.",
    labels: [
      { text: "JOIN", kind: null },
      { text: "<code>SHK_SET_JOIN</code>", kind: null },
      { text: "<code>l</code>", kind: null },
      { text: "<code>r</code>", kind: null },
      { text: "<code>set_root</code>", kind: null },
    ],
  },
  {
    id: "share-claims-in-a-dag", title: "Shared claims in a DAG", duration: 23,
    narration: "A DAG, a directed acyclic graph, lets a proved claim feed multiple consumers without creating a cycle. Reusing a claim never licenses a duplicated run interval inside MERGE. The address-free shape digest <code>shape_d</code> binds the proof graph's connections and operation kinds. Different graph shapes preserve the claims they contain, and the same keys serve every depth.",
    labels: [
      { text: "Shared claim", kind: null },
      { text: "Consumers", kind: null },
      { text: "DAG", kind: null },
      { text: "<code>shape_d</code>", kind: null },
      { text: "Same keys", kind: null },
    ],
  },
  {
    id: "retain-a-checkpoint", title: "Checkpoint", duration: 24,
    narration: "CONTINUATION saves a checkpoint, the proved prefix and the witness data needed to continue it. The checkpoint retains the prefix proofs and accumulators, its ending height and <code>post_d</code>, registers, memory records, stack contents, program and input. Restoring it checks the key set and recomputes the committed state. Altered checkpoint data cannot give the suffix a different starting state.",
    labels: [
      { text: "CONTINUATION", kind: null },
      { text: "Checkpoint", kind: null },
      { text: "Proved prefix", kind: null },
      { text: "Retained witness", kind: null },
      { text: "<code>post_d</code>", kind: null },
      { text: "Key set", kind: null },
    ],
  },
  {
    id: "resume-the-run", title: "Resumed run", duration: 23,
    narration: "The resumed suffix starts at the saved height, and its <code>pre_d</code> equals the prefix's <code>post_d</code>. MERGE checks these equalities exactly as it checks an uninterrupted run. Pausing and resuming execute no machine step and change no fuel or logical counter. Proving the run in parts gives the same final statement as proving it in one go.",
    labels: [
      { text: "Proved prefix", kind: null },
      { text: "Resumed suffix", kind: null },
      { text: "<code>post_d</code>", kind: null },
      { text: "<code>pre_d</code>", kind: null },
      { text: "Saved height", kind: null },
      { text: "Same statement", kind: null },
    ],
  },
  {
    id: "root-handoff", title: "Root handoff", duration: 20,
    narration: "The root proof carries the complete segment's outer boundaries and one accumulator ring. Every merge check is inside that proof. The accumulator still carries the deferred pairing checks. The decider receives this root so it can establish genesis, authenticate the terminal outcome and settle the accumulated verification obligations.",
    labels: [
      { text: "Root proof", kind: null },
      { text: "Outer boundaries", kind: null },
      { text: "Accumulator", kind: null },
      { text: "Deferred pairing", kind: null },
    ],
  },
];
