Cannot write `experiments/zkir-k/docs-surge-2026-09-05/audits/05-fm-codex.md`: this session has a read-only filesystem. Report follows.

# Audit of 05-fields-curves-and-hashes.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared field, curve, hash and extension descriptions against the K rules.
- Regenerated constants in memory and confirmed exact equality with `zkir-constants.k`.
- Checked Jubjub constants, generator and decompression against the pinned Rust dependencies.
- Independently recomputed generator subgroup relationships and Edwards non-residuosity.
- Compared test coverage with both unit scripts and recorded receipts.
- Attempted both documented commands; uv failed before testing because its cache is read-only.
- Confirmed one chapter title and no prohibited placeholders, drafting terms or em-dashes.

## Findings

### F1 blocking Incorrect totality diagnosis

Claim: Chapter lines 300–306 state that `fsqrt` and `legendre` are not total for zero or negative moduli because `modInt` is undefined there.

Evidence: **Repository observation:** `experiments/zkir-k/semantics/zkir-field.k:68` defines `legendre` through `fpow`. The guard at line 53 and fallback at line 62 make `legendre(A, P)` return `0` for every `P <= 1`. It does not evaluate `A modInt P` on that branch.

K’s installed `include/kframework/builtin/domains.md:1394` gives the definedness condition for `modInt` as a **nonzero** divisor. Negative divisors are permitted. Consequently, the rules at `zkir-field.k:81`–84 yield `sqrtOk(0)` for negative moduli dividing `A`, and `noSqrt()` otherwise.

Fix: **Recommendation:** Separate the two functions. State that `legendre` returns zero for `P <= 1`; `fsqrt` is undefined at `P = 0`, returns `sqrtOk(0)` at `P = 1`, and has the negative-modulus behavior above. Restrict the mathematical square-root guarantee to the configured primes.

### F2 major Missing collection-domain restrictions

Claim: Chapter lines 233–254 present totality and off-domain behavior without explaining the collection-shape assumptions of the named hash functions.

Evidence: **Repository observation:** `experiments/zkir-k/semantics/zkir-hash.k:60`–63 requires integer elements when absorbing a nonempty list, with no fallback for other element sorts. Thus a well-sorted K term such as `poseidonHash(ListItem(true))` has no complete evaluation path under these rules.

Similarly, `keccakF` at lines 254–258 reaches `lane` at lines 228–229, which indexes the supplied list and casts entries to `Int`. `keccakF(.List)` cannot produce the documented permutation result. The `[total]` attribute does not supply missing cases.

Fix: **Recommendation:** State that the Poseidon interfaces require lists of integers, and `keccakF` requires a state of 25 integer lanes. Distinguish declared `[total]` attributes from established totality on argument sorts. Explain that malformed collections lack defined fallback results.

## Coverage

- Field moduli, arithmetic, Tonelli–Shanks and bit helpers: covered; totality correction required.
- Curve representations, operations, constructors, generators and subgroup checks: covered.
- Poseidon, sponges, commitments, hash-to-curve and byte hashes: covered.
- Unit checks and Rust references: covered; recorded passes inspected, fresh runs blocked by filesystem permissions.
- Totality and off-domain behavior: partly covered; F1 and F2 require correction.