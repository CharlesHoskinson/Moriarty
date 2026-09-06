# R4: symbolic backend, claims, and tooling

VERDICT: WARNING — the twelve claims match the README and are not vacuous, `[concrete]` keeps the named hash symbols from unfolding, but the symbolic theory does not identify `transientCommit` with `poseidonHash`, the compiler-obligation template is not a compiler-correctness statement, and several tools can exit 0 on an empty or failed run.

## Findings

### 1. major — `[concrete]` on `transientCommit` splits the commitment check from `commGate`

`zkir-hash.k:81` is the only equation relating the two names:

```
rule transientCommit(Vs, Rand) => poseidonHash(ListItem(Rand) Vs) [concrete]
```

The off-circuit check uses the left-hand name (`zkir-vm.k:582`):

```
requires Cm ==Int transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
```

The in-circuit gate uses the right-hand name (`zkir-constraints.k:132`):

```
requires "poseidon" in Chips andBool poseidonHash(ListItem(Rand) Enc #encodeAll(Os)) ==Int C
```

`[concrete]` means that equation is available only when `Vs` and `Rand` are ground. On a symbolic input list the two applications stay distinct uninterpreted terms. The kompiled Haskell definition records the same attribute (`semantics/zkir-symbolic-kompiled/parsed.txt`, rule `transientCommit(_,_)_ZKIR-HASH_Int_List_Int`).

The README (`claims/README.md:32-39`) and `zkir-symbolic.k:9-16` say that `[concrete]` makes those symbols uninterpreted and that "the only property a claim can use of such a term is functionality". Functionality of each symbol separately does not give `transientCommit(Vs, Rand) = poseidonHash(Rand ++ Vs)`.

This is the program that exposes it: any `do_communications_commitment: true` circuit (every Moriarty artifact under `experiments/moriarty-core-swap/output/zkir/` and `experiments/moriarty-compact-escrow/output/zkir/`) with a symbolic input `A` and an honest opening, e.g. `C = transientCommit(ListItem(A), Rand)`. The VM path accepts (`Cm` matches `transientCommit`). `eval(commGate(...))` stays `#commOutcome(..., poseidonHash(ListItem(Rand) ListItem(A)), C)` and does not reduce to `holds()`, so `<witnessSpace>` cannot be proved `true`. The twelve claims all use `noComm()` (`add-spec.k:13`, `native-ops-spec.k:12`, `transient-hash-spec.k:18`, `spec-compiled-observable.k:12`) and so do not see it. Instantiating `spec_compiled_observable` on `swap-expire` or `escrow-fund` with a symbolic input would.

Fix: drop `[concrete]` from `zkir-hash.k:81` (keep it on `poseidonHash` at line 73). The bridge is definitional, not computational. Alternatively, change `zkir-vm.k:582-584` to `poseidonHash(ListItem(Rand) ...)` so both layers mention one symbol.

### 2. major — `spec_compiled_observable` is an observation instance, not a compiler obligation

The template (`tools/run_claims.py:56-98`) claims `job(P, Pre) => .K` with `<observable> obs(status, outputs, pis)` and every other cell existential, including `<status>`, `<outputs>`, `<pi>`, `<witnessSpace>`, `<verdicts>`, `<constraints>` and `<doComm>`. The generated instance (`claims/spec-compiled-observable.k:11-29`) is `add %a 1 -> %b ; impact 1 [%b]` with `noComm()`, binding input `7`, and `requires 0 <=Int A <Int #r`. Chapter 16 (`docs/16-compilation-target-contract.md:53`) describes this as "one instance of the completeness direction".

A Moriarty compiler proof needs, and this template does not give:

- A compilation relation `compile(S) = P`. `P` is a literal term.
- Source-level execution producing `Pre`. `Pre` is a hand-written K term.
- Tier one: `wf(P)` / `targetContract(P)`. The claim runs `job`, not `checkedJob` (`zkir-vm.k:108` vs `118`).
- Tier two: `<witnessSpace> true` and empty `<unconstrainedRegs>` (or an explicit residual). Both cells are `?_` (`run_claims.py:69`).
- Tier three: circuit acceptance. Not a K cell.
- Soundness: every memory in `witnessSpace(P, pi)` is a source execution. The claim is one honest preimage.
- Quantification over source states. One `A`.
- Communications commitment. The instance has `noComm()`; every compiled Moriarty circuit has `"do_communications_commitment": true` (e.g. `experiments/moriarty-core-swap/output/zkir/expire.zkir`). Finding 1 then applies.
- Binding input as the ledger's transaction hash. The instance pins a constant `7`.

