# Decisions taken overnight 2026-11-11 (owner AFK)

The owner is asleep and asked for finished, validated plans. Where a choice was needed, the lead took the recommended option and recorded it here. Each entry names its source. The owner can reverse any of them; entries marked OWNER are also listed in the morning report as questions.

## From the run-length study and its audit

Sources: `research/sherlock-runlength-consensus-20261111/CONSENSUS.md` (5 lanes, Gemini 4 Carbon High) and `AUDIT.md` (Fable 5.2: adopt with changes; 3 blocker, 10 major, 7 minor).

- R-1. **2^64 is the SRC-004 counter range, not a circuit size.** Adopted (CONSENSUS Q1, confirmed by AUDIT).
- R-2. **GP-1 and GP-001 text: use the audit's corrected text (AUDIT.md section 5), not the CONSENSUS text.** The CONSENSUS text narrows GP-001 ("every admitted program ... every valid input", from Lane E only) and adds bounds the current text does not have (AUDIT blockers 1 to 3). The audit text keeps every current sentence and only appends. Applied in night-plan step D3.
- R-3. **Memory: dense tail-buffered memory with hash-chain FRAMES and TASKS stacks.** Adopted (5/5 lanes on the core mechanism; AUDIT found no attack on the flush rule, address reuse, tail forgery, the 2^64 - 1 edge or domain tags). The invariants of AUDIT finding 8 (canonical `tail_d`, tail open and pass-through rules, kind-tag checks on reads, tagged hashes) become MEM module contract items and must be in the architecture and the specs.
- R-4. **Tree depth: the address space must not bind before the step counter.** With at most 3 appends per step (unverified, AUDIT), TOP = 61 (2^64 cells) can run out before 2^64 steps (AUDIT blocker 3). Recommended: size the tree so cells >= appends-per-step x 2^64 (TOP >= 63 with 8-cell segments), and measure the extra depth cost in experiment C1. OWNER: confirm TOP >= 63. (Superseded in part by M-2 and A-6: a memory bound below the step bound narrows GP-1 and is not an acceptance path of the implementation change.)
- R-5. **Claim counter: no `n_claims < 2^64` bound in GP-001.** With occurrence counts summed through joins, a shared DAG reaches 2^64 at depth 64 with about 128 proofs, which would limit GP-4's DAGs of any depth (AUDIT blocker 2). Recommended: count distinct claims, or carry no count in the public statement; the architecture merge (B1) decides the mechanism. OWNER: confirm.
- R-6. **Arenas: open.** One arena (3 lanes) vs four arenas (2 lanes). The one-arena variant depends on a 108-rule non-interleaving claim that is a code reading, not a checked invariant. Recommended: the MEM interface hides the arena count; experiment C1 measures both and checks the non-interleaving claim mechanically; decide in C4.
- R-7. **Continuations: tier 1 (segment tree) plus the retained-memory checkpoint mode only.** The address-free checkpoint mode is not defined well enough (AUDIT major). Continuations compose under GP-4 and the checkpoint domain tag, not "via GP-002" (GP-002 forbids that use).
- R-8. **Slot-0 zeroing does not replace write-before-read.** The architecture keeps a write-before-read obligation for scratch (AUDIT major).
- R-9. **D-1 (linked slices) stays open until measured** (`dissent.md:202`, task 4.18). It is not closed by the study.
- R-10. **Row and throughput figures in the study are ESTIMATE and not of this design.** They are not quoted as results; experiment C1 measures leaf and node rows.

## Other overnight actions

- O-1. Branch `land-3x` merged into main with a true merge (`e39bc7af8`) so the receipt's bound commit `2947727d6` stays valid. No task ticked: U023a, U023b and U023c are still `unimplemented.sh`, and the task 3.4 receipts and inventory flips are not written.

## From experiment C1 (dense memory backend, `EXPERIMENTS.md`)

- E-1. **TOP = 63 is cheap.** Measured +280 rows per flush and +140 per old-memory read, no new column, no change of k in any leaf size (run `20261111T071045Z-c1-rows`). This supports R-4 (TOP >= 63); still OWNER to confirm.
- E-2. **Domain tags in the sponge capacity, not the first input.** Same separation at 0 extra permutations; first-input tags would cost about 2x on every tree node (ESTIMATE). Accepted for the prototype; the C1 Quint lane checks equivalence (question 8).
- E-3. **Arenas lean to one arena.** One arena with one flush unit: 12,281 backend rows; four arenas: 49,172 (measured). At S = 16 four arenas leave room for only 2 old-memory reads (or 6 with shared flush units, not built). Final choice in C4 after the non-interleaving check.

## From the lookup-cache study and its audit

Sources: `research/sherlock-lookup-cache-20261111/CONSENSUS.md` (5 lanes, Gemini 4 Carbon High) and `AUDIT.md` (Fable 5.2: adopt with changes; 3 blocker, 11 major, 9 minor).

