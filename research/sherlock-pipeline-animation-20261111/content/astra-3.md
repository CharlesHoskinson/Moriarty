# Narration script: steps 6 to 8

## Step 6: The transaction

Arrives: The decider seal and the four tokens `[prog_d, input_d, outcome_d, n_steps]` on the handoff plate.  
Leaves: The transaction envelope on the handoff plate.  
Scene length: 90 s (target 60 s)

### Beat 6.1: pack-the-call (27 s)

On screen: Establish the crossing station, with paper beneath the incoming plate and slate beneath the outgoing plate. The decider seal and its public-value carrier slide together into a flat slate envelope addressed to `SherlockRun.acceptRun`. The envelope rests with its contents exposed and the original public values visible along its front edge.

Narration: The prover places the decider proof and its four public values in a Midnight transaction, an envelope submitted for ledger validation. `prog_d` is the program image digest, and `input_d` is the input value digest. `outcome_d` is the outcome digest under `sherlock-value/1`, the canonical value encoding. `n_steps` gives the machine step count. The envelope calls `SherlockRun.acceptRun`. Its proof is about 6.4 KB, whatever the length of the run.

Labels:

- Decider proof | on: the titanium seal
- `prog_d` | on: the violet digest token
- `input_d` | on: the blue digest token
- `outcome_d` | on: the teal digest token
- `n_steps` | on: the step-count position in the carrier
- `SherlockRun.acceptRun` | on: the envelope address

Numbers used: Four public values: BRIEF.md §3, steps 5 and 6. The version in `sherlock-value/1`: §3, steps 1 and 5. About 6.4 KB: §3, step 6.

### Beat 6.2: refuse-a-swapped-value (23 s)

On screen: Inspect a comparison envelope beside the unchanged original. The comparison envelope holds the same seal but an `outcome_d` token with a different engraved face. It stops at the verification boundary, returns slightly and receives a red hatch. The original envelope holds still beside the refused substitution.

Narration: The call transcript, the ordered data bound to the proof, includes these exact public values. When a sender replaces `outcome_d` and keeps the proof, verification against the altered transcript fails and the transaction is refused. A changed `prog_d`, `input_d` or `n_steps` meets the same refusal. A valid proof therefore authorizes only its own claim and step count.

Labels:

- Call transcript | on: the original envelope’s public-value strip
- Same proof | on: the comparison seal
- Changed outcome | on: the substituted token
- Refused | on: the red-hatched comparison envelope

Numbers used: none.

### Beat 6.3: keep-the-contract (20 s)

On screen: Inspect the original envelope beneath the stationary gold key rack. Its `prog_d` token slides into the call-data position aligned with the contract entrance. The token remains visibly part of the envelope, while the contract housing and key cards hold their positions.

Narration: The contract receives `prog_d` as call data, a value carried by the invocation. One contract and one key set serve every program and every input, so the program identity selects no contract and no verifier key. The proof binds the supplied program digest to the execution it proves.

Labels:

- `prog_d` | on: the program token in the call-data position
- Call data | on: the envelope’s input strip
- Shared key set | on: the gold key rack
- `SherlockRun` | on: the contract entrance

Numbers used: One contract and one key set: BRIEF.md §2, “One contract.”

### Beat 6.4: cross-to-slate (20 s)

On screen: Establish the crossing with the fee entry attached to the original envelope. The envelope slides across the paper-to-slate boundary while the camera follows it. It stops on the outgoing handoff plate with the seal, public values and call address visible. The fee entry reads DUST without an amount.

Narration: The prover pays the transaction fee in DUST, Midnight's fee resource. The envelope carries the decider proof, the public values and the call to `SherlockRun.acceptRun` across to the chain. Its contents give the contract everything it needs to check the run claim. The witness tape stays with the prover.

Labels:

- DUST | on: the transaction fee entry
- Decider proof | on: the seal inside the envelope
- `SherlockRun.acceptRun` | on: the envelope address
- Transaction | on: the outgoing envelope

Numbers used: none.

## Step 7: The contract

Arrives: The transaction envelope on the handoff plate.  
Leaves: The stored claim: drawer `claim_d` holding `outcome_d` and `n_steps`, on the handoff plate.  
Scene length: 534 s (target 100 s)

### Beat 7.1: check-the-installed-key (25 s)

On screen: Establish the slate contract station with the incoming envelope in its previous pose. A gold verifier key card remains fixed above the exposed seal and call-data strip. A verification sweep crosses the seal and its bound values, leaving a short bright-gold edge. The envelope rests beside the `runs` cabinet.

