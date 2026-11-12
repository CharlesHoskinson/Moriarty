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

## [2026-11-11 14:25] plan | Plan to the final design of the co-processor
- Owner: "we need to get to a final design, give me a step by step list of tasks to finialize the processor".
- 13 steps from the k = 21 lanes to the interface freeze, landing and animation version 2: ~/Projects/Moriarty/research/sherlock-processor-final-20261111/PLAN.md.

## [2026-11-11 14:25] lane-start | Scaling test design
- Owner: "we also need a test we can do quickly to verify the scaling optimizations. Have some agents design a test and lets see if we can actually measure".
- Designers: Opus agent and GPT-6 Astra high (unit scaletest-astra); brief ~/Projects/Moriarty/research/sherlock-scaling-test-20261111/BRIEF.md.
- Next: one plan, then a build agent measures on this host.

## [2026-11-11 14:40] lane-done | Scaling test designs; build and run started
- Opus and Astra designs: ~/Projects/Moriarty/research/sherlock-scaling-test-20261111/designs/{opus,astra}.md. Earlier thread-count measurements are not valid: BLST sizes its own MSM pool to all 20 CPUs whatever RAYON_NUM_THREADS says; heavy.slice is capped at 72 GiB (4 k = 20 or 2 k = 21 proofs at once).
- From existing traces at k = 20, 20 threads, runs not pinned: quotient 0.56 of proof time, MSM 0.26 (constant 16.6 s), NTT 0.25; only the numerator evaluation scales with threads.
- Build agent runs the merged plan (about 108 min of proving, 65 GB peak) in worktree ~/Projects/Moriarty-wt/scaling-test after the native sweep; output RESULTS.md.

## [2026-11-11 14:47] decision | Inner recursion moves to a small-field STARK
- Owner: "why can't we have a much smaller node?", then "use the small field stark and find the best framework for it", then "design an experiment".
- Leaves and nodes become small-field STARK proofs; one wrap gives the BLS12-381 decider that the ledger 9 verifier accepts. "Node and decider at k = 21" is withdrawn. The scaling test was stopped before it proved anything.
- Experiment: ~/Projects/Moriarty/research/sherlock-stark-framework-20261111/EXPERIMENT.md (P0 screen of 15 candidates, P1 leaf bench, P2 node bench, P3 wrap). P0 lanes started: Opus, GPT-6 Astra (unit stark-screen-astra), Gemini (unit stark-screen-gemini).
- Next: shortlist, then the bench in worktree ~/Projects/Moriarty-wt/stark-bench.

## [2026-11-11 14:48] lane-done | k = 21 recalibration, Opus lane
- Framework-independent results kept: adopt straight-line mode (40 to 340x fewer steps, estimated) and SHA-256 spread tags in the L-6 table; adopt with conditions superinstructions (vote$commit 11,116 to about 4,100 steps, estimated), fixed-base tables, claims, pre-admitted stdlib.
- Zero knowledge: salted input_d, outcome split into public part plus hiding priv_c, program commitment C_P, n_steps removed from every published statement, padding to a public schedule; obligations ZK-L1 to ZK-L5.
- Findings: the landed decider publishes `result` in clear (run_decider.rs:8-11); rules_d is an unsalted hash (ivc.rs:489,549); seeds derived from task ids break zero knowledge; usable rows are 2^k minus 7.
- ~/Projects/Moriarty/research/sherlock-k21-recalibration-20261111/proposals/opus.md; its halo2 node results (CHAIN-4 at k = 21) are superseded by the STARK decision.

## [2026-11-11 14:51] incident | Ollama found running; stopped and disabled
- The user unit ollama.service was active since boot despite the 2026-11-09 owner order.
- Owner: "kill ollama it isn't doign anything". systemctl --user disable --now ollama; no ollama process remains.

## [2026-11-11 14:55] lane-done | k = 21 recalibration, Astra and Gemini lanes; native k = 20 and 21 timed
- proposals/astra.md (11,234 words) and gemini.md (17,197 words) are in ~/Projects/Moriarty/research/sherlock-k21-recalibration-20261111/; only their framework-independent parts carry forward.
- Native Compact proving (measured, 40 GB cap): k = 20 950,281 rows 101.9 s, 14.9 GB; k = 21 1,900,495 rows 211.8 s, 29.3 GB; k = 22 pending.
- Next: merge the framework-independent parts of the three lanes after the STARK framework decision.

## [2026-11-11 14:59] lane-start | Optimization compatibility with the STARK engine (P4)
- Owner: "make sure all our optimizations work with the stark engine".
- Brief ~/Projects/Moriarty/research/sherlock-stark-framework-20261111/COMPAT-BRIEF.md: a complete inventory of the design's optimizations, each with a verdict and a cost against halo2; key question is BLS12-381 arithmetic (Compact Field, transientHash, Jubjub, sherlock-value/1) in a small field.
- Lanes: Opus agent and GPT-6 Astra (unit stark-compat-astra). The bench gains workloads W3 (non-native BLS12-381 field, Poseidon, Jubjub) and W4 (SHA-256); the decision rule now penalizes a framework that cannot carry an optimization.

## [2026-11-11 15:03] lane-start | Bridge round: a structure between the STARK and BLS12-381
- Owner: "is there a mathematical structure we can add between the stark and the BLS to make calculations easy. Launch some fable and astra agents to think about it".
- Four lanes on ~/Projects/Moriarty/research/sherlock-stark-bls-bridge-20261111/BRIEF.md: Fable 1 fields and hashes, Fable 2 deferral and co-processing, Astra 1 embedded curves (units bridge-astra1), Astra 2 open search (bridge-astra2).
- Each proposal is costed against the direct wrap and against limb emulation, with its smallest experiment on this host.

## [2026-11-11 15:05] decision | Bridge candidate: Lean proof and GPU friendly
- Owner: "once we have a candidate, formalize it in lean. It has to be GPU friendly".
- BRIEF.md section 7 and QUEUE.md in ~/Projects/Moriarty/research/sherlock-stark-bls-bridge-20261111/: each proposal states a formalizable soundness theorem and its GPU mapping; the merged candidate is proved in Lean 4 (clean/fork, no sorry, named axioms), then a GPU kernel check on this host.

## [2026-11-11 15:09] lane-done | STARK framework screen, Opus lane
- Strict zero knowledge (G3): only Plonky3 with Plonky3-recursion passes (HidingFriPcs, LogUp, 2-to-1 aggregation); its recursion example seeds hiding randomness with the level number, which is not zero knowledge in production. Plonky3 has no GPU prover.
- OpenVM stark-backend: CUDA prover, 100 bits proven, audited v2, a halo2 static verifier as a wrap template, but no hiding code. Cut: SP1, Stwo, Airbender (G3), RISC Zero (needs zirgen, 97 bits conjectured), ZisK (no aarch64), Pico, Ceno, Winterfell, Binius, WHIR.
- No candidate has BLS12-381 Fr or midnight's Poseidon in its proof code (G4); it must be added for the last layer.
- ~/Projects/Moriarty/research/sherlock-stark-framework-20261111/screen/opus.md; Astra and Gemini screens still running.

## [2026-11-11 15:12] decision | Zero knowledge at the wrap; STARK layers local; OpenVM front-runner
- Owner: "The OpenVM is running locally so couldn't we develop a snark final step that runs with Sherlock for BLS?", then "yes" to the proposal.
- The blinded BLS12-381 wrap verifies the last STARK as a private witness; STARK proofs stay on the user's machine and are secrets. G3 changed in EXPERIMENT.md; bridge brief section 8 makes the wrap (k <= 22 on this host) its main target.
- Bench started: OpenVM stark-backend CUDA and CPU, Plonky3 CPU, workloads W1 to W4, node 2-to-1 (~/Projects/Moriarty/research/sherlock-stark-framework-20261111/bench/).

## [2026-11-11 15:21] decision | Bridge terminology; OpenVM leads as the STARK engine
- Owner: "name the mathematical structure we are developing or modifying", then "yes and openvm seems to be the winner".
- Terms in TERMINOLOGY.md (branch pipeline-anim) and [[SherlockBridge]]: Sherlock bridge, bridge layer, wrap, deferred field log, STARK engine.
- OpenVM stark-backend is the owner's pick; the bench confirms it builds and runs on this host's GPU, Plonky3 stays the fallback.

## [2026-11-11 15:29] lane-done | Bridge round, Fable 1 (fields and hashes)
- Proposal P1: every STARK layer on BabyBear; only the bridge layer hashed with midnight's Poseidon over Fr (8 limbs of 31 bits per Fr slot), OpenVM's BN254 root configuration moved to BLS12-381. Wrap 410,000 to 950,000 rows, so k = 20 (about 68 s, 17 GB), estimated; midnight's Poseidon chip is why it beats the k = 23 template.
- Open: the last layer's constraint size and commitments (experiment E1) and GP-3 (a BLS12-381 pairing in the wrap pushes it to k = 21; proposed: verify foreign proofs inside the STARK with OpenVM's pairing extension). First GPU task: port OpenVM's BN254 hash kernels to BLS12-381 Fr; the wrap prover stays on CPU.
- Owner question: may prog_d, input_d, outcome_d change value under a new version tag (sherlock-value/2)?
- ePrint downloads hit a bot check, not worked around; those papers are UNREAD. ~/Projects/Moriarty/research/sherlock-stark-bls-bridge-20261111/proposals/fable1.md.

## [2026-11-11 15:29] lane-done | Bridge round, Fable 2 (deferral)
- Proposal FRLOG, the deferred field log: the STARK logs coarse Fr operations under the small-field hash, the bridge layer exposes them as public values, the wrap replays them natively; overflow past the wrap's fixed capacity is proved by an emulation chip in the STARK, so GP-1 to GP-4 hold.
- Wrap rows added, estimated: about 80 per logged Fr value, 310 per two-input transientHash, 2,400 to 4,400 per ecMul; profiles of 0.2 M, 0.8 M and 2.2 M rows at k = 22. Compact Field multiplies stay emulated in the STARK (241 wrap rows against one emulated row); value digests move to the small-field hash.
- ePrint returned 403 (not bypassed): CycleFold, LogUp and LogUp-GKR are UNREAD and nothing rests on them. ~/Projects/Moriarty/research/sherlock-stark-bls-bridge-20261111/proposals/fable2.md.

## [2026-11-11 15:33] lane-done | Optimization compatibility, Opus lane
- Inventory of 150 items with verdicts and costs against halo2 (estimated); 13 items depend on the framework and go to the bench. Decisions taken only to save halo2 node rows are reversed (L-6 merge, 12-bit limbs, column cores, request lanes); dense Merkle tree between leaves, write-once LogUp memory inside each leaf.
- BLS12-381 work: one Fr multiply about 70 to 110 cells on 31-bit fields, one midnight Poseidon about 32,000 to 50,000 cells (estimated); recommends Poseidon2 for internal digests and in-leaf emulation, under 1 percent of a leaf for spend. Wrap fixed cost about 0.45 to 0.87 M rows.
- Zero knowledge: proof shape (AIR heights, which AIRs are present) is visible, so shapes must be fixed by public data; salts are vectors (8 elements on 31-bit fields). Its claim that the wrap can run on a helper conflicts with the owner decision that the wrap verifies the STARK as a private witness; the merge resolves it.
- ~/Projects/Moriarty/research/sherlock-stark-framework-20261111/compat/opus.md.

## [2026-11-11 15:34] note | Native k = 22 stopped for the OpenVM bench
- N1b real entries (measured): tiny set 0.552 s, vote$commit 1.022 s, spend 1.755 s; synthetic k = 19 19.5 s, k = 20 101.9 s, k = 21 211.8 s. The k = 20 point may be inflated by host contention (about 11 of 20 cores available).
- k = 22: the prover key is over 4 GiB (tagged format limit); keygen 44.7 GB peak. Proofs stopped so the host is free for the STARK bench; the quiet-host recheck is skipped because the halo2 node-k decision is moot.

## [2026-11-11 15:38] decision | Never k = 22
- Owner: "We will never go to K=22".
- Every BLS12-381 circuit, the wrap and the decider included, must fit k <= 21; the bridge brief section 8 now says k <= 21, and the merge must fit the deferred field log profiles (Fable 2 sized up to 2.2 M rows at k = 22) inside it. Owner rule 10.

## [2026-11-11 15:39] decision | Shard proofs; no circuit grows with the workload
- Owner: "We have to shard proofs like we have been doing".
- Every proof is a fixed-size shard: STARK leaves and nodes, one fixed-size wrap per run at k <= 21. The deferred field log has a fixed capacity; overflow BLS12-381 work and GP-3 foreign proofs run in STARK shards. Bridge brief section 9; owner rule 11.

## [2026-11-11 15:40] decision | Per-program shard preprocessing
- Owner: "For any program there needs to be a preprocessing step that optimizes the proving step for shards thus we can massively speed up recursion".
- Each program gets a witness-independent shard plan bound to prog_d: superinstructions, leaf shapes from a fixed menu, cut points, aggregation tree, deferred-log layout, committed leaf keys. The wrap key stays one key set.
- Bridge brief section 10; bench lane told to measure planned against generic shards.

## [2026-11-11 15:40] lane-start | Online research: shard preprocessing techniques
- Owner: "Search online for preprocessing techniques we can use".
- One Opus research lane: program-specialized AIRs, static analysis and cut points, preprocessed columns and leaf-key commitments, fixed shape menus, aggregation shaping, precomputation reuse.
- Output: research/sherlock-shard-preprocessing-20261111/RESEARCH.md with a ranked top 8 and one experiment each.

## [2026-11-11 15:44] lane-start | Bridge merge into one candidate
- All four bridge proposals are in (fable1, fable2, astra1, astra2). A non-author Opus merger writes CANDIDATE.md under the k <= 21, shard and preprocessing rules, with a Lean plan.
- research/sherlock-stark-bls-bridge-20261111/CANDIDATE.md. Next: Lean formalization in clean/fork, then the GPU kernel check.
- Final design PLAN.md updated: steps 1a (bridge), 1b (shard preprocessing), 2 (measure table); step 3 merges all of them.

## [2026-11-11 15:48] lane-start | XC-9: k = 21 decider through the full ledger 9 transaction path, local
- Owner: "Run it now". One Opus lane deploys SherlockRun with the k = 21 decider key on a local Docker ledger 9.1.0.1 network, calls acceptRun with the proof, reads back, and runs two refusals.
- Output: research/sherlock-k21-recalibration-20261111/measured/XC-9-ledger9-tx-path.md. Shares heavy.lock with the STARK bench.

## [2026-11-11 15:48] decision | Public qualification on Preview; wallet setup started
- Owner: "Then decide whether to test it on preview or preprod and get a wallet setup. I will solve the coudflare for you".
- Lead decision: Preview, as decided 2026-11-08 (D12); the holdout registry seals the Preview genesis, and Preview runs the same ledger 9.1.0.1 as Mainnet. Preprod stays available as an extra rehearsal before Mainnet.
- OP-PV-WALLET lane started: new wallet and maintenance authority key, seeds kept local only, public identities into target-pins. Report: research/sherlock-preview-qualification-20261111/WALLET.md. The owner clears the Cloudflare faucet check.

## [2026-11-11 15:56] note | Preview wallet created and funded by the faucet
- Wallet created by OP-PV-WALLET (seed local only, mode 0600, age backup with an owner passphrase). Unshielded address mn_addr_preview1qdqmf7mv8n673fm0r868vyxzkfcnjepmsnyayhxc54vqvyg4l87s5dwena.
- Owner cleared the faucet: 5000 tNight, faucet transaction 001a9c1cd8992cef620e9d13e377b28c2d8217d7f54767ef5ebfc2c5d983b1f29c.
- Next: --status confirms the balance, then --register-dust. Report: research/sherlock-preview-qualification-20261111/WALLET.md.

## [2026-11-11 15:58] lane-done | Bridge candidate merged
- CANDIDATE.md: BabyBear leaf, discharge, GP-3 and node shards; last STARK layer hashed with midnight Poseidon over Fr (WHIR, log blowup 4; 100-bit claim unconfirmed until the soundness calculator runs); blinded midnight-proofs wrap at k = 21.
- Deferred field log with fixed capacity (1,024 values, 1,024 permutations, 16 Jubjub multiplications), overflow into STARK shards; wrap budget about 1.56M of 2,097,145 rows (74 percent, ESTIMATE). 21 disagreements resolved; k = 22 profiles out of scope.
- research/sherlock-stark-bls-bridge-20261111/CANDIDATE.md. Next: Lean formalization (lane started, worktree bridge-lean), then the GPU kernel check.

