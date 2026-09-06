# Review R2: the VM (`zkir-vm.k`) against `IrSource::preprocess`

Reviewer: Fable (formal methods). Date: 2026-09-05.
Sources of truth: `zkir-v3/src/ir_vm.rs` lines 185-691, `ir_instructions/*.rs` off-circuit
functions, `ir_types.rs`, `transient-crypto/src/{hash,repr,fab}.rs` at midnight-ledger 92e8bdd3.

VERDICT: WARNING — the 34 instruction rules match `preprocess` value for value and error for error on every path I could reach, but the VM is only faithful *after* the static check `wf` has run, and neither `job` nor the tools run it, so the shipped pipeline accepts three programs the crate rejects and reports a stuck configuration as `ok`; the differential harness compares error runs by status only, which is why none of this showed up.

## 1. Findings

### F1 (major) The VM does not run the static check it depends on; three Rust runtime rejections are absent

- `experiments/zkir-k/semantics/zkir-vm.k:87` — `job(program(...), Pre) => #loadInputs(...) ~> #seedPi ~> Is ~> #verdicts`. No call to `wf`. `zkir.k:4-6` imports only `ZKIR-VM`; `wf` lives in `ZKIR-CHECK`, a separate kompiled definition. `tools/zkir_run.py:271-309` and `tools/diff_test.py:146-149,167` build `job` straight from `zkir_kast.load_program` and never invoke `ZKIR-CHECK` (only `divergence_tests.py:163-174` does).
- `zkir-vm.k:301-302` — comment "bits <= 248 checked statically"; the rule matches only `divModPowerOfTwo(V, N, (Q, (R, .Ids)))` and has no bound on `N`. `zkir-vm.k:310-322` — `reconstitute_field` has no `N <= 248` check either. `zkir-syntax.k:296-301` puts these three checks (output count, `div_mod` bits, `reconstitute_field` bits) in `ZKIR-WF` only.
- Rust: `ir_vm.rs:396-401` bails `"DivModPowerOfTwo requires exactly 2 outputs"` and `"Excessive bit count"` at run time; `ir_vm.rs:418-420` bails `"Excessive bit count"` for `reconstitute_field`. These are dynamic checks in `preprocess`, not load-time checks.
- Observed (commands in section 5):
  - `corpus/handmade-negative/divmod_outputs.zkir` (one output): K `status ok`, `<k>` cell left at `#exec(divModPowerOfTwo(var("%a"), 8, "%q", .Ids)) ~> .Instrs ~> #verdicts` (stuck, no rule applies); `zkir_run.py` reports `ok` because it reads only `<status>` (`zkir_run.py:279-285`; the k cell is captured at line 295 but never inspected). Rust: `error: DivModPowerOfTwo requires exactly 2 outputs`.
  - `div_mod_power_of_two` with `bits: 250` on input 5: K `ok`, `%q = 0`, `%r = 5`. Rust: `error: Excessive bit count`.
  - `reconstitute_field` with `bits: 250` on (3, 7): K `ok`, `%o = 3*2^250 + 7 mod r`. Rust: `error: Excessive bit count`.
- Why it matters: the definition is cited (wiki/zkir/zkir-k-definition.md:30, :48) as a model of `preprocess`; standing alone it is not. A stuck configuration reported as `ok` is the worst failure mode for a differential oracle: it silently produces a partial witness. The corpus has no such program, so the 314/314 receipt cannot detect it.
- Fix (two parts, both small): (a) mirror the Rust dynamic checks in the VM — add `rule <k> #exec(divModPowerOfTwo(_, N, _)) => #fail("Excessive bit count") ...</k> requires N >Int #frBytesStored *Int 8`, an `[owise]` rule for `divModPowerOfTwo` with `lenIds(Os) =/=Int 2` failing with the Rust message, and the same bound in `#exec(reconstituteField(...))`, so the VM is faithful without `wf`; (b) make `job` evaluate `wf(P)` first and set `<status> error(S)` on `wfError(S)` (the K side is then strictly stricter, as intended by CLM-0711/0725), and make `zkir_run.py` report `krun-stuck` whenever `<k>` is not `.K`.

