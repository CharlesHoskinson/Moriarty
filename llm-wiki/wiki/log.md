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

## [2026-11-11 03:23] ingest | Per-rule memory maxima and the one-arena non-interleaving check
- Ingested EXPERIMENTS-MAXIMA.md as `rule-maxima-experiment`: test-only mori-core module, static scan of all 120 enabled rules plus 6,668 corpus steps.
- Result: at most 3 appends, 4 distinct heap records (all can be old), 1 frame or 1 task op plus rewrite per step; one-arena non-interleaving holds, no counterexample (induction step is a code reading).
- Files: wiki/sources/rule-maxima-experiment.md, raw/SOURCES.tsv; research/sherlock-final-design-20261111/maxima/.
- Next: freeze K_page against the bound of 4 in C4; fire the 18 unfired rules from raw images.

## [2026-11-11 03:25] qa | C1 Gemini Quint lane final: TOP = 1 gap fixed, six mutant models repaired
- The TOP = 1 gap (`WALK_LAST` shadowed `WALK_EMPTY`, so `x0 = EMPTY_SEG` was unconstrained) is fixed in commit `f36eb714d` with no new gate or row; new tests `c1_top1_positive` and `c1_gap_top1_empty_seg_forgery` (accepted before, run 20261111T101257Z-c1-top1-prefix; refused after, run 20261111T101433Z-c1-top1-fix); C1-ROWS at TOP = 61 and 63 byte-identical.
- Six Quint attacks had left derived cells stale (read-port record hash and walk, f = 0 new walks, one row hash); helpers `rederiveRead` and `rederiveNewWalks` fix them; `non_canonical_tail` now drops the mask on both row kinds. Re-run: 30 of 30 lines match, 34 `quint test` passing, all 23 mutants caught.
- Files: qa/gemini-quint/c1-dense/verdict.md, dense.qnt, rerun-results.txt; crates/mori-chip/src/dense.rs, dense/tests.rs; source `c1-dense-memory-experiment` updated.
- Next: carry conditions C1 to C4 into the MEM and CORE contracts; the clippy errors in other mori-chip files are still open.

## [2026-11-11 03:36] qa | Rule-maxima review fixes: ADT view ROM rows and scan soundness
- The rule-maxima scan now follows every definition of an ambiguous call, scans `adt_runtime.rs`, asserts each family entry is found and lists unresolved calls; it also counts ADT ROM row reads through `program_view()`.
- Earlier columns unchanged. New STATIC result: ADT rules read up to 6 ROM row call sites in 4 tables in one step (`XAdtHelperPlanSwitch`), up to 9 ROM rows per step by call sites; `K_rom` blocks per table stay open. Heap bound 4 relabelled STATIC for 116 rules, READ for 4. T0 passed (`20261111T103539Z-maxima-t0`).
- Files: `rule-maxima-experiment` page, `research/sherlock-final-design-20261111/EXPERIMENTS-MAXIMA.md`, `maxima/`.
- Next: log the ADT view rows in `LogView` to observe `K_rom` blocks.

## [2026-11-11 03:40] note | Withdraw the ROM total in the 03:36 rule-maxima entry
- Corrects the `[2026-11-11 03:36] qa` entry: "up to 9 ROM rows per step by call sites" is withdrawn. Nine rules log more ROM reads than their counted sites (for example `BHelperScalarTag`), since value reads can land on constant rows and selection reads are not counted.
- The 6 ADT view row sites in 4 tables (`XAdtHelperPlanSwitch`) stand as a count of that site class only; total ROM rows per step and `K_rom` blocks are open.
- Files: `rule-maxima-experiment` page, `research/sherlock-final-design-20261111/EXPERIMENTS-MAXIMA.md`.

