# Audit of experiments/zkir-k/docs/13-known-divergences.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed: ran `uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py` from the repository root (exit 0, `20/20 divergence cases behave as expected`; PASS lines match `evidence/zkir-k-divergence-tests-2026-09-05b.txt` except the live k02 note now says "K reports the same panic"); ran `zkir_run.py --checked` on `corpus/divergence/k02_transcript_too_short_panics.zkir` and `k01_jubjub_from_coordinates_parity_only.zkir` with reconstructed preimages; reproduced the `0x0x01` loader split with `zkir_run.py`/`zkir_kast.py` (exit 2) and the 92e8bdd3 oracle (`%y` encoded `2`); reproduced extension `test_eq` of `Bytes<32>` vs `Bytes<4>` with `zkir_run.py --ext --checked` and the 2ffe2d1 oracle; compared K fragments with `zkir-curves.k`, `zkir-ops.k`, `zkir-vm.k`, `zkir-values.k`, `zkir-constraints.k`, `zkir-syntax.k`, `zkir-ext.k`; compared Rust locators with `repos/_build/ledger-92e8bdd3/zkir-v3/src/{ir.rs,ir_vm.rs,ir_instructions/{from_coordinates.rs,encode.rs}}` and `repos/_build/midnight-zkir-2ffe2d1/zkir/src/{ir_vm.rs,ir_instructions/eq.rs}`; compared statuses with `wiki/contradictions.md` and `wiki/zkir/zkir-v3-divergence-review.md`; confirmed `08-constraints-and-verdicts.md` exists; grepped the chapter for TODO/TBD, em-dashes, extra `#` titles, and "this document".

## Findings
### F1 blocking Notes for maintainers, all three bullets
Claim: `zkir-ext.k` lines 184-185 define overlapping `#eqSupported` rules for `bytesV`/`bytes32` `test_eq`; `zkir-vm.k` lines 450-452 say the short-transcript read "here is an error"; `divergence_tests.py` line 175 ends "K reports an error".
Evidence: `experiments/zkir-k/semantics/zkir-ext.k:181-184` comments that mixed-length `test_eq` uses the generic different-types rule and the only `#eqSupported` there is `bytesV`/`bytesV` `cond_select`. `experiments/zkir-k/semantics/zkir-vm.k:450-452` already says the `#input` rules report `panic(...)`. `experiments/zkir-k/tools/divergence_tests.py:175` ends "K reports the same panic". A developer opening those lines will not see the cited text.
Fix: Delete the `## Notes for maintainers` section. If a current defect remains, cite the file and line as they are now.

### F2 blocking "The panic on limbs of 192 bits or more"
Claim: Finding 5's historical panic was on limbs of 192 bits or more.
Evidence: `wiki/zkir/zkir-v3-divergence-review.md:42` records the panic on limbs `≥ 2^192`. A 192-bit integer lies in `[2^191, 2^192-1]` and can be below that bound. The chapter brief requires numbers to match the recorded source.
Fix: Write "limbs at least `2^192`" (or "`≥ 2^192`"), matching the wiki row.

### F3 blocking "The one place where the circuit is weaker than `preprocess`"
Claim: Finding 2 (`assert` of 2) is the only place the circuit is weaker than `preprocess`.
Evidence: The same chapter, finding 4 (`experiments/zkir-k/docs/13-known-divergences.md:95`), says off-circuit `checkBits` rejects 8 as not 3-bit while the chip would accept the padded 4-bit bound. `zkir-constraints.k:314-318` evaluates `lessThan` with `#ltBits(N) = maxInt(N +Int (N modInt 2), 4)`. Finding 3 (`f03_guard_uncoupled`) is also circuit-weaker: the `publicInput` gate never reads the guard (`divergence_tests.py:61`, `zkir-v3-divergence-review.md:40`). The live `f02` gate is `holds` and the live `f04` gate is `unknown` because `%lt` is absent; the exclusivity claim is still false of the chip and of finding 3.
Fix: Drop "the one place". Say that for input 2 the `assert` gate holds while `preprocess` errors, and that booleanity is a producer obligation.

