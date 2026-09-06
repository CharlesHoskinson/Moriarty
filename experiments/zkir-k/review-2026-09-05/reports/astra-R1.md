VERDICT: WARNING — The reviewed arithmetic and hash checks matched, but the semantics admits noncanonical native inputs, conflates Rust panics with recoverable errors, and narrows two decoder APIs.

## 1. Findings

Paths beginning `experiments/` are relative to the repository root. Source abbreviations:

- `Ledger/` = `/home/charl/Moriarty/repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/`
- `Circuits/` = `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/`
- `Curves/` = `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/`

### Major — The native-value boundary admits integers outside the Rust field

**Locations:** `experiments/zkir-k/semantics/zkir-values.k:152`, `experiments/zkir-k/tools/zkir_run.py:56`, `experiments/zkir-k/tools/zkir_run.py:60`, `experiments/zkir-k/semantics/zkir-vm.k:87`.

**Repository observation:** `decodeValue([X], native())` returns `decOk(native(X))` without checking `0 ≤ X < #r`. The Python preimage adapter constructs arbitrary K integers for inputs, binding input, commitments and transcripts. VM initialization supplies no missing field-domain check.

**Source fact:** Rust’s decoder receives actual field elements, not arbitrary integers: `Ledger/ir_instructions/encode.rs:143` and `Circuits/utils/types.rs:74`. The matching oracle’s decimal adapter explicitly rejects noncanonical representations at `/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:52`. Its underlying `Fr::from_le_bytes` uses checked `from_repr` at `/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/transient-crypto/src/curve.rs:346`.

**Experiment observation:** A program declaring one `Scalar<BLS12-381>` input, no outputs and no instructions, with raw input

```text
52435875175126190479447740508185965837690552500527637822603658699938581184513
```

finished in K with `status = ok`, `%x = native(#r)` and a holding binding-input verdict. The Rust oracle exited 1 with “is not a canonical field element”.

**Why it matters:** This is a syntactically accepted preimage JSON outside the Rust `Fr` domain. K manufactures an impossible native value; arithmetic subsequently reduces it while encoding preserves it (`zkir-values.k:133`, `zkir-ops.k:36`). Thus acceptance and value reasoning depend on an unenforced representation invariant.

**Concrete fix — recommendation:** Validate every preimage field integer against `[0,#r)` before constructing or executing a job, including all transcript and commitment fields. Add the corresponding K entry predicate so direct K callers cannot bypass it. Guard native decoding as well. Reject noncanonical representations rather than silently reducing them.

### Major — Bytes32 decoder panics are modeled as recoverable errors

**Locations:** `experiments/zkir-k/semantics/zkir-values.k:155`, `experiments/zkir-k/semantics/zkir-values.k:160`, `experiments/zkir-k/tools/unit_values.py:223`.

**Repository observation:** Oversized Bytes32 components return `decErr("Bytes32 decoding assertion failed")`. VM input loading converts this into ordinary `error(...)` at `zkir-vm.k:107`; execution can continue through error-handling rules and verdict generation. The unit test explicitly expects that abstraction.

**Source fact:** `Ledger/ir_instructions/encode.rs:155` asserts that the low component’s final byte is zero. Lines 157–159 assert that all but the first byte of the high component are zero. These are unconditional Rust assertions, not returned decoding errors.

**Experiment observation:** Using the same minimal program shape with one `Bytes<32>` input:

| Raw inputs | K outcome | Rust oracle outcome |
|---|---|---|
| `[2^248, 0]` | Ordinary decoding error | Exit 101; panic at `encode.rs:155` |
| `[0, 256]` | Ordinary decoding error | Exit 101; panic at `encode.rs:159` |

All four component integers are canonical native field elements. These preimages reach the decoder through the normal Rust input boundary.

**Why it matters:** The model loses the distinction between a returned `Err` and unwinding/aborting witness computation. Consequently, it cannot support exact failure-behavior or panic-freedom claims. This is a deliberate modeling choice in the comment, but it does not implement the requested Rust behavior exactly.

**Concrete fix — recommendation:** Introduce a distinct panic outcome and propagate it through input and transcript decoding. Preserve the distinction in the runner and differential comparator. Replace the current expected ordinary error with separate tests for both assertion sites. If panic/error abstraction is intentional, specify that observation relation explicitly and qualify correspondence claims accordingly.

### Minor — `decodeValue` does not match the underlying decoders on irregular lengths

**Locations:** `experiments/zkir-k/semantics/zkir-values.k:177`, `experiments/zkir-k/semantics/zkir-values.k:199`, `experiments/zkir-k/semantics/zkir-values.k:200`.

**Repository observation:** K requires exactly one element for Jubjub scalars and exactly five for either Weierstrass point.

