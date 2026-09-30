# MIL/3 draft for evidence, authority, persistent history and recovery (independent proposal)

**Status:** This is an independent proposal from one agent, and it is **specified only**. Nothing here is implemented, proved, compiled, adopted or accepted. Every rule below is a candidate. I edited no files and ran no tests, proofs or compilers.

**Startup:** I read `plugins/moriarty-dev/skills/develop/SKILL.md` and ran `cli.py --repo . status --json`. It reports capability `SP01.6 loan-swap-subset` and no pending transactions. The next action is `sp01-loan-report`, and the evidence gaps are `binding-input-stale`, `current-accounting-missing` and `resource-live-state-unavailable`. None of this blocks read-only design research.

**Tags:** I keep the repo convention: **[checked]** means verified against files in this checkout or true by construction here, **[obligation]** means required before freeze, with a proof route, and **[deferred]** means not claimed in this version.

**What I read:** `DESIGN-MIL2.md`, `MIL2-PROPOSED-SEMANTICS.tex`, all eight `opus55-*/SYNTHESIS.md` files, and extracts of the lens reviews most relevant to this lens: bridges 03, oracles 01 and 03, governance 01, staking 03 and derivatives 01. I also read the U0 exit gate and its JSON records, and the captured IBC and W3C sources.

**Reading of "the five reviews":** I took it to mean the five lens reviews behind each category synthesis. I did not read `concepts/intent-language/review/*` in full. If "five reviews" meant something else, §5 should be rechecked against it.

---

## 1. Design thesis and scope

### Thesis

**Evidence can be reused. Verdicts, entitlements and authority are linear. Duties are persistent.**

MIL/2 has one evidence index, `Obs<T, ε, d>` with ε ∈ {anchored, imported, attested} (`DESIGN-MIL2.md:176-177`). That index is asked to do four separate jobs, and it does none of them well enough:

1. **Identity:** which observation this is.
2. **Trust:** who asserted it, and where it was checked.
3. **Status:** whether it is current, final, revoked or disputed.
4. **Consumption:** whether it may be used once or many times.

MIL/3 splits these four jobs. The split then supports four more things:

- one **authority** state machine that is checked against the acceptance head;
- a **duty-preservation law** that no amendment, revocation, pause, timeout or phase failure can bypass;
- **paired cross-domain claims**, where the destination's one-shot receipt decides the race and a local deadline does not;
- a **local release/refund safety relation** whose cross-domain strength is exactly as strong as a named nonreceipt evidence policy, plus a separate **conditional** liveness statement.

### In scope

- Observation identity and binding to an authenticated value.
- Provenance propagation.
- Evidence and trust policy, including attested and imported evidence.
- Authenticated stage time, freshness and round selection.
- Finality and status.
- Replay and unique consumption.
- Grant, revoke, suspend and delegate, with epochs and budgets.
- Policy pinning, amendment and migration.
- Preservation of existing duties.
- Phase-failure transitions.
- Episode link records.
- Paired cross-domain transfer claims.
- The release/refund safety relation.
- Nonreceipt evidence.
- Conditional liveness.
- Verdict evidence: oracle, governance, bridge, derivative fixing and staking slash.

### Out of scope

- Pool, vault and priced arithmetic. The AMM, lending and staking syntheses propose certified operations for these. I only use their outputs as values that carry provenance.
- Φ₁.
- Multi-signer stages.
- n-party clearing.
- Concentrated liquidity.
- Flash loans.
- Voting mathematics.
- Concrete foreign verifier adapters.
- Federation (U5).

### Guardrails

- **All U0 items stay OPEN.** The U0 exit gate (`deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md`) records 0 enforced and 84 NOT_ENFORCED leaf fields, and 6 of 9 trust premises open. **[checked]**
- **Native gates stay open.** The U1 native certificates and the U2 native source-to-ledger path (`ROADMAP.md:22-23`) remain gates. The backend rows ZR01–ZR16 and MNR01–MNR08 stay specified-only. **[checked]**
- **No public process gates.** Nothing here adds a compile, prove or deploy prerequisite for developers. Every rule is a language or acceptance rule.

---

## 2. Normative draft text

### 2.0 Symbols

| Symbol | Meaning |
| --- | --- |
| `d, S, D` | Domains. `S` is the source and `D` the destination of a paired claim. `exec` is the executing domain of a stage. |
| `h` | An authenticated ledger head (state root plus height) on some domain. `h_pred` is the predecessor head a stage binds. |
| `clk_d(h)` | The authenticated clock value of head `h` on domain `d`. |
| `τ` | `stageTime`: the authenticated time at which the stage is evaluated, of type `Instant(clk_exec)`. |
| `H(·)` | Poseidon, used with a domain-separation tag, as in `DESIGN-MIL2.md:263`. |
| `o, i` | An observation record and its identity `i = ObsId(o)`. |
| `L` | A source set: a finite set of observation identities. |
| `λ(i)` | The verifier-assigned label of `i`. |
| `π, π̂` | An evidence policy and its digest. |
| `g` | A grant (authority) cell. |
| `x` | A paired transfer-claim identity. |
| `N` | The consumed (nullifier) set of a domain. It is authenticated ledger state. |
| `Duties(s)` | The set of live persistent duties in state `s`. |
| `Rem(z)` | The protected remedies of a persistent object `z`. |
| `Reject(c)` | A named rejection. Every `c` listed here is a proposed code, not an existing wire code. |

### 2.1 Observation record, identity and binding

**Record.** An observation record has these fields:

```
ObsRec ::= {
  kind      : ObsKind            -- price | balanceRead | receipt | verdict | fixing | outcome | status
  subject   : SubjectId          -- feed / claim / obligation / instrument / policy cell
  value     : T                  -- typed; unit and orientation carried by T
  origin    : Domain             -- the domain whose state or assertor produced the value
  assertor  : Principal | ledger(d)
  keyEpoch  : u32 | none         -- none iff assertor = ledger(d)
  seq       : u64                -- round / sequence, monotone per (subject, assertor)
  observedAt: Instant(clk_origin)
  policy    : π̂                  -- digest of the governing EvidencePolicy
  commit    : Field              -- commitment to the raw authenticated bytes
}
ObsId(o) = H(tag_obs ‖ canonical(o))
```

**Rule OB-1 (identity):** Provenance carries `ObsId`s, not classes. **[checked as a repair]** Oracles review 01, E1, gives the counterexample: `A/B` and `B/A` feeds are both `anchored@midnight`, so swapping them passes a class-only check.

**Rule OB-2 (labels are assigned by a verifier):** A witness supplies bytes and cannot supply a label. The admission judgment is:

```
Π ⊢ admit(o, h_pred, τ) ⇒ λ(ObsId(o)) = (locus, assertor, π̂, fin)   or   Reject(c)
```

Here:

