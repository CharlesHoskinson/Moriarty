# Narration script: steps 4 and 5

## Step 4: The node

Arrives: Leaf slabs, each with seven pins `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`, on the handoff plate.
Leaves: The root slab with its accumulator ring, on the handoff plate.
Scene length: 376 s (target 130 s)

### Beat 4.1: leaf-arrival (22 s)

On screen: Establish the node station on the paper workbench, with the arriving leaf slabs on the shared handoff plate. A leaf slab slides into a child socket beside a stationary node slab of the same footprint. The final pose holds both proof types beneath the fixed key rack, with the program and input carrier visible at the front edge.

Narration: Leaf proofs arrive with their authenticated public values. The node treats each proof as a segment, a consecutive interval of the run. Its self-recursive circuit verifies leaf proofs and proofs of its own circuit under pinned keys: keys that the verifier fixes. The circuit size setting is `k = 20` throughout the tree.

Labels:

- Leaf proof | on: the arriving leaf slab
- Node proof | on: the stationary node slab
- Pinned keys | on: the wall rack
- `k = 20` | on: the node housing

Numbers used: Seven leaf pins: BRIEF.md §3, step 3. One self-recursive circuit and `k = 20`: BRIEF.md §3, step 4.

### Beat 4.2: authenticate-children (24 s)

On screen: Inspect the child sockets, with each slab connected to its matching key card and front-edge values. A single verification sweep crosses the child assembly and gives the authenticated slab edges a short bright-gold finish. An unused socket remains still with its neutral encoding visible in the final pose.

Narration: Child verification binds each proof to the exact values it publishes. A key selector chooses the permitted proof type and its matching key together. A prover cannot attach altered boundaries to a valid proof or swap in a circuit that has weaker checks. Unused child slots and the initial carry use neutral encodings, which add no verification obligation.

Labels:

- Child proof | on: the active slab
- Public values | on: its front-edge pins
- Key selector | on: the connection to its key card
- Neutral encoding | on: the unused socket

Numbers used: none.

### Beat 4.3: bound-the-heights (22 s)

On screen: Inspect the leaf’s height and step-count cubes beside the node’s interval rail. A verification sweep crosses the cubes and their connections to the interval endpoints. The final pose holds a counter ruler marked with the range boundary, without showing a numerical height for this run.

Narration: A leaf enters with `h_lo = h_pre` and `h_hi = h_pre + steps`. Heights count completed machine steps. The node checks that `steps` is positive and fits the leaf, and that both heights stay below 2<sup>64</sup>. The 64-bit counter cannot wrap, so no invalid interval passes as valid. Its range is not a circuit size.

Labels:

- `h_pre` | on: the leaf’s starting-height cube
- `steps` | on: the leaf’s step-count cube
- `h_lo` | on: the segment’s starting-height cube
- `h_hi` | on: the segment’s ending-height cube
- 64-bit counter | on: the counter housing
- Below 2<sup>64</sup> | on: the height ruler

Numbers used: The 64-bit counter and heights below 2<sup>64</sup>: BRIEF.md §3, steps 2 and 4. The height expressions describe interval endpoints; they assign no numerical length to the run.

### Beat 4.4: match-the-program (18 s)

On screen: The left and right child slabs hold their violet program tokens above the front pins. The right token slides onto the left token’s position, and their equal engraved faces coincide in one outline. Both slabs remain visible beneath the shared program token.

Narration: MERGE combines adjacent segments. It first requires the left child's `prog_d` to equal the right child's `prog_d`, so both proofs describe the same program. Execution spliced from different programs cannot form a segment that claims one program identity.

Labels:

- MERGE | on: the node housing
- Left segment | on: the left slab
- Right segment | on: the right slab
- `prog_d` | on: the coincident program tokens

Numbers used: The left and right children and their shared program identity: BRIEF.md §3, step 4.

### Beat 4.5: match-the-state (21 s)

On screen: Inspect the gold state tokens at the adjoining ends of the child slabs. The right child’s starting-state token slides onto the left child’s ending-state token, and the matching faces coincide. The final pose holds a continuous connection through that shared boundary.

