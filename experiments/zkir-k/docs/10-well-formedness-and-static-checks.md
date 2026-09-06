# Well-formedness and static checks

Whether a program is well formed holds or fails for every preimage; whether a run succeeds depends on the preimage. This chapter covers the first question: the predicate `wf` of module ZKIR-WF (`zkir-syntax.k`), the tools that evaluate it, and the boundary between what the static check rejects, what the crate rejects, and what stays a run-time check in the virtual machine (VM, `zkir-vm.k`).

## The predicate `wf`

`wf` is a total function from `Program` to `WfResult`, which is `wfOk()` or `wfError(String)`. It reports the first failure it meets and never inspects a value, a preimage or the type of a register. A program built by `tools/zkir_kast.py` has already passed the format checks that mirror `IrSource::load` (see 03-program-model.md). The checks run in this order:

| Order | Predicate | Decided by | Message on failure |
|---|---|---|---|
| 1 | The minor version is 0 | `wf` | `unsupported minor version N` |
| 2 | Input names are pairwise distinct | `#wfInputs` | `duplicate input %x` |
| 3 | Every operand in `reads(I)`, taken in order, is a variable defined by an input or an earlier instruction, or an immediate in [0, r), r the BLS12-381 scalar modulus | `#checkReads` | `undefined variable %x` or `immediate out of field range: V`, for the first operand that fails |
| 4 | Structural arities and bit bounds hold (next table) | `#checkArity` | see next table |
| 5 | No identifier is written twice: not an input name, not an earlier output, not twice by one instruction | `#wfWrites` | `reassignment of %x` |

Checks 3 to 5 are applied instruction by instruction by `#wfInstrs`, which carries the set `Defined` of names seen so far. Within one instruction the order is reads, then arity, then writes:

```k
rule #wfInstrs((I ; Is), Defined, N) => #wfAfterReads(#checkReads(reads(I), Defined), I, Is, Defined, N)
rule #wfAfterArity(wfOk(), I, Is, Defined, N) => #wfWrites(writes(I), Is, Defined, N)
```

Check 3 is one walk over the operand list, so an out-of-range immediate that precedes an undefined variable is the one reported. `add %x, %x -> %x` with `%x` undefined fails as `undefined variable %x`, while `copy %a -> %a` with `%a` an input fails as `reassignment of %a`. `N` is `lenIrTypes(Outs)`, the number of declared return types, used only by the `output` arity check.

### `reads` and `writes`

Two total functions in ZKIR-SYNTAX describe the data flow of every instruction without executing it. `reads(I)` returns the operands the instruction resolves off-circuit, in the order `IrSource::preprocess` resolves them; `writes(I)` returns the identifiers it defines.

```k
rule reads(condSelect(B, A, C, _))           => B, A, C, .Operands
rule reads(reconstituteField(D, M, _, _))    => M, D, .Operands
rule reads(publicInput(noGuard(), _, _))     => .Operands
rule reads(publicInput(guard(G), _, _))      => G, .Operands
rule writes(intoCoordinates(_, X, Y))        => X, Y, .Ids
rule writes(divModPowerOfTwo(_, _, Os))      => Os
```

Instructions that only constrain (`assert`, `constrain_bits`, `constrain_eq`, `constrain_to_boolean`, `impact`, `output`) write nothing. Module ZKIR-EXT-SYNTAX (`zkir-ext.k`) adds the extension types, instruction constructors, `encodedLen`, `reads` and `writes`, but no `#checkArity` rules, so the extension instructions take the `owise` rule (see 09-extension-surface.md).

### `#checkArity`

`#checkArity(I, N)` has six error rules and an `owise` rule that returns `wfOk()`. The bounds come from the constants `#frBits` (255) and `#frBytesStored` (31) in `zkir-syntax.k`, which are `FR_BITS` and `FR_BYTES_STORED` of the crate.

| Instruction | Predicate | Message on failure |
|---|---|---|
| `output` | number of operands equals `N` | `output: signature declares N return values but instruction has M` |
| `div_mod_power_of_two` | exactly two output identifiers | `div_mod_power_of_two requires exactly 2 outputs` |
| `div_mod_power_of_two` | bits at most 248 (tested only when the arity holds) | `div_mod_power_of_two: excessive bit count` |
| `reconstitute_field` | bits at most 248 | `reconstitute_field: excessive bit count` |
| `constrain_bits` | bits below 255 | `constrain_bits: excessive bit bound` |
| `less_than` | bits below 255 | `less_than: excessive bit bound` |

The arity of `encode` is not a static check because it depends on the run-time type of the encoded value. The VM reports `Unexpected output length of encode instruction: T` (`#encode` in `zkir-vm.k`, `T` the type name), and the gate evaluates to `synthErr("Unexpected output length of encode instruction")` (`#encMatch` in `zkir-constraints.k`).