Leaving `<status>`, `<outputs>` and `<pi>` existential is not itself vacuous: `#observable` (`zkir-vm.k:555-556`) writes `obs(St, #encodeAll(Os), Pi)`, so the postcondition determines those three cells. The gap is everything the compiler statement must say besides that triple.

Fix: a compiler-obligation module with parameters `(S, P, Pre)` that (i) requires `P = compile(S)` and `Pre = sourcePreimage(S, ctx)`, (ii) uses `checkedJob`, (iii) pins `<observable>`, `<witnessSpace>` and `<unconstrainedRegs>`, (iv) states `targetContract(P)` as `met`, (v) has a separate non-K obligation for the MockProver. Keep the current instance as a smoke test of the observable cell, under a name that does not say "compiler-obligation".

### 3. major — Makefile `| tee` hides a failing tool behind exit 0

`experiments/zkir-k/Makefile` has no `.SHELLFLAGS`, no `pipefail`, and no `-` prefix. Every receipt target is a pipeline whose last command is `tee`:

```
27:	$(UV) $(TOOLS)/contract_corpus.py  | tee $(RECEIPT)/zkir-k-contract-corpus-$(DATE).txt
30:	$(UV) $(TOOLS)/moriarty_contexts.py --ext | tee $(RECEIPT)/zkir-k-moriarty-contexts-$(DATE).txt
33-34, 40-47: the same pattern for circuit, unit, corpus, divergence, differential
```