### F2 (major) The differential harness compares error runs by status only; 277 of 314 "agreements" are "both failed"

- `tools/diff_test.py:98-103` — `compare` returns `[]` as soon as `k['status'] == r['status'] != 'ok'`. The error texts are only printed as a note (`diff_test.py:172-174`, first 20 characters, not counted). The receipt `evidence/zkir-k-differential-92e8bdd3-2026-09-05.txt` has 37 `K=ok Rust=ok` rows out of 314.
- Consequence observed: `reconstitute_field` with `bits: 0`, divisor of type `JubjubScalar`, modulus 0: K `error Excessive bit bound`, Rust `error cannot convert JubjubScalar to "Native"` (see F4). The harness would count this as agreement. Any wrong error kind in K (e.g. an instruction failing for the wrong reason, a type check firing where Rust decodes) is invisible on 88% of the comparisons.
- Fix: compare a normalised error class. Map each side's message to a small enum (variable-not-found, type-conversion, bit-bound, boolean, assertion, constrain-eq, decode, transcript-short, transcript-unconsumed, commitment, impact-mismatch, unsupported-op, output-arity, output-type) by prefix, and fail on class mismatch; record the raw pair in the receipt. Also fail on `oracle-failed` vs `error` explicitly labelled as "Rust panic" so the two known panic classes (F5) are visible instead of being avoided.

### F3 (major) Option alignment segments: K errors where `preprocess` succeeds

- `zkir-ops.k:221` — `#alignedBytes((option(_), _), _, _) => bErr("unsupported: alignment option segment")`, reached from `#exec(persistentHash/keccak256)` at `zkir-vm.k:351-352`, so the run ends in `<status> error(...)`.
- Rust: `ir_vm.rs:493` calls `alignment.parse_field_repr(&inputs)`; `transient-crypto/src/fab.rs:310-334` parses an `Option` segment (variant tag as `u16`, chosen alternative, zero padding to the longest alternative), and `ValueReprAlignedValue::binary_repr` (`fab.rs:428-433`) lays it out; `preprocess` returns `Ok`. Only the *circuit* half rejects it (`ir_vm.rs:114-119`).
- Observed: `persistent_hash` with alignment `[option [[field], [bytes 1]]]` and inputs `["0x00", "%x"]`: K `error unsupported: alignment option segment`; Rust `ok`, `%h` = Bytes32.
- Why it matters: Compact emits option alignments for `Maybe`/enum-typed hashed values; a K model of `preprocess` that fails on them cannot be used as an oracle for such programs, and because F2 folds every K error into "error", a Rust *error* on such a program would be counted as agreement. No corpus program uses an option segment (grep over the corpus and the escrow/swap artifacts), so this is untested rather than wrong-and-tested.
- Fix: either implement the option arm of `parse_field_repr_inner` (variant from `repr[0]` via `u16` range check, recurse into `Alignments[variant]`, then require `maxFieldLen - chosenFieldLen` zero elements) plus the binary layout, or introduce a distinct status `unsupported(String)` so the divergence is never conflated with a Rust error. State the limitation in wiki/zkir/zkir-k-definition.md.

### F4 (minor) `checkBits` tests the bound before the type; Rust converts first

- `zkir-ops.k:253` — `rule checkBits(_, N) => cErr("Excessive bit bound") requires N >=Int #frBits` fires before the `isNative` test at `:257`.
- Rust `ir_vm.rs:255-273` — `resolve_operand_bits`: `val.try_into()?` (`"cannot convert T to Native"`) precedes `if n >= FR_BITS { bail!("Excessive bit bound") }`.
- Reachable after `wf`: `reconstitute_field` with `bits = 0` gives `checkBits(DV, 255)` at `zkir-vm.k:314`; a non-native divisor then yields K `"Excessive bit bound"` vs Rust `"cannot convert JubjubScalar to Native"` (observed, section 5). Without `wf` (F1) also `constrain_bits`/`less_than` with `bits >= 255` on a non-native operand.
- Fix: reorder — `checkBits(V, _) => cErr("cannot convert ...")` when `notBool isNative(V)` first, then the `N >= #frBits` rule, then the fits test.

