---
title: "Audit of the lookup cache consensus (Fable 5.2)"
type: source
tags: [lookup, cache, memory, chip-leaf, audit]
sources: [lookup-cache-audit, lookup-cache-consensus, c1-dense-memory-experiment, runlength-consensus-audit, final-design-decisions]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-lookup-cache-20261111/AUDIT.md
source_url: local
---

# Audit of the lookup cache consensus (Fable 5.2)

## Summary

Read-only audit (no build, test, proof or model run) of [[lookup-cache-consensus]] at checkout `9973d633b`, on the final file of 84,457 bytes. **Verdict: adopt with changes. Counts: 3 blocker, 11 major, 9 minor.** Adopt the direction: memory, PROG and the type tables stay on the dense tail-buffered Merkle design (R-3) with a small number of 8-record cache lines per leaf opened against the pre-state root and served on the existing bus; one-pass with Poseidon boundary digests; run-wide write-once LogUp does not replace it. The audit found no stale-line, aliasing, partial-segment, forgery or overlap attack on the block cache under the ten rules of its section 5.1. Sections 2.2 and 3 of the consensus are not adopted as written. Profile B is sound in outline and needs no fork, but it is not a two-trait swap and cannot stop and resume without reloading live memory; record it as benchmark-only. The lead's decisions L-1 to L-10 follow it ([[final-design-decisions]]). See [[BlockCache]].

## Key Claims

Blockers:
1. Cache capacity per step has no bound and the scalar fallback is removed (GP-1): a step needing more than `K_page` old segments or `K_rom` ROM blocks has no proof even at one step per leaf. Required: a per-rule table (bus requests, distinct old segments, distinct ROM blocks per table, appends, stack spills) whose maxima fit the chip heights at S = 1; keep `LeafShape` open until then.
2. The value-digest module does not compute `sherlock-value/1`: the sponge carries the input length in its initial state, so the state after the first block is one constant per preimage length (4 to 7 words), not one; a 2-input bus chain is a different hash; the reference file does not exist; Tuple, Record, Array are not forms of `sherlock-value/1`. The saving is real and larger: the constant header `[V, 1, tag, arity]` folds two permutations into constants for `UInt`, `Bool`, `Leaf`, `Cons`, `Branch`.
3. Section 3 is not the landed interface: the key `(arena << 64) | addr` is not injective at TOP = 63 (the landed `8 addr + kind` is); two MEM chip ids against one landed tuple; `WindowBoundary`, `FlushPort`, `StateTok`, `assign_backend` exist nowhere in `crates/mori-chip/src`; the landed trait has `persist` and `cold_read`; chip ids differ from the code. Required: the merge takes the landed C1 interface as base and adds one block-line method.

Major (4 to 14):
- 4: the block-cache contract is incomplete; hot reads need `addr < len_cur` at the step, not `< len_post`; line leaves must be `H_REC(kind, a, b, c)` as in `persist`.
- 5: the ALU tuple loses the overflow flag (landed key `op + 8 flag`) and the multiply rule makes overflow (`Rejected(10)`) unprovable, narrowing GP-1; the op list adds operations the machine lacks.
- 6: the tagged DEC + RANGE lookup is given in two forms; the three-column form is sound unconditionally and saves one lookup only, about -22,600 node rows (ESTIMATE), not -29,062; the two-column form needs a bound on `ctrl` or `syn`.
- 7: row figures are not of this design and ignore C1. Composed one-arena leaf (ESTIMATE, from measured parts: 15,226 rows with no old read, 4 lines x 6,020, 2 ROM lines x about 3,850, value digest, stack spills, decompositions, bound checks): about 50,700 to 51,800 rows, margin about 14,000. Measured `persist` at W = 1 is 12,417 rows (TOP = 63). Node base about 847,494 (ESTIMATE) with the dense backend, not 823,466. An isolated old read costs more with a line (6,020) than the scalar walk (4,761, measured); the per-record figure is a best case; no locality census exists.
- 8: four arenas do not fit with the proposed cache sizes: 53,341 (one flush unit each) or 35,699 (two shared units, ESTIMATE) without old reads, about 85,100 or 67,500 with 4 + 2 lines, above 65,530. Required: state the arena count as open (R-6, decide in C4); four arenas with 4 + 2 lines need `k_leaf = 17` or fewer lines.
- 9: Profile B changes the leaf statement, node, seal and continuation and replaces the `digest_inj` premise by a probabilistic one; it can resume only by reloading all live memory, so not production; one base-field challenge pair meets 128 bits (about 2^-187 at 2^64 accesses), "< 2^-250" is not derived; the `m_w` range check is harmless and not needed; keep the subgroup check, but the loss of binding is overstated. Do not put P5 before M4.
- 10: the 2-leaf `RunLift` is unbuilt; the adopted node gate is 950,000 rows; with the dense backend term the upper end is at or above 2^20.
- 11: row 6 is not a passive replica; promotion needs a new layout plan.
- 12: ROM depth 32 is a circuit constant that bounds the program; no ROM leaf domain, padding rule or path mapping; the rule and table counts cite the wrong lines.
- 13: SRC-025, the new value forms, `ArrayGet`/`ArrayLen` and `specs/quint/` do not exist (models are in `models/sherlock/quint/`).
- 14: "5/5" holds for four claims only (Merkle with block cache as default, compatible with the run-length consensus, LogUp behind the interface, b2 rejected).

Minor (15 to 23): wrong baseline commit; wrong lines for the identity commitment (it is `inner.rs:181-182`) and for `EMPTY_STACK`; `digest_inj` is a premise the digest instantiates; G-C margin depends on the unmeasured miss rate (at 2 steps per leaf about 160 min serial, ESTIMATE, above the 2 h gate); multi-flush has a MockProver test but no theorem; rename the study's dissents (for example LC-1 to LC-6); leaf public lanes differ from `ivc.rs:70-74`; prior art not re-checked, CVE date differs from the run-length audit.

Section 5 lists 28 module interface items: ten block-cache rules (5.1), CORE memory obligations 11 to 14, ROM items 15 to 19, DEC/RANGE, ALU, HASH and VALUE DIGEST items 20 to 23, and backend and boundary seams 24 to 28 (including `LeafShape` and the arena count staying parameters until the per-rule table, the locality census and the non-interleaving check exist).

## Key Quotes

- "Adopt with changes."
- "A step whose requests do not fit cannot be enabled."
- "Deduplication and ordering of lines are not soundness conditions: the lookup is set inclusion and a provider is a function of its key."

## Connections

- Target: [[lookup-cache-consensus]]. Decisions: [[final-design-decisions]] (L-1 to L-10). Measured base: [[c1-dense-memory-experiment]]. Earlier audit: [[runlength-consensus-audit]].
- Concepts: [[BlockCache]], [[DenseTailMemory]], [[ValueDigest]], [[ChipLeaf]], [[GeneralPurposeRecursion]].

## Contradictions

- Against [[lookup-cache-consensus]]: all blockers and majors above (also listed on that page). The consensus labels the C1 cold-read rows 4,621 to 4,761 ESTIMATE; this audit and C1 call them measured.
- Against [[final-design-decisions]] L-9: the audit (finding 8, item 28) keeps the arena count open until C4 and the non-interleaving check; L-9 picks one arena now, using the audit's own figures. The run-length audit (finding 4) would make four arenas the default meanwhile.
- The audit's composed four-arena figure without old reads (53,341 at TOP = 63) differs from the 52,205 on [[c1-dense-memory-experiment]] (TOP = 61); both are ESTIMATE.