- `locus ∈ {ledger(d), registry(R)@d, foreign(D, V)}`. It says where the check ran.
- `assertor` says who vouched for the value. It is kept separate from `locus`, following oracles review 01, E2. A ledger cell that any account can write is `ledger(d)` as a locus but not as an assertor.
- `fin` is the finality class from §2.5.

Admission has three cases:

| Case | What admission requires |
| --- | --- |
| `ledger(d)` | `d = exec`, and `value`/`commit` equal an **authenticated read** at `h_pred`. |
| `registry(R)` | The signature verifies under a key whose status at `h_pred` is active (§2.3). The on-chain registry cell is itself a `ledger(exec)` read. |
| `foreign(D, V)` | Verifier `V` named by `π` accepts the proof bytes against a `D`-head that `π` accepts. **[deferred to U4]** |

Rejection codes for admission:

- `E_OBS_MISSING`: a named observation has no bytes.
- `E_OBS_BIND`: the value differs from the authenticated read.
- `E_OBS_POLICY`: `o.policy ≠` the signed `π̂`.
- `E_OBS_LABEL`: the witness supplied or overrode a label.
- `E_OBS_KEY_STATUS`: the key is not active at `h_pred`.
- `E_OBS_DUP_ISSUER`: see §2.3.
- `E_IMPORTED_UNSUPPORTED`: `foreign` evidence appeared in a profile below U4.

**Rule OB-3 (admission does not depend on branches):** Every observation named statically by the stage program or the intent is admitted before evaluation, whether or not a short-circuit branch later uses it. This matches the existing static-both-branches rule (`MIL2-PROPOSED-SEMANTICS.tex:83,95`) and oracles synthesis item 3. A malformed observation on a skipped branch still rejects the stage.

**Rule OB-4 (public statement):** The stage's public statement binds the sorted list of `(ObsId, λ)` for every admitted observation. This fills the "observation identities and authenticated values" field listed at `.tex:240`. ZR03 (complete statement binding) and ZR10 (ledger consumption binding) remain **[obligation]**. The enforcement map currently has 0 enforced leaves. **[checked]**

**Cap (proposal, to be measured):** at most 8 admitted observations per stage. This joins the caps table at `DESIGN-MIL2.md:272-283`, and it bounds public-input growth under OB-4.

### 2.2 Provenance propagation

The judgment extends `.tex` (T) with ObsId-keyed sets:

```
Σ;Γ;phase;Π ⊢ e : T ! L,     L ⊆ ObsIds
```

Rules, in addition to the `+` and `And` rules at `.tex:74-82`, which carry over unchanged:

```
 Π ⊢ admit(o) ⇒ λ(i)          i = ObsId(o)
 ────────────────────────────────────────── (P-Obs)
 Σ;Γ;phase;Π ⊢ o.value : T ! {i}

 Γ ⊢ c : 𝔹 ! Lc   Γ ⊢ t₁ : T ! L₁   Γ ⊢ t₂ : T ! L₂
 ──────────────────────────────────────────────────── (P-Ite; branch selection is influence)
 Γ ⊢ if c then t₁ else t₂ : T ! Lc ∪ L₁ ∪ L₂

 Γ ⊢ h : T ! Lh      Lh ⊆ Allowed(hole)
 ──────────────────────────────────────── (P-Hole; otherwise Reject(E_HOLE_SOURCE))
 Γ ⊢ σ(hole) : T ! Lh

 Γ ⊢ t : T ! L     ∀i∈L. Req(λ(i))
 ──────────────────────────────────── (P-Pos; otherwise static type error E_PROV_POSITION)
 Γ ⊢ t : T @ position requiring Req
```

`Req` is a predicate on labels, for example:

- `locus = ledger(exec) ∧ assertor ∈ {ledger(exec)} ∪ PinnedAssertors`, or
- `fin ⊒ final`.

`not`, `min`, `max`, `k_of_n` and comparison take the union of their operands' source sets.

**Obligation O2′ (non-laundering, strengthened) [obligation]:** If `Γ ⊢ t : T ! L` and two admitted observation environments agree on every `i ∈ L`, then `t` evaluates to the same result in both, whether that is a value or the same `Reject` code.

- This is noninterference with respect to `L`. It is strictly stronger than "L contains every influencing observation", because it gives that phrase a definition.
- Proof route: induction over the complete term grammar, including P-Ite, then a TypeScript/K differential on adversarial cases.
- The comparative basis for label joins is Myers and Liskov's decentralized labels (captured at `source-text/cornell-labels.md`, https://www.cs.cornell.edu/andru/papers/sp98/paper.html). The comparative basis for derivation provenance is PROV-O `prov:wasDerivedFrom` (`source-text/w3c-prov-o.md:319`, https://www.w3.org/TR/prov-o/).
- These are comparative sources, not Moriarty evidence.

### 2.3 Evidence policy, attestation and imported policy

```
EvidencePolicy π ::= {
  mode      : ledger | registry(RegistryId) | foreign(DomainId, VerifierId)
  issuers   : IssuerSetRef           -- for registry: authenticated key-status cell
  threshold : (k, n) | none          -- counts DISTINCT issuers
  message   : MessageSchemaId        -- domain-separated preimage schema
  maxAge    : Duration(clk_exec)
  skew      : Duration(clk_exec)     -- tolerated future skew; 0 for ledger mode
  select    : latest_round | round(r) | first_in(Window) | fixing_key(κ)
  finRule   : local | depth(k) | checkpoint(LightClientId) | attestedFinal(π')
  reuse     : reusable | consume_once
  challenge : Window(clk) | none
}
π̂ = H(tag_policy ‖ canonical(π))
```

**Rule EP-1:** An intent or program signs `π̂`, not a name. This repairs the post-signing rebinding counterexample in oracles review 03, E1. **[checked as a repair]**

**Rule EP-2 (distinct issuers):** `attested(o, k, n)` holds only if `k` **distinct** issuer keys in the policy's issuer set, all active at `h_pred`, signed the same `ObsId` preimage. Two signatures by one issuer count as one. Rejection: `E_OBS_DUP_ISSUER`. This repairs oracles review 03, E2.

**Rule EP-3 (key status at the head):**

```
KeyOK(o, h_pred) ⇔ status(issuer(o), keyEpoch(o)) @ h_pred = active
```

