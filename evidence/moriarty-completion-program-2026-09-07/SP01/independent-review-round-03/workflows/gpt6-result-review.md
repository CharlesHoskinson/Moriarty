BLOCKED — independent corrected financial-design result review.

Candidate `bc11324fec60da61b0a5a95766e2f1dc02c8f98958eccc94ca5aa527b473cc27` (374883 bytes). All owned-file and binding input hashes matched before and after review. The author receipt is terminal with exit 0; author success does not establish financial acceptance.

Independent Python standard-library checks replayed all 24 positive steps and 24 transfers, including funding, complete account records, ordinary/reserve work, history adjacency, successor counts, controller preservation, null debt/share records, and observation ages. Work totals are 40/0, 46/0, 20/0, 22/0, 14/8, 24/0, 28/0. All 732 structural references and row mutation references resolve; nine local source pins match. All 88 declared arithmetic/record examples recompute. All 25 negatives specify complete rollback identity, empty rejected financial effects and fees, and retention of previously charged work.

WF-F02, WF-F04 and WF-F07 are corrected within this review scope: fee-aware floors 1508/1008; funded 1501 output leaves maker1 3499 and fails 3002>=3015; Alice net is exactly -5030; C1-cover persists and the nominal claim references it. WF-F03 integer counters are corrected, but new unit metadata is inconsistent.

WF-R01 (WF-F01) — Underfunded alternates retain the funded genesis and financial history.

Both completeState objects conserve total cash and carry prior premium/observation records, but their genesis/admin still bind the original trace preState. Original C1 starts insurer 20000 and ends premium/observation/admission with insurer 20500; original C9 starts 15000 and ends premium/adjudication with 15200. The alternates silently introduce insurerTreasury 20400 or 15200, reduce insurer to 100 or 0, and preserve the same history heads and prefix work. No transfer or distinct admitted whole-alternate boundary accounts for this. C1 also changes claim/duty/cover payout authorization under the original admission history. fundingProvenanceConserved=true and noBalanceRewrite=true do not supply that missing provenance.

Paths: `/mutations/10/alternateAdmittedContext/completeState/genesis/preStateRef`, `/mutations/10/alternateAdmittedContext/completeState/admin/preStateRef`, `/mutations/10/alternateAdmittedContext/completeState/history`, `/mutations/18/alternateAdmittedContext/completeState/genesis/preStateRef`, `/mutations/18/alternateAdmittedContext/completeState/admin/preStateRef`, `/mutations/18/alternateAdmittedContext/completeState/history`.

Required fix: Bind each alternate to a distinct complete admitted genesis/admin/context boundary, with its own intent/plan and authorization/history/work records. Either explicitly admit the entire alternate state as a fixed toy starting boundary instead of referencing the original preState, or give the finite authorized treasury transfer/payout-authorization history and account for its work. Then enumerate all candidate prerequisites, keeping custody the sole failure and complete rejection equality to that alternate.

WF-R02 (WF-F05) — Full-record references do not yet give bounded acceptance and currentness predicates.

All four claim objects now refer to complete financial records and include a fixed positive genesis/admin boundary, controller lineage and a generic frame condition. However their summaries and binding lists contain no current semantics/key/spec status admission, no explicit pre-verification resource predicate, and no binding of the environment/assumption registry needed to decide those conditions. The input packet requires unknownOrRevokedSemanticsReject and preVerificationBudgets. There is no occurrence of revoked/currentness/unknown-semantics logic anywhere in this candidate. Binding a record by reference does not define the missing admission conditions.

Paths: `/traces/0/acceptance`, `/traces/1/acceptance`, `/traces/2/acceptance`, `/traces/3/acceptance`, `/traces/4/acceptance`, `/traces/5/acceptance`, `/traces/6/acceptance`.

Required fix: Write bounded specified-only conjunctions for the four claims over the complete intent/plan/pre/steps/post and admitted environment, including current registered semantics/key/spec status, resource limits checked before expensive verification, exact update relation, frame, authorization and history/successor currentness. A local fixed toy registry/context is sufficient if explicit. Do not add runtime/proof claims.

WF-R03 (WF-F05) — Several negative cases still lack candidate prerequisites and admission ordering.

