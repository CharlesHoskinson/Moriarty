---
title: "Conditional escrow: reference-to-target exercise"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: tutorial
tags: [moriarty, simplicity, research]
---

# A proof-oriented escrow exercise

This is a proposed research exercise, not executable Moriarty.

A depositor locks an asset for a recipient. Delivery requires the recipient's signature and a proof of a documented predicate; a refund is allowed after a specified absolute height or relative delay. Make that temporal choice explicit.

Define the legal transitions and exact financial effects first. A submitted witness selects a branch but cannot redefine its conditions. Bind signatures and document evidence to the contract, asset, amount, destination, stage and domain. Prove that acceptance implies every mandatory predicate for the selected authorized branch. Separately provide a valid witness for non-vacuity; general completeness requires its own quantified argument. Preserve resource uniqueness, residual obligations and alternative-branch exclusion through the ledger and PCD relation.

Replace one reference primitive with an optimized implementation. Test a changed carry bit, wrong signature message, stale primitive version, absent condition assertion and swapped time unit. Each must be rejected or excluded by an explicit proved premise. Show one valid witness too, so the verifier is not merely rejecting everything.

Finally prove the relevant source-to-ZKIRv3 correspondence. The Simplicity examples are design precedents; they do not establish this Midnight result.
