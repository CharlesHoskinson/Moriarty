# Sherlock end-to-end prototype spec: brief (2026-11-15)

Owner order: "We need end-to-end OpenSpecs to finish the prototype of Sherlock. Have 3 fable 5.2 and 3 astra agents coordinate. Make sure the entire system is covered. Have a look at the local work in drwatt for architecture as well. We need an MADR and DDD for the project as well."

Sherlock is "a universal, elastic proving co-processor for Midnight" (`research/.../TERMINOLOGY.md`, ARCHITECTURE.md). Users prove runs on machines they own (case A), and settle one blinded proof on the Midnight ledger 9.1.0.1 verifier.

## Deliverable (one OpenSpec change, in this worktree, branch `sherlock-e2e-spec`)

`openspec/changes/sherlock-e2e-prototype-20261115/`
- `proposal.md`, `design.md` (Fable-A integrates), `tasks.md` (the build order to the working prototype, each task names its EARS IDs and its LOOP row).
- `specs/<capability>/spec.md`: one spec per capability. Every requirement is EARS with an ID (prefix per capability), has at least one `#### Scenario:`, and a `Verification:` line naming a test, Quint model, Lean theorem or run that checks it. `openspec validate --strict` must pass. The `ears` skill linter must pass with `--require-ids --strict`.
- `architecture/DDD.md`: bounded contexts, context map (relations: upstream/downstream, ACL, shared kernel, published language), aggregates with invariants, domain events, ubiquitous language (reuse TERMINOLOGY.md). Proportionate, per the profile below.
- `architecture/decisions/D-NNN.md`: MADR records (template below). Reuse existing decision IDs and rulings; one record per real decision (owner rulings, LEAD PICKs, hard rules). Each record: drivers, options, outcome, authority (OWNER / LEAD PICK confirmed / LEAD PICK for T15), confirmation (the EARS IDs and checks that bind it), revisit_when. `architecture/decisions/INDEX.md`.
- `architecture/COVERAGE.md`: a matrix from every system component and every LOOP.md row to its capability spec and EARS IDs. No blank cells; a gap is listed as a gap with an owner lane.

## Slices (each Fable lead owns one; together they cover the whole system)

- **A. Language, execution and integration (Fable-A, also integrator).** Compact contracts → Mycroft compiler → ZKIR/rsl-call/1 lowering, the R_SL statement, kits (SHA-256, keccak, jubjub, secp256k1, Poseidon, ALU), the universal interpreter, the Compact test suite (T23 Part B). Plus: DDD context map, MADR set, design.md, COVERAGE.md, end-to-end user flow (developer writes Compact → user proves on own boxes → settles on Preview).
- **B. Proving (Fable-B).** Leaf (R_SL, L_Fr, tapes, transcript), universal leaf key and the single rekey (T25), nodes and aggregation (MERGE-WORK 5,400, PCD, GP-1..GP-4), FP-3 L binding, anchors (T12 slot 15), elastic multi-box proving (T16: muninn/huginn/odin, heavy-slot, gpu.lock, box bus), GPU paths, setup mode and keys, zero knowledge end to end (T11), small-field STARK leaves and nodes.
- **C. Wrap, settlement and operations (Fable-C).** One BLS12-381 wrap (VK ≤ 50,000 bytes, k ≤ 21), the unchanged ledger 9.1.0.1 verifier, SherlockLedger contracts and acceptRun (T23 Part A, t23-args map), Midnight batching (one root per checkpoint, tx counts), governance (Fenrir operator keys, HFC eras, era registry T22, sealed contracts), Preview deployment, wallet handling (never logged), SDK/CLI, observability, failure and recovery, helper machines (case A now; case B deferred to Veritee).

## Pairing with Astra (GPT-6 Astra via Codex)

Each Fable lead runs one Astra lane (its own), in two phases:
1. **Independent draft first** (start it before you write, so it is not anchored on your text): Astra reads the sources and writes its own capability list, EARS draft and decision list for the slice to `research/sherlock-e2e-spec-20261115/astra-<X>/DRAFT.md`.
2. **Adversarial review**: after your specs exist, Astra reviews them against its draft and the sources, and writes `astra-<X>/REVIEW.md` (missed components, wrong claims vs code, untestable EARS, decisions without authority). You resolve every finding yourself with a reason (memory rule: the lead decides disagreements; never "for owner confirmation"), recorded in `astra-<X>/RESOLUTION.md`.

