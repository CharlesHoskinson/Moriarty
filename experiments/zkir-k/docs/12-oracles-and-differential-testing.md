# Oracles and differential testing

The verification protocol compares off-circuit witness computation with the Rust crate, checks individual K functions, and exercises selected in-circuit outcomes. These establish different predicates. Agreement with `preprocess` does not establish agreement with a Rust circuit prover or equivalence for every program.

All commands start at the repository root; paths beginning with `tools/` or `semantics/` in prose are relative to `experiments/zkir-k/`.

## The Rust oracle

`zkir-oracle` is a local Rust harness crate in each pinned build workspace. Its `src/main.rs` calls `IrSource::load`, constructs a `ProofPreimage` from JavaScript Object Notation (JSON), and calls `IrSource::preprocess`. It does not invoke `circuit`, key generation, or proof verification.

| Surface | Build workspace under `repos/_build/` | Dependency | Full source commit |
|---|---|---|---|
| Base surface | `ledger-92e8bdd3/` | `midnight-zkir-v3`, path `../zkir-v3` | `92e8bdd3a97b61b229e38916e1b180de6f448dd5` |
| Extension surface | `midnight-zkir-2ffe2d1/` | `midnight-zkir`, path `../zkir` | `2ffe2d17bbb736aec36fb300aeaca679a10d2278` |

Both workspaces carry two one-line visibility changes beyond the upstream commits. In `ir_vm.rs`, `pub(crate) fn preprocess` becomes `pub fn preprocess` (`zkir-v3/src/ir_vm.rs:185` at 92e8bdd3, `zkir/src/ir_vm.rs:210` at 2ffe2d1). In `ir_types.rs`, `IrValue::get_type` becomes `pub` (`zkir-v3/src/ir_types.rs:161`, `zkir/src/ir_types.rs:310`), because the harness prints `get_type()` as the `type` field. `encode_offcircuit` is already public. Each workspace `Cargo.toml` also lists `zkir-oracle` as a member; the crate directory itself is untracked. Build the harnesses with:

```bash
cargo build --release --manifest-path repos/_build/ledger-92e8bdd3/Cargo.toml -p zkir-oracle
cargo build --release --manifest-path repos/_build/midnight-zkir-2ffe2d1/Cargo.toml -p zkir-oracle
```

Each binary is `target/release/zkir-oracle` inside its workspace and accepts a program path followed by a preimage path. The receipts do not record build durations. `tools/diff_test.py`, `tools/unit_hash.py`, and `tools/divergence_tests.py` require the binaries at `~/Moriarty/repos/_build/<workspace>/target/release/zkir-oracle`, built from `Path.home()` independently of the current directory. They do not follow the repository's `repos` symlink; that symlink only makes the `repos/_build/...` paths in the build commands above resolve from the repository root.

The two [`main.rs` implementations](../../../repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs) differ only in their crate imports. On success, standard output contains:

| JSON field | Meaning |
|---|---|
| `status` | `"ok"` |
| `memory` | Register-name map |
| `memory[name].variant` | Rust value variant, extracted from debug formatting |
| `memory[name].type` | Debug representation of `get_type()`, including byte-string length |
| `memory[name].encoded` | `encode_offcircuit` vector as decimal strings |
| `pis` | Public inputs as decimal strings |
| `pi_skips` | Skip vector, containing `null` or integer positions |

A returned preprocessing error prints `status: "error"` and `error`, omitting memory and vectors, and still exits zero. The Python adapter maps exit code 101 to `panic`; other nonzero exits become `load-error`. Loading and preimage conversion fail before preprocessing. In particular, `fr_from_dec` rejects noncanonical field elements. The oracle therefore provides no partial Rust memory for failed runs.

## Differential protocol

[`tools/diff_test.py`](../tools/diff_test.py), especially `main`, `build_preimage`, and `compare`, implements the following sequence.