### F5 (minor) Two Rust panics are modelled as errors; only one is recorded as a divergence

- `zkir-values.k:155-161` — a Bytes32 raw pair with `Lo >= 2^248` or `Hi >= 256` gives `decErr("Bytes32 decoding assertion failed")`. Rust `encode.rs:150-164` executes `assert_eq!(bytes[31], 0)` and panics (observed: `panicked at zkir-v3/src/ir_instructions/encode.rs:155:17`, oracle exits non-zero). The other panic (transcript shorter than `encoded_len`, `ir_vm.rs:359-360, 378-380`) is modelled at `zkir-vm.k:395-401` and recorded as K2 in `divergence_tests.py`; the Bytes32 one is only a code comment.
- Why it matters: the K error is the right modelling choice, but the record should say so, and the harness must never generate these preimages by accident (F6 explains why it currently cannot).
- Fix: add a `k03_bytes32_noncanonical_input_panics` case to `divergence_tests.py` with expected Rust `oracle-failed`, and list both panics in wiki/zkir/zkir-k-definition.md next to CLM-0725.

### F6 (minor) Harness blind spots: perturbation shape, generation fixpoint, mismatch path, seeds

- `diff_test.py:179-182` — the only perturbation replaces `inputs[0]` with a uniform field element. For the 22 corpus programs whose first input is a point, Bytes<32> or a foreign field element the replacement is (almost surely) an invalid encoding, so the perturbed run is an error/panic comparison, never a second successful run; the receipt has exactly one successful perturbed row (`test_ec_proof.zkir`).
- `diff_test.py:87` — `public_transcript_inputs` is overwritten with the values the K generation run pushed, so on the real run `#impactCheck` (`zkir-vm.k:426-429`) and the Rust check (`ir_vm.rs:524-541`) always pass; the "Public transcript input mismatch" path is never compared on a controlled input, and a K bug in the index bookkeeping (`Idx` versus `public_transcript_inputs_idx - count + i`) would be masked because both the expected vector and the check come from K.
- `diff_test.py:162` with `corpus/ledger9-92e8bdd3-tests/manifest.json` — programs whose crate test builds inputs programmatically carry `test_preimage.inputs = []`; attempt 0 is then always `"Not enough raw inputs"`, wasting one of eight attempts (24 receipt rows).
- `diff_test.py:76-77,89-90` — the commitment is taken from the K generation run (`needComm`, `zkir-vm.k:457-458`) and fed to both sides, so the commitment check is compared as "Rust accepts what K computed" — a legitimate oracle for the hash, but the `"Communications commitment mismatch"` branch (`zkir-vm.k:475`) is only ever hit on error-only comparisons.
- Not compared at all: the three cursors, the `<outputs>` list (only indirectly through the commitment, on the 6 successful `do_communications_commitment` programs) and `<pi_skips>` for programs that never succeed (F7).
- Fix: add a second perturbation that re-encodes a valid random value of the declared type for a random input position; add one run per program with a deliberately wrong `public_transcript_inputs[k]` and a wrong commitment, expecting the mismatch class on both sides; drop empty seeds; print `cursors` from an extended oracle.

### F7 (minor) Single assignment: the VM overwrites, the header and the plan say otherwise

- `zkir-vm.k:7` — "`<mem>` registers, String |-> Value, write-once"; `zkir-vm.k:185,187,277,305,321,332,374` — every write is `M[X <- V]`, which overwrites. `wiki/zkir-k-semantics-plan.md:53` (CLM-0711) — "a rule that writes an identifier already present in the map is an error". Only `ZKIR-WF` (`zkir-syntax.k:277-278`) rejects reassignment, and it is not wired into `job` (F1).
- Rust `ir_vm.rs:304` etc. — `memory.insert` overwrites; observed `corpus/handmade-negative/reassignment.zkir`: K `ok`, `%b = 10`; Rust `ok`, `%b = 10`. So the VM agrees with the crate here, and CLM-0711 describes a property of the static check, not of the VM.
- Consequences of the K-side strictness that are not yet stated in the record: (i) the constraint checker `verdicts` assumes one binding per name, so on a reassigning program (accepted by the crate) every gate referring to the old value is evaluated against the new one — the faithfulness statement (CLM-0704) is only meaningful for WF programs; (ii) the in-circuit `mem_insert` (`ir_vm.rs:761-781`) only checks a re-inserted id against the *witness* value, so a reassigning program that proves in Rust may have gates over the first binding that K would report as violated; (iii) `writes(divModPowerOfTwo(_, _, (Q, Q)))` and `intoCoordinates(_, X, X)` are rejected by `wf` but accepted (second write wins) by the crate.
- Fix: correct the header comment; restate CLM-0711 as "the static check rejects reassignment; the VM, like the crate, overwrites; all faithfulness claims are relative to WF programs"; wire `wf` into `job` (F1b).

