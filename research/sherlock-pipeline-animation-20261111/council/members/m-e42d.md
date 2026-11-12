# Council verdict m-e42d

Verdict: changes_requested
Bundle: head_sha 680d2b5afec18a4b8686d20d28cdb7a551feee70
storyboard_sha256 41ee3a69f8be4d34ca12aa20a491567128c168b85922ce51b6eb245c936375d4
architecture_sha256 9e10bc3ca429e55f4a5b5b8688c81a086a0ad52c3dad206f8f3870844caeb054

Abbreviations: `ARCH` = `research/sherlock-final-design-20261111/ARCHITECTURE.md`; `SB` = `research/sherlock-pipeline-animation-20261111/council/bundle/STORYBOARD.md`; `ML` = `openspec/changes/sherlock-modular-leaf-20261111/`; `O/` = `openspec/changes/sherlock-vm-consensus-20261107/`.

## Findings

### F1 The run decider has its own seal path, and that path does not check `deps_d`
- Category: soundness (latent) and fragility
- Severity: major
- Step(s): 4, 5
- Evidence:
  - NODE SEAL has the dependency check. ARCH:1744: "the opened `deps_d` equals the constant `EMPTY_DEPS` (a run with consumed claims seals only through SEAL-WITH-DEPS)". `ML/specs/sherlock-node-integration/spec.md:26`: "SEAL SHALL require `h_lo == 0`, a terminal opened `outcome_tag` and `deps_d == EMPTY_DEPS`".
  - The M4 decider seals again, from a SEG root. ARCH:1842: "M4 path: the root SEG of the linear node, opened in the decider as `run_decider.rs` does today ... the terminal state opening (M4 path only): `open_state_header` of BOUNDARY COMMITMENT on the root SEG's `post_d`". ARCH:1843 lists its checks: `n_steps` range, outcome tag not Running, `claim_d`. There is no `deps_d` check. The mutant list at ARCH:1863 has no dependency mutant. `ML/specs/sherlock-decider/spec.md:4,11` (DCD-001) also has no `deps_d` clause.
  - The decider also assumes that the node already sealed. ARCH:1851: "Assumed: NODE INTEGRATION authenticates payload and carry and SEAL refuses Running (GP path); BOUNDARY COMMITMENT authenticates the terminal opening (M4 path)". On the M4 path no NODE SEAL runs.
  - The storyboard puts the dependency rule in the decider scene. SB:2068 (Beat 5.10): "Ordinary SEAL requires `deps_d` to equal the empty dependency commitment. A nonempty list therefore requires dependency verification before the run seals."
  - The hole is latent today. ARCH:1989: "Until then `deps_d` is constant in every run". The design plans to close OD-20(b) with no new node relation (ARCH:2696: "(b) needs no new node relation"), so nothing in the plan makes the decider change when OD-20(b) lands.
  - `deps_d` is not in the list of words that no rule writes. ARCH:1563: "`input_d`, `prog_d` and `prog_root` are in no rule's write set."
