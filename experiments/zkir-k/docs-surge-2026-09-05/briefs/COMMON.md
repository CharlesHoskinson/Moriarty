# Common brief for every chapter author

You are writing one chapter of the end-to-end documentation of the executable
K semantics of ZKIR v3 that lives in this repository. Read this brief in full,
then your chapter brief, then the sources, then write the chapter.

## Where things are (all paths relative to the repository root, which is your working directory)

- `experiments/zkir-k/semantics/*.k` — the K definition. Main modules:
  `zkir.k` (ZKIR = ZKIR-VM, base surface, midnight-ledger 92e8bdd3),
  `zkir-ext.k` (ZKIR-EXT, midnight-zkir 2ffe2d1 surface), `zkir-check.k`
  (ZKIR-CHECK = static well-formedness only), `zkir-test.k` (ZKIR-TEST, pure
  functions for unit tests). Module files: `zkir-syntax.k` (ZKIR-SYNTAX,
  ZKIR-WF), `zkir-field.k`, `zkir-curves.k`, `zkir-values.k`,
  `zkir-constants.k` (generated), `zkir-hash.k`, `zkir-ops.k`, `zkir-vm.k`,
  `zkir-constraints.k`, `zkir-sha512-constants.k`.
- `experiments/zkir-k/semantics/*-kompiled/` — the four compiled definitions
  (LLVM backend, `kompile <main>.k --backend llvm --emit-json -O1`, K v7.1.337).
- `experiments/zkir-k/tools/*.py` — pyk preprocessor and harnesses:
  `zkir_kast.py` (JSON artifact to K term; `kore|kast|check` commands, `--ext`),
  `zkir_run.py` (run a program on a preimage: `uv run --group zkir-k python
  experiments/zkir-k/tools/zkir_run.py PROGRAM.zkir PREIMAGE.json [--ext]
  [--checked] [--gen] [--depth N]`; `--gen` runs `genJob` and reports the
  transcript needs and the commitment it recorded; a format error exits 2), `diff_test.py` (differential harness against the Rust oracle,
  `--ext`, `--only`, `--seed`, `--no-perturb`, `--attempts`),
  `divergence_tests.py` (20 targeted cases), `unit_values.py` (43 checks),
  `unit_hash.py` (18 known-answer checks), `check_corpus.py` (well-formedness
  of the corpus), `gen_handmade.py`, `gen_constants.py`,
  `extract_test_inputs.py`, `zkir_values.py` (independent Python encoders).
  Python is run through uv: `uv run --group zkir-k python ...` from the
  repository root (the group is in `pyproject.toml`).
- `experiments/zkir-k/corpus/` — programs: `ledger9-92e8bdd3-tests/` (43 +
  manifest.json), `midnight-zkir-2ffe2d1-tests/` (61),
  `midnight-zkir-2ffe2d1-precompiles/` (6), `handmade/` (9 + manifest.json),
  `handmade-negative/` (7), `divergence/` (20).
- `experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md` — the accepted
  findings of the earlier eight-reviewer audit and how each was fixed (read it
  for the reasons behind many rules; do not cite reviewers by name in the
  chapter).
- `wiki/zkir/zkir-k-definition.md` — the current summary page of the
  definition. `wiki/zkir-k-semantics-plan.md` — the plan and its decisions.
  `wiki/zkir/zkir-instruction-set.md`, `wiki/zkir/zkir-type-system.md`,
  `wiki/zkir/zkir-vm-semantics.md` — pages on the ZKIR surface itself.
  `wiki/contradictions.md` — the recorded divergences (rows K1 to K5 and the
  extension test_eq row). `wiki/k-framework/k-best-practices.md` — K usage notes.
- The Rust sources the definition follows, at the pinned commits:
  `repos/_build/ledger-92e8bdd3/zkir-v3/src/` (`ir.rs`, `ir_types.rs`,
  `ir_vm.rs`, `ir_instructions/*.rs`; midnight-ledger at 92e8bdd3) and
  `repos/_build/midnight-zkir-2ffe2d1/zkir/src/` (midnight-zkir at 2ffe2d1).
  Use these copies, not the working trees under `repos/midnightntwrk/`, whose
  checkouts may be at other commits (`git show 92e8bdd3:path` also works
  there). The same `_build` directories hold the `zkir-oracle` harness crates
  (binary at `target/release/zkir-oracle`). midnight-circuits 7.2.4 and
  midnight-curves 0.3.1 are in the cargo registry (`find ~/.cargo/registry/src
  -maxdepth 2 -name 'midnight-*'`).