## Evaluating the check

### ZKIR-CHECK and the `check` command

`zkir-check.k` imports ZKIR-WF and ZKIR-EXT-SYNTAX, holds one `<k>` cell over a `Program`, and has the single rule `<k> P:Program => wf(P) </k>`. The `check` command of `tools/zkir_kast.py` builds the term from a `.zkir` file, runs the compiled definition (`semantics/zkir-check-kompiled` by default, `--definition DIR` to override) and prints the result through `wf_result`:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py check experiments/zkir-k/corpus/handmade-negative/reassignment.zkir
wfError ( "reassignment of %b" )
```

The exit status is 0 for both `wfOk` and `wfError`; a program the preprocessor rejects prints `format error: ...` on standard error and exits with 2. With `--ext` the preprocessor accepts the extension surface and the same compiled definition evaluates `wf` on it:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py check --ext experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/test_bool_gates.zkir
wfOk
```

Without `--ext` the same file stops in the preprocessor with `format error: unknown IR type 'Bool'`.

Two `wf` failures cannot be produced from a file, because the preprocessor rejects the same programs first: a minor version other than 0 (`format error: unhandled version: 3.1`) and an immediate at or above r. The rules keep `wf` total on every `Program` term.

### The corpus check

`tools/check_corpus.py` runs the same evaluation over five directories: `corpus/ledger9-92e8bdd3-tests`, `corpus/midnight-zkir-2ffe2d1-precompiles`, `corpus/handmade-negative`, and the compiled circuits in `experiments/moriarty-compact-escrow/output/zkir` and `experiments/moriarty-core-swap/output/zkir`. Programs whose `version.major` is not 3 are skipped. Every file is loaded on the base surface, so `corpus/midnight-zkir-2ffe2d1-tests` is not among its corpora. The expectations live in the script's `EXPECTED` table, keyed by file name: `wfOk` (the default for an unlisted file), `format` (any format error), or `wfError:<substring>`. The corpus manifests (`manifest.json`) record provenance and test preimages, not expectations.

```
uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py
```

The last line is the summary, `63 programs, 63 as expected, 0 unexpected, 8.0s` (the time varies); the exit status is 0 only when nothing is unexpected. The receipt is `evidence/zkir-k-milestone2-corpus-check-2026-09-05c.txt`. The 63 programs are 43 ledger tests, 6 precompiles, 7 escrow and swap circuits and the 7 negatives. The five ledger tests listed in `EXPECTED` are the three `test_invalid_operand_*` format errors, `output_arity_mismatch.zkir` (`wfError`) and `output_operand_type_mismatch.zkir` (`wfOk`: the run-time type of an output operand is dynamic).

## The negative programs

Every file in `corpus/handmade-negative/` fails exactly one predicate. The last column is the output of the `check` command on the file; the 74-digit immediate is shortened.

| File | Content | Predicate that fails | Output of `check` |
|---|---|---|---|
| `wrong_version.zkir` | version 3.1, no instructions | version (rejected before K) | `format error: unhandled version: 3.1` (exit 2) |
| `duplicate_input.zkir` | inputs `%a: Scalar<BLS12-381>` and `%a: Bytes<32>` | distinct inputs | `wfError ( "duplicate input %a" )` |
| `undefined_variable.zkir` | `add %a, %c -> %d` with only `%a` declared | definition before use | `wfError ( "undefined variable %c" )` |
| `immediate_out_of_range.zkir` | `assert` on a 37-byte hex immediate | immediate range (rejected before K) | `format error: immediate '0x0100...ed73' out of range for field element` (exit 2) |
| `divmod_outputs.zkir` | `div_mod_power_of_two %a, 8 -> [%q]` | exactly two outputs | `wfError ( "div_mod_power_of_two requires exactly 2 outputs" )` |
| `excessive_bits.zkir` | `constrain_bits %a, 255` | bits below 255 | `wfError ( "constrain_bits: excessive bit bound" )` |
| `reassignment.zkir` | `copy %a -> %b` then `add %a, %b -> %b` | single assignment | `wfError ( "reassignment of %b" )` |

## The static check against the crate

The crate has no static pass. `IrSource::load` (`ir.rs`) accepts version 3.0 and lets serde deserialise the fields, so operand format and immediate range are load-time errors; every other failure occurs inside `preprocess` (`ir_vm.rs`) while the instructions execute, or inside `circuit` at key generation. `job(P, Pre)` executes every program `IrSource::load` accepts and reproduces the run-time checks of `preprocess` with the crate's messages; `checkedJob(P, Pre)` evaluates `wf(P)` first and on failure sets `<status>` to `error("well-formedness: " +String S)` and skips the run, so it accepts strictly fewer programs:

```k
rule <k> checkedJob(P, Pre) => #wfGate(wf(P)) ~> job(P, Pre) ... </k>
rule <k> #wfGate(wfError(S)) => .K ... </k> <status> ok() => error("well-formedness: " +String S) </status>
rule <k> job(_, _) => .K ... </k> <status> error(_) </status>
```

| Predicate | `checkedJob` | `job` and `preprocess` |
|---|---|---|
| Minor version 0 | not reached | load error `Unhandled version: 3.1` |
| Immediate in [0, r) | not reached | load error `Out of range for field element` |
| Definition before use | `well-formedness: undefined variable %c` | run-time error `variable not found: Identifier("%c")` (`resolve`, `zkir-vm.k`) |
| Distinct inputs | `well-formedness: duplicate input %a` | accepted; the second value overwrites the first in the memory map |
| Single assignment | `well-formedness: reassignment of %b` | accepted; the later write overwrites (`#put`, `#put2`, `#bindEncoded`, `zkir-vm.k`) |
| `output` arity | `well-formedness: output: signature declares ...` | run-time error `Output: signature declares 1 return values but instruction has 2` |
| `div_mod_power_of_two` outputs | `well-formedness: div_mod_power_of_two requires exactly 2 outputs` | run-time error `DivModPowerOfTwo requires exactly 2 outputs` |
| bits at most 248 (`div_mod_power_of_two`, `reconstitute_field`) | `well-formedness: ...: excessive bit count` | run-time error `Excessive bit count` |
| bits below 255 (`constrain_bits`, `less_than`) | `well-formedness: ...: excessive bit bound` | run-time error `Excessive bit bound` (`checkBits`, `zkir-ops.k`) |

The columns differ in more than wording: a run-time error is raised only when execution reaches the instruction with status `ok()`. The negative programs ship without preimages; a one-input preimage in a file `one.json` shows the difference (keys in 06-configuration-and-run-lifecycle.md, printed fields in 11-tooling-reference.md):

```
{"inputs":[1],"binding_input":0,"private_transcript":[],"public_transcript_inputs":[],"public_transcript_outputs":[]}
```

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py experiments/zkir-k/corpus/handmade-negative/undefined_variable.zkir one.json
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py experiments/zkir-k/corpus/handmade-negative/undefined_variable.zkir one.json --checked
```

The first run ends with `"status": "error"`, `"error": "variable not found: Identifier(\"%c\")"`, `%a` in `memory`, `"constraints": 2` and the `add` gate at `unknown`. The second ends with `"error": "well-formedness: undefined variable %c"`, an empty `memory` and `"constraints": 0`. The differential harness of 12-oracles-and-differential-testing.md therefore compares the oracle with `job`.

Checks that need values remain in the VM in both modes: operand types, boolean guards, bit-bound failures on actual values (`Bit bound failed: V is not N-bit`), the `encode` arity, the run-time type of every `output` operand, transcript lengths, the `impact` comparison and the communications commitment. Bounds that only key generation enforces, such as `less_than` at 253 or 254 bits, are gate outcomes rather than errors (see 08-constraints-and-verdicts.md and 13-known-divergences.md).

## Why single assignment matters

After the last instruction the VM evaluates every emitted gate against the final contents of `<mem>` (`#verdicts` in `zkir-vm.k`, `verdicts` in `zkir-constraints.k`). A gate names registers, not values, so a verdict states whether the relation holds between the values those registers finally have. `job` accepts a program that reassigns a register: a later `#put` overwrites the earlier value. For such a program a verdict says nothing about the value the instruction produced when it ran: a `holds` on a gate whose output register was later overwritten only means the final value satisfies the relation. Single assignment is the condition under which evaluation over the final memory coincides with evaluation at each instruction; `#wfWrites` enforces that condition, no more and no less. It does not inspect values, so writing the same value twice is rejected as well.

`reassignment.zkir` shows the effect. Under `job` with input `%a = 1`, `copy` sets `%b` to 1, `add` overwrites it with 2, and the run ends with status `ok`. The verdicts are evaluated with `%b = 2`: the `copy` gate requires `%b = %a`, the `add` gate requires `%b = %a + %b`, and both report `violated`. The crate agrees: `circuit` checks every cell it inserts against the final witness memory (`mem_insert` in `ir_vm.rs`), so the in-circuit `copy` inserts 1 for `%b` against a witness value of 2 and synthesis fails with a misalignment error. A `job` run can thus end with status `ok` and violated verdicts at once; `checkedJob` rejects the program before any gate exists. For programs that pass `wf`, every verdict of 08-constraints-and-verdicts.md is a statement about the instruction that emitted the gate.
