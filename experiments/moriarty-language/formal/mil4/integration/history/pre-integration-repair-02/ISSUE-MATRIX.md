# W-D2 integration issues and missing decisions

**Status: integration repair-01 / proposed / specified-only reconciliation.**
Inputs are F05 and G04 with accepted bounded specified-only dispositions, and
H03 frozen candidate with GPT bounded approval/Grok pending. The original
integration packet is preserved under history/pre-integration-repair-01.
Hashes and review scope are in [RESULT](RESULT.md). No cases were executed by
this agent. Candidate incorporation does not adopt a verifier or close a gate.

| ID | Classification and exact evidence | Consequence / proposed disposition |
| --- | --- | --- |
| I01 | **Fixed in G04; historical contradiction preserved.** Prior G03 G29 changed current action to Repay but claimed first failure10. Current G04 G29 keeps actionKey/current-action/builtin-selector Transfer and changes only expected Core program/constructor. | Tag6 now passes its own current-action check; with B05/intervening gates passing, B04 expected Core ID differs at10. G04 bounded disposition confirms resolution; no executed registry proof is claimed. |
| I02 | **Fixed in G04; historical contradiction preserved.** Prior G18+G29 paragraph retained a changed current action despite a10 oracle. G04 explicitly retains Transfer current action and changes signed/expected Core ID to Repay. | Direct Source-versus-signed Core ID fails10 after earlier gates pass. Genuine current-action mismatch remains a separate isolating control needing executable coverage. |
| I03 | **Unadopted extension.** F FIELD-MAP D lines183–189 checks stage/episode wire only; F interface has no identityContext. G SPEC lines89–104/168–185 adds required wrapper and sourcePath context.*. | Add explicit interface/domain schema in a reviewed successor. Present wrapper before wire; missing wrapper waits for4/5. This is not a current F parser/AST slot. |
| I04 | **Incorporated in H03 candidate.** Asset16 now fixes direct Source → B10 authentication/prior domain/agreement scope → B01 membership → B10 asset → B05 definition → B07 policy. Scale17 is direct Source → B10 → B05 → B07. | SPEC mirrors the exact total order and B01/B10/B05/B07 fact paths. H03 GPT approved the candidate; Grok pending. Candidate order supplies no working provider. |
| I05 | **Partly incorporated in H03; remaining integration refinement.** H03 at8 defines six prior Source-body checks, all FIELD_MISMATCH with B05.sourceDefinition.<name>, before hash. At12 it admits keyRef before nine policy-body checks/hash. Tag11 package-label value order remains less explicit. | SPEC aligns8/12 with H03 rather than prior integration tuple-error reinterpretation. Its explicit11 label/order diagnostic refinement remains a new unreviewed integration proposal. |
| I06 | **Incorporated in H03 candidate.** H03 operation.debtor/creditor explicitly checks AST authenticated obligation cells before retained B11, with Source path/null binding versus B11 fact/null Source path. H-H26 isolates wireX/SourceY/B11Y, wireX/SourceY/B11X and wireY/SourceY/B11X. | SPEC preserves H03/F05 order and paths, no early debtor check at payer. Safe obligation indexing before later unsigned-tail length checks remains F low implementation residue. H03 Grok pending. |
| I07 | **Compatible separation requiring an explicit rule.** G G38–G40 requires B04 selected-image linkage (wrong link invalid); H H-H11/H-H19/H-H20 permits authentic selected artifact with differing policy body values (mismatch at12). | External scoped selection/reference authentication and authenticated body equality are different predicates. No automatic invalid-proof label for a genuine policy body conflict, and no body equality can repair a wrong selected artifact. |
| I08 | **Missing evidence/decision.** F direct C constructor uses full B16 history and prepareMil4S0. H purpose3 commits only frontend/Core/Source-wrapper files; H SPEC lines240–246 explicitly says absent adapter is uncommitted. Existing wrapper feeds lowerer.state. | Not a contradiction: H acknowledges the gap. Adopt exact future adapter implementation/execution evidence or versioned package closure before positive B06 consumer correspondence. Standalone package content equality cannot satisfy it. |
| I09 | **Open authority.** G's same-view registry/stage references require B11 snapshot linkage; trusted root, update/rotation, proof formats, stage allocation and episode genesis/concurrency are absent. | Preserve sole B11 head custody. B02/B03 may authenticate references/history without independently vetoing current preHead. Define bounded relation/provider behavior and atomicity before any positive identity claim. |
| I10 | **Compatible incomplete history obligations.** F B16 validates ordered typed canonical tuples/distinctness/B13 consistency and H pre/post cap16. G replay omits agreement/episode/stage; H excludes replay from S/C/P and requires actual B16 handoff. | No truncation, reset, agreement-key widening or [] shortcut. Decide cross-instance nonce scope under existing (domain,signer,nonce), complete-history proof/transport and long-history rejection/upgrade policy. Pre16→post17 remains a closed-profile rejection. |
| I11 | **Suite selection specified; authority still open.** H03 recommends purpose1 B; purpose2 A supplies only a preimage. Wire3 has no suite field; G references remain symbolic. | Select one authenticated suite before any comparison. No purpose1/2 search or fallback ever under the selected suite, even if another digest would match. Concurrent suites require explicit authenticated version binding; trusted-view/provider formats remain open. |
| I12 | **Compatible evidence limit with separate codec result.** Old effect positives use actionId=Action; strict Source6 needs builtins. G has symbolic cases, H design remains specified-only. A separate codec packet records72 tests and finite image vectors, with an open high Proxy finding. | Preserve source-packet scope. Codec bytes/content results establish no Source AST adapter, registry, correspondence or full consumer. No implementation/testing was done for this integration successor. |
| I13 | **Open signature/consumption.** F B09 preprocessing remains unselected; B17 only separate L. H wire content digest unchanged; G introduces no verifier. | Do not sign raw SHA256 by assumption, use flags as evidence, or make absent B17 reject local E. Actual B17 needs atomic current snapshot/digest/effects/grant/replay/successor consumption. |
| I14 | **Current evidence dependency.** F05/G04 dispositions accept bounded specified-only candidates after GPT high and Grok returned grok-4.7-build/xhigh reviews. H03 packet9a459b…c8efa has GPT high bounded approval; Grok pending. | H03 incorporates8/12, suite, B14 scope and16/17/35 fixes. Accept neither H03 nor this integration until exact fresh required review. No historical review is rewritten. |

