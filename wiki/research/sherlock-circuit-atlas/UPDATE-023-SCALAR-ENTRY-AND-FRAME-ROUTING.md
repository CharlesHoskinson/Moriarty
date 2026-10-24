---
id: moriarty.research.sherlock.atlas.update023
title: UPDATE023 Scalar Entry and Frame Routing
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T17:37:31.400771+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-13a910cb3b18189f7bd6
---

# UPDATE-023 — Scalar Entry and Frame Routing

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback. Earlier Opus labels are historical; this is Muse prose and rewrites no earlier chapter.
**Internal scope:** This header and the closing note stand outside the visitor copy. The visitor copy describes the finished Moriarty Sherlock website, live only when Sherlock is fully finished and accepted. It makes no claim about current source, proof, native or target status. UPDATE-022 and all prior chapters remain unchanged.

## Finished visitor copy

Past the bound-port wall of UPDATE-022, the finished Sherlock site does something quieter and more revealing: it stops moving and installs help where the work actually is. The visitor looks down a physical stack of frames, laid bottom-to-top on the board, and watches the topmost frame — the only frame that matters here — meet a single Return control. From those two real sources, and from nothing else, a helper record is born already knowing where it came from, which operands it will later examine, and what type token it may later test. Nothing is computed yet. That restraint is the lesson.

The Moriarty wordmark stays flat at weight 600. Identifiers are tabular monospace and never reflow. Every element sits on the 4 px spine. Copper marks divergence only. The thirty-nine footprints on the floor remain semantic selectors: they select meanings, they are not thirty-nine completed chiplets.

## The chosen journey: the Return landing

I have chosen a Return landing with an installation gantry, over a mode-picker dial or a floating helper badge, because entry must be visibly sourced. There is no site label hovering above the scene waiting to be believed. The Return control itself supplies the returning reference lane, and the dynamically selected top frame supplies its own stored lanes. The gantry reads those actual lanes, routes exact copies into an initially empty helper socket, and leaves the frame, control and environment standing exactly where they were.

**Stage one — find the real top.** The frame stack is drawn as a physical vector, bottom at the floor and top at the gantry. The selected frame is the one at index `frame_count - 1`: a dynamic position, not a hard-coded shelf. If the stack grew by one overnight, the gantry would read the new top, because it follows the count. The visitor sees the counting finger climb the stack and stop one below the count — never at a painted marker.

**Stage two — the Return control supplies its reference.** Beside the stack stands the Return control, carrying its actual value reference as two lanes: a space and an index. Those lanes are the source, not a paraphrase. At RetRight the returning reference becomes operand B, present and exact. At the unary, conditional and type-testing sites it becomes operand A. In every case the copied space and index are the Return control's own, so a visitor can trace one unbroken copper-free path from control to helper field and read the same numbers at both ends.

**Stage three — eleven shapes through five doors.** The top frame opens five doors, and through them pass eleven shapes making ten distinct modes. At the RetRight door, six shapes arrive — Add, Sub, Mul, Lt, Cons and Branch. For each, operand A is the top frame's stored left reference, copied exactly by space and index, and operand B is the actual Return reference. Eq is pointedly not among them: equality belongs to Equal, and the gantry turns it away from this door. At the RetUnary door, Leaf and Not arrive with operand A taken from the Return reference and operand B absent — an empty socket, drawn as an empty socket. At RetIf, the shape becomes the TestBool mode, again with the Return reference as sole operand. At RetCall, a Return frame carrying a return type becomes TestType, with the expected slot filled by that exact type identifier. And at the fifth door, RetCtorFields, a constructor-fields frame with its checked flag still false becomes TestType with the expected slot filled from the frame's exact field type. TestType therefore appears at two sites, which is why eleven shapes make only ten distinct modes. At both type doors the token is copied whole — built-in identifiers 0 to 3 and nominal identifiers from 4 upward alike — never replaced by a UInt-only plug.

**Stage four — the fifth door does less than it could.** The RetCtorFields door is drawn deliberately plain. Entry copies the field type; it does not run a new arity checker and it does not evaluate the type test. A frame whose checked flag is true never reaches this door at all: it selects Append, a different journey shown elsewhere on the board. The visitor learns that installation is faithful copying from a checked source read, not a fresh judgment smuggled in at the doorway.

