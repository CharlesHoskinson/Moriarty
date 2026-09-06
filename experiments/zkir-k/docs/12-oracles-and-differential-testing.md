# Oracles and differential testing

The verification protocol compares off-circuit witness computation with the Rust crate, checks individual K functions, exercises selected in-circuit outcomes, and compares the verdict layer with the crate's own circuit under the halo2 MockProver. Each of these establishes a different predicate: agreement with `preprocess` does not establish agreement with the circuit, an accepted MockProver run does not establish that a proof verifies, a verified proof does not establish ledger acceptance, and none of them establishes equivalence for every program.

All commands start at the repository root; paths beginning with `tools/` or `semantics/` in prose are relative to `experiments/zkir-k/`.

## The Rust oracle

`zkir-oracle` is a local Rust harness crate in each pinned build workspace. Its `src/main.rs` calls `IrSource::load`, constructs a `ProofPreimage` from JavaScript Object Notation (JSON), and calls `IrSource::preprocess`. It does not invoke `circuit`, key generation, or proof verification; those belong to the second harness, `zkir-circuit-oracle`, described in the section on the circuit oracle below.

| Surface | Build workspace under `repos/_build/` | Dependency | Full source commit |
|---|---|---|---|
| Base surface | `ledger-92e8bdd3/` | `midnight-zkir-v3`, path `../zkir-v3` | `92e8bdd3a97b61b229e38916e1b180de6f448dd5` |
| Extension surface | `midnight-zkir-2ffe2d1/` | `midnight-zkir`, path `../zkir` | `2ffe2d17bbb736aec36fb300aeaca679a10d2278` |

Both workspaces carry two one-line visibility changes beyond the upstream commits. In `ir_vm.rs`, `pub(crate) fn preprocess` becomes `pub fn preprocess` (`zkir-v3/src/ir_vm.rs:185` at 92e8bdd3, `zkir/src/ir_vm.rs:210` at 2ffe2d1). In `ir_types.rs`, `IrValue::get_type` becomes `pub` (`zkir-v3/src/ir_types.rs:161`, `zkir/src/ir_types.rs:310`), because the harness prints `get_type()` as the `type` field. `encode_offcircuit` is already public. Each workspace `Cargo.toml` also lists `zkir-oracle` as a member; the crate directory itself is untracked. Build the harnesses with:

```bash
cargo build --release --manifest-path repos/_build/ledger-92e8bdd3/Cargo.toml -p zkir-oracle
cargo build --release --manifest-path repos/_build/midnight-zkir-2ffe2d1/Cargo.toml -p zkir-oracle
```

Each binary is `target/release/zkir-oracle` inside its workspace and accepts a program path followed by a preimage path. The receipts do not record build durations. `tools/diff_test.py`, `tools/unit_hash.py`, and `tools/divergence_tests.py` require the binaries at `~/Moriarty/repos/_build/<workspace>/target/release/zkir-oracle`, a path they construct from `Path.home()` without regard to the current directory. They do not follow the repository's `repos` symlink; that symlink only makes the `repos/_build/...` paths in the build commands above resolve from the repository root.

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

A returned preprocessing error prints `status: "error"` and `error`, omitting memory and vectors, and still exits zero. The Python adapter maps exit code 101 to `panic`; other nonzero exits become `load-error`. A failure in loading or preimage conversion, for instance `fr_from_dec` rejecting a noncanonical field element, happens before preprocessing begins. The oracle therefore provides no partial Rust memory for failed runs.

## Differential protocol

[`tools/diff_test.py`](../tools/diff_test.py) implements the following sequence, chiefly in `main`, `build_preimage`, and `compare`.

1. **Select programs.** `CORPORA` selects ledger tests, standalone precompiles, escrow and swap artifacts, and handmade programs. `--ext` switches to standalone tests, standalone precompiles, and handmade programs, together with the extension definition and oracle. Files are sorted within each corpus, and non-version-3 programs are skipped. `--only` matches a substring of the filename.

2. **Check format rejection.** If `zkir_kast.load_program` raises `ZkirFormatError`, the harness calls the oracle with empty inputs and binding input zero, and the comparison passes only if the adapter reports `load-error`. This tests the rejection of those particular artifacts rather than general parser equivalence.

