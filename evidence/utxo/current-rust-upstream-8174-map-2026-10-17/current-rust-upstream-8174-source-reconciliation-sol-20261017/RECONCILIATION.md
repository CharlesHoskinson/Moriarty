# Current Rust upstream reconciliation: 8174e529

Upstream public head is `8174e52975d8db9bf59636fea0fa338c0f47c855`: Merge pull request #128 from CharlesHoskinson/evidence/sherlock-d01-host-observation-20261017. This report compares complete source bytes with the existing delivered Rust package; it does not adopt upstream code or establish compatibility.

All **157** delivered files match SOURCE-DELIVERY.json. Of the **70** pinned Core members, **69** are byte-identical to the new head (and to both prior inspected refs). The sole changed member is `crates/mori-core/src/lib.rs`: delivered 462 bytes; current upstream 1,277 bytes.

The actual admission and financial sources are unchanged bytes: `parser.rs`, `ir.rs`, `runtime.rs`, `intent.rs`, `policy.rs`, `replay.rs`, `logical_cost.rs`, `source_sponsor.rs`, and all eight delivered `stage/` modules. This is a byte-currentness finding, not a semantic-equivalence or security result.

## Exact reconciliation required

1. **Core source membership.** New `lib.rs:8–31` declares public Sherlock and two source adapters; draft modules are test-gated. `sherlock/mod.rs:8–16` introduces further ordinary module dependencies. The following eight files are absent from the frozen70 membership:

   - `crates/mori-core/src/sherlock/mod.rs`
   - `crates/mori-core/src/sherlock/compare.rs`
   - `crates/mori-core/src/sherlock/reference.rs`
   - `crates/mori-core/src/sherlock/reference_stream.rs`
   - `crates/mori-core/src/sherlock/reference_display.rs`
   - `crates/mori-core/src/sherlock/reference_types.rs`
   - `crates/mori-core/src/sherlock/source_adapter.rs`
   - `crates/mori-core/src/sherlock/bounded_scalar_source.rs`

   Reconcile the package membership and the explicit `CORE_FILES` list in delivered `r1cs/source/mori-r1cs/src/build_identity.rs`. This report does not modify that identity or authorize compilation.

2. **Source projection versus language admission.** New `source_adapter.rs:117–146,178–206,260–302` maps only stateless scalar `/1` actions with ordered lets and one final emit value; its report declares function/branch/recursion/data support false and evaluation false. Projection refusals are metadata. It does not add financial-effect execution or expand `/1` grammar.

3. **Draft capability remains separate.** `draft_source.rs:9` names `moriarty-sherlock-draft/1`; its function/program/entry parser is gated by `lib.rs:12–26`. Lists, trees, helper calls and typechecking in this branch must not be silently presented as the shipped `/1` frontend.

4. **Real upstream consumer changed.** Main `crates/mori/src/commands/compile.rs:37–55` uses `compile_reported` and writes `source-projection.json`. `mori-backend/src/source_projection.rs:31–48` still calls the existing signed/unsigned compiler before creating metadata. Source control flow is observed; artifact equality or failure behavior was not executed. Delivered `mori-k2` has a different consumer (`k2/src/relation.rs:258–268` → `mori_r1cs::compile_source`), with `/1`/staged `/2` routing in its own `frontend.rs`. No new main-CLI behavior is inferred for K2.

5. **New host and backend sources.** `bounded_scalar_source.rs:126–151`, `reference.rs:76–97` and `reference_stream.rs:22–72` add bounded observation/grouping APIs. The new backend `sherlock_machine` sources are source constructors; their security/circuit meaning is reserved for Opus. Test-gated image/layout/helper sources are distinct from ordinary public admissions.

Across the selected public `crates`/root-manifest scope, 72 files are added or changed after `85d825ea…`; this is not a documentation-only update. Every selected file has old/new byte hashes and an individual source classification in MACHINE-DELTA.json.

## Evidence and limits

SOURCE-MANIFEST.json pins all157 delivery members, compared public Git blobs and precise source line spans. MACHINE-DELTA.json records each of the70 Core comparisons, all selected crate source changes, consumer pointers and the minimum ordinary module gap. Prior public refs are `16089e3b8953a4996d5c84ccb8761ef06f9c8a6b` and `85d825ea6b04fd5713ebf23b72526532f67c4925`.

The graphify skill was inspected. `graphify-out/graph.json` does not exist in this checkout; no graph rebuild, installation, project import or helper execution was performed under the source-only boundary. AGENTS and the develop skill were loaded; guarded status returned0 and no historical campaign was dispatched.

No project code, tests, SDK, model, K/Quint, native, circuit, proving, verification, setup, VM, wallet or ledger operation ran. The checkout, Git refs, prior source/history and receipts were not written. No private capture or cache was traversed. Output is limited to this new directory.

The retained obligations remain open: 20 UTxO families; ACTUS 277 fixtures/18 executable types/32 taxonomy dispositions/DS01-07; DeFi 72 rows/DA01-24/held-outs/intents/regressions; general financial obligations; recursion; private handoff; multi-parent composition; all U0-U7, M1-M17 and MC/SP/G exits; existing failed findings/counters and consumer obligations.

The next source reconciliation concerns the new module membership and consumer/projection boundary, rather than a demonstrated change to the unchanged financial parser/runtime. Security interpretation, actual compilation behavior, source-to-Cardano correspondence and target acceptance remain separate work.

The exact path `experiments/moriarty-cardano-rust` is absent from the upstream8174 tree (`git ls-tree` exit0, empty result). The delivery’s R1CS/driver members are verified against their frozen membership, not equated to the differently named main backend. The direct Core comparison still uses all70 complete upstream root-path blobs. Broader roadmap/product-contract documents were not freshly read in this exact-source scope; the earlier session observed the skill-referenced `docs/MORIARTY-PRODUCT-CONTRACT.md` missing, which this task did not revalidate. No product/spec acceptance is inferred from that incomplete documentation intake.
