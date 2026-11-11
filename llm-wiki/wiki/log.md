# Log

## [2026-11-10 21:20] plan | Wiki created
- Scaffolded llm-wiki/ from the workflowskills wiki-llm template.
- Next: ingest the first sources.

## [2026-11-10 21:29] ingest | Owner rules: AGENTS.md and the GP-1 to GP-4 definition
- Ingested `agents-charter` and `general-purpose-definition`; listed the source folders and standing owner rules in CLAUDE.md.
- Result: OwnerRules and GeneralPurposeRecursion pages; GP-1 to GP-4 are never narrowed without the owner.
- Files: wiki/concepts/OwnerRules.md, wiki/concepts/GeneralPurposeRecursion.md.
- Next: ingest the C6 review.

## [2026-11-10 21:29] ingest | C6 outcome-only and ledger 9 review
- Ingested `c6-outcome-ledger9-review`: Q1 adopt with changes (3 of 3), Q2 runs on ledger 9 yes with changes (3 of 3).
- Result: C6OutcomeOnlyPublication, ValueDigest, AcceptRunContract, MidnightLedger9 and RunDecider pages.
- Contradictions flagged: exhaust kind 7 against SVM-009; n_steps in out_d of the GP study.
- Next: ingest design 2.7 and the dissent record.

## [2026-11-10 21:29] ingest | Design 2.7 chip leaf and the C6 dissent section
- Ingested `design-2-7-chip-leaf` (proposed, not adopted; figures are cost_model rows, no proof) and `dissent-c6-section` (D-1 to D-6, exhaust kind 7, run ceiling).
- Result: ChipLeaf, RunLengthCeiling and OpenOwnerQuestions pages.
- Contradiction flagged: design 2.7.11 still marks the packed-selector Quint lane as OPEN.
- Next: ingest the Quint verdicts.

## [2026-11-10 21:30] ingest | Gemini Quint verdicts on the chip leaf
- Ingested `quint-chipleaf-final`, `quint-chipleaf-hash` and `quint-chipleaf-pairrow-m3`.
- Result: packed selectors and M3 as built SOUND WITH CONDITIONS, RANGE SOUND, HASH SOUND with C1 to C6; C6 addresses open.
- Files: wiki/sources/quint-chipleaf-final.md, quint-chipleaf-hash.md, quint-chipleaf-pairrow-m3.md; wiki/entities/MoriChip.md.
- Next: ingest the experiment measurements.

## [2026-11-10 21:30] ingest | E2 to E4 and the chip spike experiments
- Ingested `e2-e4-experiments` (k=20 leaf proves in 66.2 s at 11.1 GB peak RSS) and `chip-spike-experiments` (E0 to E8, cost_model rows on an unsafe SRS, no proof).
- Result: IvcNode page; the C6 digest leaf node figure (741,708) is kept apart from the chip-key node figure (823,466).
- Files: wiki/sources/e2-e4-experiments.md, wiki/sources/chip-spike-experiments.md.
- Next: ingest the GP architecture study.

## [2026-11-10 21:30] ingest | GP architecture study, section 1
- Ingested `gp-architecture` briefly (section 1 only): binary PCD node at k=20, claims, key registry, committed memory.
- Contradictions flagged: out_d with n_steps (D-3) and shape_d bracketing (D-6).
- Next: record the two running studies as pending.

## [2026-11-10 21:31] ingest | Two running studies recorded as pending
- Ingested the briefs `runlength-consensus-brief` and `lookup-cache-brief`; no lane output or result was read.
- Result: owner statements of 2026-11-11 on 2^64 recorded; per the brief, 2^64 is the SRC-004 step-counter width.
- Next: ingest each CONSENSUS.md and its audit when they exist.

## [2026-11-10 21:31] note | Workstream status written
- WorkstreamStatus lists C6 in the spec, the chip page animation, the README and the E4 k=20 cap as on main and pushed (origin/main at c57351dd4).
- The land-3x dispatch branch is landing; its U023c receipt passing 6 of 6 is reported by the lead and not yet recorded in any repository file.
- Next: confirm the U023c receipt on the next sync.

## [2026-11-10 21:36] note | Corrections after review of the first wiki commit
- Corrects the entries of 21:29 to 21:31: GP-1 rule quoted as written; only the run-length study is on the bound; status line no longer goes stale; AGENTS.md dated 2026-10-24.
- Unsourced lines removed; E2 figures labelled (unlabeled) or "Measured model rows" as their sources say; scope line added to `chip-spike-experiments`.
- U023c 6 of 6 is still not found in commits on land-3x or in the main checkout.

## [2026-11-10 22:43] lane-done | Astra area drafts 1 to 3 for the final design
- Astra wrote three area drafts for run `sherlock-final-design-20261111`: processor modules, memory and values, recursion and system. Times are file modification times (22:38, 22:40, 22:43).
- Result: drafts for a later merge; written before the run-length CONSENSUS.md existed; nothing built or run.
- Files: research/sherlock-final-design-20261111/drafts/astra-area1.md, astra-area2.md, astra-area3.md.
- Next: architecture merge.

