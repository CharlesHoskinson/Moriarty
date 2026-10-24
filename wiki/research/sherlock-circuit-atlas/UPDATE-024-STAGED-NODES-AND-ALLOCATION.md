---
id: moriarty.research.sherlock.atlas.update024
title: UPDATE024 Staged Nodes and Allocation
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

# UPDATE-024 — Staged Nodes and Allocation

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback. Earlier Opus labels are historical; this is Muse prose and rewrites no earlier chapter.
**Internal scope:** This header and the closing scope note stand outside the visitor copy. The visitor copy describes the finished Moriarty Sherlock website, live only when Sherlock is fully finished and accepted. It makes no claim about current source, proof, native or target status. UPDATE-023 and all prior chapters remain unchanged.

## Finished visitor copy

Past the Return landing of UPDATE-023, the finished Sherlock site keeps the same helper in view and asks a plainer question: what does it mean to decide a node's shape before any node exists? The answer is built as a stencil bench. A tag is cut and held. Nothing is poured. Only later, at a separate station, does the held tag become a row in DATA with real fields and a real address.

The Moriarty wordmark stays flat at weight 600. Identifiers are tabular monospace and never reflow. Every element sits on the 4 px spine. Copper marks divergence only. The thirty-nine footprints on the floor remain semantic selectors: they select meanings, they are not thirty-nine completed chiplets.

## The chosen journey: the stencil bench

I have chosen a stencil bench with a held-tag tray, over a header plaque that merely announces a mode or a chip that pops into existence fully formed, because staging must be visibly different from allocation. The bench has three lanes, one held tray in the helper, and one later pour spout at Exit. A visitor who learns to read the tray — tag present, address absent — has learned the whole chapter.

**Stage one — three lanes, two histories.** The bench receives exactly three construction modes, and they do not arrive the same way. Leaf arrives in phase First, straight from Entry: it never passes the Tag station, because a leaf's single operand needs no first-operand inspection on this path. Cons and Branch arrive in phase Second, having already passed Tag, where their first operand was found eligible — a UInt for Cons, a Leaf or Branch for Branch. That earlier finding is context they carry in, not a verdict Construct re-derives. The lane a mode travels is part of its meaning, and the bench draws the short Leaf lane and the two mezzanine lanes at different lengths so the eye learns it before the text says it.

**Stage two — the right-kind test, and only that test.** At the end of each lane stands one kind gate, and each gate inspects a different operand. The Leaf gate reads operand A itself: a UInt passes, anything else does not. The Cons gate reads operand B: Nil or Cons passes. The Branch gate reads operand B: Leaf or Branch passes. Construct checks the operand in front of it against that lane's accepted kinds; it invents no broader proof about the whole program, and no other error code is substituted here — a wrong kind is PrimError 11, full stop.

**Stage three — the stencil is cut, nothing is poured.** A passing gate cuts one stencil: a PendingNode carrying only a tag — Leaf, Cons or Branch — and lays it in the helper's staged slot. The phase becomes Done. The tray is deliberately shallow. It holds a tag, not a node: no DATA address, no field rows, no new reference identity. The helper's operand references still sit beside the tray exactly as installed, space and index unchanged, pointing at the values they always pointed at. A visitor can cover the pour spout with one hand and lose nothing, because nothing has reached it yet.

**Stage four — the refusal fork.** A failing gate cuts no stencil. It stages PrimError 11 and the phase likewise becomes Done, but the tray's contents are an error, not a pending shape. Copper appears only here and where lanes diverge: the error fork turns away from the bench toward the Failure station, where it will be reported as a scalar error with code 11. No success node is drawn on this path, not even faintly.

**Stage five — Exit pours, somewhere else, later.** The pour belongs to a different station and a different moment. Exit reads the held tag and finally allocates, at the DATA address that was already active before the pour. Leaf pours one field, filled by operand A. Cons and Branch each pour two ordered fields, filled by operand A then operand B, in that order, from the references the helper has held all along. Only now does a reference identity exist for the new node: the result is the DATA reference of the row just allocated, and the helper is cleared as the value is published. Staged tags were never addresses; allocated rows are never mere tags. The bench keeps the two facts on two different tables so they can never be confused.

**Stage six — etch and insert, kept apart.** Across the finished board, two kinds of thing must never be mistaken for each other. The etched lanes are the constraints: the fixed relations that say which kind each gate accepts, which fields each tag pours, and in which order. The loose inserts are witness proposals: values a host supplies and slides into the lanes for checking. An insert that fits is checked; an insert that glows is merely drawn. Satisfaction is the etch accepting the insert — the kind gate passing on the actual operand, the pour following the held tag — never the brightness of a proposal and never a payload a host merely asserts. The scene lights accepted paths by tracing them, not by decorating proposed ones.

## Alternatives considered

A header or status plaque naming the mode was considered and set aside: it asserts a stage instead of showing one, and a plaque cannot distinguish a held tag from an allocated row. An instantly materialising chiplet was considered and set aside: it teaches that Construct allocates, which is false, and it erases the Exit pour that deserves its own station. A single undifferentiated lane for all three modes was considered and set aside: Leaf's direct First-phase arrival and the Tag-mediated Second-phase arrival of Cons and Branch are real history, and flattening them hides why the first operand is not re-examined. The recommended arrangement is the one above — three lanes with two histories, one kind gate per lane, a shallow held-tag tray with no address, an honest PrimError 11 fork to Failure, and a later ordered pour at the previously active DATA address.

## Three.js, PCB treatment and accessible form

In the explorable Three.js scene the build-up follows the bench order: the short First-phase Leaf lane and the two Second-phase lanes from the Tag mezzanine light in turn, each kind gate inspects its operand, a flat stencil drops into the helper's tray with its tag legible and its address face blank, the PrimError 11 fork turns away in copper toward Failure, and only at the Exit station does a DATA row appear at the previously active address with one field for Leaf or two ordered fields for Cons and Branch. Copper is used only where paths diverge — the three lanes, the error fork, and the separation between etched constraint lanes and loose witness inserts. Nothing rotates or pulses for decoration, and no meaning is carried by glow alone; a proposed witness insert is drawn as an unetched outline until the etch accepts it.

The flat, reduced-motion and screen-reader form renders the same journey as six stacked sections on the 4 px spine, each listing its components in tabular text: Leaf selected in phase First directly, Cons and Branch selected in phase Second after Tag; Leaf testing operand A as UInt, Cons testing operand B as Nil or Cons, Branch testing operand B as Leaf or Branch; success staging a PendingNode tag with phase Done and no allocation, no DATA address and no new reference; a wrong kind staging PrimError 11, bound for Failure with no success node; Exit later allocating at the previously active DATA address with one ordered field for Leaf and two ordered fields, operand A then operand B, for Cons and Branch; and constraints shown as etched relations distinct from supplied witness proposals, with no proposal counted as satisfaction by appearance alone. Keyboard order follows the build-up. A screen reader announces, for example: 'Stage three. Mode Cons, phase Second. Operand B kind Cons passes. PendingNode Cons staged in the helper. Nothing allocated yet; no DATA address exists.' No distinction is carried by hue, glow or motion alone.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, live only once the full Sherlock goal is finished and accepted. It adds no site code, assets or deployment. Engineering status is kept in the separate dated note, not in the visitor copy.