# Muse independent consequential-design vote — mapped-role source-static design R2

Verdict: **APPROVE** the exact R2 design (R2 plus the immutable R1 it incorporates).
This approves the implementation direction only. It approves no code, no
execution result, and no native, proof, or ledger predicate. Root adoption is
still required before shared implementation, and a fresh Muse audit of the
full implemented candidate and its actual QA results remains mandatory.

Requested reviewer: `muse-wonder-1.0`, high effort. Returned identity: as
recorded by host telemetry (this review asserts no model identity beyond that
record). Native authority: **0**. Method: read-only inspection of the tracked
production source and frozen public references, plus SHA-256 hashing. No source
edits (except this review), no tests, builds, provider/network calls, native SDK
query/check/model/`k`/proof/key/SRS/cache access, credentials, private
reasoning, session logs, subagents, or web access were used.

## Exact candidate identity (verified by direct hash, second route in parentheses)

- `unsigned-multi-payer-static-design-r2.md`:
  `bc72fa30a959e3cb232ff1efd199a2e58affc4893cbedf1d0db2c18e2eb83519` (matches the
  supplied SHA; R2 JSON field `design_sha256` agrees).
- `unsigned-multi-payer-static-design-r2.json`:
  `50fcff4555541f180befe150387efdf60e7037c50561c5a2ac8973ca4eb46903` (matches the
  supplied SHA).
- Incorporated immutable R1 `unsigned-multi-payer-static-design.md`:
  `984a3cfdfd398f857a1431fb559852644e1486ab12c8dd9583f1cda7934a4f88`, matching
  both R2 section text and R2 JSON `supersedes_only_by_addendum.sha256`. R1 JSON
  independently re-hashed: `57da0c16…831ee`.
- Prior review used only as finding background, not as approval:
  `muse-mapped-role-design-review.md` `fcccd4e8…8e2`,
  `.json` `71e2a84f…3d2`, verdict CHANGES_REQUIRED with MUST-1…MUST-6.