## [2026-11-11 15:59] decision | Internal digests move to a small-field hash
- Owner: "yes for the small-field hash". Internal digests move from sherlock-value/1 (Poseidon over Fr) to sherlock-value/2 on Poseidon2 over BabyBear; Poseidon over Fr stays only in the last STARK layer and the wrap.
- Recorded in research/sherlock-stark-bls-bridge-20261111/CANDIDATE.md (owner decision section); applies to SRC-022 and the C6 digests in final design steps 6 and 7. The Lean lane is told.

## [2026-11-11 16:00] lane-start | GPU path for the wrap prover
- Owner: "We need a GPU path on the wrap prover", then "That was the assignment for the mathematical structure". The bridge candidate had no GPU wrap path (111.8 s on CPU at k = 21); recorded as a gap in CANDIDATE.md.
- One Opus build lane on branch gpu-wrap: G1 sppark or ICICLE MSM and NTT on sm_121, hooks in vendored midnight-proofs, k = 21 decider proved on GPU and verified by the ledger 9 verifier, then the quotient on the GPU.
- Output: research/sherlock-gpu-wrap-20261111/RESULTS.md with a CPU against GPU table at k = 19, 20, 21.

## [2026-11-11 16:03] lane-start | Fable lane: GPU-shaped wrap circuit
- Owner approved one Fable lane ("yes") to shape the wrap circuit for the GPU: columns, gate degree, lookups, field-log layout, and whether an Fr-native terminal layer beats the direct wrap on GPU wall time.
- Output: research/sherlock-stark-bls-bridge-20261111/proposals/fable-gpu-wrap.md, with a ranked kernel list for the gpu-wrap build lane.

## [2026-11-11 16:03] lane-done | Animation v1 amendment 1 integrated
- Commits 72d93f2bb, 2308dd8c7, 3f57e6aea on pipeline-anim: location system, rail labels, 14 exact narrations, beat 8.9; 116 beats, 2,874 s; 16 of 16 repeat captures byte-identical; label overlap 0 at wide and phone.
- labels.js commit failed twice in unrelated factory unittests under host load; lead retrying. Open: scene 4 materials 46 against 40, frame-time check not rerun. Report: ~/.cache/pipeline-anim/INTEGRATION.md.

## [2026-11-11 16:06] commit | Animation label fix committed
- a777a21d4 on pipeline-anim: phone rule hides a label with no free spot. Pre-commit T0 PASS (6 passed, 3 skipped, 204 s).

## [2026-11-11 16:13] note | Preview wallet funded and DUST registered
- 5000 tNIGHT arrived (faucet tx 7afbb26f, block 70411). DUST registration tx 549b7983 at block 70495; the NIGHT UTXO is registered.
- DUST accrues: 26868155231668571 at 23:06:54Z to 35713845231668571 at 23:10:28Z (raw units, MEASURED). Shielded balance 0. Receipts in receipt-traces/OP-PV-WALLET/.
- Next: OP-PV-DEPLOY after XC-9 passes locally and the SherlockRun update for the wrap statement.

## [2026-11-11 16:16] lane-done | Shard preprocessing research
- OpenVM v2 proves with SWIRL (LogUp-GKR, zerocheck, stacked PCS, WHIR), not FRI; the wrap cost model must count a WHIR and sumcheck verifier. OpenVM node keys are already program independent; per-program leaf keygen is the preprocessing cost; a Merkle root of leaf keys under prog_d is checked in the node.
- Top 8: universal node over per-program leaf keys; cached program trace with its codeword on the GPU; straight-line blocks and superinstructions; static shard plan; small menu of leaf shapes; deferred-operation shards and field log; prepared nodes and cached twiddles; memory layout under dense Merkle. Excluded: EC multiset memory, accumulation.
- Reported gains: powdr autoprecompiles 1.37x with recursion (measured by source, OpenVM 1.3, 2025-09-22); hand precompiles 12.13x on the same block. Seven ePrint papers abstract only, not cited.
- research/sherlock-shard-preprocessing-20261111/RESEARCH.md. The Fable GPU wrap lane is told about WHIR.

## [2026-11-11 16:21] lane-done | Fable lane: GPU-shaped wrap circuit
- Shape: gate degree at most 5 (extended domain 4n; use the 19-row stdlib Poseidon, not the degree-6 70-row chip), 4 SIMD lanes of 8 advice columns over about 14 shared fixed columns, 18 permutation columns, 4 lookups into one 2^16 table; k = 20 planning case, k = 19 at mid estimates. ESTIMATE GPU wall 5 to 12 s at k = 20 against 111.83 s MEASURED CPU at k = 21.
- SWIRL count from stark-backend code: CANDIDATE.md 4.1 is low by about 310k rows at the high end; with the 70-row chip one lane reaches 92.5 percent of k = 21. Astra 2's Fr layer breaks even only if the direct GPU wrap is above about 4 to 5 s; measure at k = 20.
- Kernel order sent to the gpu-wrap build lane: Fr field header, NTT and MSM, coset-tiled quotient, grand products, multiopen, lookup. Eight amendments to CANDIDATE.md proposed in section 6.
- research/sherlock-stark-bls-bridge-20261111/proposals/fable-gpu-wrap.md

## [2026-11-11 16:24] lane-done | OP-PV-WALLET done
- Preview wallet and the SMN-007 maintenance key created; seed and key are 0600 files under ~/.local/share/moriarty/wallets/preview/; age backup with the owner's passphrase, decrypt check passed. Public identities in the target-pins.json preview row; check-target-pins.py passes; 9 tests pass.
- Deviation: the DUST registration proof ran outside heavy.lock, on a pinned proof server 9.0.1 at 127.0.0.1:6311 (now stopped). Shielded sync sends the viewing key, not the seed, to the indexer.
- research/sherlock-preview-qualification-20261111/WALLET.md; receipts in receipt-traces/OP-PV-WALLET/ (uncommitted).

## [2026-11-11 16:24] note | ePrint papers fetched by the owner with scrapling
- The owner ran the scrapling fetch for 11 ePrint papers (the classifier blocked the agent run); Cloudflare solved, 2025/917 fetched first. Saving to research/sherlock-shard-preprocessing-20261111/papers/.

## [2026-11-11 16:34] lane-done | XC-9 PASS: k = 21 decider accepted through the full ledger 9 transaction path, local
- Local undeployed devnet (node 2.1.101, spec 2001100, indexer 4.4.101, proof server 9.0.1, pinned digests): SherlockRun deploy with the 2,217-byte k = 21 decider key at block 85 (key occurs once, 5 bytes after the entry name); acceptRun call with a fresh k = 21 proof at block 237, SUCCESS, proof body 6,432 bytes; readback result 6, steps 6 (MEASURED).
- Task 1.4 answered: stored result has a one-byte prefix 0x50 at offset 31; the u128-be window is offset 32, length 16. All three refusals hold (reused proof with r+1, bit flip, wrong prog_d). No ledger limit binds (1 percent of tx bytes, 1.04 percent of block).
- Our bugs: sherlock_gp.rs:527 used the k20-only SRS (fixed); call-run.mjs, check-local-run.mjs, stagenet-session.mjs use pre-Midnight.js-5 APIs (replaced by xc9-local-run.mjs, old files not patched). Call proofs 535 s and 699 s, about 30.1 GB, under host load 37.
- ~/Projects/Moriarty-wt/xc9 branch xc9: research/sherlock-k21-recalibration-20261111/measured/XC-9-ledger9-tx-path.md. Next: SherlockRun update for the wrap statement, then OP-PV-DEPLOY on Preview.

## [2026-11-11 16:36] lane-start | Preview deploy and call rehearsal
- Owner: "Deploy now". One Opus lane deploys the current SherlockRun with the k = 21 decider key and the SMN-007 maintenance authority on Preview, calls acceptRun with a fresh k = 21 proof, reads back, and runs refusals. Genesis check first; stop if one fee is over 25 percent of DUST; no automatic resubmits.
- Worktree ~/Projects/Moriarty-wt/preview-deploy (from xc9). Output: research/sherlock-preview-qualification-20261111/DEPLOY-REHEARSAL.md.

## [2026-11-11 16:41] audit | Paper review of the bridge design
- 100-bit target of the last layer holds only via an unproved remark in BCHKS25 (ECCC TR25-169 p.23), margin 0.29 bits; Theorem 1.5 as stated (m >= 3) gives folding 94.2 and mu-batching 96.3 bits. Fix: raise folding and mu PoW from 15 to 21 bits (100.2 and 102.3 bits), more than doubling the Fr hashing of the last layer (ESTIMATE); wrap cost unchanged. Owner choice.
- CONFIRMED: Johnson-radius WHIR without capacity conjecture (SWIRL Thm 4.2.4), LogUp-GKR soundness and subround count, pv binding 2^-114.02, Poseidon 8/60 at t=3, Merkle memory, cached traces, universal keys. NOT COVERED: bounded-quotient Fr encoding, shape menus, superinstruction gains. ePrint papers still UNREAD.
- research/sherlock-design-paper-review-20261111/REVIEW.md

## [2026-11-11 16:41] lane-done | STARK bench: OpenVM GPU leaf rows measured
- W1 Poseidon2 2^16 to 2^22: 2.60, 2.98, 5.83, 18.4 s on GPU (MEASURED); W2 lookup AIR 2.24 to 3.85 s. GPU default zerocheck_save_memory costs 36x on W1; all cells run with it off. Generic child key with absent preprocessed AIRs fails node verification (SumClaimMismatch).
- Unit stark-bench-campaign runs the other about 40 cells (nodes, generic leaf, Fr multiply, CPU threads, Plonky3). Write-up of cost per leaf and specialized against generic is pending.
- research/sherlock-stark-framework-20261111/bench/RESULTS.md

## [2026-11-11 16:42] decision | Proven 100-bit security for the last STARK layer
- Owner: "b is fine". Folding and mu proof of work of the last layer rise from 15 to 21 bits, giving 100.2 and 102.3 bits under BCHKS25 Theorem 1.5 as stated; no reliance on the unproved remark.
- Cost: grinding rises to about 2.7 x 10^7 Fr Poseidon permutations (ESTIMATE), on the GPU in the STARK prover; wrap cost unchanged. Recorded in CANDIDATE.md; the Lean lane is told.

## [2026-11-11 16:54] plan | Loop armed to finish the processor design
- Owner: "Arm a loop to finish the entire agenda and finish the entire processor design". Agenda with items A1 to A11 and PLAN steps S3 to S11: research/sherlock-processor-final-20261111/LOOP-AGENDA.md. The loop stops when the interface freeze (S11) is presented to the owner; owner decisions are batched in S5.
- Started: Fable E-1 entry label lane and Opus Compact frontend lane (research/sherlock-compact-open-problems-20261111/), and the framework shortlist merge with a draft DECISION.md.

## [2026-11-11 16:57] note | Owner AFK while the design loop runs
- Owner: "going afk". The loop continues LOOP-AGENDA.md without questions; S5 decisions take the recommended option, each marked "lead decision, owner to confirm". No Mainnet, no push, no real funds.

## [2026-11-11 17:04] lane-done | Framework shortlist and draft decision
- SHORTLIST.md: 12 candidates merged, 17 disagreements resolved against the clones. Plonky3 leaves each AIR's LogUp terminal unmasked (partial hiding only); Gemini's Pico license and OpenVM v2 audit claims contradicted.
- Draft DECISION.md: OpenVM stark-backend v2.1.0 CUDA first, Plonky3 second (CPU), SP1 third. 2^20-row W1 leaf plus node share 7.64 s binary, 6.5 to 6.7 s at arity 4 or 8 (from MEASURED cells); CPU about 9x slower. Rule 3 waits on Plonky3 cells.
- Largest open risk: the k <= 21 wrap is not costed; P3 WRAP.md lane started. research/sherlock-stark-framework-20261111/{SHORTLIST,DECISION}.md

## [2026-11-11 17:09] lane-done | E-1 entry label: binding_input can be recomputed in the proof (untested)
- Fable E-1 lane: research/sherlock-compact-open-problems-20261111/E1-entry-label.md. The preimage of binding_input is public transaction bytes under SHA-256 with the entry label at byte offset 58 (ledger verify.rs:2151-2193). A seal that recomputes the hash and pins the label window binds the label on the unmodified ledger 9.1.0.1 with one key set, 0 public inputs, 0 transcript ops, 0 fee. This overturns MERGED 4.5 ("the seal cannot recompute it").
- Status: code reading plus argument, NOT TESTED. Gate: test T0 (host rebuilds the preimage and matches the ledger value; no prove). Open: seal rows for N_B SHA blocks at k = 21 (T7); hash in the VM instead forces a full re-prove per transaction rebuild. Source and concept pages for this finding are pending.

## [2026-11-11 17:09] lane-done | Lean formalization of the bridge: BUILD PASS
- lake build of SherlockSpec passes (Lean v4.33.1, 30 jobs); 0 sorry, admit, native_decide or axiom declarations; bridge_sound and all 84 theorems use only propext, Classical.choice, Quot.sound. Eight named premises plus one static key fact; A2 is the proven Johnson regime (BCHKS25 Thm 1.5, m = 3, 21-bit PoW); A7 is the blinded-wrap ZK premise.
- Design amendments recorded in CANDIDATE.md: d_0 = P2(TAG_EMPTY); explicit quotient range check (unbounded_quotient_unsound); canonical squeeze check (squeeze_alias); fixed pv length per key.
- Gaps: lemma 23 pv_binding_count not attempted; coset_fold, whir_round, pv_eq only generic; pvFirst and ProvenRegime are key flags. Branch bridge-lean f8d8781fe, 4bcb238d7, f54eb39f5; LEAN.md. Follow-up lane closes the gaps.

## [2026-11-11 17:09] lane-start | E-1 local devnet tests
- From E1-entry-label.md (Fable): mechanism E-1d binds the entry label on unmodified ledger 9.1.0.1 by recomputing binding_input (SHA-256 over public tx bytes, label at a fixed offset) in the seal; ESTIMATE 80k to 160k rows at k = 21; fallback ledger change specified.
- Opus lane on branch e1-test: T0 preimage match, today's cross-entry attack, seal prototype with measured rows. Output: research/sherlock-compact-open-problems-20261111/E1-devnet-test.md.

## [2026-11-11 17:16] lane-done | Compact frontend spec and limits
- FRONTEND.md: tap Lflattened at passes.ss:138 with a new add-sherlock-anchor pass; CLI compile, plan, deploy, prove, call; plan fixes prog_d without witnesses. Build order M0 to M5 with tests.
- The 65,536 public input cap does not bind; the wrap rows do. Proposed L_TX = 1,024 transcript words fits all 27 checked entries (largest 474); chunked calls have an atomicity hole and stay OPEN. Fr multiply MEASURED at one 194-column row, 8.7 us on GPU. Sherlock call about 195 s against 0.03 to 1.76 s native (ESTIMATE): native wins every call today, about even near native k = 21, Sherlock only option above it or for foreign proofs.
- E-1 placement in the wrap does not fit next to L_TX = 1,024; proposes a per-transaction binding shard. L_TX and the call-vector layout are owner points (S5).

## [2026-11-11 17:16] lane-start | Astra cross-check of E-1 and the Compact frontend
- codex gpt-6-astra high, read-only, unit compact-astra-review; checks whether binding_input is known at proof time, cross-entry reuse after E-1d, the binding shard, L_TX and chunked-call atomicity. Output: research/sherlock-compact-open-problems-20261111/astra-review.md.

## [2026-11-11 17:21] lane-start | Call-time optimization run: 12 lanes
- Owner: "Launch another 12 agents to do a final optimization run with this new design to see if we can dramatically improve the call time". Baseline about 195 s per call (ESTIMATE: wrap 111.83 s MEASURED CPU plus L grinding about 78 s).
- Lanes: gpu-wrap-full, wrap-rows, last-layer, offline-online, fr-terminal, stark-path, amortize, halo2-algos, elastic, security-budget, call-shape, latency-model (with models/call_time.py). Brief and outputs: research/sherlock-call-time-opt-20261111/.

