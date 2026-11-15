# Astra lane A, phase 1: independent draft (slice A: language, execution, integration)

You are the Astra lane for slice A of the Sherlock end-to-end prototype spec. Read `research/sherlock-e2e-spec-20261115/BRIEF.md` first. It defines the deliverable, the three slices, the sources and the hard rules.

## Rules
- Write only inside `research/sherlock-e2e-spec-20261115/astra-A/`. Do not change any other file.
- No git commands. No network. No builds, no provers, no cargo. Read files and run `rg`, `ls`, `sed`, `wc` only.
- Do not read `openspec/changes/sherlock-e2e-prototype-20261115/` or the `bus/` folder. Your draft must be independent of the Fable lead's text.
- Each factual claim about code names a file:line or a commit. If you cannot find evidence, write "NOT FOUND" and say where you looked.
- Simple English (ASD-STE100). No hedges, no filler.

## Scope of slice A
Compact contracts → Mycroft compiler → ZKIR / rsl-call/1 lowering; the R_SL statement as the language and execution layers see it; kits (SHA-256, keccak, jubjub, secp256k1, Poseidon, ALU); the universal interpreter; the Compact test suite (T23 Part B); and the end-to-end user flow (developer writes Compact → user proves on machines the user owns → settles on Preview). Slice B owns proving internals and slice C owns the wrap, settlement and operations; name the interfaces you need from them, do not specify their internals.

## Sources (read these; the BRIEF lists more)
- `~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` (rows T1..T25, owner rulings) and `~/.cache/lead/CHECKLIST-20261115.md`.
- `~/Projects/Moriarty-wt/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md` and `~/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115/` (62 EARS; reuse its IDs, do not duplicate).
- `openspec/changes/sherlock-processor-final-20261111/` (governing), `sherlock-modular-leaf-20261111`, `sherlock-universal-interpreter-design`, other `sherlock-*` changes.
- Mycroft: `~/Projects/Moriarty-wt/mycroft-factory/mycroft/`. T23 worktrees: `~/Projects/Moriarty-wt/t23-*`. Kits and chips: `~/.cache/m7-wrap/src/openvm-rekey-chips` (HEAD dcf1b4c), `~/.cache/rekey-chips/SECP-INTERFACE.md`, `~/.cache/rekey-t11/LIST.md`.
- Wiki: `~/Projects/Moriarty/llm-wiki/wiki/` (`rg -i TERM wiki/index.md` first). Terminology: find `TERMINOLOGY.md` under `~/Projects/Moriarty/research/`.

## Output: `research/sherlock-e2e-spec-20261115/astra-A/DRAFT.md`
1. **Component inventory** for slice A: each component, where its code or design lives (file:line or commit), its state (built / partly built / design only), and the LOOP row that covers it.
2. **Capability list**: capability name (kebab-case), ID prefix, one-line purpose, the components it covers.
3. **EARS draft** for each capability: requirements with IDs, in EARS form (ubiquitous, While, When, If-then, Where; one "shall" each), each with one scenario (WHEN/THEN) and a `Verification:` line that names a test, Quint model, Lean theorem or run that exists or must be built (say which). State where a requirement already exists in `sherlock-completion-20261115` or `sherlock-processor-final-20261111` and give its ID.
4. **End-to-end user flow**: the ordered steps from Compact source to a settled proof on Preview, the artifact that passes between each step, and the slice (A, B or C) that owns each step.
5. **Decision list**: each real decision in slice A (owner ruling, LEAD PICK, hard rule), its source (file:line), its authority (OWNER / LEAD PICK confirmed / LEAD PICK for T15), the options that were rejected, and the EARS IDs that bind it.
6. **Interfaces needed from slices B and C**: named types or events, with the fields you need.
7. **Risks and gaps**: components with no spec, claims in existing specs that the code contradicts, requirements with no possible test.

When DRAFT.md is complete, reply with a summary of 10 lines or fewer.
