# Mapped-role source-static design R2

Sol vote APPROVE exact R2; awaiting Muse/root adoption. Native authority 0.
No execution authority. Incorporates immutable
R1 unsigned-multi-payer-static-design.md, SHA256
984a3cfdfd398f857a1431fb559852644e1486ab12c8dd9583f1cda7934a4f88.
Amendments override R1 ambiguity. Base
9c4dce4547781293b5cd089f018a63aedad75428; all ten R1 compiler hashes match.

1. CLI/consumers. Add unsigned-mapped-roles only with --all-actions,
   static-bindings/work-limits, exactly one out/inspect; forbid action,
   deployment and signer-policy. Reject bad shape before reads. run_static
   exhaustively selects enum and exact mode byte count, without downgrade.
   StaticAuthorization::qualification() returns legacy
   source-static-unproven-uninstalled for unsigned/signed-v2 and
   source-static-mapped-roles-unproven-uninstalled-unauthenticated for mapped.
   Export status: `{q} {n} actions into {path}; identity {id}`.
   Old inspect: `source-static-recompile-admitted {n} actions; identity {id}`.
   Mapped inspect appends `; qualification {q}`; q comes from validated mode. Migrate BOTH FeePool programs' positive export/reopen
   tests and README commands to mapped mode; assert new profile/qualification.
   Add legacy unsigned exact one-public-payer refusal with no output directory;
   retain valid signed-v2 refusal. Preserve financial funding/custody roles.

2. Inventory/errors. debit_sources is ONLY deduplicated from RoleBinding rows
   of that action's Transfer/Fee statements, sorted by role name, computed from
   validated Program, ALWAYS serialized (empty array without payments).
   Recipients remain committed in full ABI/actions. Preserve validator order:
   Core, toolchain/limits/profile, roles/key-address uniqueness, payer role,
   asset/token mappings, declared-asset mapping, duties, source-order statements.
   Undeclared asset fails early source-profile/core-validation (Core
   runtime.rs:263-278); declared but unmapped asset fails identity/source-asset
   (layout.rs:175-177). Neither is masked by role errors. Within mapped payment
   statements, missing from first: source-profile/source-debit-role; missing to
   second: source-profile/source-recipient-role. Strict branch/error precedence
   remains unchanged. Test combined asset/role faults and both missing roles.

3. Identity. New manifest compiler_hash is EXACTLY proving::source_closure(),
   pure text hashing; no SDK. Covers layout validation,
   emitter, inventory, serializer and CLI. Keep code in already covered files;
   extend closure for any new output-affecting file. Preserve legacy hash rules
   and normal stale-build invalidation. Reopen retains fresh exact-byte checks.

4. Types/parser. validate_source_static_roles, ValidatedSourceStaticBindings,
   compile_source_static_roles and SourceStaticArtifact are pub(crate), fields
   private; no Deserialize/From/Deref to native; strict signatures unchanged. Custom StaticAuthorization decoder
   adds ONLY length-1 object mode=unsigned-mapped-roles; extra fields,
   signer_policy, unknown modes, nulls and duplicate keys refuse. Privacy
   compile-fail doctests belong on new types in layout.rs/relation/mod.rs;
   public no-conversion doctests on StaticBundle in static_compile.rs target
   Artifact, AdmittedSeed and DerivedLocalDeployment. Assert private literal
   construction and assignment/Into conversions fail. No public raw validated
   constructor or relaxed native flag.

5. Native fixtures. Follow duty_reads_source.rs:751-773: build coherent public
   Deployment through helper on a single-payer variant; overwrite ONLY
   source_hash/program_hash for real mixed source. Never call helper on mixed
   source. Variant ABI hash remains: strict guard precedes ABI hash comparison.
   relation::compile/admit, admit_local_seed and prepare_native_call must assert
   EXACT one-public-payer for each selected action, with valid public request,
   empty predecessor/parameter slices and no output. This proves termination
   before native decoding/VM/SDK check/k/key/proof/SRS/cache. Unit-test strict
   compile_all_policy without derive_all. Keep signed checks/full-escrow refusal.

6. Ownership/checks. Compiler implementer owns layout.rs,
   relation/{compiler,mod}.rs, static_compile.rs, commands/compile_call.rs,
   tests/static_compile.rs, CLI check_static_compile_io.rs and private
   compile_all_policy refusal unit test in local_deployment.rs. Consumer owner
   updates crates/mori/tests/fee_pool_component.rs and FeePool README after
   interface freeze. Financial .mori/bindings and Quint unchanged. Under shared
   build lock run public static positives/negatives/reopen, compile-fail docs,
   legacy unsigned/signed structural tests and FeePool suite, with no SDK
   query/check/model/k/proof/key work. Retain actual results; fresh Muse audits
   full candidate/results. R1's distinct profiles/qualifications/artifact route,
   absent authority/custody predicates and exact fresh reopen remain normative.
   M11/native acceptance open. Sol approves direction only.
