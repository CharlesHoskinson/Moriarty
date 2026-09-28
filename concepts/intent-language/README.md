# Moriarty Intent Language (MIL/1) — 2026-09-28

**Specified-only proposal.** Nothing implemented, no milestone has accepted it, nothing committed.

A design for an AI-friendly language of signed financial intention that expresses abstract,
cross-chain **conditional settlement with programmable escrow**, and its mapping to every
DeFi and asset category.

- **[EXECUTION-SUMMARY.md](EXECUTION-SUMMARY.md) — start here.** What was executed, the current support checklist, the five decisions and their reversal costs, the six outstanding obligations, and what to do on return.
- **[DESIGN-MIL2.md](DESIGN-MIL2.md) — the current design.** Supersedes MIL/1 after the nine-reviewer review.
- [review/REVIEW-REPORT.md](review/REVIEW-REPORT.md) — the combined review that forced the revision, plus nine individual reviews.
- [DESIGN.md](DESIGN.md) — **MIL/1, superseded.** Retained as the reviewed artifact: types, the predicate language Φ, authority kinds,
  escrow and conditional settlement, holes and monotone completion, canonical form and
  in-circuit authentication, effects and conservation, surface syntax, the ten properties
  that make it AI-friendly, what it closes and what it does not.
- [CATEGORY-MAP.md](CATEGORY-MAP.md) — eight DeFi categories and twelve asset categories,
  each with the constructs that carry it, what it closes, what stays library work, and what
  stays open.

**Origin.** The owner placed conditional settlement and programmable escrow in the intent
language (`wiki/moriarty-architecture.md`, CLM-0978). The inputs are the
[U0 unified proposal](../../deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md), the
[DeFi coverage report](../../deliverables/u0-study-2026-09-28/DEFI-COVERAGE-REPORT.md) and the
[architecture report](../../deliverables/u0-study-2026-09-28/ARCHITECTURE-COVERAGE-REPORT.md), against
Moriarty `8f73784042bd692733c296d0d49f5173be96725e` and defiformal `8c5dd103cd40369a763b02b1504441acce0ce3c2`.

**Five things left open, deliberately visible:** n-party clearing, flash loans (excluded by
the atomic-stage decision), a liquidation latency bound, competing-slash ordering, and the
general form of policy state over reachability.

**MIL/1 is superseded.** Nine PL reviewers (three Claude Opus 5.5, three GPT-6 Sol, three Grok 4.7) unanimously endorsed its shape and unanimously refused to freeze it: three load-bearing formal claims were false, and the surface examples were not in the grammar. MIL/2 is cut against that review. CATEGORY-MAP.md was written against MIL/1 and several of its verdicts are wrong; it must be re-derived after MIL/2 settles.