| I15 | **Open low implementation residue from accepted F/G dispositions.** F: binding versus retainedFactBinding, safe35 obligation indexing, signed/submitted paths. G: tag10 step5 unreachable as first failure, missing isolated genuine current-action control, abbreviated Core-program diagnostic detail. | Track in independent executable expectations; no invented branch/test or early relocation of unsigned-value checks. Low residue does not negate bounded design acceptance or establish implementation success. |
| I16 | **Open high codec implementation finding.** GPT review of packet34e345…7d332 validates descriptor admission but observes discarded values followed by Proxy get rereads; selector/kind and Source/policy asset coherence can change across phases. It reports72 original tests passed. | Preserve the original finite result while requesting bounded recursive owned snapshots, cross-phase value coherence, hostile controls and fresh exact-result audit. Proxy repair underway is not accepted resolution. No B05–B07/W-D2 gate closes. |

All blocker IDs below remain open. “Proposal detail” means specified relation,
not adopted format, working provider or closed obligation.

| Binding | Owner/anchor and reconciliation | Remaining requirement |
| --- | --- | --- |
| B01 | F/G agreement3; strict instance, nominal sort and future association membership | Trusted immutable registry authority/view, bounded records and instance/cell namespace authenticity |
| B02 | F/G stage4; wrapper/unique scoped occurrence/reference, no second head | Allocation/uniqueness proof, bounded stage record and B11 linkage |
| B03 | F/G episode5; exact authenticated prior-stage membership | Genesis, continuing-history semantics, provider and concurrency authority |
| B04 | F/G action6; current action6, Core ID10, constructor35, image links8/11/12 | G29/I02 fixed in G04; adopted authenticated mapping, unique manifest and verifier still absent |
| B05 | F/H sourceHash8; purpose1 definition, separate external link/prior body/hash equality | Adopted image/suite and selected Source authentication; no current computed vectors |
| B06 | F/H coreHash11; purpose3 package plus independently verified execution/lowering | Loaded artifact/toolchain and actual future full-history adapter evidence |
| B07 | F/H policyHash12; nine prior body comparisons then hash; later terms at own tags | Adopted policy/suite/authority; H03 merged schedule still under Grok review |
| B08 | F signer13; retain authorized key for15; H signer/keyRef term checks | Ownership/reference resolution, validity/rotation and policy relation |
| B09 | F scheme14; authorized B08 key, independent of later wire key equality | Adopted exact message preprocessing and real selected verifier |
| B10 | F/G/H asset16, scale17; proposed merged order | Registered asset/unit/scale authority and intersection relation |
| B11 | F/G snapshot18; identity/view/stage linkage then core/domain/asset/head facts | One authenticated complete same-instance snapshot and actual provider |
| B12 | F predecessor19; authenticated prior head, distinct from current head | Exact prior-linkage protocol/provider |
| B13 | F nonce20; full (domain,signer,nonce), genuine consumed separate from invalid | Authenticated unused-status/uniqueness proof and explicit cross-instance policy |
| B14 | F/H grant23 then predicates23/24/25; policy equality does not establish authority | Exact allowance/work authority predicate, scope and same-snapshot counters |
| B15 | F successor after unsigned tail; digest-independent extension | Adopted independent successor generation/validity and real proof |
| B16 | F/H history after B15; integrity/types/distinctness/B13 consistency/Source claim then H | Complete bounded transport/proof and actual direct-Core handoff |
| B17 | F separate L after local E; G/H no substitute | Real atomic compare-and-consume/apply and ledger evidence |

The smallest current follow-on is the existing codec's bounded Proxy repair,
then a separate Source AST-to-purpose1 adapter after actual-result acceptance
and image-suite adoption. This replaces the historical new-encoder recommendation
because a separately reviewed codec now exists. Full consumer/provider/ledger
work remains separate; this integration supplies no code, tests or gate closure.
