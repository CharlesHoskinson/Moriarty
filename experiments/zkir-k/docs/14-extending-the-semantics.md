# Extending the semantics

A change to the executable K definition of Zero-Knowledge Intermediate Representation (ZKIR) v3 reaches every interpreter that imports the edited module. The four main modules (`ZKIR` in `zkir.k`, `ZKIR-EXT` in `zkir-ext.k`, `ZKIR-CHECK` in `zkir-check.k`, `ZKIR-TEST` in `zkir-test.k`) share the rest of `semantics/`. Paths under `semantics/`, `tools/` and `corpus/` are relative to `experiments/zkir-k/`; `evidence/` is at the repository root. Commands start at the repository root.

## Conventions of the definition

Every production a Python tool constructs carries `[symbol(name)]`. In `zkir-syntax.k` the symbol of an `Instr` is the JavaScript Object Notation (JSON) `op`. An `IrType` takes the Rust `IrType` variant name (`Native`, `Bytes32`, `JubjubPoint`) as its symbol, and `TYPE_SYMBOLS` in `tools/zkir_kast.py` maps the JSON type string (`Scalar<BLS12-381>`, `Bytes<32>`, `Point<Jubjub>`) to it. The tool builds terms with pyk `KApply` on those names; without `symbol(_)`, the Low Level Virtual Machine (LLVM) backend mangles labels from module and sort. List productions also set `terminator-symbol` (`.instrs` on `Instrs`). Value constructors in `zkir-values.k` use a `V` suffix (`nativeV`, `bytes32V`) so they do not collide with the type symbols. There is no concrete ZKIR grammar: `zkir_kast.py` reads the JSON artifact, so a new production needs a pyk mapping in the same change.

Operational helpers are `[function, total]`; a missing arm becomes `#Bottom` and aborts the LLVM interpreter. Cover the domain with pairwise-disjoint `requires` guards, then an `[owise]` fallback that returns an explicit error or a documented dummy. `addV` in `zkir-ops.k` otherwise returns `vErr("Unsupported addition: " +String tn(A) +String " + " +String tn(B))`. `fmod` in `zkir-field.k` requires `P >Int 1` and otherwise returns `0`, a totality convention rather than field mathematics. Default function-rule priority is 50; `[owise]` is 200. Overlapping same-priority rules with different results make the function ill-defined.

`IrSource::preprocess` resolves and converts operands one at a time, so the first failing operand determines the error. K evaluates the arguments of a function application before matching its rules, so `#bin(resolve(A, M), resolve(B, M), "add")` resolves both operands before either is inspected. That is harmless for `add`, because `resolve` only looks up registers. Sequential resolution places the next `resolve` on the right-hand side of the arm that has already matched the previous result, either as a chain of `KItem`s or as a recursive function whose recursion sits only in the success arm. In `zkir-vm.k`, `less_than` is `#lt` through `#lt4` (resolve `a`, `checkBits` on `a`, resolve `b`, `checkBits` on `b`, compare). `reconstitute_field` is `#recon` through `#recon4` and starts with the modulus because `reads(reconstituteField(D, M, _, _)) => M, D, .Operands`. The hash instructions use `resolveNatives` and `#rn`, whose error arms return before the remaining operands are resolved.

```k
rule <k> #exec(lessThan(A, B, N, O)) => #lt(resolve(A, M), B, N, O) ... </k>
     <mem> M </mem> <status> ok() </status>
rule <k> #lt(vErr(S), _, _, _) => #fail(S) ... </k>
rule <k> #lt(vOk(AV), B, N, O) => #lt2(checkBits(AV, N), AV, B, N, O) ... </k>
```

Off-circuit messages follow the crate's `anyhow!` and panic texts closely enough that `ERROR_CLASSES` in `tools/diff_test.py` maps both sides to the same class. Strings are not compared character by character: K prints decimals where Rust prints little-endian hex. A new failure must match an existing class pattern or add a `(name, regex)` pair. Status `panic(msg)` is reserved for process aborts (short transcripts, and the Bytes32 decoder assertions on the base surface); `compare` catches a panic reported as `error(msg)` as a status mismatch before any class is compared.

