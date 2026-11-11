# Night plan 2026-11-11: from studies to a validated implementation spec

Owner, 2026-11-11 (going to sleep): "arm a loop to get this all done while I sleep. I should wake up to finished plans that are validated. Launch whatever you need".

The lead (Claude, the loop) drives this file. Tick a step only on evidence (file present, check passed, commit hash). Owner is AFK: take the recommended option, record every decision in `DECISIONS.md` in this folder, never ask questions. GP-1 to GP-4 are never narrowed. Store every finding in `llm-wiki/` in the same change.

## Phase A: studies and drafts
- [x] A1 Run-length study `CONSENSUS.md` written (agy pane w1:pC).  Evidence: CONSENSUS.md 11,684 words, written 2026-11-11.
- [x] A2 Fable 5.2 audit of A1 -> `research/sherlock-runlength-consensus-20261111/AUDIT.md`.  Evidence: AUDIT.md (adopt with changes; 3 blocker, 10 major, 7 minor); decisions R-1..R-10 in DECISIONS.md.
- [x] A3 Lookup-cache study rerun in pane w1:pC (keep lanes/B/round1.md; lanes A, C, D, E round 1; round 2 with LEAD-NOTE-ROUND2.md; CONSENSUS.md).  Evidence: CONSENSUS.md 2,895 words; all 10 lane files (A-E round1, round2); Profile A (dense Merkle + 8-record PAGE/ROM block cache + Poseidon boundary) recommended 5/5, Profile B (ZmLogUp + Horner) kept behind the interfaces.
- [x] A4 Fable 5.2 audit of A3 -> `research/sherlock-lookup-cache-20261111/AUDIT.md`.  Evidence: AUDIT.md (adopt with changes; 3 blocker, 11 major, 9 minor); decisions L-1..L-10.
- [x] A5 Three Astra drafts `drafts/astra-area{1,2,3}.md` (units astra-area1..3).  Evidence: units exited 0 at 05:38, 05:40, 05:43Z; 7467/7137/8182 words.
- [x] A6 Three Gemini drafts `drafts/gemini-area{1,2,3}.md` (pane w1:pC subagents).  Evidence: gemini-area1.md 7,333, gemini-area2.md 11,299, gemini-area3.md 12,240 words (02:44-02:45).

## Phase B: merged architecture
- [x] B1 Fable 5.2 merge of A1-A6 -> `ARCHITECTURE.md` (modules per MODULARITY.md; Lean obligations and the OpenSpec outline per ARCH-BRIEF.md addendum); dissent kept; decisions in `DECISIONS.md`.  Evidence: ARCHITECTURE.md (2,770 lines): 19 kits with all nine items, build order groups F, 0-7 and L, prerequisite experiments X1-X9, open decisions OD-1..OD-16 with OWNER marks, Lean table 14 proved / 41 open, OpenSpec outline, about 150 resolved conflicts and 10 unresolved (section 6.5).
- [x] B2 Independent audit of B1 by Astra (Codex, read-only) -> `ARCH-AUDIT-astra.md`; Fable fixes every admissible finding.  Evidence: ARCH-AUDIT-astra.md (GPT-6 Astra, adopt with changes: 4 blocker, 15 major, 1 minor); ARCH-AUDIT-RESPONSE.md: 20 of 20 admissible and fixed in ARCHITECTURE.md (now 3,536 lines, 23 kits); new OWNER items OD-17, OD-18, OD-20, OD-21; new experiments X10-X15.
- [x] B3 Commit and push (docs commit), wiki ingest.  Evidence: commit of ARCHITECTURE.md, audits and wiki (this commit; see git log).

