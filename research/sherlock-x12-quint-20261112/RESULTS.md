# X12 results: Quint model of the GP-2 discharge protocol, and rows per mode

Loop item X-EXP, part X12 (`research/sherlock-e-path-20261112/LOOP.md`). Definition: `research/sherlock-final-design-20261111/ARCHITECTURE.md`, experiments table (X12) and kit "GP-2 hook", items 1 to 7. Owner ruling AD-20 (2026-11-12): GP-2 is built as input consumption now (a producer's Halted value is the consumer's input, checked by the input link at the seal); the mid-run consume rule stays specified for later.

Model: `qa/gemini-quint/x12/claimConsumption.qnt` (the kit names it `models/sherlock/quint/claimConsumption.qnt`; it stays beside its tests and runner until the GP-2 spec adopts it). Tests: `qa/gemini-quint/x12/claimConsumption_test.qnt`. Runner: `qa/gemini-quint/x12/rerun.sh`. Logs: `qa/gemini-quint/x12/runs/`.

## 1. Status

**Run set complete (2026-11-13); every mutant caught, 3 of them by deterministic tests only.** The model typechecks, and every deterministic test passes (the kit positives, the refusals, and one attack test per mutant module). The sampled `soundAll` run of the sound instance finds no violation. All 16 mutants now have a sampled run: 6 in `full2` (20,000 samples), the other 10 and a second `cc_noValueTie` in `full2b` (5,000 samples, `rerun-rest.sh`), and the two that `full2b` did not catch again at 20,000 in `full2c` (`rerun-deep.sh`). Sampling catches 13 of the 16. `cc_subUnrelated`, `cc_linkAnyStatus` and `cc_accumSorted` found no trace in 20,000 samples, and the reach run `reachChainSeal` found none at 10 steps (20,000 samples) or 20 steps (5,000 samples); these four rely on their deterministic tests. The protocol of items 1 to 7 stands, with the four revisions of section 7. No node-mode circuit is built, so nothing here qualifies a circuit.

## 2. What the model is

