# Unsigned mapped-role source-static export — design proposal

Status: Sol design vote APPROVE; awaiting fresh Muse consequential-design vote and root adoption.
Native authority: zero. This report authorizes no implementation or native work.

## Observed failure and boundary

`static_compile.rs:265–280` maps unsigned authorization to `None` and calls
`local_deployment::compile_all_policy`. Its unsigned branch (`local_deployment.rs:213`)
calls `relation::compile_bindings`, which constructs `layout::ValidatedStaticBindings`
through `validate_static` (`layout.rs:620`). That validator invokes source-wide
`admit_bindings` (`layout.rs:140–264`), requiring every Transfer/Fee `from` to equal
`StaticBindings.payer`, with `one-public-payer` on failure (`layout.rs:197–201`).
Consequently valid source with one funding role and a separate pool debit role
cannot use this export path, even when each individual action has one payer.

The guard must remain in native admission. `layout::attach_request:534–580`
requires selected spends owned by that one payer and sets the native caller to
that payer. `custody::prepare_offer:25–150` aggregates selected funding, groups
outputs by token/recipient and returns change to the first selected owner;
`validate_offer:174–266` independently checks this shape. These routines do not
implement general per-effect debit ownership. `relation/compiler.rs:316–385`
binds recipient claim keys; Transfer/Fee handling at 553–576 records `from` in
signed observation labels/payments, but unsigned claim constraints do not
establish the funding role's custody or authorization. Removing admission's
payer check would give unsupported debit semantics to native consumers.

`prepare_admitted_signed` rechecks strict admission before simulation
(`prepare.rs:455`). Installed/native and signed import/preflight paths compile
against strict bindings again; signed construction additionally checks owner,
sole payer and sole debtor (`signed/mod.rs:204–300`). These boundaries remain.

## Proposed additive interface

Add explicit `StaticAuthorization::UnsignedMappedRoles`, serialized exactly as
`{"mode":"unsigned-mapped-roles"}`. Existing `--authorization unsigned` and
`--authorization signed-v2` remain unchanged. The existing all-actions CLI's
`--authorization` string accepts the new spelling; it needs no new command or
flag. Extend `validate_shape` at `compile_call.rs:98–105` so the new spelling
requires `--all-actions`, static bindings, work limits, exactly one output/inspect
destination, no action/deployment and no signer policy. Preserve all existing
shape refusals. In `run_static:165–182`, select the enum and account the exact
authorization spelling length; the current policy-absent branch hardcodes
Unsigned and must become an exhaustive match, never a default/downgrade.
FeePool's unsigned example chooses this mode explicitly. This general
mode is available for any Core-valid fixed-role program, without FeePool names,
financial changes, synthetic funding or fabricated ownership.

The new mode means: compile source arithmetic, state/duty operations and
recipient claims with an exact public source/role mapping, with no debit
custody or owner authorization established. Every Transfer/Fee source and
recipient must name an independently validated mapped role, and every asset
must be declared and mapped. `payer` remains a validated mapped role for the
existing bindings schema and immutable provenance; in this mode it does not
assert that all source debits belong to that role. Role keys and addresses are
public identities, not evidence of control. Supplied economic bounds remain
committed input metadata; this unsigned export supplies no economic authority.

## Concrete implementation shape

1. In `layout.rs`, retain strict `admit_bindings` and `validate_static`, their
   private-field `ValidatedStaticBindings` constructor, error order and native
   semantics. Factor only their shared checks into a private implementation
   parameterized by a private explicit debit rule. Strict entry uses the current
   `from == payer` rule and current error. A separate
   `validate_source_static_roles` entry uses `roles.contains(from)` and
   `roles.contains(to)` and returns a new private-field
   `ValidatedSourceStaticBindings`. Both retain exact source hash/parse/Program,
   ABI/template hash, toolchain/limit, role key-to-address derivation, role and
   address uniqueness, asset/token uniqueness, duty identity, state-key and work
   checks. Missing debit-role is a specific `source-debit-role` refusal, with no
   error-triggered fallback to relaxed validation.
2. In `relation/compiler.rs`, retain typed strict `compile` and `compile_signed`
   entry signatures. Add `compile_source_static_roles`, accepting only the new
   validated type and always using unsigned compilation. Share the existing
   emitter behind a private module helper; no public raw Program/bindings or
   `allow_multiple_payers` boolean. The new path cannot pass signed context or
   construct `ValidatedStaticBindings`.
3. In `relation/mod.rs`, add private-constructor `SourceStaticArtifact` with
   private IR/manifest and bounded byte accessors only. Its distinct manifest
   commits source/program hashes, full ABI (including exact action source roles,
   role keys/addresses, assets and duties), action, input count/groups, IR hash,
   compiler hash, qualification/open predicates, and `debit_sources`: the sorted,
   deduplicated full RoleBinding rows actually used by that action's Transfer/Fee
   statements. Compute this inventory from source, never caller claims. It is
   disjoint from `relation::Artifact`; implement no From/Deref/deserialization
   bridge into native artifacts. The shared emitter's recipient arithmetic is
   unchanged. Source-role identity is committed by the full manifest/bundle;
   do not claim new IR constraints prove debit ownership. Two different debit
   roles can produce identical recipient-claim IR but must produce different
   source/manifest/bundle identities.
