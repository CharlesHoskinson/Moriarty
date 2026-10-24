---
id: moriarty.research.sherlock.atlas.update016
title: Sherlock circuit atlas Update016
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T05:25:41Z
tags:
  - moriarty
  - sherlock
  - design
sources:
  - src-3280f584c509eccc2f3e
---

# UPDATE-016 — Program, Context and Configuration on One Lane

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose. It is append-only and rewrites neither UPDATE-015, DESIGN-015, nor any legacy provenance.
**Proposed destination:** `wiki/research/sherlock-circuit-atlas/` as a new linked wiki continuation. That placement is proposed here, not claimed applied, and no older note is migrated or relabelled by this chapter.

## Finished visitor copy

Welcome to the reading head of the Sherlock circuit atlas. Before a board can be explored, it must be read, and this board is read one lane at a time. A single luminous lane enters the head carrying one value; the registers standing at the entrance decide what that value means — a magic word, a version, a section tag, an entry point, a row of the program, a row of the context. The lane leaves into the next register set, and the next lane takes its place. Nothing is allowed to hurry. Walk the head slowly and the whole section assembles itself in front of you: first the program, then the context that gives it meaning, then the retained configuration against which the reading is checked.

Beyond the head, the board keeps what it was told to keep. Two rulers, each twenty-two lanes long, lie side by side and never merge: one records the bounds under which this reading was admitted, the other records the bounds of the layout itself. Further on, the arithmetic apron widens by exactly one lane where two addresses are added, because a sum can be one bit wider than either of its parts. The thirty-nine selectors of this atlas remain its semantic footprints throughout — named places you can stand, each traceable to a declaration. Follow any route and you can always ask what it rests on, and the board will show you the lane, the register, and the name.

## How this station is drawn

This chapter joins UPDATE-015's DATA terraces and the Representation Library rather than replacing them. The Library catalogues declarations; the DATA station placed a section; this station shows a section being read. Three channels remain visually distinct everywhere, as the atlas requires. Source is the unlit solid hairline with a path anchor. Idea — every drawing in this chapter — is labelled `PROPOSAL` where geometry goes beyond a declaration. Evidence is a hatched static fill that appears only where an independent result exists. Selection is a steady glow and a thicker stroke only; selection never sends anything down a lane.

The Moriarty wordmark stays flat, unlit and unextruded, in its existing sans at weight 600 with tight tracking. Identifiers and figures are tabular monospace and never reflow. Layout sits on the measured 4 px spine, ink grounds with the copper accent held for divergence. The luminous routes on dark layered boards are an aesthetic departure recorded by this log; no franchise logos, typefaces, characters or set designs are used.

## The one-lane reading head: `org` becomes `nxt`

The central device is a pair of register silhouettes labelled `org` and `nxt`, drawn as two overlapping frames offset by one lane-width. The entry frame, `org`, is solid; the exit frame, `nxt`, is an outline. A single lane tile slides from left to right between them. Every predicate in the drawing reads only `org`; every write lands only in `nxt`. When the tile clears the frame, the outline becomes solid and the pair advances by exactly one lane. This makes the governing semantics legible at a glance: one lane cannot cascade through several header phases in a single step, because the phase that interprets it is fixed at entry. A zero count selects that table's zero successor for the next lane; a non-zero count is held as a remainder and selects only its own row phase. Stepping is the only motion, and the visitor controls it.

Three stations stand along the head, and the drawing refuses to collapse them. The first is pre-state canonical parsing of PROG and CTX: magic, version and section tag, the entry lane, then the expression, occurrence and type rows, each checked against the interpretation `org` selected. Standalone, this station makes no claim about any later state. The second station is an empty, labelled socket: the configured-context binding, which would compare a retained expected context against the pre-state context. It is drawn unpopulated and marked `uncalled`, because that is its place in the composition described to this author. The third station is a narrow equality bridge marked as a composed requirement: where the full step is assembled, post-state ranges remain unconditional and post must equal pre. The bridge is a requirement drawn as a gate, not a result drawn as a fill.

## Retained configuration: two rulers, never one

Past the head, the META view lays two 22-lane rulers on parallel tracks with different end caps. The upper ruler is `admission22 laneBounds`; the lower is `layout22 laneBounds`. Each lane is a numbered cell in tabular figures, and a toggle that highlights one ruler dims the other rather than blending them, because admission bounds and layout bounds are distinct retained values and neither is a summary of the other. Beside the layout ruler sits the retained layout segment: a version pair, an optional field identifier, the raw ordered width list with its duplicates kept rather than sorted away, an optional scalar identifier, four capacities, and a boundary boolean. Beside the pair sits a separate card for the retained limits, stamped in plain text: `circuit parameter — not authenticated`. The concept states directly that retaining parameters inside a circuit drawing is not authentication against any host, serialized circuit or verifier authority, and that digest bytes drawn here are opaque cells, not commitments. The bound relation, where a caller selects it, is drawn as a clamp spanning both rulers and the layout segment, and captioned as the stronger caller-selected relation it is — never as codec behaviour, admission, or acceptance. Admission identity, run identity and program state stay outside the clamp, on the runtime side of a ruled line.

## The arithmetic apron: 128, 129, 130

Where the DATA terraces from UPDATE-015 meet this station, the floor widens in measured steps. Each source lane is a tile of 128 cells. Where a `start` and a `len` are added, the apron extends by one cell to hold the derived `end`, because the sum of two 128-bit values can reach a value needing 129 bits; that extension is drawn in the source layer's own hairline and labelled with its range. Beyond it stands a separate gantry in a different stroke, labelled as the SDK lowering layer at 130 bits, with its far side open and captioned `remaining bridge`. The three widths are never drawn as one ramp: 128 is what a lane carries, 129 is what the source derives and compares, 130 is what a lower layer works with, and the join between the last two is shown as unfinished work rather than as a hidden step. No performance figures, constraint counts or verification numbers are printed anywhere in this station; the log invents none.

## Alternatives considered

A single fused register frame was considered and set aside: it hides exactly the entry-versus-exit distinction this station exists to teach. A free-orbit board was considered for the rulers and set aside: parallel tracks make non-collapse visible without a legend. A continuous ramp for the three widths was considered and set aside for the same reason. The recommended arrangement is the stepped lane walk, the paired register frames, the twin rulers and the open gantry, because each separation the source keeps is a separation the drawing keeps.

## Accessible and reduced-motion form

The reduced-motion form is a flat schematic on the same 4 px spine. The reading head becomes a table whose rows are lanes and whose columns are the entry interpretation, the check applied, and the next interpretation, with `org` and `nxt` given as words. The two rulers become two labelled tables of twenty-two rows each. The apron becomes a short worked example in figures: two 128-bit lanes, their sum, and the width each layer uses. Keyboard order follows head, rulers, layout segment, apron, and evidence sockets. A screen reader announces, for example: 'Lane 12. Entry phase: expression row. Check: usable index. Next phase: expression row, remainder held.' No distinction is carried by hue or motion alone.

## Scope

This is a concept chapter for a finished site that goes live only once the full Sherlock goal is finished and accepted. It adds no site code, assets, shaders or deployment, and it asserts no physical circuit beyond what its own evidence channel shows. The native and stagenet programme remains primary; this asynchronous log never gates it, blocks it, or speaks for it.