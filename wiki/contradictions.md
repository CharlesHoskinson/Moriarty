---

id: research.contradictions
type: contradiction
title: Contradictions and documentation drift
status: active
updated_at: 2026-09-07T16:50:22.642457+00:00
sources:
  - SRC-0070
  - SRC-0071
  - SRC-0072
  - SRC-0055
  - SRC-0056
  - SRC-0043
  - SRC-0044
  - SRC-0045
  - SRC-0046
  - SRC-0002
  - SRC-0005
  - SRC-0006
  - SRC-0007
  - SRC-0009
  - SRC-0012
  - SRC-0015
  - SRC-0016
  - SRC-0017
  - SRC-0036
  - SRC-0024
  - SRC-0038
  - SRC-0029
  - SRC-0030
  - SRC-0023
  - SRC-0026
  - SRC-0039
  - SRC-0040
  - SRC-0041
  - SRC-0042
  - SRC-0047
  - SRC-0048
  - SRC-0049
created: 2026-09-02
updated: 2026-09-07
tags:
  - moriarty
  - research
---

<!-- markdownlint-disable MD013 MD025 MD060 -->

# Contradictions and documentation drift

## Intents report versus Moriarty requirements — 2026-09-06

SRC-0047 recommends an IKL name, optional early proofs and other-backend-first
milestones. SRC-0041/0046/0049 require Moriarty, finite bounds, ACTUS/DeFi targets
and mandatory PCD with Midnight priority. Disposition: adopt authority/plan/
receipt separation and refinement, retain the user's stronger constraints.
Exact-plan R2 is a legitimate restricted profile, not outcome authorization with
route freedom after signing. The report's embedded prototype is not present and
its test/proof claims remain unverified. This resolves the execution-order
conflict without asserting that the report's external claims were reproduced.

SRC-0048 checks the report's four-primitive withdrawal against DeFiFormal's
prior design pin. The live checkout had advanced, so immutable `git show`
snapshots were used. Taxonomy remains a coverage guide; withdrawal does not
establish a replacement primitive theorem. See CLM-0195.

