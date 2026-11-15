# Astra lane B, phase 1: independent draft for slice B (proving)

You are the Astra lane paired with Fable-B. Work in the current folder (the worktree `sherlock-e2e-spec`).

Read `research/sherlock-e2e-spec-20261115/BRIEF.md` first. It defines the deliverable, the slices, the sources and the hard rules. Your slice is **B. Proving**: leaf (R_SL, L_Fr, tapes, transcript), universal leaf key and the single rekey (T25), nodes and aggregation (MERGE-WORK 5,400, PCD, GP-1..GP-4), FP-3 L binding, anchors (T12 slot 15), elastic multi-box proving (T16: muninn/huginn/odin, heavy-slot, gpu.lock, box bus), GPU paths, setup mode and keys, zero knowledge end to end (T11), small-field STARK leaves and nodes.

## Rules

- Write only inside `research/sherlock-e2e-spec-20261115/astra-B/`. Do not edit any other file.
- No git commands. No network. No builds, no provers, no tests. Read files and use `rg` only.
- Do not read `openspec/changes/sherlock-e2e-prototype-20261115/` or `research/sherlock-e2e-spec-20261115/bus/`. Your draft must be independent of the Fable drafts.
- Every factual claim about code or a ruling names a `file:line` or a commit. If you cannot find evidence, write "NO EVIDENCE FOUND" for that claim.
- Simple English (ASD-STE100). No hedges. No filler.

## Sources (read them; paths with `~` are outside the worktree and are read-only)

- `~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` (rows T1..T25, Finish plan, owner rulings) and the row folders next to it.
- `~/.cache/lead/CHECKLIST-20261115.md`.
- `~/Projects/Moriarty-wt/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md` and `~/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115/` (62 EARS; reuse its IDs, do not duplicate).
- `openspec/changes/sherlock-processor-final-20261111/` (governing), `openspec/changes/sherlock-modular-leaf-20261111/`, the other `openspec/changes/sherlock-*`.
- `~/Projects/Moriarty/llm-wiki/wiki/` (`rg -i TERM index.md` first) and `log.md`.
- Code: `~/.cache/m7-wrap/src/openvm-rekey-chips` (HEAD dcf1b4c), stark-backend clones under `~/.cache/m7-wrap/src/`, `~/.cache/rekey-chips/SECP-INTERFACE.md`, `~/.cache/rekey-t11/LIST.md`.
- TERMINOLOGY.md and ARCHITECTURE.md of the Sherlock design (find with `rg --files | rg -i 'TERMINOLOGY|ARCHITECTURE'` in the worktree and in `~/Projects/Moriarty/research/`).

## Output: `research/sherlock-e2e-spec-20261115/astra-B/DRAFT.md`

1. **Capability list** for slice B: name (kebab-case), ID prefix, one-line scope, the LOOP rows it covers, the components it covers. Cover every slice B component. State which components you found that no capability covers.
2. **EARS draft** per capability: requirements in EARS form ("The <system> shall ...", "When ..., the <system> shall ...", "While ...", "If ..., then the <system> shall ...", "Where ..."), each with an ID, one scenario (GIVEN/WHEN/THEN) and one `Verification:` line that names an existing test, Quint model, Lean theorem or run (with its path), or says "NEW: <what must be built>". Reuse IDs from `sherlock-completion-20261115` when the requirement exists there; mark them "REUSE".
3. **Hard rules**: for each hard rule in BRIEF.md that touches slice B, the requirement that states it and the evidence (file:line) for its current status in code.
4. **Decision list** (MADR candidates): ID (reuse existing decision IDs from LOOP.md and the wiki), title, drivers, options, outcome, authority (OWNER / LEAD PICK confirmed / LEAD PICK for T15) with the source line of the ruling, revisit condition.
5. **DDD draft** for slice B: bounded contexts, aggregates with invariants, domain events, the interfaces to slice A (language, R_SL statement, kits) and slice C (wrap, settlement, operations).
6. **Open gaps and contradictions**: places where sources disagree (stale spec vs current ruling vs code), with both citations. Rank by risk to the working prototype.

Your final message: a 10-line summary of DRAFT.md.
