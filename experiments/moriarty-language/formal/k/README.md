# Bounded funded repayment in K

The installed K LLVM backend executed all 16 frozen cases of the
`moriarty-funded-repayment/0` projection on September 9, 2026. Every complete
result matched the independent expectation and real `.mori` source preparation
result. See [raw evidence and retained failures](../../../../deliverables/bounded-k-2026-09-09/README.md).
This finite comparison does not establish full Core semantics, a correspondence
theorem, ledger acceptance or SP03 completion. No proof was attempted.

## Admission and semantics

`codec.admit(text)` validates at most 65,536 UTF-8 bytes of JSON with closed record
shapes and UInt128 decimal strings. Duplicate JSON keys reject. Unlike the retained
kernel's compact-string entry point, the codec accepts whitespace and JSON escape
variants; the compared domain is the parsed projection. It accepts exactly two input
balance rows, one allowance, one obligation, empty used-ID lists, and exactly Transfer
then Repay. Identity conversion is exactly mantissa 1, scale 0, rounding none;
allocation is AccrualFirst or PrincipalFirst. Other shapes are `UNSUPPORTED_PROJECTION`;
malformed fields are `MALFORMED_INPUT`. Neither is a K financial rejection.
Direct raw K terms are outside this codec-admitted claim domain; the host supplies
lexical UInt128 input checks while K owns financial invariants and arithmetic.

Actual identifiers and amounts are passed into K. The host does not require the
transfer's parties, asset or IDs to match the obligation or Repay. K checks duplicate
balance pairs, state invariants, work, Transfer, and Repay in retained-kernel order.
K's arbitrary-precision Int operations are constrained by UInt128 guards on state
sums and Transfer additions; checked subtraction follows sufficient-balance/funding
and outstanding guards. Identity conversion makes settlement equal nominal payment.
The allocation rules use guarded subtraction and minInt. No division or ProRata is
admitted. The full identity-conversion financial path is in `moriarty.k`.

Transfer amount and nominal repayment are independent. K resolves sender and receiver
indices from the actual two input rows, retaining row order. A missing sender rejects;
a missing receiver can append a third balance, with its amount computed in K. No
host success filter hides missing balance/allowance, wrong party/recipient/asset/ID,
zero, overpayment, insufficient funding, invalid outstanding/status or overflow.
Third-party repayment and receiver creation are now exercised in the separate
[16-case branch suite](../../../../deliverables/repayment-k-branches-2026-09-09/README.md); the original suite remains unchanged.

Each stage checks one condition and stops at the first failure. Failure clears the
continuation and exposes only code and index. State validation precedes work, then
Transfer (index 0), then Repay (index 1). Null index is encoded as -1 internally.
The only success rule emits every changed financial amount: existing and appended
balances, allowance remaining/spent, work remaining/spent, principal/accrued/
outstanding, status, settlement and both discharge components. K returns a receiver
index to identify an appended row. Input SHA-256 is echoed in both output variants.

## Codec boundary

`codec.encode(packet)` creates the K input term. `codec.decode(kastJson, packet)`
requires the pinned KAST JSON version 4 format observed from K 7.1.337, with
structured KLabel/KSort records, empty type parameters, exact integer arities,
a digest-bound result and bounded numbers. Other KAST versions reject.
It reconstructs unchanged metadata, ordered Transfer/Repayment effects, and tombstone
appends from the input. It never computes changed financial values or substitutes a
TypeScript result. Malformed, extra, stale or mismatched output fails closed. The
codec and selected KAST schema remain trusted interface code, not verified codecs.
`fixtures/observed-kast-v4-principal-partial.json` preserves the exact first successful
krun output from attempt 02. Offline decoding checks it against the independent full
principal-partial expectation; this regression does not invoke K or establish the
remaining 15 traces.
Source typing and nominal-unit witnesses are outside this lowered projection.

## Commands and limits

Run from this directory, inside the reviewed whole-tree systemd user scope with
`MemoryMax=4G`, `MemorySwapMax=0`, and the aggregate runtime limit:

```sh
python3 run.py compile-and-traces --all
```

This command has one monotonic 512-second deadline including wrapper overhead,
one LLVM compile attempt at the compiler’s default optimization settings, capped at 180 seconds,
and at most 16 krun attempts,
each capped at 20 seconds and clamped to remaining time. Subprocesses run sequentially;
a timed-out process group is killed and reaped before exit. Root supplies whole-tree
memory containment. Do not run concurrent wrappers in the same build directory.

Separate commands are `compile`, `evaluate INPUT.json`, and `traces --all`. Use the
combined command for the initial aggregate budget; starting separate processes resets
the per-process deadline and is not authorized as a way to extend the initial run.
`prove` explicitly rejects as unimplemented. There are no compile retries, automatic
compilation, backend fallbacks, network calls or proof attempts. The persistent compile
attempt and krun counter retain failed attempts. Deleting them requires a separately
reviewed execution decision; it is not a retry mechanism.

The pinned Nix store path, revision, NAR hash and executable digests are in
`toolchain.lock.json`. After compilation, `.build/binding.json` binds source/codec/
runner/toolchain hashes and every compiled file hash. Execution rejects missing or
changed artifacts. Raw stdout/stderr, exact command arrays, exit codes and elapsed
seconds are retained in `.build`. Individual results and `observations.json` retain
full results keyed by fixture ID. No observed K result is claimed until these actual
commands run successfully and independent reviews check their bytes and outputs.

## Initial checks and remaining coverage

`fixtures/cases.json` contains exactly 16 static expectations: six prior independent
complete positive results and ten hand-specified financial failures. The positive
cases cover principal partial, interest-first, principal-first, full repayment,
cross-interest-boundary and cash 40/debt 30. Failures cover cash, allowance, ordinary
work despite reserve, receiver overflow, outstanding invariant plus insufficient
work (precedence), missing transfer ID, insufficient unallocated funding, payer
mismatch, zero Transfer and excessive nominal payment. They are not generated by K
or TypeScript. Root compares them separately through the source/reference path.

Local codec checks use `python3 -m unittest discover -s . -p test_codec.py` and do
not invoke K. The separately bounded `branches` suite adds five successful and eleven rejecting
traces covering missing receiver creation, swapped rows, third-party repayment,
PrincipalFirst crossing, missing sender/allowance, self-transfer, duplicate rows,
wrong obligation/recipient/asset, zero Repay, settled status and two additional
multiply-invalid cases. See the [branch evidence](../../../../deliverables/repayment-k-branches-2026-09-09/README.md).
Neither suite is exhaustive: further invalid state-sum/status combinations and
all general-domain metatheorems remain open. Each selected suite still has the
same sixteen-call ceiling; new execution requires a separate bounded allocation.
The default command selects the initial suite; choose the additional suite with
`run.py --suite branches compile-and-traces --all` inside its admitted containment.
