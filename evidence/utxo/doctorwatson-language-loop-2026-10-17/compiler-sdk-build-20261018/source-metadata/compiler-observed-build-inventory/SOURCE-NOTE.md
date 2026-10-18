# Observed backend build surface

Root's sanitized offline Cargo queries succeeded; this lane ran no Cargo/metadata/rustc/package/build/proc-macro/source execution. The backend tree has726 rows and280 unique exact package identities. All280 join unambiguously to fullworkspace metadata, retaining full registry/Git/path namespaces and lock checksums. Metadata contains374 workspace packages; earlier manual lock adjacency contained381 identities. **None of these counts is a backend compilation-unit count.** The tree query requested mori-backend, Linux x86_64-unknown-linux-gnu and normal/build/dev edges; it did not select --test stages or produce a Cargo unit graph.

The joined observed surface contains33 custom-build candidate packages and22 proc-macro-library candidate packages (54 distinct packages; paste declares both). These are target declarations on packages present in the observed tree, not executed builds or observed compiler units. All280 manifests and33 root build sources were read from explicit repo/public-cache paths and hashed. No permitted read path is missing. Proc-macro library bodies were neither read nor executed; their declared source paths are reconstructed only.

## Observed feature strings and manual reconciliation

Raw feature strings are preserved per occurrence, including libc's two strings (default,extra_traits,std versus default,std). Twenty-eight identities differ between the textual tree feature union and fullworkspace metadata feature union. Fullworkspace resolve features are stored separately and are **not** imported as backend features. In particular:

- blst0.3.17 tree: default; workspace metadata additionally has portable.
- curves0.3.1 tree: default,serde,std; workspace additionally has portable.
- cc1.7.0 tree: parallel. This resolves a previously manual optional-feature unknown for this observed tree, without selecting a build branch/tool.
- aws-lc-rs1.18.1 tree: aws-lc-sys,prebuilt-nasm; aws-lc-sys0.45.0 tree: prebuilt-nasm. Defaults and broader workspace features must not be substituted for these exact strings.

Forty-two of the prior50 finite manifest identities appear in this tree. glob,hermit-abi,legacy curves0.2.1,halo2derive,syn1,ledger,zswap and onchain-runtime do not appear. All57 later-acquired registry frontier identities do appear. The three registry packets cover86 of the observed registry identities. Eight selected pinned SDK Git manifests and five workspace manifests are also read; the remaining181 manifest reads have current source hashes without new archive-byte verification. Onchain-runtime's Scheme/vendored branch remains real comparison source from the older inventory, but is absent from the actual observed backend tree.

## Reconstructed path provenance

Original query-owned CARGO_HOME roots were removed. START/RESULT/CLEANUP remain original; root supplied supplemental PUBLIC-CACHE-PATH-MAPPING.json records after the queries (`recordedAfterQuery=true`). Only those exact registry/git link prefixes were replaced, with suffixes preserved and traversal/other namespaces rejected. A reconstructed current path does not assert that a symlink still exists or prove current source bytes equal query-time bytes. SOURCE pins distinguish current public reads from the 86 manifest/14 build-source comparisons to prior checksum-verified archive copies and 8 matching pinned SDK Git blob comparisons. All those comparisons match. New public-cache manifests/build sources were hashed, not newly archive-verified or acquired.

## Concrete source references and unknowns

The33 root build sources yield533 lexical reference lines,134 literal environment references and eight literal tool occurrences (bindgen,clang,clang-cl,freebsd-version,nasm). These are source text and may occur in comments/tests/conditional branches; no actual call/branch/effect is established. Dynamic commands, compiler wrappers/paths, environment values, generated outputs and input files remain unresolved. No tool lookup or environment query was run by this lane.

Nine external build-module declarations were recorded without a recursive walk. Seven are in aws-lc-sys builder/main.rs (cc_builder,cmake_builder,fips_probe,nasm_builder,sys_bindgen,system_detect,system_library), alongside two other root-script module declarations listed exactly in the JSON. Their cfg/path conditions and helper bodies were not followed. AWS builder/native header/source/bindings paths, cc/compiler/archiver decisions and blst prebuilt/source/assembly/feature/host-dependent branches remain designated-owner source-review inputs. Proc-macro bodies likewise remain separate review input; manifest/target declarations alone do not establish behavior. This is a finite observed surface for small reviews, not a full source/effects closure or a broad blocker judgment.

The unrun compiler-command candidate pins the existing exact stages test with explicit Linux target and j1. It is a proposal for the separate designated-owner/resource decision; it grants no execution/retry/cache-warming/cap-change authority. Existing U0/U1/U2 financial/compiler/target acceptance gaps and all original U0–U7/M/MC/SP/G exits, failures, caps/counters and rosters stay unchanged.

## Verification and preservation

Both query stdout digests match their successful RESULT receipts. All313 current public source reads were rechecked for stable bytes. The original707-file,808-file and2124-file source packets and selected-build inventory are unchanged and their metadata/source hashes were reverified. Only compiler-observed-build-inventory was written. No network, archive acquisition, Git mutation, SDK/runtime/model/VM/proof/key/SRS operation or private/config/auth/provider/prover-cache read occurred. This packet supplies source facts and observed tooling data, with no security/no-I/O/resource judgment or execution admission.
