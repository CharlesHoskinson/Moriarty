# Muse independent consequential-design audit — unsigned mapped-role source-static export

Verdict: **CHANGES_REQUIRED** on the exact proposal as written.
Requested reviewer: `muse-wonder-1.0`, high effort. Actual returned identity is
recorded by host telemetry; this review was produced in a fresh non-author
Muse session. Native authority: **0**. This vote authorizes no source edit,
no test/build execution, and no native/provider work. Root adoption and the
implementation/result audits remain separate. Full M1–M17/U0–U7 remains open.

Reviewed proposal: `unsigned-multi-payer-static-design.md`
SHA-256 verified by direct hash:
`984a3cfdfd398f857a1431fb559852644e1486ab12c8dd9583f1cda7934a4f88` (matches the
supplied value), with its companion `.json` (`sol_vote: APPROVE`). No prior
design approval was reused; this audit re-derived every claim from the current
production call chain. Method was source reads and public-data hashes only —
no tests, builds, native SDK query/check/model/`k`/proof/key/SRS/cache, network,
keys, credentials, or subagents were used.

## Candidate identity observed

The design JSON pins `current_head dc313458…` and ten compiler-source hashes.
During this review HEAD advanced to `9c4dce4547781293b5cd089f018a63aedad75428`
("Add funded FeePool source component and conformance cases", which commits the
previously untracked FeePool consumer with its known 6 PASS / 1 RED state).
I re-hashed all ten pinned files at the new HEAD: every hash in the design JSON
still matches byte-for-byte (`static_compile.rs 8f2bcd14…`, `layout.rs
a0186338…`, `relation/mod.rs 4452dbc9…`, `relation/compiler.rs e31e9e72…`,
`local_deployment.rs 7ef140bd…`, `custody.rs 62cc6936…`, `prepare.rs
e85449c4…`, `profile.rs 8bde32c8…`, `signed/mod.rs a198909d…`,
`compile_call.rs 4f8a92cf…`). The design therefore still describes the exact
current production code, but its recorded `current_head` is now stale and must
be refreshed at adoption.

## Current production facts (verified, not assumed)

- The unsigned all-action failure is real and source-wide. `static_compile::compile`
  (`crates/mori-ledger/src/static_compile.rs:273-280`) maps `Unsigned` to `None`
  and calls `local_deployment::compile_all_policy`, whose unsigned branch
  (`crates/mori-ledger/src/local_deployment.rs:210-214`) calls
  `relation::compile_bindings` → `layout::validate_static`
  (`crates/mori-ledger/src/layout.rs:620-644`) → `admit_bindings`
  (`layout.rs:140-264`), which refuses any Transfer/Fee with `from != payer`
  as `one-public-payer` (`layout.rs:197-201`). The QA record confirms it
  end-to-end: `cli-qa/pre-export-repair` is 6 PASS / 1 RED, the RED being
  `public_source_static_all_actions_and_separate_process_reopen_both_programs`
  failing with `mori::ledger::source-profile: one-public-payer`, and
  `sol-static-refusal-results.json` shows both FeePool programs refused with
  no output directory created. The earlier assumption that the existing
  unsigned mode admitted multi-payer sources is therefore invalid, as stated.
- The real funded source is mixed by design. `examples/core/fee-pool/fee-pool.mori`
  has 23 actions; intake transfers `funding-source → fee-pool` (line 171) and
  every claim transfers `fee-pool → <beneficiary>`, while
  `static-bindings.json` binds `payer: fee-pool` with nine mapped roles.
  `contrasting.mori` has the same shape (`capital-source → alternate-pool`,
  then pool claims). Any single-payer "fix" that rewrote these sources would
  falsify the funded source; the design correctly keeps it.
- Removing the guard would be unsafe, as the design claims. Native custody is
  single-owner: `layout::attach_request` (`layout.rs:534-591`) requires every
  selected spend to be owned by `roles[payer]` and sets the native caller to
  that payer; `custody::prepare_offer` (`crates/mori-ledger/src/custody.rs:26-176`)
  aggregates only the selected inputs and returns all change to the first
  selected owner (lines 57-92); `validate_offer` (`custody.rs:180-266`)
  independently reconstructs that same single-payer change shape
  (lines 232-251). The unsigned relation emitter records `from` only in
  signed wire descriptors (`relation/compiler.rs:553-577`); the actual claim
  constraint (`compiler.rs:316-387`) binds only asset/token, recipient
  address, and amount. Nothing establishes per-debit custody or authorization.
