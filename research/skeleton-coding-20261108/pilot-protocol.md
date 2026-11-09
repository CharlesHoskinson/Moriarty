# Skeleton-coding pilot: protocol

Status: fixed before any build (2026-11-08). Design source: `review.md` §3
"Minimum experiment". Owner decisions (2026-11-08): staged; this protocol and
the analysis are committed first; then a SMOKE of 6 builds; then WAVE 1 of 48
builds, which starts only when the lead says so.

Changes after this commit are allowed only (a) after the smoke, to fix harness
defects or the mutant budget, recorded in `pilot-smoke.md` before wave 1, and
(b) never after the first wave-1 build.

## 1. Question

Does a hand-written skeleton (condition B) raise the fraction of per-rule step
circuits that have no soundness finding, compared with the task and spec only
(condition A)? How much of any gain is leakage from the reference (C − B)?

## 2. Components

The units are C6 per-rule step fragments,
`crates/mori-backend/src/sherlock_machine/<component>_step.rs`. They were
chosen because they (1) share one interface (`C6<X>Emitter::new/emit`,
`<X>_RULES`, `<x>_supported`), so one grader covers all; (2) are leaf modules,
used only by their consumer, so a candidate replaces the file without other
edits; (3) are the smallest of the 29 step files, so a build fits one lane;
(4) have honest owner steps in the four test programs (fact(3), fold, build,
cons), so completeness is checkable.

| Stage | Component | Lines | Rules |
|---|---|---|---|
| Smoke | `c6_admin_publish` | 138 | 1 (`KAdminPublishValue`) |
| Smoke | `c6_call_bind` | 196 | 3 |
| Wave 1 | `c6_scalar_construct` | 228 | 1 |
| Wave 1 | `c6_append_helper` | 256 | 4 |
| Wave 1 | `c6_call_helper` | 274 | 6 |
| Wave 1 | `c6_eval_schedule` | 295 | 7 |

Smoke and wave-1 components do not overlap, so the smoke builds are excluded
from the analysis.

## 3. Conditions

No per-rule Lean specs exist. The spec is therefore:

- **A**: task + the reference machine's step logic for the rule + the rule's
  prose description. Generated mechanically by
  `scripts/qa/skeleton_pilot.py spec <component>` from
  `pilot/prompts/spec-template.md`: the fixed interface, the correctness
  statement, the step entry in `owner_runtime.rs`, every non-test line of
  `crates/mori-core/src` that names a component rule (CamelCase or dotted row
  name), and the prose.
- **B**: A + a skeleton written by a blind lane (`pilot/prompts/skeleton-blind.md`):
  signatures, statement-lane stubs, per-function `Requires/Ensures/Check`
  contracts, a global-invariant list, `todo!()` bodies. The author lane gets
  the same sandbox as a builder: it never sees the reference circuit or its
  tests.
- **C**: A + a skeleton in the same format derived from the reference
  (`pilot/prompts/skeleton-derived.md`): the author lane sees
  `reference/<component>_step.rs`. C − B estimates leakage.

The skeleton is installed as the content of the target file; the builder
prompt adds `pilot/prompts/builder-skeleton-section.md`. One B and one C
skeleton per component, shared by all models and seeds of that component.

**Known spec leakage (same in A, B and C).** The prose is the module doc
header (`//!` lines) of the reference step file, the only prose description of
each rule in the repository. It was written by the reference author and
states design choices (for example "charge is 0", "canonicality is composed").
It raises the absolute pass rate of every condition; it does not bias B − A.
Older narrow circuits for some rules stay visible (for example
`b_admin_publication.rs`, `return_publication_step.rs` for the publish row):
deleting them would break the crate. This also is the same in all conditions.

## 4. Lanes

All lanes run in a sandbox: `skeleton_pilot.py sandbox` extracts `HEAD`
(`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates/`) with
`git archive`, so there is no `.git`, no `research/`, no `qa/`. It deletes the
reference step file, its tests, every mori-backend test file that names a
component rule (for the publish row this includes `c6_dispatch_tests.rs`),
and the pilot grader, and strips their `mod` declarations. The target file
holds the stub (A: one comment line; B/C: the skeleton). Sandboxes live in the
session scratchpad at a fixed path per vendor (`lane-claude`, `lane-codex`) so
that the cargo cache stays warm; `CARGO_TARGET_DIR` is
`target/skeleton-pilot-lane-<hash>` under the main checkout.

