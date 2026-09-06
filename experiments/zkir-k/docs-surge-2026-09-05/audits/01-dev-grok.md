# Audit of experiments/zkir-k/docs/01-overview.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed: ran `uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py check experiments/zkir-k/corpus/ledger9-92e8bdd3-tests/native_identity.zkir` from the repository root (prints `wfOk`, exit 0); ran `zkir_run.py` on `corpus/handmade/transient_hash.zkir` with `{"inputs":["5","123456789","987654321987654321"]}` because the chapter's second command is a template with no preimage; compared that JSON with the keys the chapter lists; compared the `checkedJob` quotation with `zkir-vm.k:98`; compared the module table with every `*.k` header and `module` line; counted `Instr` / `IrType` / `Value` productions in `zkir-syntax.k` and `zkir-values.k`; counted `*.zkir` files in each corpus directory; compared checking-pipeline numbers with `evidence/zkir-k-*-2026-09-05b.txt`; compared `zkir_run.py:309-351` status and JSON fields with the chapter; compared gate emission with `zkir-vm.k:167-188`; confirmed `add_incircuit` in `repos/midnightntwrk/midnight-ledger/zkir-v3/src/ir_instructions/add.rs:66`, `IrSource::preprocess` at `ir_vm.rs:208`, `impl Relation for IrSource` / `fn circuit` at `ir_vm.rs:717-728`; compared the trust section with `wiki/zkir/zkir-k-definition.md` "What this abstraction does not model"; grepped the chapter for `TODO`, `TBD`, em-dashes, extra `#` titles, and "this document"; listed `experiments/zkir-k/{semantics,tools,corpus,docs}` and `evidence/zkir-k-*`.

## Findings
### F1 major "the second prints a JSON object with `status`, `memory`, `pis`, `pi_skips`, `cursors`, `constraints`, `verdicts`, `all_verdicts` and `violations`"
Claim: those names are the useful fields of a run, presented in the same breath as the K cells `<constraints>` and `<verdicts>`.
Evidence: `tools/zkir_run.py:333-351` sets `out['constraints'] = len(...)` and `out['verdicts'] = len(verdicts)`; `all_verdicts` is the list of `(outcome, message, gate)` triples and `violations` is the non-`holds` subset. A run of `handmade/transient_hash.zkir` printed `"constraints": 8`, `"verdicts": 8`, plus keys the chapter does not name: `k_cell`, `needs`, `outputs`. The CLI (`zkir_run.py:359-369`) has no `--depth`, so the documented command can emit `stuck` but not `depth-exhausted` (`zkir_run.py:315` only chooses `depth-exhausted` when `depth is not None`).
Fix: say that `constraints` and `verdicts` are integer counts of the K cells, that the per-gate outcomes are `all_verdicts` (and `violations` when not `holds`), and list `needs`, `outputs` and `k_cell`. Point at `02-getting-started.md` for a full printed object. Say `depth-exhausted` is a `Runner.run(..., depth=N)` status, not something the two commands on this page can print.

### F2 major module table row for `zkir-vm.k`: "emitting one gate per instruction"
Claim: every instruction contributes one `gate(...)` to `<constraints>`.
Evidence: `zkir-vm.k:172-173` emits `gate(I)` only when `notBool isSpecialEmit(I)`. `zkir-vm.k:181-188` makes `impact` and `output` special: `impact` appends `guardGate(G)` plus one `piGate` per operand; `output` appends `outputGate(Vs, Ts)`. Start-of-run rules at `zkir-vm.k:145-158` also append `bindGate(0)` and, when communications are on, `commGate(...)`. A successful `transient_hash` run has 7 instruction gates plus `bindGate ( 0 )`, which is why JSON `constraints` is 8.
Fix: replace the one-liner with: most instructions append `gate(I)`; `impact` appends `guardGate` and `piGate`s; `output` appends `outputGate`; the run also seeds `bindGate` / `commGate`. Point at `06-configuration-and-run-lifecycle.md` and `08-constraints-and-verdicts.md`.

