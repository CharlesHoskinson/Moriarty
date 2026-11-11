# Night plan 2026-11-11: from studies to a validated implementation spec

Owner, 2026-11-11 (going to sleep): "arm a loop to get this all done while I sleep. I should wake up to finished plans that are validated. Launch whatever you need".

The lead (Claude, the loop) drives this file. Tick a step only on evidence (file present, check passed, commit hash). Owner is AFK: take the recommended option, record every decision in `DECISIONS.md` in this folder, never ask questions. GP-1 to GP-4 are never narrowed. Store every finding in `llm-wiki/` in the same change.

## Phase A: studies and drafts
- [x] A1 Run-length study `CONSENSUS.md` written (agy pane w1:pC).  Evidence: CONSENSUS.md 11,684 words, written 2026-11-11.
- [x] A2 Fable 5.2 audit of A1 -> `research/sherlock-runlength-consensus-20261111/AUDIT.md`.  Evidence: AUDIT.md (adopt with changes; 3 blocker, 10 major, 7 minor); decisions R-1..R-10 in DECISIONS.md.
- [ ] A3 Lookup-cache study rerun in pane w1:pC (keep lanes/B/round1.md; lanes A, C, D, E round 1; round 2 with LEAD-NOTE-ROUND2.md; CONSENSUS.md).
- [ ] A4 Fable 5.2 audit of A3 -> `research/sherlock-lookup-cache-20261111/AUDIT.md`.
- [x] A5 Three Astra drafts `drafts/astra-area{1,2,3}.md` (units astra-area1..3).  Evidence: units exited 0 at 05:38, 05:40, 05:43Z; 7467/7137/8182 words.
- [ ] A6 Three Gemini drafts `drafts/gemini-area{1,2,3}.md` (pane w1:pC subagents).

## Phase B: merged architecture
- [ ] B1 Fable 5.2 merge of A1-A6 -> `ARCHITECTURE.md` (modules per MODULARITY.md; Lean obligations and the OpenSpec outline per ARCH-BRIEF.md addendum); dissent kept; decisions in `DECISIONS.md`.
- [ ] B2 Independent audit of B1 by Astra (Codex, read-only) -> `ARCH-AUDIT-astra.md`; Fable fixes every admissible finding.
- [ ] B3 Commit and push (docs commit), wiki ingest.

## Phase C: experiments that decide the open choices
- [ ] C1 Prototype both memory backends behind one MEM interface in `crates/mori-chip` (dense tail-buffered Merkle; write-once LogUp if B1 keeps it as a candidate): cost-model leaf rows at S = 16, 64, 128 and node rows; MockProver positive and negative tests. Through `scripts/doctorwatson heavy`, T0 before commit.
- [ ] C2 Quint models for the chosen memory and boundary designs, run by a Gemini Quint lane (argon-run) and re-run by the lead.
- [ ] C3 Lean: statements (and proofs where they close in the night) for the partition-independence and modular-composition obligations in `clean/fork/Clean/Sherlock/`; `lake build` green with no new `sorry` outside a marked obligations file.
- [ ] C4 Record results in `EXPERIMENTS.md`; update `DECISIONS.md` (memory backend, boundary commitment, S).

## Phase D: implementation spec (OpenSpec 1.17.0)
- [ ] D1 New OpenSpec change `sherlock-modular-leaf-20261111`: proposal.md, design.md (= ARCHITECTURE.md as decided), one `specs/<module>/spec.md` per module (requirements under 500 characters, WHEN/THEN scenarios tied to MockProver mutants, Quint models, Lean obligations), tasks.md (module build order, one task per module kit).
- [ ] D2 `openspec validate sherlock-modular-leaf-20261111 --strict` passes with no warnings.
- [x] D3 GP-1 / GP-001 text in `sherlock-vm-consensus-20261107` replaced per A1 + A2 (no narrowing), traceability updated, F14 check passes, `openspec validate` passes.  Evidence: AUDIT.md section 5 text applied (general-purpose.md lines 14 and 16, GP-001 row, spec mirror plus one scenario); openspec validate passes; F14 PASS run 20261111T061635Z-F14; traceability.md re-rendered.
- [ ] D4 Commit, push, README Sherlock section and wiki updated.

## Phase E: morning report
- [ ] E1 `MORNING-REPORT.md` in this folder: what is decided, what is measured, what is validated, open owner questions, links.

## Rules
Heavy jobs through `scripts/doctorwatson heavy` only, one at a time, heavy.slice under 40 GiB. Jobs over 30 minutes as `systemd-run --user` units with logs under `~/.cache/final-design/`. Agents rebase main first and never push; the lead pushes after scanning for secrets and AI trailers. No Co-Authored-By. Never Ollama. Subagents use model opus; Fable 5.2 (model fable) for the merge and audits as the owner asked.
