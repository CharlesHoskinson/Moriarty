# mori-cardano: source-owned local reference APIs

The crate composes actual current Rust Core source parsing and reference semantics.
Its `/1` metadata and projection APIs are UNIT-only; its separate `moriarty/2`
staged API retains the upstream `local-reference-stage` / `none` / `untracked`
qualification. Public metadata, bodies, premises and reference results provide
no authorization, consent, custody, value or ledger evidence.

## Metadata and atomic projection

`inspect_cardano(source, accounts, InspectionTarget::Unit)` calls the crate's
source-owned `export_source`, using current `mori_core::parse` and the existing
canonical Program hash. It preserves the complete Program and exact raw-source
and canonical identities. The export does not require a Core source modification.

Metadata preserves declaration order for assets/actions, bytewise name order for
states/accounts/duties, and full AssetId then account order for framed cells.
Bindings and asset identities are symbolic. Matrix counts and conservative trace
bounds are informational, not measured target resources. `PinnedCardano` and
`require_layout_hash` remain unresolved. See the exact
[metadata/1 scope and F-10 disclosure](INSPECTION-SCOPE-AND-F10-DISCLOSURE.md).

`prepare_projection(raw_source, symbolic_accounts)` constructs a private context
from source, never from a caller Program or public metadata record. Its additive
inspection/2 reports completed-run logical work W under
`moriarty-reference-logical-cost/1`, static KeyHeld from-name sets and unreferenced
bound names. These API version numbers do not denote `moriarty/2` source support.
Metadata/1 requires transfer, fee and duty-party bindings; projection/2 requires
bindings only for transfer and fee parties. “Unreferenced” means absent from every
transfer and fee statement, including a bound name used only as a duty party.

`project_body` and `scenario_from_body` preserve typed state, framed amounts and
absent versus present-zero duties. KeyHeld premises are caller-supplied values.
`project_reference` invokes actual Core `simulate_metered`, with cap W and
completed used W. ProjectionData's Constr/Integer JSON is human notation, not
Plutus Data, CBOR or a wire codec. Read the exact
[projection/2 scope and full F-10 disclosure](PROJECTION-SCOPE-AND-F10-DISCLOSURE.md)
with every citation of either specimen's metadata or results.

## Current staged reference

`prepare_staged_reference(raw_source)` calls actual `mori_core::stage::load` for
`moriarty/2` and retains its private StagedProgram. Metadata preserves complete
Program/StagePlan, raw-source/program/plan identities and the upstream labels.
`start_reference` and `resume_reference` return complete upstream outcomes or
rejections, retaining frames, captures, full execution, ordered effects, duty
presence and attempted/committed work. Await is not flattened into atomic W.

The two chained positive tests pass actual returned continuations to later
resumes after comparing each complete continuation against independent fixtures.
Raw enum JSON equality is an internal assertion alongside fully asserted labelled
metadata; it does not create or qualify a labelled persisted transition record,
target record or codec. Read the exact attributed
[staged F-11/F-12 disclosures](STAGED-SCOPE-AND-F11-F12-DISCLOSURE.md)
with every citation of delivery or reserve staged results. These specimens do
not establish conditional settlement, event authentication or fund protection.

## Actual evidence and open work

On the pinned PR54 candidate, Run A passed 9 tests (3 metadata and 6 projection).
Run B passed 11 tests: those same 9 plus 2 chained staged tests comparing 8 complete
outcomes. There are **11 unique tests**, not 20. Full source hashes, independent
freeze, exact commands/results/logs and public reviews are in the
[curated evidence directory](../../evidence/cardano/current-reference-20261015/REFERENCE-RESULTS.md).
The [source mapping](../../evidence/cardano/current-reference-20261015/SOURCE-MAPPING.json)
distinguishes the base commit/tree and earlier remote feature from the frozen
uncommitted candidate; these results do not imply that this candidate was pushed.

Both runs used the same standalone consumer manifest/lock, Cargo/rustc 1.99.0,
`--offline --locked --lib --no-fail-fast` and separate fresh target directories.
The normal/build dependency tree selected 61 package/version pairs within its
63-package lock, without Git, Midnight, proptest or SDK packages. Locked registry
build scripts/proc-macros ran; no root-workspace build ran. Production libraries,
manifests and locks were identical between A and B. See the preserved exact
[command records](../../evidence/cardano/current-reference-20261015/commands/).

This is partial SP1–5 local reference evidence. NS1–17, NT1–24, SP6 and supplementary
SP7 remain unexecuted; the full Run B gate is open. There is no stage-to-UTXO body
codec, layout/hash qualification, target descriptor/comparator, Aiken build,
proof or ledger/Preprod qualification. No SDK, native, key, wallet or ledger action
was run. The wider goal, all 20 pattern families, general obligations, private PCD,
recursion and all U/M/MC/SP/G exits remain open.

The retained root Cargo.lock member entry changes
`mori_ledger::proving::source_closure()` and declared native build identities.
That identity is unqualified: the separately scoped ledger source-closure check
was not run and remains required before merge. See the
[exact identity disclosure](../../evidence/cardano/current-reference-20261015/NATIVE-IDENTITY-DISCLOSURE.md).
Maintainer review and execution controls do not become public developer compile,
prove or deploy permissions.

## Dated update: source-membership check, 2026-10-15 08:16 UTC

At Run A/B time the separately scoped ledger source-closure selection had not
run. The later unchanged std-only test was compiled directly with rustc (no
Cargo or SDK) and passed **6 source-membership tests**, with all 159 pinned files
unchanged. These six are separate from the eleven reference tests above.
See the [actual receipt and exact command/log records](../../evidence/cardano/current-reference-20261015/ledger-source-membership/RESULT.json)
and [verbatim Opus scope/limitations](../../evidence/cardano/current-reference-20261015/ledger-source-membership/OPUS-EFFECT-REVIEW.md).

The earlier “not run” statements in the A/B-time evidence and identity disclosure
are preserved as history. This update establishes declared-closure membership
selection only, not the closure hash, build authentication, cryptography, SDK,
ledger or native-identity qualification. Declared native identity remains
unqualified. It does not establish merge readiness; the separate UTXO integration
and its unresolved review boundaries are unchanged. The original
PUBLIC-ARTIFACT-PINS.json records the first proposal; use the additive
[revision 2 pins](../../evidence/cardano/current-reference-20261015/PUBLICATION-REVISION-2-PINS.json)
for current publication bytes.