## [2026-11-11 04:18] ingest | Architecture revision B2: 23 module kits, OD-1 to OD-22, Lean 14 proved and 53 open
- Ingested `sherlock-architecture` (ARCHITECTURE.md, file time 04:10): Fable 5.2 merge of six drafts, revised after the Astra audit; 23 kits, two seams (MEMORY BACKEND, BOUNDARY COMMITMENT), build groups F to 8, X1 and X2 done, X3 to X15 open.
- Lead decisions with OWNER marks: OD-1 (TOP >= 63), OD-2 (no claim count), OD-17, OD-18, OD-20, OD-21 and others; nothing built, modelled or measured by the revision.
- Files: wiki/sources/sherlock-architecture.md, raw/SOURCES.tsv.
- Next: ingest the audit and its response; concept page ModularArchitecture.

## [2026-11-11 04:18] ingest | Astra audit of the B1 architecture: adopt with changes
- Ingested `arch-audit-astra` (file time 03:32, before the newest earlier entry; logged at ingest time): GPT-6 Astra, read-only, 4 blocker, 15 major, 1 minor.
- Blockers: scratch enforcement, one stack operation per slot, no Init owner, incomplete GP-2 discharge.
- Files: wiki/sources/arch-audit-astra.md, raw/SOURCES.tsv.
- Next: the response.

## [2026-11-11 04:18] ingest | Response to the architecture audit: all 20 findings fixed as design text
- Ingested `arch-audit-response` (file time 04:11): every finding admissible; kits INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE, GP-3 general verifier added; X10 to X15 new.
- Limits: design text only; OD-17, OD-18, OD-21 left to the owner; two fixes leave open Lean obligations.
- Files: wiki/sources/arch-audit-response.md, raw/SOURCES.tsv.
- Next: update the decisions page with M-1 to M-4 and A-1 to A-6.

## [2026-11-11 04:18] ingest | Lead decisions M-1 to M-4 and A-1 to A-6 added
- Updated `final-design-decisions` (DECISIONS.md, file time 04:12): rule maxima M-1 to M-4 and architecture audit A-1 to A-6; lead decisions pending owner review, not owner rulings.
- New OWNER items: M-2 (TOP = 63), A-2 (OD-20), A-3 (OD-17, OD-18), A-4 (OD-21); contradiction flagged between R-4's lower-bound option and A-6.
- Files: wiki/sources/final-design-decisions.md, raw/SOURCES.tsv row refreshed.
- Next: Gemini draft source pages.

## [2026-11-11 04:18] ingest | Gemini area drafts 1 to 3 (labelled drafts)
- Ingested `gemini-area1-processor-draft`, `gemini-area2-memory-draft`, `gemini-area3-recursion-draft` (file times 02:50, 02:51, 02:47); drafts, not adopted, merged into the architecture.
- Main divergences from decided inputs: split chip ids, other memory keys, two-column tagged table, four requests per step, 2-leaf RunLift, Profile B as co-equal or default.
- Files: three source pages, raw/SOURCES.tsv.
- Next: concept and synthesis pages.

## [2026-11-11 04:21] plan | Concept and synthesis pages for architecture B2
- New concept `ModularArchitecture` (23 kits, seams, build order, freeze prerequisites, INIT and REFERENCE BRIDGE sections); B2 sections added to Continuations, GeneralPurposeRecursion, ValueDigest, DenseTailMemory, BlockCache.
- OpenOwnerQuestions: OD-17, OD-18, OD-20, OD-21 added; OD-1 (TOP >= 63, M-2) and OD-2 (claim counter) cross-referenced; earlier questions kept.
- WorkstreamStatus and overview refreshed (final design at revision B2, nothing built); Astra draft pages marked merged.
- Next: lint, then owner review of the OD items and the X10 to X15 experiments.

## [2026-11-11 04:31] ingest | OpenSpec change sherlock-modular-leaf-20261111
- New implementation spec: 23 capability specs, 119 requirements, 203 scenarios, 84 tasks; validate --strict passes.
- Source `openspec-modular-leaf`; the architecture page no longer says the change is not created.
- README Sherlock section rewritten for the modular design, dense memory and the C1 measurements.
- Next: morning report and owner review of the OWNER decisions.