Narration: MERGE requires `L.post_d == R.pre_d`: the state where the left segment ends equals the state where the right segment begins. The boundary digest binds the entire header and body. Registers, stacks or memory replaced between valid segments fail this check, so disconnected executions cannot pass as one run.

Labels:

- `L.post_d` | on: the left slab’s ending-state attachment
- `R.pre_d` | on: the right slab’s starting-state attachment
- Equal boundary | on: the coincident tokens

Numbers used: none.

### Beat 4.6: match-the-height (20 s)

On screen: Inspect the adjoining height cubes beneath the matched state tokens. The right starting-height cube slides onto the left ending-height cube, using the same coincidence motion as digest equality. The final pose shows the interval rails meeting at that position without a gap or overlap.

Narration: MERGE also requires `L.h_hi == R.h_lo`: the left segment's ending height equals the right segment's starting height. State equality alone does not fix this position in the run. With height equality, no skipped or overlapping interval can leave a gap or repeated steps in the claimed coverage.

Labels:

- `L.h_hi` | on: the left interval’s ending-height attachment
- `R.h_lo` | on: the right interval’s starting-height attachment
- Equal height | on: the coincident cubes

Numbers used: none.

### Beat 4.7: retain-the-outer-boundaries (22 s)

On screen: The checked child slabs remain still inside the housing, with their shared inner boundary recessed behind them. The output slab slides forward with only the outer segment values on its front edge. Its footprint matches the child slabs, and the final pose exposes all five output values.

Narration: The merged segment publishes `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`. Its first state and height come from the left child. Its last state and height come from the right child. The matching inner digests leave the output, because the node proof has already established their equality. The next merge needs only the outer boundaries.

Labels:

- `prog_d` | on: the output program pin
- `L.pre_d` | on: the output starting-state pin
- `R.post_d` | on: the output ending-state pin
- `L.h_lo` | on: the output starting-height pin
- `R.h_hi` | on: the output ending-height pin

Numbers used: Five segment values: BRIEF.md §3, step 4, output tuple.

### Beat 4.8: fold-the-obligations (24 s)

On screen: Inspect the output slab’s gold accumulator ring, with fixed connections from the verified child sockets. Folding adds a mark to the existing ring without changing its diameter or creating another output ring. The final pose holds the marked ring on the slab, with every enabled child connection intact.

Narration: Verifying a child proof leaves a pairing check, a cryptographic equation that completes proof verification. The node defers that pairing and folds the new check and every carried child check into one accumulator, a fixed-size encoding of deferred checks. Dropping a child's accumulator discards its verification obligation, so the circuit binds every enabled carry into the fold.

Labels:

- Child checks | on: the incoming connections
- Fold | on: their meeting point
- Accumulator | on: the gold ring
- Deferred pairing | on: the ring’s unclosed joint

Numbers used: One carried accumulator: BRIEF.md §3, step 4.

### Beat 4.9: fixed-node-linear-work (22 s)

On screen: A schematic tree occupies the station’s inspection tray, with break marks indicating additional depth. A node slab slides from a lower position to the root position while keeping its footprint and ring diameter unchanged. Beside it, still rulers relate execution length to the number of leaf and node proofs without numerical tick values.

Narration: The accumulator keeps the same size as more checks enter it. A node at any tree height has the same circuit size and the same per-node proving cost. Longer runs require more leaf proofs and more node proofs. Their numbers grow in proportion to the number of machine steps, so total proving work grows linearly.

Labels:

- Fixed node size | on: the slab’s footprint outline
- Fixed accumulator size | on: the ring outline
- Machine steps | on: the execution ruler
- Leaf proofs | on: the leaf-work ruler
- Node proofs | on: the node-work ruler
- Linear work | on: the aligned rulers

Numbers used: none.

### Beat 4.10: node-proving-cost (18 s)

On screen: Inspect a node slab inside a prover housing on the same workbench. A verification sweep crosses the slab, while stationary annotations give the reference-host time, thread count and memory use. The final pose holds those annotations beside the completed slab without animating a numerical step counter.

Narration: On the reference host, one node proof takes about 65 s at 20 threads and uses 17 GB of memory. That cost is per node proof: a node near the root verifies accumulated work through the same circuit as a node near the leaves.

