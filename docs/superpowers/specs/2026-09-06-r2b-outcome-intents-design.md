# R2b bounded outcome intents

Status: design for the next sprint authorized by the user's “begin the next
sprint”, following the adopted intents report amendment. This specializes that
approved design into a finite atomic local profile. R2 exact-plan mode remains.

## Scope and chosen design

Separate a signed outcome IntentIR from a concrete PlanIR. A user fixes authority,
goals, permitted agreement programs, validity and nonce before a plan is chosen.
A plan chooses one registered agreement action plus at most four explicit fees.
This provides route choice without introducing arbitrary backend calls or a new
Core instruction. Demonstrate two constant-product pools and a loan settlement.
The existing R2 evaluator computes agreement behavior; a separate checker checks
principal authority and complete asset accounting. Both must pass locally.

Alternatives considered: signing the final plan again would retain R2's narrower
meaning; arbitrary transfer plans would fail to authorize counterparties; general
multi-action routing would add intermediate shared-state composition before the
atomic authority boundary works. Use one registered action now. Future atomic
routing and asynchronous residual capabilities extend an explicit versioned profile.

## Canonical contracts

All quantities, nonces, revisions and times are canonical UInt128 decimal strings. Use
R2's bounded canonical JSON codec (64 KiB, depth16), SHA-256 and local Ed25519.
Identifiers are ASCII, 1–64 characters. Collections: authority/goals/fees at most
8 each, recipients/agreement choices at most8, balance cells32, trace effects16,
plan steps exactly1, added fee effects at most4, consumed nonces128. Reject
unknown fields, duplicates, accessors, non-plain values and semantic extensions.
Asset identity is the entire domain/issuer/reference/kind tuple; no ticker aliases.

```ts
import type { Action } from './core.ts';
export type AssetId = {domain:string; issuer:string; reference:string; kind:'token'|'claim'};
export type OutcomeDomain = {network:string; deployment:string};
export type IntentIR = {
  version:'moriarty-intent/1'; domain:OutcomeDomain; principal:string; nonce:string;
  validity:{notBefore:string; expiresAt:string}; lifecycle:'atomic';
  authority:{asset:AssetId; maxDebit:string; recipients:string[]}[];
  goals:{asset:AssetId; account:string; minCredit:string}[];
  fees:{asset:AssetId; maxFee:string}[];
  agreements:{id:string; programHash:string}[];
  requiredClaims:['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance'];
  extensions:[];
};
export type SignedIntent = {kind:'SignedOutcomeIntent'; intent:IntentIR;
  intentHash:string; publicKey:string; signature:string};
export type TrustContext = {domain:OutcomeDomain; principal:string; publicKey:string};
export type AssetEffect = {kind:'Transfer'|'Fee'; asset:AssetId; from:string; to:string; amount:string};
export type Balance = {asset:AssetId; account:string; amount:string};
export type PlanIR = {version:'moriarty-plan/1'; intentHash:string;
  steps:{agreementId:string; programHash:string; predecessorHash:string; action:Action}[];
  fees:AssetEffect[]};
```

The intent hash commits only the intent, not a plan. The signature has its own
outcome-intent domain separator and signs that hash. Verification uses a caller's
independent principal/public-key/domain trust record; an envelope's key cannot
appoint itself authoritative. Validity uses `notBefore <= now < expiresAt`.
Required claim names/order/version are fixed by this local profile; none can be
stripped or changed to optional. Hashes are interface identities, not certificates.

The independent trace checker does not call Core. It validates a complete list
of typed balances and effects, enforces sufficient balance at every effect prefix,
and reproduces every final balance. For the signed principal it sums gross
outgoing transfers AND fees by asset across all recipients; refunds never restore
allowance. Every recipient must be allowed. Fee totals have a separate cap and
still consume gross authority. Goals compare net final-minus-initial credits.
Unsigned debits from other actors require the registered agreement relation in
the outer runtime; this checker alone establishes no counterparty authority.

## Registered agreements and local atomic commit

The local runtime owns cloned R2 package sources/programs, current financial
state, an asset registry and one shared balance table. Two pools share the same
trader balances; reserves belong to different pool instances. Map synthetic IDs
`demo:A`, `demo:B`, `demo:USD6` to distinct structured token identities. Recompute
an agreement state using the current shared balances before hashing its anchor.
A program hash binds an exact elaborated program and is allowlisted by the intent.
Unknown programs, assets, aliases, action shapes and predecessor hashes reject.

Re-evaluate the plan's action with the trusted program and current state; never
trust plan-supplied after-state or effects. Project Core Transfers to structured
asset effects and check every cash delta. Retain DueSettled effects as obligation
updates; for the loan require full existing principal/interest dues to be settled,
paid fields to advance and no due to be labeled paid without its transfer.
Only swap and loan settle actions enter this atomic adapter profile; loan accrual
is prepared as an explicitly local example state, not certified history.

Fees follow the agreement action and may debit only the signed principal. The
independent checker checks the complete trace and resulting global balances.
Preview is read-only. Simulation re-verifies signature, freshness, agreement and
policy against current state. Capture a private generation before asynchronous
work; compare it immediately before the synchronous state+nonce commit. Two
concurrent valid proposals cannot both consume one nonce. Keys are indexed by
network/deployment/principal/nonce, not intent hash, so editing and re-signing an
intent does not reset a consumed nonce. Full nonce capacity rejects, never evicts.
The runtime's logical clock is explicit, UInt128 and monotone.

A receipt binds intentHash, planHash, before/after state commitments, observed
asset effects, due updates, gross usage, net goal credits and status
`simulated-complete`. One-shot unused authority expires on commit; it is not a
new reusable capability. There is no pending success in this atomic profile.
The receipt prominently records local simulation and unavailable mandatory proofs.
Real acceptance always remains unavailable and never mutates the simulation.
Changing examples or reloading creates a new local world and resets nonce history.
The logical clock is a demo input, not trusted UTC. No durable ledger or replay
protection is claimed.

## Developer interface

Add `/intents` beside `/language`: edit outcome JSON, render the canonical signing
summary, sign with an in-memory local key, then select/propose either pool plan.
Show signature preservation across permitted plan changes, independent rejection,
expiry, replay and competing-plan results, receipt and explicit missing-proof
status. Plan edits invalidate checks/receipts but preserve an unchanged signed
intent. Intent edits clear signature immediately. Async completions cannot restore
stale evidence. Export contains public objects and receipts only. Show all safety
fields from the decoded canonical intent, including full asset identities,
recipients, aggregate caps, fees, goals, validity, nonce and allowed program hashes.

## Acceptance and limits

Positive: same signed intent admits two pool proposals and one can simulate;
loan settlement shares the authority checker and clears dues. Negative: aggregate
overspend despite goal success; refund cap evasion; intermediate unauthorized
recipient; net goal below minimum after fees; same ticker in a different domain;
nonce reuse across different intent hashes; before/at expiry; too-early time;
wrong signer/domain; altered program/anchor; stale shared balance; concurrent
nonce use; invalid JSON/unknown semantics; missing proofs never real acceptance.
All rejection paths leave current financial state and nonce usage unchanged.

Freeze ACTUS NAM negative amortization, lending refinance/liability change and
pending request/claim redemption as held-out cases before any Core expansion.
Their source pins and explicit unsupported/disposition results remain visible.
This sprint does not implement their new financial semantics. Full ACTUS277,
DeFi72, contract metatheorems, durable consumption, private composition and native
PCD remain open. No R3 proving campaign runs during R2b.
