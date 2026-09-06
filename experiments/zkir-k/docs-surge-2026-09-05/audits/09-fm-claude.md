# Audit of experiments/zkir-k/docs/09-extension-surface.md (formal methods expert)

Verdict: REVISE
(one major finding on the outcome description of the extension gates; the
rest are minor precision fixes. No blocking finding: every number, name,
command and rule description checked is true of the files as they are now.)

Checks performed:
- Read briefs AUDIT-COMMON.md, AUDIT-fm.md, COMMON.md, 09.md, then the chapter in full.
- Compared every K symbol, rule, guard, priority and message named in the chapter with experiments/zkir-k/semantics/zkir-ext.k (all 276 lines) and zkir-sha512-constants.k.
- Compared the inherited rules the chapter cites with zkir-vm.k (job/genJob/checkedJob lines 40-42, 95-105; #loadInputs 132-133; sequencing 172; #put 216-222; not 281-284; resolveBool/#toBool 200-204), zkir-values.k (bytes32 decoding 160-167; decodeStrict/#canonical 231-238), zkir-ops.k (negV 61-73; eqDispatch/#testEq 105-125; sameType 21-22; asNative 26-28; alignedBytes 236-255; asBool 350-354), zkir-constraints.k (usedChips/#ioTypes/#chipsOfInstrs/chipOfType 46-85; #need 150-153; #and 160-162; #chipFor 164-166; #boolean 178-181; #matches 192-198; #eqSupported 241-247; testEq/condSelect gates 249-254; #chipNamed/#needAll 446-451) and zkir-syntax.k (ZKIR-WF 242-307, reverseBytes 101).
- Compared the Rust claims with repos/_build/midnight-zkir-2ffe2d1/zkir/src: ir.rs Instruction variants (Reverse 645, Slice 662, Nth 733, Concat 749, LoadConstant 770, Sha512 848, Not 962, And 974, Or 986, Xor 998; no ReverseBytes), ir_types.rs (MAX_BYTES_LEN 45, IrType 56-72), ir_vm.rs (resolve_operand_bool 265-276, resolve_bool_list 278-293, Not/And/Or/Xor arms 364-380, error messages 282-290, 673-735, TestEq arm, used_chips 1461-1512), ir_instructions/encode.rs (decode_bytes 144, decode_offcircuit 170, re-encoding 232-242), eq.rs (test_eq_offcircuit 44-50, test_eq_incircuit 93-107), neg.rs (Bool arms 50, 101-103), select.rs (select_offcircuit 45-54, in-circuit support list 56-70); and with repos/_build/ledger-92e8bdd3/zkir-v3/src (ir.rs ReverseBytes 615, ir_types.rs Bytes32 42-44).
- Confirmed the full commit hash with `git -C repos/midnightntwrk/midnight-zkir rev-parse 2ffe2d1`.
- Compared the tooling claims with experiments/zkir-k/tools/zkir_kast.py (TYPE_SYMBOLS 35-49, EXT/EXT_TYPES/EXT_OPS 59-61, ir_type 73-84, immediate 90-109, u32 149-152, instruction 199 and its ext arms 251-279, load_program 367, main 393-410), zkir_run.py (default_ext_definition 43-44, Runner 294-296, main 358-372) and diff_test.py (ORACLE/ORACLE_EXT 37-38, --ext handling 171-181, summary 265-272).
- Counted corpus programs (61, 6, 9) and confirmed all eleven corpus files named in the instruction entries exist and use the instruction they are cited for.
- Checked the receipt evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05b.txt: summary line and row counts (418 PASS, 0 FAIL, 50 ok/ok, 364 error/error, 4 format rows, oracle 2 line).
- Checked wiki/contradictions.md line 50 (extension test_eq row).
- Ran the three commands of "Running and testing" from the repository root with the shown environment variables (all exit 0); ran the first command without `--definition` to confirm the stated need for the extension definition (fails with KeyError 'Bool').
- Ran test_sha512_proof.zkir with input [7] under `--ext` and compared the 64-byte output with Python hashlib.sha512(b'\x07'): equal.
- Ran a scratch program whose `and` gate reads an absent register to establish finding F1.
- Checked hard rules: one `#` title, no em-dash, no placeholder or TODO, no mention of drafting; 1198 words outside tables, code and the maintainers' section.

## Findings

### F1 major "Instruction entries", sentence "Extension `eval` rules use `zkir-constraints.k`, `#matches`, to compare the computed value with the output register: equal values hold, unequal values violate, and computation errors become synthesis errors."
Claim: the sentence reads as the complete list of outcomes for the extension gates.
Evidence: zkir-constraints.k:195-196 (`#matches(vOk(_), vErr(S), _) => unknown(S)`) gives `unknown` when the output register is absent. More important, the `eval` rules at zkir-ext.k:266-272 (and, or, xor, concat, slice, nth, reverse) call no `#need`, so an absent input register reaches `#matches` as `vErr` and becomes `synthErr`, whereas every base gate reports `unknown` for an absent register (zkir-constraints.k:150 comment, `#need` 151-152, `#boolean` 178-179). Reproduced: a program with `private_input` of `%p` (Bool) followed by `and [%p] -> %o`, run under `--ext` with an empty private transcript, ends with status panic and verdicts `unknown | register %p is not in the witness | gate(privateInput(...))` but `synthErr | and output: variable not found: Identifier("%p") | gate(andI(...))`; the base `not` gate on an absent register in the same run gives `unknown`. Only `sha512` (via `#needAll`, zkir-ext.k:274) follows the base convention; `load_constant` reads no register. For a reader using verdicts to reason about constructibility this matters: for these seven gates a `synthErr` can mean "an earlier instruction failed", not "the circuit cannot be built".
Fix: replace the sentence with: "Extension `eval` rules use `zkir-constraints.k`, `#matches`, to compare the computed value with the output register: equal values hold, unequal values violate, a computation error (including an absent input register, since these rules call no `#need`) is a synthesis error, and an absent output register is unknown. This differs from the base gates, which report unknown for any absent register (see 08-constraints-and-verdicts.md); only the `sha512` gate, through `#needAll`, keeps that convention."

### F2 minor "Version boundary", sentence "Definition and tooling descriptions are repository observations; test results are experiment observations."
Claim: classifies the chapter's statements by evidence kind.
Evidence: the terms "repository observation" and "experiment observation" are defined nowhere in the fifteen chapters; they are the evidence vocabulary of wiki/contradictions.md. A reader of the documentation set cannot use the sentence.
Fix: delete the sentence.

### F3 minor "Byte-string equality divergence", sentence "The gate rejects the pair: `zkir-constraints.k`, `#eqSupported`, rejects unequal types, and extension rules explicitly reject mixed representations."
Claim: presents the extension rules as the operative rejection for a `bytesV`/`bytes32` pair.
Evidence: for such a pair both the inherited rule zkir-constraints.k:242 (`requires notBool sameType(A, B)`; sameType is `typeOf(A) ==K typeOf(B)`, zkir-ops.k:22, and `typeOf(bytesV(B))` is `bytesT(lengthBytes(B))`, zkir-ext.k:65) and the extension rules zkir-ext.k:184-185 match, with different messages ("Unsupported test_eq: Bytes == Bytes32" versus "Unsupported test_eq: byte strings of different length"). Neither rule has a priority or owise attribute, so the message is not fixed by the definition; the outcome sort (`synthErr`) is. For two `bytesV` values of unequal length only rule 242 applies.
Fix: "The gate rejects the pair with a synthesis error: `zkir-constraints.k`, `#eqSupported`, rejects any two values whose `typeOf` differ, and `zkir-ext.k` adds two rules for the mixed `bytesV`/`bytes32` case; both rules match that case, so the message of the synthesis error is not fixed."

### F4 minor "Modules and representation", sentence "The removed instruction also remains in imported K syntax: the JavaScript Object Notation (JSON) boundary enforces its removal."
Claim: only the syntax of `reverse_bytes` survives in ZKIR-EXT.
Evidence: ZKIR-EXT imports ZKIR-VM (zkir-ext.k:161), so the `#exec(reverseBytes(...))` rule (zkir-vm.k:263) and the `reverseBytes` gate rule (zkir-constraints.k:409) are also part of the extension definition; a hand-built `reverseBytes` term executes normally. Only `tools/zkir_kast.py`, `instruction` (line 251-253) rejects it.
Fix: "The removed instruction remains in the extension definition with its execution and gate rules (it is inherited from `ZKIR-VM`); only the JSON boundary, `tools/zkir_kast.py`, `instruction`, rejects `reverse_bytes` under `--ext`."

### F5 minor "sha512" entry, phrase "Gate: `eval(gate(sha512(...)))` requires inputs"
Claim: the gate requires the inputs.
Evidence: zkir-ext.k:274 calls `#needAll(Xs, M)`, which is `#native(rd(X, M), "hash input")` for each operand (zkir-constraints.k:449-451): each input register must be present and of type Native; an absent one gives unknown, a present non-Native one gives a synthesis error.
Fix: "Gate: `eval(gate(sha512(...)))` requires every input register to be present and Native (`#needAll`), checks the `sha2_512` chip (`#chipNamed`), and compares the same `#sha512V` result."

### F6 minor "Instruction entries", sentence "`ZKIR-WF` in `zkir-syntax.k` checks reads and writes generically; its `#checkArity` adds no extension-specific checks."
Claim: correct as stated; one useful consequence is missing.
Evidence: zkir-syntax.k:279-291 `#checkReads` checks the immediates returned by `reads`; for `loadConstant` those are the encoding elements (zkir-ext.k:42), so an out-of-field encoding element is a `wf` error ("immediate out of field range") before strict decoding runs. Nothing else about `Bytes<n>` lengths, `slice` bounds or Boolean types is checked statically.
Fix: append: "Because `reads(loadConstant(_, Es, _))` returns the encoding immediates, `#checkReads` does reject an encoding element outside the field; no other extension fact (byte-string lengths, `slice` and `nth` bounds, Boolean operand types) is checked statically."

## Coverage
- What 2ffe2d1 changes relative to the base surface (added instructions, removed reverse_bytes, added types, canonical re-encode decoding, test_eq over byte strings): covered; every Rust locator in the table verified.
- How `zkir-ext.k` is structured (four modules, Bytes<32> keeps the base representation, priority-30 job/genJob rules, `negV(boolV)`, eqDispatch extension, `#eqSupported`, concat bound, load_constant chip check): covered; all rule quotations and descriptions match the file.
- One entry per extension instruction in the structure of chapter 07: covered; nine entries with Syntax, Off-circuit, Gate, Run-time checks and Corpus, all symbols and messages verified (see F1 and F5 for the gate wording).
- How to run and test it (`--ext` in the three tools, the second oracle, the corpus directories, the receipt numbers): covered; the three commands run, the oracle path and corpus counts match diff_test.py and the directories, the receipt numbers match.
- The recorded divergence on test_eq of unequal-length byte strings: covered; matches eq.rs and wiki/contradictions.md line 50 (see F3 for the overlapping rules).