| Conflict | Evidence | Disposition |
|---|---|---|
| Marlowe live sitemap targets versus live site | The current sitemap points all 100 entries at dead `play.marlowe.iohk.io` hosts; the same paths work at `docs.marlowe-lang.org` and were acquired 100/100 | Documentation deployment drift; use the live-host crawl, preserve the sitemap as evidence |
| Marlowe abstract interval versus formal placeholder | Haskell V1 implements inclusive bounds; Isabelle `BlockchainTypes.thy` retains a TODO about endpoint treatment | Normative ambiguity; release blocker for V2 equivalence |
| Isabelle coverage versus Haskell behavior | Haskell comments identify refund-order differences and lack of Isabelle Merkleization | Do not claim full implementation correspondence |
| `marlowe-runtime-ng` name versus implementation | Repository has one initial commit and an empty README | S1 placeholder, not an implemented Runtime replacement |
| V2 proposal versus implementation status | April 2026 report proposes `WhenAll`, `EntryDeposit`, compression, iteration, and types; current active validator branch does not contain those language features | S2 design must not be described as S3+ implementation |
| Midnight `compact` API flag versus README | GitHub metadata says `archived=false`; README says archived/no longer maintained and development moved to LFDT Minokawa | Treat as S5 release mirror; use `LFDT-Minokawa/compact` for source authority |
| Current Midnight docs versus compiler development head | Docs describe language 0.26.0/compiler 0.34.0; active source builds compiler 0.34.100 with language 0.26.0 and runtime 0.19.100 | Record the complete version tuple; do not use “Compact 0.26” as a toolchain identifier |
| ZKIR generations | Some shipped/precompiled artifacts remain ZKIR 2 while the extracted `midnight-zkir` default branch is `zkir-v3` and Moriarty generated 3.0 | Require explicit IR major/minor and backend conformance; no implicit latest |
| Imported report citations | The user-supplied report contains internal `turn...` citation markers that cannot be resolved outside its original session | Treat the prose as design input only; replace claims with pinned local evidence |
| Koios transaction-detail capture | The first all-at-once POST is preserved as a 413 response in the top-level `.json`; seven subsequent bounded Scrapling POSTs returned all 137 rows | Use the chunk manifest as evidence; never parse the failed response as JSON |
| Taxonomy report access versus authorized repository | `SRC-0015` correctly reports that its online environment could not retrieve a public DeFiFormal repository, but `SRC-0016` is an authorized clean local checkout at commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95` | Preserve the report's limitation as historical context; use the local pinned artifacts for the 72-row and 60-construction reconstruction without inferring public availability |
| DeFiFormal working-note totals versus current pinned artifacts | An earlier working note expected 47 partial, 13 inadmissible, 573 covered, and 686 residue; direct aggregation at `SRC-0016` yields 45 partial, 15 inadmissible, 570 covered, and 689 residue from the same 1,259 obligations | The reproduced current-commit totals control; earlier figures are superseded unless a different commit is produced |
| Marlowe baseline abbreviated hash | The first wiki draft expanded prefix `99f432d8` to the wrong full hash; the repository lock and checked-out `HEAD` agree on `99f432d8ef9dbd1b52b7fa089254de15913b490f` | Corrected in `marlowe-baseline.md`; all future claims use the full lock-record hash |
| First taxonomy recommendation versus updated run | SRC-0015 recommends M4+ after it could not inspect DeFiFormal; SRC-0017, after repository inspection, recommends M2+M3 for human-facing families and facets with M5 as the internal formal profile | SRC-0017 supersedes the top-level taxonomy decision; preserve M4+ only as a historical crosswalk and useful facet decomposition |
| Updated-run archive provenance versus Moriarty repository pin | SRC-0017 reports that its supplied ZIP had no recoverable commit; Moriarty has a separate clean checkout at `8ae0bbfaa3193078d1cabf6999db1382985b7f95` | Bind all reproduced results to SRC-0016 and the full commit; do not attribute that commit to the ZIP |
| Updated-run complete-linkage ARI at 12 clusters versus independent harness | SRC-0017 reports 0.36; the deterministic SciPy nearest-neighbour-chain reproduction gives 0.3547259508, which rounds to 0.35 at two decimal places | Preserve the raw value and pinned method; treat 0.36 as a minor reporting or implementation-version discrepancy that does not change the best-ARI conclusion |
| NEAR Intents atomic and automatic-refund overview versus route behavior | The overview states atomic execution and automatic refunds; Verifier documentation says external calls complete asynchronously, simulation excludes them, and deposit, withdrawal, storage, and indexer paths include detached, nonrefundable, or manual recovery | Define atomicity and recovery per layer and route; never lift Verifier batch atomicity to bridge fulfillment |
| NEAR Intents non-custodial description versus implementation boundaries | The overview says users maintain control; `intents.near` records contract-held internal balances, 1Click says it temporarily transfers assets to a trusted swapping agent, and confidential execution adds a treasury and PoA bridge | Publish a route-specific custody and authority manifest before approval |
| NEAR Intents narrative status versus OpenAPI | The quickstart lists `KNOWN_DEPOSIT_TX`, which is absent from the published swap status enum; the order enum contains `UNTRIGGERED`, which the narrative omits; fill and payout use independent states | SDKs must accept unknown statuses, preserve raw evidence, and reconcile service status with chain evidence |
| NEAR Intents guaranteed-delivery name versus transport semantics | The relay replays unacknowledged events, requires client deduplication, has a seven-day retention limit, and is described as live but not yet exercised by a solver | Treat it as bounded at-least-once delivery, not exactly-once or permanent delivery |
| NEAR Intents confidentiality label versus trust boundary | `basic` and `advanced` lack public normative leakage definitions; the embedded profile uses a private NEAR fork, small permissioned validator set, treasury, private relay, and PoA bridge | Model confidentiality as a named adapter profile with explicit disclosure and custody assumptions |
| Current ERC-7683 versus prior draft and OIF terminology | The 2026 resolver draft removes the prior standardized order/open/fill objects; current OIF code and prose still call `StandardOrder` and `MandateOutput` ERC-7683 | Pin the exact revision and treat OIF as a separate compatibility profile; never blend the two semantics |
| ACTUS documentation sitemap origin versus deployed origin | All 220 documentation sitemap entries use `https://your-docusaurus-site.example.com`, while the same paths are deployed at `https://documentation.actusfrf.org` | Preserve the sitemap bytes; replace only the recorded origin during acquisition; record every rewritten URL and never treat the placeholder host as live authority |
| ACTUS taxonomy breadth versus executable contract-type surface | The pinned taxonomy contains 32 rows, while the dictionary `contractType` term, technical specification, and public fixture files define 18 executable contract types | Publish a 32-row disposition matrix and a separate 18-type vector matrix; never count taxonomy-only or unavailable rows as implemented-and-vector-tested |
| ACTUS public vector corpus versus public Haskell test harness | The pinned corpus has 277 vectors across 18 types, but the Haskell suite declares 14 types, excludes seven fixtures, omits the analysis-date fixture, compares only three fields, and downcasts payoff to `Float` | Use the Haskell code as independent comparative evidence only; Moriarty must discover all 277, exclude none, preserve decimal intent, and compare every present ordered field |
| ACTUS core FOSS description versus public source access | The public core-license README describes the core as FOSS and publishes license terms, while the public service instructions require an authorization token for the Java core and CI uses a secret checkout token | Distinguish legal permission from practical source availability; do not bypass access controls or require private code; obtain license review before distribution or a conformance claim |
| ACTUS service dependency version versus README version | The service build pins `org.actus:actus-core:1.1.0`, while its README says the service includes core version 1.0.1 | Treat build configuration as the dependency evidence for the pinned commit and preserve the README as documentation drift; require exact version identity for any optional oracle run |
| ZKIR v3 baseline branch | The Moriarty ledger pin is `midnight-ledger` `ledger-8` (`a8ab82ba2124c36f92795c683e70bd888bc1d1fb`); the arc-zkir specification and Agda mechanization pin `ledger-9` commit `92e8bdd3a97b61b229e38916e1b180de6f448dd5` (`midnight-zkir-v3` 3.0.0), and arc-zkir's own `CLAUDE.md` cites `04c9c5d9` (3.0.0-rc.2) | Fetched `92e8bdd3` locally and extracted its `zkir-v3/src`; a K semantics targets the spec-pinned `92e8bdd3` surface and records the ledger-8 pin as the deployed-ledger scope; see [zkir-formal-spec-agda.md](zkir/zkir-formal-spec-agda.md) |
| ZKIR v3 instruction and type surface drift | `92e8bdd3` defines 34 instructions and 13 `IrType` variants; the standalone `midnightntwrk/midnight-zkir` at `2ffe2d17bbb736aec36fb300aeaca679a10d2278` defines 42 instructions (`reverse_bytes` renamed `reverse`; `slice`, `nth`, `concat`, `load_constant`, `sha512`, `and`, `or`, `xor` added) and 15 types (`Bool`, `Byte`, `Bytes(u32)` added, `Bytes32` removed); counts reproduced by enum variant count on 2026-09-03 | The mechanized specification covers only the 34-instruction surface; version every K claim by commit; the additional instructions are S5 unmechanized until arc-zkir tracks them; see [zkir-instruction-set.md](zkir/zkir-instruction-set.md) |
| Writer miscount of the 2ffe2d1 surface | Two agy-authored drafts reported 37 and 41 instruction variants at `2ffe2d1`; direct variant counting gives 42 | Pages corrected to 42; the count method is recorded in the contradiction above |
| Jubjub `from_coordinates` uses only the parity of `x` off-circuit | At `92e8bdd3`, `from_coordinates_offcircuit` calls the midnight-circuits `CircuitCurve::from_xy` for Jubjub, which compresses `(y, x mod 2)` and decompresses with `JubjubAffine::from_bytes`, so any `x` with the right parity yields the point with the real `x`; in circuit, `point_from_coordinates` pins the exact `(x, y)`. Reproduced with the K semantics and the Rust crate on the same preimage: both accept `(x+2, y)` off-circuit and return the true point, while the emitted constraint is violated (`experiments/zkir-k/tools/divergence_tests.py`, case `k01`; CLM-0730; evidence/zkir-k-divergence-tests-2026-09-05.txt; executed test; reproduced; high; S1) | Not in the arc-zkir review's thirteen findings; the Agda trust base states `fromCoordsJ-coordsJ` (a successful `fromCoordsJ x y` returns a point whose coordinates are exactly `(x, y)`), which the Rust off-circuit path does not satisfy. Report upstream; the K definition follows the Rust off-circuit behaviour and flags the gate |
| Bytes32 raw inputs outside the canonical form panic the crate | At `92e8bdd3`, `decode_offcircuit` for `Bytes32` uses `assert_eq!` on the low element's byte 31 and the high element's upper bytes (`ir_instructions/encode.rs`), so a well-formed preimage with such an element aborts the process instead of returning an error; the K definition reports a `panic` status at the same point (`experiments/zkir-k/tools/divergence_tests.py` case `k03`; CLM-0741; evidence/zkir-k-divergence-tests-2026-09-05b.txt; executed test; reproduced; high; S1) | Candidate robustness finding for upstream (same class as K2, the short-transcript panic); recorded by both an 8-reviewer report (fable-R2 F5, astra-R1) and the test |
| `jubjub_scalar_from_native` and native `from_coordinates` use the Jubjub chip that `used_chips` does not enable | `used_chips` enables the Jubjub chip only for Jubjub-typed inputs and public/private inputs and for `hash_to_curve`; the circuit arms of `jubjub_scalar_from_native` and of `from_coordinates` on two natives call `std.jubjub()`, whose accessor panics when the chip is absent. Reproduced with the K checker (`synthErr: chip not initialised`) while `preprocess` succeeds (cases `k04`, `k01b`; CLM-0742; evidence/zkir-k-divergence-tests-2026-09-05b.txt; executed test; reproduced; high; S1) | The review's finding 13 calls `from_bytes32` the only such entry; these are two more. Report upstream as an extension of finding 13 |
| `less_than` with 253 or 254 bits is accepted by `preprocess` but cannot be built | The chip pads the width to `max(bits + bits mod 2, 4)` and `bounded_of_element` asserts it is at most 253 (`midnight-circuits` `native_gadget.rs`); off-circuit only `bits >= 255` is rejected. Reproduced: both K and the crate accept `less_than` with 253 bits off-circuit, the K gate reports a synthesis error (case `k05`; CLM-0743; evidence/zkir-k-divergence-tests-2026-09-05b.txt; executed test; reproduced; high; S1) | Finding-10 class (accepted by preprocess, keygen panics); report upstream; the K static check keeps the specification's bound of 255 and the gate reports the chip limit |
| Extension `test_eq` on byte strings of unequal length | At `midnight-zkir` 2ffe2d1, `test_eq_offcircuit` compares two `Bytes` vectors and returns false for different lengths, while the in-circuit arm requires equal lengths (`eq.rs`); the first K extension rejected the pair as a type error (fable-R4, astra-R4), corrected on 2026-09-05 to return false off-circuit and a synthesis error in the gate (CLM-0744; experiments/zkir-k/semantics/zkir-ext.k; repository observation; reproduced; high; S1) | An off-circuit/in-circuit divergence of the newer crate, not covered by the review, which is pinned at 92e8bdd3 |
| Non-canonical Bytes32 element: panic at 92e8bdd3, error at 2ffe2d1 | The element of K3 (low element at or above $2^{248}$ or high element at or above 256) aborts `decode_offcircuit` at `92e8bdd3` but is an ordinary error at `midnight-zkir` 2ffe2d1, whose `decode_bytes` returns `None`; the definition models both (`decPanic` on the base surface, `#canonical(decPanic(_), _, bytes32()) => decErr(...)` under strict decoding). Reproduced on `test_bytes32_proof.zkir` with raw inputs `[1,1,0,1,0,2^248,0]`: K `--ext` and the 2ffe2d1 oracle both report a decode error, K and the 92e8bdd3 oracle both panic (CLM-0750; experiments/zkir-k/tools/unit_values.py check `dec bytes32 bad high, strict` and experiments/zkir-k/docs/13-known-divergences.md; executed test; reproduced; high; S1) | K6: the newer crate already rejects the element cleanly; the first extension definition inherited the base panic and was corrected on 2026-09-05 |
| Foreign field limb layout on the type-system page | [zkir/zkir-type-system.md](zkir/zkir-type-system.md) described `Secp256k1Base`, `Secp256k1Scalar` and the point coordinates as a 248-bit lower limb and an 8-bit upper limb; at `92e8bdd3` the value minus one is split into four 64-bit limbs, three packed into the first element and one in the second (five 51-bit limbs for the Curve25519 scalar), as `zkir-values.k` implements and the register encodings compared with the crate confirm (CLM-0751; experiments/zkir-k/semantics/zkir-values.k, midnight-circuits 7.2.4 field/foreign/field_chip.rs, evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt; executed test; reproduced; high; S1) | Page corrected on 2026-09-05; the earlier text is superseded |
| Source identifier collision between two workstreams | Two branches each allocated `SRC-0023` to `SRC-0026`: the main line to the CAKE, NEAR Intents and Moriarty intent-research sources, the ZKIR K line to the K monorepo, the kframework.org crawls and arc-zkir; discovered at the merge of `zkir-k-semantics` into main on 2026-09-05 (CLM-0758; evidence/source-inventory.csv; repository observation; reproduced; high; S1) | The ZKIR K line's four sources were renumbered to `SRC-0036` to `SRC-0039` across the wiki and the inventory at merge time; the raw reviewer transcripts under `experiments/zkir-k/review-2026-09-05/reports/*.events.jsonl` keep the old numbers as recorded; identifiers are allocated from the inventory's current maximum, never from a branch's own count |
| Extension `load_constant` of a Jubjub value with no Jubjub input or transcript entry | At `midnight-zkir` 2ffe2d1, `used_chips` enables the Jubjub chip only from the input types and the `public_input`/`private_input` types; a program whose only Jubjub value comes from `load_constant` passes `preprocess` (K and the crate both succeed) while the K gate reports `synthErr: chip not initialised for JubjubPoint`, the target contract fails `chips.gating`, and the crate's circuit panics at synthesis with `ZkStdLibArch must enable jubjub` under the MockProver (CLM-0764; experiments/zkir-k/corpus/divergence/k08_load_constant_jubjub_chip.zkir and experiments/zkir-k/plan-iter3/upstream-issues/K7.md; executed test; reproduced; high; S1) | K7: same class as K4, on the extension surface only; found by the target contract's chip-gating obligation and confirmed by the circuit oracle; draft issue written |
| `reconstitute_field` with `bits` 0 | At `92e8bdd3` (and `2ffe2d1`) `IrSource::load` accepts `bits: 0`; `preprocess` then rejects every value ("Excessive bit bound", the divisor bound becomes 255 bits) and key generation panics in the decomposition chip's assertion rather than returning a synthesis error; the definition's static check and the target contract now reject the program up front (`wf` and the both-stage obligation `width.reconstitute_field.assertion`, which also covers 255 bits and above, where the same assertion fails, while 249 to 254 bits key), reproduced with the circuit oracle in keygen mode (CLM-0770; experiments/zkir-k/corpus/handmade-negative/reconstitute_bits_0.zkir and experiments/zkir-k/plan-iter3/upstream-issues/K8.md; executed test; reproduced; high; S1) | K8: availability, same class as K5; found by the third-iteration audit of the target contract; draft issue written |