- `evidence/zkir-k-*-2026-09-05b.txt` — receipts of the current results.

## What the documentation is for

A reader who knows ZK circuits and some K, but has never seen this
repository, must be able to understand, run, extend and trust the definition
from the documentation alone. The set has fifteen chapters under
`experiments/zkir-k/docs/`; yours is one of them. Chapter list:

01-overview.md, 02-getting-started.md, 03-program-model.md,
04-values-and-encoding.md, 05-fields-curves-and-hashes.md,
06-configuration-and-run-lifecycle.md, 07-instruction-reference.md,
08-constraints-and-verdicts.md, 09-extension-surface.md,
10-well-formedness-and-static-checks.md, 11-tooling-reference.md,
12-oracles-and-differential-testing.md, 13-known-divergences.md,
14-extending-the-semantics.md, 15-design-rationale-and-limits.md.

Cross-reference other chapters by file name when a topic belongs there
(e.g. "see 08-constraints-and-verdicts.md"); do not duplicate their content.

## Hard rules

1. Every statement about the semantics must be true of the files as they are
   now. Read the rules before describing them. When you describe a rule,
   function, cell or constructor, name the file and the K symbol so the reader
   can find it (for example `zkir-vm.k`, rule for `impact`). Quote short K
   fragments verbatim where they help; keep quotations under ten lines.
2. Numbers (counts of instructions, types, checks, comparisons) must be taken
   from the sources or the receipts, not remembered. If you cannot verify a
   number, leave it out.
3. Do not invent names for things. Use the names in the K files, the tools and
   the Rust sources.
4. The chapter must stand alone as documentation: no mention of how it was
   written, of authors, reviewers, personas, waves, agents, drafts, audits or
   "this document". No placeholders, no TODO, no "TBD", no "coming soon", no
   empty sections. No status tags. Do not mention the date of writing.
5. Style: GitHub markdown; one `#` title (the chapter title), `##` and `###`
   sections; short paragraphs; tables for parallel facts; fenced code blocks
   for commands, JSON and K fragments (use the `k` language tag for K). Plain
   prose, present tense, no marketing. No em-dashes. Expand an abbreviation the
   first time it appears. Aim for the length your chapter brief gives.
6. Write exactly one file: `experiments/zkir-k/docs/<your chapter file>`.
   Do not modify any other file in the repository. Do not create other files.
   Do not run any git command that writes (add, commit, stash, checkout,
   reset, branch). Do not run kompile. You may run the Python tools
   read-only to check facts (they write nothing outside the scratch
   directories of the OS), and you may run `git log`/`git show` to read.
7. Do not edit the K semantics or the tools, even if you believe you found a
   defect. Instead, describe the behaviour as it is, and put any suspected
   defect in a final section titled `## Notes for maintainers` at the very end
   of your chapter (this section will be removed before publication, so keep
   it factual: file, line, what you expected, what you saw). If you have no
   notes, omit the section.

## Terminology to use consistently

- "the definition" for the K definition as a whole; "the base surface" for
  ZKIR v3 at midnight-ledger 92e8bdd3; "the extension surface" for
  midnight-zkir 2ffe2d1; "the crate" for the Rust implementation being
  modelled; "the oracle" for the `zkir-oracle` harness.
- "off-circuit" (witness computation, `preprocess`) and "in-circuit"
  (constraint relation, `circuit`); "gate" for an entry of `<constraints>`;
  "verdict" for an entry of `<verdicts>`; "outcome" for holds / violated /
  synthErr / unknown / unsupported.
- Run entry points: `job` (raw), `checkedJob` (static check first), `genJob`
  (generation mode). Run statuses: ok, error, panic, and the runner-level
  stuck and depth-exhausted.
- Preimage: the JSON object with `inputs`, `binding_input`,
  `communications_commitment` (optional; a two-element array
  [commitment, opening]), `private_transcript`, `public_transcript_inputs`,
  `public_transcript_outputs` (see `preimage_term` in `tools/zkir_run.py`).
