---
title: "Astra audit of the architecture merge (B1)"
type: source
tags: [audit, final-design, architecture, astra]
sources: [arch-audit-astra]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/ARCH-AUDIT-astra.md
source_url: local
---

# Astra audit of the architecture merge (B1)

## Summary

Read-only audit by GPT-6 Astra of the B1 text of `ARCHITECTURE.md` (2,776 lines before revision B2; [[sherlock-architecture]]), at baseline commit `b85b00213`. Verdict: **adopt with changes**, with 4 blocker, 15 major and 1 minor finding. "The overall direction is sound" (dense tail-buffered memory, cache lines, hash-chain stacks, canonical value digests, segment and claim separation, no claim-occurrence counter), but the text was "not ready to freeze as an implementation specification". The findings concern defects in the proposed contracts, not executed exploits of the landed circuit. The file modification time is 03:32. All 20 findings were answered in [[arch-audit-response]].

## Key Claims

- Blockers: (1) CORE scratch rule (zero a scratch cell only if the rule does not write it) does not give write-before-read; a rule can read `Pre(v0)` and write `v0` later. (2) One operation per stack cannot express `BHelperValidateField` and `BHelperEqualField` (rewrite then push in one owner step); `below == EMPTY` is possible at count one. (3) Genesis authentication (Init, `prog_d` opening, initial memory, `val_d`, empty dependencies) has no kit or owner. (6) The GP-2 "SET hash" dependency format is not a complete discharge protocol.
- Majors: (4) no shared authenticated-read interface for VALUE DIGEST and PAGE/ROM; (5) BOUNDARY flat sponge gives no selective opening, and the opening API is absent; (7) the C3 composition theorem covers bus tuples, not copied ports, and the landed bus is membership, not `BusBalanced`; (8) `FitsLeaf` is at most 8 appends per leaf while the W = 1 window takes `15 - r_pre`; (9) capacity bounds must count physical cells, initial length and ROM sub-rows, and narrowing alternatives must stay rejected; (10) fixed DEC tag against a witness-disabled slot; (11) append transport inconsistent; (12) `rules_d` is a required computation, not a naming issue; (13) CONTINUATION cites a persistence interface that `MemBackend` lacks; (14) the nominated reference harness cannot represent the corpus (`UnsupportedGeneric`); (15) the GP-3 general verification path has no kits; (16) GP-4 inline work and shape base cases are not in the node relation; (17) ZmLogUp imported-tail entries are not tied to the balance; (18) completion could be reported before required interfaces and proofs; (19) OpenSpec outline missing work and wrong evidence mappings.
- Minor (20): `WindowCfg::bits` returns accumulator cells; HASH family wording; Horner with dense memory allowed in one place, forbidden in another; two `LeafShape`s; blanket provider disjointness.
- Judgment of the merge's proposals: adopt ROM chip id 4, `open_line`, the LogUp tuple compression and the new names; adopt with correction the tag values; adopt with conditions `H(SHK_SET_LEAF, claim_d, n_steps)`; do not adopt the dependency-join discharge as specified; adopt the binding of boundary item 5 but revise the opening.
- Kit readiness table: only RANGE, ALU and HASH were locally implementable; STACKS "not implementable correctly"; ZmLogUp appropriately deferred; most other kits blocked or conditional.
- Not verified: no build, test, proof, Quint run or OpenSpec validation; no executable counterexample; per-rule maxima, inventory and non-interleaving not proved.

## Key Quotes

- "The additional OWNER labels record unresolved choices; they are not permission to narrow GP-1 through GP-4."
- "Smaller program bounds, adapter-only GP-3, two-node substitutes for inline work and per-program contracts cannot become final acceptance paths."
- "This was a read-only audit. No files were written."

## Connections

- Subject: [[sherlock-architecture]]; response: [[arch-audit-response]]; decisions A-1 to A-6: [[final-design-decisions]].
- Concepts: [[ModularArchitecture]], [[DenseTailMemory]], [[Continuations]], [[GeneralPurposeRecursion]], [[ValueDigest]].

## Contradictions

- The audit's line citations (`A:` numbers) and its count of "19 capability outlines" refer to the B1 text; revision B2 changed the line numbers and has 23 kits ([[arch-audit-response]]).
- Finding 8 says the C3 theorem `cuts_agree` needs `FitsLeaf`; the B1 text listed no premise. The B2 text corrects this ([[sherlock-architecture]]).