POSIX `sh` (Make's default) takes the exit status of `tee`. If `moriarty_contexts.py` returns 1, `make contexts` still succeeds and `make check` prints `all six check layers ran` (`Makefile:50`). The receipt file can contain `N failing rows` while the process that was supposed to gate the iteration exits 0.

`DATE ?= $(shell date +%F)` (`Makefile:14`) also writes `zkir-k-moriarty-contexts-2026-09-06.txt` with no `c` suffix, so a same-day rerun overwrites the receipt the review is looking at.

Fix: `.SHELLFLAGS := -eu -o pipefail` (or `set -o pipefail` in each recipe), and pass a suffix into `DATE` for iteration receipts. Stop `check` from echoing success except as the last command of a recipe that already failed on a non-zero tool.

### 4. minor — `--only` with no matches exits 0 and prints a success summary

`tools/run_claims.py:161-175`: `--only bogon` selects nothing, `total = 0`, `proved == total`, prints `0/0 claims proved`, returns 0.

`tools/moriarty_contexts.py:133-152`: `--only` defaults to `''`, and `names = [n for n in manifest['artifacts'] if args.only in n]`. `--only nosuch` yields `names = []`, `failures = 0`, prints `all rows as expected`, returns 0. Substring match also means `--only swap` silently drops the three escrow artifacts without a warning.

A receipt produced with a mistyped `--only` is indistinguishable from a full green run.

Fix: if `selected` is non-empty and no name matched, or if `names` is empty, exit 2. Require `--only` to match a whole artifact name.

### 5. minor — `run_claims.py` treats kprove's trivial-claim success as `proved`

Success is `rc == 0 and '#Top' in out` (`tools/run_claims.py:138-139`). kprove prints `#Top` and `WarnTrivialClaim` for a claim whose `requires` is unsatisfiable (the left-hand side is never reached). None of the twelve committed claims have an unsatisfiable `requires`, so the receipt `evidence/zkir-k-claims-2026-09-06c.txt` is not itself vacuous. The runner would still record such a claim as `proved` and exit 0.

`#Top` is also a substring test, not "stdout is exactly the remaining goal `#Top`". The add-claim log starts with `#Top` then compiler warnings; that is the intended success path. A residual goal whose pretty-print happened to contain the four characters `#Top` would also pass. kprove residual goals in this definition have not been observed to do that.

Fix: fail the row on `WarnTrivialClaim` / `Claim proven during initialization`. Require a line that is exactly `#Top` (or the kore-exec success status) rather than a substring.

### 6. minor — `upstream_drift.py` is blind to field-type changes

`variants()` (`tools/upstream_drift.py:225-228`) records struct-variant payload as sorted field names only:

```
fields = tuple(sorted(re.findall(..., inner_no_attrs)))
payload = '{' + ', '.join(fields) + '}'
```

`compare()` (`line 323`) then treats `payload` inequality as "shape changed". `keccak256` growing `output` into `outputs` is visible (receipt `evidence/zkir-k-upstream-drift-2026-09-06.txt:15`). Changing `output: String` to `output: Vec<String>` while keeping the name is not. A counted "no drift" / exit 0 (`upstream_drift.py:421`, receipt line 59) can hide an ABI change that still parses as the same serde names.

The ledger-8 comparison in that receipt is labelled informational because the pin is not an ancestor (`upstream_drift.py:379-387`). That part of the receipt is honest. The type-blind payload is the defect.

Fix: include each field's type text in `payload` (the substring after `name:` up to `,` or `}`), and count a type-only change as drift.

### 7. minor — `[concrete]` does not cover the helpers, and functionality is not the only remaining property

The defining rules of `poseidonHash`, `transientCommit`, `varLenSponge`, `hashToCurve`, `sha256Bytes`, `keccak256Bytes` (`zkir-hash.k:73,81,86,91,170,238`) and both `ecMul` rules (`zkir-curves.k:126-127`) carry `[concrete]`. The kompiled Haskell definition has the same attributes. `TRANSIENT-HASH-SPEC` proving in 8.7 s (`evidence/zkir-k-claims-2026-09-06c.txt:18`) is the empirical check that `poseidonHash` of a two-element list of symbolic natives does not enter `absorbAll` / `permute` / `sbox`.

Helpers those rules rewrite to do not have `[concrete]`:

- `absorbAll`, `permute`, `#rounds`, `sbox`, `linear` (`zkir-hash.k:39-69`)
- `#htc`, `mapToCurve`, `#svdw*` (`zkir-hash.k:92-137`)
- `#shaPad`, `#shaBlocks`, `#kPad`, `#kAbsorb` (`zkir-hash.k:171-249`)
- `#ecMulAcc` (`zkir-curves.k:128-134`), which is a double-and-add loop on the bits of `K`

A claim or a residual term that names `#ecMulAcc` or `mapToCurve` still unfolds. `ecMul(C, _, K) => identity(C) requires K <=Int 0 [concrete]` (`zkir-curves.k:126`) binds only `C` and `K`; a concrete non-positive scalar with a symbolic point still reduces to `identity(C)`. That is more than functionality.

The symbols are also `[function, total]` (`zkir-hash.k:72,80,85,90,167,237`). Totality and the result sort (`Int` / `Point` / `Bytes`) remain. Integer lemmas apply to a `poseidonHash(...)` term. The README sentence "the only property a claim can use of such a term is functionality" (`claims/README.md:34-35`) is therefore too strong. Collision resistance, injectivity and range are correctly not assumed: there is no lemma that rewrites `poseidonHash(Xs)` for symbolic `Xs`, and `poseidonHash([A]) =/= poseidonHash([B])` is not an available fact.

Fix: mark the computational helpers `[concrete]` as well (`#ecMulAcc`, `absorbAll`, `permute`, `#shaPad`, `#kAbsorb`, `mapToCurve`). Keep a non-concrete equation only for definitional bridges (finding 1). Narrow the README to: no collision / injectivity / range lemma; totality and the result sort remain; helpers without `[concrete]` are still interpreted.

### 8. nit — claim file comment disagrees with the claim it introduces

`add-spec.k:4` says "both emitted gates (binding input, add) hold". The claim body lists four verdicts: two `inputGate`s, `bindGate(0)`, and the add gate (`add-spec.k:26-29`). The README (`claims/README.md:68`) names the four. The claim is the one that was proved; the header is stale.

### 9. nit — `make kompile` / `make check` never touch the Haskell definition or the claims

`Makefile:16,20-24` kompiles `zkir`, `zkir-ext`, `zkir-check`, `zkir-test`, `zkir-contract-main` with `--backend llvm`. There is no `zkir-symbolic` target and no `claims` target. `make check` is four LLVM unit/corpus/divergence layers plus two `diff_test.py` runs. A green `make check` is not evidence that the twelve claims still prove or that `zkir-symbolic-kompiled` matches `zkir-symbolic.k`. `docs/11-tooling-reference.md:15-20` still lists four LLVM directories and does not mention `run_claims.py`, `upstream_drift.py`, `moriarty_preimages.mjs` or `moriarty_contexts.py`.

## The twelve claims against the README

All twelve `requires` clauses are satisfiable. Postconditions are not implied by the preconditions without running `ZKIR-VM`. `false` variants (wrong output register, e.g. `(A +Int B +Int 1) modInt #r`) do not prove. None of the twelve leave `<k>` or the cells they claim to constrain as free variables.

| Claim | README (`claims/README.md`) | Claim file | Vacuity |
|---|---|---|---|
| `ADD-SPEC` | `%o = (A+B) mod r`, PI = `[42]`, four gates hold, `witnessSpace` true | `add-spec.k:12-35` matches | Not vacuous. `0 <= A,B < #r` does not imply the memory or the verdicts. |
| `MUL-SPEC` | `%o = (A*B) mod r`, gates hold | `native-ops-spec.k:11-33` | Same. |
| `NEG-SPEC` | `%o = (0-A) mod r` | `native-ops-spec.k:38-60` | Matches `fneg` (`zkir-field.k:49`). |
| `COPY-SPEC` | `%o = A` | `native-ops-spec.k:65-87` | The copy is the content; still has to run `#exec(copy)` and `eval(gate(copy))` (`zkir-constraints.k:345`). |
| `COND-SELECT-1/0-SPEC` | bit immediate 1 selects `A`, 0 selects `B` | `native-ops-spec.k:92-146` | Bit is ground; values are symbolic. Matches `selectV` / `#selBit` (`zkir-constraints.k:308-309`). |
| `CONSTRAIN-TO-BOOLEAN-OK-SPEC` | input equal to 1, status `ok`, gates hold | `native-ops-spec.k:151-176`, `requires C ==Int 1` | Singleton. README states it. Not implied by `C==1` without the VM: the error string and the gate message are different functions (`zkir-ops.k:353` vs `zkir-constraints.k:234`). |
| `CONSTRAIN-TO-BOOLEAN-FAIL-SPEC` | `2 <= C < r` → `error("Expected boolean, found: " + C)` and gate `violated("constrain_to_boolean is not boolean: " + C)` | `native-ops-spec.k:178-204` | Matches `asBool` (`zkir-ops.k:353`) and `#boolean` (`zkir-constraints.k:234`). `<witnessSpace> false` is correct: the violated gate makes `witnessSpace` false (`zkir-constraints.k:173-174`). |
| `ASSERT-OK-SPEC` | input 1, status `ok` | `native-ops-spec.k:209-233` | Singleton, as the README says. |
| `ASSERT-FAIL-SPEC` | input 0 → `error("Failed direct assertion")`, gate `violated("assert of zero")` | `native-ops-spec.k:235-260` | Matches `zkir-vm.k:307` and `zkir-constraints.k:272`. |
| `TRANSIENT-HASH-SPEC` | `%s=(A+B) mod r`, `%m=((A+B)*C) mod r`, `%h=poseidonHash([%m,A])` uninterpreted, chips `{poseidon}`, four gates hold | `transient-hash-spec.k:13-48` | The modInt congruence lemmas (`zkir-symbolic.k:46-49`) are what make `%m`'s reduced form meet `fmul(fadd(...), ...)`. Hash register and hash gate both mention `poseidonHash`, so they match without finding 1. |
| `SPEC-COMPILED-OBSERVABLE` | `obs(ok, .List, [7, (A+1) mod r])` | `spec-compiled-observable.k:11-29` | See finding 2. The triple is determined. The unconstrained witness-space / verdict cells mean the claim does not say the gates hold. |

The six `[simplification, preserves-definedness]` rules (`zkir-symbolic.k:40-51`) are Euclidean-mod congruences for `P > 0`:

- `(X mod P) mod P = X mod P`
- `((X mod P) + Y) mod P = (X + Y) mod P` and the symmetric form
- `((X mod P) * Y) mod P = (X * Y) mod P` and the symmetric form
- `(0 - (X mod P)) mod P = (0 - X) mod P`

No counterexample on `Int` with `P > 0`. `preserves-definedness` is present on all six; `modInt` is undefined at `P = 0` and the requires blocks that. The prelude already has `X modInt P => X` when `0 <= X < P` (`INT-SYMBOLIC`); these lemmas are the ones that fire when the inner residue is not yet known to lie in range, which is the `transient_hash` mul-of-sum case. There is no general subtraction lemma `(X - (Y mod P)) mod P => (X - Y) mod P`; only the `0 - _` form.

`CONSTRAIN-TO-BOOLEAN-OK` / `ASSERT-OK` / `ASSERT-FAIL` are not trivially true; they are barely quantified. That is a coverage fact, not a false README.

## Moriarty preimages and `preimage-json`

`moriarty_preimages.mjs` drives `output/contract/index.js` through `@midnight-ntwrk/compact-runtime` 0.19.100 and serialises with `proofDataIntoSerializedPreimage` (`moriarty_preimages.mjs:69-73`). That WASM entry is `onchain-runtime-wasm/src/primitives.rs:227-274` at ledger `92e8bdd3`:

- `private_transcript` ← `value_only_field_repr` of each private transcript aligned value (`:239-242`)
- `public_transcript_outputs` ← `Popeq` results only (`:244-248`)
- `public_transcript_inputs` ← `Op::field_repr` of the whole public transcript (`:249-252`)
- `binding_input: 0.into()` (`:258`)
- `communications_commitment: Some((transient_hash([0] ++ encode(input) ++ encode(output)), 0.into()))` (`:253-270`)
- `tagged_serialize` of `ProofPreimage` (`:273-274`)

Scenario order is the ledger-state order the circuits assert: `fundAlice → fundBob → {decide, expire}` from the post-`fundBob` state; `fund → {release, refundAfterTimeout}` from the post-`fund` state (`moriarty_preimages.mjs:112-134`). `expire` / `refundAfterTimeout` are not run after `decide` / `release`. Witnesses are the deterministic 32-byte fills (`0xa1`, `0xb0`, `0xe5`); authorities are `rt.persistentHash` of those secrets (`:50, 97-100, 120-121`). `privateState` is `{role: 'test'}` / `{role: 'buyer'}`; the witness closures ignore it and return the fills.

This is the preimage the *onchain runtime* produces for a circuit call, not the preimage the *ledger* would attach to a `Transaction`. The function itself writes binding input 0 and commitment randomness 0; the ledger overwrites both at assembly (`plan-iter3/m4-contexts.md:119-130`, `moriarty_preimages.mjs:62-63`). `createCircuitContext(..., dummyContractAddress(), COIN_PK, ..., time)` (`moriarty_preimages.mjs:76`) uses the all-zero dummy address and does not pass `parentBlockHash` (the 10th argument, `circuit-context.js:40`). `decide` is only `decision = 1`. Those residuals are written on the manifest (`corpus/moriarty-contexts/manifest.json:5`) and are not silent.

`preimage-json` (`tools/preimage-json/ledger-92e8bdd3/src/main.rs:36-47`) is `tagged_deserialize::<ProofPreimage>` of that binary, then decimal strings via `BigUint::from_bytes_le(&f.as_le_bytes())`. `Fr::as_le_bytes` is `to_bytes_le` and round-trips with `from_le_bytes` (`transient-crypto/src/curve.rs:370-372`). That is the inverse of the oracle's `fr_from_dec` (`zkir-circuit-oracle/.../main.rs:152-160`). Field names match `ProofPreimage` (`transient-crypto/src/proofs.rs:716-732`, tag `"proof-preimage"`). `communications_commitment` is `Option<(String, String)>`, JSON `[c, r]`; the harness reads `cc[0]`, `cc[1]` (`docs/11-tooling-reference.md:42`). `key_location` is decoded and dropped (`moriarty_preimages.mjs:82`). Decoding of the tagged crate type is faithful. Extra JSON elements on a longer commitment array would be ignored by the K harness and rejected by the oracle (`docs/11-tooling-reference.md:47`); the generator does not emit that shape.

The contexts receipt (`evidence/zkir-k-moriarty-contexts-2026-09-06.txt`) is consistent with that construction: seven honest rows `ok/ok/agree/accepted` on both surfaces; `wrong-phase` and `wrong-witness` are `error/error/agree/preprocess-error` with `Failed direct assertion`; `expire` and `refundAfterTimeout` have no `wrong-witness` row because `private_transcript` is empty. `moriarty_contexts.py:117-121` requires the negative error class `assertion`, so a transcript-format failure would be a `FAIL` row.

## Coverage gaps

Symbolic claims exist only for `add`, `mul`, `neg`, `copy`, `cond_select` with a ground bit, `constrain_to_boolean` at `{1} ∪ [2, r)`, `assert` at `{0,1}`, `transient_hash` of two natives, and one `add; impact` observable instance. Missing:

- `constrain_to_boolean` of `0` (off-circuit `asBool(native(0))` is `boolOk(false)`, `#boolCheck` accepts it, `zkir-ops.k:351` and `zkir-vm.k:312`).
- Symbolic `cond_select` bit; `inv`, `not`, `less_than`, `constrain_eq`, `constrain_bits`, `test_eq`, `div_mod_power_of_two`, `reconstitute_field`, `encode`, `output`.
- Transcript instructions, `public_input` / `private_input`, guarded-off registers (`unconstrained`).
- `hash_to_curve`, `persistent_hash`, `keccak256`, `sha256` as uninterpreted applications; `ec_mul` / `ec_mul_generator`; any non-native type.
- Communications commitment (finding 1); `checkedJob` / `wf`; extension surface (`ZKIR-SYMBOLIC` imports `ZKIR-VM` only, `zkir-symbolic.k:34-37`).
- A Moriarty artifact as a `kprove` claim.
- `assert` of a non-boolean (`C = 2`): off-circuit error, gate `holds` via `#nonZero` `[owise]` (`zkir-constraints.k:272-273`) — the known f02 shape, unclaimed.

`make check` does not run claims, drift, contexts, circuit or contract. `check_corpus.py:92-93` silently `continue`s on `version.major != 3` and does not walk `corpus/handmade/` or `corpus/midnight-zkir-2ffe2d1-tests/`.

## Questions for the authors

1. Is the intended symbolic theory that `transientCommit` *is* `poseidonHash` of the cons, or that they are two uninterpreted symbols? Finding 1 assumes the former, because the concrete VM and the crate treat them as one hash.
2. Should a compiler obligation quantify over source executions and pin `witnessSpace`, or is `[[P]](pre)` on a single compiled term the interface Moriarty's spec will actually instantiate?
3. `createCircuitContext` is called without `parentBlockHash`. Do any of the seven circuits read the parent block hash from the kernel, so that the dummy/absent hash is in the public transcript?
4. kprove logs for the claims receipt were under `/tmp/zkir-k-claims-pivsva65` (receipt line 5) and are not in `evidence/`. Is the archived artefact only the twelve-line summary?

## What was checked

Read, not modified: `experiments/zkir-k/semantics/zkir-symbolic.k`, `zkir-hash.k`, `zkir-curves.k`, `zkir-field.k`, `zkir-vm.k` (job, `#seedPi`, `#exec`, `#finish`, `#commCheck`, `#observable`), `zkir-constraints.k` (`eval` of the claimed gates, `commGate`, `witnessSpace`), `zkir-ops.k` (`asBool`), `zkir-static.k` (`usedChips`); `experiments/zkir-k/claims/*.k` and `README.md`; `tools/run_claims.py`, `upstream_drift.py`, `moriarty_preimages.mjs`, `moriarty_contexts.py`, `preimage-json/ledger-92e8bdd3/src/main.rs` and `Cargo.toml`, `check_corpus.py` (skip path), `Makefile`; receipts `evidence/zkir-k-claims-2026-09-06c.txt`, `zkir-k-upstream-drift-2026-09-06.txt`, `zkir-k-moriarty-contexts-2026-09-06.txt`; `plan-iter3/PLAN.md`, `m4-contexts.md`; `docs/16-compilation-target-contract.md`, `docs/11-tooling-reference.md`; `wiki/zkir/zkir-k-definition.md` (CLM-0746 still says uninterpreted hashes are not implemented); `review-2026-09-05/CONSOLIDATED.md` item 34 (the previous `transient_hash` non-termination; not re-reported); `ProofPreimage` at `repos/_build/ledger-92e8bdd3/transient-crypto/src/proofs.rs:716-732` and `Fr::as_le_bytes` at `curve.rs:370-372`; `proof_data_into_serialized_preimage` at `onchain-runtime-wasm/src/primitives.rs:227-274`; compact-runtime `createCircuitContext` at the Nix store path recorded in `corpus/moriarty-contexts/manifest.json`; Moriarty `*.zkir` `do_communications_commitment` flags; `semantics/zkir-symbolic-kompiled/{backend.txt,mainModule.txt,parsed.txt}` for `[concrete]` on the hash and `ecMul` rules.

Did not run `kprove` or `run_claims.py` in this review: `run_claims.py:160` rewrites `claims/spec-compiled-observable.k`, and `kprove` writes a spec-kompiled directory. Claim success is taken from `evidence/zkir-k-claims-2026-09-06c.txt` (12/12, 7.1–9.9 s) together with the rule-level reading above. Did not re-run `moriarty_preimages.mjs` or `moriarty_contexts.py`; the contexts receipt and the `ProofPreimage` construction in `primitives.rs` were compared instead.