Labels:

- Reference host | on: the prover housing
- About 65 s | on: the proof-time annotation
- 20 threads | on: the thread annotation
- 17 GB | on: the memory annotation

Numbers used: One node proof, about 65 s, 20 threads and 17 GB: BRIEF.md §3, step 4.

### Beat 4.11: parallel-merge-trees (20 s)

On screen: Three prover housings occupy adjacent positions at the node station, each connected to ready child proofs in the schematic tree. Their verification sweeps run together, making parallel work the beat’s single subject. The final pose holds the completed slabs at their merge positions, with all execution intervals still ordered left to right.

Narration: Machines prove independent leaves and ready merges at the same time. Every merge preserves execution order from left to right, while the tree can group adjacent segments in any shape. Work proceeds whenever the required child proofs are ready. On the reference host, three provers in parallel give 1.73x throughput.

Labels:

- Parallel provers | on: the prover housings
- Ready children | on: the incoming slabs
- Left to right | on: the execution-order rail
- 1.73x throughput | on: the reference-host annotation

Numbers used: Three parallel provers and 1.73x throughput: BRIEF.md §3, step 4.

### Beat 4.12: include-new-work (28 s)

On screen: Prior segment slabs sit inside the node housing beside a short witness-tape slice. The tape slice slides into the work socket at the right end of the proved interval. The final pose shows its starting-state and height connections aligned with the preceding segment, and its ending-height connection reaching the output pin.

Narration: MERGE-WORK combines prior segment proofs with new machine steps inside the same node. The new work uses the leaf's execution checks. Its program must equal `prog_d`, its starting state must equal the right child's `post_d`, and its starting height must equal that child's ending height. The output height includes exactly the new steps and stays below 2<sup>64</sup>. Detached work and uncounted execution cannot enter the segment.

Labels:

- MERGE-WORK | on: the node housing
- Prior segments | on: the child slabs
- New steps | on: the witness-tape slice
- `prog_d` | on: the shared program connection
- `post_d` | on: the preceding segment’s ending-state token
- Below 2<sup>64</sup> | on: the output-height ruler

Numbers used: Heights below 2<sup>64</sup>: BRIEF.md §3, step 4. The new work has no numerical step count.

### Beat 4.13: join-complete-runs (25 s)

On screen: A side tray holds node slabs for sets of complete run claims, with their gold set-root tokens attached. The ordered tag and child-root inputs pass through the fixed gold hash block, which emits the combined set-root token. The final pose holds the joined set beside its child sets, while the main run segment remains stationary on the central rail.

Narration: JOIN combines sets of claims, statements of complete runs. It computes `set_root = H(SHK_SET_JOIN, l, r)`, where `l` and `r` are the child set roots and `H` is Poseidon. The tag `SHK_SET_JOIN` identifies this hash as a set join. The ordered hash binds both child roots. JOIN needs no execution boundary between independent runs, as MERGE does, and carries no claim counter.

Labels:

- JOIN | on: the side-tray operation housing
- `SHK_SET_JOIN` | on: the hash’s domain-tag cell
- `l` | on: the left child set-root token
- `r` | on: the right child set-root token
- `set_root` | on: the emitted token

Numbers used: none.

### Beat 4.14: share-claims-in-a-dag (23 s)

On screen: The side tray shows a claim token connected to separate consumer proofs whose results feed a later join. A verification sweep follows the connected proof assembly, with no connection returning to an ancestor. The final pose holds the shared claim and the address-free shape token beside the graph.

Narration: A DAG, a directed acyclic graph, lets a proved claim feed multiple consumers without creating a cycle. Reusing a claim never licenses a duplicated run interval inside MERGE. The address-free shape digest `shape_d` binds the proof graph's connections and operation kinds. Different graph shapes preserve the claims they contain, and the same keys serve every depth.

Labels:

- Shared claim | on: the claim token
- Consumers | on: the receiving proof slabs
- DAG | on: the graph’s inspection tray
- `shape_d` | on: the shape token
- Same keys | on: the fixed key rack

Numbers used: none.

### Beat 4.15: retain-a-checkpoint (24 s)