Narration: The transaction reaches `acceptRun`, the contract operation that accepts a proved run. It checks the decider proof against the contract's verifier key, the public key that defines the accepted proof relation, and the call transcript. A failed check refuses the call, so a sender cannot pass off a proof of an unrelated relation. The same verifier key serves every program and input.

Labels:

- `acceptRun` | on: the contract entrance
- Verifier key | on: the fixed gold key card
- Decider proof | on: the seal
- Call transcript | on: the public-value strip
- `runs` | on: the cabinet

Numbers used: none.

### Beat 7.2: derive-the-claim (27 s)

On screen: Inspect the contract’s gold Poseidon block. The domain marker, version cell and call-bound digest inputs line up in formula order and slide into the block. A gold-rimmed `claim_d` token emerges with its engraved face visible. The `n_steps` position stays outside the hash-input track.

Narration: The contract recomputes `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`. Poseidon is the digest function. `SHK_RUN` marks the run-claim domain, separating this digest from other kinds of digest, and `1` is the claim version. The remaining inputs come from the verified call.

With the identity derived from these values, no sender can give an existing result a fresh identity. The step count stays outside this identity.

Labels:

- `SHK_RUN` | on: the domain marker
- Claim version | on: the version cell
- Poseidon | on: the gold hash block
- `claim_d` | on: the emerging gold-rimmed token
- `n_steps` | on: the stationary step-count position

Numbers used: Claim version 1: BRIEF.md §3, step 5.

### Beat 7.3: look-up-the-claim (21 s)

On screen: Inspect the `runs` cabinet index. The recomputed `claim_d` token slides to the lookup position beside the stored engraved claim faces. Its destination is an unused drawer position, with no matching stored face. Hold the token beside that empty position.

Narration: The contract looks up the recomputed `claim_d` in `runs`, its record of accepted run claims. It compares this digest with the stored claim keys. An absent key passes the freshness check. An existing key refuses the call. The lookup stops a valid proof from creating a second acceptance of the same claim.

Labels:

- `runs` | on: the cabinet
- `claim_d` | on: the lookup token
- Claim absent | on: the unused drawer position

Numbers used: none.

### Beat 7.4: store-the-outcome (22 s)

On screen: The fresh drawer carries `claim_d` on its front. The `outcome_d` token and step-count object slide together into the drawer’s storage recess. The program and input tokens remain visible in the carrier outside the recess. Hold on the stored payload and its claim key.

Narration: For a fresh `claim_d`, the contract stores `outcome_d` and `n_steps` under that key. These are the values that the verified proof binds, so the record preserves both the result identity and the authenticated step count. The contract stores no state digest. Memory roots, tail digests, memory lengths and addresses remain inside the proof.

Labels:

- `claim_d` | on: the drawer front
- `outcome_d` | on: the stored teal token
- `n_steps` | on: the stored step-count object
- `runs` | on: the cabinet

Numbers used: none.

### Beat 7.5: refuse-the-same-proof (22 s)

On screen: A repeated envelope sits at the cabinet entrance with the same seal and public-value faces. Its claim face matches the occupied drawer’s face. The envelope stops, returns slightly and receives the fixed red refusal hatch. The occupied drawer and its contents remain still.

Narration: A sender submits the same proof again with the same public values. Proof verification succeeds again, the contract derives the same `claim_d`, and its lookup finds the existing record and refuses the repeated call. The stored outcome and step count remain unchanged. A valid proof lets its claim be accepted once, never repeatedly.

Labels:

- Same proof | on: the repeated seal
- `claim_d` | on: the occupied drawer
- Claim present | on: the cabinet lookup position
- Replay refused | on: the repeated envelope

Numbers used: none.

### Beat 7.6: refuse-another-partition (22 s)

On screen: A comparison envelope arrives with a differently engraved proof seal. Behind it, stationary tape strips show different leaf boundaries without numeric cut positions. Its public-value faces and claim face match those of the stored claim. The comparison envelope stops at the occupied drawer, returns slightly and receives a red hatch.

Narration: The sender proves the same run again with different boundaries between leaves. The proof changes, while `prog_d`, `input_d`, `outcome_d` and `n_steps` stay equal. The contract recomputes the same `claim_d` and refuses the call. Leaf boundaries, proof randomness and aggregation shape never enter this identity, so changing the proof's construction cannot bypass replay refusal.

Labels:

- Different leaf boundaries | on: the comparison tape strips
- Different proof | on: the comparison seal
- Same claim | on: the matching engraved claim faces
- Replay refused | on: the comparison envelope

Numbers used: none.

### Beat 7.7: authenticate-without-counting (24 s)

On screen: Inspect an authenticated collection slab beside the accepted claim. Its child-proof connections and claim-opening path are visible as fixed structural tracks. A verification sweep crosses the slab and leaves its gold edge bright. Hold on the authenticated structure without a claim-count register or count label.