### F8 (nit) Comment contradicts (correct) code on byte order of `bytes_from_field_repr`

- `zkir-ops.k:226-227` — "stray = n mod 31 bytes come first (from repr[0]), then the 31-byte chunks". The code (`zkir-ops.k:233,240`) appends the stray bytes *after* the chunks, which is what Rust does: `transient-crypto/src/repr.rs:137-159` writes the stray bytes to `res[n - stray..]` and chunk `i` (from `repr[chunks - 1 - i]`) to `res[i*31..(i+1)*31]`. Fix the comment.

### F9 (nit) Dead rules and an unused error branch

- `zkir-vm.k:278` — `#bindEncoded((_, _), .List) => .K` is unreachable after the length check at `:271-274`.
- `zkir-vm.k:472` — `#commCheck(noComm(), _) => #fail("Expected communications randomness")` is unreachable: `#seedPi` (`:119,124`) already failed with `"Expected communications commitment"`, and `#finish` requires `<status> ok()`. Rust has the same dead branch (`ir_vm.rs:663-665`); keep the rule but comment it as mirroring dead code.
- `zkir-ops.k:135,141`, `zkir-values.k:136` — `jubjubPoint(inf())` / `curve25519Point(inf())` arms are unreachable given `zkir-curves.k:8` ("Edwards curves never use inf()").

### F10 (nit) Error precedence differences that are unobservable only because of `wf`

- `output`: K resolves all operands (`zkir-vm.k:437`) and then checks arity (`:440`); Rust checks arity first (`ir_vm.rs:626-632`) and resolves per position.
- `reconstitute_field`: K resolves divisor then modulus and fails on the modulus error first (`zkir-vm.k:312-313`); Rust resolves the modulus first and *also* type-checks it before touching the divisor (`ir_vm.rs:432-434`).
- `transient_hash`, `hash_to_curve`, `persistent_hash`, `keccak256`, `impact`: K resolves every operand before converting any (`resolveAll` then `asNatives`); Rust interleaves resolve/convert per operand.
- All differences need an undefined variable to be observable, which `wf` excludes; with F1 fixed they are moot. Otherwise no fix needed beyond a comment.

### F11 (nit) Preimage field elements are not range-checked on the K side

- `zkir_run.py:60-70` — `preimage_term` passes any integer through; `zkir-vm.k:110` binds decoded `native(X)` with `X` possibly `>= r`. The oracle rejects such values (`zkir-oracle/src/main.rs:52-60`), so the two sides are never compared on them, but `#impactCheck` (`zkir-vm.k:427`) compares raw preimage integers to canonical pushed values, and a non-canonical `public_transcript_inputs` entry would be a spurious mismatch. Fix: reduce or reject in `preimage_term`, or add `requires` guards on `decodeValue(ListItem(X), native())`.

## 2. Instruction-by-instruction check (what matched)

For each arm of `preprocess` (ir_vm.rs line) against the K rule, I checked operand order, accepted type pairs, side effects and error condition. All of the following match on every path reachable from a WF program; deviations are the findings above.