On screen: Return to the main run, whose proved prefix rests beside its retained witness tray. The tray contains register cubes, memory tiles, stack-chain preimages and the program and input resources, with a fixed connection to the prefix’s ending-state token. The complete checkpoint tray slides into a storage recess and holds there with the prefix slab and accumulator visible.

Narration: CONTINUATION saves a checkpoint, the proved prefix and the witness data needed to continue it. The checkpoint retains the prefix proofs and accumulators, its ending height and `post_d`, registers, memory records, stack contents, program and input. Restoring it checks the key set and recomputes the committed state. Altered checkpoint data cannot give the suffix a different starting state.

Labels:

- CONTINUATION | on: the checkpoint housing
- Checkpoint | on: the stored tray
- Proved prefix | on: the prefix slab
- Retained witness | on: the resource tray
- `post_d` | on: the prefix’s ending-state token
- Key set | on: the checkpoint’s key connection

Numbers used: none.

### Beat 4.16: resume-the-run (23 s)

On screen: The restored prefix slab holds still beside the resumed suffix slab. The suffix slides into alignment so its starting-state token coincides with the prefix’s ending-state token, while their height attachments meet on the same interval rail. The final pose holds the joined execution with unchanged fuel and logical-counter cubes in the retained witness tray.

Narration: The resumed suffix starts at the saved height, and its `pre_d` equals the prefix's `post_d`. MERGE checks these equalities exactly as it checks an uninterrupted run. Pausing and resuming execute no machine step and change no fuel or logical counter. Proving the run in parts gives the same final statement as proving it in one go.

Labels:

- Proved prefix | on: the prefix slab
- Resumed suffix | on: the suffix slab
- `post_d` | on: the prefix’s ending-state attachment
- `pre_d` | on: the suffix’s starting-state attachment
- Saved height | on: the common height position
- Same statement | on: the joined output slab

Numbers used: none.

### Beat 4.17: root-handoff (20 s)

On screen: Establish the node station with its completed root slab and attached accumulator ring. The slab slides along the bench onto the handoff plate at the right edge of the station. The final pose holds the root’s outer values and unclosed ring in the exact arrangement that begins the decider scene.

Narration: The root proof carries the complete segment's outer boundaries and one accumulator ring. Every merge check is inside that proof. The accumulator still carries the deferred pairing checks. The decider receives this root so it can establish genesis, authenticate the terminal outcome and settle the accumulated verification obligations.

Labels:

- Root proof | on: the root slab
- Outer boundaries | on: the root’s front-edge pins
- Accumulator | on: the attached gold ring
- Deferred pairing | on: the ring’s unclosed joint

Numbers used: One carried accumulator: BRIEF.md §3, step 4.

## Step 5: The decider

Arrives: The root slab with its accumulator ring, on the handoff plate.
Leaves: The decider seal and the four tokens `[prog_d, input_d, outcome_d, n_steps]`, on the handoff plate.
Scene length: 317 s (target 110 s)

### Beat 5.1: root-arrival (21 s)

On screen: Establish the decider station with the root slab and accumulator in the preceding scene’s final pose. A verification sweep crosses the root slab beneath its pinned node key card. The final pose holds the authenticated root values and ring beside the decider housing, with the program and input carrier visible.

Narration: The root slab and its accumulator enter the decider, the circuit that completes verification and publishes the run statement. It verifies the root proof under pinned node keys and authenticates its boundary values and carry. With pinned keys, no prover can swap in a node circuit that accepts an invalid run.

Labels:

- DECIDER | on: the decider housing
- Root proof | on: the arriving slab
- Pinned node key | on: the key card
- Accumulator | on: the root’s ring

Numbers used: none.

### Beat 5.2: require-genesis (25 s)

On screen: Inspect the root’s starting-height cube beside the genesis position on the interval ruler. The cube slides onto the position marked zero, making the equality a physical coincidence. The final pose holds the joined position with a fixed connection back through the proof tree to the genesis leaf.

Narration: SEAL, the operation that turns a complete segment into a claim, requires `h_lo == 0`. The authenticated segment then starts at the genesis leaf, where INIT proves `Init(prog_d, input_d)`, so the continuous segment starts from the required initial state. A valid suffix cannot pass as a complete run with everything before it omitted.