`zkir-vm.k` emits a constraint and then runs the witness half; `isSpecialEmit` is true only for `impact` and `output`. After the instruction list, `#finish` checks transcripts and the communications commitment, and `#verdicts` fills `<verdicts>` from `verdicts` in `zkir-constraints.k`. When `<status>` is `error` or `panic`, later `#exec` terms disappear while emission continues, so a gate that reads a missing register evaluates to `unknown`.

```k
rule <k> I:Instr => #exec(I) ... </k>
     <constraints> Cs => Cs ListItem(gate(I)) </constraints>
  requires notBool isSpecialEmit(I)
```

The off-circuit function and the `eval` rule stay separate even when they share a helper such as `addV`. `inv` of zero is `vErr` off-circuit and `violated` in-circuit (`#invNonZero`); `less_than` compares at the declared width off-circuit and at `#ltBits(N)` in-circuit. Reuse the witness function as the gate relation only when `*_offcircuit` and `*_incircuit` agree.

`ZKIR-EXT` turns on canonical decoding by rewriting `job` and `genJob` onto themselves while setting `<strictDecode>` from `false` to `true`, with `[priority(30)]`. Those left-hand sides coincide with the base rules at priority 50, so without it the two would race. Specific extension arms (`negV(boolV(B))`, extra `eqDispatch` arms, `#chipsOfInstrs` for `sha512`) win over the base `[owise]` fallbacks because they are more specific, and need no priority. `ZKIR-CHECK` imports `ZKIR-WF` and `ZKIR-EXT-SYNTAX`, so the static checker parses both surfaces.

## Adding an instruction

The files below move together. Skip a row only when the crate has no counterpart.

| Step | Where | What to add |
|---|---|---|
| Syntax | `zkir-syntax.k` (base) or `zkir-ext.k` `ZKIR-EXT-SYNTAX` | `Instr` production with `symbol(op)`, argument order matching `enum Instruction`, identifiers written last |
| Data flow | same module, `reads` and `writes` | operands in `preprocess` resolve order; defined identifiers |
| Static arity | `zkir-syntax.k` `ZKIR-WF`, `#checkArity` | structural checks that hold for every preimage; disjoint `requires` from the `[owise]` `wfOk()` |
| Run-time check | `zkir-vm.k` `#exec`, or the conversion helper the chain calls | the same structural checks `preprocess` performs, because `job` does not run `wf` |
| Off-circuit | `zkir-ops.k` (or a `#...` chain in `zkir-vm.k` / `zkir-ext.k`) | one function per `*_offcircuit`, result `vOk` / `vErr` / `vPanic` |
| VM rule | `zkir-vm.k` or `zkir-ext.k` | `#exec(I)` under `<status> ok()`; write through `#put` / `#put2` |
| Gate | `zkir-constraints.k` or `zkir-ext.k` `eval` | relation over final `<mem>`; chip check; do not fall through to `unsupported` |
| Chip | `usedChips` / `#chipsOfInstrs` / `chipOfType` | the chips the crate's `used_chips` enables |
| Preprocessor | `zkir_kast.py` `instruction` | required JSON fields, `KApply` on the symbol; `EXT_OPS` and `if EXT` for the extension surface |
| Error class | `diff_test.py` `ERROR_CLASSES` | a regex that matches both K and the oracle |
| Handmade program | `corpus/handmade/` via `gen_handmade.py` | at least one successful run on every type the instruction supports |
| Unit check | `unit_values.py` or `unit_hash.py` | a new formula or known-answer; `ZKIR-TEST` does not import `zkir-ext.k` |
| Divergence case | `divergence_tests.py` | only if off-circuit and in-circuit, or K and the oracle, disagree |