## [2026-11-11 04:40] note | Night plan finished; morning report written
- 17 of 18 night-plan steps ticked; C2 stays open (boundary commitment designed, not built, so no Quint model yet).
- Morning report: ~/Projects/Moriarty/research/sherlock-final-design-20261111/MORNING-REPORT.md lists the OWNER questions OD-1, OD-2, OD-17, OD-18, OD-20, OD-21.
- Next: owner review; then the interface freeze experiments X3 to X15.

## [2026-11-11 04:58] note | Corrections after the final review of the night output
- Night plan count is 16 of 18 ticked (C1 and C2 partial), not 17 as the 04:40 entry says.
- README and morning report now state the Lean scope (leaves of at most 8 appends; abstract one-lane bus) and depth 63 as proposed, pending OD-1.
- ARCHITECTURE.md reconciled with the late Gemini draft text (B2a) and the stale X1/X2 text removed; design.md 2.7 of sherlock-vm-consensus-20261107 carries a dated superseding note.
- Next: owner review; the OpenSpec change still lacks the harness spec of outline 5.2.3.

## [2026-11-11 11:50] decision | Node and decider at k = 21; recalibration round queued
- Owner: "expand to K=21 and recalibrate our design and all optimizations under it including the lookups", then examine six optimizations.
- Brief and run order: ~/Projects/Moriarty/research/sherlock-k21-recalibration-20261111/ (BRIEF.md, QUEUE.md); measurement lane M started.
- Next: design lanes (Opus, Astra, Gemini) once M, S1, N1b and the 12 elastic chip proposals exist.

## [2026-11-11 12:00] decision | Zero knowledge end to end is a hard requirement
- Owner: "we need to make this zero knowledge or else its useless".
- Added as section 4a of the k = 21 brief; concept page [[ZeroKnowledge]]; owner rule 8.
- Next: the recalibration lanes design and price it.

## [2026-11-11 12:15] ingest | Animation version 2 record, terminology, k = 21 brief
- Wrote `animation-v2-notes`, `sherlock-terminology`, `k21-recalibration-brief`; new pages [[AnimationV2]] and [[ZeroKnowledge]].
- Owner: "keep a log in wiki-llm for version 2 of the animation that reflects all the new design and the compact and ZK compatibility".
- Next: add each landing design round to [[AnimationV2]]; ingest the compact feasibility, cost and elastic chip rounds.

## [2026-11-11 12:20] lane-done | Compact feasibility Gemini lane and elastic chip Gemini B
- Compact feasibility: all three lanes done (proposals/fable.md, astra.md, gemini.md under ~/Projects/Moriarty/research/sherlock-compact-feasibility-20261111/).
- Elastic chip: B-gemini.md done; A-gemini and D-gemini still running.
- Next: merge of the feasibility lanes into MERGED.md, with the ledger 9 constants re-checked and one zero-knowledge section.

## [2026-11-11 09:32] note | Pipeline animation run started
- late entry (time from commit `680d2b5af`).
- Owner asked for a three.js walkthrough of the eight steps with Fable 5.2 as creative director over 3 Gemini, 3 Astra, 10 Terra and 10 Sonnet agents, and a council review of the design.
- Shell and brief committed on branch `pipeline-anim` (worktree ~/Projects/Moriarty-wt/pipeline-anim, folder research/sherlock-pipeline-animation-20261111/).
- Next: content, concepts, storyboard, scene builds.

## [2026-11-11 10:24] note | Compact repository graph built
- late entry (time from graph.json).
- LFDT-Minokawa/compact `a36ebaa`: 6,907 nodes, 13,202 edges, 356 communities, with a nanopass extractor (tools/scheme_extract.py).
- ~/Projects/Moriarty/research/compact-graph-20261111/graphify-out/.
- Next: Compact feasibility round on the graph.