Narration: The public statement carries no count of claims. A count says nothing about whether a particular claim is valid or present. Proof checks establish validity, digest openings establish membership, and `claim_d` controls replay refusal. Collections combine authenticated children without adding a claim counter, so shared dependencies require no count that grows with every path through the proof graph.

Labels:

- Verified collection | on: the collection slab
- Membership opening | on: the claim-opening path
- `claim_d` | on: the accepted claim token

Numbers used: none.

### Beat 7.8: open-a-sealed-dependency (29 s)

On screen: Inspect the producer’s collection proof on the paper-side proof rail, with the accepted drawer still visible across the boundary. An exploded path connects its root to the requested claim. At the selected path link, opening inputs enter the fixed Poseidon block and reproduce the parent token’s engraved face. Hold the completed path with the sealed claim at its end.

Narration: A later run consumes this claim at its own seal. Consume means authenticate a claim for use as a dependency. The producer supplies a verified SET, a digest commitment to sealed claims. Hash openings connect the requested claim to that verified root, through joined collections or dependency coverage. Every opening must reproduce its parent digest. An unrelated claim, an unfinished segment or a state digest offered as a result fails here.

Labels:

- Verified SET | on: the producer’s collection slab
- `set_root` | on: its root token
- Membership opening | on: the selected path link
- Sealed claim | on: the authenticated claim token

Numbers used: none.

### Beat 7.9: match-the-dependency-list (26 s)

On screen: The later run’s final-header opening sits beside a discharge-chain token on the paper-side inspection tray. The `deps_d` token and discharge-chain token slide onto the same position. Their equal engraved faces merge into a single outline. The producer membership evidence remains attached to the discharge chain.

Narration: The later run uses SEAL-WITH-DEPS, the seal mode that checks dependencies. Its `deps_d` commits to the ordered dependency list, including repeated uses. The proof constructs a discharge chain from authenticated claims and requires that chain to equal the `deps_d` opened from the run's final state header. Missing, added or reordered entries and removed repetitions fail this equality. Plain SEAL accepts only an empty dependency list.

Labels:

- SEAL-WITH-DEPS | on: the later seal’s mode plate
- `deps_d` | on: the final-header token
- Discharge chain | on: the authenticated dependency-chain token
- Equal dependencies | on: the merged outline

Numbers used: none.

### Beat 7.10: link-the-input-value (31 s)

On screen: The producer’s authenticated outcome opening exposes a Halted wrapper and its value-digest token. The consumer’s authenticated input opening exposes the corresponding value-digest token. These tokens slide into physical coincidence and merge into one outline. The producer claim, outcome opening and consumer input opening remain visible around the equality position.

Narration: When the earlier result supplies the later input, SEAL-WITH-DEPS recomputes the producer's claim identity and opens its `outcome_d` under `sherlock-value/1`. It requires Halted and compares the resulting value digest with the input value digest authenticated by the consumer's `input_d`. A mismatch fails. Rejected and Exhausted outcomes remain usable as claims, but never as input values.

This input link requires SEAL-WITH-DEPS even with an empty dependency list. The later run uses the result without executing the producer again.

Labels:

- Producer claim | on: the authenticated producer claim
- `outcome_d` | on: the producer’s outcome token
- Halted | on: the outcome wrapper
- `input_d` | on: the consumer’s input token
- Equal value digests | on: the merged value-digest outline
- SEAL-WITH-DEPS | on: the consumer’s seal mode plate

Numbers used: The version in `sherlock-value/1`: BRIEF.md §3, steps 1 and 5.

### Beat 7.11: retain-dependency-evidence (24 s)

On screen: A completed consumer proof slab rests beside the dependency evidence that it authenticates. The slab slides along the paper-side proof rail with its claim token and dependency-coverage token attached. The producer’s accepted drawer stays fixed on slate. Hold with the new proof and the reusable producer claim both visible.

Narration: The later seal commits its own claim and step count together with the verified dependency coverage and any input link. The resulting proof retains the evidence for its dependencies. Consuming a claim does not remove it from `runs`. The same producer can support other consumers, and repeated uses remain recorded in the dependency list. Each consumer proves its own execution.

Labels:

- Consumer proof | on: the completed proof slab
- Consumer claim | on: its claim token
- Dependency coverage | on: its coverage token
- Input link | on: its authenticated input-link connection
- `runs` | on: the unchanged producer record cabinet

Numbers used: none.

### Beat 7.12: authenticate-foreign-keys (26 s)

On screen: A foreign proof slab arrives along the paper-side proof rail. Gold key cards for its verifier and adapter sit beside registry-opening paths that end at the same `reg_root` token. A verification sweep crosses the grouped membership evidence. Hold the authenticated cards beside the foreign slab and registry root.