3. **Seed inputs.** The pseudorandom generator is `random.Random(f'{args.seed}:{path.name}')`, with a default seed of 2026, and `zkir_values.small_encoded` draws values of the declared types. On the first attempt, if the program's manifest entry has a nonempty `test_preimage.inputs`, those literal values seed the preimage, together with whichever of binding input, private transcript and public transcript outputs the entry carries. For the ledger tests, [`extract_test_inputs.py`](../tools/extract_test_inputs.py) copies literal `ProofPreimage` fields from the Rust tests into `corpus/ledger9-92e8bdd3-tests/manifest.json`; the extension corpus has a separate manifest of the same shape, which this script does not produce. Later attempts use generated values.

4. **Discover transcript needs.** `build_preimage` makes at most eight generation passes. In `semantics/zkir-vm.k`, `genJob` enables `<genMode>`, and an active transcript read in `#input` that runs past the end of its transcript records `needPubOut(T)` or `needPriv(T)` and temporarily binds a default. The harness appends typed encodings for those requests, replaces the public transcript inputs with the `needPubIn` values, and uses `needComm` from `#finish` as the commitment with its sampled opening. It stops when no public-output or private requests remain, or when it reaches the pass limit. Generation assists preimage construction, but it does not guarantee a successful witness.

5. **Run both implementations.** The harness runs `Runner.run(program, pre)` and the oracle on the same preimage and compares every attempt. The default `--attempts 8` limits the candidate preimages per program, a bound separate from the generation-pass limit, and the attempts stop once both sides return `ok`.

6. **Perturb a success.** Unless `--no-perturb` is set, the harness compares each applicable mutation below. Each starts from the successful preimage, and transcripts and commitment are not regenerated afterward; a mutation may still succeed.

| Row label | Mutation | Condition |
|---|---|---|
| `perturbed-raw` | Replace the first raw field element with a random canonical field element | Raw inputs exist |
| `perturbed-typed` | Replace one randomly selected declared input's complete encoding with a valid random encoding of its type | Raw inputs exist |
| `wrong-pubin` | Increment one random public transcript input modulo the native field modulus | Public transcript inputs exist |
| `wrong-comm` | Increment the commitment modulo the native field modulus, preserving its opening | A commitment exists |

### Raw runs, checked runs, and comparisons

The differential harness uses raw `job`, not `checkedJob`. In [`zkir-vm.k`](../semantics/zkir-vm.k), `checkedJob` first evaluates `wf` through `#wfGate`, and a static failure produces an error prefixed `well-formedness:`. Static acceptance is an additional predicate, so checked execution is not interchangeable with testing the crate's `preprocess`. `tools/divergence_tests.py` passes `checked=True`, and `tools/check_corpus.py` tests the static expectations on its own. See [10-well-formedness-and-static-checks.md](10-well-formedness-and-static-checks.md).

`compare` first requires equal statuses. For `error` or `panic`, it compares classes from `ERROR_CLASSES`: ordered, case-insensitive regular expressions that normalize messages such as conversion, decoding, commitment, and transcript failures. Unmatched text becomes `other:` followed by its first 40 characters. Error wording is otherwise outside the comparison.

For `ok`, it requires identical `pis`, identical `pi_skips`, identical register sets, and equal `type` and `encoded` fields for every register. Both adapters print `variant`, but `compare` does not compare it directly. Length-bearing types matter because different byte-string lengths can share an encoding. Runner-level `stuck`, `depth-exhausted`, and `krun-failed` are not successful witnesses; see [11-tooling-reference.md](11-tooling-reference.md).

Each row prints `PASS` or `FAIL`, then the corpus, the filename, the run label and the details. Labels are `run<N>` for unperturbed attempts, the four mutation labels above, and `format`. If a generation pass returns `krun-failed`, the row is labelled `gen<N>` with the summary `krun failed`, counts as a disagreement, and ends the attempts for that program. The final line has this form:

```text
N comparisons: S successful-run agreements, E error-run agreements (status and error class), A agree, D disagree, Ts
```

`S` and `E` count matching status pairs; `D` is the number to read, because those two counters alone do not guarantee matching memory or error classes. Exit zero means no recorded difference. The `oracle 2:` line that follows counts successful unperturbed K attempts with at least one non-holding gate; it is informational, excludes perturbations, and does not affect the exit code. The verdicts themselves are compared with the crate's circuit in the `--circuit` mode described in the next section, which runs the same preimages through the halo2 MockProver; without that flag they are checked only by this count and by the divergence cases below. Gate outcomes are explained in [08-constraints-and-verdicts.md](08-constraints-and-verdicts.md).

