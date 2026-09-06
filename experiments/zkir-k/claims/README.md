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
`proved` or `not proved` with the reason, the wall time in seconds, and the
result of the vacuity probe. Options: `--only name,...`, `--timeout seconds`
(default 1200), `--log-dir DIR` for the kprove output, `--definition DIR`,
`--stamp STAMP`, `--no-vacuity`. A single claim can also be run by hand:

    cd experiments/zkir-k/claims
    kprove add-spec.k --definition ../semantics/zkir-symbolic-kompiled --spec-module ADD-SPEC

A line that is exactly `#Top` on standard output is a proof; the prover's
"cannot be rewritten further" with a printed configuration is a residual goal.

Vacuity. A claim whose `requires` is unsatisfiable proves anything. The runner
therefore runs every module a second time as a companion probe, the same claim
with `ensures false`, which must not prove; a probe that proves reports the
claim as vacuous, which counts as not proved. `make claims` writes the receipt
`evidence/zkir-k-claims-<stamp>.txt` with the kprove logs beside it.

## The symbolic definition

`semantics/zkir-symbolic.k` (module `ZKIR-SYMBOLIC`) imports `ZKIR-VM` and adds
what the proofs need without changing a concrete run.

Uninterpreted hashes. The defining rules of `poseidonHash`, `varLenSponge`,
`hashToCurve`, `sha256Bytes` and `keccak256Bytes` and of their helpers
(`absorbAll`, `permute`, `#rounds`, `#htc`, `mapToCurve`, `#shaPad`,
`#shaBlocks`, `#kPad`, `#kAbsorb` in `zkir-hash.k`) and of `ecMul` with its
accumulator `#ecMulAcc` (`zkir-curves.k`) carry `[concrete]`, so they evaluate
only on fully concrete arguments. On a symbolic argument the application stays
an opaque term. What a claim has of such a term is its sort, its totality,
functionality (two applications to the same argument are the same term, which
is what makes a register holding the hash agree with the gate that recomputes
it) and, for `poseidonHash`, the range lemma below. No collision, injectivity
or preimage fact is assumed, and no equation between digests of different
arguments can be derived. `transientCommit` carries no `[concrete]`: it is the
definitional alias `transientCommit(Vs, R) = poseidonHash(ListItem(R) Vs)` and
unfolds once on any argument, so the witness-side commitment check
(`#commCheck`) and the commitment gate (`commGate`) meet on the same
`poseidonHash` term. A concrete argument still evaluates to the real digest, on
both backends.

Reduction lemmas. `modInt` over a positive modulus is idempotent and a
congruence for `+Int`, `*Int` and negation, as `[simplification]` rules. They
let a claim state a field result in reduced form and meet the nested term the
semantics builds: `((A +Int B) *Int C) modInt #r` for the product of a sum,
against `(((A +Int B) modInt #r) *Int C) modInt #r`. The reduction of an
already-reduced value, `X modInt P => X` when `0 <= X < P`, is in the K prelude
(`INT-SYMBOLIC`, `domains.md`). `fadd`, `fmul` and `fneg` with the concrete
modulus `#r` unfold by their defining rules on symbolic operands, since their
only condition is on the modulus.

Range lemmas. A Euclidean remainder by a positive modulus lies in `[0, P)`, and
a Poseidon digest lies in `[0, r)`. Both are needed by `wellTyped`
(`zkir-constraints.k`), which demands `0 <= x < r` of every native register,
on a register holding a reduced sum or a hash of symbolic inputs; the digest
lemma states the codomain of a total function into the field, not a property
of the hash.

The observable result of a run, `[[P]](pre) = obs(status, encoded outputs,
pis, skips)`, is written to the `<observable>` cell by `#observable` at the end
of `job(P, pre)` (`zkir-vm.k` `observable`), so a claim states it by pinning
that one cell; the eleven verdict claims pin it beside the other cells, and
the template claim pins it alone.

Every cell of the configuration is pinned in a verdict claim. An unmentioned
cell is abstracted to a variable, and the input decoder then cannot choose
between the strict and the non-strict rule. The `<witnessSpace>` cell, pinned
to `true`, states that the final memory is in the modelled witness space:
every gate holds or is unconstrained and every register satisfies
`wellTypedValue`, which the claims discharge from their `requires` and the
range lemmas.

## What each claim establishes

All claims range over inputs `A`, `B`, `C` with `0 <= X < #r`, the native field
modulus, unless stated otherwise; the binding input is concrete. Every claim
below pins `<observable>` to `obs(status, .List, ListItem(42), .List)` with
its status, and `<witnessSpace>` to the value stated.