Narration: A foreign proof, a proof from another supported proof system, enters through an authenticated registry of verifier keys. `reg_root` is the digest committing to that registry. The node checks membership openings for both the foreign key and the adapter key against this root. An adapter is the checked relation that translates verified public inputs into a claim. An unregistered key or adapter fails membership.

Labels:

- Foreign proof | on: the incoming slab
- Foreign key | on: the foreign verifier key card
- Adapter key | on: the adapter key card
- Registry membership | on: the opening paths
- `reg_root` | on: the registry-root token

Numbers used: none.

### Beat 7.13: bind-the-verification-rules (26 s)

On screen: Inspect the authenticated registry entry as an exploded row of named cells beside the foreign key card. The supplied interpretation and registered interpretation occupy aligned tracks. A verification sweep crosses the aligned fields. Hold on the matching interpretation with its connection to `reg_root` intact.

Narration: The registry entry fixes the proof system, the curve used for group arithmetic and the commitment scheme that binds the proof's algebraic data. It also fixes setup parameters, the shared cryptographic parameters used by those commitments. The node checks the supplied verification interpretation against this authenticated entry. Substituted arithmetic or commitment rules fail this check, even under a matching key digest.

Labels:

- Proof system | on: the proof-system field
- Curve | on: the curve field
- Commitment scheme | on: the commitment-scheme field
- Setup parameters | on: the setup-parameter field
- Registry entry | on: the authenticated entry tray
- `reg_root` | on: the registry-root token

Numbers used: none.

### Beat 7.14: bind-the-input-interpretation (25 s)

On screen: Inspect the remaining fields of the same registry entry. The foreign proof’s supplied input interpretation sits in a parallel track. A verification sweep crosses the schema, transcript and adapter bindings together. Hold the authenticated fields without displaying a numeric input count or format value.

Narration: The entry also binds the transcript rules, public-input schema and exact input count, adapter digest, format version and admission policy. The schema specifies how the public inputs are interpreted. The foreign verifier checks these supplied details against the registered entry. A sender who keeps the registered key still cannot change the input order, omit an input or substitute a different translation.

Labels:

- Transcript rules | on: the transcript field
- Input schema | on: the public-input schema field
- Input count | on: the count field
- Adapter digest | on: the adapter token
- Format version | on: the format field
- Admission policy | on: the policy field

Numbers used: none.

### Beat 7.15: parse-the-foreign-proof (26 s)

On screen: The foreign slab has an exposed strip of encoded proof elements beside its registered format card. A verification sweep crosses the element strip and its end boundary. The accepted elements retain a gold edge, with no loose element beyond the boundary. Hold on the parsed proof and the authenticated format card.

Narration: The proof parser decodes the foreign proof under the registered format. It checks curve points against the curve equation and required subgroup, the allowed group of points. It checks scalars, the field numbers used in proof arithmetic, against their canonical encodings. It refuses missing or trailing proof data. Malformed encodings and unconsumed data never acquire the meaning of a valid proof.

Labels:

- Proof parser | on: the parsing tray
- Registered format | on: the format card
- Curve points | on: the point-encoding strip
- Canonical scalars | on: the scalar-encoding strip
- Proof boundary | on: the end boundary

Numbers used: none.

### Beat 7.16: reproduce-the-foreign-transcript (23 s)

On screen: Inspect an ordered row of parsed proof elements and public-input cells. Their transcript-derived challenge cells sit at the end of the row, connected to the registered transcript rules. A verification sweep crosses this ordered structure. Hold on the challenge cells attached to the transcript that derives them.

Narration: The foreign transcript verifier processes the proof elements and public inputs in the registered order. It derives challenges, the field values that select the proof's verification equations, from that transcript. The arithmetic checks use those derived challenges. A prover-supplied replacement challenge fails the relation. The sender cannot choose favorable equations after seeing the proof data.

Labels:

- Registered transcript | on: the transcript-rule card
- Proof elements | on: the ordered element row
- Public inputs | on: the input cells
- Derived challenges | on: the resulting challenge cells

Numbers used: none.

### Beat 7.17: check-the-foreign-equations (26 s)

On screen: Inspect the verifier’s arithmetic tray, with challenge cells connected to constraint evaluations and curve-point sums. The resulting sides of the commitment opening equation occupy aligned positions. A verification sweep crosses the equation and its authenticated inputs. Hold on the checked equation without numeric operation totals.

