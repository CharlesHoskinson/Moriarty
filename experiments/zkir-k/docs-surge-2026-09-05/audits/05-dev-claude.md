# Audit of experiments/zkir-k/docs/05-fields-curves-and-hashes.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed:
- Ran both commands of section "How the primitives are checked" from the repository root (`uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py`, `.../unit_hash.py`): output `42/42 checks passed` and `18/18 checks passed`, identical to `evidence/zkir-k-unit-values-2026-09-05b.txt` and `evidence/zkir-k-unit-hash-2026-09-05b.txt`.
- Compared the eight moduli of the table with `semantics/zkir-field.k` lines 25-32 (identical) and, in Python, with the hex values quoted from midnight-curves `bls12_381/fq.rs` line 3 and `jubjub/fr.rs` line 3, with `2^256 - 2^32 - 977`, `2^255 - 19`, `2^252 + 27742317777372353535851937790883648493`, and with the SEC 2 / FIPS 186-4 hex constants; all eight pass a Fermat primality test.
- Compared every function name and arity in the chapter with `zkir-field.k`, `zkir-curves.k`, `zkir-hash.k`, `zkir-constants.k`, `zkir-sha512-constants.k` and `zkir-ext.k` lines 106-158 (`ZKIR-SHA512`), `zkir-vm.k` line 412 (`#stdHash3`), `zkir-ops.k` lines 165-173 (`fromCoordinatesV`, `#ptToValue`), `zkir-test.k` lines 8-15 (imports). All exist with the stated arity and attributes.
- Recomputed in Python: Jubjub `d = -(10240/10241) mod r` equals `EDWARDS_D` of midnight-curves 0.3.1 `jubjub/curve.rs` lines 384-389 (limbs assembled little-endian); Curve25519 `d = -(121665/121666) mod p` equals `CURVE_D` of `curve25519/curve.rs` lines 199-204; both `d` are non-residues (Legendre symbol `P - 1`).
- Recomputed in Python: the Jubjub curve generator of the chapter equals `JubjubAffine::generator()` (`jubjub/curve.rs` lines 1373-1390); it is on the curve; `8 G` (also computed as three doublings, matching `mul_by_cofactor` at line 707-709) is `(28336281903124990867587793011069573392383982287722241916350956173377953689573, 39385640392217313770878525135509063452020585410343666726093009378539878503883)`; `rJ * 8G = (0, 1)` and `rJ * G != (0, 1)`. `JubjubSubgroup::generator()` is `JubjubExtended::generator().clear_cofactor()` (lines 1319-1321, 1337-1339).
- Recomputed in Python: the Curve25519 generator is on the curve, has `y = 4/5`, and `L * B = (0, 1)`; midnight-curves 0.3.1 `Cargo.toml` lines 91-92 pins `curve25519-dalek = "4.1.3"`, and that crate is in the registry. The secp256k1 and secp256r1 generators equal the SEC 2 / FIPS 186-4 hex values and satisfy their curve equations; the secp256r1 `b` equals `0x5ac635d8...d2604b` (also quoted in midnight-curves `p256/mod.rs` line 58).
- Recomputed in Python: `#svdwZ`, `#svdwA`, `#svdwB`, `#montJ`, `#montK` equal the limbs of midnight-circuits 7.2.4 `ecc/hash_to_curve/mtc_params.rs`; `#svdwZ = r - 2`; `#montJ = 40962`; the `c3` radicand `-g(Z)(3Z^2 + 4A)` is a residue; `3Z^2 + 4A` is non-zero; `J = 2(a + d)/(a - d)` and `K = 4/(a - d)` hold for `a = -1`. The 2-adicity of `r - 1` is 32 and the smallest non-residue at or above 2 is 5.
- Regenerated `zkir-constants.k` with `python3 experiments/zkir-k/tools/gen_constants.py <midnight-circuits-7.2.4/src>` and diffed against the checked-in file: identical (204 round constants, 9 MDS entries, five SVDW/Montgomery constants). Confirmed `blstrs.rs` header cites `generate_parameters_grain.sage 1 0 255 3 8 60 0x73ed...0001`; `constants/mod.rs` lines 20-26 give `RATE = 2`, `NB_FULL_ROUNDS = 8`, `NB_PARTIAL_ROUNDS = 60`; `poseidon_cpu.rs` line 36 gives `NB_SKIPS_CPU = 2`, lines 63-70 use a zero triple for the last round, lines 131-133 set the capacity to `1 << 64` for `init(None)`, lines 165-167 push the length as padding, lines 148-179 absorb in chunks of `RATE`.
- Compared the SVDW, Weierstrass-to-Montgomery and Montgomery-to-Edwards steps with `ecc/hash_to_curve/mtc_cpu.rs` lines 33-40 and 108-165 (sign by `is_odd`, `inv0`, `y' = 1` when `tv2 = 0`, cofactor clearing) and the K rules in `zkir-hash.k` lines 105-143.
- Checked SHA-256 `H_0[0] = 0x6a09e667`, SHA-512 `K[0] = 0x428a2f98d728ae22`, `H_0[0] = 0x6a09e667f3bcc908`, the 24 Keccak round constants and the 25 rotation offsets against the standard values; checked the `rotr64` call sites in `zkir-ext.k` lines 135-141 (shifts 1, 8, 19, 61, 28, 34, 39, 14, 18, 41).
- Checked the Rust references: `ct_quadratic_non_residue` (`ff_ext/mod.rs` line 22, `legendre == -1`), `Curve25519Affine::from_xy` (`curve25519/affine.rs` line 89), `K256Affine::from_xy` (`k256/curve.rs` line 105, SEC 1 uncompressed encoding), `affine_from_xy` for P-256 (`p256/mod.rs` line 48, SEC 1 uncompressed encoding), Jubjub `from_xy` in midnight-circuits `ecc/curves.rs` lines 120-138, the ZIP 216 check in `jubjub/curve.rs` lines 500-516.
- Checked file existence: all K files, `tools/gen_constants.py`, `tools/unit_values.py`, `tools/unit_hash.py`, `corpus/handmade/transient_hash.zkir`, `corpus/handmade/std_hashes.zkir`, `corpus/midnight-zkir-2ffe2d1-tests/test_sha512_proof.zkir`, both receipts, every Rust path in the closing paragraph. Chapters 06, 09, 12 and 13 referenced by the chapter are not yet present under `experiments/zkir-k/docs/` (only 01 to 05 exist); they are in the planned set, so not reported.
- Checked the divergence cross-reference: the parity-only `from_coordinates` divergence is row `wiki/contradictions.md` line 46 (case `k01`), consistent with the K1 description.
- Lint: exactly one `#` title, no em-dash, no TODO/TBD/placeholder, no mention of drafting or reviewing. Word count outside tables and code blocks is 2133 (brief: about 1500).

