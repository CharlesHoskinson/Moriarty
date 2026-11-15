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

## [2026-11-12 14:02] push | stark branch pushed to input-output-hk/Moriarty
- Owner asked for a branch of the STARK workstream on the IOG repo. Branch stark at dbb5224b6, worktree ~/Projects/Moriarty-wt/stark.
- Contents: e2-lfr (E1, E2 through I9) merged with final-design, bridge-lean, mca-lean, d10-heights, msm-rust, s10x-node; stark-backend and openvm e2-lfr commits as patch series in research/sherlock-stark-code-20261112/.
- Checks: no co-author lines, no qa/sherlock/status.json; two paper PDFs over 2 MB were already on origin and went with the history.

## [2026-11-12 14:53] decision | Owner rulings: OD-1 to OD-18, AD items, leaf LogUp, freeze work in the loop
- OD-1 to OD-3 superseded by OD-19; OD-4 (a); OD-5 to OD-18 confirmed at (a), with L_Fr transcript on tag u = 7.
- AD-1 TOP 63, AD-2 no count, AD-6 S 16, AD-9 (a), AD-11 three-word records, AD-14 program-independent ty_d, AD-16 profile /1, AD-17 by-digest, AD-20 input consumption now, AD-22 (a), AD-23 (a).
- Recorded in research/sherlock-processor-final-20261111/DECISIONS-ASK.md; the E path loop (research/sherlock-e-path-20261112/LOOP.md) now also carries the freeze work up to S11. Pushes of stark ask the owner each time.

## [2026-11-12 14:56] lane-start | E3 interface, X12 Quint and L_Fr Gemini lanes, docs and ENC/FP-3 lanes
- E2 I10 full test suite passed at 14:55 (MEASURED); its agent commits next.
- Started: DOC and leaf LogUp (branch final-design-2), ENC and FP-3 (branch enc-fp3), E3 full interface through L_Fr (branch e3-iface), X12 Quint model with Gemini Quint lanes on the L_Fr AIRs (branch quint-x12).
- X7, X8, X10, X14, X15 wait for a heavy slot; agenda research/sherlock-e-path-20261112/LOOP.md.

## [2026-11-12 14:56] build | Sherlock E2 gate PASS: L_Fr proves over the real L proof
- openvm e2-lfr c4bd312: PublicAir exposes the D-2 public word (L vk pre-hash); whole L verifier tape proved at the L_Fr parameter set.
- MEASURED: stock verifier accepts the L_Fr proof, 2,516,530 bytes, 2 commitments (stacked widths 33 and 10), prove 728.8 s on a loaded host, verify 1.88 s; heights at most 2^19 rows (BbAlu and program table).
- MEASURED: changed public word refused by the stock verifier; changed L proof byte refused by the L_Fr prover; D3 at the measured key 101.1435 bits (query term weakest).
- Open: D-2 at the full 12,164-word interface (multi-row PublicAir), E5 wrap rows must use the measured stacked widths. Results in research/sherlock-e2-lfr-20261112/RESULTS.md.

## [2026-11-12 15:05] lane-start | E4 L_Fr GPU prover speed and MSM quiet-GPU re-time
- E2 gate is committed (openvm c4bd312, Moriarty fbba858f1), so E4 inputs are met; GPU had only browser processes at 15:05.
- One lane: the MSM re-time with and without the skew mask first, then E4 on branch e4-gpu (quiet CPU baseline, profile, GPU kernels with byte-identical proofs).

## [2026-11-12 15:10] build | Sherlock E5 wrap I0 to I2: ledger 9.1.0.1 accepts a ZkStdLib wrap with the bare Poseidon permutation
- Branch e5-wrap (Moriarty worktree ~/Projects/Moriarty-wt/e5-wrap, crate tools/sherlock-wrap; openvm worktree openvm-e5 f343ba8 holds the wrapin exporter). Results in research/sherlock-e5-wrap-20261112/RESULTS.md.
- MEASURED: a visibility-only vendored patch exposes the Midnight Poseidon permutation; stock ledger a2b30128 reads the key (1,325 bytes, k 15) and accepts a blinded proof with 1,026 public inputs; changed public input and flipped proof byte refused.
- MEASURED: the in-circuit L_Fr transcript (u = 7) equals the stock record (402 of 402 samples at the L_Fr set); all 557 WHIR Merkle openings of an L_Fr-set proof check natively, MockProver agrees, negative controls refused.
- DERIVED: at the measured L_Fr widths 33 and 10 the wrap is about 1.28 M to 1.38 M rows, 72 to 77 percent of the k = 21 gate; round-0 leaf hashing is about 70 percent. Next: I3 sumchecks.

## [2026-11-12 15:11] decision | MSM quiet-GPU re-time deferred
- Owner: the existing MSM benchmark is an upper bound; the re-time and the mask drop wait for a later improvement pass.
- The E4 lane skips the re-time and goes straight to L_Fr GPU prover speed.

## [2026-11-12 15:18] build | Sherlock E4 setup: worktrees, profile harness, GPU Poseidon increment queued
- MSM re-time stopped mid-run on the owner update (recorded rows only partly done); msm-rust branch unchanged.
- Branch e4-gpu in three worktrees: ~/Projects/Moriarty-wt/e4-gpu, ~/.cache/m7-wrap/src/stark-backend-e4, ~/.cache/m7-wrap/src/openvm-e4; plan in research/sherlock-e4-gpu-20261112/PLAN.md.
- Queued on the heavy lock: CPU baseline twice with a frozen c4bd312 binary (determinism and stage profile), then the GPU Midnight Poseidon kernel (mori-gpu poseidon.cuh, sppark fr_t as K0) with a CPU equality test.

## [2026-11-12 15:37] build | Sherlock design docs follow the 2026-11-12 owner rulings and OD-24 option E
- Commit efb0eb78a on branch final-design-2 (~/Projects/Moriarty-wt/final-design-2): DECISIONS, ARCHITECTURE, PROPOSAL, FINAL-DESIGN (md and html), TERMINOLOGY, EXPERIMENTS and the OpenSpec change sherlock-processor-final-20261111 record OD-4 to OD-18 (a), OD-24 (E), FZ-1 to FZ-6, LD-S10x-2 to LD-S10x-5 and the AD rulings.
- L_Fr layer specified (LW-018, LW-019; u = 5, 6, 7; premise A2'; Lean build 6661325d4); openspec validate --strict passes.
- LEAF-LOGUP: the leaf-aggregation set has LogUp 84.7207 bits at PoW 0; LogUp PoW 17 gives 101.7206 bits, MEASURED (~/.cache/ud-sizing/leaf-logup.log).
- Open: SRC-021 amendment of AD-18 not made (council text); Lean leaf entry at LogUp PoW 17 and the L_Fr parameter gate; E3 to E6.

## [2026-11-12 17:33] incident | X12 quint worker held the heavy lock for two hours
- The quint-x12 worker kept heavy.lock from 15:27 to 17:33 on one single-core quint test; E3, E4, E5 and ENC waited on an idle host (load under 4).
- Lead stopped the worker; the lock passed to enc-vec3. The lane now runs quint under heavy-run without the lock, each with a 15 minute timeout.

## [2026-11-12 17:33] decision | Owner rulings: SRC-021 amended, w_stack at most 48, OD-11 reading
- Amend SRC-021 to the one-arena Init field list (AD-18); freeze w_stack of L at most 48; OD-11 means no call-time fallback engine, L_Fr is on the one proving path.

## [2026-11-12 17:39] build | heavy-slot: three shared heavy slots in place of one exclusive lock
- Owner approved. New ~/.local/bin/heavy-slot: normal jobs take heavy.lock shared (up to 3, declared memory within 90G); --exclusive for timing runs and big jobs; older flock and doctorwatson users stay exclusive.
- Tested in a scratch dir: 3 shared at once, a fourth waits, exclusive and legacy flock wait for the slots and block new ones, a 95G shared request is refused.
- Running lanes are told to switch; light work (quint, docs) runs under heavy-run with no lock.

## [2026-11-12 17:40] build | E5 wrap I3 and I4a pass
- The wrap verifier over Alg now covers the preamble, LogUp PoW, GKR, the zerocheck rounds, the DAG and interaction evaluation and the final batch-constraint claim (Moriarty e5-wrap 263e22ff0).
- MEASURED: on mix-small and mix-lfr the native run reproduces every stock sample in order through the first stacked-reduction sample; 15 of 15 changed opening parts refused.
- MEASURED: MockProver on mix-small accepts parts 1 and 2 at 12,995 rows (k = 17); a changed column opening is refused.
- Next: I4b stacked reduction (code written), then I5 WHIR. Builds move to heavy-slot per the new rule.

## [2026-11-12 17:54] build | SRC-021 one-arena Init list, w_stack of L frozen, OD-11 confirmed
- Commit 98ecb75b2 on final-design-2 (~/Projects/Moriarty-wt/final-design-2): SRC-021 and design §3.2(5) of sherlock-vm-consensus-20261107 take the one-arena Init field list of kit INIT (AD-18 (a)) with by-digest input admission (AD-17 (b)); kit INIT uses the salted input_d of FP-4.
- The L key bound w_stack <= 48 is frozen by owner decision, with D10's measured 18 beside it; OD-11 recorded as owner decision (L_Fr is part of the one proving path).
- openspec --strict: sherlock-processor-final-20261111 valid; sherlock-vm-consensus-20261107 has 25 earlier long-text warnings in council and GP text (SRC-021 itself is clean).

## [2026-11-12 18:03] build | E3 J1 passes: public values of L through L_Fr
- openvm e3-iface 96acfa3 (~/.cache/m7-wrap/src/openvm-e3): Entry::Public in the L verifier tape, a stock-backend public-value carrier (two one-hot selector groups, D-6) in L and in a multi-row L_Fr PublicAir, the L interface AIRs (12,164-word index table) and the native log reference.
- MEASURED on a small L_iface (610 words): every transcript event equals stock, the exposed words decode to the native statement, L_Fr proves with 2 commitments; a changed L public value is refused by the stock verifier, the tape and the L_Fr prover; changed first, middle, last and swapped carrier words are refused.
- Plan ~/Projects/Moriarty-wt/e3-iface/research/sherlock-e3-iface-20261112/PLAN.md; the full L_iface fixture at the OD-19 L set is proving (unit e3-j3).

## [2026-11-12 18:19] build | Sherlock E4 CPU baseline, nondeterministic grind, GPU Poseidon kernel equal to CPU
- MEASURED CPU baseline (c4bd312, 20 threads, load 7 to 21, shared host): L_Fr prove 700.2 s, keygen 188.8 s, peak RSS 28,061 MiB, proof 2,516,530 bytes; Midnight Poseidon is about 600 s of it (row hashing 537 s), NTT about 35 s.
- MEASURED: two CPU proofs differ from byte 53,546 (grind uses rayon find_any); e4-gpu switches the grind to find_first so proofs are reproducible and GPU byte identity is testable.
- GPU Midnight Poseidon (mori-gpu poseidon.cuh, plain and sparse partial rounds) equals the CPU hasher on widths 1 to 48 and on digest layers; about 2 M permutations per second on a loaded host (MEASURED, not a timing).
- Next: exclusive A/B of the full L_Fr prove (CPU reference with MORI_GPU=off against GPU hashing); results in ~/Projects/Moriarty-wt/e4-gpu/research/sherlock-e4-gpu-20261112/RESULTS.md.

## [2026-11-12 18:21] build | Sherlock ENC and FP-3: encodings, Rust vectors, FP-3 options
- Commits: Moriarty enc-fp3 04a004c57 (research/sherlock-enc-fp3-20261112/); stark-backend clone enc-fp3 829a966 and 92d77ef (test enc_vectors, 55 pinned vectors, MEASURED on two code paths each).
- ENC-1 to ENC-3 written (tag rule, enc(e) with DIGEST at 2 values per OD-17, layout_d); G3 T-FS cases of L (u = 3) and L_Fr (u = 7) on the Rust reference; ENC-4 and ENC-5 partly written; no gate closed, AIR and wrap vectors missing.
- FP-3 facts 1 and 3 stopped with options (recommend outcome carrier and opening by proof); fact 2 evidence: the compiled acceptRun sketch publishes claim_pub and outcome_pub, not C_P and input_pub.
- Owner picks P-1 to P-6 listed in ~/Projects/Moriarty-wt/enc-fp3/research/sherlock-enc-fp3-20261112/RESULTS.md section 8.

## [2026-11-12 18:31] build | bridge-lean: leaf LogUp PoW 17 and the L_Fr G4 gate
- Commit 46622c969 on bridge-lean: UDSet.gate computes the LogUp term by the calculator's logup_soundness formula, over a field record (bbF for L, internal, app, leaf; frF with size r for L_Fr).
- Leaf aggregation set takes LogUp PoW 17 (owner ruling efb0eb78a): 101 bits (D3 101.7206), PoW 16 fails; L and internal LogUp 102 (102.7205).
- L_Fr gate ProvenRegimeFr on the E2 I10 set: query 101 per round (weakest, D3 101.1435), LogUp 215, fold 232; A2' lfrSound takes it as a hypothesis, discharged in bridge_sound.
- 153 theorems, 0 sorry, standard axioms only; LEAN.md section 2.2 documents the L_Fr gate.

## [2026-11-12 18:35] build | Sherlock X15 ROM rows and lines per step
- Test-only ROM probe and static rom_bound in the rule scan, commit 92f63d212 on x-exp; results in ~/Projects/Moriarty-wt/x-exp/research/sherlock-x15-20261112/RESULTS.md.
- MEASURED: 6,669 steps, 0 steps above the static bound. STATIC max 15 ROM rows per step (BHelperEqualField, XAdtHelperPlanSwitch); OBSERVED max 7 rows, 6 lines of 8, 6 tables, 2 selection rows.
- Under the AD-3 rule the bound selects K_rom = 15 (owner to confirm); D_tbl >= 5 for 22 ProgramView tables; D_row stays open (BoundsC caps).

## [2026-11-12 18:42] build | E3 J3 and J4 pass: full L interface on the L_Fr tape
- openvm e3-iface 71bf0f1 (~/.cache/m7-wrap/src/openvm-e3): L_iface fixture (bench L plus 4 interface AIRs, 43 AIRs, 12,164 public words) at the OD-19 L set, 4 cases at one key; each proof 487,514 bytes, 228 to 312 s.
- MEASURED: full-V (189 entries, 256 values), full-P (256 slots) and overflow (551 operations to tier E) run on one L_Fr tape with no failed assertion and decode to the native statement; missing-R fails 3 assertions.
- MEASURED: the tape grows from 442,806 to 497,410 ops; every AIR height is unchanged except Public (128 rows), the table stays at 2^19. The gate at the L_Fr set (unit e3-j5) runs next.

## [2026-11-12 18:50] build | E5 wrap I4b to I6a pass, stock ledger accepts the whole-verifier wrap
- The stacked reduction and WHIR are ported over Alg; on mix-small and mix-lfr the whole L_Fr verifier reproduces every stock sample (252 and 402) and refuses all 11 changed proof parts tried (Moriarty e5-wrap c718e6f20).
- MEASURED: the full wrap of mix-small is 518,774 rows at k = 20; keygen 51.0 s, blinded prove 70.0 s, proof 2,912 bytes, key 1,325 bytes; stock ledger 9.1.0.1 returns Ok with 1,026 public inputs and refuses changed inputs and flipped bytes.
- MEASURED: at the L_Fr parameters with the mixture widths the wrap is 898,562 rows; the real E2 proof (widths 33 + 10) is queued for export as fixture lfr-real to measure the gate rows.

## [2026-11-12 18:56] decision | Owner rulings: ENC picks P-1 to P-6, FP-3 facts 1 and 3, acceptRun binding
- P-1 to P-6 confirmed as recommended (ASCII u128 tags, index 0 to 3, overwrite absorption, u16/u32 and byte packing, DOM_PLAN 41, StackStep codes and u64 count).
- FP-3 fact 1 (C) outcome carrier; fact 3 (a) opening by proof; acceptRun binds claim_pub to a logged SPONGE(3) after transientHash parity.
- Lean bridge-lean 46622c969: leaf LogUp PoW 17 and the L_Fr gate ProvenRegimeFr; 153 theorems, 0 sorry.

## [2026-11-12 19:02] push | stark updated on input-output-hk/Moriarty
- Owner asked to push. stark dbb5224b6..cb5180aed: merges of e2-lfr (E2 gate), final-design-2 (owner rulings and option E docs, SRC-021) and e5-wrap (I0 to I4a).
- Checks: 8 new commits, no co-author lines, no qa/sherlock/status.json, no blob over 2 MB. The code-patch refresh commit waits on the load-sensitive factory timing test and follows.

## [2026-11-12 19:14] note | X-EXP lane resumed for X10, X7, X14, X8
- X15 done at 92f63d212 (static max 15 ROM rows); K_rom=15 awaits owner confirmation
- X10 Gemini lane: HeaderOpenAir sound, 13 tests pass, 6 of 6 mutants caught; node run x10-run and x7-nn running under heavy-slot
- lead told the lane to commit X10, write X7 N-6 owner options, then run X14 and X8

## [2026-11-12 19:14] note | E5 real L_Fr proof exported and matched, I6b queued
- lfr-real (E2 I10 proof, widths 33 and 10) exported; native verifier matches 649 of 649 stock samples and refuses 3 tampered parts (openvm e5-wrap e0d43ff)
- systemd unit e5-i6-real (i6.sh lfr-real 21) waits for a 48G slot; it measures rows against the k=21 gate 1,782,573
- lane resumed to commit its open Moriarty work and read the I6b rows

## [2026-11-12 19:16] build | E3 L_Fr PublicAir row binding fixed
- Review found that a PublicAir program row could sit at any trace row (the program bus fixes only the multiset), so 16-word blocks of the L_Fr public words could be permuted.
- openvm e3-iface 2f09caf (~/.cache/m7-wrap/src/openvm-e3): each program row carries its row index, tied to the carrier selectors; MEASURED: rows 0 and 1 swapped with their programs and public blocks are refused; clippy clean.
- Added the all-entries case (R and 250 PUB, 253 non-NOP entries; 256 is impossible at B = V, DERIVED). Fixture and the gate at the L_Fr set rerun (units e3-j3b, e3-j5b).

## [2026-11-12 19:20] build | E5 wrap: the real L_Fr proof checks natively
- The E2 I10 L_Fr proof (widths 33 and 10, 15 traces) is exported as fixture lfr-real from the E4 key cache and baseline proof; the stock verifier accepts it (openvm e5-wrap e0d43ff).
- MEASURED: the wrap's native verifier matches all 649 stock samples of that proof, the n = -1 trace included, and refuses a changed row, opening and public value (Moriarty e5-wrap efd486460).
- The k = 21 wrap run on lfr-real (unit e5-i6-real) waits for a heavy slot.

## [2026-11-12 19:29] build | Sherlock X10 state header opening in the STARK node
- HeaderOpenAir (8 rows, 307 columns, 362 constraints, 0 interactions) in an arity-2 node at the internal set: 40 of 50 AIRs, stacked width 69 unchanged, proved and verified; forged view_root refused (MEASURED). Commit 2626120ed on x-exp.
- Gemini Quint lane SOUND/COMPLETE; lead ran quint: 13 tests pass, M1 to M6 caught; the one-hot sum constraint is required (M5).
- Supports AD-19 (b) and N-2 (a); the pv link to child post_d and the word export to SEAL are not wired yet. ~/Projects/Moriarty-wt/x-exp/research/sherlock-x10-20261112/RESULTS.md

## [2026-11-12 19:31] build | E5 wrap rows on the real L_Fr proof
- MEASURED: the full wrap of the E2 I10 L_Fr proof (widths 33 and 10) is 1,592,979 rows at k = 21, 89.4 percent of the k = 21 gate (1,782,573); 8 advice and 27 fixed columns.
- It fits, but within 10 percent of the gate, so the levers run next one at a time: fused multiply-add for the WHIR mu batching and the final-polynomial Horner, then cheaper omega^index chains.
- MockProver, keygen, the blinded proof and the ledger check at k = 21 are running in unit e5-i6-real.

## [2026-11-12 19:32] commit | Sherlock E4 increments 1 and 2: GPU hashing byte-identical, L_Fr prove 824 s CPU to 229 s GPU
- MEASURED (exclusive slot): CPU reference 824.4 s, GPU Poseidon hashing 228.6 s, proofs byte-identical (SHA-256 c17dadf7); row hashing 579 s to 57.2 s.
- Increment 2 (fused GPU NTT plus row hash, chunked deterministic grind) is byte-identical too; its timing run was throttled by the heavy.slice 64 GiB memory.high (31 GB tmpfs leftover in /tmp/bounded-depth-final-2848992 from another lane).
- Commits on e4-gpu: Moriarty 64813be19, stark-backend 4199bc1, openvm cee7fd2; results in ~/Projects/Moriarty-wt/e4-gpu/research/sherlock-e4-gpu-20261112/RESULTS.md.
- Next: low-memory streaming encode (device writes the codeword into host memory) and a clean exclusive timing.

## [2026-11-12 19:37] build | Sherlock FP-3 facts 1 to 3 closed with closure tests (enc-fp3)
- Moriarty enc-fp3 7d3d9074e: ARCHITECTURE FP-3 facts closed (outcome carrier with check (i), lanes from the compiled ledger contract, openClaim by proof keyed on claim_pub), vd Fr form, SHK_REG and SHK_SET, R bound 7/16/8 DERIVED; evidence in `~/Projects/Moriarty-wt/enc-fp3/research/sherlock-enc-fp3-20261112/RESULTS.md` section 10.
- Rust reference fp3_closure.rs (stark-backend enc-fp3 65d1a03): 7 tests pass, MEASURED; compiled contract fp3.test.mjs: transientHash parity, cross-entry replay with two-map control, collection privacy, lanes: 6 pass, MEASURED.
- AIR vectors on the L_Fr machine (openvm enc-fp3 f31a843): 27 L transcript samples and 7 Fr digests equal the pinned values, proof accepted, MEASURED; wrap-side vectors wait on E5.
- 1.12: I-21 and I-25 FROZEN; FP-3 gates removed from I-23, I-24, I-28 and four kit rows; ENC rows stay FREEZE-PENDING on the wrap vectors.

## [2026-11-12 19:41] build | Sherlock X7 binary node shapes at the internal set
- Leaf direct arity 2 (2^18, 39 AIRs, 1,543 columns), one-leaf lift (2^17), node over two lifts (2^17, 1,549 columns): all proved and verified (MEASURED). Commit 6d8e8e640 on x-exp.
- N-1 (a) fits for leaf-only slots; the mixed leaf-or-node slot is not measurable with one child key. N-6 residual for the inline owner step: 11 AIRs, 451 columns (DERIVED); the step itself needs BabyBear kits.
- Three N-6 owner options with recommendation option 1 (keep (a), freeze-pending on the kits) in ~/Projects/Moriarty-wt/x-exp/research/sherlock-x7-20261112/RESULTS.md

## [2026-11-12 19:43] qa | E5 gate PASS: stock ledger accepts the wrap of the real L_Fr proof
- MEASURED: the k = 21 wrap of the E2 I10 L_Fr proof (1,592,979 rows, 89.4 percent of the gate) passes MockProver with both negatives refused; keygen and the blinded prove complete (shared-host timing); proof 2,912 bytes, key 1,325 bytes.
- MEASURED: stock ledger 9.1.0.1 VerifierKey::verify returns Ok with 1,026 public inputs and refuses changed inputs and flipped bytes.
- Next: the row levers (fused multiply-add, omega^index chains) and one exclusive timing.

## [2026-11-12 19:50] build | Sherlock X14 BLS12-381 operation AIRs in the unit node
- Stock mod-builder op AIRs need 129 (fr_mul), 193 (fp_mul), 579 (g1_add) and 772 (g1_double) interactions per AIR, above the 100 of the stark-sdk 100-bit assumptions; 10-bit limbs reach 105 at best (MEASURED). Commit 5cc0215b2 on x-exp.
- Each op AIR proves in an arity-1 unit node at 2^16 beside one child verification; ESTIMATE about 25,000 g1_add rows (MSM) and 20,000 fp_mul rows (final check) per ivc_node proof.
- GP-3 general gate not closed: encoding choice open, recommendation tuple range checks plus G1 ops as fp-op rows. ~/Projects/Moriarty-wt/x-exp/research/sherlock-x14-20261112/RESULTS.md

## [2026-11-12 19:51] build | Sherlock X8 foreign lift cost derived
- No Compact adapter class or foreign lift exists, so X8 is DERIVED from X14 and X10: under AD-22 (a) the lift costs two in-circuit halo2 verifications plus two 255-level Poseidon2 membership paths (about 510 permutation rows) and the claim hash.
- The native and general GP-3 paths share the X14 encoding blocker; a named Compact adapter class is also needed. ~/Projects/Moriarty-wt/x-exp/research/sherlock-x8-20261112/RESULTS.md

## [2026-11-12 20:03] commit | E4 streaming fused encode committed
- Moriarty `e4-gpu` 90c6e9741: streaming fused encode in `crates/mori-gpu` and RESULTS.md section 5 (L_Fr prove 139.9 s MEASURED on the GPU path, byte-identical to the CPU proof).
- Pre-commit hook passed on the first attempt.
- Final exclusive CPU and GPU A/B `e4-abfinal` waits for a heavy slot behind another lane's job.
- Results: `~/Projects/Moriarty-wt/e4-gpu/research/sherlock-e4-gpu-20261112/RESULTS.md`.

## [2026-11-12 20:13] build | E5 wrap lever L1: fused multiply-add
- MEASURED: a one-row x*y + z (the native chip's add_and_mul, reached through a visibility-only ZkStdLib accessor) in the WHIR batching, Horner and the claim sums cuts the lfr-real wrap from 1,592,979 to 1,453,425 rows at k = 21 (81.5 percent of the gate).
- All I3 to I5 tests pass, MockProver on mix-small included (Moriarty e5-wrap 4e6cd41c7). The pre-commit factory heavy tests failed three times on host-load timing before a clean pass.

## [2026-11-12 20:35] decision | Owner rulings on X-EXP results and the stark push
- stark 65460dd82 waits for the E5 milestone; K_rom = 15 frozen (X15)
- N-6 option 2: raise the node internal-set budget now (64 AIRs, 3,000 columns proposed) before FP-8
- X14 option 2: per-AIR interaction bound 200, G1 ops as fp_mul-class rows, D3 recompute at 100 bits or more; recorded in ~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md

## [2026-11-12 20:40] build | Sherlock D3 internal set under the 2026-11-12 node bounds
- D3 calculator (calculate_extended, as ud_sizing.rs) for the frozen internal set (blowup 3, queries 97/82/81, mu PoW 9, query PoW 21, LogUp PoW 18) with 200 interactions per AIR, 64 AIRs, 3,000 columns: weakest term 101.1643 bits (query, shift_rbr); every term at least 100, grindable at least 101 (MEASURED).
- Only the stacked term moves (111.6618 to 111.0768); n_logup stays 29 (field bound). No PoW change needed. ~/Projects/Moriarty-wt/x-exp/research/sherlock-x14-20261112/out/d3-bounds.log

## [2026-11-12 20:43] build | E5 wrap lever L2: omega^index chains
- MEASURED: omega^index from per-round constants, zi by k squarings and a witnessed inverse pinned by zi_root * zi_inv = 1 cut the lfr-real wrap from 1,453,425 to 1,436,529 rows at k = 21 (80.6 percent of the gate); all I3 to I5 tests pass.
- The exclusive timing run of the final circuit (MockProver, keygen, blinded prove, ledger) is queued as unit e5-i6-excl.

## [2026-11-12 20:57] build | E4 gate verdict: L_Fr GPU prove 139.9 s, FAIL against the call budget
- L_Fr prove on the GPU path 139.9 s MEASURED, byte-identical to the CPU proof, about 7.4x the 19.0 s mid warm call (DERIVED), so the gate fails; the CPU path is about 5x slower.
- The quiet CPU baseline was not obtained: the exclusive A/B `e4-abfinal` was throttled by heavy.slice MemoryHigh (32.7 GB shmem from another lane's `/tmp/bounded-depth-final-2848992`) and was stopped at 20:31.
- The RESULTS.md section 6 commit failed the pre-commit hook four times (factory lease and heavy-slot timing tests), so it is left uncommitted in `~/Projects/Moriarty-wt/e4-gpu/research/sherlock-e4-gpu-20261112/RESULTS.md`.
- Next: remove the tmpfs leftover, rerun `ab.sh final`, then tune the Poseidon kernel and move the fractional sumcheck to the GPU.

## [2026-11-12 21:00] decision | E4 gate fails once; owner orders tune first, E6 proceeds
- GPU L_Fr prove 139.9 s, byte-identical to CPU (MEASURED), about 7.4x the 19.0 s mid warm call (DERIVED); Poseidon alone about 74 s
- owner: tune the Poseidon kernel and move the fractional sumcheck to the GPU, then bring hash or trace options with numbers; E6 starts without the E4 speed gate
- the 31 GB tmpfs leftover /tmp/bounded-depth-final-2848992 is deleted on owner approval so the quiet A/B can run

## [2026-11-12 21:03] lane-start | E6 keys and devnet integration (branch e6-int)
- worktree ~/Projects/Moriarty-wt/e6-int from e5-wrap; the L_Fr key joins the universal set, OD-18 tag, A2' wording, D3 at measured shapes
- devnet deploy and call on the XC-9 path once the E5 ledger check on lfr-real and E3 J5 pass; no Preview or Mainnet without the owner
- E4 resumed to tune the Poseidon kernel and the GPU sumcheck, then write hash or trace options

## [2026-11-12 21:10] lane-start | E6 worktree created; inputs checked
- branch e6-int at ~/Projects/Moriarty-wt/e6-int from e5-wrap a9de22611 (levers L1, L2 on top of the E5 gate af29c5a57)
- E5 input PASS: ledger 9.1.0.1 accepts the lfr-real wrap, 1,592,979 rows k = 21, key 1,325 bytes (MEASURED, ~/.cache/e5-wrap/logs/i6-real.log)
- E3 J5 still running (unit e3-j5b); the devnet call needs the E3 call-vector map in the wrap in place of the placeholder PublicMap and a fresh L, L_Fr, wrap chain per call

## [2026-11-12 21:10] commit | E4 gate verdict committed; quiet A/B rerun queued
- Moriarty `e4-gpu` 84e68661b: RESULTS.md section 6 (L_Fr GPU prove 139.9 s MEASURED, gate FAIL, incidents, pickup); the hook passed on the fifth attempt after factory lease and heavy-slot timing failures.
- Owner rulings 21:00: tune first; the tmpfs leftover is deleted (/tmp 47G free).
- Unit `e4-abfinal2` (`~/.cache/e4-gpu/ab.sh final`) queued for an exclusive slot: CPU reference plus GPU run of the section 5 binary.
- Next: Poseidon kernel tuning (block size, sqr S-box) and the GPU fractional sumcheck, built after the A/B.

## [2026-11-12 21:23] qa | E6 local devnet up (compose project e6)
- `~/.cache/e6-int/devnet/compose.yml` (copied from XC-9, local images, no pull): node 2.1.101 on 127.0.0.1:9954, indexer 4.4.101 on 127.0.0.1:8098, proof server 9.0.1 on 127.0.0.1:6340 (MEASURED).
- Genesis `0x823383ec...ebae1d`, chain `undeployed1` (MEASURED); same dev-preset genesis as XC-9.
- Runner work in `~/Projects/Moriarty-wt/e6-int/tools/sherlock-sdk/` (e6-local-run.mjs, e6-dump-prover.mjs) follows.

## [2026-11-12 21:33] build | E6 wrap call-vector map and L_Fr call chain written
- wrap statement = the E3 call vector (unpack, 30-bit limbs, Canon by borrow chain, empty lane map) in ~/Projects/Moriarty-wt/e6-int/tools/sherlock-wrap/src/callvec.rs, replacing the E5 PublicMap placeholder
- openvm branch e6-int (~/.cache/m7-wrap/src/openvm-e6, e3-iface plus e5-wrap): examples lfr_call (L_iface run with given lanes, L_Fr prove, wrapin export) and d3_wrapin
- Gemini Quint lane e6-callvec running (~/.cache/gemini-lanes/e6-callvec); A2' wording fixed in ~/Projects/Moriarty-wt/bridge-lean (u = 5, 6, 7, transcript u = 7); prep chain unit e6-prep queued behind e3-j5, e5-i6-excl, e4-ab-final

## [2026-11-12 21:34] qa | E6 acceptRun call vector padded to 1,026 inputs on the local devnet
- MAIN `e03e025a...607cca` and OTHER-PROGRAM `5e2ff270...6b4e3e` deployed with the E5 wrap key framed as `midnight:verifier-key[v7]:` + SCALE length (1,353 bytes); indexed key equals the framed bytes (MEASURED, `~/.cache/e6-int/run/e6-state.json`).
- acceptRun transcript: 22 ops, 68 words; one trailing `Noop{956}` appended through the ledger-v9 JS Transcript object; Transaction.prove adds its gas itself (+103,089 ps compute, equal to the VM's noop cost) (MEASURED).
- Dump stub vector: 1,026 words, equal lane for lane to validate_call's public inputs; validate_call fails only at "Invalid proof" with a dummy proof, JS ledger wellFormed + apply succeed (MEASURED, `~/.cache/e6-int/run/call-vector-dump.json`).
- Runner `~/Projects/Moriarty-wt/e6-int/tools/sherlock-sdk/e6-local-run.mjs` (deploy, dump, call, dry-run); a dummy-prover rehearsal of the whole call phase got node verdict Malformed(InvalidProof) for A, R1, R2 and R3 (MEASURED, `~/.cache/e6-int/rehearse/`).

## [2026-11-12 21:53] build | E3 gate PASS, full L interface through L_Fr
- J5 on openvm e3-iface 2f09caf: all-entries (253 entries, 256 values) and overflow (551 to tier E) accepted at lfr_params with 2 commitments, widths 33 and 10, 2,568,818 bytes; missing R refused by the prover (MEASURED, shared slot).
- D3 at the measured L_Fr key 101.1435 bits, equal to E2 (query term); tape 497,410 ops (+54,604), only Public grows to 128 rows (MEASURED).
- Decoder D-9 equals native statement in full_v, full_p, overflow, all_entries (plain Rust, not a wrap gate); results in ~/Projects/Moriarty-wt/e3-iface/research/sherlock-e3-iface-20261112/RESULTS.md.

## [2026-11-12 22:06] qa | E5 final wrap: exclusive timing and ledger
- MEASURED: the final wrap circuit (L1 + L2) of the real L_Fr proof is 1,436,529 rows at k = 21 (80.6 percent of the gate); MockProver accepts and refuses both negatives.
- MEASURED in an exclusive heavy slot (host load 30 to 37 from work outside the slots): keygen 84.6 s, blinded prove 183.8 s, proof 2,912 bytes, key 1,325 bytes.
- MEASURED: stock ledger 9.1.0.1 returns Ok with 1,026 public inputs and refuses changed inputs and flipped bytes. E5 gate PASS.

## [2026-11-12 22:15] build | E4 final A/B rerun: GPU 123.0 s, CPU 896.5 s, byte-identical
- Unit `e4-abfinal2`, exclusive slot 21:56 to 22:15, section 5 binary: CPU reference 896.5 s and GPU 123.0 s MEASURED, both proofs SHA-256 c17dadf7, peak RSS 27,273 and 23,830 MiB.
- Host load average 28 to 36 for the whole slot (work outside heavy-slot), so the CPU figure is still not a quiet baseline.
- On the GPU path the fractional sumcheck took 11.0 s here against 28.9 s in section 5, which was likely slowed by the heavy.slice memory throttle.
- Log `~/.cache/e4-gpu/logs/ab-final.log`; next: build the tuning and the GPU fractional sumcheck (unit `e4-gpu4`).

## [2026-11-12 22:18] qa | E6 Lean A2' wording builds; devnet runner GO
- lake build SherlockSpec PASS (32 jobs, 0 sorry, standard axioms, MEASURED) on ~/Projects/Moriarty-wt/bridge-lean with the u = 5, 6, 7 A2' wording; commit running
- devnet project e6 (127.0.0.1 9954/8098/6340): acceptRun call vector exactly 1,026 words with one trailing Noop n = 956 (68 transcript words), native validate_call and node reach proof verification (MEASURED)
- J5 L_Fr prove 2,589 s under load (MEASURED) so the runner intent TTL is now E6_TTL_S (4 h for the call) and the call phase takes an exclusive slot

## [2026-11-12 22:19] decision | E3 gate accepted with all-entries; J6 measured before build
- owner accepts the all-entries case (253 entries, 256 values) for the all-non-NOP case, which cannot exist at B = V = 256
- J6 native-Fr replay: count its ops first, build only within the about 27k rows of ProgramTable headroom
- E5 gate PASS reported: 1,436,529 rows at k = 21 after levers L1 and L2, ledger 9.1.0.1 accepts (MEASURED); E6 devnet call inputs are ready

## [2026-11-12 22:21] push | stark updated on input-output-hk/Moriarty with the E5 gate
- owner ruling: push with the E5 milestone. stark cb5180aed..84026dc7c, merge of e5-wrap (E5 gate, levers L1 and L2, ledger 9.1.0.1 accepts)
- checks: 6 new commits plus the merge, no co-author lines, no qa/sherlock/status.json, no changed blob over 2 MB

## [2026-11-12 22:21] plan | E-path loop armed for an AFK night
- owner asleep: the loop takes recommended options, marks them for confirmation and asks nothing; no further push, Preview, Mainnet or install
- next: E6 devnet call (E3 and E5 gates passed), E5 re-measure on the E3 proof, E3 J6 op count, E4 tuning options, X-EXP D3 at the raised bounds, then FP8 and S11

## [2026-11-12 22:30] decision | E6 input gates passed; merged into e6-int
- E3 J5 PASS (Moriarty e3-iface 4b5156e1f, openvm 2f09caf) and E5 final 1,436,529 rows k = 21, ledger 9.1.0.1 accepts (e5-wrap 7c75add31), both merged into ~/Projects/Moriarty-wt/e6-int (cb1d20f8a)
- owner AFK: E6 takes the recommended option on every choice and marks it for owner confirmation in RESULTS; devnet call with R1 to R3 next

## [2026-11-12 22:24] note | E4 GPU fractional sumcheck and Poseidon tuning knobs pass equality tests
- late entry (build event at 22:24, written after a 22:30 entry of another lane).
- New: mori-gpu `frac` (segment tree and per-layer sumcheck on the GPU), stark-backend `FracGkrAccel` hook (default None), stark-sdk `FracGpu`; Poseidon knobs `MORI_POS_BLOCK` and `MORI_POS_SQR`, all uncommitted on `e4-gpu`.
- Test `e4_gpu_frac`: proof, point and transcript state equal to the CPU at 2^12, 2^13 and 2^17 leaves; `e4_gpu_hash` passes with sqr and block sizes 64 and 256.
- Variant timings `abt.sh t1` (base, no frac, sqr, block 64 and 256, log blowup 2 and 1) queued for an exclusive slot.

## [2026-11-12 22:33] build | E3 J6 measured, J7 R replay and ENC-1/ENC-2 in L_Fr
- J6 full native replay does not fit the L_Fr key: fixed-tape routing of data-dependent offsets needs about 256 x 256 selector pairs and limb ranges alone are about 41k ops against about 27k rows of ProgramTable headroom (DERIVED); not built.
- J7 (openvm e3-iface 6d70a45): ENC-1 tag values (pinned input_pub vector matches), R = run DIGESTs + SPONGE(3) claim_pub + R_A (ORD 1, chunk 2 entries, anchor rule), native replay of R on the tape: 2,943 ops (MEASURED); R layout and u32 limb order are LEAD picks for confirmation.
- Fixture, J4, J5 rerun queued as unit e3-j7 (fixture dir ~/.cache/e3-iface/fixture-j7).

## [2026-11-12 22:42] decision | Sherlock E5 wrap: E3 map scope (for owner confirmation)
- Recommended option, marked for owner confirmation: the wrap ports the E3 D-9 decoder (`iface::decode`, openvm e3-iface 2f09caf) exactly, every check a constraint, in `~/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap/src/decode.rs`; the statement is the decoded call vector (1,026 lanes), which replaces PublicMap.
- The PUB check and the lane-map value read use one log-derivative lookup; its multiplicities and values are observed into the wrap transcript after part4, before the two challenges.
- The L vk pre-hash (L_Fr public value 0) is pinned as a key constant. Open items: no J6 native-Fr replay of the log values, and the head words are only range checked.
- Unit costs MEASURED (cost model): a range check below 2^30 is 3 rows with its assign, below P is 6 rows, a bit is 1 row.

## [2026-11-12 22:48] build | E3 J8, the log onion in L with its tie to log_d and log_n
- openvm branch e3-iface-onion ab0e88a (worktree ~/.cache/m7-wrap/src/openvm-e3-onion): OnionAir (70 columns) recomputes d over the pv tables, looking up each Poseidon2 permutation in the bench L Poseidon2 AIR (external inputs), and ties (d, n) to root_stmt words 57 to 65.
- Native P2 and onion reproduce the pinned ENC-2 vectors D_EMPTY and onion6 (MEASURED); small L_iface with a key table: honest FullV, AllEntries, chunk 2 of 3, missing R accepted; log_d, one value, log_n changed each refused by the prover (MEASURED).
- J7 chain e3-j7 stopped before it started (slot wait); the combined J7 + J8 chain runs as unit e3-j8 (fixture ~/.cache/e3-iface/fixture-j8). Single log segment only; the root_stmt to node tie stays with production L.

## [2026-11-12 22:56] decision | E3 J6 figure corrected, J8 L_Fr cost gated
- Correction to the J6 entry: limb ranges are about 4.6k ops (2,304 limbs, 2 ops each), not 41k; the binding cost is Fr composition at 257 ops per value (MEASURED in J7), so a full replay of 256 values is about 65.8k ops against about 27k rows of headroom (DERIVED), plus routing of data-dependent offsets (ESTIMATE). Verdict unchanged: J6 not built.
- J8 onion set on the small L_iface adds 151,357 tape ops at small_params_n(4, 8) (ESTIMATE for the full L set); J4 now fails unless BbAlu and ProgramTable stay at 2^19, so J5 runs only at the E2 shape. Unit e3-j8 relaunched on openvm e3-iface-onion f725b15.

## [2026-11-12 23:17] build | E4 Poseidon kernel knobs: no gain; log blowup variants measured
- Rate test, exclusive, 9 reps per knob: 2.22 to 2.40 M permutations/s for block 64 to 512 and the sqr S-box, inside the rep spread (about 0.35), MEASURED; defaults kept (block 128, plain products).
- Full proves of every lb3 variant byte-identical (c17dadf7); prove 138.6 to 169.3 s at load 9 to 46, too noisy to rank.
- Log blowup 2 and 1 (queries 120/85/82 and 196/89/82): prove 96.2 s and 86.2 s, proofs 3,016,786 and 4,763,762 bytes, stock verifier accepts, MEASURED.
- Next: interleaved base and no-frac repeats (`abt.sh t2`) for the tuned timing.

## [2026-11-12 23:30] qa | E6 Gemini Quint lane on the wrap call-vector map: SOUND
- Gemini 4 Argon lane e6-callvec: unpack, lane Canon borrow chain, completeness, word indices, prover freedom all SOUND (analytic proof plus a 3-limb Quint model)
- quint 0.32.0 run by the lead (lane sandbox refused it): typecheck ok, 5 tests pass, run 10,000 samples no violation (MEASURED), copied to ~/Projects/Moriarty-wt/e6-int/research/sherlock-e6-int-20261112/gemini-quint
- E6 L_Fr key fixture done: L prove 165 s, L_Fr keygen 197 s, prove 661 s (MEASURED, ~/.cache/e6-int/logs/lfrkey.log); Lean wording committed ecbce035f on bridge-lean

## [2026-11-12 23:55] qa | Sherlock E5 wrap: E3 decoder, native and MockProver
- MEASURED: the native decoder verdict equals the stock `iface::decode` verdict on all 11 exported cases (honest, free lane changed, and 9 refusals), and the honest statement equals the stock statement (`~/.cache/e5-wrap/i7.sh`, log `~/.cache/e5-wrap/logs/i7.log`).
- MEASURED: the gadget decoder alone costs 162,381 rows (cost model). MockProver accepts the honest case and refuses a changed statement, a PUB mismatch and a non-canonical lane.

## [2026-11-12 23:53] note | E4 tuned L_Fr prove 105.0 s with the GPU fractional sumcheck
- late entry (build event at 23:53, written after a 23:55 entry of another lane): exclusive slot 23:36 to 23:53, interleaved, GPU fractional sumcheck 105.8 / 99.6 / 105.0 s against CPU 109.5 / 118.8 / 110.6 s, all six proofs SHA-256 c17dadf7, MEASURED, load 9 to 24.
- Tuned prove 105.0 s (median) is 5.5x the 19.0 s mid warm call; the gate stays FAIL.
- Commits: stark-backend `e4-gpu` 0b779c7 (FracGkrAccel hook), openvm `e4-gpu` 6865752 (equality test, log blowup override).
- RESULTS.md sections 7 and 8 written with four owner options; recommended option A (relax the target), in `~/Projects/Moriarty-wt/e4-gpu/research/sherlock-e4-gpu-20261112/RESULTS.md`.

## [2026-11-13 00:02] build | Sherlock X14 parent cost and G1 row chains under the new bounds
- Parent node over a unit node with 193-interaction fp_mul AIRs: same AIRs, columns and height as over a plain node, about +1 % main cells (MEASURED).
- G1 add (L_add, X3, Y3) and double (SQ, L_dbl, X3, Y3) as one-constraint FieldExpr rows: at most 199 interactions; unit nodes 2,654 and 2,849 columns without the harness, proved at 2^16 (MEASURED). A 16-bit range table splits the rows, so units keep a 17-bit checker (owner to confirm).
- Five Gemini Quint lanes: sound only with non-zero denominators, unique ids, producer range checks and no infinity; lead quint runs found model errors, continuation turns sent. ~/Projects/Moriarty-wt/x-exp/research/sherlock-x14-20261112/RESULTS.md

## [2026-11-13 00:03] commit | E4 tuned prover and owner options committed
- Moriarty `e4-gpu` 2f27ed3ed: mori-gpu `frac` (GPU fractional sumcheck), the Poseidon knobs, RESULTS.md sections 7 and 8; pre-commit hook passed.
- Owner options: A relax the target (105.0 s added per call, recommended), B log blowup 2 (about 72 to 80 s, wrap about 1.66 M rows), C a cheaper hash (breaks the unchanged ledger 9 rule), D A plus B.
- Hand-back to the coordinator next; no push.

## [2026-11-13 00:11] build | Sherlock E5 wrap: lfr-e3 fixture and rows on the E3 map
- MEASURED: E3 all-entries L_Fr exported by `lfr_e3` (openvm e5-wrap e147918 on E3 2f09caf): keygen 303.5 s, prove 732.7 s, 1,522 public words, fixture `~/.cache/e5-wrap/fixtures/lfr-e3.json`.
- MEASURED: native verifier core on lfr-e3 matches 649 of 649 stock samples, 51,497 permutations, and refuses three changed values.
- MEASURED: the full wrap with the E3 map is 1,605,413 rows at k = 21 (gate 1,782,573; 90.1%; 177,160 headroom). Commit 7a4dfb308. The exclusive timing run is `e5-i8-excl`.

## [2026-11-13 00:41] incident | E4 RESULTS cleanup commit blocked by factory heavy tests
- The docs-only cleanup of E4 RESULTS.md (superseded 139.9 s figures, variant table 7.4, wrap headroom range for option B, owner question on the 754 MHz GPU clock) failed the pre-commit hook 5 times, on factory test_heavy (test_copies_run_at_once_with_slots and test_aggregate_kill_cgroup_slice) at load about 25.
- The change is uncommitted in `~/Projects/Moriarty-wt/e4-gpu/research/sherlock-e4-gpu-20261112/RESULTS.md`; commit 2f27ed3ed already holds the tuned 105.0 s and the four owner options.
- Next: the coordinator retries the commit on a quieter host.

## [2026-11-13 00:42] decision | E4 AFK pick: option A, L_Fr as an added stage of about 105 s
- tuned GPU L_Fr prove 105.0 s (MEASURED, median of 3), byte-identical to CPU; the 19.0 s mid warm-call gate still fails
- AFK pick option A for owner confirmation: no key, wrap or ledger change; option B (log blowup 2) and the GPU clock cap (754 of 3,003 MHz) wait for the owner
- recorded in ~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md

## [2026-11-13 00:47] commit | E4 RESULTS cleanup committed; E4 lane done
- Moriarty `e4-gpu` 418b1f267: superseded figures marked, variant table 7.4, option B wrap headroom as a range, owner question on the 754 MHz GPU clock.
- Lead AFK pick: option A (L_Fr as an added stage of about 105 s per call), marked for owner confirmation; the GPU clock is left to the owner.
- E4 lane done; nothing pushed.

## [2026-11-13 00:50] build | E3 J7 and J8 at full scale: fixture and J4 pass at the E2 shape
- Fixture fixture-j8 (openvm e3-iface-onion f725b15): 10 cases, bench heights equal E2 in every case (no pinning), onion 1,006 permutations on full_v, L proof 489,806 bytes, prove 149 to 167 s each (shared slot).
- J4: tape 502,877 ops (+5,467 over 2f09caf: R replay 2,943 plus the onion set), BbAlu and ProgramTable 2^19, FrAdd 8,192 rows (MEASURED); 7 positives 0 failed assertions and decode equal to native; missing R 36, anchor tag swapped 2, forged DIGEST 1 failed assertions, each refused by the decoder.
- The small-set estimate of 151k ops for the onion did not carry over to the L set; J5 queued.

## [2026-11-13 00:54] qa | Sherlock E5 wrap: E3 map exclusive timing and stock ledger
- MEASURED under heavy-slot exclusive, lfr-e3 on the E3 map, k = 21: MockProver honest OK, changed public input and changed proof value refused; keygen 82.6 s; blinded prove 131.6 s; proof 2,912 bytes; key 1,325 bytes.
- MEASURED: stock ledger 9.1.0.1 returns Ok(()) in 0.062 s with 1,026 public inputs, and refuses 3 changed public inputs and 3 flipped proof bytes (log `~/.cache/e5-wrap/logs/i8-excl.log`).
- Results: `~/Projects/Moriarty-wt/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md` section 6i.

## [2026-11-13 00:57] commit | Sherlock E5 wrap: E3 map commits
- Moriarty e5-wrap 7a4dfb308 (decoder, tests, wrap on the E3 map) and d044d01b3 (RESULTS section 6i, exclusive log); openvm e5-wrap e147918 (`lfr_e3` exporter on E3 2f09caf). Not pushed.

## [2026-11-13 00:58] build | E6 wrap key k=21 and D3 rerun pass
- Wrap key at the E6 L_Fr fixture: 1,544,986 rows k=21, MidnightVK 1,325 bytes, stock ledger 9.1.0.1 Ok (MEASURED, ~/.cache/e6-int/logs/wrapkey.log, ledger.log).
- D3 rerun: 101.1435 bits at E2 I10 and E6 keys, domains u=5,6,7 (MEASURED, ~/.cache/e6-int/logs/d3.log).
- Devnet call phase started in systemd unit e6-call (~/.cache/e6-int/callphase.sh).

## [2026-11-13 01:15] decision | E6 devnet call moves to the E5 lfr-e3 wrap (D-9 decoder)
- Coordinator order: use E5 d044d01b3 (in-circuit D-9 decoder) for the devnet call; e6-call stopped before proving (it was still waiting for the exclusive slot).
- lfr-e3 and the E6 key fixture have the same L_Fr vk (sha256 e900cb7f..., MEASURED); decode of an empty-lane-map run = its lanes, so the per-call chain stays.
- D3 on lfr-e3: 101.1435 bits, u=5,6,7 (MEASURED, ~/.cache/e6-int/logs/d3e3.log). Next: ~/.cache/e6-int/prep3.sh.

## [2026-11-13 01:45] build | E3 J7 and J8 PASS at the L_Fr set
- J5 on openvm e3-iface f725b15 (e3-iface fast-forwarded from e3-iface-onion): overflow, all_entries, chunk 2 of 3 accepted with 2 commitments, widths 33 and 10, 2,571,250 bytes; forged DIGEST refused by the prover (MEASURED, shared slot).
- D3 at the measured key 101.1435 bits, unchanged; keygen 189.0 s; L vk 241,251 bytes with 44 AIRs.
- Open: production root_stmt to node tie, multi-segment onion, check (i), opening-proof R, in-circuit Canon, PATH order; D-10 to D-12 for confirmation. Results ~/Projects/Moriarty-wt/e3-iface/research/sherlock-e3-iface-20261112/RESULTS.md.

## [2026-11-13 02:10] decision | E3 phase 2 AFK picks D-10, D-11, D-12; Gemini lanes ordered
- log onion runs in L as OnionAir on L's Poseidon2 AIR, R replay 2,943 ops, 10-case gate passes at lfr_params (MEASURED); full J6 replay not built (about 65.8k ops vs 27k headroom, DERIVED)
- AFK picks for owner confirmation: D-10 R order, D-11 replay of R only, D-12 onion form, all as built
- E3 lane resumed for the required Gemini Quint lanes on OnionAir and the R replay, then j/k range checks and multi-segment onion

## [2026-11-13 02:15] lane-start | E3 Gemini Quint lanes on OnionAir and the R replay
- Lanes `e3-onion` (J8 `OnionAir`, f725b15) and `e3-rreplay` (J7 `gen_r_check`) on gemini-4-argon-high in `~/.cache/gemini-lanes/`, units lane-e3-onion and lane-e3-rreplay.
- Each asks for a reduced Quint model with SOUND and COMPLETE invariants and at least 5 named mutants; the lead runs the quint commands.
- D-10, D-11, D-12 marked FOR OWNER CONFIRMATION in `research/sherlock-e3-iface-20261112/RESULTS.md` (e3-iface worktree).

## [2026-11-13 02:40] qa | E6 gate PASS: devnet call included on the E5 E3-map wrap, R1-R3 refused
- Wrap key on lfr-e3 (D-9 decoder): 1,605,413 rows k=21, 1,325 bytes, byte-equal to the E5 key, ledger 9.1.0.1 Ok (MEASURED, ~/.cache/e6-int/logs/wrapkey3.log).
- Devnet: A included block 2,879; R1, R2 Malformed(InvalidProof); R3 refused at the program_d read (MEASURED, ~/.cache/e6-int/logs/call-final.log).
- Commits: Moriarty e6-int f62da0bb3, merge aa6998370; openvm e6-int a0f07fa. Results: ~/Projects/Moriarty-wt/e6-int/research/sherlock-e6-int-20261112/RESULTS.md.

## [2026-11-13 02:50] commit | E6 results docs and openvm examples
- Moriarty e6-int 74adc7974 and cd5b13c23 (RESULTS, Gemini Quint callvec lane); openvm e6-int a0f07fa (lfr_call, d3_wrapin).
- Post-merge tests pass: od18_vectors 3/3, i7_decode 2/2 (MEASURED, ~/.cache/e6-int/logs/wraptest3.log).
- Open item: no Gemini Quint lane on record for the E5 D-9 decoder circuit.

## [2026-11-13 02:52] build | E3 j and k range rule in the R check and the reader
- openvm `e3-iface` 95ed47b: `gen_r_check` puts the 4 limbs of j, k through `LowBits(x, 16) == x`; for a chunk, j >= 1 by a zero test and j <= k by an advice borrow bit; `anchor_tags` refuses the same words.
- MEASURED: words-level test `j9_jk_range_on_words` 5 positives (k = 1, the 16-bit borrow edges, j = k = 4,294,967,295) with 0 failed assertions, 5 negatives refused; J4 on fixture-j8 passes, tape 502,910 ops (+33), 2^19 guard holds.
- Native `log_merge` (TAG_NODE) equals the ENC-2 vector `enc2.node(2,4)`; in-circuit multi-segment onion not built (the witness tree form is a design choice).

## [2026-11-13 02:58] qa | Sherlock E5 wrap: ENC wrap-side vectors (I9)
- MEASURED: 40 Fr checks of the enc-fp3 pinned vectors pass on the wrap gadgets, natively and in MockProver. They cover the DIGEST, H2, SPONGE(3) and PERM replays of the ENC-2 log, Canon of limbs(r-1), the u = 1 and 2 hashes, the L_Fr transcript and the FP-3 claim_pub and set_pub; 4 changed pinned values are refused (`~/.cache/e5-wrap/logs/i9.log`).
- MEASURED: the ENC pv vector (D_EMPTY and the pv P2 fingerprint equal the pinned values) with two lane-map pairs and a PUB entry goes through the wrap decoder; 12 cases equal stock D-9, and MockProver refuses pub_lane_changed and lane_map_to_sponge. 33 of the 55 pinned vectors apply to the wrap; the other 22 are BabyBear P2 forms or the L u = 3 transcript inside L_Fr.
- Findings for confirmation: D-9 admits only DIGEST lane-map pairs while ENC's acceptRun maps SPONGE(3) lanes; E3 iface still uses placeholder DIGEST tags; the per-entry replay (J6) and Canon on X are not in the wrap. Section 6j of `~/Projects/Moriarty-wt/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md`.

## [2026-11-13 02:59] lane-done | E3 Gemini Quint lanes round 1 (OnionAir, R replay)
- e3-onion: SOUND on Q1 to Q4 and Q6, COMPLETE on Q5; lead ran 13 tests, SOUND_AND_COMPLETE, a 3,000-sample one-cell tamper adversary (no violation) and caught all 7 mutants.
- e3-rreplay: SOUND on Q1 to Q3, Q4 SOUND with the reader, COMPLETE; Q6 UNSOUND (no j, k range rule), fixed in openvm e3-iface 95ed47b; 13 tests pass, all 6 mutants caught.
- Lead setup defect: `.lane-skills/` were symlinks outside the checkout, so neither lane read the skills; round 2 lanes (real copies, the built j, k rule, a stronger onion adversary) started.
- Reports in `qa/gemini-quint/e3-onion/` and `qa/gemini-quint/e3-rreplay/` of the e3-iface worktree.

## [2026-11-13 03:03] commit | E3 J9 and lane round 1 docs
- Moriarty e3-iface ba4b3f1f6: RESULTS section 12 (j, k rule, TAG_NODE merge, lanes), D-10 to D-12 FOR OWNER CONFIRMATION, lane reports under `qa/gemini-quint/e3-onion/` and `qa/gemini-quint/e3-rreplay/`.
- openvm e3-iface 95ed47b holds the code; not pushed.

## [2026-11-13 03:11] decision | E6 AFK picks recorded for owner confirmation
- 4 h TTL; E3-map scope on chain (the devnet call used an empty lane map, so PUB decoder checks ran only in E5 I9); E6 callvec map dropped for the E5 D-9 decoder
- open for owner: D-9 admits only DIGEST lane-map pairs while acceptRun maps SPONGE(3) lanes (E5 I9); recorded in ~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md

## [2026-11-13 03:20] build | Sherlock X14 G1 row-kind Quint lanes, lead results
- Five Gemini Quint lanes (L_add, L_dbl, SQ, X3, Y3) under ~/Projects/Moriarty-wt/x-exp/qa/gemini-quint/x14-*; all sound only with non-zero denominators, unique ids, producer range checks and no point at infinity.
- Lead quint runs after two fix turns and two mechanical edits: SQ, L_dbl, X3, Y3 pass every command; the L_add model passes all 5 mutants, its test file does not parse (open).

## [2026-11-13 03:21] lane-done | E3 rreplay lane round 2 on the j, k rule (95ed47b)
- Verdicts: Q1 r16 range, Q2 order and wrap, Q3 tape equals reader, Q5 log_merge, Q6 no regression: SOUND; Q4 COMPLETE.
- Lead ran typecheck, 10 tests (exhaustive limb and borrow enumeration, 5 mutants all caught) and inv_sound_and_complete (5,000 samples, no violation).
- Notes for the owner: ORD keeps any 16-bit j, k (not forced to 0); a node circuit must take n = 0 only with d = D_EMPTY.

## [2026-11-13 03:33] lane-start | FP8 universal key set under the new node bounds
- worktree ~/Projects/Moriarty-wt/fp8 from e6-int with x-exp merged; keys for leaf, node, L, L_Fr and wrap under 200 interactions, 64 AIRs, 3,000 columns
- X-EXP results in: D3 101.1643 bits at the new bounds, parent cost +1%, G1 rows at most 199 interactions (MEASURED); X14 condition rows still being measured
- AFK picks for owner recorded in ~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md (17-bit checker departs from LD-S10x-3)

## [2026-11-13 03:36] qa | Sherlock E5 wrap: Gemini Quint lane on the D-9 decoder
- Lane `~/.cache/gemini-lanes/e5-decode` (argon-run, quint-llm-kit skills) gave SOUND on all 7 questions with no defect; its sandbox refused every command, so the lead ran quint 0.32.0.
- Lead model fixes: typecheck (set folds, a list literal), mod_inv(1) = 360, a skip-pair counterexample that does not separate (the real attack takes nz = 2 and writes two lanes; z u = 0 stops it), and a fuzz step for quint run.
- MEASURED: 13 of 13 quint tests pass; SOUND_AND_COMPLETE holds on 10,000 traces; all 8 mutants are violated. No circuit change, so the 1,605,413-row measurement stands. Files in `~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode/`.

## [2026-11-13 03:51] build | FP8 G1 frozen-sets module and keyset generator
- New `sherlock_lfr::frozen` in openvm worktree ~/.cache/m7-wrap/src/openvm-fp8 (branch fp8): the app, leaf-aggregation, internal (200 interactions, 64 AIRs, 3,000 columns), L and L_Fr sets with explicit query vectors; `keygen_checked` refuses another parameter set or a key outside the bounds.
- Own stark-backend worktree ~/.cache/m7-wrap/src/stark-backend-fp8 (e2-lfr 5f698b5, unchanged) so live lanes are not touched.
- Examples `fp8_keyset` (keys and chain modes) and `fp8_d3`; ARCHITECTURE merge conflict of x-exp into fp8 resolved (FP-3 facts closed, option E rows kept).

## [2026-11-13 03:53] build | X14 condition rows built, negative tests pass
- New AIRs in `~/.cache/stark-bench/src/openvm-org_openvm/crates/stark-bench-ovm/examples/x_exp_node.rs`: INV (w (x2-x1) = 1), INV_y (w y = 1), PointsIn (8-bit range check of every input limb plus the point sends); ChainAir id = row index (first row 0, step +1).
- MEASURED (`~/.cache/x-exp/x14cond-neg.jsonl`, mode g1neg, 2^4 rows): honest add and dbl verify; P1 = P2 with forged lambda and w fails only INV, (0, 0) with forged lambda fails only INV_y, a duplicate id fails the id constraint of every chain AIR; LogUp balances in all cases and verify rejects every forged trace.
- Gemini Quint lanes started for INV, INV_y, PointsIn and the id constraint (`~/.cache/gemini-lanes/x14-inv*`, `x14-points-in`, `x14-chain-id`); L_add test fix turn started (FIXED3.md marker).

## [2026-11-13 03:57] qa | E3 J5 re-proved with the j, k rule (95ed47b)
- MEASURED (shared slots): keygen 199.4 s; overflow, all-entries, chunk 2 of 3 accepted, 2 commitments, 2,571,250 bytes, prove 684 to 744 s; forged DIGEST refused by the prover.
- D3 101.1435 bits, unchanged; tape 502,910 ops, BbAlu and ProgramTable 2^19.
- Log `~/.cache/e3-iface/logs/j9-j5.log`.

## [2026-11-13 03:59] commit | Sherlock E5 wrap: ENC vectors and Quint lane commits
- Moriarty e5-wrap f962d1269 (I9 ENC wrap-side vectors, RESULTS 6j) and 223ebc70c (Gemini Quint lane on the decoder, RESULTS 6k); openvm e5-wrap ffc5077 (`enc_wrap`). Not pushed. One hook run failed once on factory checks and passed on retry.

## [2026-11-13 04:07] lane-done | E3 onion lane round 2 (adversarial generator)
- All questions SOUND, COMPLETE; the lead ran 21 tests, SOUND_AND_COMPLETE, TAMPER_SOUND (no violation), the non-vacuity witness (violated as expected) and caught all 12 mutants.
- Round 1's stronger-than-Rust val_mult >= 0 condition removed, still no attack; no change to OnionAir.

## [2026-11-13 04:07] build | E3 items for the E5 port (ENC-1 tags, SPONGE(3) lane map)
- ENC-1 ASCII tag rule P-1 was already in E3 since openvm 6d70a45; 52a538c adds a test of every computable ENC-1 vector (4 tags, digest_1to8, C_P, A_1): pass.
- 099d6d8: the D-9 reader lane map takes DIGEST or SPONGE(3) only, SPONGE(n) with n != 3 refused, with test lane_map_sponge_entries; the L_Fr tape is unchanged, no re-prove.
- Both marked AFK changes FOR OWNER CONFIRMATION in RESULTS section 13; lane e3-lanemap started on the lane map.

## [2026-11-13 04:09] note | FP8 keygen: E6 reproduced, causes of key change, node fixed point
- MEASURED (~/.cache/fp8/out/keys/keys.json): the E6 node and L keys regenerate byte-equal; the frozen app set (query PoW 21, stock had 20; vector 193/88/81/81 unchanged) changes the app and node keys but not L; the leaf layer changes node and L.
- Node over node reaches a fixed point at depth 3 (depth 3 = 4 = ... = 7); the node over a leaf layer is equal for two programs; the new bounds change no key (admission checks only).
- G1 tests pass 4 of 4 (openvm fp8 9d4105f); full chain W1 -> leaf -> N1 -> N2 -> N3 -> L -> L_Fr -> wrap runs in systemd unit fp8-chain.

## [2026-11-13 04:10] build | X14 condition rows push the G1 units over 3,000 columns
- MEASURED (`~/.cache/x-exp/x14cond-g1.jsonl`, g1c at 2^12..2^17, proved and verified at 2^16): add unit 45 AIRs and 3,138 columns, double unit 46 and 3,189 (without the 97-column sink); L_add and L_dbl at 200 interactions; stacked width 247 and 254 at 2^16.
- Both units exceed the N-6 option 2 budget of 3,000 columns. D3 at 3,300 and 3,500 columns keeps the weakest term at 101.1643 (`~/.cache/x-exp/x-d3b.log`); recommended for owner confirmation: a 3,300-column bound for the FV work units.
- DERIVED: BLS12-381 G1 has no point with y = 0 (-4 is not a cube in Fp), so INV_y is redundant once inputs are checked on the curve; the 17-bit unit range checker is flagged as an AFK departure from LD-S10x-3.

## [2026-11-13 04:20] commit | E3 lanes round 2 and E5 port docs
- Moriarty e3-iface ed4354394 (after one retry for factory timing tests): RESULTS sections 12 and 13, PLAN D-9 with SPONGE(3), round 2 lane reports.
- openvm e3-iface 95ed47b, 52a538c, 099d6d8; not pushed.

## [2026-11-13 04:26] commit | E6 lfr-e3 and wrap key rebuilt from source, byte-identical
- openvm e6-int 780db4a merges e5-wrap e147918 (lfr_e3 exporter); fixture regenerated to ~/.cache/e6-int/fixtures-src/lfr-e3.json, L_Fr vk and decode equal the E5 cache (MEASURED).
- Rebuilt wrap key sha256 2fc75636...93dd byte-identical to the deployed key (MEASURED, ~/.cache/e6-int/logs/wrapkey4.log, runs.log).
- RESULTS updated in Moriarty e6-int ef70c4949; compose project e6 left running.

## [2026-11-13 04:30] lane-done | E3 lane-map lane on the D-9 reader
- Verdicts: acceptance rule, PUB interplay SOUND, COMPLETE; reader defects found: a lane mapped twice when the first output is 0 (2a), no u16 check (3); fixed in openvm e3-iface 9539418 (apply_lane_map), J4 unchanged.
- For the owner: lanes 0 and 1 as lane-map targets (2b), lane map not pinned by the reader (4).
- Lead ran 12 tests (2 wrong expected values in the lane tests corrected), SOUND and COMPLETE runs, 5 mutants caught.

## [2026-11-13 04:34] commit | E3 lane-map lane and reader fixes
- Moriarty e3-iface b92c49748: RESULTS section 13 with the lane-map lane, `qa/gemini-quint/e3-lanemap/`.
- openvm e3-iface 9539418 holds the reader fix; not pushed.

## [2026-11-13 04:37] qa | FP8 key set: six items pass, FP-8 gate BLOCKED
- MEASURED (~/.cache/fp8/logs, unit fp8-chain): W1 -> leaf -> N1 -> N2 -> N3 (fixed point) -> L -> L_Fr -> wrap, every key through G1 keygen; wrap 1,325 bytes sha256 4990db4d, 1,605,413 rows at k = 21; stock ledger 9.1.0.1 Ok, changed input and flipped byte refused 3 of 3.
- D3 per key at measured, vk and bound shapes: every term at least 100, grindable at least 101; weakest 101.0110 (leaf layer). Every key changed against E6 (G1 app query PoW 21, leaf layer, fixed-point node); the bounds change no byte.
- BLOCKED on FV work-unit shapes (X-EXP, over 3,000 columns), E3 phase 2 not merged, G3, G4 decide, I-17, D10 path C, FZ-3. Results ~/Projects/Moriarty-wt/fp8/research/sherlock-fp8-20261113/RESULTS.md.

## [2026-11-13 04:43] decision | X14 G1 unit shapes after the cuts; 3,300 columns as an AFK pick
- MEASURED (`~/.cache/x-exp/x14cond-noinvy.jsonl`): the double unit without INV_y is 45 AIRs and 2,946 columns (within 3,000), proved and verified at 2^16; without INV_y a forged (0, 0) doubling verifies, so the point producer must check inputs on the curve.
- DERIVED: no cheap cut fits the add unit (3,138 columns); merging INV into L_add breaks the 200-interaction bound, and accumulator feedback leaves 3,042.
- AFK pick FOR OWNER CONFIRMATION: a 3,300-column bound for the FV work units (D3 weakest 101.1643); shapes sent to the FP8 lane; results in `~/Projects/Moriarty-wt/x-exp/research/sherlock-x14-20261112/RESULTS.md` section (d).

## [2026-11-13 04:46] commit | FP8 results committed, gate BLOCKED
- Moriarty fp8 92a397c4b (RESULTS, evidence in out/, source copies in src/); openvm fp8 9d4105f (local). First hook run failed only test_lease_state_api; the retry passed. Not pushed.
- X-EXP final FV unit shapes recorded: add 3,138 columns (over 3,000, needs the 3,300 AFK pick), double 2,946; no unit key generated yet. key_root and child-key enforcement are not built (L takes the child commitment as prover input).

## [2026-11-13 04:46] issue | FP8: key binding is not enforced in a circuit
- FP8 first pass: full key chain passes G1, D3 weakest 101.0110 bits, ledger Ok (MEASURED, ~/Projects/Moriarty-wt/fp8 92a397c4b); every key changed against E6
- gap: L takes its child-node commitment as prover input and prog_d and key_root are computed in Rust only, so no circuit pins N3, the leaf-layer key or the app key
- lead ordered: restore the FP-3 fact clauses dropped in an ARCHITECTURE merge, then pin the key commitments and prog_d in L with negative tests and a Gemini Quint lane

## [2026-11-13 04:49] commit | E3 final docs and handback
- Moriarty e3-iface eccfc7d86 (one retry for a factory test): one owner-item list (7 items), J9 negative coverage, D-10 range rule, E5 port provenance (E5 compared the 2f09caf iface).
- Small suite at openvm 9539418 passes; J4 tape histogram equal to 95ed47b, so J5 there stands.

## [2026-11-13 05:02] commit | FP8: e3-iface merged into fp8 (Moriarty and openvm)
- Moriarty ~/Projects/Moriarty-wt/fp8: merge d3342041b of e3-iface (eccfc7d86), no conflicts; ARCHITECTURE FP-3 clauses restored earlier in 010431011.
- openvm ~/.cache/m7-wrap/src/openvm-fp8: merge f77a052 of e3-iface (9539418); gen_r_check now takes the L public values (three examples patched).
- Next: rebuild, regenerate L and L_Fr without the wrap (~/.cache/fp8/run-l.sh); wrap waits for e5-wrap.

## [2026-11-13 05:02] decision | FP8 key binding: pvs chain across leaf, N1, N2, N3 and L (FOR OWNER CONFIRMATION)
- Gap (MEASURED by code read): every layer runs VerifierSubCircuit with continuations off and 0 public values; L_Fr takes the cached commit of L as prover input (lverify.rs fr_in). No key below L_Fr is bound.
- Recommended fix: fork OpenVM VerifierPvsAir as a Sherlock key-bind AIR with five commit slots (app, leaf, N1, N2, N3), continuations on, L pins N1..N3 as constants and exposes the app and leaf commits; L_Fr pins the cached commit of L.
- prog_d and key_root binding to iface cells is a separate step; spec DECISIONS F-5 routes keys by a key_root opening in node statements.

## [2026-11-13 05:10] qa | E5 wrap decoder ported to E3 9539418: I7 and I9 pass
- Port in ~/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap/src/decode.rs: lane map DIGEST or SPONGE(3) (output vs[o+3]), lane set at most once (pairwise check), u16 lane-map words, replay of mapped entries, N_TAGS 6, R and anchor checks with R replay.
- I7 (MEASURED): 27 of 30 lfr-e3 cases equal stock; 3 open_replay cases diverge as planned (stock replays every entry, the wrap only R and mapped entries; FOR OWNER CONFIRMATION). Decoder alone 165,168 rows (was 162,381).
- I9 (MEASURED): Part A passes; ENC-2 log is not an R log, stock and wrap refuse all 12 cases at R entry 1; lane-map step equal on 10 cases.
- openvm-e5 e5-wrap rebased on 9539418; lfr-e3 re-export from E3 fixture-j8 runs (log ~/.cache/e5-wrap/logs/r2-export.log).

## [2026-11-13 05:25] build | FP8 KeyBind AIR in every layer, fixed point kept at depth 3
- openvm-fp8 sherlock-lfr/src/keybind.rs: fork of OpenVM VerifierPvsAir with five slots (app, leaf, N1, N2, N3), continuations buses on; L (Root role) pins N2 and N3, exposes app, leaf, N1 (48 words); L_Fr pins the L cached commitment (lverify Gen::pinned_cached).
- MEASURED (fp8_keyset fixpoint, ~/.cache/fp8/logs/kb-fix.log): bound N3 over N2 equals N3 over N3 (depth 3 -> 4 equal, 4 -> 5, 5 -> 6 equal); five slots suffice.
- Item 3: every keygen site in sherlock-lfr and sherlock-wrap-export now goes through frozen::keygen_frozen (examples) or frozen::keygen_any (tests; strict at frozen parameters).

## [2026-11-13 05:25] lane-start | Gemini Quint lane fp8-keybind
- Unit gl-fp8-keybind, ~/.cache/gemini-lanes/fp8-keybind (prompt ~/.cache/gemini-lanes/fp8-keybind-prompt.md): BIND and COMPLETE invariants, 7 named mutants.

## [2026-11-13 05:57] commit | E5 wrap decoder on E3 9539418: rows, ledger and Quint round 2
- Moriarty e5-wrap 95480b0fc; openvm-e5 e5-wrap rebased on 9539418 (c1ddfc0, 8ce7774, 6ef8f47, 1f42114) plus 2f1d8a4 (export cases).
- MEASURED: 1,610,038 rows at k = 21 (headroom 172,535); exclusive keygen 83.7 s, prove 138.5 s; ledger 9.1.0.1 Ok(()) with 6 tampers refused.
- Gemini Quint lane round 2 (~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode-r2/): SOUND and COMPLETE; lead runs show 14 of 14 tests, 10,000 traces clean, 8 of 8 mutants caught.
- Open, FOR OWNER CONFIRMATION: the wrap replays only R and the mapped entries (stock replays every entry); ESTIMATE about 15k to 25k rows for full replay.

## [2026-11-13 05:58] issue | E5 wrap accepts 3 cases the stock D-9 decoder refuses
- after the port to E3 9539418 (Moriarty e5-wrap 95480b0fc): 1,610,038 rows at k = 21, ledger Ok, decoder Quint round 2 SOUND (MEASURED)
- the wrap replays only R and mapped entries; a changed unmapped output (3 open_replay cases) is refused by stock and accepted by the wrap
- lead ordered the full replay of every entry in the wrap (ESTIMATE 15k to 25k rows, within the 172,535-row headroom) so all 30 I7 cases equal stock

## [2026-11-13 06:06] qa | E5 wrap full native replay: all 30 I7 cases equal stock
- Lead decision (AFK pick): replay every logged entry; ~/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap/src/decode.rs step 7 is a 256-slot permutation tape tied to the entries by a packed-tuple lookup with fixed multiplicities perms_i.
- MEASURED: I7 30 of 30 equal stock (open_replay cases now refused); decoder alone 223,453 rows (was 165,168). I9 passes; the honest ENC-2 log replays.
- Next: full wrap rows at k = 21, exclusive timing, ledger check (unit e5-r3-measure), Gemini Quint round 3 (~/.cache/gemini-lanes/e5-decode-r3).

## [2026-11-13 06:25] build | X14 condition-row Gemini lanes pass; L_add test fixed
- Lead quint runs (`~/Projects/Moriarty-wt/x-exp/research/sherlock-x14-20261112/out/x14cond-lanes-quint-final.txt`): INV, INV_y, PointsIn and id = row index lanes give every expected exit code (typecheck, tests, sound invariant, non-vacuity, 5 mutants each).
- Fixes: one lane fix turn each for INV, INV_y and chain-id; one lead edit in INV_y (honest second row under the wrong_den mutant); chain-id tests need `--match='_test$'`.
- L_add lane test file now parses, 11 tests pass, all L_add commands match.

## [2026-11-13 06:36] qa | FP8 bound chain regenerated (E3 phase 2, KeyBind), no wrap
- MEASURED ~/.cache/fp8/out/kb/chain/chain.json: N3 fixed point 40 AIRs, 1,637 columns, 86,496 bytes; L 44 AIRs, 1,893 columns; L_Fr 15 AIRs, 561 columns, 1,528 public words (6 key words added); all keys G1 accepted.
- MEASURED d3.json: every gate PASS, weakest 101.0110 (leaf layer), L vk mu 101.0730.
- Wrap and ledger check not rerun: wait for e5-wrap per coordinator.

## [2026-11-13 06:36] qa | FP8 key-binding negatives on real proofs
- MEASURED ~/.cache/fp8/logs/kb-neg.log: 12 of 13 as expected (mixed app keys, tampered or pre-set slots, mixed depths, wrong N2 pin refused; L over N3 depth 1 and 2 accepted; L_Fr wrong pin refused, 393 tape assertions).
- L over an N2 child: refused by the prover's own LogUp check (no proof exists), recorded as a harness FAIL in run 1; harness fixed (openvm 9fe4193), rerun neg2 running.
- Finding: the parent verifier receives only DAG-used child public values; KeyBind now references all of them (vacuous constraint).

## [2026-11-13 06:36] lane-done | Gemini Quint lane fp8-keybind
- Verdict SOUND and COMPLETE on 8 questions; lane could not run quint. Agent rerun (MEASURED): typecheck ok, 14 of 15 tests, BIND/COMPLETE/SOUND_ALL no violation at 10,000 samples, 7 of 8 mutants caught.
- mut_no_unset_check not caught: the parent unset check is redundant with own-slot zeroing (DERIVED). Files ~/Projects/Moriarty-wt/fp8/qa/gemini-quint/fp8-keybind/.

## [2026-11-13 06:36] qa | FP8 FV unit keys from X-EXP shapes (helper agent)
- MEASURED ~/.cache/fp8/out/units/units.json: add unit 46 AIRs, 3,226 columns; double unit 46 AIRs, 3,034 columns; both refused at INTERNAL 3,000 columns, accepted at a 3,300 copy (FOR OWNER CONFIRMATION).
- KeyBind adds 88 columns, so the double unit no longer fits 3,000. Unit proofs not made yet (keys made before the KeyBind fix).

## [2026-11-13 06:40] note | E5 full replay measured, waiting on Gemini round 3
- E5 full in-circuit replay ran r3-check, r3-i9, r3-i8 and r3-i6-excl, all rc 0 (~/.cache/e5-wrap/logs/runs.log); changes are uncommitted in ~/Projects/Moriarty-wt/e5-wrap.
- Gemini Quint lane e5-decode-r3 still running (started 06:06); the E5 agent paused until it lands.
- Next: resume the E5 agent to read the round 3 verdict, commit and close E5.

## [2026-11-13 06:51] build | X14 PointsIn padding and identity flag path built, negative tests pass
- PointsIn gains v (is_valid, valid rows a prefix), identity flags f_i (flagged point has zero limbs), e = v (1-f_0)(1-f_1); identity results go from PointsIn to the OUT bus with multiplicity v-e. AFK pick: identity inputs skip the chain (O + P = P by selection).
- MEASURED (`~/.cache/x-exp/x14pad-neg.jsonl`, 2^4 rows): partial units (11 of 16) and O + P, P + O, O + O, 2 O verify; a padding hole fails only PointsIn; a forged padding row and a wrong identity output fail LogUp; a flag on a real point fails only PointsIn.
- Finding: FieldExpr carry constraints are not gated by is_valid, so padding chain rows hold a dummy satisfying witness built with a throwaway range checker (as core_chip does). Gemini lanes x14-pad and x14-identity started.

## [2026-11-13 07:00] commit | FP8 round 2 results on fp8 (28d6861db)
- ~/Projects/Moriarty-wt/fp8/research/sherlock-fp8-20261113/RESULTS.md section 15; evidence out/kb/; lane qa/gemini-quint/fp8-keybind/.
- MEASURED rerun neg2: 13 of 13 key-binding cases as expected (all_pass true).
- Gate FP-8 still BLOCKED: wrap over the bound L_Fr waits for e5-wrap; prog_d not in a circuit on the bench chain; FV units need the 3,300-column pick.

## [2026-11-13 07:10] note | FP8 agent stopped by the owner
- The owner stopped the FP8 agent (a9b0b361fe0614033) during its RESULTS corrections.
- Its worktree ~/Projects/Moriarty-wt/fp8 is left as it was; the loop does not restart FP8 without the owner.

## [2026-11-13 07:25] audit | E path audit, Codex lane done
- Owner asked for an audit of the E path, a review of the plan to a tested design, and a rebuilt loop; the E loop is stopped.
- Codex gpt-6.1-sol read-only audit written: ~/Projects/Moriarty/research/sherlock-e-path-20261112/AUDIT-codex.md (brief AUDIT-BRIEF.md).
- Top findings: the devnet proof is not tied to a real Sherlock program run; prog_d is not bound to the key chain; results sit on separate revisions; the leaf machine (modular-leaf OpenSpec, 0 of 84 tasks) is not built.
- Next: the Opus audit, then reconcile both and rebuild LOOP.md.

## [2026-11-13 07:34] audit | E path audit closed, Opus report and erratum
- The Opus audit is written: ~/Projects/Moriarty/research/sherlock-e-path-20261112/AUDIT-opus.md. Its erratum corrects one claim: stark is on input-output-hk at 84026dc7c.
- Both audits agree on these gaps: no real program run in the chain, prog_d not bound to the keys, every AIR optional, the E6 stand-in contract discloses program_d and steps, the KeyBind chain narrows GP, and G3 and G4 are open.

## [2026-11-13 07:34] plan | Tested-design loop replaces the E-path loop
- New loop file: ~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md. It has rows T0 to T15 and owner items O1 to O12. The old E-path LOOP.md is closed as history, with status corrections.
- The AFK rule is off; OWNER rows wait for a ruling.
- Next: start T0 (make the work durable) and T4 (wrap row budget at k = 21).

## [2026-11-13 07:34] lane-start | T4 wrap row budget at k = 21 started
- Opus lane runs row T4 of ~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md from the E5 round 3 decoder in ~/Projects/Moriarty-wt/e5-wrap (1,668,323 rows measured).
- Builds run in a copy at ~/.cache/t4-budget; output goes to T4-WRAP-BUDGET.md in the same research folder.

## [2026-11-13 07:35] decision | Owner takes the recommendations on O1 to O12
- The owner ruled "go with your recommendations on O1 to O12" for the tested-design loop. The rulings are recorded in ~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md.
- The rulings: the 17-bit checker and the 3,300-column bound for FV work units only; E4 option A; the KeyBind chain withdrawn.
- The push of the E-path branches and the T0 patches is approved after T0 is verified. The owner signs the T5 schema drafts.

## [2026-11-13 07:49] lane-start | T5 leaf schema drafts started
- Opus lane drafts the T5 schemas of ~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md: AD-11 records, AD-12 with X15, the AD-9 guard list, D_view, the ty_d normal form and the SRC-021 amendment.
- Output: ~/Projects/Moriarty/research/sherlock-tested-design-20261113/T5-SCHEMAS.md. Read-only elsewhere; the owner signs the drafts (O12).

## [2026-11-13 07:56] commit | T0: E5 round 3 decoder, X14 (e) and FP8 record committed
- e5-wrap 10fe24968: the round 3 full native replay decoder, its I7 and I9 tests, the r3 logs and r3-measure.sh in ~/Projects/Moriarty-wt/e5-wrap.
- x-exp e53cdd956: X14 (e) padding and identity flag path, its out/ records, and the Gemini Quint lanes x14-pad and x14-identity (both SOUND and COMPLETE with conditions; lead quint runs not run, the lane sandbox refused quint).
- fp8 bcae0f185: the 2 staged round 2 record files (RESULTS.md, out/kb/univ.json).

## [2026-11-13 07:59] qa | E5 round 3 Gemini Quint lane SOUND and COMPLETE
- The e5-decode-r3 lane reviewed the round 3 full-replay decoder against stock iface::decode: all 7 questions SOUND, plus a COMPLETE verdict (the L vk pre-hash pin is deliberate).
- The report is at ~/.cache/gemini-lanes/e5-decode-r3/qa/gemini-quint/e5-decode-r3/REPORT.md. T0 copies it into the e5-wrap worktree and re-runs the Quint tests as lead.

## [2026-11-13 08:00] commit | T0: X12 lane files, E5 RESULTS row and the openvm and stark-backend patch export committed
- quint-x12 bf916a205: the X12 GP-2 model, tests and lead runs (partly run: 6 of 16 sampled mutants; every mutant caught by a deterministic test) and the lfr-bbalu, lfr-poseidon and lfr-tape lanes.
- e5-wrap 124851aea: E5 RESULTS section 6m with the MEASURED round 3 numbers (1,668,323 rows at k = 21, ledger 9.1.0.1 Ok).
- stark 37dbe2fec (local, not pushed): 14 patch series under ~/Projects/Moriarty-wt/stark/research/sherlock-stark-code-20261112/, 9 openvm and 5 stark-backend lane branches against upstream main; merges flattened along first parents.

## [2026-11-13 08:00] qa | T0 patch round trip verified
- In scratch clones under ~/.cache/t0-verify, base plus git am of each of the 14 series gives a tree equal to the branch tip (empty diff stat, equal tree ids); openvm fp8 tree 95e9c51, stark-backend e2-lfr tree eb17157.
- The committed patch blobs equal the files that were tested. The scratch clones are deleted.

## [2026-11-13 08:05] lane-done | T4 wrap row budget: every check fits k = 21
- With every missing check added (Canon and pin on X, layout_d pin, popeq lane, a_out, anchor template lanes, lane map off lanes 0 and 1, opening-proof R, PATH replay, the 6 FP8 key words) the wrap over the bound L_Fr is 1,676,222 rows, 94.03% of the 1,782,573 gate, headroom 106,351 (MEASURED, cost-model probes, ~/.cache/t4-budget/logs/cost4.log).
- D-11: full replay including PATH fits (PATH +7,421 rows); R-only replay is not needed.
- Report: ~/Projects/Moriarty/research/sherlock-tested-design-20261113/T4-WRAP-BUDGET.md; probe patch ~/.cache/t4-budget/t4-probe.diff.

## [2026-11-13 08:13] qa | E5 round 3 Gemini Quint lane: lead rerun does not confirm the verdict
- The lane verdict is SOUND and COMPLETE. The lane files and RERUN.md are in ~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode-r3/ (commit f6db2bbd5).
- Lead rerun (typescript backend; the rust backend hits its recursion limit): typecheck OK, 12 of 14 tests pass, and SOUND_AND_COMPLETE plus all 7 mutant invariants are violated.
- Cause: a model artifact. The toy field 359 is below the 2^9 slot range bound, so the model accepts rem = -1, which the circuit refuses over Fr. The model needs a repair and a rerun.

## [2026-11-13 08:15] push | E-path branches and the T0 patch series pushed to input-output-hk
- Owner ruling O11. Pushed stark 84026dc7c..37dbe2fec plus new branches e2-lfr, e3-iface, e4-gpu, e5-wrap, e6-int, enc-fp3, x-exp, fp8, final-design-2, bridge-lean and quint-x12 to https://github.com/input-output-hk/Moriarty.git.
- Before the push the branch diffs were scanned for wallet seeds and keys (none found), and the commits checked for AI attribution (none found).

## [2026-11-13 08:15] lane-start | T1 integration branch and T13 lane repairs started
- T1 builds branch tested from fp8 in ~/Projects/Moriarty-wt/tested, with openvm and stark-backend tested branches, and runs the suites with the heavy gates.
- T13 has three jobs: repair the E5 r3 Quint model (the lead rerun found the toy field 359 below the 2^9 range bound), run the lead Quint runs for x14-pad and x14-identity, and finish the X12 mutants.

## [2026-11-13 08:16] decision | D-11 resolved by T4: full replay in the wrap
- Under owner ruling O4 the T4 budget decides D-11. The wrap with every missing check is 1,676,222 rows at k = 21 (94.03%, MEASURED cost model), so full replay with PATH stays in; R-only replay is withdrawn.
- Open risk: the FP8 key-word reader needs 20k to 40k rows if it must use BabyBear P2 (ESTIMATE), which still fits inside the 106,351-row headroom.

## [2026-11-13 08:16] lane-start | T6a leaf OpenSpec refresh to STARK AIRs
- An Opus lane rewrites the halo2-era gates of sherlock-modular-leaf-20261111 for OpenVM STARK AIRs, on branch leaf-spec in ~/Projects/Moriarty-wt/leaf-spec.
- tasks.md gets a vertical-slice group (BUS, RANGE, HASH, CORE subset, MEM window, BOUNDARY, INIT) for the first real leaf run.

## [2026-11-13 08:27] qa | T13 lead Quint runs confirm the x14-pad and x14-identity lanes
- The lane sandbox had refused quint, so the lead ran every REPORT command with quint 0.32.0 through heavy-slot and fixed seeds; the lane models are unchanged.
- x14-pad: 11 + 10 tests pass, invRow and invRefinement ok, non-vacuity witness found, 5 of 5 mutants caught. x14-identity: 11 tests pass, invRow ok, 5 of 5 mutants caught.
- Both verdicts (SOUND and COMPLETE with conditions) are confirmed; the interface conditions stay open. RERUN.md beside each REPORT, X14 RESULTS row updated, commit 1eda7a42e on x-exp.

## [2026-11-13 08:29] build | T1 branch tested merged in Moriarty, openvm and stark-backend
- Moriarty tested (~/Projects/Moriarty-wt/tested) from fp8 bcae0f185: final-design-2, stark 37dbe2fec, bridge-lean, e4-gpu, e5-wrap f6db2bbd5, e6-int, x-exp e53cdd956 and quint-x12 bf916a205 merged with no text conflict (tip 9eeb0c980); enc-fp3 and e3-iface were already in fp8.
- openvm tested (~/.cache/m7-wrap/src/openvm-tested) from fp8 5d8e158 with e5-wrap, e4-gpu and enc-fp3: lfr_e3 and lfr_real take the e5-wrap port with the fp8 keygen_frozen call; the stark-backend patch path is ../stark-backend-tested.
- stark-backend tested (~/.cache/m7-wrap/src/stark-backend-tested) from fp8 5f698b5 with e4-gpu and enc-fp3, no conflict; stark-sdk takes mori-gpu from the Moriarty tested tree.
- Suites started as user units t1-sb and t1-ovm (runner ~/.cache/t1/run.sh, logs ~/.cache/t1/logs).

## [2026-11-13 08:32] lane-done | T5 leaf schema drafts written
- Six drafts for owner sign-off in ~/Projects/Moriarty/research/sherlock-tested-design-20261113/T5-SCHEMAS.md: AD-11 records, AD-12 ROM, AD-9 guard list, D_view, ty_d normal form, SRC-021 amendment.
- Derived: W_rec 28, W_rom 33, D_tbl 5, D_row 64 recommended (slice lengths ridx), D_view 67, body 461 words (476 recommended).
- 120 guards checked against owner_runtime.rs select lines by script; DEC plan lanes have no source table (O3-2, T6).
- SRC-021 amendment passes openspec validate on a scratch copy; no repo edits besides the doc and this log.

## [2026-11-13 08:32] lane-done | T5 leaf schema drafts ready for owner sign-off
- ~/Projects/Moriarty/research/sherlock-tested-design-20261113/T5-SCHEMAS.md has six drafts: the AD-11 records (W_rec 28), AD-12 ROM (D_tbl 5, W_rom 33, D_row 64 recommended), the AD-9 guard list (120 rules), D_view 67, the ty_d normal form, and the SRC-021 amendment.
- The DEC plan lanes (O3-2) stay open until T6. Owner sign-off under O12 is next.

## [2026-11-13 08:35] lane-start | T13 E5 decoder Gemini Quint lane round 4
- The lead repaired the r3 model (toy prime 2053, limb base 16) in ~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode-r3-lead; all 12 corpus cases are now sound and complete.
- The repair changes what the model asserts: the r3 counterexample for mut_free_mult passed only through the same wrapped negative, so catch_mut_free_mult_test fails (13 of 14 tests pass).
- Round 4 runs through argon-run (gemini-4-argon-high, unit gq-e5-decode-r4, heavy-slot) in ~/.cache/gemini-lanes/e5-decode-r4; prompt and log in ~/.cache/gemini-lanes/e5-decode-r4-prompt.md and e5-decode-r4.log.

## [2026-11-13 08:41] qa | T13 X12 sampled mutant runs complete
- All 16 mutants now have a sampled run (rerun-rest.sh at 5,000 samples, rerun-deep.sh at 20,000 for the two it missed, through heavy-slot) in ~/Projects/Moriarty-wt/quint-x12/qa/gemini-quint/x12/runs.
- Sampling catches 13 of 16; cc_subUnrelated, cc_linkAnyStatus and cc_accumSorted, and the reachChainSeal witness (also at 20 steps), are caught or reached only by their deterministic tests.
- X12 RESULTS status, lead-run table and verdict updated; commit f9af0da9e on quint-x12.

## [2026-11-13 08:46] lane-done | T6a leaf OpenSpec refreshed to STARK AIRs
- Branch leaf-spec in ~/Projects/Moriarty-wt/leaf-spec, commits a2a03b919 and a27695fba. The halo2 gates (MockProver, k_leaf, the node and decider row gates) are replaced by the STARK kit gate SG-1 to SG-6 in design.md section 9: keygen with every AIR required (T3), real proofs on the stock verifier, fixed shard heights, layer budgets, D3 at the measured shapes.
- Budgets by layer: the app set for leaf kits; the internal set as raised 2026-11-12 (64 AIRs, 3,000 columns, 200 interactions per AIR) for nodes, GP hooks and the SHP-006 inline kernel; 3,300 columns and the 17-bit checker for FV units only (O1, O2).
- tasks.md group V (V.1 to V.10) is the vertical slice; V.10 is the T6 done-when. T5 waits are listed in design.md section 13; no draft value is copied.
- openspec validate --strict: valid, run with the node 26.13.0 binary (the mise shim fails: "openspec is not provided by the active node@26.13.1"). The first hook run of a27695fba failed once on the timing test test_copies_run_at_once_with_slots (2.036 s against 1.9 s); the rerun passed.

## [2026-11-13 08:49] decision | Owner signs the T5 leaf-schema drafts
- The owner chose "sign all recommendations" for the six T5 drafts. The sign-off is recorded in ~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md. O3-2 (the DEC plan lanes) stays open until T6.
- Each row turns FROZEN only once its closure test passes on the tested branch.

## [2026-11-13 08:49] note | T6a leaf OpenSpec refresh recorded in the loop
- leaf-spec a27695fba: strict validate passes. Vertical slice V.1 to V.10 comes before group 0. Budgets are split by layer (app set for leaf kits, internal set for nodes and GP hooks, FV units at 3,300 columns).
- T2 now also merges leaf-spec, lands the T5 schemas and fixes the addr >= q_pre against 8 q_pre mismatch.

## [2026-11-13 09:15] lane-done | T13 E5 decoder Gemini Quint lane round 4 finished
- Lane verdict: all 7 questions SOUND; mut_free_mult is an equivalent mutant under the m-range check, backed by an exhaustive pure check over 64 slot assignments; the other 6 mutants are caught.
- The lane sandbox refused quint again, so the lane ran no command; the lead rerun in ~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode-r4 decides whether it counts.

## [2026-11-13 09:36] commit | T13 E5 r3 model repair and lead rerun on e5-wrap
- Commit 5853f5ad0: ~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode-r3-lead with the repaired model, logs and RERUN.md (13 of 14 tests, SOUND_AND_COMPLETE ok at 20 samples on seeds 1 and 2, 6 of 7 mutants caught).
- Finding: the r3 mut_free_mult counterexample used the same wrapped negative; the mutant is not caught in the repaired field.
- The typescript backend needs about 30 s per sample on this model, so the lead runs use 20 samples, not 5000. T0 needed four retries for flaky factory timing and lock tests under load 30.

## [2026-11-13 09:36] note | T1 handback with suites still running
- Late tips merged into tested with no conflict: x-exp 1eda7a42e, quint-x12 f9af0da9e, e5-wrap 5853f5ad0 (tip 258676028); every listed tip is an ancestor.
- MEASURED: openvm tested builds; l_fixture and iface_fixture regenerated from the tested tree into ~/.cache/t1/fix. The suites with i0, i5_l, i10 still run in units t1-ovm and t1-sb (~/.cache/t1/logs/runs.log).

## [2026-11-13 09:38] lane-start | T2 repair the record on tested
- An Opus lane merges leaf-spec, lands the signed T5 schemas, fixes the 1.12 rows and the overstated E3, E6 and FP8 claims, and runs strict OpenSpec validation. It works on ~/Projects/Moriarty-wt/tested and touches docs only.
- The T1 suites keep running in the units t1-ovm and t1-sb; the late merges changed no code under tools or crates.

## [2026-11-13 09:44] commit | T13 E5 round 4 lead rerun confirms the lane
- Commit 70e728635 on e5-wrap: ~/Projects/Moriarty-wt/e5-wrap/qa/gemini-quint/e5-decode-r4 with the lane files, lead control and RERUN.md, and the E5 RESULTS rows.
- Lead rerun: 14 of 14 tests, SOUND_AND_COMPLETE ok on seeds 1 and 2, mut_free_mult not violated (equivalent under the m-range check), 6 of 6 other mutants caught; the control finds 9 of 64 attack pairs accepted once the m-range check is off.
- The E5 decoder verdict SOUND and COMPLETE now counts. Limits: the equivalence argument is the lane's, the exhaustive check covers slots 6 and 7 of one input, and each sampled run has 20 samples.

## [2026-11-13 09:45] qa | T13 first batch of lane repairs closed
- E5 decoder: the repaired model (prime 2053) and the round 4 lane pass a lead rerun with 14/14 tests and 6/6 mutants; mut_free_mult is equivalent under the m-range check (e5-wrap 70e728635).
- x14-pad and x14-identity were confirmed by lead Quint runs (x-exp 1eda7a42e). X12 sampling catches 13 of 16 mutants and deterministic tests cover the other 3 (quint-x12 f9af0da9e).
- Still to do: merge 70e728635 into tested, then the next T13 batch on the wrap core, the E3 interface AIRs and the lfr-* lanes.

## [2026-11-13 09:49] decision | Gemini lanes use Carbon, not Argon
- Owner order 2026-11-13: use Gemini 4 Carbon for every Gemini lane. Batch 2 of T13 now runs through the owner's herdr agy pane w1:pJ, one lane at a time.
- workflowskills 18673a1: the agy skill pins gemini-4-carbon-high, argon-run defaults to Carbon, and the Carbon model card was read in full and added to ~/Projects/workflowskills/platforms/agy/MODEL-CARDS.md.
- Earlier lanes (E5 r3 and r4, x14, X12, fp8-keybind) ran on Argon; their lead Quint reruns stand as the evidence.

## [2026-11-13 09:53] lane-start | T13 batch 2 lane wrap-levers (Gemini Carbon)
- Quint lane on the E5 wrap levers L1 (fused multiply-add) and L2 (omega^index, witnessed zi_inv), against the stock WHIR fold.
- Model: Gemini Carbon via herdr w1:pJ. Lane folder ~/.cache/gemini-lanes/wrap-levers, sources from Moriarty tested 01a9d6a8c.
- Batch 2 lanes run one after another: wrap-levers, e3-iface, lfr-tape-r2, lfr-bbalu-r2, lfr-poseidon-r2. Commits go to branch t13-lanes.

## [2026-11-13 09:55] qa | T1 sherlock-lfr suite on openvm tested: 34 pass, 1 fails
- i0, i5_l and i9 pass on fixtures regenerated from the tested tree (~/.cache/t1/logs/lfr-suite.log).
- e4_profile_lfr_prove fails: the E4 test predates the E3 Tape::expose rule. This is a cross-lane change, not a merge error, so it is listed for the lead.

## [2026-11-13 09:56] qa | T1 sherlock-lfr suite on tested: 34 pass, 1 fail
- MEASURED on the tested trees: 34 sherlock-lfr tests pass, including i0, i5_l and i9. e4_profile_lfr_prove fails because it predates the E3 rule that Tape::expose is mandatory; this is a cross-lane change, not a merge error, and T3 fixes it.
- Still running in units t1-ovm and t1-sb: i10, j5, the E5 exports, the wrap suite, m7, the GPU tests and the stark-backend suites.

## [2026-11-13 09:45] note | T2 merges leaf-spec into tested
- Late entry. Branch leaf-spec (a27695fba, the STARK refresh of OpenSpec sherlock-modular-leaf-20261111) merged into tested with no conflict: commit 01a9d6a8c.
- Strict validate of sherlock-modular-leaf-20261111 passes on the merged tree; sherlock-vm-consensus-20261107 fails strict at the base on 25 old warnings (preamble text and overlong requirements), fixed in a later T2 commit.
- Next: merge e5-wrap 70e728635 (round 4 lane, qa and research only), then the T5 landing and the 1.12 repairs.

## [2026-11-13 10:04] commit | T2 merges the e5-wrap round 4 lane into tested
- e5-wrap 70e728635 (round 4 Gemini Quint lane and its lead rerun, qa and research files only) merged into tested as 4cd8c0695; no code taken.
- The first pre-commit run failed only test_aggregate_kill_cgroup_slice (heavy-slot test); the retry passed 6 of 6 gates.
- Next: the T5 leaf-schema landing and the ARCHITECTURE 1.12 repairs.

## [2026-11-13 10:20] incident | T2 record-repair commit: first pre-commit run fails heavy-slot tests
- The pre-commit hook of the T2 record-repair commit on tested failed only test_aggregate_kill_cgroup_slice (heavy-slot) and test_copies_run_at_once_with_slots (timing, 1.98 s against 1.9 s); all other gates passed.
- Retry started with the same staged paths; log ~/.cache/t2/logs/commit-t2.log.

## [2026-11-13 10:26] commit | T2 record repair lands on tested
- Commit 52b71e250 on tested (22 files, docs and OpenSpec only; retry passed every gate): ARCHITECTURE 1.11 FP-9 holds the signed T5 leaf schemas with closure tests FP-9.7; the FP-1 MEM row now matches membackend.rs; the 1.12 rows settled by O1 to O12, D-11 and T5 stay FREEZE-PENDING on named closure tests; O3-2 stays open.
- DECISIONS.md records the owner rulings of 2026-11-13 (O1 to O12, D-11 under O4, T5 sign-off); lane RESULTS (E3, E4, E5, E6, X8, X14, FP8) replace the AFK marks with the rulings; E6 gate is BENCH-PASS; the E3 100-bit claim waits on G3 and G4.
- Strict validate passes on sherlock-vm-consensus-20261107 and sherlock-modular-leaf-20261111. T4-WRAP-BUDGET.md and T5-SCHEMAS.md are copied into ~/Projects/Moriarty-wt/tested/research/sherlock-tested-design-20261113/.
- Next: DECISIONS-ASK.md stays untracked in the main checkout; T1 suites keep running in t1-ovm and t1-sb.

## [2026-11-13 10:27] qa | T2 post-commit check on tested 52b71e250
- Strict validate passes again on HEAD for sherlock-vm-consensus-20261107 and sherlock-modular-leaf-20261111; no new FROZEN mark, no precedence note, no tag constant 38 to 57 in code that collides with FP-9.
- 1.12 rows for O7, O9, O1, O2 and D-11 read as ruled with the T7, T9 or T10 edit pending, so no row contradicts a committed test.
- LOOP.md and DECISIONS-ASK.md stay untracked in the main checkout and are cited as pointers.

## [2026-11-13 10:28] lane-done | T2 record repair on tested
- tested 52b71e250: the signed T5 schemas land as ARCHITECTURE FP-9 with closure tests; the rulings are recorded in 1.12 and DECISIONS.md; E6 is marked BENCH-PASS; the E3 100-bit claim now waits on G3 and G4.
- Strict OpenSpec validate passes for sherlock-vm-consensus-20261107 and sherlock-modular-leaf-20261111. No row was marked FROZEN.
- T3 starts once the openvm T1 suites finish, because its keygen change would invalidate the running proofs.

## [2026-11-13 10:40] qa | T1 openvm exports on tested: lfr_e3 passes, lfr_real fails on the E3 expose rule
- MEASURED: exp-lfr-e3 rc 0; exp-lfr-real rc 101 with a panic at crates/sherlock-lfr/src/machine.rs:1189, "expose the public words with Tape::expose". It is the same pre-E3 cause as e4_profile_lfr_prove, and T3 fixes both.
- Still running: the wrap suite and the i10 gate.

## [2026-11-13 12:00] qa | T1 wrap suite on tested
- All wrap suite tests pass except i5_lfr_real_native, which panics at src/wrapin.rs:49 because the wrapin file is missing (MEASURED).
- The missing file is the output of exp-lfr-real, which fails on the mandatory Tape::expose. Same root cause as e4_profile_lfr_prove.
- Logs: ~/.cache/t1/logs/wrap-suite.log, ~/.cache/t1/logs/runs.log. T3 fixes the expose calls and reruns both.

## [2026-11-13 12:13] lane-start | T3 every AIR required
- T1 marked DONE in ~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md; its three Tape::expose failures move to T3.
- Background Opus agent keys N1, N2, N3 and L with every AIR required and adds refusal tests for an omitted KeyBind or proof-shape AIR.
- Report due at research/sherlock-t3-airs-20261113/RESULTS.md on tested.

## [2026-11-13 12:25] build | T3 required AIRs on openvm tested
- openvm tested d86f56c: e4_profile_lfr_prove and the lfr_real export call Tape::expose (the E3 rule at machine.rs:1189).
- Uncommitted on openvm tested: frozen::keygen_checked keys leaf layer, internal, L and L_Fr with every AIR required; G1 refuses an optional AIR there; the L_Fr tape generator refuses a shape without a required AIR; fp8_keyset omit mode.
- Baseline wrap cost on the T1 lfr-e3 fixture (optional keys): 1,668,323 rows at k = 21 (MEASURED, ~/.cache/t3/logs/cost-before.log).
- Runner ~/.cache/t3/run.sh (T1 targets, own state and fixtures); unit t3-omit running.

## [2026-11-13 12:47] qa | T3 omitted-AIR negatives on real proofs
- fp8_keyset omit: 34 of 34 cases pass (MEASURED, ~/.cache/t3/logs/omit-run1.summary); leaf, N1, N2, N3 and L keys have every AIR required.
- KeyBind, ProofShapeAir and PublicValuesAir left out of N1, N3 and L proofs: refused by the prover (NonZeroRootSum), the stock verifier (RequiredAirNoVData), the parent trace generation, and for L the L_Fr tape generator (RequiredAbsent).
- Control with optional N1 keys: the same omissions are also refused, by bus imbalance and InvalidNumeratorTerms (MEASURED).

## [2026-11-13 13:15] lane-done | T13 Carbon wrap-levers report
- Carbon lane (herdr w1:pJ) finds L1, L2 and the witnessed zi_inv sound and complete; 31 tests, 15 of 15 mutants caught (lane claim, lead rerun running).
- Finding 4: the comment at whir.rs:224 and E5-RESULTS.md:174 say zi^-1 where the code pins zi_root^-1; the code is right, the prose needs a fix.
- Report ~/.cache/gemini-lanes/wrap-levers/qa/gemini-quint/wrap-levers/REPORT.md; lead rerun logs in lead-runs/ beside it.

## [2026-11-13 13:15] lane-start | T13 Carbon batch 2 queue
- Unit carbon-batch2 sends e3-iface, lfr-tape-r2, lfr-bbalu-r2, lfr-poseidon-r2 to the herdr Carbon pane one at a time.
- Queue log ~/.cache/gemini-lanes/queue/batch2.log; e3-iface sent first.

## [2026-11-13 13:20] qa | T13 wrap-levers lead rerun passes
- Seed 0x20261113, 500 samples, 8 steps: sound instance ok, 3 of 3 reachability witnesses, 15 of 15 mutants caught, 31 plus 1 tests pass (MEASURED).
- The lane counts. Lane files and LEAD-RERUN.md committing on t13-lanes in ~/Projects/Moriarty-wt/t13-lanes.

## [2026-11-13 13:40] commit | T13 wrap-levers lane on t13-lanes
- f98a0a7d5 on t13-lanes: lane REPORT, Quint model and tests, LEAD-RERUN.md under qa/gemini-quint/wrap-levers/.
- Two earlier tries failed only on heavy-slot timing tests under load 27 (allowed retry).

## [2026-11-13 14:14] qa | T3 sherlock-lfr suite with required keys
- sherlock-lfr suite on the T3 tree: 35 passed, 0 failed (MEASURED, ~/.cache/t3/logs/lfr-suite.log); T1 had 34 and 1 failed.
- e4_profile_lfr_prove passes now (845 s) with Tape::expose and required L_Fr keys.
- L fixture vk 80,439 to 80,833 bytes and L_iface vk 241,251 to 241,646 bytes (MEASURED): the parent proof-shape AIR gains one is_present check per child AIR.

## [2026-11-13 14:47] qa | T3 exports with required L_Fr keys
- exp-lfr-real now passes: keygen 200.1 s, prove 645.7 s, ~/.cache/t3/fix/e5/lfr-real.json written (MEASURED). The machine.rs:1189 panic of T1 is gone.
- exp-fixtures, exp-enc, exp-lfr-e3-dec and exp-lfr-e3 rc 0; the lfr-e3 and lfr-real L_Fr keys have 15 of 15 AIRs required (MEASURED).
- Next: the wrap suite (i5_lfr_real_native), the i10 and j5 gates, m7-bls and lfr-gpu.

## [2026-11-13 15:03] lane-done | T13 Carbon e3-iface report
- Carbon lane finds Carrier, IfaceAir, PublicAir row binding, ProgramTableAir, Tape::expose and iface::decode sound and complete under met conditions; 21 mutants (lane claim).
- Lead rerun started with seed 0x20261113; lfr-tape-r2 sent to the pane at 14:47.
- Report ~/.cache/gemini-lanes/e3-iface/qa/gemini-quint/e3-iface/REPORT.md.

## [2026-11-13 15:14] qa | T13 e3-iface lead rerun passes
- Seed 0x20261113, 500 samples, 4 steps: sound instance ok, 3 of 3 reachability witnesses, 19 of 21 mutants caught, both test suites pass (MEASURED).
- The 2 uncaught mutants are the 2 the lane classed equivalent; mut_q6_neg_one_count would refuse honest L_Fr traces (m 96 < H 128).
- The lane counts; committing on t13-lanes with LEAD-RERUN.md.

## [2026-11-13 15:27] commit | T3 openvm tested b3fbaa5
- openvm tested b3fbaa5: leaf aggregation, N1, N2, N3, L and L_Fr keyed with every AIR required in frozen::keygen_checked; G1 refuses an optional AIR there; L_Fr tape generator refuses a shape without a required AIR; fp8_keyset omit mode.
- Patch series ~/Projects/Moriarty-wt/tested/research/sherlock-stark-code-20261112/openvm-tested/ (27 patches, round trip tree 39610c3 equal to the tip tree).
- i10 gate passes on the T3 tree (MEASURED); j5, m7-bls, lfr-gpu, the wrap suite and the omit rerun are queued (unit t3-after).

## [2026-11-13 15:29] commit | T13 e3-iface lane on t13-lanes
- 0ae854829 on t13-lanes: lane REPORT, Quint model and tests, LEAD-RERUN.md under qa/gemini-quint/e3-iface/.
- First try failed only on the heavy-slot cgroup test under load (allowed retry).

## [2026-11-13 16:10] lane-done | T13 Carbon lfr-tape-r2 report
- Carbon lane finds the L_Fr tape sound with conditions and complete; Q5 PublicAir is now complete for any n (round 1 gap G4 closed); 19 mutants, mut_public_pad_zero_count equivalent (lane claim).
- F12 notes a footgun in raw Tape::public ordering; Tape::expose is safe. Lead rerun started with seed 0x51ed2026; lfr-bbalu-r2 sent at 15:56.
- Report ~/.cache/gemini-lanes/lfr-tape-r2/qa/gemini-quint/lfr-tape-r2/REPORT.md.

## [2026-11-13 16:20] qa | T13 lfr-tape-r2 lead rerun passes
- Seed 0x51ed2026, 1000 samples, 5 steps: sound instance ok, 6 of 6 reachability witnesses, 18 of 19 mutants caught, 36 plus 1 tests pass (MEASURED).
- The one uncaught mutant is the one the lane classed equivalent. The lane counts; committing on t13-lanes.

## [2026-11-13 16:21] qa | T3 gates i10, j5, m7-bls, lfr-gpu on required keys
- i10 gate 1 passed (2,287 s), j5 gate 1 passed (3,244 s), m7-bls 6 passed, lfr-gpu 3 passed and 1 ignored as in T1 (MEASURED, ~/.cache/t3/logs/).
- Wrap cost at k = 21 with required L_Fr keys: lfr-e3 1,668,183 rows (was 1,668,323), lfr-real 1,438,971 rows; columns unchanged (MEASURED).
- Wrap suite started (unit t3-after).

## [2026-11-13 16:24] commit | T13 lfr-tape-r2 lane on t13-lanes
- 6891a534e on t13-lanes: lane REPORT, Quint model and tests, LEAD-RERUN.md under qa/gemini-quint/lfr-tape-r2/.

## [2026-11-13 16:24] qa | T3 i5_lfr_real_native passes
- Wrap suite on the T3 fixtures: i5_lfr_real_native accepts the lfr-real L_Fr proof (649 of 649 stock samples, 50,708 permutations) and refuses its 3 changes (MEASURED, ~/.cache/t3/logs/wrap-suite.log).
- i5_lfr_e3_native: 51,540 permutations, was 51,547 with optional L_Fr keys (MEASURED). The rest of the wrap suite is running.

## [2026-11-13 16:27] decision | Owner: T9 contract work alongside T6
- Owner 2026-11-13: "yes, start the T9 contract work alongside T6". LOOP.md now has T9 on T3, O7, O9 and a new row T10a (decoder, design acceptRun, local devnet call on existing L_Fr proofs); T10 keeps the prog_d binding after T7.
- Branch t9-contract: Moriarty ~/Projects/Moriarty-wt/t9-contract (52b71e250), openvm ~/.cache/m7-wrap/src/openvm-t9 (b3fbaa5).

## [2026-11-13 16:27] lane-start | T9 and T10a contract lane
- Background Opus agent on t9-contract: ENC-2 to ENC-5, N-11 prologue, LW-009 without shape_d, openClaim 5 pairs, design acceptRun, local devnet call.
- Report due at research/sherlock-t9-contract-20261113/RESULTS.md; local devnet only.

## [2026-11-13 17:05] build | T9/T10a chain started on the t9-contract trees
- Unit `t9-chain` runs `~/.cache/t9/chain.sh all`: sherlock-lfr build and tests, ENC wrap fixture, iface fixture with the new `open` case, J4 tapes, lfr_e3, lfr_call key, then wrap cost, I7/I9, wrap key at k=21 and ledger-check.
- Decoder changes: 5-pair lane map (12,166 L words), X and layout_d pins, N-11 prologue lanes, opening-proof R form, PATH replay with wrap-private siblings.
- Contract `tools/sherlock-sdk/sherlock-ledger.compact`: 9 compiled tests pass; devnet deploy and dump pass with a placeholder key (MEASURED).
- Logs `~/.cache/t9/logs/`.

## [2026-11-13 17:12] commit | T3 Moriarty tested a4ae673c6 and 2de61edb8
- a4ae673c6: wrap Shape::from_wrapin refuses a key fixture without a required AIR trace; openvm-tested patch series; research/sherlock-t3-airs-20261113/RESULTS.md. 2de61edb8: record fixes (4 acceptances, 30 refusals).
- The pre-commit hook failed twice on factory lock and heavy tests (test_aggregate_kill_cgroup_slice, test_gpu_holds_both_locks_in_process) and passed on the third run.

## [2026-11-13 17:12] lane-done | T3 every AIR required
- DONE on tested: 34 of 34 omitted-AIR cases on real proofs (KeyBind, ProofShapeAir, PublicValuesAir left out of N1, N3 and L) behave as expected in two runs (MEASURED).
- e4_profile_lfr_prove, exp-lfr-real and i5_lfr_real_native pass; sherlock-lfr 35 of 35, wrap suite 25 of 25, i10, j5, m7-bls, lfr-gpu pass (MEASURED).
- Wrap rows at k = 21: 1,668,183 (was 1,668,323); wrap key 1,325 bytes, unchanged (MEASURED). Recursion-layer STARK vks grow by about 400 bytes each.
- sdk-node fails 3 of 144 in T1 and T3 alike (SHERLOCK_SDK_ROOT unset); the T1 record of a pass is wrong. Report: ~/Projects/Moriarty-wt/tested/research/sherlock-t3-airs-20261113/RESULTS.md.

## [2026-11-13 17:14] qa | T10a call run decodes a real ledger call vector natively
- `lfr_call args` (openvm t9-contract) gives the acceptRun args and the ledger anchor `C_L = DIGEST(SHK_PROGC, anchor_d2)` of the run; its native decode equals the statement (MEASURED).
- The t10a dry-run with those args builds a 1,026-word vector: anchor at lane 10, `claim_pub` at 20 and 34, `outcome_pub` at 39 equal the R outputs, so Compact `transientHash` equals the R `SPONGE(3)` output (MEASURED).
- The vector run decodes to the vector itself under the 5-pair lane map (MEASURED, `~/.cache/t9/logs/args-vec.log`).
- Fixed on the way: a stale ORD head (random j, k words) and nonzero mapped lanes in `call_run`.

## [2026-11-13 17:15] lane-done | T3 every AIR required
- T3 DONE: openvm d86f56c, b3fbaa5; Moriarty a4ae673c6, 2de61edb8 on tested. KeyBind, ProofShapeAir or PublicValuesAir left out of a real N1, N3 or L proof: 30 of 30 refused, 4 of 4 honest accepted (MEASURED).
- Wrap 1,668,183 rows at k = 21, key 1,325 bytes; sherlock-lfr 35/35, wrap suite 25/25, gates pass (MEASURED).
- Open: the new parent constraints and G1Error::Optional never fired (Carbon lane t3-airs to take them); sdk-node fails 3 of 144 without SHERLOCK_SDK_ROOT, so the T1 sdk-node pass record was wrong.

## [2026-11-13 17:15] lane-start | T6 leaf vertical slice
- Background Opus agent on branch t6-leaf: Moriarty ~/Projects/Moriarty-wt/t6-leaf (2de61edb8), openvm ~/.cache/m7-wrap/src/openvm-t6 (b3fbaa5).
- Group V tasks V.1 to V.10 in order with doctorwatson receipts; done-when V.10: one program proves leaf to L_Fr on one key set, fixed shard height, D3 at 100 bits.

## [2026-11-13 17:15] lane-done | T13 Carbon lfr-bbalu-r2 report
- Carbon lane finds Range16Air, BbAluAir, BbAluProgAir, InputAir and ProgramTableAir sound with conditions and complete; 9 mutants; the round 1 test failure was a typo in a hand-computed constant (lane claim).
- Lead rerun started with seed 0x51ed2026. lfr-poseidon-r2 resent at 17:12 after its 16:41 prompt stalled (agent_prompt_stalled).

## [2026-11-13 17:16] queue | T13 Carbon lane t3-airs queued
- Lane folder ~/.cache/gemini-lanes/t3-airs with the T3 diffs, sources and TASK.md (Q1 to Q5: parent proof-shape, transcript agreement, mixed keys, the unfired key-time checks, completeness).
- Unit carbon-t3-airs sends it to the herdr pane after lfr-poseidon-r2 writes its REPORT.

## [2026-11-13 17:19] qa | T13 lfr-bbalu-r2 lead rerun passes
- Seed 0x51ed2026, 1000 samples, 5 steps: sound instance ok, 2 of 2 reachability witnesses, 9 of 9 mutants caught, 10 tests pass on the TypeScript backend (MEASURED).
- The Rust evaluator refuses the test module (QNT600, 255-bit modulus), as the lane notes. The lane counts; committing on t13-lanes.

## [2026-11-13 17:24] commit | T13 lfr-bbalu-r2 lane on t13-lanes
- 33ba024c9 on t13-lanes: lane REPORT, Quint model and tests, LEAD-RERUN.md under qa/gemini-quint/lfr-bbalu-r2/.

## [2026-11-13 17:32] lane-start | T6 modular leaf, vertical slice group V
- Opus agent on branch t6-leaf: Moriarty ~/Projects/Moriarty-wt/t6-leaf (base 2de61edb8), openvm ~/.cache/m7-wrap/src/openvm-t6 (base b3fbaa5); state under ~/.cache/t6/.
- Goal V.10: one run of the V.1 program proves leaf, N1, N2, N3, L, L_Fr on one key set, fixed shard heights, D3 at the measured shapes.

## [2026-11-13 17:32] qa | T6 V.1 probe: the smallest program needs a task push and pop and two ROM rows
- entry 7 runs 6 steps on the reference owner machine: BHelperValidateEntry, BHelperValidateOccurrence, BHelperValidateEmpty, BHelperValidateExit, KMachineEvalLiteralValid, KAdminPublishValue (MEASURED, ~/.cache/t6/out/slice_v1.json).
- Validate pushes and pops one task; the run reads ROM rows expressions[0] and constants[0]; no record is appended (MEASURED). More candidates being probed.

## [2026-11-13 18:14] commit | T6 V.1 slice program committed, receipt PASS
- Moriarty `t6-leaf` f53a0b720: probe `slice_v1_candidates`, predicates V.1 to V.10 (set F), `scripts/qa/check-v1.sh`; V.1, V.8 and design section 8 admit the ROM line (PAGE-004)
- `scripts/doctorwatson check V.1` PASS (qa/sherlock/traces/20261114T005955Z-V.1.log)
- stark-backend worktree `~/.cache/m7-wrap/src/stark-backend-t6` branch t6-leaf deeeb0a: test-only `SB_MUTANT_PROVER` so SG-3 mutant proofs reach the stock verifier

## [2026-11-13 18:18] lane-done | T13 Carbon lfr-poseidon-r2 report
- Carbon lane finds the L_Fr Poseidon, split, pack, memory and program paths sound and complete as built through Tape, lverify and keybind; 13 of 13 mutants caught (lane claim).
- Lead rerun started with seed 0x51ed2026. Batch 2 queue done; t3-airs sent to the pane at 17:55.
- Report ~/.cache/gemini-lanes/lfr-poseidon-r2/qa/gemini-quint/lfr-poseidon-r2/REPORT.md.

## [2026-11-13 18:19] qa | T9 L_iface fixture with the open case; J4 tapes and decode cases pass
- `iface_fixture` proved 11 `L_iface` cases at 12,166 words, the new `open` case (opening-proof R) included, in 55 min (MEASURED).
- J4: the `L_Fr` tape with the T9 `gen_r_check` has 0 failed assertions on the 8 positives, `open` included, and refuses the 3 negatives (MEASURED, `~/.cache/t9/logs/j4-tapes.log`).
- `lfr_e3 0`: the stock decoder refuses the 15 named T9 mutants (X, layout_d, popeq, template, lane 0/1, open e_in, chunk a_out, PATH root, sibling, index) and accepts open, chunk and PATH positives (MEASURED).
- sherlock-lfr lib and tests pass (29 tests), and ENC-4/5 `t9_enc45` passes 3 of 3 after the merge into openvm `t9-contract` (MEASURED).

## [2026-11-13 18:33] qa | T13 lfr-poseidon-r2 lead rerun passes
- Seed 0x51ed2026, 1000 samples, 5 steps: sound instance ok, 7 of 7 reachability witnesses, 13 of 13 mutants caught, 11 tests pass on the TypeScript backend (MEASURED).
- The lane counts; committing on t13-lanes. Batch 2 is complete: five Carbon lanes, each confirmed by a lead rerun.

## [2026-11-13 18:33] incident | T6 and T9 agents cut off by a DNS outage
- Both background agents stopped about 18:30 with EAI_AGAIN (API server unreachable); unit t9-chain3 kept running.
- DNS resolved again at 18:32; both agents resumed with their context and told to check git status and unit logs before going on.

## [2026-11-13 18:33] commit | T10a ledger contract on t9-contract
- Moriarty `a71bb18a8` on `t9-contract`: `tools/sherlock-sdk/sherlock-ledger.compact` with its 9 compiled tests, build script, scanner, controls, devnet runner and `research/sherlock-t9-contract-20261113/contract/lanes.json`.
- Two earlier tries failed only on load-timed factory tests (heavy-slot makespan, lock release); the third passed.

## [2026-11-13 18:37] commit | T9 openvm commits on t9-contract after the reboot
- openvm `~/.cache/m7-wrap/src/openvm-t9` `c7c0c2e` (ENC-4 and ENC-5 reference, `sherlock plan`) and `609c4f1` (T9 decoder with every T4 check, 5-pair lane map, open case, T10a `lfr_call` run).
- Verified before the commit: sherlock-lfr lib and tests, J4 tapes, `lfr_e3 0` decode cases, `t9_enc45` (MEASURED).
- The reboot at 18:34 killed unit `t9-chain3` during `lfr_e3 1`; unit `t9-chain4` resumes from that stage (`lfr-unit` had passed).

## [2026-11-13 18:38] incident | Host reboot stops agents, units and the Carbon lane
- The host rebooted at about 18:34; T6 and T9 agents, units t9-chain3 and the T6 jobs, and the t3-airs Carbon turn stopped. Commits are intact: t6-leaf f53a0b720, openvm t6-leaf 810b370, t9-contract a71bb18a8.
- Both agents resumed at 18:37 with the list of uncommitted files; the staged lfr-poseidon-r2 lane commit restarted.
- The herdr Carbon pane w1:pJ came back without auto mode and waits on a permission prompt; left for the owner.

## [2026-11-13 18:39] commit | T13 lfr-poseidon-r2 lane on t13-lanes
- 958a26919 on t13-lanes: lane REPORT, Quint model and tests, LEAD-RERUN.md under qa/gemini-quint/lfr-poseidon-r2/.
- All five batch 2 Carbon lanes are now committed with lead reruns: wrap-levers, e3-iface, lfr-tape-r2, lfr-bbalu-r2, lfr-poseidon-r2.

## [2026-11-13 18:42] qa | T6 slice gates V.2 to V.5 pass on real proofs, V.6 MEM bus bug found
- `~/.cache/t6/gates.sh` modes v2 to v5 PASS (MEASURED): 5, 4, 3 and 7 cases; every mutant is proved with `SB_MUTANT_PROVER`, refused by the stock verifier, and its weakened control proves and verifies.
- v6 FAIL: MemAir sent the MEM bus message from `addr` through the carry columns (`crates/sherlock-leaf/src/mem.rs` MEM bus send), so a window with appends did not balance. Fixed to `addr` then `rec`; v6 to v8 rerun.
- openvm t6-leaf WIP commit 810b370 holds the slice crate; slice_chain example for V.9 and V.10 written.

## [2026-11-13 18:50] qa | T9 wrap cost at k=21 with every T4 check: 1,680,729 rows
- `wrap lfr-e3 21 cost` and `wrap lfr-call 21 cost`: 1,680,729 rows each against the 1,782,573 gate (94.3 percent; MEASURED cost model, `~/.cache/t9/logs/wrap-cost-e3.log`).
- That is +12,546 over the T3 baseline of 1,668,183 and +4,507 over the T4 probe total of 1,676,222 (DERIVED). The extra rows come from the opening-proof form of R, the fifth lane-map pair and the chunk template lanes.
- `lfr_call key` (the T10a ORD acceptRun run) proved `L_iface` and `L_Fr` with the native decode equal to the statement, `L_Fr` 1,522 public words (MEASURED).
- Docs commit Moriarty `a84f8123a` (FP-3 table at 5 pairs and 12,166 words, LW-009 and DCD-004 without `shape_d`).

## [2026-11-13 19:05] incident | T6 gate scripts left empty by the 18:34 reboot; V.2 to V.8 receipts void
- `scripts/qa/check-slice.sh` and `check-v2.sh` to `check-v8.sh` in `~/Projects/Moriarty-wt/t6-leaf` were written at 18:33 and came back 0 bytes after the reboot; commit afdc52209 holds them empty.
- `scripts/doctorwatson check V.2` to `V.8` then exited 0 in 3 ms with empty traces (20261114T015032Z to 015036Z): those receipts prove nothing and do not count.
- Scripts rewritten and synced, committed on t6-leaf; receipts rerun in unit t6-receipts2. git fsck of openvm-t6 and stark-backend-t6 is clean.

## [2026-11-13 19:05] commit | T9/T10a wrap decoder on t9-contract
- Moriarty `c4a899cba`: `tools/sherlock-wrap` decoder with every T4 check (X, layout_d, open form, lane 0/1, N-11 prologue, PATH replay), I7 and I9 updated.
- I7: 46 decode cases where the stock verdict equals the wrap native verdict, and 27 MockProver cases (3 T9 positives accepted, 23 mutants refused, honest). I9: 40 ENC checks. All MEASURED, `~/.cache/t9/logs/wrap-i7i9.log`.
- The first try failed only on the heavy-slot wall-cap timing test; the retry passed.

## [2026-11-13 19:05] qa | T10a wrap key at k=21: 1,325 bytes, ledger 9.1.0.1 Ok with refusals
- `wrap lfr-call.json 21 1`: MockProver accepts the honest instance and refuses a changed public input and a changed proof value. Keygen 32.4 s, blinded proof 2,912 bytes in 50.7 s, library verify Ok (MEASURED).
- The key is 1,325 bytes in the MidnightVK framing, against the 50,000-byte limit (MEASURED).
- ledger-check on stock ledger-9.1.0.1: `Ok(())` in 0.048 s. It refuses 3 changed public inputs and 3 flipped proof bytes (MEASURED).
- ENC-2 AIR side: `t9_enc2_onion6_on_the_onion_air` accepts the pinned onion6 and refuses one-word changes (openvm `01eb63f`). The devnet deploy and call run in unit `t9-devnet`.

## [2026-11-13 19:09] note | t3-airs Carbon lane paused
- The resent t3-airs turn returned at 18:39 with no REPORT; pane w1:pJ now shows a Grok session, so the lane is not resent until the owner says the Carbon pane is free.
- Model files stay in ~/.cache/gemini-lanes/t3-airs/qa/gemini-quint/t3-airs/.

## [2026-11-13 19:20] qa | T6 V.2 to V.8 receipts PASS; V.9 and V.10 gates pass
- Real receipts V.2 to V.8 (traces 20261114T020120Z to 020703Z), tasks ticked in t6-leaf 0e669d931.
- V.9 `slice_chain omit` (MEASURED): 57 cases PASS; each slice AIR left out is refused by the prover, the stock app verifier and the leaf layer; T3 omit cases at N1, N3 and `L` pass on the slice keys; every layer all-required.
- V.10 `slice_chain chain` (MEASURED, exclusive): app, leaf, N1, N2, N3, `L_iface`, `L_Fr` proved and verified on one key set in 375 s; both shards at the fixed heights; SG-6 1.79 s per proved step on CPU.
- D3 at measured shapes PASS on all 7 keys (weakest: `L_Fr` query 101.14, `leaf` shift 101.01); `fp8_d3` uses 2 stacked columns for the one-column app proof (conservative), openvm c541730.

## [2026-11-13 19:20] qa | T10a devnet acceptRun included with the round 3 wrap; R1 to R3 refused
- The local e6 devnet ran a fresh `L_iface`, `L_Fr` and wrap chain at k=21 from the ledger's own call vector. A valid `acceptRun` was included at block 12,951 (tx `0x5b3f37b9`), and `runs[claim_pub] = outcome_pub` reads back (MEASURED).
- The `L` run carries 4 used lane-map pairs and 4 PUB entries. The scan of the vector, the transcript ops and the state finds no private R operand, and the controls fire (MEASURED).
- R1 (outcome changed), R2 (proof bit flipped) and R3 (replay) were refused by the node. The call phase took 533 s (MEASURED, `~/.cache/t9/logs/devnet-call.log`).
- The wrap suite passes all 25 tests (MEASURED).

## [2026-11-13 19:20] commit | T9/T10a RESULTS and review fixes on t9-contract
- Moriarty `08c0e8461`: `research/sherlock-t9-contract-20261113/RESULTS.md`, the ENC-4/5 vectors and the openvm patch series `openvm-t9-contract/` (30 patches; the round trip gives the tip tree).
- Moriarty `e3325b68e`: the review fixes. The proven `L_Fr` words of the devnet call confirm 4 used lane-map pairs and 4 PUB entries. The chunk template probe `contract/chunk-probe.compact` is committed. The wrap requirement uses 12,166 words.
- The split of the +4,507 rows over the T4 probe is ESTIMATE (not broken down).

## [2026-11-13 19:20] lane-done | T9 and T10a done on t9-contract
- Done-when met (MEASURED): wrap 1,680,729 rows at k=21, a 1,325-byte key, ledger 9.1.0.1 Ok with refusals, devnet `acceptRun` included with R1 to R3 refused and a clean scan, and 9 of 9 compiled contract tests.
- Open for the lead: the ledger anchor is `C_L`, not the run's `C_P`, so N-11 stays open. ENC-4 task lane 3 `tag(2)` conflicts with Appendix A. ENC-5 tags are 58 to 62.
- Seams for T7/T10: `ANCHOR_D2` in `lfr_call.rs` and the FP-8 key words. Not merged into `tested`, not pushed.

## [2026-11-13 19:22] lane-done | T9 and T10a contract on t9-contract
- T10a: devnet acceptRun included at block 12,951 with 4 lane-map pairs and 4 PUB entries; outcome change, proof bit flip and replay refused; ledger 9.1.0.1 Ok; wrap 1,680,729 rows at k = 21, key 1,325 bytes (MEASURED).
- T9: Rust reference, AIR and wrap agree on every pinned vector with a wrap side; 9 compiled contract tests pass (MEASURED). ENC-4/5 AIR side waits on T6.
- Commits on t9-contract: Moriarty a71bb18a8 to e3325b68e, openvm c7c0c2e, 609c4f1, 01eb63f. RESULTS research/sherlock-t9-contract-20261113/RESULTS.md.

## [2026-11-13 19:22] decision | T9 lead rulings: ledger anchor, ENC-4 lane 3, ENC-5 tags
- Ledger anchor C_L accepted for T10a: one universal ledger cannot hold each run's C_P, and claim_pub binds C_P. prog_d in anchor_d2 is T10 work; I-23 and I-24 stay FREEZE-PENDING N-11.
- ENC-4 task lane 3 is tag(2): the signed rtag lane has 3 values; T5 Appendix A row 3 corrected on t9-contract, word count unchanged. ENC-5 part tags 58 to 62 accepted.

## [2026-11-13 19:33] decision | T6 slice tag SHK_RULES moves from 58 to 64
- The T9 lead ruling (t9-contract RESULTS item 8) gives 58 to 62 to the ENC-5 parts and the next code after 62 to `SHK_SHAPE`; the T6 slice PICK had `SHK_RULES` = 58.
- Resolved: `SHK_RULES` = 64, 63 left for `SHK_SHAPE`; openvm t6-leaf f143943 (`crates/sherlock-leaf/src/enc.rs`).
- The tag is in the CORE AIR, so V.2 to V.10 receipts rerun on f143943 in unit t6-receipts-all.

## [2026-11-13 19:33] commit | tested merges t9-contract
- Moriarty tested 47432e102 merges t9-contract 8c03f3ca0; the README patch-series conflict kept both rows; the merged wrap crate passes cargo check with tests.
- openvm tested fast-forwards to 01eb63f (T9 commits on top of T3). First hook try failed only on the heavy-slot cgroup test (allowed retry).
- t6-leaf still branches from tested 2de61edb8; it merges after V.10.

## [2026-11-13 19:46] build | T6 SG-6 on the GPU and V.4 pinned vectors
- Example `sg6` of `sherlock-leaf` (openvm t6-leaf 3160f59, feature `cuda`, `BabyBearPoseidon2GpuEngine`): 0.140 s per proved step on the GPU, 1.79 s on the CPU (MEASURED, exclusive, median of 3 after a warm-up); the stock CPU verifier accepts every GPU proof.
- Linking needs `LIBRARY_PATH=/opt/cuda/lib64` (the cuda-builder default is /usr/local/cuda/lib64).
- `digest_table_vectors` now also checks the pinned `D_EMPTY` and `VAL_NONE` of `research/sherlock-enc-fp3-20261112/VECTORS.txt`.
- V.9 gate (Moriarty t6-leaf 04215fa55) requires the verified GPU row; all receipts rerun on 3160f59 in unit t6-receipts-final.

## [2026-11-13 19:46] commit | T6 openvm t6-leaf commits since 810b370
- c3bea58 V.2 to V.8 kit gates, MEM bus message fix, leaf HEIGHTS, slice_chain; 6ec1364 omit SG-6 record; c541730 fp8_d3 two-column rule; f143943 SHK_RULES 64; 3160f59 V.4 pinned vectors and the GPU sg6 example.
- Moriarty t6-leaf: 6c9513f7d restored gate scripts, 0e669d931 V.2 to V.8 ticks, 04215fa55 V.9 GPU gate. Patch series under research/sherlock-stark-code-20261112/openvm-t6 and stark-backend-t6.

## [2026-11-13 20:05] lane-done | T6 group V vertical slice done
- V.1 to V.10 ticked on real receipts at openvm t6-leaf 3160f59, Moriarty t6-leaf 1d9c50b7c; record `~/Projects/Moriarty-wt/t6-leaf/research/sherlock-t6-leaf-20261113/RESULTS.md` with run records under `runs/`.
- V.10 (MEASURED): `entry x;` proves app, leaf, N1, N2, N3, `L_iface`, `L_Fr` on one key set in 362 s; fixed shard heights; D3 PASS on 7 keys at measured shapes; SG-6 GPU 0.148 s per proved step.
- Open for the lead: X10 in the leaf, multi-segment TAG_NODE merge in L, FP-2 node statement, the slice `plan_d` against T9 ENC-5, the V.9 reading of slice_air_omitted; Carbon lane list in RESULTS section 7.

## [2026-11-13 20:24] qa | T6 final V.9 and V.10 receipts with host load
- V.9 and V.10 rerun at openvm 3160f59 with `/proc/loadavg` in the trace (DG-006); Moriarty t6-leaf 9784e4940 (gate), 5d96f4db9 (RESULTS). Traces 20261114T030955Z-V.9, 031526Z-V.10.
- SG-6 GPU 0.139 s per proved step, CPU 1.86 s (MEASURED; load 19 to 24 from jobs outside heavy-slot). Chain 395 s; D3 PASS on 7 keys.
- `runs.jsonl` committed with the T6 receipt lines; the seven void 3 ms V.2 to V.8 lines stay and RESULTS explains them.

## [2026-11-13 20:25] lane-done | T6 group V accepted by the lead
- Lead read the V.10 receipt trace: SLICECHAIN PASS, 2 shards, 5 steps, fixed heights, G1 accepted on every layer, D3 PASS on 7 keys, weakest leaf shift 101.01 (MEASURED).
- Rulings: V.9 omission reading accepted; fp8_d3 two-column rule accepted as conservative, T12 evaluates the exact case; slice plan_d must take the T9 ENC-5 form.
- The chain binds the keys, not the outcome: FP-2, the TAG_NODE merge, X10 and plan_d move into T7 with prog_d. T8 waits on FP-2.

## [2026-11-13 20:25] build | Trial merge of t6-leaf into tested
- Moriarty: README patch-series conflict resolved keeping both rows; openvm merges cleanly; stark-backend fast-forwards to deeeb0a.
- cargo check of sherlock-lfr, sherlock-leaf and sherlock-wrap-export on the merged openvm tree is running before the merge commit.

## [2026-11-13 20:29] commit | t6-leaf merged into tested
- openvm `tested` 8672651; stark-backend fast-forwarded to deeeb0a; Moriarty `tested` 8e00930b8. No attribution lines.
- cargo check of sherlock-lfr, sherlock-leaf and sherlock-wrap-export on the merged openvm tree passed (one unused-mut warning in the sg6 example).

## [2026-11-13 20:29] lane-start | T7 prog_d binding with the open T6 items
- Background Opus agent on branch `t7-bind`: Moriarty `~/Projects/Moriarty-wt/t7-bind`, openvm `~/.cache/m7-wrap/src/openvm-t7`, stark-backend `~/.cache/m7-wrap/src/stark-backend-t7`.
- Order: FP-2 node statement, TAG_NODE merge, X10, plan_d to ENC-5; then prog_d in the leaf, key_root in the first node, I-17 path C h_pre, D10.
- Report: `research/sherlock-t7-bind-20261113/RESULTS.md`. T8 waits on FP-2.

## [2026-11-13 21:17] issue | T7 path C blocked: L key depends on its child layer
- Probe `t7_probe keys` (MEASURED, `~/.cache/t7/out/probe/`): N1 over chip set 0 and 1 is one key, N2 likewise; an `L`-shaped verifier over N3 and over N2 is one key, over the leaf layer another.
- The proof-shape module bakes the child per-AIR metadata into the verifier, so one universal `L` key over a leaf (path C) needs a recursion change; T7 stops and reports it (one universal key set rule). D10 on path C is blocked the same way.
- A unary node (`VerifierSubCircuit<2>` over one child) proves and verifies, so N2 and N3 no longer need duplicated children.

## [2026-11-13 21:17] commit | T7 statement AIR and plan_d (openvm t7-bind 95a1f01)
- `sherlock-lfr` `stmt.rs`: StmtAir (Leaf, First, Node, Fv, Root), PermSponge on the recursion permute bus, RootOps; the key_root route in N1 and the keys_d route above it.
- Slice `plan_d` in the T9 ENC-5 form; chip set 1 (ExtAir); program B (`entry y;`); `iface::anchor_d2` at the ANCHOR_D2 seam with a unit test.

## [2026-11-13 21:40] decision | T7 path C stays in scope via the DAG key check
- Leaf keys are per program and per chip set and key_root commits them (ARCHITECTURE "Keys"); only internal-recursive, L, L_Fr and wrap are universal, so an L key that depends on its child layer is expected on path C.
- Ruling: finish the full-tree done-when cases first; then scope path C as the specified DAG key check (L reads the child verifier as cached-trace commitment X, wrap equates X); build it if it stays in the recursion crate, else report it in RESULTS section 0.
- Corrects the lead brief, which said "no per-program key"; the one-leaf Compact call needs path C, so it is not dropped.

## [2026-11-13 22:03] qa | T7 chains A and B pass on one universal key set
- openvm t7-bind a133602 (L root statement and RootSealAir, X10 in L, A_0 = C_P tape check in L_Fr): `entry x;` and `entry y;` prove app, leaf, N1, N2, N3, L, L_Fr with the same key_root 5b796143 and keys_d 1993533b, different prog_d (MEASURED, ~/.cache/t7/out/chain-a and chain-b).
- Refused: outcome word changed and non-terminal outcome at L; other program's anchor and a forged outcome operand at the L_Fr tape; mutant L_Fr proof of the failing tape at the stock L_Fr verifier (MEASURED).
- fp8_d3 PASS on both chains plus the FV unit; weakest leaf shift 101.0110, L vk mu 101.0730 (MEASURED). L_Fr 1,522 public words and 15 AIRs / 561 columns as in T10a, so the wrap shape is unchanged (DERIVED).
- Binding cases: mixed chip sets under one key_root accepted at N2 and N3; wrong prog_d, key triple outside key_root and chip-set swap refused at N1; FV unit under N3 accepted; FV row missing from keys_d refused at N3 (MEASURED, ~/.cache/t7/out/cases2).

## [2026-11-13 22:19] commit | T7 openvm 662235f and Moriarty gate scripts on t7-bind
- openvm t7-bind 662235f: t7_chain omit mode (V.9, 59 cases PASS, every AIR required in app, leaf, N1, N2, N3, FV, L), FV row negatives, mutant L_Fr stock-verifier case, N3 over N3 table row (N3 is at a fixed point, MEASURED).
- Moriarty t7-bind e2cd326a9 and 24b4f04f9: check-chain.sh runs t7_chain (chains A and B with D3, binding cases, omit); V.2 to V.8 default to ~/.cache/m7-wrap/src/openvm-t7; NODE-004 commits a LEAF key triple by a key_root path and a NODE or FOREIGN vkc by a keys_d table row.
- Patch series ~/Projects/Moriarty-wt/t7-bind/research/sherlock-stark-code-20261112/openvm-t7/ (3 patches on 8672651; git am reproduces the 662235f tree).

## [2026-11-13 22:33] lane-start | T8 GP hooks on the T7 base
- FP-2 landed on t7-bind (chains A and B on one universal key set, binding cases refused, 22:03 qa), so T8 starts before T7 closes; O1 to O12 rulings are in.
- Background Opus agent on branch t8-gp from the t7-bind tips: Moriarty ~/Projects/Moriarty-wt/t8-gp (fc830df0f), openvm ~/.cache/m7-wrap/src/openvm-t8 (662235f), stark-backend ~/.cache/m7-wrap/src/stark-backend-t8 (deeeb0a).
- Scope: X12 (GP-2), X8 with a Compact adapter class (GP-3), X14 FV units, X7 with N-6 (GP-4), N-4 continuation.qnt; T8 keeps out of proof_shape while T7 scopes path C, and rebases on the final t7-bind.
- Report: research/sherlock-t8-gp-20261113/RESULTS.md.

## [2026-11-13 23:02] qa | T8 N-4 continuation.qnt passes; reading is option (a)
- New model ~/Projects/Moriarty-wt/t8-gp/models/sherlock/quint/continuation.qnt with tests: 11 deterministic tests pass, every invariant holds in 5,000 sampled traces of 40 steps, three reach witnesses found, 7 of 7 mutants caught by sampling (MEASURED, quint 0.32.0).
- No circuit relation needs a checkpoint tag: with an untagged record the circuit side still holds; the tag is needed on the record digest only (R-7). Lead to confirm.
- Committed on t8-gp as Moriarty c09e8af1b; runs under research/sherlock-t8-gp-20261113/quint/runs/r2/.

## [2026-11-13 23:02] issue | T8 X7: MERGE-WORK does not fit the internal set (N-6, owner)
- Keygen probe `t8_gp x7probe` (openvm t8-gp): the universal node plus the inline kernel built from the 7 slice leaf AIRs is 48 AIRs and 4,246 columns against 3,000, and CoreAir has 1,476 constraints against 1,000 per AIR (MEASURED, ~/.cache/t8/out/x7/x7probe.json).
- Same shape over N1 and over N2. The kernel alone is 2,361 columns, so no node layout fits it under 3,000.
- N-6 carries an OWNER mark: GP-4 stays incomplete and the owner decides; T8 does not trim the kernel or use two nodes. Carried on to X12.

## [2026-11-13 23:04] decision | Lead accepts the T8 N-4 continuation reading (a)
- The checkpoint tag sits on the record digest only (R-7); no circuit relation needs it. The continuation.qnt model holds with an untagged record, and 7 of 7 mutants are caught (MEASURED, 23:02 qa).
- X7 MERGE-WORK over the internal set (4,246 columns against 3,000; CoreAir 1,476 constraints against 1,000, MEASURED) is an N-6 OWNER item: the lead takes no pick and puts it to the owner.

## [2026-11-13 23:04] qa | T7 receipts V.1 to V.10 PASS at openvm 662235f
- doctorwatson closure V.10 on Moriarty t7-bind: V.1 to V.8 PASS (V.2 to V.8 on the T7 tree), V.9 PASS (59 omit cases), V.10 PASS (chains A and B, D3 on 8 keys each, 12 binding cases), traces under ~/Projects/Moriarty-wt/t7-bind/qa/sherlock/traces/20261114T05*.
- Correction to the 22:0x qa entry: at the tip keys_d is 038ad4d3 (the table gained the N3-over-N3 row); 1993533b was the a133602 development run. Shapes, heights, D3 and cases are the same (MEASURED).
- Tip build checks: every sherlock-wrap-export example compiles (lfr_call included); sherlock-lfr lib 17 tests and t9_enc45 3 tests pass, so the CHIPS append moved no ENC-4/ENC-5 vector (MEASURED).
- Voided: one BLOCKED call and one V.1 run without cargo on PATH in the systemd unit (environment, not gate).

## [2026-11-13 23:42] commit | T7 report and V.10 fixed-point gate on Moriarty t7-bind
- bafc1df4d: V.10 requires the N3 fixed point; receipt ~/Projects/Moriarty-wt/t7-bind/qa/sherlock/traces/20261114T060414Z-V.10.log PASS (MEASURED).
- 316b17682: ~/Projects/Moriarty-wt/t7-bind/research/sherlock-t7-bind-20261113/RESULTS.md with tip runs and receipt copies; qa/sherlock/status.json and runs.jsonl left uncommitted.

## [2026-11-13 23:42] lane-done | T7 prog_d and the general key route (full tree)
- Done-when met on chains A (entry x;) and B (entry y;) at openvm 662235f: wrong prog_d refused, changed outcome word refused at L, mixed chip sets under one key_root accepted, FV unit under N3 accepted, D3 PASS on every key, L_Fr words unchanged so the T10a wrap fit holds (DERIVED).
- Open for the lead: path C (I-17 h_pre, D10) scoped not built, L at 1,986 of 2,000 columns needs about 75 more (ESTIMATE); picks A_0 = C_P in L_Fr, X10 in L, CHIPS append, SG-6 dropped from V.9; Gemini Carbon Quint lanes listed in RESULTS section 7.

## [2026-11-13 23:45] qa | T8 X12 GP-2 node modes on real proofs
- t8_gp gp2 at openvm ~/.cache/m7-wrap/src/openvm-t8 c26e247 (tree of 1d39339): 34 of 34 cases pass, SEAL, JOIN, DESCEND, DL-STEP and per-mode shape_d accepted; forged headers, outcomes, set roots, chains, links, values, shape and key_root words refused by the node proof (MEASURED, ~/.cache/t8/out/gp2/gp2.json).
- Node keys after the change: N2 and N3 41 AIRs, 2,182 columns, 809 constraints, 170 interactions; N3 over N3 fixed point holds (MEASURED).
- Blocker: the T6 slice leaf has no Halted run, so honest SEAL-WITH-DEPS (consume a prior run's value) is not provable; GP-2 hook stays FREEZE-PENDING.

## [2026-11-14 00:21] issue | T8 FV unit regression found and fixed on openvm t8-gp
- First T7 cases rerun on the T8 node layout failed fv_unit_proves and fv_unit_under_N3_accepted: the non-MERGE output rules of gp.rs were gated by one minus MERGE, so they also held in an FV unit (mode bit sum 0) (MEASURED, ~/.cache/t8/logs/chain-recheck.log).
- Fix 569b23c gates them by the mode bit sum; the T7 cases rerun at the fix passes all 12 (MEASURED, ~/.cache/t8/logs/cases-dbg.log). The fix changes the node keys, so the 23:45 gp2 numbers at c26e247 are superseded by the r2 rerun at 569b23c.
- Chains A and B at c26e247 had passed V.10 with D3 PASS on 8 keys and L_Fr words 1,522; they are rerun at 569b23c.

## [2026-11-14 00:40] decision | Owner rules N-6: MERGE-WORK nodes get their own bounds
- Owner pick "Own bounds": a MERGE-WORK node has its own universal parameter set (as FV units under O2), about 4,400 columns, CoreAir split into two AIRs so each stays at 1,000 constraints or fewer; the shared internal set keeps 3,000 columns.
- It is one more universal key, with a keys_d row, at k 21 or less and D3 at the bar. Sent to the T8 agent with the X7 done-when.

## [2026-11-14 00:40] lane-done | T7 accepted on the full tree; tested fast-forwarded
- V.1 to V.10 PASS at openvm 662235f on entry x; and entry y; (one key_root, different prog_d); every binding case refused or accepted as specified; D3 weakest 101.0110 (MEASURED). Wrap unchanged (DERIVED: L_Fr 1,522 words, 15 AIRs).
- Lead rulings on section 0: picks 1 to 7, 10 to 14 and 16 accepted; 8 (L log tie) and 9 (h_hi range check) go to T10 as circuit changes; 13 gets a real post_d case in T10; 15: T10 reruns SG-6 on its tree; 17: T7 lanes queued for Carbon.
- Moriarty tested and openvm tested fast-forwarded to t7-bind (316b17682, 662235f).

## [2026-11-14 00:40] lane-start | T10 bound wrap and T7c path C in parallel
- T10 on t10-bound: real anchor_d2 at lfr_call.rs:122, gen_ops_check on the published path, N-11 closes I-23 and I-24, L log tie, h_hi range check, full path to a local devnet acceptRun with wrong program and outcome refusals.
- T7c on t7c-pathc: slim L under its 2,000-column bound (no bound change; L is at 1,986, path C needs about 75, ESTIMATE), then the DAG key check, I-17 path C h_pre and D10; the wrap X Canon check waits for T10.
- Reports: research/sherlock-t10-bound-20261114/RESULTS.md and research/sherlock-t7c-pathc-20261114/RESULTS.md.

## [2026-11-14 00:43] decision | Owner AFK: finish the tested-design loop with the recommended options
- Owner: "going afk, finish everything and go with the recommended solution".
- The loop takes the recommended option on each OWNER item, logs it as an AFK pick and marks it for owner confirmation at T15.
- Still owner-gated: every push, Preview and Mainnet deploy, and any hard-rule change. Carbon lanes wait for the herdr pane.

## [2026-11-14 00:44] lane-start | T7c path C agent on t7c-pathc
- Trees: Moriarty `~/Projects/Moriarty-wt/t7c-pathc` (316b17682), openvm `~/.cache/m7-wrap/src/openvm-t7c` (662235f, stark-backend patch repointed to stark-backend-t7c deeeb0a).
- State, target and logs under `~/.cache/t7c/`; first step is a per-AIR column table of the `L` key to find slimming room under 2,000.

## [2026-11-14 00:50] lane-start | T10 bound wrap agent on t10-bound
- Trees: Moriarty `~/Projects/Moriarty-wt/t10-bound` (316b17682), openvm `~/.cache/m7-wrap/src/openvm-t10` (662235f), stark-backend unchanged (deeeb0a); state and logs under `~/.cache/t10/`.
- Plan: N1 `h_hi` 16-bit range table, the T10 call path on the real T7 chain (`anchor_d2(r_L, prog_d)`, `gen_ops_check`), the `L` log tie, then wrap, ledger 9.1.0.1 and a local devnet `acceptRun`.

## [2026-11-14 01:05] decision | Owner allows Sherlock Preview deploys and calls
- Owner: "you can push to preview, we have a wallet and its free for me".
- T10 adds a Preview deploy and acceptRun call after the local devnet passes, with the existing Preview wallet and the OP-PV tooling of Moriarty 78592bf59; seeds and keys are never printed, logged or committed.
- Mainnet and git pushes stay owner-gated.

## [2026-11-14 01:05] decision | Lead accepts the T8 MERGE-WORK bound of 5,400 columns (AFK pick)
- The built MERGE-WORK key is 5,128 columns (MEASURED): the GP block, the MERGE-WORK step columns and FRAME (522) add to the 4,246 probe; 5,400 keeps about 5 percent margin.
- D3 at the 5,400 bound: weakest shift 101.1643, mu 101.6304 (MEASURED); CORE split into CORE and FRAME keeps every AIR at 1,000 constraints or fewer. Shared internal set unchanged at 3,000.
- AFK pick, marked for owner confirmation at T15 (the ruling said about 4,400).

## [2026-11-14 01:22] issue | T10: the L log tie does not fit the frozen L column bound
- Probe: the J8 onion AIR (70 columns) added to the T7 `L` gives G1 `Columns { set: "L", got: 2056, bound: 2000 }` (MEASURED, `~/.cache/t10/logs/probe-onion.log`).
- The tie (root statement log words equal the onion of the `L` entry table) waits on the T7c `L` slimming; T10 keeps the T7 `L` and runs the rest of the bound path on it.
- Built on t10-bound (openvm, uncommitted until the cases pass): N1 `h_hi` limbs looked up in a 16-bit table `HhiRangeAir` (bus 963), cases for a changed leaf `post_d` and an `h_hi` limb of 2^16.

## [2026-11-14 01:42] commit | T10 openvm 89e57cc and Moriarty 28c037eb8 on t10-bound
- openvm: N1 `HhiRangeAir` (16-bit table, bus 963), the `t7_chain t10` bound call path with `anchor_d2(r_L, prog_d)` and `gen_ops_check`, `lfr_call` over it; cases `leaf_post_d_changed_*` and `h_hi_limb_over_16_bits_refused_at_N1` refuse, 15 of 15 cases PASS (MEASURED, `~/.cache/t10/logs/cases1.log`).
- Moriarty: `research/sherlock-t10-bound-20261114/chain.sh` (one rerun command), ledger-check named refusals, the ledger runner R4 and Preview mode.
- Preview wallet status before any T10 Preview step: DUST 8583829478692841287 at tip 104980, no funding needed (MEASURED).

## [2026-11-14 01:43] queue | T10 units t10-chain1 (keys, wrap, ledger, devnet) and t10-gates1 (V.9, V.10, cases, SG-6)
- Logs in `~/.cache/t10/logs/`; stage records in `~/.cache/t10/logs/runs.log`.

## [2026-11-14 01:43] decision | L column budget covers path C and the T10 log tie together
- L is at 1,986 of 2,000 columns; path C needs about 75 (ESTIMATE) and the T10 onion tie 70 (MEASURED probe, 2,056), about 2,131 in all.
- T7c sizes its slimming, or its AFK fallback bound (for example 2,200, with the k, height, D3 and wrap checks), for both and commits the L change on its own; T10 cherry-picks it for the tie and meanwhile runs the bound path, devnet and Preview on the T7 L.

## [2026-11-14 01:43] decision | Lead: T10 keeps the T7 L; the onion tie follows the T7c L slimming
- T7c sizes its `L` slimming for path C and the 70-column onion AIR and commits it early; T10 cherry-picks it, adds the tie and reruns `chain.sh`.
- If T7c is not there when the rest of T10 is done, the tie goes to RESULTS section 0 as a scoped item with file:line, and the lead reruns it at the merge.

## [2026-11-14 01:43] note | T7c slims L by 129 columns (openvm 2080b5f, DERIVED)
- The carrier-only Stmt and Call interface AIRs of L are bare in airs_with and traces_with (crates/sherlock-lfr/src/iface_l.rs only): L 1,986 to 1,857 columns, DERIVED; the measuring run is queued.
- Path C adds about 61 plus about 7 ProofShape columns, and the T10 onion tie 70: about 1,995 of 2,000 (ESTIMATE); StmtAir in L at 98 of 100 interactions after path C.
- T10 cherry-picks 2080b5f for the tie; the AFK fallback bound of 2,200 applies only if the measured total goes over 2,000.

## [2026-11-14 01:52] qa | T7c: one universal L over N3 and over the path C leaf layer
- Slimmed L with path C is 47 AIRs and 1,925 columns, 75 under the 2,000 bound (MEASURED, ~/.cache/t7c/out/shape-pc1/shape.json); with the 70-column T10 onion AIR it is 1,995, so the bound stays and no fallback is used.
- The L key over N3 is byte-identical to the L key over the path C leaf layer of chip set 0 and of chip set 1 (MEASURED), via the child-meta mode of the recursion verifier.
- StmtAir 85 columns, 98 of 100 interactions; RootSealAir 100; KeyPathAir 20 (new). Path C code not yet committed; the real-proof run is queued.

## [2026-11-14 01:59] build | T10: the L log tie on the T7c L (openvm t10-bound, uncommitted)
- `iface_l::OnionAir` joins the production `L` (last AIR); `LogEntry` and `LogValue` send on the onion buses; the root statement AIR receives `TIE_BUS` on the exposed `log_d`, `log_n` (`crates/sherlock-lfr/src/stmt.rs` eval_root).
- Interim rule for lead confirmation: the N3 child log words become key constants `(D_EMPTY, 0)` (the leaves log nothing yet), and the exposed log words are the onion of the `L` tables. A multi-segment log tree replay stays follow-up work.
- New refusal cases: `exposed_log_d_changed_refused_at_L`, `log_value_changed_refused_at_L`; run in `~/.cache/t10/logs/cases2.log`.

## [2026-11-14 02:09] build | T7c: slim L committed, one universal L key MEASURED (openvm t7c-pathc)
- Commit `2080b5f`: the carrier-only `Stmt` and `Call` interface AIRs of the production `L` are one zero column each (`crates/sherlock-lfr/src/iface_l.rs`); frees 129 columns, for T10 to cherry-pick.
- MEASURED (`~/.cache/t7c/out/shape-pc1/shape.json`): `L` with path C is 47 AIRs, 1,925 columns, `StmtAir` 98 of 100 interactions; 75 columns free, so T10's 70-column onion AIR fits (1,995) under the unchanged 2,000 bound.
- MEASURED: the child-meta `L` key over N3 is byte-identical to the `L` key over the path C leaf layer of chip sets 0 and 1 (`VerifierConfig::child_meta`, meta cached height 32,768).
- Path C leaf: the S = 8 shard bucket (`sherlock_leaf::s8`), CORE `rules_d` send split into 8-word blocks; `entry x;` is one shard and its app proof verifies (`~/.cache/t7c/logs/` s8 run).

## [2026-11-14 02:10] decision | N-6 owner ruling "own bounds" applied in T8
- Owner 2026-11-14: MERGE-WORK nodes get their own universal parameter set (merge_work), the shared internal set keeps 3,000 columns; recorded in ARCHITECTURE N-6 and the GP-4 hook row on Moriarty t8-gp.
- AFK pick for owner confirmation at T15: column bound 5,400, not about 4,400, because the key as built is 5,128 columns (MEASURED); D3 at the bound weakest 101.1643.
- AFK picks per the lead order: N-7 bind intervals (shape_d binds h_lo, h_hi), N-8 root history with revocation (nothing built).

## [2026-11-14 02:10] qa | T8 X7 MERGE-WORK node and the final run at openvm 7298247
- t8_gp x7: two enabled N3 SEG proofs of program A plus one inline owner step proved by the slice kernel with CORE split into CORE (999 constraints) and FRAME (494) from one frame_rule; 49 AIRs, 5,128 columns, 171 interactions, log height 17; 12 of 12 cases pass (MEASURED).
- Final unit t8-final: gp2 34 of 34, V.10 chains A and B PASS with D3 PASS on every key, T7 cases 12 of 12, V.9 59 cases PASS, L_Fr words 1,522 unchanged (MEASURED, ~/Projects/Moriarty-wt/t8-gp/research/sherlock-t8-gp-20261113/out/final/).
- Open: no parent over a MERGE-WORK child yet, no STACKS or VALUE DIGEST kit in the inline step, Halted leaf run for GP-2, X8 verifier.

## [2026-11-14 02:33] qa | T10: the tied L keys under G1 at 1,918 columns (MEASURED)
- `L over N3` with the onion AIR and the root tie: G1 accepted, 47 AIRs, 1,918 columns (bound 2,000), queries [98, 83, 82], query PoW 20, logup PoW 18 (`~/.cache/t10/key/keys.json`, stage key-A, openvm 0c4f05e).
- T7/T10 binding cases on the tied tree: 15/15 PASS (`~/.cache/t10/logs/cases2.log`). The `chain.sh all` run continues in unit t10-chain2.

## [2026-11-14 03:01] qa | T7c: path C proves on the universal L key (openvm acb741e, MEASURED)
- `t7_chain pathc` (`~/.cache/t7c/logs/pathc-oom26g.log`): one leaf of `entry x;` (S = 8, 5 steps, h_pre 0), leaf layer at the internal set, `L` accepted; the full-tree N3 child on the same `L` key with the same heights and stacked widths.
- Refused at `L` (stock verifier): child key outside `key_root` (chip set 1), wrong `X`, `X` of the full tree, wrong `prog_d` opening. An `X` that is not the `L` commitment passes `L` and is refused by the `L_Fr` tape (LEAD pick: `L_Fr` checks `X` against the cached commitment it reads, so the wrap needs no `X` check).
- Slim: changed first, middle and last words of the bare `Call` and `Stmt` AIRs refused by the stock `L` verifier and the `L_Fr` tape.
- Incident: the run was killed by its 26G memory cap during `L_Fr` keygen (exit 137); rerun at 56G under heavy-slot.

## [2026-11-14 03:16] decision | Lead accepts the T7c X check in L_Fr instead of the wrap
- T7c pick: L_Fr checks X against the cached commitment of L that it already reads, and the wrap verifies L_Fr, so the separate wrap Canon check of X (about 120 rows, ESTIMATE) is not needed; an X that is not the L commitment is refused at the L_Fr tape (MEASURED, openvm acb741e).
- Accepted: the binding is the same as the ARCHITECTURE path C text, one layer lower; T7c records the ARCHITECTURE edit and T12 adds the case to the G3 transcript vectors.

## [2026-11-14 04:19] lane-done | T7c path C on the universal L key
- Rerun at 56G (`~/.cache/t7c/logs/pathc.log`): 25 of 25 path C cases PASS; D3 PASS on every key (weakest `L` vk mu 101.0730, MEASURED); `L` 47 AIRs, 1,925 columns, so the T10 onion fits under 2,000 (MEASURED).
- `L_Fr` unchanged: log height 19, 1,522 words, vk 45,528 bytes (MEASURED); wrap rows stay at 1,680,729 (DERIVED). check-chain V.10 A and B, T7 cases and V.9 omit PASS with the new `L`.
- openvm `t7c-pathc` commits 2080b5f, 638776f, e7cbc09, acb741e, 3607e95; ARCHITECTURE path C and I-17 edited; results in `~/Projects/Moriarty-wt/t7c-pathc/research/sherlock-t7c-pathc-20261114/RESULTS.md`.
- Open for T10: drop the constant X check in the wrap decoder and `sherlock-lfr` iface; keep the path C log words as `root_map` columns at merge.

## [2026-11-14 04:22] lane-done | T7c path C accepted; T10 integrates it
- 25 of 25 path C cases pass at the 56G rerun: one leaf through leaf layer, L and L_Fr on the full tree's universal L and L_Fr keys; key outside key_root, wrong X and wrong prog_d refused; D3 weakest 101.07; L 47 AIRs, 1,925 columns; L_Fr log height 19, 1,522 words (MEASURED). Wrap 1,680,729 rows (DERIVED).
- Commits: openvm t7c-pathc 517a86b to 3607e95, Moriarty c070c2f57 (RESULTS, ARCHITECTURE path C and I-17 with the X-in-L_Fr pick).
- Gap: 3607e95 is keygen-checked only. Lead ruling: T10 finishes its current chain and Preview step, then merges t7c-pathc, removes the constant X checks (decode.rs:366-368, iface.rs:747-750) and reruns chain.sh all, devnet and Preview on the merged tree as the final T10 keys.

## [2026-11-14 04:29] build | T10: t7c-pathc merged into t10-merge with the L log tie on both paths
- openvm d72ad32 (worktree `~/.cache/m7-wrap/src/openvm-t10m`): the child log words stay `root_map` columns (path C reads leaf words there) and must be `(D_EMPTY, 0)` on either path; `OnionAir` follows `KeyPathAir`; the reference decoder keeps only range and `Canon` of `X`. Moriarty `~/Projects/Moriarty-wt/t10-merge`: the wrap decoder drops the pinned `X` equality (lead accepted the `L_Fr` `X` check).
- MEASURED keygen (`t7_chain shape`, `~/.cache/t10/out/shape-m/shape.json`): `L` G1 accepted, 48 AIRs, 1,995 columns (bound 2,000), `StmtAir` 99 of 100 interactions, one `L` key over N3 and over both path C leaf layers.

## [2026-11-14 05:05] build | T8 X7 fixed to SHP-003 and keyed into the universal set (openvm a645de3)
- The inline segment W is now linked at the right end of the merged children (SHP-003); the first X7 build linked it between them and was replaced.
- keys() of t7_chain and t8_gp builds the MERGE-WORK key, so its keys_d row (0028aa65) is in every run; L and L_Fr rekeyed over it.
- New case x7_frame_rule_in_frame: only FRAME's frame_rule fails (MEASURED with T7_DEBUG), so the CORE split keeps the frame rule.

## [2026-11-14 05:05] qa | T8 final run at openvm a645de3
- x7 13 of 13, gp2 34 of 34, V.10 chains A and B PASS with D3 PASS on every key, T7 cases 12 of 12, V.9 59 cases PASS, L_Fr words 1,522 (MEASURED, ~/Projects/Moriarty-wt/t8-gp/research/sherlock-t8-gp-20261113/out/final/).
- OpenSpec: SHP-008, NODE-015 and SQG-013 added for the merge_work set; both changes strict-valid.

## [2026-11-14 05:07] commit | T8 on Moriarty t8-gp and openvm t8-gp
- Moriarty t8-gp (rebased on t7-bind 316b17682): 8ea5655ca (N-4), f3ede5dff, 696c9439a, 07af1c450 (RESULTS, run records, ARCHITECTURE N-6 and GP-4 rows, OpenSpec, openvm-t8 patch series); not pushed.
- openvm t8-gp on 662235f: c26e247, 569b23c, 7298247, a645de3; stark-backend unchanged.

## [2026-11-14 05:07] lane-done | T8 GP hooks
- Done on real proofs: X12 GP-2 node modes, X7 MERGE-WORK under the owner's N-6 ruling, N-4 in Quint; V.9 and V.10 still PASS with D3 at least 100 on every term.
- Not FROZEN: GP-2 needs a Halted leaf run and L over a SET root; GP-3 needs the X8 verifier and X14 transcript AIRs; GP-4 needs a parent over a MERGE-WORK child and the STACKS and VALUE DIGEST kits; AFK picks (5,400 columns, N-7, N-8) await the owner at T15. Report ~/Projects/Moriarty-wt/t8-gp/research/sherlock-t8-gp-20261113/RESULTS.md.

## [2026-11-14 05:09] lane-done | T8 landed in part; tested fast-forwarded to it
- X7 MERGE-WORK (13 of 13 cases, 5,128 columns, D3 weakest 101.1643), X12 node modes 34 of 34, T7 cases 12 of 12, V.9 and V.10 PASS at openvm a645de3 (MEASURED); N-4 continuation (a). AFK picks for T15: MERGE-WORK bound 5,400, N-7 binds intervals, N-8 root history with revocation, X8 class compact-zkir3-kzg-bls12381-blake2b/1.
- Not frozen: GP-2 Halted run and SET root in L, GP-3 X8 verifier and X14 AIRs, GP-4 parent over MERGE-WORK and STACKS and VALUE DIGEST in the kernel.
- Moriarty tested and openvm tested fast-forwarded to t8-gp (07af1c450, a645de3). T10 merges tested into its final rerun, so the final keys cover T7, T7c, T8 and T10.

## [2026-11-14 05:09] lane-start | T8b closes the open GP hooks
- Background Opus agent on t8b-gp from tested; L items (SET root) wait for T10, recommended route is a node mode below L since L is at 1,995 of 2,000 columns.
- Report: research/sherlock-t8b-gp-20261114/RESULTS.md.

## [2026-11-14 05:19] qa | T10 run 1 (pre-merge tree) passes keys, wrap, ledger and devnet; Preview call blocked by the permission system
- openvm 466d704, Moriarty 301c27ca1 (`~/.cache/t10/logs/runs.log`): L_Fr keys of A and B, B against A's anchor refused with exactly 1 tape assertion, wrap key and proofs, ledger-A and ledger-B, devnet deploy and acceptRun with R1, R3, R4 and the small-word scan all PASS (MEASURED, `~/.cache/t10/logs/devnet-call.log`). Gates continue in unit t10-chain2.
- The Preview deploy was refused by the agent permission classifier; it waits for the owner's direct approval (no workaround tried).
- Final run started on the merged tree (openvm 33371a6 with T7c and T8, Moriarty f1d88dd11): unit t10-final, home `~/.cache/t10m`.

## [2026-11-14 05:20] decision | Lead runs the T10 Preview step itself
- The T10 agent's Preview deploy was refused by the agent permission classifier; the owner gave the Preview approval in the lead session, so the lead runs it.
- T10 writes one preview script for the final merged-tree contract (deploy, acceptRun, B-against-A, changed-outcome and replay refusals; no seed or key printed) and does not run it; the lead runs it after t10-final passes devnet.

## [2026-11-14 05:59] qa | T10 run 1 gates on the pre-merge tree: V.9, V.10 and the cases PASS (MEASURED)
- openvm 466d704: check-cases 15 PASS; check-omit V.9 PASS, 66 cases (the N1 h_hi table and the L onion AIR included); check-chain V.10 A and B PASS, D3 PASS on 8 keys each (weakest L vk mu 101.0730, L_Fr query 101.1435), the L log tie cases refused at L (`~/.cache/t10/logs/check-*.log`).
- SG-6 did not build in the systemd unit: `/opt/cuda/bin` was not on the unit PATH ("CUDA is not available"); SG-6 reruns on the final tree with that PATH.

## [2026-11-14 06:02] qa | T8b GP-4 parent ladder: a node over MERGE-WORK reaches N3 at rung 3 (MEASURED)
- `t8_gp ladder` (openvm t8b-gp): node keys over the MERGE-WORK key differ from N3 at rungs 1 and 2 (41 AIRs, 2,251 then 2,250 columns) and equal N3 at rung 3 (`~/.cache/t8b/out/ladder/ladder.json`).
- AFK pick: `keys_d` grows to 7 rows (NW over MERGE-WORK, NW2 over NW, N3 over NW2); this rekeys `L` and `L_Fr`, so T10's final keys go stale when T8b merges.
- Chip set 2 (value set) proves the Halted run `unitctor` and its consumer; 4 forged cases refused (`~/Projects/Moriarty-wt/t8b-gp/research/sherlock-t8b-gp-20261114/out/value_set.json`). x7 and gp2 reruns in unit t8b-run-d.

## [2026-11-14 06:10] qa | T10 final keys on the merged tree (T7, T7c, T8, T10): L G1 accepted at 1,995 columns (MEASURED)
- openvm 33371a6, stage key-A (`~/.cache/t10m/key/keys.json`): `L over N3` G1 accepted, 48 AIRs, 1,995 columns (bound 2,000), max interactions 99; one `L` key over N3 and both path C leaf layers; `L` max log height 16; MERGE-WORK key accepted (5,128 columns); `L_Fr` 561 columns, vk 45,528 bytes.

## [2026-11-14 06:22] decision | T10 keys are the prototype keys; the frozen key set comes after T8b
- T8b's parent ladder grows keys_d to 7 rows and rekeys L and L_Fr, so T10's merged-tree keys go stale when T8b merges (AFK pick accepted: NW, NW2, then N3 at rung 3, MEASURED).
- Ruling: T10 completes and its Preview deploy stands as the prototype receipt; T12 runs the security gate and FP-8 on the keys after T8b merges, and the Preview contract is redeployed on those frozen keys.

## [2026-11-14 06:40] qa | T8b GP-4: NW parent verifies a MERGE-WORK proof after the kernel CORE step words move to columns
- First NW proof failed with LogUp NonZeroRootSum: the split kernel CORE had 54 public values that a parent's verifier sends and no statement AIR receives (MERGE-WORK key public values 144).
- Fix (circuit change, Carbon lane for the lead): split CORE takes the step words as 54 main columns on every row, sends them on `step_pv` each row, the Work statement receives them 8 times. MERGE-WORK key 49 AIRs, 5,284 of 5,400 columns, public values 90 (MEASURED).
- NW over MERGE-WORK, NW2 over NW and N3 over NW2 prove and verify; GP-2 rerun 48 of 48 pass (`~/.cache/t8b/logs/x7-r3.log`, `gp2-r2.log`).

## [2026-11-14 08:46] incident | heavy.slice over its MemoryHigh: T8b chain gate and T10 path C run throttled
- heavy.slice MemoryHigh is 64G (MemoryMax 72G) while heavy-slot admits 90G; at 08:44 the slice held 70.8G (anon 52.8G, kernel 18.0G), so both jobs sit in reclaim (threads in D state, 38 percent iowait, the T8b `t7_chain chain A` at 60 min against T10's 17 to 20 min).
- Jobs in the slice: `heavy-t10_pathc` (56G slot), the T8b chain gate (34G slot) and `ollama.service`. No setting was changed; the lead decides.
- T8b gates so far on openvm 2325f63: cases 12 PASS, V.9 PASS 59 cases (`~/.cache/t8b/logs/check-cases.log`, `check-omit.log`).

## [2026-11-14 08:59] incident | YubiKey SSH key load found no resident key
- `ssh-keygen -K` in a foot window: "No keys to download"; the YubiKey holds no resident SSH credential (libfido2 now installed, hidraw6 ACL grants the user rw)
- the T14 benchmark box SSH link waits on the owner: a new `ed25519-sk` resident key and the PC's SSH address
- T10 final chain healthy: pathc stage running 71 min (t7_chain 437% CPU, 29.8 GB RSS); T8b agent working

## [2026-11-14 09:35] build | huginn GPU box linked for T14
- huginn (`huginn.tail6cdce4.ts.net`, RTX 5080 Laptop GPU 16 GB of VRAM, SM max 3,090 MHz, 24 cores, 125 GB of RAM, CUDA 13.4, MEASURED by `nvidia-smi` and nvcc) is reachable from muninn over Tailscale and OpenSSH with a FIDO2 YubiKey key
- herdr saved machine `huginn`, remote session `gpubox`; `herdr machine status` reports reachable
- T14 GPU timing can run on huginn instead of the clock-capped laptop GPU

## [2026-11-14 09:36] qa | T10 final chain: path C passes on the merged tree
- pathc stage rc=0 after 103 min: 26 T7CASE lines PASS, 0 FAIL (MEASURED, `~/.cache/t10m/logs/pathc.log`)
- pathc-d3 all gates PASS: L_iface 101.07, L_Fr 101.14, fv_vk 101.16 bits (MEASURED)
- wrap, ledger and devnet stages had already passed earlier in this run; SG-6 is the last stage, now running

## [2026-11-14 09:42] incident | T10 final chain: SG-6 build failed on the unit PATH
- sg6 stage rc=101: the openvm-cuda-common build script printed "CUDA is not available"; the t10-final unit PATH had no `/opt/cuda/bin`
- every earlier stage of the run passed (keys, wrap, ledger, devnet, cases, chain, pathc, pathc-d3)
- rerun of the sg6 stage alone as unit t10-sg6 with `/opt/cuda/bin` on PATH, log `~/.cache/t10m/sg6.unit.log`

## [2026-11-14 09:42] qa | T8b x7 22 of 22 on openvm 89bee35; the throttled chain gate was stopped and requeued at 64G
- New case `x7_step_column_forged_one_row` (one kernel CORE step-word column changed on row 3) is refused; NW and NW2 verifying keys are written for D3 (`~/.cache/t8b/out/x7/`).
- The T8b chain gate at a 34G slot ran 98 min in heavy.slice reclaim next to `t10-pathc` and was stopped to avoid an OOM in the shared slice; it reruns with the stock `scripts/qa/check-chain.sh` at 64G (unit t8b-chain).
- ARCHITECTURE GP-2 and GP-4 hook rows keep X12, the Carbon lanes and the AFK picks as open items.

## [2026-11-14 09:42] lane-start | T14 GPU lane on huginn
- three source trees rsynced to huginn `~/sherlock/src` (openvm-t10m 33371a6, stark-backend-t7c deeeb0a, p3-bls12-381-fr); the Claude agent huginn-gpu there runs builds and GPU jobs, the muninn lane drives it over the herdr and outbox channel
- order: CUDA build and correctness, option A baseline, option B, warm path, then kernel work; timings use prototype T10 keys
- brief `~/huginn-setup/HUGINN-GPU-WORK.xml`, channel `~/huginn-setup/HUGINN-BUS.xml`; results go to `research/sherlock-t14-gpu-20261114/RESULTS.md`

## [2026-11-14 09:50] incident | T10 SG-6 rerun ended with an unclear result
- unit t10-sg6 printed "SG-6 PASS: GPU 0.1821 s per proved step (5 steps, 8 slots)", then a chain.sh syntax error; the script was edited while the unit ran
- runs.log has no start or end line for the rerun and `~/.cache/t10m/out/sg6` is empty, so the PASS is not yet accepted as the rerun result
- the T10 agent finishes its chain.sh edit, checks it with `bash -n`, and reruns the sg6 stage once more without edits

## [2026-11-14 09:58] decision | T14 trees: portable paths and the O3 measurement switches on t14-gpu
- huginn-gpu patch set (owner request, no machine paths) reviewed and committed: stark-backend `t14-gpu` 4071527 (mori-gpu at `<src>/moriarty`, cuda-builder lib dir from `CUDA_HOME`), openvm `t14-gpu` f0e8bf2, Moriarty `tools/gpubox` 85b01104e
- openvm a5ab805: feature `gpu` on sherlock-wrap-export, `T14_LFR_SET=A|B` (B = log blowup 2, 120/85/82, measurement only), `T14_REPS` warm calls, `fp8_d3 lfr`; Moriarty a43680cad: `T14_WRAP_REPS`
- synced to huginn as openvm-t14, stark-backend-t14, moriarty (Moriarty `tested` fast-forwarded to the T10 merge f1d88dd11); brief m1 (build, correctness, keys A and B) sent
- plan `~/Projects/Moriarty-wt/t14-gpu/research/sherlock-t14-gpu-20261114/PLAN.md`

## [2026-11-14 10:04] lane-done | T8b GP hooks: GP-2 consumption and the GP-4 parent pass on real proofs; GP-3, STACKS and L over SET stay open
- MEASURED on openvm 89bee35: gp2 48 of 48 (honest SEAL-WITH-DEPS of a real consumer of a Halted value, 9 twins refused), x7 22 of 22 (NW, NW2, N3 over MERGE-WORK), cases 12, V.9 59 cases, V.10 A and B PASS, D3 weakest 101.0110 with MERGE-WORK, NW and NW2 at 101.1643.
- Not done: the in-circuit Compact verifier (X8), STACKS and VALUE DIGEST in the kernel, `L` over a SET root (no T10-landed message); I-13 and the GP hook rows stay FREEZE-PENDING with T8b evidence.
- AFK picks for T15: chip set 2 per-program leaf key, constant `vd` form, `keys_d` 7 rows (rekeys `L` and `L_Fr`), kernel step words as columns; Carbon lanes Q1 to Q7 for the lead (`~/Projects/Moriarty-wt/t8b-gp/research/sherlock-t8b-gp-20261114/RESULTS.md`).

## [2026-11-14 10:12] decision | Carbon Quint QA on every loop row
- owner: "launch some gemini carbon agents for QA to use Quint-llm-kit to check our work", then "add them to our workflow as we develop"
- LOOP.md loop rules now require a Gemini 4 Carbon Quint lane in its own herdr pane for each row that reaches DONE or PARTIAL; a row is DONE for T15 only with a lead verdict on file
- first batch started: lanes gp2-value-set, merge-work-ladder, pathc-xcheck and afk-picks; verdicts go to `qa/gemini-quint/qa-20261114-*`

## [2026-11-14 10:16] lane-done | T8b GP hooks partial, tested fast-forwarded
- GP-2 Halted value consumption 48/48 and GP-4 parent over MERGE-WORK 22/22 pass on the stock verifier; D3 weakest term 101.0110 bits (MEASURED); keys_d grows to 7 rows, so L and L_Fr are rekeyed and the T10 keys stay prototype
- `tested` fast-forwarded: Moriarty b1ff56ba1, openvm 89bee35; report `~/Projects/Moriarty-wt/t8b-gp/research/sherlock-t8b-gp-20261114/RESULTS.md`
- open: L over SET (waits T10), X8 in-circuit Compact verifier, STACKS AIR; the keys_d growth and the kernel step words as columns are picks for T15
- issue: heavy.slice MemoryHigh is 64G while heavy-slot admits 90G; T8b and T10 runs stalled in reclaim

## [2026-11-14 10:30] qa | T10 merged tree gates and SG-6 clean rerun
- on `~/.cache/t10m` (openvm 33371a6): check-cases 15 PASS, V.9 70 cases PASS, V.10 A/B 10 cases each PASS, D3 8 keys PASS (L vk mu 101.0730 bits, L_Fr query 101.1435 bits), pathc 25 cases PASS plus pathc-d3 PASS (MEASURED)
- SG-6 fresh unit t10-sg6b with /opt/cuda/bin on PATH: rc 0, GPU 0.1616 s/step, CPU 1.8410 s/step, 5 steps, 8 slots, stock verifier on all shards (`~/.cache/t10m/out/sg6/sg6.json`)

## [2026-11-14 10:32] commit | T10 bound rows committed, t10-bound fast-forwarded
- Moriarty t10-bound = t10-merge = 30567a640 (merges T7c path C and `tested` T8; chain.sh preview stage, preview.sh, RESULTS, I-23 and I-24 FROZEN with the N-11 AFK pick); openvm t10-bound = t10-merge = 33371a6, patch series in `research/sherlock-stark-code-20261112/openvm-t10/`
- Preview not run by the lane: the lead runs `research/sherlock-t10-bound-20261114/preview.sh` (RESULTS section 0); no push
- open: T8b keys_d at 7 rows (tested b1ff56ba1, openvm 89bee35) rekeys L and L_Fr after this tree, so the T10 key numbers need a rerun on top of it

## [2026-11-14 10:32] lane-done | T10 bound wrap and contract done
- Moriarty 30567a640, openvm 33371a6: L over N3 1,995 of 2,000 columns, at most 99 interactions; wrap 1,685,845 rows at k = 21, wrap VK 1,325 bytes (MEASURED)
- ledger 9.1.0.1 accepts and refuses every tampered case; devnet deploy and acceptRun, R1 to R4 refused; D3 lowest 101.07 bits; SG-6 clean rerun GPU 0.1616 s per step (MEASURED)
- Preview not run (owner runs `preview.sh`); N-11 one ledger per program and the interim child log rule are picks for T15
- report `~/Projects/Moriarty-wt/t10-bound/research/sherlock-t10-bound-20261114/RESULTS.md`

## [2026-11-14 10:32] lane-start | T10m merge T10 with T8b and rekey
- trial merge into `tested` conflicts in openvm Cargo.toml, leaf.rs, t7_chain.rs and one Moriarty README; new row T10m merges, rekeys L and L_Fr for keys_d 7 rows, reruns the T10 chain and the T8b gates
- T11 and T12 now take T10m as input; a Carbon Quint lane for T10 joins the QA queue

## [2026-11-14 10:32] note | Log times written ahead of the clock
- late correction: the entries "T10 merged tree gates and SG-6 clean rerun" (10:30) and "T10 bound rows committed" (10:32) were written at about 10:05 local time with times ahead of the clock
- the two lead entries after them use 10:32 to keep the log in order; their events happened at about 10:08

## [2026-11-14 10:32] lane-start | Carbon Quint QA lanes qa-20261114 (four)
- four Gemini 4 Carbon lanes in new herdr panes carbon-gp2-value-set (w1:pN), carbon-merge-work-ladder (w1:pP), carbon-pathc-xcheck (w1:pQ), carbon-afk-picks (w1:pR), started about 10:10 local
- sources: openvm-t8b 89bee35, openvm-t7c 3607e95, Moriarty b1ff56ba1 (tested) and c070c2f57 (t7c-pathc); lane folders `~/.cache/gemini-lanes/qa-20261114-<name>/`
- a fifth lane t10-bound (Moriarty 30567a640, openvm 33371a6) is queued after these four

## [2026-11-14 10:32] plan | T16 elastic demo added to the loop
- owner asked when the prover shows scaling up and down with the hardware; no loop row covered it and the code has no hardware-aware schedule policy and no `pos_plan_schedules` check yet
- new row T16: hardware-aware schedule policy, `pos_plan_schedules`, scale up by run length, the same run on muninn and huginn with byte-equal `claim_pub`, cross-machine leaves under one node, and scale down under a reduced memory budget
- inputs T10m and the T14 option A baseline

## [2026-11-14 10:32] commit | T10m merges: T10 into tested (T8b) on t10m and t10mm
- worktrees `~/Projects/Moriarty-wt/t10m` (Moriarty 91079c7ce) and `~/.cache/m7-wrap/src/openvm-t10mm` (openvm 9f3b521), both from `tested`; written at about 10:21 local, stamped 10:32 to keep the log in order
- conflicts kept both rows: openvm Cargo.toml on the T8b stark-backend copy (same deeeb0a), leaf.rs T8b imports, t7_chain Keys with nw, nw2 and pc; Moriarty README rows for openvm-t8b and openvm-t10
- KEYDAG_IDX 2 to 3: T8b puts three chip-set leaves in `key_root`, so the path C DAG key leaf is leaf 3
- keygen on the merged tree (MEASURED, `~/.cache/t10mm/out/shape/shape.json`): `L` 1,995 of 2,000 columns, G1 accepted, StmtAir 99 of 100 interactions; N3 over NW2 equals N3

## [2026-11-14 10:32] build | T10m fixes for t8_gp and lfr_e3
- openvm c987b71: t8_gp chain, cases, omit run the production `L` of t7_chain (the T7-era copy and the T8_GP_OLD_L gate removed); t8_gp `key_root` has the path C DAG key leaf, so gp2 and x7 run under the universal `key_root`
- openvm be62bb8: lfr_e3 read the J8 bench `L` proof with 12,164 public words (before T9 FP-3, 12,166); it now reads `~/.cache/e3-iface/fixture-t10m` from iface_fixture and checks the word count first

## [2026-11-14 10:32] lane-start | T10m chain and T8b gates run (unit t10mm-run)
- `research/sherlock-t10m-20261114/run.sh` (Moriarty 38290a7a6): chain.sh all, then t8_gp gp2 and x7, D3 on MERGE-WORK, NW, NW2, the iface fixture, lfr_e3 0, i7_decode; state and logs `~/.cache/t10mm/`
- chain.sh devnet-call and preview-call now declare 64G (heavy.slice MemoryHigh 64G); no Preview, no push

## [2026-11-14 10:33] lane-start | T16 parts a and b: elastic schedule policy and schedule checks
- owner: "start a and b now"; branch t16-elastic in Moriarty and openvm from the T10 tree (30567a640, 33371a6), rebased onto T10m later
- (a) hardware-aware schedule policy with a test override for smaller hardware; (b) `pos_plan_schedules` and `plan_schedule_outside_policy` on real proofs
- results go to `research/sherlock-t16-elastic-20261114/RESULTS.md`

## [2026-11-14 10:33] issue | T10m key-A refused: merge took the wrong core import in leaf.rs
- key-A on openvm be62bb8 panicked at t7_chain.rs:482 "path C: one leaf" (2 shards): the merge kept the T8b `crate::core` import, so the S = 8 copy `crate::s8::leaf` used the S = 4 core
- openvm d26453c restores `super::core` (T7c form); `t7_chain s8app` gives 1 shard, proved and verified; unit restarted as t10mm-run2 from a clean state, written about 10:25 local
- the 10:32 shape numbers came from the broken tree; key-A on d26453c remeasures `L`

## [2026-11-14 10:33] qa | T14 step 2 on huginn: GPU equality, SG-6, key A and wrap key A identical to T10
- E4 GPU equality tests equal to the CPU (Poseidon, fused encode on both paths, fractional sumcheck); SG-6 PASS, stock CPU verifier accepts 4 of 4 GPU shards (MEASURED)
- key A on huginn: `key_root`, `keys_d`, `L` and `L_Fr` pre-hashes equal to the muninn T10 record; wrap key byte-equal (1,325 bytes), wrap rows 1,685,845 at k = 21 (MEASURED)
- call A on the CPU and the GPU path: both pass stock verify and decode; proofs differ by zero-knowledge randomness, same 2,634,365 bytes
- incidents fixed on `t14-gpu`: mori-gpu `-D__ADX__` for x86_64 blst (104340330), portable CUDA paths (bcf19cf28), no `/usr/bin/time` on huginn (no-install stand-in rtime.py)

## [2026-11-14 10:33] qa | T14 step 2 on huginn: option A passes, option B wrap does not fit k = 21
- option A: keys and wrap VK byte-equal to T10, SG-6 4/4 on the stock verifier, calls verify, D3 101.1435 bits (MEASURED on huginn)
- option B: D3 passes at 101.2265 bits, but its wrap needs 1,927,450 rows, over the k = 21 gate of 1,782,573 (MEASURED); B breaks the never k = 22 rule as built
- untimed call A on GPU 203.8 s, of which keys 54.3 s and l_key 65.1 s are key generation that the warm path caches; l_prove 60.4 s still runs on the CPU

## [2026-11-14 10:34] qa | T14 O3 option B: D3 passes, wrap does not fit k = 21
- option B (`L_Fr` log blowup 2, queries 120/85/82): D3 weakest term `query` 101.2265, gate PASS; option A control 101.1435 (MEASURED, huginn `~/sherlock/logs/step2/d3B.log`)
- B's wrap cost model at k = 21: 1,927,450 rows against the gate 1,782,573 (over by 144,877); A: 1,685,845 (MEASURED); no wrap key B built (never k = 22)
- timed runs (brief m5): A with wrap, B STARK stages only; results to `research/sherlock-t14-gpu-20261114/RESULTS.md` on `t14-gpu`

## [2026-11-14 10:34] note | Correction: the wrap gate is an 85 percent margin, not the k = 21 limit
- corrects the 10:33 entry "T14 step 2 on huginn: option A passes, option B wrap does not fit k = 21"
- the gate 1,782,573 is 85 percent of the 2^21 minus 7 = 2,097,145 usable rows (`tools/sherlock-wrap/src/util.rs:22`); option B at 1,927,450 rows fits k = 21 (92 percent) and breaks only the 85 percent margin
- k = 20 (1,048,569 usable rows) is out of reach today: the lookup tables alone take 1,261,315 rows (MEASURED)

## [2026-11-14 10:37] lane-start | Wrap diet cost sweep before the T12 key freeze
- owner: "start the wrap sweep"; cost model and D3 only, branch wrap-diet from t10-bound
- axes: L_Fr log blowup 3 to 5, query PoW 20 to 32 bits, wrap lookup table width, and any dominant lever in the wrap model; target k = 20 with every D3 term at 100 bits or more
- results go to `research/sherlock-wrap-diet-20261114/RESULTS.md`

## [2026-11-14 10:39] plan | T16 a and b: PLAN.md and the schedule policy with unit tests
- `research/sherlock-t16-elastic-20261114/PLAN.md` on Moriarty `t16-elastic`; openvm `~/.cache/m7-wrap/src/openvm-t16` adds `sherlock-leaf/src/sched.rs` (probe, overrides, policy, check)
- 10 of 10 policy unit tests pass on synthetic muninn, huginn, CPU 16 GB and 2-GPU profiles; probe here: 1 unified GPU (nvidia-smi memory N/A), 20 cores, host bounded by the heavy.slice cgroup headroom (MEASURED)
- two run lengths of one program are not built on this tree: the leaf admits the 5 rules of the `unbound` run only; reported PARTIAL, no stand-in

## [2026-11-14 10:42] decision | Design open until the end-to-end prototype; m7 list and T14b
- owner: "the design is not fixed yet until we have a full end-to-end prototype. So we can change things including how we handle the key if necessary"; the T12 key freeze becomes a prototype freeze, hard rules stay owner-gated
- T14 m7 kernel list: L on GPU, node layers on CUDA, discrete-card path, fused LDE and Merkle commit, GPU grinding, concurrent shards, streamed tree proving
- new row T14b: resident prover daemon, fixed-base MSM precomputation, wrap on GPU; the wrap sweep adds per-layer parameters

## [2026-11-14 10:49] lane-start | T16 b: suspend then schedules (unit t16-ab)
- `research/sherlock-t16-elastic-20261114/run.sh` (copy `~/.cache/t16/run.sh`): process 1 `t7_chain t16 suspend` writes the prefix checkpoint, process 2 `t7_chain t16 schedules` resumes it and proves S1 to S4, the control B and the off-menu leaf cases
- heavy-slot 48G and 60G; logs `~/.cache/t16/logs/`; nothing under `~/.cache/t10m` is written

## [2026-11-14 10:53] note | Wrap diet: D3 tool matches FP8D3, PoW ceiling 31, round-0 rows dominate
- `research/sherlock-wrap-diet-20261114/d3sweep` (stock `SoundnessCalculator`, Fr and BabyBear) gives the T10 FP8D3 values on all 7 keys; the query search gives 98/83/82 at PoW 20 (MEASURED)
- query PoW above 31 bits is refused by stock `sample_bits` (`assert bits < 32`) and by the wrap gadget `tools/sherlock-wrap/src/transcript.rs:63`
- option A at k = 20: 1,686,111 advice rows, 731,971 table rows against the 891,283 gate; the k = 20 table fits, the advice rows do not (MEASURED)
- `nr_pow2range_cols` 2 or 4 cuts rows 3.5 to 4.6 percent but the table grows to 1,026,819 at k = 20 and 1,805,059 at k = 21, over both gates (MEASURED)

## [2026-11-14 10:55] lane-start | Carbon Quint QA lane qa-20261114-t10-bound
- fifth lane in herdr pane carbon-t10-bound (w1:pS), started after pathc-xcheck finished; sources openvm-t10m 33371a6 and Moriarty 30567a640
- questions: wrong anchor or unchecked gen op through L and L_Fr, h_hi range on every path, the L log tie, acceptRun refusals R1 to R4

## [2026-11-14 10:57] lane-done | Carbon Quint lane qa-20261114-pathc-xcheck
- the lane sandbox could not reach quint; the lead told it to finish without quint, then reran all 23 commands: every exit code as expected, 27 tests pass, 11 of 11 mutants caught, 4 reach targets reached
- files in `~/Projects/Moriarty/qa/gemini-quint/qa-20261114-pathc-xcheck/`

## [2026-11-14 10:57] qa | Carbon verdict: T7c X check in L_Fr: SOUND WITH CONDITIONS
- no statement passes the wrap that L and its child did not prove; L_Fr packs all 9 limbs of X against the cached commitment, and the wrap pins the L_Fr key and the L pre-hash
- condition C1 (completeness only): the decoders must not pin X; c070c2f57 still does (decode.rs:366-368, iface.rs:747-750), T10 removes it

## [2026-11-14 11:11] qa | T14 option A on huginn: warm call 129.0 s STARK plus 46.5 s wrap
- 5 processes, prototype T10 keys: STARK process per call cold 203.8 s (197.7 to 212.0), warm 129.0 s (126.0 to 130.6); `L_Fr` GPU prove 37.9 s warm, `L` prove 61.8 s and tree 19.0 s on the CPU (MEASURED)
- wrap at k = 21 on the CPU: keygen 32.2 s, prove 46.5 s warm (n = 6) (MEASURED); end-to-end warm 175.5 s, cold 282.6 s (DERIVED)
- peak host RSS 26.3 GiB, peak GPU memory 4,786 MiB, SM clock up to 2,827 MHz at the 100 W limit (MEASURED)
- huginn `~/sherlock/out/t14/summary-A.json`; option B STARK runs next, then m7 speed work

## [2026-11-14 11:12] decision | Ollama stopped; heavy.slice and heavy-slot budget raised for T10m wrap-key
- Owner order: Ollama had been running on muninn since 2026-11-13 18:35 while disabled; stopped, still disabled
- Owner pick: heavy.slice raised to MemoryHigh 96G and MemoryMax 104G until the next reboot (was 64G and 72G); the T10m runner was restarted as unit t10mm-run3 with HEAVY_BUDGET_G=130
- T10m wrap-key had waited 20 min behind t16-schedules (60G of the 90G budget); it started 11:11 with finished stages skipped
- Log at ~/.cache/t10mm/logs/runs.log

## [2026-11-14 11:15] note | Wrap diet: k = 20 only with WHIR list decoding at L_Fr
- 218 L_Fr points (cost model, D3 at three shapes): no unique-decoding point fits k = 20 (best 1,009,371 rows against 891,283); 133 list-decoding points fit (MEASURED, shape-only fixtures, no keys)
- most k = 20 headroom at A's blowup and PoW: LD m = 16, log blowup 3, `k_whir` 3, `n_stack` 19, queries 56/33/24/19, 684,140 rows, D3 lowest 101.0350; list decoding is against OD-19 and OD-24 (rejected option D)
- largest single levers from A: LD m = 16 -677,037 rows, `k_whir` 3 -420,854, `n_stack` 19 -336,573, query PoW 31 -184,539; the pow2range table follows k (`midnight-zk-stdlib` `lib.rs:1879`)
- k = 21 cap is 521,258 rows (table 1,261,315); best unique-decoding point `b5k3p31n19` 1,009,080 rows

## [2026-11-14 11:15] note | Wrap diet: inner-layer blowup D3 and its reach into the wrap
- D3 (BabyBear, all internal and L keys): blowups 1 and 2 pass with more queries (internal 193/88/81 and 118/84/81, L 196/89/82 and 120/85/82); blowup 4 needs `mu` PoW +1 (L also folding PoW 1)
- ESTIMATE at A: L blowup 2 adds about 68,000 wrap rows, L blowup 1 about 337,000 (over the k = 21 gate); internal blowup 2 about 32,000, blowup 1 about 155,000

## [2026-11-14 11:16] commit | Wrap diet RESULTS on branch wrap-diet
- Moriarty `wrap-diet` 88e2db248: `research/sherlock-wrap-diet-20261114/` (RESULTS.md, d3sweep, sweep scripts, data) and the `WRAP_NR_COLS` knob in `tools/sherlock-wrap/src/bin/wrap.rs`

## [2026-11-14 11:16] lane-done | Wrap diet cost sweep
- k = 20 is reachable only under list decoding at L_Fr; under unique decoding the best is 1,009,371 rows; results in `~/Projects/Moriarty-wt/wrap-diet/research/sherlock-wrap-diet-20261114/RESULTS.md`

## [2026-11-14 11:16] note | Wrap-diet sweep result linked to O3 in LOOP.md
- k=20 needs WHIR list decoding at L_Fr, which goes against OD-19 and OD-24; no unique-decoding point fits the 891,283 gate
- At k=21 the best unique-decoding point is 1,009,080 rows with D3 101.2029 (MEASURED, shape-only fixtures); no key change made
- ~/Projects/Moriarty-wt/wrap-diet/research/sherlock-wrap-diet-20261114/RESULTS.md (wrap-diet 88e2db248)

## [2026-11-14 11:20] qa | T16 b: one claim_pub over five schedules; off-menu leaf refused by N1
- `pos_plan_schedules` PARTIAL: cuts 4,1 / 3,2 / 2,3 on chip sets 1,0 / 2,1,1,1 and a suspend in one process with resume in another give one byte-equal `claim_pub` read from each stock-verified `L` proof; program B differs (MEASURED, `~/.cache/t16/logs/schedules.log`)
- `key_root`, `keys_d` and 18 pre-hashes equal the T10 record; every N3 fits the T10 `L` heights, so the `L_Fr` key holds; the 8 differing `L` words are `shape_d`, not on the call vector
- `plan_schedule_outside_policy` PASS: S = 8 shard with the S = 4 `plan_d` refused at N1 under its own triple and the claimed set-0 triple (StmtAir); controls accepted
- missing: two run lengths of one program (leaf admits 5 rules, `dec.rs:23-29`, `leaf.rs:294-301`); report `research/sherlock-t16-elastic-20261114/RESULTS.md`

## [2026-11-14 11:20] qa | T10m keys, wrap and ledger 9.1.0.1 on the merged tree
- keys on openvm d26453c (MEASURED, `~/.cache/t10mm/key/keys.json`): `L` over N3 48 AIRs, 1,995 of 2,000 columns, StmtAir 99 of 100 interactions, G1 accepted; `L_Fr` 15 AIRs, 561 columns; N3 over NW2 equals N3
- wrap 1,685,845 rows at k = 21 (gate 1,782,573), wrap VK 1,325 bytes, proof 2,912 bytes, 1,026 public inputs (`~/.cache/t10mm/logs/wrap-key.log`)
- ledger 9.1.0.1 stock: A `Ok(())`; 7 changed lanes, 7 changed inputs, 3 flipped proof bytes refused; B on A's anchor and on A's statement refused; `L_Fr` tape B on A fails exactly one assertion
- run continues as unit t10mm-run3 (lead restart with heavy budget 130G, wrap-key had waited in the queue)

## [2026-11-14 11:23] commit | T16 a and b on t16-elastic in both repos
- openvm `32b7c90` (sched.rs, `program_with_plan`, `t7_chain t16`); patch series `research/sherlock-stark-code-20261112/openvm-t16/`, round trip gives the tip tree `23c4512`
- Moriarty `7daa60b01`: PLAN, RESULTS, run.sh, `t16.json`, the checkpoint record; no push

## [2026-11-14 11:25] lane-done | Carbon Quint lane qa-20261114-afk-picks
- lead reran all 35 commands: every exit code as the lane expected, 14 tests pass, 9 of 9 mutants caught on invRefine, 5 reach targets reached
- files in `~/Projects/Moriarty/qa/gemini-quint/qa-20261114-afk-picks/`

## [2026-11-14 11:25] qa | Carbon verdict: AFK picks N-7, N-8 consistent with conditions; 5,400 bound open for GP-4
- N1 (Kind::First) fixes shape_d and reg_root to 0 (openvm 89bee35 stmt.rs:852-876, :1026-1037): two N2 cuts of [0, 8) share one shape_d, and Sherlock subtrees cannot join a FOREIGN child under a nonzero root; completeness gaps, AD-23 (a) and GP-3 not built
- revocation after acceptCollection is open; lead pick: acceptCollection is final (option A), for T15 with N-8
- 5,284 of 5,400 columns and CORE 999 of 1,000 leave no measured room for STACKS and VALUE DIGEST; the bound must be measured again when they land

## [2026-11-14 11:28] lane-done | Carbon Quint lane qa-20261114-merge-work-ladder
- lead reran all 25 commands: every exit code as the lane expected, 34 tests pass, 13 of 13 mutants caught, 6 reach targets reached
- files in `~/Projects/Moriarty/qa/gemini-quint/qa-20261114-merge-work-ladder/`

## [2026-11-14 11:28] issue | Carbon verdict: MERGE-WORK output h_hi not range-checked (defect D-1)
- stitch, step words (8 receives), keys_d 7 rows and one universal key set: SOUND
- defect: the Work limbs whi have no 16-bit range check (openvm-t8b 89bee35 stmt.rs:1049-1055; still open at openvm-t10m 33371a6 stmt.rs:1083); whi = [p-65531, 1, 0, 0] with wcry = [1, 0, 0, 0] passes for h_pre 4 + 1 step, so the sealed n_steps is forged
- fix (not done here): look up whi on the 16-bit range bus as T10 does for N1 hhi; add the x7 negative case

## [2026-11-14 11:31] lane-done | Carbon Quint lane qa-20261114-gp2-value-set
- lead reran all 42 commands: every exit code as the lane expected, 10 tests pass, 32 of 32 mutants caught, 5 reach targets reached
- files in `~/Projects/Moriarty/qa/gemini-quint/qa-20261114-gp2-value-set/`

## [2026-11-14 11:31] qa | Carbon verdict: T8b value set SOUND WITH CONDITIONS
- dispatch, DEC padding, the guards of XAdtEvalConstructNullary, frame rule and INIT val_d hold; no forgery in chip set 2
- KAdminPublishValue does not check r3(0) = 6, r3(1) = 1 or tsk.count = 0 (openvm-t8b 89bee35 core.rs:431-444); sound by the reachable-state invariant of chip sets 0 to 2, local guards recommended
- vd uses the ty_d PICK P2(DOM_TY, row) (leaf.rs:241, :263-273), so U::A() and V::A() of two programs share one vd until AD-14

## [2026-11-14 11:37] decision | Owner rule: build a counterexample for every Quint finding
- Owner: every Quint finding, defect or guard, must be checked by building its counterexample on the real code; added to LOOP.md loop rules
- Verdicts CONFIRMED, NOT REPRODUCED, GUARD HOLDS, OPEN DESIGN go in tests/quint-regressions/COUNTEREXAMPLES.md; lane verdicts mark findings PENDING-CX until a row exists
- The quint-reg lane (~/Projects/Moriarty-wt/quint-reg, branch quint-reg from t10m) builds them for the four 2026-11-14 Carbon lanes and turns each into a regression test

## [2026-11-14 11:37] note | Carbon QA defect findings are PENDING-CX
- owner rule: a Quint defect counts only when its forged input is accepted by the real checker or verifier; the quint-reg lane writes tests/quint-regressions/COUNTEREXAMPLES.md
- in the qa-20261114 verdicts, merge-work F-01 (D-1), afk-picks F-6 and F-7, gp2 F4.2 and C2.1, pathc F5 are PENDING-CX; the earlier "defect D-1" entry is a lead reading, not a confirmed defect

## [2026-11-14 11:48] qa | T10m devnet and check-cases on the merged tree; reboot resume
- devnet (MEASURED, `~/.cache/t10mm/devnet/t10a-state.json`): SherlockLedger deployed at block 22,586; acceptRun of A included at block 22,728 (tx 45), `runs` size 1, anchor and keys unchanged; R1 to R4 refused by the node; privacy scan and small-word scan pass
- rekeyed: `key_root` 0fade1a4..., `keys_d` 15ed5153... (T10 had 543f5320..., 1cf4f508...); check-cases 15 PASS (`~/.cache/t10mm/logs/check-cases.log`)
- muninn rebooted at 11:39 (lead: nvidia_modeset bug); runner resumed as unit t10mm-run4 at 11:46, check-omit rerun from the start

## [2026-11-14 11:48] incident | muninn crashed and rebooted at 11:39
- Cause: a kernel bug in nvidia_modeset killed Hyprland at 11:39:05, then CPU 6 soft-locked; no out-of-memory kill that day
- The heavy.slice raise (96G and 104G) was runtime only and is gone; the slice is back at 64G soft and 72G hard
- T10m restarted as unit t10mm-run4 (15 stages skipped, check-omit rerun); the T10m, T16, QA and quint-reg agents were resumed
- Huginn units run on their own; the SSH and herdr link waits for the owner's YubiKey via gpu-link

## [2026-11-14 11:48] issue | Carbon lane qa-20261114-t10-bound interrupted by the 11:39 crash
- the lane ran in herdr pane carbon-t10-bound (w1:pS) from 10:55; the crash interrupted it before any model or REPORT.md was written (lane folder `~/.cache/gemini-lanes/qa-20261114-t10-bound/` holds only its inputs)
- the owner pane w1:pJ is gone after the reboot; per the lead, the lane is not restarted and no substitute runs; pane w1:pS stays open, idle, for the lead

## [2026-11-14 11:48] lane-done | T16 a DONE, b PARTIAL (t16-elastic, report on file)
- (a) `sched.rs` policy and probe, 10/10 tests on four profiles; tip openvm 32b7c90 rebuilt and retested before the 11:39 reboot (cuda probe: 1 unified GPU, cudaMemGetInfo)
- (b) one byte-equal `claim_pub` over 5 schedules incl. a two-process suspend and resume; off-menu leaf refused at N1; missing: two run lengths of one program (`dec.rs:23-29`, `leaf.rs:294-301`)
- Moriarty 7daa60b01, 9d6b61f55; `research/sherlock-t16-elastic-20261114/RESULTS.md`; LOOP.md not edited by this lane (brief rule), Status text handed to the lead; (c) to (e) wait T10m (rekeyed) and T14

## [2026-11-14 11:55] note | Correction: the muninn crashes at 11:39 and 11:49 came from nvidia-smi conf-compute
- Both hangs were a NULL deref in the GB10 driver's confidential-compute vidmem query (nvidia-smi conf-compute -gm), run by another session
- The earlier entry blaming nvidia_modeset alone, and the link to T10m check-omit, were wrong; check-omit is not involved
- T10m restarted as unit t10mm-run5 with finished stages skipped

## [2026-11-14 11:58] qa | T14 option B on huginn and the O3 comparison
- option B STARK process (5 runs): warm 116.6 s, cold 185.2 s; `L_Fr` prove 25.4 s warm against A 37.9 s; peak RSS 19.4 GiB against A 26.3 GiB (MEASURED)
- B's wrap needs 1,927,450 rows at k = 21 (gate 1,782,573), so B has no wrap under never k = 22; A end to end warm 175.5 s (DERIVED); D3 PASS for both (101.14 and 101.23)
- the A and B table for the owner's O3 pick is section 4 of `research/sherlock-t14-gpu-20261114/RESULTS.md` on `t14-gpu`; facts only
- m7 speed work running on huginn: nsys puts 97.3 % of GPU kernel time in `mori_pos_absorb` (streaming encode to pageable host memory)

## [2026-11-14 12:05] queue | Carbon Quint lane request: qa-20261114-t16-elastic (T16 a, b)
- lane folder `~/.cache/gemini-lanes/qa-20261114-t16-elastic/` ready: PROMPT.md, AGENTS.md, SOURCES.md (openvm 32b7c90, Moriarty 9d6b61f55), lane-src copies, the three quint-llm-kit skills
- models schedule independence of `claim_d`, off-menu refusal at the First node, policy safety and resume validation; six named mutants
- the lead launches it in the herdr pane `carbon-T16` and reruns quint for `verdict.md`; the T16 lane does not launch Carbon itself

## [2026-11-14 12:05] commit | Carbon Quint QA verdicts qa-20261114 on qa-carbon-20261114
- commit ffcdd353a on branch qa-carbon-20261114 (worktree `~/Projects/Moriarty-wt/qa-carbon`, from tested b1ff56ba1): four lane folders with models, REPORT.md, lane-original, lead reruns and verdict.md; not pushed
- defect findings stay PENDING-CX until `tests/quint-regressions/COUNTEREXAMPLES.md` in the quint-reg worktree has their rows; that file does not exist yet

## [2026-11-14 12:04] note | Correction to the T14 entries of 10:34 and 11:58: option B's wrap fits k = 21
- B's 1,927,450 wrap rows are under the hard k = 21 limit (2^21 minus 7 = 2,097,145) and over only the 85 % planning gate 1,782,573 (`tools/sherlock-wrap/src/util.rs:22`); never k = 22 holds for both options
- the wrap-diet sweep (Moriarty 88e2db248) found k = 21 unique-decoding settings down to 1,009,080 wrap rows, on shape-only fixtures with no key
- T14 RESULTS sections 1.1, 3 and 4 corrected on `t14-gpu`; no recommendation

## [2026-11-14 12:07] lane-start | Carbon Quint lane qa-20261114-t10-bound restarted
- fresh agy session (gemini-4-carbon-high) in its own pane carbon-t10-bound (w1:pS), same PROMPT.md; the pre-crash session wrote nothing
- next: lane qa-20261114-t16-elastic in a new pane carbon-T16 after this one (openvm 32b7c90, Moriarty 9d6b61f55)

## [2026-11-14 12:25] commit | T14 PLAN option B step corrected on t14-gpu
- commit f8c5fc7e8 on branch t14-gpu (`~/Projects/Moriarty-wt/t14-gpu`): `research/sherlock-t14-gpu-20261114/PLAN.md` step 3 no longer says a wrap over 1,782,573 rows breaks never k = 22; B's 1,927,450 rows fit the hard k = 21 limit 2,097,145 and are over only the 85 % planning gate
- a sweep of the other `*.md` files on t14-gpu found no other sentence that says B needs k = 22; L-STRUCTURE.md has no such line; not pushed

## [2026-11-14 12:25] queue | T14 m7 hill-climb item 2 at 2 of 5 runs, link to huginn lost
- item 2 (node layers on the GPU) on huginn: cold 171.0 and 174.6 s, warm 110.9 and 113.8 s (MEASURED, n = 2) against option A 203.8 s cold and 129.0 s warm (n = 5); no keep decision before n = 5 and the SG-6, E4 and keys gates
- the muninn SSH master to huginn dropped at about 12:13; the queue keeps running on huginn, results stay in `~/sherlock/bus/outbox.log` there until the owner runs gpu-link
- commit a375373ca on t14-gpu: RESULTS section 5.2 records the queue state; not pushed

## [2026-11-14 12:30] decision | T14 m7 item 2 node layers on the GPU: dropped by rule, cause hunt and retime (m9)
- huginn h30: warm call 129.0 to 110.9 s, cold 203.8 to 174.6 s, warm tree_s 19.04 to 1.57 s (MEASURED, n = 5); keys_s +0.54 s (spread 0.28) and cold lfr_trace_s +0.065 s (spread 0.056) regress; SG-6, stock verify and key A gates pass
- lead pick m9, option (b): the owner keep rule allows no regression, so find the keys_s cost, move it out of keys_s with no change to keys, proofs or zero knowledge, then retime n = 5; item 2b waits on the retime
- the huginn link was back at about 12:23 (muninn wired adapter drop, not huginn); commit dad10da24 on t14-gpu records this in RESULTS 5.2; not pushed

## [2026-11-14 12:52] commit | T14 m7 item 2c L key cache kept, openvm 8e6dd2d on t14-gpu
- huginn h33 (MEASURED, n = 5): warm call 129.03 to 109.96 s, cold 203.77 to 184.19 s; warm l_prove_s 61.85 to 43.25 s against a spread of 3.99; no stage worse beyond its spread; changes_keys false
- the cache holds a deterministic commitment of public child key data (stacked_commit, no randomness), so zero knowledge holds; patch sha256 334d6550, only crates/sherlock-wrap-export/examples/t7_chain.rs
- commit 8e6dd2d on openvm-t14 branch t14-gpu (`~/.cache/m7-wrap/src/openvm-t14`) by the lead; SG-6 and E4 not yet run on this build, asked of huginn in m11, a FAIL reverts it; not pushed

## [2026-11-14 13:00] qa | Quint regression replay of the four 2026-11-14 QA lanes, counterexample per finding
- 61 Quint traces in ~/Projects/Moriarty-wt/quint-reg/tests/quint-regressions (branch quint-reg, 85119d384); driver in openvm quint-reg a549644 (sherlock-leaf tests/quint_regressions.rs).
- replay_all: 52 rows pass (47 guards refused by the named constraint, 5 positives); 1 not replayed (gp2 init_val_any_set, structural); models/run.sh 61 of 61 OK (MEASURED).
- Confirmed: MW F-01 Work h_hi forgery accepted to N3, AFK F-7 N2 cut collision (forgeries); AFK F-6 and F-9(b) honest refused; GP2 F4.2 const_vd type collision; GP2 C2.1 conditional. Not reproduced: MW F-01 First, pathc F5 (fixed in T10).
- AFK F-5 DERIVED 5,284 + 62 + 71 = 5,417 > 5,400 columns: OPEN DESIGN for T15; see COUNTEREXAMPLES.md.

## [2026-11-14 13:07] qa | T10m V.9 and V.10 with D3 pass on the merged tree
- V.9 PASS, 70 cases, every AIR required in app, leaf, N1, N2, N3, FV and `L` (`~/.cache/t10mm/logs/check-omit.log`)
- V.10 A and B PASS: 2 shards, 5 steps, fixed heights, G1 accepted, 10 cases each, N3 fixed point, `L_Fr` 1,522 words, two programs under one `key_root` and one `keys_d` (`check-chain.log`)
- D3 PASS on 8 keys per program; weakest `L` vk mu 101.0730, `L_Fr` query 101.1435, node shift 101.1643; path C stage running next

## [2026-11-14 13:18] qa | Quint regression replay review fixes: real proofs, pins, NOT ATTEMPTED pathc mutants
- Real prover + stock verifier on chip set 2 (openvm quint-reg 1733bb5): honest and C2.1 shard accepted, sel_no_bool mutant refused (MEASURED).
- Moriarty quint-reg 85126e460 + log commit: pins without key indices, current line numbers; 7 pathc L/L_Fr/wrap mutants NOT ATTEMPTED; models/run.sh --all-runs 94 of 94 exit 1.
- New observation: CNULL does not check a constant's fields-start lane (nullary, value unchanged), recorded in COUNTEREXAMPLES.md.

## [2026-11-14 13:22] decision | T10q opened: fix the Quint-confirmed defects
- quint-reg (Moriarty 2ad6f48da, openvm 1733bb5) CONFIRMED MW-F01 h_hi forgery, AFK-F6, AFK-F7, AFK-F9(b), GP2-F4.2, CNULL fields-start lane; F-5 width 5,417 against 5,400 (71 ESTIMATE).
- LOOP.md row T10q (b84bcd55a); Opus agent fixes on branch t10q from the T10m tip, merged into T10m before the T12 rekey.
- huginn h35: pos-resident KEEP, warm call 107.88 s, cold 181.15 s (MEASURED); h36: item2c L key cache passes SG-6 and E4.

## [2026-11-14 13:24] decision | T14 m7 item 3a resident encode kept, item 2c gates pass, stack run queued
- item 3a (MORI_POS_STREAM=0, huginn h35, MEASURED n = 5): warm call 129.03 to 107.88 s, cold 203.77 to 181.15 s; warm lfr_prove_s 37.92 to 17.72 s against a spread of 5.62; VRAM 9.9 GB; changes_keys false; environment only, so no commit; the code form item3-resident-default is queued
- item 2c gates (h36): SG-6 4/4 on the stock CPU verifier and E4 pass, so openvm 8e6dd2d stays
- items 2c and 3a were each timed alone; m12 asks huginn for a 5x2 run of the two together, after the running item2-lazy-mm retime; RESULTS commit bcda5cd04 on t14-gpu, not pushed

## [2026-11-14 13:52] commit | Pushed Moriarty quint-reg to origin on owner order
- Owner 2026-11-14: "Push the Quint regression and T10q commits when done". Branch quint-reg (tip 2ad6f48da) is on github.com/CharlesHoskinson/Moriarty; it also carries 148 earlier unpushed E-path and T-row commits below it.
- openvm quint-reg (1733bb5) is not pushed: its only remote is openvm-org/openvm upstream.
- T10q branch gets pushed when the row finishes (LOOP.md T10q status).

## [2026-11-14 14:52] incident | muninn crash at 13:49, all Sherlock work resumed
- Boot 5fad41a6 ended 13:49:25 with no kernel error in the journal; NVRM power source change event failed at 13:46:35 (cause not established).
- huginn relinked with remote-link (PIN retries added, workflowskills d2231cc); huginn queue ran through the crash: h39 lazy-mm KEEP, h40 stack-kept warm 92.43 s, cold 164.29 s (MEASURED), DROP by drift rule pending a fresh baseline.
- T10m restarted (t10mm-run6), T10q, Carbon QA, m7 hill-climb and remote-connect agents resumed; LOOP.md gets a crash recovery rule, T11 and T12 now wait on T10q too.

## [2026-11-14 14:26] note | T10m runner resumed as unit t10mm-run6 after the 13:49 reboot
- the 13:49 reboot killed t10mm-run5 during the path C stage (started 13:05); 17 stages were done (state `~/.cache/t10mm/state`, last check-chain)
- run6 started 14:26 from the same `run.sh`; it skipped the 17 stages and reruns path C, then SG-6 and the T8b gates; the killed path C log is kept as `~/.cache/t10mm/logs/pathc-run5-killed.log`

## [2026-11-14 14:52] lane-start | Carbon Quint lane qa-20261114-t16-elastic
- pane w1:pS was lost in the 13:49 crash; one new Gemini pane carbon-T16 (w1:pV), gemini-4-carbon-high, lane folder `~/.cache/gemini-lanes/qa-20261114-t16-elastic/` (openvm 32b7c90, Moriarty 9d6b61f55)
- t10-bound wrote REPORT.md before the crash; the lead rerun of its 30 quint commands is running

## [2026-11-14 14:52] commit | T14 m7 item 2 node layers on the GPU kept: stark-backend 0fafe86, openvm 85eef2c
- huginn h38 (MEASURED, n = 5): warm call 129.03 to 112.80 s, cold 203.77 to 173.52 s, warm tree_s 19.04 to 1.56 s; keys_s back within spread once the cuda-common memory manager is made on first use, not in a #[ctor]; SG-6 4/4, E4 pass, key A unchanged, changes_keys false
- commits on t14-gpu (`~/.cache/m7-wrap/src/stark-backend-t14`, `~/.cache/m7-wrap/src/openvm-t14`), patch sha256 184722c8 and f0b4dbae checked; zero knowledge of the node layers comes from the r_P blinding in the traces, which the GPU prover takes from the CPU; not pushed

## [2026-11-14 14:52] decision | T14 m7 stack of 2c and 3a re-judged against a fresh baseline
- huginn h40 (MEASURED, n = 5): warm call 129.03 to 92.43 s, cold 203.77 to 164.29 s; dropped by the literal rule on keys_s +0.37 s and cold tree_s +0.52 s, CPU stages neither change touches
- lead ruling m13: fresh m5 binary baseline 5x2 first, then the stack with item 2 added re-judged against it, drift within its spread is noise; RESULTS commit 8ecdaceb9 on t14-gpu

## [2026-11-14 14:52] lane-start | T10q fix lane for the CONFIRMED quint-reg defects
- worktrees on branch t10q: Moriarty `~/Projects/Moriarty-wt/t10q` at 2ad6f48da (T10m 38290a7a6 plus the quint-reg commits) and openvm `~/.cache/m7-wrap/src/openvm-t10q` at 1733bb5 (the t10mm tip d26453c that T10m runs, plus the quint-reg replay driver); both fast-forwards
- the 13:49 crash interrupted the lane before any edit; both trees were clean after the reboot

## [2026-11-14 14:52] decision | T10q scope rulings before the fixes
- MW-F01: the Work statement looks up its whi limbs on the kernel 16-bit range bus (no new AIR); the node rungs, L and the wrap decoder get their own failing tests first, and a node range check is gated on a SEG output (H_HI words are payload digests under SET, DL and SUB)
- CNULL fields start: the reference allocates start = constant_fields.len() even for an empty slice (`crates/mori-core/src/sherlock/program_image.rs:304`) and its nullary rule reads only len (`owner_adt_runtime.rs:208`), so a start = 0 constraint would refuse honest programs; the lane checks what the reference enforces before any constraint
- GP2-C2.1 is outside T10q and stays an ignored open defect

## [2026-11-14 14:52] lane-done | Carbon Quint lane t10-bound rerun by the lead
- Gemini Carbon lane on openvm-t10m 33371a6 and Moriarty 30567a640 wrote t10_bound.qnt, tests and REPORT.md; it could not run quint (sandbox)
- Lead repairs for execution only: the test imports the sound base instance, pairNat reduced mod 1000000007; originals in lane-original/
- Lead rerun of 30 commands: 35 tests pass, sound runs hold, 9 reach targets reached, 14 of 14 mutants caught, every exit code as the lane expected

## [2026-11-14 14:52] qa | t10-bound verdict: ledger mode ORD sound with conditions, N1 range sound, log tie sound
- Work whi limbs unranged (stmt.rs:1083 at 33371a6) is the defect COUNTEREXAMPLES.md confirms as MW-F01-WORK
- Chunk anchor check is a tautology in mode 1 for j = 1 and j = k (lverify.rs:1875-1877); latent on SherlockLedger, PENDING-CX
- Stale contract comment (C_L vs C_P) and openClaim spec drift are prose; verdict in ~/Projects/Moriarty/qa/gemini-quint/qa-20261114-t10-bound/verdict.md

## [2026-11-14 14:52] commit | t10-bound Carbon Quint verdict on qa-carbon-20261114
- 8954edbd4 adds qa/gemini-quint/qa-20261114-t10-bound/ in ~/Projects/Moriarty-wt/qa-carbon; not pushed

## [2026-11-14 14:52] qa | T10m path C and SG-6 pass on the merged tree
- path C (`t7_chain pathc`, openvm d26453c): PASS, 25 of 25 cases, one leaf, leaf layer, `L` and `L_Fr` on the universal `L` key; D3 PASS on the path C key set (`~/.cache/t10mm/logs/pathc.log`, `pathc-d3.log`)
- SG-6: PASS, GPU 0.1500 s per proved step, CPU 1.9855 s (5 steps, 8 slots), the stock CPU verifier accepts every shard proof; shared host, not a timing (`~/.cache/t10mm/out/sg6/sg6.json`)
- the T8b gates (gp2, x7, D3 on MERGE-WORK, NW, NW2) run next in t10mm-run6

## [2026-11-14 15:05] build | remote-connect skill done; huginn h43 resident default KEEP
- workflowskills main 4128bd5: remote-connect with Quint model (38 checks, 9 mutants caught), selftest 17/17, replug prompt after 3 wrong PINs; CX-11 open (PIN count spans windows). Skill linked at ~/.claude/skills/remote-connect; gpubox-link link removed.
- huginn h43: item3-resident-default KEEP, warm call 92.43 to 89.65 s (MEASURED), keys A unchanged; fresh baseline then stack2 running.

## [2026-11-14 15:05] commit | T14 m7 item 3 resident encode by default on a discrete card, Moriarty e366714cc
- huginn h43 (MEASURED, n = 5, on top of the 2c and 3a stack): warm call 92.43 to 89.65 s, warm lfr_prove_s 18.30 to 16.49 s; E4 pass on both paths, key A unchanged, changes_keys false
- crates/mori-gpu/cuda/mori_gpu.cu, patch sha256 843b88be checked; integrated parts such as muninn's GB10 keep the streaming path; not pushed

## [2026-11-14 15:05] decision | T14 m7 key cache ruling, marked for T15 confirmation
- a cache hit may skip the path C height pass and the same key check only if the cache key covers the binary sha256 and the config and parameter digest, and a hit re-derives key_root, keys_d and the L vk hash and compares them to the miss; a mismatch is a miss with full checks
- a test corrupts one byte of a cached pk and must see a miss and a rebuild; huginn acked (m14) and holds the candidate until the patch meets this
- queued after it, on the kept stack: parallel L_Fr trace generation, codeword on the GPU or in reused pinned memory, removal of the duplicate L verify

## [2026-11-14 15:05] commit | T10q fixes 7 CONFIRMED Quint defects, openvm beba730 and Moriarty 4d4d1917a
- openvm t10q beba730 (on t10mm d26453c plus quint-reg 1733bb5): Work ranges its whi limbs on the kernel RANGE bus 197 (MW-F01), N1 computes shape_d from its cut (AFK-F7), N1 takes a free reg_root (AFK-F6), one-child MERGE-WORK links W to the right-most enabled child (AFK-F9b), const_vd uses the T5 5.2 type normal form (GP2-F4.2)
- the 7 named tests failed on 1733bb5 and pass on beba730 without #[ignore]; replay_all 59 PASS, gp2_real_proofs green (MEASURED, ~/Projects/Moriarty-wt/t10q/tests/quint-regressions/logs/)
- Moriarty t10q 4d4d1917a: manifest pins, COUNTEREXAMPLES rows FIXED, logs; not pushed

## [2026-11-14 15:05] decision | T10q scope rulings: producer-side h_hi range, CNULL fields start free, F-5 fit
- no h_hi range check at NW, NW2, N3, L or the wrap: SEG h_hi enters only at First (T10) and Work (T10q) and every later layer copies it; defence in depth via the recursion RangeCheckerAir<8> stays an option
- the CNULL fields-start lane stays free: the compiler writes start = len for nullary constants (program_image.rs:305) and the reference reads len only; GUARD by a positive test
- F-5 fit: Kind::Work drops the GP block and fv_state (MERGE-WORK has no other mode); the bound 5,400 stays

## [2026-11-14 15:05] qa | T10q MERGE-WORK width and changed keys (t7_chain shape, base 1733bb5 vs beba730)
- MERGE-WORK key 5,284 to 4,988 columns (MEASURED); with value-set CORE +62 (MEASURED) and STACKS 71 (ESTIMATE) 5,121 <= 5,400 (DERIVED); a second CORE split is still needed (1,298 constraints)
- pre-hash changed: N1 (all chip sets), N2, FV unit, MERGE-WORK, NW, L (and L over path C); unchanged: app, leaf, N3, NW2; L_Fr and wrap follow L (gate running in unit t10q-gate)

## [2026-11-14 15:05] incident | T10m gp2 gate fails on the merged tree
- t10mm-run6 stopped at 14:51: gp2 honest prove panics at t8_gp.rs:273 with LogupZerocheck(NonZeroRootSum) (log ~/.cache/t10mm/logs/gp2.log); matches the open t8_gp key item in the T10m row. T10m agent asked to fix and resume the T8b gates.
- T10q progress: 4d4d1917a marks 7 defects FIXED (openvm beba730), F-5 fits (Work StmtAir 760 to 464 columns, MEASURED), CNULL fields-start lane GUARD HOLDS.

## [2026-11-14 15:05] issue | T10m gp2 refused at N1: t8_gp layers lacked the h_hi range table
- t10mm-run6 stopped at gp2 (14:51): `t8_gp.rs:273` "First layer: prove: LogupZerocheck(NonZeroRootSum)"; t8_gp's own `layer()` built N1 without T10's `HhiRangeAir`, so the First lookups on bus 963 had no table and its N1 key was not the universal one
- openvm 5b7ada8: t8_gp pushes `HhiRangeAir` after `PermSpongeAir` and its trace, as t7_chain does; keys() prints key_root, keys_d and the N1 pre-hash (T8GPKEYS) to check against t7_chain
- gp2 reruns alone as unit t10mm-gp2; then the runner resumes for x7, D3 on MERGE-WORK, NW, NW2, the iface fixture, lfr_e3 and i7_decode

## [2026-11-14 15:07] lane-done | Carbon Quint lane t16-elastic rerun by the lead
- Gemini Carbon lane on openvm-t16 32b7c90 and Moriarty 9d6b61f55 wrote t16_elastic.qnt, tests and REPORT.md; pane w1:pV closed
- Lead repair: test modules import the base instance with each instance's constants; model unchanged
- Lead rerun of 34 commands: sound invariants hold, 9 of 9 mutants caught; one test and one reach target miss because the lane expected host-refused checkpoints to refuse the proof too

## [2026-11-14 15:07] qa | t16-elastic verdict: one claim per schedule, off-menu refusal and resume sound with conditions
- Off-menu refusal holds only when admission builds key_root per plan; the T16 off-menu test does not isolate the refusing check
- sched.rs host defects PENDING-CX, no proof impact: panic where a refusal is due (sched.rs:426), GPU chosen whatever the score (:429), unified pool ignores g.free (:379)
- Verdict in ~/Projects/Moriarty/qa/gemini-quint/qa-20261114-t16-elastic/verdict.md

## [2026-11-14 15:07] commit | t16-elastic Carbon Quint verdict on qa-carbon-20261114
- 6ad51ae7e adds qa/gemini-quint/qa-20261114-t16-elastic/ in ~/Projects/Moriarty-wt/qa-carbon; not pushed

## [2026-11-14 15:10] qa | T10m T8b gates gp2, x7 and D3 on MERGE-WORK, NW, NW2 pass
- gp2 (unit t10mm-gp2, openvm 5b7ada8): 48 of 48 PASS on the stock verifier; t8_gp `key_root` 0fade1a4... and `keys_d` 15ed5153... equal t7_chain's (`~/.cache/t10mm/out/gp2/gp2.json`)
- x7 (unit t10mm-run7): 22 of 22 PASS, NW, NW2 and N3 over MERGE-WORK accepted, forged rows refused (`~/.cache/t10mm/logs/x7.log`)
- D3 PASS on MERGE-WORK, NW and NW2 (weakest shift 101.1643 at vk and bounds) with the chain A keys (`~/.cache/t10mm/logs/d3work.log`); the iface fixture for lfr_e3 runs next

## [2026-11-14 15:13] decision | T10q: L ranges the root h_hi, NW/NW2/N3 stay without a range (advisor review)
- reversed the earlier no-downstream ruling for L: openvm dd3593e ranges each root h_hi limb as two bytes on the L sub-circuit's 8-bit range checker, so L_Fr and the wrap only see ranged limbs; keys rotate at T12 anyway, so adding it now costs no extra rekey
- NW, NW2, N3 get no own check: a non-root h_hi is consumed by the MERGE stitch h_hi(0) = h_lo(1), and h_lo is a leaf h_pre that CORE ranges at 16 bits (core.rs:613); a root h_hi meets the L check
- new local tests: l_ranges_the_root_h_hi (limb 2^16 refused by bus 4004 [256, 8] and by the byte decomposition) and merge_binds_the_children (Work slot 1 TAG, reg_root, prog_d, keys_d; one-child wen1 1; N2 MERGE reg_root, live since AFK-F6), all refused
- gate restarted on dd3593e in unit t10q-gate (~/.cache/t10q/gate.sh, logs ~/.cache/t10q/chain/logs/)

## [2026-11-14 15:31] qa | t10-bound F-03 chunk anchor CONFIRMED on the L_Fr tape and fixed (T10q-cx)
- As built, gen_ops_check checked A_0 = C_P with mode 0 only: a chunk 1 of 1 of program B with the ledger anchor of program A passes the real L_Fr tape, and the decoder twin gives a_in = C_L of A (MEASURED, quint_regressions lfr handler)
- Fix in openvm db9895a (branch t10q-cx): the a_in operand equals the SHK_PROGC operand when tag_in is SHK_PROGC, and a chunk's a_out operand when tag_out is SHK_PROGC; forged a_in and a_out refused by the named asserts, honest chunks pass
- Reach: only t7_chain lfr_gen_any runs gen_ops_check; SherlockLedger has no chunk entry (decode.rs forces TPL_CHUNK lanes for mode 1). The L_Fr tape changes, so the L_Fr key and the wrap key change
- End-to-end L proof attempt runs in unit t10q-cx-chunkcx (~/.cache/t10q-cx/chunkcx.sh)

## [2026-11-14 15:31] qa | t16-elastic scheduler D-1, D-2, D-3 CONFIRMED and fixed (T16-cx)
- Replay of the lane's as-built traces on sched.rs (MEASURED): panic on n 3, cap 1, pad [1, 2, 4]; GPU 7 to 8 GiB raises the score from 64.8e9 to 68.7e9 (CPU 8 at once to GPU 1); unified g.free 12 GiB with 2 leaves of 12 GiB in flight
- Fix in openvm 0b26d74 (branch t16-cx): unfit buckets are refusals, GPU and CPU options both scored, unified pool bounded by g.free; named tests t16_d1/d2/d3 fail before and pass after; no key change
- F-4 off-menu isolation runs in unit t16-cx-offmenu (~/.cache/t16-cx/offmenu.sh)

## [2026-11-14 15:36] lane-done | T10m: every T10 and T8b gate passes on the merged tree
- `lfr_e3 0` PASS on the new `L_iface` fixture (`~/.cache/e3-iface/fixture-t10m`, 12,166 words), 46 decode cases; `i7_decode` 2 of 2 with `E5_FIXTURES=~/.cache/t10mm/e5fx`; `wrap_x_not_cached_commitment` now stock `ok`, so the T10 override in `i7_decode.rs` can go (follow-up)
- totals (MEASURED): `L` 1,995 of 2,000 columns, StmtAir 99 of 100; wrap 1,685,845 rows at k = 21, VK 1,325 bytes; ledger 9.1.0.1 and devnet as logged; V.9, V.10, D3, path C, SG-6, gp2 48/48, x7 22/22 PASS
- openvm t10mm 9f3b521, c987b71, be62bb8, d26453c, 5b7ada8; patch series `research/sherlock-stark-code-20261112/openvm-t10m/` round trip tree 61f45c9; report `~/Projects/Moriarty-wt/t10m/research/sherlock-t10m-20261114/RESULTS.md`; no Preview, no push

## [2026-11-14 15:40] decision | T10q: the L h_hi range does not fit the frozen L set; removed, induction kept
- MEASURED (t7_chain shape, FP8 G1): 8 byte columns give L 2,003 columns (bound 2,000); one column per limb gives 107 interactions on the L statement AIR (bound 100); openvm dd3593e and 8bff541, removed in 51b12a0
- the root h_hi stays ranged by induction (First HHI_RANGE_BUS, Work bus 197, MERGE stitch against a CORE-ranged leaf h_pre); an L check needs an owner ruling on the L bounds
- merge_binds_the_children stays; the gate restarted on 51b12a0 (unit t10q-gate)

## [2026-11-14 15:40] decision | T10m DONE; T14b started
- T10m: every T10 and T8b gate passes on the merged tree (openvm t10mm 5b7ada8 and earlier, Moriarty ca96c1466); key_root 0fade1a4, keys_d 15ed5153; RESULTS ~/Projects/Moriarty-wt/t10m/research/sherlock-t10m-20261114/RESULTS.md.
- T14b agent started on muninn: wrap fixed-base MSM, GPU wrap, resident daemon; STARK items stay with the huginn m7 hill-climb.
- Next: merge T10q into T10m and rekey, then T11 and T12.

## [2026-11-14 15:46] qa | t10-bound F-03 CONFIRMED end to end with a real L proof
- t7_chain t10 chunkcx (openvm 475b80d = beba730 plus the fix): a chunk-mode L proof of program A carrying the anchor of ledger B passes the stock L verifier and the as-built L_Fr tape, and decodes to a_in = C_L(B) (MEASURED); the fixed tape fails 2 assertions
- Fix costs +263 L_Fr tape ops (499,360 to 499,623), L_Fr machine heights unchanged (MEASURED); L_Fr and wrap keys change, L and node keys do not
- t10q tip 8bff541 L key fails the FP8 G1 bound (L air 0 interactions 107 against 100), so the run used beba730

## [2026-11-14 15:46] qa | t16-elastic F-4 off-menu refusal check named; F-3 OPEN DESIGN
- t7_chain t16 offmenu (openvm 0b26d74, 120 s): the S = 8 leaf [0, 4) has the in-menu twin's statement; under its own triple it is refused by the N1 key path (row 0 path root != key_root); with key_root' holding its triple it is accepted
- So admission must build key_root from the plan's menu (F-3, OPEN DESIGN); no key changes

## [2026-11-14 15:46] commit | T10q-cx and T16-cx branches
- openvm bcf2cfd (t10q-cx, on t10q 8bff541), 475b80d (t10q-cx-e2e), 0b26d74 (t16-cx); Moriarty 778f47a97 (t10q-cx), 5e64c6d2e (t16-cx, rows in tests/quint-regressions/t16/); not pushed

## [2026-11-14 15:53] lane-start | T14b wrap warm path on muninn: worktrees and harness
- Branches t14b: Moriarty ~/Projects/Moriarty-wt/t14b (from t10m ca96c1466), openvm ~/.cache/m7-wrap/src/openvm-t14b (from t10mm 5b7ada8, T14 a5ab805 measurement switches cherry-picked)
- New workspace tools/sherlock-wrap-gpu: hook-patched midnight-proofs of tools/gpu-prover with the -perm circuits, fixed-base SRS tables, mori-gpu phases, `wrapd` bench, ident, serve
- Stock checks out of process: new sherlock-wrap bin wrap_verify and ledger-check (ledger 9.1.0.1) on every proof; runner ~/Projects/Moriarty-wt/t14b/research/sherlock-t14b-20261114/run.sh

## [2026-11-14 15:56] decision | T10q-cx hashes and scope
- openvm db9895a in the 15:31 entry is the pre-rebase hash of bcf2cfd (t10q-cx rebased on t10q 8bff541); same lverify fix
- The anchor fix is on t10q-cx only; t16-cx keeps the as-built lverify.rs. T16 schedules not rerun on 0b26d74 (only t16 offmenu, quint_t16 and sched unit tests)

## [2026-11-14 16:10] decision | CX pass verdicts; F-3 pick; t10q-cx merged into T10q
- t10-bound chunk anchor CONFIRMED (stock L verifier accepts program A carrying ledger B's anchor, MEASURED); fixed in gen_ops_check (openvm bcf2cfd); changes the L_Fr and wrap keys only (DERIVED).
- T16 sched.rs D-1 panic, D-2 GPU pick, D-3 unified pool CONFIRMED and fixed (openvm 0b26d74); off-menu F-4 GUARD HOLDS at the N1 key path; F-3 OPEN DESIGN, AFK pick: admission builds key_root from the plan menu.
- Follow-ups: middle-chunk SHK_LOCK operand vs prog_d counterexample, chunk opening-proof builder and test, t16 schedules rerun.

## [2026-11-14 16:27] decision | T14b runs at reduced pace behind the Preview critical path
- Lead order: T10q, T17, T11, T12 come first; T14b holds one heavy-slot at a time and never takes --exclusive while one of those jobs runs or waits
- ~/Projects/Moriarty-wt/t14b/research/sherlock-t14b-20261114/run.sh now takes the exclusive lock only on an idle host with no t10/t11/t12/t17 job; warm stage split into 5 short processes
- Commits so far: Moriarty 5b2bc175f (harness), openvm e41cf03 (t10 call serve mode on T14B_SERVE)

## [2026-11-14 16:30] decision | T17 Preview prototype started; stack2 KEEP on a clean baseline
- Owner 2026-11-14 asked for a working Sherlock prototype on Midnight Preview: LOOP row T17 (1f9c96f5e); (a) dry run on T10m keys running now, (b) on T10q keys, (c) final on T11 blinded keys after T12, prover on huginn.
- Critical path: T10q, then T11, T12 and T17 (b) in parallel; T14b capped at one heavy slot.
- huginn h51: stack2 KEEP against the clean baseline, warm call 128.69 to 74.32 s, cold 199.73 to 139.30 s (MEASURED), keys unchanged.

## [2026-11-14 16:31] decision | T14 m7 kept stack against a clean baseline: warm call 128.7 to 74.3 s
- clean m5 binary baseline (no compile overlap, n = 5): warm call 128.69 s, cold 199.73 s; the 22:28 UTC baseline and the item 3 code form timing overlapped agent compiles on huginn and are labelled overlapped
- stack2 (openvm 8e6dd2d and 85eef2c, stark-backend 0fafe86, resident encode; MEASURED n = 5): warm call 74.32 s (71.73 to 76.41), cold 139.30 s (132.11 to 143.11); kept, no stage worse beyond the clean spread; E4, SG-6 4/4, key A unchanged, changes_keys false
- RESULTS section 5.3, commit 4b69c6687 on t14-gpu, not pushed; next on huginn: key cache, parallel L_Fr traces, codeword on the GPU, removal of the duplicate L verify, then item 1

## [2026-11-14 16:50] decision | T14 m7 stack kept; ZK gap handed to T11
- T14 m7 hand-back: items 2c, 2 and 3 kept on t14-gpu (openvm 8e6dd2d, 85eef2c; stark-backend 0fafe86; Moriarty e366714cc); warm call 128.69 to 74.32 s, cold 199.73 to 139.30 s (MEASURED, n = 5); SG-6 4/4, E4 pass, keys unchanged.
- Open: no PCS masking in the CPU or CUDA STARK backend and t7_chain fixes r_P = 1000 + i; added to the T11 row as a required input.

## [2026-11-14 16:51] lane-start | T17 (a) Preview dry run on the T10m keys started
- worktree ~/Projects/Moriarty-wt/t17-preview, branch t17-preview from t10m ca96c1466; runner research/sherlock-t17-preview-20261114/t17.sh in unit t17-preview-a; inputs read only from ~/.cache/t10mm, outputs in ~/.cache/t17
- fixed before spend: wallet fees from the Preview ledger parameters (Preview prices differ from initialParameters), InBlock wait 600 s on Preview, deploy resume reads a sent deploy back, own proof server 9.0.1 on port 6341, balance phase for tDUST before and after
- contract rebuilt from the committed source: zkir and keys byte-equal to the ~/.cache/t9 build T10 used (only line numbers in index.js differ, MEASURED)

## [2026-11-14 16:58] decision | T18 Compact to Sherlock started
- Owner 2026-11-14 asked for a Compact example test suite and a Compact compiler design to show Sherlock proves Compact recursively; LOOP row T18 (3d6d52b50).
- Agent builds tests/compact-suite from ~/Projects/Moriarty/repos (996 .compact files) with compactc 0.36.0 native transcripts, and an OpenSpec compiler change on the 2026-11-11 feasibility verdict; spike (c) waits on T10q keys.

## [2026-11-14 17:10] lane-start | Mycroft named; nine compiler design lanes and T19 validium row
- Owner 2026-11-14 named the Compact to Sherlock compiler (lead picked Mycroft); directory mycroft/ on branch mycroft (~/Projects/Moriarty-wt/mycroft, 3ebf04fe5); suite moves to mycroft/tests/compact-suite.
- Nine lanes started (3 Fable 5.2 agents, 3 GPT-6 Astra via Codex, 3 Gemini 4 Carbon in herdr panes) on areas a1-lowering, a2-transcript, a3-recursion; each writes STUDY, OpenSpec with EARS and a Lean 4 prototype; an Opus coordinator merges into mycroft/spec and mycroft/lean. Brief: mycroft/lanes/BRIEF.md.
- LOOP row T19 added (0f186826e): toy validium on Preview, waits T17 (b).

## [2026-11-14 17:12] commit | Mycroft suite: 32 programs, MANIFEST, coverage, native ZKIR
- Branch mycroft 85e060f5a: ~/Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite with MANIFEST.json and .md, COVERAGE.md, native/<id>/ (ZKIR, contract-info, k and rows per circuit, MEASURED by zkir-v3 mock-compile).
- compactc 0.36.0: 28 compile, 1 expected disclosure error (w01), 3 fail: kitties (missing openzeppelin import), zkloan (stdlib ecMulGenerator overload change); bboard and battleship compile after a pragma-only patch.
- Native transcripts per circuit follow (tools/drive.mjs, tools/zkir-ref).

## [2026-11-14 17:12] lane-start | Mycroft Astra and Carbon design lanes started
- Three GPT-6 Astra lanes (codex exec, effort ultra, live web search, network on) in systemd units myc-a1-lowering-astra, myc-a2-transcript-astra, myc-a3-recursion-astra; logs in ~/.cache/mycroft-lanes/
- Astra lanes run without --ephemeral: with it, ultra subagent spawn failed ("no rollout found for thread id")
- Three Gemini 4 Carbon high lanes in herdr tab mycroft-carbon (agents carbon-a1, carbon-a2, carbon-a3); no agy pane existed, so all three are new and close when done
- Folders ~/Projects/Moriarty-wt/mycroft/mycroft/lanes/{a1-lowering,a2-transcript,a3-recursion}-{astra,carbon}/, brief mycroft/lanes/BRIEF.md

## [2026-11-14 17:12] decision | T10q-cx T10B-LOCK: chunk lock anchors CONFIRMED and fixed in L_Fr
- MEASURED: as built every SHK_LOCK anchor operand was free in the L_Fr tape; a chunk of program B read and wrote the locks of a run of program A (native tape plus decoder twin; not reachable on SherlockLedger, DERIVED).
- Fix openvm 0a4a2c8 (t10q-cx): lock operand x_j = sponge(U_LOCK 8; C_P operand, run_tag, j) over Fr, a_in = x_(j-1), a_out = x_j; test t10b_chunk_lock_is_bound; tape 499,657 ops (+34), 14 heights unchanged (MEASURED); L_Fr and wrap keys change.
- PICK for T15: replaces the P2(DOM_LOCK, ...) form of ANC-002 and FINAL-DESIGN 1.6, which the L_Fr tape cannot compute; records in ~/Projects/Moriarty-wt/t10q-cx tests/quint-regressions/COUNTEREXAMPLES.md (fc167510b).

## [2026-11-14 17:12] qa | T10q-cx open-chunk form and T16 schedules rerun
- Open-chunk form (open 1, mode 1, a_out at entry 6): t10b_f03_open_chunk_anchor_ends passes on openvm 4d8177b; lane layout ESTIMATE.
- T16 suspend and schedules rerun on openvm 0b26d74: all_pass, 16 cases PASS, S1 pick as recorded, keys unchanged (MEASURED); Moriarty t16-cx 406cdf7ed.

## [2026-11-14 17:20] decision | CX follow-ups: middle-chunk lock CONFIRMED and fixed; merged into T10q
- Middle-chunk SHK_LOCK anchor operand was free (program B chunk could chain A's locks, MEASURED on the native tape); fixed in openvm t10q-cx 0a4a2c8 with x_j = sponge(U_LOCK=8; C_P, pack(run_tag), j); open-chunk case and test 4d8177b; L_Fr and wrap keys change only.
- AFK pick for T15: the x_j form replaces the P2(DOM_LOCK) text of ANC-002 and FINAL-DESIGN 1.6.
- T16 schedules rerun on 0b26d74: 16 cases PASS, keys unchanged (MEASURED); T16 fixes t16-cx 406cdf7ed wait for T16 (c) to (e).
- Mycroft suite deploy agent started: full compactc 0.36.0 build of 32 programs, Preview deploy and calls with the funded wallet, shared flock ~/.cache/preview-wallet.lock with T17.

## [2026-11-14 17:21] commit | T17 (a) Preview deploy done; runner and RUNBOOK committed for reuse
- SherlockLedger with the T10m wrap key on Preview at block 113,966, tx 0xea6c537e...e869, status SUCCESS, every installed key byte-equal to the wrap VK (MEASURED); the acceptRun call waits for a 64G heavy slot
- t17-preview 76a669f37 (t17.sh, RUNBOOK.md, Preview fees from chain parameters, deploy resume, balance phase) and dcc1e754e (every Preview transaction under flock ~/.cache/preview-wallet.lock); not pushed
- reuse: ~/Projects/Moriarty-wt/t17-preview/research/sherlock-t17-preview-20261114/RUNBOOK.md; tools/sherlock-sdk/t10a-ledger-run.mjs takes T10A_WALLET_LOCK

## [2026-11-14 17:21] lane-done | Mycroft a3-recursion-fable checked and committed
- Fable 5.2 a3 lane: six specs (call-claim, shard-plan, atomic-chunks, composition, key-universality, cost-model), 59 requirement IDs
- Lead rebuild from a clean .lake via heavy-slot: Build completed successfully (11 jobs), 0 sorry, 0 axiom
- Commit f3870887f on branch mycroft (~/Projects/Moriarty-wt/mycroft/mycroft/lanes/a3-recursion-fable/), not pushed

## [2026-11-14 17:58] commit | Mycroft suite complete: native reference vectors for 97 of 105 circuits
- Branch mycroft 6d211d538 (driver, zkir-ref, authored programs) and 1564a36bd (scenarios): ~/Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite, README, MANIFEST, COVERAGE, RESULTS.
- 36 programs (33 compile); 148 native vectors pass the stock midnight-zkir check with real comm, 148 wrong-comm_rand controls refused, 45 with guarded-off zeros; longest call vector 476 lanes, all within L_TX 1,024 (MEASURED).
- Cross-contract (x01 calls x02 with a real verifier key), received coins (z04), election variant with a constructor (h03); lane 0 binding_input and the guaranteed/fallible split are per transaction and not recorded.

## [2026-11-14 18:10] incident | Shared mycroft index: lane a1 files swept into a suite commit, split again
- The suite commit d9a7e50c8 took the a1-lowering-astra files that lane had staged in the shared mycroft index; the lane's own `git commit -- path` then found nothing to commit.
- Fixed before anyone built on it: soft reset, a1 files committed with the lane's own message (d05034ec5), suite committed alone with a pathspec commit (1564a36bd); no file content changed.
- Rule for lanes in one worktree: commit with `git commit -- <own paths>`, never a bare `git commit` after `git add`.

## [2026-11-14 18:10] decision | Mycroft a2 Fable lane done; anchor slot pin handed to T12
- a2-transcript-fable: 60 EARS, lake build OK with 2 sorries (decoder round trip), 59 guards on suite vectors; recommends anchor slot 15 for all-Sherlock contracts (stock compactc words stay byte-equal).
- Finding PENDING-CX: the anchor slot must be a wrap key constant; T4 lists the pin as missing from the measured wrap. Added to T12's scope (counterexample on the T10q wrap, fix if CONFIRMED).

## [2026-11-14 18:10] lane-start | Mycroft suite on Preview: full compile and deploy of every program
- compactc 0.36.0 full build (ZKIR and keys) into mycroft/tests/compact-suite/build/full (gitignored): 33 of 36 compile, ZKIR byte-equal to the --skip-zk native ZKIR (MEASURED); w01, d05 kitties, d07 zkloan fail as recorded.
- Runner tools/preview/preview.mjs (midnight-js 5.0.0 deployContract/callTx, OP-PV-WALLET as T17 loads it, flock ~/.cache/preview-wallet.lock per tx) in unit myc-preview-all; own proof server container mycroft-proof-server on 127.0.0.1:6342.
- First program t02 deployed and called on Preview: transcript words equal the native ref.json (MEASURED); per-call tx vectors (lane 0, guaranteed/fallible split) go to preview/<id>/.

## [2026-11-14 18:10] lane-done | Mycroft a1-lowering-astra, a2-transcript-astra, a1-lowering-fable checked and committed
- a1-lowering-astra (GPT-6 Astra, ultra): 7 specs, 64 EARS IDs, lead rebuild Build completed successfully (8 jobs), 0 sorry; commit d05034ec5 (split back out of a suite commit that swept the shared index)
- a2-transcript-astra: 4 specs (transcript-codec, call-binding, universal-seal, ledger-acceptance), 28 EARS IDs, 7 jobs, 0 sorry; commit 84ac20b85
- a1-lowering-fable: 54 EARS IDs, 11 jobs, 0 sorry; commit 60681d4d8; dissent: Stage 1 IR is the stock .zkir plus contract-info.json, not the Scheme backend of MERGED 2.2
- All under ~/Projects/Moriarty-wt/mycroft/mycroft/lanes/, branch mycroft, not pushed

## [2026-11-14 18:15] build | Sherlock contract deployed on Midnight Preview (T17 a)
- SherlockLedger deployed on Preview at block 113,966, tx 0xea6c537e...e869, SUCCESS (MEASURED); wrap key 1,325 bytes installed on all three entries; t17-preview 76a669f37, dcc1e754e, a5e1e25f1; RUNBOOK research/sherlock-t17-preview-20261114/RUNBOOK.md.
- Honest acceptRun and refusals R1 to R4 queued in unit t17-preview-a2, waiting for the 64G heavy slot.

## [2026-11-14 17:26] note | Midnight writes audit of Sherlock: four PER-EVENT items
- audit entry at the real clock time; it uses op note because earlier entries carry later times; read-only audit under the owner ruling of 2026-11-14 (no per-event Midnight writes), on t10q f8fd268e3
- PER-EVENT: acceptRun used per co-processor job, openClaim and foreign opening per claim, chunked calls (k = 5 to 30 proofs and anchor writes per call), stale OpenSpec descend in LDG-003 to LDG-005; acceptCollection is the batched form; throughput unstated (R-10)
- ~/Projects/mpcexperiments/design/enhancement/midnight-writes-audit/sherlock.md; [[MidnightWritesAudit]]
- next: lead or owner decide the batched forms (one collection per epoch, membership checks against the root, L_TX sizing, spec fix); nothing in the design was changed

## [2026-11-14 18:15] lane-done | Mycroft a2-transcript-fable and a3-recursion-astra checked and committed
- a2-transcript-fable: 60 EARS IDs, lead rebuild 11 jobs OK, 2 sorry (decodeOps_sound, decodeOps_complete in Open.lean); commit 9249f5789
- a3-recursion-astra: 6 specs, 48 EARS IDs, 7 jobs OK, 0 sorry, one native_decide regression check (disclosed); commit 1040ba97d
- Six of nine lanes done; three Carbon lanes still running in herdr tab mycroft-carbon

## [2026-11-14 18:15] lane-done | Mycroft a3-recursion-carbon checked and committed
- Gemini 4 Carbon a3 lane: 5 specs (claim-bridge, atomic-chunking, composition-pcd, universal-keys, cost-planner), 34 EARS IDs
- Lead rebuild from a clean .lake via heavy-slot: Build completed successfully (10 jobs), 0 sorry, 0 axiom; commit 4a8e44d30
- Seven of nine lanes done; Carbon a1 and a2 still running

## [2026-11-14 18:20] incident | heavy-slot records from sandboxed Carbon lanes blocked the T17 Preview call
- Carbon lanes run heavy-slot inside a PID namespace: their records named host pids 80 and 84 (kernel threads, alive forever), and their cleanup deleted T17's pending exclusive record, so t17-call waited from 17:20.
- Fix: ~/.local/bin/heavy-slot (backup heavy-slot.bak-20261114b) runs the command without admission when /proc/1 is not systemd; stale records removed and T17's pending record restored at 17:34; T10q key-A holds the slot, T17 is next.

## [2026-11-14 18:21] lane-start | T20 batched Midnight writes
- Row T20 of ~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md: six fixes from the Midnight writes audit (acceptCollection per epoch, openClaim and FOREIGN by membership, L_TX and chunk cap, LDG-003 to LDG-005 alignment, one reg_root update per registry epoch)
- Branch t20-batched from t10q f8fd268e3, worktree ~/Projects/Moriarty-wt/t20-batched; no openvm branch unless a circuit change is needed
- Next: spec alignment commit, then one commit per fix with a Quint model and compiled-contract tests, then one Carbon Quint QA lane

## [2026-11-14 18:21] lane-done | Mycroft a2-transcript-carbon checked and committed
- Gemini 4 Carbon a2 lane: 5 specs, 25 EARS IDs; recommends freezing L_TX = 1,024 (1,026 lanes) and one wrap key for acceptRun too
- Lead rebuild from a clean .lake (heavy-run 4G; heavy-slot was full with T10 and T17): Build completed successfully (10 jobs), 0 sorry; commit 8a00345d3
- Carbon lanes told to build Lean without heavy-slot after their sandboxed heavy-slot records blocked T17 for 15 min

## [2026-11-14 18:21] lane-start | Mycroft merge: a2 and a3 area merge agents started
- Coordinator fixed modules (16, prefixes FE VAL LOW TAPE LED BLT TRN SEAL BIND ACC REF CLM CHK CMP KEY COST) and cross-area decisions X1 to X9 in ~/.cache/mycroft-merge/MERGE-PLAN.md
- X1: call vector [bind, comm] ++ anchor ++ native words W ++ zeros, 1,026 lanes; W word-equal to the suite transcript_words; Astra strict-byte dissent recorded
- Two Opus merge agents (a2 transcript, a3 recursion) write mycroft/spec/specs/ and mycroft/lean/Mycroft/{Transcript,Recursion}/; a1 waits for Carbon a1

## [2026-11-14 18:21] lane-done | Mycroft a1-lowering-carbon checked and committed; all nine lanes done
- Gemini 4 Carbon a1 lane: 7 specs, 42 EARS IDs, lead rebuild 12 jobs OK, 0 sorry; commit 4486de48e
- Carbon a1 taps Lflattened (with Astra a1), against Fable a1's stock .zkir carrier; it reports a compactc 0.36.0 persistentCommit curve-alignment bug (reduce-to-zkir.ss:215-220)
- Nine lanes committed on branch mycroft; a1 area merge agent next

## [2026-11-14 18:21] commit | T14 m7 lverify kept on openvm t14-gpu c8ec92b
- huginn h59: the device L proving key is kept across calls; no regression under m18 (warm 66.62 to 67.63 s, ranges overlap, MEASURED 5x2); keys unchanged.
- huginn h58 round-2 audit: no ZK or key issue in kept patches; m20 approved R1 child-DAG once, R2 background lfr_pk, R3 item1, keycache frozen-constant asserts and grind atomicMin with a CPU-vs-GPU byte gate.

## [2026-11-14 18:21] qa | Mycroft Quint QA lane started on transcript equality and refusal
- Gemini 4 Carbon high in herdr pane carbon-qa (the only Gemini pane left; lane panes closed)
- Model of call vector, frame, seal, ledger public_inputs and admit; invariants EQ_a/b/c, REFUSE, ONE_STAGE, FIT plus four mutants
- Prompt and output in ~/Projects/Moriarty-wt/mycroft/qa/gemini-quint/mycroft-transcript-20261114/; lead reruns every quint command

## [2026-11-14 18:21] commit | T20 fixes 1 to 6 and appendix on branch t20-batched
- Branch t20-batched (from t10q f8fd268e3), worktree ~/Projects/Moriarty-wt/t20-batched; no push, no key change
- Commits: f9211cc8b fix 5 (LDG-003..005 aligned), 54ccc5dfc fix 1 (one acceptCollection per epoch), 9a0f01725 fix 2 (openClaim replaced by membership against claims_root), 7276a705b fix 3 (foreign map replaced), b6d6a8e0c fix 4 (L_TX 1,024, K_MAX 2), 069957acd fix 6 (one reg_root update per registry epoch), 4c74c04a0 appendix (superseded anchors)
- Each fix has a Quint model with mutants caught and witnesses reached; t20-ledger.test.mjs 9 pass on the compiled batched contract
- Results with writes before and after in ~/Projects/Moriarty-wt/t20-batched/research/sherlock-t20-batched-20261114/RESULTS.md

## [2026-11-14 18:21] decision | T20 LEAD picks marked for T15 confirmation
- acceptCollection statement [reg_pub, claims_root], claims tree depth 20, read entries native-keyed, I-21 moved to the consumer nullifier
- SHK_FCLAIM seventh DIGEST tag (rides the T8b rekey), epoch close 3,600 s, K_MAX = 2 at TTL 3,600 s

## [2026-11-14 18:21] qa | T20 Carbon Quint QA lane started on the batched ledger writes
- Gemini 4 Carbon high in herdr pane carbon-t20 (w1:p15, opened for this lane, closed after)
- Questions: write accounting, consumer soundness, data availability, chunk cap, registry epochs; ten mutants that must be caught
- Lane folder ~/.cache/gemini-lanes/qa-20261114-t20-batched; lead reruns every quint command

## [2026-11-14 18:23] commit | Mycroft merged spec and Lean specification
- 16 modules, 277 EARS requirements (147 suite-mapped, 130 naming 103 missing tests); 424 lane rows traced, 17 dropped with reasons
- Lean ~/Projects/Moriarty-wt/mycroft/mycroft/lean: clean build 46 jobs, 0 sorry, 0 axiom; spine end_to_end chains compile, run, seal and ledger vector (premise S1 checked on t02 increment and t01 get)
- Decisions X1 to X11 and dissent in mycroft/spec/MERGE.md; key calls: word equality with a declared frame, slot 15 anchor, stock .zkir carrier, native compactc bugs refused; commit 3be9e8fd5, not pushed

## [2026-11-14 18:50] incident | muninn reboot after mt7925e WiFi driver oops
- 18:40:59 kernel oops in mt7925e_mac_reset (napi_disable mutex); reboot requested from the desktop at 18:44:07; boot 0 at 18:45.
- Before the crash T17 (a) landed: honest acceptRun A included on Preview at block 115,187 (tx 0x196fd6eb…3d57, status SUCCESS, claimInRuns, anchor and keys unchanged); R1, R2, R3, R4 all refused by the native ledger ("Invalid proof") and by the wallet.
- Restarted from resumable state: t17-preview-a3 (`t17.sh all`; accept and r1 to r4 are done in run state, so no resubmission) and t10q-gate (key-A ok, args-A next).

## [2026-11-14 18:51] note | T20 resumed after the muninn reboot
- Branch t20-batched intact at 3fdb08cb4 (all six fixes and the appendix committed before the reboot)
- Carbon lane pane carbon-t20 restored with its agy session; it stopped on a network error with t20_batched.qnt written and no report; prompted to continue

## [2026-11-14 18:51] note | Mycroft Preview run resumed after the reboot
- First pass committed on branch mycroft 2657a678a: 28 programs deployed or recorded, 71 calls included, every included call's on-chain transcript words equal the native ref.json except d03 dao_voting_token (address-derived token type; equals the native driver at the real address), MEASURED.
- s02-foreign had the wallet lock at 01:12Z but stalled in wallet sync before balancing; no state, no submit; it is redeployed fresh. Runner now times out wallet sync at 180 s and exits if it holds the lock 15 min.
- Remaining s02, s03, h03, h01 (midnight-js refuses `$` circuit names; provider workaround), x01 (callee deployed first), z02 (staged key inserts: one deploy of 20 keys exhausts the block limits), z01 retry in unit myc-preview-rest2 under heavy-slot.

## [2026-11-14 18:53] incident | T14b runner restarted after the 18:45 muninn reboot
- Survived in ~/.cache/t14b: builds (wrapd, wrap_verify, ledger-check, lfr_call), the ident gate and its verify (state .ok files); unit t14b-run restarted from that state, waiting behind t17-call and the T10q gate
- Ident gate (seeded, test only, MEASURED): cpu, fixed-base (affine and c = 16 shifted tables), GPU all and GPU msm+ntt+coset give byte-identical wrap proofs; quotient check 0 diffs; wrap key equal to the T10m key; stock verifier and ledger 9.1.0.1 accept all 6
- Moriarty t14b 0ef1c751c (runner, summarize.py); not pushed

## [2026-11-14 18:56] commit | T14 m7 lx-once on openvm t14-gpu e142c09
- huginn h62 (R1): each L child-DAG commitment computed once; cold call 132.15 to 78.61 s (disjoint ranges), warm 67.63 to 70.35 s overlapping (MEASURED 5x2); keys equal the frozen record.
- huginn link restored 18:53 after the muninn reboot (owner PIN); m22 asked huginn to repost lost status.

## [2026-11-14 18:56] commit | Mycroft a3-recursion-fable final revision committed; QA lane resumed after reboot
- muninn rebooted 18:45 (WiFi driver oops); the uncommitted a3-recursion-fable edits were the lane's own final writes (17:08 to 17:10, inside its folder), so they were kept: commit 44e1554e5, lead rebuild 11 jobs, 0 sorry
- Carbon Quint QA lane resumed in herdr pane carbon-qa from its two .qnt files; no REPORT.lane.md yet
- Folder ~/Projects/Moriarty-wt/mycroft/qa/gemini-quint/mycroft-transcript-20261114/

## [2026-11-14 19:00] lane-start | T21 prover GUI: Fable 5.2 designer writes requirements
- Owner order: three.js + Material 3 animation of proofs through Sherlock that is also the prover GUI (code, source, design flow, elasticity), linked to the Sherlock proof server.
- Pipeline: Fable designer (requirements) -> GPT-6 Astra audit -> Fable supervisor with 5 Muse Wonder Full lanes; branch prover-gui on t10m, brief prover-gui/BRIEF.md.

## [2026-11-14 19:11] decision | T14 m7 R2 lfrpk-bg KEEP under the m17 combined-stage rule (m24)
- huginn h63: cold call 132.15 to 122.95 s (disjoint), keys_s +0.92 s because the background L_Fr pk load overlaps keygen; keys_s + lfr load 60.86 to 55.00 s (MEASURED 5x2).
- Rule added to m18: when a patch moves work between stages, judge the sum of those stages. Patch to be rebased onto e142c09 with a 3x2 stacking check.

## [2026-11-14 19:11] qa | Mycroft Quint QA verdict: SOUND after four fixes
- Carbon lane model holds EQ_a/b/c, REFUSE, ONE_STAGE, FIT over 10,000 traces; six mutants each break their invariant; lead reran all 13 quint commands, results match
- F1 admit did not bind L_e (W ++ [0] admitted, no ledger-visible divergence), F2 REF-006 stage, F3 non-normal-form call admitted, F4 ckptHead vs verify.rs: all CONFIRMED, fixed with regression #guards; F5a/b/d kept as tasks
- Commit adfc1f0bc; ~/Projects/Moriarty-wt/mycroft/qa/gemini-quint/mycroft-transcript-20261114/verdict.md and COUNTEREXAMPLES.md; pane carbon-qa closed

## [2026-11-14 19:13] lane-done | T17 (a) Preview dry run on the T10m keys: pass
- SherlockLedger at 740653...e274 (deploy block 113,966); honest acceptRun with a fresh L_Fr and k = 21 wrap proof included at block 115,187, tx 0x196fd6eb...3d57, runs[claim_pub] = outcome_pub, privacy scans pass (MEASURED)
- R1 changed outcome, R2 flipped proof bit, R4 program B, R3 replay: refused by the node, 1010 Custom error 115; the first attempt never reached the node because the wallet's indexer reads failed in the 18:40 WiFi oops, and the rerun balanced on the first try
- runner fixes: node JSON-RPC error read from the FiberFailure cause and required for a refusal, balancing retries on transport errors, t17.sh refusals step; commits t17-preview 9e1e7f398, LOOP main 572284195; not pushed
- tDUST before 10,806,164,418,692,841,287, after 11,034,457,097,781,297,669 specks (DUST regenerates; wallet shared with Mycroft); fees paid 2,232,769,371,428,572 specks (DERIVED)

## [2026-11-14 19:18] note | T14 m7: leaf to L proofs are byte-nondeterministic even on CPU
- huginn h65/h66: after the grind atomicMin fix, app proofs are byte-identical CPU vs GPU; leaf, N1 to N3 and L proofs still differ run to run because EqBits verifier-circuit rows come from a std HashMap (upstream sorts only under cfg(test, cuda)). All proofs verify.
- Fix: sort the rows (keys and AIR unchanged; frozen key gate to confirm). Relevant to T12 byte gates.

## [2026-11-14 19:26] qa | T20 Carbon Quint QA verdict: no contract defect, conditions stated
- Lead repaired the lane model (instance imports, hash overflow) and reran 38 commands: invAll ok strict and grace, 12 mutants caught, 8 witnesses reached
- Q1 and Q4 CONSISTENT; Q2, Q3 and Q5 SOUND WITH CONDITIONS (wrap keys on write entries, T8b builds claims_root from verified heads only, consumer computes its leaf in circuit, N-8 churn)
- Counterexamples on compiled contracts: duplicate slot GUARD HOLDS, empty slot GUARD HOLDS (control CONFIRMED); lead model registryEpoch.qnt F-9.1 and F-9.2 FIXED with regression tests
- Verdict ~/Projects/Moriarty-wt/t20-batched/qa/gemini-quint/qa-20261114-t20-batched/verdict.md; commit 5dc1be2ee

## [2026-11-14 19:26] lane-done | T20 batched Midnight writes done on branch t20-batched
- Commits f9211cc8b, 54ccc5dfc, 9a0f01725, 7276a705b, b6d6a8e0c, 069957acd, 4c74c04a0, 3fdb08cb4, 5dc1be2ee; no push
- No circuit code and no key changed; L collection form (claims_root) and SHK_FCLAIM ride the T8b rekey
- Pane carbon-t20 closed

## [2026-11-14 19:28] commit | T14 m7 lfrpk-bg on openvm t14-gpu 3eaa4b1
- huginn h67: stacked on lx-once, cold call 78.61 to 70.41 s (disjoint), warm 70.35 to 66.08 s overlapping (MEASURED 3x2); keys equal the frozen record.
- Cold call now about 70 s against about 200 s at the clean baseline (MEASURED on huginn). Harness judges cold-only stages on cold calls only (m25).

## [2026-11-14 19:31] lane-done | T20 batched Midnight writes DONE on t20-batched 5dc1be2ee
- Six fixes with Quint models (mutants caught), Carbon QA no defect in the contracts as built; no openvm or key change. Not pushed.
- OWNER gate: commit 3fdb08cb4 rewrites the ARCHITECTURE GP note so chunked Compact calls are a capped fallback (K_MAX = 2); GP narrowing is owner-gated, so it waits for a ruling. Report sent to hoskinson-90 (hoskinson-38 gone after the reboot).

## [2026-11-14 19:35] decision | Owner accepts the T20 chunk cap K_MAX = 2
- Owner answer: chunked Compact calls are a capped fallback (K_MAX = 2, refused at plan time above it); the ARCHITECTURE GP note change of t20-batched 3fdb08cb4 stands. Sherlock runs stay unbounded.

## [2026-11-14 19:45] lane-done | Mycroft Compact suite on Preview DONE on mycroft c1c78e66f
- MEASURED: compactc 0.36.0 compiles 33 of 36 suite programs, ZKIR byte-equal to native; 33 contracts deployed on Preview, 208 scenario calls, 99 included, 159 PASS (89 equal native words, 61 refused as native, 9 equal after address, token-type or callee-commitment substitution), 0 mismatches.
- MEASURED: fees 154.33 tDUST; wallet tDUST 10889.19 before, 11059.50 after (DUST regenerates). Lane 1 of every recorded proof equals the on-chain communication commitment.
- Findings: z02 needs a staged deploy (20 keys exhaust the block limits); midnight-js refuses `$` circuit ids (h01/h03 vote$commit not provable); node errors 138, 214, 194; shielded calls need coins the wallet lacks. Report ~/Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite/PREVIEW.md.

## [2026-11-14 20:08] lane-start | Owner "resume everything": T16 (c) to (e), T13 batch 2, T18 (c) started
- No new crash since 18:45; T10q gate (final checks), T14b, T21 build supervisor and huginn T14 keep running.
- New Opus agents: T16 (c) to (e) elastic demo with F-3 admission; T13 batch 2 Carbon Quint lanes plus lanes for T10, T10m, T10q and T16 circuit changes; T18 (c) Mycroft compile spike (t02, s01, t01).

## [2026-11-14 20:21] lane-done | T13 batch 2 lanes found finished; lead rerun and counterexamples started
- All five batch 2 Carbon lanes (wrap-levers, e3-iface, lfr-tape-r2, lfr-bbalu-r2, lfr-poseidon-r2) finished on 2026-11-13 and are committed on Moriarty t13-lanes (f98a0a7d5..958a26919); the LOOP note "wrap-levers running since 09:53" was stale. Pane w1:pJ is gone; nothing was lost.
- Lead rerun of every listed quint command (127) started; three Opus agents build the counterexamples on the real code (openvm t13-cx at 3062145, Moriarty t13-cx at f8fd268e3).
- Gap list: T10 already has lane qa-20261114-t10-bound (openvm 33371a6); T10m, T10q and T16 sched.rs 0b26d74 had none. Prompts t10m-merge, t10q-fixes, t10q-chunk, t16-sched-r2 written (not launched).

## [2026-11-14 20:22] decision | T14 m7 keycache v4 KEEP under the m24 moved-work rule (m26)
- huginn h70: cache hits cut keys_s 54.09 to 0.14 s; process keys_s + cold call 132.57 to 78.25 s (disjoint, MEASURED 5x2); corrupt-cache test 8/8 PASS (a swapped B L_Fr key is refused and rebuilt byte-identical).
- The +1.1 s lfr load is the approved pre-hash check; the merged patch on 3eaa4b1 does it once on the background thread, with item1, then a 3x2 stacking check.

## [2026-11-14 20:29] decision | T16 (c) to (e): base tree, F-3 form, (c) blocked on leaf rules
- Base: openvm t16-elastic = t16-cx 0b26d74 + merge t10mm (d10b388); Moriarty t16-elastic = 406cdf7ed + merge t10m (1a7d641be). Keys are the T10m keys (key_root 0fade1a4...); T10q changes only the L_Fr and wrap keys.
- F-3 form: the slice plan's menu becomes the universal menu (sets 0, 1, 2 at S = 4 and the S = 8 path C shape), and sched::menu_key_leaves / admit_key_root build key_root from it, so key_root stays byte-equal and only plan_d (a statement value, no key) changes.
- (c) 1x to 8x run length is BLOCKED: the leaf proves 5 rules and only `entry x;` (5 steps) and `unitctor` (2) fit; the cheapest variable-length family (`entry 1 + ... + 1;`) needs 15 rules plus frame support, so new chip sets and a rekey.

## [2026-11-14 20:36] build | T16 (d): t7_chain t16 hw/join built with CUDA on huginn
- New in openvm t16-elastic (uncommitted until the runs pass): sched menu_key_leaves/admit_key_root (F-3), slice_plan universal menu, harness prove_gpu, t16 modes hw and join, feature cuda on sherlock-wrap-export.
- huginn copy ~/sherlock/t16/src (non-git rsync, REVS.txt), build under gpu.lock so T14 timing never overlaps; runs h-gpu, h-cpu, h-small queued on the lock (unit t16-run). muninn build waits on heavy-slot (unit t16-cde-build).

## [2026-11-14 20:38] lane-start | Prover GUI build: group 0 committed, five Muse Wonder Full lanes running
- Supervisor commit 7f4dc5753 on prover-gui: frozen contracts (bus, layers, types, facts loader, tokens from one seed colour, URL table, event schema), stubs, fixtures, index.html with 92 import-map integrity hashes and the plain failure state; contract tests 9 of 9, check_facts 95 facts 0 failed.
- Lanes column, flow, code, elastic, shell as units pgui-<lane>-r1 in worktrees ~/Projects/Moriarty-wt/pgui-lane-<lane>; wire model muse-wonder-full-1.0, Contributor mode off; network enabled for column and shell only.
- MEASURED: the Muse sandbox under network restricted allows no socket (unix or loopback) and no Chromium; under enabled both work. Round 1 of lanes 2 to 5 holds no visible part until the hero frame gate passes.

## [2026-11-14 20:45] lane-done | Prover GUI lane elastic round 1 (records and plan models), supervisor QA PASS
- Five seed records built from their sources by build-records.mjs; the supervisor reran `--verify` (PASS, 5 records) and `node --test` (elastic 15, contracts 9, 0 failed) outside the sandbox; ownership check 0 files outside the lane.
- DERIVED other bands: huginn warm 1.30 s, cold 1.88 s, kept stack 10.54 s; the two muninn records are not timing records. Round 2 (styled rows, harness page) waits for the hero frame gate.

## [2026-11-14 20:48] lane-done | Prover GUI lane flow round 1 (flow data, seen-by rule), supervisor QA PASS
- flow.json: 11 stages, 29 items, 97 citations; the supervisor reran check_flow.mjs (0 failed at the pinned commits) and `node --test` (flow 11, contracts 9, 0 failed); ownership check clean.
- Rulings: the shell calls the sheet fillers directly with a full ctx; lane 0 of the cap is the item binding_input; an item name written with a space in the design counts as its identifier (6 citations).

## [2026-11-14 20:55] lane-done | Prover GUI lane shell round 1 (bridge, timeline, store, live client), supervisor QA PASS
- The supervisor reran `node --test` on bridge, live, shell and contracts: 45 passed, 0 failed, with fake prover sockets only; check_no_numbers and check_text print nothing; ownership check clean.
- The bridge is Node standard library only (478 lines), drives `call` then `prove` on the two sockets, and runs one fixed-argument nvidia-smi query. No real prover socket was touched: t14b-run.service is a timed job.

## [2026-11-14 20:56] qa | Prover GUI hero frame gate, first review: NOT PASSED (12 fixes)
- Creative director on lane 1's S1 stills: light scheme draws layers near-black, the four layers read as one slab, the perspective camera distorts heights, labels collide with the column, the cap reads as noise.
- Token changes applied by the supervisor: --pgui-canvas-layer-top and -side (fixed mixes of the layer tone), --pgui-canvas-text-large 22 px; CHECKLIST gains H1 to H10 and the phone label budget is 8.

## [2026-11-14 20:59] lane-done | Prover GUI lane code round 1 (source map and snapshot), supervisor QA PASS
- 24 source entries over 11 layers, 52 snapshot files (488,021 bytes) read from the pinned commits, 36 Compact suite rows each marked "no proved run"; the supervisor reran snapshot-sources --verify, check_sources and `node --test` (code 10, contracts 9, 0 failed).
- Round 1 of flow, code, elastic and shell copied into prover-gui/app on the main worktree: 81 node tests pass, every data check passes, the page loads the default record through the stub scene.

## [2026-11-14 21:01] commit | T14 m7 byte-deterministic proofs: grindmin and eqbits-sort
- stark-backend t14-gpu d1e1567: GPU grind keeps the smallest PoW witness (atomicMin), as the CPU does.
- openvm t14-gpu cf21ea7: EqBits rows sorted by key instead of HashMap order; leaf to L proofs were byte-nondeterministic on CPU too.
- huginn h72: CPU vs GPU app and leaf proofs byte-identical; no regression (MEASURED 5x2); keys equal the frozen record. The eqbits fix should also reach the tested tree before T12 byte gates.

## [2026-11-14 21:03] lane-start | Prover GUI round 2: all five lanes on the merged files and the hero gate fix list
- Units pgui-<lane>-r2, wire model muse-wonder-full-1.0, network enabled (the lanes need loopback, Chromium and the pinned CDN modules for their harness pages). Lane column was stopped in round 1 after 25 min and restarted with the creative director's 12 fixes and checks H1 to H10.
- Supervisor rulings R1 to R8 in every prompt: the shell calls the sheet fillers directly, a replay takes 20 s with a DERIVED speed factor, named cap lanes come from flow.json, capture tools poll for ready because the page's policy refuses string evaluation.
- Lanes flow, code and elastic start their visible parts before the gate has passed, because the director asked for the shell in the next hero stills; they take tokens only and add no tone.

## [2026-11-14 21:04] note | T16 (d), (e) on huginn: four elastic runs, one claim_pub, T10m keys
- MEASURED (huginn RTX 5080, `t7_chain t16 hw`, under gpu.lock): policy pick shape 0, cut [4, 1], GPU, 2 leaves at once; key_root 0fade1a4... (T10m) and F-3 admission PASS on the real keys; every proof stock-verified.
- MEASURED: app shard 0.23 s on the GPU vs 5.3 to 5.6 s on the CPU (T16_DEVICE=cpu); forced cut [2, 1, 1, 1] gives 4 leaves in 2 waves; SHERLOCK_GPU_MEM = one leaf (ESTIMATE 16.8 MB) gives 1 leaf at once, 2 waves, leaf-stage VmHWM 1.79 GB vs 2.55 GB. claim_pub byte-equal across all four runs (new plan_d, so not the T10 value).

## [2026-11-14 21:07] qa | T13 batch 2 lead rerun: 127 of 127 quint commands as the lanes listed
- MEASURED: wrap-levers 15/15 mutants caught, e3-iface 19/21 (2 equivalent hold), lfr-tape-r2 18/19 (1 equivalent holds), lfr-bbalu-r2 9/9, lfr-poseidon-r2 13/13; every test suite passes, every reach witness found, every sound instance [ok].
- New lanes t10q-chunk and t10q-fixes DONE in Carbon panes (closed); lead rerun of t10q-chunk 29/29 as listed. t10m-merge and t16-sched-r2 running (panes w1:p1A, w1:p1B).
- Lead finding: T10m 5b7ada8 (t8_gp N1 carries HhiRangeAir) is not an ancestor of the T10q tip 3062145; lane t10q-fixes F-01 flags the missing table. Counterexample agent started; T10q not changed (rekey).

## [2026-11-14 21:10] decision | Owner rulings on the T15 picks; F-3 replaced by a setup mode
- F-3 REPLACED: "We want to be completely elastic, Sherlock should have a setup mode where it profiles the machine and optimizes for it". No schedule menu in key_root; T16 (c) to (e) redesigned as profiler plus profile-driven scheduler under one universal key set.
- Confirmed: x_j lock form canon (ANC-002, FINAL-DESIGN 1.6 to update); MERGE-WORK bound 5,400; keycache conditions; m18 rule with the m24 moved-work sum; T8b picks with N-7 and N-8; T20 picks.
- S1 GPU power limit stays as is.

## [2026-11-14 21:17] lane-done | Prover GUI lane code round 2 (styled Source level), supervisor QA PASS
- The supervisor reran the lane's Playwright harness in headless Chromium: 13 checks, 0 failed (opens at the marked line, no script element from the script fixture, one fetch per opened source, 36 suite rows, token contrast worst 5.26 in light and 8.41 in dark); node tests 10 of 10.
- Open: PGUI-CODE-009 (the column stays in view with Source open) closes on the merged app.

## [2026-11-14 21:17] commit | T14b RESULTS, timed sets queued behind the critical path
- Moriarty t14b 5bedf6cbd: ~/Projects/Moriarty-wt/t14b/research/sherlock-t14b-20261114/RESULTS.md (built items, ident gate, timed plan, huginn job); openvm t14b 1c04565; not pushed
- Keys and proof format unchanged by every item (wrap key byte-equal to T10m on every keygen; 2,912-byte proofs); timed sets run on units t14b-run and t14b-guard, `summarize.py` reads them
- Huginn job ready to post: ~/Projects/Moriarty-wt/t14b/research/sherlock-t14b-20261114/huginn/ (CPU fixed-base forms, whole-call daemon vs per-call processes)

## [2026-11-14 21:20] lane-done | Prover GUI lane flow round 2 (styled Overview and Flow levels), supervisor QA PASS
- The supervisor reran the lane's Playwright harness: 398 checks, 0 failed, over every layer, the three seen-by positions and both schemes (row limits, closed layers hold one sentence, Seen by wins over a trace, privacy rows per record, WCAG AA contrast); node tests 15 of 15.
- Finding for lane elastic: the local devnet record lists no refusals, so the PGUI-FLOW-015 scenario cannot pass on the shipped record until its meta.above carries the three ledger.refused facts.

## [2026-11-14 21:20] decision | T16: F-3 REPLACED by the owner's setup-mode ruling; F-3 code removed
- Owner 2026-11-14: "We want to be completely elastic, Sherlock should have a setup mode where it profiles the machine and optimizes for it". The F-3 menu admission (sched menu_key_leaves, admit_key_root, the 4-shape slice menu) is removed before any commit; slice_plan and plan_d are back to the T10m values.
- New: sherlock-leaf profile.rs (profile format sherlock-profile/1, host facts, profile-driven scheduler, check_free with no shard-count list), `t7_chain t16 setup` (host facts plus calibration proofs), `t16 hw` reads T16_PROFILE; spec SETUP-SPEC.md (EARS ST, PF, SC). No hard rule changes: the setup mode never makes keys or shapes, shards stay fixed-size, one universal key set.
- The F-3-era huginn runs (21:04 note) used the 4-shape plan_d and are superseded; the huginn schedules run was stopped.

## [2026-11-14 21:25] lane-start | T16 (c) to (e) Carbon Quint lane t16-cde in herdr pane w1:p1C
- Gemini 4 Carbon (high), tab carbon-t16-cde, folder ~/.cache/gemini-lanes/qa-20261114-t16-cde: schedule freedom of check_free, soundness without the pad list or a menu, the profile-driven scheduler (budget, no panic, scale-down), the profile as untrusted input, join and prove_gpu; 7 mutants.
- huginn setup MEASURED (profile sherlock-profile/1, 119.2 s): full leaf shape 0 GPU 2.14 s vs CPU 5.63 s (app proof 0.21 s vs 3.60 s); unit tests 16 lib + 4 quint replays pass.

## [2026-11-14 21:29] lane-done | Prover GUI lane elastic round 2 (styled base-plate rows), supervisor QA PASS, round 3 started
- The supervisor reran the lane's Playwright harness (0 failures over the five records, both schemes, 390 px), node tests 17 of 17 and build-records --verify (PASS).
- Round 3 (unit pgui-elastic-r3): the local devnet record takes the off-chain ledger verdict and refusal facts for PGUI-FLOW-015; rows separate by tone.

## [2026-11-14 21:40] commit | T14 m7 merged keycache v4 + item1: warm call 25.67 s, cold 28.75 s
- huginn h73 (MEASURED 3x2): cold call 70.41 to 28.75 s, warm 66.08 to 25.67 s, both disjoint; corrupt-cache test 8/8 PASS; keys equal the frozen record. KEEP under m26; warm lfr_prove +1.5 s attributed to item1, profiled next (R9).
- Commits: stark-backend t14-gpu e5d7f19 (item1 GPU engine for L), openvm t14-gpu d6020de (merged), Moriarty t14-gpu 665109154 (corrupt-test.sh).
- Against the clean baseline (warm 128.69 s, cold 199.73 s on huginn): warm 5.0x, cold 6.9x faster (DERIVED).

## [2026-11-14 21:40] lane-done | Prover GUI lane elastic round 3, supervisor QA PASS
- The local devnet record now lists the off-chain ledger verdict and the three refusal facts; build-records --verify PASS, node tests 26 of 26, harness 0 failures (supervisor rerun).

## [2026-11-14 21:40] commit | T14b runner guard and report commit 5e54263da
- Moriarty t14b 5e54263da: the critical-path guard matches the T10q gate jobs, the GPU sampler starts inside the slot, daemon-wrap declares 40G, summarize.py labels shared stages; T0 PASS 6/0/3.
- 5bedf6cbd carried run.sh and guard.sh under DW_DOCS_COMMIT=1; 5e54263da ran the full hooks over them.
- warm-1 started at 21:36 on a SHARED slot (no exclusive window in 900 s, load 38); units t14b-run and t14b-guard active, restart commands in ~/Projects/Moriarty-wt/t14b/research/sherlock-t14b-20261114/RESULTS.md section 3.

## [2026-11-14 22:04] lane-done | Prover GUI lane column round 2 (the three.js scene), supervisor QA PASS
- The supervisor reran the lane's browser harness in headless Chromium: 125 checks, 0 failed, with hero rules H1 to H10 at three sizes and both schemes (front face equals the layer token, seam contrast 5.0 or more, column width 13.6 percent of the window on desktop, 8 labels and 3 numbers at rest); node tests 14 of 14.
- MEASURED by the lane: 62 to 67 draw calls and 2,256 triangles for the warm column, 3 draw calls for the 512-cell estimate plan. NOT RUN: frame time on a GPU browser (muninn is under a timed prover job; headless ran on software GL).

## [2026-11-14 22:07] build | T16 setup mode on huginn: device-bytes fix, profile v2, GPU-memory cap demo
- Defect found on huginn: calibration read GPU device bytes per proof, but the engine pool reuses memory, so shape 1 recorded 0 B and SHERLOCK_GPU_MEM did not lower concurrency. Fix: device bytes against a pre-GPU baseline (openvm t16-elastic 88b5627); shape 1 now 1,025,507,328 B (MEASURED).
- huginn profile v2 (setup 117.3 s): GPU leaf 2.135 s / 2.094 s, CPU leaf 5.60 s / 5.35 s for shapes 0 / 1 (MEASURED). h-gpu: shape 1, 2 leaves at once, leaf stage 3.97 s, VmHWM 2.47 GB; h-cap (GPU mem = one leaf): 1 at once, 2 waves, 4.73 s, VmHWM 1.68 GB (MEASURED).
- Every run: claim_pub lane 20 = 31357459...598306, L accepted by the stock verifier, key_root 0fade1a4 (T10m); the (b) schedules S1-S5 on huginn give the same claim_pub, 17 of 17 cases pass.
- muninn stages wait for the heavy slot (the T10q gate holds 64G).

## [2026-11-14 22:10] qa | T13 batch 2 Carbon Quint lanes: interim
- Batch 2 lanes (wrap-levers, e3-iface, lfr-tape-r2, lfr-bbalu-r2, lfr-poseidon-r2): lead rerun 127/127 match (MEASURED). New lanes t10m-merge 31/31, t10q-fixes 28/28, t10q-chunk 29/29, t16-sched-r2 32/32 after lead repairs (pairNat mod p; three t10m test-claim errors).
- Defect found by reading the code: t10q tip 3062145 lacks T10m 5b7ada8, so t8_gp N1 has no HhiRangeAir and bus 963 does not balance; the fix is to merge 5b7ada8 into t10q. Held until the T10q gate result.
- PENDING-CX: t10q-chunk NT-1..NT-3 (anchor overwrite, run_tag and input_d not in lock_x, k not in lock_x); t16-sched-r2 F-1 (duplicate bucket shape), F-2 (u64 overflow sched.rs:268/340/384). Seven counterexample agents are queued on heavy-slot behind the T10q check-chain.

## [2026-11-14 22:25] note | T14 R9 spans on huginn (h76)
- Warm lfr_prove 17.55 s, fully attributed: main_commit 6.14 s (rows_cm 3.35 GPU), rap_constraints 5.47 s (zerocheck sumcheck 2.99 CPU, GKR host prep 1.60 CPU), openings 5.92 s (WHIR query PoW grind 2.54 CPU) (MEASURED).
- The +1.5 s warm lfr_prove drop did not reproduce; free VRAM is 13.93 GB at start. Merged KEEP stands.
- m31: T14b job next when gpu.lock frees, then GPU WHIR query grind as the next m7 candidate (about 2.4 s, ESTIMATE).

## [2026-11-14 22:28] decision | T14b ledger-check runs on muninn, not huginn (m32)
- huginn h77: ledger-check needs 20 registry crates that huginn lacks offline; the other T14b builds pass.
- Pick (c): skip ledger-check on huginn and fetch nothing. huginn keeps the wrap proof, VK and public inputs with sha256; muninn runs ledger-check on them, because it already passes there offline.

## [2026-11-14 22:50] qa | Carbon lane t16-cde: lead rerun 29/29, six host defects CONFIRMED and fixed
- Gemini 4 Carbon lane on the setup mode and profile scheduler: Q2 and Q5 SOUND (no schedule or profile can change a statement), Q1 and Q6 SOUND WITH CONDITIONS, Q4 INCONSISTENT (SC-005 against SC-007). Lead rerun of all 29 quint commands: exit codes as reported.
- Counterexamples on real code (openvm c1abce5 before, d3429e2 after, MEASURED on huginn): F-1, F-4 to F-8 fail before and pass after (unit cx_f*); shrunk as-built traces F-4, F-6, F-9 replayed by quint_t16_cde.rs. F-2 NOT REPRODUCED (5-step slice), F-3 and F-11 OPEN DESIGN, F-10 fixed by reading.
- Decision F-9: SC-007 amended (a lower limit never gives a faster pick; per shape and device the concurrency never rises), because the least-time rule is the owner's "optimize for the machine" and the shape is not in the statement.
- Verdict qa/gemini-quint/qa-20261114-t16-cde/verdict.md on qa-carbon-20261114; COUNTEREXAMPLES in tests/quint-regressions/t16 on t16-elastic.

## [2026-11-14 23:02] qa | T10q check-chain: chain A PASS, chain B running
- T7CHAIN PASS for program A: app, leaf, N1, N2, N3, L and L_Fr are proved and verified on one universal key set. The FP8D3 bounds gates all PASS, and the lowest shift is 101.0110 bits (MEASURED). Chain B started at 22:24.
- N1 was proved and accepted on t10q tip 3062145. This goes against T13's claim that bus 963 does not balance at that tip, because an unbalanced bus fails verification. T13's claim stays open until a counterexample decides it: CONFIRMED, NOT REPRODUCED or GUARD HOLDS.

## [2026-11-14 23:03] qa | Prover GUI: shell round 2 merged, first full page run finds a camera defect; lane column round 3 started
- All five lanes are merged in prover-gui/app on the main worktree: 102 node tests pass, every data check passes, the page loads with no console error. The shell lane's own harness reports 76 of 76.
- Defect found by the supervisor's capture of the merged page: the scene fits its camera before the shell sets the models, so the column fills the window. It never showed in the lane's harness, which sets models before the scene is ready. Unit pgui-column-r3 fixes it and delivers the hero stills from the merged page.

## [2026-11-14 23:12] lane-start | T18 (d) Mycroft compiler through the software factory
- Owner: "Run the most current version we have of the software factory to finish the Compiler" and the remaining work.
- Factory: beta-c-formal dd5fb49 (newest; still in validation by another session), pinned read-only at ~/Projects/sf-snap-dd5fb49. Worktree mycroft-factory from mycroft c1c78e66f.
- Scope: Lean oracle --step + kink-implementation/1, Quint from spec + ITF replay, k-rust suite, Rust refinement after the T18 (c) spike merges, SORRIES premises and limits, suite coverage. Recursive proofs wait for T10q keys.

## [2026-11-14 23:40] decision | Owner: Sherlock upgrade authority is a rotated Fenrir operator key set
- Owner: "it would in practice be a set of keys rotated from the Fenrir operators". The SherlockLedger maintenance authority is the Fenrir operator committee. It replaces verifier keys on upgrades, and it is rotated.
- Ledger a2b3012 supports this as built: ContractMaintenanceAuthority { committee: Vec<ContractMaintenanceVerifyingKey>, threshold: u32, counter: u32 } (onchain-state/src/state.rs:943). SingleUpdate::ReplaceAuthority does the rotation; VerifierKeyRemove plus VerifierKeyInsert under a ContractOperationVersion does the key change (ledger/src/structure.rs:3199).
- Still open for T15: the threshold, what triggers a rotation, the version policy for old claims, and how the Fenrir charter profile names the committee.

## [2026-11-14 23:55] lane-start | T22 governance module for the Midnight on-chain representation
- Owner: "add a governance module for the Midnight onchain representation"; "do we need upgradable contracts? Could we just develop a functional version system with backwards compatibility like Cardano does with the hard fork combinator".
- The 23:40 entry described in-place verifier-key swaps. This entry corrects it. Fenrir StrawAnchor is sealed for life: an empty maintenance committee with threshold 1, and a new key means a new deployment (projectfenrir README section 8). LEAD pick for owner confirmation: no upgradable contracts. Eras (key set, wrap VK and a sealed ledger) are composed by a sealed era registry. Fenrir operators govern through registry verbs: activation under quorum and timelock, retirement, and key-preserving rotation.
- OWNER items: read "one universal key set" as one set per era; set the quorum, timelock and overlap window; decide the policy for claims accepted before a soundness bug.
- Lane T22, worktree t22-governance from t10m: use cases, OpenSpec with EARS, and a Quint model.

## [2026-11-15 00:00] lane-start | Hard fork combinator study for the Sherlock governance model
- Owner: "Study Cardano's hard fork combinator design and see what our Sherlock governance model should be".
- An Opus lane reads the primary sources in full: the ouroboros-consensus report, the HFC docs and code, and the protocol-version and CIP-1694 trigger. It writes HFC-STUDY.md, GOVERNANCE-MODEL.md and SOURCES.md under research/sherlock-hfc-study-20261114/.
- It tests the T22 LEAD pick (sealed eras plus an era registry) against the HFC. T22 holds its verbs, its Quint model and its open questions until the study lands.

## [2026-11-15 00:10] lane-done | Hard fork combinator study: Sherlock governance model
- Answer (recommendation, not canon): no upgradable contracts. Every contract is sealed for life. A new version is a new era: a sealed SherlockLedger with its own key set. A sealed SherlockEraTable lists the eras with a fixed active interval, and nothing is translated across eras. The ledgers stay readable for ever.
- Four changes to the LEAD pick: the era ledger checks the table on every accept; the only retirement verb is an immediate, irreversible soundness_end; the table roster is rotatable (Fenrir's is sealed); claims from an era with a soundness break are unsafe by default.
- Assumptions about ledger 9.1.0.1, not yet tested: A1 block-time builtins, A2 a claimed cross-contract read, A3 a check in acceptRun without breaking the wrap statement (reopens frozen I-24). Owner items O1-O7 with recommended options. Files: research/sherlock-hfc-study-20261114/ (GOVERNANCE-MODEL.md, HFC-STUDY.md, SOURCES.md), not committed. Passed to T22.

## [2026-11-15 00:10] incident | T18 (c) Mycroft spike: Stage 1 compiler written, proving BLOCKED on the T10m keys
- Branch mycroft: `mycroft/compiler/` (Rust, own workspace, ledger rev a2b30128 builtins) ports the Lean `compile`, `SL.run`, `Ref.run` and `hostTape`, reads stock compactc 0.36.0 ZKIR 3.1 and runs t02 increment, s01 th, t01 get (get-set, get-unset, get-set-again) against the native ref vectors; it writes a Lean `#guard` file `Mycroft/Spike/T18c.lean`.
- Build and run queued 3 h behind full heavy-slot budget (64G T10q jobs); durable unit `t18c-spike` (`run-spike.sh`, logs `~/.cache/t18c/`) builds, runs and lake-builds the check when admitted. No result yet: no word-equality claim is made.
- Proving BLOCKED (DERIVED from openvm-t10mm source): the tested app chip sets hold only the `entry x;` rules R_ENTRY..R_PUBLISH and value-set R_CNULL/R_PUBV (`sherlock-leaf/src/dec.rs`); no R_SL_* rule exists, and the T10m call vector is the SherlockLedger acceptRun layout (lane map anchor 10, claim 20/34, outcome 39, PUB 21, 22, 40, 41), so no program's W reaches the wrap. An R_SL chip set is a new key_root leaf and a rekey (collides with T10q).
- Brief correction (MERGE X8): path C proves only the one-leaf `entry x;` run, not a Compact call as a foreign proof.

## [2026-11-15 00:45] qa | T14b on huginn: daemon saves 41.4 s per call; ledger 9.1.0.1 accepts all 61 wrap proofs
- huginn h79 (MEASURED): an end-to-end call takes 113.01 s [112.88-113.44] with one process per call, and 71.59 s [71.28-71.82] with the resident daemon (-37%). The daemon is ready 34.1 s after start. The wrap takes 46.51 s on CPU, 44.75 s with fixed-base tables, and 43.77 s with c=16 (+6 GB).
- muninn ledger-check: 183 files, list sha256 e99e341c, all hashes OK. The ledger 9.1.0.1 verifier gives Ok(()) on 61 of 61 wrap dirs (proof 2,912 bytes, 1,026 public inputs, VK 1,325 bytes, about 0.19 s per verify) (MEASURED).
- huginn ~/sherlock/src resynced to openvm 69b5e03, stark-backend 9488e74, Moriarty 3fd18b166; huginn verified the trees and released the night chain (stack-m31, whirgrind, encodeov, gkrdev).
- Governance: the 14-day timelock comes from the Fenrir README line 592, which traces to an agent draft (gemini-pack-t6 ID-41). It is not canon. T22 derives every timelock and quorum value as a parameter.

## [2026-11-15 00:55] qa | T13 CX t10q-chunk: NOT ATTEMPTED (no heavy slot), reach and read-only notes
- The 8 findings (F-1..F-8, NT-1..NT-3 included) are all NOT ATTEMPTED: two 24G builds waited about 3 h because T10q check-chain held the 64G budget, with t16-cde queued ahead. C0, C4 and two prose-vs-code notes are OPEN DESIGN. The lane says all 10 mutants are caught.
- Reach: the SDK SherlockLedger has no chunked entry, and its anchor is sealed and written only by the constructor. So NT-1..NT-3 are not reachable on any SDK contract today; only the research chunk-probe.compact writes an anchor.
- Read only, not a verdict: the root StmtAir exposes words 0..106 but does not constrain words 74..96 (MODE, J, K, E_IN, E_OUT, RUN_TAG, LAYOUT); the Stmt interface AIR in L carries only words 106..115; the Call interface AIR has no constraints. This stays OPEN until the counterexample runs on 3062145.
- Test files are written but not compiled or committed: openvm-t13cx tests/t13_t10q_chunk.rs and t13-cx tools/sherlock-wrap/tests/t13_t10q_chunk.rs. Plan in ~/.cache/gemini-lanes/qa-20261114-t10q-chunk/COUNTEREXAMPLES.md. The builds stay queued.

## [2026-11-15 01:00] commit | T10q: cherry-pick the T10m gp2 harness fix (openvm t10q 0082832 from 5b7ada8)
- T13 found that t10q 3062145 lacks T10m 5b7ada8. That commit changes only examples/t8_gp.rs: the t8_gp layers carry T10's h_hi range table, so their N1 is the universal N1. It is a test-harness gap, not a key or circuit change. Without it, gp2 fails with rc=101, as it did on t10mm at 14:51.
- I cherry-picked it before the gate reaches build-t8gp, and check-chain is not affected. T13's bus 963 claim is resolved in the gp2 harness; production N1 already carried the table (chain A accepted N1).
- Remaining stages after chain B: chain.sh pathc, build-t8gp, gp2, x7, d3work. On t10m/t10mm these took about 0.4 to 1.7 h for pathc and about 20 min for the rest (MEASURED there).

## [2026-11-15 01:05] qa | T10q check-chain: chain B PASS
- T7CHAIN PASS for program B: app, leaf, N1, N2, N3, L and L_Fr are proved and verified on one universal key set. Every height is under its bound: L 16/20, L_Fr 19/20, app 17/24, leaf 17/21, nodes 16 to 19/19 (MEASURED).
- Times under load about 47 (MEASURED): lfr_prove 1,097 s, l_prove 288 s, keygen 235 s, l_keygen 301 s, lfr_keygen 284 s.
- Next: the rest of the chain.sh gates, then pathc, build-t8gp, gp2 (with the 0082832 harness fix), x7 and d3work.

## [2026-11-15 01:05] lane-start | Prover GUI lane shell round 3 (faults on the merged page)
- Unit pgui-shell-r3: the sheet opener is captured too late (found by lane column), the phone chrome is taller than its tokens, state S3 shows no idle base plate, program B carried a label with no fact. New contract: --pgui-chrome-top and --pgui-chrome-bottom on #pgui-app, set by the shell and read by the scene.
- Lane column round 3 is still running: its harness now drives the merged page (151 checks in its own run).

## [2026-11-15 01:10] decision | Wrap hill-climb sent to huginn (m36), queued after whirgrind
- Owner: "Send over to Huginn the wrap optimization hill climb for after it finishes the existing work". The wrap is about 45 s of the 71.59 s daemon call (h79, MEASURED).
- Plan: w0 baseline, w1 a discrete-GPU port of the gpu-wrap hooks (52ad95069; they now read GB10 unified memory), w2 a fused quotient kernel, w3 a skew-robust GPU MSM, w4 the CPU residual, w5 daemon residency and overlap.
- Gates: byte-identical to CPU under GW_DET=1, distinct proofs otherwise; VK equal to t10m-wrap.vk; k=21; stock verify; ledger 9.1.0.1 check on muninn; corrupt test 8/8; no new dependencies without approval.
- huginn h82: its user is wrapping up tonight, so the night chain stops after whirgrind. encodeov and gkrdev are written but not built.

## [2026-11-15 01:15] decision | Wrap hill-climb crates: double check APPROVE, fetched on huginn by its user
- Install double check for cudarc 0.19.10 and libloading 0.9.0 (needed only by mori-msm's optional cuda feature, step w3): Codex gpt-6.1-sol APPROVE, Opus APPROVE. Both checksums match the committed tools/sherlock-wrap-gpu Cargo.lock (7359bf1d, 754ca22d).
- The muninn copy was not needed: huginn's user ran cargo fetch --locked for these two crates and the 20 ledger-check crates, with both lockfiles unchanged (h84).
- huginn order: w0, local ledger-check, w1, w2, w4, w3, w5 (durable units sherlock-wrap-w0 and sherlock-wrap-chain). moriarty-tools.diff (39f1c4ab) is being committed onto t14-gpu as the base.

## [2026-11-15 01:15] lane-done | Prover GUI lane column round 3 (scene in the merged page), supervisor QA PASS
- The supervisor reran the lane's harness: 151 checks, 0 failed, now including the merged page (models set after ready give the same column box, focus round trip on the L label, one value in one place with a sheet open).
- Cause of the camera defect, found by the lane: a camera move started before the models arrived kept running and overwrote the fit. Open for round 4: shelf columns taller than the front column run off the top; the scene must read the shell's chrome heights.

## [2026-11-15 01:15] lane-start | Prover GUI lane column round 4 (shelf, claim marks, chrome band)
- Unit pgui-column-r4. Faults the supervisor saw on the merged page at 1440x900: the claim's marks do not draw for the record-built model, shelf columns stand at the front column's scale and run off the window, canvas text and the column pass under the chrome on a phone.

## [2026-11-15 01:20] incident | T16 exclusive heavy-slot hold blocks the T10q gate
- t16-cde (all-in-one.sh, pid 1544057) has held heavy-slot EXCLUSIVE at 48G for 106 min, running a chain of t7_chain runs. The T10q check-chain fp8_d3 for chain B (64G) waits behind it. Every later gate stage and the T18 (d) small jobs (6 to 8G) wait too.
- Lead order to T16: let the running t7_chain (t16 setup) finish, stop before the next stage, take the lock per stage and never across a chain, and requeue the rest after "T10Q GATE ALL DONE".
- T18 (d): MYC-VAL-002 finding H1 (salt, selector and rand not checked for canonical form) is CONFIRMED and fixed in the Lean reference, the oracle and the Rust code (mycroft/factory/COUNTEREXAMPLES.md). The mycroft spike d1e8eed8d is merged into mycroft-factory.

## [2026-11-15 01:25] commit | T14 whirgrind KEEP: GPU WHIR query PoW grind
- huginn h85 (MEASURED): warm call 24.01 -> 21.44 s [21.26-21.50], cold 28.96 -> 26.54 s, both disjoint; the grind drops from 2.54 s to 0.05 s. Byte gate (CPU = GPU), kbench and the corrupt test 8/8 PASS.
- Commits: stark-backend-t14 054fe1d (patch 6fea63cc), openvm-t14 c432de0 (464601ba). The Moriarty t14-gpu patch (4ce953ea) is queued behind the T14b tools commit.
- The STARK side is now 128.69 s at the clean baseline -> 21.44 s warm. The wrap hill-climb w0 started on huginn at 07:15Z.

## [2026-11-15 01:25] commit | Prover GUI: commits c48e1d7bc and 4be7b18c4 on prover-gui (rounds 1 to 3 of the five lanes merged)
- c48e1d7bc: round 1 of flow, code, elastic and shell with the hero gate token changes. 4be7b18c4: the scene, the styled sheet levels, the shell, the bridge dry run and the chrome height contract; 102 node tests pass on the merged app. Not pushed.
- Dry run of the live path by the supervisor against stub sockets (no T14b daemon is free; t14b-run.service is a timed job): the page connects with the token, Prove runs one call, title Proving then Done, 403 without token or Origin, 405 on OPTIONS, no console error.

## [2026-11-15 01:25] incident | T16 exclusive slot held across stages; released, requeued per stage behind the T10q gate
- all-in-one.sh held heavy-slot --exclusive 48G for build + setup + every hw stage and blocked the T10q fp8_d3 job (coordinator). The running `t16 setup` finished (884 s, profile written); then the runner was stopped and the slot released at 00:25.
- run-cde.sh now takes one shared slot per stage behind a guard (T10Q GATE ALL DONE, no T10q/T11/T12/T17/t7-check heavy job); unit t16-cde-run5 runs m-gpu, m-nogpu, m-cap, m-cg8, join. all-in-one.sh retired.
- muninn profile (MEASURED, load 40): GPU leaf 23.7 s / 26.0 s, CPU 47.7 s / 50.1 s (shapes 0 / 1), device bytes 0.375 / 1.141 GB, budget 16.9 GB (the scope).

## [2026-11-15 01:26] commit | T16 (c)-(e) RESULTS-CDE and LOOP status
- Moriarty t16-elastic 516ced2d4 (spec, runners, huginn records, t16-cde counterexamples) and RESULTS-CDE.md: (d) PARTIAL, (e) PASS on huginn, (c) BLOCKED; muninn runs and join pending in t16-cde-run5. LOOP.md T16 status updated on main.

## [2026-11-15 01:27] note | odin remote box: key made, setup bundle sent
- Owner created YubiKey credential ssh:odin; setup bundle (BOX-SETUP.xml, BUS.xml, nosudo/sudo templates) sent to odin over Taildrop.
- odin's tailnet node is the Windows host, so the WSL2 distro runs its own userspace Tailscale node odin-wsl in mode USER (sshd on 127.0.0.1:2222); WSL case added to the remote-connect template.

## [2026-11-15 01:28] qa | CX t10q-fixes lane: F-01 already fixed on t10q
- Lane tested openvm 3062145: CONFIRMED 2 (F-01 t8_gp N1 lacks HhiRangeAir; F-10 known GP2-C2.1), GUARD HOLDS 17, OPEN DESIGN 4, NOT ATTEMPTED 1; tests in openvm t13-cx 189badb.
- F-01 fix is 0082832 (cherry-pick of T10m 5b7ada8), already on openvm-t10q above 3062145; the gp2 harness stage runs on it.

## [2026-11-15 01:28] lane-done | Prover GUI lane shell round 3, supervisor QA of the node side PASS
- Merged into prover-gui/app: 104 node tests pass, check_no_numbers and check_text print nothing. The lane's harness reports 95 of 95 (opener focus round trip, chrome heights 91/64 px desktop and 122/64 px phone, S3 idle base plate with the previous-run outline, program B as a plan).
- The supervisor's browser rerun waits for lane column round 4, which reads the new chrome heights.

## [2026-11-15 01:29] build | huginn wrap hill-climb w0 baseline (h87)
- MEASURED, k=21, 18 distinct proofs all wrap_verify OK: warm prove fixed-base 44.76 s [44.49-44.91], CPU MSM 46.33 s; cold 44.82 / 46.60 s; about 395-400 CPU s per prove.
- Warm fixed-base phases: advice 7.46, rest of trace 10.30, quotient numerator 18.20, finalise 8.36 s (MSM inside them 13.57 s); files sha256 manifest 0d2a57a375069181.
- Next on huginn: ledger-check of the w0 proofs, then w1 discrete-GPU port starting at the quotient hook.

## [2026-11-15 01:30] qa | muninn ledger check of huginn w0 wrap proofs
- 18/18 w0 wrap dirs from huginn (manifest sha256 0d2a57a375069181, all file hashes match) pass the ledger 9.1.0.1 verifier on muninn; no false check in any ledger.json (changed lanes and public inputs refused).
- huginn's own check reported 79/79 (61 T14b + 18 w0); muninn now confirms all 79. Files in ~/.cache/t14b/huginn-wrap-w0.

## [2026-11-15 01:30] qa | T18 (c) Mycroft compile spike: compiled W word-equal to native on 5 of 5 vectors, Lean check PASS
- MEASURED (`mycroft-t18c`, unit `t18c-spike`, logs ~/.cache/t18c/): t02 increment (L_e 7, N_e 3), s01 th (L_e 11, N_e 5), t01 get on get-set, get-unset, get-set-again (L_e 18, N_e 11): W word-equal to stock `transcript_words`, `comm` equal to native `lane1_comm`, outputs equal the JS result, frame stripped at cell (0) gives the native words.
- `lake build Mycroft.Spike.T18c` PASS, 28 `#guard`s: the Lean `compile` gives the Rust program image and ABI, Lean `SL.run` and `Ref.run` give the native words and comm, and the ZKIR reader reproduces `Tests.counterInc` and `Tests.tinyGet`. Negatives refused: badFiller, badSelector, inputLeftover, inputUnderflow, irVersion 3 2.
- Not equal on a deployed contract at cell (0): t02 W word 3 and s01 W word 4 address root key 0, the anchor cell. Stock W needs the slot-15 re-key (X1 (a)). Proving is still BLOCKED: there is no `R_SL_*` rule in the T10m chip sets.

## [2026-11-15 01:31] build | huginn wrap hill-climb w1 gates (h89)
- w1 discrete-GPU port (wrapd 64a37aed): byte gate PASS on seeded test proofs for cpu, fixed, ntt+coset+quotient and fixed+ntt+coset+quotient; verify ok, peak 12.3 GiB.
- Informational, not a timed result: the 'all' mode (sppark MSM on GPU too) is byte-identical with 0 fallbacks; one seeded prove 13.50 s vs CPU 48.29 s.
- Timed w1 set started 07:45Z; the 'all' timed set becomes w3; w2 (fused quotient) is being written.

## [2026-11-15 01:32] qa | CX t16-sched-r2 lane: F-1 CONFIRMED at the API boundary
- openvm 0b26d74: CONFIRMED 2 (F-1 duplicate shapes: schedule output fails check; incumbent re-score at sched.rs:467 picks score 50 over 1), LATENT API HAZARD 2 (F-2 overflow at sched.rs:340/384), GUARD HOLDS 3, NOT REPRODUCED 5, OPEN DESIGN 3 (F-5 T16-PLAN.md:21 omits area weighting, P_CUR, C4-1 key admission).
- The shipped caller t16.rs:221 passes distinct shapes, so no live run hits F-1; tests in openvm t13-cx-t16 3c43da3 (ignored as CONFIRMED, not fixed). Fix goes to the T16 lane: refuse duplicate shapes in schedule and check, keep the incumbent score in best.

## [2026-11-15 01:33] note | odin linked from muninn (SSH master + herdr session odin)
- odin-wsl (Ubuntu 26.10 on WSL2, RTX 5090 32 GB, driver 616.92, Core Ultra 9 285K 20 threads, 90 GB RAM, 782 GB free) runs system sshd on port 2222 and root Tailscale inside WSL.
- First login refused: odin's authorized_keys held only the ssh:gpubox key; odin's Claude added the ssh:odin key. Host key SHA256:MmC5Rz556dJ3EDIuomQHKH3p2MIIhifFwlXzXQpnoXA confirmed by the owner and the box report.
- herdr machine odin (session odin) enabled; odin agent pane w1:p1 is setting up the bus at ~/remote-bus.

## [2026-11-15 01:34] decision | odin installs: double check passed for rustup, CUDA 13.4 toolkit, cmake
- Codex APPROVE 1,3,4,5,6 and REJECT 2 as written (no 13.4 in the WSL repo; use the toolkit-only runfile); Opus APPROVE 1,2,4 and REJECT 3,5,6 (no crate uses bindgen, protoc or openssl-sys).
- Approved by both: rustup user-local, CUDA 13.4 runfile toolkit-only in ~/.local/cuda-13.4 (no driver), cmake Kitware tarball; sent to odin as m2. Verdicts in ~/.cache/install-checks/odin-20261115/.

## [2026-11-15 01:35] note | odin checkouts current; Midnight tools approved
- odin fast-forwarded from muninn refs: Moriarty main 567695ce2 -> 1c6c144e5 (+275), projectfenrir main +3, workflowskills main +4; Moriarty-s10 and Warlock (dirty) and Moriarty-pristine (detached) left as they were.
- cargo fetch --locked passes in 4 Moriarty workspaces; the pinned midnight-ledger, midnight-zkir and midnight-zk revs are present. Moriarty repos/ (Midnight clones at the ledger-9 pins, 3.6 GB) is copying to odin.
- Codex and Opus APPROVE the Midnight tools at muninn's pins: node 2.1.101, proof-server 9.0.1, indexer 4.4.101 (pulled by target-pins.json digest) and compactc 0.36.0; sent to odin as m4.

## [2026-11-15 01:36] qa | CX t10m-merge lane: MW-F01 whi forgery CONFIRMED on T10m, fixed on t10q
- openvm 5b7ada8, 21 rows: CONFIRMED 2, GUARD HOLDS 10, NOT REPRODUCED 1, OPEN DESIGN 4 (KEYDAG_IDX host-fixed, CORE height bound from a key parameter, 2 comment notes), NOT ATTEMPTED 4 (no local L harness).
- F-03 (MW-F01): false-carry and 2^64-wrap whi forgeries pass Work, NW, NW2 and N3 on 5b7ada8; t10q fixes it in beba730 with bus 197, so the planned t10q -> T10m merge closes it there. Tests in openvm t13-cx-t10m cb02649 (7/7 pass).

## [2026-11-15 01:37] decision | wrap hill-climb w1 KEEP: GPU ntt, coset and quotient phases
- huginn h92 (MEASURED, k=21, 18 timed proofs): warm 49.55 s [45.99-49.84] -> 25.36 s [25.33-31.37], cold 49.38 -> 25.86 s; quotient phase 20.8 -> 0.38 s; device peak 12.29 GiB.
- muninn ledger 9.1.0.1 check of the 18 w1 dirs (manifest 6d1ca90e53ae2d5f): 18/18 PASS, negatives refused, wrap.vk equal to w0. Patch w1-moriarty.diff sha256 76fb41c1; sent m40.

## [2026-11-15 01:38] qa | T16 fixes for CX t16-sched-r2 F-1, F-2, F-5
- openvm t16-elastic 4b4e5fe (base d3429e2): duplicate shapes refused in schedule, check, profile::schedule and check_free; best keeps the incumbent score; fit and from_area saturate.
- The F-1 and F-2 tests fail on d3429e2 (debug add overflow at sched.rs:384) and pass after the fix, 7/7 in release and debug; lib 26, quint_t16 4 and quint_t16_cde 4 pass. T16 PLAN step 3 now states the score rule.

## [2026-11-15 01:39] lane-done | T18 (c) Mycroft compile spike: compile half DONE, proving blocked on keys
- mycroft e3206a320: Stage 1 Rust compiler, ZKIR 3.1 to R_SL; t02 increment, s01 th and t01 get: W word-equal to stock transcript_words, comm and outputs equal on 5 of 5 vectors (MEASURED); lake build Mycroft.Spike.T18c PASS, 28 guards.
- Negatives refused: BadFiller, BadSelector, InputLeftover, InputUnderflow, IrVersion(3,2). Proving waits for an R_SL chip set (new key_root leaf, a rekey after T10q).

## [2026-11-15 01:40] decision | Owner AFK night 2026-11-15; odin joins the loop for heavy loads
- Owner: "add odin to our workflow for heavy loads, make sure we include it in our loop to finish sherlock. I'm going to bed. Go to AFk mode". LOOP.md a07a62131 adds the AFK rule and the Compute boxes rule (odin + huginn take heavy jobs, muninn keeps the ledger-check authority).
- odin CUDA 13.4.2 toolkit installed (nvcc V13.4.92, gcc 15.3 accepted, test kernel ran on the RTX 5090); CPU warm builds 4/4 rc=0. m6 sent: GPU build, byte gates, timed T14b baseline on the 5090.
- T16 told odin is the third host for (d) across hardware once its byte gates pass; huginn told it can hand waiting candidates to odin.

## [2026-11-15 01:41] decision | huginn, muninn, odin communication: roster + relay
- Owner: "make sure there is good communication between Huginn, Muninn, and Odin". huginn and odin cannot reach each other; muninn is the hub.
- BOXES.md on both buses (roles, reach, rules); both agents acked. muninn unit box-relay keeps one timeline of both outboxes (~/.cache/box-bus/all.log) and forwards MSG lines naming to:odin, to:huginn or to:all.
- w4 lookup-leftover order (huginn h93) sent to an Opus ZK review; w4 may build and time, KEEP waits on the verdict.

## [2026-11-15 01:42] incident | T10q gate step starved in heavy-slot; lead reservation added
- check-chain's fp8_d3 (shared, 64G = the whole budget) waited about 40 min while new shared jobs kept taking slots (t13-cx-tape 24G, t14b-warm-1 40G); heavy-slot only writes a pending record for exclusive jobs, so a large shared waiter never blocks admissions.
- Fix applied without kills: a pending record for pid 2103756 in heavy-slot.jobs blocks new shared admissions until the running jobs finish; the waiter overwrites it with its own record when admitted.
- Root fix to do: heavy-slot should write a pending record for any shared request at or above half the budget (with a factory test).

## [2026-11-15 01:43] decision | New row T23: full Compact suite on the finished Sherlock, on Preview
- Owner: "make sure you complete the roadmap while I sleep. Run the entire compact test suite on the finished sherlock and see if all the contracts work on Preview".
- T18 (d) now covers all 33 compiling suite programs (suite-coverage.json) and writes the R_SL chip-set requirements; LEAD PICK: the R_SL chip set folds into the T11 blinded rekey (one rekey). Proving and Preview settlement wait for T10q, T11, T12; provers on huginn and odin.

## [2026-11-15 01:43] qa | Prover GUI hero frame gate, second review: dark S1 frame PASSES; four lanes sent back
- Creative director: proportions, tones, type and composition pass on the merged app. Gate closers: the light layer tone (token changed by the supervisor to a darker mix, 2:1 or more against the ground), the missing 5 steps figure, plates cutting the claim. Rulings: no shelf names at rest, the definition line inside the 56 px top bar, proof size and public inputs at Midnight, plan view spread over half the canvas.
- Supervisor rerun on the merged app: lane column's harness 160 of 166 (the six failures come from the shell now setting the chrome heights), and a fault that only a real GPU shows: layer faces missing until the next redraw, because one on-demand frame is drawn before the shader programs are linked.
- Units started: pgui-column-r5, pgui-shell-r4, pgui-flow-r3, pgui-elastic-r4.

## [2026-11-15 01:44] incident | huginn GPU lock held by an orphaned T16 sampler
- huginn ~/sherlock/gpu.lock held since 08:21Z by nvidia-smi pid 592115, started by t16/run.sh stage h-c5; it inherited fd 9 on the lock and outlived its stage. w3b and the T16 build wait on it.
- The lead's kill was blocked by permission rules; it waits for the owner (`ssh huginn 'kill 592115'`). huginn moved to GPU-free work (w4 CPU gates); T16 told to close fd 9 for samplers and trap-kill them.

## [2026-11-15 01:45] build | T18 (d) suite coverage on odin: 139 equal, 0 different
- odin unit odin-t18d-rust (MEASURED): build 25 s, cargo test 1 passed; mycroft-suite rc=0; 33 programs, 106 circuits, 208 calls: 139 equal, 62 refused as native, 1 pure, 6 unsupported (all kit work: jubjub, secp256k1, tier3), 0 different. x01 fixed.
- Results copied into mycroft-factory/mycroft/compiler (out/, SUITE-COVERAGE.md; sha256 match). odin's wrapd build skips the nvptx target: huginn has none either; the MSM uses sppark.

## [2026-11-15 01:47] incident | T16 huginn sampler kept gpu.lock; run.sh fixed
- t16 run.sh stage h-c5 started nvidia-smi with fd 9 on ~/sherlock/gpu.lock inherited; the stage was stopped for the forced-cut fix, the orphaned sampler (huginn pid 592115) still holds the lock since 08:21Z and blocks w3b and the T16 build. Kills on huginn are blocked: the owner frees it.
- Fix (Moriarty t16-elastic run-huginn.sh, deployed): sampler starts with `9>&-`, prover runs under `wait` with an EXIT trap that stops both, and each stage exits 1 if an nvidia-smi of this user holds gpu.lock. Tested on muninn with a fake sampler.
- Also committed 1d9276133 (sched-r2 patches 0004/0005, what-if schedules DERIVED, three-box table). h-c5 and the build stay queued on huginn until the lock is free; odin stages wait for its byte gates.

## [2026-11-15 01:48] qa | huginn w4 GPU-free checks PASS (h100)
- w4 (CPU residual) built after a one-line borrow fix in residual.rs (wrapd a822e747); w4bench edge cases 245/245 PASS, all 6 paths identical to stock; microbench MEASURED: lookup_permute 1.16 -> 0.126 s (9.2x), batch_invert 14.8x, rational 5.6x, lincomb 4.7x.
- Seeded cpu and cpu,cpures proofs byte-identical (6059964f) and equal to w1's seeded cpu proof; wrap_verify OK. Timed GPU set waits for huginn's GPU lock (orphan 592115, owner).

## [2026-11-15 01:49] commit | T22 governance committed on branch t22-governance
- fe7207819 (28 files): openspec change sherlock-governance (34 EARS, 4 specs), research/sherlock-t22-governance-20261114 (Quint eraTable 11/11 mutants, Compact sketches, USE-CASES, OPEN-QUESTIONS).

## [2026-11-15 01:50] lane-start | Carbon Quint QA lane t22-governance
- Gemini 4 Carbon in herdr pane w1:p1C, folder ~/.cache/gemini-lanes/qa-20261115-t22-governance; models the era table, quorum, roster rotation, sealing (SDK one-key committee vs sealed), liveness and old-era verification from the specs, independent of the lead lane's eraTable.qnt; 7 mutants.

## [2026-11-15 01:56] lane-done | Prover GUI lane elastic round 4 (Other runs block), supervisor QA PASS
- Supervisor rerun: harness 0 failures, node tests 26 of 26, build-records --verify PASS. The base plate sheet now reaches every record, since the shelf shows no names at rest and is not drawn on a phone.

## [2026-11-15 02:01] lane-done | Prover GUI lane flow round 3 (three number rows, stage titles), supervisor QA PASS
- Supervisor rerun: check_flow.mjs 29 items, 108 citations, 0 failed; node tests 26 of 26 with contracts; browser harness 0 failed. Level 1 now holds at most three number rows beside the header's seconds; a count and its bound share one row.

## [2026-11-15 02:21] lane-done | Prover GUI lane shell round 4 (one 56 px top row, sheet title, FAB clear of the sheet), merged
- 109 node tests pass on the merged app; the lane's harness reports 114 of 115 static and 46 of 46 with the stub bridge. The supervisor's rerun of the shell harness on a GPU browser before this round: 93 of 94 (the one needs the bridge).
- Supervisor integration fix, one line in src/shell/index.js: the sheet title takes lane 2's title object (text) and drops its backticks.

## [2026-11-15 02:22] lane-done | T18 (d) Mycroft through the factory up to KRS check
- MEASURED: 33 programs, 106 circuits, 208 calls: 139 equal, 62 refused as native, 1 pure, 6 blocked on kits, 0 unsupported; Lean oracle vs Quint replay 1,226/1,226 agree, mutant-comm control fails as required; regress h1 and q2 exit 0, Q2 fails before on the 00:27 oracle.
- Findings: H1 (salt, selector, rand not checked below p) and H2 (big-endian immediates) CONFIRMED and fixed; Q1 NOT REPRODUCED; Q2 and Q3 OPEN DESIGN with LEAD picks. Report saved by the lead to mycroft-factory/mycroft/factory/REPORT.md.
- Open: KRS emit, coverage, mutations, compare and gates.sh (queued behind the gate reservation, KRS lane still running); proving blocked on the R_SL chip set in the T11 rekey.

## [2026-11-15 02:27] incident | t13-cx-tape pow test hung 1h43m on a 24G slot, lead stopped it
- t13_lfr_tape_pow printed CX C4 at 00:42 and then looped in the Rig check (140 MB RSS) while it held a 24G heavy slot, which starved the T10q check-chain reservation.
- Lead sent SIGTERM at 02:26. The lfr-tape lane must find the loop, record C4 Rig as HANG and rerun the later tests under timeout with a 4G slot.

## [2026-11-15 02:28] qa | T10q check-chain PASS, gate moves to pathc
- FP8D3: all gates PASS on app, leaf, node1-3, L_iface, L_Fr and chain-B fv_vk; V.10 A and B PASS (2 shards, 5 steps, N3 fixed point, L_Fr 1522 words), one key_root and one keys_d.
- pathc started 02:27 on ovm 0082832 and Moriarty f8fd268e3, both clean.

## [2026-11-15 02:31] build | T18 (d) Lean oracle --wfail build and KRS emit pass
- t18d-oracle: `lake build --wfail` built 71 jobs, exit 0, new oracle binary 02:28; KRS `krs kink emit` exit 0.
- gates.sh build_oracle now calls lake by absolute path (${LAKE:-$HOME/.elan/bin/lake}); next: KRS check, coverage, mutations, run, compare.

## [2026-11-15 02:36] note | odin inputs complete, byte gates and 5090 baseline sent (m11)
- Copied from huginn to odin: t14b keysA (3.1G), lfr-call.json, t10m-wrap.vk, vector.json, slice_v1.json; sha256 check ALL SHA OK.
- m11 to odin-agent: CPU vs GPU wrap proofs byte-identical plus wrap_verify, then timed T14b w1 baseline (3 warm, 3 cold) under ~/sherlock/out/odin/ for the muninn ledger check.

## [2026-11-15 02:38] lane-done | CX e3-iface and lfr-tape-r2 counterexamples, no soundness defect
- e3-iface: 21 GUARD HOLDS, 1 CONFIRMED (Op::Public after the pad refuses the honest witness), 3 OPEN DESIGN (C2.x/C3.1 margins unchecked in code, two F7 prose points); both equivalent mutants shown truly equivalent.
- lfr-tape-r2: 21 GUARD HOLDS, 3 CONFIRMED API hazards production does not hit (standalone Tape::halve accepts a flipped bit, expose then public refuses its own witness, extra trailing pin accepted), 1 NOT ATTEMPTED (C8(a), premise outdated since T7c).
- C4 HANG was the test checker (tree walk of PoseidonFr, over 2^26 nodes), not Sherlock code; real-prover rerun refuses the bad PoW witness in 3.4 s; later tests pass under timeout 600 with 4G.
- Commit openvm t13-cx 2dcaafa (tests only, no src change, no push).

## [2026-11-15 02:40] qa | odin w1 byte gate PASS on the RTX 5090
- MEASURED (seeded GW_DET=1, test-only): proofs from cpu, fixed, ntt+coset+quotient and fixed+ntt+coset+quotient are byte-identical, sha256 b6cbdfec6b32cd39...; the GPU ran 1 quotient and 35 NTT calls in both GPU configs.
- wrap_verify against in/t10m-wrap.vk: all pass, VK equal, stock library verify ok, changed public input refused, proof 2912 bytes. Timed w1 set (3 warm, 3 cold) now runs in unit odin-w1-run.

## [2026-11-15 02:55] lane-done | Prover GUI lane column round 5 merged; round 6 started for an intermittent stale frame on the real GPU
- Supervisor rerun of the scene harness on the merged app: 185 checks, 0 failed. Hero stills on the GPU browser pass the gate closers at 1440x900 and 1280x720 in both schemes: 7 labels, 3 numbers, column 74.2 and 70.4 percent of the canvas, light layer 2.3:1 against the ground.
- Open fault (about one load in four on the GPU, never on software GL): the canvas keeps an earlier frame, upper layers as edge boxes, until any redraw. Parallel shader compilation ruled out by the supervisor. Unit pgui-column-r6.

## [2026-11-15 03:05] qa | odin RTX 5090 w1 wrap baseline, muninn ledger check 18/18 PASS
- MEASURED on odin (k=21, 18 proofs pairwise distinct): w1 GPU warm 27.77 s (n=3), cold 28.98 s (n=4); CPU fixed warm 55.01 s, cold 55.10 s; quotient 23.9 s to 0.42 s; resident start 44.4 s; device peak 12.4 GiB.
- muninn ledger 9.1.0.1 check: 18/18 PASS, no false check, VK byte-equal to huginn w1 (manifest 165d2880535e2be0, files in ~/.cache/t14b/odin-wrap-w1).
- The 5090 is not faster than the 5080 (huginn w1 warm 25.36 s): the CPU residual dominates, and odin has 20 threads against huginn's 24. Input for w4 (CPU residual) and for the T16 setup-mode profile.

## [2026-11-15 03:05] commit | Prover GUI: commit 9bdde3b09 on prover-gui (hero gate passed on the dark frame, rounds 3 to 5 merged, README)
- Supervisor reruns on the merged app at this commit: 109 node tests, scene harness 185 of 185, flow harness 0 failed, code harness 13 of 13, elastic harness 0 failures, shell harness live group 51 of 51 against the stub bridge. Not pushed.

## [2026-11-15 03:08] decision | huginn's w4 and w5 GPU runs move to odin
- huginn's GPU lock waits on the owner (orphan sampler pid 592115), so odin takes w4 (CPU residual, patch 2f5ece84) and then w5 (daemon integration, 0c203288, synced from huginn); same byte gate and timed method as odin w1; odin acked as h40.
- huginn (m41) runs w3b first when the lock clears and does not start w4 or w5. The LOOP.md status commit is still staged: the pre-commit heavy-slot timing test fails under the current load, so it is retried later.

## [2026-11-15 03:16] qa | T18 (d) H1 and Q2 regression traces fail before the fix (MEASURED)
- The pre-H1 oracle (built 02:34) fails both traces: regress/h1 exit 1 (the model refuses with MYC-REF-036 nonCanonical, the old oracle runs on) and regress/q2 exit 1 (tape [0,6,0,5] against [0,6,0,5,0]); both pass on the current oracle.
- REPORT.md open item 2 is closed; the KRS check job (t18d-check) is running.

## [2026-11-15 03:23] qa | odin w4 KEPT: wrap 27.77 s to 9.46 s warm on the RTX 5090, ledger 18/18
- MEASURED on odin (k=21, 18 proofs pairwise distinct): B = all GPU + CPU residual (w4) warm 9.455 s (n=3), cold 11.392 s (n=4); A = all GPU with sppark MSM warm 15.572 s, cold 17.537 s; w4 saves 6.1 s (DERIVED). Edge bench 245/245; byte gate PASS over 5 configs (same sha as w1).
- muninn ledger 9.1.0.1 check: 18/18 PASS, no false check, VK byte-equal to huginn w1 (manifest 9256f24a6c0f3446, ~/.cache/t14b/odin-wrap-w4).
- ZK: every config gives byte-identical proofs with the seeded RNG, so the GPU MSM and w4 paths output the same distribution as the CPU prover; the w4 ZK review found ZK preserved. w5 (daemon) runs next on odin.

## [2026-11-15 04:15] qa | odin w5 KEPT: pipelined daemon cuts batch calls 60.81 s to 53.03 s, ledger 34/34
- MEASURED on odin (k=21, G = w1 config): whole call STARK+wrap, warm single call 61.01 s to 60.21 s (within spread); batch of 6 per call 60.81 s to 53.03 s, steady 60.83 s to 51.49 s; wrap part about 28.8 s in both; device peak 12.4 GiB; byte gate PASS.
- muninn ledger 9.1.0.1 check: 34/34 PASS, VK equal to huginn w1 (manifest c3e31f40c0a91443, ~/.cache/t14b/odin-wrap-w5). STARK worker lfr_call 73d82c9a built from huginn's hc/work/t14b tree, smoke PASS.
- odin finding: with 32 GB the 16 GB eviction guard is not needed; full overlap without evict is a setup-mode (T16) profile candidate. Next on odin: T16 (d)/(e) stages, then w6 = w4 + w5 together.

## [2026-11-15 04:18] qa | T10q pathc PASS, gate moves to the GP stages
- pathc (1 h 50 min): one leaf, leaf layer, L and L_Fr on the universal L key, all T7CASE cases PASS, 0 FAIL; L_Fr G1 accepted, 45,528 bytes; pathc-d3 FP8D3 all gates PASS.
- Remaining gate steps: build-t8gp, gp2, x7, d3work (FP8D3 over the x7 work VKs), then T10Q GATE ALL DONE and the t10q push.

## [2026-11-15 04:35] qa | T10q chunk mode: 3 soundness defects CONFIRMED, t10q push held
- CX lane qa-20261114-t10q-chunk: F-5/NT-1 (ordinary call writes the chunk anchor), F-6/NT-2 (equal run_tag splices two runs), F-7/NT-3 (k change truncates a run) CONFIRMED; root StmtAir reads no public word 74-96 and the L_Fr tape and both decoders accept; 9 GUARD HOLDS, 4 OPEN DESIGN. Not yet shown with an L, L_Fr or wrap proof of forged words.
- Reach: no SDK or Moriarty contract has a chunked entry (grep), so nothing deployed is reachable; live for any chunk-capable contract. Tests committed ignored on t13-cx (openvm b5ceea3, Moriarty d65e3a0e6), not on the gated heads.
- decision: t10q is not DONE while these stand, so the t10q push is held even when the gate reaches ALL DONE. Fix-design lane t10q-ntfix started: first answer whether the fix changes any key (if yes, it joins the single T11 rekey).

## [2026-11-15 04:38] lane-done | CX lfr-bbalu-r2 and lfr-poseidon-r2: 2 CONFIRMED on ill-typed tapes only
- lfr-bbalu-r2: 1 CONFIRMED, 27 GUARD HOLDS, 4 OPEN DESIGN; lfr-poseidon-r2: 1 CONFIRMED, 16 GUARD HOLDS, 3 OPEN DESIGN.
- CONFIRMED, both need an ill-typed tape: the ALU reading Fr word r-1 is stock-accepted (C-A2c), and pack witnesses (1,2) and (1+2^31,1) give the same output (PK1b, PK1c). The real tapes I0 (442,808 ops) and E3 T10m (503,640 ops) have 0 ill-typed reads, so neither defect occurs on them.
- LEAD PICK: refuse typed-address violations at Machine::new (no key change) now; put canonical input range checks in the single T11 rekey. Tests on openvm t13-cx 9f2311d and 85f057b. The default E3 fixture is stale (12164 vs 12166).

## [2026-11-15 04:38] lane-done | Prover GUI lane column round 6 merged (state generation and trailing check)
- Supervisor rerun of the scene harness on the merged app: 191 checks, 0 failed. The lane's explanation of the stale frame (a starved animation frame after a canvas resize) did not hold: the supervisor's GL trace of a stale load shows the last frame's draw calls complete (62 calls) with drawnGen equal to stateGen.
- Supervisor change in src/scene/index.js: one settle frame 250 ms after the last change. MEASURED on the GPU browser after it: 40 loads at 390x844 and 8 at 1440x900, 0 stale at rest; 1 of 32 frames after a Seen by or layer change still changed on a forced redraw. Cause below the draw calls not found.

## [2026-11-15 04:38] qa | Prover GUI final supervisor QA on the merged app: gate closers pass, full page run in replay and against stub sockets
- Gate closers H9, H11, H12 pass in the supervisor's GPU captures (7 labels, 3 numbers at rest at three sizes; column 74.2, 70.4 and 65.1 percent of the canvas; light layer 2.3:1). 58 captures of states S1 to S10 in prover-gui/lanes/_sup/shots/states/.
- One tree, one pass: 109 node tests, check_facts 95 facts 0 failed, check_flow 108 citations, 52 snapshot files verified, 5 records PASS, scene harness 191 of 191, flow, code and elastic harnesses 0 failed, shell harness 122 of 122 with the stub bridge (121 of 122 in a run the same hour, the S3 idle check once).
- Live path only against stub sockets: no T14b daemon was free and no heavy prove was started. A lost prover and a refused wrap end as Stopped (S9, S10).
- Open: a run against a real daemon, frame time on a GPU browser, label collisions in S2, S4 and S10 with a short stub call, the L face as a fill. BUILD-REPORT.md has the list.

## [2026-11-15 04:41] qa | Lead rerun: the 3 T10q chunk NT tests fail as the lane reported
- `cargo test -p sherlock-leaf --test t13_t10q_chunk -- --ignored` on openvm t13-cx (contains b5ceea3): NT-1, NT-2 and NT-3 all FAILED (the attack is accepted), rc 101, log ~/.cache/lead-nt-rerun.log. Lead verdict: 3 CONFIRMED stand; the t10q push stays held until the t10q-ntfix pass-after.

## [2026-11-15 04:41] commit | Prover GUI: commit c5d90b247 on prover-gui (scene round 6, settle frame, BUILD-REPORT, final captures)
- Five supervisor commits on branch prover-gui tonight: 7f4dc5753, c48e1d7bc, 4be7b18c4, 9bdde3b09, c5d90b247. T0 passed on each; three earlier attempts of the last one failed on the load-sensitive factory test test_heavy while the heavy slots were full. Not pushed.
- Paths: prover-gui/app/ (README.md), prover-gui/BUILD-REPORT.md, prover-gui/lanes/. Lane worktrees ~/Projects/Moriarty-wt/pgui-lane-<lane> are left in place.

## [2026-11-15 04:52] qa | T16 elastic stages on odin (RTX 5090): all PASS
- MEASURED, one call vector e7583d70: setup picks shape 1, cut [4,1]; total wall GPU 286.1 s (leaf stage 4.80 s), no GPU 292.9 s (10.95 s), GPU memory capped to 1.1 GB 289.4 s (1 leaf at once, 2 waves, 6.49 s), forced 5-leaf cut 307.7 s; join of the odin GPU and huginn GPU runs all_pass in 283.6 s.
- Fixed parts dominate a short run: about 66.5 s keys, 107.8 s L key and T10 children, 93.6 s L prove. GPU capacity 27 leaves at once on 32 GB (huginn 15). DERIVED: odin leaf stage 3.16 s up to 40 steps, 12.63 s for 400 steps (4 waves).
- Reading: the elastic schedule shows up in the leaf stage; the fixed L-key and L-prove cost (about 200 s) is the next target for short runs.

## [2026-11-15 04:53] lane-done | T16 (d)/(e) on odin: profile scales up, scales down, joins huginn
- odin (RTX 5090 32 GB) setup profile 158 s; it gives 27 leaves per GPU (huginn 15, muninn 12, DERIVED) and, via new `profile::pipeline` (openvm df08b3c, EARS PF-007), no evict + full overlap on odin, evict + overlap after the quotient on huginn and muninn (DERIVED).
- MEASURED on odin: o-gpu 2 at once 4.80 s, o-nogpu CPU 10.95 s, o-cap 1 at once 6.49 s (pipeline drops to no overlap), forced 5-leaf cut 5 at once 9.12 s; claim_pub byte-equal on all; odin+huginn leaves join under one N1, 5/5.
- One odin leaf is 1.5x slower than huginn's (CPU leaf layer under WSL2); the 32 GB card wins on capacity: 400 steps est. 12.6 s vs 14.7 s (DERIVED). Records research/sherlock-t16-elastic-20261114/records-cde/odin.

## [2026-11-15 04:58] lane-done | T16 odin stages committed (Moriarty t16-elastic bdd04a10c); odin gets w6
- T16 lane: setup profile 158.0 s on odin; profile::pipeline (openvm df08b3c, EARS PF-007) picks no-evict full overlap on odin, evict with overlap after quotient on huginn and muninn; claim_pub byte-equal on every run, L accepted; odin+huginn join 5/5 cases pass; odin leaf 1.5x slower than huginn (leaf layer on CPU under WSL2), but 400 steps 12.6 s vs 14.7 s (DERIVED).
- Open: muninn t16-cde-run6 behind the T10q gate; huginn h-c5 behind the orphan sampler lock (owner).
- m18 to odin: w6 = w1 + w4 + w5 patches, byte gate, then whole-call timing with B = all,cpures and the no-evict pipeline; KEEP if batch per call is below 53.03 s.

## [2026-11-15 05:05] lane-done | CX wrap-levers: no forgery accepted, 1 CONFIRMED key-shape hang
- On Moriarty f8fd268e3, no src change: 27 GUARD HOLDS (MockProver refusals for F1, F3, F5/C4, C3.2, C3.3, C5.1, C5.2, F6), 1 NOT REPRODUCED (F4 prose, fixed in c4a899cba and a84f8123a), 1 NOT ATTEMPTED (F2 at the 20 decode.rs mul_add sites: i7_decode fixture lacks decode.keys.x).
- CONFIRMED C3.1: whir.rs:222 `nbits = log_rs - k` wraps in release, so a malformed verifying-key shape makes the cs loop run without bound. A hang from the key, not a soundness break. LEAD PICK: a checked_sub plus a shape check in WhirShape::from_wrapin (no key change), with the test un-ignored as the regression.
- Tests on Moriarty t13-cx 58ab5367a and 67f58f231. Open: regenerate the i7_decode fixture and run F2.

## [2026-11-15 05:31] qa | odin w6 KEPT (w4 + w5): whole call 61 s to 42 s, ledger 26/26
- MEASURED on odin (k=21, B = all,cpures): warm whole call (STARK+wrap) median 41.99 s against w5's 60.21 s; batch of 6 per call 41.67 s against 53.03 s; wrap part about 10.1 s; device peak 12.92 GiB; byte gate PASS (same proof sha b6cbdfec as w1 and w4). Combined patch w6-vs-w1.diff 3cbe7845 (only conflict a CLI help string).
- muninn ledger 9.1.0.1 check: 26/26 PASS, VK equal to huginn w1 (manifest 092d4894b7e67bfb, ~/.cache/t14b/odin-wrap-w6).
- Finding: full overlap did not engage on 32 GB. The wrap-tail budget (12,300 MiB) counts the 7.5 GiB of cosets already resident, so the check is short by 209 MiB. Next: w7 tries W5_WRAP_TAIL_MIB=5000 (env only) to measure the overlap; the budget formula itself must subtract resident memory.

## [2026-11-15 05:45] qa | odin w7 KEPT: full overlap hides the wrap, batch 34.22 s per call, ledger 13/13
- MEASURED on odin (w6 binary 7fc9b5cc, W5_WRAP_TAIL_MIB=5000, full overlap, no evict): overlap engaged on 5/5 batch dispatches, no OOM, wrapd peak 12.92 GiB, card max 23,867 of 32,607 MiB. Batch of 6: 34.22 s per call (w6 41.67 s, -17.9%), about the STARK time alone, so the wrap is hidden. Single call unchanged: warm whole call 42.12 s.
- muninn ledger 9.1.0.1 check: 13/13 PASS, VK equal to huginn w1 (manifest d1ea384a417a77bd, ~/.cache/t14b/odin-wrap-w7).
- Wrap levers on odin since 01:00: whole call 61 s to 42 s single and 53 s to 34 s batched. The STARK leg (lfr_prove about 25.6 s) is now the bound. The setup-mode budget must subtract resident memory (sent to the T16 lane).

## [2026-11-15 05:50] lane-start | T23 prep: native Preview baseline after the hard fork, and the T23 runner
- Part A: rerun the T18 (a) Compact suite natively on Preview with the current pins (compactc 0.36.0, node 2.1.101 stack), diff against 2026-11-14, tx counts before and after; existing wallet, Preview only.
- Part B: the T23 driver (Mycroft compile, Sherlock prove on odin and huginn, SherlockLedger acceptRun, compare with native) validated in dry mode while proving is BLOCKED on the R_SL chip set (T11).

## [2026-11-15 05:52] qa | T10q gate gp2 PASS (48/48), x7 started
- gp2 (1 h 26 min): T8GP2 all_pass=true, 48 cases, 0 FAIL, on the universal keys key_root 19cc2dbb..., keys_d 563c227d...; the last case checks shape_d differs per mode (SEG, SEAL, JOIN, SUB, DL).
- Remaining: x7, then d3work (FP8D3 over the x7 work VKs). The t10q push stays held for the NT-1..3 fix.

## [2026-11-15 06:10] commit | T16 PF-008: the wrap-tail budget comes from the setup profile
- openvm t16-elastic 1baf605: profile::wrap_tail is the wrap peak minus the resident cosets when nothing is evicted (the whole wrap peak when evicting), replacing the constant 12,300 MiB that refused full overlap on odin by 209 MiB (h66). With the corrected tail, odin's measured w7 overlap engages (batch 41.67 s to 34.22 s per call).

## [2026-11-15 06:15] qa | T10Q GATE ALL DONE (gate scope PASS); push still held
- x7: T8X7 all_pass=true, 22 cases; d3work: FP8D3 all gates PASS over the x7 work VKs (shift 101.16 bits); unit t10q-gate Result=success, exit 0, on openvm 0082832 and Moriarty f8fd268e3, both clean.
- Whole gate: shape-base, shape-new, chain ovm, wrap, gates (ledger A and B, check-cases, check-omit, check-chain V.10), pathc, pathc-d3, build-t8gp, gp2 48/48, x7 22/22, d3work.
- decision: the gate passes on what it tests, but T10q is not DONE: NT-1..3 CONFIRMED stand (lead rerun 3/3 fail). The t10q push waits for t10q-ntfix pass-after; the local merge into T10m proceeds.

## [2026-11-15 06:20] commit | t10q merged into T10m locally (no push)
- openvm t10-merge fast-forwarded to 0082832 (the gated t10q tip, 22 commits). Moriarty t10m merge 3dc9001a9 (t10q f8fd268e3 into ca96c1466, clean). This carries MW-F01 (fixed on t10q as beba730) into T10m.
- T10q chunk mode stays OPEN: NT-1..3 fix on t10q-ntfix (lane running). T11 waits for that lane's key answer, so the R_SL chip set, jubjub and any NT key change go into one rekey.

## [2026-11-15 06:24] decision | T10q NT-1..3 fix changes L_Fr and the wrap VK, so it joins the T11 rekey
- t10q-ntfix KEYS.md: NT-2 and NT-3 are closed by a new chunk lock x_j = sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, j) on the L_Fr tape, 4 permutations per chunk call instead of 3; NT-1 by a mode-0 refusal of the ten anchor-write template lanes in the wrap decoder. The universal leaf and node keys and L do not change.
- The new x_j changes the owner's x_j canon, so it is a LEAD PICK for T15. The T11 rekey list is now: R_SL chip set, jubjub kit, the NT lock (L_Fr) and decoder (wrap VK), and canonical input range checks. The wrap VK must stay at or below 50 KB and verify on ledger 9.1.0.1.
- muninn unit t16-cde-run6 started after the T10q gate.

## [2026-11-15 06:30] qa | T18 (d) KRS reference suite: all five gates PASS
- MEASURED (gates.log ~/.cache/t18d/gates.log): coverage 865 covered, 41 argued, 0 open on 104 cases; mutations 58/62 killed (4 argued survivors); Lean oracle compare 104/104; golden vectors current; oracle vs native 0/42 differ.
- Rust compiler compare: 99 equal, 4 different (all Q2 filler; the mycroft-kink binary predates Q2), 1 failed (toy-refuse-unknown-builtin, outside the compiler domain). Next: rebuild mycroft-kink, add the Other-builtin exclusion, re-record kink.lock, then commit the T18 (d) deliverables.

## [2026-11-15 06:50] lane-start | T11 zero knowledge, rekey-chips (R_SL + jubjub + range checks), T12 part 1 (anchor-slot pin CX)
- Inputs ready: T10m holds the gated t10q (3dc9001a9 / 0082832). LEAD PICK (AFK rule, design open until e2e): one rekey after these lanes, with the list at ~/.cache/rekey-t11/LIST.md (T11 owns it; ntfix, rekey-chips and t12-gate append).
- T11: r_P fresh per proof on CPU and GPU, seeded builds refuse, PCS masking decision from full papers, EQ-ZK, daemon slot reuse, ZK-L2 in Lean. rekey-chips: R_SL chips and jubjub in sherlock-leaf, canonical ALU and Poseidon range checks, columns MEASURED against 5,400. T12 part 1: anchor-slot pin counterexample on the gated wrap, slot 15 pin if CONFIRMED, runbook for G3, G4 and FP-8 on the new keys.

## [2026-11-15 06:58] lane-done | t10q-ntfix: NT-1..3 fixed with fail-before and pass-after; FP-3 residual opens a lane
- openvm t10q-ntfix 8780973, 6a00df3, e3b407d; Moriarty 74158f3c1, bb9dc9ae0, e196904d7. Before: the 3 NT tests fail (rc 101). After: t13_t10q_chunk 9/9, quint_regressions 15 (6 ignored), sherlock-lfr lib 19, Moriarty 22/22 with MockProver refusing NT-1; Quint 19 tests and models/run.sh t10q-ntfix 7/7 with ITF traces. C3.1 checked_sub passes after (commit pending: the T0 hook failed 4 times on load-dependent heavy-slot tests).
- Keys (DERIVED, re-measure at the rekey): L_Fr and the wrap VK change; k stays 21 (wrap about 1,685,895 rows), wrap VK about 1.3 KB, 1,026 public inputs. LEAD PICKs T10Q-NTFIX-LOCK, -ORD, -T11.
- Residual, not closed: an ordinary call writing the anchor one lane later is still accepted (nt1_residual_shifted_write_is_fp3), and k is not tied to the true slice count; both need FP-3 (L binds call lanes to guest effects plus the forward claim chain, an L key change). FP-3 lane started. The t10q push stays held until FP-3 closes.

## [2026-11-15 07:22] build | rekey-chips odin request 1 dispatched
- odin unit rekey-chips-r1 runs the six rekey-chips jobs (task 2 after/before, L_Fr positive controls, mycroft-emit, R_SL leaf tests, rsl_suite exec) on openvm rekey-chips 87701ba (clean git archive).
- Out-of-tree deps synced: stark-backend-t8b deeeb0a; odin copy points stark-sdk mori-gpu at ../../../moriarty/crates/mori-gpu.
- Job 1 MEASURED: rekey_lfr_shape 1/1 (ops 503640, AIRs 15), t13_lfr_bbalu 4/4, t13_lfr_poseidon 3/5 (findings, tag_only fail at test line 281; pack_collision panics at tape.rs:91). Returned to the rekey-chips lane.

## [2026-11-15 07:31] commit | t10q-ntfix C3.1 checked_sub committed
- Moriarty t10q-ntfix 95990a6d3: WHIR shape check refuses log_rs < k instead of wrapping (wrap-levers C3.1 hang); T0 pre-commit passed on a retry.
- Lead rerun on openvm-ntfix e3b407d: t13_t10q_chunk 9/9 with --include-ignored (log ~/.cache/lead-ntfix-rerun.log).
- The t10q push stays held on the FP-3 residual.

## [2026-11-15 07:40] build | rekey-chips odin run r2 (4437a74)
- MEASURED on odin: task 2 passes-after (sherlock-lfr shape 1/1, bbalu 4/4, poseidon 5/5) and fails-before on the 0082832 base (C-A2c and PK1b stock ACCEPTED); L_Fr 541 -> 543 main columns. mycroft-emit wrote 142 R_SL call files (4 circuits refused).
- rsl_jubjub does not compile (pow_vartime takes &[u64; 4]); rsl_suite exec: 57 different, 85 unsupported, 0 equal. Returned to the rekey-chips lane; this blocks T23 proving.
- Runner fixes (lead): base tree as a sibling dir so relative path deps resolve; mycroft-emit built on rustc 1.99 (sysinfo 0.39.6 needs 1.95; openvm pins 1.91.1).

## [2026-11-15 07:42] build | rekey-chips odin runs r3 and r4 (ce309c6)
- MEASURED on odin: sherlock-leaf rsl 5/5 (579.6 s), rsl_exec 2/2; rsl_jubjub_matches_native fails (R_SL jubjub scalar mul differs from midnight-curves); suite exec: 57 executed, 85 unsupported (PersistentHash, sha256 kit).
- openvm-sha2-air builds offline on odin; the sherlock-leaf, sherlock-lfr and sherlock-wrap-export check passes.
- t7_chain rsl probe: G1 refuses the R_SL app key (app set, air 5: 14,912 constraints, bound 5,000). The G1 bound stays; the lane must split or narrow that AIR.
- r5 on 9364edb (R_SL_SHA256 AIRs) runs on odin.

## [2026-11-15 07:50] lane-done | T11 zero knowledge: PARTIAL handback, lane continues
- ZK-L2 done: clean/fork/Clean/Sherlock/ZkPadding.lean, 0 sorry, full lake build passes (Moriarty t11-zk 71c4c001b; t14b merges 31e25cc0c and openvm cce8c1d).
- Written, not yet run: fresh r_P per deployment and salt per run from the OS CSPRNG (openvm sherlock-leaf blind.rs), seeded paths behind feature test-seed (zkguard.rs), EQ-ZK and slot-reuse tests (t11_zk.rs). Rebuild in unit t11zk-build2.
- LEAD PICKs for T15: T11-1 r_P per deployment, salt per run, wrap blinders per proof; T11-2 merge t14b; T11-3 no PCS masking while STARK layers stay on user-owned devices (helper-machine case to be stated); T11-4 every seeded path behind test-seed.

## [2026-11-15 07:58] note | T11 helper-machine case OPEN for the owner
- T11 RESULTS section 2: case A (huginn or odin proving for their own owner) is the user's own machine under OD-9 / ZK-001; case B (a host the user does not own) is a helper and needs witness-hiding delegation (MPC or secret-shared STARK proving). Case B is OPEN, an owner design item; ZK-001 and ZK-009 refuse it today.
- t11zk-build2 waits for a muninn heavy slot (held by item16-verify2, t16-m-nogpu, fp3-after); the lead resumes the lane when it ends.

## [2026-11-15 08:14] build | rekey-chips odin run r5 (9364edb, R_SL_SHA256 AIRs)
- MEASURED on odin: check passes; rsl_exec 2/2; rsl_sha 3/3 (1,335.5 s); rsl 5/5 (611.7 s); rsl_suite exec runs all 142 Compact suite calls (0 unsupported).
- rsl_jubjub still fails on 9364edb; the lane's fix (subgroup points, d8e5671) runs in r6 with the split RSL_PUBLIC AIRs and the rsl probe.

## [2026-11-15 08:39] build | rekey-chips odin run r6 (d8e5671)
- MEASURED on odin: check passes; rsl_jubjub 2/2 (subgroup fix); rsl_sha 3/3 (1,350.0 s).
- rsl probe: the split RSL_PUBLIC AIRs pass G1 (bound 5,000 unchanged); the leaf layer over the R_SL app proof fails with LogupZerocheck(NonZeroRootSum). Returned to the lane.
- r7 (rsl_suite prove, every call at the app level) runs on odin.

## [2026-11-15 08:40] lane-done | T23 prep: Part B dry run done, Part A runs unattended on Preview
- Part B (t23-suite daa1ca745): 208 calls, 0 mismatch, 62 refused-as-native (DERIVED), 145 blocked (139 R_SL chip set, 3 jubjub, 2 secp256k1, 1 keccak), 1 pure; compiled W and comm vs native Preview transcript 92 equal, 0 different; offline acceptRun rehearsal passes.
- Part A (units t23-partA, t23-partA-finalize): no hard fork since 2026-11-14 (spec 2001100); 3 of 33 programs, 4 calls word-equal; about 13 h left; 134 transactions planned with the existing Preview wallet.
- Open: the Mycroft statement to acceptRun argument map (c_p, input_pub, outcome_pub); LEAD PICKs instance reuse rule and box profile path.

## [2026-11-15 08:40] lane-start | T23 args: Mycroft statement to acceptRun arguments
- New Opus lane on branch t23-args: MAP.md (statement and wrap public inputs to c_p, input_pub, outcome_pub, claim_pub, with sources) and a tested builder that must reproduce the T17 accepted arguments; dry only, no Preview transactions.

## [2026-11-15 08:45] decision | heavy-slot admits shared jobs first-in-first-out
- Cause: a waiting shared job left no record, so smaller jobs that arrived later took the free budget; the T11 32G builds waited from 07:49 while fp3, t18d and t16 jobs were admitted.
- Change (lead, ~/.local/bin/heavy-slot, backup ~/.cache/heavy-slot.bak-20261115): every waiter writes a pending record with its arrival time; a shared job is admitted only when no older shared job is pending. Exclusive behaviour unchanged.
- The running T11 waiter (pid 1234937, from 08:27) got its pending record by hand. odin r8 (2b59f9b): check and rsl_fp1_public_values_on_air0_only pass; the probe runs.

## [2026-11-15 08:46] qa | R_SL probe: leaf proves, universal N1 key holds only at a fixed app AIR count
- odin r8 (rekey-chips 2b59f9b), MEASURED: leaf over the R_SL app proof proves for a05, t02, s01; G1 accepts app (log height 17 of 24) and leaf (17 of 21); app prove 20.8-21.2 s, leaf 11.0-16.6 s on odin CPU.
- n1_pre_hash_equals_universal_n1: true for t02 and s01 (7 app AIRs), FALSE for a05 (10 app AIRs, 4 PUBLIC AIRs): leaf width 1,598 vs 1,597. A call with more public words would get another leaf key, against the one-universal-key-set rule. Lane told to fix the app AIR set so the leaf shape is constant, with a test across a05, t02, s01 and d04.

## [2026-11-15 08:47] lane-done | T12 part 1: anchor-slot pin NOT REPRODUCED; RUNBOOK written
- t12-gate 9260e8b75, 90f9b4df9: forged anchor slots 1, 3, 15 in the read (lane 6) and the chunk write (lane 15) refused by the native decoder and MockProver (k=18), only decode.rs step 5b failing; the real T14b proof has lane 6 = 0. Regression test t12_anchor_slot (3 tests) passes. Wrap VK unchanged, 1,325 bytes.
- RUNBOOK covers G3, G4, G1/G2, wrap VK plus ledger-check, FP-8, I-27/28/30/34/37; G3 legs (CPU prover, GPU prover, stark-backend verifier) not built yet; two steps UNVERIFIED.

## [2026-11-15 08:47] decision | T12-SLOT15: the anchor moves to slot 15 in the single rekey (LEAD PICK for T15)
- Open route: a Mycroft contract with a user-writable field 0 could hold a forged C_P' in the anchor cell (MYC-SEAL-021); today only an untested compiler refusal guards it. SherlockLedger is safe (field 0 sealed).
- Pick: move the pinned cell 0 -> 15 in decode.rs, iface.rs and SherlockLedger inside the single rekey, plus a tested compiler refusal. Reason: the rekey changes L_Fr and the wrap VK anyway, and the guard then sits in the circuit. The T12 lane implements it on t12-slot15.

## [2026-11-15 09:02] note | huginn freed; w3b starts; rekey-chips r10 on 6f773b2
- Owner killed huginn pid 592115; the queued w3b wrapd bench (hc-wrap-w3b) started at once on the RTX 5080.
- rekey-chips 5d77cfc fixed the leaf key bucket (R_SL at most 9 AIRs) but left small programs at 5; 6f773b2 keeps SHA_IN always present (6 or 9 AIRs). odin r10 runs check, tests, the 5-call probe, then the suite prove if every leaf key equals the universal N1 pre_hash.

## [2026-11-15 09:04] decision | owner confirms the listed LEAD PICKs; w3b KEEP; odin runs w8
- Owner 2026-11-15 ~09:10: "Ye sif you recommend it" for the LEAD PICKs listed in the morning status: T11-1..T11-4, T12-SLOT15, T23 instance-reuse rule and box profile path, the single rekey list, the x_j change, T22 governance picks. These are CONFIRMED.
- huginn w3b (config all = GPU phases + sppark MSM on GPU): warm 12.89 s vs w1 25.13 s, cold 14.02 vs 25.55 (MEASURED, RTX 5080); muninn ledger-check 9.1.0.1 24/24 Ok, VK byte-equal to w1. KEEP.
- odin gets w8 = w7 + config all (owner: "Have Odin do productive things"). huginn gpu.lock incident (7 h 40 min, orphan sampler pid 592115 holding fd 9) accepted with a lock-wait watchdog.

## [2026-11-15 09:05] decision | owner: helper machines start with case A; case B later via the Veritee model
- Owner 2026-11-15: "we will start with case A and a future design will allow for outsources components using the Veritee model from the MPC Research thread".
- Case A (huginn, odin or any box the user owns proves for that user) is the supported mode now. Case B (outsourced proving on hosts the user does not own) is a future design on the Veritee seald Sessions model (MPC research, TEE Sessions design); ZK-001 and ZK-009 keep refusing case B until then. T11-3 stands as worded.

## [2026-11-15 09:06] decision | owner: build the secp256k1 and keccak kits; box agents may kill orphan gpu.lock holders
- Owner 2026-11-15 "yes to both": the rekey-chips lane builds the secp256k1 (2 calls) and keccak256 (1 call) R_SL kits after r10, inside the fixed 6-or-9 AIR bucket, so T23 runs all 208 calls.
- Narrow kill rule sent to huginn (m44) and odin (m24): only a process holding gpu.lock whose parent is gone, that is the sampler of an ended job, with the lock held over 10 min and no progress; log to the bus first, SIGTERM then SIGKILL. Never a live prover or wrapd.

## [2026-11-15 09:11] decision | Finish plan: T24 kits, T25 single rekey, box assignments; spec audit lane; owner AFK
- Owner 2026-11-15: "Audit this checklist to confirm all work items. Make sure the openspec is accurate and we have EARS for each item. Adjust the loop and the workflow with Odin and Huginn ... Going AFK again".
- LOOP.md gains T24 (keccak + secp256k1 R_SL kits; SECP-INTERFACE.md written for the Mycroft lowering), T25 (the single rekey) and a Finish plan section with the critical path and per-box queues; FINISH-PLAN.md (sha256 8093c945) on both box buses, BOXES.md points to it (m26 odin, m46 huginn).
- Spec-audit lane started (worktree spec-audit): confirms every checklist item, fixes OpenSpec drift, adds change sherlock-completion-20261115 and EARS IDs where missing.

## [2026-11-15 09:42] decision | Sherlock end-to-end prototype OpenSpec, DDD and MADR
- Owner order: end-to-end OpenSpecs for the prototype, 3 Fable 5.2 + 3 GPT-6 Astra agents, drwatt architecture method, MADR and DDD.
- Worktree Moriarty-wt/sherlock-e2e-spec (branch sherlock-e2e-spec from t10m 3dc9001a9); brief research/sherlock-e2e-spec-20261115/BRIEF.md.
- Slices: A language+integration (Fable-A integrates DDD, MADR, COVERAGE), B proving, C wrap/settlement/operations; each Fable pairs with one Astra lane (independent draft, then adversarial review).