1. **Select programs.** `CORPORA` selects ledger tests, standalone precompiles, escrow and swap artifacts, and handmade programs. `--ext` switches to standalone tests, standalone precompiles, and handmade programs, together with the extension definition and oracle. Files are sorted within each corpus; non-version-3 programs are skipped. `--only` matches a substring of the filename.

2. **Check format rejection.** If `zkir_kast.load_program` raises `ZkirFormatError`, call the oracle with empty inputs and binding input zero. The comparison passes only if the adapter reports `load-error`. This tests rejection of those particular artifacts, not general parser equivalence.

3. **Seed inputs.** The pseudorandom generator is `random.Random(f'{args.seed}:{path.name}')`; the default seed is 2026. `zkir_values.small_encoded` draws values of declared types. On the first attempt, a matching manifest's nonempty `test_preimage.inputs` enables literal seed values, including available binding input, private transcript, and public transcript outputs. For the ledger tests, [`extract_test_inputs.py`](../tools/extract_test_inputs.py) copies literal `ProofPreimage` fields from the Rust tests into `corpus/ledger9-92e8bdd3-tests/manifest.json`; the extension corpus has a separate manifest of the same shape that this script does not produce. Later attempts use generated values.

4. **Discover transcript needs.** `build_preimage` makes at most eight generation passes. In `semantics/zkir-vm.k`, `genJob` enables `<genMode>`; missing active transcript reads in `#input` record `needPubOut(T)` or `needPriv(T)` and temporarily bind defaults. The harness appends typed encodings, replaces public transcript inputs with `needPubIn` values, and uses `needComm` from `#finish` as the commitment with its sampled opening. It stops when no public-output or private requests remain, or reaches the pass limit. Generation assists preimage construction; it does not guarantee a successful witness.

5. **Run both implementations.** Execute `Runner.run(program, pre)` and the oracle on the same preimage. Every attempt is compared. The default `--attempts 8` limits candidate preimages per program; this is separate from the generation-pass bound. Stop attempting after both sides return `ok`.

6. **Perturb a success.** Unless `--no-perturb` is set, compare each applicable mutation below. Each starts from the successful preimage; transcripts and commitment are not regenerated afterward. A mutation may still succeed.

| Row label | Mutation | Condition |
|---|---|---|
| `perturbed-raw` | Replace the first raw field element with a random canonical field element | Raw inputs exist |
| `perturbed-typed` | Replace one randomly selected declared input's complete encoding with a valid random encoding of its type | Raw inputs exist |
| `wrong-pubin` | Increment one random public transcript input modulo the native field modulus | Public transcript inputs exist |
| `wrong-comm` | Increment the commitment modulo the native field modulus, preserving its opening | A commitment exists |

### Raw runs, checked runs, and comparisons

The differential harness uses raw `job`, not `checkedJob`. In [`zkir-vm.k`](../semantics/zkir-vm.k), `checkedJob` first evaluates `wf` through `#wfGate`; static failure produces an error prefixed `well-formedness:`. Static acceptance is an additional predicate, so checked execution is not interchangeable with testing the crate's `preprocess`. `tools/divergence_tests.py` explicitly uses `checked=True`; `tools/check_corpus.py` separately tests static expectations. See [10-well-formedness-and-static-checks.md](10-well-formedness-and-static-checks.md).

`compare` first requires equal statuses. For `error` or `panic`, it compares classes from `ERROR_CLASSES`: ordered, case-insensitive regular expressions that normalize messages such as conversion, decoding, commitment, and transcript failures. Unmatched text becomes `other:` followed by its first 40 characters. Error wording is otherwise outside the comparison.

For `ok`, require identical `pis`, identical `pi_skips`, identical register sets, and equal `type` and `encoded` fields per register. Although both adapters print `variant`, `compare` does not compare it directly. Length-bearing types matter because different byte-string lengths can share an encoding. Runner-level `stuck`, `depth-exhausted`, and `krun-failed` are not successful witnesses; see [11-tooling-reference.md](11-tooling-reference.md).