Builders may run `cargo check`, `cargo clippy` and `cargo fmt` only (no
`cargo test`: the memory rule reserves circuit tests for
`scripts/doctorwatson heavy`). This is the same for all conditions.

Exact commands (from `skeleton_pilot.py lane_cmd`; the prompt is the last
argument):

- Claude (builder and every skeleton author):
  `claude -p --restricted --safe-mode --strict-mcp-config --tools Bash,Read,Edit,Write,Glob,Grep --allowedTools "Read,Edit,Write,Glob,Grep,Bash(cargo check:*),Bash(cargo clippy:*),Bash(cargo fmt:*)" --permission-mode dontAsk --model claude-opus-5-5 --output-format stream-json --verbose --no-session-persistence <prompt>`
  (cwd = sandbox). `--restricted` confines the file tools to the sandbox;
  `dontAsk` denies every other Bash command. Blindness is enforced.
- Codex (builder): `codex exec -m gpt-6.1-sol -c model_reasoning_effort=high -s workspace-write --ephemeral --skip-git-repo-check -c 'sandbox_workspace_write.writable_roots=["<lane target dir>","~/.cargo"]' --json -C <sandbox> -o <log>/final.md <prompt> < /dev/null`.
  The Codex sandbox can read outside the workspace, so blindness is
  instructed and audited, not enforced.
- Environment: `OPENAI_API_KEY`, `CODEX_API_KEY`, `XAI_API_KEY` and
  `ANTHROPIC_API_KEY` are removed (subscription logins only);
  `CARGO_NET_OFFLINE=true`; lane timeout 3600 s; at most 2 lanes at once.

**Blindness audit** (`skeleton_pilot.py audit`): every tool call in the event
log is scanned for a path under `/home/` or `/tmp/claude-*` outside the
sandbox, the cargo home and the lane target dirs; for `git`, `curl`, `wget`,
`gh` or `cargo test`; and for the name of any hidden file. A build with a
violation that reads reference material is void and is rerun once with the
next seed; other violations are reported.

## 5. Runs

- **Smoke** (harness proof, no inferential claim): 2 components × 3
  conditions × 1 seed = 6 builds. `c6_admin_publish` builds with Claude,
  `c6_call_bind` with Codex, so both lane types are exercised. Plus 4 skeleton
  lanes (B and C for each).
- **Wave 1**: 4 components × 3 conditions × 2 models (Claude
  `claude-opus-5-5`, Codex `gpt-6.1-sol` high) × 2 seeds = 48 builds, plus 8
  skeleton lanes. A seed is a repeated independent lane run (the CLIs expose
  no sampling seed); the seed also seeds the mutant sample. Build order is
  randomized once with Python `random.Random(20261108)` over the 48 cells and
  recorded before the first build.

## 6. Grading

The grader is `crates/mori-backend/src/sherlock_machine/skeleton_pilot.rs`,
driven by `skeleton_pilot.py grade`. It exports `HEAD` into a throwaway grader
copy, installs the candidate as `skeleton_pilot_cand_step.rs` with a generated
adaptor (`skeleton_pilot_cand.rs`), and runs
`scripts/doctorwatson heavy --id SKP-<cell> --max-wall 3600 -- env RUST_TEST_THREADS=1 ... cargo test -p mori-backend --lib --locked sherlock_machine::skeleton_pilot::skeleton_pilot_grade -- --ignored --exact`.
Nothing from a grader copy lands on `main`.

Stage: IR preprocessing, the SDK's nonproving `ProofPreimage::check`. Step
circuits declare every lane public and have no private inputs, so the IR
relation is a predicate over the statement. The ZKIR-to-PLONK compiler and the
stdlib gadgets are shared by every candidate and are out of scope (SAS-004
label: "IR preprocessing"). Outcomes reuse `qa_mutation::MutantOutcome`;
only a `ProofPreimage::check` refusal counts as `Rejected`; a build error,
panic or timeout is never a rejection.

Per build:

1. **Build**: the candidate builds one circuit per rule (errors recorded).
2. **Completeness**: for up to 2 owner steps per rule from the four programs,
   the enabled pair and the disabled identity pair (post = pre, enable = 0)
   must be accepted.
3. **Static under-constraint check** (all lanes, full coverage): a statement
   lane that the reference body reads and the candidate body never reads is a
   finding. Private-input instructions in the candidate are a finding.
4. **Raw-statement mutants** (the extension of the 2.2 harness): each honest
   enabled pair and each disabled identity pair is a base. Every statement
   lane (enable, schema, carrier version, pre and post META, PROG, CTX, IMAGE,
   DATA, HELPER) gets `plus_one`, `minus_one` (field arithmetic), `zero` and
   `random` (uniform field element); each pre lane with a post twin also gets
   `paired_plus_one` (pre and post together). The oracle is the reference
   verdict on the same statement:
   - reference rejects, candidate accepts: **soundness finding**;
   - reference accepts, candidate rejects: over-constraint (completeness);
   - otherwise agreement, or untested (any non-verdict outcome).
   Budget: a stratified sample of at most 4000 mutants per build (shuffle in
   each (base kind, section) stratum, round-robin over strata), seeded by the
   build seed. The receipt records the space size, the number run, and per
   section the lanes and mutants covered. Coverage below 1 is declared in
   every receipt (SAS-014).

Receipts: `<cell>/grade/grade.json` (summary, honest verdicts, static
findings, coverage, first 50 findings), `<cell>/grade/mutants.jsonl` (every
mutant), `<cell>/lane/{lane.json,events.jsonl}`, `<cell>/audit.json`,
`<cell>/cell.json`. Run directory:
`research/skeleton-coding-20261108/pilot/runs/<stage>/`.

Limits of the oracle: a defect that the reference shares is not found; single
and paired lane mutants do not express coordinated multi-lane forgeries; the
sample does not cover every lane. The static check covers every lane but only
proves "never read", not "read but too weakly constrained".

## 7. Outcomes

- **Primary** (per build, binary): `primary_pass` = the candidate builds, is
  complete (every honest and disabled-identity pair accepted), has zero
  soundness mutants, zero static findings and zero private inputs. A build
  that fails to compile, times out or is void fails. A circuit that rejects
  everything is not "sound by default": it fails completeness.
- Also reported: `sound` alone, `complete` alone, over-constraint count.
- **Secondary**: instruction count relative to the reference (row budget);
  lane tokens and wall; skeleton-author tokens and wall (authoring effort,
  agent-hours; no human authors in this pilot).

## 8. Analysis (fixed before runs)

Smoke: none beyond "the harness ran end to end"; no inferential claim.

Wave 1: the unit of pairing is (component, model, seed): 4 × 2 × 2 = 16
pairs for each contrast.

- **B vs A** (primary): exact McNemar test on the discordant pairs of
  `primary_pass`, two-sided, α = 0.05. With 16 pairs this detects only large
  effects; report the discordant counts (b, c) and the paired difference in
  pass rate with a 95% paired bootstrap interval (10 000 resamples, resampling
  components as clusters, seed 20261108).
- **C vs B** (leakage): the same statistics.
- Secondary contrasts (tokens, wall, instruction ratio): paired differences
  with cluster bootstrap intervals; no tests.
- Per-model results are reported but not tested separately.

## 9. Decision rule (from `review.md` §3)

Adopt skeletons for circuit builds if B beats A on the primary outcome
(McNemar p < 0.05 and a positive paired difference) **and** the skeleton
authoring cost (author-lane wall and tokens per component, shared by its
builds) is lower than the builder time saved (A − B builder wall per build
times the builds per component). Otherwise drop it. If C − B is large and
B − A is not, skeleton gains come from reference leakage, not structure.
Wave 1 alone is unlikely to reach significance; a non-significant positive
result means "extend to more components", not "adopt".

## 10. What stays blind

- Builders (all conditions) and the B skeleton author never see the
  reference step file, its tests, the test files that name the rules, the
  grader, the receipts, `research/` or git history.
- The C skeleton author sees the reference step file only.
- Builders never see the mutant list, the static check or the grading
  programs list beyond what the spec states.
- The grader takes no condition input: it sees only the component and the
  candidate file, and the same code computes every summary.
