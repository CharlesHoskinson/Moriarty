---
id: moriarty.research.sherlock.atlas.update017
title: Sherlock circuit atlas Update017
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T07:51:58Z
tags:
  - moriarty
  - sherlock
  - design
sources:
  - src-3a379ed0d342235e646f
---

# UPDATE-017 — Padding, Namespaces and Record Boundaries

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose. It is append-only and rewrites neither UPDATE-016, UPDATE-015 nor any earlier note.
**Proposed destination:** `wiki/research/sherlock-circuit-atlas/` as a new linked continuation. That placement is proposed here, not claimed applied.

## Finished visitor copy

Welcome deeper into the Sherlock circuit atlas, where every board is read honestly and every lane earns its place. This station follows a single record as it is made complete, kept apart from its neighbours, and finally set down beside the moment that produced it.

First, the record is padded whole. No lane is left to imply what it does not hold. Where a form uses only part of its frame, the unused lanes are driven plainly to zero and shown as deliberate copper, not as silence. Next, every temporary name is given its own enclosed yard. Helpers work inside their own fences, each yard labelled, none reaching across to borrow a name from another. Finally, the finished record steps to the boundary board, where two snapshots stand before and after a single event. The visitor reads the three together — before, event, after — and sees exactly what one step claims to join.

The thirty-nine semantic chiplets of this atlas remain its footprints throughout. Stand on any one and the board shows the lane, the register and the name beneath your feet.

## How this station is drawn

The station continues the language built across the atlas: luminous routes on dark layered boards in the Tron and printed-circuit tradition, rendered as an explorable Three.js scene when the finished site opens. Three channels stay visually distinct everywhere. Source is the unlit solid hairline with a path anchor. Idea — every drawing in this chapter — is labelled `PROPOSAL` where geometry goes beyond a declaration. Evidence is a hatched static fill that appears only where an independent result exists. Selection is a steady glow and a thicker stroke only; a selected glow is not proof, and selection never sends anything down a lane.

The Moriarty wordmark stays flat, unlit and unextruded, in its existing sans at weight 600 with tight tracking. Identifiers and figures are tabular monospace and never reflow. Layout sits on the measured 4 px spine. Copper is held for divergence: where this station departs from the boards before it, the divergence is named in copper rather than hidden.

## First movement: the whole control record, padded whole

The entrance is a control frame seventeen lanes wide, entered one lane at a time through the paired `org` and `nxt` registers carried forward from UPDATE-016. The entry frame is solid, the exit frame is outline, and the interpretation fixed at entry governs the lane until it clears.

Eleven tag silhouettes stand above the frame as chiplets. Selecting a tag lights the lanes that form actually uses — two lanes for the narrowest, seventeen for the widest — and the remaining lanes do not fade away. They fill with flat deliberate copper and a small tabular counter reads, for example, `tag 4 · used 5 · padded 12`. The padding rule is drawn as a single gate spanning the full width of the frame, because the rule speaks about the whole record at once: every unused lane, for every tag, is held at zero. No tag is drawn as an exception. One special lane near the end of one form carries its own narrower gate, drawn inside the whole-frame gate and labelled as a separate, tighter holding, so the visitor sees a rule within a rule rather than a footnote.

The DATA terraces from UPDATE-015 meet this frame at its floor. Their 128-cell lanes continue underneath unchanged, the derived 129-cell apron still extends by one where a start and a length are added, and the separate 130-bit gantry still stands beyond with its far side open. Padding lives in the control frame above; width lives in the apron below. The drawing never lets one explain the other.

## Second movement: composed temporary namespaces, kept apart

Beyond the frame, the board opens into five enclosed yards in a row. The nearest yard belongs to the record itself and is fenced in the source hairline. Behind it stand four further yards, one for each composed block the atlas already knows — data, helper, metadata, and the program-and-context pair from the previous station. Each yard has its own serial counter at its gate, drawn as a small tabular dial, and each mints its temporary names only inside its own fence.

A parent scope is drawn as the ground all five yards stand on. The ground itself is checked at the gatehouse before any yard opens: an empty ground, or a ground whose name already contains a reserved yard marker, does not open. Inside the yards, every operand arriving from outside is passed along a checkpoint rail. If an arriving name already begins with the parent ground followed by any yard's reserved marker, the rail stops it at the fence and stamps the tabular identifier on a refusal tab. The refusal is drawn as a stopped tile, never as a rerouted one, because the composition refuses rather than renames.

One emitter figure walks the row for the retained composition, carrying each yard's serial forward so repeated visits never mint a name an earlier visit already used. The single-visit path is drawn as a shorter spur that builds its own emitter, uses it once, and ends. The two paths are never merged into one line: retained composition and single use are different promises, and the board keeps them apart.

## Third movement: the snapshot and event relation at the host-record boundary

The final board is a boundary table at the ordinary host-record edge. Three panels stand in a fixed order: a before snapshot, an event panel, and an after snapshot. A bounded relation gate spans all three, checking the event for internal consistency and for agreement with the two snapshots that accompany it. The gate is drawn with a measured span on the 4 px spine and a hard end stop, because the relation reaches exactly as far as the three panels and no further.

Beneath the event panel, a plain caption in tabular figures carries the station's central honesty, written as visitor copy without qualification marks elsewhere on the board: consistency of a supplied record is not authenticated provenance. A public record arriving at this boundary is provided data. Provenance belongs only to the path that took the step itself, and that path is drawn on the far side of a ruled line as a single owner step with a non-rollback marker: once taken, the step stands even where a later check refuses, and a refusal is never drawn as an undo. The visitor who understands this board understands the difference between a record that agrees with itself and a record whose origin is vouched for.

Behind the boundary table, out of focus, stand the paired 22-lane rulers from UPDATE-016 — admission above, layout below, never merged — reminding the visitor that retained bounds frame this boundary without being absorbed into it.

## Alternatives considered

Padding drawn as empty space was considered and set aside: silence reads as absence, and this station exists to show unused lanes actively held. A single shared temporary yard with prefixed names was considered and set aside: a prefix can be read past, while a fence cannot. Drawing the boundary relation as a seal or stamp over the event panel was considered and set aside: a seal claims provenance, and this boundary claims only consistency. The recommended arrangement is the whole-frame padding gate, the five fenced yards with a checkpoint rail, and the three-panel boundary with its ruled provenance line, because each separation the design keeps is a separation the drawing keeps.

## Accessible and reduced-motion form

The reduced-motion form is a flat schematic on the same 4 px spine. The control frame becomes a table of seventeen lane rows per selected tag, with columns for used or padded and the holding applied. The five yards become five labelled lists, each with its own serial count and any refused identifier given as words. The boundary becomes three stacked tables — before, event, after — followed by the provenance caption as a full sentence. Keyboard order follows frame, yards, checkpoint rail, boundary, and evidence sockets. A screen reader announces, for example: 'Tag 4. Used lanes 5. Padded lanes 12, held at zero. Yard: metadata. Refused operand at fence.' No distinction is carried by hue, glow or motion alone.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, which goes live only once the full Sherlock goal is finished and accepted. It adds no site code, assets, shaders or deployment, and it asserts no physical circuit beyond what its own evidence channel shows. Engineering status, limits and attribution are kept in the separate status note, not inside the visitor copy. This asynchronous log never gates, blocks or speaks for the native programme.