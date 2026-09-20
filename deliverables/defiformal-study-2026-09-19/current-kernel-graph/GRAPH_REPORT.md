# Current DefiKernel module map

Pinned commit: `33b9a9550ac1ed12c83c32d15277e530741787db`. This replaces no historical evidence. It is a bounded textual module/import graph, **not Lean AST extraction, elaborated proof dependencies, theorem validation or source/runtime correspondence**. Import direction is consumer → dependency. Comment masking handles nested block comments; declaration and proof bodies are not analyzed. External imports are terminal placeholder nodes. Families are directory groups, not inferred proof domains. Root reachability does not establish that a file is dead, accepted, built, checked, or suitable for deployment. No Lean builds, remote models, or theorem/axiom checks ran. No tracked source or tracked graph changed. Extraction uses zero LLM tokens; host analysis tokens are unmeasured.


193 source modules; 216 nodes; 518 distinct directed import edges; 518 import occurrences. Root entrypoint reaches 149 selected modules.

## Families

| Family | Modules |
|---|---:|
| Arithmetic | 11 |
| Atomic | 15 |
| CapabilityProvenance | 4 |
| Certificates | 21 |
| Composition | 11 |
| ConcentratedLiquidity | 10 |
| Interface | 14 |
| Interleaving | 18 |
| Metatheory | 12 |
| Nary | 30 |
| Parallel | 16 |
| Root | 11 |
| Typed | 11 |
| Vault | 9 |

## Cross-family imports

| Importer | Dependency | Imports |
|---|---|---:|
| Arithmetic | Root | 1 |
| Arithmetic | Typed | 2 |
| Atomic | Interleaving | 9 |
| Atomic | Parallel | 2 |
| Atomic | Root | 1 |
| CapabilityProvenance | Composition | 3 |
| Certificates | Arithmetic | 1 |
| Certificates | Composition | 5 |
| Certificates | Parallel | 5 |
| Certificates | Typed | 5 |
| Composition | Root | 1 |
| Composition | Typed | 5 |
| ConcentratedLiquidity | Arithmetic | 4 |
| ConcentratedLiquidity | Root | 1 |
| ConcentratedLiquidity | Typed | 1 |
| Interface | Atomic | 2 |
| Interface | Interleaving | 3 |
| Interface | Metatheory | 2 |
| Interface | Root | 1 |
| Interleaving | Composition | 1 |
| Interleaving | Parallel | 9 |
| Interleaving | Root | 1 |
| Metatheory | Atomic | 3 |
| Metatheory | Composition | 2 |
| Metatheory | Interleaving | 1 |
| Metatheory | Parallel | 3 |
| Metatheory | Root | 1 |
| Nary | Composition | 1 |
| Nary | Interface | 9 |
| Nary | Interleaving | 3 |
| Nary | Parallel | 6 |
| Nary | Root | 1 |
| Nary | Typed | 1 |
| Parallel | Composition | 7 |
| Parallel | Root | 1 |
| Parallel | Typed | 2 |
| Root | Arithmetic | 1 |
| Root | Atomic | 1 |
| Root | CapabilityProvenance | 1 |
| Root | Composition | 1 |
| Root | Interface | 1 |
| Root | Interleaving | 1 |
| Root | Metatheory | 1 |
| Root | Nary | 2 |
| Root | Parallel | 1 |
| Root | Typed | 1 |
| Typed | Root | 1 |
| Vault | Arithmetic | 5 |
| Vault | ConcentratedLiquidity | 3 |
| Vault | Root | 1 |
| Vault | Typed | 1 |

## Entry-point scope

Modules absent from the root import closure (presence here is not a defect or dead-code claim):

- `DefiKernel.CapabilityProvenance.FoundationChecks`
- `DefiKernel.Certificates.CanonicalJson`
- `DefiKernel.Certificates.Check`
- `DefiKernel.Certificates.CompatibilityExamples`
- `DefiKernel.Certificates.CompatibilityStatusRegression`
- `DefiKernel.Certificates.CompositionDepth`
- `DefiKernel.Certificates.Correspondence`
- `DefiKernel.Certificates.Decode`
- `DefiKernel.Certificates.Delivered`
- `DefiKernel.Certificates.DepthBridge`
- `DefiKernel.Certificates.Encode`
- `DefiKernel.Certificates.IRDepth`
- `DefiKernel.Certificates.KernelCorrespondence`
- `DefiKernel.Certificates.Lexical`
- `DefiKernel.Certificates.LibraryInstantiation`
- `DefiKernel.Certificates.Observation`
- `DefiKernel.Certificates.Roundtrip`
- `DefiKernel.Certificates.RunFixtures`
- `DefiKernel.Certificates.Schema`
- `DefiKernel.Certificates.Soundness`
- `DefiKernel.Certificates.Tests`
- `DefiKernel.Certificates.TrustedHost`
- `DefiKernel.ConcentratedLiquidity.Examples`
- `DefiKernel.ConcentratedLiquidity.FullMath`
- `DefiKernel.ConcentratedLiquidity.ProofAudit`
- `DefiKernel.ConcentratedLiquidity.RuntimeAudit`
- `DefiKernel.ConcentratedLiquidity.SqrtPriceMath`
- `DefiKernel.ConcentratedLiquidity.Tests`
- `DefiKernel.ConcentratedLiquidity.Token0Bridge`
- `DefiKernel.ConcentratedLiquidity.Token0Proofs`
- `DefiKernel.ConcentratedLiquidity.Types`
- `DefiKernel.ConcentratedLiquidity.Verify`
- `DefiKernel.Nary.Tree.FoundationChecks`
- `DefiKernel.Nary.Tree.RecoveryChecks`
- `DefiKernel.Vault.Adapter`
- `DefiKernel.Vault.Conversion`
- `DefiKernel.Vault.Examples`
- `DefiKernel.Vault.Operations`
- `DefiKernel.Vault.ProofAudit`
- `DefiKernel.Vault.RuntimeAudit`
- `DefiKernel.Vault.Tests`
- `DefiKernel.Vault.Types`
- `DefiKernel.Vault.Verify`