**Source facts:**

- `Circuits/ecc/native/edwards_chip.rs:153` rejects scalar lists only when their length exceeds two. It validates each element’s 254-bit bound, concatenates the bits, and takes the first 252. Therefore `[]` and `[0,0]` decode to scalar zero. Empty bits produce zero through `Circuits/circuit_field.rs:145`.
- `Circuits/ecc/foreign/weierstrass_chip.rs:249` returns identity whenever the last supplied element is one, **before** checking length. Consequently `[1]` decodes to identity for both secp curves.

**Experiment observation:** Direct K calls rejected all four examples: Jubjub scalar `[]`, Jubjub scalar `[0,0]`, and `[1]` for each secp point. The corresponding Rust outcomes above follow from source inspection; I did not execute those Rust helpers directly.

**Why it matters:** The exported helper is narrower than `decode_offcircuit`/`from_public_input`. Normal VM execution does not expose these particular differences: raw-input and transcript decoding first slice the declared encoded width (`Ledger/ir_vm.rs:201`, `Ledger/ir_vm.rs:362`, `Ledger/ir_vm.rs:382`). This finding concerns helper-level correspondence and future callers, not demonstrated rejection of a valid fixed-width VM preimage.

**Concrete fix — recommendation:** Either reproduce these permissive helper behaviors and leave width enforcement to callers, or explicitly define `decodeValue` as a fixed-width wrapper and restrict its correspondence statement to lists of `encodedLen(T)` elements.

## 2. Coverage gaps

**Repository observations:**

- `unit_values.py:167` exercises very few square-root inputs. It lacks a matrix covering zero, modulus boundaries, residues and nonresidues across all eight fields.
- Group tests at `unit_values.py:174` concentrate on generators and scalar multiples. There are no explicit identity-multiplication tests for every curve, ZIP 216 zero-coordinate cases, or direct tests of Weierstrass doubling with `y = 0`. The latter requires a synthetic curve because the two configured secp groups have odd prime order.
- Encoding tests at `unit_values.py:198` do not provide direct encode/decode cases for every type. In particular, they omit secp256r1 value encodings, secp256k1 scalar encoding, and Curve25519 base encoding.
- Random inputs from `zkir_values.py:106` are predominantly canonical encodings; point generation excludes identity. The existing foreign noncanonical divergence case does not cover the full matrix of batch overflow, nonzero tail limbs, and reduced aliases for both limb layouts.
- `unit_hash.py:92` uses one small input triple. Hash-to-curve is tested at lengths 0, 1 and 3; there is no length-2 case in that file.
- `unit_hash.py:104` lacks empty-message and padding-boundary SHA/Keccak cases. Its 232-byte multiblock case tests Keccak, not SHA-256. The SHA-512 corpus example uses a one-byte message (`corpus/midnight-zkir-2ffe2d1-tests/test_sha512_proof.zkir:22`).
- SVDW exceptional intermediate values and all candidate-selection branches are not explicitly targeted. Random hash-to-curve outputs are unlikely to exercise exceptional denominators.

**Experiment observation:** This review exercised several of these missing boundaries successfully, as listed below. Those checks were run in memory and are not retained regression tests.

## 3. Questions for the authors

1. Is correspondence intended to preserve Rust’s distinction between returned errors and panics? If both map to one abstract failure, where is that observation relation specified?
2. Are correspondence claims for `decodeValue` restricted to fixed-width lists, and are claims for arithmetic restricted to canonical values over the eight configured prime fields? The exported K signatures and `[total]` attributes do not express those restrictions.

## 4. What I checked and how

### Source and constant checks

**Repository/source observations:** I read the six assigned K files, `gen_constants.py`, `unit_values.py`, `unit_hash.py`, and `zkir_values.py`; inspected the relevant input adapter, VM initialization and arithmetic callers; queried `wiki/index.md`; and read the schema, design records and relevant receipts. I inspected the pinned Rust encoding code, field/point `Instantiable` implementations, curve adapters, Poseidon CPU and round-skip implementation, SVDW parameters and CPU map, and hash-to-curve sponge calls. I also inspected the relevant spec passages and Agda trust-base declarations.

**Experiment observations:**

- Regenerated `zkir-constants.k` into an in-memory string using the supplied generator against the installed 7.2.4 source. The entire output matched: **204 round constants, 9 MDS entries, and 5 SVDW/Montgomery parameters**.
- Independently decoded Rust modulus limbs and compared the native and Jubjub scalar moduli. Checked the remaining moduli against the underlying k256, p256 and curve25519-dalek sources.
- Decoded both Edwards `d` constants from Rust limbs and checked the K rational expressions. Both `d` values were nonsquares; `−1` was square in both fields.
- Checked Jubjub’s full generator coordinates and cofactor-clearing definition against `Curves/jubjub/curve.rs:1373` and `:1319`; reconstructed the Curve25519 generator from dalek’s 51-bit limbs; compared the secp generators and P-256 coefficient with their underlying crate definitions.
- Compared all 64 SHA-256 and 80 SHA-512 round constants with installed `sha2-0.10.9/src/consts.rs`.