If the instruction is `impact`-like or `output`-like, extend `isSpecialEmit` and emit a dedicated `Constraint` rather than `gate(I)`. Every new `Constraint` constructor needs its own `eval` arm: the six-ary `eval` in `zkir-constraints.k` when it needs the outputs or the binding input (`bindGate`, `commGate`, `outputGate`), otherwise the four-ary one (`guardGate`, `piGate`). Only `gate(I)` matches the `unsupported` fallback. The `[owise]` of the six-ary `eval` forwards a constructor with no arm to the four-ary one, where no rule applies, so `verdicts` leaves an unreduced `eval(...)` term inside its `verdict` entry instead of an outcome. An extension opcode is rejected without `--ext`; with `--ext`, `reverse_bytes` is rejected because the crate removed it.

### Worked example: `add`

`add` is the JSON `op`. Each row is the same change as the checklist, already present in the definition.

| Step | Locator |
|---|---|
| Syntax | `zkir-syntax.k`: `add(Operand, Operand, String) [symbol(add)]` |
| `reads` / `writes` | `reads(add(A, B, _)) => A, B, .Operands`; `writes(add(_, _, O)) => O, .Ids` |
| `wf` arity | no `#checkArity` arm; for `add`, `wf` checks only the defined names, the immediate range and single assignment (`#checkReads`, `#wfWrites`) |
| Off-circuit | `zkir-ops.k`, `addV`: `fadd` / `ecAdd` per type, else the unsupported-addition `vErr` |
| VM | `zkir-vm.k`: `#exec(add(A, B, O)) => #put(O, #bin(resolve(A, M), resolve(B, M), "add"))` while `<status>` is `ok()`; `#bin(..., "add") => addV(A, B)` |
| Gate | `zkir-constraints.k`: `eval(gate(add(A, B, O)), ...)` uses `#need`, `#chipsForValues` and `#matches(#bin2(..., "add"), rdId(O, M), "add output")` |
| Chip | native addition: `chipOfType(native())` is `.Set`. Two Jubjub points need `"jubjub"` already present from an input, a `public_input` / `private_input`, or a `hash_to_curve` instruction |
| Preprocessor | `zkir_kast.py`: `case 'add' \| 'mul'`, fields `a`, `b`, `output`, `KApply(op, ...)` |
| Error class | `ERROR_CLASSES` maps `Unsupported` to `unsupported-op` |
| Handmade | `native_bytes.zkir` (native); `curve_jubjub.zkir`, `curve_secp256k1.zkir`, `curve_secp256r1.zkir`, `curve_curve25519.zkir` (points) |
| Unit check | none: no check in `unit_values.py` calls `addV`, `fadd` or `ecAdd` (`ecMul` is checked on all four curves); the instruction is covered by the handmade programs and the differential run |
| Divergence | none: off-circuit and in-circuit dispatch both call `addV` |

```k
rule <k> #exec(add(A, B, O)) => #put(O, #bin(resolve(A, M), resolve(B, M), "add")) ... </k>
     <mem> M </mem> <status> ok() </status>
rule #bin(vOk(A), vOk(B), "add") => addV(A, B)
```

`less_than` is the same checklist with a different split: no `*V` function, sequential `#lt`..`#lt4`, a `#checkArity` arm for `bits >= 255`, the run-time bound in `checkBits` of `zkir-ops.k` (type first, then `Excessive bit bound`) rather than an `#exec` arm, gate padding through `#ltBits`, and divergence cases `f04_less_than_odd_bits` and `k05_less_than_253_bits_keygen` in `corpus/divergence/`.

## Adding a type

A type is an `IrType` constructor, a `Value` constructor, an encoding and a decoder.