- Statements are their fields only: SET `(set_root)`, SUB `(set_root, sub_root)`, DL `(chain_d, cover_root, in_link, prem_root)`, SEG `(prog_d, input_d, deps_d, outcome_d, h_lo, h_hi, terminal)` (the opened header words of `post_d` are given as fields; X10 measures the opening).
- Hash: an injective registry `id -> (tag, args)`; ids, raw values and the sentinels `EMPTY_DEPS`, `NO_INPUT_LINK`, `NO_PREMISE` and the absent cover are disjoint ranges. The prover can hash anything (`hashAny`, any tag, any arguments from the known ids and values). An opening of `x` as form `F` succeeds only if `x` was registered with tag `F` (premises A5 and hash injectivity over the SET domains).
- Node relations as guarded actions: SEAL (premise in slot 1 as a SUB, `h_lo == 0`, terminal, `deps_d == EMPTY_DEPS`, premise opens as `ADM(H(SHK_ADM, 1, prog_d))`), JOIN, DESCEND (JOIN side 0 or 1, DEP cover), DL-STEP premise element (from the absent DL only), DL-STEP chain element (`SHK_DEPS` chain, `cover' = R` or `JOIN(cover, R)`), DL-STEP input-link element (at most one, only while `chain_d == EMPTY_DEPS`, `R` joined into the cover), SEAL-WITH-DEPS (premise check on `DL.prem_root`, cover present, `DL.chain_d == deps_d`, and for a link: `H(IN, val_d) == input_d`, the link re-hashed as a run claim `(prog', input', outcome')`, `outcome' == H(O, Halted, 0, val_d)`). Guards read statement fields and openings only.
- Ground truth: a fixed table of 8 genuine runs and 10 segment statements (one terminal genesis segment per run, a non-terminal genesis segment of A, and a terminal tail segment of A with `h_lo = 2` that ends Halted 99 from a state that is not A's genesis). Runs: A (producer, Halted 7), B (input 7: honest input link from A), R (Rejected with value 8), C (input 8: R's value, no Halted producer), E (input 3: no producer), D (deps `[A, A]`), F (deps `[A, R]`, input 2 = D's value), G (input 7: second consumer of A). The deps lists exercise the chain as a node relation, as AD-20 states; under AD-20 every real run has `deps_d = EMPTY_DEPS`.

## 3. Invariants (kit names) and what each states

| Invariant | Statement (over every reachable state) |
|---|---|
| `SegmentNeverConsumed` | every `LEAF` or `DEP` node under a verified root carries the claim of a genuine run from Init to a terminal state, with that run's `n_steps`; `FOREIGN` heads are the registered foreign claim; `ADM` heads are admissions (half of GP2-L1) |
| `SubRootSound` | every `SUB(R, x)` has `R` verified and `x` a sub-term of `R`'s grammar term (GP2-L2) |
| `DepsDischargedIffVerifiedSet` | every DL element (chain or link) is a run or foreign head claim of a verified SET, never a premise, and its root is inside the DL's cover; in every verified `DEP` the run's dependencies and its input link are members of `cover_root` (the other half of GP2-L1) |
| `ChainOrderAndRepetitionExact` | the discharged head claims of every SEAL-WITH-DEPS equal the consumer's consumed claims, with order and repetition (GP2-L3) |
| `EmptyDepsOnlyBySeal` | a `LEAF` is a run with no dependency; a `DEP` has a present cover and a dependency or an input link |
| `InputLinkHaltedOnly` | **AD-20 soundness**: every verified `DEP` with `in_link` set names a genuine run that Halted with exactly the consumer's input value |
| `soundAll` | the conjunction, checked in one `quint run` |

Soundness in words: no consumer is accepted with an input link unless a genuine producer Halted with exactly that consumer's input value, and that producer is a member of the consumer's cover. A run can always seal as a `LEAF` with no link; that is not a claim about its input.

Completeness is reachability, so it is checked by deterministic `run` tests (the kit's positives) and by negated `reach*` invariants that must be violated in the sound instance (`reachBConsumesA`, `reachJoin`, `reachDescend`, `reachChainSeal`).

## 4. Lead runs (MEASURED, quint 0.32.0, `runs/full2/summary.txt`, `runs/full2b/summary.txt`, `runs/full2c/summary.txt`, `runs/witness/`)

Every sampled run uses `--max-steps 10 --seed 0x20261112` unless the row says otherwise. `full2` ran through `heavy-run`; `full2b` and `full2c` (2026-11-13) ran through `heavy-slot` (`q-slot.sh`).

| Command | Result |
|---|---|
| `quint typecheck` of the model and the tests | exit 0, both |
| `quint test` of `claimConsumption_test` (kit positives and refusals) | 20 passing; the witness run with the `pos_gp2_witness_*` tests gives 22 passing (`runs/witness/claimConsumption_test.log`) |
| `quint test` of the 16 mutant test modules (`t_*`) and `t_subUnrelated_sound` | every module passes: under each mutant its attack breaks a named invariant; `t_subUnrelated_sound` (7 passing) shows the same attacks refused by the sound instance |
| `quint run --main cc_sound --invariant soundAll` (20,000 samples, 10 steps) | `[ok]`, no violation |
| reach witnesses `reachBConsumesA`, `reachJoin`, `reachDescend` | `[violation]` as expected (witness found) |
| reach witness `reachChainSeal` | `[ok]`: no witness in 20,000 samples at 10 steps (`full2`) or in 5,000 samples at 20 steps (`full2b`); the deterministic `pos_gp2_witness_chain_seal` test reaches it |
| sampled mutants, 20,000 samples (`full2`): `cc_noTerminal`, `cc_ignoreTag`, `cc_headOfJoin`, `cc_noValueTie` | `[violation]` (caught); `cc_noValueTie` from `runs/full2/31.log`, its summary line was not written before the cut |
| sampled mutants, 20,000 samples (`full2`): `cc_subUnrelated`, `cc_linkAnyStatus` | `[ok]`: not caught by sampling; caught by their deterministic tests |
| sampled mutants, 5,000 samples (`full2b`): `cc_noValueTie` (again), `cc_nstepsForged`, `cc_plainSealNonEmpty`, `cc_noChainCheck`, `cc_premiseInChain`, `cc_accumSet`, `cc_coverSkipsLater`, `cc_linkNoCover`, `cc_sealDepsEmpty` | `[violation]` (caught) |
| sampled mutants, 5,000 samples (`full2b`): `cc_valueMismatch`, `cc_accumSorted` | `[ok]`: not caught at 5,000 samples |
| the same two at 20,000 samples (`full2c`) | `cc_valueMismatch` `[violation]` (caught); `cc_accumSorted` `[ok]`: not caught by sampling, caught by its deterministic test |

## 5. Verdict

The sound instance holds every invariant in the deterministic tests and in the sampled run. Every mutant is caught by a deterministic attack test. Sampling alone catches 13 of the 16 mutants; `cc_subUnrelated`, `cc_linkAnyStatus` and `cc_accumSorted` are caught only by their deterministic tests, and the `reachChainSeal` witness is reached only by its deterministic test. The planned run set is complete, so X12 passes on the deterministic tests with this sampling record; it is not a clean sampled pass.

## 6. Rows per mode (ESTIMATE)

The rows are counted from the model's hash calls per mode, sized with the `sherlock-value/2` digest table of ARCHITECTURE section 1.11 (FP-4): SET forms and the run digests are `P2` (Poseidon2 over BabyBear, width 16, rate 8, tag word first, a digest is 8 words, `u16` 1 word, `u64` 4 words), so a `P2` of `w` words with its tag takes `ceil(w / 8)` permutations. The node is an OpenVM BabyBear STARK (OD-24 option E), where one Poseidon2 permutation is one row of the Poseidon2 AIR; the node has no fixed row gate and its cost is seconds per proof (ARCHITECTURE 1.6 and the kit NODE INTEGRATION, MEASURED: arity 2 node over 2^20 children 1.809 s on the GPU).

The kit's row line cites "about 22 rows per two-input Poseidon level" (A:739, `research/sherlock-processor-analogy-20261110/experiments-2.md:157-164`). Checked: that file measures 21 rows in the leaf and 22 in the stdlib relation for a two-input Midnight Poseidon over Fr in the halo2 prototype (section 3.1). It is a halo2 figure for the Fr prototype, not a figure for the `P2` hashes of the final node. It is not used below.

| Hash (FP-4 form) | Words with tag | `P2` permutations |
|---|---:|---:|
| `LEAF(claim_d, n_steps)` | 13 | 2 |
| `DEP(claim_d, n_steps, cover_root, in_link)` | 29 | 4 |
| `JOIN(l, r)` | 17 | 3 |
| `FOREIGN(fclaim_d)`, `ADM(adm_d)` (SET forms) | 9 | 2 |
| `deps_d` step `H(SHK_DEPS, deps_d, claim_d)` | 17 | 3 |
| `claim_d` (`SHK_RUN`) | 26 | 4 |
| `input_d` (`SHK_IN`) | 29 | 4 |
| `outcome_d` Halted wrapper | 11 | 2 |
| `adm_d` (`SHK_ADM`) | 10 | 2 |
| state header opening (FP-5, 42 words) | 42 | 6 |

| Mode | Hash calls in the model | `P2` permutations |
|---|---|---:|
| DESCEND | 1 opening (`JOIN` or `DEP`) | 3 (`JOIN`) or 4 (`DEP`) |
| DL-STEP, premise element | 0 (the premise is checked at the seal) | 0 |
| DL-STEP, input-link element | 1 head opening, at most 1 `JOIN` for the cover | 2 to 7 |
| DL-STEP, chain element | 1 head opening, 1 `SHK_DEPS` step, at most 1 `JOIN` | 5 to 10 |
| JOIN | 1 | 3 |
| SEAL | header opening, `claim_d`, `LEAF`, premise opening `ADM`, `adm_d` | 16 |
| SEAL-WITH-DEPS, no link | header opening, `claim_d`, `DEP`, premise opening, `adm_d` | 18 |
| SEAL-WITH-DEPS, input link | the above plus `input_d` re-hash, link claim re-hash, Halted wrapper | 28 |

Head opening: 2 permutations for a `LEAF` or `FOREIGN` head, 4 for a `DEP` head; the first element of a list joins no cover. Under input admission form (b) of AD-17, the premise opens as `JOIN(ADM, VIEW)`: add 3 (`JOIN`) and 2 to 3 (`VIEW`, `vw_d`) permutations to each seal. Every mode also pays the node's fixed costs, which X12 does not change: `shape_d` (GP-4 hook) and the key route; X10 measures the header opening.

Node proofs per discharge: a claim at depth `d` under a verified root costs `d` DESCEND nodes plus 1 DL-STEP node; a consumer with `m` chain elements and one link costs `m + 2` DL-STEP nodes (premise, link, chain) plus its SEAL-WITH-DEPS. Under AD-20 (input consumption only, `deps_d = EMPTY_DEPS`), a consumer whose producer is its own sealed proof (`d = 0`) costs 2 DL-STEP nodes (premise, link) and one SEAL-WITH-DEPS in place of SEAL: 3 node proofs in place of 1. At the measured 1.809 s per arity 2 node on the GPU (ARCHITECTURE 1.6, MEASURED), that is about 3.6 s more per consumer (ESTIMATE). The `P2` permutations above are a few rows of the node's Poseidon2 AIR, so the node's height and its seconds per proof do not move with the mode; the cost of a mode is its node count.

## 7. Spec findings for the GP-2 hook (X12 decides "stands or is revised")

The protocol of items 1 to 7 stands: no invariant fails in the sound instance, and every named attack that the model covers is refused. The model needed four choices that the kit text does not make. Each is a small revision for the GP-2 spec:

1. **No constant for an absent cover.** DL-STEP says "`cov` absent: `cov' = R`", and the premise element leaves the cover unchanged, so a DL after its premise element has no cover. The constants table of FP-4 has `EMPTY_DEPS`, `NO_INPUT_LINK` and `NO_PREMISE` but no absent-cover word. The model uses a sentinel outside the id range. Proposed: `NO_COVER = P2(SHK_SET_JOIN)` (preimage length 1, so it equals no `JOIN` root under A5), and SEAL-WITH-DEPS requires `cover_root != NO_COVER`.
2. **SEAL-WITH-DEPS must refuse a premise-only DL.** Item 6 says a run with empty dependencies and no link seals with SEAL to a `LEAF`, but the SEAL-WITH-DEPS checks do not refuse a DL that holds only the premise. The probe `cc_sealDepsEmpty` shows that such a DL seals the run as `DEP(claim_d, n, NO_COVER, NO_INPUT_LINK)`, a second SET form for the same claim (`EmptyDepsOnlyBySeal` violated). Finding 1's check closes it. The sound model has it: `neg_premise_only_dl_refused` passes.
3. **The input-link element joins `R` into the cover.** Item 4 says the input-link element is "the same with `ch` unchanged and `link = c`". The model reads "the same" as including `cov' = R` or `JOIN(cov, R)`, which is what GP2-L1 needs ("its input link [is a member] of its `cover_root`"). The probe `cc_linkNoCover` (the other reading) breaks `DepsDischargedIffVerifiedSet`. Under that reading the link is still a genuine Halted run, because the SUB came from a verified SET, so `InputLinkHaltedOnly` would still hold. But the consumer's `DEP` would not open its producer, and a collection that holds the consumer would not commit to the producer. Proposed: state `cov'` explicitly for the input-link element.
4. **A `FOREIGN` head as an input link always fails at the seal.** Item 4 lets the input-link element take a `LEAF`, `DEP` or `FOREIGN` head. The seal re-hashes the link as a run claim `(prog_d', input_d', outcome_d')` under `SHK_RUN`, and a foreign claim is a `SHK_CLAIM` hash, so the seal refuses it (`neg_foreign_as_input_link_refused_at_seal`). This is sound but dead. Proposed: the input-link element takes `LEAF` or `DEP` heads only. A Halted foreign value as an input would need its own wrapper, which is out of scope for AD-20.

Confirmed as written (each by a refusal test in the sound instance and a mutant that breaks an invariant):

- The premise element can be forced first, from the DL fields alone: the step takes only the absent DL.
- "Link before any chain element" can be forced by `chain_d == EMPTY_DEPS`, because `EMPTY_DEPS = P2(SHK_DEPS)` has preimage length 1 and equals no chain hash (A5). `neg_link_after_chain_refused` and `neg_second_link_refused` pass.
- The chain is order- and repetition-exact only with the `SHK_DEPS` hash chain. A commutative accumulator (`cc_accumSorted`) or a set accumulator (`cc_accumSet`) breaks `ChainOrderAndRepetitionExact`.
- `n_steps` must be the sealed segment's `h_hi`. A free `n_steps` (`cc_nstepsForged`) puts a wrong height under a genuine claim.
- The input-link check needs all three parts: the `input_d` re-hash (`cc_noValueTie`), the comparison of the Halted wrapper with the consumer's `val_d` (`cc_valueMismatch`), and the Halted kind (`cc_linkAnyStatus`). Each mutant lets a consumer take a value that no producer Halted with (runs C and E).

## 8. Not modelled, and why

- Mutants `claim_wrong_set_root`, `claim_swap_outcome`, `claim_address_in_identity` and `claim_steps_in_identity` from the kit list. `claim_wrong_set_root` is modelled as `cc_coverSkipsLater` (a later element's root not joined into the cover). `claim_swap_outcome` (the consumer's own outcome in place of the producer's) is the same check as `cc_valueMismatch`. The two identity mutants change which digest a claim is, so they are completeness and replay properties under re-segmentation. The model has one segmentation per run, so they are out of scope, and the ledger kit covers them (LDG).
- Positive `pos_gp2_struct_value_across_programs`: value digests are opaque integers here. The structured value is VALUE DIGEST's obligation 4.
- `VIEW` premises (input admission form (b) of AD-17): the model uses the `ADM` premise only. `VIEW` has the same role, a premise-only head (`claim_premise_in_chain` covers both tags in `headTagOk`).
- The salt in `input_d`, the fuel limbs and the stack limit: INIT range-checks them at genesis, and they do not change the link argument.
- Hash collisions: injectivity is a premise (A5 and the SET-domain separation of GP2-L2). The model makes it true by construction. `cc_ignoreTag` shows what the domain word buys.
- `quint verify`: not run (the skill rule; sampled `quint run` plus the deterministic attack tests).

