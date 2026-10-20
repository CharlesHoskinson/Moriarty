# Native linear-combination recording source proposal — 2026-10-20

This package delivers the exact B44 candidate06 SDK source proposal. Two fresh non-author Muse Wonder high reviews approved **static source only**. No SDK installation, compiler/typecheck, doctest, unit test, MockProver run, keygen/proving capture test or native operation has run for this candidate.

The opt-in recorder observes configure bindings, region bases, keygen fixed/selector/copy data and proving advice. Its public join checks a generic linear-combination shape. It does not identify an actual NativeAdd coefficient pattern or establish gate/copy satisfaction, same-circuit/key authentication, adversarial accepted-assignment refinement, canonical Fr/Fq conversion, BLST/FFI correctness, verifier acceptance or a B43 Lean bridge. The existing B43 Fin proof is unchanged and was not rerun.

## Files and applicability

`candidate/proofs/` and `candidate/circuits/` contain ten exact reviewed afterimages (234,059 bytes). `source-repo.patch` and `source-repo.inverse.patch` use source-repository paths `proofs/` and `circuits/`; only package-directory labels were remapped. Source contents are unchanged. `SOURCE-MEMBERSHIP.json` records every afterimage and all seven required existing-file preimages; the three new files must be absent. `PATCH-ROUNDTRIP.json` records passive text application in both directions, not SDK execution.

This is a **source-repository patch**, with no registry-cache install. The circuits manifest preimage comes from published `Cargo.toml.orig`. Proofs test placement relies on source-repository automatic test discovery and its circuits dev-dependency. Generated registry manifests with `autotests = false` are different inputs. The published crates also report different upstream commit IDs; no single matching checkout is asserted. See `APPLICABILITY.json`. A later isolated source installation must match every preimage and resolve both crates to the same patched proofs instance.

## Proposed checks: all unrun

- 18 recorder unit tests and one `!Send` compile-fail doctest.
- Three actual NativeChip/MockProver binding-and-region tests. Their sessions intentionally lack keygen/proving dimensions and tables.
- One fixed-`k = 6` test holding guards across real `keygen_vk_with_k` and `create_proof`, asserting captured dimensions/tables and a structural join. Its test-only `ParamsKZG::unsafe_setup` is explicitly unsuitable for production parameter generation. It makes no verifier call.

Both-crate compiler/lint/doc checks, intended feature coverage and these tests remain unselected and unrun. Even a future finite capture-test pass would not discharge the deferred correspondence obligations above.

## Review record and operating limits

`reviews/review-{1,2}.verbatim.md` preserve the complete current opinions. `reviews/PROVENANCE.public.json` derives public model/provider/effort and terminal fields from actual receipts; it is labeled as derived and omits internal profile identifiers and token metrics. `reviews/SOURCE-API-REFERENCES.public.json` gives 47 exact source/API references. The deterministic `.json.gz` preserves those portable bindings losslessly; it contains metadata, not compiled artifacts or private witnesses. `history/` retains both earlier REVISE opinions and the factual correction lineage.

Armed recording adds an extra name callback for every region and requires the documented pure/idempotent callback subset. It retains full tables, including advice encodings, in thread-local sessions until taken/replaced/thread exit; thread reuse is no privacy boundary. Session/view Debug is redacted. Public records and caller-declared phase/count provide no provenance. Reviewer 2 also retained a nonblocking comment/performance mismatch: copy descriptors are recomputed on every active copy call although the comment says first call; no reviewed source was changed to clean that up.

Publishing this patch delivers reviewed source for later isolated validation. It is not runtime adoption, a deployable SDK installation, or Sherlock/EARS/native closure.