- The status is read from an authenticated cell.
- **Revocation is irreversible. Suspension is reversible.** This mirrors W3C Bitstring Status List (captured at `source-text/w3c-status-list.md`, https://www.w3.org/TR/vc-bitstring-status-list/), which is comparative only.
- A revoked epoch invalidates **all** signatures under that epoch, whatever `observedAt` they claim. This closes the backdating counterexample in oracles review 03, E3.

**Rule EP-4 (imported evidence below U4) — owner decision required, see §6 D1:**

- **Option A (recommended):** every profile below U4 **rejects** `foreign` mode with `E_IMPORTED_UNSUPPORTED`.
- **Option B:** keep `imported` as a named premise (MIL/2 §5, `DESIGN-MIL2.md:187`).

Option A follows oracles review 03, E2. Its reason is that a premise with no verifier can only be discharged by a host Boolean. TP03 names issuers and oracles as trust assumptions, not as verified facts (`trust-premises.json`). **[checked]**

### 2.4 Time, freshness and selection

- **Rule TM-1:** `τ` is an authenticated read of `clk_exec(h_pred)`, or of the including block's clock, whichever the pinned ledger interface exposes. `now` in a **guard** is a type error. `now` survives only in validity fields, where it elaborates at signing (`DESIGN-MIL2.md:78`).
  - Whether Midnight exposes an authenticated block time to the circuit or to the ledger check is an **unresolved interface premise**. It belongs with TP09 and ZR01, and I do not assert it.
- **Rule TM-2:** `fresh(o, Δ) ⇔ τ − Δ ≤ o.observedAt ≤ τ + π.skew`. Both sides must be on the same clock. An assertor timestamp in any other clock needs a policy-declared, typed conversion. Otherwise the stage rejects with `E_OBS_CLOCK`.
  - Rejection codes: `E_OBS_STALE`, `E_OBS_FUTURE`.
  - This fixes the counterexample in which signing-time `now` let a 23-hour-old quote pass (oracles review 01, E4).
- **Rule TM-3 (selection is mandatory for risk decisions):** `fresh` alone does not select a value. A guard that decides a transfer, seizure, fixing or health check must use an observation admitted under `π.select`. `latest_round` requires an authenticated `latestSeq(subject)` cell and `o.seq = latestSeq`. Otherwise the stage rejects with `E_OBS_ROUND`.
  - This closes the "pick the best fresh quote" attack (oracles review 03, E4; lending synthesis item 3) and the 3100/3150 fixing counterexample (derivatives review 01, E1).
  - **Counterexample I raise:** in pure push-attestation mode with no on-ledger `latestSeq` cell, "latest" cannot be proved. Such a policy is **inadmissible for `latest_round`** and may only use `round(r)` or `fixing_key`.

### 2.5 Finality and status: two axes

```
Outcome ::= unknown | received(seq, q) | timedOut | closed(qΣ) | disputed(reason)
FinClass ::= soft | final           -- per FinRule of the governing policy
```

- **Rule FS-1:** `final(obs)` is **removed as a Φ atom**. It becomes `λ(i).fin`, and only a P-Pos requirement may test it. In MIL/2, `final(o)` was a free guard (`DESIGN-MIL2.md:145`) and schema finality was a free string (`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:249,264`). **[checked]**
- **Rule FS-2:** Release, reimbursement, refund and any tombstone that depends on foreign evidence require `fin = final`. `soft` evidence may only create a **reservation** that counts under K1 (`.tex:229-236`).
- **Rule FS-3 (dispute):** If a later admitted observation contradicts a `final` observation that has already been relied on, the affected claim moves to `disputed`. An example is misbehaviour evidence that freezes a light client, which is comparative: IBC freezes clients on misbehaviour (captured `ibc-ics004.md`, https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md).
  - `disputed` never auto-resolves to success.
  - The signed recovery policy names who bears the loss and the maximum exposure.
  - **Global rollback is never claimed.** That would contradict `DESIGN-MIL2.md:254`.

### 2.6 Unique consumption and replay

- **Rule UC-1:** Every consumable item has a **semantic key**, not a serialization key. Consumable items are receipts, verdicts, approvals, entitlements, one-shot intents and exercise marks. The nullifier is:

  ```
  ν(item) = H(tag_kind ‖ domainPair ‖ semanticKey(item))
  ```

  Examples of semantic keys:
  - `semanticKey(bridge message) = (S, D, x, outcomeKind)`
  - `semanticKey(slash verdict) = (verdictId)`
  - `semanticKey(gov approval) = (policyCell, proposalId, actionDigest)`
  - `semanticKey(intent) = (intentDigest, phase)`

- **Rule UC-2:** An accepted stage requires `ν ∉ N@h_pred` and writes `N' = N ∪ {ν}` in the same accepted transition. Otherwise the stage rejects with `E_REPLAY`. The same item carried in two proof encodings has one ν. This follows bridges synthesis item 2.
- **Rule UC-3:** Observations with `π.reuse = reusable` are never inserted into `N`. Reusable examples are price reads and balance reads. This resolves the reuse-versus-consumption split in the oracles synthesis table: reads are not consumed, verdicts are.
- **Obligation:** No accepted history consumes the same ν twice on one domain lineage. Proof route: an inductive invariant over `N` plus the ledger-uniqueness premise. Native binding is ZR10. **[obligation]**

### 2.7 Authority: grant, revoke, suspend, delegate and version

```
grant(g) ::= { holder, right ∈ {initiate,complete,reconcile,recover,disclose,amend,issue,enforce},
               scope, budget: Qty | count, used, reserved, window, epoch: u64,
               parent: g' | root, depth ≤ 3, status ∈ {active, suspended, revoked, expired, exhausted},
               successor: SuccessorRule | none }
```

This replaces `allowance(authId)` (`DESIGN-MIL2.md:241`), following governance review 01, Edit 1.

**Rule AU-1 (fresh at the head).** A stage exercising `g` requires:

- `status(g)@h_pred = active`
- `epoch(g)@h_pred = the signed epoch`
- `used + reserved + this ≤ budget`
- `τ ∈ window`

Otherwise it rejects with `E_AUTH_STATUS`, `E_AUTH_EPOCH`, `E_AUTH_BUDGET` or `E_AUTH_WINDOW`. A private "not revoked" bit is forgeable, for the same reason MIL/2 needed a ledger read for `anchored`.

**Rule AU-2 (attenuation).** Delegation `g → g'` requires:

- `scope(g') ⊆ scope(g)`
- `window(g') ⊆ window(g)`
- `right(g') = right(g)`

The budget is **shared**: a spend through `g'` debits both `g'` and every ancestor. This closes the 100 A → 200 A duplication counterexample (governance review 01, Edit 5).

**Rule AU-3 (epoch monotonicity).** Revoke and suspend increment `epoch(g)` and every descendant's epoch. `revoked` is terminal. `suspended → active` needs the grantor's right.

**Rule AU-4 (non-retroactivity).** Changing a grant's status affects only stages whose `h_pred` is at or after the change. It never changes `Duties(s)` (§2.8).

**Rule AU-5 (no stranding).** Suppose revoking or suspending `g` would leave some live persistent object `z` with a remedy in `Rem(z)` (§2.8) that no active grant can exercise. Then the revocation must do one of two things in the same stage:

- (a) bind a successor holder under `successor(g)`, or
- (b) be a pure **actor-key revocation** that leaves the *right slot* in place and marks it `awaiting-successor`, where the object's pre-signed default remedy holder (the owner or debtor) may exercise it.

Otherwise the revocation rejects with `E_REMEDY_BLOCKED`.

**Counterexample and tension (not smoothed over):** the default holder may be the party whose key is compromised. AU-5 then trades security against liveness, and I leave the choice to the owner (§6 D4). Governance review 01's stranding counterexample, where a counterparty's `recover` right is revoked before the deadline, is closed. Security-driven revocation with no successor still leaves the object pending. It is never auto-refunded.

### 2.8 Pinned policy, amendment, and preservation of existing duties

**Rule PV-1 (pin).** Every persistent object carries `policyRef = (policyCellId, version, π̂_terms)` fixed at creation. Persistent objects are obligations, encumbrances, escrows, positions, transfer claims, grants and queued actions. Evaluating a transition on `z` uses `z.policyRef`, never the current `policy(id)`. A mismatch rejects with `E_POLICY_PIN`. This closes the rate-hike-after-borrow counterexample (governance review 01, Edit 3).

**Rule PV-2 (amend).** An amendment is a typed effect:

```
amend(cell, v_old → v_new, Δterms)
```

It requires:

- an active `amend` grant at `h_pred`;
- `version(cell)@h_pred = v_old`;
- `Δterms` inside the grant's scope;
- if the cell has `minDelay`: a queued action whose `readyAt ≤ τ`, **and** the **pre-amendment** `minDelay` governs. A delay cannot shrink itself in the same stage (governance review 01, Edit 4).

**Migration** of an existing `z` to `v_new` requires `z`'s affected party's consent or a signed preservation relation. Otherwise `z` stays on `v_old`. This connects to UNI-015 (`openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:157`) and ZR15.

**Rule PV-3 (queued action).** A queue entry has these states:

```
queued(actionDigest, approvalsν, readyAt, epochs) → executed | cancelled | void
```

Execution **rechecks** AU-1 for every approving grant at the execution head's `h_pred`. Any epoch mismatch makes the action `void`. At one head, only one of `{execute, cancel, void}` can be written, because the queue entry is one-shot under UC-2.

**Duty set.** Define:

```
Duties(s) = { obligations with u_o > 0 } ∪ { escrows in pending } ∪ { xfer claims not closed }
          ∪ { reservations and encumbrances that can still settle } ∪ { queued withdrawals / redemptions }
          ∪ { fixed-but-unsettled derivative liabilities } ∪ { residual recovery obligations }
```

**Rule DP (duty preservation).** For every accepted stage `(s, e, s')`:

```
Duties(s') = (Duties(s) \ Closed(e)) ∪ Created(e)
```

`Closed(e)` may contain `z` only through one of these effects:

- (i) a funded discharge under L1 (`.tex:180-187`);
- (ii) an authorized forgiveness **recorded as such**;
- (iii) a recorded impairment, with the loss bearer named;
- (iv) a terminal escrow transition with complete effects;
- (v) a claim close backed by §2.10 evidence.

**Amend, revoke, suspend, pause, timeout, phase failure and migration are not in this list.** A stage whose effects remove a duty any other way rejects with `E_DUTY_ERASED`. This makes `ROADMAP.md:40`'s "erased debt" hostile control a static category of effect, not a test case. The status as a theorem is **[obligation]**: proof route is an inductive invariant over the effect grammar.

**Protected remedies.** `Rem(z)` is signed at creation. Examples: `repay` for a loan; `withdraw-request` for a vault share; `refund-on-NR` and `recover` for an escrow or claim; `challenge` for a verdict. A pause, whether a `suspend` right or a policy flag (§6 D5), rejects with `E_REMEDY_BLOCKED` if it would make a remedy in `Rem(z)` unexecutable for a live `z`. This closes governance review 01's counterexample, where repayment was paused while accrual continued.

### 2.9 Phase failure

The product contract requires modelling Midnight phase semantics, in which a failed fallible phase can keep guaranteed-phase effects and fees (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`). **[checked]**

**Rule PF-1.** The signed intent carries:

```
PhaseLayout = [(phase_k, effects_k, authConsumed_k, nonceConsumed_k, fee_k)]
```

For every **permitted failure outcome** `f`, it also carries `Retained(f)`, `Remedy(f)` and `DutiesAfter(f)`.

**Rule PF-2.** An accepted failure is a transition with `outcome = f`. Its effects must equal `Retained(f)` exactly. Consumed authority, nonces and fees count against the signed caps. DP applies, so no duty created in a guaranteed phase may disappear because a fallible phase failed. A failure outcome not declared in the signed layout rejects the whole stage with `E_PHASE_UNDECLARED`. The lowerer cannot invent a charged failure policy (`PRODUCT-CONTRACT.md:47`).

**Rule PF-3.** A local evaluator's atomic rejection publishes nothing (`.tex:56`). It is **not** a ledger failure outcome, and PF-2 does not apply to it.

### 2.10 Episode links, paired claims and nonreceipt

**Link record.** This refines `DESIGN-MIL2.md:46`:

```
Link ::= { episodeId, domain, stageIndex, predecessorHead, predecessorStageDigest, intentDigest,
           cumulativeGrossDebit, cumulativeFees, dutyCommitment = H(Duties), consumedCommitment,
           pairRefs: [(x, peerDomain)], terminal: tombstone | none }
```

**Rule EL-1 (one domain per Episode).** An Episode is ledger-linked within **one domain**. Cross-domain workflows are **pairs of Episodes** joined by a claim `x`, not by heads. No single ledger authenticates both. This follows bridges review 03, disagreement 3. The 8-stage cap applies per domain Episode. Rejections:

- `E_LINK_PRED`: predecessor not at the authenticated head.
- `E_LINK_FORK`: two successors of one predecessor. Joins consume each predecessor once.

**Paired claim:**

```
xfer_S(x) ::= { S, D, owner, recipient, assetS, assetD, form ∈ {lockMint, burnMint, release},
                committed: Qty<assetS>, rate: (n, m) literal, deadline_D: Instant(clk_D),
                maxFills ≤ 8, π̂_D, state ∈ {open, closedRel, closedRef, recovering, closedRec, disputed},
                backingCounter }
rcpt_D(x)  ::= absent | received(seq, holder, q)* | timedOut | closed(qΣ)      -- write-once per (x, seq); closed/timedOut terminal
```

**Rule PC-1 (the destination decides).** A delivery stage on `D` for `(x, seq)` requires all of the following. Violations reject with `E_XFER_LATE`, `E_XFER_CLOSED` or `E_XFER_OVERDELIVER`.

- `rcpt_D(x)` is not terminal;
- `seq < maxFills`;
- `τ_D < deadline_D`;
- `(qΣ + q) · m ≤ committed · n`, which is Φ₀ literal cross-multiplication under the limb rule (`DESIGN-MIL2.md:167-171`).

After `deadline_D`, `D` may write only `timedOut` or `closed(qΣ)`. This follows bridges review 03, Edits 1 and 3. Comparative source: IBC receivers refuse packets past their timeout, and the sender times out only on receipt absence (captured `ibc-v2-packet-handler.md:261,324`, `ibc-ics004.md:1341`; https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md).

**Nonreceipt evidence.** `NR(x, qΣ)` holds on `S` if and only if an observation `o` is admitted under `π_D` with `λ.fin = final` and one of:

- **(NR-a)** membership: `rcpt_D(x) ∈ {timedOut, closed(qΣ)}` at `D`-head `h`, with `qΣ = 0` for `timedOut`.
- **(NR-b)** non-membership of every `rcpt_D(x, ·)` at `h`, **with** `clk_D(h) ≥ deadline_D` **and** `π_D` naming a destination program identity whose PC-1 enforcement is itself part of the verified relation. Without that program-identity binding, (NR-b) is inadmissible.

Rejections: `E_NR_INSUFFICIENT` and `E_NR_PREDEADLINE`.

**Rule PC-2 (refund).** The source refund of `x` requires:

- `NR(x, qΣ)`;
- `r ≤ committed − ⌈qΣ · m / n⌉`, with rounding against the refund recipient, so custody never goes negative;
- ν consumed under UC-2;
- a tombstone written.

The refund does not reduce `cumulativeGrossDebit` (`.tex:189`).

**Rule PC-3 (fast fill).** A filler's delivery writes `received(seq, holder = filler, q)` and creates an Obligation `x → filler`. Canonical settlement pays the **receipt holder**. The owner is never paid twice. This follows bridges review 03, Edit 4.

**Rule PC-4 (trusted recovery).** Branch `recovering → closedRec` requires:

- a `registry(R)` k-of-n NR attestation under the signed `π_rec`;
- **creation** of a residual Obligation on the recovery authority, backed by an encumbered bond, covering any later-proved delivery.

It is `recover`-right gated and listed in `Rem(x)`. This is the only path for destinations that cannot enforce PC-1, for example non-programmable chains. This follows bridges review 03, Edit 5.

### 2.11 Local release/refund safety relation

**Single-domain escrow (local safety, LS).** For escrow `E` on domain `d`, over any accepted history `H_d` on one lineage:

```
(LS1) |{ accepted terminal transitions of E in H_d }| ≤ 1                         -- ESC tombstone + UC-2
(LS2) terminal(E) = release  ⇒ release_when held at that stage's h_pred with admitted evidence
(LS3) terminal(E) = refund   ⇒ refund_when held, and if release_when's source set contains any
                               i with λ(i).locus ∉ {ledger(d)}, then refund_when entails NR(·) or recovered(·)
(LS4) DP holds across every stage of H_d
(LS5) Σ outflows of E ≤ funded(E)   and residual goes to the signed party
```

**Rule AR-1 (authoring).** LS3 is checked statically. Consider an escrow whose release guard depends on non-local evidence. If its `refund_when` does not contain `nonreceiptProved(x)` or `recovered(x, π_rec)` in its source set, it is rejected at authoring with `E_ASYNC_REFUND_UNSOUND`. `after(deadline)` alone is rejected in that case. This follows bridges review 03, Edit 2. It makes the rule "a deadline only enables a signed policy branch" (`.tex:200`) and TP05, "Timeout is not evidence of nonexecution", syntactic.

**Paired safety (PS)** is stated relative to a policy premise:

```
Prem(π_D) := VerifierSound(V) ∧ FinalityHolds(finRule, h) ∧ WriteOnce(rcpt_D) ∧ (NR-b used ⇒ PC-1 enforced by bound program id)

Prem(π_D) ⇒  ∀x. deliveredEq(x) + refunded(x) + recoveredPaid(x) − residualRecoveryObligation(x) ≤ committed(x)
where deliveredEq(x) = ⌈qΣ_final · m / n⌉
```

- Status: **[obligation]**. Proof route: a product-of-two-state-machines invariant over (`xfer_S`, `rcpt_D`), proved per domain by induction on accepted stages, with a single cross-domain premise, `Prem`.
- **What happens when `Prem` fails.** Suppose the verifier is compromised or a reorg exceeds `finRule`. Then PS does not hold, and the design claims only this:
  - (i) the local stage history is still LS-safe;
  - (ii) the discrepancy is representable as `disputed` with a named loss bearer;
  - (iii) no stage reports it as success.

  This follows bridges synthesis, "Reorg response", and `PRODUCT-CONTRACT.md:47`: "Safety proofs do not supply … bridge security".

**Counterexample showing why NR-b needs a program binding:** `D`'s program accepts late deliveries. Non-membership at `h` with `clk_D(h) ≥ deadline_D` is observed, and `S` refunds 100. A late relay then delivers on `D`, and 100 is paid twice. PS fails without PC-1 enforced on `D`, so (NR-b) is inadmissible unless `π_D` binds a `D` program identity that enforces it.

### 2.12 Conditional liveness

**Assumptions** `A(Δ)`, all named in the intent's `recoveryPolicy` (U0 `signedIntent.recoveryPolicy`):

- `Incl_S(Δ_S)`, `Incl_D(Δ_D)`: a well-formed stage submitted by an authorized actor is included within Δ.
- `Relay(Δ_R)`: a proof of any final `D` outcome reaches `S` within Δ_R.
- `Fin(Δ_F)`: `D` heads become `final` under `finRule` within Δ_F.
- `Actor`: some holder of each needed right, or its successor, submits.
- `Wit`: the witness data is available (TP08).
- `VerifierAvail`: the verifier `V` or `R`-quorum responds.

**Obligation LV [obligation]:**

```
A(Δ) ∧ state(x) = open at τ₀  ⇒  a terminal S-transition for x is includable by
τ₀ + max(0, deadline_D − τ₀) + Δ_D + Δ_F + Δ_R + Δ_S
```

The same holds for a single-domain escrow, with `Δ_R = Δ_F = 0`.

- **If any assumption fails, the state stays pending.** Liveness is never converted into a refund entitlement.
- **AR-2:** A workflow whose signed text *claims* recovery but omits any of the assumptions that apply is rejected (`E_LIVENESS_UNSTATED`). This fulfils `DESIGN-MIL2.md:219`.
- **Interaction with AU-5:** LV depends on `Actor`, and AU-5 is what keeps `Actor` satisfiable after a revocation.

### 2.13 Verdict evidence: one typed carrier and five kinds

```
Verdict<K> ::= ObsRec with kind = verdict, subject, verdictId, policyRef, effectiveAt, challenge window
```

| K | Selection / key | Reuse | Write target | Challenge |
| --- | --- | --- | --- | --- |
| `oracleRead` (a price, not a verdict) | `π.select` (TM-3) | reusable | none | none |
| `derivFixing` | `fixing_key κ_I = (feedId, t_fix, π̂)` signed at issuance | write-once cell `fixing(I)`, then reusable reads | `fixing(I) : unfixed → fixed(v, ObsId, t)` | optional window before `fixed` becomes final |
| `govOutcome` | `(policyCell, proposalId, actionDigest)` | consume_once (UC-2) | queued action (PV-3) | cancel window |
| `bridgeMsg` | `(S, D, x, outcomeKind)` | consume_once | `xfer_S(x)` state | dispute (FS-3) |
| `slash` | `verdictId`, bound to `(service, obligation, offenseTime)` | consume_once | `seized_o += q ≤ cap_o`, and `Σ seized ≤ amount` | challenge window before seizure is final |

**Rule VD-1.** A verdict's `policyRef` must equal the **consumed object's pinned policy**. For a slash, that is the `Obligation.consent` fixed at origination. Mismatches reject with `E_VERDICT_POLICY`. This closes staking review 03, E4: "verdict for service A presented under B".

**Rule VD-2.** Verdicts affect duties only through DP's allowed closures. A slash is **seizure plus impairment** with the loss bearer named. It is not forgiveness.

**Rule VD-3.** `offenseTime` must be `≤ slashable_until` of the encumbrance at `h_pred`. Deallocation does not end slashability before `slashable_until`. This closes staking review 03, E1.

Voting, tally and snapshot mathematics are **library** code (§2.14).

### 2.14 Core versus library

| Core (safety; cannot be a library) | Library / profile (policy) |
| --- | --- |
| ObsRec/ObsId, verifier-assigned labels, OB-1..4, ObsId-keyed provenance with O2′ | Concrete verifier adapters (light client, quorum, optimistic) — **[deferred U4]** |
| EvidencePolicy digest, distinct-issuer counting, key status at head, TM-1..3 | Specific `maxAge`, skew and TWAP/median aggregation profiles — **[deferred]** |
| Outcome × FinClass split, FS-1..3 | Specific `finRule` depths per chain |
| UC-1..3 nullifiers by semantic key | Nullifier storage layout |
| Grant state machine, AU-1..5, PV-1..3, DP, `Rem(z)` | Voting, quorum, snapshot and delegation-by-token logic; timelock lengths |
| PF-1..3 phase failure | Fee schedules |
| EL-1, paired claim, PC-1..4, NR, LS/PS, AR-1/2 | Transfer forms, fill pricing, bond sizing, challenge window lengths |
| Verdict carrier, VD-1..3 | Slash fractions, service allocation, reward curves |

---

## 3. Effect on the eight DeFi categories

1. **AMM.** Reserves enter as `ledger(exec)` reads keyed by ObsId (OB-2), so the AMM synthesis's "authenticated reserves at current pool head" becomes a label requirement. Pool-version pinning is PV-1. A standing maker offer is a persistent object with a one-shot fill/cancel under UC-2. No imported evidence in the first slice. Pool arithmetic is outside this lens.
2. **Lending.** Price for health checks needs TM-3 selection plus P-Pos with `assertor ∈ PinnedAssertors`. Freshness alone is rejected, which closes the "best fresh quote" attack. Loan terms are pinned (PV-1), so a rate change does not reach existing debt. `repay ∈ Rem(loan)` cannot be paused (AU-5, `E_REMEDY_BLOCKED`). Liquidation ordering is ledger-head serialization on the encumbrance version, which conflicts with MIL/2 (§5 C7). Debt survives every non-discharge event (DP).
3. **Stablecoins.** A reserve attestation is `registry(R)` evidence with a named assertor. It is *never* `ledger` (OB-2), so a reserve claim cannot be laundered into a ledger fact. Redemption requests are duties. System-mode changes (active, guarded, shutdown) are `amend` effects under PV-2 and cannot erase redemption duties (DP). Mint authority is an `issue` grant with epoch and shared budget (AU-1/2).
4. **Derivatives.** Fixing is a `derivFixing` verdict with a signed fixing key and a write-once cell (§2.13). Exercise is a produced, consumable mark under UC-2. The producer-less `exercised(opt)` atom is removed from Φ₀ until the mark exists. Settlement after fixing is a fixed Obligation covered by DP, so refund branches cannot erase it. Reorg of a fixing source leads to FS-3 `disputed`, not re-fixing.
5. **Oracles.** Most of §2.1–2.5 is oracle core. First slice: one `ledger(exec)` price with `latest_round` selection and a signed threshold. Registry attestation (U3) and foreign attestation (U4, rejected before then under D1-A) follow.
6. **Governance.** Grant cell, epochs, attenuating delegation, pinned policies, a timelock that cannot shrink itself, void-on-revocation queued actions, `govOutcome` consumed once, and protected remedies during pause. Vote counting is a library. One-signer U0 stages may consume a pre-existing outcome only if the outcome's ObsId and ν are native-bound (ZR03/ZR10, open).
7. **Bridges.** Paired claim, destination-decided race (PC-1), NR evidence (NR-a/NR-b), refund (PC-2), fast-fill debt (PC-3), bonded recovery (PC-4), and PS conditional on `Prem(π_D)`. Midnight-to-Midnight lineage pairs are admissible at U3 as `ledger` evidence. They prove nothing about foreign trust and must be labelled so.
8. **Staking and yield.** Slash is a `slash` verdict under VD-1..3 with a per-obligation cap and an aggregate seized total. Pending withdrawals are duties that stay slashable. Reward-index checkpoints are persistent objects under PV-1. Pooled write-down creates one duty-closure line, not per-holder lines.

---

## 4. Transaction traces

**Setup.** The executing lineage is `S = midnight.preview`. There are two policy variants for `D`:

- `P_U3`: `D` is a second **Midnight program lineage**. Evidence is `ledger(S)`, `finRule = local`.
- `P_U4`: `D` is a foreign domain with `foreign(D, lightclient L)`, `finRule = checkpoint(L)`.

The owner signs:

```
xfer x { S; D; committed 100 A; rate (1,1); deadline_D T_D; maxFills 2; π̂_D; recoveryPolicy {Incl_S 1h, Incl_D 1h, Relay 2h, Fin 30m, Actor, Wit} }
```

Gross cap 100 A, fee cap 1 A.

### 4.1 Valid trace (profile `P_U3`; under D1-A the same trace with `P_U4` rejects at T4, see H0)

| Stage | Domain | Key checks | Result |
| --- | --- | --- | --- |
| T1 commit | S | Intent ν fresh (UC-2); `initiate` grant active at `h_pred` (AU-1); lock 100 A into custody, K1 holds; `xfer_S(x)=open`; link record with `dutyCommitment` including `x` | **Accept** |
| T2 deliver | D | `rcpt(x)` non-terminal; `seq 0 < 2`; `τ_D < T_D`; `60·1 ≤ 100·1`; write `received(0, owner, 60)`; conservation holds on D | **Accept** |
| T3 close | D | `τ_D ≥ T_D`; only `closed` or `timedOut` writable; write `closed(60)` | **Accept** |
| T4 refund | S | Admit `o = rcpt_D(x)=closed(60)` as `ledger(S)` read (OB-2) at head `h`; `fin = final` (local); `NR(x,60)` by NR-a; `r = 100 − ⌈60⌉ = 40`; ν = `H(tag‖S,D‖x‖closed)` ∉ N; tombstone; `cumulativeGrossDebit` stays 100; DP closes `x` via rule (v) | **Accept** |

Outcome: owner net outflow 60, recipient +60 on the `D` lineage, E1 per lineage, and `Duties` reduced only by the justified closure.

### 4.2 Well-formed hostile traces

Each hostile trace keeps every envelope field valid and feasible, and mutates **one** bound fact.

| ID | Mutation from the valid trace | Expected stage result |
| --- | --- | --- |
| H0 | Profile `P_U4` under D1-A at U0–U3 | T4 **Reject(`E_IMPORTED_UNSUPPORTED`)**; `x` stays `open`. It is a pending duty, not a failure. |
| H1 | T4′ refunds 100, citing non-membership of `rcpt_D(x)` at head `h₀` taken before T2, with `clk_D(h₀) < T_D` | **Reject(`E_NR_PREDEADLINE`)**. A deadline is not nonreceipt (TP05). |
| H2 | After T4, resubmit T4 with the same `closed(60)` fact under a different proof encoding and new stage nonce | **Reject(`E_REPLAY`)**. ν is keyed by `(S,D,x,closed)`, not by bytes (UC-1). |
| H3 | T2′ delivers `seq 1` of 50 after T2 | **Reject(`E_XFER_OVERDELIVER`)**, because `110 > 100`. |
| H4 | Author changes `refund_when` to `after(T_D)` with no NR atom | **Reject at authoring (`E_ASYNC_REFUND_UNSOUND`)**. |
| H5 (oracle) | Lending stage cites a registry price signed under issuer epoch `e`, revoked at `h_pred`, with `observedAt` backdated inside `maxAge` | **Reject(`E_OBS_KEY_STATUS`)**. Revocation invalidates the whole epoch (EP-3). |
| H6 (oracle) | Two fresh rounds r, r+1; the solver presents r, while `latestSeq = r+1` | **Reject(`E_OBS_ROUND`)** (TM-3). |
| H7 (governance) | A queued amendment approved by grant `g`, where `g` was revoked after queueing; executed after `readyAt` | **Queue → `void`**. The execute stage **rejects with `E_AUTH_EPOCH`**; a separate void stage is accepted. |
| H8 (governance) | Grantor revokes the counterparty's `recover` on a pending escrow with no successor rule | **Reject(`E_REMEDY_BLOCKED`)** (AU-5). |
| H9 (staking) | Slash verdict v consumed once, then re-presented against the same obligation | **Reject(`E_REPLAY`)**. Presented under service B's obligation instead: **Reject(`E_VERDICT_POLICY`)**. |
| H10 (derivatives) | Settlement fixes with a 17:02 print while `κ_I` names `t_fix = 17:00` | **Reject(`E_OBS_ROUND`)**. The `fixing(I)` cell stays `unfixed`. |
| H11 (phase) | Fallible phase fails; the solver's claimed retained effects drop the guaranteed-phase obligation creation | **Reject(`E_DUTY_ERASED`)**. If the outcome was not declared: `E_PHASE_UNDECLARED`. |

All codes are **proposed**. These traces are expected results for a future evaluator. **No trace was executed.**

---

## 5. Conflicts with MIL/2 and the reviews

| # | Conflict | Evidence | This draft's position |
| --- | --- | --- | --- |
| C1 | MIL/2 source sets are `{ε@d}` classes (`DESIGN-MIL2.md:178,184`) | `.tex:67,72` already moves to identities; oracles review 01 E1 | ObsId-keyed (OB-1). This **changes a U0 item** (evidence type index, `DESIGN-MIL2.md:345`). |
| C2 | `imported` is a named premise until U4 (`DESIGN-MIL2.md:187`) | Oracles review 03 E2 recommends rejection | D1: recommend reject; not decided here. |
| C3 | `final(obs)`, `attested(obs,k,n)` and `fresh` are Φ atoms (`:145-146`) | Oracles review 03 E5; schema finality is a free string (`stage-relation.schema.json:249,264`) | `final` becomes a label test. `attested` counts distinct issuers. `fresh` needs authenticated τ and selection. |
| C4 | `now` elaborates at signing (`:78`) | Oracles review 01 E4 | Forbidden in guards (TM-1). |
| C5 | ESC decides races "at the same authenticated head" with `priority` (`.tex:192-197`; `DESIGN-MIL2.md:206`) | Bridges review 03: no shared head across domains | `priority` is local only. Cross-domain races are decided by `D`'s receipt (PC-1). MIL/2's showcase `refund_when after(validity.end)` (`:310`) is fine locally and would be **rejected by AR-1** if its release guard read foreign evidence. |
| C6 | Status list `unknown/received/nonreceivedProved/final` (`.tex:202`) mixes outcome and finality | Bridges review 03 Edit 5 | Two axes (FS). |
| C7 | "Signed total order on seizes of one encumbrance" (`DESIGN-MIL2.md:193`) | Staking review 03 E3: with `|S|=1` the enforcer orders itself first. Lending synthesis prefers ledger-head serialization. | Origination priority plus reservation cells, with head-version serialization among equals. D6 remains open. |
| C8 | `allowance(authId)` cell and bare "revocation" (`:191,241`) | Governance review 01 Edits 1–2 | Grant cell with status machine (AU). |
| C9 | Episode may span domains implicitly; 8-stage cap per Episode (`:46,281`) | Bridges review 03 disagreement 3 | One domain per Episode (EL-1). |
| C10 | `exercised(opt)` has no producer (`:149`) | Derivatives review 01 E4 | Remove from Φ₀ until a produced mark exists. |
| C11 | Bridges review 03 wants late receipt *impossible*; the category review (`07-bridges.md:19`) asks for a race rule | Both are cited in the review | Adopt "impossible" where `D` is programmable (PC-1). Where it is not, a race is unavoidable and is handled only by bonded recovery (PC-4). **Both positions are needed. Neither is universal.** |
| C12 | Governance: `suspend` as a ninth right (review 03) vs pause as a policy parameter (review 04) | Governance synthesis table | D5. Either way, `Rem(z)` must survive. |
| C13 | **Internal tension:** AU-5 non-stranding vs emergency revocation of a compromised key | This draft | D4. Not resolved. |
| C14 | MIL/2 treats `recover` as an ordinary revocable right (`:322`) | Governance review 01 counterexample 1 | `recover ∈ Rem(z)` makes it slot-protected. |
| C15 | Oracle TWAP/median needs bounded collections (`DESIGN-MIL2.md:360`) | Oracles synthesis | Unchanged. The 8-observation cap limits naive medians. **[deferred]** |
| C16 | Staking review 03 E5's aggregate write-down uses `assetsFor` (Φ₁) | `DESIGN-MIL2.md:160-161` | Verdict handling here is Φ₀. Conversion arithmetic is not settled in this lens. |

---

## 6. Unresolved decisions and verification obligations

### Owner decisions

For each decision, I give a recommendation, the alternatives, and the evidence needed to decide. Nothing here is decided.

- **D1: `imported` before U4.**
  - A: reject with `E_IMPORTED_UNSUPPORTED` (recommended).
  - B: keep it as a named premise, as MIL/2 does.
  - Evidence needed: whether any U2/U3 discriminator (`ROADMAP.md:38`) needs foreign evidence. If none does, A costs nothing.
- **D2: authenticated stage time.**
  - A: block/head clock read.
  - B: validity interval only, with no in-guard time.
  - Evidence needed: the pinned Midnight ledger/ZKIR interface (TP09, ZR01). If no authenticated clock exists, `fresh` and every deadline guard are unenforceable natively and must be ledger-validity checks.
- **D3: NR-b admissibility.**
  - A: admit it only with a bound destination program identity (recommended).
  - B: NR-a only.
  - Evidence needed: whether the target destinations write sentinel receipts, as IBC's `TIMEOUT_RECEIPT` does (`ibc-ics004.md:335,424`, comparative).
- **D4: revocation versus stranding.**
  - A: successor rule is mandatory at object creation.
  - B: default the slot to the owner or debtor.
  - C: allow stranding under an emergency right, with the loss bearer named.
  - Evidence needed: a threat model for key compromise of the default holder.
- **D5: pause form.**
  - A: a `suspend` right.
  - B: a policy parameter.
  - Evidence needed: whether a non-delegable emergency scope needs its own right kind. A new right kind is a U0 enum change, so it is expensive to reverse after freeze.
- **D6: seize ordering.** Origination priority plus reservations, versus owner-signed order, versus ledger first-come.
- **D7: observation cap.** The cap value (8) and whether ObsIds sit in public inputs or behind a commitment. Evidence needed: ZR12/ZR13 cost measurement.
- **D8: disputed loss bearer default.** The recovery authority's bond, versus the owner, versus the filler.

### Obligations

None of these is claimed.

| ID | Statement | Route | Milestone |
| --- | --- | --- | --- |
| O2′ | Non-laundering as noninterference w.r.t. L (§2.2) | Induction plus TS/K differential | U0 rule, U1/U2 evidence |
| EV1 | Label assignment only by admission (OB-2); a witness cannot choose λ | Native statement binding ObsId+λ (ZR03) | U2 |
| EV2 | UC: ν unique per lineage | Invariant plus ledger premise (ZR10) | U2 |
| AU1 | Budget never exceeded across delegation trees | Invariant over AU-2 | U0/U3 |
| DP1 | Duty preservation over the full effect grammar | Induction on effects | U0 statement, U3 evidence |
| PF1 | Declared failure outcomes exhaust ledger phase outcomes | Against the pinned ledger phase semantics | U2 |
| LS | LS1–LS5 | ESC plus UC plus DP | U3 |
| PS | Paired safety under `Prem(π_D)` | Product-state invariant | U3 (Midnight pair), U4 (foreign) |
| LV | Conditional liveness under A(Δ) | Bounded-response argument; hostile traces for each dropped assumption | U3 |
| VD | Verdict single use and cap sums | Invariant | U3 |

The MIL/2 §17 obligations and `.tex` O1–O6 remain in force. So do the U0 exit-gate items, all OPEN, and ZR01–ZR16 and MNR01–MNR08, all specified-only.

### Deferred

- Concrete foreign verifiers.
- Aggregation (TWAP, median).
- Voting mathematics.
- Recursive native history (U4, ZR09).
- Federation and epoch migration (U5, ZR15).
- Multi-signer stages.
- Priced arithmetic.
- Rebasing.
- Global rollback: never claimed.

---

## 7. Anchors and primary sources

### Repository anchors (checkout `983a4bb`, verified in this session unless noted)

**MIL/2 design and semantics**
- `concepts/intent-language/DESIGN-MIL2.md:46,78,145-149,176-187,191-193,206,215-219,241,254,263-264,272-283,309-311,322,345-350,360`
- `deliverables/mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex:56,67,72-83,95,180-189,192-205,229-240`

**Syntheses and lens reviews**
- `deliverables/mil2-deep-research-2026-09-29/opus55-{amm,bridges,derivatives,governance,lending,oracles,stablecoins,staking_yield}-recommendations/SYNTHESIS.md`
- Lens reviews: `opus55-bridges-recommendations/review-03.md`, `opus55-oracles-recommendations/review-01.md`, `review-03.md`, `opus55-governance-recommendations/review-01.md`, `opus55-staking_yield-recommendations/review-03.md`, `opus55-derivatives-recommendations/review-01.md`

**U0 records**
- `deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md` (all items OPEN)
- `trust-premises.json` (TP03, TP05, TP08, TP09)
- `backend-requirement-matrix.json` (ZR01–ZR16 and MNR01–MNR08, specified-only)
- `judgments.json` (six keys; `signedIntent.recoveryPolicy`, `replayPolicy`)

**Roadmap, contract and specs**
- `ROADMAP.md:21-26,38-40`
- `docs/MORIARTY-PRODUCT-CONTRACT.md:43,47`
- `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:249,264`
- `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:157` (UNI-015)

**Cited through the reviews, not re-verified by me:** `category-review/07-bridges.md:19-20,33` and `category-review/06-governance.md:22,46`.

### Primary sources (comparative, not Moriarty evidence; captures listed in `sources.json`)

- **IBC ICS-004 v1:** receipt absence for unordered timeout, sentinel receipts, client freeze. https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md (captured `source-text/ibc-ics004.md:335,424,1341`)
- **IBC v2 packet handler:** timeout checked against the receiving-chain clock; no receipt after timeout. https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md (captured `ibc-v2-packet-handler.md:204,261,324`)
- **W3C Bitstring Status List v1.0:** status-purpose entries, revocation versus suspension. https://www.w3.org/TR/vc-bitstring-status-list/ (captured `w3c-status-list.md`)
- **W3C PROV-O:** `wasDerivedFrom`. https://www.w3.org/TR/prov-o/ (captured `w3c-prov-o.md:319`)
- **Myers and Liskov, decentralized labels.** https://www.cs.cornell.edu/andru/papers/sp98/paper.html (captured `cornell-labels.md`)
- **SMT-LIB QF_IDL/QF_LIA** (for AR-1 checking). https://smt-lib.org/logics-all.shtml
- **ZKIR v3 specification** (statement-soundness premises). https://github.com/midnightntwrk/midnight-zkir/blob/47793c8ab042aa5a91d1a4672c6b82de6bdf9dd8/zkir-spec/src/zkir-v3/README.md

I did not consult any other MIL/3 agent output.