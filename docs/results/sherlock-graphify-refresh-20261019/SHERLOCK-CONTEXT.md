# Sherlock: use the actual Graphify graph first

Dated cut: October 19, 2026. The canonical graph is `/home/charl/Moriarty/graphify-out/graph.json`; its installed interpreter is recorded in `.graphify_python`. This incremental layer contains exact AST hashes from the publication checkout `373593264d0ebe30d750c3aff0b63393c441383a` (draft PR290), whose Root-reported merged parent is `9d0df91308d968a7413d04959dfb9dce88b31972` (PR289). The 32,061-byte native test at `crates/mori-backend/src/sherlock_machine/literal_witness_native_tests.rs`, SHA256 `18fdeeb6596976eb2a5e3ad9dc3edf7b8433beef15ea1be9f0cc33eadf46d449`, is a draft overlay, not a merged-parent claim. Other selected files are observed in that publication checkout; no Git comparison was performed by this preparer.

## Query before broad source reads

Run the installed Graphify CLI with the actual graph, for example:

```sh
/home/charl/.local/share/uv/tools/graphifyy/bin/python -m graphify query "Sherlock compiler circuit native prove stagenet formal" --budget 1400 --graph /home/charl/Moriarty/graphify-out/graph.json
```

Use the named Sherlock context nodes as seeds and a small budget. AST entries are structure, not execution. Resolve new relative source paths against `/home/charl/Moriarty/.worktrees/sherlock-actual-evidence-publication-20261019`, or the equivalent maintained checkout on another machine. Hashes and source cuts live on the nodes and in the allowlist receipt. The local graph's legacy imported material remains historical; its old built-at metadata is not globally advanced. The separate wiki-llm Sherlock semantic graph is advisory and is not this Graphify graph.

## Source, circuit and native route

The public compiler preparation entry is `compile_ext_host` in `crates/mori-core/src/sherlock/compile_artifact/ext_compile.rs`; the existing Draft route includes `run_draft_host`. Follow real AST edges through admitted source and owner runtime. This does not establish that arbitrary whole-program execution is lowered to an accepted native circuit.

The literal bridge exposes `prove_literal_transition` and `verify_literal_transition`. The draft native test `native_a102_literal_proof_roundtrips_and_independently_verifies` is source for one A102 proof/verification consumer. Its published source README explicitly separates source approval from compilation, SDK checks, proof soundness, recursion, ledger and stagenet evidence. Handwritten Add/Sub/Mul work and draft PR288's three Lt source files retain their own source/result scope; no complete machine or native theorem follows from registration or hashes.

Root's latest supplied observation for this cut is P13 native build exit0 in95.38s, with a473,870,536-byte ELF preserved and cleanup next. This is a Root-reported build observation pending its public archive binding here. No ELF was opened/copied/executed by the graph preparer. Proof is NOT RUN at this cut. Independent proof verification and Midnight stagenet submission/settlement remain separate unobserved next exits. The older source README's uncompiled/unrun wording remains a dated source-publication record, not a replacement for this later build fact.

## Formal profiles and current negative boundaries

K's published uncharged full-state/primary-boundary prefix evidence has explicit finite and provenance limits; it does not identify primary B work or physical state. See `models/sherlock/k/nominal-full-state-20261018/README.md` and the prefix16/first-arm18/caller-return22 result archives.

The maintained Quint B06/B18 source models publication boundaries and supplied invocation context. Root reports newer B19 six-state trace equality qualified by its positive fragment and source/runtime/admission premises; that newest result is a Root-reported supplement, not a blanket model theorem or charged B counterpart. Do not read a same-label scalar/terminal clock as equality between profiles.

Root reports Clean B24 revision2 failure and revision3 source fix pending. Retain that negative and pending status; no kernel success or theorem adoption is inferred. Clean, Quint and K need their own exact cuts, checks and complete comparison/proof evidence. Source approval, a finite result and an EARS adoption are different facts.

## Cheap maintenance

Use the installed Graphify deterministic AST `extract` and `build_merge` APIs on an explicit changed-file allowlist, anchored at the maintained source root, with an isolated cache. Preserve the historical graph, hyperedges and metadata; stamp only the selected AST files. This is a mixed historical/multi-root graph: do not run an unrestricted whole-root deletion scan or replace it with the advisory semantic graph. No provider extraction, SDK/proof run or HTML/whole-graph reclustering is required for a small refresh. Keep `.moriarty-dev/runtime`, private wallet/keystores, dependencies, generated binaries and huge raw outputs outside extraction.

## Later formal supplement in this preparation

After the earlier pending cut above, Root reported Clean B24 revision3 kernel exit0 in2.013854957s with empty stdout/stderr, using the16,443-byte corrected source (digest prefix6e576b05). Preserve revision2 failure as history. This is a scoped kernel observation, not a whole runtime/native or global formal claim; public result archive binding is pending here.

Root also reported the K Lt revision2 first compile exit0 in15.911s, with no orphan processes and one retained unused-O warning. Fifteen compiled files include a1,903,957-byte definition.kore, SHA256456fdb3b375761cc2adbb857b143e0c002ec1d6dc01210a07d7456bc4302d246. No K runtime/reduction/new independent oracle for that Lt cut was observed. Its compile success must not be relabelled runtime correspondence. These facts are attributed Root summaries, not a read of private model/trace bodies by this graph preparer.

## Published native build receipt

The P13 build archive is now published at `docs/results/sherlock-native-p13-proof-consumer-build-20261019/README.md` (SHA256 `1ec11afa1df800e2fff9b8f2726ae9aef280d95e2d5165e688331f44032bed81`); Root reports evidence-only publication commit `b97ae6c42dfe7bed6b78a21834ec3ca8dce14fd3`. The 206 source hashes remain the earlier source cut. The archive is build/cleanup evidence; proof remains NOT RUN at this cut.