- Problem: The claim-forming checks (genesis, terminal, empty dependencies, `outcome_d`, `claim_d`) exist in two places: `seal()` in NODE (ARCH:1818) and `open_terminal`/`decide_run` in DECIDER (ARCH:1877-1879). The two lists already differ. The attack: when a consume rule is added (OD-20(b)), a run consumes a claim `c'` that was never verified. Its `deps_d` becomes `H(SHK_DEPS, EMPTY_DEPS, c')`. The prover takes the root SEG straight to the M4 run decider, which opens `post_d`, sees a terminal tag, and publishes `[prog_d, input_d, outcome_d, n_steps]`. `acceptRun` then accepts a claim whose consumed dependency was never discharged, which breaks the GP-002 rule "that claim is checked in-circuit, not trusted" (`O/general-purpose.md:24`). The same risk applies to any future SEAL predicate that is added to `seal()` and not copied into the decider.
- Proposed revision: Give the design one seal. In kit DECIDER (ARCH:1842), replace the M4-path terminal opening with: "The run decider accepts only a SET root whose head is the `LEAF` or `DEP` form. It witnesses `(prog_d, input_d, outcome_d, n_steps)`, recomputes `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)`, opens the root as `H(SHK_SET_LEAF, claim_d, n_steps)` or `H(SHK_SET_DEP, claim_d, n_steps, cover_root, in_link)`, range-checks `n_steps`, and publishes the four values. Every seal predicate lives only in NODE `seal()` and `seal_with_deps()`." On the M4 linear path, add a SEAL step: either a final node in SEAL mode, or the same `seal()` gadget called inside the decider circuit, not a re-implementation. If the M4 path must keep the direct SEG opening, the minimum fix is three changes. First, add "opened `deps_d == EMPTY_DEPS`" to DCD-001 and to ARCH:1843. Second, add the mutant `decider_nonempty_deps`. Third, add `deps_d` to the no-write-set list at ARCH:1563 until OD-20(b) is decided.
- Cost of the revision: In the decider, the six-input header opening (3 permutations, ARCH:1582) gives way to a claim hash plus a SET-leaf hash (about 2 permutations more). That is a few hundred rows, ESTIMATE, against the 1,000,000 gate. The M4 path gains one SEAL node proof, about 65 s (SB:1653, Beat 4.10). Kits touched: DECIDER, NODE INTEGRATION (shared `seal()`), and the decider spec DCD-001. The minimum fix costs one equality and one mutant.

### F2 `reg_root` is not tied across child slots, and the run entry does not bind it
- Category: soundness
- Severity: major
- Step(s): 4, 5, 7, 8
- Evidence:
  - ARCH:1736: "`reg_root`: full field; forwarded unchanged; consumed by the GP-3 hook." Compare ARCH:1735 for the key representations: "`keys_repr = [...]`: full field; equal in every child and the parent". No such equality is stated for `reg_root`.
  - ARCH:2048: "`reg_root`: full field, forwarded by NODE INTEGRATION unchanged and read into the decider transcript from contract state."
  - The FOREIGN SET form does not contain the registry. ARCH:1970: "`FOREIGN(fclaim_d) = H(SHK_SET_FOREIGN, fclaim_d)`".
  - A LEAF child has no `reg_root` lane (ARCH:1710, seven lanes).
  - The node spec has no `reg_root` requirement or mutant (`ML/specs/sherlock-node-integration/spec.md:1-70`).
  - The registry lifecycle recommendation expects older roots to stay in use. ARCH:2741: "history with revocation, so long runs survive key admissions."
  - The run entry has no registry. ARCH:1843: "Provides, run entry: exactly `[prog_d, input_d, outcome_d, n_steps]`". Yet a DEP cover can hold foreign members. ARCH:1972 says head claims are "`claim_d` or `fclaim_d`", and DL-STEP accepts any head (ARCH:1981).
  - SB:2940 (Beat 8.6): "Verification binds these values and requires the registry root to match contract state. A stale registry fails."
- Problem: "Forwarded unchanged" is defined for one child, not for two. The attack: a foreign lift proves membership of a key under an old root `R_old`, one that was later revoked or never current. JOIN combines that FOREIGN SET with an honest SET whose node statement carries `R_cur`. If the parent takes `reg_root` from slot 1 and does not require slot 0 to match, the collection publishes `[R_cur, set_root, shape_d]`. The contract's check against contract state passes, and the foreign claim was never checked under `R_cur`. A subtree built only from LEAF children has an unconstrained `reg_root` witness, and the root value depends on an equality that no row of the node relation table (ARCH:1750-1759) states. Second path, latent until OD-20(b): a run that consumes a foreign claim settles through the run entry, which binds no `reg_root` at all, so no registry is ever compared with contract state.
- Proposed revision:
  1. NODE INTEGRATION, at ARCH:1736 and in NODE-004: "`reg_root` SHALL be equal in every enabled child whose statement carries it and in the parent. A node whose children carry none SHALL take it from its own witness, and the root value is the one the decider compares." Add the mutant `node_reg_root_mixed`, a JOIN of two SETs under different `reg_root` values, which must be unsatisfiable.
  2. Make the binding local as well. Use `FOREIGN(reg_root, fclaim_d) = H(SHK_SET_FOREIGN, reg_root, fclaim_d)`, so `set_root` itself names the registry each foreign member was admitted under. This also gives N-8's root history a precise check: the contract tests membership of that root in the history.
  3. DECIDER run entry: either refuse a root whose cover contains a FOREIGN member, or add `reg_root` to the run-entry transcript and check it against contract state as the collection entry does.
