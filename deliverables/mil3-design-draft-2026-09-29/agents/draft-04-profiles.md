I've drafted a MIL/3 proposal for the eight DeFi family profiles, grounded in the MIL/2 design, the proposed semantics, the eight category reviews, the eight Opus syntheses and the U0 records. It is written from those sources only; no other MIL/3 draft was read. Nothing was edited, compiled, tested or proved. The `status` check shows `SP01.6 loan-swap-subset` blocked, with next action `sp01-loan-report` and no pending transactions. The development focus is still I2/MC02, so everything below is specification research and does not replace that work.

# MIL/3 draft, family-profile and roadmap lens: an independent agent proposal

**Date:** 2026-09-29 · **Baseline:** `983a4bb4` · **Status:** specified-only. Every normative sentence below is a candidate rule, not an adopted one. A tag of **[checked]** means I checked it in this session against a file or by arithmetic. **[obligation]** means it must be proved or measured before freeze. **[deferred]** means this version does not claim it.

---

## 1. Design thesis and scope

### 1.1 Thesis

MIL/2 treats every family-specific computation as either Φ₀ (the trader's signed promise) or Φ₁ (deferred general nonlinear arithmetic, placed at U4). All eight category reviews find the same gap in that split. The missing piece is the **transition rule of the program the intent touches**: the pool, the loan, the vault, the option. None of it needs a general nonlinear predicate language. The AMM synthesis states it most clearly: without such a rule, a prover can give a trader an acceptable output by draining LP reserves (`opus55-amm-recommendations/SYNTHESIS.md:17`).

MIL/3 therefore has three strata:

1. **Core (mandatory, profile-independent):** thirteen invariants, K1–K13 (§2.2). Every accepted stage in every profile must satisfy them. No profile may weaken them.
2. **Certified operation basis Ω (versioned and small):** three arithmetic operations with fixed width premises and a fail-closed result, usable only inside a profile's transition rule. They are not Φ terms, so the signed intent stays in Φ₀.
3. **Family profiles (versioned library policy):** `moriarty-profile/<family>/<n>`. Each fixes a constructor set, its state cells, its transition schemas and its parameters (fee tier, close factor, delay, verifier mode and so on). A profile is a language-version declaration that any developer may target. It is **not** a maintainer approval, registry or process gate. The permissionless boundary of the product contract governs.

The stage relation gains a conjunct:

$$
\mathsf{Stage}_3(I,\sigma,s,o,e,s') \;\triangleq\; \mathsf{Stage}_2(I,\sigma,s,o,e,s') \wedge \mathsf{ProgramValid}_{P,v}(s,o,\vec a,e,s') \wedge \mathsf{HeadFresh}(R\cup W,s) \wedge \mathsf{PolicyPinned}(s,e).
$$

Symbols:

- $\mathsf{Stage}_2$: the twelve-clause conjunction in the proposed semantics, `MIL2-PROPOSED-SEMANTICS.tex`, section "Stage acceptance relation".
- $I$: the signed intent. $\sigma$: its completion.
- $s, s'$: the pre- and post-state. $o$: the admitted observations. $e$: the complete effect list.
- $P$: a program identity. $v$: a profile version. $\vec a$: the program arguments.
- $R, W$: the derived read and write footprints.

The three new conjuncts are defined in §2.3. Any unsatisfied conjunct yields $\mathsf{Reject}(code)$. A $\mathsf{Reject}$ is not a truth value, and the stage fails closed (MIL/2 §4.2).

### 1.2 Scope of this lens

**In scope:**

- A family-to-constructor map.
- Minimal operational schemas for eight families:
  - AMM exact-input swap.
  - Loan originate, repay and liquidate.
  - CDP mint and burn.
  - Capped cash-settled option write, fixing and exercise.
  - Oracle admission.
  - Governance queue, execute and cancel.
  - Bridge source lock, destination receive and destination timeout.
  - Vault deposit, redeem, request, finalize, slash and reward.
- The core-versus-profile split.
- The smallest accepted vertical slice per family, with U0–U6 sequencing, required native evidence and positive and hostile controls.

**Out of scope, preserved as category limits [deferred]:**

- Concentrated liquidity (MIL/2 decision 5).
- Weighted pools.
- Perpetuals, funding, portfolio margin, ADL and socialized loss.
- Algorithmic, reflexive or undeclared-backing stablecoins.
- Rebasing tokens.
- Shared-write multi-signer clearing (MIL/2 decision 4: the U0 profile admits $|S|=1$).
- Flash loans and intra-stage traces (decision 2).
- TWAP and median aggregation.
- Vote snapshots.
- Bonded bridge fast-fill and dispute economics.
- Restaking allocation across services.
- Strategy mandates.
- Native parent-proof recursion (U4).

**Preserved without change:**

- All six MIL/2 pre-freeze obligations O1–O6 (`DESIGN-MIL2.md:363-365`; `.tex` table O1–O6).
- All U0 exit-gate items. All eight are OPEN (`deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md:5-14`), and the enforcement map records **84/84 leaves NOT_ENFORCED**.
- The U1 native-certificate gate and the U2 native-path gate (`ROADMAP.md:22-23`).

---

## 2. Normative draft text

### 2.1 Notation used throughout

| Symbol | Meaning |
|---|---|
| $d$ | executing domain of the stage (one per stage) |
| $\mathsf{hv}_s(c)$ | authenticated head version of cell $c$ in state $s$ |
| $\mathsf{hv}_\pi(c)$ | head version the proof witness claims to have read |
| $t$ | authenticated stage time on clock $\mathsf{clk}(d)$. Its source is an open premise; see D9 |
| $\mathsf{bal}_s(d,p,a)$ | balance cell of party/custody $p$ for asset $a$ |
| $\mathsf{cust}(x)$ | the custody account of program object $x$ (a party in E1) |
| $\lfloor n/m\rfloor$, $n \bmod m$ | integer quotient and remainder, $m>0$ |
| $\mathsf{Rej}.F.\mathit{CODE}$ | rejection code for family $F$ (`CORE`, `AMM`, `LOAN`, `CDP`, `OPT`, `ORC`, `GOV`, `BRG`, `VLT`) |
| $\mathbb{U}_w$ | unsigned integers $<2^w$ |
| $\mathsf{Tomb}(x)$ | one-shot terminal/replay mark; writing an existing mark rejects `Rej.CORE.REPLAY` |

### 2.2 Core invariants K1–K13 (mandatory for every profile) [obligation, each]

- **K1. Nominal identity.** Assets, pools, instruments, claims, transfers and policies are opaque ledger identifiers. Metadata equality never establishes identity (MIL/2 §3.1).
- **K2. Complete effects and conservation.** $e$ lists every balance, supply, share, lock, liability, receipt, replay and policy write. For each $(d,a)$: $\sum_p \Delta\mathsf{bal}(d,p,a)=\Delta\mathsf{supply}(d,a)$. The sum ranges over custody and reserve accounts (E1). A write in $s'\setminus s$ with no effect line rejects `Rej.CORE.HIDDEN_WRITE`.
- **K3. Supply is an effect.** $\Delta\mathsf{supply}(d,a)$ is the sum of typed `mint`/`burn` lines, never a claimed field. Each `mint` line carries a scoped `issue` grant for $(d,a)$ with a remaining quantity budget, and a **backing tag** $\beta\in\{\mathsf{cdp}(v),\mathsf{bridge}(\tau),\mathsf{reward}(r),\mathsf{reserve}(q),\mathsf{unbacked}(\pi)\}$. `burn` does not replenish the quota unless the pinned policy says so. No profile in this draft admits $\mathsf{unbacked}$. The tag exists so that such issuance stays visible rather than silent.
- **K4. Liability persistence.** For every obligation: $u' = u + \mathsf{create}+\mathsf{accrue}-\mathsf{discharge}-\mathsf{forgive}-\mathsf{impair}$, with $u'=p'+a'$ (L1). `discharge` needs a matching funded transfer in the same $e$. `forgive` needs the creditor's grant. `impair` records a recognized loss and moves the obligation to the `impaired` state. It does not reduce the loss-bearer's claim unless a waterfall rule is named. No terminal, expiry, pause, amendment or timeout transition deletes a non-zero $u$.
- **K5. Aggregate lock safety.** A cell $\mathsf{lockTotal}(d,p,a)$ is authenticated and updated in the same stage as every encumbrance reserve, commit, release, seize or cancel. Every debit of $\mathsf{bal}(d,p,a)$ reads it and requires $\mathsf{lockTotal}'\le\mathsf{bal}'$, otherwise `Rej.CORE.LOCK_EXCEEDS`. This operationalizes (K1) of the `.tex` draft.
- **K6. Derived, alias-closed footprints.** $R_{decl}\supseteq R_{der}$ and $W_{decl}\supseteq W_{der}$ after hole resolution. An unresolved alias rejects `Rej.CORE.FOOTPRINT`.
- **K7. One-shot consumption.** Receipts, claims, exercise marks, approval sets and intent digests are consumed through $\mathsf{Tomb}$.
- **K8. Verifier-assigned evidence.** Observation class ($\mathsf{anchored}$, $\mathsf{imported}(\Pi)$, $\mathsf{attested}$) is an output of the verification locus. It is never a witness field. Source sets carry observation **identities** (the `.tex` typing rule).
- **K9. Policy pinning.** Every persistent object (obligation, encumbrance, position, claim, vault class, queue item) stores the policy digest $\pi$ under which it was created. Transitions on it evaluate under $\pi$ unless a preservation relation or recorded consent authorizes migration.
- **K10. ProgramValid** (§2.3).
- **K11. HeadFresh** (§2.3).
- **K12. Role-directed rounding with a posted remainder.** Every non-exact Ω use names a direction derived from the operation role (never an author-selectable parameter) and a remainder beneficiary cell that actually receives it. A remainder with no posting cell rejects at authoring time: `Rej.CORE.REMAINDER_UNPOSTED`.
- **K13. Branch-independent admission.** All named observations and grants are admitted before evaluation. A short-circuited branch does not skip admission.

### 2.3 New stage conjuncts

$$
\mathsf{ProgramValid}_{P,v}(s,o,\vec a,e,s') \iff \tau_{P,v}(s,o,\vec a)=\mathsf{Ok}(e_P,\,W_P)\;\wedge\; e = e_P \uplus e_I \;\wedge\; s'=\mathsf{apply}(s,e)
$$

Here $\tau_{P,v}$ is the profile's transition schema (§2.6) and $e_I$ contains only intent-side lines (owner-paid solver fee, disclosure). $\mathsf{Stage}_2.\mathsf{GuardsTrue}$ evaluates the signed Φ₀ over the **same** $e$ and $s'$. A candidate that satisfies the trader and violates $\tau$, or the reverse, rejects.

$$
\mathsf{HeadFresh}(C,s)\iff \forall c\in C:\ \mathsf{hv}_\pi(c)=\mathsf{hv}_s(c)\quad\text{else } \mathsf{Rej.CORE.HEAD\_STALE}.
$$

This is ledger-version concurrency: two proofs that read the same pool or lock head cannot both be accepted. It is the serialization rule for permissionless keepers (see conflict C4).

$$
\mathsf{PolicyPinned}(s,e)\iff \forall x\in\mathsf{objects}(e):\ \mathsf{evalPolicy}(x)=\mathsf{pin}_s(x)\ \vee\ \mathsf{MigrationAuthorized}(x)
$$

### 2.4 Certified operation basis Ω [obligation: U1 certificate each]

| Operation | Relation checked in-circuit | Width premise $W$ | Failure |
|---|---|---|---|
| $\Omega.\mathsf{divmod}(n,m)\to(q,r)$ | $q\cdot m+r=n \wedge 0\le r<m \wedge m>0$ | $n<2^{252}$, $m<2^{252}$, $q\cdot m<2^{252}$ | `Rej.CORE.DIV0` / `Rej.CORE.WIDTH` |
| $\Omega.\mathsf{mulcmp}(a,b,c,k)$ | $a\cdot b \le c\cdot k$ (strict variant $<$) | $\lceil\log_2 a\rceil+\lceil\log_2 b\rceil\le 252$, likewise for $c\cdot k$ | `Rej.CORE.WIDTH` |
| $\Omega.\mathsf{pos}(x,y)$ | returns $x-y$ if $x\ge y$, else $0$, with a constrained Boolean selector | same width as $x$ | none (total) |

The 252-bit bound keeps every product under the 253-bit `less_than` limit recorded in MIL/2 §4.5. Operands that exceed it must use the MIL/2 two-limb gadget, which is a separate certificate. Ω results may feed only a profile transition rule or a literal-coefficient Φ₀ comparison. They never become Φ terms (so Φ₁ stays deferred).

### 2.5 Family-to-constructor map

"Cells" lists additions to the MIL/2 §9 vocabulary. "Core" lists which of K1–K13 each family leans on hardest. Every family also inherits all thirteen.

| Family | Profile id | New cells | Constructors (effects) | Ω used | Rights | Core leaned on | Profile policy (versioned) |
|---|---|---|---|---|---|---|---|
| AMM | `amm-cp/1` | `poolHead(p)`; reserves = `bal(d,cust(p),X/Y)` | `swapExactIn` | divmod, mulcmp | `complete` | K2, K6, K10–K12 | fee $f/F$, width $w$, one hop, exact-in only |
| Lending | `loan-fixed/1`, `loan-coll/1` | `obligation(o)`, `encumbrance(k)`, `lockTotal(d,p,a)` | `originate`, `accrue`, `repay`, `lock`, `release`, `liquidate` | divmod, mulcmp | `initiate`, `enforce` | K4, K5, K7, K9 | rate, period, LT, close factor, bonus, oracle Δ |
| Stablecoin | `cdp/1` | `vault(v)`, `debtTotal(ilk)`, `mode(sys)` | `mintAgainst`, `burnRepay` | mulcmp | `issue` (scoped) | K3, K4, K5 | ceiling, collateral ratio, mode rules |
| Derivative | `opt-capped/1` | `instrument(ι)`, `fixing(ι)`, `claim(c)` | `write`, `fix`, `exercise`, `reclaim` | divmod (literal den), pos | `issue` (claims), `complete` | K4, K7, K8 | $K$, $K_c$, $T$, fixing rule, size |
| Oracle | `obs/1` (core-adjacent) | `observation(feed)` with round/seq | `admit` (read-only) | none | none | K8, K11, K13 | feed id, Δ, clock tolerance |
| Governance | `gov-timelock/1` | `policyHead(π)`, `grantEpoch(g)`, `queue(q)` | `queue`, `execute`, `cancel` | none | `amend` (threshold) | K7, K9, K11 | delay, grace, $k$-of-$n$, protected exits |
| Bridge | `brg-lock-mint/1` | `transfer(τ)` (src), `receipt(τ)` (dst), `backing(dst,asset)` | `srcLock`, `dstReceive`, `dstTimeout`, `srcRefund` | divmod (decimals) | `issue` (β=bridge) | K3, K7, K8 | verifier policy digest, timeout clock, ratio |
| Vault | `vault-4626/1` | `shareSupply(v)`, `managed(v)`, `req(q)`, `index(v)`, `ckpt(v,p)` | `deposit`, `redeem`, `request`, `finalize`, `claim`, `slash`, `reward` | divmod | `issue` (shares), `enforce` (slash) | K2, K4, K7, K12 | offset $(V_s,V_a)$, queue delay, rate rule |

### 2.6 Minimal operational schemas

Every schema is a rule of the form "premises ⟹ $\tau=\mathsf{Ok}(e_P,W_P)$". Each premise lists the rejection code produced if it fails. Premises are checked in the listed order, following the repository's admission order (bounds → declaration → static → snapshot → metered reduction, `.tex` §1).

#### 2.6.1 AMM `swapExactIn` (`amm-cp/1`)

**Arguments:** pool $p$ over assets $(X,Y)$; input $\Delta x\in\mathbb U_w$ (a hole); owner $u$; recipient $r$.
**Profile policy:** literal fee $f<F\le 2^{16}$; width $w=112$ (decision D2).
**Reads:** $x=\mathsf{bal}_s(d,\mathsf{cust}(p),X)$ and $y=\mathsf{bal}_s(d,\mathsf{cust}(p),Y)$, both at $\mathsf{hv}_s(\mathsf{poolHead}(p))$.

$$
\frac{
\begin{array}{ll}
x,y,\Delta x,\ x+\Delta x \in\mathbb U_{w} & \text{else } \mathsf{Rej.AMM.WIDTH}\\
\Delta x>0,\ x>0,\ y>0 & \mathsf{Rej.AMM.EMPTY}\\
\gamma=F-f;\ (q,\rho)=\Omega.\mathsf{divmod}(y\gamma\Delta x,\; xF+\gamma\Delta x) & \mathsf{Rej.CORE.WIDTH}\\
\Delta y = q,\ 0<\Delta y<y & \mathsf{Rej.AMM.ZERO\_OUT}/\mathsf{DRAIN}\\
\Omega.\mathsf{mulcmp}\big(x\,y,\;F,\;(x{+}\Delta x)F-f\Delta x,\; y-\Delta y\big) & \mathsf{Rej.AMM.K}\\
\mathsf{HeadFresh}(\{\mathsf{poolHead}(p),\mathsf{bal}(\cdot)\}) & \mathsf{Rej.CORE.HEAD\_STALE}
\end{array}}
{\tau=\mathsf{Ok}\big(\ [\,\mathsf{bal}(u,X){-}\Delta x,\ \mathsf{bal}(\mathsf{cust}(p),X){+}\Delta x,\ \mathsf{bal}(\mathsf{cust}(p),Y){-}\Delta y,\ \mathsf{bal}(r,Y){+}\Delta y,\ \mathsf{poolHead}(p){+}1\,]\big)}
$$

- **Width check** [checked by arithmetic]: $y\gamma\Delta x<2^{112+16+112}=2^{240}$ and $xF+\gamma\Delta x<2^{130}$. The fee-adjusted K products are below $2^{242}$. Everything fits Ω's 252-bit premise with no limbs. With $w=128$ the numerator reaches $2^{272}$ and needs limbs.
- **Exactness:** $\Delta y$ is fixed by divmod's uniqueness. A prover cannot pay the trader more than the quote, so LP value cannot leak.
- **Trader clauses** stay Φ₀ with literal coefficients: $\Delta x\le G$; $\Delta y\ge m$; LP-fee cap $\Delta x\cdot f\le \mathit{feeCap}\cdot F$; recipient $=r$.
- **Rounding role:** output is floored against the trader. The remainder $\rho$ stays in the pool (beneficiary: LPs through the reserves). This conflicts with the U0 default; see C2.

#### 2.6.2 Lending (`loan-fixed/1`, then `loan-coll/1`)

**Obligation state:** $o=(p,a,u,\mathit{terms},\pi,\mathit{st})$ with $\mathit{st}\in\{\mathsf{active},\mathsf{discharged},\mathsf{impaired}\}$.

**Originate.** Premises:

- The debtor's consent signature covers $H(\mathit{terms})$, checked at the ledger boundary (not in-circuit; MIL/2 §11). Else `Rej.LOAN.CONSENT`.
- A funded transfer creditor→debtor of exactly $p$. Else `Rej.LOAN.UNFUNDED`.
- The new obligation has $u=p$, $a=0$, $\pi$ pinned, and $\mathsf{Tomb}(o)$ absent.

**Accrue** (period index $k$, written into the obligation): $\Delta a=\lceil p\cdot r_n/r_d\rceil$ through divmod (+1 when $\rho>0$). This is the U0 `accrual-interest` primitive, whose remainder beneficiary is `protocol-reserve`, currently absent (C2). Replaying period $k$ rejects `Rej.LOAN.PERIOD`.

**Repay$(n)$:**

- Requires $0<n\le u$ (`Rej.LOAN.OVERPAY`) and a funded transfer debtor→creditor of $n$.
- Applies $d_a=\min(n,a)$, $d_p=n-d_a$ (AccrualFirst).
- If $u'=0$: set $\mathit{st}'=\mathsf{discharged}$ and write $\mathsf{Tomb}$.
- An encumbrance $k$ with $\mathit{against}(k)\ni o$ may be released only when **every** obligation in $\mathit{against}(k)$ is discharged (`Rej.LOAN.RELEASE_EARLY`).

**Lock$(k,c)$:** $\mathsf{lockTotal}'=\mathsf{lockTotal}+c\le \mathsf{bal}(\text{debtor},C)$, per K5.

**Liquidate$(n)$** (`loan-coll/1` only):

- An admitted anchored observation $\pi_{C/D}$ (Price⟨C,D,s⟩, meaning mantissa$/10^s$ units of C per one D) whose round is the round at the current head (`obs/1`).
- Unhealthy: $\Omega.\mathsf{mulcmp}(c\cdot\theta_d,\;10^s,\;u\cdot\theta_n,\;\pi)$ is **false** in the healthy orientation. Else `Rej.LOAN.HEALTHY`.
- Close factor in Φ₀: $n\cdot CF_d\le u\cdot CF_n$ (`Rej.LOAN.CLOSE`).
- Seizure: $z=\lfloor n\,(B_d{+}B_n)\,\pi/(B_d\,10^s)\rfloor$, floored against the keeper, remainder left to the debtor.
- If $z>c$: $z=c$, and the unrecovered portion of the obligation moves to $\mathsf{impaired}$ with residual $u'>0$. It is never deleted (K4).
- Must be head-fresh on $\mathsf{encumbrance}(k)$ and $\mathsf{lockTotal}$. `enforce` is scoped by the consented policy $\pi$.

#### 2.6.3 CDP (`cdp/1`)

$\mathsf{mintAgainst}(v,\Delta)$:

- $\mathsf{mode}\ne\mathsf{shutdown}$.
- `issue` grant for $(d,S)$ with budget $\ge\Delta$.
- $\mathsf{debtTotal}'=\mathsf{debtTotal}+\Delta\le\mathit{ceiling}$ (Φ₀), head-fresh.
- $u'_v=u_v+\Delta$.
- $\Omega.\mathsf{mulcmp}$ collateral test: $\mathit{ink}\cdot\pi\cdot CR_d \ge u'_v\cdot 10^s\cdot CR_n$ (`Rej.CDP.UNSAFE`).
- Effects: `mint`$(S,\Delta,\beta{=}\mathsf{cdp}(v))$ and $\mathsf{bal}(\text{owner},S){+}\Delta$. K2 and K3 then give $\Delta\mathsf{supply}=\Delta$.

$\mathsf{burnRepay}(v,\Delta)$: burn $\Delta$ from the owner, $u'_v=u_v-\Delta$, $\mathsf{debtTotal}{-}\Delta$. Collateral release requires the ratio after release. The mode is one-way ($\mathsf{active}\to\mathsf{guarded}\to\mathsf{shutdown}$). Shutdown settlement needs a complete claimant snapshot. [deferred]

#### 2.6.4 Option: capped cash-settled call (`opt-capped/1`)

The instrument is $\iota=H(U,Q,K,K_c,T,\text{European},\text{cash},\phi,\mathit{size})$, with $K<K_c$ as price mantissas at scale $s$.

**Write.** Premises:

- The writer funds $\mathsf{cust}(\iota)$ with $M=\lfloor\mathit{size}(K_c-K)/10^s\rfloor$ Q (exact when the profile requires $10^s \mid \mathit{size}(K_c{-}K)$).
- Issue claim $c$ to the holder, with $\mathsf{Tomb}(c)$ absent.

**Fix.**

- Fixing is write-once: `Rej.OPT.REFIX` if already set.
- Takes an admitted observation of feed $\phi.\mathit{feed}$, with round $\rho$ satisfying $\mathsf{observedAt}(\rho)\ge T$ and $\mathsf{observedAt}(\rho-1)<T$. This needs predecessor-round evidence; see D8.

**Exercise$(c)$.**

- Requires the fixing set and $\mathsf{Tomb}(c)$ absent (`Rej.CORE.REPLAY`).
- Payoff: $P=\lfloor \mathit{size}\cdot\min(\Omega.\mathsf{pos}(S,K),\,K_c-K)/10^s\rfloor$, floored against the holder, with the remainder to the writer via the residual.
- Effects: transfer $P$ from $\mathsf{cust}(\iota)$ to the holder, then write $\mathsf{Tomb}(c)$.

**Reclaim.** Allowed after the fixing. The writer takes $M-P$. $P$ stays reserved until the claim is exercised. Expiry never erases the holder's payable (K4).

#### 2.6.5 Oracle `admit` (`obs/1`)

An observation record is $o=(\mathit{id},\mathit{feed},\mathit{val}{:}\mathsf{Price}\langle B,Q,s\rangle,\mathit{origin},\mathit{attestor},\pi_{ev},\mathit{round},t_{obs},\mathit{status},\mathit{cm})$.

$o$ is admitted as $\mathsf{anchored}@d$ iff all of the following hold:

- $o$ is read from $\mathsf{observation}(\mathit{feed})$ at $\mathsf{hv}_s$ (K11), and the cell commitment equals $\mathit{cm}$. Else `Rej.ORC.UNBOUND`.
- Unit and feed match the declaration (`Rej.ORC.UNIT`/`FEED`).
- $t_{obs}\le t$ (`Rej.ORC.FUTURE`) and $t-t_{obs}\le\Delta$ on the same clock (`Rej.ORC.STALE`).
- $\mathit{status}=\mathsf{live}$ (`Rej.ORC.REVOKED`).

Because the cell holds only the latest round at the head, "fresh" cannot select a favorable older round. `imported`/`attested` admission is [deferred] to U4 except under a named, signed trust premise.

#### 2.6.6 Governance (`gov-timelock/1`)

**Queue$(\alpha)$** records $q=(\alpha,\ \mathit{eta}=t+\delta,\ v=\mathsf{policyHead}.\mathit{ver},\ g=\mathsf{grantEpoch},\ \mathit{appr},\ \mathsf{pending})$. Premises:

- $\delta\ge\delta_{min}$ (profile).
- $\mathit{appr}$ holds $\ge k$ **distinct** eligible keys at epoch $g$, verified at the ledger boundary (C7).
- The approval-set digest is consumed via $\mathsf{Tomb}$.

**Execute$(q)$.** Premises:

- $q.\mathit{st}=\mathsf{pending}$.
- $\mathit{eta}\le t\le\mathit{eta}+\mathit{grace}$ (`Rej.GOV.EARLY`/`EXPIRED`).
- $\mathsf{grantEpoch}=g$ (`Rej.GOV.REVOKED`).
- $\mathsf{policyHead}.\mathit{ver}=v$ (`Rej.GOV.HEAD`).
- The effect is exactly $\alpha$: $\mathsf{policyHead}\leftarrow(v{+}1,H(\text{new}))$.
- Protected exits (repay, withdraw, recover, challenge) listed in the pinned policy remain enabled (`Rej.GOV.EXIT_BLOCKED`).
- Existing objects stay pinned to $v$ (K9).

**Cancel$(q)$:** writes $q.\mathit{st}=\mathsf{cancelled}$. Execute and cancel both write $\mathsf{queue}(q)$, so K11 admits at most one at a given head.

#### 2.6.7 Bridge (`brg-lock-mint/1`)

The transfer is $\tau=H(d_s,d_t,A,A',\text{lock-mint},n,\mathit{ratio},\mathit{from},\mathit{to},\mathit{nonce},\pi_{ver},T_{out}@\mathsf{clk}(d_t))$.

- **srcLock** (on $d_s$): transfer $n$ A to $\mathsf{cust}(\tau)$ and set $\mathsf{transfer}(\tau)=\mathsf{pending}$.
- **dstReceive** (on $d_t$). Premises:
  - $\mathsf{receipt}(\tau)=\mathsf{absent}$ (`Rej.BRG.REPLAY`).
  - $t<T_{out}$ (`Rej.BRG.LATE`).
  - Imported evidence of srcLock verified under $\pi_{ver}$. The verifier assigns the class, and replay is keyed on $\tau$, not on proof bytes. Else `Rej.BRG.EVIDENCE`.
  - $n'=\lfloor n\cdot\mathit{ratio}_n/\mathit{ratio}_d\rfloor$, with the dust remainder to $\mathit{from}$ via the source refund.
  - `mint`$(A',n',\beta{=}\mathsf{bridge}(\tau))$ and $\mathsf{backing}'\le$ attested-locked (under the premise).
  - Write $\mathsf{receipt}(\tau)=\mathsf{received}$.
- **dstTimeout** (on $d_t$): $t\ge T_{out}\wedge\mathsf{receipt}(\tau)=\mathsf{absent}$ writes $\mathsf{timedOut}$. It is one-shot and excludes a later receive.
- **srcRefund** (on $d_s$): requires imported evidence that $\mathsf{receipt}(\tau)=\mathsf{timedOut}$. A local deadline alone gives `Rej.BRG.NONRECEIPT_UNPROVED`. An unknown outcome stays `pending`.

#### 2.6.8 Vault (`vault-4626/1`)

Totals $S=\mathsf{shareSupply}(v)$ and $A=\mathsf{managed}(v)$ are accounted cells, not custody balances (D5). The offset $(V_s,V_a)$ is literal and immutable.

| Operation | Rule | Rounding role |
|---|---|---|
| deposit$(a)$ | $sh=\lfloor a(S{+}V_s)/(A{+}V_a)\rfloor$, $sh>0$ (`Rej.VLT.ZERO`) | floor, remainder to the vault |
| redeem$(sh)$ | $a=\lfloor sh(A{+}V_a)/(S{+}V_s)\rfloor$ | floor, remainder to the vault |
| mint / withdraw | ceil on assets in / shares burned | [deferred to /2] |

- **Request$(sh)$:** escrows **shares** into $\mathsf{req}(q)$, so pending requests remain slashable pro rata.
- **Finalize:** rate $=\min(\text{request},\text{finalize})$ (D6).
- **Claim:** one-shot.
- **Slash$(L)$:** $A'=A-L$ with $S$ unchanged. Needs an `enforce` verdict replay-consumed via $\mathsf{Tomb}(\mathit{verdict})$.
- **Reward$(R)$:** $\mathit{index}'=\mathit{index}+\lfloor R\cdot 10^{k}/S\rfloor$. Any share write first settles $\mathsf{ckpt}(v,p)$. The source of $R$ must be an `issue` with $\beta=\mathsf{reward}$ or a reserve transfer.
- **Width:** $a(S{+}V_s)$ with 128-bit operands exceeds the field, so this profile needs $w=112$ or limbs (D2).

---

## 3. Effect on each of the eight categories

| Category | Smallest accepted vertical slice | Earliest milestone | Native statement must bind | Positive control | Hostile controls (valid envelope, one fact mutated) | Preserved limit |
|---|---|---|---|---|---|---|
| AMM | one pool, exact-in, one hop, fixed fee, pre-existing liquidity, no LP mint/burn | U2 (after U1 certifies divmod + mulcmp) | intent digest, pool id/head, $x,y$, $\Delta x,\Delta y,\rho$, four balance lines, tombstone | §4 trace T1 | +1 output; stale head; missing cell; wrong recipient; LP fee hidden; K violation | concentrated/weighted liquidity, multi-hop Episodes, clearing |
| Lending | `loan-fixed/1`: originate → partial repay → discharge, no oracle | U2 (the existing loan requirement, I2/MC02 lineage) | consent digest, obligation before/after, funded transfers, AccrualFirst split, tombstone | originate 1,000, repay 400, repay 600 → discharged | debt-reducing flag without transfer; repay > $u$; release before all `against` discharged | variable rates, pooled shares, flash |
| Lending (collateral) | `loan-coll/1` one liquidation | U3 (needs `obs/1`) | + observation id/round, lockTotal, encumbrance head, $z$, impaired residual | liquidate at close factor | stale round; double pledge (T3); concurrent keeper stale head; seize > c without impairment | ADL, backstop waterfall |
| Stablecoin | `cdp/1` mint against one vault, fixed ceiling | U2/U3 boundary (needs mulcmp + a literal or anchored price) | grant id/budget, β tag, debtTotal, vault, supply, price | mint within ratio and ceiling | positive supply with no debt line; ceiling exceeded against a stale total; burn replenishing quota | algorithmic/reflexive backing, rebasing, shutdown settlement |
| Derivatives | `opt-capped/1`: write → fix → exercise → reclaim | U3 (persistent claim across stages) | $\iota$, fixing round, claim tombstone, payoff, custody | T4 | double exercise; wrong round; early exercise; payoff with $K_c$ removed | perps, funding, margin, socialized loss |
| Oracles | `obs/1` admission inside the loan-coll or CDP slice | U2 (anchored only) | feed, value, unit, round, $t_{obs}$, head, commitment | admitted fresh price | future timestamp; wrong unit; older round; witness-supplied `anchored` label; malformed observation on a skipped branch | TWAP/median, imported/attested (U4) |
| Governance | single amend of one policy cell via queue → execute, pre-existing obligation stays on $v$ | U3 (pending queue state) | policyHead before/after, grantEpoch, queue item, approval-digest tombstone | T5 | revoke before execute; replayed approvals; beneficiary-changing $\alpha'$; paused repayment | vote snapshots, delegation chains |
| Bridge | srcLock + dstTimeout + srcRefund **under a named premise**; dstReceive under a named verifier premise | U3 local; U4 for actual imported verification | $\tau$, receipt state, evidence bytes/policy, backing, supply | lock → receive | late receive after timeout; replay with re-serialized proof; refund on local deadline only; amount/ratio mutation | fast fill, reorg compensation, global rollback |
| Vault | deposit + redeem, one class, fixed offset | U2/U3 (divmod, $w=112$) | $S,A$ heads, sh/a, $\rho$, offset constants | deposit then redeem | forged quotient; zero-share deposit; donation-inflated $A$; stale total | restaking, strategy mandates, mint/withdraw |

**Structurally contrasting U2 pair:** U2 needs "a newly authored program plus a structurally contrasting program" (`ROADMAP.md:23`). I recommend `loan-fixed/1` and `amm-cp/1`. The first continues the existing loan requirement and the I2/MC02 focus. The second exercises Ω and program-custody reserves. Both are single-domain, $|S|=1$, and need no oracle.

**Development focus:** the develop skill defers "new profiles, broad ACTUS/DeFi expansion" during I2 (`plugins/moriarty-dev/skills/develop/SKILL.md`, §2). This sequencing is therefore specification preparation only. It does not change the current delivery item.

### 3.1 U0–U6 sequencing (proposed)

| Milestone | Core work | Family work |
|---|---|---|
| **U0** | Reserve every new cell tag in §2.5. Add the ProgramValid, HeadFresh and PolicyPinned judgment keys to the stage relation. Add the β backing tag, Ω signatures and width premises, role-directed rounding, and a remainder-posting cell. Resolve C2. | Profile ids and schemas recorded as specified-only. No enforcement claim. |
| **U1** | Certify Ω.divmod, Ω.mulcmp and Ω.pos natively: valid-witness completeness, adversarial soundness, measured cost. Measure $w=112$ against the limb path. | Pick AMM invariant form (divmod versus K plus tightness, D3) by equivalence proof plus cost. |
| **U2** | General single-stage path. | Accept `loan-fixed/1` and `amm-cp/1` with native verification and ledger readback. |
| **U3** | Ledger-linked persistence, pending, races, recovery. | `loan-coll/1` + `obs/1`, `cdp/1`, `opt-capped/1`, `gov-timelock/1`, vault request/finalize, bridge local legs under a premise. |
| **U4** | Imported and attested verification, recursion. | Bridge dstReceive with an actual verifier mode; attested oracles. |
| **U5** | Solvers and federation. | Route neutrality for AMM; keeper competition. |
| **U6** | Library conformance. | Every retained DeFi row, held-out behaviors, second profile versions. The deferred list stays counted as not complete. |

---

## 4. Transaction traces

All traces use domain $d$ = `midnight.preview` and $|S|=1$.

**T1. Valid AMM swap: expected ACCEPT at the stage relation (native acceptance not claimed).** [numbers checked by arithmetic]

- Pool state: $x=1{,}000{,}000$ X and $y=2{,}000{,}000$ Y at head 41. Fee: $f/F=3/1000$.
- Intent: $G=10{,}000$ X, $m=19{,}700$ Y, feeCap $=30$ X, recipient = owner.
- Completion: $\sigma(\Delta x)=10{,}000$.
- divmod: $(19{,}940{,}000{,}000{,}000,\ 1{,}009{,}970{,}000)\to q=19{,}743$, $\rho=162{,}290{,}000$.
- K check: $(1{,}010{,}000\cdot1000-30{,}000)\cdot1{,}980{,}257=2{,}000{,}000{,}162{,}290{,}000\ \ge\ 2\cdot10^{15}$ ✓.
- Φ₀ trader clauses: $10{,}000\le G$ ✓, $19{,}743\ge m$ ✓, $10{,}000\cdot3\le30\cdot1000$ ✓.
- Effects: $-10{,}000$ X owner, $+10{,}000$ X cust, $-19{,}743$ Y cust, $+19{,}743$ Y owner. Pool head → 42. Intent tombstone written.
- E1 holds per asset, the footprint has 7 cells (≤ 32 cap), and there are 6 effect lines (≤ 16).

**T2. Hostile over-delivery: expected REJECT `Rej.CORE.WIDTH`/divmod failure, before `GuardsTrue` matters.**

- Same as T1, but the witness claims $\Delta y=19{,}744$. The envelope is fully valid: $19{,}744\ge m$ and the recipient is correct.
- divmod requires $19{,}744\cdot1{,}009{,}970{,}000+\rho=19{,}940{,}000{,}000{,}000$, so $\rho<0$. There is no witness.
- Independently, K gives $1{,}999{,}999{,}152{,}320{,}000<2\cdot10^{15}$, so `Rej.AMM.K` also fires [checked].

**T2′. Hostile stale head:** the witness reads reserves at head 41 while the ledger is at 42 (another swap landed). Expected `Rej.CORE.HEAD_STALE`, even though every arithmetic check passes on the stale values.

**T3. Hostile double pledge (`loan-coll/1`): expected `Rej.CORE.LOCK_EXCEEDS`.**

- Debtor balance: 100 C. The existing encumbrance $k_1$ of 60 is recorded, so $\mathsf{lockTotal}=60$.
- A new origination locks $k_2$ of 50. The envelope is valid, and $k_2$ alone satisfies $50\le100$.
- The core check fails: $60+50=110>100$.
- Variant: the witness reads $\mathsf{lockTotal}=0$ at an older head, giving `Rej.CORE.HEAD_STALE`.

**T4. Valid option exercise, then hostile re-exercise.**

- Terms: $K=200$, $K_c=300$, $s=2$, size $=1{,}000{,}000$ Q units. Collateral $M=1{,}000{,}000$.
- Fixing: $S=250$. Payoff $P=\lfloor10^6\cdot\min(50,100)/100\rfloor=500{,}000$ [checked]. ACCEPT. Writer reclaim $=500{,}000$.
- A second exercise of claim $c$ with an otherwise identical envelope gives `Rej.CORE.REPLAY` ($\mathsf{Tomb}(c)$ is present).

**T5. Hostile governance: revocation before execute.**

- Queue item $q$ was queued at grantEpoch 7 with 3-of-5 approvals. A later stage revokes one key, bumping the epoch to 8. $t\ge\mathit{eta}$.
- The execute envelope is valid: the action digest matches and the head version is $v$.
- Expected `Rej.GOV.REVOKED`. The pre-existing obligation pinned to $v$ is unaffected either way.

**T6. Hostile bridge refund on local deadline: expected `Rej.BRG.NONRECEIPT_UNPROVED`.** $t_s>T_{out}$ on the source clock, but there is no evidence of $\mathsf{receipt}(\tau)=\mathsf{timedOut}$. The transfer stays `pending` and the duty is retained.

---

## 5. Conflicts with MIL/2 and the eight five-review syntheses

Each of the eight category directories has a five-review synthesis. I found these conflicts and counterexamples rather than smoothing them over.

- **C1. Ω before Φ₁ versus MIL/2 owner decision 1** (`DESIGN-MIL2.md:158-165`, `:348-350` puts pool arithmetic and variable products at U4). The AMM, lending, stablecoin, oracle and vault syntheses all recommend a narrow certified operation earlier. This draft adopts that direction as **alternative A** but does not decide it (D1). It changes owner decision 1, so a design vote is needed.
- **C2. Rounding remainder has nowhere to post** [checked].
  - MIL/2 §3.3 says the vault remainder class is `retained-in-pool` (`DESIGN-MIL2.md:89`).
  - The U0 numeric profile's default beneficiary is `protocol-reserve`, but `reserveMechanism.status` is `"absent"` and every floor/ceil primitive is an `open-gap` (`numeric-profile.json`, `defaultPolicy`/`reserveMechanism`).
  - K12 makes this a hard authoring-time rejection, so **no schema in §2.6 is admissible until U0 either posts the remainder or adopts per-profile beneficiaries** (pool LPs, vault holders, writer residual).
- **C3. Author-selectable rounding.** MIL/2 `sharesFor(…, Rounding)` and U0 `AccrualTerms.rounding` (floor/ceil selectable, already an open gap) conflict with the operation-directed rounding recommended by the vault synthesis. K12 removes the parameter.
- **C4. Seize ordering.** MIL/2 §6 requires a "signed total order on seizes of one encumbrance id" (`DESIGN-MIL2.md:193`). The lending synthesis (review 5) recommends ledger head serialization, and the vault synthesis (review 3) recommends origination priority. This draft uses HeadFresh for permissionless keepers and keeps the signed order only for owner-declared priority among consented enforcers (D4).
- **C5. Width.** MIL/2 `Qty` is u128 (`:283`). Ω without limbs needs $w=112$ for AMM and vault [checked by bound arithmetic]. The AMM synthesis records this as disputed: review 1 favors two-limb u128.
- **C6. Showcase footprint.** The MIL/2 §13 `AcquireB` footprint omits `balance(counterparty,A)` and `balance(owner,A)` (`RESEARCH-FINDINGS.md` R2). K6 rejects it. This draft does not repair the showcase.
- **C7. Threshold authority without in-circuit signatures.** MIL/2 §11 states there is no Ed25519/SHA-512 in ZKIR v3. Governance `k`-of-`n` distinct keys must therefore be checked at the ledger boundary or through pre-recorded approval receipts. That is a trust premise on the enforcement map, not a circuit fact.
- **C8. The derivatives slice is contradictory as written.** The derivatives synthesis proposes a "fully collateralized, cash-settled European call" (`opus55-derivatives-recommendations/SYNTHESIS.md:91`). An uncapped call's quote-asset payoff is unbounded, so no finite quote collateral fully collateralizes it. Repairs:
  - (a) a capped call (this draft);
  - (b) payout in underlying units, $\le$ size U, which needs divmod by $S$;
  - (c) a cash-settled put, bounded by $K$.
- **C9. Escrow priority versus bridge timeout.** MIL/2 escrow `priority release|refund|signed_order` is a local choice. For bridges the receive/timeout race must be decided at the **destination** receipt (bridge synthesis items 3 and 5), so a source-side priority field is insufficient and cannot be the refund authority.
- **C10. Bridge milestone.** The bridge reviews 3 and 5 describe a U3 async slice. MIL/2 places imported verification at U4 (`:349`). This draft splits them: local legs at U3 under a named premise, verified receipt at U4. No "proven bridge" claim is made before U4.
- **C11. `fresh` permits round shopping.** MIL/2 `fresh(obs, Duration)` (`:144`) allows any fresh round. `obs/1` fixes this by reading only the head-current round. That removes the ability to select among valid rounds, which a TWAP profile might later want.
- **C12. Stablecoin backing dissent.** Review 1 allows an unbacked bucket; review 5 requires a backing co-effect. This draft requires the β tag on every mint and admits no `unbacked` profile. That is a partial resolution; the policy question stays open (D7).
- **C13. S0 supply exclusion.** The stablecoin category review cites a U0 S0 exclusion of nonempty supply changes (`R3-stablecoins.md:321`, via `category-review/03-stablecoins.md`; I did not open R3). A U2 CDP slice would contradict S0 if S0 is the U2 profile. `cdp/1` is placed at the U2/U3 boundary pending this reconciliation.
- **C14. Donation versus custody.** AMM reserves are custody balances here, so donations accrue to LPs. Vault $A$ is an accounted cell, so donations are unaccounted surplus. This asymmetry is deliberate but must be stated, and it differs from the comparative Uniswap v2 cached-reserve `sync`/`skim` design.

---

## 6. Unresolved decisions and verification obligations

### 6.1 Decisions (none taken here)

| # | Decision | Alternatives | Recommendation | Evidence needed |
|---|---|---|---|---|
| D1 | Where family arithmetic lives | A: Ω basis at U0/U1. B: keep MIL/2 (families wait for Φ₁ at U4). C: literal-price-only profiles | A | U1 certificate and cost for divmod/mulcmp, with valid and hostile witnesses. Needs two agreeing votes (owner decision 1 reversal). |
| D2 | Width | $w=112$ profile; u128 with two limbs; both | $w=112$ first, limbs as `/2` | Measured constraint counts for both on T1/T2 |
| D3 | AMM invariant form | divmod-exact; K plus $(\Delta y{+}1)$ tightness | divmod, with K as a redundant check | Equivalence proof over the declared widths and denominators |
| D4 | Seize ordering | signed total order; HeadFresh first-come; origination priority | HeadFresh plus signed priority among consented enforcers | Two-keeper concurrent trace; consent model review |
| D5 | Vault totals | accounted `managed` cell; custody balance | accounted cell | Donation/inflation hostile trace; reconcile with the AMM choice (C14) |
| D6 | Withdrawal rate | min(request, finalize); finalize-only | min | Fairness and reserve analysis |
| D7 | Unbacked issuance | tag exists but no profile; forbid in core | tag without profile | Issuer-class policy decision |
| D8 | Fixing round selection | first round ≥ T with predecessor proof; oracle-program "round-at(T)" | undecided | Oracle cell interface on Midnight; reorg control |
| D9 | Authenticated stage time $t$ | ledger block time; signed bound; interval | undecided (premise) | Target pin for a time primitive (`target-pins.json` has 10 unresolved) |
| D10 | Grant epoch granularity | global epoch; per-grant epoch | per-grant (a global epoch lets one revocation stall every queue) | Liveness trace |
| D11 | U2 contrasting pair | loan-fixed + amm-cp; loan-fixed + vault | loan-fixed + amm-cp | U2 generality argument |

### 6.2 Verification obligations [obligation, all open]

O1–O6 from MIL/2 and the `.tex` draft are unchanged. Proposed additions:

- **O7.** $\mathsf{NativeAccept}\Rightarrow\mathsf{ProgramValid}$: every $e_P$ line, and $\mathsf{hv}$ of every read, is bound in the public statement.
- **O8.** Ω certificates: completeness, soundness and width rejection. No field wrap.
- **O9.** K5 inductive: every debit path reads and updates `lockTotal`, with no bypass through a seize, slash or bridge lock.
- **O10.** Bridge pairing: at most one of {received, timedOut} per $\tau$, and mint ≤ attested lock under $\pi_{ver}$. State the failure under verifier compromise.
- **O11.** K9 preservation: amendment never changes the evaluation of pinned objects without an authorized migration.
- **O12.** K12 posting: every non-exact Ω remainder lands in a declared cell, with E1 still holding.
- **O13.** Per-profile cap measurement: the effect and footprint counts of each §2.6 schema fit the §12 caps.
- **O14.** K4 non-erasure over every terminal, expiry, pause and timeout transition in all eight profiles.

Every one of these needs a K/TypeScript differential plus native hostile controls before any category is counted. None is claimed here.

---

## 7. Anchors

### 7.1 Repository anchors (read this session)

- `concepts/intent-language/DESIGN-MIL2.md`: :46 Episode link fields; :80-89 pool arithmetic and `retained-in-pool`; :100-102 lock sum; :158-165 Φ₀/Φ₁; :167-171 width; :181-187 evidence; :191-193 authority and seize order; :199-219 escrow; :239-250 cells and derived footprints; :256 E1; :264 no in-circuit Ed25519; :270-285 caps; :332-338 decisions 2–5; :340-351 milestones; :363-365 obligations.
- `deliverables/mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex`: sections "Stage acceptance relation" (E1, L1), "Escrow…" (ESC), "Footprints…" (F1, K1), "Native correspondence" (O1–O6).
- `deliverables/mil2-deep-research-2026-09-29/RESEARCH-FINDINGS.md`: R1 through R7.
- `deliverables/mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md`: :9-16, :29-36.
- `category-review/01…08-*.md`: the closing sections read above.
- `opus55-*-recommendations/SYNTHESIS.md`, all eight. In particular AMM :16-38, derivatives :91, lending :160, staking :251-255.
- `ROADMAP.md`: :21-28 (U0–U7), :38 (U3 discriminator).
- `deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md`: :5-14.
- `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json`: `defaultPolicy`, `reserveMechanism`, `primitives[accrual-interest …]`.
- `plugins/moriarty-dev/skills/develop/SKILL.md`: product boundary; step 2 focus.

### 7.2 Comparative primary sources (not Moriarty evidence)

**In the captured corpus** (`deliverables/mil2-deep-research-2026-09-29/sources.json`):

- ERC-4626, <https://eips.ethereum.org/EIPS/eip-4626>: direction-specific rounding per operation.
- IBC ICS-004, <https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md>: timeout requires destination non-receipt proof.
- IBC v2 packet handler, <https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md>.
- SMT-LIB logics, <https://smt-lib.org/logics-all.shtml>.
- ZKIR v3 (pinned), <https://github.com/midnightntwrk/midnight-zkir/blob/47793c8ab042aa5a91d1a4672c6b82de6bdf9dd8/zkir-spec/src/zkir-v3/README.md>.
- Nomos, <https://arxiv.org/abs/1902.06056>.

**Cited by the syntheses but not captured in `sources.json`; I did not re-fetch them this session:**

- Uniswap v2 pair, <https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol>: `uint112` reserves; fee-adjusted K with $1000^2$ scaling. This is the basis of the $w=112$ and T1 K-check forms.
- Uniswap v3 whitepaper, <https://app.uniswap.org/whitepaper-v3.pdf>.
- Balancer weighted math, <https://github.com/balancer/balancer-v2-monorepo/blob/master/pkg/pool-weighted/contracts/WeightedMath.sol>.

**My own comparative additions, not captured and not fetched this session; verify before relying on them:**

- MakerDAO `vat.frob` collateral-safety check, <https://github.com/makerdao/dss/blob/master/src/vat.sol>. Analogy for `cdp/1` only.
- Compound `Timelock` queue/execute/grace, <https://github.com/compound-finance/compound-protocol/blob/master/contracts/Timelock.sol>. Analogy for `gov-timelock/1` only.

Everything above is an independent agent's specified-only proposal. Nothing here was accepted, adopted, implemented, compiled, proved or submitted to Preview.