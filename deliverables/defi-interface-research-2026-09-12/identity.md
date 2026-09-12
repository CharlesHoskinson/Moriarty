# Identity — what is missing from Part I section 2

Scope: the identity surface only. Read against Part I sections 1-7 of
`docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md`, and against what the
document already settles below the line: section 10 (`RecoveryPolicy`, `NetworkBinding`, `CallPermission`),
section 11 (`createCapability`, `revokeCapability`, `deriveForeignAccount`), section 12 (no `signHash`,
no caller-supplied derivation path), section 16 (the derivation path, which contains a `keyEpoch`
rotation index, and the sentence "Derivation separates control, not visibility").

Two structural observations frame everything below.

- Section 2 today makes four negative promises (you never create a key, never connect a wallet, never
  select a chain, never see your identity handed over) and one positive one (your account already
  exists). It says nothing about the *lifetime* of the identity or of anything granted under it. Every
  gap below is an event in that lifetime: a lost authenticator, a stolen one, an expiring grant, a
  third party asking "is this account yours", a network leaving the routable set.
- Below the line the document has already decided things that contradict or undercut section 2, and a
  reader of Part I cannot learn them. `keyEpoch` in the derivation path means rotation changes every
  derived account, while section 2 promises "your receiving address ... already exists and is already
  yours". Section 16 states there is no privacy claim for derived addresses, while section 2 presents
  one identity used everywhere with no warning that using it everywhere is observable.

---

## GAPS

### G1. The identity has no lifetime. Nothing covers a lost or stolen authenticator, and rotation silently changes every account.

**The gap.** Section 2 never says what authorizes the identity, that more than one thing can, that any
of them can be retired, or what happens when one is lost or compromised. `RecoveryPolicy` in section 10
is intent recovery (a stranded leg), not identity recovery. Section 16 carries `keyEpoch` as "Moriarty's
rotation index", so a rotation is expressible below the line — and it changes the derivation path, hence
every derived account on every network.