- Every native/signed consumer re-derives strict bindings, so a distinct
  source-static type cannot leak into native authority through existing
  paths: `relation::compile`/`admit` (`relation/mod.rs:86-95,192-212`),
  `signed::compile_signed` (`signed/context.rs:138-141` via
  `SignedStaticBindings::from_source`, `signed/mod.rs:203-220`, including
  `sole-payer` before strict validation),
  `prepare_native_call` (`prepare.rs:263-281`, strict compile at line 271
  before any predecessor decode), `prepare_admitted_signed`
  (`prepare.rs:439-456`, strict `admit_bindings` recheck at line 455),
  `local_deployment::admit_local_seed` (`local_deployment.rs:27-66`,
  `admit_source` at line 34 before `decode_native` at line 42),
  `preflight`/`admit_source` keyset imports (`local_deployment.rs:417-461,539-559`),
  and `proving::generate_call_keys` (`proving.rs:319-344`, strict compile at
  line 327 before `ir.k()` at line 340; the material-deriving helpers at
  lines 346/399 are `pub(crate)`). Existing compiler/build hashes cover the
  files this design edits — `relation::compile_bindings` hashes the relation
  sources (`relation/mod.rs:117-126`) and `proving::source_closure`
  (`proving.rs:150-209`) includes `layout.rs`, `static_compile.rs`, and
  `compile_call.rs` — so the design's "do not freeze hashes; stale receipts
  stay invalid" position is correct and required.

## What the design gets right

The additive shape is the right one: a new explicit
`{"mode":"unsigned-mapped-roles"}` authorization, distinct
`ValidatedSourceStaticBindings` and `SourceStaticArtifact` types with private
fields and no conversion into `relation::Artifact`, `AdmittedSeed`, key, or
proving capabilities; strict `admit_bindings`/`validate_static` retained with
their error order; a new bundle profile
(`moriarty-mapped-role-source-static-bundle/1`), bundle qualification
(`source-static-mapped-roles-unproven-uninstalled-unauthenticated`), relation
format and qualification; explicit absent predicates for debit custody,
authorization, predecessor/deployment, economic authority, intent/history/PCD,
and native proof/settlement; reopen by independently supplied authorization
with fresh byte-for-byte recompile before any tagged IR decode (mirroring
`static_compile.rs:377-507`); and no relaxed parameter on
`compile_all_policy`/`derive_all`. The observation that two different debit
roles can yield identical recipient-claim IR but must yield different
manifest/bundle identities is correct, because the manifest commits the full
ABI (source actions, role keys/addresses, assets, duties) plus
source/program hashes. The proposed native-refusal tests genuinely terminate
before forbidden work under empty/bad input: `relation::compile` refuses in
`admit_source` before IR construction, `admit_local_seed` refuses before
native decoding, `prepare_native_call` refuses at its first statement before
decoding or VM execution, and a crate-internal `compile_all_policy` unit test
never reaches `derive_all`, `k()`, keygen, SRS, or cache. None of the proposed
tests can invoke an SDK query/check/model/proof/key path.

## Required changes before implementation

1. **The CLI surface and the existing FeePool consumer are unspecified, and
   the current CLI text would mislabel the new mode.** `compile_call.rs:226`
   hardcodes the status string `source-static-unproven-uninstalled`, and the
   existing RED consumer asserts exactly that substring on stdout
   (`crates/mori/tests/fee_pool_component.rs:470-476`) while invoking
   `--authorization unsigned` (`fee_pool_component.rs:429-457`); the README
   consumer commands likewise use `unsigned`
   (`examples/core/fee-pool/README.md:86-99`). The design must state
   explicitly that (a) the FeePool consumer and README migrate to
   `--authorization unsigned-mapped-roles`, (b) the export/inspect status
   line is derived from the selected mode's qualification so a mapped-role
   bundle never presents under the legacy unsigned qualification, and
   (c) the legacy `unsigned` expectation for FeePool becomes a *refusal*
   assertion, keeping the current RED test's failure mode covered rather
   than deleted. Without this, the one RED test either stays red or is made
   green under a misleading qualification.