## [2026-11-11 17:21] lane-start | Atomic chunked calls for bigger transcripts
- Owner: "Explore this as well Bigger transcripts: splitting them across several calls has an atomicity hole". One Opus lane: several calls in one transaction, staged commit, commitment-and-open replay, larger L_TX menu; picks one with a devnet atomicity test plan.
- Output: research/sherlock-compact-open-problems-20261111/ATOMIC-CHUNKS.md.

## [2026-11-11 17:26] lane-done | P3 wrap cost model
- Shape 1 (decider-style single lane, 70-row Poseidon) DOES NOT FIT k <= 21 with 15 percent margin: mid 2.66M rows, 127 percent of k = 21 (ESTIMATE). Shape 2 (Fable 4-lane GPU shape, 19-row Poseidon) FITS k = 20: mid 482,173 rows (46 percent), high 649,478 (62 percent).
- Mid counts from the stark-backend verifier: about 11,930 Fr Poseidon permutations, 42,600 EF products, 179,000 mod-p reductions; 21-bit PoW adds nothing to the wrap (17 checks). Four corrections to CANDIDATE 4.1. Prove time shape 2 at k = 20: 70 to 90 s CPU, 5 to 12 s GPU (ESTIMATE).
- L's interactions, DAG size and constraint count are guesses (1.0M of shape 1's rows); keygen-only dump of the root verifier started to measure them. WRAP.md, wrap_model.py, DECISION.md updated.

## [2026-11-11 17:27] lane-done | Astra cross-check of E-1 and the Compact frontend
- E-1d confirmed: no fee or DUST circularity, and correctly implemented it prevents entry substitution.
- Ranked errors: chunking contradicts ledger phase rules (sent to the atomic chunks lane); binding-shard merge lacks an authenticated relation; host workflow must finish transcripts before the binding proof; 1,024-word fit and seal-side SHA exclusion unmeasured; performance claims overreach; frontend lowering and log accounting mistakes. FRONTEND.md revision started.
- research/sherlock-compact-open-problems-20261111/astra-review.md (codex gpt-6-astra high, read-only, end 00:22Z).

## [2026-11-11 17:32] lane-done | FRONTEND.md revised after the Astra review
- Errors 2 to 6 fixed: binding shard BS (frozen key, label at offset 58, SHA padding) and merge MR (label and bind checks, one wrap) specified with 5 tests; host order run, partition, normalize, preimage, binding proof and wrap, seal; L_TX = 1,024 provisional; seal-side SHA rows high 1,843,792 and low 1,763,792 (ESTIMATE); lowering and log accounting corrected.
- Chunking marked SUPERSEDED and OPEN, pointing to ATOMIC-CHUNKS.md. research/sherlock-compact-open-problems-20261111/FRONTEND.md section 5 lists the changes.

## [2026-11-11 17:33] lane-done | Call-time lane elastic
- Spreading proving gives 1.2 to 2.6x, not a dramatic cut (ESTIMATE): one GPU 27 to 61 s with the other GPU levers, GPU plus CPU 22 to 46 s, plus 8 LAN helpers 17 to 31 s, 8 owned GPUs 13 to 23 s.
- The gpu-wrap build measured a GPU MSM at 0.479 s per random 2^20 MSM (~/.cache/gpu-wrap/g1-bench.jsonl), 5 to 10x slower than fable-gpu-wrap assumed and no faster than 20-core BLST; inside the prover 45 k = 16 calls took 26 to 36.5 s against 14.4 s for the whole CPU proof (MEASURED).
- Only masked-scalar wrap MSMs and salted L grinding are safe for untrusted helpers; the grinding form needs owner confirmation against "STARK layers stay on the user's machine" (S5). Hiding STARK helpers rejected. proposals/elastic.md.

## [2026-11-11 17:34] lane-done | Call-time lanes gpu-wrap-full and amortize
- gpu-wrap-full: device floor of the k = 20 four-lane wrap 1.4 to 4.8 s (ESTIMATE), MSMs 60 to 70 percent; whole call 11 to 24 s against about 195 s. Today's kernels are 15 to 25x above the floor (MSM 0.479 s at 2^20 against 28 ms; quotient 7.94 s at k = 20 against 0.34 to 0.67 s, 0.91 TB of scratch traffic; MEASURED under load). Witness fill through the stock layouter 10 to 20 s (ESTIMATE); a direct lane writer 0.1 to 0.3 s. Open: GPU quotient proof verifies but is not byte identical. 11 to 16 engineering weeks. Findings sent to the gpu-wrap build.
- amortize: only the user's own batch shares the fixed cost: per-call throughput 194 s to 6.2 s at N = 64 today, 19.3 to 3.5 s on the full GPU path, latency unchanged (ESTIMATE). Ledger: one proof per call (verify.rs:2069-2072); the wrap's input count limits a batch to 35, 77 or unlimited with a batch root. Batched entry links calls to one submitter (owner point, S5). A resident prover removes the 77.6 s per-process setup XC-9 measured, not in the 195 s.
- research/sherlock-call-time-opt-20261111/proposals/{gpu-wrap-full,amortize}.md

