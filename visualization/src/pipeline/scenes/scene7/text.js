// Generated from STORYBOARD.md scene 7: titles, narration and label text, verbatim.
export const TEXT = [
 {
  "id": "check-the-installed-key",
  "idx": 0,
  "dur": 25,
  "title": "Verifier key check",
  "labels": [
   {
    "text": "<code>acceptRun</code>",
    "on": "the contract entrance"
   },
   {
    "text": "Verifier key",
    "on": "the fixed gold key card"
   },
   {
    "text": "Decider proof",
    "on": "the seal"
   },
   {
    "text": "Call transcript",
    "on": "the public-value strip"
   },
   {
    "text": "<code>runs</code>",
    "on": "the cabinet"
   }
  ],
  "narration": "From here the work is on-chain. The transaction reaches <code>acceptRun</code>, the contract operation that accepts a proved run. It checks the decider proof against the contract's verifier key, the public key that defines the accepted proof relation, and the call transcript. A failed check refuses the call, so a sender cannot pass off a proof of an unrelated relation. The same decider verifier key serves every program and input. The contract verifies this one proof and never runs the Sherlock chip."
 },
 {
  "id": "derive-the-claim",
  "idx": 1,
  "dur": 27,
  "title": "Claim identity",
  "labels": [
   {
    "text": "<code>SHK_RUN</code>",
    "on": "the domain marker"
   },
   {
    "text": "Claim version",
    "on": "the version cell"
   },
   {
    "text": "Poseidon",
    "on": "the gold hash block"
   },
   {
    "text": "<code>claim_d</code>",
    "on": "the emerging gold-rimmed token"
   },
   {
    "text": "<code>n_steps</code>",
    "on": "the stationary step-count position"
   }
  ],
  "narration": "The contract recomputes <code>claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)</code>. Poseidon is the digest function. <code>SHK_RUN</code> marks the run-claim domain, separating this digest from other kinds of digest, and <code>1</code> is the claim version. The remaining inputs come from the verified call.\n\nWith the identity derived from these values, no sender can give an existing result a fresh identity. The step count stays outside this identity."
 },
 {
  "id": "look-up-the-claim",
  "idx": 2,
  "dur": 21,
  "title": "Claim lookup",
  "labels": [
   {
    "text": "<code>runs</code>",
    "on": "the cabinet"
   },
   {
    "text": "<code>claim_d</code>",
    "on": "the lookup token"
   },
   {
    "text": "Claim absent",
    "on": "the unused drawer position"
   }
  ],
  "narration": "The contract looks up the recomputed <code>claim_d</code> in <code>runs</code>, its record of accepted run claims. It compares this digest with the stored claim keys. An absent key passes the freshness check. An existing key refuses the call. The lookup stops a valid proof from creating a second acceptance of the same claim."
 },
 {
  "id": "store-the-outcome",
  "idx": 3,
  "dur": 22,
  "title": "Stored outcome",
  "labels": [
   {
    "text": "<code>claim_d</code>",
    "on": "the drawer front"
   },
   {
    "text": "<code>outcome_d</code>",
    "on": "the stored teal token"
   },
   {
    "text": "<code>n_steps</code>",
    "on": "the stored step-count object"
   },
   {
    "text": "<code>runs</code>",
    "on": "the cabinet"
   }
  ],
  "narration": "For a fresh <code>claim_d</code>, the contract stores <code>outcome_d</code> and <code>n_steps</code> under that key. These are the values the verified proof binds, so the record preserves both the result identity and the authenticated step count. The contract stores no state digest. One proof verification, the computation of <code>claim_d</code>, one lookup and one insert are the whole on-chain work of a call. Memory roots, tail digests, memory lengths and addresses remain inside the proof."
 },
 {
  "id": "refuse-the-same-proof",
  "idx": 4,
  "dur": 22,
  "title": "Same proof refused",
  "labels": [
   {
    "text": "Same proof",
    "on": "the repeated seal"
   },
   {
    "text": "<code>claim_d</code>",
    "on": "the occupied drawer"
   },
   {
    "text": "Claim present",
    "on": "the cabinet lookup position"
   },
   {
    "text": "Replay refused",
    "on": "the repeated envelope"
   }
  ],
  "narration": "A sender submits the same proof again with the same public values. Proof verification succeeds again, the contract derives the same <code>claim_d</code>, and its lookup finds the existing record and refuses the repeated call. The stored outcome and step count remain unchanged. A valid proof lets its claim be accepted once, never repeatedly."
 },
 {
  "id": "refuse-another-partition",
  "idx": 5,
  "dur": 22,
  "title": "Another partition refused",
  "labels": [
   {
    "text": "Different leaf boundaries",
    "on": "the comparison tape strips"
   },
   {
    "text": "Different proof",
    "on": "the comparison seal"
   },
   {
    "text": "Same claim",
    "on": "the matching engraved claim faces"
   },
   {
    "text": "Replay refused",
    "on": "the comparison envelope"
   }
  ],
  "narration": "The sender proves the same run again with different boundaries between leaves. The proof changes, while <code>prog_d</code>, <code>input_d</code>, <code>outcome_d</code> and <code>n_steps</code> stay equal. The contract recomputes the same <code>claim_d</code> and refuses the call. Leaf boundaries, proof randomness and aggregation shape never enter this identity, so changing the proof's construction cannot bypass replay refusal."
 },
 {
  "id": "authenticate-without-counting",
  "idx": 6,
  "dur": 24,
  "title": "Authentication without a count",
  "labels": [
   {
    "text": "Verified collection",
    "on": "the collection slab"
   },
   {
    "text": "Membership opening",
    "on": "the claim-opening path"
   },
   {
    "text": "<code>claim_d</code>",
    "on": "the accepted claim token"
   }
  ],
  "narration": "The public statement carries no count of claims. A count says nothing about whether a particular claim is valid or present. Proof checks establish validity, digest openings establish membership, and <code>claim_d</code> controls replay refusal. Collections combine authenticated children without adding a claim counter, so shared dependencies require no count that grows with every path through the proof graph."
 },
 {
  "id": "open-a-sealed-dependency",
  "idx": 7,
  "dur": 29,
  "title": "Sealed dependency opened",
  "labels": [
   {
    "text": "Verified SET",
    "on": "the producer’s collection slab"
   },
   {
    "text": "<code>set_root</code>",
    "on": "its root token"
   },
   {
    "text": "Membership opening",
    "on": "the selected path link"
   },
   {
    "text": "Sealed claim",
    "on": "the authenticated claim token"
   }
  ],
  "narration": "A later run consumes this claim at its own seal. Consume means authenticate a claim for use as a dependency. The producer supplies a verified SET, a digest commitment to sealed claims. Hash openings connect the requested claim to that verified root, through joined collections or dependency coverage. Every opening must reproduce its parent digest. An unrelated claim, an unfinished segment or a state digest offered as a result fails here."
 },
 {
  "id": "match-the-dependency-list",
  "idx": 8,
  "dur": 26,
  "title": "Dependency list match",
  "labels": [
   {
    "text": "SEAL-WITH-DEPS",
    "on": "the later seal’s mode plate"
   },
   {
    "text": "<code>deps_d</code>",
    "on": "the final-header token"
   },
   {
    "text": "Discharge chain",
    "on": "the authenticated dependency-chain token"
   },
   {
    "text": "Equal dependencies",
    "on": "the merged outline"
   }
  ],
  "narration": "The later run uses SEAL-WITH-DEPS, the seal mode that checks dependencies. Its <code>deps_d</code> commits to the ordered dependency list, including repeated uses. The proof constructs a discharge chain from authenticated claims and requires that chain to equal the <code>deps_d</code> opened from the run's final state header. Missing, added or reordered entries and removed repetitions fail this equality. Plain SEAL accepts only an empty dependency list."
 },
 {
  "id": "link-the-input-value",
  "idx": 9,
  "dur": 31,
  "title": "Input value link",
  "labels": [
   {
    "text": "Producer claim",
    "on": "the authenticated producer claim"
   },
   {
    "text": "<code>outcome_d</code>",
    "on": "the producer’s outcome token"
   },
   {
    "text": "Halted",
    "on": "the outcome wrapper"
   },
   {
    "text": "<code>input_d</code>",
    "on": "the consumer’s input token"
   },
   {
    "text": "Equal value digests",
    "on": "the merged value-digest outline"
   },
   {
    "text": "SEAL-WITH-DEPS",
    "on": "the consumer’s seal mode plate"
   }
  ],
  "narration": "When the earlier result supplies the later input, SEAL-WITH-DEPS recomputes the producer's claim identity and opens its <code>outcome_d</code> under <code>sherlock-value/1</code>. It requires Halted and compares the resulting value digest with the value digest <code>val_d</code> bound in the consumer's <code>input_d</code>. A mismatch fails. Rejected and Exhausted outcomes remain usable as claims, but never as input values.\n\nThis input link requires SEAL-WITH-DEPS even with an empty dependency list. The later run uses the result without executing the producer again."
 },
 {
  "id": "retain-dependency-evidence",
  "idx": 10,
  "dur": 24,
  "title": "Retained dependency evidence",
  "labels": [
   {
    "text": "Consumer proof",
    "on": "the completed proof slab"
   },
   {
    "text": "Consumer claim",
    "on": "its claim token"
   },
   {
    "text": "Dependency coverage",
    "on": "its coverage token"
   },
   {
    "text": "Input link",
    "on": "its authenticated input-link connection"
   },
   {
    "text": "<code>runs</code>",
    "on": "the unchanged producer record cabinet"
   }
  ],
  "narration": "The later seal commits its own claim and step count together with the verified dependency coverage and any input link. The resulting proof retains the evidence for its dependencies. Consuming a claim does not remove it from <code>runs</code>. The same producer can support other consumers, and repeated uses remain recorded in the dependency list. Each consumer proves its own execution."
 },
 {
  "id": "authenticate-foreign-keys",
  "idx": 11,
  "dur": 26,
  "title": "Foreign keys authenticated",
  "labels": [
   {
    "text": "Foreign proof",
    "on": "the incoming slab"
   },
   {
    "text": "Foreign key",
    "on": "the foreign verifier key card"
   },
   {
    "text": "Adapter key",
    "on": "the adapter key card"
   },
   {
    "text": "Registry membership",
    "on": "the opening paths"
   },
   {
    "text": "<code>reg_root</code>",
    "on": "the registry-root token"
   }
  ],
  "narration": "A foreign proof, a proof from another supported proof system, enters through an authenticated registry of verifier keys. <code>reg_root</code> is the digest committing to that registry. The node checks membership openings for both the foreign key and the adapter key against this root. An adapter is the checked relation that translates verified public inputs into a claim. An unregistered key or adapter fails membership."
 },
 {
  "id": "bind-the-verification-rules",
  "idx": 12,
  "dur": 25,
  "title": "Verification rules bound",
  "labels": [
   {
    "text": "Proof system",
    "on": "the proof-system field"
   },
   {
    "text": "Curve",
    "on": "the curve field"
   },
   {
    "text": "Commitment scheme",
    "on": "the commitment-scheme field"
   },
   {
    "text": "Setup parameters",
    "on": "the setup-parameter field"
   },
   {
    "text": "Registry entry",
    "on": "the authenticated entry tray"
   },
   {
    "text": "<code>reg_root</code>",
    "on": "the registry-root token"
   }
  ],
  "narration": "The registry entry fixes the proof system, the curve used for group arithmetic and the commitment scheme that binds the proof's algebraic data. It also fixes setup parameters, the shared cryptographic parameters used by those commitments. The node checks the supplied verification interpretation against this authenticated entry. Substituted arithmetic or commitment rules fail this check, even under a matching key digest."
 },
 {
  "id": "bind-the-input-interpretation",
  "idx": 13,
  "dur": 25,
  "title": "Input interpretation bound",
  "labels": [
   {
    "text": "Transcript rules",
    "on": "the transcript field"
   },
   {
    "text": "Input schema",
    "on": "the public-input schema field"
   },
   {
    "text": "Input count",
    "on": "the count field"
   },
   {
    "text": "Adapter digest",
    "on": "the adapter token"
   },
   {
    "text": "Format version",
    "on": "the format field"
   },
   {
    "text": "Admission policy",
    "on": "the policy field"
   }
  ],
  "narration": "The entry also binds the transcript rules, public-input schema and exact input count, adapter digest, format version and admission policy. The schema specifies how the public inputs are interpreted. The foreign verifier checks these supplied details against the registered entry. A sender who keeps the registered key still cannot change the input order, omit an input or substitute a different translation."
 },
 {
  "id": "parse-the-foreign-proof",
  "idx": 14,
  "dur": 25,
  "title": "Foreign proof parsed",
  "labels": [
   {
    "text": "Proof parser",
    "on": "the parsing tray"
   },
   {
    "text": "Registered format",
    "on": "the format card"
   },
   {
    "text": "Curve points",
    "on": "the point-encoding strip"
   },
   {
    "text": "Canonical scalars",
    "on": "the scalar-encoding strip"
   },
   {
    "text": "Proof boundary",
    "on": "the end boundary"
   }
  ],
  "narration": "The proof parser decodes the foreign proof under the registered format. It checks curve points against the curve equation and required subgroup, the allowed group of points. It checks scalars, the field numbers used in proof arithmetic, against their canonical encodings. It refuses missing or trailing proof data. Malformed encodings and unconsumed data never acquire the meaning of a valid proof."
 },
 {
  "id": "reproduce-the-foreign-transcript",
  "idx": 15,
  "dur": 22,
  "title": "Foreign transcript reproduced",
  "labels": [
   {
    "text": "Registered transcript",
    "on": "the transcript-rule card"
   },
   {
    "text": "Proof elements",
    "on": "the ordered element row"
   },
   {
    "text": "Public inputs",
    "on": "the input cells"
   },
   {
    "text": "Derived challenges",
    "on": "the resulting challenge cells"
   }
  ],
  "narration": "The foreign transcript verifier processes the proof elements and public inputs in the registered order. It derives challenges, the field values that select the proof's verification equations, from that transcript. The arithmetic checks use those derived challenges. A prover-supplied replacement challenge fails the relation. The sender cannot choose favorable equations after seeing the proof data."
 },
 {
  "id": "check-the-foreign-equations",
  "idx": 16,
  "dur": 26,
  "title": "Foreign equations checked",
  "labels": [
   {
    "text": "Verifier arithmetic",
    "on": "the arithmetic tray"
   },
   {
    "text": "Constraint evaluations",
    "on": "the evaluated constraint cells"
   },
   {
    "text": "Curve-point sum",
    "on": "the accumulated group calculation"
   },
   {
    "text": "Opening equation",
    "on": "the final equation"
   },
   {
    "text": "Derived challenges",
    "on": "the connected challenge cells"
   }
  ],
  "narration": "The foreign arithmetic verifier evaluates the circuit constraints and consistency equations at the transcript's challenge values. It combines scalar multiples of curve points to form the commitment opening equation. That equation checks claimed evaluations against committed polynomials, the algebraic expressions encoding the circuit constraints. The accumulated arithmetic must match this equation. A valid encoding alone cannot establish that the foreign circuit's constraints hold."
 },
 {
  "id": "continue-the-same-verification",
  "idx": 17,
  "dur": 28,
  "title": "Continued verification",
  "labels": [
   {
    "text": "Verification work unit",
    "on": "the successor slab"
   },
   {
    "text": "Committed verification state",
    "on": "the merged state-token outline"
   },
   {
    "text": "Registry entry",
    "on": "the bound entry field"
   },
   {
    "text": "Proof position",
    "on": "the cursor field"
   },
   {
    "text": "Partial arithmetic",
    "on": "the arithmetic-state field"
   },
   {
    "text": "Consumed proof elements",
    "on": "the consumed-data commitment"
   }
  ],
  "narration": "Foreign verification proceeds in bounded work units, each proving a portion of the registered verifier. Every unit authenticates its predecessor's committed state before continuing. That state binds the registry entry, phase, proof position, transcript, partial arithmetic and consumed proof elements. A mismatched continuation fails. The schedule admits no skipped, repeated or replaced element and releases no claim before every phase completes. Verification work adds no machine steps to the run."
 },
 {
  "id": "finish-the-foreign-check",
  "idx": 18,
  "dur": 26,
  "title": "Final foreign check",
  "labels": [
   {
    "text": "Final opening check",
    "on": "the final verifier tray"
   },
   {
    "text": "Registered setup parameters",
    "on": "the authenticated parameter card"
   },
   {
    "text": "Shared accumulator",
    "on": "the existing gold ring"
   },
   {
    "text": "Completed in circuit",
    "on": "the foreign verification completion position"
   }
  ],
  "narration": "The final verifier checks the commitment opening obligation using the registered setup parameters. When those parameters match Sherlock's, the obligation joins the accumulator, the carried commitment to deferred checks, for the decider's final pairing. A pairing is the cryptographic operation that closes those checks. With different setup parameters, the foreign verifier completes the opening check inside its bounded work units. It never combines incompatible obligations."
 },
 {
  "id": "admit-the-foreign-child",
  "idx": 19,
  "dur": 29,
  "title": "Foreign child admitted",
  "labels": [
   {
    "text": "Authenticated adapter",
    "on": "the adapter relation"
   },
   {
    "text": "Verified public inputs",
    "on": "the foreign input cells"
   },
   {
    "text": "Foreign claim",
    "on": "the foreign claim token"
   },
   {
    "text": "FOREIGN",
    "on": "the node’s child position"
   },
   {
    "text": "Adapter obligation",
    "on": "the adapter evidence connection"
   },
   {
    "text": "Foreign obligation",
    "on": "the foreign proof evidence connection"
   }
  ],
  "narration": "The authenticated adapter consumes the verified foreign public inputs and derives the foreign claim. The claim binds the foreign key, adapter and translated output in a domain distinct from run claims. An arbitrary foreign field cannot pass for a Sherlock value digest. The proof enters a node as a FOREIGN child, the child type for authenticated foreign claims. The node retains both the adapter's verification obligation and the foreign proof's verification obligation."
 },
 {
  "id": "hand-off-the-record",
  "idx": 20,
  "dur": 25,
  "title": "Record handoff",
  "labels": [
   {
    "text": "<code>claim_d</code>",
    "on": "the record front | kind: state"
   },
   {
    "text": "<code>outcome_d</code>",
    "on": "the stored token | kind: outcome"
   },
   {
    "text": "<code>n_steps</code>",
    "on": "the stored collar | kind: state"
   },
   {
    "text": "<code>prog_d</code>",
    "on": "the carrier | kind: program"
   },
   {
    "text": "<code>input_d</code>",
    "on": "the carrier | kind: input"
   },
   {
    "text": "Stored claim",
    "on": "the record | kind: state"
   }
  ],
  "narration": "The contract's accepted state transition leaves the stored claim under <code>claim_d</code>, holding <code>outcome_d</code> and <code>n_steps</code>. The program and input remain bound through the claim identity and verified call. This record is the result the network checks and records. Later proofs authenticate dependencies through their own proof checks, so the stored record and the sealed proof serve distinct parts of the same claim."
 }
];