`add-spec.k`, `ADD-SPEC`. On `add %a %b -> %o` the run ends with status `ok`,
`%o = (A + B) mod r`, the public-input vector is the binding input alone, and
every emitted gate (the two input gates, binding input, add) holds on the
witness, so the final memory is in the modelled witness space. This is the
baseline claim of the second iteration, recovered.

`native-ops-spec.k`, one module per claim:

- `MUL-SPEC`: `%o = (A * B) mod r`, gates hold.
- `NEG-SPEC`: `%o = (0 - A) mod r`, gates hold.
- `COPY-SPEC`: `%o = A`, gates hold.
- `COND-SELECT-1-SPEC`, `COND-SELECT-0-SPEC`: with the bit immediate 1 the
  output is `A`, with 0 it is `B`; gates hold.
- `CONSTRAIN-TO-BOOLEAN-OK-SPEC`: an input that is a boolean passes, status
  `ok`, gates hold. The module holds two claims, `C ==Int 0` and `C ==Int 1`,
  which kprove proves together: the disjunction `C ==Int 0 orBool C ==Int 1`
  in one `requires` does not prove, because `asBool` has one rule per value
  and the prover does not split the case.
- `CONSTRAIN-TO-BOOLEAN-FAIL-SPEC`: an input `C` with `2 <= C < r` ends the run
  with `error("Expected boolean, found: " +String Int2String(C))` and the gate
  is `violated` on that witness; the memory is outside the space.
- `ASSERT-OK-SPEC`: an input equal to 1 passes, status `ok`, gates hold.
- `ASSERT-FAIL-SPEC`: an input equal to 0 ends the run with
  `error("Failed direct assertion")` and the gate reports `assert of zero`.
- `ASSERT-NON-BOOLEAN-SPEC`: an input `C` with `2 <= C < r` ends the run with
  `error("Expected boolean, found: " +String Int2String(C))` while the gate,
  which demands a non-zero value only, holds, so the memory is in the modelled
  witness space although `preprocess` rejects the preimage (finding 2 of the
  divergence review as a proved statement).

`transient-hash-spec.k`, `TRANSIENT-HASH-SPEC`. On
`add %a %b -> %s ; mul %s %c -> %m ; transient_hash [%m, %a] -> %h` the run
ends with status `ok`, `%s = (A + B) mod r`, `%m = ((A + B) * C) mod r`,
`%h = poseidonHash([%m, A])` with the hash uninterpreted, the chip set is
`{poseidon}`, and all seven gates hold, the hash gate because it recomputes the
same uninterpreted application; the digest range lemma discharges the typing
of `%h`. This is the claim that did not terminate in the second iteration,
when the hash unfolded.

`commitment-spec.k`, `COMMITMENT-SPEC`. On `copy %a -> %o` with
`do_communications_commitment` true and the preimage commitment
`C = transientCommit([A], 5)` with the opening 5, the run ends with status
`ok`, `pi = [42, C]`, the chip set is `{poseidon}`, and all four gates hold:
the commitment gate recomputes `poseidonHash([5, A])` over the re-encoded
input and the empty outputs and meets the witness-side `transientCommit([A],
5)` because the alias unfolds to the same term. This is the shape of every
Moriarty artifact, all of which set the flag; before the alias unfolded, no
verdict-pinned claim with the flag could prove.

`spec-compiled-observable.k`, `SPEC-COMPILED-OBSERVABLE`. The observable
instance of the compiler-obligation template `spec_compiled_observable` in
`tools/run_claims.py`, which relates a program term, a preimage and the
expected `obs(status, outputs, pis, skips)`: `job(P, Pre)` from the initial
configuration finishes with that `<observable>` cell, every other cell
existential, or, with the template's `witness_space` parameter, with
`<witnessSpace>` and `<unconstrainedRegs>` pinned as well. The template runs
`job`, not `checkedJob`; `targetContract(P)` is discharged separately. The
instance is the program `add %a 1 -> %b ; impact 1 [%b]` on a symbolic input
`A` whose public transcript input equals the impacted value: status `ok`, no
outputs, public inputs `[7, (A + 1) mod r]`, skips `[skipNone()]`, witness
space true with no unconstrained register. The runner regenerates the file
before each run; edit the template, not the file.

The receipt of a run is `evidence/zkir-k-claims-<stamp>.txt`.