| Arm (ir_vm.rs) | K rule | Notes |
|---|---|---|
| Encode 287-299 | vm.k:268-278, values.k:132-146 | length check before any write; message uses `get_type()` Debug = `typeName`. |
| Add/Mul/Neg/Inv 300-321 | ops.k:35-98 | type tables identical to add.rs:43-67, mul.rs:39-58, neg.rs:43-66, inv.rs:39-77 (11/7/11/7 arms); `"cannot invert zero of type T"`. |
| Not 322-325 | vm.k:234-237 | `resolve_operand_bool`: 0/1 only, `"Expected boolean, found: "` (Rust prints hex, K decimal). |
| ConstrainEq 326-330 | vm.k:255-259, ops.k:110-113 | type mismatch before value mismatch, same as constrain_eq.rs:42-58. |
| CondSelect 331-336 | vm.k:261-266, ops.k:115-118 | bit resolved first, then a, b, then same-type check (select.rs:43-52); any same type accepted, incl. Bytes32/JubjubScalar (divergence F6/F7 belong to the circuit side). |
| Assert 337-341 | vm.k:239-243 | `"Failed direct assertion"`; non-boolean is a conversion error first. |
| TestEq 342-347 | vm.k:196, ops.k:103-105 | same-type pairs incl. Bytes32 and JubjubScalar (eq.rs:42-67); `==K` on canonical values is sound because Edwards identities are always `pt(0,1)` and Weierstrass identities always `inf()`. |
| PublicInput/PrivateInput 348-386 | vm.k:365-404 | guard error propagates; guard false -> `IrValue::default` (values.k:57-70 matches ir_types.rs:182-203, Weierstrass default = identity, Edwards = (0,1)); slice `[idx, idx+w)`, cursor advanced before decode; short transcript: K error vs Rust panic (F5/K2). |
| Copy 387-390 | vm.k:213 | |
| ConstrainToBoolean 391 | vm.k:245-248 | |
| ConstrainBits 392-394 | vm.k:250-253, ops.k:252-257 | order of type/bound checks differs only for bits >= 255 (F4). |
| DivModPowerOfTwo 395-411 | vm.k:302-306 | `outputs[0] = val >> bits`, `outputs[1] = val mod 2^bits` = `highBits`/`lowBits` (field.k:113-118); missing runtime checks (F1). |
| ReconstituteField 412-453 | vm.k:310-322 | modulus < 2^bits, divisor < 2^(255-bits), `divisor*2^bits + modulus <= r-1` (bitwise compare against `Fr::from(-1)` is the integer compare since the sum < 2^255), result mod r; bits = 0 -> `"Excessive bit bound"` on both sides for native operands; missing bits <= 248 check (F1); precedence (F4, F10). |
| LessThan 454-462 | vm.k:324-332 | both operands bounded by `bits`, result `a < b` as integers. |
| JubjubScalarFromNative 463-467 | vm.k:217,225,232 | `from_bytes_wide` of 32 LE bytes = `x mod rJ`. |
| TransientHash 468-477 | vm.k:335-341 | all operands Native; Poseidon not re-verified here (R-hash), validated by `transient_hash.zkir` and 6 commitment programs. |
| PersistentHash/Keccak256 478-506 | vm.k:351-363, ops.k:212-247 | field atom = 32 LE bytes (`Fr::binary_repr`, repr.rs:49-52); bytes atom = chunks then stray (repr.rs:127-160), non-zero high bytes rejected; compress -> `None` -> same message; option -> F3. |
| Impact 507-543 | vm.k:151-159, 407-434 | inactive: `count` zeros pushed, `Some(count)`, `public_transcript_inputs_idx` unchanged; active: values pushed one by one (non-Native -> conversion error), `None`, idx += count, then per-position compare against `public_transcript_inputs.get(idx)` with `None` on overrun = K `Idx < size andBool ... ==Int`; first mismatching index reported. |
| HashToCurve 544-552 | vm.k:343-349 | `FieldRepr for Vec<Fr>`/`[Fr]` writes the elements verbatim (repr.rs:203-211, 279-286). |
| EcMul 553-558 | ops.k:121-126 | four pairs, ec_mul.rs:33-46. |
| EcMulGenerator 559-568 | ops.k:128-131 | Jubjub and Secp256k1 only; error text identical. |
| IntoCoordinates 569-574 | ops.k:133-142 | Weierstrass identity error texts identical (into_coordinates.rs:49-66); Edwards identity -> (0,1). |
| FromCoordinates 575-580 | ops.k:144-153 | Jubjub via `from_xy` + `into_subgroup` (CtOption: rejects torsion) = `jubjubFromXY` + `inSubgroup`; foreign curves on-curve then subgroup check (from_coordinates.rs:39-77). |
| IntoBytes32 581-585 | ops.k:156-164 | Native + six foreign fields, 32 LE bytes. |
| FromBytes32 586-595 | vm.k:293-299, ops.k:167-175 | operand must be Bytes32 (`"cannot convert T to Bytes32"`); Native via `from_uniform_bytes` = int mod r; foreign via `from_le_bytes_with_reduction` = int mod p; unsupported target types -> same message. |
| ReverseBytes 596-601 | vm.k:216,224,229 | |
| Bytes32IntoLowHigh 602-610 | vm.k:285-291, ops.k:178-179 | high = byte 31, low = bytes 0..31 as Native. |
| Bytes32FromLowHigh 611-624 | vm.k:199, ops.k:182-194 | both operands through `into_bytes32` (foreign fields accepted), `low[31] == 0` and `high[1..] == 0`, else the long message. |
| Output 625-644 | vm.k:437-446 | arity, then per-position `get_type() == expected`, values appended to `outputs` (multiple `output` instructions accumulate on both sides; observed via commitment mismatch on a two-`output` program). |
| Start 189-221 | vm.k:87-125, values.k:238-251 | `"Not enough raw inputs"` before decode, decode error, then `"Expected N raw inputs, received M"`; `[binding] ++ [commitment]`, `"Expected communications commitment"` after input decoding. |
| End 648-681 | vm.k:453-480 | all three cursors must equal the transcript lengths; commitment = `transient_hash([rand] ++ inputs ++ encode(outputs))` (hash.rs:84-88, repr.rs:279-286) = `transientCommit` (hash.k:73-75); `"Communications commitment mismatch"`. |
| Generation mode | vm.k:84, 381-384, 391-394, 430-431, 456-458 | every gen rule is guarded by `<genMode> true`, `job` never sets it, no rule reads it otherwise; it cannot change a real run. |
| Dead witness | vm.k:135-159 | constraints still emitted; `impact` positions continue from `size(Pi) + deadPis`; consistent with the frozen `<pi>`. |