- Cost of the revision: One field equality per enabled slot (a handful of rows), one extra word in the FOREIGN hash, and possibly one extra run-entry public value. Kits touched: NODE INTEGRATION, GP-3 hook, DECIDER, and the contract (see F3).

### F3 No kit owns the contract state that steps 7 and 8 depend on
- Category: gap
- Severity: major
- Step(s): 6, 7, 8
- Evidence:
  - The decider kit makes a promise that only a contract can keep. ARCH:1867: "DEC-L2. `claim_d` settles at most once in the contract set even when it has many paths in `set_root`."
  - ARCH:1850: "collection settlement has no per-claim side effect keyed by `set_root` or `shape_d`". `ML/specs/sherlock-decider/spec.md:29,37`: "Settlement SHALL have no per-claim side effect keyed by `set_root` or `shape_d`" and "it settles at most once".
  - ARCH:2754 (N-11) leaves "a claims contract with root acceptance and claim opening" as an option name only. `ML/tasks.md:40,43` (F.7, F.10) are gated tasks. No section-2 kit has a contract interface.
  - The storyboard shows these effects. SB:2378 (Beat 7.3): "The contract looks up the recomputed `claim_d` in `runs` ... An existing key refuses the call." SB:2961 (Beat 8.7): "a second path grants no second settlement."
  - `O/general-purpose.md:27`: "(a) ... accepted once on-chain; (b) one proof that aggregates at least three runs ..., accepted once on-chain, from which each claim can be opened."
- Problem: Whichever way N-11 goes, the interface is undefined. Settling a collection publishes only `set_root`, so the contract cannot know which `claim_d` values it holds. That means one of two things. Either collection members never enter `runs`, and then acceptance of one claim through `acceptRun` and again inside a collection is two acceptances, so "accepted once on-chain" has no meaning across the two entries. Or members do enter `runs`, and then something must open them on chain, at a cost that grows with path depth, and must refuse a member that is already in `runs`. Neither DECIDER nor any hook can enforce the at-most-once rule, because it is contract state. DEC-L2 is therefore assigned to a kit that cannot discharge it. Nothing says what is stored for a `DEP`-form claim (`n_steps` is inside the SET leaf, ARCH:1971), or for a FOREIGN member.
- Proposed revision: Add a kit LEDGER CONTRACT (Compact, outside the recursion crates) with this interface. Move DEC-L2 and `DiamondAncestorOpenedAtMostOnce` to it.
  - One map `runs: claim_d -> (outcome_d, n_steps)`, shared by every entry.
  - `acceptRun(prog_d, input_d, outcome_d, n_steps)` keeps SRC-007 as it is.
  - `acceptCollection(reg_root, set_root, shape_d)` verifies the collection decider proof, checks `reg_root` against the registry state (history per N-8), and inserts `set_root` into `collections`. It is idempotent and makes no per-claim write, which keeps the cost of a collection settlement independent of depth and claim count (GP-004).
  - `openClaim(set_root, path, form, preimage)` requires `set_root` in `collections`, recomputes the hash path with the four domain words of ARCH:1966-1970, recomputes `claim_d` from `(prog_d, input_d, outcome_d)` for a LEAF or DEP head, and inserts into `runs` with the same freshness refusal as `acceptRun`. A FOREIGN head goes into a separate `foreign` map keyed by `fclaim_d`.
  - Invariant: a `claim_d` is in `runs` at most once, whichever entry inserted it.