## CLM-0188: ACTUS business-day wording and PCD version limits

**ACTUS source contradiction (SRC-0029, direct inspection 2026-09-06).** At
techspec commit `94ef09e4992f79d573f84f41d8480f557365870e`,
`actus-techspecs.tex` around lines480–495 describes shifting to a non-working
day. At dictionary commit `356f7663f26091105cc4fef4ae3496942dcf0ebf`,
`actus-dictionary.json`, `businessDayConvention`, describes shifting to a
business day. The dictionary also repeats acronym SCMP for its final option,
where the identifier distinguishes the calculate/shift case. Preserve original
identifiers/spellings; do not silently generate a parser from these acronyms.

Provisional disposition (recommendation): use the dictionary's business-day
meaning as the interpretation to test, keep calculation and payment dates
separate, and confirm against relevant fixtures and independent semantics in
the next target study. This is not yet a verified compatibility resolution.
Authority normative source conflict; scope pinned ACTUS source interpretation;
reproduction direct source read only; confidence high that the conflict exists,
unknown for complete convention coverage; lifecycle S2 design obligation.

**HyperNova version limit (SRC-0040).** The acquired CMU author PDF, SHA256
`4c3319b2cf751fd49dea527a7a719099d1d21656b77e3258e5c0fa1f9ffbd9fb`,
does not contain the NovaBlindFold name present in latest ePrint metadata read
by the practical-research lane. The mirror's exact revision is unstated.
Disposition: derive claims from the acquired bytes only; do not transfer the
latest revision's sections or security conclusions onto that mirror. This is a
version mismatch, not evidence that either construction is incorrect. Authority
primary descriptive research; scope source-version correspondence; confidence
high for the bounded limitation; reproduction PDF read, no implementation;
lifecycle S2 research input.