Labels:

- SEAL | on: the sealing housing
- `h_lo = 0` | on: the coincident starting-height position
- Genesis leaf | on: the start of the proof connection
- INIT | on: the genesis check

Numbers used: Genesis height zero: BRIEF.md §2, input admission, and §3, steps 3 and 5.

### Beat 5.3: expose-the-header (22 s)

On screen: Inspect the final `post_d` token above a recessed header tray. The tray slides outward as one assembly, exposing the outcome cells and the input, dependency and body digest tokens in their fixed order. The final pose keeps the body behind its digest and all header fields inside the decider housing.

Narration: The final `post_d` opens to five header fields. `outcome_tag` names the outcome kind. `outcome_payload` carries the result digest, rejection code or exhaustion kind. `input_d` identifies the run input. `deps_d` commits to the dependency list, and `body_d` commits to the remaining machine state. The body stays behind its digest while the seal reads the header.

Labels:

- `post_d` | on: the authenticated final-state token
- `outcome_tag` | on: the outcome-kind cell
- `outcome_payload` | on: the payload cell
- `input_d` | on: the header’s input token
- `deps_d` | on: the dependency token
- `body_d` | on: the body token

Numbers used: Five header fields: BRIEF.md §2, boundary commitment.

### Beat 5.4: authenticate-the-header (22 s)

On screen: The fixed state-domain cell and header fields line up in their stated order at the gold hash block. They pass through the block, whose output slides directly onto the authenticated `post_d` token and coincides with its engraved face. The final pose holds the shared outline with every header input still connected to it.

Narration: BOUNDARY COMMITMENT rehashes the supplied header fields with the fixed state domain tag, the marker that identifies a state hash. The result must equal the root proof's authenticated `post_d`. This equality binds every opened field to the proved final state. A prover cannot replace the input, outcome or dependency list at the seal.

Labels:

- BOUNDARY COMMITMENT | on: the header-check assembly
- State domain | on: the fixed domain-tag cell
- Header fields | on: the ordered input tray
- Recomputed digest | on: the hash output attachment
- `post_d` | on: the coincident state tokens

Numbers used: none.

### Beat 5.5: refuse-running (21 s)

On screen: A Running outcome cell occupies a comparison socket beside the main sealing path. It approaches the terminal gate, stops, moves back slightly and receives a red hatch. The final pose holds that refused cell outside the seal, beside a stationary checkpoint slab.

Narration: SEAL requires the authenticated `outcome_tag` to name a terminal outcome, a state with no successor machine step. Running fails this check. Unfinished execution, halted at an intermediate state, cannot be labeled a completed claim. A checkpoint remains a segment until the run reaches a terminal outcome.

Labels:

- Running | on: the refused outcome cell
- Terminal required | on: the seal gate
- Checkpoint | on: the stationary segment slab

Numbers used: none.

### Beat 5.6: seal-every-outcome (22 s)

On screen: A side tray holds terminal-outcome examples for Halted, Rejected and Exhausted, each beside a seal and its outcome token. A single verification sweep crosses the terminal-kind cells, giving each a bright-gold edge. The final pose holds the three valid outcome classes with distinct engraved outcome faces, while the main run remains unchanged.

Narration: Halted, Rejected and Exhausted each seal as a valid claim. Halted carries a value digest. Rejected carries a rejection code. Exhausted carries an exhaustion kind. Each kind gives its own `outcome_d`. The proof certifies the machine's outcome, including rejection and exhaustion, so successful proof verification does not imply that the program returns a value.

Labels:

- Halted | on: the value-result example
- Rejected | on: the rejection-code example
- Exhausted | on: the exhaustion-kind example
- `outcome_d` | on: the Halted example’s token
- `outcome_d` | on: the Rejected example’s token
- `outcome_d` | on: the Exhausted example’s token

Numbers used: Three terminal outcomes: BRIEF.md §3, step 2; terminal sealing and outcome computation: §3, step 5.

### Beat 5.7: compute-the-outcome (24 s)