| Surface | File | Symbols |
|---|---|---|
| Base syntax | `zkir-syntax.k` | `IrType` production, `encodedLen` |
| Base values | `zkir-values.k` | `Value` with `symbol(*V)`, `typeOf`, `defaultValue`, `typeName`, `encodeValue`, `decodeValue` / `decodeStrict`; `#decField` and `#wrapPoint` for a field or point type |
| Conversions | `zkir-ops.k` | any `as*` helper and the `*V` dispatch arms; for a field type `#wrapField`, `#invField`, `intoBytes32V`, `fromBytes32V` |
| JSON | `zkir_kast.py` `TYPE_SYMBOLS` | serde name to K symbol |
| Runner | `zkir_run.py` | `type_string`, `encode_value` |
| Generators | `zkir_values.py` | `ENCODED_LEN`, `random_encoded`, `small_encoded` |
| Chip | `zkir-constraints.k` | `chipOfType` (the chip `used_chips` enables, or `.Set`) and `isZeroField` |
| Extension | `zkir-ext.k` `ZKIR-EXT-SYNTAX` / `ZKIR-EXT-VALUES`; `EXT_TYPES` or the `Bytes<n>` regex in `ir_type` | extra arms of the same functions |

Several `[owise]` arms return a typed dummy instead of an error, so a missing arm for the new type returns a wrong value rather than failing. `#wrapField(X, _) => native(X)` in `zkir-ops.k` makes `inv` of a new field type produce a `native` result, and `#wrapPoint(_, _) => native(0)` in `zkir-values.k` does the same for a new point type in decoding. `isZeroField(_) => false` in `zkir-constraints.k` makes `inv` of zero of a new field type a `synthErr` through `#matches` instead of `violated` through `#invNonZero`. `chipOfType(_) => .Set` and `#fromCoordsChip(_, _, _) => holds()` let the new type pass every chip check. `#decField(_, T)` rejects every decode of a new field type with `decErr` until its arm exists. Grep `[owise]` in `zkir-ops.k`, `zkir-values.k` and `zkir-constraints.k` before running the harness, which reports these as memory mismatches or wrong verdicts, never as a missing arm.

`Bytes<32>` stays `bytes32()` / `bytes32(_)` so every base Bytes32 rule still applies; `mkBytes` and `asBytesAny` are the length split. `ZKIR-TEST` does not import `zkir-ext.k`, so encode or decode unit checks for `boolV`, `byteV` and `bytesV` cannot run through `unit_values.py` until that import is added. `diff_test.py --ext` checks the extension types. Keep `encodedLen(T)` equal to the length `encodeValue` produces.

A decoder that panics in Rust returns `decPanic`; `#toValue` turns it into `vPanic` and `#put` into status `panic`. Strict decoding applies on the extension surface and to `load_constant` on either surface, which calls `decodeStrict(#immInts(Es), T, true)`. Under strict decoding, `#canonical` in `zkir-values.k` intercepts `decPanic` for `bytes32()` and returns `decErr("Failed to decode as Bytes32")`, because the 2ffe2d1 decoder returns `None` there; every other type's `decPanic` passes through. A new type whose 2ffe2d1 decoder does not panic needs its own `#canonical` arm.

## Adding a chip

Chip names are strings in `<chips>`: `jubjub`, `poseidon`, `sha2_256`, `keccak_256`, `secp256k1`, `p256`, `curve25519`, and on the extension surface `sha2_512`. Each is the spelling of a boolean field of `ZkStdLibArch` that `IrSource::used_chips` (`ir_vm.rs`) sets to `true`. Fields the crate never sets at either pin (`sha3_256`, `blake2b`, `bls12_381`, `base64`, `automaton`) have no K name. `usedChips(Program)` unions `#chipsOfTypes` (input types and `public_input` / `private_input` types via `chipOfType`), `#chipsOfInstrs` (hash instructions) and `#commChip` when the communications-commitment flag is true. `from_bytes32`, `jubjub_scalar_from_native` and native `from_coordinates` add no chip, yet their gates demand one through `#chipFor`: the synthesis errors recorded as finding 13 (`from_bytes32`) and K4 (the other two) in 13-known-divergences.md.

