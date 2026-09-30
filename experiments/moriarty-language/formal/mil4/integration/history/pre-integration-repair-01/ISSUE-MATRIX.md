# W-D2 integration issues and missing decisions

**Status: independent proposed / specified-only reconciliation.** Findings
refer to the hashes in [RESULT](RESULT.md), with G repair-03 retained as the
review target. [SPEC](SPEC.md) offers explicit successor rules; it does not
silently adopt them or certify any provider. No cases were executed.

| ID | Classification and exact evidence | Consequence / proposed disposition |
| --- | --- | --- |
| I01 | **Contradiction.** G DECISION-MATRIX G29 (line160) changes selectedTarget's current action from TransferLiteralFee to RepayAccrualFirst but claims B04 passes6 and Core ID fails10. G SPEC lines194–209 requires current-action equality after B04 authentication at6. F FIELD-MAP B04 anchor also owns current action. | First authentic-fact difference is actionId6/B04. If an isolated Core-ID10 oracle is desired, keep current action Transfer and change only expected Core ID (G42). Parent reports repair-04 in progress; neither its adoption nor fresh approval is claimed. |
| I02 | **Contradiction.** G multiple-defect paragraph lines224–226 says G18+G29 reaches Source-versus-Core literal failure at10. G29 still changes retained current-action fact. | B04 actionId6 precedes10 even with changed signed Core ID. Repair-04 must correct both the standalone row and combined paragraph, with preceding-gate conditions. |
| I03 | **Unadopted extension.** F FIELD-MAP D lines183–189 checks stage/episode wire only; F interface has no identityContext. G SPEC lines89–104/168–185 adds required wrapper and sourcePath context.*. | Add explicit interface/domain schema in a reviewed successor. Present wrapper before wire; missing wrapper waits for4/5. This is not a current F parser/AST slot. |
| I04 | **Missing total order.** G SPEC lines237–239 requires direct asset, B10 auth, B01 membership, B10 fact. F17 requires Source literal then B10 scale. H SPEC lines381–393 adds retained definition/policy asset/scale but does not place all competing facts in one sequence. | Proposed16: literal → B10 auth/scope → B01 membership → B10 asset → B05 asset → B07 asset. Proposed17: literal → B10 scale → B05 scale → B07 scale. Multi-defect examples are in SPEC. Fresh audit required. |
| I05 | **Missing merged diagnostic contract.** H8 adds prior definition identity comparisons; H11 adds package labels/Core ID; H12 precisely adds nine prior policy-body checks. F FIELD-MAP lines243–245 does not enumerate all those checks/loci. | Insert authentication before body/fact comparisons and computed signed-hash equality last. SPEC defines order, owner, comparisonTag, field/inputPath/factPath and null sourcePath. Fixed-suite impossible version branches stay conditional future behavior. |
| I06 | **Omission risk, not proven semantic contradiction.** F repair05 FIELD-MAP lines295–311 explicitly compares Source authenticated debtor/creditor claims before retained B11 facts. H SPEC lines406–417 names Source signed/submitted action slots, but these two are authenticated Source slots, absent from signedAction. H has no explicit repetition of F's literal diagnostic distinction. | Successor35 must explicitly preserve Source claim → B11 fact, with separate sourcePath/factPath controls. Payer never checks B11 debtor early. SPEC adds B06 constructor so no retained constructor is dropped. |
| I07 | **Compatible separation requiring an explicit rule.** G G38–G40 requires B04 selected-image linkage (wrong link invalid); H H-H11/H-H19/H-H20 permits authentic selected artifact with differing policy body values (mismatch at12). | External scoped selection/reference authentication and authenticated body equality are different predicates. No automatic invalid-proof label for a genuine policy body conflict, and no body equality can repair a wrong selected artifact. |
| I08 | **Missing evidence/decision.** F direct C constructor uses full B16 history and prepareMil4S0. H purpose3 commits only frontend/Core/Source-wrapper files; H SPEC lines240–246 explicitly says absent adapter is uncommitted. Existing wrapper feeds lowerer.state. | Not a contradiction: H acknowledges the gap. Adopt exact future adapter implementation/execution evidence or versioned package closure before positive B06 consumer correspondence. Standalone package content equality cannot satisfy it. |
| I09 | **Open authority.** G's same-view registry/stage references require B11 snapshot linkage; trusted root, update/rotation, proof formats, stage allocation and episode genesis/concurrency are absent. | Preserve sole B11 head custody. B02/B03 may authenticate references/history without independently vetoing current preHead. Define bounded relation/provider behavior and atomicity before any positive identity claim. |
| I10 | **Compatible incomplete history obligations.** F B16 validates ordered typed canonical tuples/distinctness/B13 consistency and H pre/post cap16. G replay omits agreement/episode/stage; H excludes replay from S/C/P and requires actual B16 handoff. | No truncation, reset, agreement-key widening or [] shortcut. Decide cross-instance nonce scope under existing (domain,signer,nonce), complete-history proof/transport and long-history rejection/upgrade policy. Pre16→post17 remains a closed-profile rejection. |
| I11 | **Open image/profile decision.** H recommends purpose1 B; purpose2 A has only a preimage, no full consumer/policy schedule. Wire3 has no separate image-suite field; G references are symbolic. | Adopt one authenticated suite/view selection and reference representation. No purpose1/2 fallback until hash equality. Concurrent suites need explicit version binding; registry bytes/hash and proof formats remain separate. |
| I12 | **Compatible evidence limit.** Old effect positives use actionId=Action; strict Source/6 requires builtin selector. G baseline is a symbolic projection, H image positives lack bytes/digests and real providers; F's six positives are only candidate inputs. | Freeze new complete independent documents/bytes/expectations. Preserve existing codec/equality fixture interpretation. No full-consumer positive, computed hash vector or acceptance gate is established by these drafts. |
| I13 | **Open signature/consumption.** F B09 preprocessing remains unselected; B17 only separate L. H wire content digest unchanged; G introduces no verifier. | Do not sign raw SHA256 by assumption, use flags as evidence, or make absent B17 reject local E. Actual B17 needs atomic current snapshot/digest/effects/grant/replay/successor consumption. |
| I14 | **Pending review dependency, parent-reported.** H repair01 Grok review found four medium defects: tag8 definition body-before-hash order, suite selection without purpose search, exact B14 scope diagnostic and keyRef format/opaque failure at12. H repair02 and G repair04 are frozen with bounded GPT approval and Grok pending; F repair05 Grok is also pending. | SPEC proposes explicit8 order/no suite search, B14 normalized scope order/loci and genuine malformed keyRef artifact diagnosis at12. These proposals and successor bytes need independent exact-candidate audit. Do not treat one provider or a pending review as adopted approval. |