On screen: Return to the main run’s authenticated outcome kind and payload. The outcome domain, version, kind and payload line up and pass through the fixed gold hash block, which emits a teal-rimmed `outcome_d` token. The final pose holds that token beside the existing program and input tokens, with no address attached.

Narration: VALUE DIGEST applies `sherlock-value/1`, the canonical encoding of values and outcomes. The outcome hash binds the outcome domain, version, kind and authenticated payload in their fixed order. For Halted, the payload is the value digest already established by execution. For Rejected and Exhausted, it is the checked code or kind. The kind inside `outcome_d` stops one outcome passing as another.

Labels:

- VALUE DIGEST | on: the outcome-hash assembly
- `sherlock-value/1` | on: the encoding plate
- Outcome kind | on: the authenticated kind cell
- Authenticated payload | on: the payload attachment
- `outcome_d` | on: the emitted teal token

Numbers used: The versioned name `sherlock-value/1`: BRIEF.md §3, step 5. No numerical outcome discriminator is displayed.

### Beat 5.8: equal-values-equal-faces (20 s)

On screen: A comparison tray holds two decider seals above memory representations of the same value, one with shared subvalues and one with separate copies. The outcome token from one seal slides onto the other token’s position, and their engraved faces coincide. The final pose retains both seals and the differing storage diagrams beneath the shared outcome face.

Narration: The value digest follows value content and ordered fields. A reference contributes the digest of its target, so memory addresses and sharing do not enter `outcome_d`. Two Halted runs that compute the same value publish the same `outcome_d`, even when one shares equal subvalues and the other stores separate copies.

Labels:

- Shared subvalues | on: the shared storage diagram
- Separate copies | on: the duplicated storage diagram
- Equal value | on: the comparison tray
- `outcome_d` | on: the coincident outcome tokens

Numbers used: Two runs with equal outcomes: BRIEF.md §3, step 5.

### Beat 5.9: compute-the-claim (24 s)

On screen: The run-domain cell, version cell and program, input and outcome tokens line up in the stated hash order. They pass through the gold hash block, which emits a gold-rimmed `claim_d` token into a private side socket. The final pose keeps the step-count cube outside this hash input and the claim token separate from the public handoff carrier.

Narration: The seal computes `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`. `SHK_RUN` identifies a run claim, and `1` identifies its version. The program, input and outcome come from the authenticated run. This digest identifies the claim for replay checks. Neither `n_steps` nor a physical state field enters it, so changing proof cuts does not create a different claim identity.

Labels:

- `SHK_RUN` | on: the domain-tag cell
- Version 1 | on: the version cell
- `prog_d` | on: the program input token
- `input_d` | on: the input token
- `outcome_d` | on: the outcome token
- `claim_d` | on: the emitted claim token

Numbers used: Claim version `1`: BRIEF.md §3, step 5, `claim_d` formula.

### Beat 5.10: authenticate-dependencies (22 s)

On screen: A dependency-list proof sits beside the header’s `deps_d` token, with earlier claim tokens attached to the proof. Its authenticated list commitment slides onto `deps_d` and coincides with the engraved face. The final pose holds this dependency-enabled seal route beside the stationary empty-dependency route for ordinary SEAL.

Narration: A run consumes earlier claims at its seal through SEAL-WITH-DEPS. This mode checks the proved dependency list against the opened `deps_d`, so a prover cannot replace or omit the claims the run consumes. Ordinary SEAL requires `deps_d` to equal the empty dependency commitment. A nonempty list therefore requires dependency verification before the run seals.

Labels:

- SEAL-WITH-DEPS | on: the dependency-enabled seal route
- Earlier claims | on: the attached claim tokens
- Proved dependency list | on: the dependency proof
- `deps_d` | on: the coincident commitment tokens
- Empty dependencies | on: the ordinary SEAL route

Numbers used: none.

### Beat 5.11: bind-the-step-count (26 s)

On screen: Inspect the authenticated ending-height cube connected to the `n_steps` counter cube. An eight-byte strip lies beneath it, with fixed connections showing the recombination into the same count. A verification sweep crosses the strip and count, leaving the final pose with all byte values abstract and no numerical run length.

