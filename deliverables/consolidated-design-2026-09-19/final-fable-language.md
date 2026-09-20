**Verdict: changes_requested** (design consolidation only). The candidate honors the user's steering. It keeps permissionless public programs, native ZKIRv3, no Lean dependency, full MC03/MC06 and MC/SP/G obligations, treats March 2027 as a dated planning assumption, allows accepted phase partiality to create only authorized duties, forbids duties from mere request receipt, requires representation correspondence rather than literal hash equality, and implies no perpetual authority. The defects below are internal inconsistencies or ambiguities that would weaken the single design if left as written. Open implementation exits are not counted as defects.

**Required changes**

1. **openspec/.../requirements.md, first paragraph.** It names UNI-001–016 as the normative clauses, but spec.md adds UNI-017. Change to UNI-001–017. Otherwise the backend contract (ZR01–ZR16) has no normative EARS anchor in the package.

2. **openspec/.../traceability.md, MPLR owner table and conflict table.** ZR01–ZR16 have no crosswalk row. Add one row per ZR ID naming its U milestone, UNI-017 plus the specific UNI it refines (for example ZR04/ZR09 to UNI-009, ZR14 to UNI-010, ZR15 to UNI-015, ZR06/ZR11 to UNI-003), and evidence status "specified-only". The roadmap says the register maps all obligations without resetting evidence. As written, the ZR rows sit outside the register and could be closed or dropped silently.

3. **ROADMAP.md, "Recursion planning horizon".** The sentence "Current interfaces can support an early scoped milestone" is a present-tense capability claim. The traceability dispositions require fresh backend inspection before any such claim, and the user states older interface findings are historical. Reword to "U1 will determine whether currently released interfaces support an early scoped milestone." Same correction for docs/MORIARTY-BACKEND-REQUIREMENTS.md intro: "U0 freezes this contract" conflicts with "revalidate actual released interfaces." Say U0 adopts a versioned requested contract that U1/U4 revise against the released tuple.

4. **docs/MORIARTY-CONSOLIDATED-DESIGN.md, "Native proof-carrying history"; ZR09.** "Bounded causal composition" is ambiguous about whether history depth is bounded. The user's target is full recursive multi-parent scope. State explicitly that per-stage predecessor fan-in is bounded (at least two for MC06) while lineage depth is unbounded across stages, with ZR04 well-foundedness covering unbounded depth. Without this, "bounded" can be read as reducing MC06 to a fixed-size DAG.

5. **docs/MORIARTY-BACKEND-REQUIREMENTS.md, ownership table row ZR14; ROADMAP.md U4/U5.** ZR14 names the kernel as an owner "where joint proving is selected". U4 precedes U5 and U5 promises no federation requirement for direct Midnight use. Add to ZR14 and to U4's exit that separate-party private handoff and split/join must be demonstrable without the federated kernel. Kernel-assisted joint proving is an optional additional profile under UNI-011 assumptions, never the sole MC06 evidence.

6. **ZR03 and design "Canonical stage statement".** "Canonical public inputs" alongside "source/Core/program identity" can be read as demanding one literal identity across representations. State that source, Core, ZKIR and VK identities are distinct commitments linked by the pinned correspondence relation, and that ZR03 mutation tests reject unauthorized mismatch of that linkage, not mere inequality of hashes. This aligns ZR03 with the traceability disposition on representation hashes.

7. **openspec/.../traceability.md, "All five retained composition operators".** The five are not named in the package. List them (sequential prefix, disjoint parallel, shared-state interleaving, atomic publication within a domain, cross-domain conditional/compensation) or cite the frozen source. An unnamed denominator cannot be validated at U4.

8. **openspec/.../traceability.md, MPLR-007.** "Joins and late results" maps only to UNI-009. Late results belong to UNI-007/008 as well as UNI-009. Add them so the late-success/refund race in U3 has an EARS owner.

**Consistency confirmed, no change needed**

- Ledger induction versus native recursion: ROADMAP U2/U4, design history section, UNI-009 and the conflict table agree. Ledger induction is a labeled U2 subset and cannot close MC03/MC05/MC06.
- Kernel boundary: the responsibility table, UNI-011 and UNI-001 keep the kernel optional, non-authorizing and unable to weaken signed constraints. Threshold-only foreign destinations are a named trust boundary, not a hidden bypass.
- Authority and duties: affine spending permission, linear receipts, persistent liabilities requiring consent, separate scoped recovery authority, and no default perpetual right are consistent across design, UNI-004/005/008 and ZR15.
- Fees decided at debit: ZR03/ZR11 integration text allows ledger cap checks for late-bound exact fees while the circuit binds the signed cap. This is consistent with the stage statement's "circuit constraint or ledger check" rule.
- Pel template: it is a bounded symbolic internal lane with one correction round and cannot authorize public programs. Its unbound artifacts are explicitly open implementation, not a design defect.

**Recommended single design after corrections**

Keep the U0–U7 sequence as the only schedule. U0 adopts the ZR contract and correspondence pinning. U1 certifies the exact-arithmetic basis and measures released recursion feasibility. U2 ships the general single-stage path with a labeled ledger-induction history subset. U3 adds conditional two-asset escrow, partiality and recovery. U4 qualifies native recursion, unbounded-depth bounded-fan-in private composition and kernel-free handoff. U5 adds the optional kernel and adapters. U6/U7 close libraries and release. Next ZKIR/recursion requirements are ZR01–ZR16 as listed, with items 4, 5 and 6 above clarified.