- Base: worktree HEAD is `9c4dce4547781293b5cd089f018a63aedad75428` ("Add funded
  FeePool source component and conformance cases"), working tree clean. This
  resolves the stale `dc313458…` head recorded in R1.
- All ten compiler-source hashes in R2 JSON re-computed at this HEAD and
  unchanged: `static_compile.rs 8f2bcd14…`, `layout.rs a0186338…`,
  `relation/mod.rs 4452dbc9…`, `relation/compiler.rs e31e9e72…`,
  `local_deployment.rs 7ef140bd…`, `custody.rs 62cc6936…`, `prepare.rs
  e85449c4…`, `profile.rs 8bde32c8…`, `signed/mod.rs a198909d…`,
  `compile_call.rs 4f8a92cf…` (full digests in the companion JSON).

## Security architecture re-derived from full current source (not diff-only)

- The failure R1/R2 address is real and source-wide.
  `static_compile::compile` maps `Unsigned` to `None` and calls
  `local_deployment::compile_all_policy` (`static_compile.rs:273-280`), whose
  unsigned branch calls `relation::compile_bindings` →
  `layout::validate_static` (`layout.rs:620-644`) → `admit_bindings`
  (`layout.rs:140-264`), refusing any Transfer/Fee with `from != payer` as
  `one-public-payer` (`layout.rs:197-201`).
- The funded sources are mixed by design and must not be rewritten:
  `fee-pool.mori` (23 actions, payer `fee-pool`, nine mapped roles) transfers
  `funding-source → fee-pool` at intake (line 171) and `fee-pool → beneficiary`
  on claims; `contrasting.mori` (4 actions, payer `alternate-pool`) has the
  same shape. Bindings re-read directly confirm these payer/role sets.
- Native custody is single-owner and must keep the guard.
  `layout::attach_request` (`layout.rs:534-591`) requires every selected spend
  to be owned by `roles[payer]` and sets the native caller to that payer;
  `custody::prepare_offer` (`custody.rs:26-176`) funds payouts only from the
  selected inputs and returns all change to the first selected owner;
  `validate_offer` (`custody.rs:180-266`) independently reconstructs that exact
  single-payer change shape. No per-effect debit ownership exists.
- The unsigned emitter proves no debit custody: `relation/compiler.rs:316-387`
  (`claim`) constrains only asset/token, recipient address, and amount;
  `from` is recorded only in signed wire descriptors/payments
  (`compiler.rs:553-576`, recorded only when `signed.is_some()`). A mapped-role
  export may therefore claim source arithmetic, state/duty operations, and
  recipient claims only — never debit custody, ownership, authorization, or
  enforced economic bounds. R2's distinct profiles, qualifications
  (`source-static-mapped-roles-unproven-uninstalled-unauthenticated`,
  relation `source-static-mapped-debits-unauthenticated`), and absent
  predicates state exactly that.
- Every native/signed consumer re-derives strict bindings before any native
  work, so a distinct source-static type cannot be promoted through existing
  paths: `relation::compile`/`admit` (`relation/mod.rs:86-95,192-212`),
  `SignedStaticBindings::from_source` sole-payer/sole-debtor checks plus
  `validate_static` (`signed/mod.rs:203-220,263-309`),
  `prepare_native_call` strict compile at `prepare.rs:271` before any decode,
  `prepare_admitted_signed` strict `admit_bindings` recheck (`prepare.rs:455`),
  `admit_local_seed` `admit_source` at `local_deployment.rs:34` before
  `decode_native` at line 42, installed keyset preflight recompilation
  (`local_deployment.rs:417-461`), and `proving::generate_call_keys` strict
  compile (`proving.rs:327`) before `ir.k()` (line 340). Strict signatures and
  semantics stay unchanged under R2.

## The six R1 required changes, as resolved by R2

1. **MUST-1 (CLI/consumer) — resolved.** R2 §1 allows `unsigned-mapped-roles`
   only with `--all-actions`, static bindings and work limits, exactly one
   out/inspect destination, and no action, deployment, or signer policy, with
   bad shapes rejected before reads (current `validate_shape`,
   `compile_call.rs:84-119`, already runs before any read and hardcodes the
   legacy qualification at line 226 that R2 replaces). `run_static` must
   exhaustively select the enum and account the exact mode byte count (current
   code hardcodes `Unsigned`/`"unsigned".len()` at `compile_call.rs:165-183`;
   R2 forbids that downgrade). `StaticAuthorization::qualification()` returns
   the legacy qualification for unsigned/signed-v2 and the mapped
   qualification for mapped mode; the export line is `{q} {n} actions into
   {path}; identity {id}` (identical to today's legacy line under the legacy
   `q`), legacy inspect is unchanged, and mapped inspect appends
   `; qualification {q}` derived from the caller's validated mode, never from
   imported marker claims. Both FeePool programs' positive export/reopen tests
   and the README commands migrate to mapped mode with the new
   profile/qualification asserted; a legacy-unsigned exact `one-public-payer`
   refusal with no output directory is added; the existing valid signed-v2
   refusal (`fee_pool_component.rs:612-656`, which correctly reaches
   `one-public-payer` before any signed sole-payer statement check) is
   retained; the funded financial funding/custody roles are preserved.
2. **MUST-2 (inventory/error order) — resolved.** R2 §2 pins `debit_sources` to
   only the deduplicated full `RoleBinding` rows of that action's Transfer/Fee
   `from` roles, sorted by role name, computed from the validated Program,
   always serialized (empty array when an action has no payments); recipients
   remain committed through the full ABI/actions (`LedgerAbi.actions`,
   `layout.rs:259`). Validator order is preserved exactly as in current
   `admit_bindings`: Core validation, toolchain/limits/profile, role
   key/address uniqueness, payer role, asset/token mappings, declared-asset
   mapping, duties, then source-order statements. Verified in source: an
   undeclared statement asset fails in Core validation
   (`mori-core/src/runtime.rs`, `validate_asset` called for Transfer/Fee,
   mapped to `source-profile/core-validation` at `layout.rs:141`) before any
   role check; a declared-but-unmapped asset fails
   `identity/source-asset` (`layout.rs:171-173`) before duties/statements;
   within mapped payment statements a missing `from` fails first as
   `source-profile/source-debit-role` and a missing `to` second as
   `source-profile/source-recipient-role`. The strict branch keeps today's
   combined `one-public-payer` precedence unchanged, and combined asset/role
   fault tests plus both-missing-roles tests are required.
3. **MUST-3 (compiler_hash coverage) — resolved.** R2 §3 pins the new
   manifest `compiler_hash` to exactly `proving::source_closure()`. I verified
   that function (`proving.rs:150-217`) is a pure `codec::hash` over
   `include_str!` text with no SDK invocation, and that its closure already
   names every output-affecting file the design touches: `layout.rs`,
   `relation/{mod,compiler,arithmetic,event,process}.rs`,
   `static_compile.rs`, `call_artifacts.rs`, `profile.rs`, `custody.rs`,
   `prepare.rs`, `local_deployment.rs`, the signed modules, `proving.rs`
   itself, the CLI files including `compile_call.rs`, `Cargo.toml`/`Cargo.lock`,
   and the `mori-core` sources including `runtime.rs`/`ir.rs`/`json.rs`. This
   matches the signed-manifest precedent (`signed/context.rs:206`). Legacy
   hash rules (the `relation/mod.rs:117-126` concat for legacy artifacts) are
   preserved, code must stay in covered files or the closure must be extended,
   and stale-build invalidation plus fresh exact-byte reopen checks remain.
4. **MUST-4 (visibility/parser/compile-fail) — resolved.** R2 §4 pins
   `validate_source_static_roles`, `ValidatedSourceStaticBindings`,
   `compile_source_static_roles`, and `SourceStaticArtifact` as `pub(crate)`
   with private fields (mirroring `ValidatedStaticBindings`,
   `layout.rs:604-619`), strict entry signatures unchanged, and no
   `Deserialize`/`From`/`Deref` bridge into `relation::Artifact`,
   `AdmittedSeed`, or any installed/proving capability, and no public raw
   validated constructor or relaxed native flag. The custom
   `StaticAuthorization` decoder (`static_compile.rs:52-74`) gains only the
   length-1 `mode=unsigned-mapped-roles` arm; extra fields, any
   `signer_policy`, unknown modes, and nulls still refuse, and duplicate keys
   refuse through the strict decoder (`mori-core/src/json.rs` rejects
   duplicate decoded keys before schema decoding). Compile-fail locations are
   named: privacy doctests on the new types in `layout.rs`/`relation/mod.rs`,
   and public no-conversion doctests on `StaticBundle` in `static_compile.rs`
   targeting `Artifact`, `AdmittedSeed`, and `DerivedLocalDeployment`,
   asserting that private literal construction and assignment/`Into`
   conversions fail.
5. **MUST-5 (native early-refusal fixtures) — resolved.** R2 §5 follows the
   coherent pattern at `tests/duty_reads_source.rs:751-773`: build the public
   Deployment through the helper on a single-payer variant (the helper itself
   calls `admit_source` and would panic on a mixed source,
   `duty_reads_source.rs:13-117`), overwrite only `source_hash`/`program_hash`
   for the real mixed source, and leave the variant ABI hash in place because
   the strict guard precedes any ABI-hash comparison. I verified the guard
   does precede it on every named path: `relation::compile` reaches
   `admit_bindings` before `validate_static`'s ABI-identity comparison,
   `relation::admit` compiles before comparing caller bytes,
   `admit_local_seed` admits at line 34 before `decode_native` at line 42, and
   `prepare_native_call` compiles at `prepare.rs:271` as its first operation.
   Each test must assert the exact `one-public-payer` code for each selected
   action with a valid public request, empty predecessor/parameter slices, and
   no output, which proves termination before native decoding, VM/query,
   SDK check, `k`, key, proof, SRS, and cache work. The private
   `compile_all_policy` refusal unit test in `local_deployment.rs` never calls
   `derive_all`; signed checks and the full-escrow refusal are kept.
6. **MUST-6 (candidate identity) — resolved.** R2 records base
   `9c4dce4547781293b5cd089f018a63aedad75428` and all ten compiler hashes,
   which I re-verified byte-for-byte at that HEAD (see above).

## Why relabel, downgrade, and native promotion fail under this design

The bundle marker carries no new field; expected profile, qualification, and
authorization are derived from the independently supplied caller
authorization, and reopen validates the caller-chosen validator before any
imported read, then requires exact profile/qualification/authorization,
source/program/ABI/template/build identities, action inventory/order, leaf
set/lengths/hashes, and byte-for-byte fresh recompile equality before any
generic tagged IR decode (the current `reopen` discipline at
`static_compile.rs:377-507`). A tampered `debit_sources`, a mode/profile
downgrade, or an upgraded qualification with attacker-recomputed checksums
therefore still fails fresh equality. The mapped relation format
(`moriarty-mapped-role-source-static-relation/1`) is disjoint from the legacy
format, `SourceStaticArtifact` is disjoint from `relation::Artifact`, and
every native consumer recompiles strictly, so no mapped artifact can enter
`Vec<relation::Artifact>`, an admitted seed, or key/proving derivation.
Two different debit roles may yield identical recipient-claim IR but must
yield different manifest/bundle identities, because the manifest commits the
full ABI and the per-action `debit_sources` inventory.

## Test and ownership obligations carried into implementation

R2 §6 assigns the compiler implementer `layout.rs`,
`relation/{compiler,mod}.rs`, `static_compile.rs`,
`commands/compile_call.rs`, `tests/static_compile.rs`, the CLI
`check_static_compile_io.rs`, and the private `compile_all_policy` refusal
unit test in `local_deployment.rs`; the consumer owner updates
`crates/mori/tests/fee_pool_component.rs` and the FeePool README only after
interface freeze. Financial `.mori` sources, bindings, and Quint are
unchanged. Required coverage remains the R1 set as clarified: the generic
mixed-source program (cross-action and mixed same-action, including Fee)
exporting and reopening under mapped mode while legacy unsigned and signed-v2
refuse; the actual FeePool 23-action and contrasting 4-action positives;
missing debit/recipient role, duplicate role, aliased addresses, bad
key/address derivation, unmapped asset, and mismatched duty identity
refusals; stale-ABI debit-role mutation refusal and fresh-ABI identity change
with reopen refusal against the original inputs; exact profile/qualification/
debit-inventory/open-predicate assertions; hostile IR length and lowered
reader-cap tests; CLI shape refusals before any source/output read; and the
native early-refusal set above. All checks are public-data only, with no SDK
query/check/model/`k`/proof/key work.

## Non-blocking observations

- R2's line citations drift slightly from current source (for example the
  declared-asset check is at `layout.rs:171-173`, not 175-177; the Core
  Transfer/Fee asset validation sits in the same cited region of
  `runtime.rs`). No substance depends on these numbers.
- `SourceStaticArtifact` will need crate-internal accessors (or manifest-byte
  parsing) so the `static_compile.rs` helper can enforce the repeated action,
  public-input/operation, leaf, and aggregate bounds and fill marker action
  rows. Private fields do not preclude `pub(crate)` methods; the implementing
  candidate must not widen them to public access, and the fresh candidate
  audit should check this.
- Compile-fail doctests on `pub(crate)` types prove inaccessibility only in
  combination with the private-field assertions; the candidate audit should
  confirm the doctests assert literal construction/conversion failures rather
  than merely failing to name the types.

## Bottom line

R2 resolves all six R1 required changes without widening scope, weakening
the native single-owner custody guard, or altering unsigned/signed/native
qualification, error-order, schema, or IR-shape semantics apart from the
natural build/compiler-hash invalidation. The full roadmap, M11, native
proof, and ledger settlement remain open, and native authority remains zero.

**APPROVE exact R2** (`bc72fa30…519` plus incorporated R1 `984a3cfd…a4f88`)
for root adoption. Implementation, its actual results, and any native claim
require the later fresh full-candidate Muse audit.
