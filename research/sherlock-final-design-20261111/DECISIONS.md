# Decisions taken overnight 2026-11-11 (owner AFK)

The owner is asleep and asked for finished, validated plans. Where a choice was needed, the lead took the recommended option and recorded it here. Each entry names its source. The owner can reverse any of them; entries marked OWNER are also listed in the morning report as questions.

## From the run-length study and its audit

Sources: `research/sherlock-runlength-consensus-20261111/CONSENSUS.md` (5 lanes, Gemini 4 Carbon High) and `AUDIT.md` (Fable 5.2: adopt with changes; 3 blocker, 10 major, 7 minor).

- R-1. **2^64 is the SRC-004 counter range, not a circuit size.** Adopted (CONSENSUS Q1, confirmed by AUDIT).
- R-2. **GP-1 and GP-001 text: use the audit's corrected text (AUDIT.md section 5), not the CONSENSUS text.** The CONSENSUS text narrows GP-001 ("every admitted program ... every valid input", from Lane E only) and adds bounds the current text does not have (AUDIT blockers 1 to 3). The audit text keeps every current sentence and only appends. Applied in night-plan step D3.
- R-3. **Memory: dense tail-buffered memory with hash-chain FRAMES and TASKS stacks.** Adopted (5/5 lanes on the core mechanism; AUDIT found no attack on the flush rule, address reuse, tail forgery, the 2^64 - 1 edge or domain tags). The invariants of AUDIT finding 8 (canonical `tail_d`, tail open and pass-through rules, kind-tag checks on reads, tagged hashes) become MEM module contract items and must be in the architecture and the specs.
- R-4. **Tree depth: the address space must not bind before the step counter.** With at most 3 appends per step (unverified, AUDIT), TOP = 61 (2^64 cells) can run out before 2^64 steps (AUDIT blocker 3). Recommended: size the tree so cells >= appends-per-step x 2^64 (TOP >= 63 with 8-cell segments), and measure the extra depth cost in experiment C1. OWNER: confirm TOP >= 63, or accept a memory bound below the step bound (that would narrow GP-1 and needs the owner).
- R-5. **Claim counter: no `n_claims < 2^64` bound in GP-001.** With occurrence counts summed through joins, a shared DAG reaches 2^64 at depth 64 with about 128 proofs, which would limit GP-4's DAGs of any depth (AUDIT blocker 2). Recommended: count distinct claims, or carry no count in the public statement; the architecture merge (B1) decides the mechanism. OWNER: confirm.
- R-6. **Arenas: open.** One arena (3 lanes) vs four arenas (2 lanes). The one-arena variant depends on a 108-rule non-interleaving claim that is a code reading, not a checked invariant. Recommended: the MEM interface hides the arena count; experiment C1 measures both and checks the non-interleaving claim mechanically; decide in C4.
- R-7. **Continuations: tier 1 (segment tree) plus the retained-memory checkpoint mode only.** The address-free checkpoint mode is not defined well enough (AUDIT major). Continuations compose under GP-4 and the checkpoint domain tag, not "via GP-002" (GP-002 forbids that use).
- R-8. **Slot-0 zeroing does not replace write-before-read.** The architecture keeps a write-before-read obligation for scratch (AUDIT major).
- R-9. **D-1 (linked slices) stays open until measured** (`dissent.md:202`, task 4.18). It is not closed by the study.
- R-10. **Row and throughput figures in the study are ESTIMATE and not of this design.** They are not quoted as results; experiment C1 measures leaf and node rows.

## Other overnight actions

- O-1. Branch `land-3x` merged into main with a true merge (`e39bc7af8`) so the receipt's bound commit `2947727d6` stays valid. No task ticked: U023a, U023b and U023c are still `unimplemented.sh`, and the task 3.4 receipts and inventory flips are not written.
