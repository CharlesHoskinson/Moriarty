---
id: moriarty.research.sherlock.atlas.update022
title: UPDATE022 Wide Products and Bound Ports
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T16:37:22.922562+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-d23ee2de2634fede6675
---

# UPDATE-022 — Wide Products and Bound Ports

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose and rewrites no earlier chapter.
**Internal scope:** This header and the closing note are outside the visitor copy. The visitor copy describes the finished Moriarty Sherlock website, which goes live only when Sherlock is fully finished and accepted. It is not a claim about current source, native, proof or target status. UPDATE-021 and all prior chapters remain byte-identical.

## Finished visitor copy

The arithmetic room of UPDATE-021 does not end this journey. The camera in the finished Sherlock site pushes past its wide benches and stops over one multiplication, enlarged until its traces fill the board. What looked from the room like a single MUL footprint is, at this range, a full-width product: two checked 128-bit values enter, four partial products are formed, and the complete answer exists across both its low and high halves before Sherlock decides what may be staged.

The Moriarty wordmark stays flat and unlit at weight 600. Identifiers are tabular monospace and never reflow. The layout sits on the measured 4 px spine, copper marks divergence only, and the thirty-nine semantic selector footprints remain the floor: a footprint selects a meaning, it is not an implemented chiplet, and its glow never sends anything down a lane.

## The chosen journey: one product, then its ports

I have chosen a magnified cross-carry lattice followed by a bound-port wall, over a wider calculator or a residue display, because the lesson here is retention before projection. The visitor first watches width being made honestly, then watches how that staged result is bound to a statement without confusing a derived label for the constrained state itself.

**Stage one — two checked values arrive by different routes.** From the Second-phase entrance, the first value is the cached register value, bound at this station to its own source: the bridge re-reads the actual operand-A row and the register must equal the unsigned integer that checked read produces. The second value is read from the actual operand-B row. NOT keeps its separate First-phase path and reads operand A only. No absent operand-B lane is treated as a reference, and no lane borrows another lane's operand.

**Stage two — each 128-bit value splits into 64-bit limbs.** On the multiplication bench each operand opens into a low limb and a high limb, written in the drawing as x0 and x1 for the first value and y0 and y1 for the second. The split is shown as a physical widening of the trace, not as a new number typed in. Every partial product formed from these limbs stays below the field modulus, so the additions and multiplications on the bench are exact integer work.

**Stage three — partial products and the cross carry.** Four products are laid out in a grid: x0·y0, x0·y1, x1·y0 and x1·y1. The low product x0·y0 splits at 64 bits into a low piece and a carry c0. That carry joins the two cross products, and their sum splits again into a low piece and a second carry c1. The full product is then assembled in view: the low 128 bits below, and above them, on a raised copper tray, the complete high half x1·y1 + c1. The tray is the centre of this chapter. The high half is formed, named and retained before any projection is taken, so overflow evidence cannot vanish into a wrapped answer.

**Stage four — projection is a decision, not a truncation.** Only after the high half rests on its tray does the bench decide. If the high half is zero, the low 128 bits are the exact product and are staged as the value. If the high half is non-zero, no low residue is presented as success: primitive error 10 is staged instead. The wrapped low half remains visible on the bench as the thing that was refused, which is precisely why this room teaches checked multiplication rather than field arithmetic.

**Stage five — carry and borrow keep their own shapes.** Beside the lattice, the neighbouring benches are drawn at the same enlarged scale so their differences stay legible. At ADD, the exact sum is split at the 128-bit boundary and the carry beyond it stays as a single copper tab: one bit of overflow evidence. At SUB, one comparison serves as both the borrow and the less-than answer, and the difference is reconstructed with that borrow before its split: a borrowed 128 bits, not a wrapped residue. MUL retains a whole high half; ADD retains a carry; SUB retains a borrow. They are three distinct forms of evidence, and the drawing never flattens them into one overflow light. LT leaves by its own exit, staging a Boolean, true or false, and never an integer; NOT stages a negated Boolean or primitive error 11 for any other kind, and is never treated as an integer operation either.