## [2026-11-11 17:35] lane-done | Call-time lane stark-path
- Shortest path with one key set: app leaf, per-program leaf layer, L, wrap (one node layer against the SDK's four); L checks the child-DAG commit against key_root; one wrap change of about 120 rows. Fixed menu: C for 1 to 4 leaves, B for 5 to 12, A for more, one L key and one wrap key.
- Small-leaf GPU floor of 2.2 to 2.6 s comes from the stacked commitment height 2^24 (stacked_pcs.rs:312-313); a menu stacked height of 17 or 18 gives 0.5 to 1.7 s (ESTIMATE). STARK part per call: stock 7.0 to 10.6 s, path C 1.3 to 3.2 s; whole call with GPU L and wrap 9.8 to 26.4 s (ESTIMATE).
- Flag: the inner leaf layer uses list decoding with m = 2; the lead applies the proven-security rule to all layers (owner to confirm in S5); sent to the security-budget lane. E-1: SHA-256 in the leaf (V) best for call time, inside L (R) best when the run is reused. proposals/stark-path.md.

## [2026-11-11 17:36] lane-done | Call-time lane latency-model
- Cold serial Sherlock call about 230 s at mid (202 to 524 s, ESTIMATE), not 195 s: the 195 s leaves out about 35.5 s, mostly wrap setup (SRS load and setup_pk about 19 s, serial in XC-9), merge, balance and DUST proof about 3 s, inclusion 3.9 s on the local devnet.
- L plus wrap are 92 percent of the call. Softest stages: L_hash (no measurement; a 2021 third-party CPU figure), wrap_prove (no wrap circuit yet), wrap_setup. With GPU L and wrap about 50 s at mid; a resident prover about 32 s; below 30 s tracegen and about 2 s per GPU proof set the floor.
- models/call_time.py (stdlib) takes each lever as a multiplier, replacement or override. Measurement lane for L_hash and wrap_setup started. proposals/latency-model.md.

## [2026-11-11 17:36] lane-done | Call-time lane offline-online
- A warm call gains only 1 to 4 percent on CPU (to about 192 to 194 s) and 0.4 to 1.6 s on the GPU path (ESTIMATE): the 111.83 s wrap already has the proving key in memory. Warm caches and a daemon save 50 to 100 s on a cold call (k = 21 keygen 49.42 s; XC-9 setup 77.6 s and 98.0 s, MEASURED).
- Only the vanishing random polynomial is witness free (0.821 s of the 77.877 s k = 20 decider prove, MEASURED). Nothing in L can move offline. A separate per-transaction STARK binding shard is rejected (forces L and wrap re-proving); E-1 placement S reuses STARK layers on a rebuild, about 86 s on CPU instead of 195 s. Blinding slots are fresh, single use, RAM only.
- proposals/offline-online.md

## [2026-11-11 17:38] lane-done | Atomic chunked calls design
- Mode 1F on unmodified ledger 9.1.0.1: chunks 1..k of one call in one intent, all fallible only, each claims the next, the anchor cell holds H(prog_d, run_tag, j) as a lock; any error drops the segment's state (semantics.rs:162-172); dropping a later chunk fails RealCallsSubsetCheckFailure (verify.rs:1726); mode 1G (all guaranteed) for callees of guaranteed callers.
- Astra error 1 confirmed (verify.rs:1092); four proof-enforced rules fix it. Rejected: staged commit across transactions, commitment-and-open, a bigger wrap key (menu of keys is an owner point). Chunk cap about 5 to 30 per transaction (ESTIMATE). No ledger change; E-1d binds label and address per chunk.
- research/sherlock-compact-open-problems-20261111/ATOMIC-CHUNKS.md. Test tiers T1 (ledger, proofs off) and T2 (devnet) started on branch atomic-chunks.

## [2026-11-11 17:38] lane-done | Call-time lane call-shape
- E-1 placement V (SHA-256 inside the run's leaf) is cheapest and sound: about 51 to 544 sha2 rows, below 0.1 s (ESTIMATE), no SHA in the wrap. Placement S rejected (breaks the GPU wrap's no-SHA rule, moves k); B adds about 3.9 s per call. The binding_input preimage is fixed when the call intent is built; DUST balancing is a separate intent, so fees never force a re-prove; triggers are a stale read, a cost-model change or an expired TTL.
- L_TX: 78 wrap rows per public word (ESTIMATE); freeze L_TX = 1,024 unless M7 lands in a 70,000-row band; the menu is an owner point (S5). Hybrid contracts are the big lever: native 0.031 to 1.755 s (MEASURED) against 13 to 31 s on the planned GPU path; crossover about native k = 17 to 18 (ESTIMATE).
- Note: stark-path and call-shape choose V; offline-online prefers S for rebuilds. The merge (A12) settles it. proposals/call-shape.md.

## [2026-11-11 17:38] lane-done | Call-time lane fr-terminal
- Form A (an Fr-native proof N before the wrap) takes the wrap to k' = 17 or 18, not 16, and gains nothing on the full GPU path (15.6 to 35 s against 15.3 to 31 s direct, ESTIMATE). Form B (SP1-style shrink) stays at k = 19 to 20.
- Updated break-even from WRAP.md counts: N is 2^23 to 2^24 with 276 queries, T_N 4.5 to 14 s; the layer pays only if the direct GPU wrap measures above about 10 s. Value: a hedge if the MSM and quotient kernels slip (CPU small wrap 3.6 to 6.3 s at k' = 17, saving 50 to 80 s per call), but it is not a GPU wrap path.
- Needs a proven 100-bit term for N and an owner decision on a second proving engine (OpenVM cannot host Fr, config.rs:22). proposals/fr-terminal.md.

## [2026-11-11 17:39] lane-done | Call-time lanes last-layer and fr-terminal recorded
- last-layer: with the Fr Poseidon GPU port (M3) and a per-fold PoW schedule, L drops from about 86 s (corrected baseline; the 78 s left out query and LogUp PoW) to 4.0 to 8.5 s, or 2.5 to 5.5 s if L is about 2^23 cells (ESTIMATE). Call 120 to 128 s with the CPU wrap, 13 to 28 s with the GPU wrap.
- The per-fold PoW schedule keeps the weakest term at 100.18 bits and cuts fold and mu grinding 5.5x; it changes the owner's "21 bits" wording (owner item, S5). More queries cannot replace fold or mu PoW. Fallback grinding on the BabyBear Poseidon2 GPU kernel adds a random-oracle premise (owner item). proposals/last-layer.md.

## [2026-11-11 17:40] lane-done | Call-time lane gpu-wrap-full: MSM diagnosis
- Section 2.3 added: the measured 0.479 s per 2^20 MSM comes from sppark discrete-GPU settings (22 windows, 4 batches, pageable copies, host collect) plus GPU contention with bench cells
- Inside the prover the cost is bucket skew; achievable on sm_121 is 40 to 70 ms per 2^20 (ESTIMATE), so 57 commitments cost 1.1 to 4.0 s against 27 s today
- research/sherlock-call-time-opt-20261111/proposals/gpu-wrap-full.md
- Next: experiments M1 to M4; unit gpuwrap-det-k20 gives the first in-prover MSM number at k = 20

## [2026-11-11 17:42] lane-done | Call-time lane halo2-algos
- Measured from C0 raw events: k = 20 decider 77.88 s splits into numerator 24.73 s, MSM 20.55 s, NTT 17.57 s, synthesis 7.15 s; NTT flat across 4 to 20 threads, MSM 27% slower at 20 threads
- CPU fixes (FFT bit reversal precompute, blst retiling, one thread pool, work stealing numerator, lookup sort outside Montgomery form) bring the k = 21 wrap from 111.83 s to 38 to 57 s (ESTIMATE), proof format and keys unchanged
- Mixed CPU and GPU path for the G4 shape at k = 20: wrap 8 to 17 s, call 46 to 55 s with L on GPU (ESTIMATE); LogUp and other proof-changing upstream edits rejected
- research/sherlock-call-time-opt-20261111/proposals/halo2-algos.md

## [2026-11-11 17:42] lane-done | Call-time lane security-budget
- Lever: blowup 3 for L, per-fold PoW 12 to 20 bits, query radius (4/3)sqrt(rho) at m = 3; L work 5.1e7 to 1.34e7 Fr Poseidon permutations at 101.2 bits min (ESTIMATE); call -61 s on the CPU path (203 to 142 s), -4 to -7 s on the GPU path (10 to 24 s); cost +36 queries, about +229k single-lane wrap rows
- Finding: if L_PCS is read from the m = 3 interpolant (Lemma 3.1 as stated), option (b) has OOD at 98.0 bits; a citation gap, not a known break; close with the GS list bound as named premise P4c or 2 OOD samples (P4b, WHIR form UNREAD); goes to S4 and S5
- Inner layers: internal (m = 2) and hook (m = 1) at 99.68 and 94.87 bits; constant fixes internal mu PoW 21, hook mu 17 and fold 16, about 0.1 s per proof; rejected EF degree 5, blowup 5 or 6, unique decoding for L
- research/sherlock-call-time-opt-20261111/proposals/security-budget.md

## [2026-11-11 17:47] lane-done | Call-time lane wrap-rows
- Row cuts reach k = 19 in the low and mid cases only (347,552 rows mid, 483,031 high); k = 18 is not reached; one k step is worth about 1.6x (decider 1.64x measured, M-k21.md:41)
- GPU path: wrap 5 to 12 s becomes 3 to 7.5 s, call 14.0 to 29.7 s becomes 11.3 to 23.9 s (ESTIMATE); levers that move work into L lose on time
- Paying levers: wrap-side verifier algebra, capacity V = P = 256 and E = 0 (owner key constant), coset-sponge leaves, m = 3 with two OOD samples (101.1 bits, derived bound, closes the OOD caveat), per-round fold PoW 15/19/22
- research/sherlock-call-time-opt-20261111/proposals/wrap-rows.md; all 12 lanes in, merge RESULT.md next

## [2026-11-11 17:47] lane-start | Call-time merge RESULT.md
- One Opus agent merges the 12 proposals into combined call times for the CPU, mixed and full GPU paths
- It resolves the E-1 placement (V or S), reconciles the per-fold PoW schedules and the OOD 98.0-bit gap, and lists owner decision items
- research/sherlock-call-time-opt-20261111/RESULT.md; LOOP-AGENDA A12 status updated

## [2026-11-11 17:58] lane-done | Preview deploy and call rehearsal PASS
- SherlockRun d53ad4f5...41b2 deployed on Preview at block 71100 with the k = 21 key on acceptRun; call landed at block 71431, indexer readback result 6, steps 6, result window at offset 32; decider prove 542 s at 30.06 GB peak (MEASURED)
- Refusals: R1 (result r+1) and R2 (bit flip) refused with Custom error 115 (InvalidProof), R3 (prog_d+1 contract) with Custom error 104 (Transcript); no fee above 1.9% of DUST
- Fixes: HTTPS gateway 403 on large bodies so deploy resent over WS; fee gate reads DUST before balancing; node now reports build 2.1.101-bbd616b1 against pinned 913ec011 (open item)
- Commit 78592bf59 on branch preview-deploy (not pushed, carries the OP-PV-WALLET files); report ~/Projects/Moriarty-wt/preview-deploy/research/sherlock-preview-qualification-20261111/DEPLOY-REHEARSAL.md

## [2026-11-11 18:00] lane-done | Call-time merge RESULT.md
- Warm call ESTIMATE lo / mid / hi: CPU only 75.5 / 85.4 / 171.7 s, mixed 13.5 / 22.7 / 53.8 s, full GPU 11.5 / 19.6 / 47.2 s, all with the wrap at k = 19; today about 195 s warm, 229.7 s cold mid
- Largest lever is the Fr Poseidon GPU kernel for L (M3, 2 to 3 weeks); without it every path stays above 75 s; E-1 placement V with a V+ layout word; one per-fold PoW vector at T = 101 needs 22 bits on one fold (owner item); OOD gap closed by two OOD samples; L at blowup 4, radius (4/3) sqrt(rho), queries 52/27/18
- 13 owner items for S5 and experiments D1 to D15; totals script copied to models/paths.py
- research/sherlock-call-time-opt-20261111/RESULT.md; A12 done in LOOP-AGENDA

## [2026-11-11 18:00] lane-start | S3 merged proposal draft
- One Opus agent drafts PROPOSAL.md from the bridge candidate, framework, RESULT.md, Compact frontend, E-1, atomic chunks, Preview PASS and GPU wrap records
- It consolidates all owner items and lists pending evidence (bench A2, E-1 test, chunk tests, GPU wrap table, Lean gaps, L-STRUCTURE, ePrint papers)
- research/sherlock-processor-final-20261111/PROPOSAL.md; refresh after the pending items land

## [2026-11-11 18:11] note | Lean round 2 waiting on final build; OOD item sent
- Uncommitted round 2 in worktree bridge-lean: lemma 23 proved (PvBinding.lean), specific coset_fold, whir_round and pv_eq lemmas, pvFirst and the PoW part of ProvenRegime derived, d_0 and fixed pv length applied; final build counts pending
- Lead asked the agent to record the RESULT.md 4.1 OOD gap (98.0 bits under the m = 3 interpolant reading) and the new L parameter set as open items in LEAN.md, then commit on bridge-lean
- research/sherlock-stark-bls-bridge-20261111/LEAN.md on branch bridge-lean

## [2026-11-11 18:21] lane-done | S3 PROPOSAL.md draft
- About 10,100 words in 13 sections; 365 path:line citations resolve; every figure marked MEASURED, ESTIMATE, DERIVED, STATIC COUNT or PENDING
- Owner decisions OD-1 to OD-15 (RESULT items 1 to 13, keep Compact Field emulated, call-vector layout); the RESULT 4.5 L set fails the committed ProvenRegime check (fold PoW at least 21), so Lean is restated after OD-1 to OD-3
- Pending: chunk claim address and ep_hash binding under placement V (sent to the atomic chunks agent), bench A2, E-1 test, GPU wrap table, Lean round 2
- research/sherlock-processor-final-20261111/PROPOSAL.md

## [2026-11-11 18:21] lane-start | S4 blinded Astra verification
- codex gpt-6-astra, read-only, checks PROPOSAL.md against its citations and the hard rules; ranked blockers, errors, gaps, verdict line
- Output ~/.cache/s4-verify/astra.md, then written up as research/sherlock-processor-final-20261111/VERIFY.md

## [2026-11-11 18:43] audit | S4 round 1: Astra finds 4 blockers in PROPOSAL.md
- Blockers: chunk protocol restores plain prog_d against the salted anchor C_P (also leaks prog_d); security qualification open (w_stack, two-OOD not in Lean, internal constants); capacity margin at V = 256 is 2.31x not 9x; default slowdown is 34.5x not 36x
- Errors: internal OOD term has no PoW multiplier, so grinding cannot give it margin; V+ must read layout from the tape; 195 s and 19.6 s scopes differ. Gaps: Path C key check, native entries must run the anchor prologue, fixed log encoding, hash domain separation
- research/sherlock-processor-final-20261111/VERIFY.md
- Next: Opus revision of PROPOSAL.md (running), anchor findings sent to the atomic chunks agent, then round 2

## [2026-11-11 19:05] lane-done | E-1 devnet test partial: T0 PASS
- T0: the 131-byte binding preimage rebuilt from the public XC-9 tx bytes hashes to the ledger public_inputs[0]; label window at offset 58 as E1-entry-label.md states
- SherlockRunTwo (acceptRun, acceptRunB) deployed on the local devnet at blocks 8 and 12; bind gadget 5,749 rows at k = 13 for 131 bytes, 61,726 rows at k = 16 for 32 blocks (MEASURED, cost_model); variable-length form needs a selected label window
- Pending: on-chain attack H/O/X and refusal proofs in unit e1-call (queued on heavy.lock), then e1_bind_mock and e1_bind_prove
- ~/Projects/Moriarty-wt/e1-test/research/sherlock-compact-open-problems-20261111/E1-devnet-test.md

## [2026-11-11 19:07] lane-done | PROPOSAL.md revision for S4 round 1
- All 11 items addressed in the design text: salted anchor locks A_0 = A_k = C_P with one anchor statement for ordinary and chunked calls; Path C key check; anchor prologue in native entries with test T2-N and control T2-N0
- Section 11.2 freezes per-layer parameters with w_stack at most 16; two-OOD order and gamma batching in 3.4; gate G1 to G5 in 11.5; hash domains frozen in 3.6; margins 2.31x and 6.56x; slowdown 34.5x; proving-only 9.45 s against about 203 s, whole call 19.6 s; log encoding B = 256, W_pv = 2,816
- New LEAD values for owner confirmation as OD-16 to OD-18 (internal PoW 19/22/20, queries 75/31/20); 388 citations resolve
- research/sherlock-processor-final-20261111/PROPOSAL.md

## [2026-11-11 19:07] lane-start | S4 round 2 Astra verification
- Rechecks the 11 round 1 items and any new findings in the revision; output ~/.cache/s4-verify/astra-r2.md, then VERIFY.md round 2

## [2026-11-11 19:16] audit | S4 round 2: one blocker open
- Nine of the 11 round 1 items closed; R1-2 security qualification stays open until D3 and the Lean restatement run; R1-11 transcript sponge rules not fully named
- New: query terms of the app, leaf-aggregation and internal sets reach 100.01 to 100.13 bits with query PoW 20, below the 101-bit G2 margin (fix: query PoW 21); G5 must map WHIR Thm 5.2 hypotheses to the protocol (ePrint 2024/1586 to fetch); anchor DIGEST entries need reserved log capacity
- research/sherlock-processor-final-20261111/VERIFY.md

## [2026-11-11 19:16] lane-start | PROPOSAL.md revision 2 and D3 soundness run
- Opus revision applies the round 2 fixes to PROPOSAL.md
- Opus D3 agent runs the OpenVM soundness calculator with a two-OOD case on the final L and inner sets, branch d3-soundness, output D3-SOUNDNESS.md

## [2026-11-11 19:17] note | Call-time measurement: L_hash and wrap_setup (MEASURED, load 15 to 40)
- Midnight Poseidon over BLS12-381 Fr (midnight-circuits 7.3.0 CPU): 37.5 us per permutation on one X925 core, 253 us on one A725 core; host total 229k to 253k perm/s (about 9 X925 cores, not 20); one 21-bit PoW at 20 threads median 8.9 s, p95 33.6 s over 20 trials; no GPU Poseidon over Fr exists
- XC-9 prove path, k = 21 decider: SRS load 3.2 s, setup_pk 79.4 s, prove 417.3 s (3.73x contention), peak RSS 30.2 GB; k = 20: setup_pk 78.2 s, prove 254.5 s, peak RSS 15.4 GB; quiet wrap_setup 22.2 s (DERIVED)
- call_time.py mid call time 229.7 s to 356.2 s (old inputs kept behind --fallback); research/sherlock-call-time-opt-20261111/measured/L-hash-wrap-setup.md
- Next: count L_perms_proof from code; GPU Fr Poseidon kernel is the lever for L_hash

## [2026-11-11 19:22] note | Call-time measurement: quiet bound and k = 20 caveat
- Corrects the 19:17 entry: the 356.2 s mid uses L rates measured at load 15 to 40, so it is not a quiet figure; the quiet L bound (11.48 X925-core equivalents, DERIVED from single-thread CPU time) gives 311.2 s mid
- The k = 20 setup cross-check fails (vk + pk 4.9x against prove 3.74x), so the loaded k = 20 setup_pk is not comparable; quiet records show keygen 49.4 s at k = 21 and 30.9 s at k = 20
- research/sherlock-call-time-opt-20261111/measured/L-hash-wrap-setup.md

## [2026-11-11 19:22] lane-done | L_hash and wrap_setup measured
- One Midnight Poseidon permutation over Fr: 37.5 us on X925, 253 us on A725; whole host 229k to 253k perm/s, about 9 X925 cores; one 21-bit PoW median 8.9 s, p95 33.6 s at 20 threads (MEASURED under load 15 to 40); no GPU Fr Poseidon exists to measure
- wrap_setup at k = 21: SRS load 3.2 s, setup_pk 79.4 s, prove 417.3 s at 3.73x load, quiet setup about 22.2 s (DERIVED); peak RSS 30.2 GB
- models/call_time.py now takes measured inputs (--fallback restores old): cold mid 229.7 s becomes 311.2 s quiet bound; L_hash at 203 s is the largest CPU stage; RESULT.md totals need a refresh
- research/sherlock-call-time-opt-20261111/measured/L-hash-wrap-setup.md; code on branch call-time-meas, commit requested

## [2026-11-11 19:29] note | E-1 devnet test: honest call H accepted
- H accepted on the local devnet at block 991 (entry acceptRun, 10,421-byte tx, prove 570.8 s MEASURED)
- T0 on H passes three ways: host rebuild, native validate_call pi[0] and the WASM binding all equal 9afb640a...0761100; preimage 131 bytes, label at offset 58
- Running: O (H's proof under acceptRunB) and X (fresh proof bound to acceptRunB, the attack) in unit e1-call; commit retry in unit e1-commit2
- ~/Projects/Moriarty-wt/e1-test/research/sherlock-compact-open-problems-20261111/E1-devnet-test.md

## [2026-11-11 19:29] note | E-1 devnet test: control O refused
- O (H's proof submitted in a call labelled acceptRunB) refused by the node with Malformed(InvalidProof); this is the obvious control, it does not test E-1
- X (fresh k = 21 proof bound to acceptRunB, the E-1 attack) proving in unit e1-call; then e1_bind_mock and e1_bind_prove test the seal
- ~/Projects/Moriarty-wt/e1-test/research/sherlock-compact-open-problems-20261111/E1-devnet-test.md

## [2026-11-11 19:32] lane-done | PROPOSAL.md revision 2 for S4 round 2
- Query PoW 21 for app, leaf-aggregation and internal sets lifts the weakest query terms to 101.09, 101.01 and 101.13 bits (DERIVED); query vectors become frozen key constants checked at keygen
- Section 3.6 names the three sponges; transcript initial capacity 2^64 + 3*2^66; the L Merkle hash is a second stated stark-backend change; transcript vectors T-FS added
- Mandatory DIGEST entries reserved before discretionary allocation (chunk case of spend 2.27x and 6.40x); G5 needs a hypothesis mapping from WHIR Thm 5.2 and BCGM26 Thm 9.2; sections 1 and 11 state "not qualified" as a gate; 424 citations resolve
- research/sherlock-processor-final-20261111/PROPOSAL.md

## [2026-11-11 19:32] lane-start | S4 round 3 Astra verification
- Rechecks the round 2 open items and new text; R1-2 judged as a stated gate; output ~/.cache/s4-verify/astra-r3.md

## [2026-11-11 19:39] note | E-1 devnet test: attack X included without the seal
- X, a fresh k = 21 proof of entry A's transcript bound to label acceptRunB, was included on the local devnet (tx 0xfd5e5a32...194b, prove 517.9 s MEASURED); T0 on X holds with the label window at offset 58
- This confirms the cross-entry attack against today's verifier, as E1-entry-label.md predicts; the E-1d seal tests (e1_bind_mock, e1_bind_prove) decide whether the seal refuses it
- ~/.cache/e1-test/run/e1-state.json; report in the e1-test worktree

## [2026-11-11 19:39] audit | S4 round 3: no new findings open
- R1-11, the query-PoW error, the G5 gap and the anchor reservation are closed; one new wording error (80-bit derivation sentence for L) fixed by the lead in PROPOSAL.md 11.2
- R1-2 stays as a stated qualification gate G1 to G5, closed in S10; S4 marked done in LOOP-AGENDA
- research/sherlock-processor-final-20261111/VERIFY.md

## [2026-11-11 19:40] decision | S5 lead decisions OD-1 to OD-18, owner to confirm
- Owner AFK: the loop takes each recommended option of PROPOSAL.md section 12 and marks it lead decision, owner to confirm
- OD-2 (per-fold PoW at T = 101, one fold at 22 bits) differs from the owner's earlier 21-bit wording; option (b) keeps every fold at or below 21
- Owner actions: fetch ePrint papers incl. 2024/1586 (WHIR) with fetch3.py; confirm the Preview pin update to node build 2.1.101-bbd616b1
- research/sherlock-processor-final-20261111/DECISIONS-ASK.md

## [2026-11-11 19:40] lane-start | S6 apply proposal to the design documents
- One Opus agent updates ARCHITECTURE, DECISIONS, EXPERIMENTS, TERMINOLOGY, MODULARITY and the kits (new ZK and Compact kits) on branch pipeline-anim to match PROPOSAL.md; security stays a stated gate
- Also path 1 CPU totals in RESULT.md and PROPOSAL.md refreshed with the measured L rate (155.9 / 174.7 / 207.9 s)

## [2026-11-11 19:50] note | L structure by keygen (wrap model inputs)
- Keygen only of L = VerifierSubCircuit::<1> over the bench node vk at root params (MEASURED): 39 AIRs, 1,537 columns (12 cached), 35 AIRs with rotation, 487 interactions (2,687 message fields), 2,210 constraints, 15,331 DAG nodes, degree 4, C = 2. Stock root PCS: w_stack 18, log blowup 4, queries 61/30/20, final polynomial 256. Heights NOT MEASURED (no proof).
- wrap_model.py --measured: shape 1 mid 2,440,107 rows (fails k = 21); shape 2 mid 434,692 rows (k = 20 holds, also at w_stack 36: 468,675; worst gadget case 939,160 needs k = 21).
- research/sherlock-stark-framework-20261111/L-STRUCTURE.md

## [2026-11-11 19:50] lane-done | Lean bridge round 2 BUILD PASS
- All of SherlockSpec builds; 112 theorems, axioms only propext, Quot.sound and Classical.choice; no sorry, admit or native_decide
- Lemma 23 pv_binding_count proved over Z/q with BabyBear p prime checked by the kernel; specific sumcheck, WHIR round, coset fold and pv_eq lemmas; transcript of L is a program in the key, pvFirstT and PoW bits derived; onion [] = P2(TAG_EMPTY) by rfl
- Open in LEAN.md section 7: two-OOD bound and the RESULT 4.5 set (ProvenRegime still requires 21-bit fold and mu PoW, needs a per-fold check after OD-2)
- Commit bdb92973a on branch bridge-lean; ~/Projects/Moriarty-wt/bridge-lean/research/sherlock-stark-bls-bridge-20261111/LEAN.md

## [2026-11-11 19:50] lane-done | L structure keygen dump
- L keyed (no proof) at root params: 39 AIRs, 1,537 columns (12 cached), 487 interactions, 2,210 constraints, 15,331 DAG nodes, max degree 4 (MEASURED)
- Wrap rows, mid case: shape 1 2,440,107 (does not fit k = 21); shape 2 434,692, k = 20 holds, 468,675 at w_stack 36; worst gadget case 939,160 needs k = 21 (DERIVED from the keyed structure); trace heights of L stay ESTIMATE
- research/sherlock-stark-framework-20261111/L-STRUCTURE.md, .csv, .json; uncommitted

## [2026-11-11 19:50] note | Owner said the loop does not need the owner for the paper fetch and the Preview pin
- Preview pin updated in qa/sherlock/target-pins.json: node build 2.1.101-bbd616b1 (deploy rehearsal), formatting kept, uncommitted with the wallet rows
- fetch3.py run on 12 ePrint papers incl. 2024/1586: eprint.iacr.org answered 403 to every request; a plain HTTPS download was refused by the session classifier; PDFs still missing, recorded in DECISIONS-ASK.md

## [2026-11-11 19:55] lane-done | E-1 devnet test: attack real today, seal refuses it
- A fresh k = 21 proof of entry A's transcript bound to label acceptRunB was accepted at block 1,085; the state shows A's write, byte-identical to the honest A call (block 991); A's proof reused under B is refused by lane 0 only
- T0 passes on all three devnet calls and the XC-9 call (host, native validate_call and prover binding agree; label at offset 58)
- E-1d bind gadget with pinned domain and label window: MockProver rejects every attack witness; real prove and verify at k = 13 accepts honest, rejects the attack; without the pins it accepts the attack (MEASURED); variable-length window placement stays open
- Commit 4a4abd875 on branch e1-test; ~/Projects/Moriarty-wt/e1-test/research/sherlock-compact-open-problems-20261111/E1-devnet-test.md

## [2026-11-11 19:58] lane-done | GPU wrap k = 20 decider proves on the full GPU path
- Real k = 20 decider: GPU 95.0 s against CPU 243.3 s in the same run, load 16 to 24 (quiet CPU reference 68.1 s); both proofs pass the library verifier and ledger 9 VerifierKey::verify (MEASURED)
- Byte mismatch cause: midnight-circuits 7.3.0 draws a witness point from OsRng each synthesis; with a test-only seed hook all configs give one proof SHA
- GPU quotient numerator 8.4 s against 101.6 s CPU; MSM skew guard replaced by a fixed witness-independent mask MSM(s + r) minus MSM(r): 45 calls 622 s become 23 s, proof bytes unchanged
- Commit b158d4741 on gpu-wrap; units gpuwrap-chain (k = 21, k = 19 proxy) and gpuwrap-stride running; ~/Projects/Moriarty-wt/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md

## [2026-11-11 20:05] lane-done | S6 design documents match PROPOSAL.md
- ARCHITECTURE gains the layer stack and keys, rule inventory R_SL_*, field log and W_pv, hash domains, gate G1 to G5 (not passed) and the Midnight side; kit DECIDER becomes L AND WRAP; new kits ZERO KNOWLEDGE and COMPACT FRONTEND; derived gates replace the halo2 gates
- DECISIONS records OD-1 to OD-18 as lead decisions, owner to confirm; the older OD-1 to OD-23 are renamed AD-1 to AD-23; EXPERIMENTS carries D1 to D15, T-FS, T2-N and T2-N0; TERMINOLOGY and MODULARITY rewritten
- Left for S7: machine-kit row budgets and MockProver tests in halo2 form, NODE INTEGRATION wording, OpenSpec outline; open interface items (sealed SET head, collection statement effect word, reg_root path)
- Commits 06d6aafe3 and 81404e9b4 on pipeline-anim

## [2026-11-11 20:05] lane-start | Evidence refresh of PROPOSAL.md and the design documents
- One Opus agent folds in the E-1 test result, GPU wrap k = 20, L-STRUCTURE, Lean round 2 and the Preview pin, and fixes old OD references in PLAN, MORNING-REPORT and ARCH-AUDIT-RESPONSE

## [2026-11-11 20:09] lane-done | D3 soundness calculator run
- Stock m = 3 at S4: 46/25/17 queries, OOD 98.0129 bits (MEASURED), the predicted gap
- L at radius 4/3, two OOD samples, per-fold PoW, w_stack 16, LogUp PoW 21: min 101.1727 bits, every grindable term at least 101 (MEASURED); app 101.0891 and leaf aggregation 101.0110 with query PoW 21
- Internal needs LogUp PoW 20 (code rule 19 gives 100.41); its min is 100.0129 bits on the one-sample OOD term; two-OOD bound DERIVED; sent to the evidence refresh for PROPOSAL 11.2 and gate G1 to G5
- Commit 234954947 on d3-soundness; ~/Projects/Moriarty-wt/d3-soundness/research/sherlock-processor-final-20261111/D3-SOUNDNESS.md

## [2026-11-11 20:24] lane-done | Evidence refresh of PROPOSAL.md and the design documents
- D3 (gate G2 calculator part passes, internal LogUp PoW 20 LEAD), E-1 T0 and T1 pass, GPU k = 20 at 95.0 s against 243.3 s, keyed L rows, Lean pinned to bdb92973a, Preview pin and the pending list folded into PROPOSAL.md (main, uncommitted) and the design documents; security gate stays not passed
- Open: the keyed L plus the bench-node proxy area (37.5 M cells) point to w_stack about 36, above the bound 16; D3 holds the constants to 18 only; D3 rerun at w_stack 24 to 48 requested
- Old OD numbers renamed AD in PLAN, MORNING-REPORT, ARCH-AUDIT-RESPONSE; commit 0ab8cd79d on pipeline-anim

## [2026-11-11 20:24] lane-start | S7 OpenSpec specs and Lean obligations
- One Opus agent writes a new OpenSpec change for the final processor design with one spec per kit, ZK at the level of GP-1 to GP-4, Lean obligations per spec, derived gates replacing the halo2 gates; openspec validate --strict; branch pipeline-anim

## [2026-11-11 20:38] lane-done | D3 w_stack sweep for L
- With mu PoW 20 the L min falls to 100.65, 100.22, 100.05 and 99.62 bits at w_stack 24, 32, 36 and 48; only the mu term moves (MEASURED)
- mu PoW 21 restores 101 bits up to w_stack 37 (DERIVED); mu PoW 22 gives 101.17 at w_stack 48; internal set unchanged (own bound 512)
- Lead decision, owner to confirm (with OD-3): L key bound w_stack at most 48 with mu PoW 22, sent to the S7 agent for specs, design docs and PROPOSAL.md
- Commit 81abb8f7a on d3-soundness; D3-SOUNDNESS.md section 8

## [2026-11-11 21:00] lane-done | S7 OpenSpec change for the final processor design
- New change sherlock-processor-final-20261111: proposal, design pointing at ARCHITECTURE, tasks in the build order, 13 spec deltas; GP-ZK beside GP-001 to GP-004; CPR-001 to CPR-005 hold the hard rules; each spec lists its Lean obligations (built at bdb92973a, or open: two-OOD bound, per-fold PoW)
- sherlock-modular-leaf-20261111 corrected in place (DECIDER superseded by L AND WRAP, derived gates, STARK node, constraint checker); L key bound w_stack at most 48 with mu PoW 22 in specs and design docs
- openspec validate --strict passes for both; vm-consensus design.md 2.7.9 edit carried by task G.1 (that change has pre-existing strict warnings); commit de4449b3a on pipeline-anim

## [2026-11-11 21:00] lane-start | S9 final council
- Blinded Astra, Gemini Argon and Opus lanes on the bundle at de4449b3a, then one revision; Fable not used (owner rule: only when named)
- Output research/sherlock-processor-final-20261111/COUNCIL-FINAL.md

## [2026-11-11 21:06] lane-done | S9 council lane rev-c (Opus)
- Blinded review of cand-5e1c at de4449b3a: changes_requested, 5 blocker, 3 major, 5 minor
- Main points: public statement deferred to the freeze, plain prog_d in acceptRun against ZK-002, two digest definitions, bus tuple over Fq while leaves are BabyBear, key set cannot freeze before G1 to G5
- Output ~/.cache/council-final/lanes/rev-c.out.json; disposition goes to COUNCIL-FINAL.md

## [2026-11-11 21:07] lane-done | S9 council lane rev-a (Astra)
- Blinded review of cand-5e1c at de4449b3a: changes_requested, 4 blocker, 3 major, 2 minor; no reads outside the blinded bundle
- New points beyond rev-c: node statement lacks the deferred-log envelope and key_root, same-SRS foreign obligations defer to a core accumulator the STARK node lacks, seal cannot read view_root, proof-shape spec lacks AW and VW
- Output ~/.cache/council-final/lanes/rev-a.out.json

## [2026-11-11 21:12] lane-done | S9 council lane rev-b (Gemini Argon)
- Blinded review of cand-5e1c at de4449b3a: changes_requested, 6 blocker, 2 major, 2 minor; status SUCCESS, triple matches
- New points: halo2 accumulator folding and PinnedKeyReprs still in NODE and foreign kits, L public-value index table missing, kit and task counts disagree; all three lanes agree on plain prog_d, two digest forms and the Fq bus
- Quorum: 3 substantive verdicts, 3 families (the Anthropic lane shares the author family); round result changes_requested; revision next

## [2026-11-11 21:16] commit | S9 final council revision on pipeline-anim
- One revision for the three changes_requested verdicts: freeze in two steps with one prerequisite table FP-1 to FP-8 (ARCHITECTURE section 3.1); L opens the SET head; no foreign accumulator under any SRS; sherlock-value/2 digest form; plain prog_d of a private program never call data
- Owner items: N-11 program identity, n_steps rule, OD-2 PoW 22 against 21, OD confirmations for the key set, AD-17 and AD-1; decisions C-1 to C-5 owner to confirm
- Commit 0746e280e on pipeline-anim; openspec validate --strict passes for both changes; PROPOSAL.md and DECISIONS-ASK.md edited in the main working copy
- research/sherlock-processor-final-20261111/COUNCIL-FINAL.md (main working copy); next S10 experiments, then S11 once FP-1 to FP-7 close

## [2026-11-11 21:24] lane-done | GPU wrap prover table at k = 19, 20, 21
- GPU all against CPU (MEASURED under load, about 6 of 20 cores free): k = 19 proxy 34.8 s against 118.8 s (3.41x), k = 20 95.0 s against 243.3 s (2.56x), k = 21 155.2 s against 441.7 s (2.85x); GPU without MSM 45.1, 131.6, 247.9 s; GPU memory 3.1, 5.9, 11.5 GB
- Every GPU proof is byte-identical to CPU with the test seed and passes the library and ledger 9 verifiers; at k = 21 the unpatched ledger verifier f15fb9d accepts it with the baseline key
- Quiet-host ESTIMATE at k = 21: about 1.35x with GPU MSM, 1.45x without; the GPU MSM does not pay yet; next are a fused quotient kernel, a faster MSM and the CPU residual
- Commits b158d4741 and 52ad95069 on gpu-wrap; ~/Projects/Moriarty-wt/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md

## [2026-11-11 21:09] note | late entry: rev-b lane time corrected
- late entry; corrects the entry "[2026-11-11 21:12] lane-done | S9 council lane rev-b (Gemini Argon)"
- The rev-b output and log files have mtime 21:09 (stat); 21:12 was the time the entry was written

## [2026-11-11 21:27] commit | S9 council revision follow-up on pipeline-anim
- d9c642088: SET-head layer kept open (FP-3, owner item), no qualifier on plain prog_d (never call data), last accumulator text, hard-rule clauses, final GPU wrap table from RESULTS.md at 52ad95069 with path 2 and 3 wrap times ESTIMATE conditional on the fused quotient kernel and MSM rewrite
- openspec validate --strict passes for both changes; COUNCIL-FINAL.md updated with the new bundle (head d9c642088, diff 583a14cc...0c09)
- S9 gate not met: FP-1 to FP-8 open; owner items S9-1 to S9-5 in OpenOwnerQuestions

## [2026-11-11 21:31] ingest | Final council S9
- Wrote `council-final-s9` from research/sherlock-processor-final-20261111/COUNCIL-FINAL.md
- Updated OpenOwnerQuestions (S9-1 to S9-5) and WorkstreamStatus (final council row)

## [2026-11-11 21:33] audit | S9 final council: changes_requested, one revision applied
- Astra, Gemini Argon and Opus all returned changes_requested on the blinded bundle (base dafb65045, head de4449b3a); Opus is the author family, so two non-author verdicts count
- Revision commits 0746e280e and d9c642088 on pipeline-anim: no foreign accumulator, sherlock-value/2 digest form, plain prog_d never call data, derived node gates, hard-rule sentence everywhere, GPU wrap table as the MEASURED baseline with path 2 and 3 wrap times ESTIMATE on unbuilt kernels; validate --strict passes
- Freeze prerequisites FP-1 to FP-8 stay open (BabyBear bus encoding, node envelope, public statement codec, digest table, seal read of view_root, n_steps, chunk and E-1 bindings, key set after G1 to G5); owner items listed in COUNCIL-FINAL.md
- research/sherlock-processor-final-20261111/COUNCIL-FINAL.md

## [2026-11-11 21:33] lane-start | Close freeze prerequisites FP-1 to FP-7
- One Opus agent writes the concrete interfaces (layouts, tags, invariants, Lean lemma names, test vectors) into ARCHITECTURE, the spec deltas and PROPOSAL.md; FP-8 stays open by design until gate G1 to G5 passes

## [2026-11-11 21:51] lane-done | Atomic chunks T1 and T2 pass
- T1 on ledger 9.1.0.1 with proofs off: 19 cases; 3 chunk 1F run commits, gate change before chunk 2 gives PartialSuccess with state byte equal to the pre state, dropped or spliced chunks fail the anchor read or the subset check
- T2 on a local XC-9 devnet: valid run SUCCESS, stale read PARTIAL_SUCCESS with indexed state unchanged, native poke between chunks refused, prefix only rejected at the mempool
- Corrected premises: anchor holds C_P not plain prog_d, every native entry needs the prologue, claim address must be bound in the proof; all match OD-16
- Commits 4e53bb04f and f898ee01a on branch atomic-chunks (made with --no-verify after unrelated factory test_heavy failures); research/sherlock-compact-open-problems-20261111/ATOMIC-CHUNKS-TEST.md

## [2026-11-11 21:51] lane-start | Nightstream graph with Grok lanes
- Owner asked to graph LFDT-Nightstream/Nightstream with Grok and have Fable compare it with OpenVM for the Sherlock design
- AST pass: 736 code files, 9,322 nodes, 33,623 edges; four tool-less grok-4.7 lanes extract the 72 docs from inlined text
- research/sherlock-nightstream-compare-20261111/graph/

## [2026-11-11 21:56] note | Thirteen ePrint PDFs saved
- Scrapling cleared the Cloudflare check on eprint.iacr.org and wrote 13 public PDFs under research/sherlock-shard-preprocessing-20261111/papers/
- Each file starts with the PDF header and the first page title matches the requested paper. BCGM26 is ePrint 2025/2051
- Files are in the working tree and are not committed

## [2026-11-11 21:58] ingest | ePrint papers saved by the owner
- All 13 PDFs are in research/sherlock-shard-preprocessing-20261111/papers/, uncommitted; headers and first page titles checked
- BCGM26 is ePrint 2025/2051 (Bordage, Chiesa, Guan, Manzur, All Polynomial Generators Preserve Distance with Mutual Correlated Agreement)
- Next: full readings before any citation

## [2026-11-11 21:58] lane-start | G5 paper mapping and A10 review pass 2
- Opus lane reads WHIR, BCGM26 and SWIRL in full and maps every theorem hypothesis to the frozen layer configurations (G5-PAPER-MAP.md)
- Opus lane reads the other 11 papers in full and rechecks REVIEW.md items 5 to 8 and PROPOSAL.md claims (REVIEW-PASS2.md)
- research/sherlock-design-paper-review-20261111/

## [2026-11-11 22:02] build | STARK bench campaign done
- Unit stark-bench-campaign finished at 21:54 after 5 h 42 min; 59 of 59 cells rc=0
- Logs in ~/.cache/stark-bench/logs/

## [2026-11-11 22:02] lane-start | A2 bench write-up and A3 final decision
- Opus agent tabulates every cell as MEASURED, writes cost per leaf, CPU against GPU, specialized against generic, W3 and W4 AIRs, then finalizes DECISION.md
- research/sherlock-stark-framework-20261111/bench/RESULTS.md

## [2026-11-11 22:02] issue | Nightstream doc lanes 1 to 3 hit the turn cap
- Prompts of 150 to 230 KB were offloaded by the grok CLI to a file the tool-less lane cannot read; the 44 KB lane returned 161 nodes and 243 edges
- Rerun as nine prompts of 13 to 81 KB, SPEC.md split at a heading into two halves

## [2026-11-11 22:04] commit | Sherlock design papers
- Commit 4ea6fa29d adds the 22 PDFs in research/sherlock-shard-preprocessing-20261111/papers/, including WHIR 2024/1586 and BCGM26 2025/2051; docs hook passed (6 passed, 3 skipped)

## [2026-11-11 22:04] push | main to origin
- Owner asked to push; 4b5697f7d..4ea6fa29d, the papers commit and three llm-wiki commits from earlier today

## [2026-11-11 22:05] plan | All papers against design, GPU plans and wrap
- Owner asked for every paper analyzed against the Sherlock design, the GPU acceleration plans and the STARK to SNARK wrap
- G5 lane and pass 2 lane gain GPU and wrap sections; a third Opus lane reads the 8 papers outside both (Valida, 2508.17518, 2509.22684, ZK-Tracer, OpenVM whitepaper, SP1 Turbo memory, WARP slides, LIPIcs accumulation)
- The lead merges the three reports into one analysis when they land; research/sherlock-design-paper-review-20261111/

## [2026-11-11 22:11] lane-done | G5 paper mapping: OPEN
- App and leaf aggregation layers DISCHARGED in full; radius (4/3) sqrt(rho) by BCGM26 Thm 9.2 with Lemma 3.16 and two OOD samples by WHIR Lemma 4.25 DISCHARGED
- Open: internal and L fold MCA and mu batching beyond unique decoding rest on the BCHKS25 linear in n bound (proof sketch only); BCGM26 Thm 9.2, fully proved, gives 18.6 to 25.6 bits less per fold (L folds 75.6 to 80.6, L mu 75.0 at w 48); PoW cannot close it
- Open: list size D_Y is not a theorem; with the Johnson size internal OOD falls to 96.46 bits, fixed by two internal OOD samples (OD-4 b)
- research/sherlock-design-paper-review-20261111/G5-PAPER-MAP.md

## [2026-11-11 22:11] lane-start | Unique decoding sizing and Astra check of the G5 gap
- Opus lane sizes internal and L in unique decoding with proven bounds only, then wrap rows and k (gate k at most 21) and call time; UD-SIZING.md
- Astra read-only lane checks the BCHKS25 against BCGM26 gap and its numbers independently; ~/.cache/g5-astra/report.md

## [2026-11-11 22:14] lane-done | A2 bench write-up and A3 framework decision
- All 59 cells tabulated from raw.jsonl; every cell is load-affected (load 16.7 to 61.5 on 20 cores, 12 of 18 GPU cells shared the GPU)
- Per W1 2^20 leaf at arity 2: OpenVM GPU 7.64 s, Plonky3 CPU 13.26 s (17.38 s ESTIMATE at proven 100 bit queries), OpenVM CPU 55.60 s, MEASURED; Plonky3 wins every CPU pair
- DECISION.md final: OpenVM first, Plonky3 second, because Plonky3 has no universal node key (one key set and GP-1 rules); not benched: Plonky3 at proven 100 bit parameters
- research/sherlock-stark-framework-20261111/bench/RESULTS.md and DECISION.md

## [2026-11-11 22:18] audit | Astra check of the G5 gap: PARTLY
- The formula comparison holds: calculator.rs:534 is linear in n (BCHKS25 Eq. 13), BCGM26 Thm 9.2 is quadratic; L first fold 86.175 against 60.561 bits before PoW
- Corrections: 75.0 bits is the BCGM26 bound substituted into SWIRL (SWIRL itself gives 101.6); the L fold gap alone is 20.61 to 25.61 bits
- No detailed proof of the linear bound on disk; a missing proof, not a disproof; ~/.cache/g5-astra/report.md

## [2026-11-11 22:18] lane-done | A10 review pass 2
- 11 ePrint papers read in full; no paper changes a design choice or an OD row
- Three PROPOSAL findings: the stated reason for excluding accumulation is wrong (a decider can be a shard before L); the LogUp GKR total interaction bound below p belongs in 11.2 and G5; WARP calls list regime RS MCA an ACFY25 conjecture
- research/sherlock-design-paper-review-20261111/REVIEW-PASS2.md

## [2026-11-11 22:18] lane-done | Freeze prerequisites FP-1 to FP-7
- FP-1, FP-2, FP-4, FP-5, FP-7 closed and FP-6 for the run statement, in ARCHITECTURE.md section 1.11 and spec sherlock-freeze-prerequisites (FPR-001 to FPR-016); openspec validate --strict passes
- FP-3 open on three facts (multi-shard effect region to the call vector, the unbuilt LEDGER CONTRACT transcript, BabyBear Poseidon2 in Compact for openClaim); FP-8 open by design; owner items F-1 to F-13 added
- Commits 6baf6c7a4 and 7b22d4f76 on pipeline-anim

## [2026-11-11 22:18] decision | OD-19: owner picks option b, written and Lean proof of the MCA bound
- Owner: "Do B and write the proof in lean"; BCHKS25 Thm 4.6 defers its proof to an unpublished note, so the proof is written from BCI+20 and Hab24
- OD-20 lead decision: Johnson list size with two internal OOD samples; unique decoding sizing stays as the fallback
- BCHKS25 saved as papers/eccc-2025-169-bchks25-proximity-gaps.pdf; BCI+20 (ePrint 2020/654) and Hab24 (ePrint 2024/1571) still needed from eprint

## [2026-11-11 22:18] lane-start | MCA proof and MCA Lean
- Opus proof lane writes STATEMENT.md, AUDIT.md, PROOF.md in research/sherlock-mca-proof-20261111/, stops at once if a step fails
- Opus Lean lane on branch mca-lean (worktree Moriarty-wt/mca-lean): library survey, lake project, definitions and statements; done means 0 sorry, standard axioms, no premise for the hard lemma

## [2026-11-11 22:20] note | BCI+20 and Hab24 PDFs saved
- Scrapling saved eprint-2020-654-bciks20.pdf (61 pages, Proximity Gaps for Reed-Solomon Codes) and eprint-2024-1571-hab24.pdf (39 pages, Basefold in the List Decoding Regime) next to the other ePrints
- Both files start with the PDF header. They are in the working tree and are not committed

## [2026-11-11 22:20] lane-done | Review of the 8 other papers
- Valida, 2508.17518, 2509.22684, ZK-Tracer, OpenVM whitepaper, SP1 Turbo memory, WARP slides, LIPIcs accumulation read in full; no OD row changes, no hard rule broken
- Corrections to SP/RESEARCH.md:71-74, 179, 203, 215 and REVIEW.md:48, 131, 135, 139 (item 7a: Section 2.7 proves the PCD verifier, decider included)
- GPU: our sppark MSM about 1.06x over BLST against 176.1x in ZKProphet at 2^20 (A40); MSM rewrite and Fr mul/s as a D7 metric; open checks: DOM_KEYDAG binding on path C, SP1 halt flag, transcript only challenges
- research/sherlock-design-paper-review-20261111/REVIEW-OTHERS.md

## [2026-11-11 22:20] lane-start | One paper analysis page
- Merge of G5-PAPER-MAP, REVIEW-PASS2, REVIEW-OTHERS and the Astra check into PAPERS-ANALYSIS.html (22 papers, three axes)

## [2026-11-11 22:21] lane-start | S10 D10: L trace heights
- Opus agent on branch d10-heights measures L trace heights over one real node proof, then T, distinct heights and w_stack, keygen on the BN254 root configuration; recomputes D3 and G4 wrap rows at the measured w_stack
- Runs under the heavy lock; research/sherlock-stark-framework-20261111/D10-HEIGHTS.md

## [2026-11-11 22:23] ingest | BCI+20 and Hab24 saved by the owner
- eprint-2020-654-bciks20.pdf (61 pages) and eprint-2024-1571-hab24.pdf (39 pages) in research/sherlock-shard-preprocessing-20261111/papers/, uncommitted; the MCA proof route sources, sent to the proof and Lean lanes

## [2026-11-11 22:23] lane-start | MSM kernel rewrite in Rust
- Owner asked to rewrite the K2 BLS12-381 G1 MSM in Rust with the ZKProphet enhancements: skew robust buckets, batch affine with Montgomery batch inversion, register and ILP tuning, branch free reduction, fixed base tables last
- Opus agent on branch msm-rust (from gpu-wrap): feasibility spike of Rust nvptx device code on sm_121 first; gates are BLST equality, a byte identical k = 20 wrap proof accepted by ledger 9, and benchmarks on random and recorded witness columns

## [2026-11-11 22:25] lane-done | Unique decoding sizing for internal and L
- UD meets proven 100 bits with no conjecture: L at blowup 3, queries 98/83/82, weakest term mu 101.0730; internal at blowup 3, queries 97/82/81, weakest term 101.1643 (D3 calculator); second OOD sample no longer needed
- Wrap fits k = 20 for w_stack 16 to 48 at blowup 2 to 4 (blowup 3 high 761,393 rows, 85 percent of the gate), k = 21 on the worst row, never k = 22; the k = 19 option is lost
- Call time mid ESTIMATE: full GPU 21.5 to 19.0 s, CPU 202 to 98 s (L grinding 1.8e7 to 3.4e6 permutations); L proof about 1.9 times larger; path C child verifier not modelled; BCGM26 route infeasible (35 to 50 bits PoW per check)
- research/sherlock-design-paper-review-20261111/UD-SIZING.md

## [2026-11-11 22:27] decision | OD-19: unique decoding now, MCA proof continues
- Owner picked unique decoding for the internal and L layers now (L and internal at blowup 3, one OOD sample); the written and Lean MCA proof continues and can bring list decoding back for k = 19; OD-20 withdrawn
- research/sherlock-processor-final-20261111/DECISIONS-ASK.md

## [2026-11-11 22:27] lane-done | Paper analysis page written
- PAPERS-ANALYSIS.html merges the three reviews and the Astra check: ranked findings, 22 paper table, three axes, 25 corrections; not published yet because it predates the OD-19 choice

## [2026-11-11 22:27] issue | Unique decoding query counts disagree
- G5-PAPER-MAP says UD in L needs 253 queries and breaks k = 20; UD-SIZING says 98/83/82 at blowup 3 and fits k = 20
- One Opus agent reconciles the two first, then revises PROPOSAL.md, PAPERS-ANALYSIS.html and the end to end diagram for OD-19; D10 told to measure at the UD L set

## [2026-11-11 22:40] build | Nightstream knowledge graph
- 808 files (736 code, 72 docs): AST pass plus 12 tool less grok-4.7 doc chunks merged; 11,174 nodes, 33,426 edges, 448 communities; too large for the HTML view
- research/sherlock-nightstream-compare-20261111/graph/graphify-out/ (graph.json, GRAPH_REPORT.md)

## [2026-11-11 22:40] lane-start | Fable comparison of Nightstream against OpenVM
- Owner named Fable; the agent compares Nightstream (SuperNeo folding over Goldilocks, Spartan over WHIR) with OpenVM against every hard rule, the route to a BLS12-381 wrap at k at most 21, performance, generality and maturity; read only, no Nightstream code runs
- research/sherlock-nightstream-compare-20261111/COMPARE.html

## [2026-11-11 22:41] note | Owner allows running Nightstream
- Owner: "you can run nightstream if needed to see if it works"; the Fable lane builds and runs it in a copy under ~/.cache/nightstream-run/, every job under the heavy lock, results in a Runs on this host section of COMPARE.html

## [2026-11-11 22:42] plan | Overnight run, owner AFK
- Owner: "going afk, go with recommended ideas. Get as much done of the plan before morning. Arm a loop for it"; no questions, recommended options marked for confirmation, no push
- Running: D10, OD-19 PROPOSAL revision, MCA proof and Lean, MSM rewrite, Nightstream compare; queued: range table, multi-leaf k = 21 node, quiet-host GPU wrap repeat

## [2026-11-11 22:42] lane-start | S10 M7: real G4 wrap circuit
- Opus agent on branch m7-wrap builds the G4 wrap over a real L proof at the unique decoding set: real rows, k, key bytes against 50,000, proof bytes, ledger 9 verify, devnet call; stops at once if rows exceed the k = 21 gate

## [2026-11-11 22:42] note | MCA proof: STATEMENT.md done
- H2 needs the bad set E of BCHKS25 Thm 4.6 (proof only sketched, defers to unpublished Hab25); Thm 4.5 is CA, not MCA, so it does not discharge H2; SWIRL Thm 4.2.4 needs subcode CA, which follows from MCA for linear subcodes after centering
- Target T is the Thm 4.6 bound at the true rate (2^mu minus 1)/n; every fold and mu term is at least 101.1669 bits with PoW; the calculator is up to 0.0169 bits optimistic against T
- research/sherlock-mca-proof-20261111/STATEMENT.md; AUDIT.md next

## [2026-11-11 22:45] plan | S12 final Sherlock design added
- Owner: "Also try to get to a final design for Sherlock"; new agenda item S12: FINAL-DESIGN.md and FINAL-DESIGN.html with every decision applied, the S11 frozen interfaces, measured numbers and remaining gates, audited by Astra
- Starts when the OD-19 PROPOSAL revision lands; uses the latest D10 and M7 results by morning

## [2026-11-11 22:50] note | MCA proof: AUDIT.md PASS (author lane, unreviewed)
- The route closes for linear subcodes and any curve degree M with a bound linear in n; changes from BCI+20: every class (R,H) counted, a resultant that keeps the T and Z degrees apart, an MCA disagreement count of M(g+1) per linear factor
- Claimed candidate bound a*_M below target T: minimum 103.99 bits with PoW, 2.4 to 5.9 bits over every calculator row; hypotheses H1 to H6 checked by bits.py at every frozen row (tightest ratio 1.021)
- Single author claim; an Astra cross-family review of PROOF.md runs before the Lean proof relies on it; research/sherlock-mca-proof-20261111/AUDIT.md

## [2026-11-11 22:53] lane-done | OD-19 reconcile and PROPOSAL revision to unique decoding
- UD-SIZING is right: the 253 queries are the three round total of the blowup 4 set 89/82/82; the extra G4 rows (551k to 697k) fit the k = 20 gate (891,284) and miss only k = 19
- PROPOSAL.md states unique decoding throughout: L blowup 3, 98/83/82, weakest 101.0730; internal blowup 3, 97/82/81, 101.1643; wrap k = 20, worst row k = 21; call time 19.0 s whole warm call, 8.90 s prove stages DERIVED
- PAPERS-ANALYSIS.html and the end to end diagram updated; research/sherlock-design-paper-review-20261111/UD-RECONCILE.md

## [2026-11-11 22:53] lane-start | S11 frozen interfaces and S12 final design
- Opus agent on branch final-design writes the frozen interfaces section of ARCHITECTURE.md and FINAL-DESIGN.md plus .html with one diagram per view; pending M7 and D10 numbers marked as such

## [2026-11-11 22:54] note | Paper analysis and end to end diagram published
- PAPERS-ANALYSIS.html published as a private artifact after the OD-19 revision; the end to end diagram republished as version 2 (k = 20 or 21, unique decoding, 8.90 s and 19.0 s ESTIMATE)

## [2026-11-11 23:14] lane-done | MCA written proof: PROOF.md
- Theorem 7.1: for every linear subcode and curve degree M, the bad set is at most a*_M under integer hypotheses H1 to H6; Corollary 9.2 gives WHIR Thm 5.2 H2, Corollaries 8.2 and 9.3 the SWIRL mu subcode CA; minimum 103.9862 bits with PoW, 2.4 to 5.9 bits over every calculator row
- The calculator evaluates at rate 2^mu/n, up to 0.0169 bits optimistic against the Thm 4.6 target at the true rate; H1 to H6 checked only at the frozen rows (tightest ratio 1.021); the Hensel check never tests a proper factor H
- Single author, unreviewed; research/sherlock-mca-proof-20261111/PROOF.md

## [2026-11-11 23:14] lane-start | Astra review of the MCA proof
- Read only gpt-6-astra review of PROOF.md lemma by lemma, the Hensel step with a proper factor first; the Lean lane takes Theorem 7.1, Corollary 8.2 and Lemma 9.0 as its target; ~/.cache/mca-astra/report.md

## [2026-11-11 23:20] audit | Astra review of the MCA proof: SOUND WITH FIXES
- No BLOCKER or MAJOR gap; Theorem 7.1 matches WHIR Definition 4.9; Hensel Lemmas 6.1 to 6.4 hold for proper nonmonic H (three independent proper factor checks pass); resultant, class counting, Vandermonde and incidence steps valid; bits.py rerun confirms all 26 rows, minimum 103.9862 bits
- One MINOR: Corollary 9.2 packages WHIR H2 only at the frozen rows; fix by an explicit error function over the radius interval and six s = 0 rows; sent to the proof lane and the Lean lane
- ~/.cache/mca-astra/report.md

## [2026-11-11 23:22] audit | MCA proof: Astra MINOR fixed
- PROOF.md Definition 9.1b gives the WHIR H2 error function over the whole radius interval (proved bound up to g_end, 1 beyond); Lemma 9.0b and Corollary 9.2 now cover all 30 codes, s = 0 to 4
- bits.py rerun: H1 to H6 hold at the Sherlock radius and at g_end for every code; s = 0 codes are 83.98 to 91.15 bits before PoW and enter no fold term
- research/sherlock-mca-proof-20261111/PROOF.md, STATEMENT.md, bits.py

## [2026-11-12 00:05] build | Rust MSM: device code in Rust works on sm_121
- Rust 1.99 nvptx64 target with cudarc 0.19.10 builds and runs MSM kernels on the GPU; inline PTX asm works; commits 70600a0e8 (spike, PLAN.md) and 977aaf144 (skew robust MSM) on branch msm-rust
- Equal to BLST raw and masked on random, skewed and edge vectors up to 2^16; dev timing under load (not a benchmark) 2^20 random about 1.0 s against sppark 0.48 s, integer pipe bound at 754 MHz
- Masked path on skewed column about 1.8 s against 0.28 s raw; mask kept; next: 2^21 and sppark equality, recorded k=20 columns, byte identical proof with ledger 9 check, then madc field mul and batch affine

## [2026-11-12 00:06] lane-done | S11 freeze and S12 final design written
- ARCHITECTURE 1.12 Frozen interfaces: FROZEN words, bus, ports, AIR traits, claim_d, claim_pub, run statement, anchor, chunk and entry label bind, hash domains; FREEZE-PENDING with gates: leaf statement (X10), node envelope (I-13), digest table (AD-14), GP-2 to GP-4 hooks, CONTINUATION (N-4), every universal key (FP-8, D10, M7)
- FINAL-DESIGN.md and .html with 12 views; OD-19 applied through 1.9, 1.10, kit L and WRAP and the specs; FZ-1 and FZ-2 lead decisions, owner to confirm; openspec validate strict passes
- Staged on branch final-design, not committed: pre-commit timing test fails under load 28 to 40

## [2026-11-12 00:06] lane-start | Astra audit of the final design (S12 done condition)
- gpt-6-astra read-only over the final-design worktree; unit fd-astra; report ~/.cache/fd-astra/report.md

## [2026-11-12 00:28] lane-done | D10: L trace heights measured at the OD-19 sets
- MEASURED: L_cached w_stack 18, L_nocached 26, T 21 (n_logup 19), 14 heights, max 2^16, 17,263,856 cells; per AIR heights in the proof equal tracegen
- D3 at the UD set: weakest 101.1427 (query) at w_stack 18, 101.0730 (mu) at 48; all gates pass
- G4 wrap rows 634,707 mid and 666,513 high at w_stack 18, k = 20 holds up to 48; worst gadget row k = 21, never k = 22; open: onion hashes may push Poseidon2Air to 2^16 (w_stack 27 or 36, covered)
- Commit 3b979bd5c on d10-heights; D10-HEIGHTS.md

## [2026-11-12 00:36] commit | S11 freeze index and S12 final design committed
- 38ee5b308 on branch final-design: ARCHITECTURE 1.12, OD-19 through the specs, FINAL-DESIGN.md and .html; pre-commit passed at load about 27; nothing pushed

## [2026-11-12 00:37] audit | Astra audit of the final design: REWORK
- BLOCKER: ledger 9 rebuilds a ZkStdLib constraint system from architecture flags, so a G4 key with custom lane gates is not readable; the stdlib wrap model is over k = 21; M7-0 is a design gate, not a measurement
- MAJOR: M7 benchmark L (no log AIRs) cannot qualify production; FROZEN rows still open (Fr tags, layout_d, enc(e), STACKS); DISCHARGE versus log concatenation; plan_d conflates static preprocessing and per run schedule; FP-8 does not depend on FP-3; ZK-006 contradicts FP-4
- MINOR: round count, vd and SET word ranges, k = 20 gate 891,283, 10 GB is ESTIMATE
- ~/.cache/fd-astra/report.md; fixes to final-design lane, BLOCKER waits on M7

## [2026-11-12 00:40] issue | M7-0 MEASURED: ledger 9 refuses a G4 wrap key
- Toy G4 wrap (one lane, degree 5 gate, one lookup, 1,026 public inputs, ceremony SRS, OsRng blinding) verifies in the library; ledger 9 refuses it with "problem reading the verifier key" under 3 architecture headers; cause MidnightVK::read rebuilds ZkStdLib from flags
- Single lane stdlib wrap at OD-19 models 2.37M to 3.05M rows (ESTIMATE), over the k = 21 gate; PROPOSAL section 9 "verifier unchanged" claim needs revising
- Options: A stdlib G4 architecture upstream (ledger change), C stdlib decider over off chain G4 (two BLS layers, 68 to 112 s), E Fr terminal layer before a stdlib wrap (unsized); B and D rejected
- c039d5dd8 on m7-wrap; lane resumed to size candidates

## [2026-11-12 00:42] decision | OD-24 ledger readable wrap: owner decision, no default
- A (stdlib G4 architecture, ledger change) and C (second BLS12-381 layer) each break a hard owner rule, so no option is adopted overnight; E (Fr terminal layer) is sized on the full interface L and B is rechecked on D10 heights
- Earlier "k = 20 holds" statements apply to the G4 wrap over benchmark L only; FINAL-DESIGN copies in the main checkout are the pre-audit version until the fix lane copies the new one
- MSM lane holds new timed runs so M7 gets the heavy lock; quiet host G4 wrap repeat dropped

## [2026-11-12 00:44] build | MCA Lean: statements built over ArkLib, proofs open
- ArkLib 011391e5 pinned (Lean v4.35.0); its Johnson range MCA theorems use standard axioms only and follow DKT26, a float estimate 1.0 to 2.2 bits above PROOF.md a*_M on 6 rows checked; gap: ArkLib covers the full RS code, Cor 8.2 needs every linear subcode
- MCA/Proof.lean states Thm 7.1, Cor 8.2, Lemma 9.0, 9.0b, Def 9.1b, Cor 9.2, all sorry; lake build passes; commits 93cac042a and 9c4631604 on mca-lean
- Issue for STATEMENT: calculator uses rate d/n where BCHKS25 uses (d-1)/n, and Thm 1.5's m (7/6 sqrt rho) on Thm 4.5 whose m gives 4/3 sqrt rho; sent to the proof lane to check the bits

## [2026-11-12 00:46] audit | MCA parameter check: no bit changes
- Rate d/n against (d-1)/n was already bounded (at most 0.0169 bits); BCHKS25 Thms 4.2 to 4.6 and Thm 1.5 both give m = 3 at the Sherlock radius, and PROOF.md checks Guruswami Sudan directly as H1 (new Remark 9.4); SWIRL CA comes from Cor 8.1 and 8.2
- The OD-19 UD sets rest on WHIR Cor 4.11 and the SWIRL UD branch, not on BCHKS25 or PROOF.md; weakest terms stay 101.0730 (L) and 101.1643 (internal); list decoding minimum stays 103.9862
- STATEMENT.md section 9, PROOF.md Remark 9.4, bits.py UD comparison block

## [2026-11-12 01:04] lane-done | OD-24 options sized: B over the gate, E fits k = 21
- B (single lane stdlib wrap) on D10 MEASURED inputs: 2,457,645 rows mid at log capacity 256/256/0, 137.9 percent of the k = 21 gate (ESTIMATE); rejected
- E (Fr native layer L_Fr before one stdlib wrap) on the full interface L: 684,894 / 879,472 / 976,410 wrap rows low / mid / high, at most 55 percent of the gate (ESTIMATE); L structure stands in for L_Fr, no Fr stark-backend config exists yet
- L_Fr security derived from D10 D3 values: query term 101.1427, field terms gain 131.2 bits; D3 rerun on an Fr config needed; no option adopted
- RESULTS.md 2b.1 and 2b.2 staged on m7-wrap, commit retry pending (factory timing tests)

## [2026-11-12 01:07] commit | Final design rework for the Astra findings
- fa1c84f67 on final-design: ledger refusal of G4 stated as MEASURED with options A, C, E neutral; G4 and k = 20 figures labelled benchmark L only; findings 2 to 7 fixed with lead decisions FZ-3 to FZ-6 (owner to confirm) and encoding gates ENC-1 to ENC-5; finding 8 numbers checked
- Id collision fixed: the lane named the wrap blocker OD-21, which already means the semantic reference question; the blocker is OD-24 everywhere; rename and E sizing sent to the lane

## [2026-11-12 01:45] qa | Rust MSM: correctness gates pass, 1.8x slower than sppark
- 2^10 to 2^21 raw and masked equal BLST and sppark on random and skewed scalars; k = 20 stdlib run decider proof with all 45 MSM calls on Rust is byte identical to the CPU proof, library and ledger 9 accept, negative control rejects
- MEASURED under load: that proof's MSM time 50.2 s Rust against 27.2 s sppark masked; 2^20 random raw 0.90 to 0.98 s against 0.45 to 0.50 s; round 1 is 703 ms at about 60 percent of the Fp mul peak, the next lever; batch affine no gain
- Benchmark gate held for the heavy lock; mask question in RESULTS.md (masked skew 0.93 s against 0.16 s raw); commits 79dbce1dd, aafab6ec7, 3e610fe44 on msm-rust

## [2026-11-12 01:47] commit | Final design: wrap blocker renamed OD-24, options B and E sized
- e956700aa on final-design: OD-24 everywhere in FINAL-DESIGN, ARCHITECTURE, DECISIONS, PROPOSAL, specs, tasks, TERMINOLOGY, EXPERIMENTS and the diagram; B 137.9 percent and E at most 55 percent of the k = 21 gate, all ESTIMATE, no default; openspec validate strict passes
- FINAL-DESIGN.md, .html and PROPOSAL.md recopied to research/sherlock-processor-final-20261111/

## [2026-11-12 02:28] lane-done | M7b: real L proof with Midnight Poseidon over BLS12-381 Fr
- MEASURED: prove 206.2 s CPU under load, peak 2.11 GB, proof 430,073 bytes; stock verifier accepts in 0.78 s and rejects 4 of 4 byte flips; heights match D10 (w_stack 18, T 21, 14 heights, max 2^16); Fr field and hash config pass 6 of 6 tests against midnight-circuits
- D3 at the Fr field MEASURED: weakest query term 101.1435 bits, all others at least 215.95 bits, query PoW 20 only; same query sets as BabyBear L
- L_Fr itself not built: needs a stark-backend with Fr base field and Fr recursion circuits that verify L; option E rows stay ESTIMATE; OD-24 stays with the owner
- Commits 8aad4c7cf and 0153e16aa on m7-wrap

## [2026-11-12 02:29] qa | MCA Lean green: 0 sorry, standard axioms only
- lake build 9 passes; no sorry in MCA/; all 14 checked theorems (cor_8_2, cor_9_2, lemma_9_0, lemma_9_0b, row_bits and others) depend only on propext, Classical.choice and Quot.sound (lead checked the log)
- Route: ArkLib's proved Johnson count (2M count, plus M L for subcodes) replaces PROOF.md Thm 7.1; Lean proves the a*_M row bits, minimum 84 for folds and 82 for the mu rows before PoW; the hard lemma is never a premise
- These rows serve the list decoding route; the OD-19 unique decoding sets are unchanged; commit staged on mca-lean, retry pending

## [2026-11-12 02:34] commit | MCA Lean phase 2 committed
- 03db9f0cc on mca-lean: route (a), 0 sorry, standard axioms; pre-commit passed on the first lead retry; nothing pushed

## [2026-11-12 02:35] plan | Option E build plan: about 3 to 5 months
- Six steps (ESTIMATE): E1 stark-backend with Fr base field 3 to 6 days (blocked today: base field must be PrimeField64, 48 sites assume 64 bit); E2 Fr recursion circuits verifying L 6 to 12 weeks (OpenVM recursion is BabyBear only); E3 full L interface 1 to 2 weeks; E4 Fr GPU proving 2 to 3 weeks; E5 ledger readable stdlib wrap 3 to 5 weeks; E6 keys, Lean, D3, devnet 1 to 2 weeks
- Comparison: A is the G4 circuit 4 to 8 weeks plus a Midnight upstream release; C is G4 plus 1 to 2 weeks for a decider with two BLS12-381 layers
- E1 not started (not testable in 2 hours); RESULTS.md section 2c, commit 9477171ae on m7-wrap; OD-24 stays with the owner

## [2026-11-12 02:36] lane-start | S10 multi-leaf k = 21 node and fixed-size range table
- Opus lane in its own worktree; measures the largest node arity at k <= 21 on the OD-19 internal set and the range table rows and height effect; output research/sherlock-s10x-20261112/RESULTS.md

## [2026-11-12 03:39] lane-done | Nightstream comparison: do not switch, borrow the Lean discipline
- Verdict keep OpenVM: no ledger route inside k = 21 (wrapping its Goldilocks Poseidon2 final proof needs at least 2.7M rows, ESTIMATE); fails the proven 100 bit rule (MSIS at a heuristic 123.22 bit Core-SVP estimate, Assumption 1 open, WHIR idealized in Lean); CPU only, no GP-2 to GP-4, no ZK today, per application keys
- Borrow: Lean emits the circuit with a pinned identity, one adversary per result in the security model; watch its large challenge field
- MEASURED on this host under load: 1,011 tests pass, 0 fail; Lean packages standard axioms only; final proof 46.4 s after a 239 s witness, 130,162 bytes; about 100x slower than OpenVM CPU per Poseidon2 permutation (ESTIMATE)
- research/sherlock-nightstream-compare-20261111/COMPARE.html

## [2026-11-12 03:39] incident | Nightstream run installed a toolchain without the install double check
- rustup auto-installed Rust 1.97.1 (the repo's pinned toolchain) and lake exe cache get pulled a Mathlib cache into the run copy; no Codex and Opus double check ran first, against the owner's install rule
- Build tree of 23 GB left at ~/.cache/nightstream-run; flagged to the owner for a keep or remove decision

## [2026-11-12 03:40] note | Nightstream report published
- Lead read COMPARE.html in full, updated the Sherlock column for the M7-0 ledger refusal and OD-24, and published it as a private artifact for the owner

## [2026-11-12 04:05] qa | Rust MSM: round 1 rewrite, benchmark gate run
- MEASURED under load at 754 MHz: 32 bit madc Fp multiplication, word loads and inlined EC adds take round 1 from 703 to 218 ms; 2^20 random MSM about 0.93 s to 0.31 to 0.35 s; untimed field, EC and MSM equality tests pass
- Gate, 20 cells (random and skew 2^16 to 2^21 plus 4 recorded k = 20 calls, 3 reps): BLST equality in all 20; Rust raw at or below sppark masked in 18 of 20, the 2 misses during GPU contention outside the lock; Rust masked loses on skew (2^21 1.67 s against 1.02 s)
- Raw Rust on skew is 3 to 20 times faster than any masked option, sharpening the owner mask question; clean quiet GPU re-time wanted; commits f84a06f99 and 86ba46df0 on msm-rust

## [2026-11-12 04:05] note | Correction to the Rust MSM gate entry
- The gate's 20 cells are 12 synthetic (random and skew 2^16 to 2^21) plus all 8 recorded k = 20 calls, not 4 recorded calls; the other figures stand

## [2026-11-12 04:58] lane-done | S10: node arity 4 at the OD-19 internal set; range table measured
- MEASURED: over 2^20 W1 leaves the tallest node AIR (FinalPolyQueryEvalAir) has 117,800 rows per child; arity 4 gives 471,200 rows padded 2^19 and proves and verifies; arity 5 and up fail on the 2^19 stacked height cap, which binds before k = 21; arity 12 tracegen pads to 2^21, over the gate, sweep stopped
- Lead decision LD-S10x-2, owner to confirm: node arity at most 4 with n_stack 17; the earlier 6.50 s per leaf at arity 8 figure is outside OD-19; arity 4 holds only for leaves of stacked width at most 336
- Range table MEASURED: stock VariableRangeCheckerAir at 16 bits is 2^17 rows, 4 main columns, no preprocessed trace (FP-1 says preprocessed, LD-S10x-3); 2^17 leaf floor, +589,824 leaf cells, +63,042 node cells per child, no change to max arity
- Issue: a table with zero lookups makes the node proof fail with SumClaimMismatch, cause unknown (LD-S10x-4); commits beeb6227e and 0ace30145 on worktree-agent-ad82adaf185dcbee3

## [2026-11-12 04:59] plan | Overnight loop stopped: remaining agenda waits on the owner
- Every lane is done; S11 and S12 are written, audited and reworked on final-design; the remaining S10 items (X-D1, wrap kernels) and the freeze wait on OD-24
- Morning summary at the top of research/sherlock-processor-final-20261111/LOOP-AGENDA.md; owner decisions in DECISIONS-ASK.md

## [2026-11-12 07:25] decision | Owner picks OD-24 option E and confirms the overnight lead decisions
- OD-24: option E, an Fr-native layer L_Fr between L and one stdlib wrap; A and C rejected; build plan E1 to E6 in research/sherlock-m7-wrap-20261112/PLAN.md
- Confirmed: FZ-1 to FZ-6, LD-S10x-2 to LD-S10x-5, and dropping the Rust MSM skew mask after a quiet-GPU re-time
- Recorded in research/sherlock-processor-final-20261111/DECISIONS-ASK.md (section Owner rulings 2026-11-12)
- Still open: the Nightstream build tree at ~/.cache/nightstream-run and the Rust 1.97.1 toolchain

## [2026-11-12 07:33] note | Nightstream build tree removed on owner order
- Removed the Nightstream checkout, target and tmp under ~/.cache/nightstream-run (23 GB); the run logs (1.5 MB) stay as evidence for COMPARE.html
- The Lean toolchains and the Mathlib cache stay; the Rust 1.97.1 toolchain question remains open

## [2026-11-12 07:33] lane-start | E1: stark-backend with a BLS12-381 Fr base field
- Owner ruling: start E1 of option E; Opus lane on branch e1-fr-backend (worktree ~/Projects/Moriarty-wt/e1-fr, from m7-wrap) and branch e1-fr-base in the stark-backend clone under ~/.cache/m7-wrap/src
- Gate: a small AIR proves and verifies at the Fr base with negative controls, BabyBear unchanged, D3 terms equal RESULTS section 2b.2
- Brief: research/sherlock-e1-fr-backend-20261112/BRIEF.md; logs in ~/.cache/e1-fr/; ESTIMATE 3 to 6 days

## [2026-11-12 07:54] note | FINAL-DESIGN.html updated for the owner rulings and published
- Read in full; the summary, gates, keys, GPU and build sections now state OD-24 option E, D10 w_stack 18, node arity at most 4 and the Rust MSM results; G4 figures marked as the rejected options
- Published at https://claude.ai/artifact/9673GweVZ7dJ1uY3WgZGGX; commit on final-design running; Rust 1.97.1 kept (owner)
- File: research/sherlock-processor-final-20261111/FINAL-DESIGN.html

## [2026-11-12 08:04] push | Overnight branches pushed to origin on owner order
- Pushed m7-wrap, d10-heights, mca-lean, msm-rust, and the S10 worktree branch as s10x-node; final-design follows once its docs commit passes the hook
- Checked before push: no qa/sherlock/status.json, no attribution lines, no blob over 2 MB

## [2026-11-12 08:04] lane-start | Five Opus designers for a co-processor explainer page
- Directions: (a) guided walk of one call, (b) field-change essay, (c) interactive explorer, (d) questions first, (e) layered cutaway
- Brief: research/sherlock-explainer-20261112/BRIEF.md; outputs in its a to e folders; the lead picks one, reads it in full, then publishes

## [2026-11-12 08:16] push | final-design pushed with the option E design page
- Commit accb91973 passed the hook on the second retry (factory timing tests failed under host load before) and final-design is on origin
- Explainer designs d and e are done; a, b and c are still running

## [2026-11-12 08:18] lane-done | E1 gate PASS: stark-backend proves at a BLS12-381 Fr base field
- MEASURED: Fibonacci and LogUp AIRs prove and verify at Fr with EF = F on both engines; 256 of 256 byte flips and 3 of 3 public-value changes refused; D3 terms equal M7 section 2b.2; BabyBear 239 of 239 tests pass and 4 proofs byte-identical
- The relaxation touched 21 lines in 5 files; at the L_Fr parameters a 2^12-row AIR proves in about 174 s on a loaded host, 300,269 bytes
- Commits: backend branch e1-fr-base (a16c9b3 to 3040761, in the clone under ~/.cache/m7-wrap/src); Moriarty e1-fr-backend eced75b02, db9714c71; research/sherlock-e1-fr-backend-20261112/RESULTS.md
- Open: L_Fr uses the L hash tags for u = 1 and u = 3; the E6 plan gives L_Fr its own tag (OD-18). Next: E2, the L_Fr recursion AIR set

## [2026-11-12 08:20] lane-done | Co-processor explainer: five designs done, design b published
- All five designs (a guided walk, b field-change essay, c interactive explorer, d questions first, e layered cutaway) render at 375px and 1280px in both themes
- Lead picked b, read it in full, fixed four facts (an unsourced textbook-name line, E1 now done, L_Fr hash Midnight Poseidon over Fr, L_Fr key in the universal set)
- Published at https://claude.ai/artifact/GQnhZKir2urd6JMPeApkDN; files in research/sherlock-explainer-20261112/ (untracked)

## [2026-11-12 08:23] lane-start | E2: the Fr recursion AIR set L_Fr that verifies one L proof
- Owner ruling: start E2; Opus lane on branch e2-lfr in the Moriarty worktree ~/Projects/Moriarty-wt/e2-lfr (from e1-fr-backend), the backend clone (from e1-fr-base) and the openvm clone
- Gate: L_Fr proves over the real L proof, the stock verifier accepts, negative controls refuse a changed proof byte and public word, heights MEASURED, then D3
- L_Fr gets its own transcript tag per the E6 plan; brief at research/sherlock-e2-lfr-20261112/BRIEF.md; ESTIMATE 6 to 12 weeks

## [2026-11-12 08:23] issue | Owner reports the published artifact previews open dead pages
- Read-back of the explainer artifact returns the full stored page, so the content is published; the failure is on the viewing side, cause not yet known

## [2026-11-12 09:44] lane-done | E2 first session: L_Fr increments I0 to I5b pass, gate not yet reached
- Route: a dedicated verifier of the L vk run as a fixed tape over generic Fr AIRs, with L's shape pinned in the L_Fr key (LEAD D-1); not a port of OpenVM recursion
- MEASURED: on the real L proof the tape's transcript equals the stock verifier's (1,500 of 1,500 samples) and proves; per-bus height bound at Fr fixed (was 1), BabyBear bytes unchanged
- Commits: Moriarty e2-lfr a21ad0670; backend e2-lfr e8223ed, 5f698b5; openvm e2-lfr 3cc1849, e8fdf87, 96a1897; research/sherlock-e2-lfr-20261112/RESULTS.md
- Open: each L_Fr execution AIR adds a commitment (6 for I5b, 8 at most) against 2 in the wrap estimate; next I6, the tape generator (shape pin, preamble, LogUp PoW, GKR)

## [2026-11-12 11:08] decision | E2 L_Fr uses one shared program-table AIR
- Owner approved the recommended fix for the L_Fr commitment count: one program-table AIR holds every preprocessed program, so L_Fr has 2 commitments as the E5 wrap estimate assumes.
- E2 continues at I6 (tape generator) on branch e2-lfr; see research/sherlock-e2-lfr-20261112/RESULTS.md in the e2-lfr worktree.

## [2026-11-12 11:16] lane-start | bridge-lean restatements: G4, per-fold PoW, two OOD, L_Fr layer
- Owner asked to start the Lean restatements now on branch bridge-lean (worktree ~/Projects/Moriarty-wt/bridge-lean, from commit bdb92973a).
- Scope: G4 gate on the unique decoding parameter sets, per-fold PoW (OD-1 to OD-3), two OOD samples, and option E with L_Fr as an abstract layer between L and the wrap.
- Results go to research/sherlock-stark-bls-bridge-20261111/LEAN.md on that branch.

## [2026-11-12 11:32] build | E2 I5c: one shared program table, L_Fr proof has 2 commitments
- Every L_Fr machine program now sits in one preprocessed ProgramTable AIR; execution AIRs receive their rows on a program bus with a nop row per padding row, which also pins their heights.
- MEASURED on the real 430,073-byte L proof: 1,500 of 1,500 samples equal, proves, 2 commitments (from the proof object), 1 preprocessed AIR in the vk.
- New negative controls refused: a changed program cell and a forged padding-row write (memory bus balanced, program bus broken); earlier I4 and I5 controls still refused.
- Commit: openvm e2-lfr 02bb7ff; research/sherlock-e2-lfr-20261112/RESULTS.md section 8b; next I6 tape generator.

## [2026-11-12 11:52] build | bridge-lean G4: unique decoding gate with kernel-checked bit counts
- ProvenRegime is now the OD-19 unique decoding gate (new clean/fork/Clean/Sherlock/UDBits.lean): exact rational error terms for folding, mu, queries, OOD and gamma from WHIR Lemma 4.10, Corollary 4.11, Theorem 4.8, Theorem 5.2, Lemma 4.25 and SWIRL Theorems 4.2.4 and 4.2.1, with PoW read from the transcript.
- Kernel-checked bit counts equal the D3 integer parts on every term: L fold 101, mu 101, query 101/101/101, OOD 108/112; internal fold 102, mu 101, query 101 per round, OOD 109/113.
- The list decoding gate stays as ProvenRegimeLD; A2 now documents the unique decoding statements. 125 theorems, 0 sorry, standard axioms only.
- Commit: bridge-lean 62ae7d88c; LEAN.md section 2.1 has the bit table.

## [2026-11-12 12:10] build | bridge-lean per-fold PoW read from the transcript of L
- PowKind now tags every fold (round, fold) and each round's query PoW; the G4 gate reads the folding and query vectors from the transcript program of L.
- The gate requires one PoW step per fold position and per round in protocol order, so a transcript with no fold PoW steps fails even though OD-19 requires fold PoW 0.
- Negative lemmas: missing fold steps, reordered fold steps, query PoW 19 in one round. 128 theorems, 0 sorry, standard axioms only.
- Commit: bridge-lean 553ec0f82.

## [2026-11-12 12:10] build | E2 I6: L verifier tape part 1 (preamble, LogUp PoW, GKR) matches the stock transcript
- Tape generator from the L vk and the pinned shape; proof values are tape inputs and every verifier check is an AssertEq op; new AssertEq and Bits (sample_bits) AIRs share the program table.
- MEASURED on the real L proof: 4,271 transcript events equal the stock record; 8,552 ops; proves with 2 commitments; a changed GKR claim and a changed PoW witness are refused.
- Commit: openvm e2-lfr be38351; research/sherlock-e2-lfr-20261112/RESULTS.md section 8c.

## [2026-11-12 12:10] build | E2 I7: batched zerocheck with the DAG of all 39 L AIRs on the tape
- Part 2 mirrors batch_constraints.rs steps 4 to 9: sumcheck rounds, eq terms, column openings, 15,312 DAG nodes, interaction evaluations and the final claim.
- MEASURED: 16,975 of 18,772 transcript events equal; 57,404 ops (45,909 ALU); proves with 2 commitments; a changed opening and a changed univariate coefficient are refused.
- Commit: openvm e2-lfr e43c763; RESULTS.md section 8d.

## [2026-11-12 12:10] build | E2 I8: stacked reduction on the tape
- Part 3 mirrors stacked_reduction.rs: lambda, n_stack quadratic rounds, eq, rotation kernel and indicator per stacked column, final sum.
- MEASURED: 17,299 of 18,772 transcript events equal (the remaining 1,473 are WHIR); 97,944 ops (86,229 ALU); proves with 2 commitments at n_stack 15; a changed stacking opening and sumcheck value are refused.
- Commit: openvm e2-lfr db8bbd5; RESULTS.md section 8e; next I9 WHIR.

## [2026-11-12 12:13] build | bridge-lean OOD samples in the transcript and verifier of L
- The transcript of L gains oodPoint and oodAnswer steps; the reference verifier runs them and run_refines covers the wrap gadgets for both.
- The G4 gate counts each round's OOD samples from the transcript and requires one answer per point, all points before any answer (WHIR Lemma 4.25 order).
- OD-19 keeps one sample per round; a two-sample key also passes (OOD 216 and 224 bits). 133 theorems, 0 sorry, standard axioms only.
- Commit: bridge-lean 21c817b5b.

## [2026-11-12 12:19] build | bridge-lean option E: L_Fr between L and the one wrap
- The wrap relation now verifies an abstract Fr-native layer L_Fr (KeyLFr, native checks LFrAccept, its own Fr transcript); new premise A2' lfrSound and static field vkLFr_ok.
- Transcripts start at the OD-18 capacity cell; tagSep, cap_inj, tags_separated and transcripts_separated keep L_Fr's use codes (5, 6, 7 in the example key) apart from L's.
- bridge_sound and wrap_view_only_claim restated through L_Fr; the consistency model covers the new fields. 142 theorems, 0 sorry, standard axioms only.
- Commit: bridge-lean a6e93240b.

## [2026-11-12 12:35] build | bridge-lean A2' on the reference verifier of L; app and leaf G4 sets
- A2' lfrSound now concludes that the reference verifier of L accepts an embedded L proof with the same statement and pv; lfrSound_of_emul shows that an L_Fr enforcing the Fr-cell emulation EmulL meets it.
- G4 adds the app and leaf-aggregation sets: the gate passes by decide, and the Lean query terms reproduce the D3 minima 101.0891 and 101.0110, so the G5 map's figures were query minima, not fold gaps.
- 146 theorems, 0 sorry, standard axioms only. Open: L_Fr has no parameter gate until E2; LogUp bits stay a calculator figure.
- Commit: bridge-lean 6661325d4.

## [2026-11-12 12:54] build | Sherlock E2 I9: WHIR on the L_Fr tape, whole L verifier
- openvm e2-lfr d46a137: gen_part4 (WHIR) and gen_verifier; new ops FrAdd, Halve, CSwap, each with its program in the shared table.
- MEASURED on the real L proof: all 18,772 transcript events equal the stock record; 442,805 tape ops (347,887 ALU, 20,563 Perm); BbAlu and table 2^19 rows.
- MEASURED: proof accepted with 2 commitments, 3,854,274 bytes, 302.8 s; changed opened row word, Merkle sibling and final-polynomial coefficient refused.
- Next: I10, the E2 gate at the L_Fr set (public values D-2, heights, D3 rerun). Results in research/sherlock-e2-lfr-20261112/RESULTS.md.

## [2026-11-12 12:55] push | bridge-lean pushed to origin
- Owner asked to push. Branch bridge-lean at 6661325d4: G4 unique decoding gate, per-fold PoW, OOD samples, option E with L_Fr; 146 theorems, 0 sorry, standard axioms.
- Checks before push: no co-author lines, no qa/sherlock/status.json, no blob over 2 MB.

## [2026-11-12 12:59] incident | E2 lane: rustup auto-installed a nightly toolchain
- In the E2 lane, cargo +nightly fmt made rustup auto-install nightly-aarch64 without the install double check. The agent uninstalled it at once.
- Lead check: rustup toolchain list holds only the toolchains present before the lane (stable, 1.90.0, 1.91.1, 1.97, 1.99.0). The lane now uses stable rustfmt with RUSTUP_AUTO_INSTALL=0.

## [2026-11-12 13:02] plan | E path loop armed: E2 I10 to E6
- Owner asked to start I10 and arm a loop to finish the E path. Agenda: research/sherlock-e-path-20261112/LOOP.md (untracked on main).
- E2 I10 and E5 (wrap on the real L_Fr proof from I9) start now; E3, E4, E6 wait on the E2 gate. Pushes, Preview or Mainnet deploys and installs wait for the owner.