All 25 expectedStateRef results correctly require complete selected-state identity and empty rejected effects/fees; prefix work is thereby retained. Most earlierStagePredicates list historical positive steps, however, rather than the proposed action's admission checks. The T5 concurrency candidate has no currentTime, the replayed M2 is also underfunded, and no global or per-case stage order separates message admission from custody. Two-successor candidates remain labels; destination re-credit and CreateDebt candidates remain partial payloads. The fee-aware negative balances are fully correct, but no reference supplies its full proposed duties/allowance/history/work outcome; its otherFinanceUnchanged even names work remaining after lock rather than explicitly inheriting the legal fill's proposed update.

Paths: `/mutations/1/mutatedInput`, `/mutations/4/mutatedInput`, `/mutations/4/earlierStagePredicates`, `/mutations/11/mutatedInput`, `/mutations/12/mutatedInput`, `/mutations/14/earlierStagePredicates`, `/mutations/19/mutatedInput/otherFinanceUnchanged`, `/mutations/21/mutatedInput`, `/mutations/22/mutatedInput`.

Required fix: For each affected control provide a complete proposed record via a resolved legal-step reference plus exact overrides, or an explicit complete input/schema boundary where the forbidden operation is rejected before constructing effects. Supply candidate admission time, signer/controller/currentness/budgets/history prerequisites and explicit stage ordering. State which subsequent failures are unreachable. Preserve complete rollback equality, all prefix work, and separate external fee scope.

WF-R04 (WF-F06) — Footprints still omit changed financial resources and associated reads.

The corrected RR-N7=3, D-cover-C9=3, DC-M1=2 write bounds match actual changes. The per-trace access convention is explicit. Independent complete-state diffs still show unlisted writes: recurring allowances.aggregate on five payments; contingent allowances.premiumPaid once, allowances.claimNominal once and insurerCustodyAssumption once; cross-domain allowances.fee once and top-level status three times; async top-level status three times; insurance allowances.coverPayout once and insurerCustodyAssumption twice. New allowance/cover writes also lack explicit corresponding read resources in several rows. totals:remaining_aggregate is not a declared alias for the complete allowance including cumulative amount.

Paths: `/rows/0/boundedFootprint/reads`, `/rows/1/boundedFootprint`, `/rows/3/boundedFootprint`, `/rows/4/boundedFootprint`, `/rows/5/boundedFootprint`, `/rows/6/boundedFootprint`.

Required fix: Map every state field that changes to one canonical financial resource, or explicitly define aliases/derived fields and account for their dependencies. Add the listed accesses with correct per-trace bounds and reads. Resolve duplicate singular/plural allowance resource names in the delegated row. Reconcile the resulting list mechanically against all 24 complete-state diffs.

WF-R05 (WF-F03 / WF-F06) — New quantity records use undeclared units.

The cross-domain asset table declares Midnight.USDC and Assumed.ForeignUSDC quantities in unit USDC at scale 0. New remainingLockAuthority instead uses unit Midnight.USDC in all four states, and mutation 22 uses unit Assumed.ForeignUSDC. These aliases have no declared equality or conversion. Integer totals agree, but the stated original=remainingLockAuthority+cumulativeLocked adds differing declared units.

Paths: `/traces/4/assets/Midnight.USDC/unit`, `/traces/4/preState/allowances/lock/remainingLockAuthority/unit`, `/traces/4/steps/0/postState/allowances/lock/remainingLockAuthority/unit`, `/traces/4/steps/1/postState/allowances/lock/remainingLockAuthority/unit`, `/traces/4/steps/2/postState/allowances/lock/remainingLockAuthority/unit`, `/mutations/22/mutatedInput/creditDest/unit`.

Required fix: Use unit USDC consistently with the asset descriptors, or declare an explicit unit equivalence/conversion and apply it at every relevant typed comparison. Recheck every quantity record, including negatives, rather than only checking field presence.

The JSON report contains all 25 individual mutation assessments, all 24 step diffs and cumulative work values, reference/source/quantity results, commands and exit codes, and the exact candidate/input hashes.

Toy calendars, prices, signers, custody and bounds are accepted only as explicit assumptions for internal review. No evaluator, implementation/build, network, wallet, git mutation, native proof, chain operation, or subagent was used. Raw worker stdout and private reasoning were not read. The 600-second review allowance was an admitted limit, not cgroup enforcement.

This is not acceptance of runtime, proofs, full map, semantic freeze, Preview, public release, SP01, or any of the twelve sprints.
