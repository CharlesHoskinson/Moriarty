# Audit of experiments/zkir-k/docs/06-configuration-and-run-lifecycle.md (formal methods expert)

Verdict: REVISE
(one major finding: the missing-commitment seeding path is described as emitting fewer gates than it does; the rest is accurate and the traced example is exactly what the rules produce)

Checks performed:
- Compared the configuration table (20 cells: sort, initial value, role) with the `configuration` block of `experiments/zkir-k/semantics/zkir-vm.k` lines 63-84 and the `Status`, `Skip`, `Need`, `Preimage`, `CommOpt`, `Job` sorts (lines 35-42, 54-55, 91-94).
- Read every rule of `ZKIR-VM` (zkir-vm.k lines 95-548) and checked each described rule, helper, guard, message and outcome: `genJob`, `checkedJob`, `#wfGate`, the discarded `job` under `error(_)`, the `job` rule, `#loadInputs`, `#put`/`#put2`/`#fail`/`#panicNow`, the five `#seedPi` rules, instruction sequencing, `isSpecialEmit`, `impact`/`output` emission, `resolve`, `resolveNatives`/`#rn`, `#lt`..`#lt4`, `#recon`..`#recon4`, `#output`/`#output1`, `#input` (all six branches), `#decodeSlice`, `#impact`/`#impactPush`/`#impactOne`/`#impactCheck`, `#finish` (all seven rules), `#commCheck`, `#verdicts`.
- Checked the gate constructors (`gate`, `piGate`, `guardGate`, `bindGate`, `commGate`, `outputGate`) and `usedChips`, `#encInputs`, `#encodeAll`, `verdicts`, `eval(commGate ...)` in `experiments/zkir-k/semantics/zkir-constraints.k` lines 34-43, 46-87, 88-121.
- Checked `transientCommit` in `zkir-hash.k` line 74-75, `decodeStrict` in `zkir-values.k` line 231, `defaultValue` line 57, `addV` in `zkir-ops.k` line 36, the instruction constructors, `Operands`, `IrType native()` and module `ZKIR-WF`/`wf` in `zkir-syntax.k` (lines 20-31, 65-66, 93-119, 242-253), and the two `priority(30)` rules in `zkir-ext.k` lines 168-169.
- Checked `Runner.run`, `preimage_term`, `fr`, `need` and `main` in `experiments/zkir-k/tools/zkir_run.py` (lines 61-82, 272-283, 298-352, 358-382) and `build_preimage` in `tools/diff_test.py` lines 104-134.
- Confirmed there is no `strict` attribute in zkir-vm.k (grep), that the chapter has one `#` title, no em-dash, no placeholder or drafting language, and 1390 words outside tables and code blocks.
- Built the traced program and preimage as JSON and ran `uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py` from the repository root: plain run, `--depth 3`, `--gen` with an empty public transcript, a plain run with an empty public transcript, a `do_communications_commitment: true` variant with no commitment pair, and a duplicate-assignment variant with and without `--checked`. Program and preimage files are reproduced under Findings and Coverage so the runs can be repeated.

## Findings