**Recovery precedence (SRC-0041).** Prior Candidate A continuation prose remains
historically accurate about unfinished work but conflicts with the latest task
order. The 2026-09-06 user reset controls. New footguns and roadmap notices
supersede automatic continuation without rewriting the old failed-run evidence.


## CLM-0190: ACTUS design-study source gaps and DeFi model scope

SRC-0042's [target study](../docs/research/2026-09-06-actus-defi-design-study.md)
records DS-01 through DS-07. Business-day interpretation now has pinned Haskell
and pam08/pam09 support, but the missing CSMP vector prevents a conformance
claim. ANN initial Prnxt contains an incomplete formula; COM has quantity/sign
and TD/PRD reference discrepancies; CLM redemption references and FXOUT event
naming need explicit rules. Each has a proposed disposition and an independent
comparison obligation. No source bytes were rewritten.

The DeFi study also prevents filename/family overclaims: the Hyperliquid model
covers Bridge2 rather than the exchange, Derive's inspected model takes an
empty-option path, and Hegic accepts a bounded supplied payoff. The 72-row matrix
keeps these omissions and protocol-specific requirements visible. These are
pinned-source boundaries, not allegations about current deployed systems.

Metadata: SRC-0042; authority primary and comparative pinned source observation;
date 2026-09-06; scope proposed package semantics, S2; reproduced by source reads,
not executed conformance; confidence high for the named discrepancies and
medium for proposed corrections. Remaining formula/numeric questions stay open
for implementation acceptance.