- Cost of the revision: One new kit and spec, with no circuit rows. On chain, one Poseidon per path level per opened claim. Openings are optional per claim and are paid by whoever needs that claim settled. This turns the open N-11 into a concrete interface for the owner to approve; it does not decide SRC-007's sealed `program_d` question.

### F4 `rules_d` has no consumer after the leaf
- Category: cost
- Severity: minor
- Step(s): 3, 4
- Evidence:
  - ARCH:589: "`rules_d = Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1))` over exactly S slot lanes ... `S + 1` inputs".
  - The SEG statement drops it. ARCH:1748: "A LEAF child counts as a SEG `(prog_d, pre_d, post_d, h_pre, h_pre + steps)`". The leaf shape uses only `h_pre` and `steps` (ARCH:2106). The decider publishes four values (ARCH:1843).
  - SB:1310 (Beat 3.31): "With these bindings a prover cannot claim another rule sequence".
- Problem: The slot rules are already fixed by DEC's one-hot selection and by CORE's transition constraints inside the leaf (SRC-003, `O/requirements.md:28`). Lane 5 is verified by the node and then discarded, so it adds no guarantee to any statement after the leaf. Its cost is a 17-input sponge per leaf at S = 16 (about 9 permutations, ESTIMATE), plus one more interface that CORE, NODE and the spec must keep in step.
- Proposed revision: Either name a consumer (for example, put `rules_d` into the LEAF base case of `shape_d`, if N-7 wants executed rules in the shape), or amend SRC-001 so that lane 5 becomes a reserved zero lane. This is an owner-level spec edit, because SRC-001 fixes the lane.
- Cost of the revision: Removing it saves about 9 permutations per leaf. Keeping it with a consumer costs one more word in `leaf_shape`.

### F5 The MERGE-WORK fallback allows a second implementation of the machine relation
- Category: fragility
- Severity: minor
- Step(s): 3, 4
- Evidence: ARCH:2110: "If the leaf's layout cannot be embedded in the node's configuration, a node emitter of the same rule specification is run against the same differential vectors". ARCH:2102: "The inline kernel constrains the same reference steps as the leaf".
- Problem: A separate node emitter would be a second encoding of the 120 rules, plus INIT and the memory, stack and value-digest checks, inside the recursion circuit. Differential vectors test agreement on the vectors only. A divergence is a soundness hole in whichever circuit is weaker, and NODE-L1 and GP4-L2 (ARCH:1797, 2127) assume one CORE contract.
- Proposed revision: In ARCH:2110, require the inline kernel to be built from the same kit `configure`/`assign` functions as the leaf (CORE, DEC, ALU, RANGE, HASH, STACKS, MEM, backend, VALUE DIGEST, INIT), instantiated in the node's constraint system. Remove the separate-emitter option. If that does not fit the gate, the result goes back to the owner under N-6 rather than to a second implementation.
- Cost of the revision: The kits must be generic over the host constraint system, which they largely are already, since they talk through ports and the bus (ARCH:70). No new rows.

## What the design gets right
- Step 3, BOUNDARY COMMITMENT: the header-and-body state digest (ARCH:1567-1572) lets NODE and DECIDER read the outcome, `input_d` and `deps_d` with one six-input sponge while keeping the seven landed lanes.
- Step 4, NODE INTEGRATION: the complete node relation table, with every other combination unsatisfiable (ARCH:1748-1761), plus height range checks and `steps >= 1` (ARCH:1738), closes gap, overlap and wrap attacks across MERGE and MERGE-WORK.
- Step 5 and 7, DECIDER and SRC-007: `claim_d` excludes `n_steps` and every physical field (ARCH:1711, 1843). Replay refusal across partitions (SB:2441) therefore follows from SRC-023 partition independence, not from proof shape.
- Step 7, GP-2 hook: DESCEND plus head opening (ARCH:1975-1979) discharges a claim from any batching shape of the producer in bounded work per node, and `deps_d` chain equality keeps order and repetition exact.
- Step 4, CONTINUATION: mode 1 adds no circuit cells and reuses MERGE (ARCH:1901), and soundness never depends on the checkpoint store (ARCH:1911).