## [2026-11-10 22:55] lane-done | Run-length consensus report
- Five Gemini 4 Carbon High lanes finished; the final CONSENSUS.md was written (time is the file modification time).
- Result: 2^64 is the SRC-004 counter guard; M3-Tail64 dense memory with hash-chain stacks; two-tier continuations; arena count split 3 against 2.
- Files: research/sherlock-runlength-consensus-20261111/CONSENSUS.md.
- Next: Fable 5.2 audit.

## [2026-11-10 23:08] commit | Merge land-3x into main (e39bc7af8)
- Main commit e39bc7af8 merges land-3x: dispatch partition, U023c receipt and status notes for tasks 3.1 to 3.4 and 4.1 (commit time).
- Result: receipt testdata/receipts/dispatch.json from run 20261111T013913Z-land3x-u023c, 6 passed; no task ticked.
- Files: openspec/changes/sherlock-vm-consensus-20261107/tasks.md, crates/mori-backend/src/sherlock_machine/testdata/receipts/dispatch.json.
- Next: per-class-group T2 runs; predicates U023a to U023c.

## [2026-11-10 23:15] audit | Fable 5.2 audit of the run-length consensus
- Fable 5.2 audited CONSENSUS.md read-only at 9ffc2fe76 (time is the file modification time).
- Result: adopt with changes; 3 blocker, 10 major, 7 minor; corrected GP-1 and GP-001 text that only appends.
- Files: research/sherlock-runlength-consensus-20261111/AUDIT.md.
- Next: lead decisions.

## [2026-11-10 23:16] decision | Lead overnight decisions R-1 to R-10 and O-1
- The lead recorded decisions while the owner is asleep (time is the file modification time). Lead decisions pending owner review, not owner rulings.
- Result: audit text for GP-1 and GP-001; dense tail-buffered memory; tier 1 plus retained-memory continuations; R-4 and R-5 marked OWNER; R-6 arenas open.
- Files: research/sherlock-final-design-20261111/DECISIONS.md.
- Next: owner review of R-4 and R-5; experiment C1.

## [2026-11-10 23:25] ingest | Run-length consensus and its audit
- Ingested `runlength-consensus` and `runlength-consensus-audit`; refreshed `runlength-consensus-brief` from pending to done.
- Result: new concept pages DenseTailMemory and Continuations; RunLengthCeiling and GeneralPurposeRecursion updated; contradictions flagged on both pages with the audit's correction marked as adopted by the lead.
- Also flagged on gp-architecture (occurrence count), dissent-c6-section (D-1) and design-2-7-chip-leaf (D3 and D7 replaced).
- Next: ingest the lead decisions.

## [2026-11-10 23:26] ingest | Lead overnight decisions
- Ingested `final-design-decisions` as lead decisions pending owner review.
- Result: OpenOwnerQuestions gains R-4 tree depth TOP >= 63, R-5 claim counter and R-6 arenas; question 1 marked answered, pending owner; no question closed.
- Files: wiki/sources/final-design-decisions.md, wiki/syntheses/OpenOwnerQuestions.md.
- Next: ingest the land-3x merge.

## [2026-11-10 23:27] ingest | land-3x merge and the U023c receipt
- Ingested `land-3x-merge` from the tasks.md notes on main and the receipt dispatch.json.
- Result: 3.x dispatch partition is on main and origin/main at e39bc7af8; U023c 6 of 6 is recorded in the 3.3 note and the receipt; no task ticked.
- Files: wiki/sources/land-3x-merge.md, wiki/syntheses/WorkstreamStatus.md.
- Next: ingest the Astra drafts.

## [2026-11-10 23:28] ingest | Astra area drafts 1 to 3
- Ingested `astra-area1-processor-draft`, `astra-area2-memory-draft` and `astra-area3-recursion-draft` as drafts for a later merge.
- Result: draft conflicts flagged: memory key encodings differ between areas 1 and 2; area 2 recommends four arenas against R-6; "tier" differs from the consensus.
- Files: wiki/sources/astra-area1-processor-draft.md, astra-area2-memory-draft.md, astra-area3-recursion-draft.md.
- Next: refresh status and overview, lint.

## [2026-11-10 23:29] note | U023c receipt now found in repository files
- Settles the entries of 2026-11-10 21:31 and 21:36: U023c 6 of 6 is now in the tasks.md 3.3 note and in testdata/receipts/dispatch.json on main.
- Source: `land-3x-merge`.

## [2026-11-10 23:30] lint | Wiki lint after the run-length, decisions, land-3x and draft ingests
- Ran lint.py --write-index, then lint.py: 35 pages, 23 SOURCES.tsv rows.
- Result: PASS, no failures, no orphans.
- Next: the lead reviews and commits llm-wiki/.