## [2026-11-11 11:02] note | Compact feasibility first pass read ledger 8
- late entry (time from fable.ledger8.md).
- Owner: "Midnight is on ledger 9, you are out of date". The midnight-ledger checkout moved to branch `ledger-9` at `f15fb9d`; first-pass reports kept as proposals/*.ledger8.md; lanes revised on ledger 9.
- ~/Projects/Moriarty/research/sherlock-compact-feasibility-20261111/BRIEF.md ("Ledger version").

## [2026-11-11 11:05] note | Animation scenes 1 to 8 committed
- late entry (time from commit `3adece369`).
- Storyboard, scene contract, eight scenes and the council synthesis on branch `pipeline-anim`.
- Open build items in ~/.cache/pipeline-anim/BUILD-NOTES.md; integration lane started.

## [2026-11-11 11:12] note | Native Compact circuit sizes measured (N1)
- late entry (time from N1-native-circuits.md).
- compactc 0.36.0: entry points from k 7 (68 rows) to k 15 (`zerocash spend`, 24,025 rows), measured; verifier keys 2,121 bytes.
- ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/measured/N1-native-circuits.md.
- Next: N1b proving time against k, S1 Sherlock step profile.

## [2026-11-11 11:24] note | Council revisions committed
- late entry (time from commit `f372ab74c`).
- Council blocker C5 (`reg_root` across child slots) and majors fixed; DECISIONS P-1 to P-10; new spec sherlock-ledger-contract; OD-23 new owner question; openspec validate passes.
- On branch `pipeline-anim` only until merge.

## [2026-11-11 11:30] note | Cost round lanes and prover graph done; elastic chip round started
- late entry (times from proposal and graph files).
- Cost round (owner: "the 1000 to 10,000 times cheaper is not a number you just say"): Fable estimates 10,400 to 31,300x in rows for `spend`; Astra finds the node 98.42 percent of rows (estimated).
- Prover graph of midnight-ledger ledger-9, midnight-zk and midnight-zkir: 21,437 nodes (~/Projects/Moriarty/research/prover-graph-20261111/).
- Elastic multicore chip round: 12 lanes, areas A to D (~/Projects/Moriarty/research/sherlock-elastic-chip-20261111/BRIEF.md).

## [2026-11-11 11:48] note | Elastic chip Fable and Astra lanes done
- late entry (times from proposal files).
- Area A: row cores beat column cores (241,000 to 254,000 node rows per column core, estimated). Area B: `b` parallel block chains. Area C: memory limits this host (k = 20 node 16.3 GB, measured). Area D: quotient and NTT 57 to 71 percent of a node proof (measured).
- Prover findings for the wiki: stock seeded proofs depend on thread count; midnight-zk HEAD draws blinders from `OsRng`; ledger 9 pins midnight-proofs 0.8.2 against our 0.8.3.
- Next: Gemini lanes, then merge under k = 21.

## [2026-11-11 12:25] note | Owner: keep a living log
- Owner: "make sure we keep a living log in wiki-llm so nothing is lost".
- Rule: every lane start and finish, owner decision, measurement and commit gets a log entry when it happens; this block backfills the morning.
- Next: ingest the day's round reports as source pages.

## [2026-11-11 12:35] lane-done | Animation integration (Sonnet 10)
- Commit `2e6aa8c8b` on branch `pipeline-anim`: label determinism fixed, carrier collisions cleared, 8.8 wide shot 317 to 227 draw calls (budget 260), stepped mode and WebGL2 fallback checked, impeccable detect clean.
- Open: materials over budget in scenes 4, 5, 7, 8 (139, 59, 83, 63 against 40); a 1 px side-panel text flake from web fonts; one overlapping phone label pair at 7.18.
- Report: ~/.cache/pipeline-anim/INTEGRATION.md.
- Next: STORYBOARD-AMEND-1 (location chip, rail, gate, new beats), material atlas, vendored fonts, then Terra QA.

## [2026-11-11 12:38] lane-start | Animation amendment build (Sonnet 10)
- STORYBOARD-AMEND-1: location chip, rail, boundary gate, 14 new narration beats, beat 8.9 (116 beats, 2,874 s); AA location colours distinct from the outcome teal.
- Also the material atlas (scenes 4, 5, 7, 8 to 40 or fewer), vendored fonts for the text flake, the 7.18 phone overlap, and the DESIGN, SCENE-API and SCENE-CONTRACT updates.
- Next: Terra 1 to 10 QA on the amended build.

## [2026-11-11 12:33] note | Correction: times of the 12:35 and 12:38 entries
- late entry. The entries "Animation integration (Sonnet 10)" and "Animation amendment build (Sonnet 10)" carry times later than the events; both happened before 12:32 local.
- Native proving time N1b measured up to k = 19 (19.5 s, 7.5 GB, 471,433 rows); k = 20 at the 16 GB cap was throttled (132.8 s) and is being re-run at 40 GB with k = 21 and k = 22.
- ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/measured/N1b-native-proving-time.md.

## [2026-11-11 14:10] incident | Network outage stopped every Claude agent
- About 12:40 to 14:08 the API server was unreachable (EAI_AGAIN); seven agents stopped: k = 21 measurement, N1b, S1, Compact merge, wiki ingest, animation Sonnet 10 and its scene 7 helper.
- systemd units kept running: the k = 20 and k = 21 node runs finished (rc 0, ~/.cache/k21-measure/run-gp.unit.log); the animation commit `72d93f2bb` (location system, vendored fonts) landed before the outage.
- All seven agents resumed at 14:10.

## [2026-11-11 14:10] lane-done | Elastic chip Gemini A; all 12 chip proposals done
- A-gemini.md is complete (sections 1 to 6, written by 12:07); its argon run hung after that and the unit was stopped.
- All 12 proposals now exist in ~/Projects/Moriarty/research/sherlock-elastic-chip-20261111/proposals/.
- Next: the k = 21 design lanes, once M-k21.md, S1 and N1b k = 21 exist.

## [2026-11-11 14:10] ingest | Council findings and revisions (pipeline-anim)
- Wrote `council-findings` (C1 to C10: changes_requested, blocker C5, major C1 to C4 and C6, C10 refuted) and `council-revisions` (P-1 to P-10, 24 kits, Lean 14 proved and 55 open); both files are on branch `pipeline-anim` only.
- Updated ModularArchitecture, ValueDigest, IvcNode, RunDecider, AcceptRunContract, OpenOwnerQuestions (new OD-23), AnimationV2, WorkstreamStatus, overview.
- Next: merge of `pipeline-anim` into `main`; owner on OD-23 and OD-17 with I7.

## [2026-11-11 14:10] ingest | Cost round brief, N1, N1b and three lanes
- Wrote `cost-round-brief`, `n1-native-circuits`, `n1b-native-proving-time`, `cost-lane-fable`, `cost-lane-astra`, `cost-lane-gemini`.
- Native `spend` k = 15, 24,025 rows, 1.755 s (measured); lanes put Sherlock at about 10^4 times native in rows (estimated) and disagree on 10x and 2x; contradictions recorded on each lane page.
- N1b k = 20 to 22 rows were unfilled placeholders as read; S1 not in `measured/`.
- Next: lead merge of the cost lanes; re-ingest N1b when complete; S1.

## [2026-11-11 14:10] ingest | Compact and prover graphs
- Wrote `compact-graph` (6,907 nodes, 13,202 edges, manifest 909 entries) and `prover-graph` (21,437 nodes, 57,968 edges; no manifest in the folder).
- Health notes: Compact corpus-size warning and six import cycles; prover corpus check reads about 0 words, 20 one-file import cycles.
- Next: ingest the Compact feasibility round.

## [2026-11-11 14:10] ingest | Elastic chip round brief
- Wrote `elastic-chip-brief`: 12 lanes, areas A to D, one key set for every program and host, proofs byte for byte as the stock prover.
- The 12 proposals get one page from the lead's merge.

## [2026-11-11 14:10] ingest | Elastic chip area D Fable lane (GPU backend, F-D1 to F-D3)
- Wrote `elastic-chip-gpu-fable`: five GPU kits; F-D1 seeded proofs depend on the rayon thread count, F-D2 HEAD draws quotient blinders from `OsRng`, F-D3 proof format change and the 0.8.2 against 0.8.3 pin.
- Linked from ZeroKnowledge (fresh blinders in production, seeded proofs as a test mode) and MidnightLedger9 (k = 20 decider verification confirmed from sources).
- Next: X-D1, X-D8, X-D0 in the merged round.

## [2026-11-11 14:12] lane-done | S1 Sherlock step profile measured
- Hand-translated Compact circuits on the reference machine (measured): tiny set 21 zkir instructions, 1,846 proved steps; vote$commit 132 instructions, 11,116 proved steps (about 85 steps per instruction; hash and Impact builtins are stubs).
- Hot spot: BHelperEnvGetCell 3,868 of 11,116 steps in vote$commit (environment lookup); PushFrame 389 of 2,239 reference steps.
- ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/measured/S1-sherlock-steps.md.
- Next: profile input for superinstructions and straight-line mode in the k = 21 round.

## [2026-11-11 14:14] lane-done | Compact feasibility merged
- Verdict: feasible with named modules, no ledger change; conditions XC-9 (a k = 20 proof verifies on ledger 9, untested) and E-1 (one key for every entry leaves the entry label unbound).
- Tap at `Lflattened` after `optimize-circuit2`; tier 1 kits COMPACT FRONTEND, ABI, SEAL, HOST, TX PLAN, FIELD, SHA256; rules 120 to 134-146 (estimated); zero-knowledge rules Z-1 to Z-10, end-to-end zero knowledge not yet established (XC-7).
- ~/Projects/Moriarty/research/sherlock-compact-feasibility-20261111/MERGED.md; now a source of the k = 21 brief.
- Next: ingest MERGED.md and fill the Compact section of [[AnimationV2]]; owner points listed in the next answer.

## [2026-11-11 14:18] lane-done | k = 21 measurements (lane M)
- Node rows flat across k (chip node 823,466 to 823,470, cost model); node proof 71.7 s to 113.4 s (1.58x) and 16.25 to 31.64 GB; decider 929,170 rows fits k = 20; free range-table space stays about 16,102 rows at both k (measured).
- A k = 21 decider proof verified through the ledger 9 VerifierKey::verify; tampered inputs rejected; the full transaction path not run.
- ~/Projects/Moriarty/research/sherlock-k21-recalibration-20261111/measured/M-k21.md; harness on branch `k21-measure` (`208a9e9be`).

## [2026-11-11 14:18] lane-start | k = 21 design lanes
- Opus agent, GPT-6 Astra high (unit k21-astra), Gemini 4 Carbon High (unit k21-gemini), one shared brief ~/.cache/k21-recal/brief-lane.md.
- Outputs: research/sherlock-k21-recalibration-20261111/proposals/{opus,astra,gemini}.md.
- Next: merge, blinded verifier, apply to the design and specs.

## [2026-11-12] measurement | M7-0: ledger 9 refuses any non-stdlib key; G4 needs a ledger change or a decider layer
- `MidnightVK::read` rebuilds the constraint system from `ZkStdLib::configure(arch, k - 1)`; key bytes carry chip flags, not gates (CODE).
- Toy `tools/gpu-prover/src/bin/m7_ledger_toy.rs` (one G4 lane, degree-5 gate, lookup, 1,026 public inputs): library verify accepts, ledger `VerifierKey::verify` gives `problem reading the verifier key` under 3 headers (MEASURED).
- Single-lane stdlib wrap at OD-19: 2.37 M to 3.05 M rows, over the k = 21 gate (ESTIMATE).
- Lead decision, owner to confirm: no on-chain path chosen; M7c to M7e paused. Options: A G4 arch in zk-stdlib (ledger change), C stdlib decider over G4 (two BLS layers, NOT CHECKED), E Fr-terminal layer (OD-11, NOT CHECKED); B and D rejected.
- Ledger-compatible path (lead decision, owner to confirm): P3, the stdlib wrap split into 3 chunk proofs at k = 21 as chained calls in one transaction; largest chunk 1.07 M rows, 60 percent of the gate (ESTIMATE).
- Source: research/sherlock-m7-wrap-20261112/RESULTS.md on branch m7-wrap.