2. **Pin the debit inventory and mapped-mode statement error codes.**
   "Full RoleBinding rows actually used" is ambiguous between debit sources
   only and debit-plus-recipient rows. State that `debit_sources` is the
   deduplicated set of `from` RoleBindings of that action's Transfer/Fee
   statements, sorted by role name, always serialized (empty array for
   actions with no Transfer/Fee), computed from the validated Program; that
   recipients remain committed through the full ABI and action statements;
   and pin deterministic refusal codes for missing debit role
   (`source-debit-role`), missing recipient role, and undeclared/unmapped
   asset in mapped mode, preserving the strict path's statement-check
   position and precedence inside the shared validator.
3. **Pin the new manifest's `compiler_hash` coverage.** Specify a hash that
   commits to every file that can change mapped output — the shared emitter,
   the mapped validator, the inventory computation, and the new manifest
   serialization (for example `proving::source_closure()`, as the signed
   manifest already uses at `signed/context.rs:206`, or an extended
   `include_str!` concat naming the new files). The existing relation concat
   (`relation/mod.rs:117-126`) does not name `layout.rs` or
   `static_compile.rs`, so reusing it unchanged could leave mapped-mode
   identity insensitive to validator changes at the manifest layer.
4. **Pin visibility and deserialization surface.** State that
   `validate_source_static_roles`, `ValidatedSourceStaticBindings`, and
   `compile_source_static_roles` are `pub(crate)` (mirroring
   `layout.rs:604-624`); that the custom `StaticAuthorization` deserializer
   (`static_compile.rs:52-74`) gains exactly the length-1
   `unsigned-mapped-roles` arm and still refuses mapped mode with extra
   fields, unknown modes, and any signer policy; and name where the
   compile-fail coverage lives (doctests on the new capability types, in the
   style of `local_deployment.rs:67-75`) proving no construction of, or
   conversion into, a native artifact/seed/proving capability.
5. **Pin the native early-refusal fixtures.** The `admit_local_seed` and
   `prepare_native_call` tests must build a coherent public Deployment from
   a single-payer variant and overwrite only source/program hashes — the
   existing pattern at `tests/duty_reads_source.rs:751-773` — because the
   shared fixture helper itself calls `admit_source` and panics on a mixed
   source (`duty_reads_source.rs:13-117`). Each refusal test must assert the
   exact `one-public-payer` code (not merely an error), so the test proves
   termination at the strict guard rather than at source-identity or
   fixture construction, with empty predecessor/parameter slices and no
   output artifact created.
6. **Refresh the candidate identity at adoption.** Record the current HEAD
   (`9c4dce45…`, which now commits the FeePool consumer and its 6 PASS /
   1 RED state) instead of `dc313458…`; the ten pinned compiler hashes are
   unchanged, so no design substance changes.

## Non-blocking observations

- `prepare.rs:272` in the design is line 271 in current source; custody line
  ranges are approximate (`prepare_offer` ends at line 176, `validate_offer`
  starts at line 180). No substance depends on these.
- Reusing `StaticBundle`/`ArtifactLeaves` at the bundle layer is acceptable
  because the marker commits mode-specific profile, qualification, and
  authorization and reopen requires exact fresh-byte equality; byte-level
  reuse grants nothing, since every native consumer above recompiles
  strictly before any key, VM, or proof work.
- Supplied `economic_bounds` staying committed-but-inert metadata in mapped
  mode is consistent with the legacy unsigned export and is covered by the
  absent `economic intention authority` predicate; it must not be described
  as enforced anywhere in the new mode's output or docs.

## Bottom line

The security architecture — additive explicit mode, strict native guard
untouched, distinct unconvertible types, fresh-compile reopen, honest
unproven qualifications — is approved in direction. The exact proposal is
not yet implementable without ambiguity at the CLI/consumer surface, the
debit-inventory definition, and manifest hash coverage, so the vote is
CHANGES_REQUIRED with the six bounded amendments above. None of them widens
scope, touches native authority, or alters the financial source; after they
are folded into the design text, this design can proceed to root adoption
and a fresh exact-candidate implementation audit.