## CLM-0192–0193: PCD report scope versus Moriarty and Midnight

**Optional recursion versus mandatory history.** SRC-0043's generic transaction
roadmap recommends delaying mandatory recursion. SRC-0041/SRC-0046 require PCD
in Moriarty acceptance. Disposition: simple subclaims may use ordinary checks,
but every accepted Moriarty state transition retains required history evidence
and constrained genesis. Optional accelerator fallback cannot remove it.

**Compact recursion versus native proof recursion.** SRC-0043 lines 370, 857 and
its maturity table cite prohibited source recursion. SRC-0045 demonstrates
native recursive verification/IVC code in pinned midnight-zk. These are different
layers. Disposition: correct the inference, prioritize native Midnight, and
retain explicit application/ledger compatibility gates. Negative lexical
searches do not prove universal API absence. Do not silently alter the raw report.

Metadata: observed 2026-09-06; authority secondary report, normative user input,
primary pinned source and design inference; S2 disposition; no proof deployment
reproduced; confidence high for scope distinction, unknown for adapter feasibility.


## CLM-0203: Faucet endpoints and historical network names

The current quickstart/network/funding guides link Nethermind faucets. The
previous workflow used faucet.*.midnight.network. At the captured time the
Nethermind Preview page returned503; the other checked faucet pages and health
endpoints responded. The two Preprod frontends have different API contracts.
Disposition: preserve all source/live responses, prefer the matching documented
UI for the existing Preprod wallet, and do not infer token delivery from health.
No root cause for the user's invalid-address error is established.