## Phase C: experiments that decide the open choices
- [x] C1 Prototype both memory backends behind one MEM interface in `crates/mori-chip` (dense tail-buffered Merkle; write-once LogUp if B1 keeps it as a candidate): cost-model leaf rows at S = 16, 64, 128 and node rows; MockProver positive and negative tests. Through `scripts/doctorwatson heavy`, T0 before commit.  Done for the dense backend; the write-once LogUp backend is not built, by decision L-5 (benchmark-only, after M4). Progress: dense backend done (9973d633b on main): MemBackend trait, 9 positive tests, 21 mutants refused; measured flush unit 8,541/8,821 rows and old-memory read 4,621/4,761 at TOP 61/63; leaf rows ESTIMATE 14,942 (S=16), 36,543 (S=64), 58,565 (S=128). Open: LogUp backend, stacks, RfLeaf wiring, node rows measured.
- [ ] C2 Quint models for the chosen memory and boundary designs, run by a Gemini Quint lane (argon-run) and re-run by the lead.  Progress: dense-memory lane running (unit quint-c1-dense, folder ~/.cache/gemini-lanes/c1-dense).  Status: memory model done (qa/gemini-quint/c1-dense, lead re-run 30/30); the boundary commitment is designed but not built, so it has no model yet.
- [x] C3 Lean: statements (and proofs where they close in the night) for the partition-independence and modular-composition obligations in `clean/fork/Clean/Sherlock/`; `lake build` green with no new `sorry` outside a marked obligations file.  Evidence: d6304b4f0 on main; 52 theorems proved (PartitionIndependence.lean 41, ModularComposition.lean 11), no sorry, no new axiom; lake build SherlockSpec green (heavy c3-lake); F21 green at 90; EXPERIMENTS-C3.md. Open: leaf_sound not derived from the module theorem; 8-appends-per-leaf limit; len < 2^64; LogUp Schwartz-Zippel.
- [x] C4 Record results in `EXPERIMENTS.md`; update `DECISIONS.md` (memory backend, boundary commitment, S).  Evidence: EXPERIMENTS.md (C1), EXPERIMENTS-C3.md, EXPERIMENTS-MAXIMA.md (X1, X2); DECISIONS.md E-1..E-3, M-1..M-4: memory backend dense Merkle (ZmLogUp benchmark-only), boundary Poseidon, S = 16 kept, one arena, TOP = 63 recommended (OWNER).

## Phase D: implementation spec (OpenSpec 1.17.0)
- [x] D1 New OpenSpec change `sherlock-modular-leaf-20261111`: proposal.md, design.md (= ARCHITECTURE.md as decided), one `specs/<module>/spec.md` per module (requirements under 500 characters, WHEN/THEN scenarios tied to MockProver mutants, Quint models, Lean obligations), tasks.md (module build order, one task per module kit).  Evidence: openspec/changes/sherlock-modular-leaf-20261111/: proposal.md, design.md, 23 specs (119 requirements, 203 scenarios), tasks.md (84 tasks).
- [x] D2 `openspec validate sherlock-modular-leaf-20261111 --strict` passes with no warnings.  Evidence: `openspec validate sherlock-modular-leaf-20261111 --strict`: "Change 'sherlock-modular-leaf-20261111' is valid" (re-run by the lead).
- [x] D3 GP-1 / GP-001 text in `sherlock-vm-consensus-20261107` replaced per A1 + A2 (no narrowing), traceability updated, F14 check passes, `openspec validate` passes.  Evidence: AUDIT.md section 5 text applied (general-purpose.md lines 14 and 16, GP-001 row, spec mirror plus one scenario); openspec validate passes; F14 PASS run 20261111T061635Z-F14; traceability.md re-rendered.
- [x] D4 Commit, push, README Sherlock section and wiki updated.  Evidence: this commit (README Sherlock section, wiki source openspec-modular-leaf, lint PASS).

## Phase E: morning report
- [x] E1 `MORNING-REPORT.md` in this folder: what is decided, what is measured, what is validated, open owner questions, links.  Evidence: MORNING-REPORT.md.

## Rules
Heavy jobs through `scripts/doctorwatson heavy` only, one at a time, heavy.slice under 40 GiB. Jobs over 30 minutes as `systemd-run --user` units with logs under `~/.cache/final-design/`. Agents rebase main first and never push; the lead pushes after scanning for secrets and AI trailers. No Co-Authored-By. Never Ollama. Subagents use model opus; Fable 5.2 (model fable) for the merge and audits as the owner asked.
