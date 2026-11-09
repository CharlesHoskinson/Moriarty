# Council review 2.14: aggregation design and formal results

Task: `openspec/changes/sherlock-vm-consensus-20261107/tasks.md` item 2.14. Date: 2026-11-09.
Skill: `council` (advisory, blinded non-author review, dissent preserved, typed abstentions).

This review is an input, not evidence (assurance §3). It does not tick 2.14; the lead does that.
It does not change any requirement, constraint, model or theorem. It lists what each OBJECT asks
to change and gives the lead's assessment of each.

Raw lane outputs, the prompt, the bundle manifest and the round scripts are in
`research/sherlock-speed-replan-20261108/council-review/`.

## 1. Round

### 1.1 Bundle

| Field | Value |
|---|---|
| `base_sha` | `49bb0200b848b72e45eb377c1ed63936874cc427` |
| `head_sha` | `49bb0200b848b72e45eb377c1ed63936874cc427` |
| diff hash | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` (empty diff: the review covers the state at that commit) |
| file manifest | `council-review/bundle-MANIFEST.sha256`, SHA-256 `f7519e46987713d8a1025d5644b1646e14ce98aacaf8deea678a25c1f08c47ae` (30 files) |
| prompt | `council-review/PROMPT.md`, SHA-256 `cfc28b2093d4c2b1d6243032325c329c7f032ad8b19c930f527089540a2da741`; the same prompt for every lane |

Bundle contents (copies at `49bb0200b`, built by `council-review/build-bundle.sh`):
design.md, requirements.md, assurance.md, the task excerpt (2.0-2.16, 4.1-4.10, 5.2-5.10);
the F20 model, harness, runner and the two F20 run logs; the F21 Lean files, runner and the two
F21 run logs; `tree.md`, `prover.md`, `order.md`; skeleton-coding `builds.md`, `testing.md`,
`review.md`; `qa/gemini-quint/ivc/` and `qa/gemini-quint/c6/` (reports and models, independent
Quint checks of 2026-11-09); the pilot record `pilot-smoke.md`, `pilot-amendment-1.md`,
`pilot-protocol.md`; `crates/mori-recursion/src/ivc.rs`.

Blinding: `council-review/blind.sh` replaced vendor, model, CLI and earlier-lane names in the
bundle copies with neutral tags (`vendorX`, `model-A`, `member-f2`, `indep-quint`, ...). It did
not change substance. Lanes ran read-only from the bundle directory, not from the repository.

### 1.2 Lanes

Every lane ran under `heavy-run --mem 8G --name council-<lane>`, at most two at once, with
subscription logins only (no API key variables set). Each CLI passed a smoke prompt
("What is 1+1? Reply with exactly: 2") before the round.

| Lane | Family | CLI and model | Mode | Wall (UTC) | Transport |
|---|---|---|---|---|---|
| codex | OpenAI | `codex exec -m gpt-6.1-sol`, effort high | `-s read-only` | 22:00:22-22:07:18 | exit 0, complete answer in the required shape |
| agy | Google | `agy --model gemini-4-argon-high` | commands, writes, URLs and MCP denied by `--settings` | 22:00:22-22:10:51 | exit 0, status SUCCESS, complete answer |
| grok | xAI | `grok -m grok-4.7` (billed `grok-4.7-build`), effort xhigh | no subagents, no web search | 22:07:27-22:20:05 | exit 0, `end_turn`, 12 turns, complete answer |
| muse | Meta | `muse exec --model muse-wonder-full-1.0`, effort high, contributor mode off | sandboxed workspace copy, network restricted | 22:11:04-22:16:47 | exit 0, terminal `completed`, complete answer |

Family exclusion: the design, the F20 model and the F21 theorems come from Anthropic-family
lanes, so no Anthropic lane reviews. The Google lane reviews two inputs from its own family
(`qa/gemini-quint/*`); those are inputs, not the reviewed candidate, and the Google lane reports
a defect in one of them (agy 1.6).

Transport notes: the grok answer starts with two sentences of progress prose before the
`BUNDLE` line, and its `manifest=` value is misquoted (the first 27 hex digits are correct, the
tail repeats the tail of the diff hash). `base`, `head` and `diff` match. The lead classifies the answer as
completed and bound to the bundle triple, with the manifest misquote recorded here. No lane
failed, timed out or needed a retry.

### 1.3 Classification

Four completed substantive verdict sets from four distinct lanes and four model families. Quorum
(three verdicts, two families) is met. No lane abstains on any item. Every lane returns OBJECT on
item 1 and item 4, so the round result is `changes_requested`. Under the skill rules each
admissible OBJECT needs a fix and a deterministic check; it does not start a second round.

## 2. Verdicts

| Item | codex (OpenAI) | agy (Google) | grok (xAI) | muse (Meta) |
|---|---|---|---|---|
| 1 Soundness of composition | OBJECT | OBJECT | OBJECT | OBJECT |
| 2 Generality | OBJECT | APPROVE | OBJECT | OBJECT |
| 3 S-slot leaf semantics | OBJECT | APPROVE | OBJECT | OBJECT |
| 4 Gate criteria | OBJECT | OBJECT | OBJECT | OBJECT |

One-line reasons, verbatim:

- codex 1: "The conditional segment theorem is sound, but L-D does not justify the required
  genesis construction, and the shape abstraction loses slot information."
- agy 1: "Lean L-D (`genesis_scale_by_bit`) leaves `carried` (`prev_acc`) unscaled and omits
  `collapse`, F8 (`tree.md`) omits per-slot `collapse` before `accumulate` on disabled slots, and
  Lean `Premises.leaf_sound` assumes unconditional `Carrier s` (`StepInv(pre)`) rather than
  `SVM-002`'s conditional preservation from genesis."
- grok 1: "The statement-level chain is proved in Lean only under a node statement and a genesis
  fold that F8, the Quint model, and `ivc.rs` do not share."
- muse 1: "The statement-level linkage proof is sound, but genesis carry, the field/Nat height
  bridge, and decider opening are assumed in a form stronger than the circuits/F8 specify."
- codex 2: "The uniform abstract relation supports arbitrary finite shapes; the planned chain and
  tree circuits require different keys, and F20 does not exhaust every bracketing."
- agy 2: "One fixed key set covers any program, step count, depth, and shape at the statement
  level (proved universally in Lean F21 and checked on bounded instances in Quint F20), provided
  the C6 leaf emitter pads all programs to the single frozen qualification `Bounds` layout
  (`pilot-smoke.md` D1)."
- grok 2: "F21 quantifies over shapes and S; F20 is a bounded sample; the key that 2.3 will cut
  is the linear statement, and the shape-general node is already over the row kill."
- muse 2: "Fixed-key generality is not shown for option F with S>1, for a node/leaf S mismatch, or
  for a keyable tree; F20 is bounded and F21 makes leaf validity S-free."
- codex 3: "The intended slot behavior is clear, but Lean assumes its main semantic obligation,
  and the one-step `rules_d` requirements conflict."
- agy 3: "Disabled-slot identity lanes, zero `steps` contribution, terminal publication via P0
  (`outcome == Running`), `S=1` as one enabled slot, and `steps` in `1..S` are sound and match
  Quint `leaf`/`leafOk` and Lean `Leaf`/`leafSlot`/`link`, with minor additions needed for Lean
  `L-B` internal slot induction and `SRC-001` wording."
- grok 3: "Design §2.3 and Quint `leafOk` state the slot rules; the Lean `Leaf` that task 4.2 is
  told to match has no slots, and the terminal-last rule has no mutant."
- muse 3: "Quint models slots, Lean does not; terminal/publication and the disabled-slot
  `rules_d` contribution are not defined in either artefact."
- codex 4: "G-A has a dependency cycle; G-A′ requires clarification of unavailable measurements;
  U/F selection contradicts the prescribed S choice."
- agy 4: "Gate G-A (`2.15`) is in a dependency cycle with Task 2.3 (`S_MAX`), Task 2.4, and Task
  2.6 (`A16`); Gate G-A' (`4.10`) and Task 4.1 are ordered before Task 4.2 (leaf circuit code)
  and Task 4.4 (`R` freeze and measured leaf RSS/time); and `SRC-013` contradicts `SRC-014` at
  `k_U = 18`."
- grok 4: "G-A is ordered two ways, the tree keep-clause can override the F22 kill, and an
  F20/F21 pass does not measure the SRC-004/SRC-005 fixes 2.3 exists to make."
- muse 4: "G-A/A16/2.3/`S_MAX`/G-A' are circular, several gates have undefined middle outcomes or
  escape clauses, and F22 has already killed the tree as specified."

## 3. OBJECTs on soundness (item 1)

Thirteen findings carry severity OBJECT on item 1. The `problem` text of each is quoted verbatim
from the raw answers; the proposed change and evidence are in the raw files under the same id.
The lead checked each cited location in the bundle before writing the assessment. The findings
fall into five groups.

### S1. Genesis accumulator: L-D proves the unhardened fold (all four lanes)

- **codex 1.1:** "L-D models accumulation as a binary fold with `zero_add`, and assumes
  `scale false a = zero` as an equality of accumulator values. The implementation retains bases
  when scaling, and its folding challenge hashes all input encodings. Dropping a neutral input
  therefore need not preserve the accumulator or challenge. Furthermore, `genesis_scale_by_bit`
  leaves `carried` untouched: it proves neither realization (a) nor the required realization (b)
  for `prev_acc`."
- **agy 1.1:** "Gap in the **Lean statements (F21)** and **current code (`code/ivc.rs`)** on
  genesis accumulator soundness and completeness (`SRC-005`). At genesis (`prev_h == 0`,
  `not_genesis == 0`), `code/ivc.rs:353, 361` scales `proof_acc` by `0` and collapses it to
  `trivial_acc`, which discards `prev_pi` (`code/ivc.rs:344`) and leaves the witness-supplied
  carried accumulator `prev_acc` (`code/ivc.rs:327-336`) completely unauthenticated, yet
  `code/ivc.rs:239, 377` folds `prev_acc` unscaled into `next` (`qa/ivc/REPORT.md:19, 131-140`,
  `CEX-3` and `CEX-4`). In Lean, theorem `genesis_scale_by_bit` (`lean/Aggregation.lean:1172-1179`)
  proves only `alg.accumulate [alg.scale false proofAcc, carried, leaf] = alg.accumulate [carried,
  leaf]`, which leaves `carried` (`prev_acc`) unscaled at genesis—modeling the unhardened code
  rather than proving that genesis reduces to `alg.accumulate [leaf]` under `SRC-005` realization
  (a) or (b) (`research/builds.md:67-69, 90`). Furthermore, `AccAlgebra`
  (`lean/Aggregation.lean:1153-1163`) omits the law `collapse (scale false a) = zero`, which is why
  `SHKIVC05` (`code/ivc.rs:354-361`) must collapse `proof_acc` *before* `accumulate` so the
  Fiat-Shamir challenge hash sees `trivial_acc`."
- **grok 1.1:** "`genesis_scale_by_bit` scales only `proofAcc` and leaves `carried` in the fold.
  That is the current circuit (`prev_acc` assigned and folded unscaled). It is not SRC-005
  realization (a) or (b), and it is not the F8 fold, which scales both the proof accumulator and
  the carried accumulator by the slot bit. Task 2.3 tells the implementer to match this lemma. A
  key cut from the lemma keeps an unauthenticated carry at genesis."
- **muse 1.1:** "Genesis hardening is modelled for `proofAcc` only. `genesis_scale_by_bit` proves
  the fold equals `accumulate [carried, leaf]`, leaving the witness-supplied carried accumulator
  unscaled and unconstrained. That is today's code, not SRC-005 realization (a) or (b). Quint
  instead idealizes the opposite way: a missing carried slot contributes an empty accumulator
  list. Neither artefact therefore proves "no unauthenticated deferred obligation at genesis" for
  the circuit to be built in 2.3. Gap: Lean statement and Quint model; the code gap itself is
  already tasked in 2.3."

Lead assessment: **valid, unanimous.** `Aggregation.lean:1172-1179` concludes
`accumulate [scale false proofAcc, carried, leaf] = accumulate [carried, leaf]`, and
`AccAlgebra.scale_false` (`:1159`) asserts that a zero-scaled accumulator equals the neutral
element, which the `ivc.rs:354-361` comment says is false for the uncollapsed encoding. The
skeleton (`builds.md:63-69`) already names the missing lemma `genesis_scale_carried`. This is a
gap in the Lean statement and the Quint abstraction; the code gap is SRC-005, already tasked in
2.3.
What it changes before any node key: (1) Lean: replace or supplement L-D with a lemma per SRC-005
realization, (a) the carried accumulator constrained to the neutral encoding, (b) both the
previous-proof and the carried accumulator scaled by the bit and collapsed before the fold, each
concluding that the pairing value equals that of the leaf accumulator alone; replace the
`scale_false` equality by a collapse-after-scale law (agy's `collapse (scale false a) = zero`) so
the premise matches the encoding-sensitive challenge. (2) Quint: an adversarial genesis-carry
action at the accumulator abstraction (muse), or an explicit statement in F20 that genesis
accumulator soundness is outside the model. (3) Task 2.3: name the realization and cite the new
lemma, not `genesis_scale_by_bit`.

### S2. F8 disabled-slot encoding and child-key authentication (agy, muse)

- **agy 1.2:** "Gap in the **F8 constraints** for the three-slot tree node. F8 specifies
  `acc = collapse(accumulate([scale(proof_L, has_L), scale(carried_L, has_L), scale(proof_R,
  has_R), scale(carried_R, has_R), scale(leaf_acc, has_leaf)]))` with only a single outer
  `collapse` after `accumulate`. As demonstrated by `SHKIVC05` (`code/ivc.rs:354-361` and
  `qa/ivc/REPORT.md:13-17` `CEX-0`), `AssignedAccumulator::scale_by_bit` zeroes the scalars of a
  disabled slot's accumulator while keeping the variable-base points parsed from its dummy witness
  proof, and `Accumulator::accumulate` hashes the full encoding (including variable-base points
  and base counts) of every input accumulator to derive the Poseidon folding challenge `r`.
  Passing zero-scaled slot accumulators uncollapsed into `accumulate` makes the in-circuit folding
  challenge `r` depend on dummy witness bases and diverge from off-circuit `next_acc` on every
  disabled tree slot (`has_L = 0`, `has_R = 0`, or `has_leaf = 0`)."
- **muse 1.6:** "F8 is not yet a complete constraint list for a keyable tree node. It does not
  specify: canonical neutral encoding or collapse for each of the three disabled slots (the
  SHKIVC05 failure mode, times three, including the leaf slot); malformed point-encoding and
  identity-point child accumulator handling; or how `L.vk_repr = R.vk_repr = self
  transcript_repr` authenticates children, as opposed to equating three lanes, when the assigned
  VK is constructed from fixed domain/constraint system with only the representation as public
  input. Quint checks a `NodeKey` tag and Lean has no keys, so neither artefact covers child-key
  authentication."

Lead assessment: **valid for F8; no effect on the linear node.** `tree.md:157-159` has one outer
`collapse` only, which is the SHKIVC05 completeness failure repeated per disabled slot (agy calls
it a completeness divergence; with an uncollapsed carried input it is also the SRC-005
unauthenticated-carry case). The tree is closed by F22 (task 2.13: 1,078,147 rows, k=21), so
this does not block 2.3 or 4.2; it blocks any reopening of 5.7.
What it changes in F8: per-slot collapse (or the neutral-encoding constraint) for the proof,
carried and leaf accumulators of each disabled slot, with the off-circuit mirror; negatives for
identity-point and malformed child accumulators; a stated mechanism by which lane 0 of each child
instance binds the child to the self relation. Keep 5.7 blocked until F8 carries these as
testable constraints.

### S3. Shape and manifest binding in the Quint model (codex, muse)

- **codex 1.2:** "The manifest abstraction is a flattened certificate list without slot bits or
  tree structure. By inspection, moving a unary child from L to R, with no local leaf, preserves
  `accOf`, the segment, coverage, and depth. The model therefore cannot distinguish this shape
  change. Its shape mutant merely truncates the list. Also, manifest checking is placed inside
  `canDecide`, although the design assigns manifest replay to `verify_chain`."
- **muse 1.4:** "Shape binding is placed in different components by different artefacts. Quint
  `canDecide` takes a manifest argument and requires `manifest == n.acc`. The design decider takes
  no manifest; shape replay belongs to `verify_chain`, and the manifest is only "hash-bound into
  the bundle". Lean has no manifest, accumulator, or shape in `Decided` at all, yet proves
  statement-level bracket independence. It is therefore unspecified what binds the manifest to
  the statement accepted on chain, and a reader can wrongly credit the decider with I9. The
  `AccAlgebra` premise also abstracts `accumulate` as a plain fold with scale-to-zero, while the
  real fold challenge hashes full encodings; the SHKIVC05 collapse exists precisely because
  scaled-but-uncollapsed bases change that challenge."

Lead assessment: **valid as a model-fidelity defect; not a hole in on-chain soundness.**
`aggregation.qnt:157-162` (`accOf`) concatenates L, leaf, R and self with no slot markers, and
`canDecide` (`:179-182`) checks `manifest == n.acc`. On-chain soundness rests on the pairing of
the root accumulator and the statement chain, not on the manifest; the manifest is a
`verify_chain` (SRC-010) input. I9 as modeled therefore overstates what the decider checks.
What it changes in the model: a structured manifest with slot bits and child references; the
manifest check moved from `canDecide` to a `verifyChain` predicate (or a manifest digest bound
into the decider statement, if on-chain shape binding is wanted); a mutant that moves a unary
child from L to R. In the design (§3.6): one sentence that on-chain soundness does not depend on
the manifest and who checks the manifest hash.

### S4. Node statement and genesis placement differ across artefacts (grok)

- **grok 1.2:** "Three node statements are in use. The linear circuit publishes `[self_vk_repr,
  prog_d, init_d, cur_d, h, acc]` and treats `is_zero(prev_h)` as genesis. F8 and Lean publish a
  segment `(prog, pre, post, h_lo, h_hi)` and check genesis at the decider (`root.lo = 0`,
  `root.pre = init`). Quint checks genesis inside the node only when `TREE = false`, and only at
  the decider when `TREE = true`. Design §3.1 and §3.6 say the chain and the tree share one
  statement and one decider. Task 2.11 records the placement as open. Task 2.3's edit list (range
  checks, `steps <= S_MAX`, genesis carry) does not change the public statement, so a node keyed
  from that list will not be the `link` the theorems talk about."
- **grok 1.3:** "The 2.3 skeleton says the carried slot is absent if and only if `prev_h = 0`,
  and gives the reason as `h_lo < h_hi`. That reason is false on the tree. The parent of the
  genesis segment has `h_lo = 0` and `has_L = 1`. An `is_zero(h_lo)` gate on that parent scales
  away the left child's proof while the statement still claims the child's segment."

Lead assessment: **valid.** `ivc.rs:184-188` publishes the six-lane linear statement; Lean
`Seg` and F8 are segment statements; task 2.11's note leaves genesis placement open. agy 1.4
(concern) reaches the same point and recommends one of grok's two options: keep the linear
statement with in-node genesis and leave decider-side genesis to the F8 node only. grok 1.3 is a
correct warning about the skeleton text (`builds.md:56-58`): the "iff `prev_h == 0`" premise holds
for a prefix chain, not for a tree parent; F8 itself uses explicit bits and is not affected.
What it changes before node keygen: the design records one choice. Either (a) the linear node
keeps `[vk, prog_d, init_d, cur_d, h, acc]` with in-node genesis, and a refinement lemma in Lean
shows that a prefix chain with range-checked heights and `steps >= 1` is `chainOf` with
`hpre = 0` iff the carried slot is off (agy 1.4 states the lemma), and §3.1/§3.6 stop saying the
chain and tree share one statement; or (b) the linear statement becomes the F8 segment with an
explicit slot bit before keygen. Quint `genesisAtNode` and Lean `Decided` follow the choice. The
skeleton's `genesis_link` contract is labelled as valid for the prefix chain only.

### S5. Lean premises stronger than the circuits enforce (agy, muse)

- **agy 1.3:** "Gap in the **Lean statements (F21)**: `Premises.leaf_sound`
  (`lean/Aggregation.lean:204-208`) is stronger than what the mid-run leaf circuit enforces.
  `Premises.leaf_sound` assumes unconditionally that for every accepted leaf `leafValid f`, there
  exist states `s, t` with `Carrier s ∧ Carrier t` and `Run (Step p) f.steps s t`. However, under
  `design/design.md:31, 44`, `design/requirements.md:10` (`SVM-002`), and
  `design/assurance.md:11-12, 19` (`O1`, `O2`), canonical decoding (`decodeCanonical`) is enforced
  only at genesis (`init_d`), whereas mid-run leaves use `decodeStep` and only guarantee that *if*
  the pre-state satisfies `StepInv(pre)` (`Carrier s`), *then* the post-state satisfies
  `StepInv(post)` (`Carrier t`) and follows `Run (Step p) f.steps s t` (`SVM-002`: `"A bad
  pre-state outside StepInv is not by itself a full-run exploit"`)."
- **muse 1.2:** "The height arithmetic proved in Lean is not integrated into the certificate
  checker. `Seg` heights and `NodeCert.stmt` are unbounded naturals with no `< 2^64` condition.
  The isolated field lemmas do not close the bridge: `field_eq_exact` does not assume
  `h2 < 2^64`, so it permits the natural sum to leave the u64 lane domain at exactly the next
  iteration. The sufficient check pair is also unstable across documents: direct checks on
  `prev_h`, `steps`, and `h` in the design, versus `prev_h < 2^64 - S` in L5, which is strictly
  stronger and rejects some sums below `2^64` when `steps < S`. Gap: Lean statements and design
  wording; missing range checks in current code are code-only and already tasked."
- **muse 1.3:** "L6 does not state what the decider circuit must enforce. `Decided.halt` assumes
  existence of a halted carrier state opening `root.post` with result `r`; `AcceptRun` is only
  four equalities. There is no `fin_d` Poseidon opening, no halted-tag/result derivation, no u128
  range or big-endian encoding, no `vk_N_repr` pin, no pairing, and no replay set.
  `decider_pinning` also takes an arbitrary carrier `s0` with `digest s0 = initD`; no premise ties
  `s0` to the canonical initial state of the sealed program `p`, so the theorem proves a run from
  whatever state the caller supplies, not genesis of that program. Gap: Lean statements are weaker
  than the design claim, while their premises are stronger than anything the bundle shows the
  decider checking."

Lead assessment:
- agy 1.3: **valid.** `Aggregation.lean:207-208` gives `Carrier s` for every accepted leaf's
  pre-state, which the leaf circuit does not enforce (SVM-002 is conditional). The theorem stays
  true under the weaker premise by carrying `Carrier` forward from canonical genesis, so the fix
  is a restatement plus a proof change, not a design change. Change: a conditional
  `leaf_sound` (pre-state in `Carrier` implies post-state in `Carrier` and the run) and a
  conditional `Sound`, with `segment_sound` and `decider_pinning` proved from it.
- muse 1.2: **valid as a missing bridge; the circuit plan is right.** `field_eq_exact`
  (`:1144-1147`) concludes `h₂ = h₁ + steps` but not `h₂ < 2^64`; that bound comes from the F8
  range check on `h₂`, which no theorem ties to `NodeCert`. grok 1.5 (concern) says the same and
  adds that `height_no_wrap` is mislabelled "the SRC-004 check pair". Change: one Lean lemma whose
  hypotheses are the SRC-004 triple (`prev_h`, `steps`, `h` each below 2^64), `1 <= steps <= S`,
  the field equation and `2^65 <= p`, concluding the natural-number equation and `h < 2^64`;
  state the direct triple as the circuit contract and `prev_h < 2^64 - S` only as an alternative.
- muse 1.3: **partly valid, as a scope and label defect.** `decider_pinning` quantifies over any
  carrier `s₀` with `digest s₀ = initD`; by `digest_inj` that state is unique, so the theorem is
  sound, but nothing in F21 ties `initD` to the canonical initial state of `p` (that is the
  sealed contract value and authenticated genesis, task 4.9), and the decider's `fin_d` opening,
  key pin, pairing, result encoding and replay set are not in F21. Change: rename or annotate L6
  as the conditional statement it proves, and list those decider obligations as SRC-006/007
  circuit and test obligations outside F21 in design §3.6 and assurance §3.

### Severity disagreements on item 1

- The current-code gaps (no SRC-004 range checks, unscaled `prev_acc` at genesis) are a `note` in
  codex 1.3 and grok 1.6 and part of agy 1.5 (concern); all four lanes agree they are code defects
  already forbidden by SRC-004/005 and tasked in 2.3, and that an F20 pass does not close them.
- The Nat/field height bridge is OBJECT in muse 1.2 and a concern in grok 1.4 and grok 1.5.
- Determinism and terminal halting as unnamed premises of L-C/L7 are a concern in codex 1.4,
  agy 1.5 and grok 1.4; no lane objects on them. This repeats the 2.12 wording note.
- The independent IVC model's failing `hardenedTask23SoundAndCompleteTest`: agy 1.6 and grok 1.7
  (notes) both trace it to the model reducing `prevH` modulo `P` before the range check, so the
  failure does not show that the SRC-004 checks are wrong. codex 1.3 and muse 4.3 treat the
  proposed hardening as not validated. The two readings are compatible: the model needs the
  retarget before it can validate anything.

## 4. Generality (item 2)

Three OBJECT, one APPROVE.

Shared findings:
- F20 is a bounded execution (L <= 9, S in {1, 2, 4, 8}, sampled bracketings, `HMAX = 2^62`,
  sweeps at `--max-steps 30`), not "every L and every bracketing" as design §3.6 and task 2.11
  state (codex 2.2, agy 2.2, grok 2.1, muse 2.2). F21 is general over certificate shape and depth
  under its premises. Every lane asks the design and assurance text to state F20 as bounded and
  point the unbounded claim at F21.
- One key set for every shape does not hold in the plan: 2.3 keys the linear relation, the
  shape-general node is F8 (`SHKTRE01`, a different statement and a re-keyed decider), and F22
  closed the tree (codex 2.1, grok 2.2, muse 2.2).
- `stmt_independent_of_S` and `stmt_mono_S` compare decided roots at two S values under an
  S-free leaf premise; they do not show one key serving several S (grok 2.3, muse 2.2, codex 2.1).
- Option F excludes S > 1 and is not modelled by F20/F21; a node `S_MAX` keyed before the final
  leaf S is not modelled (muse 2.1, OBJECT; agy 2.3, note).
- F21 shows statements for accepted certificates; it has no construction theorem that every valid
  execution has a certificate for every permitted chunking and bracketing (codex 2.2).
- Zero-step runs have no certificate; the generality claim is bounded to runs of at least one
  owner step and total height below 2^64 over `R` and profile `/1` (muse 2.3, note).

Dissent (agy APPROVE): the Google lane holds that the statement-level generality is proved by F21
and checked on bounded instances by F20, so item 2 passes, on one condition: the leaf emitter
pads every program to the single frozen `Bounds` layout (agy 2.1, from pilot defect D1: a C6 step
circuit built for `fact(3)` expected 4255 statement lanes and got 4297). The other three lanes do
not dispute the statement-level result; they object to the key-set and coverage claims the design
text makes on top of it. The dissent is preserved; the condition in agy 2.1 is a concrete
obligation for 4.1-4.4 that no other lane raised.

## 5. S-slot leaf semantics (item 3)

Three OBJECT, one APPROVE. All four lanes agree on the facts:
- Lean `Leaf` is the boundary statement `(prog, pre, post, hpre, steps)`; there are no slots,
  enable bits, disabled identities or terminal-last rule, and `prover.md` §6b L-B (chunk
  composition by induction over slots) is not a theorem in `Aggregation.lean` (codex 3.1, agy
  3.1, grok 3.1, muse 3.1).
- The external correspondence holds: the certified segment is `(prog, pre, post, hpre,
  hpre + steps)` with `1 <= steps <= S`, and `steps = 0` and `steps = S + 1` are refused in both
  artefacts (grok 3.3, agy 3).
- Quint `leafOk` states the structural slot rules (enabled prefix, disabled identity, chaining,
  enabled count) but has no terminal/publish predicate and no mutant that enables a slot after
  publication (grok 3.2, muse 3.2).
- `rules_d`: SRC-001 says `rules_d == rule` "when `steps == 1`", design §2.2(6) says "for S=1";
  in a capacity-S leaf with one enabled slot these differ, and the disabled-slot rule-id encoding
  is undefined (codex 3.2, agy 3.2, muse 3.2).

The lanes differ on severity. codex, grok and muse treat the missing L-B and the undefined
`rules_d` encoding as OBJECT because SRC-014 and task 4.2 say "the slot semantics equal the `Leaf`
of the Lean theorems", which is false as written. agy rates the same gaps as concerns and approves
the semantics themselves. Dissent preserved.

Changes asked before 4.2: Lean L-B (slot list, enabled prefix, lane chaining, disabled identity,
`steps` = enabled count, publication followed only by disabled slots, S = 1 as one slot) whose
conclusion is `leaf_sound`'s run; or SRC-014 and 4.2 amended to say Lean `Leaf` is the boundary
only and Quint `leafOk` is the slot contract (muse 3.1 offers this alternative). One `rules_d`
encoding with a fixed disabled-slot id and the `steps == 1` / `S == 1` wording reconciled. In
Quint: a publish bit and a post-publication mutant; guard assertions (`not(canNode(...))`) in
`mutantRangeCaught` and `mutantPaddingCaught` (agy 3.2). In the design: the in-circuit prefix
constraint `enable_0 = 1`, `enable_{j+1} * (1 - enable_j) = 0` (agy 3.2).

## 6. Gate criteria (item 4)

Four OBJECT. Shared findings:
- **G-A cycle** (codex 4.1, agy 4.1, grok 4.1, muse 4.1): G-A waits on A16; A16 is produced by
  2.6 after keygen 2.4 at the G-A thread count; 2.3 embeds `S_MAX` = "the G-A provisional S";
  `order.md` §4 lists both orders. Each lane proposes a split: a provisional G-A from A15, A17,
  A23, F22 and the 2.3 `cost_model` row check, then A16 as a confirmation that cannot change
  relation constants (or feeds G-A'), with the node bound equal to the final leaf S at keygen.
- **G-A' before the evidence exists** (codex 4.2, agy 4.2, muse 4.2): G-A' requires leaf RSS and
  `t_leaf(S)`, which exist only at 4.4; 4.1 probes a leaf that 4.2 has not built over an `R` that
  4.4 freezes. Proposals: label forecasts and their bounds, freeze `R` first, build the leaf
  parameterized by S, probe `k_leaf(S)` without SRS, then key once and step S down on a measured
  overrun.
- **SRC-013 vs SRC-014 at `k_U = 18`** (codex 4.3, agy 4.3): SRC-013 selects F (S = 1) whenever
  `k_U > 17`; SRC-014, design §2.3 and task 4.10 prefer S = 4 at `k_U = 18`. agy 4.3 also notes
  that SRC-013, SRC-014, SWF-012, D4 and §2.3 still name G-A where the tasks now use G-A'.
- **Tree gate** (grok 4.2, muse 4.2, codex 4.4): design §3.5 row 5 ends with "or `k_U >= 18` /
  k=19 failed / chain alone misses 2 h", which can reopen the tree after the F22 row kill; D13'
  says "`f3 <= 1.4` or `U <= 6`" where `tree.md` §7 requires all criteria. The lanes ask to record
  the tree closed on F22 and require a new row probe to reopen.
- **Formal gates measure less than their labels** (grok 4.3, muse 4.3): F20 excludes SRC-004 range
  gadgets and SRC-005 scaling by its own header (`aggregation.qnt:21-24`); Q-B of `prover.md` §6b
  is not an invariant; the 16 "mutants" inject bad states via `certifyRaw` and do not delete
  `canNode` guard conjuncts; both runners exit 2 when the tool is absent. Proposals: state in
  assurance §3 that an F20 pass leaves SRC-004/005 open; require exit-0 receipts; add true guard
  mutants; land a passing model of the chosen SRC-005 realization before 2.3.
- Other: GPU gate undefined between `m + q` 0.25 and 0.4, and SRC-015(a) and G-B escape clauses
  that turn a failing row into a recorded decision (muse 4.2); A15 inconclusive (bound 1.017 >
  1.01) and how G-A proceeds with the `overhead-unqualified` label (agy 4.4, grok 4.5, notes);
  task 2.3 still lists dropping `binding` as a k=19 candidate, which breaks the SRC-001
  seven-lane freeze (grok 4.4, agy 1.5, muse 1.5); skeleton and reviewer outputs stay inputs, and
  the pilot regrade is required before inference (codex 4.5, note).

## 7. Consolidated changes asked by OBJECTs

Grouped by target. Items marked (S) come from soundness OBJECTs and, by task 2.14, change the F8
constraints or the model before any key exists.

Lean (`clean/fork/Clean/Sherlock/Aggregation.lean`):
1. (S) SRC-005 realization lemma covering both the previous-proof and the carried accumulator,
   with collapse before the fold; replace the `scale_false` equality with a collapse-after-scale
   law (S1).
2. (S) Conditional `leaf_sound` / `Sound` from canonical genesis (S5, agy 1.3).
3. (S) Height bridge lemma on the SRC-004 triple; relabel `height_no_wrap` (S5, muse 1.2).
4. (S) Linear-chain refinement lemma (`hpre = 0` iff carried slot off), if the linear statement
   is kept (S4).
5. (S) L6 scope: annotate what `decider_pinning` proves and list decider obligations outside F21
   (S5, muse 1.3).
6. L-B chunk composition over S slots (item 3).

Quint (`models/sherlock/quint/aggregation.qnt`, `aggregation_test.qnt`):
7. (S) Structured manifest with slot bits; manifest check moved to a `verifyChain` predicate;
   L/R-placement mutant (S3).
8. (S) Adversarial genesis-carry action, or an explicit exclusion in the header (S1).
9. (S) `genesisAtNode` and `canDecide` aligned with the chosen node statement (S4).
10. Publish bit and post-publication mutant; guard-deletion mutants; guard assertions in the two
    injected-state mutants (items 3, 4).

F8 constraints (`tree.md` §1 F8; binding only if 5.7 reopens):
11. (S) Per-slot collapse or neutral encoding for disabled proof, carried and leaf accumulators,
    with the off-circuit mirror; identity-point and malformed accumulator negatives; child-key
    authentication through lane 0 stated (S2).

Design, requirements and tasks:
12. (S) One node statement decision for the linear node before 2.3 keys (S4); remove "chain and
    tree share one statement" if the linear statement stays.
13. (S) Task 2.3 cites the new SRC-005 lemma, not `genesis_scale_by_bit`; `genesis_link` labelled
    prefix-chain only (S1, S4).
14. §3.6/assurance §3: F20 stated as bounded; L-C/L7 premises `det` and `stuck` named; on-chain
    soundness independent of the manifest; F20 pass leaves SRC-004/005 open.
15. SRC-017 generality claim restated per branch (U keyed S, F at S = 1), bounded to runs of at
    least one owner step over `R` and `/1`; one key set per node relation, tree closed by F22.
16. `rules_d` encoding and the `steps == 1` / `S == 1` wording; enable-prefix constraint.
17. Gate order: G-A split into provisional and confirmation; G-A' inputs labelled as forecasts;
    SRC-013/SRC-014 reconciled at `k_U = 18`; G-A/G-A' naming; tree gate closed on F22 with no
    override clause; total GPU criterion; escape clauses keep the row failing until a rerun
    passes; `binding` removed from the k=19 cut candidates.
18. Leaf emitter pads every admitted program to the frozen `Bounds` (agy 2.1, from pilot D1).

Independent IVC model (`qa/gemini-quint/ivc/`):
19. Range check on the unreduced witness, or attack A1 retargeted to a residue that a range check
    can refuse; then re-run `hardenedTask23SoundAndCompleteTest` (agy 1.6, grok 1.7).

## 8. Abstentions

No lane abstains on any item. Two lanes name sub-claims they did not judge:
- muse: the cryptographic soundness of the in-circuit verifier, accumulator and pairing, and
  per-rule fragment soundness, which are declared premises in `AggregationAxioms.lean` and out
  of scope in `aggregation.qnt`; the bundle has no gadget source to judge them from. Next action:
  keep them as SRC-005/006 and SAS O2 test obligations at the keyed boundary.
- grok: the practical cost of grinding `prev_acc` against the Poseidon fold challenge (CEX-3 is
  algebraic only); SRC-005 requires the constraint or a separate argument regardless.

## 9. Open after this review

- The round result is `changes_requested`. Each soundness OBJECT (S1-S5) needs a fix and a
  deterministic check (a Lean build with the new statements, a Quint run with the new mutants, a
  design diff) before 2.3 keys the node; S2 binds only if 5.7 reopens.
- The linear-node statement and genesis placement (S4) need a lead decision; agy 1.4 and grok 1.2
  give the two options.
- The G-A order and the SRC-013/014 conflict need a lead decision before 2.15 and 4.10.
- The dissent of the Google lane on items 2 and 3 stands; it does not lift the OBJECTs of the
  other lanes.
- No Anthropic-family lane reviewed, by family exclusion. No second round runs unless the owner
  asks.