## Results and reproduction

The differential receipts record the following:

| Receipt under `evidence/` | Agree / total | Successful-run | Error-run | Format rejection | Duration |
|---|---:|---:|---:|---:|---:|
| [zkir-k-differential-92e8bdd3-2026-09-06d.txt](../../../evidence/zkir-k-differential-92e8bdd3-2026-09-06d.txt) | 375/375 | 56 | 316 | 3 | 437 s |
| [zkir-k-differential-ext-2ffe2d1-2026-09-06d.txt](../../../evidence/zkir-k-differential-ext-2ffe2d1-2026-09-06d.txt) | 428/428 | 53 | 371 | 4 | 250 s |

Both report zero disagreements, zero successful unperturbed runs with a non-holding gate, and every successful K run in the modelled witness space (41 of 41 on the base surface, 40 of 40 on the extension). The format rows account for the difference between the total comparisons and the successful and error rows. Totals count repeated failed attempts and mutations, not distinct successful programs: the 56 successful-run rows of the base receipt are 34 unperturbed runs (21 ledger tests, 12 handmade programs and swap `expire.zkir`), the 7 `context` rows of the Moriarty artifacts on their real preimages, and 15 perturbed preimages that still succeed (6 `perturbed-raw`, 9 `perturbed-typed`); the 53 of the extension receipt are 40 unperturbed runs, 3 `perturbed-raw` and 10 `perturbed-typed`. With generated inputs, no preimage for the escrow artifacts, the six precompiles or the swap artifacts other than `expire.zkir` produces a successful witness, so their `run` rows compare error paths only, up to eight attempts per program; `expire.zkir` reaches `K=ok Rust=ok` on `run2`, its third attempt, with 28 registers, 394 public inputs and 571 verdicts. The `context` rows are the seven real transaction contexts of `corpus/moriarty-contexts/` (`tools/moriarty_preimages.mjs`, `11-tooling-reference.md`): each escrow and swap entry point is `K=ok Rust=ok` on its context, and `decide.zkir` reaches 37 registers, 495 public inputs and 721 verdicts. Those are one scenario per entry point, not coverage of the transaction contexts a ledger would present.

With K v7.1.337, matching pyk, compiled definitions, and both binaries available, reproduce the full differential runs with:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --seed 2026 --attempts 8
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --ext --seed 2026 --attempts 8
```

The receipts support a budget of roughly fifteen and thirteen minutes respectively, subject to machine load. For a quick protocol check, add `--only native_identity` to the base command or `--only bool_identity` to the extension command; each focused run produces four agreeing comparisons in about one second. Setup is described in [02-getting-started.md](02-getting-started.md).

| Additional receipt under `evidence/` | Recorded result | Timing |
|---|---|---|
| [zkir-k-unit-values-2026-09-06d.txt](../../../evidence/zkir-k-unit-values-2026-09-06d.txt) | 49/49 checks | About 7 s in a fresh local run |
| [zkir-k-unit-hash-2026-09-06d.txt](../../../evidence/zkir-k-unit-hash-2026-09-06d.txt) | 18/18 checks | About 3 s in a fresh local run |
| [zkir-k-milestone2-corpus-check-2026-09-06e.txt](../../../evidence/zkir-k-milestone2-corpus-check-2026-09-06e.txt) | 66 programs as expected; zero unexpected | 8.2 s recorded |
| [zkir-k-divergence-tests-2026-09-06e.txt](../../../evidence/zkir-k-divergence-tests-2026-09-06e.txt) | 31/31 cases, each with its circuit outcome | Not recorded in the receipt |

The first three checks run with:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py
```

The divergence suite checks expected statuses, the outcome of the first matching target gate in `all_verdicts` (a missing target gate fails), typed memory when both runs succeed, and the circuit oracle's outcome on the case, with the predicted cell for the injected cases. It tests selected instruction-level relations and their whole-circuit outcome on one preimage each, not proof acceptance; see [13-known-divergences.md](13-known-divergences.md). Its normal entry point rewrites `corpus/divergence/`. To reproduce the assertions without touching repository files, redirect `OUT` to scratch:

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

## The circuit oracle