All blocker IDs below remain open. “Proposal detail” means specified relation,
not adopted format, working provider or closed obligation.

| Binding | Owner/anchor and reconciliation | Remaining requirement |
| --- | --- | --- |
| B01 | F/G agreement3; strict instance, nominal sort and future association membership | Trusted immutable registry authority/view, bounded records and instance/cell namespace authenticity |
| B02 | F/G stage4; wrapper/unique scoped occurrence/reference, no second head | Allocation/uniqueness proof, bounded stage record and B11 linkage |
| B03 | F/G episode5; exact authenticated prior-stage membership | Genesis, continuing-history semantics, provider and concurrency authority |
| B04 | F/G action6; current action6, Core ID10, constructor35, image links8/11/12 | Repaired G29/I02, adopted mapping, unique selected manifest and verifier |
| B05 | F/H sourceHash8; purpose1 definition, separate external link/prior body/hash equality | Adopted image/suite and selected Source authentication; no current computed vectors |
| B06 | F/H coreHash11; purpose3 package plus independently verified execution/lowering | Loaded artifact/toolchain and actual future full-history adapter evidence |
| B07 | F/H policyHash12; nine prior body comparisons then hash; later terms at own tags | Adopted exact policy/suite/authority and merged multi-provider schedule |
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

The smallest runnable follow-on is the purpose1 content encoder/comparator in
SPEC, conditional on exact format/mapping adoption. It makes one missing byte
producer concrete while honestly leaving every authentication/ledger blocker
open. Full consumer implementation remains dependent on the merged contract
and real providers; this integration draft supplies neither.
