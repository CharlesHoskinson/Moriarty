# Independent final review: kernel and backend boundaries

Verdict: **approved — design consolidation only**.

Scope: the complete root-authored `review-candidate-v1.txt` and all 11 files in `candidate-v1-manifest.json`, read in full. I verified all 11 candidate files' SHA-256 digests and byte counts against the manifest; all match. No other final-review outputs were read. No candidate edits, builds, proof generation, network actions or deployments were performed.

The candidate honors the controlling scope and is suitable to adopt as a specified-only consolidated design. No blocking boundary or trust defect found. Approval does not establish implementation feasibility, current upstream support, financial acceptance, a cryptographic theorem or release readiness.

## Why the boundary is sound as a design

- **Permissionless language versus services:** ROADMAP and consolidated design make the federation optional for direct Midnight use. UNI-001 removes project/reviewer/provider admission from public validity. UNI-011/012 preserve owner consent, explicit delegation and external signing enforcement without requiring universal solver membership.
- **Exact execution target and no Lean dependency:** the design commits to native Midnight/ZKIRv3, permits Compact only with explicit correspondence and keeps DeFiFormal as a semantic reference. UNI-003's explicitly qualified future successor requires qualification rather than presuming a version upgrade. No automatic theorem transfer or alternate proving backend is introduced.
- **ZK/MPC/TEE and external truth:** UNI-011 and the design name the common statement and separate cryptographic, threshold, hardware, observation and finality assumptions. The bare-threshold compromise boundary is explicit. The TEE cannot substitute for a missing mandatory program proof. ZR external-evidence integration correctly avoids interpreting comprehensive native recursion as automatic support for every foreign proof or settlement system.
- **Currentness and history:** ZR04/05/07–10/15 distinguish legitimate genesis, relation identity, mandatory verification, final cryptographic discharge, current-state consumption and migration. Ledger induction remains a scoped early mechanism and cannot close MC03/MC06. Reusable proof evidence does not recreate spending authority.
- **Partial effects and consent:** UNI-005–008 preserve gross economics, fees and duties across phase failure, workflow partiality and recovery. The design expressly allows authorized duties from accepted guaranteed phases while forbidding recipient duties from mere request recording. Refund/compensation and local atomicity/multichain progress remain separate. Recovery rights have their own signed scope; no perpetual authority is invented.
- **Privacy:** ZR14/16 and UNI-010 name observer/leakage, private completeness, successor witness requirements, isolation and missing-witness behavior. The candidate does not claim that recursion transports hidden inputs or makes them available.
- **Future recursion:** the approximately March 2027 horizon is consistently a dated user planning assumption. The target explicitly retains full recursive financial history and bounded private multi-parent composition. ZR01–16 have owners and observable acceptance requirements; missing current functionality is openly an implementation dependency, not silently removed scope.
- **One roadmap:** U0–U7 is the governing sequence. P/C/K remain aliases and retained detail. MC/SP/G financial, recursion, private-composition and release obligations remain, with scoped U2 history separated from full closure. Pel is explicitly symbolic and must bind actual commands, reviews and resource limits before dispatch.

## Non-blocking editorial/test precision corrections

1. `openspec/changes/consolidated-language-kernel/requirements.md`, opening link: update **UNI-001–016** to **UNI-001–017**. The normative spec contains UNI-017, and the backend contract already incorporates its substance, so this is a navigation/count correction rather than a missing behavioral requirement.

2. `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, **ZR08 acceptance test** and **Integration requirements / Complete verification**: qualify “reordered accumulators … reject” as “a reordering that breaks the specified statement-to-accumulator binding rejects.” The same document appropriately permits safe batching. A canonical encoding may forbid all reorderings, but cryptographically and semantically equivalent permutations in an explicitly order-insensitive batching scheme need not be rejected. Preserve required rejection of dropped, substituted, unbound or invalid obligations. This refinement avoids overprescribing serialization while leaving the substantive ZR08 requirement unchanged.

No code, empirical result or additional theorem is required as a precondition to adopting this clearly specified-only design. These items can be corrected deterministically without reopening architecture choices.

## Acceptance boundaries to preserve in implementation packets

U0 must turn the canonical statement into a field-by-field enforcement map: circuit constraint, authenticated read, signature commitment or ledger check. ZR03's authoritative network/time/fees linkage cannot remain host metadata. U1/U2/U4 must pin proof-verifying binary configuration and test the complete deferred verifier, not just a successful outer-proof API. U3/U5 need actual entitlement consumption for settle/refund races and durable shared reservations. U4 needs usable positive private continuation and join examples with explicit witness availability. U6/U7 cannot reduce retained conformance denominators or call adviser agreement implementation evidence.

These are already required by the candidate and remain future acceptance work, not grounds to reject the design.