**Stage five — one helper, newly occupied.** Before this stage the helper socket is empty, and the board says so. After it, the socket holds one Scalar helper. Its site is derived from the actual top frame just read — no host flag invents it. Its origin reference cursor is copied exactly from the current cursor. It has no parent resume. Its phase is First, its register A holds 0, and its staged slot is None: nothing tested, nothing computed, nothing held. This is operand-reference and type-token installation. The values those references point at have not been checked, combined or allocated here; Tag, Arithmetic, Construct and Exit remain separate stations downstream, each with its own relation.

**Stage six — everything else stands still.** The camera holds. The top frame is not popped; the full frame — its type, constructor, index, completed fields, rest, saved environment and origin — remains intact on the stack. Control does not advance. The environment does not move. The reference cursor does not advance: the helper records where the cursor was, the cursor itself stays. No data node is allocated. A pair of counters by the gantry tells the cost honestly: charged work rises by one — remaining falls by one, consumed and helper work each rise by one — while the helper-visit counter does not move at all and allocation stays at zero. Zero visits is not zero work, and zero allocation is not zero charge; three different facts, three different dials.

**Stage seven — the refusal, drawn honestly.** If the helper-work limit has already been reached under its own separate admission bound, the landing closes exhausted for helper work. Only the outcome changes. The helper socket stays empty, and every counter stays exactly as it was: no charge is taken for a refusal. A separate, visibly different administrative closure covers a machine with no fuel at all; it is not this door's refusal and is never labelled as one. Beside the gantry, a small receipt strip names the shape that would have been installed — but it is recessed and unlit, because it describes the applied shape even when an installation was refused or the relation disabled. Only the constrained lanes inside the helper record itself are the post-state; the receipt is a label, and no trace ever leaves it.

## Alternatives considered

A floating mode badge, chosen from a dial and then attached to a helper, was considered and set aside: it would make the site and mode look asserted rather than sourced. Drawing eleven doors for eleven supposed modes was considered and set aside: TestType serves two sites, and inventing an eleventh mode teaches a false count. Collapsing the stack to its top frame alone was considered and set aside: the dynamic `frame_count - 1` selection from a bottom-to-top vector is itself worth seeing. Performing the type test at the fifth door was considered and set aside: Entry copies the token, and evaluation belongs downstream. The recommended arrangement is the one above — a counted climb, exact reference and type-token routing from the actual Return and top-frame lanes, one helper installed in First with register A at zero and nothing staged, a stationary frame, control, environment and cursor, and a refusal that changes only the outcome.

## Three.js, PCB treatment and accessible form

In the explorable Three.js scene the build-up follows the stage order: the counting finger climbs the frame vector to `frame_count - 1`, the Return reference lanes light as a traced pair, the five doors open in place, exact space-and-index copies travel into the helper fields, the expected token arrives whole at the two TestType doors, and the helper socket fills while frame, control, environment and cursor visibly hold still. Copper is used only where paths diverge — Eq turned toward Equal, a checked constructor frame turned toward Append, and the receipt strip's separation from the constrained record. Nothing rotates or pulses for decoration, and no meaning is carried by glow alone.

The flat, reduced-motion and screen-reader form renders the same journey as seven stacked sections on the 4 px spine, each listing its components in tabular text: the dynamic top at `frame_count - 1`; the actual Return reference as space and index; the five sites, eleven shapes and ten distinct modes with TestType named at two sites; RetRight taking operand A from the frame's left reference and operand B from the Return reference, with Eq routed to Equal; RetUnary, RetIf, RetCall and unchecked RetCtorFields taking the Return reference as operand A with operand B absent; expected type identifiers copied exactly, built-in and nominal; the helper installed as Scalar with site derived from the actual frame, origin cursor copied, no parent resume, phase First, register A 0 and staged None; no frame pop, control, environment or cursor advance and no allocation; work charged by one with exactly zero helper visits; a helper-work refusal changing only the outcome to exhausted, with the helper still absent and no charge; and the derived receipt stated as a label, never as the constrained post-state. Keyboard order follows the build-up. A screen reader announces, for example: 'Stage five. Site RetCall. Mode TestType. Operand A is the Return reference. Expected type copied exactly. Helper installed at phase First. Nothing value-checked, computed or allocated yet.' No distinction is carried by hue, glow or motion alone.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, live only once the full Sherlock goal is finished and accepted. It adds no site code, assets or deployment. Engineering status is kept in the separate dated note, not in the visitor copy.