`zkir-circuit-oracle` is the second harness crate, one source under `tools/circuit-oracle/ledger-92e8bdd3/` and `tools/circuit-oracle/midnight-zkir-2ffe2d1/` built as a member of each workspace (`target/release/zkir-circuit-oracle`, the paths `circuit_compare.py` and `provability.py` construct from `Path.home()`). It loads the program and the preimage as `zkir-oracle` does, runs `IrSource::preprocess`, builds the stdlib's `MidnightCircuit::new(&ir, Value::known(pis), Value::known(preprocessed), k)` with the known instance and witness (the wrapper configures `used_chips` and loads the lookup tables, which a bare `Relation::circuit` would skip), takes `k` from `optimal_k`, runs `MockProver::run` and calls `verify`. The MockProver checks every constraint of the synthesised circuit against the assigned cells without producing a proof and needs no proving key or structured reference string. The harness prints one JSON line whose `outcome` names where the run stopped, in the order the crate produces them:

| Outcome | Where it arises |
|---|---|
| `preprocess-error` | `preprocess` returns `Err`; no circuit is built |
| `witness-consistency-error` | `MockProver::run` returns the `error_if_known_and` error that `mem_insert` or `pi_push` stashed in `Relation::circuit`, and the run logged the crate's misalignment event; both are required |
| `synthesis-error` | any other `Err` from `MockProver::run`, a synthesis error that depends on the known witness |
| `constraint-failure` | `verify` returns the failing constraints |
| `panic` | a panic anywhere after loading, including every witness-independent `Error::Synthesis`, which `optimal_k` unwraps |
| `accepted` | `verify` returns `Ok` |

A seventh line, `instance-length-mismatch`, is a harness outcome rather than a verdict on the witness: the instance column handed to the MockProver has a different length from the number of `pi_push` calls, and a longer column leaves surplus cells unconstrained while a shorter one fails on the first missing cell. A malformed `--inject`, `--pis` or `--instance` file is `inject-error` with exit status 2, a harness error that is never counted. `--inject FILE` replaces Native entries of the preprocessed memory before the circuit is built, as `prove_unchecked` intends; `--pis` and `--binding-input` replace the witness-side public-input vector (the instance column follows it, so a changed entry surfaces in `pi_push`); `--instance FILE` sets only the instance column, which is the one way to reach a constraint failure on public-input cells; `--model-only` reports `k` and the row count of the cost model without running the prover. The typed interface bounds what an injection can express: `IrValue` is typed, so only Native values, the public-input vector, the binding input and the commitment can be perturbed, and an off-curve point, a non-canonical scalar or an out-of-range limb tuple cannot be written through it.

### The comparison table

There is no one-to-one map from K's outcomes to the oracle's. `plan-iter3/circuit-comparison-table.md` was written before the first `--circuit` run and is the contract of the comparison: its rows are K's summary of a run (`ok`, `ok/synthErr`, `ok/violated`, `ok/unknown`, `ok/unsupported`, `error`, `panic`), its columns the honest preimage, the four preimage perturbations, the injection columns (`inject-input`, `inject-computed`, `inject-ignored`, `inject-transcript`, `inject-unconstrained`, each with a `:bound` variant that sets the register to the boundary value of its first consumer) and `instance`, and every cell is either an expected agreement or a named, accepted difference, D1 to D10, with the cell code the oracle must report: `A` accepted, `P` preprocess-error, `W` witness-consistency-error, `S` synthesis-error, `C` constraint-failure, `X` panic with a message class. A `violated` verdict is realised per gate (D1: an equality assertion is `C`, a recomputed `mem_insert` output is `W`, a hint that panics is `X`); a `synthErr` is `X` with the unwrap, chip or width message class (D2); an injection is predicted by a taint walk over the instruction list and the honest memory (D3 to D5); an `error` row is `P` whatever the gates say, because no circuit is built (D9). A comparison that lands outside the table is a blocking finding, reported in the receipt and never absorbed into the table; a cell the walk cannot decide is printed as not comparable and never counted as agreement. `tools/circuit_compare.py` is the executable form of the table.

### The `--circuit` mode

`diff_test.py --circuit` (`make -C experiments/zkir-k circuit`) runs the circuit oracle after every row of the differential run: on the honest preimage and the four perturbations of every program, and on a joint `ok` also on the injections `choose_injections` picks (a declared Native input, a computed register, a register stored by a bypass arm, a transcript register with guard 1 and an `unconstrained` one, each with its boundary value) and on the instance perturbation. Each comparison prints a `CIRC` line with the K summary, the oracle outcome and time, the expected cell with its difference name, and `AGREE` or the reason it could not be compared, and the footer is `circuit: N comparisons against the table: N agree, 0 outside the table, 0 not comparable, T in the oracle` followed by `cells hit:` with the count per cell. The two receipts of the third iteration:

