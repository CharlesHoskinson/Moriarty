# Signed-intent stage: measured result

Status: implementation verified locally; fresh whole-candidate result audits pending.
Base main2905cb6d. This stage does not complete the full signed-finance/Preview goal.

## Implemented consumer

The versioned Rust owner codec builds and verifies exact canonical statements with
actual pinned Midnight BIP340/ECDSA APIs. The beta API and CLI invoke that binary,
regenerate terms from actual source/action, refuse source mismatch, distinguish
checked false from unavailable verification, and prepare the actual Core5 candidate.
Readable ASCII review includes signed values, claimed/computed identities and local
effects. The existing JSON default remains available. CLI never signs private keys.

LocalSettlementStore adds an explicit local registry, copy-on-write CAS, replay
receipts, revocation recheck, deterministic successor head, complete financial
Core5 effects and failure atomicity. This is an in-memory stipulation, not chain
state or account authority. Three packed public examples exercise transfer and
repayment with real signatures and complete independent effect/post expectations.

## Fresh observations

- Beta187tests passed,0failed,0skipped, with MORIARTY_REQUIRE_NATIVE=1 and actual
  separately built Rust verifier/helper. Packed install works outside checkout
  with PATH empty, copied verifier, bundled guide, typed API and negative cases.
- Rust all-features32tests passed,0failed; the new numerical experiment remains
  ignored in the default suite. This suite also reruns the prior tiny k6 native
  equality smoke and privileged local ledger/state tests; they retain their limits.
- The separately approved one-shot k10 numerical transfer test passed real native
  KZG verification,28public mutations and3proof-byte negative controls. It is the
  one new numerical proof attempt; it is not the old k6 regression. See NATIVE-RESULT.
- TypeScript typecheck passes; packed strict declaration client includes the actual
  signed statement, frame and boolean signature fields.

Commands and complete outputs are retained in evidence/final-beta-tests.txt,
evidence/final-rust-tests.txt and evidence/native-run.log. Failure evidence includes
SEC1prefix acceptance, independent oracle prehash mismatch, process timeout,
public declaration inference, readable CLI integration and pack-test assumptions.
The failed pack checks were test mistakes: repayment action is repay_loan; an
all-zero Schnorr signature is malformed, so the checked-false control now changes
one valid scalar hex nibble. No production validator was weakened to satisfy tests.

## Workers and review

Three GPT6.1 Sol and three exact claude-sonnet-5-5 high logical seats supplied
independent plans. Sonnet S1 first implementation exhausted turns, then a focused
finish completed; original failures remain. S2 completed the local atomic scope.
S3 implemented display/tests but hit its weekly cap before the walkthrough was
finished. Root and G2 completed remaining CLI, docs, examples and distribution.
Actual receipts are retained; separate calls do not imply session continuity.
No substitute is reported as Sonnet. Independent final result audits remain owed.

## Product gates remain open

The signed statement is a production codec for this closed experimental protocol,
not a production authorization system. Key/account lifecycle, authenticated state
and head, asset/address identity, source/compiler/native proof correspondence,
full mandatory financial/property/intent/transition/history proofs, atomic native
financial settlement, Preview transfer and repayment finality are unfulfilled.
All required premises and unverified bindings remain. No real wallet, seed or
contract state was changed; no Preview financial transaction was submitted.

The new numerical circuit excludes identities, signatures, hashes, replay and
head. Latest proofs0.8/curves0.3 compatibility with ledger0.7.3/curves0.2.1 is open.
The installed Compact31.1 rejects the proposed secp ECDSA symbol; upgrading or
using a different signature family needs actual compatibility work and proof.
No Accepted, Authenticated or LedgerCommitted product status is manufactured.