### F4 blocking "`tools/zkir_run.py PROGRAM PREIMAGE --checked`"
Claim: A developer can rerun one case with that command, and `all_verdicts` lists every gate.
Evidence: `PROGRAM` and `PREIMAGE` are placeholders (common brief: no placeholders). The path is not the root command `uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py ...`. `experiments/zkir-k/corpus/divergence/` holds only `.zkir` files; preimages are built in `divergence_tests.py:204-214` and are not written to disk. Copying the shown command fails. A reconstructed k02 run (`inputs: ["1"]`, `binding_input: "42"`, `--checked`) did print `all_verdicts` with `bindGate`, `privateInput` unknown, and `add` unknown, so the JSON key exists once the files are supplied.
Fix: Give one complete root command with real paths, or state that preimages live only in `divergence_tests.py` and that the supported reproduction is the twenty-case harness. Example that ran: `uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py experiments/zkir-k/corpus/divergence/k02_transcript_too_short_panics.zkir PRE.json --checked` with `PRE.json` equal to `{"inputs":["1"],"binding_input":"42"}`.

### F5 major "`selectV` and `constrainEqV` in `zkir-ops.k` accept any pair of the same type"
Claim: Off-circuit both operations accept any same-type pair.
Evidence: `zkir-ops.k:131-133`: `constrainEqV` is `cOk()` only when `sameType(A, B) andBool A ==K B`; unequal same-type values are `cErr("Equality constraint failed")`. `selectV` (`zkir-ops.k:137-138`) does return a value for any same-type pair. Case `f07` uses equal scalars (42, 42), so the overclaim is not caught by that run.
Fix: "Off-circuit, `selectV` returns a value for any same-type pair. `constrainEqV` type-checks any same-type pair and succeeds only when the values are equal."

### F6 major K fence with `fsqrt(...)`
Claim: The fenced fragment is a K rule for `jubjubFromXY`.
Evidence: `zkir-curves.k:149-150` is `rule jubjubFromXY(X, Y) => ptErr("coordinate out of range") requires Y <Int 0 orBool Y >=Int #r orBool X <Int 0 orBool X >=Int #r` and `rule jubjubFromXY(X, Y) => #jubjubDecompress(X modInt 2, Y, fsqrt(fdiv(fsub(fsq(Y, #r), 1, #r), fadd(1, fmul(#jubjubD, fsq(Y, #r), #r), #r), #r), #r))`. The chapter's `fsqrt(...)` is not verbatim (common brief: quote K verbatim; no placeholders).
Fix: Quote the real `jubjubFromXY` / `#jubjubDecompress` rules without `...`, or drop the fence and describe parity-only decompression in prose.

### F7 major "classes 3 and 12 as non-defects, and lists 5 to 13 as active"
Claim: Finding 12 is both a non-defect and in the active 5-13 list.
Evidence: `wiki/zkir/zkir-v3-divergence-review.md:32-34` retires 1, 2, 4; calls 3 intentional; and says 5-13 identify active discrepancies, bug fixes, or missing chip integrations. Row 12 (`zkir-v3-divergence-review.md:49`) is "Open design omission; not a behavioral divergence". A reader cannot tell whether 12 is open-active or a non-defect.
Fix: "Findings 1, 2 and 4 are retired as producer obligations or undefined behaviour. Finding 3 is by design. Finding 12 is a design omission, not an off-circuit/in-circuit split. Findings 5-11 and 13 are the active discrepancies."