| Receipt under `evidence/` | Comparisons against the table | Outside the table | Not comparable | Cells hit |
|---|---:|---:|---:|---|
| [zkir-k-circuit-differential-92e8bdd3-2026-09-06d.txt](../../../evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06d.txt) | 506, all agree | 0 | 0 | `ok: A` 56, `error: P` 316, `instance: C` 41, `inject: W` 48, `inject: C` 6, `inject: A` 5, `inject-bound: W` 9, `inject-bound: C` 1, `inject-bound: X` 2, `inject-ignored: A` 7, `inject-transcript: W` 10, `inject-transcript: C` 1, `inject-transcript: A` 1, `inject-unconstrained: A` 3 |
| [zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06d.txt](../../../evidence/zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06d.txt) | 531, all agree | 0 | 0 | `ok: A` 53, `error: P` 371, `instance: C` 40, `inject: W` 39, `inject: C` 7, `inject: A` 5, `inject-bound: W` 8, `inject-bound: C` 1, `inject-bound: X` 2, `inject-ignored: A` 1, `inject-transcript: W` 2, `inject-transcript: C` 1, `inject-transcript: A` 1 |

The 48 s and 41 s the two runs spend in the oracle come on top of the differential timings above; the largest corpus circuits are the curve programs at `k = 16` (`test_curve25519_ec_mul_proof.zkir`, `curve_curve25519.zkir`), which the spike receipt `evidence/zkir-k-circuit-oracle-spike-2026-09-06.txt` sized at under a minute and well under 1 GB. What these receipts establish is the two tested claims of 08-constraints-and-verdicts.md: no `violated` or `synthErr` verdict on a circuit-accepted witness (the soundness direction), and the predicted cell on every injection (the tightness direction), each up to its residual list. The 31 divergence cases run the same comparison one case at a time.

### Keygen and prove with the official parameters

`tools/provability.py` (`make -C experiments/zkir-k provability`) runs the oracle's two provability modes, which use the structured reference string rather than the MockProver. `--keygen` runs `Zkir::keygen_vk` and `Zkir::keygen`, both an unknown-witness synthesis, and reports `k`, the `k` recorded in the proving key and whether the two verifier keys serialise equal; `--prove` runs keygen, `preprocess`, `Zkir::prove` with a fixed ChaCha20 seed and `VerifierKey::verify` against the crate's embedded `PARAMS_VERIFIER`, and reports the proof size and the three times. The parameters are the official Midnight KZG parameters `bls_midnight_2p{k}` for `k` from 0 to 17, fetched once from `https://srs.midnight.network/` (the host midnight-base-crypto downloads from), verified against the SHA-256 digests in `base-crypto/src/data_provider.rs`, and read from `--params DIR`, else `$MIDNIGHT_PP`, else `repos/_build/params`, exactly as the crate's test provider `TestParams` reads them; a missing file for the circuit's `k` is the outcome `params-unavailable`, never a silent skip. No `unsafe_setup` parameters are used.

The sweep keys every parseable corpus program, whatever its contract says, and classifies each by the stage tags of its failed obligations (16-compilation-target-contract.md): `clean` and the two `keyed-despite` classes must key, `must-reject` must fail keygen by a panic or a synthesis error whose message names the failed check, and a disagreement in either direction is a contradiction. It then proves and verifies the seven Moriarty contexts and the six spike preimages. The receipt [zkir-k-provability-2026-09-06e.txt](../../../evidence/zkir-k-provability-2026-09-06e.txt) keys 162 programs, 137 accepted and 25 rejected, in the classes 125 `clean`, 2 `keyed-despite-preprocess`, 3 `keyed-despite-static` and 25 `must-reject`, with 8 programs the loader rejects on both sides, 13 proofs verified, 23 of 23 negative controls rejected on the named check, 4 of 4 keyed-despite controls keyed, and 0 contradictions, in 125 s. Its predecessor [zkir-k-provability-2026-09-06d.txt](../../../evidence/zkir-k-provability-2026-09-06d.txt) found three contradictions, an undefined operand, a one-output `div_mod_power_of_two` and a 256-bit `reconstitute_field` keyed by the stage tags and rejected by keygen, which the obligations `circuit.static.defined`, `circuit.static.div_mod_outputs` and `width.reconstitute_field.assertion` close.

