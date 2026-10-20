---
id: moriarty.research.sherlock.atlas.update024.status
title: UPDATE024 Muse engineering status snapshot
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T18:44:22.765895+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-6d4481ee0888b1e9d32f
---

# UPDATE-024 — internal engineering status

**Date:** 2026-10-20. Internal only, kept apart from the finished visitor copy. The website is UNPUBLISHED and goes live only when the full Sherlock goal is finished and accepted. No site-live, circuit, native, proof, target or EARS qualification is claimed. The finished-future visitor direction does not license any current-result claim below.

## Current facts: Entry merged, Construct NEXT

Existing ScalarEntry is merged in PR 359, exact-12 remote readback, merge commit `1bfe8cda0935e0d8fc54df1873d296facaf9cd41`: the Entry emitter and real consumer plus 20 test functions in source (7 genuine-owner and 13 raw). All 20 tests are UNRUN and uncompiled. That bounded Entry source, its two bounded source reviews and its merged status establish nothing about Construct execution, SDK synthesis, MockProver, proof, native qualification, measured degree k, performance or stagenet.

For ScalarConstruct there is, at this cut, NO delivered handwritten emitter, consumer or tests, and no review. The NEXT ScalarConstruct emitter source is only being prepared. No implemented constraint geometry, exact gates or ports, driver charges, preservation plan, witness route or SDK relation for Construct exists or is claimed here, and the existing Entry 20 UNRUN tests do not execute a Construct SDK relation and do not cover the whole Entry/Tag/Construct/Exit/Failure pipeline.

## What the current Core fragment defines semantically

The public Core full fragment and the lossless Construct row and helper-rule selector define the branch meaning depicted in the concept. Construct is selected iff the helper is Scalar with (phase First, mode Leaf) or (phase Second, mode Cons or Branch). Leaf: operand A of kind UInt stages PendingNode Leaf, otherwise PrimError 11. Cons: operand B of kind Nil or Cons stages PendingNode Cons, otherwise PrimError 11. Branch: operand B of kind Leaf or Branch stages PendingNode Branch, otherwise PrimError 11. Any applied Construct outcome sets phase Done. Tag has already established first-operand eligibility for the Second-phase branches; Construct fabricates no new global SI proof. Construct allocates nothing: it writes only the updated helper's staged PendingNode or error. In the existing later Exit row, a PendingNode allocates at the previously active DATA address with one field from operand A for Leaf or two ordered fields [operand A, operand B] for Cons and Branch and publishes the corresponding DATA reference; a staged PrimError 11 instead selects Failure and yields the scalar error, with no success node. Reference space, index and ordering and the kind checks are source semantics, distinct from any decorative scene object. These Core, Entry and later Exit/Failure boundaries are separate source rows, not one already-accepted native proof.

## Open scope, unchanged

SI08, SI15, SI17, SI20, SI21, SI22, full StepInv, program membership, P1 provenance and source authentication, retained-family and circuit-issuer questions, the whole 39-selector scope, refinement, grouping, residual, finance, persistence, recursion, stagenet and EARS remain open. Maintainer C13 and native accounting and execution boundaries are unchanged. UPDATE-023's five authored fields and its dated historical status remain unchanged and are not rewritten by this note.