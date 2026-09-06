# Claims over the ZKIR K semantics

The claims in this directory are reachability properties of the witness
semantics (`ZKIR-VM`) proved with the K Haskell backend. They quantify over
symbolic preimage values on fixed program terms.

## Running

From the repository root:

    cd experiments/zkir-k/semantics && kompile zkir-symbolic.k --backend haskell && cd -
    uv run --group zkir-k python experiments/zkir-k/tools/run_claims.py

The runner invokes `kprove` once per claim module against
`semantics/zkir-symbolic-kompiled` and prints one line per claim: the name,
`proved` or `not proved` with the reason, and the wall time in seconds. Options:
`--only name,...`, `--timeout seconds` (default 1200), `--log-dir DIR` for the
kprove output, `--definition DIR`. A single claim can also be run by hand:

    cd experiments/zkir-k/claims
    kprove add-spec.k --definition ../semantics/zkir-symbolic-kompiled --spec-module ADD-SPEC

`#Top` on standard output is a proof; anything else is a residual goal.

## The symbolic definition

`semantics/zkir-symbolic.k` (module `ZKIR-SYMBOLIC`) imports `ZKIR-VM` and adds
what the proofs need without changing a concrete run.

Uninterpreted hashes. The defining rules of `poseidonHash`, `transientCommit`,
`varLenSponge`, `hashToCurve`, `sha256Bytes` and `keccak256Bytes`
(`zkir-hash.k`) and of `ecMul` (`zkir-curves.k`) carry `[concrete]`, so they
evaluate only on fully concrete arguments. On a symbolic argument the
application stays an opaque term. The only property a claim can use of such a
term is functionality: two applications to the same argument are the same term,
which is what makes a register holding the hash agree with the gate that
recomputes it. Nothing is assumed about collision resistance, injectivity,
preimage resistance or the range of a digest, and no equation between digests
of different arguments can be derived. A concrete argument still evaluates to
the real digest, on both backends.

Reduction lemmas. `modInt` over a positive modulus is idempotent and a
congruence for `+Int`, `*Int` and negation, as `[simplification]` rules. They
let a claim state a field result in reduced form and meet the nested term the
semantics builds: `((A +Int B) *Int C) modInt #r` for the product of a sum,
against `(((A +Int B) modInt #r) *Int C) modInt #r`. The reduction of an
already-reduced value, `X modInt P => X` when `0 <= X < P`, is in the K prelude
(`INT-SYMBOLIC`, `domains.md`). `fadd`, `fmul` and `fneg` with the concrete
modulus `#r` unfold by their defining rules on symbolic operands, since their
only condition is on the modulus.

The observable result of a run, `[[P]](pre) = (status, outputs, pis)`, is read
off the `<status>`, `<outputs>` and `<pi>` cells once `job(P, pre)` has
finished. The `job` rule rewrites the whole `<k>` cell, so nothing can be
sequenced after it and the triple cannot be packaged into one term without
changing `ZKIR-VM`.

Every cell of the configuration is pinned in a claim. An unmentioned cell is
abstracted to a variable, and the input decoder then cannot choose between the
strict and the non-strict rule.

## What each claim establishes

All claims range over inputs `A`, `B`, `C` with `0 <= X < #r`, the native field
modulus, unless stated otherwise; the binding input is concrete.

`add-spec.k`, `ADD-SPEC`. On `add %a %b -> %o` the run ends with status `ok`,
`%o = (A + B) mod r`, the public-input vector is the binding input alone, and
every emitted gate (the two input gates, binding input, add) holds on the witness, so the final memory is in the modelled witness space. This is the
baseline claim of the second iteration, recovered.

`native-ops-spec.k`, one module per claim:

- `MUL-SPEC`: `%o = (A * B) mod r`, gates hold.
- `NEG-SPEC`: `%o = (0 - A) mod r`, gates hold.
- `COPY-SPEC`: `%o = A`, gates hold.
- `COND-SELECT-1-SPEC`, `COND-SELECT-0-SPEC`: with the bit immediate 1 the
  output is `A`, with 0 it is `B`; gates hold.
- `CONSTRAIN-TO-BOOLEAN-OK-SPEC`: an input equal to 1 passes, status `ok`,
  gates hold.
- `CONSTRAIN-TO-BOOLEAN-FAIL-SPEC`: an input `C` with `2 <= C < r` ends the run
  with `error("Expected boolean, found: " +String Int2String(C))` and the gate
  is `violated` on that witness.
- `ASSERT-OK-SPEC`: an input equal to 1 passes, status `ok`, gates hold.
- `ASSERT-FAIL-SPEC`: an input equal to 0 ends the run with
  `error("Failed direct assertion")` and the gate reports `assert of zero`.

`transient-hash-spec.k`, `TRANSIENT-HASH-SPEC`. On
`add %a %b -> %s ; mul %s %c -> %m ; transient_hash [%m, %a] -> %h` the run
ends with status `ok`, `%s = (A + B) mod r`, `%m = ((A + B) * C) mod r`,
`%h = poseidonHash([%m, A])` with the hash uninterpreted, the chip set is
`{poseidon}`, and all four gates hold, the hash gate because it recomputes the
same uninterpreted application. This is the claim that did not terminate in
the second iteration, when the hash unfolded.

`spec-compiled-observable.k`, `SPEC-COMPILED-OBSERVABLE`. The instance of the
compiler-obligation template `spec_compiled_observable` in
`tools/run_claims.py`, which relates a program term, a preimage and the
expected `(status, outputs, pis)`: `job(P, Pre)` from the initial configuration
finishes with those three cells, every other cell existential. The instance is the program
`add %a 1 -> %b ; impact 1 [%b]` on a symbolic input `A` whose public
transcript input equals the impacted value: status `ok`, no outputs, public
inputs `[7, (A + 1) mod r]`. The runner regenerates the file before each run;
edit the template, not the file.

The receipt of a run is `evidence/zkir-k-claims-<date>.txt`.