### F8 major Extension `test_eq` "Checked by hand" with `zkir_run.py ... --ext`
Claim: A hand run of `test_eq` on `%a: Bytes<32>` and `%b: Bytes<4>` with `--ext` and the 2ffe2d1 oracle gives ok, `%eq` = 0, and K `synthErr: Unsupported test_eq: Bytes32 == Bytes`.
Evidence: The paragraph has no program, preimage, or root command. I had to invent them. With `/tmp` files `inputs [%a Bytes<32>, %b Bytes<4>]`, `test_eq` into `%eq`, preimage `{"inputs":["1","0","2"],"binding_input":"0"}`, `uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py ... --ext --checked` returned status `ok`, `%eq` encoded `["0"]`, verdict `synthErr` / `Unsupported test_eq: Bytes32 == Bytes`; the 2ffe2d1 oracle returned the same memory. The behaviour is true; a reader of this chapter cannot rerun it from the text.
Fix: Paste that program, preimage, and the two commands. Note that `zkir_kast.py` still maps `Bytes<32>` to `bytes32()`, which is why the K message says `Bytes32 == Bytes` rather than `Bytes == Bytes`.

### F9 minor K2 status "recorded in `wiki/contradictions.md`"
Claim: The opening paragraph sends the reader to `wiki/contradictions.md` for every finding's upstream status, including K2.
Evidence: `wiki/contradictions.md` has rows for K1, K3, K4, K5, and extension `test_eq`. K2 appears only inside the K3 disposition ("same class as K2, the short-transcript panic"). There is no K2 row.
Fix: For K2, say the status is the same class as K3 (candidate robustness finding) and is named only in that K3 disposition, or add that the harness case `k02_transcript_too_short_panics` is the record.

### F10 minor Finding 4 has no named security relevance
Claim: Each divergence must state soundness, completeness, or availability. Finding 4's section ends "Retired; the Agda model follows the chip."
Evidence: Chapter brief "Must cover" requires security relevance for each item. Finding 2 at least says the circuit is weaker. Finding 4 describes the chip accepting a value that `preprocess` rejects, but never names soundness or completeness.
Fix: "If the output register were present, the padded 4-bit relation could hold on a value `preprocess` rejects (soundness, retired as a producer obligation)."

## Coverage
- K1 Jubjub `from_coordinates` parity: covered (live k01: ok, `%px` is the true generator x, gate `violated("from_coordinates: (x, y) is not on the curve")`).
- K2 transcript read past the end: covered (live k02: panic both sides, `private_input` unknown; F9 is wiki lookup only).
- K3 Bytes32 non-canonical panic: covered (live k03: panic both sides; `--ext` decode of the same element is `error` / `Failed to decode as Bytes32`).
- K4 Jubjub chip for `jubjub_scalar_from_native` and native `from_coordinates`: covered (live k04 / k01b: ok / synthErr).
- K5 `less_than` 253 or 254 bits: covered (live k05: ok / `synthErr: less_than: padded bound 254 exceeds MAX_BOUND_IN_BITS = 253`).
- Extension `test_eq` on unequal-length byte strings: partly (behaviour and K symbols match; F8, no command in the chapter).
- Assert of 2 (finding 2): covered (F3 is the exclusivity sentence).
- `cond_select` on Bytes32 (finding 6): covered (shared `###` with finding 7; live f06: ok / `Unsupported cond_select: Bytes32 ? Bytes32`).
- `constrain_eq` on JubjubScalar (finding 7): covered (shared section; F5 on `constrainEqV`; live f07: ok / `Unsupported constrain_eq: JubjubScalar == JubjubScalar`).
- `bytes32_from_low_high` with foreign high (finding 8): covered (live f08 message matches).
- `less_than` 3-bit (finding 4): partly (case and outcomes match; F3/F10).
- secp256k1 chip via `from_bytes32` (finding 13): covered (live f13: `chip not initialised for Secp256k1Base`).
- Order-2 Curve25519 point (finding 11): covered (live f11: error / `point is not in the prime-order subgroup`).
- Non-canonical secp256k1 input and the commitment gate (finding 5): covered except the 192-bit threshold (F2); live f05: ok / `commGate` violated with the quoted Poseidon message.
- Summary table: covered (rows exist for the required findings; short names `k01`/`k04` are prefixes of the corpus files).