Narration: The foreign arithmetic verifier evaluates the circuit constraints and consistency equations at the transcript's challenge values. It combines scalar multiples of curve points to form the commitment opening equation. That equation checks claimed evaluations against committed polynomials, the algebraic expressions encoding the circuit constraints. The accumulated arithmetic must match this equation. A valid encoding alone cannot establish that the foreign circuit's constraints hold.

Labels:

- Verifier arithmetic | on: the arithmetic tray
- Constraint evaluations | on: the evaluated constraint cells
- Curve-point sum | on: the accumulated group calculation
- Opening equation | on: the final equation
- Derived challenges | on: the connected challenge cells

Numbers used: none.

### Beat 7.18: continue-the-same-verification (28 s)

On screen: Proof slabs for adjacent verification work units rest along the paper-side rail. The predecessor’s committed-state token and the successor’s required starting-state token slide onto the same position and merge into one outline. The state’s named fields remain exposed in a fixed inspection tray. Hold with the continuation link complete and the foreign-claim output position still empty.

Narration: Foreign verification proceeds in bounded work units, each proving a portion of the registered verifier. Every unit authenticates its predecessor's committed state before continuing. That state binds the registry entry, phase, proof position, transcript, partial arithmetic and consumed proof elements. A mismatched continuation fails. The schedule admits no skipped, repeated or replaced element and releases no claim before every phase completes. Verification work adds no machine steps to the run.

Labels:

- Verification work unit | on: the successor slab
- Committed verification state | on: the merged state-token outline
- Registry entry | on: the bound entry field
- Proof position | on: the cursor field
- Partial arithmetic | on: the arithmetic-state field
- Consumed proof elements | on: the consumed-data commitment

Numbers used: none.

### Beat 7.19: finish-the-foreign-check (26 s)

On screen: Inspect the final opening obligation beside the authenticated setup-parameter card. The diagram shows its checked destination: the shared accumulator for matching parameters, or the completed in-circuit check for different parameters. A verification sweep crosses the active obligation and its destination. The final hold keeps the setup match explicit and leaves no obligation detached.

Narration: The final verifier checks the commitment opening obligation using the registered setup parameters. When those parameters match Sherlock's, the obligation joins the accumulator, the carried commitment to deferred checks, for the decider's final pairing. A pairing is the cryptographic operation that closes those checks. With different setup parameters, the foreign verifier completes the opening check inside its bounded work units. It never combines incompatible obligations.

Labels:

- Final opening check | on: the final verifier tray
- Registered setup parameters | on: the authenticated parameter card
- Shared accumulator | on: the existing gold ring
- Completed in circuit | on: the foreign verification completion position

Numbers used: none.

### Beat 7.20: admit-the-foreign-child (30 s)

On screen: The authenticated adapter relation and verified foreign inputs rest beside the completed foreign proof. Its claim token carries an engraved face distinct from the run-claim family. The foreign proof slab slides into the node’s FOREIGN child position with its adapter and foreign verification obligations attached. Hold on the admitted child and its intact evidence connections.

Narration: The authenticated adapter consumes the verified foreign public inputs and derives the foreign claim. The claim binds the foreign key, adapter and translated output in a domain distinct from run claims. An arbitrary foreign field cannot pass for a Sherlock value digest. The proof enters a node as a FOREIGN child, the child type for authenticated foreign claims. The node retains both the adapter's verification obligation and the foreign proof's verification obligation.

Labels:

- Authenticated adapter | on: the adapter relation
- Verified public inputs | on: the foreign input cells
- Foreign claim | on: the foreign claim token
- FOREIGN | on: the node’s child position
- Adapter obligation | on: the adapter evidence connection
- Foreign obligation | on: the foreign proof evidence connection

Numbers used: none.

### Beat 7.21: hand-off-the-record (25 s)

On screen: Return to the contract station’s establishing view. The stored claim drawer slides onto the outgoing handoff plate with `claim_d` on its front and `outcome_d` and `n_steps` in its recess. The program and input tokens remain in the adjacent carrier. The final pose isolates the stored claim as the object passed to network validation.

Narration: The contract's accepted state transition leaves the stored claim under `claim_d`, holding `outcome_d` and `n_steps`. The program and input remain bound through the claim identity and verified call. This record is the result the network checks and records. Later proofs authenticate dependencies through their own proof checks, so the stored record and the sealed proof serve distinct parts of the same claim.

Labels:

- `claim_d` | on: the drawer front
- `outcome_d` | on: the stored outcome token
- `n_steps` | on: the stored step-count object
- `prog_d` | on: the program token beside the drawer
- `input_d` | on: the input token beside the drawer
- Stored claim | on: the outgoing handoff plate

Numbers used: none.

## Step 8: The network

Arrives: The stored claim: drawer `claim_d` holding `outcome_d` and `n_steps`, on the handoff plate.  
Leaves: The accepted claim on the final handoff plate, with the original four public-value objects grouped around its stored record.  
Scene length: 203 s (target 80 s)