## 3. Coverage gaps

- 31 of the 62 differential programs never reach a successful run in eight attempts (receipt analysis, section 5): all 13 Compact-generated programs (escrow `fund/release/refundAfterTimeout`, swap `decide/expire/fundAlice/fundBob`, micro-dao x6) and 16 crate tests (`test_bytes32_*`, `test_reverse_bytes_proof`, `test_coordinates_proof`, all `test_curve25519_*`, `test_secp256r1_*`, `test_secp256k1_proof`, `test_jubjub_point_ops`, the three `*_constrain_eq_fails_on_unequal`). Their success paths — guarded `impact` with realistic guards, guarded `public_input`/`private_input` mixed with commitments, Bytes32 round trips on decoded inputs — are covered only by `handmade/transcripts*.zkir` and `native_bytes.zkir`. Every one of the 34 ops does appear in at least one successful run, thanks to the handmade set.
- Never compared on a success path with a controlled expectation: `"Public transcript input mismatch"`, `"Communications commitment mismatch"`, `"Transcripts not fully consumed"` with a *longer* transcript, a guard that is an immediate other than `0x01`, `impact` with zero inputs (`pi_skips = [Some(0)]` / `[None]`), `public_input` past the end (Rust panic), Bytes32 inputs with a non-zero byte 31 (Rust panic), option alignments (F3), `persistent_hash` with a field atom followed by a bytes atom with stray bytes in one alignment, `reconstitute_field` with `bits = 0`, `div_mod_power_of_two` with `bits = 248`, `less_than` with `bits` in 249..254, multiple `output` instructions, programs with zero inputs and a commitment (escrow/swap never succeed).
- Types: `Scalar<Jubjub>` as a raw *input* (only appears as a `from_native` result), `Point<Secp256k1>` identity as a value (`ec_mul` by 0 or `add(P, neg P)`), Curve25519 identity through `into_coordinates`.
- The negative corpus (`handmade-negative`) is only run through `ZKIR-CHECK`, never through the VM, which is why F1 went unnoticed.