**Stage six — the bound-port wall.** Past the benches, the staged result — value, Boolean, error 10 or error 11 — reaches a wall of bound ports, and here the finished site teaches how a step is stated. The same bounded relation can be bound in two modes. In the LocalPublic mode its lanes are carried as public statement lanes. In the LedgerCall mode the same relation is called with supplied post-state lanes and its output is those constrained post image, DATA and helper lanes themselves, with the commitment recomputed from the current statement and current post lanes on every check. Neither mode is a different arithmetic; they are two bindings of one relation, drawn as two sockets on one plate with the traces between them unchanged.

On that wall, three kinds of port are kept visibly apart. The constrained supplied post ports carry the actual post-state lanes that the relation constrains. Beside them, a narrow receipt strip shows derived values — the derived phase, register and staged tag and payload — as labels computed from the step, never as the constrained state and never as an authorisation token: a receipt that reads Done is a description, not the door it describes, and it is drawn recessed and unlit so no trace ever leaves it. Further along, two host routes arrive separately: an independent-post route, where a supplied post-state is carried to the checker exactly as supplied and is never silently repaired or regenerated from the pre-state; and an owner-record route, where a complete step record is checked for internal consistency between its event and its before and after snapshots — a coherence check of the record as presented, drawn as a sealed envelope inspected for matching seals, not as a stamp of who sent it. A step taken through the owner wrapper is drawn as already walked: if that route then refuses, the board shows the one step as taken, not rewound.

**Stage seven — staged is still not allocated.** The journey ends where UPDATE-021 ended, now seen from above the lattice. The staged result rests on its shelf: a tag and a payload, nothing more. No data node has been allocated, no result has been issued, and the helper is still present; allocation at the exit station and error issue at the failure station remain separate downstream work. The reference cursor has not moved. Two counters by the entrance still read apart: allocation at exact zero, and charged work risen by the step — zero allocation is not zero charged work. When the helper-work limit is already reached, the reads still happen in full view and the step closes exhausted for helper work: its outcome alone changes, no charge is taken, and the work, cursor, helper, data and allocation lanes stay exactly as they were.

## Alternatives considered

A single wide-multiplier block with only a low-half output was considered and set aside: it hides the high half whose retention is the whole lesson. A field-residue dial was considered and set aside: it teaches that a wrapped residue could pass as an answer. Drawing the derived receipt strip as a green Done gate was considered and set aside: it confuses a computed label with the constrained post-state and would make an authorisation out of a description. Presenting the two binding modes as two different circuits was considered and set aside: LocalPublic and LedgerCall bind the same relation, and duplicating the lattice would teach a difference that does not exist. The recommended arrangement is the one above — limb grid, retained high tray, distinct carry and borrow evidence, one relation behind two binding sockets, constrained post ports separated from the recessed receipt strip, and the staged shelf before any allocation.

## Three.js, PCB treatment and accessible form

In the explorable Three.js scene the lattice is built up in the order above: traces widen into limbs, the four partial products light as tiles, c0 rises into the cross row, c1 rises to the high tray, and only then does the projection gate read the tray. Copper is used only where paths diverge — the high-tray branch, the borrow reconstruction and the receipt strip's separation from the post ports. Motion is a single slow build-up; nothing rotates, pulses for decoration, or carries meaning by glow alone.

The flat, reduced-motion and screen-reader form renders the same journey as seven stacked sections on the 4 px spine. Each section lists its components in tabular text: the register value bound to its checked operand-A source beside the checked operand-B read; each operand split into low and high 64-bit limbs; the four partial products with carries c0 and c1; the high half x1·y1 + c1 retained before projection, staging the low value when it is zero and primitive error 10 otherwise; ADD's carry and SUB's borrow stated as distinct evidence, with LT and NOT staging Booleans or error 11 and never integers; the two binding modes stated as one relation, with LedgerCall outputting the constrained supplied post lanes under a recomputed commitment, the derived receipt stated as a label and not the constrained state, an independently supplied post stated as unrepaired, and an owner record stated as internally checked rather than authenticated; and the staged shelf stated as preceding allocation, with allocation at exact zero apart from charged work. Keyboard order follows the build-up order. A screen reader announces, for example: 'Stage three. Mode MUL. Cross carry formed. High half is non-zero. Low half is retained and refused. Staged result is primitive error 10. Nothing allocated.' No distinction is carried by hue, glow or motion alone.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, live only once the full Sherlock goal is finished and accepted. It adds no site code, assets or deployment. Engineering status is kept in the separate dated note, not in the visitor copy.