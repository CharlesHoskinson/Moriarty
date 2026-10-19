# DESIGN013 — Opus concept-note prompt

Write one dated, standalone iteration note for the standing Sherlock circuit website concept log. Follow DESIGN012’s note structure: date and scope, source-grounded observations with exact path/hash/symbol anchors, visual concepts, alternatives, recommendation, status boundary, and open questions. Do not revise DESIGN012 or earlier notes.

## Audience and purpose

The note should help a designer and circuit author imagine a beautiful, explorable Sherlock circuit site with a Tron-inspired layered-PCB aesthetic and Three.js circuit boards. Keep the Moriarty wordmark and established typography recognizable. Use the Tron reference as visual language only: no franchise logo, typeface, character, or copied set design. The finished website goes live when it is fully finished. This concept note is construction documentation; it does not implement, publish, or launch the site.

## Public facts to use

Use only the public source and metadata pins listed below. Do not inspect, quote, reproduce, or infer from any private capture, oracle, expected, actual, SAFE, proof, or witness body. Do not reveal payload bytes.

- PR297 is merged to Sherlock at `35931362ca435b2ed10014ee052870716c8f10d3`; its exact publication map contains 11 source paths plus 72 archived paths. Treat that as source integration/publication status.
- In the earlier frozen worktree, the cached-F native guard compile passed and the 17 selected native calls passed: 16 A80 checks plus one K public-source capture check. The selected cohort was 17 from a 145-test listing with zero benchmarks. Do not describe this as canonical main compile/test: the PR297 record says canonical compile/tests are UNRUN.
- The K capture metadata records a statement length of 335 and says the independent oracle was not issued. The first direct oracle rejection (327 versus 335) is parent-reported only; a separate no-tools constructor review was not generated and source audit 73890 remains OPEN per parent status. Private oracle inputs and outputs are excluded. Do not call the 335 result correct or independently matched.
- The Sherlock context graph records 39 semantic rule IDs. Present a set of 39 semantic rule selectors as navigable rule concepts. State clearly that 39 is not a count of physical circuits, proofs, chiplets, or constraints. Do not invent a physical circuit count.
- The fresh route source/API review has mandatory source findings F1 (SRS pin), F2 (captured-key path), and F3 (effects snapshot ordering). The route correction is not yet approved. Keep source review, compilation, native tests, proof generation, and ledger readback as distinct states.

## Concept to develop

1. **Bootstrap / rule selector board.** Sketch how the 39 semantic rule selectors could be explored as labels and navigation. They are semantic rules, not circuit inventory. Keep unimplemented or unmeasured machinery visually quiet.
2. **Two-step 335-lane image trace.** Propose a two-step visual mapping from named source fields to 335 indexed statement lanes, then from those lanes to an image-shaped board view. Show this as a trace of structure only. Never imply that field positions are physical gates, that captured values are correct, or that an oracle comparison passed. Avoid displaying private payloads.
3. **Construction sequence.** Show five linked stages: `source → compile → 17 actual checks → real recursive proof → ledger readback`. Mark the current evidence boundary on each stage: PR297 source integration is merged; the earlier worktree compile and 17 selected checks passed; canonical compile/tests are UNRUN; a real recursive proof and final ledger readback are not established by these pins. The last two stages must remain visibly open. Do not draw a proof, chain settlement, or readback as completed.
4. **Three.js / PCB visual language.** Describe a dark, layered board with carefully routed traces, via depth, a static exploded overview, and inspectable source-linked objects. Use shape and labels to distinguish semantic rules, source, compile, test, proof, and target readback. Emission can mark selection or measured activity only; never animate proof generation or imply a running measurement.

## Boundaries

- Use complete public source declarations only if a separately supplied public source snapshot is provided; this prompt supplies metadata only. Cite exact file, SHA-256, and symbol when making a source claim. If a detail is not in the pins, label it a concept or leave it open.
- Do not claim A80, Sherlock, recursive proving, or ledger settlement is complete. “A80 checks passed in the earlier worktree” and “canonical compile/tests UNRUN” must remain separate.
- Do not generate code, mockups, assets, site files, or Three.js output. Do not make provider, SDK, native, Git, wallet, network, canonical-vault, publication, or deployment calls.
- Preserve the existing concept-log format and release policy. The finished site goes live when fully finished; keep construction observations factual.
- Keep the note concise, visually concrete, and explicit about what is observed versus proposed. Return only the draft note and a short source-pin list.

## Metadata and format source pins

- Frozen graph: `graphs/sherlock/graph.json`, SHA-256 `396e26319a2a70ddb0466956f74e90055569b48723e623e4d8d72aeef53821e4` (Graphify-ready record).
- PR297 publication result: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-C4-A80-Kcapture-source-publication-current-sherlock-20261019/revision02-actual-eighteen-evidence/root-publication-condition-checks-01/FINAL-PUBLICATION-RESULT.json`, SHA-256 `b907939699002bf0fbcb0be820748122cc11d40f68fb0fb41b7d031180298ac9`.
- Prior actual native frontier overlay: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-design-simplification-sprint-20261019/graphs/sherlock/frontier-delta-20261019-actual-17-tests-route-author-03/FRONTIER-DELTA.graph.json`, SHA-256 `9f257ab737176a4cac56e7d60ec358d5121a735f5d9a661eacbc9539708b1e04`.
- Route review: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/next-sherlock-compact-anchor-template-route-owner-request-20261019/anchor-only-critical-leaf-source-request-02/source-candidate-01/coherent-capture-route-source-candidate-01/mandatory-source-correction-owner-request-02/revision02-complete-declarations/source-candidate-01/fresh-nonauthor-source-api-review-01/revision02-complete-communication-consumer/actual-opus-full-source-api-and-JS-operation-review-01/FINAL-RESPONSE.txt`, SHA-256 `d1cbaa616820319f2fbc57996d39917781ece1a84a6353d9a46f348d48f57fcf`.
- Existing concept-log format and release policy: DESIGN012 README `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dw120-complete-declarations-and-design012-author-prep-20261019/design012-publication-01/revision02/afterimages/wiki-llm/sherlock-circuit-atlas/README.md`, SHA-256 `3484e0444b32d2fcd2f676f2260fd90cf80a1f3f33c0823f281a33dcda9bc82a`; current sealed note `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dw120-complete-declarations-and-design012-author-prep-20261019/design012-publication-01/revision02/afterimages/wiki-llm/sherlock-circuit-atlas/DESIGN-012-SEALED-HANDOFF-AND-COMPLETE-STATEMENTS.md`, SHA-256 `ab0f25d5e013baca1b4727d903ebd099d26483f21f6653acc17996974e94c602`.