### F1 major "Entry points and initialization", sentence "A missing commitment instead preserves only the binding input and raises `Expected communications commitment`."
Claim: with `<doComm> true` and `noComm()`, seeding keeps only the binding input and fails; read together with the previous sentence ("it also places the supplied commitment at position one and emits `commGate(1, Ins, Rand)`"), the word "instead" tells the reader that no `commGate` is emitted on this path.
Evidence: `zkir-vm.k` lines 150-153:
```k
  rule <k> #seedPi(Ins) => #fail("Expected communications commitment") ... </k> <pre> Pre </pre> <doComm> true </doComm> <status> ok() </status>
       <pi> _ => ListItem(#preBinding(Pre)) </pi> <piIdx> _ => 2 </piIdx>
       <constraints> Cs => Cs ListItem(bindGate(0)) ListItem(commGate(1, Ins, 0)) </constraints>
    requires #preComm(Pre) ==K noComm()
```
The rule emits `bindGate(0)` and `commGate(1, Ins, 0)` (opening randomness 0) and sets `<piIdx>` to 2, so the constraint side is identical to the successful path except for the randomness. Confirmed by running a one-input program with `"do_communications_commitment": true` on `{"inputs":["2"],"binding_input":"9"}`: status `error` "Expected communications commitment", `pis` `["9"]`, `constraints` 4, and the second verdict is `unknown "public input vector incomplete"` on `commGate ( 1 , typedId ( "%x" , native ( ) ) , .TypedIds , 0 )`. A reader who takes the chapter literally expects one gate and no `commGate` verdict.
Fix: replace the sentence with: "A missing commitment places only the binding input in `<pi>`, still emits `bindGate(0)` and `commGate(1, Ins, 0)` (opening randomness zero), sets `<piIdx>` to two, and raises `Expected communications commitment`. The commitment gate then evaluates to `unknown("public input vector incomplete")` because `<pi>` has one element."

### F2 minor "Completion and generation", formula `transientCommit(rawInputs ++ encodedOutputs, opening)`
Claim: the commitment check hashes `rawInputs ++ encodedOutputs`.
Evidence: `rawInputs` and `encodedOutputs` are not names in the K files. The rule (`zkir-vm.k` lines 265-268) is `transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)`; `#encodeAll` is defined in `zkir-constraints.k` lines 117-120 and `#preInputs` in `zkir-vm.k` line 111-112. The common brief forbids invented names.
Fix: write the formula as `transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)` and say in prose that `#preInputs(Pre)` is the raw input list of the preimage and `#encodeAll(Os)` (`zkir-constraints.k`) is the concatenated encoding of the output values; drop the `++` remark or keep it only as an explanation of K list juxtaposition.

### F3 minor "Entry points and initialization" ("when both flags are supplied") and "Runner statuses" ("a supplied depth bound")
Claim: the runner has flags selecting generation, checked execution and a depth bound, but none is named.
Evidence: `tools/zkir_run.py` lines 363-366 define `--ext`, `--checked`, `--gen` and `--depth N`; the chapter never names `--gen`, `--checked` or `--depth`, so a reader of this chapter cannot reproduce a generation run, a checked run or a `depth-exhausted` result without going to 11-tooling-reference.md.
Fix: name the flags once, for example "`Runner.run` (invoked with `--gen` and `--checked` from the command line) selects generation before checked execution" and "with a depth bound (`--depth N`)".

### F4 minor "Runner statuses", sentence "`main` prints the result and returns zero even for reported run failures, so callers inspect `status`, not just the process exit code."
Claim: the process exit code is zero for reported failures; the sentence leaves the impression that the exit code carries no information.
Evidence: `tools/zkir_run.py` lines 371-380: a `zkir_kast.ZkirFormatError` (program not accepted by the loader) and a `PreimageError` (a preimage integer outside `[0, r)`, raised by `fr`) print a message to stderr and return 2 with no JSON at all. The common brief lists this ("a format error exits 2").
Fix: add: "Two failures happen before any run and exit with code 2 and no JSON: a program the loader rejects (`format error: ...`) and a preimage integer that is not a canonical field element (`preimage error: ...`, from `fr`)."

### F5 minor "Entry points and initialization", sentence "The following job is discarded, so input loading and gate emission never begin."
Claim: correct, but the reader is not told what the run then contains, and the contrast with a witness failure (which still produces verdicts) is the point of the paragraph on failure later in the chapter.
Evidence: `zkir-vm.k` line 103 `rule <k> job(_, _) => .K ... </k> <status> error(_) </status>` touches no other cell, so `<pre>`, `<outTypes>`, `<doComm>`, `<chips>`, `<constraints>` and `<verdicts>` keep their initial values. Confirmed with `--checked` on a program assigning `%y` twice: status `error` "well-formedness: reassignment of %y", `constraints` 0, `verdicts` 0, `memory` `{}`, `pis` `[]`.
Fix: append: "`#verdicts` is never scheduled either, so `<constraints>` and `<verdicts>` stay empty and the runner reports `constraints` 0 and `verdicts` 0; every other cell keeps its initial value."