### Beat 8.1: check-at-each-node (23 s)

On screen: Establish the network station with the stored drawer in the incoming handoff pose. Slate validation trays hold representations of the same transaction, each beneath the same engraved verifier key card. Verification sweeps cross the trays independently in parallel. Hold on their matching accepted state transitions, with the original drawer still in view.

Narration: The stored claim reaches network validation as the result of the transaction's contract state transition. Midnight nodes on ledger 9.1.0.1 each verify the transaction independently. Each uses the contract's verifier key and checks the bound public values and contract operations against ledger state. Each node rejects an invalid proof or a replay on its own checks.

Labels:

- Midnight nodes | on: the validation-tray group
- Ledger 9.1.0.1 | on: the network station
- Shared verifier key | on: the matching key cards
- Independent verification | on: the validation tracks
- Stored claim | on: the incoming drawer

Numbers used: Ledger 9.1.0.1: BRIEF.md §3, step 8.

### Beat 8.2: keep-verification-fixed (24 s)

On screen: Inspect a node’s validation tray with the final seal under its gold key card. The witness tape remains on the distant paper-side rail, marked by a break rather than a frame count. A verification sweep crosses only the final seal and bound call values. Hold on the unchanged seal footprint and verifier key.

Narration: Each node uses the same verifier key for every program. It checks the final decider proof. It does not execute the witness tape or check each leaf. The proof remains about 6.4 KB. The cost of checking that proof does not depend on the length of the run. Longer execution adds prover work and leaves the node's run-verification work unchanged.

Labels:

- Decider proof | on: the final seal
- About 6.4 KB | on: the seal’s size annotation
- Shared verifier key | on: the node’s key card
- Witness tape | on: the stationary paper-side tape

Numbers used: About 6.4 KB: BRIEF.md §3, step 6.

### Beat 8.3: accept-the-run-fact (22 s)

On screen: The claim drawer and its public-value carrier form a single accepted-claim arrangement. A verification sweep crosses that arrangement and leaves a short bright-gold edge. Hold with the original program and input tokens beside the drawer and the outcome and step-count objects in its storage recess. Fixed connections lead toward a contract-use position and a later-proof position.

Narration: After acceptance, the claim is a fact on chain: program `prog_d` on input `input_d` produced `outcome_d` in `n_steps` steps. Any contract or later proof can rely on that statement. A contract identifies it through `claim_d` and the stored record. A later proof checks the sealed claim and any required value opening before using its result.

Labels:

- `prog_d` | on: the original program token
- `input_d` | on: the original input token
- `outcome_d` | on: the stored outcome token
- `n_steps` | on: the stored step-count object
- `claim_d` | on: the drawer front
- Accepted claim | on: the complete arrangement

Numbers used: none.

### Beat 8.4: join-complete-runs (25 s)

On screen: Inspect authenticated child collection roots beside a node slab. The join-domain marker and child roots line up in their stated order and slide into the fixed Poseidon block. A new `set_root` token emerges on the slab. Hold with the child-opening paths visible beneath the resulting root.

Narration: Collections combine sealed runs with JOIN, the relation that joins claim sets. It computes `set_root = H(SHK_SET_JOIN, l, r)`, where `H` is Poseidon and `l` and `r` are the authenticated child roots. The domain tag identifies a set join. Recomputing this digest binds both children in order and rejects a substituted child. Membership openings recover each included claim from the resulting root.

Labels:

- JOIN | on: the node’s relation plate
- `SHK_SET_JOIN` | on: the domain marker
- Left collection | on: the left child root
- Right collection | on: the right child root
- `set_root` | on: the emerging collection token
- Membership openings | on: the retained child paths

Numbers used: none.

### Beat 8.5: bind-the-proof-graph (26 s)

On screen: A static proof graph shows joined branches and a shared dependency, with break marks for omitted structure. Its authenticated relation and child descriptors line up at the gold Poseidon block. The hash motion produces `shape_d`, which rests beside the collection proof. The graph remains visible behind the token without exposing memory-state fields.

Narration: Runs join in any tree or DAG, a directed graph without cycles that permits shared dependencies. `shape_d` commits to the proof graph through relation types, enabled child positions, key selections, child shapes and authenticated work descriptors. Each node derives these fields from its checked children and work. A proof cannot claim another graph. The shape contains no physical state commitment or address.

Labels:

- Proof graph | on: the static graph
- Shared dependency | on: the reused claim token
- Authenticated descriptors | on: the hash-input tray
- `shape_d` | on: the resulting shape token

Numbers used: none.

### Beat 8.6: settle-the-collection (27 s)