Representative constant locators include `zkir-constants.k:7` versus `Circuits/hash/poseidon/constants/blstrs.rs:30`, MDS at `zkir-constants.k:213` versus the Rust matrix at `blstrs.rs:1403`, and SVDW parameters at `zkir-constants.k:224` versus `Circuits/ecc/hash_to_curve/mtc_params.rs:82`.

### Formula inspection

**Inferences from source inspection:**

- Edwards addition is complete on the intended on-curve domains: `a = −1` is square and `d` is nonsquare. The unrestricted generic `Curve` constructor has a broader domain.
- Weierstrass doubling explicitly handles `y = 0`; multiplication initializes its accumulator with the curve-specific identity.
- Tonelli–Shanks follows the standard decreasing-order loop for the configured odd primes. This is not a claim for arbitrary integer moduli.
- Jubjub decompression uses the same recovered-root parity and ZIP 216 rejection as `Curves/jubjub/curve.rs:478`.
- Foreign decoding matches batch bounds, zero tail limbs and final modular reduction. The one-element Jubjub scalar case correctly checks 254 bits and retains 252 bits.
- K’s plain Poseidon rounds match the raw round schedule. Rust’s round-skip construction composes three partial rounds when `NB_SKIPS_CPU = 2`; “paired partial rounds” in the design record is inaccurate terminology. The fixed-length empty hash returns zero in both implementations. Variable-length hashing appends the input length and takes registers zero and one for the two squeezes, matching `poseidon_cpu.rs:149` and `htc_gadget.rs:80`.

### Executed checks and limitations

The prescribed command failed before executing tests:

```text
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
```

`uv` could not create its lock temporary file. Direct execution through `.venv/bin/python` also failed because `pyk` required a writable temporary directory.

I then used the existing compiled interpreters through pipes, without creating files:

```text
experiments/zkir-k/semantics/<definition>-kompiled/interpreter /dev/stdin -1 /dev/stdout
```

Python constructed KORE using `KRun.kast_to_kore`; function checks used a `<k>` cell containing the requested term. Complete jobs used `definition.init_config(KSort("GeneratedTopCell"))` with `$PGM` substituted. Rust preimages were supplied through stdin, and synthetic program JSON through an inherited `os.pipe()` descriptor. All Python invocations used `PYTHONDONTWRITEBYTECODE=1`.

**Experiment observations:**

- Existing value suite: **41/41 passed**, with only execution transport replaced.
- Existing hash suite: **18/18 passed**, with execution and oracle temporary-file transport replaced.
- SHA-256 matched `hashlib` for lengths **0, 55, 56, 63, 64, 65, 127, 128, 129, 232**.
- SHA-512 matched `hashlib` for lengths **0, 111, 112, 127, 128, 129, 255, 256, 257**.
- Keccak-256 matched Rust for lengths **0, 134, 135, 136, 137, 271, 272**. Byte messages used `message[i] = i mod 256`.
- Poseidon and hash-to-curve matched Rust at lengths **0, 1, 2, 3, 4, 5, 8, 9, 16**, using `input[i] = (#r − 1 − i²) mod #r`.
- Square-root checks passed for **12 inputs per modulus**: `0,1,4,p−1,p,p+1,−1` and five pseudorandom inputs from `random.Random(19)`. Returned roots were squared; nonresidue results were checked with Euler’s criterion.
- Both limb layouts matched Rust on a modulus-valued reconstructed integer, batch overflow and tail overflow.
- Jubjub scalar inputs `0`, `2^252`, `3·2^252+5`, and `2^254` matched Rust.
- Jubjub inputs `[0,1]`, `[1,1]`, `[0,#r−1]`, and `[1,#r−1]` matched Rust.
- Identity multiplication by `0`, `1`, and `13` passed for all four curves. Doubling `(0,0)` on the synthetic curve `y²=x³+x` over `F₅` returned infinity.

The first two findings used this complete program shape, substituting the stated type and decimal-string inputs:

```json
{
  "version": {"major": 3, "minor": 0},
  "inputs": [{"name": "%x", "type": "Scalar<BLS12-381>"}],
  "outputs": [],
  "do_communications_commitment": false,
  "instructions": []
}
```

Preimages supplied `binding_input: "0"` and empty transcripts. No files were modified, no definitions were rebuilt, and no exhaustive or formal equivalence proof was performed.