// Beats of scene 5 (text copied from STORYBOARD.md).
export const BEATS = [
 {
  "id": "root-arrival",
  "title": "Root arrival",
  "duration": 21,
  "narration": "The root slab and its accumulator enter the decider, the circuit that completes verification and publishes the run statement. It verifies the root proof under pinned node keys and authenticates its boundary values and carry. With pinned keys, no prover can swap in a node circuit that accepts an invalid run.",
  "labels": [
   {
    "id": "l0",
    "text": "DECIDER",
    "anchor": "base",
    "kind": "neutral",
    "dy": 0.8
   },
   {
    "id": "l1",
    "text": "Root proof",
    "anchor": "root",
    "kind": "state",
    "dy": 1.2
   },
   {
    "id": "l2",
    "text": "Pinned node key",
    "anchor": "cardNode",
    "kind": "state",
    "dy": 0.9
   },
   {
    "id": "l3",
    "text": "Accumulator",
    "anchor": "rootRing",
    "kind": "state",
    "dy": 0.9
   }
  ]
 },
 {
  "id": "require-genesis",
  "title": "Genesis requirement",
  "duration": 22,
  "narration": "SEAL, the operation that turns a complete segment into a claim, requires <code>h_lo == 0</code>. The authenticated segment then starts at the genesis leaf, where INIT proves <code>Init(prog_d, input_d)</code>, so the continuous segment starts from the required initial state. A valid suffix cannot pass as a complete run with everything before it omitted.",
  "labels": [
   {
    "id": "l0",
    "text": "SEAL",
    "anchor": "sealBase",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "<code>h_lo = 0</code>",
    "anchor": "barNotch",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "Genesis leaf",
    "anchor": "genLeaf",
    "kind": "state",
    "dy": 0.6,
    "from": 4.8
   },
   {
    "id": "l3",
    "text": "INIT",
    "anchor": "genLeafB",
    "kind": "neutral",
    "dy": 0.6,
    "from": 4.8
   }
  ]
 },
 {
  "id": "expose-the-header",
  "title": "Opened header",
  "duration": 24,
  "narration": "The final <code>post_d</code> opens to five header fields. <code>outcome_tag</code> names the outcome kind. <code>outcome_payload</code> carries the result digest, rejection code or exhaustion kind. <code>input_d</code> binds the input value and the resource policy. <code>deps_d</code> commits to the dependency list, and <code>body_d</code> commits to the remaining machine state. The body stays behind its digest while the seal reads the header.",
  "labels": [
   {
    "id": "l0",
    "text": "<code>post_d</code>",
    "anchor": "post",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "<code>outcome_tag</code>",
    "anchor": "fl0",
    "kind": "outcome",
    "dy": 0.6,
    "from": 4.0
   },
   {
    "id": "l2",
    "text": "<code>outcome_payload</code>",
    "anchor": "fl1",
    "kind": "outcome",
    "dy": 0.6,
    "from": 4.3
   },
   {
    "id": "l3",
    "text": "<code>input_d</code>",
    "anchor": "fl2",
    "kind": "input",
    "dy": 0.6,
    "from": 4.6
   },
   {
    "id": "l4",
    "text": "<code>deps_d</code>",
    "anchor": "fl3",
    "kind": "state",
    "dy": 0.6,
    "from": 4.9
   },
   {
    "id": "l5",
    "text": "<code>body_d</code>",
    "anchor": "fl4",
    "kind": "state",
    "dy": 0.6,
    "from": 5.2
   }
  ]
 },
 {
  "id": "authenticate-the-header",
  "title": "Authenticated header",
  "duration": 22,
  "narration": "BOUNDARY COMMITMENT rehashes the supplied header fields with the fixed state domain tag, the marker that identifies a state hash. The result must equal the root proof's authenticated <code>post_d</code>. This equality binds every opened field to the proved final state. A prover cannot replace the input, outcome or dependency list at the seal.",
  "labels": [
   {
    "id": "l0",
    "text": "BOUNDARY COMMITMENT",
    "anchor": "hashAsm",
    "kind": "state",
    "dy": 0.9
   },
   {
    "id": "l1",
    "text": "State domain",
    "anchor": "domMark",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "Header fields",
    "anchor": "fieldsTray",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "Recomputed digest",
    "anchor": "hOut1",
    "kind": "state",
    "dy": 0.9,
    "from": 4.6
   },
   {
    "id": "l4",
    "text": "<code>post_d</code>",
    "anchor": "postB",
    "kind": "state",
    "dy": 0.6,
    "from": 5.6
   }
  ]
 },
 {
  "id": "refuse-running",
  "title": "Running state refused",
  "duration": 19,
  "narration": "SEAL requires the authenticated <code>outcome_tag</code> to name a terminal outcome, a state with no successor machine step. Running fails this check. Unfinished execution, halted at an intermediate state, cannot be labeled a completed claim. A checkpoint remains a segment until the run reaches a terminal outcome.",
  "labels": [
   {
    "id": "l0",
    "text": "Running",
    "anchor": "runCell",
    "kind": "error",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "Terminal required",
    "anchor": "gate",
    "kind": "neutral",
    "dy": 0.9
   },
   {
    "id": "l2",
    "text": "Checkpoint",
    "anchor": "ckpt",
    "kind": "neutral",
    "dy": 0.6
   }
  ]
 },
 {
  "id": "seal-every-outcome",
  "title": "Seal for every outcome",
  "duration": 22,
  "narration": "Halted, Rejected and Exhausted each seal as a valid claim. Halted carries a value digest. Rejected carries a rejection code. Exhausted carries an exhaustion kind. Each kind gives its own <code>outcome_d</code>. The proof certifies the machine's outcome, including rejection and exhaustion, so successful proof verification does not imply that the program returns a value.",
  "labels": [
   {
    "id": "l0",
    "text": "Halted",
    "anchor": "row1",
    "kind": "outcome",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "Rejected",
    "anchor": "row2",
    "kind": "error",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "Exhausted",
    "anchor": "row3",
    "kind": "error",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "<code>outcome_d</code>",
    "anchor": "tok1",
    "kind": "outcome",
    "dy": 0.9,
    "from": 2.0
   },
   {
    "id": "l4",
    "text": "<code>outcome_d</code>",
    "anchor": "tok2",
    "kind": "outcome",
    "dy": 0.9,
    "from": 2.0
   },
   {
    "id": "l5",
    "text": "<code>outcome_d</code>",
    "anchor": "tok3",
    "kind": "outcome",
    "dy": 0.9,
    "from": 2.0
   }
  ]
 },
 {
  "id": "compute-the-outcome",
  "title": "Outcome digest",
  "duration": 24,
  "narration": "VALUE DIGEST applies <code>sherlock-value/1</code>, the canonical encoding of values and outcomes. The outcome hash binds the outcome domain, version, kind and authenticated payload in their fixed order. For Halted, the payload is the value digest already established by execution. For Rejected and Exhausted, it is the checked code or kind. The kind inside <code>outcome_d</code> stops one outcome passing as another.",
  "labels": [
   {
    "id": "l0",
    "text": "VALUE DIGEST",
    "anchor": "hashAsm",
    "kind": "state",
    "dy": 0.9
   },
   {
    "id": "l1",
    "text": "<code>sherlock-value/1</code>",
    "anchor": "encPlate",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "Outcome kind",
    "anchor": "fl0",
    "kind": "outcome",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "Authenticated payload",
    "anchor": "fl1",
    "kind": "outcome",
    "dy": 0.6
   },
   {
    "id": "l4",
    "text": "<code>outcome_d</code>",
    "anchor": "hOut2",
    "kind": "outcome",
    "dy": 0.9,
    "from": 5.6
   }
  ]
 },
 {
  "id": "equal-values-equal-faces",
  "title": "Equal outcome digests",
  "duration": 20,
  "narration": "The value digest follows value content and ordered fields. A reference contributes the digest of its target, so memory addresses and sharing do not enter <code>outcome_d</code>. Two Halted runs that compute the same value publish the same <code>outcome_d</code>, even when one shares equal subvalues and the other stores separate copies.",
  "labels": [
   {
    "id": "l0",
    "text": "Shared subvalues",
    "anchor": "shareDiag",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "Separate copies",
    "anchor": "sepDiag",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "Equal value",
    "anchor": "insetMid",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "<code>outcome_d</code>",
    "anchor": "tokSame1",
    "kind": "outcome",
    "dy": 0.9,
    "from": 2.0
   }
  ]
 },
 {
  "id": "compute-the-claim",
  "title": "Claim digest",
  "duration": 24,
  "narration": "The seal computes <code>claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)</code>. <code>SHK_RUN</code> identifies a run claim, and <code>1</code> identifies its version. The program, input and outcome come from the authenticated run. This digest identifies the claim for replay checks. Neither <code>n_steps</code> nor a physical state field enters it, so changing proof cuts does not create a different claim identity.",
  "labels": [
   {
    "id": "l0",
    "text": "<code>SHK_RUN</code>",
    "anchor": "domMark",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "Version 1",
    "anchor": "verMark",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "<code>prog_d</code>",
    "anchor": "fp2",
    "kind": "program",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "<code>input_d</code>",
    "anchor": "fp3",
    "kind": "input",
    "dy": 0.6
   },
   {
    "id": "l4",
    "text": "<code>outcome_d</code>",
    "anchor": "fp4",
    "kind": "outcome",
    "dy": 0.6
   },
   {
    "id": "l5",
    "text": "<code>claim_d</code>",
    "anchor": "hOut3",
    "kind": "state",
    "dy": 0.9,
    "from": 5.0
   }
  ]
 },
 {
  "id": "authenticate-dependencies",
  "title": "Authenticated dependencies",
  "duration": 22,
  "narration": "A run consumes earlier claims at its seal through SEAL-WITH-DEPS. This mode checks the proved dependency list against the opened <code>deps_d</code>, so a prover cannot replace or omit the claims the run consumes. Ordinary SEAL requires <code>deps_d</code> to equal the empty dependency commitment. A nonempty list therefore requires dependency verification before the run seals.",
  "labels": [
   {
    "id": "l0",
    "text": "SEAL-WITH-DEPS",
    "anchor": "depsRoute",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "Earlier claims",
    "anchor": "claimToks",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "Proved dependency list",
    "anchor": "depsSlab",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "<code>deps_d</code>",
    "anchor": "depsMark",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l4",
    "text": "Empty dependencies",
    "anchor": "emptyTray",
    "kind": "neutral",
    "dy": 0.6
   }
  ]
 },
 {
  "id": "bind-the-step-count",
  "title": "Step count",
  "duration": 24,
  "narration": "The decider takes <code>n_steps</code> from the root's ending height because the segment begins at genesis. It requires a positive count. It range-checks the count with an 8-byte decomposition: each byte is valid, and recombining the bytes must equal the authenticated count. The count stays below 2<sup>64</sup>. An arbitrary field element or a wrapped count cannot become the reported run length.",
  "labels": [
   {
    "id": "l0",
    "text": "<code>h_hi</code>",
    "anchor": "hhiBar",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "<code>n_steps</code>",
    "anchor": "collar",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l2",
    "text": "8-byte decomposition",
    "anchor": "strip",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "Equal count",
    "anchor": "recombMark",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l4",
    "text": "Below 2<sup>64</sup>",
    "anchor": "rulerEnd",
    "kind": "neutral",
    "dy": 0.6
   }
  ]
 },
 {
  "id": "close-the-accumulator",
  "title": "Closed accumulator",
  "duration": 23,
  "narration": "The decider folds the root verification obligation together with the root's carried accumulator and performs the pairing once. This settles every deferred check in the proof tree. The pairing tests the accumulated equation, including all authenticated child carries. Skipping the pairing or dropping the root carry leaves proof verification incomplete, and no valid decider proof results.",
  "labels": [
   {
    "id": "l0",
    "text": "Root verification",
    "anchor": "cardLinkMid",
    "kind": "neutral",
    "dy": 0.6
   },
   {
    "id": "l1",
    "text": "Accumulator",
    "anchor": "rootRing",
    "kind": "state",
    "dy": 0.9
   },
   {
    "id": "l2",
    "text": "One pairing",
    "anchor": "ringJoint",
    "kind": "state",
    "dy": 0.6
   },
   {
    "id": "l3",
    "text": "Checks settled",
    "anchor": "ringTop",
    "kind": "state",
    "dy": 0.9,
    "from": 3.8
   }
  ]
 },
 {
  "id": "publish-only-the-statement",
  "title": "Public statement",
  "duration": 24,
  "narration": "The decider publishes exactly <code>[prog_d, input_d, outcome_d, n_steps]</code>. No state digest, memory root, tail digest, memory length or address becomes public. Those fields describe physical execution boundaries and storage. Leaf cuts choose which boundaries appear inside the proof. With those fields outside the public statement, the claim is independent of those cuts and of the value's storage layout.",
  "labels": [
   {
    "id": "l0",
    "text": "<code>prog_d</code>",
    "anchor": "pubProg",
    "kind": "program",
    "dy": 0.9,
    "from": 6.7
   },
   {
    "id": "l1",
    "text": "<code>input_d</code>",
    "anchor": "pubIn",
    "kind": "input",
    "dy": 1.8,
    "from": 6.7
   },
   {
    "id": "l2",
    "text": "<code>outcome_d</code>",
    "anchor": "pubOut",
    "kind": "outcome",
    "dy": 0.9,
    "from": 6.7
   },
   {
    "id": "l3",
    "text": "<code>n_steps</code>",
    "anchor": "pubSteps",
    "kind": "state",
    "dy": 1.8,
    "from": 6.7
   },
   {
    "id": "l4",
    "text": "Private state",
    "anchor": "wallMark",
    "kind": "neutral",
    "dy": 0.6,
    "from": 4.6
   }
  ]
 },
 {
  "id": "seal-handoff",
  "title": "Seal handoff",
  "duration": 20,
  "narration": "The decider fits in 2<sup>20</sup> rows and produces the seal carrying the four public values. The seal certifies a run from <code>Init(prog_d, input_d)</code> to the authenticated terminal outcome in <code>n_steps</code> steps. The transaction receives the seal with <code>prog_d</code>, <code>input_d</code>, <code>outcome_d</code> and <code>n_steps</code>, ready to bind them to its call.",
  "labels": [
   {
    "id": "l0",
    "text": "Decider seal",
    "anchor": "seal",
    "kind": "state",
    "dy": 1.2
   },
   {
    "id": "l1",
    "text": "2<sup>20</sup> rows",
    "anchor": "budget",
    "kind": "neutral",
    "dy": 0.6,
    "from": 0.6
   }
  ]
 }
];
