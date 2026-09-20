---
title: "Simplicity security findings"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, simplicity, research]
---

# Findings that change the research requirements

1. **Jets need implementation evidence.** Logical equality of a reference expression and its intended jet specification does not establish native or target implementation correctness. Track each version, precondition, failure behavior, resource model and proof dependency. No Rocq/VST build was run here.
2. **Temporal units and versions matter.** The execution-model HTLC example calls `check_lock_height(1000)`, but accompanying prose describes a delay since receipt. The [timelock reference](https://docs.simplicity-lang.org/documentation/timelocks/) defines height as absolute, distinct from relative distance. Its relative-lock section warns that direct relative-lock jets were deprecated due to an implementation error and provides a workaround. The pinned core names corresponding functions `broken_do_not_use`; the older state/covenant example still uses `check_lock_distance`. These are source discrepancies, not a new reproduced exploit or a claim about deployed nodes.
3. **A checked Boolean is not necessarily an enforced condition.** The [jet reference](https://docs.simplicity-lang.org/documentation/jets/) distinguishes value-returning comparisons from failure-producing checks. A successful computation of `false` must not be confused with transaction rejection. Moriarty needs a dominance/effect obligation that every mandatory condition constrains acceptance.
4. **Wallet descriptions can misstate intention.** The [txmanifest documentation](https://docs.simplicity-lang.org/documentation/txmanifest/) explicitly says descriptions are not verified by the format and can mislead users about transaction effects. This strengthens MPLR-018: user-visible claims must be tied to the signed formal specification and actual effect summary, with unproved prose labeled accordingly.
5. **Commitment-time bounds have a scope.** The technical report's delegation section (visually read page61) allows a later program component and warns universal commitment-time resource bounds no longer follow. This does not mean a single actual finite execution is unbounded; distinguish precommit policy from a fixed execution artifact. The report is a draft with incomplete sections; its Jets chapter on pp59–60 is only a title and blank page.
6. **Availability and privacy are independent.** A hash authenticates supplied state but does not ensure it is available. Witness data disclosure does not provide Midnight-style private evidence. Neither deterministic validation nor Turing incompleteness establishes oracle truth, fair ordering, liquidity or eventual settlement.

These observations motivate security requirements and adversarial examples. They are not blanket conclusions that a language or every jet is unsafe.
