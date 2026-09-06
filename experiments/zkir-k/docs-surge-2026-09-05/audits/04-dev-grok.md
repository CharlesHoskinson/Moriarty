# Audit of experiments/zkir-k/docs/04-values-and-encoding.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed: ran `uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py` from the repository root (exit 0, last line `42/42 checks passed`, 42 PASS names match the cross-check table and `evidence/zkir-k-unit-values-2026-09-05b.txt`); compared the 13 Value constructors, symbols, `typeOf` / `typeName` / `defaultValue`, `encodeValue`, `encForeign`, `encField64`, `encWPoint`, `encEPointForeign`, `decodeValue`, `decodeStrict` and `#canonical` with `zkir-values.k` lines 27-238; compared `encodedLen` with `zkir-syntax.k` 47-60 and `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_types.rs` 93-113; compared layouts with `encode.rs` 36-83 and 143-215, `midnight-circuits-7.2.4` `field_chip.rs` 130-165, `params.rs` 181-376, `edwards_chip.rs` 93-168 and 358-378, `weierstrass_chip.rs` 235-265, `curves.rs` 109-183 (`NUM_BITS_SUBGROUP` 252, `from_xy` parity-only on Jubjub); compared `#loadInputs`, `#seedPi`, `#impactOne`, `#input`, `#encode`, `#output1`, `#finish` with `zkir-vm.k` 105-547; compared `#encodeAll` / `commGate` / `#encInputs` with `zkir-constraints.k` 102-120; compared extension Bool/Byte/Bytes, `mkBytes`, `#encChunks`, `#decChunks`, `job`/`genJob` `strictDecode` and `load_constant` with `zkir-ext.k` 21-23, 60-104, 168-169, 245; compared `ERROR_CLASSES` / `err_class` with `diff_test.py` 67-96 and register compare at 160; compared `preimage_term` / `encode_value` with `zkir_run.py` 61-82 and 216-258; compared `f05_noncanonical_foreign_limbs` and `k03_bytes32_input_assertion_panics` with `divergence_tests.py` 70-75 and 141-144 and `evidence/zkir-k-divergence-tests-2026-09-05b.txt` line 40; grepped both `evidence/zkir-k-differential-*-2026-09-05b.txt` for `panic` (no hits); compared preprocess / encode / transcript / TryFrom strings with `ir_vm.rs` 194-210, 290-294, 359-381 and `ir_types.rs` 296-297; compared K1/K2/K3 names with `wiki/zkir/zkir-k-definition.md` line 58 and `wiki/contradictions.md` 47; confirmed `tn` in `zkir-ops.k` 18-19, `Point ::= pt | inf` in `zkir-curves.k` 19-20, `inSubgroup` at 82-84, `#blsScalarModulus` / `#r` equality, `BYTES_PER_FIELD_ELEMENT = 31` at midnight-zkir `ir_types.rs` 39, 2ffe2d1 canonical re-encode at `encode.rs` 232-244, oracle memory print at `zkir-oracle/src/main.rs` 91-103; ran `git log -1` on `repos/midnightntwrk/midnight-ledger` (`a8ab82ba…`, seven `IrType` variants, secp256k1 `encoded_len` 8/4/4); grepped the chapter for `TODO`, `TBD`, em-dashes, extra `#` titles, "this document"; confirmed one `#` title and that `05-fields-curves-and-hashes.md` and `08-constraints-and-verdicts.md` exist.

## Findings
### F1 major Decoding from raw / "The parity-only behaviour is divergence K1"
Claim: JubjubPoint `decodeValue` uses `#decJubjub(jubjubFromXY(X, Y))` (y plus parity of x), and that behaviour is divergence K1.
Evidence: The decode rule is true (`zkir-values.k:169-174`; `jubjubFromXY` at `zkir-curves.k:148-160`; midnight-circuits `JubjubExtended::from_xy` at `ecc/curves.rs:120-137`; `AssignedNativePoint::from_public_input` calls `C::from_xy` then `try_into_subgroup`, `edwards_chip.rs:99-104`). K1 is not that decode path. `wiki/zkir/zkir-k-definition.md:58` and `divergence_tests.py:127-133` name K1 as Jubjub `from_coordinates` (case `k01_jubjub_from_coordinates_parity_only`). A reader of this chapter who then opens 13-known-divergences.md would look for a decode/input case and not find one.
Fix: Keep the description of `jubjubFromXY`. Replace the K1 sentence with: the same `from_xy` parity-only rule is divergence K1 on the `from_coordinates` instruction (see 13-known-divergences.md). A JubjubPoint raw input whose x has the right parity but the wrong value therefore decodes to the real point on the base surface, and `#canonical` rejects it on the extension because `encodeValue` writes the recovered x.

### F2 minor Native decode / "Through the tools this branch is unreachable"
Claim: the `decErr("is not a canonical field element")` arm of `decodeValue` for `native()` cannot be reached through the tools.
Evidence: `zkir_run.py:61-64` (`preimage_term` / `fr`) and the oracle (`zkir-oracle/src/main.rs:61`) do refuse out-of-range integers before a run. The same chapter then lists `dec native out of field` as a `unit_values.py` check and tells the reader to run that command. Running it from the repository root prints `PASS dec native out of field` and `42/42 checks passed`. The error table's "not reached through the runner" is the accurate wording.
Fix: say the arm is unreachable through `zkir_run.py` and the oracle because they reject the integer first; `unit_values.py` still evaluates it as a ZKIR-TEST term.