**Concrete failure 1 — permanent total loss.** The holder authorizes with one device-bound
authenticator. The design's own premise removed the fallbacks: no key to create, import or back up.
The device is lost. Platform-bound WebAuthn credentials cannot be exported and do not leave the device,
and passkey sync crosses devices only inside one vendor ecosystem, so an Android-only or
non-iCloud-synced credential dies with the hardware
([slashid passkeys deep dive](https://www.slashid.dev/blog/passkeys-deepdive/),
[Corbado on synced vs device-bound](https://www.corbado.com/blog/device-bound-synced-passkeys/synced-passkeys-cross-platform)).
Every account the kernel derived for that holder, on every network, is unreachable forever. This is not
an edge case in practice: Coinbase's smart wallet requires the recovery phrase to be generated *while
the original passkey is still live*, and states that losing the passkey afterwards leaves no recovery
path ([Coinbase smart wallet recovery](https://help.coinbase.com/en/wallet/getting-started/smart-wallet-recovery));
Web3Auth's 2-of-3 share model documents "lose 2 of 3 and the key is permanently unrecoverable"
([Web3Auth MFA docs](https://web3auth.io/docs/sdk/web/web3auth/multi-factor-authentication)); FIDO's own
guidance is that relying parties must let a user enrol more than one authenticator precisely for this
reason ([FIDO multiple-authenticators white paper](https://fidoalliance.org/wp-content/uploads/2020/06/FIDO_White_Paper_Multiple_Authenticators_CDWG.pdf)).

**Concrete failure 2 — rotation breaks the one positive promise section 2 makes.** The holder's
authenticator is compromised. The kernel advances `keyEpoch`. Every derived account changes. The
holder's employer pays them monthly to the old receiving account; a venue's withdrawal allow-list
contains the old account; a counterparty owes a settlement next week against the old account. All of
them now pay into accounts under a retired epoch, and section 2 told the holder their receiving address
"already exists and is already yours". Threshold custody shows this is avoidable, not inherent:
proactive share refresh replaces the shares of the *same* key with no address change, and the reason
vendors build it is exactly that the alternative — migrating every address — is costly and leaves a
window in which the old share still works
([Fireblocks on MPC](https://www.fireblocks.com/what-is-mpc/),
[proactive secret sharing](https://en.wikipedia.org/wiki/Proactive_secret_sharing)).

**Concrete failure 3 — recovery with no delay is a takeover.** If recovery is added later without a
delay and a veto, whoever the holder nominated can rotate the identity out from under them instantly.
Production designs all impose a timelock the holder can cancel inside: Safe's recovery hub defaults to
28 days, offers 7/14/56, and lets signers reject during *and* after the delay, explicitly because the
recoverer bypasses the account's own threshold
([Safe RecoveryHub](https://help.safe.global/articles/9622260218-account-recovery-with-saferecoveryhub));
Argent delays recovery 48 hours and delays *adding* a guardian 36 hours, so an attacker cannot add a
colluding guardian immediately before acting
([Argent guardian recovery](https://support.argent.xyz/hc/en-us/articles/360007338877-How-to-recover-my-wallet-with-guardians-onchain-complete-guide));
Buterin's canonical argument is that most value is lost to lockout rather than theft, and that the
guardian quorum must act only after a delay in which the signing key can cancel
([vitalik.eth.limo, social recovery](https://vitalik.eth.limo/general/2021/01/11/recovery.html)).
Analysis of recovery modules adds a sharper point: if the recovery delay is shorter than the veto path's
own timelock, the veto is decorative ([Cantina on smart-wallet recovery risks](https://cantina.xyz/blog/smart-wallet-social-recovery-risks)).

**Why it ranks first.** It is the only gap whose failure is unbounded and irreversible, and it defeats
*bounded loss* directly: the cap in the signed intent is irrelevant if the whole identity is gone.

### G2. A capability has no end, cannot be listed, cannot be narrowed when passed on, and is never named in an outcome.

**The gap.** Section 2: "a capability naming what it may do and what it may spend." No expiry. Section 5
exposes `delegate` and `revokeDelegation` but no read: no list of standing capabilities, no remaining
budget, no end date. Section 6 records outcomes with grades but never with the authority that produced
them. Meanwhile section 1 promises *no standing authority* — "no authority that outlives its intent". A
capability that can mint new intents indefinitely is standing authority by any ordinary reading, so
section 2 as written undercuts a guarantee in section 1.

**Concrete failure 1 — the forgotten grant.** A holder tries an application once, grants it a capability,
and never returns. Eight months later the application's server signer leaks. Nothing has expired,
because nothing was ever given an expiry; the only limit is the section 3 policy, so the loss is the
full remaining per-period budget, repeatedly. This is the non-expiring-approval failure in a new
costume, and the drains are documented: revoke.cash's exploit record lists roughly $11M taken from
users through a LI.FI facet and about $1.8M through an outdated Dolomite version, in both cases against
allowances users had never revoked ([revoke.cash/exploits](https://revoke.cash/exploits)). The
industry's fix was to put expiry in the authority itself rather than in a user's diligence — a mandatory
`deadline` in permit ([EIP-2612](https://eips.ethereum.org/EIPS/eip-2612)),
[Permit2](https://github.com/Uniswap/permit2), `validAfter`/`validUntil` returned from account
validation so the infrastructure and not the application enforces the window
([ERC-4337](https://eips.ethereum.org/EIPS/eip-4337)), `start`/`end` on Coinbase spend permissions
([coinbase/spend-permissions](https://github.com/coinbase/spend-permissions)), and a timestamp policy
that ZeroDev tells developers to combine with target, selector and value caps or "you risk granting
full account control" ([ZeroDev permissions](https://docs.zerodev.app/sdk/permissions/intro)).

**Concrete failure 2 — you cannot revoke what you cannot see.** After any incident the holder needs one
question answered: what can act for me right now. Section 5 has no call that answers it, so the answer
lives in whatever applications chose to remember. Every production permission system treats liveness,
remaining budget and expiry as reads against the enforcing layer, not application state: `isModuleInstalled`
([ERC-7579](https://eips.ethereum.org/EIPS/eip-7579)), `getCurrentPeriod` returning the live period's
spend-so-far ([spend-permissions accounting](https://github.com/coinbase/spend-permissions/blob/main/docs/SpendPermissionAccounting.md)),
role membership and per-period thresholds in the Roles modifier ([Zodiac Roles](https://docs.roles.gnosisguild.org/)).
The reason is adversarial, not ergonomic: a compromised or lagging application backend must not be the
record of its own authority.

**Concrete failure 3 — unattenuated hand-off (confused deputy).** An application granted a capability
hands it to its own execution vendor, or to an agent it runs. Nothing in section 2 says authority
cannot widen or outlive its parent when passed on. Capability systems make attenuation the defining
rule — macaroons, Biscuit and UCAN let a holder only *narrow* a derived credential
([UCAN spec](https://github.com/ucan-wg/spec/blob/main/README.md),
[Biscuit](https://www.clever.cloud/blog/engineering/2021/04/12/introduction-to-biscuit/)), and ERC-7710
makes redelegation explicit through caveat enforcers that can only tighten
([ERC-7710](https://eips.ethereum.org/EIPS/eip-7710),
[MetaMask delegation concepts](https://docs.metamask.io/smart-accounts-kit/concepts/delegation/)). The
documented harm when authority is a broad shared identity instead: the principal's own ceiling
"completely vanishes from the provider's view" and a low-privilege caller reaches through a
high-privilege agent ([agent permission gap](https://www.andromedasecurity.com/blogs/ai-agent-permission-gap),
[IETF draft on attenuating agent tokens](https://datatracker.ietf.org/doc/draft-niyikiza-oauth-attenuating-agent-tokens/)).

**Concrete failure 4 — no attribution, so no dispute and no forensics.** The holder sees a movement they
do not recognise. Section 6 tells them it settled and at what grade; it cannot tell them which capability
authorised it or which means of authorisation stood behind it, so they cannot decide what to revoke, and
an application cannot defend itself against a false claim. Bybit's ~$1.5B loss in February 2025 was
precisely an attribution failure rather than a cryptographic one — the signers could not establish what
they were actually authorising ([NCC Group analysis](https://www.nccgroup.com/research/in-depth-technical-analysis-of-the-bybit-hack/)).
WebAuthn keeps per-credential attestation so a later assertion can be tied to a specific enrolled
authenticator ([MDN, AuthenticatorAttestationResponse](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse)),
and agent platforms are converging on binding every signature to a durable agent identity plus a
timestamped activity log keyed on signer and counterparty
([Turnkey on agent identity](https://www.turnkey.com/blog/agent-identity-erc-8004-siw),
[Coinbase agentic wallets](https://www.coinbase.com/developer-platform/discover/launches/agentic-wallets)).

### G3. There is no way to prove an account is yours to someone who asks — so ordinary inbound value gets blocked.

**The gap.** Section 12 refuses `signHash` and section 7 refuses to sign what the holder cannot see.
Both are right. But together they leave no path at all to produce a legible statement of control, and
third parties cannot compute one for the holder.

**Concrete failure — the withdrawal that never arrives.** The holder wants to move value from a
custodial venue into the kernel. They paste the account the kernel gave them. The venue must verify that
an unhosted destination is controlled by its own customer — under the FATF travel rule as implemented in
the EU, above a €1,000 threshold verification of ownership of a self-hosted address is required
([Sumsub on the travel rule](https://sumsub.com/blog/what-is-the-fatf-travel-rule/),
[Crypto.com travel-rule FAQ](https://help.crypto.com/en/articles/6306139-travel-rule-faq-for-crypto-deposits-and-withdrawals)).
The venue asks for a signed message or a small test transfer. The kernel cannot produce either, so the
withdrawal is held or rejected and the holder's first interaction with the kernel is that their money
will not move. This is the failure mode AOPP was built to automate — and its history is also the reason
the proof must be explicit and consented to rather than a silent capability: Trezor shipped it and then
removed it after a backlash that it legitimised mandated address checks
([Trezor's decision on AOPP](https://blog.trezor.io/a-decision-on-aopp-789540c2930b),
[Notabene on AOPP's limits](https://notabene.id/post/aopp-constraints-and-limitations)).

**Second failure — the holder cannot sign in anywhere.** Application authentication in practice is a
structured statement of control that binds the requesting origin, so that a site cannot get a user to
prove something to a different site
([EIP-4361](https://eips.ethereum.org/EIPS/eip-4361), and the wallet-side domain-mismatch warning
[MetaMask implements](https://docs.metamask.io/wallet/how-to/sign-data/siwe/)). Generalised beyond one
ecosystem by [CAIP-122](https://standards.chainagnostic.org/CAIPs/caip-122), and extended to carry the
scope actually consented to by [EIP-5573 ReCap](https://eips.ethereum.org/EIPS/eip-5573). Note also that
a verifier cannot fall back to checking a signature itself when the account is not a plain key or does
not yet exist on that network — the reason ERC-6492 wraps a verification with deployment data
([ERC-6492](https://eips.ethereum.org/EIPS/eip-6492)). So the proof has to come *from* the kernel; no
counterparty can manufacture it.

### G4. Compromise has no single stop. Revocation is per-application, and the holder must win a race.

**The gap.** Section 3's revocation is per-application and forward-looking, which is correct and
honestly stated. There is no identity-level equivalent: no one action that stops all authority
everywhere while the holder works out what happened.

**Concrete failure.** At 03:00 the holder sees a movement they did not authorise. They do not know
which of their nine capabilities is the source. Today the only path is to revoke them one at a time
while whatever is compromised keeps creating intents inside its caps. Two properties of real incidents
make the race real: a hostile delegation can be re-triggered repeatedly without further victim
interaction, which is why the sweeper pattern following EIP-7702 was so effective — published analysis
reports the overwhelming majority of observed delegations pointing at sweeper contracts, affecting
hundreds of thousands of accounts ([arXiv 2512.12174](https://arxiv.org/abs/2512.12174), and
[ethereum.org's 7702 guidance](https://ethereum.org/roadmap/pectra/7702/) on the related chain-id-0
replay hazard); and the compromise is often of an *intermediary* rather than a key, where no individual
grant looks wrong — the Ledger Connect Kit supply-chain compromise in December 2023 drained roughly
$484K–600K through dApps that had no way to declare a build untrusted immediately
([Ledger incident report](https://www.ledger.com/blog/security-incident-report),
[CoinDesk](https://www.coindesk.com/business/2023/12/14/ledger-exploit-drained-484k-upended-defi-former-staffer-linked-to-malicious-code)).
Custody platforms treat the single stop as a first-class operation: an operator can "instantly lock a
wallet or block agent signing, overriding existing policies"
([Turnkey delegated access](https://docs.turnkey.com/features/policies/delegated-access/agentic-wallets)).

### G5. "Your receiving address already exists and is already yours" claims more than a ledger will honour.

**The gap.** Derived existence is not ledger existence. Section 2 promises the account exists and can be
given out; on many networks an account cannot hold or use value until something is created or funded,
and sometimes the cost falls on the *sender*.

**Concrete failure.** The holder hands out their account. Depending on the network, the inbound transfer
fails outright, or lands but is not spendable, or charges the sender an activation fee they never agreed
to. Documented forms: an account "does not exist until it holds lamports" and must carry the
rent-exempt minimum ([QuickNode on rent](https://www.quicknode.com/guides/solana-development/getting-started/understanding-rent-on-solana));
a base reserve that is locked and unspendable for the life of the account, plus an owner reserve per
additional ledger object ([XRPL reserves](https://xrpl.org/docs/concepts/accounts/reserves),
[XRPL reserve reduction](https://xrpl.org/blog/2024/lower-reserves-are-in-effect)); a minimum balance
and an explicit create-account operation from an already-funded account
([Stellar create-account](https://developers.stellar.org/docs/build/guides/transactions/create-account));
an account number that only comes into being on first funding
([cosmos-sdk #17204](https://github.com/cosmos/cosmos-sdk/issues/17204)); and a per-transfer account
activation fee charged to the sender when the destination is new
([TRON account docs](https://developers.tron.network/docs/account)). Section 4 covers value that
arrives and cannot be routed; it does not cover value that never arrives, or an account that is "yours"
and unusable. Note the research finding that chain-abstraction products generally paper over this
procedurally (a solver funds the reserve on first use) rather than exposing readiness as a state — which
works until the first interaction is an *inbound* payment nobody routed.

### G6. There is no exit. If a network leaves the routable set, or the deployment stops, the holder has no key and no path.

**The gap.** Section 2 says adding a network gives the holder accounts on it and requires nothing from
them. Nothing says what removing one does, and section 18 makes admission status a live property that
can change. Since the holder never holds a per-chain key by design, the kernel is the only route to
their value.

**Concrete failure.** An adapter is withdrawn — or the deployment is wound down — while the holder has
value on that network. There is no key, no export, and no counterparty who can be asked. The whole
embedded-wallet industry treats this as the decisive question and answers it with an explicit escape
hatch: Privy exposes private-key export specifically "to use their embedded wallet address with another
wallet client", assembled off-origin so neither Privy nor the developer sees it
([Privy export](https://docs.privy.io/wallets/wallets/export)); Turnkey ships exportable wallets under
policy ([Turnkey export](https://docs.turnkey.com/wallets/export-wallets)). The kernel need not copy the
mechanism — but it cannot be silent on the outcome.

### G7. One identity used everywhere is one identity observed everywhere, and section 2 never says so.

**The gap.** Section 16 states the truth plainly — derived addresses are computable by anyone who knows
the account and the path, and the interface makes no privacy claim. Part I never repeats it, and the
"one identity, every chain, already yours" framing invites the opposite inference. There is also no way
to hold anything apart: no compartment, no per-application account, so every application that learns one
account learns the holder's whole position.

**Concrete failures.** (a) The holder pays a counterparty once; the counterparty derives the rest and
watches everything they hold and do. Address reuse is the highest-confidence clustering heuristic in
practice, and one identified address de-anonymises a whole cluster — which is why fresh-address-per-payment
has been standard practice since BIP-32/44 ([Coldcard on address reuse](https://coldcard.com/learn/transaction-security/bitcoin-address-reuse)),
and why stealth-address work exists at all ([ERC-5564](https://eips.ethereum.org/EIPS/eip-5564),
[ERC-6538](https://eips.ethereum.org/EIPS/eip-6538)). (b) A known account is copied and front-run; whale
tracking and copy-trading tools exist to do exactly this
([Chainalysis on address-based tracking and poisoning](https://www.chainalysis.com/blog/address-poisoning-scam/)).
(c) Blast radius: one compromised application integration reaches everything, which is the stated reason
production systems hand out per-application accounts — Coinbase sub-accounts are scoped to the
application's domain and "cannot be used on other applications on separate domains"
([Base sub-account reference](https://docs.base.org/identity/smart-wallet/technical-reference/sub-account-reference)),
standardised as `wallet_addSubAccount` ([ERC-7895](https://eip.tools/eip/7895)); venue sub-accounts are
sold as "a compromised API key only affects funds in that sub-account"
([OKX sub-account isolation](https://supa.is/article/okx-sub-account-api-key-permissions-isolate-bot-risk-safely-2026));
and agent wallets hold no funds and structurally cannot withdraw
([Hyperliquid API wallets](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets)).

### G8. Eligibility is invisible to planning, so an outcome can be refused for who the holder is after the quote is accepted.

**The gap.** `capabilities(outcome)` answers reachability, grade, time and cost. It cannot answer "and is
this holder permitted", because the identity carries no property a venue can check and section 2 has no
notion of a claim about the holder.

**Concrete failure.** The application quotes an outcome, the holder signs, and the leg is refused at the
venue because the holder lacks a required standing — or worse, is admitted and cannot exit later because
the standing lapsed. Permissioned venues enforce this in the transfer path itself: every call runs a
verification of the receiver's claims against a trusted-issuer registry and reverts otherwise
([ERC-3643](https://eips.ethereum.org/EIPS/eip-3643)), and live permissioned markets gate per asset
class rather than per protocol ([Fireblocks on permissioned DeFi](https://www.fireblocks.com/blog/permissioned-defi-goes-live-with-aave-arc-fireblocks)).
The claim must also be checked *live*, not once: revocation and expiry are the whole point of status
lists ([W3C status list](https://www.w3.org/TR/2023/WD-vc-status-list-20230427)) and of revocable
attestations ([EAS](https://docs.attest.org/docs/tutorials/create-a-schema)). Without it the kernel
violates its own rule that a route that cannot satisfy the intent is refused before value moves.

---

## ADDITIONS

Eight paragraphs. Each is chain-free, scheme-free and stated as the holder's guarantee. Together they
add one new record (the means of authorisation), one new read (standing capabilities), and three new
calls (`proveControl`, `standDown`, plus readiness on the existing `accountFor`).

### A1. Identity continuity (answers G1)

> **Your identity outlives whatever authorises it.** More than one means of authorisation may stand for
> your identity at once. You may add one and retire one, and doing either changes nothing else: not the
> identity, not the accounts it holds, not where value already sent to you will land. An account that
> was ever yours stays yours and stays able to receive. The kernel never moves you onto a new set of
> accounts, and never asks a counterparty to be told a new one because something you authorise with
> changed.
>
> Retiring the last remaining means of authorisation is refused. The kernel will not hold an identity
> that one lost device ends.
>
> Restoring authorisation through a party you nominated in advance takes effect only after a delay the
> identity states, and only if no standing means of authorisation refuses it within that delay. The
> kernel tells you a restoration has been requested by every means it has, at the moment it is
> requested, and a restoration that completes never reaches backwards: it does not adopt work
> authorised before it, and it is reported as a restoration rather than as your own act.

*Failure without it:* one lost device is permanent total loss (G1 failure 1); rotation silently
redirects every inbound payment the holder has arranged (G1 failure 2); a nominated party becomes an
instant takeover (G1 failure 3).

*Record:* the identity's means of authorisation are enumerable, each with when it was added, when it was
last used, and whether it is standing or retired. Retirement is not deletion; it stays readable so past
authority can still be explained.

### A2. Delegation is finite, listable, attenuating and attributed (answers G2)

> **Every capability ends by itself.** A capability carries the moment it stops, and at that moment it
> stops without the holder, the application or the kernel doing anything. A capability that has ended
> cannot be extended or renewed; continuing requires a new grant. Nothing the kernel issues to an
> application is open-ended, because an authority that can create new work indefinitely is standing
> authority whatever it is called.
>
> **You can see everything that stands for you.** At any time you can list every capability that can
> act for your identity: what each may do, what it may spend, what is left of that, what it has already
> spent, and when it ends. That list is the authority the kernel enforces, not a record an application
> keeps, and no application can present a different one.
>
> **Authority never widens when it is passed on.** Where a capability may be exercised by someone other
> than the party you granted it to, what they receive is narrower than what was granted and ends no
> later. There is no path by which authority delegated onward exceeds, outlives or escapes the terms you
> set.
>
> **Every outcome names the authority that produced it.** An outcome carries the capability it was
> authorised under and the means of authorisation that stood behind it. That naming is part of the
> outcome, not an application's log, so you can always say which authority moved what, and an
> application can always show which one it used.

*Failure without it:* the forgotten grant drained months later (G2 failure 1); a holder who cannot
revoke what they cannot enumerate (G2 failure 2); the confused deputy (G2 failure 3); an unrecognised
movement nobody can attribute, so neither dispute nor targeted revocation is possible (G2 failure 4).

*Calls:* add a read to the Identity group — `delegations` (list) and `delegation(id)` — returning the
four fields named above. `revokeDelegation` keeps the section 3 forward-looking semantics unchanged.

### A3. Proof of control, on request, addressed and inert (answers G3)

> **You can prove an account is yours without giving anything away.** For any account your identity
> holds, you can obtain a proof that it is yours, addressed to a recipient you name and valid for a
> period you name. The proof carries no authority to move value, cannot be used by a recipient it does
> not name, and cannot be used after its period ends. You are shown the statement being proved, in the
> same terms as anything else you authorise, before it is issued; nothing is ever proved on your behalf
> that you were not shown, and no application can obtain such a proof without you.

*Failure without it:* the holder cannot satisfy a counterparty that must verify the destination is
theirs, so inbound value is held or refused and the holder's money will not move (G3). Also: no
application sign-in, and no way to answer a counterparty's ownership question.

*Call:* `proveControl(account, recipient, validity)` in the Identity group. Note the deliberate
narrowness — this is not a general signing path and does not weaken section 7's refusal to sign what the
holder cannot see; it is the one statement whose whole content is "this account is mine", shown before
it is made.

### A4. One stop for everything (answers G4)

> **One action stops everything.** You can withdraw all authority from your identity in a single act,
> without naming what you are withdrawing it from. It takes effect at once, for every capability, on
> every network. While your identity stands down, no new capability can be created and no new intent is
> accepted under it; receiving is unaffected, and what you already hold stays yours. Standing down is
> forward-looking in exactly the sense section 3 gives: it stops authority being used again, and the
> kernel never reports that it reached work already dispatched.

*Failure without it:* during a compromise the holder must revoke application by application while the
compromised party keeps creating intents inside its caps, and if the compromise is of an intermediary
rather than a key, no individual grant looks wrong (G4).

*Call:* `standDown` and `resume` in the Identity group. Resuming is an act of the identity, and per A2 it
is attributed.

### A5. An account says whether it can receive (answers G5)

> **An account is either ready to receive or says what is missing.** Every account the kernel reports
> for you says whether value can reach it now. Where a network requires something before an account can
> hold or use value, meeting that requirement is the kernel's work and not yours to know, and the kernel
> does not describe an account as ready before it is. Where the requirement falls on whoever sends to
> the account, the kernel states it, in your settlement asset, before you hand the account out.

*Failure without it:* the holder gives out an account and the inbound transfer fails, or arrives
unusable, or silently charges the sender (G5).

*Call:* `accountFor` returns readiness alongside the account. No new call.

### A6. The kernel says what it does not hide (answers G7, first half)

> **The kernel makes no secrecy claim for your accounts.** What the kernel can compute, anyone who knows
> what it knows can compute. An account you disclose can be used to find the others, and activity under
> one identity can be connected by anyone watching. The kernel states this rather than letting one
> identity everywhere be mistaken for one identity nobody can follow.

*Failure without it:* the holder pays one counterparty and unknowingly exposes their whole position
across every network (G7 failure a, b). Stating it is also required by *honest outcomes*: section 16
already concedes there is no privacy claim, and Part I currently implies the opposite by omission.

### A7. Compartments (answers G7, second half)

> **You can keep things apart.** One identity may hold separate compartments, each with its own
> accounts, its own spending policy and its own receiving preference. A capability reaches exactly one
> compartment and can never reach another. A compartment bounds authority, exposure and accounting; it
> does not bound observability, and the kernel never suggests otherwise.

*Failure without it:* one compromised or misbehaving application reaches everything the holder has, which
is the failure per-application accounts and venue sub-accounts exist to prevent (G7 failure c); and a
holder who runs unrelated activities under one identity cannot separate their budgets or their exposure.
This does not weaken *one identity*: there is still one identity, one set of means to authorise it, and
no per-chain key or per-chain connection anywhere.

### A8. Standing as a condition of reachability (answers G8)

> **Where an outcome requires something about you, that is part of whether it is reachable.** Some
> outcomes are open only to a holder with a standing that some party verifies. The kernel treats such a
> standing exactly as it treats a cap, a deadline or a grade: a condition tested before anything moves.
> An outcome you are not eligible for is reported as unreachable, with the missing condition named,
> rather than quoted and then refused. A standing your identity presents carries its own expiry and can
> be withdrawn by whoever granted it; the kernel checks it when it is used rather than when it was
> obtained, and refuses to present one that has lapsed or been withdrawn.

*Failure without it:* the holder signs an intent whose route will be refused for who they are, and
discovers it midway — the exact outcome section 5 promises never to allow; or they are admitted somewhere
on a standing that lapsed and cannot exit (G8).

*Call:* no new call. `capabilities(outcome)` reports the missing condition; the standing is a readable
property of the identity.

---

## REFUSE AND SAY SO

Sentences to add to section 7, in its existing voice.

- **It will not hold an identity that one lost device ends.** It refuses to retire your last means of
  authorisation, and it will not let a restoration of authorisation take effect without a delay in which
  you can refuse it.
- **It will not move you onto new accounts.** Changing what you authorise with never changes where value
  sent to you arrives, and the kernel never asks a counterparty to be told a new account because
  something you hold changed.
- **It will not issue an open-ended capability**, and will not extend or renew one that has ended.
- **It will not let authority widen as it is passed on**, and will not report an outcome without naming
  the authority that produced it.
- **It will not prove anything about you that you were not shown**, and a proof that an account is yours
  carries no authority to move value, names its recipient, and expires.
- **It will not describe an account as ready to receive before it is**, and will not let a cost that
  falls on whoever pays you go unstated.
- **It will not claim that the accounts it derives for you are unlinkable**, or that a compartment hides
  you from anyone watching a ledger. Compartments bound authority, not observability.
- **It will not leave you without a route to your own accounts.** Withdrawing support for a network
  removes a route the kernel offers, never your ability to reach what you hold.
- **It will not present a standing it cannot show is still valid**, and will not quote an outcome you are
  not eligible to reach.

---

## DO NOT ADD

Each of these is tempting, and each fails the test that a named behaviour requires it and a named
failure follows from its absence.

- **A per-chain key, seed phrase or per-chain export, even as an advanced option.** It reintroduces
  exactly what *one identity* removed, and it turns the exit guarantee (A1/G6 is answered at the
  identity level) into a per-chain user obligation. The exit must be one act over the whole identity,
  and the *mechanism* belongs in Part II, not a per-chain key in the holder's hands.
- **A privacy or unlinkability guarantee.** The derivation is publicly computable by design (section 16),
  so a privacy claim in Part I would be a false claim, which *honest outcomes* forbids. Stealth-address
  style unlinkability is a different mechanism with its own strength, and if it is ever added it must
  arrive as a graded property, not as a promise attached to identity. A6 is the honest form of this.
- **A human-readable name for the identity, or a naming service.** The address-poisoning harm names are
  supposed to fix is a *sender-side display* problem, and this kernel has already removed the step where
  a holder pastes a destination: section 4's receiving preference and A5's readiness cover the holder's
  side. No guarantee in section 1 needs a name, and adding a global name worsens G7.
- **A public registry, credential document or discoverable identity record.** Nothing in the four
  guarantees requires the identity to be findable by third parties, and publishing one makes linkability
  strictly worse. A3 gives a counterparty the one thing it actually needs, addressed only to it.
- **Guardian counts, thresholds, quorum shapes or delay values in Part I.** A1 fixes the *properties*
  (more than one means, delay, veto, notification, no backward reach). The numbers have no source or
  measurement yet and belong in section 20 as open parameters, exactly as the document already handles
  every other unfixed number.
- **Per-application signers, session keys or any authority an application holds itself.** Section 5
  already forbids applications signing payloads, and A2 covers the legitimate need (scope, budget,
  expiry, attribution) without ever putting a signer in an application's custody.
- **A reputation score, activity history or profile attached to the identity.** It authorises nothing.
  A8 covers the only case where a property of the holder changes what can happen, and it does so as a
  reachability condition rather than as a stored judgement.
- **Shared or multi-party identities with internal governance (a treasury with its own quorum).** This
  requires a policy language inside Part I and a second notion of "who" alongside the holder. It is a
  distinct product surface; A7's compartments cover the separation need without it.
- **Freezing or clawing back incoming value.** The kernel bounds what leaves; it has no business
  refusing what arrives, and A4 is deliberately limited to authority so that standing down never traps
  the holder's own receipts.
- **Mandatory identification of the holder, or screening as a kernel default.** A8 makes standing a
  condition only where an outcome actually requires it. Making it universal would impose a cost on every
  holder for a failure most of them never meet, and no guarantee in section 1 needs it.

---

## SOURCES

Grouped by what each establishes.

**Authenticator loss and the absence of a fallback**
- https://www.slashid.dev/blog/passkeys-deepdive/ — platform-bound WebAuthn keys are non-exportable; no backup artefact exists.
- https://www.corbado.com/blog/device-bound-synced-passkeys/synced-passkeys-cross-platform — passkey sync covers one vendor ecosystem, not across them.
- https://help.coinbase.com/en/wallet/getting-started/smart-wallet-recovery — a recovery path must be created while the original authenticator is still live; afterwards there is none.
- https://web3auth.io/docs/sdk/web/web3auth/multi-factor-authentication — 2-of-3 share model; losing two shares is permanent loss.
- https://fidoalliance.org/wp-content/uploads/2020/06/FIDO_White_Paper_Multiple_Authenticators_CDWG.pdf — enrolling more than one authenticator is the accepted mitigation, and must not weaken security.
- https://docs.safe.global/advanced/passkeys/passkeys-safe — a passkey as one owner among several; survivable only if the account was configured for it.

**Rotation without changing accounts**
- https://www.fireblocks.com/what-is-mpc/ and https://en.wikipedia.org/wiki/Proactive_secret_sharing — shares are refreshed for the same key, with no address change; the alternative is migrating every address with a window in which the old share still works.

**Recovery needs a delay and a veto, and the ordering matters**
- https://help.safe.global/articles/9622260218-account-recovery-with-saferecoveryhub — 28-day default delay (7/14/56 offered), signers can reject during and after it, because the recoverer bypasses the account threshold.
- https://support.argent.xyz/hc/en-us/articles/360007338877-How-to-recover-my-wallet-with-guardians-onchain-complete-guide — 48-hour recovery delay, 36-hour delay to add a guardian.
- https://vitalik.eth.limo/general/2021/01/11/recovery.html — loss and theft as the two failure modes; guardian quorum acts only after a delay in which the signing key can cancel.
- https://cantina.xyz/blog/smart-wallet-social-recovery-risks — if the recovery delay is shorter than the veto path's timelock, the veto never fires.
- https://eips.ethereum.org/EIPS/eip-7947 — a standardised recovery interface exists, i.e. this is an expected part of an account's surface.

**Expiry must live in the authority, not in a user's diligence**
- https://revoke.cash/exploits — losses against stale non-expiring allowances (~$11M via a LI.FI facet; ~$1.8M via an outdated Dolomite version).
- https://eips.ethereum.org/EIPS/eip-2612 and https://github.com/Uniswap/permit2 — mandatory deadline as the fix.
- https://eips.ethereum.org/EIPS/eip-4337 — validAfter/validUntil returned from validation, enforced by infrastructure rather than the application.
- https://github.com/coinbase/spend-permissions and https://github.com/coinbase/spend-permissions/blob/main/docs/SpendPermissionAccounting.md — start/end plus an on-chain read of the live period's spend-so-far.
- https://docs.zerodev.app/sdk/permissions/intro — scope target, selector, value and expiry together or "you risk granting full account control".
- https://docs.rhinestone.dev/sdk/smart-sessions/overview, https://docs.biconomy.io/new/learn-about-biconomy/nexus, https://accountkit.alchemy.com/using-smart-accounts/session-keys — the same four fields recur in every implementation.
- https://eips.ethereum.org/EIPS/eip-7715 — granting a scoped, time-bounded permission as one request, with revocation.

**Standing authority must be enumerable from the enforcing layer**
- https://eips.ethereum.org/EIPS/eip-7579 (`isModuleInstalled`), https://docs.roles.gnosisguild.org/ (role membership, per-parameter and per-period limits) — liveness and budget are reads against enforcement, not application state.

**Attenuation and the confused deputy**
- https://eips.ethereum.org/EIPS/eip-7710 and https://docs.metamask.io/smart-accounts-kit/concepts/delegation/ — caveats, redelegation that can only narrow.
- https://github.com/ucan-wg/spec/blob/main/README.md and https://www.clever.cloud/blog/engineering/2021/04/12/introduction-to-biscuit/ — attenuation as the defining property of a capability.
- https://www.andromedasecurity.com/blogs/ai-agent-permission-gap and https://datatracker.ietf.org/doc/draft-niyikiza-oauth-attenuating-agent-tokens/ — the principal's ceiling disappears when an agent acts under a broad shared identity.

**Attribution**
- https://www.nccgroup.com/research/in-depth-technical-analysis-of-the-bybit-hack/ — ~$1.5B lost where the signers could not establish what they were authorising.
- https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse — per-credential attestation so an action can be tied to an enrolled authenticator.
- https://www.turnkey.com/blog/agent-identity-erc-8004-siw and https://www.coinbase.com/developer-platform/discover/launches/agentic-wallets — signatures bound to a durable agent identity; activity log keyed on signer and counterparty.
- https://eips.ethereum.org/EIPS/eip-8004 — agent identity registry; note that the identity registration itself is not revocable, which is why attribution must sit with the capability rather than only with the actor.

**Agent and bot delegation failures**
- https://github.com/0xfreysa/agent and https://the-decoder.com/hacker-wins-47000-by-tricking-ai-chatbot-with-smart-prompting/ — an agent talked into sending its whole balance (~13.19 ETH); the constraint has to sit outside the agent's own reasoning.
- https://www.coindesk.com/tech/2022/12/28/anonymous-twitter-user-leaks-alleged-3commas-api-database and https://cointelegraph.com/news/3commas-ceo-confirms-api-key-leak-following-warning-from-cz — ~10,000 leaked exchange keys, >$6M confirmed losses, and revocation only available at the "kill the whole integration" level.
- https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets — agent wallets that hold no funds and structurally cannot withdraw.
- https://docs.turnkey.com/features/policies/delegated-access/agentic-wallets — scoped delegated access enforced inside the signing boundary; operator can instantly lock a wallet.
- https://docs.privy.io/guide/delegated-actions/configuration — per-wallet scoped session signers, revocable by the user at any time.
- https://docs.safe.global/home/ai-overview — delegate keys that can propose but not execute.

**One-stop revocation, and compromise of an intermediary**
- https://arxiv.org/abs/2512.12174 and https://ethereum.org/roadmap/pectra/7702/ — a hostile delegation can be re-triggered without further victim interaction; sweeper prevalence and the chain-id-0 replay hazard.
- https://www.ledger.com/blog/security-incident-report and https://www.coindesk.com/business/2023/12/14/ledger-exploit-drained-484k-upended-defi-former-staffer-linked-to-malicious-code — ~$484K–600K drained through a compromised signing intermediary, with no way to declare a build untrusted.

**Proving control to a third party**
- https://sumsub.com/blog/what-is-the-fatf-travel-rule/ and https://help.crypto.com/en/articles/6306139-travel-rule-faq-for-crypto-deposits-and-withdrawals — verification that an unhosted destination belongs to the customer, above a €1,000 threshold in the EU.
- https://blog.trezor.io/a-decision-on-aopp-789540c2930b and https://notabene.id/post/aopp-constraints-and-limitations — an automated ownership-proof protocol, its narrow adoption, and the backlash that makes explicit consent non-negotiable.
- https://eips.ethereum.org/EIPS/eip-4361, https://docs.metamask.io/wallet/how-to/sign-data/siwe/, https://standards.chainagnostic.org/CAIPs/caip-122, https://eips.ethereum.org/EIPS/eip-5573 — a statement of control bound to the requesting origin, generalised beyond one ecosystem, and extended to carry the scope consented to.
- https://eips.ethereum.org/EIPS/eip-6492 and https://eips.ethereum.org/EIPS/eip-1271 — a verifier cannot check the signature itself when the account is not a plain key or does not exist yet on that network, so the proof must come from the kernel.

**Chain-qualified identifiers (why an account is never a bare string)**
- https://chainagnostic.org/CAIPs/caip-10 — an account identifier is the chain identifier plus the address.
- https://eips.ethereum.org/EIPS/eip-7930 and https://eips.ethereum.org/EIPS/eip-7828 — written because the same address is valid on many networks at once and wallets display it without that context.
- https://github.com/ChainAgnostic/CAIPs/blob/main/CAIPs/caip-25.md and https://github.com/ChainAgnostic/CAIPs/blob/main/CAIPs/caip-27.md — a session names the networks and methods actually authorised, and calls must reference it.

**Derived existence is not ledger existence**
- https://www.quicknode.com/guides/solana-development/getting-started/understanding-rent-on-solana — an account does not exist until it holds a rent-exempt minimum.
- https://xrpl.org/docs/concepts/accounts/reserves and https://xrpl.org/blog/2024/lower-reserves-are-in-effect — a base reserve that is locked and unspendable, plus an owner reserve per ledger object.
- https://developers.stellar.org/docs/build/guides/transactions/create-account — an explicit create operation from an already-funded account, plus a minimum balance.
- https://github.com/cosmos/cosmos-sdk/issues/17204 — the account number comes into being on first funding.
- https://developers.tron.network/docs/account — the sender pays an account-activation fee when the destination is new.
- https://aptos.dev/network/blockchain/accounts and https://docs.sui.io/concepts/object-model — accounts and objects are created transactionally, not passively.

**Exit and portability**
- https://docs.privy.io/wallets/wallets/export — export exists so a holder can leave; assembled off-origin so neither vendor nor developer sees the material.
- https://docs.turnkey.com/wallets/export-wallets — exportable wallets under policy.

**Linkability, isolation and per-application accounts**
- https://coldcard.com/learn/transaction-security/bitcoin-address-reuse — address reuse as the highest-confidence clustering heuristic; fresh address per payment as long-standing practice.
- https://eips.ethereum.org/EIPS/eip-5564 and https://eips.ethereum.org/EIPS/eip-6538 — what it takes to actually break the link, and therefore what a computable derivation cannot claim.
- https://www.chainalysis.com/blog/address-poisoning-scam/ and https://dl.acm.org/doi/10.1145/3658644.3690277 — address-based tracking and the measured scale of address-poisoning losses.
- https://docs.base.org/identity/smart-wallet/technical-reference/sub-account-reference and https://eip.tools/eip/7895 — per-application accounts scoped to a domain, and the wallet-level standard for them.
- https://supa.is/article/okx-sub-account-api-key-permissions-isolate-bot-risk-safely-2026 — sub-accounts sold as blast-radius containment for a compromised key.

**Standing as a gate on authorisation**
- https://eips.ethereum.org/EIPS/eip-3643 — every transfer checks the receiver's claims against a trusted-issuer registry and reverts otherwise.
- https://www.fireblocks.com/blog/permissioned-defi-goes-live-with-aave-arc-fireblocks — live permissioned markets gate admission per asset class.
- https://www.w3.org/TR/2023/WD-vc-status-list-20230427 and https://docs.attest.org/docs/tutorials/create-a-schema — revocation and expiry checked at use, not at issue.

**Chain-abstraction identity comparators (how others name the identity and its derived accounts)**
- https://docs.near.org/chain-abstraction/chain-signatures — one account plus a derivation path yields foreign accounts; adding a network needs no protocol change.
- https://developers.particle.network/intro/universal-accounts and https://docs.onebalance.io/concepts/accounts — one balance abstracted over many networks; neither surfaces a derived-but-not-yet-usable account state to the holder, which is the gap A5 closes.

**Note on figures.** Where a loss figure comes from a vendor blog or a secondary report rather than a
primary post-mortem it is attributed above to that report rather than stated as fact. Two circulating
aggregates (a 2025 address-poisoning total and a 2026 physical-coercion total) were found only in
low-quality secondary sources and are deliberately excluded; the ACM CCS measurement and the single
~$50M incident are cited instead. The session's web-search budget was exhausted before some
confirmations could be re-sourced from primary documents; the excluded figures are the only places that
mattered, and no proposal above depends on them.