Narration: The decider takes `n_steps` from the root's ending height because the segment begins at genesis. It requires a positive count. It range-checks the count with an 8-byte decomposition: each byte is valid, and recombining the bytes must equal the authenticated count. The count stays below 2<sup>64</sup>. An arbitrary field element or a wrapped count cannot become the reported run length.

Labels:

- `h_hi` | on: the root’s ending-height cube
- `n_steps` | on: the public counter cube
- 8-byte decomposition | on: the byte strip
- Equal count | on: the recombination connection
- Below 2<sup>64</sup> | on: the counter’s range ruler

Numbers used: Eight-byte decomposition: BRIEF.md §3, step 5. Heights below 2<sup>64</sup>: §3, step 4. No value is assigned to `n_steps`.

### Beat 5.12: close-the-accumulator (24 s)

On screen: Inspect the accumulator ring at the decider, with the root-verification connection entering the same ring. The ring closes once, and its edge briefly turns bright gold as the pairing succeeds. The final pose holds a closed ring against the seal, with no unresolved branch connection outside it.

Narration: The decider folds the root verification obligation together with the root's carried accumulator and performs the pairing once. This settles every deferred check in the proof tree. The pairing tests the accumulated equation, including all authenticated child carries. Skipping the pairing or dropping the root carry leaves proof verification incomplete, and no valid decider proof results.

Labels:

- Root verification | on: the incoming root-check connection
- Accumulator | on: the gold ring
- One pairing | on: the ring’s closing joint
- Checks settled | on: the closed ring

Numbers used: One final pairing: BRIEF.md §3, step 5.

### Beat 5.13: publish-only-the-statement (24 s)

On screen: The public carrier holds the program, input and outcome tokens beside the step-count cube. A ceramic shutter closes over the state tokens, memory commitments, address cells and private claim-token socket. The final pose leaves exactly the public statement beside the seal, with every physical-state object behind the shutter.

Narration: The decider publishes exactly `[prog_d, input_d, outcome_d, n_steps]`. No state digest, memory root, tail digest, memory length or address becomes public. Those fields describe physical execution boundaries and storage. Leaf cuts choose which boundaries appear inside the proof. With those fields outside the public statement, the claim is independent of those cuts and of the value's storage layout.

Labels:

- `prog_d` | on: the public program token
- `input_d` | on: the public input token
- `outcome_d` | on: the public outcome token
- `n_steps` | on: the public counter cube
- Private state | on: the closed shutter

Numbers used: Exactly four public values: BRIEF.md §3, step 5.

### Beat 5.14: seal-handoff (20 s)

On screen: Establish the decider station with the completed seal and public carrier. The seal and carrier move together onto the handoff plate at the station’s right edge. The final pose holds the seal and `[prog_d, input_d, outcome_d, n_steps]` in the exact arrangement that begins the transaction scene.

Narration: The decider fits in 2<sup>20</sup> rows and produces the seal carrying the four public values. The seal certifies a run from `Init(prog_d, input_d)` to the authenticated terminal outcome in `n_steps` steps. The transaction receives the seal with `prog_d`, `input_d`, `outcome_d` and `n_steps`, ready to bind them to its call.

Labels:

- Decider seal | on: the completed seal
- 2<sup>20</sup> rows | on: the decider housing
- `prog_d` | on: the program token
- `input_d` | on: the input token
- `outcome_d` | on: the outcome token
- `n_steps` | on: the counter cube

Numbers used: Decider capacity of 2<sup>20</sup> rows and four public values: BRIEF.md §3, step 5.

## Notes for the director