### F3 minor Native decode / "with the same wording"
Claim: `preimage_term` refuses out-of-range integers with the same wording as K (`is not a canonical field element`).
Evidence: K (`zkir-values.k:157`) is exactly `decErr("is not a canonical field element")`. `zkir_run.py:64` is `raise PreimageError(f'{n} is not a canonical field element')`. The oracle (`main.rs:61`) is `"{s} is not a canonical field element"`. The shared phrase matches; the tools prefix the integer or the source string.
Fix: quote the Python/oracle form `{n} is not a canonical field element` and say K emits the suffix only.

### F4 minor Error messages / "a class made of its first 40 characters"
Claim: a message matching no `ERROR_CLASSES` pattern gets a class made of its first 40 characters; the encode-length row then says "fallback class from the identical first 40 characters".
Evidence: `diff_test.py:92-96` is `return 'other:' + (msg or '')[:40]`. Both sides of an encode-length failure still agree, because the prefix is the same, but a developer dumping `err_class` will see `other:Unexpected output length of encode in`, not the raw 40 characters.
Fix: "a class `other:` plus the first 40 characters of the message".

### F5 minor Decoding from raw / Inputs / missing 13-known-divergences.md on K1 and K2
Claim: K3 is "see 13-known-divergences.md"; K1 and K2 are named only.
Evidence: The chapter already points at 13 for K3 (`04-values-and-encoding.md:82`). K2 is the short-transcript panic (`wiki/zkir/zkir-k-definition.md:58`; `zkir-vm.k:433,443`; `ir_vm.rs:359-361` slices before a length check). Without the same pointer, a reader of this chapter has no file to open for the recorded case and the expected statuses.
Fix: add "see 13-known-divergences.md" on the K1 (after F1's rewording) and K2 sentences.

### F6 minor Foreign fields / "Limbs above p are therefore accepted and reduced"
Claim: foreign decoding accepts "limbs above p" and reduces them (`f05_noncanonical_foreign_limbs`).
Evidence: each batch is bounded by `2^(LOG2_BASE * limbs-per-element)` (`zkir-values.k:113-114`; `field_chip.rs:149-158`). For the 64-bit fields that bound is `2^192`, which is below a 256-bit `p`, so an individual limb is never `>= p`. What can exceed `p` is the assembled integer before `fadd(Acc, 1, P)` (`zkir-values.k:109`). `divergence_tests.py:70-75` builds that case as the 192+64 split of `p+4` (the unique-zero form of `p+5`).
Fix: "a reconstructed integer greater than or equal to p is accepted and reduced modulo p (case `f05_noncanonical_foreign_limbs`)".

### F7 minor Encoding to raw field elements / "Native is the identity"
Claim: Native encoding "is the identity (`AssignedNative::as_public_input`)".
Evidence: `AssignedNative::as_public_input` is `vec![*element]` (`types.rs:70-72`) and K is `ListItem(X)` (`zkir-values.k:135`). In this chapter "identity" is also the curve identity (`inf()`, `pt(0, 1)`, the Weierstrass flag). A reader who thinks in group identities can take the sentence as "Native encodes as the identity element".
Fix: "Native encoding is the identity function: `encodeValue(native(X))` is `[X]`, which is `AssignedNative::as_public_input`."

### F8 minor Notes for maintainers / "as the common brief states"
Claim: the midnight-ledger working tree was expected to be at `92e8bdd3` because the common brief says so.
Evidence: `git log -1` on `repos/midnightntwrk/midnight-ledger` is `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`, and `ir_types.rs` there has seven variants with secp256k1 `encoded_len` 8, 4, 4. Those facts are right. The common brief (COMMON.md, "Where things are") tells authors to use `repos/_build/ledger-92e8bdd3/` and says the working trees under `repos/midnightntwrk/` may be at other commits. The note also names "the common brief", which is drafting machinery the published chapter must not mention.
Fix: drop the common-brief sentence. Keep: the definition follows `repos/_build/ledger-92e8bdd3/` (or `git show 92e8bdd3:…`); the `repos/midnightntwrk/midnight-ledger` checkout is `a8ab82ba` and is not that pin.

## Coverage
- The Value sort in `zkir-values.k`: the 13 constructors, what each carries (Int, Bytes, points), typeOf, typeName: covered
- Encoding to raw field elements (`encodeValue`): per type, exact layouts (native, bytes32 as two field elements, points, foreign field limbs: LOG2_BASE and limb counts, the value-minus-one shift for scalars where it applies): covered
- Decoding from raw (`decodeValue`, `decodeStrict`): range checks, `decErr` versus `decPanic` (Bytes32 canonical form), native range check, foreign decoding guards, and the strict re-encode check of the extension: covered (K1 label on JubjubPoint decode is F1)
- Public inputs and `from_public_input`: how a typed input becomes memory and `<pi>` entries; how outputs are encoded at the end (`#encodeAll`): covered
- Error messages: the exact strings and how the differential harness classes them: covered (F3, F4)
- Cross-check table: for each type, encodedLen, K encoder rule, Rust function, and the unit check in `unit_values.py` that covers it: covered
