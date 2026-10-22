---
id: moriarty.research.sherlock.atlas.update027
title: UPDATE027 Phase Clocks and Preserved Accumulators
type: overview
status: proposed
created: 2026-10-22
updated: 2026-10-22
tags: [moriarty, sherlock, circuit-concept]
---

# UPDATE-027 — Phase Clocks and Preserved Accumulators

Concept proposal for the user-requested Moriarty Tron/PCB/Three.js website. Author: GPT-6.1 Sol, under this explicit documentation assignment. The inspected published atlas ends at UPDATE026; this is a new chapter, not a replacement for its designated-author fields. No scene, animation or asset has been implemented or deployed. The website WILL go live after full Sherlock completion; it is unpublished today.

## The phase rail

Place a narrow timing rail beside the existing circuit board. Keep the flat Moriarty wordmark, dark PCB substrate, steady etched traces and fixed-width technical labels. A visitor can scrub the source-defined milestones while the lower circuit layer remains still: diagnostic time belongs to host execution, not to a new constraint wire.

The ten stops use the actual Source44 enum order: `model`, `child_a102_verify`, `child_a80_verify`, `accumulator_fold`, `folded_pairing`, `setup_pk`, `prove`, `stdlib_verify`, `pairing_verify`, `write`. Each stop has separate start and complete sockets. A source-only view shows unlit sockets. A future evidence view may illuminate only markers present in an exact captured run; missing markers stay unknown. No looping animation invents an elapsed time or completes an unfinished stage.

A thin ruler shows cumulative monotonic seconds from the process-local clock. Selecting two observed boundaries exposes their difference; do not add cumulative readings as if they were independent durations. The clock starts after earlier witness/SRS/key checks, so it is not whole-operation wall time. Keep `setup_pk` and `prove` as broad chambers: the current source exposes no internal FFT/MSM/quotient/opening clocks. The `prove` completion includes key destruction. A completed `write` boundary means the existing file-write calls returned; it does not promise an atomic directory publication.

At the last observed socket, a timeout ends the evidence ribbon in an open gap. The circuit traces do not flash red to claim proof rejection, and the remaining route does not turn green. Diagnostic output can fail independently; no marker is not proof of no execution. Source43's observed k20 VK and its timed-out parent attempt stay on separate dated cards. They never become a completed Source44 proof.

## Originals stay on the fold track

Beside each child bay, show two transparent containers labelled `lhs` and `rhs`. Each holds three visibly separate structures: an ordered row of bases, an ordered row of scalars, and a drawer of complete fixed-base name/coefficient pairs. Use symbolic public labels; expose no key, proof, witness or private instance payload.

The incoming child first passes its unchanged stdlib verification. Preparing its own accumulator places the original containers on a straight central track. A side branch duplicates both containers with a clear `Clone` seam. Only that copy passes through `resolve_fixed_bases`, then `collapse`, then the pairing check. Resolution visibly moves named entries into the variable rows; collapse changes their representation. The untouched originals remain in view alongside the transformed copy, making the preservation claim inspectable rather than merely showing two equal points.

After successful child checks, A102 enters the ordered fold before A80. A translucent transcript strip reads the original ordered encodings before the merged fixed bases are resolved and collapsed. Selecting a named drawer highlights it in both the fresh-reference and retained-original view; selecting lhs never silently selects rhs. Reversing the children is a comparison control, not an alternative accepted route. A caption states that equal evaluated points or equal flattened `acc_pis` do not establish equality of full ordered containers and named maps.

The final parent pairing, actual proof, stdlib verification, carried-accumulator verification and output writing remain separate stops. Neither a checked clone nor a passed source test lights the final proof/target lamp.

## Source and evidence boundary

This proposal is grounded in Source44 R2's `ParentProofPhase`, `parent_proof_phase_record`, `prove_link_parent`, `write_w_dir`, original-own helper and strengthened structural assertions. Those assertions inspect both lhs/rhs bases, scalars and complete named maps before and after mutation of the checked clone. They are source until the real selected consumer runs them.

The first restarted cycle has ordinary classifier/wrapper passes and finite Quint observations. Sampled target, benchmark and complete witnesses are zero; the directed positive chain supplies external premises. Clean's new correspondence obligations and K's 14 join laws remain unproved/unrun at their stated cuts. The actual read-only wallet observation remains UNKNOWN and authorizes no resend. None supplies native phase markers or visitor-facing proof success.

Keep these engineering qualifications outside the eventual finished-site journey. Preserve all earlier creative bytes, UPDATE026's five authored fields, prior failures, and the full Sherlock/financial/formal/target acceptance requirements. The concept is ready for the existing publication/review lane; it creates no new approval or execution mechanism.