The 1010 troubleshooting page still uses TestNet02 as a network-mismatch example,
while the network-selection guide explicitly retires testnet-02. Treat the former
as historical diagnostic context; configure preview/preprod from the current
network reference. Metadata: SRC-0055/SRC-0056, observed2026-09-07; primary source
and experiment observation; S3 investigation, acquisition/live reads reproduced,
public settlement not reproduced. Details and disposition are in the
[review](../evidence/midnight-network-review-2026-09-07/README.md).


## CLM-0921: Report roadmaps versus Midnight product requirements

SRC-0070 recommends optional ZK and deferred Compact dependency (intents lines 2012-2016), public initial intent semantics (1922-1954), and other-chain backends. SRC-0071 recommends selective recursion for a generic first transaction standard (PCD lines 1113-1117). SRC-0072 permits unbounded numeric domains (DeFi line 664) and later optional proof integration (1138-1140).

Disposition: the user's Midnight-centric language, mandatory PCD, bounded semantics and private successor handoff control. Preserve useful semantic requirements and comparative financial cases. Do not import NEAR/EVM/Cardano adapter work, optional-history acceptance or unrestricted numeric domains into Moriarty. The [amendment](../openspec/REPORT-RECONCILIATION-2026-09-07.md) records owners and early decisions. This extends CLM-0192/0193 without rewriting the source reports.

Metadata: observed 2026-09-07; secondary report statements versus normative user input; S2 explicit design disposition; report review reproduced, external report citations and product implementation not reproduced; confidence high for the conflict and chosen scope.
