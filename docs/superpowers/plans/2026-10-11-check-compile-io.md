# Bounded check and complete-file compile publication

Owner: fresh GPT-6.1 Sol HIGH. Baseline: bf9b3da160ae6f9ca5474bdfe6b0f71a7796293e.
Independent expectations: next-check-compile-independent-expectations.md,
SHA256 ead56a6af9ef2f36c1caeeadf183a923d1054ab17834e927c237d07157049b74.

1. Run the existing safe production replay CLI tests before changing logic.
2. Add actual binary tests for bounded reads and unchanged Core/beta reports,
   UTF-8 and 65,536-byte grammar boundaries; record the oversize regression red.
3. Add actual compile round trips against the selected backend artifact bytes,
   Target/preflight errors, destination preservation and concurrent reservations.
   Exercise deterministic file-size faults through the production binary.
4. Factor the existing publication loop into a private caller for already reserved
   directories. Add deterministic raced-leaf tests against that complete caller:
   circuit/manifest files, symlinks, directories, partial bundles and user additions.
   Record failure against the old truncating writes, then use publish_new for all
   raw ZKIR leaves and artifact.json last. Replace check's read with read_source.
5. Add the two optional net_floor/liability_cap replay authority cases; run only
   the inspected safe replay target. Run fmt, diff and scoped safe checks.
6. Freeze a clean local candidate and create immutable capture/hash/report records
   for both fresh source and actual-result audits. No push or merge by author.

Case mapping: C01-C06,C16-C18 -> actual binary profile/report/display cases;
C07-C11,C13-C14 -> byte boundary tests; C12 -> UTF-8/missing/directory input tests;
C15 -> token/string/depth/item refusals. P01-P04,P19 -> contrasting signed/unsigned
round trips with typed exact serializer/raw ZKIR comparisons; P05-P07,P18 ->
Target/backend preflight and input-first diagnostics; P08-P10 -> destination and
parent preservation; P11-P12 -> simultaneous CLI reservations; P13-P14,P17,P20 ->
private complete publication caller tests with explicitly inserted third-party
leaves and manifest ordering; P15-P16 -> binary file-size faults and staging
collision/cleanup tests. OS permission tests require an unprivileged POSIX host;
unsupported hardlink, sync and power-loss faults are retained limits unless
actually injected. Specified-only expectations are never reported as executions.

Contract: stable ancestors/output directory, complete individual visible files,
atomic no-clobber leaf publication, final manifest marker, no success status on
failure. Newly reserved incomplete directories and earlier complete files may
remain. Preserve third-party paths and clean only normal-owned staging files.
No whole-directory atomicity, crash durability or arbitrary ancestor defense.
Keep existing grammars, compiler/Target limits, exact UTF-8 pretty artifact bytes
without newline and artifact_bytes fixed point (Target default 16 MiB). Core's
1 MiB ASCII JSON exporter must not replace compile serialization. No dependency,
native/proof/parameter/network change or financial/model/gate closure.

Observed C16 scope amendment: actual beta human diagnostics retained raw bidi and
newline characters in NamedSource's filename. Root explicitly selected a tiny
check-only repair using existing display::text(name). Escape that human source
name only; retain the original filesystem path, raw source/hash and JSON file
identity. The original red diagnostic capture remains evidence of this inherited
defect, not a bounded-reader regression. No shared renderer/framework change.