On screen: Inspect a collection seal with `reg_root`, `set_root` and `shape_d` on its public-value strip. The contract’s registry-root token sits beside the published registry token with the same engraved face. A verification sweep crosses the seal, its statement and the registry binding. Hold the accepted collection statement above its authenticated claim paths.

Narration: A collection settles as `[reg_root, set_root, shape_d]`. `reg_root` commits to admitted verifier keys and their interpretation. `set_root` commits to the authenticated claims and their dependency coverage. `shape_d` commits to the graph that combines them. Verification binds these values and requires the registry root to match contract state. A stale registry fails. The final proof size and verification cost stay independent of graph depth and claim count.

Labels:

- Collection statement | on: the collection seal’s public-value strip
- `reg_root` | on: the registry token
- `set_root` | on: the collection-root token
- `shape_d` | on: the shape token
- Contract registry | on: the registry token read from contract state

Numbers used: none.

### Beat 8.7: preserve-claim-identity (24 s)

On screen: Different stationary collection graphs lead to the same engraved member-claim face. The member tokens slide onto the accepted drawer’s claim-key position and merge into one outline. The collection-root and shape tokens remain separate in their own tracks. Hold on the shared claim identity and the unchanged stored record.

Narration: Different groupings of the same claims can produce different collection roots and shape digests. These commitments describe the collection. Each member keeps its `claim_d`. Acceptance, replay refusal and consumption use claim identity. A shared claim can feed several branches, and a second path grants no second settlement. Collection verification creates no repeated per-claim effect keyed by collection shape.

Labels:

- Collection roots | on: the separate collection-root tokens
- Shape digests | on: the separate shape tokens
- `claim_d` | on: the merged claim-key outline
- Shared claim | on: the unchanged accepted drawer

Numbers used: none.

### Beat 8.8: follow-the-whole-path (32 s)

On screen: Begin with the accepted claim resting on the final handoff plate. The camera pulls back along the established rail into the piece’s only whole-bench wide view. All stations hold their named handoff objects in place, from the source sheet to the accepted drawer. End with the original public-value objects together at the accepted claim, with only `outcome_d` and `n_steps` inside the storage recess.

Narration: Moriarty source becomes the ROM program image with `prog_d` and `input_d`. Execution hands over the witness tape and its outcome frame. Leaf proving turns slices into slabs with their public pins. Nodes join the slabs into a root slab carrying its accumulator ring. The decider hands over a seal and `[prog_d, input_d, outcome_d, n_steps]`. The transaction carries them in an envelope. The contract produces the stored claim drawer. Midnight nodes accept the claim, with the same public values bound throughout.

Labels:

- `prog_d` | on: the original program token at the accepted claim
- `input_d` | on: the original input token at the accepted claim
- `outcome_d` | on: the stored outcome token
- `n_steps` | on: the stored step-count object
- `claim_d` | on: the drawer front
- Accepted claim | on: the final handoff plate

Numbers used: none.

## Notes for the director

- **Timing:** Step 6 totals 90 s, step 7 totals 534 s, and step 8 totals 203 s. Each exceeds its target by more than a quarter. The additional time preserves separate explanations of call binding, replay refusal, dependency authentication, foreign verification and collection settlement. Durations use whitespace-separated narration words, including identifiers and formula terms, divided by 2.5 and rounded up, with the required minimum of 12 s. Every narration contains between 30 and 90 words.

- **Contract execution and network acceptance:** Step 7 explains the contract’s accepted state transition. Step 8 explains independent node validation and acceptance of that transition. Do not imply that a separate contract actor finalizes the record before nodes validate it.

- **Composition stays on the prover’s side:** The dependency and foreign-proof inspection beats show evidence constructed on the paper-side proof rail. The accepted contract record stays on slate. These beats do not depict the contract executing the later program or performing the foreign verifier’s internal work directly. The network checks the resulting proof.

- **Public objects and stored fields:** Keep the original public-value carrier visible. Represent digest values with engraved hexagonal tokens and the step count with the established counter object in its carrier position. Never give `n_steps` a numeric example. In the closing arrangement, `prog_d` and `input_d` remain beside the drawer as bound public values. Only `outcome_d` and `n_steps` occupy its storage recess. The drawer front carries `claim_d`.

- **Motion:** Use physical coincidence for equality, a sweep followed by a brief gold edge for successful verification, and a stop, small return movement and red hatch for refusal. Every Poseidon operation uses the same ordered-input hash-block motion. Keep graph structure and explanatory fields still during the key motion. Every beat ends with a legible still hold.

