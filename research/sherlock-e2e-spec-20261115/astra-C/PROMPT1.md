# Astra-C phase 1: independent draft for slice C (wrap, settlement and operations)

You are the Astra lane for slice C of the Sherlock end-to-end prototype OpenSpec. Read `research/sherlock-e2e-spec-20261115/BRIEF.md` first. It defines the deliverable, the slices, the sources and the hard rules.

## Limits

- Write only inside `research/sherlock-e2e-spec-20261115/astra-C/`. Write no other file.
- No git commands. No network. No builds, no provers, no Preview or Mainnet transaction. Read and search only (`rg`, `sed -n`, `cat`).
- Never read, print or copy a wallet seed, a mnemonic, a private key or a key file. Describe wallet handling from code and documents only. If a file looks like key material, do not open it.
- Do not read the other lanes' specs or the `bus/` folder. Your draft must be independent.

## Scope of slice C

One BLS12-381 wrap (VK <= 50,000 bytes, k <= 21); the unchanged Midnight ledger 9.1.0.1 verifier; SherlockLedger contracts and acceptRun (T23 Part A, the t23-args map); Midnight batching (one root per checkpoint, transaction counts at scale); governance (Fenrir operator keys, HFC eras, era registry T22, sealed contracts); Preview deployment; wallet handling (never logged); SDK and CLI; observability; failure and recovery; helper machines (case A now, case B deferred to Veritee).

## Sources (read them; a claim must match the code and the current rulings)

- `~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` (rows T1..T25, Finish plan, owner rulings) and `~/.cache/lead/CHECKLIST-20261115.md`.
- `~/Projects/Moriarty-wt/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md` and `~/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115/` (62 EARS; reuse its IDs, do not duplicate).
- `openspec/changes/sherlock-processor-final-20261111/` (governing) and the other `sherlock-*` changes in this worktree.
- Worktrees under `~/Projects/Moriarty-wt/`: `t23-args`, `t23-suite`, `t22-governance`, `t20-batched`, `t17-preview`, `preview-deploy`, `t9-contract`, `e5-wrap`, `m7-wrap`, `gpu-wrap`, `wrap-diet`, `k21-measure`, `t16-elastic`.
- `~/Projects/Moriarty/llm-wiki/wiki/` (run `rg -i TERM wiki/index.md` first) and `log.md`.
- Code: `~/.cache/m7-wrap/src/openvm-rekey-chips` (HEAD dcf1b4c) and the stark-backend clones under `~/.cache/m7-wrap/src/`.

## Output: `research/sherlock-e2e-spec-20261115/astra-C/DRAFT.md`

1. **Capability list** for slice C: name (kebab-case), ID prefix, one-line purpose, the components and LOOP rows it covers.
2. **EARS draft** per capability: each requirement has an ID, one EARS sentence ("The <system> shall ...", with While / When / If-then / Where as needed), one scenario (GIVEN/WHEN/THEN), and a `Verification:` line that names an existing test, Quint model, Lean theorem or run (with path), or says `MISSING` when none exists.
3. **Decision list**: each real decision in the slice (owner ruling, LEAD PICK, hard rule): existing ID when one exists, the outcome, the authority (OWNER / LEAD PICK confirmed / LEAD PICK for T15), and the source file:line.
4. **Evidence table**: each factual claim about code or a measurement, with file:line or commit. Mark a claim you could not confirm as `UNCONFIRMED`.
5. **Gaps**: components of slice C that have no code, no test or no ruling; contradictions between sources (name both file:line).

Rules for the draft: simple English, no hedges. State the hard rules from the brief as requirements where slice C must enforce them (k <= 21, VK <= 50,000 bytes, verifier unchanged, batched writes, case A only, zero knowledge end to end, no plain witness to helpers). Do not invent a ruling. Your final message is a ten-line summary of DRAFT.md.