## Findings

### F1 major "Totality and out-of-domain results", paragraph "`fsqrt` and `legendre` do not have an `[owise]` clause", and "Notes for maintainers"
Claim: `fsqrt` and `legendre` lack an `[owise]` clause, "for the eight configured primes they always reduce", "they are not used with `P <= 0`"; the note says that for `P <= 0` the remainder "is not defined in `INT`" and expects the `P > 1` / `[owise]` convention for both.
Evidence: `zkir-field.k` line 68: `rule legendre(A, P) => fpow(A, (P -Int 1) /Int 2, P)`. `fpow` has an `[owise] => 0` rule (line 62), so `legendre` is defined for every `P` (it returns `0` whenever `P <= 1`) and needs no clause of its own; the same holds for `isQuadraticNonResidue` (line 70). For `fsqrt` (lines 81-84) the guards evaluate `A modInt P`; `modInt` is defined for every non-zero divisor (for `P = 1` the result is `sqrtOk(0)`; for `P < 0` the Legendre symbol is `0`, so the result is `noSqrt()`). The only modulus outside the definition is `P = 0`.
Fix: replace the paragraph by: "`legendre` and `isQuadraticNonResidue` have no `[owise]` clause and need none: they reduce to `fpow`, whose `[owise]` rule makes them `0` (respectively `false`) for `P <= 1`. `fsqrt` has no `[owise]` clause either; its guards evaluate `A modInt P`, so it reduces for every non-zero modulus (`sqrtOk(0)` for `P = 1`, `noSqrt()` for `P < 0`) and is undefined only for `P = 0`, a value no caller passes." Rewrite the maintainers note to name `P = 0` only, and drop `legendre` from it.

### F2 minor "Bit helpers": "Each requires a non-negative width or index and a non-negative `A`; the `[owise]` default is ... `false` for `fitsBits`"
Claim: `fitsBits` requires `A >= 0` as a side condition and falls to `[owise]` otherwise.
Evidence: `zkir-field.k` line 130: `rule fitsBits(A, N) => A >=Int 0 andBool A <Int (1 <<Int N) requires N >=Int 0`. Only the width is a side condition; a negative `A` gives `false` through the formula itself. The result is the same, but the sentence and the table row ("width `>= 0`") disagree with each other.
Fix: "`bitAt`, `lowBits` and `highBits` require a non-negative `A` and a non-negative index or width and return `0` otherwise (`[owise]`). `fitsBits` requires only `N >= 0` (`false` otherwise); a negative `A` is simply not in `[0, 2^N)`."

