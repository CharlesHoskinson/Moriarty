# Frozen public registry source preparation

Status: source acquisition/inventory only, 2026-10-18, GPT-6.1 Sol high nonsecurity preparation. No security/no-I/O/resource classification, runtime observation, execution admission or original gate closure is supplied.

Only this outside-Git directory was written. Develop skill, checkout AGENTS/product contract and read-only development status were restored; status exited0 with historical SP01 blockers retained. No Cargo/rustc/build/proc-macro/SDK/runtime/model/proof/VM/key/SRS/network command was executed. No repository/Git file changed. Reads under `.cargo` were confined to exact named package candidates under the explicitly allowed registry/cache/* and registry/src/* roots; no configuration/auth/provider/private/prover cache path was read. Package contents were copied exclusively from checksum-verified .crate archives, not source directories.

The designated owner JSON is used only to identify the missing registry source set. Its claims are not reissued or treated as admission. Exact checksum identities come from the current repository Cargo.lock, not version ranges in the acquired SDK manifest. The immutable lock hash and owner-requirements file hash are retained in PER-FILE-PROVENANCE.json.

## Scope and counts

- Requested names: midnight-curves, ff, subtle, serde, serde_core, serde_derive, serde_json, serde_repr, const-hex. Their ten locked identities were selected, including both midnight-curves versions.
- Added eleven explicitly named locked support identities: proc-macro2/quote/syn3/unicode-ident for Serde derives; paste for curves; halo2derive/syn1 plus num-bigint/num-integer/num-traits/autocfg for the separately labelled legacy curves0.2.1 proc-macro branch. These additions do not claim that all such features/targets are selected in the compiler build.
- Copied 707 exact text-source files from 21 checksum-verified package archives; total source bytes 9,406,785, below the 30,000,000-byte (30 MB decimal) bound. No archive binary or non-allowlisted payload was copied. Excluded members are individually listed in the provenance manifest.
- Root proc-macro library declarations: `serde_derive-1.0.229`, `serde_repr-0.1.21`, `halo2derive-0.2.0`, `paste-1.0.15`.
- Effective root build script source paths: `serde-1.0.229`, `serde_core-1.0.229`, `serde_json-1.0.151`, `num-traits-0.2.19`, `paste-1.0.15`, `proc-macro2-1.0.107`, `quote-1.0.47`, `syn-1.0.109`.

PER-FILE-PROVENANCE.json maps every copied file to package name/version, exact Cargo.lock checksum/source, public cache archive, verified archive digest, member path, output path, size and exact file digest. BUILD-PROC-MACRO-INVENTORY.json records each selected root Cargo.toml build/proc-macro declaration, build.rs members, source attribute occurrence locations, manifest build dependencies and locked dependency references. Declaration inventory is not macro expansion, runtime call-graph review or selected feature/target resolution.

## Exact gaps

- Missing selected package identities/verified archives: [].
- Archive checksum failures: 0.
- Cache absences for selected identities: 0.
- Unsafe archive/member or source-content/budget gaps: [].
- Optional ff_derive is declared by ff0.13.1, but no exact ff_derive package/checksum is present in the current Cargo.lock. It was not guessed, searched or copied; this is an unpinned optional declaration, not an asserted selected dependency.
- README/docs/JSON fixtures and other non-allowlisted archive members are intentionally omitted. The source subset is not a buildable vendored registry; embedded include_str/include_bytes and additional dependency needs remain for the designated owner to identify. All eligible .rs/.toml/license/build-source members in accepted archives are preserved unless an explicit gap is listed.
- The existing public SDK packet remains separately pinned; this registry packet does not replace it. Runtime leaf/build-time behavior and additional transitive dependencies outside this selected set remain for designated-owner review. No broad manually inferred Cargo graph, no-I/O conclusion, native resource budget or execution permission is claimed.

All original U0–U7, M1–M17 and MC/SP/G exits, financial/target requirements, historical evidence and resource caps/counters remain unchanged. No fetching, installing, warming or rerunning is proposed by this source packet.
