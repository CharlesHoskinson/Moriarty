# Wiki Index

Every page in the wiki. Rebuild with `python3 llm-wiki/tools/lint.py --write-index` after each ingest.

## Overview

- [Overview](overview.md): where the program is, in ten lines, with links
- [Log](log.md): append-only rolling log of every finished workstream step

## Sources (46)

- [AGENTS.md factory charter](sources/agents-charter.md): The charter of the DoctorWatson factory that develops Moriarty.
- [Sherlock co-processor animation, version 2 notes](sources/animation-v2-notes.md): Working notes for version 2 of the Sherlock animation, started at the owner's request.
- [Astra audit of the architecture merge (B1)](sources/arch-audit-astra.md): Read-only audit by GPT-6 Astra of the B1 text of `ARCHITECTURE.md` (2,776 lines before revision B2; sherlock-architecture), at baseline commit `b85b00213`.
- [Response to the architecture audit (B2)](sources/arch-audit-response.md): Night-plan step B2.
- [Draft: area 1 processor modules (Astra)](sources/astra-area1-processor-draft.md): Draft, not adopted.
- [Draft: area 2 memory and values (Astra)](sources/astra-area2-memory-draft.md): Draft, not adopted.
- [Draft: area 3 recursion and system architecture (Astra)](sources/astra-area3-recursion-draft.md): Draft, not adopted.
- [Experiment C1: dense tail-buffered memory backend (measured)](sources/c1-dense-memory-experiment.md): First backend of night-plan step C1.
- [Review: C6 outcome-only publication and ledger 9 fit](sources/c6-outcome-ledger9-review.md): Run `sherlock-c6-outcome-ledger9-review-20261110`.
- [Chip spike experiments E0 to E8](sources/chip-spike-experiments.md): Four sessions of run `sherlock-processor-analogy-20261110` (2026-11-10) that sized the proposed chip leaf in `crates/mori-chip`.
- [Knowledge graph of the Compact repository](sources/compact-graph.md): A graphify knowledge graph of the Compact compiler repository (`repos/LFDT-Minokawa/compact`, the graph root recorded in `.graphify_root`), built 2026-11-11 for the Compact...
- [Cost round, Astra lane: Sherlock cost optimization](sources/cost-lane-astra.md): The GPT-6 Astra answer to cost-round-brief, at repository snapshot `4b5697f7d`.
- [Cost round, Fable lane: making Sherlock competitive with native proving](sources/cost-lane-fable.md): The Claude Fable 5.2 answer to cost-round-brief.
- [Cost round, Gemini lane: ratio decomposition, levers and GPU multicore scaling](sources/cost-lane-gemini.md): The Gemini 4 Carbon high answer to cost-round-brief.
- [Cost round brief: making Sherlock competitive with native proving](sources/cost-round-brief.md): The owner rejected an unmeasured "1,000 to 10,000 times cheaper" claim and asked how to optimize the design to reduce the gap.
- [Council synthesis: does the Sherlock design need improvement?](sources/council-findings.md): The pipeline-animation council checked the final design (`ARCHITECTURE.md` at head `680d2b5af`) while the animation was built.
- [Revisions applied from the council: P-1 to P-10](sources/council-revisions.md): The work order of council-findings, applied as design text to `ARCHITECTURE.md`, `DECISIONS.md` and the OpenSpec change `sherlock-modular-leaf-20261111`.
- [Design 2.7: leaf as a chip (proposed, not adopted)](sources/design-2-7-chip-leaf.md): Section 2.7 of the Sherlock VM design proposes the universal leaf as one hand-written circuit: a CPU core plus the chips DEC, PAGE, MEM, ALU with RANGE, and HASH, reached...
- [Dissent record: C6 outcome-only review section](sources/dissent-c6-section.md): The section "C6 outcome-only review (2026-11-10)" of `dissent.md` records the dissent and open items of c6-outcome-ledger9-review.
- [Experiments E2 to E4: node headroom, genesis cost, k=20 leaf](sources/e2-e4-experiments.md): Experiments of the recursion design review, run through `scripts/doctorwatson heavy` on 2026-11-10.
- [Elastic chip round brief: the elastic multicore Sherlock chip and its prover](sources/elastic-chip-brief.md): The owner asked for one more chip iteration: an elastic multicore chip that scales to the host's GPU and CPU cores, with the prover developed hand in glove with the chip.
- [Elastic chip round, area D, Fable lane: the GPU proving backend](sources/elastic-chip-gpu-fable.md): The Claude Fable 5.2 answer for area D of elastic-chip-brief.
- [Lead decisions overnight 2026-11-11 (pending owner review)](sources/final-design-decisions.md): Lead decisions, not owner rulings.
- [Draft: area 1 processor modules (Gemini)](sources/gemini-area1-processor-draft.md): Draft, not adopted.
- [Draft: area 2 memory and values (Gemini)](sources/gemini-area2-memory-draft.md): Draft, not adopted.
- [Draft: area 3 recursion and system architecture (Gemini)](sources/gemini-area3-recursion-draft.md): Draft, not adopted.
- [General-purpose recursion definition (GP-1 to GP-4)](sources/general-purpose-definition.md): The owner's definition of the target for the Sherlock recursion, recorded on 2026-11-10.
- [GP architecture study (sherlock-gp-architecture-20261110)](sources/gp-architecture.md): Brief ingest of section 1 only (the recommended architecture); sections 2 to 6 (options, dissent, migration, experiments, proposed text) are not summarised here.
- [Recalibration round brief: the Sherlock chip at k = 21](sources/k21-recalibration-brief.md): The owner moved the node and the decider to k = 21 and asked for the design and every optimization, lookups included, to be recalibrated, then for six named optimizations to be...
- [land-3x merge: dispatch partition and U023c receipt](sources/land-3x-merge.md): Main commit `e39bc7af8` (2026-11-10 23:08 local) merges branch `land-3x`: the dispatch partition, the dispatch receipt...
- [Audit of the lookup cache consensus (Fable 5.2)](sources/lookup-cache-audit.md): Read-only audit (no build, test, proof or model run) of lookup-cache-consensus at checkout `9973d633b`, on the final file of 84,457 bytes.
- [Brief: lookup cache study for the chip leaf](sources/lookup-cache-brief.md): Status: done.
- [Lookup cache consensus (five Gemini 4 Carbon High lanes)](sources/lookup-cache-consensus.md): Round 3 synthesis of run `sherlock-lookup-cache-20261111` over lanes A to E, which answers the brief lookup-cache-brief.
- [N1: native Compact circuit sizes](sources/n1-native-circuits.md): The lead's first cost-round measurement (cost-round-brief): `compactc` 0.36.0 and `zkir-v3 mock-compile` on the `tiny`, `election` and `zerocash` examples on this host.
- [N1b: native Compact proving time per entry point](sources/n1b-native-proving-time.md): The lead's second cost-round measurement (cost-round-brief): real proofs with honest witnesses for all ten entry points of n1-native-circuits, on the ledger 9 proof-server pin,...
- [OpenSpec change sherlock-modular-leaf-20261111 (implementation spec)](sources/openspec-modular-leaf.md): The implementation spec of the modular Sherlock leaf and recursion.
- [Knowledge graph of the Midnight prover stack](sources/prover-graph.md): A graphify knowledge graph over `repos/midnightntwrk` (the graph root): `midnight-ledger` on branch `ledger-9` with the proof server, `midnight-zk` (proofs, circuits, curves,...
- [Quint verdict: final chip leaf (packed selectors, RANGE, M3)](sources/quint-chipleaf-final.md): Lead verdict on three Gemini Quint lanes of 2026-11-10 over `crates/mori-chip/src/` at main `f59ef318f`.
- [Quint verdict: Poseidon-only HASH chip](sources/quint-chipleaf-hash.md): Lead verdict on the Gemini Quint lane of 2026-11-10 for the Poseidon-only HASH chip (`crates/mori-chip/src/poseidon.rs` and its bus use) and constraints D1, D4 and D8 of design...
- [Quint verdict: layout B pair rows and M3 aligned segments](sources/quint-chipleaf-pairrow-m3.md): Lead verdict on the Gemini Quint lane of 2026-11-10 for layout B pair rows and the M3 aligned-segment memory root (C = 8).
- [Per-rule memory maxima and the one-arena non-interleaving check](sources/rule-maxima-experiment.md): Experiment asked for by decisions R-6, L-2, L-9 and E-3 (final-design-decisions), run-length audit finding 6 (runlength-consensus-audit) and lookup audit blocker 1 and finding...
- [Run-length consensus (sherlock-runlength-consensus-20261111)](sources/runlength-consensus.md): Final report of the five Gemini 4 Carbon High lanes (A to E, two rounds) on the run-length bound, the D3 memory ceiling and continuations, asked for in runlength-consensus-brief.
- [Audit of the run-length consensus (Fable 5.2)](sources/runlength-consensus-audit.md): Fable 5.2 audit of runlength-consensus, read-only at `9ffc2fe76` (no build, test, proof or model run).
- [Brief: run length and memory ceiling consensus](sources/runlength-consensus-brief.md): Brief of run `sherlock-runlength-consensus-20261111`.
- [Architecture of the modular Sherlock leaf and recursion (revision B2)](sources/sherlock-architecture.md): `ARCHITECTURE.md` of run `sherlock-final-design-20261111`, night-plan step B1, revised as "revision B2" after the Astra audit (arch-audit-astra, arch-audit-response).
- [Sherlock terminology](sources/sherlock-terminology.md): The category name and the term table for Sherlock, written at the owner's request ("We need some terminology", then "don't lose the terminology").

## Entities (5)

- [acceptRun contract](entities/AcceptRunContract.md): The Compact contract `tools/sherlock-sdk/sherlock-run.compact` whose `acceptRun` circuit accepts a run on Midnight (c6-outcome-ledger9-review).
- [IVC node](entities/IvcNode.md): The self-recursive node of the Sherlock recursion (`crates/mori-recursion/src/ivc.rs`).
- [Midnight ledger 9](entities/MidnightLedger9.md): The target ledger, read at tag `ledger-9.1.0.1` (`a2b30128`).
- [mori-chip crate](entities/MoriChip.md): `crates/mori-chip` holds the chip leaf spike: `rf.rs` (32-word register file and spike rules), `rfcore.rs` (layouts and leaf), `poseidon.rs` (Poseidon-only HASH chip),...
- [Run decider](entities/RunDecider.md): The terminal proof of a run (`crates/mori-recursion/src/run_decider.rs`).

## Concepts (11)

- [Block cache: 8-record PAGE and ROM cache lines in the chip leaf](concepts/BlockCache.md): The "processor cache" the owner asked for (lookup-cache-brief).
- [C6 outcome-only publication](concepts/C6OutcomeOnlyPublication.md): Owner decision, 2026-11-10: "go with publishing only the outcome for C6 after gemini carbon and fable and astra review that".
- [Chip leaf](concepts/ChipLeaf.md): The proposed universal Sherlock leaf: one hand-written circuit with a CPU core and the chips DEC, PAGE, MEM, ALU with RANGE, and HASH, reached through one lookup bus of tuples...
- [Continuations (stop and resume of one run)](concepts/Continuations.md): How a long run is proved in pieces, and across proving sessions, under one key set and with one public statement.
- [Dense tail-buffered memory (replacing D3)](concepts/DenseTailMemory.md): The memory design direction that replaces D3's padded aligned segments.
- [General-purpose recursion: GP-1 to GP-4](concepts/GeneralPurposeRecursion.md): Owner ruling, 2026-11-10: general purpose and universal means all four properties below.
- [Modular architecture: module kits, seams and build order](concepts/ModularArchitecture.md): How the Sherlock leaf and recursion are split so that one agent builds and checks one module against a frozen interface.
- [Owner rules](concepts/OwnerRules.md): Standing rules from the factory charter and the owner's GP definition.
- [Run-length ceiling (D3 against GP-1)](concepts/RunLengthCeiling.md): The conflict between the proposed chip leaf memory (D3) and the owner's GP-1.
- [Value digest sherlock-value/1](concepts/ValueDigest.md): The canonical value encoding of the C6 review, used for both `input_d` and `outcome_d` (SRC-022, SRC-021 amended) (c6-outcome-ledger9-review).
- [Zero knowledge end to end](concepts/ZeroKnowledge.md): Owner ruling, 2026-11-11: "we need to make this zero knowledge or else its useless" (k21-recalibration-brief).

## Syntheses (3)

- [Animation version 2: running record](syntheses/AnimationV2.md): Owner, 2026-11-11: "keep a log in wiki-llm for version 2 of the animation that reflects all the new design and the compact and ZK compatibility".
- [Open owner questions](syntheses/OpenOwnerQuestions.md): Questions the owner must settle.
- [Workstream status](syntheses/WorkstreamStatus.md): State as of 2026-11-11 14:10 local (cost, Compact graph, council and elastic chip rows added).