Astra command (from the worktree root; one session per lane, resume it for phase 2 with `codex exec resume <SESSION_ID>`; check the codex skill):
```
timeout 3600 codex exec -m gpt-6-astra -c model_reasoning_effort=high -s workspace-write \
  -C ~/Projects/Moriarty-wt/sherlock-e2e-spec -o research/sherlock-e2e-spec-20261115/astra-<X>/phase1.md \
  "$(cat research/sherlock-e2e-spec-20261115/astra-<X>/PROMPT1.md)" < /dev/null > research/sherlock-e2e-spec-20261115/astra-<X>/phase1.log 2>&1
```
Tell Astra: write only inside `astra-<X>/`; no git commands; no network. On "model at capacity", wait and retry the same model.

## Coordination

- Bus: `research/sherlock-e2e-spec-20261115/bus/LOG.md`, append-only lines `[HH:MM] <lane> | <message>`. Post: capability names you claim (before writing), cross-slice interfaces you need from another slice, finished milestones.
- Capability ownership: claim in `bus/CLAIMS.md` (one line per capability: name, ID prefix, owner). First claim wins; disputes go to Fable-A, who decides.
- Interfaces between slices are named domain events / published-language types in DDD.md; Fable-A owns DDD.md and decisions/; B and C send their contexts, aggregates and decisions as files `bus/ddd-<X>.md`, `bus/decisions-<X>.md` for Fable-A to merge.
- Only Fable-A commits, at the end, after B and C post DONE. Stage explicit paths; docs-only commit uses `DW_DOCS_COMMIT=1`; never `--no-verify`; T0 is flaky under load, retry. NO Co-Authored-By or any AI attribution in commits (owner's highest rule). Never push.

## Sources (read, do not copy blindly; claims must match code and current rulings)

- Ruling state: `~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` (rows T1..T25, Finish plan, owner rulings), `~/.cache/lead/CHECKLIST-20261115.md`.
- Spec audit just done: `~/Projects/Moriarty-wt/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md` and its change `openspec/changes/sherlock-completion-20261115/` (62 EARS) in that worktree. Reuse its IDs; do not duplicate requirements, reference or supersede them explicitly.
- Existing changes: `openspec/changes/sherlock-processor-final-20261111/` (governing), `sherlock-modular-leaf-20261111`, and the other `sherlock-*` changes here; `openspec/specs/`.
- Wiki: `~/Projects/Moriarty/llm-wiki/wiki/` (`rg -i TERM wiki/index.md` first), log.md.
- Code: openvm rekey-chips `~/.cache/m7-wrap/src/openvm-rekey-chips` (HEAD dcf1b4c), stark-backend clones under `~/.cache/m7-wrap/src/`, Mycroft `~/Projects/Moriarty-wt/mycroft-factory/mycroft/`, T23 `~/Projects/Moriarty-wt/` t23-* worktrees, `~/.cache/rekey-chips/SECP-INTERFACE.md`, `~/.cache/rekey-t11/LIST.md`.
- Architecture method (drwatt local work, the owner named it): `~/Projects/software-factory/tools/drwatt/` (README.md, docs/design.md, docs/decisions.tsv, docs/drwatt-requirements-organization.md, docs/requirements-organization-guidelines.md, docs/requirements-categories.md, proposed/formal-methods-workflow.md, specifications/) and the profile `~/Projects/software-factory/skills/specs/spec-architect/references/architecture-decisions.md`, `madr-template.md`, `decision-allocation.md`. Follow that profile: proportionate DDD, MADR records that bind only through mapped EARS IDs and checks.

## Hard rules the specs must state, never change

Never k=22 (all circuits k ≤ 21). Zero knowledge end to end; seeded proofs test-only; no plain witness to helpers. 100-bit security. One universal key set. Ledger 9.1.0.1 verifier unchanged. Wrap VK ≤ 50,000 bytes. One BLS12-381 wrap; leaves and nodes small-field STARKs. GP-1..GP-4 never narrowed (universal VM, composition, foreign proofs, PCD). Midnight writes batched, one root per checkpoint. Case A only; case B via Veritee later. K_MAX = 2. MERGE-WORK 5,400. x_j lock form canon. F-3 replaced by setup mode. Design params, keys and key handling may change until the end-to-end prototype; hard rules are owner-gated.

## Style

ASD-STE100 simple English (ste skill); no hedges, no filler. Every factual claim about the code names a file:line or commit.