- L-1. **Memory, PROG and type tables stay on the dense tail-buffered Merkle design (R-3), plus a small number of 8-record cache lines per leaf opened against the pre-state root and served on the bus (Profile A).** One-pass, Poseidon boundary digests. Run-wide write-once LogUp does not replace it. The ten block-cache rules of AUDIT section 5.1 become MEM / PAGE contract items.
- L-2. **The scalar `cold_read` stays.** An isolated old read costs less with the scalar walk (4,761 rows) than with a line (6,020, ESTIMATE). Cache-line counts per leaf (`K_page`, `K_rom`) are not frozen until the per-rule maximum table (run-length AUDIT finding 6, lookup AUDIT blocker 1) is measured; a step must never need more lines than a leaf has (GP-1).
- L-3. **Value digest: the study's single precomputed sponge prefix is rejected.** The sponge puts the input length in its initial state, so it needs one constant per preimage length (AUDIT blocker 2). The VALUE DIGEST module computes exactly `sherlock-value/1`; no 2-input bus re-hash.
- L-4. **The landed C1 interface is the MEM interface** (`membackend.rs`, `dense.rs`, key `8 addr + kind`). The study's section 3 interface (key `(arena << 64) | addr`, `WindowBoundary`, `FlushPort`, `StateTok`) is not adopted (AUDIT blocker 3).
- L-5. **Profile B (ZmLogUp + Horner) is benchmark-only behind the seam; not built before milestone M4.** It needs no fork, but it changes the leaf statement, node, seal and continuation and replaces the `digest_inj` premise, so it is not a two-trait swap.
- L-6. **Tagged DEC + RANGE lookup: the three-column form** (sound without conditions; about 22,600 node rows saved, ESTIMATE). The two-column form needs a bound on `ctrl`/`syn` and is not adopted.
- L-7. **ALU keeps the overflow flag in its tuple**, so overflow (`Rejected(10)`) stays provable. Ops are only the machine's ops (no division, signed or bitwise ops that lane C says the machine lacks).
- L-8. **Row 6 promotion (4 bus requests per step) needs a new layout plan** (rows 5 and 7 read its lane copies); not adopted until planned and measured.
- L-9. **Arenas (R-6, E-3): one arena.** Four arenas with 4 + 2 cache lines do not fit `k_leaf <= 16` (about 67,500 to 85,100 rows, ESTIMATE); one arena is about 50,700 to 51,800 (ESTIMATE). The non-interleaving check is still required before the spec freezes this.
- L-10. **Items the study invented are dropped**: `SRC-025`, `Value::Tuple/Record/Array`, `specs/quint/`. The 2-leaf `RunLift` is unbuilt and not part of the design until measured.

## From the rule-maxima experiment (`EXPERIMENTS-MAXIMA.md`, experiments X1 and X2)

- M-1. **One arena (L-9) stands.** The non-interleaving property holds for all 120 enabled rules with no counterexample (premises S1 to S4 checked mechanically, dynamic check 0 violations over 9,031 slices; the final induction is code reading). The four-arena layout is not needed.
- M-2. **At most 3 appends per step** (STATIC = OBSERVED, `BHelperScalarExit`). So 2^64 steps can need up to 3 x 2^64 cells: TOP = 61 (2^64 cells) is too small and TOP = 63 (2^66 cells) is enough. This confirms R-4 / E-1 (TOP = 63) as the recommendation; OWNER to confirm.
- M-3. **`W_flush = 1` in one arena** (DERIVED). `K_page >= 4` is a code-reading bound, not measured; `K_rom` has no bound yet (the ROM rows per step are open). L-2 stays: cache-line counts are not frozen.
- M-4. **Rule inventory: 120 enabled step rules** (108 of `supported_rules()` plus 12 `XAdtHelperBuild*` / `XAdtHelperPlan*`); 18 rules never fired in the corpus (16 failure or refusal paths).

## From the architecture audit (B2: `ARCH-AUDIT-astra.md`, `ARCH-AUDIT-RESPONSE.md`)

- A-1. **All 20 Astra findings were admissible and are fixed in `ARCHITECTURE.md` as design text** (proposed, unbuilt, unmeasured where new). Four kits added: INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE, GP-3 general verifier (23 kits).
- A-2. **GP-2 built path: input consumption at the seal** (no consume rule exists in the owner machine today). OWNER (OD-20): whether GP-2 also needs a mid-run consume rule; if the owner reads GP-2 that way it is required work, not a narrowing.
- A-3. **Input admission by digest with an empty genesis memory** (recommended; OWNER OD-17), then amend SRC-021 to the one-arena Init field list (OWNER OD-18).
- A-4. **Semantic reference for the full language: a separate evaluator over `moriarty-sherlock-nominal/1`** (recommended; OWNER OD-21): the nominated `reference.rs` machine has no constructor values or expressions.
- A-5. **TOP = 63 holds only if one logical record is one physical cell** (OD-11); two cells per record would need TOP = 64.
- A-6. **Narrowing options are not acceptance paths** of the implementation change (OD-1c, V-1b/c, two-node GP-4, adapter-only GP-3).