### What a verified proof establishes

A `prove=accepted` line is a proof, produced with the official parameters and verified by the crate's verifier against the public inputs `prove` itself returned. It is not ledger acceptance, and the receipt header names three residuals. The ledger verifies a contract call against the statement `[binding_input, communications_commitment, field_repr(guaranteed), field_repr(fallible)]` (`ledger/src/verify.rs`), which it assembles from the transaction, not against the `pis` vector of `prove`. The ledger pushes the commitment unconditionally, so a program without `do_communications_commitment` keys and proves here and can never satisfy a ledger statement; the receipt reports this as the `ledger.commitment` fact of each context, not as a failed obligation. The verifier key is regenerated from the program for every run and never compared with a deployed key; `--vk FILE` is specified for when key files exist. A keyed program with a verified proof on one preimage therefore establishes that the circuit can be built, keyed and proved for that witness with the real parameters, and nothing about other witnesses, other statements or deployed keys.

## Other evidence sources

### Agda

The [arc-zkir receipt](../../../evidence/arc-zkir-agda-typecheck-2026-09-05.txt) records successful type-checking of `zkir-v3/Main.agda` at `fd1c24e1`, under `--safe`:

```bash
(cd repos/input-output-hk/arc-zkir/src && nix run .#agda -- zkir-v3/Main.agda)
```

No duration is recorded. `Main.agda` takes an `Assumptions` record that provides the field, curve and hash primitives. The development has no concrete instance that executes corpus programs, so it supplies a specification comparison rather than an executable differential oracle; type-checking alone does not establish correspondence with K or Rust.

### k-rust

The [compatibility receipt](../../../evidence/k-rust-compatibility-2026-09-05.txt) uses k-rust 0.4.0 at `687ccd0`; its Rust build takes 4 min 35 s. These commands reproduce the build, the tutorial run, the syntax probe, and the bounded compilation attempt:

```bash
cargo build --release --manifest-path repos/midnightntwrk/k-rust/Cargo.toml -p k-rust
repos/midnightntwrk/k-rust/target/release/krust krun experiments/zkir-k/toolchain-check/lesson-02-a.k --main-module LESSON-02-A --sort Color -e 'colorOf(Banana())'
repos/midnightntwrk/k-rust/target/release/krust kast experiments/zkir-k/semantics/zkir-syntax.k --module ZKIR-SYNTAX --sort IrType --expression 'native()'
timeout 1500 repos/midnightntwrk/k-rust/target/release/krust kcompile experiments/zkir-k/semantics/zkir.k --main-module ZKIR --syntax-module ZKIR-VM-SYNTAX
```

The tutorial run returns `Yellow` in 2.9 s, and the syntax probe returns a `Native` term in 3.5 s. Full compilation times out after 25 minutes with exit 124 and emits no KORE (K's intermediate representation); a full-definition execution attempt produces no result within ten minutes. Within the tested bound, k-rust does not compile the definition, and the receipt neither identifies an unsupported construct nor proves compilation impossible.

### Haskell backend proofs

The claim modules under `experiments/zkir-k/claims/` are reachability properties of the witness semantics proved with the Haskell backend against `semantics/zkir-symbolic-kompiled` (`kompile zkir-symbolic.k --backend haskell`, or `make -C experiments/zkir-k zkir-symbolic`), and `tools/run_claims.py` (`make -C experiments/zkir-k claims`) runs each with `kprove` and runs it a second time with `ensures false` as a vacuity probe that must not prove. The receipt [zkir-k-claims-2026-09-06d.txt](../../../evidence/zkir-k-claims-2026-09-06d.txt) records fourteen claims proved in ten to twelve seconds each with every probe refuted: the native arithmetic and control instructions, `transient_hash`, the commitment claim and the observable instance of the compiler-obligation template. `ZKIR-SYMBOLIC` marks the defining rules of the hashes `[concrete]`, so the `transient_hash` claim that did not terminate in the second iteration, where `poseidonHash` unfolded to `modInt` arithmetic, now proves against an uninterpreted application. The claims and what they do not cover are described in [16-compilation-target-contract.md](16-compilation-target-contract.md), `claims/README.md` and [15-design-rationale-and-limits.md](15-design-rationale-and-limits.md).
