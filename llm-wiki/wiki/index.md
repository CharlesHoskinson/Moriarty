# Wiki Index

Every page in the wiki. Rebuild with `python3 llm-wiki/tools/lint.py --write-index` after each ingest.

## Overview

- [Overview](overview.md): where the program is, in ten lines, with links
- [Log](log.md): append-only rolling log of every finished workstream step

## Sources (20)

- [AGENTS.md factory charter](sources/agents-charter.md): The charter of the DoctorWatson factory that develops Moriarty.
- [Draft: area 1 processor modules (Astra)](sources/astra-area1-processor-draft.md): Draft, not adopted.
- [Draft: area 2 memory and values (Astra)](sources/astra-area2-memory-draft.md): Draft, not adopted.
- [Draft: area 3 recursion and system architecture (Astra)](sources/astra-area3-recursion-draft.md): Draft, not adopted.
- [Review: C6 outcome-only publication and ledger 9 fit](sources/c6-outcome-ledger9-review.md): Run `sherlock-c6-outcome-ledger9-review-20261110`.
- [Chip spike experiments E0 to E8](sources/chip-spike-experiments.md): Four sessions of run `sherlock-processor-analogy-20261110` (2026-11-10) that sized the proposed chip leaf in `crates/mori-chip`.
- [Design 2.7: leaf as a chip (proposed, not adopted)](sources/design-2-7-chip-leaf.md): Section 2.7 of the Sherlock VM design proposes the universal leaf as one hand-written circuit: a CPU core plus the chips DEC, PAGE, MEM, ALU with RANGE, and HASH, reached...
- [Dissent record: C6 outcome-only review section](sources/dissent-c6-section.md): The section "C6 outcome-only review (2026-11-10)" of `dissent.md` records the dissent and open items of c6-outcome-ledger9-review.
- [Experiments E2 to E4: node headroom, genesis cost, k=20 leaf](sources/e2-e4-experiments.md): Experiments of the recursion design review, run through `scripts/doctorwatson heavy` on 2026-11-10.
- [Lead decisions overnight 2026-11-11 (pending owner review)](sources/final-design-decisions.md): Lead decisions, not owner rulings.
- [General-purpose recursion definition (GP-1 to GP-4)](sources/general-purpose-definition.md): The owner's definition of the target for the Sherlock recursion, recorded on 2026-11-10.
- [GP architecture study (sherlock-gp-architecture-20261110)](sources/gp-architecture.md): Brief ingest of section 1 only (the recommended architecture); sections 2 to 6 (options, dissent, migration, experiments, proposed text) are not summarised here.
- [land-3x merge: dispatch partition and U023c receipt](sources/land-3x-merge.md): Main commit `e39bc7af8` (2026-11-10 23:08 local) merges branch `land-3x`: the dispatch partition, the dispatch receipt...
- [Pending study: lookup cache for the chip leaf](sources/lookup-cache-brief.md): Status: pending.
- [Quint verdict: final chip leaf (packed selectors, RANGE, M3)](sources/quint-chipleaf-final.md): Lead verdict on three Gemini Quint lanes of 2026-11-10 over `crates/mori-chip/src/` at main `f59ef318f`.
- [Quint verdict: Poseidon-only HASH chip](sources/quint-chipleaf-hash.md): Lead verdict on the Gemini Quint lane of 2026-11-10 for the Poseidon-only HASH chip (`crates/mori-chip/src/poseidon.rs` and its bus use) and constraints D1, D4 and D8 of design...
- [Quint verdict: layout B pair rows and M3 aligned segments](sources/quint-chipleaf-pairrow-m3.md): Lead verdict on the Gemini Quint lane of 2026-11-10 for layout B pair rows and the M3 aligned-segment memory root (C = 8).
- [Run-length consensus (sherlock-runlength-consensus-20261111)](sources/runlength-consensus.md): Final report of the five Gemini 4 Carbon High lanes (A to E, two rounds) on the run-length bound, the D3 memory ceiling and continuations, asked for in runlength-consensus-brief.
- [Audit of the run-length consensus (Fable 5.2)](sources/runlength-consensus-audit.md): Fable 5.2 audit of runlength-consensus, read-only at `9ffc2fe76` (no build, test, proof or model run).
- [Brief: run length and memory ceiling consensus](sources/runlength-consensus-brief.md): Brief of run `sherlock-runlength-consensus-20261111`.

## Entities (5)

- [acceptRun contract](entities/AcceptRunContract.md): The Compact contract `tools/sherlock-sdk/sherlock-run.compact` whose `acceptRun` circuit accepts a run on Midnight (c6-outcome-ledger9-review).
- [IVC node](entities/IvcNode.md): The self-recursive node of the Sherlock recursion (`crates/mori-recursion/src/ivc.rs`).
- [Midnight ledger 9](entities/MidnightLedger9.md): The target ledger, read at tag `ledger-9.1.0.1` (`a2b30128`).
- [mori-chip crate](entities/MoriChip.md): `crates/mori-chip` holds the chip leaf spike: `rf.rs` (32-word register file and spike rules), `rfcore.rs` (layouts and leaf), `poseidon.rs` (Poseidon-only HASH chip),...
- [Run decider](entities/RunDecider.md): The terminal proof of a run (`crates/mori-recursion/src/run_decider.rs`).

## Concepts (8)

- [C6 outcome-only publication](concepts/C6OutcomeOnlyPublication.md): Owner decision, 2026-11-10: "go with publishing only the outcome for C6 after gemini carbon and fable and astra review that".
- [Chip leaf](concepts/ChipLeaf.md): The proposed universal Sherlock leaf: one hand-written circuit with a CPU core and the chips DEC, PAGE, MEM, ALU with RANGE, and HASH, reached through one lookup bus of tuples...
- [Continuations (stop and resume of one run)](concepts/Continuations.md): How a long run is proved in pieces, and across proving sessions, under one key set and with one public statement.
- [Dense tail-buffered memory (replacing D3)](concepts/DenseTailMemory.md): The memory design direction that replaces D3's padded aligned segments.
- [General-purpose recursion: GP-1 to GP-4](concepts/GeneralPurposeRecursion.md): Owner ruling, 2026-11-10: general purpose and universal means all four properties below.
- [Owner rules](concepts/OwnerRules.md): Standing rules from the factory charter and the owner's GP definition.
- [Run-length ceiling (D3 against GP-1)](concepts/RunLengthCeiling.md): The conflict between the proposed chip leaf memory (D3) and the owner's GP-1.
- [Value digest sherlock-value/1](concepts/ValueDigest.md): The canonical value encoding of the C6 review, used for both `input_d` and `outcome_d` (SRC-022, SRC-021 amended) (c6-outcome-ledger9-review).

## Syntheses (2)

- [Open owner questions](syntheses/OpenOwnerQuestions.md): Questions the owner must settle.
- [Workstream status](syntheses/WorkstreamStatus.md): State as of 2026-11-10 23:24 local.