| Step | What to add |
|---|---|
| Name | the `ZkStdLibArch` field `used_chips` sets, as a K string of the same spelling |
| Enable | a `chipOfType` arm and/or a `#chipsOfInstrs` arm more specific than the `[owise]` skip |
| Check | `#chipFor` or `#chipNamed` from `eval` |
| Program | a handmade program that enables the chip the way the crate does |
| Divergence | a case if an instruction needs a chip that `used_chips` does not enable (`f13_chip_gating_from_bytes32`, `k04_jubjub_scalar_from_native_chip`, `k01b_jubjub_from_coordinates_no_chip`) |

The extension's `sha512` arm is the model for an instruction-triggered chip:

```k
rule #chipsOfInstrs((sha512(_, _, _) ; Is)) => SetItem("sha2_512") |Set #chipsOfInstrs(Is)
rule eval(gate(sha512(Al, Xs, O)), M, _, Chips) =>
    #and(#needAll(Xs, M), #and(#chipNamed("sha2_512", Chips),
        #matches(#sha512C(resolveAll(Xs, M), Al), rdId(O, M), "sha512 output")))
```

`#sha512C` differs from the off-circuit `#sha512V` only in decoding the alignment through `alignedBytesCircuit` (`zkir-constraints.k`), which rejects `option` and `compress` segments at synthesis. A new byte-hash gate must use the circuit decoder, not `alignedBytes`.

## Pitfalls

**Duplicate function definitions across modules.** K merges every rule for a function. The extension adds arms for `reads`, `writes`, `encodedLen`, `typeOf`, `defaultValue`, `typeName`, `encodeValue`, `decodeValue`, `negV`, `eqDispatch`, `#eqSupported`, `#chipsOfInstrs` and `eval`. An `[owise]` in the imported module steals a new arm that is no more specific than that fallback. Prefer a constructor pattern (`sha512(_, _, _)`, `boolV(_)`) over a catch-all, and test both surfaces.

**`[owise]` overlap.** `#checkArity` for `div_mod_power_of_two` has two defect arms with disjoint `requires`; `#nonResidue` stops at `Z >=Int P`; `#boolFold` and `#concat` keep typed cons arms above a single `[owise]` error. Two such instances existed and were removed. `#selBit` in `zkir-constraints.k` had two `[owise]` arms that both matched a bit other than 0 or 1 with two `vOk` operands, and the wildcard arm now carries `requires X =/=Int 0 andBool X =/=Int 1`. The extension's `#eqSupported` arm for two `bytesV` overlapped the base arm guarded by `notBool sameType(A, B)` when the lengths differed, and the extension arm now requires equal lengths. Neither was visible in a run, since both arms of each pair gave `synthErr`. Use exactly one `[owise]` arm per function and no unguarded extension arm over a guarded base arm.

**Parse ambiguities.** The preprocessor must match `IrSource::load`: `Bytes<32>` is classified as `Bytes32` before `BytesN`; `reverse_bytes` is rejected only with `--ext`; `guard` may be absent or JSON `null`; unknown object members are ignored; `version` components are integers; `inputs`, `outputs` and `instructions` must be arrays; immediates are even-length little-endian hex without whitespace. A new type string or `op` that overlaps an existing form needs an explicit branch.

**Catch-all rules that hide unsupported cases.** `eval(gate(I), _, _, _)` otherwise rewrites to `unsupported("no in-circuit relation modelled for this instruction: " +String #opName(I))`, and `#opName` itself is only the `[owise]` `"instruction"`. That is the intended report for a forgotten `eval` arm of an instruction wrapped in `gate(_)`; do not turn it back into `holds()`. `#chipsOfInstrs((_ ; Is))` otherwise skips the instruction, so a new hash whose gate demands a chip reports `synthErr("chip not initialised: <name>")` on every run until its arm exists. `diff_test.py` lists such runs on its `oracle 2` line without failing, so read that line.