- **Source disagreement: README.md, Sherlock section:** Its older pipeline publishes initial and final state digests and describes a contract bound to sealed program and initial-state fields. It also says the built replay identity includes the final state digest. The script publishes only `[prog_d, input_d, outcome_d, n_steps]`, passes `prog_d` as call data, derives `claim_d` from the canonical run formula, and stores no state digest.

- **Source disagreement: README.md, Sherlock section:** Its historical implementation describes a linear chain and treats universal recursion and broader composition as later work. Its older discussion also says execution commitments depend on leaf boundaries, while its dense-memory discussion describes partition independence. The script follows BRIEF.md: the system runs with universal keys, supports collections and shared dependencies, and refuses replay after a change of leaf partition.

- **Source disagreement: openspec/changes/sherlock-vm-consensus-20261107/design.md, decider and contract section; requirements.md, SRC-007; llm-wiki/wiki/entities/AcceptRunContract.md:** These retain a sealed program identity and describe a contract serving a particular program. The script follows BRIEF.md §2: `prog_d` is call data, and the same contract and key set serve every program and input.

- **Source disagreement: docs/sherlock/stagenet-gp-recursion.md:** The historical call signature includes initial and final state fields and a directly encoded result. Its replay identity also includes the older statement fields. The script names `SherlockRun.acceptRun` without reproducing that obsolete signature and uses the canonical public values, replay identity and stored payload.

- **Source disagreement: llm-wiki/wiki/entities/AcceptRunContract.md; llm-wiki/wiki/entities/MidnightLedger9.md; README.md:** These describe missing network acceptance and incomplete transaction qualification. The script follows the requested working-system premise and BRIEF.md §3, step 8: Midnight nodes on ledger 9.1.0.1 validate the transaction and accept the claim.

- **Source disagreement: llm-wiki/wiki/concepts/C6OutcomeOnlyPublication.md and older passages of design.md:** These explain replay problems through physical layouts that vary with leaf partitioning. The script uses the canonical partition-independent run and memory semantics. Replay identity excludes physical commitments regardless.

- **Source disagreement: research/sherlock-final-design-20261111/ARCHITECTURE.md and the modular-leaf specifications:** These retain unresolved status language around dependency consumption, foreign verification, graph shape and claim-count removal. The script presents the mechanisms as running and carries no claim count, following BRIEF.md.

- **Source disagreement: ARCHITECTURE.md, dependency-consumption kit, and requirements.md, input admission:** These distinguish seal-time input linking from a possible execution-time consumption rule and discuss authenticating dependencies at genesis. The script follows BRIEF.md’s seal-time choice. It checks the ordered dependency commitment and separately checks an input link when the producer’s Halted value supplies the consumer’s input. It introduces no execution-time consume instruction.

- **Source disagreement: ARCHITECTURE.md and requirements.md, input commitment formulas:** These include resource-policy fields in an input commitment formula. BRIEF.md names `input_d` as the input value digest. The script preserves that public description and explains input composition through the value digest authenticated by `input_d`, without introducing another public-input schema.

- **Source disagreement: general-purpose.md and design.md:** Their historical gap sections describe restricted keys, missing foreign admission and limited aggregation shapes. The script follows BRIEF.md’s universal key set, registered FOREIGN children and arbitrary trees or DAGs.

- **Source disagreement: ARCHITECTURE.md module graph and README.md memory discussion:** These retain an alternative memory backend and boundary commitment. Neither appears in this script. The closing path uses only the canonical program image, witness tape, leaf slabs, root slab, decider seal, transaction envelope and stored claim.

- **Quantities intentionally shown without numeric values:** DUST fee amount; total transaction size; verifier-key size; contract storage size; verifier operation counts and execution times; number of network nodes; number of stored claims; number, depth and repetition of dependencies; membership-path lengths; registry population and membership depth; foreign public-input count; registry format value; proof-element counts; subgroup size; arithmetic-operation counts; verification work-unit size, budget and count; graph depth, branch count and collection size. Collection proof size is described as independent of graph depth and claim count without assigning it the run proof’s byte figure.

- **Scope of the proof-size figure:** About 6.4 KB describes the decider proof carried by the run transaction. It does not describe the full transaction envelope, registry evidence, verifier keys or stored record.

- **Collection semantics:** `set_root` authenticates a structured collection, including dependency coverage. It is not presented as a grouping-independent identifier for a mathematical set. `shape_d` is checked as a commitment to the proof graph, but claim identity controls replay and consumption. Collection settlement does not automatically repeat individual claim effects.

- **Mechanisms unexplained from the sources:** None within the requested scope. The script explains dependency discharge, authenticated value linking, foreign registry interpretation, proof parsing, transcript derivation, verifier arithmetic, bounded continuation, final opening checks, adapter binding and collection settlement without adding unsupported numeric parameters.