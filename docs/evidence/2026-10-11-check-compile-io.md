# Bounded check and complete-file compile publication — author result

This local candidate repairs the existing check/compile I/O obligation 184 from
baseline `bf9b3da160ae6f9ca5474bdfe6b0f71a7796293e`. Fresh independent source and
actual-result audits remain required. This report supplies author evidence and
claims no M1/M10, U0–U7/M1–M17, financial, formal, native or ledger gate closure.

`check` now acquires the full source through the existing bounded reader. Its
1,048,577-byte read window refuses inputs above 1 MiB without parsing; the Core
and beta grammars retain their separate 65,536-byte source limit. Invalid UTF-8,
including a multibyte character cut by that window, retains the existing read
error. No truncation, parser/profile/schema or source hash change was made.

An additional actual C16 failure exposed raw bidi/newline filename characters in
beta's human diagnostic source header. Root selected the small caller repair:
apply existing `display::text` to the human NamedSource name. That display policy
escapes all non-ASCII characters as well as controls. Ordinary ASCII names retain
their diagnostic bytes; filesystem Paths, raw source/spans/hashes and decoded
JSON file identities remain exact. The original independent C16 baseline
assumption was too strong; the failed capture remains in the record.

Compile retains parse, strict bounded Target read, Target-selected compiler,
exact UTF-8 pretty manifest serialization and new-directory reservation order.
A private publication caller uses the existing ordinary `publish_new` helper
for every raw ZKIR leaf, followed by `artifact.json` last. Complete-file hard-link
publication refuses existing/raced files, symlinks (including dangling ones) and
directories. It emits no Compiled status on failure. Normal-owned staging files
are cleaned; reserved directories and earlier complete circuits can remain.
Third-party leaves and additions are preserved. Circuit visibility is individual;
the final marker does not authenticate subsequent edits or a third-party marker.
This contract assumes stable output directories/ancestors, relies on filesystem
hardlink support and claims neither whole-directory atomic visibility nor
power-loss durability or hostile ancestor replacement defense.

The compiler and shared proof publisher are unchanged. No Core ASCII exporter,
1MiB manifest cap, new newline, artifact rebinding, dependency or native operation
was introduced. The artifact_bytes fixed point remains the compiler's exact
pretty UTF-8 byte count under its Target cap (default 16 MiB).

## Executed evidence

Create-once receipts live under the repository runtime
`.moriarty-dev/runtime/check-compile-20261011/`; the final external candidate
report pins their complete public file manifest, frozen binary and source tree.
All Cargo build/test/check commands used `RUSTUP_TOOLCHAIN=1.99.0`, `--locked
--offline` and the existing readiness target directory. No toolchain installation,
parameter lookup, keygen, prove/native verify, MockProver, ignored/native suite,
network transaction, push or merge occurred.

| Receipt | Actual outcome |
|---|---|
| baseline-replay | Existing production replay CLI 5/5 before edits |
| frozen-consumers | New actual binary suite 14/14; 80 raw CLI captures in frozen-cli |
| final-binary-units | 7/7, including full publication caller 3 and existing two-writer helper/snapshots |
| final-cli-replay | Existing production replay CLI 5/5 on final production code |
| backend-replay | Safe reference replay 11/11; net_floor/liability_cap authority edits added |
| legacy-compile | Existing signed/unsigned source-wide compile dispatch test 1/1; exact filter avoids proof controls |
| final-cargo-check | mori build check exit 0 |
| frozen-fmt / frozen-diff | Formatting and diff checks exit 0 |

Exact comparisons cover all beta report fields against existing snapshots, full
Core actions/source hashes, repeated contrasting signed/unsigned compile bundles,
typed manifest reserialization, every raw circuit byte, counts/order and absence
of staging leftovers. Ordinary failed commands have empty stdout; their existing
stderr command/read/write diagnostics and absence of success status are checked.
Unix file-size limits inject real circuit and manifest write failures through the
production binary. The host ran permission tests as uid 1000, without the root skip.