## 4. Questions for the authors

1. Is `job` meant to be run only on `wf`-accepted programs? If so, where is that contract stated, and why do `zkir_run.py`/`diff_test.py` not enforce it? If not, F1 is a definition bug.
2. Should the K VM model Rust panics (`k02`, Bytes32 asserts) as `error`, or as a distinct `panic(String)` status so the harness can assert `oracle-failed` on those and `error` on the rest?
3. Is the omission of option alignment segments a deliberate scoping decision (because the circuit rejects them) or a to-do? The definition record does not say.
4. The Compact-generated programs never succeed under random inputs; is there a plan to seed them with real Compact test vectors (`experiments/moriarty-*/output`) so that the realistic `impact`/guard/commitment paths get a green comparison?
5. CLM-0711 says the VM errors on reassignment; the VM overwrites. Which is the intended statement of record?

## 5. What I checked and how

Files read in full: `experiments/zkir-k/semantics/{zkir-vm,zkir-ops,zkir-syntax,zkir-values,zkir,zkir-check,zkir-test}.k`, the identity/`ecAdd`/`inSubgroup`/`jubjubFromXY` parts of `zkir-curves.k`, `highBits/lowBits/fitsBits` in `zkir-field.k`, `transientCommit` in `zkir-hash.k`, `eval(piGate ...)` in `zkir-constraints.k:140-160`; `tools/{zkir_run,diff_test,zkir_kast,zkir_values,divergence_tests,check_corpus}.py`; Rust `zkir-v3/src/ir_vm.rs` (all 1275 lines), `ir_types.rs`, `ir.rs:175-250,920-1019`, every `*_offcircuit` in `ir_instructions/*.rs`, `zkir-oracle/src/main.rs`, `transient-crypto/src/hash.rs:60-110`, `repr.rs:108-296`, `fab.rs:295-362,415-596`, `curve.rs:346-354`, `base-crypto/src/fab/encoding.rs:263-277,635-647`; receipts `evidence/zkir-k-differential-92e8bdd3-2026-09-05.txt` and `evidence/zkir-k-divergence-tests-2026-09-05.txt`; `wiki/zkir/zkir-k-definition.md`, `wiki/zkir-k-semantics-plan.md` (single-assignment and generation-mode statements).

Commands run from the repository root (scratch files under the session scratchpad `r2/`):

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py PROG PRE
~/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle PROG PRE
```
on: `corpus/handmade-negative/{divmod_outputs,excessive_bits,reassignment,undefined_variable}.zkir` with `{"inputs":["5"],"binding_input":"42"}`; `e2_divmod250.zkir` (`div_mod_power_of_two bits 250`, input 5); `e3_recon250.zkir` (`reconstitute_field bits 250`, inputs 3,7); `e5_bytes32_noncanon.zkir` (`copy` of a `Bytes<32>` input with raw pair `[2^250, 5]`); `e7_recon0_nonnative.zkir` (`reconstitute_field bits 0`, divisor `Scalar<Jubjub>` = 3, modulus 0); `e8_lt255.zkir` (`less_than bits 255`); `e9_output_twice.zkir` (two `output` instructions, commitment `["0","1"]`); `e6_option_align.zkir` (`persistent_hash` over `[option [[field],[bytes 1]]]`, inputs `0x00, %x`). Results are quoted in F1, F3, F4, F5 and section 2 (Output row). `e8` and `excessive_bits`/`undefined_variable`/`reassignment` agreed on both sides.

Receipt analysis: a Python pass over the differential receipt collecting, per program, whether any row is `K=ok Rust=ok`, and the union of ops in those programs (`62 programs, 31 with a successful run, ops never on a success path: []`); grep of the corpus and escrow/swap artifacts for `"option"` alignments (none).

Not re-verified here (other reviews): Poseidon constants and sponge, `hash_to_curve`, foreign-field limb encodings, curve arithmetic, the constraint checker, the ZKIR-EXT surface.
