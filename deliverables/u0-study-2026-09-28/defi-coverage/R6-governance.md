# R6 — DeFi coverage review: governance and protocol administration

**Reviewer:** R6 of nine. Read-only review. This file is the only thing written.
**Date:** 2026-09-28.
**Status:** review finding. Not an accepted change, not a closed U0 predicate. Under
`AGENTS.md:96-99`, work is accepted by evidence, not by review.

---

## 1. Scope and pins

### Commits

| Repo | Command run | Output |
| --- | --- | --- |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` |
| Moriarty | `git -C /home/charl/Moriarty log -1 --format='%H %ad %s'` | `8f73784042bd692733c296d0d49f5173be96725e Wed Sep 23 21:00:14 2026 -0600 U0 T7: add U0 exit gate` |
| defiformal | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |

### Startup

The host did not expose `moriarty-dev:develop`. I read and applied the checked-in skill
at `plugins/moriarty-dev/skills/develop/SKILL.md`, plus `AGENTS.md` and `docs/FOOTGUNS.md`,
before forming conclusions, as `AGENTS.md:10-13` requires.

I ran the guarded CLI status, which is the one command `AGENTS.md:14-18` requires:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Actual output (abridged to the fields that matter here): `capability`
`SP01.6 loan-swap-subset`; `blockedAction` `implementation/repair of loan-swap-subset`;
`reason` `operational history is unresolved or unverified`; `missingEvidence` includes
`binding-input-stale`, `candidate-input-stale`,
`current-accounting-missing:.moriarty-dev/runtime/current-accounting.json`,
`resource-live-state-unavailable:sp01-loan-swap-grok-01`, `operational-history`;
seven `pendingTransactions` IDs. I dispatched no campaign action and edited no repository
source. Note for the other reviewers: the 2026-09-28 study's §11 recorded that no reviewer
could run `status` (`UNIFIED-PROPOSAL.md:287`). It ran for me in this session.

### What I read

**defiformal (abstraction level only, no Lean proof engineering):**
`README.md`; `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` (the whole Atlas, §1–§18 and Appendix A–B);
`lean/README.md`; `lean/DefiKernel/Typed/Authority.lean` (in full);
`lean/DefiKernel/Typed/AuthorityTests.lean` (test names);
`lean/DefiKernel/Composition/Execution.lean:20-60`;
`lean/DefiKernel/CapabilityProvenance/Origins.lean:1-70`;
`openspec/ROADMAP.md` (all 37 sprints); `corpus50/vocab.md`; `corpus50/lanes/*.json`
(element frequencies and the three lanes' `completeness_verdict` / `vocabulary_gaps`).

**Moriarty:** `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md`;
`deliverables/u0-semantic-contract-2026-09-23/{judgments,enforcement-map,trust-premises}.json`
and `EXIT-GATE.md`;
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`;
`openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md`
(UNI-015); `ROADMAP.md`; `docs/MORIARTY-PRODUCT-CONTRACT.md`;
`docs/MORIARTY-CONSOLIDATED-DESIGN.md`; `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`;
`docs/MORIARTY-BACKEND-REQUIREMENTS.md` (ZR15 rows);
`experiments/moriarty-language/spec/successor/semantic-contract.md` and
`financial-agreement-source-v5-grammar.ebnf`;
`experiments/moriarty-language/src/successor/financial-lifecycle.ts` (types and action kinds);
`wiki/defi-kernel-sdk-interface.md`; `wiki/security.md`; `wiki/defiformal-taxonomy.md`;
`deliverables/defi-language-design-2026-09-07/{LANGUAGE-DESIGN.md,action-targets.csv}`;
and the archived
`evidence/moriarty-completion-program-2026-09-07/SP01/successor-contract-01/candidate-01/experiments/moriarty-language/spec/successor/semantic-contract.md`.

### What I executed, and what I did not

**Executed.** The guarded `status` above. One Python aggregation over
`corpus50/lanes/*.json`, counting element symbols across all protocol entries; it reported
`protocols 72` and the counts quoted in §2.7. Ordinary `grep`, `sed`, `awk` and `diff`
reads, including `grep -c -E "Activate|Pause|Revoke|Migrate|Genesis|Observe"` on the current
`experiments/moriarty-language/spec/successor/semantic-contract.md`, which returned **0**.

**Not executed.** No `lake build` and no Lean evaluation in defiformal — every Lean claim
below is read from source text, not from a run. No Moriarty U0 checker
(`scripts/check_u0_*.py`), no TypeScript evaluator run, no K run, no proof, no network
submission, no campaign dispatch. Where I report a checker's exit code I am quoting
`EXIT-GATE.md`, which I did not re-run.

---

## 2. What the kernel abstraction requires for this category

The defiformal project has **two** abstraction layers, and they disagree about governance.
That disagreement is the most important thing in this section, so I state it first.

### 2.1 The Atlas treats control and authority as a first-class element group

`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:299-306` defines group **G11 — Control and authority**,
whose role boundary is literally the question this review asks: *"who may change what, and
when?"* Its four core elements:

| ID | Sym | Name | Definition (verbatim) |
| --- | --- | --- | --- |
| E036 | `Tg` | Delayed-governance execution | "A timelock between authorization and executability" |
| E037 | `Up` | Mutable implementation proxy | "Code replacement changes the reachable state machine" |
| E038 | `Gp` | Emergency guardian or pause | "Bounded suppression of reachable transitions" |
| E050 | `Au` | Delegated execution scope | "Persistent policy bounding the calls, assets, destinations, values, chains and time windows a delegate may reach" |

All four sit in stratum **S4**, "Multi-agent, cross-domain or mutable-control coordination"
(`:420`). Adjacent to them: `Fz` administrative freeze / forced transfer (`:352`), `Gs`
sponsored-fee liability (`:349`), `Fd` surplus and fee distribution (`:354`) as candidates,
and `Ve` vote-escrow allocation as *provisional* (`:385`) — so token voting itself is not
yet an admitted element, only the timelock/upgrade/pause/delegation machinery around it.

### 2.2 The admission criterion was rewritten specifically to admit these

Criterion **B** at `:105` reads: an element "changes claims, obligations, allocation,
valuation or settlement; **or** it changes which claim-changing transitions are reachable,
and does so as persistent, independently parameterised policy state." `:148-152` says
plainly what that second clause is for: *"This is what criterion B's second clause
discriminates: `Up`, `Gp` and `Tg` carry persistent, independently parameterised policy
over reachability; a hook does not."*

**This is the abstraction the category depends on.** A model that only expresses
transitions over *claims* cannot express these; it needs state whose effect is over the
*reachability* of other transitions.

### 2.3 The three-way discrimination

`:992` gives the contested-boundary rule that any model of this category must reproduce:

> `Au` vs `Up` vs `Gp` — Changes *who may act* = `Au`; changes *what code runs* = `Up`;
> *suppresses* transitions = `Gp`.

These are three different objects. A capability system gives you only the first.

### 2.4 The required-bond laws for the category

| Law | Text (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md`) |
| --- | --- |
| L13 | `Ex → freshness validation`; "`Gp` strongly preferred for high-value obligations" (`:753`) |
| L15 | `Up → Tg \| bounded emergency process` (`:755`) |
| L17 | `Au → bounded scope + revocation + expiry + nonce/domain separation` (`:757`) |
| L28 | `Fz →` named authority + enumerated triggers + appeal/reversal path + holder disclosure (`:768`) |
| L29 | `(In\|Ba\|Rf\|Of) →` a declared surplus-allocation rule naming the residual claimant (`:769`) |

L29 plus the `obligor` discriminator value `protocol-treasury` (`:506`) plus candidate `Fd`
(`:354`) are the Atlas's treasury-spending surface.

### 2.5 The hazard rules for the category

| # | Combination | Class | `:line` |
| --- | --- | --- | --- |
| X6 | Borrowable voting power + immediate execution (flash **or** slow accumulation) | H | `:794` |
| X9 | `Up` with immediate single-key control | H (many survivors) | `:797` |
| X16 | Unbounded delegated authority — `Au` without scope bounds, **or unlimited token approvals** | H | `:805` |

`:829-837` concedes these rows contain undefined terms and that "X9 as written passes a
correlated low-threshold multisig" — i.e. the Atlas knows its multisig/committee predicate
is missing, and registers converting "signer threshold *and administrative-domain
independence*" into a measurable predicate as open work.

### 2.6 Authority is carried by a typed *bond*, not by a permission check

`:539` defines the **trust bond** `—t→` by exactly the governance question: *"Who can
attest, censor, upgrade, pause, mint, freeze?"* with failure mode "Key compromise,
malicious upgrade, false attestation". `:567` is explicit: *"authority is a bond property
carried by `—t→`"* — which is why the Atlas declined a fifth bond type for it.

`§17` adds **governance reaction speed** as a reaction condition over `Tg` `Gp` `Up` —
*"Fast enough for emergencies, slow enough for oversight?"* (`:944`) — and `§16.3` names
"governance abandonment (`Up`, `Gp`, `Tg`)" as a decay channel (`:914-915`).

The composite obligation is pre-screen **row 1, Authority**, `:1020`:

> Every `Au`, `Up`, `Gp`, `Fz`: scope bounded, revocation, expiry, domain separation?
> Signing topology and key custody enumerated? — *Fails against L17, L15, L28, X9, X16*

`:1003-1005` records that v1.0 **reordered the screen by expected loss** and moved authority
to the front, "because v0.1 put an uncalibrated heuristic first and buried authority and
conservation — the paths that dominate actual loss." `:1049-1051` extends row 1 to the
signer set and key concentration of *every* trust-bearing party.

### 2.7 The empirical weight of this category, from defiformal's own corpus

I aggregated element symbols over `corpus50/lanes/*.json` (72 protocol entries). The two
most frequent symbols in the entire corpus are governance elements:

```
Up 49   Gp 48   Sh 41   Em 34   Ex 31   Tg 30   Ct 28   Xf 24  …  Fz 9  …  Ve 2  Au 1
```

`Up` (mutable implementation proxy) and `Gp` (emergency guardian/pause) appear in roughly
two thirds of the corpus — more often than pro-rata share accounting. `Tg` is sixth. On
defiformal's own evidence, administrative control is the single most pervasive mechanism
class in DeFi.

Three further corpus findings bear directly on the harder half of this category:

- **Lane 1** names as its single highest-priority vocabulary gap (`vocabulary_gaps[0]`):
  *"Rate model and rate policy — a utilization-indexed or **governance-set** price of
  credit, and rate-setting as a peg-defence instrument. Absent and load-bearing in all 12
  lending and CDP protocols. Highest priority by a wide margin."* That is
  **parameter-change-as-governance**, and the Atlas cannot express it.
- **Lanes 1 and 2** both name *"Delegated allocation mandate — a named agent
  (curator/allocator/strategist) with discretionary authority over other people's deposits,
  bounded by supply caps, a timelock and a performance fee."*
- **Lane 3** on fiat stablecoins: *"the model can see the freeze key and the upgrade proxy;
  it cannot see the reserve, the obligor, the custodian, or the bank."* The governance
  surface is the part that *does* survive.

### 2.8 The Lean kernel does **not** implement the Atlas's governance abstraction

This is a finding the other reviewers should carry. The kernel's capability model is:

```lean
inductive Right (Party Asset Domain : Type) where
  | invoke
  | debit (cell : Cell Party Asset Domain)
  | changeSupply (domain : Domain) (asset : Asset)
```
`lean/DefiKernel/Typed/Authority.lean:8-12`

Three rights: invoke an operation, debit a balance, change a supply. **There is no right
over the rules.** The rule-set itself is immutable configuration:

```lean
/-- The adapter supplies this immutable configuration, separately from caller requests. -/
structure AuthorityConfig (Party Domain : Type) where
  domainAdmin : Domain → Party
  operationDomain : OperationId → Option Domain
```
`Authority.lean:32-35`

`domainAdmin` is a *total function fixed for the run*. No operation changes it — there is no
`changeAdmin`, no admin transfer, no quorum. `Authority.lean:5` scopes the model out
loud: *"Authentication, registry truth, allowances and replay prevention are outside this
model."* `README.md:162-165` repeats it: *"Caller contexts, registries, administrators, and
initial capability stores are trusted model inputs. Grants express administrator authority,
which does not by itself establish an account owner's consent."*

The composition layer's administrative vocabulary is two steps:

```lean
inductive Step (Party Asset Domain : Type) where
  | invoke (invocation : Invocation Party Asset Domain)
  | issue (grant : Grant Party Asset Domain)
  | revoke (id : CapabilityId)
```
`lean/DefiKernel/Composition/Execution.lean:27-30`

Issue and revoke, both of *balance/invoke/supply* rights only. No `upgrade`, no `pause`, no
`setParameter`, no delay. Issuance is immediate (`Authority.lean:65-74`); revocation is
immediate and idempotent (`:77-85`). There is no `Tg` — no gap between authorization and
executability anywhere in the kernel.

`CapabilityProvenance/Origins.lean:9-16` makes the root of authority an *explicitly supplied
premise* (`TrustedRoot`, `RootsAccepted`), "Not inferred from well-formedness or final-store
membership" — an honest statement that where authority comes from is assumed, not derived.

Finally: I read all 37 sprints in `openspec/ROADMAP.md`. **None is a governance sprint.**
`grep -n -i -E "governance|timelock|upgrade|pause|guardian|admin|parameter change|treasury|quorum|vote"`
over that file returned nothing. P05 is "Capability provenance and isolation" — provenance
of the existing three rights, not authority over the rules.

**Net requirement statement.** The Atlas says this category needs (a) persistent policy
state over *reachability*, (b) the Au/Up/Gp three-way split, (c) a delay between
authorization and executability, (d) bounded delegation with revocation, expiry and domain
separation, (e) a typed trust relation naming who may upgrade/pause/freeze, (f) enumerated
signer topology and key concentration, (g) a named residual claimant for surplus, and
(h) the effect of an administrative change on positions already open. The defiformal *Lean
kernel* supplies (d) partially and nothing else, and its own roadmap does not schedule the
rest. So "does Moriarty cover what the kernel abstraction requires" has to be judged against
the Atlas, not against the kernel — and the kernel is a second data point that this is hard,
not a baseline Moriarty is behind.

---

## 3. Coverage verdict per requirement

Evidence grades used below, strictest first: **demonstrated** (executed evidence in the
repo) > **implemented** (code in the evaluator) > **specified** (a normative requirement
with a scenario) > **designed** (prose or a named field) > **named** (an identifier only).
A schema field name is `named`, not coverage. A checker exiting 0 is not a capability.

| # | Requirement (from §2) | Verdict | Where in Moriarty (file:line) | What is missing |
| --- | --- | --- | --- | --- |
| G1 | **Authority over a balance/budget** — `Au`-shaped spending scope bounded by value, assets, recipients (Atlas `:306`; kernel `Right.debit`) | **partial** (implemented at host, unenforced natively) | `stage-relation.schema.json:452-470` (`authority.{consumed,remaining,replayState}`); `:146-169` (`assetIdentities`, `recipients`, `grossDebitCap`, `feeCap`, `minNetOutcome`); `judgments.json` authority judgment ("stay inside the signed budget"); `experiments/moriarty-language/src/successor/financial-lifecycle.ts:50-55` (`Allowance{party,asset,remaining,spent}`), `:1463-1475` `MISSING_ALLOWANCE`/`INSUFFICIENT_ALLOWANCE`, `:1521-1522` decrement; `UNIFIED-PROPOSAL.md:93` laws A1/A2 | All three `authority.*` leaves are typed `"string"` with no pattern — the study's own F4 (`UNIFIED-PROPOSAL.md:35`) calls authority "opaque strings". `enforcement-map.json` rows `authority.consumed`/`remaining`/`replayState` are `NOT_ENFORCED` with `mechanisms: []`. The evaluator's `Allowance` has no grantor, no expiry, no revocation flag |
| G2 | **L17 completeness** — `Au →` bounded scope **+ revocation + expiry + nonce/domain separation** (`:757`) | **partial** | scope: as G1. expiry: `signedIntent.validity` (`schema:170-172`). nonce: `signedIntent.replayPolicy` (`:173-175`). domain separation: `domain.{chainId,domainId}` (`judgments.json` stage judgment). revocation: `MORIARTY-PRODUCT-CONTRACT.md:56` ("Signature, authority, nonce and **current revocation** → Owner authorization"); `MORIARTY-CONSOLIDATED-DESIGN.md:70` (recovery grants carry "scoped revocation"); `wiki/defi-kernel-sdk-interface.md:23` | **Revocation has no schema field at all.** Three of the four L17 conjuncts are named leaves; the fourth exists only in prose. `validity` and `replayPolicy` are also opaque strings, and both are `NOT_ENFORCED` in `enforcement-map.json` |
| G3 | **X16 — unbounded delegation is a hazard** (`:805`) | **partial (designed)** | Bounds exist per G1. `UNIFIED-PROPOSAL.md:74` sets `delegation=none` for slice S0; `schema:143-145` `delegationPolicy` is a bare string | No delegation grammar, no delegate principal, no sub-delegation depth, no "unlimited" rejection rule. `enforcement-map.json` `signedIntent.delegationPolicy`: `NOT_ENFORCED`, note "Signed-intent authentication is absent from the circuit, ledger-primitive, and other native-boundary code". ROADMAP.md:26 assigns OWS delegation to **U5** |
| G4 | **`Gp` — bounded suppression of reachable transitions** (emergency pause / guardian) (`:305`) | **absent** | Nothing in the current design surface. `financial-agreement-source-v5-grammar.ebnf:26-32` admits only `unit`/`party`/`asset`/`record`/`operation`/`state`/`action` declarations. Evaluator action kinds are `Transfer`/`Repay`/`Originate`/`Accrue` only (`financial-lifecycle.ts:86,95,104,120`) | Everything. Historical only: the archived candidate04 had `Activate`/`Pause`/`Revoke`/`Migrate` at `evidence/moriarty-completion-program-2026-09-07/SP01/successor-contract-01/candidate-01/experiments/moriarty-language/spec/successor/semantic-contract.md:360`, mapped to DA22. `grep -c -E "Activate\|Pause\|Revoke\|Migrate"` on the **current** `experiments/moriarty-language/spec/successor/semantic-contract.md` returns **0** (I ran it); that file is scoped "expression layer only" and says "The 38 proposed financial operations are not defined here" (`:3-9`) |
| G5 | **`Up` — code replacement changes the reachable state machine** (`:304`) | **specified, not designed** | `spec/consolidated-language-kernel/spec.md:157-166` **UNI-015 Semantic evolution**: "When program, verifier, policy, federation epoch or persistent state evolves, acceptance SHALL preserve signed semantics, consumption, recovery, privacy and obligations or require applicable amendment consent"; `docs/MORIARTY-BACKEND-REQUIREMENTS.md:29` **ZR15**; `MORIARTY-PRODUCT-CONTRACT.md:62` "Protocol upgrades and application governance must have explicit scope" | No construct, no schema leaf, no judgment clause. ZR15's own limitation column says "Full policy-preserving recursive migration is unknown. Immutable deployments are a valid initial implementation." `ROADMAP.md:26` puts ZR15/UNI-015 in **U5**; `MORIARTY-BACKEND-REQUIREMENTS.md:44` says "U4/U5" |
| G6 | **L15 — `Up → Tg \| bounded emergency process`** (`:755`), and `Tg` as a delay between authorization and executability (`:303`) | **absent** | — | Nothing anywhere in Moriarty expresses a *delay between authorization and executability*. `signedIntent.validity` is a validity **window** on an intent, not a queue-then-execute delay. There is no proposal/queue/execute lifecycle, no timelock, no veto window |
| G7 | **The Au/Up/Gp three-way split** (`:992`) | **absent** | — | Moriarty has one authority notion — a signer's consumable budget over value. It has no vocabulary to say "this changes what code runs" or "this suppresses transitions" as distinct from "this spends" |
| G8 | **Authority over the *rules*, and the effect of a rule change on in-flight obligations** | **specified (one requirement), not designed** | **UNI-015's hostile scenario is the sharpest fit anywhere in Moriarty** — `spec.md:164-166`: "**WHEN** An interface-compatible upgrade changes beneficiary or resurrects consumed authority. **THEN** Migration is rejected." Also `MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:45` "Changes to semantics, verifier, policy or federation epoch need preservation or applicable amendment consent; expiry and revocation do not erase existing debt"; `:33` "Stage authority must distinguish initiation, completion, reconciliation, recovery, disclosure and **amendment**"; `MORIARTY-CONSOLIDATED-DESIGN.md:70` "Revocation cannot erase outstanding duties"; `ZR15` acceptance test "authorized migration preserves consumed identities, outstanding debt and cumulative fees" | This is the one place Moriarty is *ahead* of both the Atlas and the Lean kernel — and it is one requirement plus three prose sentences. No `amendment` authority kind exists in `schema.json`; the six judgments (`judgments.json`) include no amendment or migration judgment; `authority.*` cannot express an authority *kind* at all. UNI-015 is scoped to **federation epoch** migration (`ROADMAP.md:26`), not to an application's own parameter change |
| G9 | **Parameter change under a fixed claim policy, affecting existing positions** (Atlas lane-1 gap: "governance-set price of credit"; pre-screen row 1) | **named only** | `deliverables/defi-language-design-2026-09-07/action-targets.csv:23` — **DA22**: `orthogonal, change parameters / pause / migrate, werner;gogol, 2 governance; III-D, bounded administrative action under fixed claim policy, cannot downgrade claims or reset work; affects existing positions, needs-pinned-policy-fixture`; `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:134` "Genesis and **administrative transitions** need explicit cases" | DA22's own disposition field says `needs-pinned-policy-fixture` — i.e. it has none. DA22 appears in no current spec, schema, judgment or evaluator path; the only implementation trace is the archived candidate04 row (see G4). `ROADMAP.md:27` puts "DeFi action rows" in **U6** |
| G10 | **Token voting, vote-escrow, delegation of voting power, proposal lifecycle (propose/quorum/vote/queue/execute)** | **absent** — and note the Atlas also has this as *provisional* `Ve` (`:385`) and as un-elemented | — | Nothing. No governance token, no voting power, no snapshot/checkpoint, no quorum, no proposal object. Moriarty's `party` declaration (`v5-grammar.ebnf:36`) is an opaque identifier with no role, weight or delegation |
| G11 | **X6 — borrowable voting power + immediate execution** (`:794`, Beanstalk) | **absent** | — | Requires G10 first. Moriarty has no notion of a power that is both transferable-as-value and authoritative-over-rules, so it cannot state the hazard |
| G12 | **X9 / signer topology — multisig, committee, quorum, key concentration, administrative-domain independence** (`:797`, `:833`, `:1045-1051`) | **partial, and scoped to federation** | `ROADMAP.md:26` (U5): "authorized federation policy naming membership, threshold, ordering, equivocation, availability and epoch-change rules"; `MORIARTY-CONSOLIDATED-DESIGN.md:97` "State that conditional enforcement boundary and the federation's membership, threshold, ordering, equivocation, availability and epoch-change rules explicitly … A foreign destination that accepts only a threshold signature can be bypassed if that threshold is compromised"; `MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:33` mentions "threshold conditions" | Threshold/committee authority exists only as a property of the **optional federated kernel**, not as an authority form a Mori program can carry. `schema.json:137-139` `signedIntent.signer` is a single string. There is no m-of-n signer set, no quorum predicate, no administrative-domain-independence notion. `MORIARTY-CONSOLIDATED-DESIGN.md:11` and TP04 (`trust-premises.json`, `accepted-assumption`) both say Moriarty can run without the federation — so a program without U5 has no committee authority at all |
| G13 | **`Fz` — administrative freeze / forced transfer, with `authority_source` and L28** (`:352`, `:509`, `:768`) | **absent** | — | No freeze, no clawback, no forced transfer, no appeal/reversal path, no holder-disclosure obligation. `MORIARTY-CONSOLIDATED-DESIGN.md:54` states the *opposite* default — "Creating a liability requires applicable consent from the party made liable" — which is a coherent design stance, but it does not express the freeze that L28 governs |
| G14 | **L29 / treasury — declared surplus-allocation rule naming the residual claimant** (`:769`, `Fd` `:354`, `obligor=protocol-treasury` `:506`) | **partial** | Fees are modelled as first-class: `schema.json` `effects.fees[].{account,asset,amount}`; `signedIntent.feeCap`; `UNIFIED-PROPOSAL.md:91` law E2 "net is derived from gross and fees"; `AGENTS.md:82` "fees count against net goals" | A fee **recipient account** is not a residual-claimant *rule*. Nothing names who owns undistributed surplus, and there is no treasury-spending authority (a spend *from* a protocol-owned account under a governance decision). `MORIARTY-CONSOLIDATED-DESIGN.md:43` mentions "custody/reserves" but the numeric reserve is unresolved (`UNIFIED-PROPOSAL.md:141-142`, `:267` "Do not declare a reserve type … before N1") |
| G15 | **Governance as a typed trust relation `—t→`** (`:539`, `:567`) | **partial (as premises, not as a relation)** | `trust-premises.json` TP01–TP09: TP03 "Issuers, oracles and signers remain explicit trust assumptions"; TP04 federation-optional; TP05 timeout-not-nonexecution; TP07 intent-auth-boundary. `schema.json` `observations[].{issuer,domain,time,finality}` types the *oracle* trust edge | Moriarty types the **informational/truth** trust edge well and the **authority** trust edge not at all. There is no field naming who may upgrade, pause, freeze or mint for a program. `wiki/security.md:39` treats "Governance capture — spec or registry changed without review" as a threat mitigated by *social* means ("Multi-party ownership, public MIPs, reproducible releases, delayed activation, signed registries"), with residual risk "Social-layer collusion cannot be eliminated technically" |
| G16 | **Recovery / emergency authority separate from ordinary authority** (nearest Moriarty analogue of `Gp`'s bounded-suppression role) | **specified** | `ROADMAP.md:24` (U3): "separate recovery authority"; `MORIARTY-CONSOLIDATED-DESIGN.md:70` "Ordinary authority can expire while a narrowly scoped recovery authority remains usable under its signed conditions. Recovery grants declare their own signed termination rule … Exclusive terminal outcomes consume/tombstone the applicable authority"; `schema.json:176-178` `signedIntent.recoveryPolicy` | Recovery is *party-scoped remedy*, not *protocol-scoped emergency*: it lets a counterparty unwind their own stage, not a guardian suppress everyone's. `recoveryPolicy` is an opaque string, `NOT_ENFORCED`. `UNIFIED-PROPOSAL.md:74` sets `recovery=none` in S0. Owner: **U3** |
| G17 | **Governance reaction speed as a bounded operating condition** (`:944`) | **absent** | — | No construct for "fast enough for emergencies, slow enough for oversight". Requires G6 first |
| G18 | **Is governance deliberately out of scope?** | **out-of-scope-by-design — and stated** | `wiki/defiformal-taxonomy.md:186-192`: "Solver search, bridge/DVN operation, sequencers, reserve custody, legal enforcement, identity providers, validator duties, oracle data acquisition, discretionary portfolio decisions, **and governance stay outside Core as explicit capabilities and assumptions.** Generated Compact/ZKIR can prove deterministic predicates and private authorization facts; it cannot turn those open-world dependencies into semantic guarantees." Supported by `AGENTS.md:54-58` ("Moriarty is a bounded financial language for Midnight"), `MORIARTY-PRODUCT-CONTRACT.md:62`, and `wiki/security.md:39` | The exclusion is **partial and one-sided.** The same page, two lines earlier (`:180-181`), says "The strongest DeFiFormal residue evidence supports a party or **authority sort** and a bounded allocation mandate" — i.e. an authority sort is wanted in Core. And UNI-015/ZR15 put *rule change* squarely **inside** the acceptance relation. So the documents exclude governance *processes* (voting, proposals, committee deliberation) while retaining governance *effects* (migration, amendment, parameter change) as in-scope obligations. No document draws that line explicitly |

### Count

| Verdict | Count | Rows |
| --- | --- | --- |
| `covered` | **0** | — |
| `partial` | **7** | G1, G2, G3, G12, G14, G15 (and G5/G8 are "specified" — counted below as partial-by-specification: see note) |
| `absent` | **7** | G4, G6, G7, G10, G11, G13, G17 |
| `named only` | **1** | G9 |
| `out-of-scope-by-design` | **1** (with a stated caveat) | G18 |

Note on counting G5 and G8: both are `specified` — a normative UNI/ZR requirement with a
scenario, but no design surface, no schema leaf, no judgment clause and no code. I count
them inside `partial` (giving 7) rather than `absent`, because a requirement with a hostile
scenario is materially more than nothing. If a stricter reviewer counts "specified with no
design surface" as absent, the split is **0 covered / 5 partial / 9 absent / 1 named /
1 out-of-scope**. I prefer the strict reading for G5, because ZR15's own limitation column
says "Immutable deployments are a valid initial implementation" — the requirement is
written so that doing nothing satisfies it.

**Nothing in this category is `covered`, and nothing is `demonstrated`.** Every
governance-adjacent schema leaf that exists —`authority.consumed`, `authority.remaining`,
`authority.replayState`, `signedIntent.consentPolicy`, `signedIntent.delegationPolicy`,
`signedIntent.recoveryPolicy`, `signedIntent.replayPolicy`, `failurePolicy.*` — is
`NOT_ENFORCED` with `mechanisms: []` in `deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json`.
That file's own limitation line records: "Signed-intent authentication was not found in a
circuit, a bound ledger primitive, or another native boundary under the declared native
roots." `EXIT-GATE.md:12` reports 0 enforced / 84 unenforced overall.

---

## 4. Category verdict

**Moriarty's language design does not cover governance and protocol administration, and the
absence is roughly half deliberate.** The deliberate half is stated at
`wiki/defiformal-taxonomy.md:186-192`, which puts governance outside Core "as explicit
capabilities and assumptions", consistent with `AGENTS.md:54-58` and
`MORIARTY-PRODUCT-CONTRACT.md:62`. On that reading, token voting, vote-escrow, proposal
lifecycles, quorum and treasury deliberation (G10, G11, G14's spend side) are legitimately
out of scope for a bounded language of signed financial stages, and a reviewer should not
score them as gaps. The undeliberate half is the part Moriarty's *own* documents claim: the
authority model. Moriarty's authority is one shape — a single signer's affine, consumable
budget over value, bounded by caps, recipients, assets, a validity window and a replay
policy (`stage-relation.schema.json:146-178`, `:452-470`;
`financial-lifecycle.ts:50-55`). That is a faithful instance of the Atlas's `Au`
(`UNIFIED-DEFI-ELEMENT-TABLE.md:306`) and of the kernel's `Right.debit`
(`Authority.lean:10`), missing only L17's revocation conjunct (`:757`). It has **no**
expressive power over reachability: no `Gp`, no `Up`, no `Tg`, and no way to state the
Atlas's own `Au`/`Up`/`Gp` distinction (`:992`). Since `Up` and `Gp` are the two most
frequent symbols in defiformal's 72-protocol corpus (49 and 48 occurrences, measured in
§2.7), a Moriarty program cannot describe the most common administrative fact about the
protocols it is meant to express. The one place Moriarty is genuinely ahead of both the
Atlas and the Lean kernel is the *consequence* question — UNI-015's hostile scenario
(`spec.md:164-166`) rejects an "interface-compatible upgrade [that] changes beneficiary or
resurrects consumed authority", and ZR15 requires an "authorized migration [to preserve]
consumed identities, outstanding debt and cumulative fees". That is exactly "what a rule
change means for in-flight obligations", and neither defiformal layer states it: the Atlas
asks the question only as a reaction condition (`:944`), and the Lean kernel's
`AuthorityConfig` is immutable by construction (`Authority.lean:32-35`), so the question
cannot arise.

**Milestone ownership on the evidence found.**

| Gap | Owning milestone | Evidence |
| --- | --- | --- |
| G16 separate recovery authority | **U3** | `ROADMAP.md:24` |
| G3 delegation (OWS), G12 committee/threshold authority, G5+G8 migration/amendment (ZR15/UNI-015) | **U5** (ZR15 also tagged U4/U5) | `ROADMAP.md:26`; `docs/MORIARTY-BACKEND-REQUIREMENTS.md:44` |
| G9 DA22 "change parameters / pause / migrate" | **U6**, by inference from "DeFi action rows" | `ROADMAP.md:27`; `action-targets.csv:23` |
| G1 authority typing (turning opaque strings into per-asset vectors) | **U0**, if the proposal is adopted | `UNIFIED-PROPOSAL.md:114` (S2: "Authority is per-asset vectors plus a replay-ID set") |
| G2 revocation as a schema leaf | **unassigned** | No document assigns it. Revocation appears in `MORIARTY-PRODUCT-CONTRACT.md:56` and `MORIARTY-CONSOLIDATED-DESIGN.md:70` but in no schema, judgment or milestone exit |
| G4 pause/guardian, G6 timelock, G7 the Au/Up/Gp split, G17 reaction speed | **unassigned** | No milestone exit in `ROADMAP.md:21-28` names any of them. The only trace is archived candidate04 (`evidence/…/semantic-contract.md:360`), which the current design does not carry |
| G13 freeze/clawback | **unassigned** | Not named in any milestone |
| G10, G11 voting / vote-escrow / proposals | **out of scope by design** | `wiki/defiformal-taxonomy.md:189` |
| G14 treasury spend authority | **unassigned**; fee accounting is U0/U2 | `UNIFIED-PROPOSAL.md:73` (literal fee line in S0); no residual-claimant rule anywhere |

Slice S0 forecloses this category for U0 explicitly and correctly: `delegation=none`,
`recovery=none`, `outcome=terminal` (`UNIFIED-PROPOSAL.md:74`). I do not recommend changing
S0. The gaps below are about the **frozen contract's shape**, which S0 instantiates but does
not define.

---

## 5. Gaps that would change the language design

Ranked by how much new language, kernel, judgment, numeric-profile or enforcement machinery
they require — not by urgency. A library or example cannot close any of these.

**1. There is no authority *kind*. Authority is a scalar budget where it needs to be a
typed, sorted thing.**
`stage-relation.schema.json:452-470` gives `authority` exactly three opaque string leaves.
`judgments.json`'s authority judgment reads them as one budget: "consumed authority,
remaining authority, replay state, and resource reservations stay inside the signed budget."
Moriarty's own alignment document already contradicts that shape:
`MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:33` says "Stage authority must distinguish
initiation, completion, reconciliation, recovery, disclosure and **amendment**" — six kinds,
of which the schema expresses zero. The Atlas's `Au`/`Up`/`Gp` split (`:992`) is the same
demand from the other side. This needs an **authority sort in Core** — which
`wiki/defiformal-taxonomy.md:180-181` independently recommends ("The strongest DeFiFormal
residue evidence supports a party or authority sort") — plus a corresponding judgment clause
and enforcement locus. The U0 proposal's S2 (`UNIFIED-PROPOSAL.md:114`) upgrades authority
to "per-asset vectors plus a replay-ID set", which fixes the *typing* of the value budget
and leaves the *kind* problem untouched. **This is the gap I would raise with the S2 author
before the freeze**, because a frozen authority shape with no kind field forecloses G5, G8,
G16 and DA22 in one move.

**2. No policy state over reachability, so `Gp` and `Up` are inexpressible — and this is
what the empirical corpus is mostly made of.**
Criterion B's second clause (`UNIFIED-DEFI-ELEMENT-TABLE.md:105`, `:148-152`) exists
precisely to admit persistent state whose effect is over *which transitions are reachable*.
Moriarty's state is financial: the v5 grammar's declarations are `unit`/`party`/`asset`/
`record`/`operation`/`state`/`action` (`financial-agreement-source-v5-grammar.ebnf:26-32`),
and the evaluator's actions are `Transfer`/`Repay`/`Originate`/`Accrue`
(`financial-lifecycle.ts:86,95,104,120`). `Up` 49 / `Gp` 48 out of 72 corpus entries (§2.7)
says this is not an edge case. A pause is not a library — it is a guard the *kernel* must
apply to every action, which makes it a Core change and an enforcement-map change, not an
example. The archived candidate04 design already had the shape
(`evidence/…/semantic-contract.md:360`: `Activate`/`Pause`/`Revoke`/`Migrate`, precondition
"admin authority; no work reset; no consumed revive"); recovering it is a design decision,
not new invention.

**3. No delay between authorization and executability (`Tg`), which is L15's only
non-emergency discharge.**
`UNIFIED-DEFI-ELEMENT-TABLE.md:755`: `Up → Tg | bounded emergency process`. Moriarty has
`signedIntent.validity` (a window in which an intent may be used) and `outcome.continuations`
(`schema:530-545`), but no object that is *authorized now and executable later*. This is a
genuinely new temporal shape: the acceptance relation currently binds authorization,
observation and effect into one stage (`MORIARTY-CONSOLIDATED-DESIGN.md:42`). Splitting
authorization-time from execution-time affects the stage relation, the history judgment
(does a queued action have a predecessor commitment?), and the replay model (can a queued
action be cancelled, and does cancellation consume the replay ID?). `AGENTS.md:83`'s rule
that "Residual duties survive partial progress" is the invariant a veto window would have to
respect.

**4. Single `signer`, so no multisig, committee or quorum authority in the language.**
`stage-relation.schema.json:137-139` is `"signer": {"type": "string"}`. Threshold authority
exists in Moriarty only as a property of the optional federated kernel
(`ROADMAP.md:26`, `MORIARTY-CONSOLIDATED-DESIGN.md:97`), and TP04 (`trust-premises.json`,
`accepted-assumption`) plus `MORIARTY-CONSOLIDATED-DESIGN.md:11` both say a program may run
without any federation — so an unfederated Mori program has no committee authority at all.
The Atlas demands the signer topology be enumerated as part of pre-screen row 1
(`:1020`, `:1049-1051`) and concedes X9 "as written passes a correlated low-threshold
multisig" (`:833`). Making `signer` a set with a threshold changes the signed-intent
digest, hence T6's in-circuit authentication decision (`UNIFIED-PROPOSAL.md:197-201`), hence
the ZKIRv3 cost estimate U1 owns. **This should be decided before T6 freezes**, because
an m-of-n digest is a different circuit from a 1-of-1 digest.

**5. Revocation is in the product contract but in no schema, judgment or enforcement row.**
`MORIARTY-PRODUCT-CONTRACT.md:56` lists "current revocation" as a condition of Owner
authorization; `MORIARTY-CONSOLIDATED-DESIGN.md:70` gives recovery grants "scoped
revocation" and states "Revocation cannot erase outstanding duties";
`wiki/defi-kernel-sdk-interface.md:23` correctly notes revocation cannot be instantaneous or
universal. None of this reaches `stage-relation.schema.json`, `judgments.json` or
`enforcement-map.json`. L17 (`:757`) makes revocation a required conjunct of *every*
delegation bond, and the Atlas's own kernel does implement it
(`Authority.lean:77-85`, idempotent, tombstoned, scope-retaining) — so this is the one
governance requirement where defiformal's Lean layer is ahead of Moriarty and supplies a
directly transplantable design, including the "revoked entries remain as tombstones"
decision (`Authority.lean:3-5`) that makes revocation replay-safe.

**6. `authority` cannot name an authority over the *rules*, so UNI-015 has nothing to
range over.**
UNI-015 (`spec.md:157-166`) and ZR15 (`MORIARTY-BACKEND-REQUIREMENTS.md:29`) require that
program/verifier/policy evolution "preserve signed semantics, consumption, recovery, privacy
and obligations **or require applicable amendment consent**". There is no amendment consent
object. `signedIntent.consentPolicy` (`schema:140-142`) is an opaque string and is
`NOT_ENFORCED`. Until authority has a kind (gap 1), "applicable amendment consent" cannot be
checked by anything, and UNI-015's positive scenario ("An authorized migration retains duty
and replay commitments") has no witness format. This is also the gap that would let Moriarty
state DA22's distinguishing test — "cannot downgrade claims or reset work; **affects
existing positions**" (`action-targets.csv:23`) — which is the precise question the Atlas
leaves open (`:944`, and lane 1's "governance-set price of credit" gap).

**7. No residual-claimant rule (L29), so treasury and surplus have accounts but no owner.**
`UNIFIED-DEFI-ELEMENT-TABLE.md:769` requires a *declared* surplus-allocation rule naming the
residual claimant, and Appendix A's worked example fails exactly this check
(`:1212-1214`, `:1238`). Moriarty types fee *lines* (`effects.fees[]`, `signedIntent.feeCap`)
but has no rule naming who owns the remainder. This interacts with the open numeric reserve
decision (`UNIFIED-PROPOSAL.md:141-142`, `:267`): the `protocol-reserve` remainder class,
if adopted, creates a protocol-owned balance with no stated spending authority. **N1 should
not define `protocol-reserve` without naming its residual claimant**, or it will create the
L29 defect in the numeric profile itself.

Gaps 1, 2 and 6 are the ones that change the *language*. Gaps 3, 4 and 5 change the *stage
relation and the signed-intent digest*. Gap 7 changes the numeric profile. G10/G11 (voting,
proposals, vote-escrow) I do **not** list as gaps: `wiki/defiformal-taxonomy.md:189`
excludes them, defiformal's Atlas holds `Ve` as merely provisional (`:385`), and I found no
Moriarty document that claims them.

---

## 6. Limits of this review

**Not verified — could not be, in a read-only pass.**

- I ran no Lean build in defiformal. Every statement about `Right`, `AuthorityConfig`,
  `Step`, `issueCapability`, `revokeCapability` and `TrustedRoot` is read from source text
  at the pinned commit. I did not check that those files compile or that the theorems quoted
  around them discharge.
- I ran no Moriarty U0 checker, no TypeScript evaluator, no K semantics. Where I state
  "0 enforced, 84 unenforced" I am quoting `EXIT-GATE.md:12` and `:24`, which I did not
  re-run. Per `UNIFIED-PROPOSAL.md:264`, a checker exit of 0 is not coverage, and I have not
  treated it as such.
- I did not verify that `experiments/moriarty-language/src/successor/financial-lifecycle.ts`
  is the evaluator actually reached by any current entry point. I read its types and its
  allowance branch; I did not trace a production path from a public entry point to an
  observable result, which `docs/FOOTGUNS.md:41-46` requires before reporting candidate
  success. I am therefore **not** reporting any of this as a working capability.
- I did not read `formal/k/`, `algebra/MODEL.md` beyond a grep (which returned no
  governance/authority hits), `docs/unified-v0.1.md`, `wiki-llm/`, `corpus/normalized/`, or
  the six per-lens study briefs in `deliverables/u0-study-2026-09-28/`. A governance
  abstraction could exist in one of those and be missed. My `grep` over
  `docs/unified-v0.1.md` was not run; the v0.1 document is superseded by the v1.0 Atlas per
  its own header (`UNIFIED-DEFI-ELEMENT-TABLE.md:1-19`), so I treated v1.0 as controlling.
- I did not check whether the eight other reviewers' categories overlap G12 (committee
  authority) or G15 (trust edges); a federation or cross-domain reviewer may reach different
  conclusions about U5's coverage of threshold authority.

**Asserted by inference, not by a document.**

- **That G4, G6, G7, G13 and G17 are unassigned.** I read all eight milestone rows
  (`ROADMAP.md:21-28`) and found no exit criterion naming pause, timelock, freeze or the
  Au/Up/Gp split. Absence from the rows I read is not proof that no document assigns them.
- **That DA22 is owned by U6.** `ROADMAP.md:27` says U6 covers "DeFi action rows"; it does
  not name DA22. The link is mine.
- **That the governance exclusion is "half deliberate".** `wiki/defiformal-taxonomy.md:189`
  is an explicit exclusion; UNI-015/ZR15 are explicit inclusions of rule-change effects. No
  document I read reconciles them or draws the line between governance *process* and
  governance *effect*. That framing is my inference and should be put to the owner as a
  question, not recorded as a finding.
- **That the archived candidate04 admin constructors are not carried forward.** I verified
  by `grep -c` (returned 0) and by `diff` against the archived file, which showed the current
  document is a different, expression-only scope (`semantic-contract.md:3-9`). I did not
  search every other file in the repository for an equivalent construct under a different
  name.
- **The corpus50 element counts.** These come from my own aggregation script over
  `corpus50/lanes/*.json`, which counts the `elements` arrays of each entry under
  `categories[].protocols[]` and strips `{...}` discriminators. It counted 72 protocol
  entries, not 50; the lanes evidently include more entries than the corpus name suggests. I
  did not cross-check the counts against any published defiformal figure, and lane
  annotations are themselves a reconciliation of prior reports, not primary sources
  (`UNIFIED-DEFI-ELEMENT-TABLE.md:1267-1283`).
- **Label.** Everything above is a repository observation or an inference over two pinned
  checkouts. Nothing here is an experiment observation, and nothing establishes any formal
  correspondence, enforcement or capability.

---

## Architecture axis (follow-up)

Read in full: `ROADMAP.md` (56 lines), `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines).
Sections 1–6 unrevised. Citations bare `:n` are the design document.

**1. Placement — nowhere.** The core inventory (`:15`) has "checked authority" but no policy
state and no roles. The nine library families (`:93`; `ROADMAP.md:48`) include none
administrative. The Federated Kernel's charter (`:11`) is solvers, evidence, threshold custody,
routing, finality, recovery. The only placements are "application policy" in the Permission row,
Moriarty column (`:27`), and "amendments" in the Intent row, Moriarty column (`:25`).
**Conflict:** `wiki/defiformal-taxonomy.md:189` puts governance outside Core; `:25` and `:68` put
amendment rights inside the Moriarty acceptance relation. The design excludes governance
*process* and claims governance *effect*. Neither document says so.

**2. Core mechanisms.**

| Mechanism | Named? | Component designated? |
|---|---|---|
| Authority kinds | **Yes** — `:68` "Initiate, complete, reconcile, recover, disclose and **amend** rights have separate scopes" | **No** — absent from `:15`, `:42`, `:48` and `:93` |
| Policy state over reachability | **One instance** — `:64` "A timeout changes which authorized transitions may be attempted" | No general form |
| Amendment of an in-flight obligation | **Yes** — `:54`, `:25`, UNI-015 (`spec.md:157-166`), ZR15 | **Partial** — `ROADMAP.md:26` gives U5, scoped to *federation epoch*; `ROADMAP.md:38` says "U3 needs no federation", so unfederated programs get no check |
| Quorum / multisig | **Yes, better placed than I found earlier** — `:30` Conditions row, **Moriarty** column: "conjunction/**threshold** rules" | **Partial** — that is threshold as a release *condition*, not authority over rules; committee authority proper is FDK-only (`:97`), U5 |
| Timelock | **No** | **No** |

**3. Architectural holes** — assertion stated, nothing on the other side.

- **H1, sharpest.** `:68` declares six rights with separate scopes and designates nothing to
  carry a right-kind. My §5 gap 1 survives: the design *states* the distinction the schema lacks.
  Authority is not even an independent judgment here — `:48` lists four, and `judgments.json`'s
  authority judgment maps to `designDocJudgment: "valid state/effect transition"`, though
  `ROADMAP.md:21` bills it as a separate U0 exit.
- **H2.** `:25` assigns "amendments" to Moriarty; no amendment construct, judgment or milestone
  exists outside U5's federation scope. It nests inside H1: amendment consent needs a right-kind
  to attach to.
- **H3.** `:11` asserts the Kernel "cannot silently strengthen its own authority". Nothing checks
  it; `:27` gives the Kernel only "service eligibility". The nearest owner, authority
  non-amplification (`deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:140`), is a
  K obligation — and `:17` calls K "regression evidence, not a compiler theorem".
- **H4.** "application policy" (`:27`) is undefined in both documents.
- **H5.** `:70` "Revocation cannot erase outstanding duties", "scoped revocation" — revocation is
  in no stage field and no milestone exit.
- **H6, dependency inversion.** `ROADMAP.md:27` closes U6 on "DeFi action rows" — including DA22
  "change parameters / pause / migrate" (`action-targets.csv:23`) — via "applicable U3/U4/U5
  interfaces per family", and U3–U5 (`:24-26`) define no administrative interface.

**4. Canonical stage statement** (`:42`). It binds "authority consumption and replay state" — a
*quantity*. Omitted: authority **kind** (despite `:68`); **grantor/origin**; **revocation
state**; **suppression/pause state**; **signer set or threshold**; and a **policy/parameter-set
identity** distinct from program identity, so a parameter change under a fixed program has
nothing an in-flight obligation can pin. `:44`'s distinct-identity rule covers
source/Core/ZKIR/keys, not policy.

**5. Ownership of the architecture work.** U0 owns the authority judgment (`ROADMAP.md:21`) —
the right home for the kind question, currently silent. U3 owns "separate recovery authority"
(`:24`), one of `:68`'s six rights. U5 owns ZR15/UNI-015 (`:26`; `MORIARTY-BACKEND-REQUIREMENTS.md:44`
says U4/U5). U6 owns DA22 by inference (`:27`). The `:68` six-right architecture itself, timelock,
pause and the policy-identity binding: **unassigned**.

**6. Minimum addition.** *(a)* One **right-kind tag** on authority, in the canonical stage
statement (`:42`) and the authority judgment (`ROADMAP.md:21`), ranging over `:68`'s six rights
with an open extension point. It is the only item that cannot be retrofitted, because U0 freezes
the stage relation (`ROADMAP.md:21`) and U2 enforces it (`:23`); pause, timelock, quorum and
upgrade can then arrive as later right-kinds or libraries, and UNI-015's "applicable amendment
consent" becomes checkable. *(b)* For a non-governance program: bind a **policy/parameter-set
commitment** beside "predecessor and obligation commitments" (`:42`), so an authenticated
continuation (`:66`, `:70`) can only extend under the policy it was signed against and a change
forces UNI-015's preservation-or-consent branch. One commitment field, no new mechanism — and it
generalizes `:64`'s single reachability instance into the abstraction the Atlas's criterion B
requires (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:105`).