Rows print `PASS` or `FAIL`, corpus, filename, run label, and details. Labels are `run<N>` for unperturbed attempts, the four mutation labels above, and `format`. If a generation pass returns `krun-failed`, the row is labelled `gen<N>` with summary `krun failed`, counts as a disagreement, and ends the attempts for that program. The final line has this form:

```text
N comparisons: S successful-run agreements, E error-run agreements (status and error class), A agree, D disagree, Ts
```

`S` and `E` count matching status pairs; inspect `D`, because those counters alone do not guarantee matching memory or error classes. Exit zero means no recorded difference. The following `oracle 2:` line counts successful unperturbed K attempts with at least one non-holding gate. It is informational, excludes perturbations, and does not affect the exit code. Because the oracle never runs the crate's `circuit`, the K verdicts are compared with nothing on the Rust side; they are checked only by this count and by the divergence cases below. Gate outcomes are explained in [08-constraints-and-verdicts.md](08-constraints-and-verdicts.md).

## Results and reproduction

The differential receipts record these experiment observations:

| Receipt under `evidence/` | Agree / total | Successful-run | Error-run | Format rejection | Duration |
|---|---:|---:|---:|---:|---:|
| [zkir-k-differential-92e8bdd3-2026-09-05c.txt](../../../evidence/zkir-k-differential-92e8bdd3-2026-09-05c.txt) | 358/358 | 46 | 309 | 3 | 387 s |
| [zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt](../../../evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt) | 418/418 | 50 | 364 | 4 | 281 s |

Both report zero disagreements and zero successful unperturbed runs with a non-holding gate. Format rows account for the difference between total comparisons and successful/error rows. Totals include repeated failed attempts and mutations, not distinct successful programs: the 46 successful-run rows of the base receipt are 31 ledger tests, 14 handmade programs and one swap program. For the escrow artifacts, the six precompiles and the swap artifacts other than `expire.zkir`, no generated preimage produces a successful witness, so those rows compare error paths only, up to eight attempts per program. `expire.zkir` reaches `K=ok Rust=ok` on `run2`, its third attempt, with 28 registers, 394 public inputs and 571 verdicts, and its `wrong-pubin` and `wrong-comm` perturbations agree. One application witness does not amount to transaction-context coverage.

With K v7.1.337, matching pyk, compiled definitions, and both binaries available, reproduce the full differential runs with:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --seed 2026 --attempts 8
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --ext --seed 2026 --attempts 8
```

Budget approximately fifteen and thirteen minutes respectively from the receipts, subject to machine load. For a quick protocol check, add `--only native_identity` to the base command or `--only bool_identity` to the extension command. Each focused run produces four agreeing comparisons in about one second. Setup is described in [02-getting-started.md](02-getting-started.md).

| Additional receipt under `evidence/` | Recorded result | Timing |
|---|---|---|
| [zkir-k-unit-values-2026-09-05c.txt](../../../evidence/zkir-k-unit-values-2026-09-05c.txt) | 43/43 checks | About 7 s in a fresh local run |
| [zkir-k-unit-hash-2026-09-05c.txt](../../../evidence/zkir-k-unit-hash-2026-09-05c.txt) | 18/18 checks | About 3 s in a fresh local run |
| [zkir-k-milestone2-corpus-check-2026-09-05c.txt](../../../evidence/zkir-k-milestone2-corpus-check-2026-09-05c.txt) | 63 programs as expected; zero unexpected | 7.7 s recorded |
| [zkir-k-divergence-tests-2026-09-05c.txt](../../../evidence/zkir-k-divergence-tests-2026-09-05c.txt) | 20/20 cases | About 4 s in a fresh local run |


```bash
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py
```

The divergence suite checks expected statuses, the outcome of the first matching target gate in `all_verdicts` (a missing target gate fails), and typed memory when both runs succeed. It tests selected instruction-level relations, not Rust proof acceptance; see [13-known-divergences.md](13-known-divergences.md). Its normal entry point rewrites `corpus/divergence/`; to reproduce the assertions without touching repository files, redirect `OUT` to scratch:

```bash
uv run --group zkir-k python - <<'PY'
import sys
import tempfile
from pathlib import Path
sys.path.insert(0, 'experiments/zkir-k/tools')
import divergence_tests
with tempfile.TemporaryDirectory(prefix='zkir-divergence-') as scratch:
    divergence_tests.OUT = Path(scratch)
    raise SystemExit(divergence_tests.main())