- **Timing:** Step 4 lasts 376 s against a target of 130 s. Step 5 lasts 317 s against a target of 110 s. Both exceed their targets by more than a quarter. Separate beats preserve the authentication, continuity, accumulation, continuation and sealing arguments.
- **Duration calculation:** Count whitespace-separated narration tokens, including identifiers and expressions. Hyphenated terms count as single tokens. Divide by 2.5, round upward and apply the 12 s minimum. Beat numbers, durations, scene totals and source references are production metadata, not additional technical quantities.
- **Shared staging:** Keep the original `prog_d` and `input_d` tokens visible in the front carrier throughout. Show only the labels listed for the active beat. Every motion ends in a still hold. Use engraved digest faces without hexadecimal values.
- **Handoff representation:** The four handoff “tokens” name the four public-value objects. The digest values retain hexagonal tokens. `n_steps` retains the rounded counter cube prescribed for a field element, without a numerical value.
- **Main path and examples:** JOIN, DAG sharing, outcome comparisons and equal-value comparisons occupy side trays on the same workbench. Their objects are present when the inspection begins. They demonstrate related operations without replacing the main run or changing its outcome. The main handoff into step 5 remains a segment root slab with its accumulator.
- **Node statement scope:** `research/sherlock-final-design-20261111/ARCHITECTURE.md`, NODE INTEGRATION, describes protocol metadata alongside the segment payload. BRIEF.md §3, step 4 specifies the visible segment tuple. The script shows those five payload values and the accumulator ring; key and shape metadata remain inside the verification assembly.
- **Leaf entry:** ARCHITECTURE.md, NODE INTEGRATION, leaves direct leaf entry versus an additional leaf-lift relation unresolved. BRIEF.md specifies a self-recursive node that verifies leaf and node proofs. The script shows direct leaf and node children under pinned keys.
- **Seal placement:** ARCHITECTURE.md, NODE INTEGRATION and DECIDER, describes sealing within nodes for collections as well as opening a segment root in the run decider. The script follows BRIEF.md’s run handoff: a segment root enters the decider, and the decider scene explains SEAL. JOIN’s side tray uses complete run claims, not unfinished execution segments.
- **Additional node work:** MERGE-WORK comes from ARCHITECTURE.md’s GP-4 hook and `openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-proof-shape/spec.md`. Its beat makes the “prior proofs plus new work” property explicit. It introduces no numerical work capacity.
- **Historical recursion limits:** `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` includes historical restrictions on proof shape and earlier circuit sizing. BRIEF.md specifies the working self-recursive node at `k = 20` and trees and DAGs of any shape. The narration uses those canonical facts without the historical restrictions or their figures.
- **Partition independence:** `openspec/changes/sherlock-vm-consensus-20261107/design.md`, its physical-commitment discussion, allows physical commitments to change with leaf partitioning. BRIEF.md §3, step 3 specifies the same final memory state for every cut. The script therefore says cuts select intermediate boundaries; it does not claim that recutting changes the canonical final memory state.
- **Input identity:** ARCHITECTURE.md’s INIT kit and the consensus `design.md` describe an input commitment that also authenticates resource-policy data. BRIEF.md describes `input_d` as the canonical input-value digest. These scenes inherit the authenticated token and `Init(prog_d, input_d)` without redefining its preimage.
- **Initialization layout:** The consensus `requirements.md` and `design.md` retain initialization descriptions with separate memory tables and unresolved input-loading forms. BRIEF.md specifies one arena, by-digest admission and empty genesis memory. These scenes rely on the genesis leaf’s INIT guarantee without introducing another memory layout or admission path.
- **Contract scope:** The consensus `requirements.md` replay requirement and `design.md` contract section retain a program identity sealed into a program-specific contract. BRIEF.md specifies `prog_d` as call data and one contract for every program. The handoff carries `prog_d` as a public value and introduces no program-specific contract.
- **Measurements and row figures:** `research/sherlock-final-design-20261111/EXPERIMENTS.md` discusses projected node rows; `EXPERIMENTS-MAXIMA.md` concerns memory-operation maxima. ARCHITECTURE.md and the modular specifications also contain separate row gates and estimates. The script takes its node time, threads, memory, parallel throughput and decider capacity exclusively from BRIEF.md. It copies no other cost or capacity figure.
- **Quantities shown without numbers:** The run’s actual heights and `n_steps`; total leaf and node counts; proof-tree depth; new work per MERGE-WORK; claim-set size; DAG depth and consumer count; dependency-list length; checkpoint storage size; register, memory-record and stack-content counts; accumulator encoding size; header-body size; value size and sharing multiplicity; individual byte values; and the duration of a pause remain unnamed numerically.
- **Mechanism coverage:** No requested mechanism remains unexplained. Dependency verification receives the specified introductory beat; the later contract scene supplies its full claim-consumption argument.