**Stuck configurations reported as success.** `Runner.run` in `tools/zkir_run.py` treats a non-empty `<k>` as `stuck`, or `depth-exhausted` when `--depth` was set, regardless of `<status>`, and differential comparison fails those statuses. Before the runner inspected `<k>`, `div_mod_power_of_two` with one output had no `#exec` rule: it stopped with `#exec(...)` in `<k>` and `<status>` still `ok()`, and was reported as a success. The rule `#exec(divModPowerOfTwo(_, _, Os))` with `lenIds(Os) =/=Int 2` in `zkir-vm.k` now fails it. After adding an `#exec` chain, run a program that should finish and confirm `<k>` is `.K`.

**Symbolic unfolding of hashes.** The defining rules of `poseidonHash`, `absorbAll`, `permute` and `sbox` in `zkir-hash.k` carry no `[concrete]` restriction, so the Haskell backend unfolds them on symbolic arguments. A pinned claim about `add` finishes; a claim about `transient_hash` unfolds Poseidon and `modInt` and does not terminate. To prove through a hash, restrict those rules with `[concrete]` and give the function a symbolic treatment (an `smtlib` declaration or `[simplification]` lemmas). None of this is implemented.

## Re-running the check layers

Rebuild every interpreter that closes over the edited file. pyk reads `compiled.json`, so each `kompile` needs `--emit-json`.

```
kompile experiments/zkir-k/semantics/zkir-check.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-check-kompiled
kompile experiments/zkir-k/semantics/zkir-test.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-test-kompiled
kompile experiments/zkir-k/semantics/zkir.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-kompiled
kompile experiments/zkir-k/semantics/zkir-ext.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-ext-kompiled
```

A new `*V` formula or encoding belongs in the unit tools; a new program in the differential harness; a new disagreement in the divergence suite. `--checked` on `zkir_run.py` exercises `checkedJob` inside `zkir-kompiled` or `zkir-ext-kompiled` and does not use `zkir-check-kompiled`.

```
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --seed 2026 --attempts 8
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --ext --seed 2026 --attempts 8
uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py
```

A focused differential check uses `--only native_identity` on the base surface or `--only bool_identity` on the extension surface. `check_corpus.py` walks only the directories in its `CORPORA` table (`corpus/ledger9-92e8bdd3-tests`, `corpus/midnight-zkir-2ffe2d1-precompiles`, `corpus/handmade-negative`, and the compiled `output/zkir` of `experiments/moriarty-compact-escrow` and `experiments/moriarty-core-swap`) and compares each file against `EXPECTED` (default `wfOk`). `diff_test.py`, whose `CORPORA` includes `corpus/handmade`, covers a new handmade positive program; `check_corpus.py` covers it only if its `CORPORA` is extended. A new handmade negative goes in `corpus/handmade-negative/` with an `EXPECTED` entry. `divergence_tests.py` regenerates `corpus/divergence/` from its `CASES` table, byte-identical while the table is unchanged. It takes no arguments, so edit the module constant `OUT` at the top of the tool if the corpus must stay untouched (see 12-oracles-and-differential-testing.md).

Replace the receipts under `evidence/` when the totals change: `zkir-k-unit-values-2026-09-05c.txt` (43 of 43), `zkir-k-unit-hash-2026-09-05c.txt` (18 of 18), `zkir-k-milestone2-corpus-check-2026-09-05c.txt` (63 programs as expected: 43 + 6 + 7 from the corpus and 7 compiled outputs), `zkir-k-differential-92e8bdd3-2026-09-05c.txt` (358 comparisons), `zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt` (418 comparisons) and `zkir-k-divergence-tests-2026-09-05c.txt` (20 of 20). Rebuild before recording; stale kompiled directories behind a receipt are a failed check. How to read the totals is in 12-oracles-and-differential-testing.md and 15-design-rationale-and-limits.md.