### F6 minor "Runner statuses", sentence "The runner reports the residual term in `k_cell` and a shortened version in `error`."
Claim: correct, but no example of the residual term is shown, and the brief asks for "what a stuck configuration looks like".
Evidence: running the chapter's traced program with `--depth 3` gives status `depth-exhausted` and `error` equal to `#loadInputs ( ListItem ( 2 ) , .TypedIds , 1 ) ~> #seedPi ( typedId ( "%x" , native ( ) ) , .TypedIds ) ~> add ( var ( "%x" ) , imm ( 3 ) , "%y" ) ; impact ( imm ( 1 ) , var ( "%y" ) , .Operands ) ; output ( var ( "%y" ) , .Operands ) ; .Instrs ~> #verdicts ~> .K`, with `memory` already holding `%x` and `constraints` 0. `error` is the pretty-printed `<k>` cut to 300 characters (zkir_run.py line 316); `k_cell` is the full text (line 334).
Fix: show this `error` value (or a shorter one) as the example of a residual computation, and state that `error` is truncated to 300 characters while `k_cell` is complete.

## Coverage
- The configuration in `zkir-vm.k`: every cell, sort, initial value, role (table); `<k>` contents during a run: covered (table matches lines 63-84; `<k>` narrative confirmed by the `--depth 3` run).
- Entry points job, checkedJob, genJob and the Job sort; `#wfGate`; status gating; after error or panic (witness cells freeze, gates still emitted): covered (see F1 and F5 for the two seeding and static-check details to add).
- Lifecycle: `#loadInputs`/`#put`, `#seedPi` with bindGate and commGate, sequencing (gate emission then witness half), `resolveNatives` and sequential helpers, transcript reads, `#finish`, `#verdicts`: covered; every statement checked against the rules, the only inaccuracy is F1.
- Generation mode: `<genMode>`, `<needs>`, harness use: covered (`--gen` run on the traced program with an empty public transcript gives `needs [['pubIn', 5]]`, status `ok`, as the chapter describes; `build_preimage` description matches diff_test.py lines 104-134).
- Run statuses ok, error, panic, stuck, depth-exhausted; stuck configurations: covered (table matches zkir_run.py lines 311-324; F3, F4 and F6 add the flag names, the exit-2 cases and a concrete residual term).
- Traced three-instruction example: covered and confirmed. Program (JSON) `{"version":{"major":3,"minor":0},"inputs":[{"name":"%x","type":"Scalar<BLS12-381>"}],"outputs":["Scalar<BLS12-381>"],"do_communications_commitment":false,"instructions":[{"op":"add","a":"%x","b":"0x03","output":"%y"},{"op":"impact","guard":"0x01","inputs":["%y"]},{"op":"output","vals":["%y"]}]}` with the chapter's preimage gives status `ok`, memory `%x` 2 and `%y` 5, `pis ["9","5"]`, `pi_skips [null]`, `cursors [1,0,0]`, `constraints` 5, `verdicts` 5 all `holds` in the order bindGate, gate(add), guardGate, piGate, outputGate, `outputs ["Native:5"]`, exactly the final row of the table. Intermediate rows re-derived from the rules by hand (`addV(native(2), native(3))` = `native(5)`; `#impactCheck(0, 1)` compares `#prePubIn(Pre)[0]` = 5 with `Pi[1]` = 5; `#finish` sees cursors (1,0,0) against transcript sizes (1,0,0)) and agree with the chapter. The chapter's remark that earlier gates observe later values was also confirmed: the same program with `%y` assigned twice runs raw with status `ok` and the first `add` gate `violated "add output"`.