The exact envelope-cap fixture has anchor `limits-2`, 27 instructions and a 5,262-byte
manifest under a 5,262-byte Target cap. Its adjacent 5,261-byte Target refuses. Target
fields participate in context hashes and decimal ZKIR lengths, so assigning an
old artifact's count to a changed Target is not a monotonic boundary calibration.
The test finds a deterministic exact-cap fixture through the unchanged compiler,
then compares actual serialized CLI bytes. Summed-ZKIR refusal is tested below
its threshold; an exact summed-ZKIR threshold is not claimed. A contrasting large
fixture produces 200 circuits / 8,200 instructions and an exact 1,099,050-byte raw
UTF-8 manifest under the original 16 MiB cap, with no final newline.

## Independent expectation mapping and remaining limits

The 38 independent cases were specified expectations, not execution receipts.
This candidate demonstrates the following variants; grouped rows do not imply
that every platform/fault variant in the independent report was executed.

| Cases | Executed checks / retained limits |
|---|---|
| C01-C06 | Core report/human status; three exact beta snapshots; malformed escape after comments; wrong Core/beta profiles |
| C07-C11 | Both grammars at 65,536/65,537/80,000/1,048,576 bytes; reader refusal at 1,048,577. No huge/sparse memory measurement |
| C12-C14 | Invalid UTF-8, absent/directory/unreadable inputs; multibyte cut by reader; UTF-8 grammar boundary/hash |
| C15 | Core small-file token/string/depth/declaration refusals; no expanded beta resource corpus |
| C16-C18 | Hostile Unicode/control/bidi filenames with JSON/human/color/verbosity; exact JSON identity; normal input symlink |
| P01-P04 | Two independently authored multi-action programs, signed/unsigned selection, repeated new outputs, exact typed bytes and large UTF-8 manifest |
| P05-P07 | Exact envelope/instruction acceptance/refusal; summed-ZKIR refusal; stale ledger/ZKIR/network, null/unknown/duplicate Target keys; parse and unselected signed-debtor refusal. Unsupported profile spelling not separately executed |
| P08-P10 | Existing empty/nonempty dirs, source/Target files, parents, valid/dangling directory symlinks, spaced/Unicode output; missing/file/unwritable parents. Special output path variants not executed |
| P11-P12 | Barrier-started actual same/different-program CLI reservations; one complete unmixed winner |
| P13-P14 | Explicitly seeded leaves after reservation at complete production publication caller boundary: first/second/manifest files; second/manifest symlink/dangling/directory/directory-symlink; originals remain unchanged |
| P15-P16 | Real circuit/manifest file-size write failures; complete earlier files, no partial final marker/leaf, no success status, normal staging cleanup; raced-leaf hardlink refusal. Stage collisions, sync/disk-full/unsupported-filesystem failures not injected |
| P17-P20 | Unrelated seeded user addition survives second-leaf failure; manifest absent on circuit failure and final on success; ASCII legal action order/filenames; input-first diagnostic priority. Late-after-first user addition schedule and exact maximum-declaration compile fixture not executed |

Retained red receipts: red-check/red-consumers demonstrate parser JSON above the
reader bound and a partial circuit; red-leaves demonstrate file/symlink clobber;
red-manifest demonstrates a partial artifact.json under old truncating writes.
Its temporary old-publication source and restored repaired source are separately
pinned. green-consumers-v2/final and exact-cap-calibrated/final-consumers-v3/v4/v5
retain test calibration failures (Target-dependent sizes and rendered line
wrapping), alongside the C16 product failure. No original failed receipt was
rewritten. frozen-consumers has zero current unresolved test failures.

Plugin status reports no pending actual transactions and retains stale SP01
admission/resource/accounting stops. Native bound, full fraud-game/deploy/DUST,
source/5 and every broader U/M obligation remain owed.