## [2026-11-10 23:31] decision | GP-1 and GP-001 text replaced with the audit's corrected text
- The lead applied AUDIT section 5 to `general-purpose.md` lines 14 and 16, the GP-001 row of `requirements.md` and the GP spec mirror (one scenario added); every current sentence is kept.
- openspec validate passes; F14 PASS (run 20261111T061635Z-F14); traceability.md re-rendered.
- Pages updated: GeneralPurposeRecursion, WorkstreamStatus. Source: `runlength-consensus-audit`.
- Next: owner review of R-2, R-4 and R-5.

## [2026-11-10 23:57] qa | C3 Lean: partition independence and modular composition proved
- New `clean/fork/Clean/Sherlock/PartitionIndependence.lean` and `ModularComposition.lean` (with Axioms companions): 52 theorems, no sorry, no axiom, standard axioms only; lake build and F21 green.
- Tail-buffered memory: every cut of N steps gives `abs (next^[N] s)`; `det` discharged for the physical step (`premises_det`); D3 padding has no deterministic physical step on reachable states (`d3_no_det_run`), per audit finding 10.
- Results: `research/sherlock-final-design-20261111/EXPERIMENTS-C3.md`. Next: C4 records them; module kits instantiate `Contract`.

## [2026-11-11 00:05] ingest | Experiment C1 dense tail-buffered memory measured
- MemBackend trait and dense Merkle backend built in crates/mori-chip (membackend.rs, dense.rs); 8 positive tests and 21 named mutants pass MockProver (run 20261111T065515Z-c1-test).
- Measured at TOP = 61 / 63: flush unit 8,541 / 8,821 rows, cold read 4,621 / 4,761, window slot 203 (run 20261111T065836Z-c1-rows); leaf and node rows estimated.
- Source `c1-dense-memory-experiment` written; DenseTailMemory updated.
- Next: LogUp backend, hash-chain stacks, wiring into RfLeaf, Gemini Quint lane over the C1 gates.

## [2026-11-11 00:12] qa | C1 mutants isolated, frontier moved into MEM, four-arena leaf rows
- Three mutants now fail only at their named gate; MEM decomposes both lengths itself; new positive test at the last segment with two flush units (run 20261111T071031Z-c1-test, 30 passed).
- Rows unchanged (run 20261111T071045Z-c1-rows); four-arena leaf rows estimated in `c1-dense-memory-experiment`.
- Next: Gemini Quint lane over the C1 gates; decide the arena count in C4.

## [2026-11-11 02:46] ingest | Lookup-cache consensus (five Gemini 4 Carbon High lanes)
- Ingested `lookup-cache-consensus`: Profile A dense Merkle memory plus 8-record PAGE and ROM cache lines and a Poseidon boundary; Profile B ZmLogUp + Horner kept behind the interface; all study figures ESTIMATE.
- Result: new concept page BlockCache; `lookup-cache-brief` no longer pending; ChipLeaf, DenseTailMemory and ValueDigest updated.
- Files: wiki/sources/lookup-cache-consensus.md, lookup-cache-brief.md, wiki/concepts/BlockCache.md.
- Next: ingest the Fable 5.2 audit.

## [2026-11-11 02:46] ingest | Fable 5.2 audit of the lookup-cache consensus
- Ingested `lookup-cache-audit`: adopt with changes; 3 blocker (no per-step cache bound, value digest prefix, section 3 interface and non-injective key), 11 major, 9 minor.
- Result: contradictions flagged on both study pages, BlockCache and the C1 page (cold-read label, row figures, four-arena fit).
- Files: wiki/sources/lookup-cache-audit.md, wiki/sources/c1-dense-memory-experiment.md.
- Next: ingest the new lead decisions.

## [2026-11-11 02:46] ingest | Lead decisions E-1 to E-3 and L-1 to L-10 (pending owner review)
- Updated `final-design-decisions` with the C1 decisions E-1 to E-3 and the lookup-cache decisions L-1 to L-10; lead decisions while the owner is asleep, not owner rulings.
- Result: L-9 (one arena) flagged against R-6, E-3, both audits and the consensus; L-2 (scalar cold read kept, line counts not frozen) against consensus P2.
- Files: wiki/sources/final-design-decisions.md, OpenOwnerQuestions (L-2, L-9 added; R-4, R-5 open), WorkstreamStatus, overview, raw/SOURCES.tsv.
- Next: lint.

## [2026-11-11 02:46] lint | Wiki lint after the lookup-cache and decisions ingests
- Ran lint.py --write-index, then lint.py: 39 pages, 26 SOURCES.tsv rows.
- Result: PASS, no failures, no orphans.
- Next: the lead reviews llm-wiki/; owner review of L-2, L-9, R-4 and R-5.