### F3 major "Every production in the K syntax carries a `symbol(...)` attribute whose name is the JSON `op` of the instruction or the JSON key of the type, so the JSON artifact maps onto the K term one to one."
Claim: every K production is a JSON op or type key, and that is why JSON maps onto terms.
Evidence: instruction constructors in `zkir-syntax.k:86-119` do use `symbol(op)`, and the 13 `IrType` constructors use `symbol(Native)` etc. Function productions in the same file do not (`encodedLen` at line 47, `reads` at 134, `writes` at 172, `wf` at 250). `program`, `typedId`, `var`, `imm`, `noGuard`, `alignment` have symbols that are neither ops nor type keys. JSON type strings are not the symbols: `tools/zkir_kast.py:35-49` maps `"Scalar<BLS12-381>"` to `Native`. Programs enter through that preprocessor (`zkir-syntax.k:10-11`), not by reading `symbol(...)` off the JSON.
Fix: restrict the claim to `Instr` and `IrType` constructors in `zkir-syntax.k` (and the extension ones in `zkir-ext.k`). Say the preprocessor in `tools/zkir_kast.py` builds the `Program` term, mapping serde names such as `"Scalar<BLS12-381>"` to `symbol(Native)`.

### F4 major pipeline picture, `<verdicts>` line
Claim: `<verdicts>: verdict(gate(...), holds()) | violated(msg) | synthErr(msg) | unknown(msg) | unsupported(msg)`.
Evidence: `zkir-constraints.k:40-43` defines `Verdict ::= verdict(Constraint, Outcome)` and `Outcome ::= holds() | violated(String) | synthErr(String) | unknown(String) | unsupported(String)`. A verdict is always `verdict(C, O)`. `holds()` takes no message. `C` is any `Constraint`, not only `gate(...)` (the `transient_hash` run's first `all_verdicts` entry is `bindGate ( 0 )`).
Fix: write `verdict(C, holds()) | verdict(C, violated(msg)) | ...` with `C` a `Constraint` (`gate`, `piGate`, `guardGate`, `bindGate`, `commGate`, `outputGate`).

### F5 major "The definition is checked in four layers" then a six-row table
Claim: four layers, each with a receipt.
Evidence: the table in this chapter has six rows (unit values, unit hashes, differential base, differential ext, divergence, corpus), and there are six matching receipts (`evidence/zkir-k-unit-values-2026-09-05b.txt`, `...-unit-hash-...`, `...-differential-92e8bdd3-...`, `...-differential-ext-2ffe2d1-...`, `...-divergence-tests-...`, `...-milestone2-corpus-check-...`). `wiki/zkir/zkir-k-definition.md` "How it was checked" groups base and extension differential as one layer and does not count corpus well-formedness there.
Fix: drop "four". Say the checks below, each with a receipt under `evidence/zkir-k-*-2026-09-05b.txt`.

### F6 major corpus map versus `check_corpus.py` "63 programs as expected"
Claim: the corpus under `experiments/zkir-k/corpus/` is 43+61+6+9+7+20 programs, and `check_corpus.py` reports 63 programs as expected, with no link between the two figures.
Evidence: `*.zkir` counts match the table (43, 61, 6, 9, 7, 20). `tools/check_corpus.py:30-36` walks only `ledger9-92e8bdd3-tests`, `midnight-zkir-2ffe2d1-precompiles`, `handmade-negative`, plus `experiments/moriarty-compact-escrow/output/zkir` and `experiments/moriarty-core-swap/output/zkir`. It does not walk `midnight-zkir-2ffe2d1-tests/`, `handmade/`, or `divergence/`. Receipt `evidence/zkir-k-milestone2-corpus-check-2026-09-05b.txt:65` is `63 programs, 63 as expected`. `corpus/README.md:11-13` is where the seven Moriarty artifacts are named.
Fix: after the 63, say which trees `check_corpus.py` walks (43 ledger tests + 6 precompiles + 7 handmade negatives + 7 Moriarty escrow/swap artifacts). Name those two trees outside `experiments/zkir-k/corpus/`. Leave the 61/9/20 programs to `diff_test.py` / `divergence_tests.py`.

### F7 major opening sentence: "ZKIR v3, the zero-knowledge intermediate representation that Midnight's Compact compiler emits"
Claim: Compact emits ZKIR v3.
Evidence: `wiki/zkir/compact-to-zkir-pipeline.md:90-96` records that `compactc` emits ZKIR version 2 by default and ZKIR v3 only with `--feature-zkir-v3`. `corpus/README.md:11-13` says the Moriarty artifacts were emitted with that flag.
Fix: "ZKIR v3, the IR that `midnight-ledger`'s `zkir-v3` crate (`IrSource::preprocess` / `Relation::circuit`) turns into a witness and a circuit. Compact can emit this surface with `--feature-zkir-v3`; without the flag it still emits ZKIR v2."

### F8 major "What a green run establishes": "that witness satisfies the instruction-level relation of every gate the circuit would contain"
Claim: `ok()` plus every verdict `holds()` means the witness meets every gate the (PLONKish) circuit would contain.
Evidence: the same chapter, "What it models and what it does not", and `wiki/zkir/zkir-k-definition.md:42` say auxiliary witness cells, assignment constraints, copy wiring and hash-gadget internals are not modelled. In this documentation set a "gate" is an entry of `<constraints>` (common brief), not a PLONKish row. The next paragraph then has to walk the claim back.
Fix: "that witness satisfies every instruction-level relation the definition emitted into `<constraints>` and evaluated into `<verdicts>`. That is not the PLONKish gate table." Keep the rest of the paragraph (no uniqueness, no gadget internals, no canonical `JubjubScalar` by assignment, no keygen/proving, `synthErr` on `ok()`).

### F9 minor differential counts: "358 comparisons agree ... (46 successful-run agreements, 309 error-run agreements)"; same pattern for 418 / 50 / 364
Claim: the parenthetical is the split of 358 (resp. 418).
Evidence: `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt:360` has the same three numbers. 46+309=355. The missing three are format-error agreements at lines 133-135 (`test_invalid_operand_*`). Extension receipt line 420: 50+364=414; the missing four are three of the same format cases plus `handmade/native_bytes.zkir` (`reverse_bytes` rejected) at line 402.
Fix: keep 358 / 418 / 0 disagree from the receipts. Either add "plus 3 (resp. 4) format-error agreements with `IrSource::load`" or drop the parenthetical split.

### F10 minor `docs/` row: "this documentation set"
Claim: the cell text refers to "this documentation set".
Evidence: common brief hard rule 4 forbids mentioning "this document". Line 34 is the only hit. No `TODO`, no extra `#` title, no em-dash.
Fix: "`docs/`: the fifteen chapters `01-overview.md` ... `15-design-rationale-and-limits.md`".

### F11 minor Reading guide omits chapter 01
Claim: must-cover asks for a guide to all fifteen chapters.
Evidence: the table lists `02-getting-started.md` through `15-design-rationale-and-limits.md` (14 rows). A reader of the overview already has 01, but a later search of the guide will not find it.
Fix: first row: "What is the definition, where does it live, and what does a green run mean? | 01-overview.md".

## Coverage
- What the definition is (executable K semantics of ZKIR v3, base 92e8bdd3 / extension 2ffe2d1, off-circuit witness + instruction-level in-circuit relations, not PLONKish rows / auxiliary cells / hash-gadget internals): covered (opening Compact claim is F7).
- Map of `experiments/zkir-k/` (semantics, tools, corpus, docs, evidence) and K-module table vs file headers: covered (evidence correctly placed at repository-root `evidence/`; module one-liners match headers except F2).
- Pipeline picture JSON -> `zkir_kast.py` -> `Program` -> `job`/`checkedJob`/`genJob` -> cells -> verdicts: covered (picture syntax is F4; JSON keys are F1).
- Checking pipeline with receipt numbers (unit 42/42, hash 18/18, differential 358 and 418, divergence 20/20, corpus 63): covered (F5, F6, F9).
- Reading guide for all fifteen chapters: partly (02-15 present, 01 missing: F11).
- Trust statement (what a green run does and does not establish): covered (positive claim overstates "every gate the circuit would contain": F8).