PY
```

For an already synchronized environment with a read-only uv cache, use `uv --no-cache run --no-sync --group zkir-k` instead of `uv run --group zkir-k`, and set `PYTHONDONTWRITEBYTECODE=1`; the fresh timings above used this invocation. Compare summaries and failures with the receipts rather than expecting identical elapsed times or panic process identifiers.

## Other evidence sources

### Agda

The [arc-zkir receipt](../../../evidence/arc-zkir-agda-typecheck-2026-09-05.txt) records successful type-checking of `zkir-v3/Main.agda` at `fd1c24e1`, under `--safe`:

```bash
(cd repos/input-output-hk/arc-zkir/src && nix run .#agda -- zkir-v3/Main.agda)
```

No duration is recorded. `Main.agda` takes an `Assumptions` record providing field, curve, and hash primitives. The development has no concrete instance that executes corpus programs, so it supplies a specification comparison, not an executable differential oracle. Type-checking alone does not establish correspondence with K or Rust.

### k-rust

The [compatibility receipt](../../../evidence/k-rust-compatibility-2026-09-05.txt) uses k-rust 0.4.0 at `687ccd0`; its Rust build takes 4 min 35 s. These commands reproduce the build, the tutorial run, the syntax probe, and the bounded compilation attempt:

```bash
cargo build --release --manifest-path repos/midnightntwrk/k-rust/Cargo.toml -p k-rust
repos/midnightntwrk/k-rust/target/release/krust krun experiments/zkir-k/toolchain-check/lesson-02-a.k --main-module LESSON-02-A --sort Color -e 'colorOf(Banana())'
repos/midnightntwrk/k-rust/target/release/krust kast experiments/zkir-k/semantics/zkir-syntax.k --module ZKIR-SYNTAX --sort IrType --expression 'native()'
timeout 1500 repos/midnightntwrk/k-rust/target/release/krust kcompile experiments/zkir-k/semantics/zkir.k --main-module ZKIR --syntax-module ZKIR-VM-SYNTAX
```

The first probes return `Yellow` in 2.9 s and a `Native` term in 3.5 s. Full compilation times out after 25 minutes, exit 124, without emitting K's intermediate representation, KORE, and a full-definition execution attempt produces no result within ten minutes. So k-rust does not compile the definition within the tested bound; the receipt does not identify an unsupported construct or prove compilation impossible.

### Haskell backend proofs

`experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md` records one completed symbolic claim under the Haskell backend: a claim over a single `add` (`semantics/zkir-vm.k`) with a symbolic canonical native input proves, returning `#Top` in about ten seconds, once every configuration cell is given explicitly. The claim module is not in the repository; its postcondition, that the output register holds the input plus an immediate constant reduced modulo the native modulus `#r` of `semantics/zkir-field.k`, is therefore a description of the experiment and not a fact checkable from the repository. Omitting the explicit cells leaves decoding stuck.

A symbolic `transient_hash` claim does not terminate. `semantics/zkir-hash.k`, function `poseidonHash`, has concrete defining rules; symbolic unfolding reaches modular arithmetic through `modInt`, and the backend repeatedly reports `DecidePredicateUnknown`. An uninterpreted-hash proof interface is not implemented. These are recorded experiments, not a maintained proof suite: their claim files and Haskell definition reside outside the repository, which therefore supplies no portable replay command. See [15-design-rationale-and-limits.md](15-design-rationale-and-limits.md).
