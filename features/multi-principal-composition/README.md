# Multi-principal intent composition

**Status: design requirement, added 2026-10-22 by the owner.** Specified only; nothing is implemented.

## Requirement

Moriarty must let the signed intents of several independent principals govern the same object at once. A stage over a governed asset is accepted only if it satisfies the composition of every governing intent whose scope it touches. No principal's protection can be removed except by an exemption that principal signed.

Today an intent has exactly one owner:
- the signed profiles reject any program in which a non-owner pays or owes (`INTENT_ACTOR_SCOPE`);
- MIL/2 decision 4 reserves multi-signer stage arity but admits only `|S| = 1`, and places multi-signer admission post-U4 (`concepts/intent-language/DESIGN-MIL2.md` §14–§16).

This requirement asks for an earlier, specific form of multi-signer admission.

## Consumer

**Exocovenants (Fenrir).** An exocovenant is a covenant enforced on a foreign chain by Fenrir. In the owner's words, it holds "the sum of the intentions ... composable ... they get locked into the exocovenant, and the exocovenant honors all of them" (Fenrir launch meeting, 2026-10-20). The canonical example has five governing principals:

| Principal | Example intent over Monument Bank tokenized pounds |
|---|---|
| User (Bob) | "earn 10% on my deposits; never send to an unregulated address" |
| Issuer (Monument Bank) | "only to holders with a KYC attestation from an approved attester" |
| Regulator (Bank of England, negotiated with the issuer) | "may be lent as collateral for USDC; may not be sold" |
| Venue (Uniswap pool policy, applied by Fenrir) | "only assets on the permitted list" |
| Jurisdiction | "India: withhold 1%"; "prove you are outside the US for yield and looping" |

Bob's **agent** acts within Bob's mandate and never holds foreign keys. Fenrir signs a foreign spend only if the composed exocovenant accepts it.

Without this feature, the exocovenant cannot be written in Moriarty, and the ideal functionality F_exo has no defined policy to enforce.

## Scope

**In scope: layered governance.** Several principals constrain and grant rights over one governed object. Every effect must satisfy all the components whose scope it touches.

**Out of scope here: n-party clearing,** meaning several owners trading with one another inside one stage, as in a batch auction. It shares the arity machinery but needs a matching and pricing semantics. It stays post-U4 as MIL/2 places it. See `CONCEPT.md` §2.

## Files

- [CONCEPT.md](CONCEPT.md): the model (constraint/grant split, `⊗`, satisfaction), algebraic laws, conflict semantics, exit safety, privacy, and interactions with MIL/2.
- [REQUIREMENTS.md](REQUIREMENTS.md): EARS requirements MPC-001 onward, and the proof obligations.

## Owner decisions needed

1. **Milestone.** MIL/2 places multi-signer admission post-U4. Layered governance is a prerequisite for the Fenrir v1 path (requirements due Nov 15, ship Jan 15, 2027). Should it move ahead of n-party clearing, to U2 or U3?
2. **Conflict-check privacy.** May a failed `Compose` reveal *which* components conflict, or only that a conflict exists? See `CONCEPT.md` §7.
3. **Precedence.** This proposal gives roles **no** precedence: a regulator does not outrank a user. Exceptions come only from consent the constrained party signed, plus a signed order on contended cells. Confirm.