4. In `static_compile.rs`, route only the new authorization variant through a
   private all-actions source-static helper. Repeat existing action-count,
   unique-name, public-input/operation, leaf and aggregate bounds for the distinct
   artifact. Existing modes keep `local_deployment::compile_all_policy`; do not
   add a relaxed parameter or mode to that function or `derive_all`. The new
   artifacts never enter `Vec<relation::Artifact>` or native derivation.

## Immutable artifact identities and reopen

New mode bundle profile: `moriarty-mapped-role-source-static-bundle/1`.
New mode bundle qualification:
`source-static-mapped-roles-unproven-uninstalled-unauthenticated`.
New mode relation format: `moriarty-mapped-role-source-static-relation/1`.
New relation qualification: `source-static-mapped-debits-unauthenticated`.
Keep `static-compile.json` and existing numbered leaf filenames; retain strict
JSON parsing, bounded/no-follow reads and exact leaf-set publication rules.
Legacy profiles, qualifications, schema fields and canonical serialization
remain unchanged for unsigned/signed-v2.

Extend the existing marker with no new field: derive the expected exact profile
and qualification from independently supplied authorization, never imported
marker claims. New relation manifests carry explicit absent predicates:
`debit-source custody and funding ownership`, `role and beneficiary authorization`,
`authenticated predecessor and deployment`, `economic intention authority`,
`final intent/history/PCD`, `native proof and ledger settlement`. No success or
support field upgrades these conditions.

`reopen` validates independent source/program/bindings with the validator chosen
by caller authorization before imported reads. It requires exact expected
profile/qualification/authorization, source/program/ABI/template/build identities,
action inventory/order, marker identity, leaves/lengths/hashes and directory set.
It then recompiles through the same explicit authorization path and requires
byte-for-byte marker and leaf equality before any generic tagged IR decode.
A relabelled source-static manifest or recomputed attacker checksum cannot
substitute for that fresh compile. Native `relation::admit` and installed
keyset imports keep strict recompilation and their existing profiles.

Source edits necessarily change existing compiler/build hashes because
`relation::compile_bindings` hashes included compiler files and
`proving::source_closure` includes layout/relation/static/CLI source. Preserve
that invalidation: do not freeze old hashes or make stale receipts appear current.
The required compatibility is unchanged native/signed IR structure, schema,
qualification, admission and serialization rules under the current build;
old build-dependent identity bytes cannot literally remain equal after edits.

## Required tests — public data, no SDK checks

Use existing public role point fixtures in `tests/static_compile.rs:592–695`;
no signing keys, predecessor frames, parameter/SRS files, native execution,
proof, degree discovery, SDK check/model or network operations.

- General source: one action Alice→Bob, another Bob→Treasury (and a separate
  mixed-source same-action case including Fee). New mode exports every action
  and reopens exactly with valid role/asset/duty maps; default unsigned and
  signed-v2 still refuse. FeePool exports/reopens all 23 actions with its actual
  funding-source→pool intake and pool→beneficiary claims unchanged.
- Missing source role, missing recipient, duplicate role, aliased addresses,
  bad key/address derivation, unmapped asset and mismatched duty identity refuse.
  Mutate a debit role while retaining stale ABI identity: refuse. Change a
  debit role with a fresh coherent ABI: new manifest/bundle identity; reopening
  against original independent inputs refuses even if recipient IR is equal.
- Exact new profile/qualification/debit inventory and open predicates asserted;
  a tampered debit inventory, mode/profile downgrade or upgraded qualification
  with attacker-recomputed hashes still fails fresh equality. Keep hostile IR
  length tests and lower reader caps, including rejection before generic decode.
- CLI new mode rejects missing `--all-actions` or required static flags and any
  signer policy before source/output reads. Exact mode is required for reopen;
  default/unknown/downgraded authorization cannot choose it.
- Native public refusal: `relation::compile` and `relation::admit` on the mixed
  source refuse `one-public-payer` for every selected action, including later
  payout actions. `local_deployment::admit_local_seed` on the same public
  descriptor with empty predecessor/parameter slices refuses the source-wide
  guard before attempting native decoding. Also invoke `prepare_native_call`
  with a public request and empty predecessor/parameter slices: its first
  operation is strict `relation::compile` (`prepare.rs:272`), so refusal must
  precede native decoding/VM execution. Existing full-escrow refusal
  (`tests/duty_reads_source.rs:751`) remains. Unit-test private native
  `compile_all_policy` refusal on mixed source without calling `derive_all`.
- Signed public construction/compile remains refused under its existing
  sole-owner policy; existing accepted single-payer unsigned and signed-v2
  structural/serialization tests remain unchanged. Add compile-fail API coverage
  that a source-static bundle cannot be used as an admitted seed, native Artifact
  or installed/proving capability, with no conversions exposed.

No tests were executed for this design. Implementation requires root adoption
and a fresh Muse design vote, then actual bounded source-static checks and a
fresh audit of the full changed candidate and results. Full M11, authenticated
multi-owner native custody and FeePool ledger withdrawal remain open.

## Sol substantive design vote

APPROVE this additive design. The distinct explicit mode solves the observed
source-static restriction without weakening the native one-owner custody
contract. Separate validated and artifact types, exhaustive CLI routing and
fresh exact-byte reopen protect the boundary. Existing native/signed behavior
and qualifications remain; build hashes must invalidate normally. This vote
approves the proposed implementation direction only, not unexecuted code,
source results, native execution or financial acceptance.