### F3 minor "Poseidon", paragraph "`absorbAll` adds the input queue in pairs"
Claim: the absorb procedure and the initial state are described; the empty input is not stated.
Evidence: `zkir-hash.k` line 61: `rule absorbAll(S, .List) => S`, so `poseidonHash(.List)` is `#ps0(ps(0, 0, 0)) = 0` with no permutation. The crate does the same (`poseidon_cpu.rs` lines 171-176 iterate over `queue.chunks(RATE)` of an empty queue and return `register[0]`), and `unit_hash.py` line 98 checks it (`poseidon []`). A reader who wants the algorithm stated precisely will otherwise assume at least one permutation.
Fix: add after the `poseidonHash` sentence: "With an empty input no permutation runs and `poseidonHash(.List)` is `0`, as in the crate; `unit_hash.py` checks this case against the oracle."

### F4 minor "How the primitives are checked", list of `unit_values.py` cases and "The remaining checks are encodings"
Claim: the listed field and curve cases plus the encoding checks account for the 42 checks.
Evidence: `tools/unit_values.py` lines 168-197 contain 23 field and curve checks; the chapter's list covers 19. Not listed: `curve25519 B on curve` (line 182), `curve25519 B in subgroup` (line 183), `secp256k1 G on curve` (line 185), `secp256r1 G on curve` (line 188). These are not encodings, so "the remaining checks are encodings" (19 checks, lines 200-224) is off by four. Also, in `unit_hash.py`, the check `sha256 bytes(32) alignment vs hashlib` (line 116) compares the oracle's register with `hashlib`, not K; the chapter's "SHA-256 also with `hashlib`" reads as two K-versus-hashlib checks.
Fix: change "the Jubjub curve generator on the curve" to "each of the four generators on its curve, and the Curve25519 generator in its subgroup", and change "SHA-256 also with `hashlib`" to "the 1-byte SHA-256 also with `hashlib`, and the oracle's 32-byte SHA-256 register with `hashlib`" (or drop the second).

### F5 minor "Totality and out-of-domain results", table row "`finv` / `fdiv` | non-zero divisor | `0` (Fermat of zero)"
Claim: the zero result for a zero divisor comes from Fermat exponentiation of zero.
Evidence: `zkir-field.k` line 50: `rule finv(A, P) => 0 requires P >Int 1 andBool A modInt P ==Int 0` is an explicit rule; the Fermat rule (line 51) applies only to non-zero `A`.
Fix: replace "(Fermat of zero)" by "(explicit rule, `zkir-field.k` line 50)".

### F6 minor "How the primitives are checked", the two commands
Claim: the two commands are all a developer needs.
Evidence: `tools/unit_values.py` line 20 and `tools/unit_hash.py` line 25 require the compiled `semantics/zkir-test-kompiled/`; `tools/unit_hash.py` line 27 hard-codes the oracle at `Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'`, a path under the home directory rather than the repository root. Neither prerequisite, nor what a failure looks like (a `FAIL` line with expected and actual text, exit status 1, `check` at lines 160-164 and 86-89), is stated, and the chapter does not point to the tooling chapter for them.
Fix: add one sentence: "Both tools need the compiled `ZKIR-TEST` definition (`semantics/zkir-test-kompiled/`); `unit_hash.py` also needs the oracle binary at `~/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle`. A mismatch prints a `FAIL` line with the expected and actual terms and the exit status is 1. See `11-tooling-reference.md`."

### F7 minor Length
Claim: the brief asks for about 1500 words outside tables and code blocks.
Evidence: `sed '/^```/,/^```/d; /^|/d' experiments/zkir-k/docs/05-fields-curves-and-hashes.md | wc -w` gives 2133.
Fix: optional; the surplus is mostly in "Hash to curve" and the two hash sections, which could defer the byte-level SHA and Keccak detail to a sentence each.

## Coverage
- `zkir-field.k`: eight moduli with names and origins, arithmetic totality convention (`P > 1`, `[owise] => 0`), Legendre symbol, Tonelli-Shanks (`#twoAdicity`, `#oddPart`, `#nonResidue`, `#ts` loop), bit helpers: covered (F1, F2 correct precision).
- `zkir-curves.k`: `pt`/`inf`, descriptors with Edwards `d` for Jubjub and Curve25519 and Weierstrass parameters for secp256k1 and secp256r1, addition, doubling, negation, `ecMul`, subgroup checks, `fromXY`, Jubjub decompression by parity of `x`, Jubjub generator as eight times the curve generator; constants verified against midnight-curves 0.3.1: covered.
- `zkir-hash.k` and `zkir-constants.k`: Poseidon (t = 3, R_F = 8, R_P = 60, constants origin), fixed and variable length sponges, `transientCommit`, `hashToCurve` with the SVDW parameters, SHA-256, Keccak-256, SHA-512 in `zkir-ext.k` and `zkir-sha512-constants.k`: covered (F3 adds the empty-input case).
- What each function is checked against (`unit_values.py`, `unit_hash.py`) and where the Rust reference lives: covered (F4, F6 refine).
- Precision statements: which functions are total, which inputs are outside the domain and what they return: covered (F1, F5 correct two rows).
