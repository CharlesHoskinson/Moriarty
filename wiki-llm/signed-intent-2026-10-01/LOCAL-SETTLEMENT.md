# Local single-writer settlement consumer

Status: implemented and tested locally by seat S2 (`claude-sonnet-5-5`, high effort). Files: `packages/moriarty-beta/src/atomic.ts`, `packages/moriarty-beta/tests/atomic.test.mjs`. The public consumer is exported from `index.ts`.

This is a local simulation. One in-memory writer commits Core/5 candidates after the native signature check and a local key registry check. It does not authenticate any account, read any ledger, or settle anything on Midnight. It cannot close the ledger-native or public owner-binding gates in `PLAN.md` items 5 and 10.

## What it does

`LocalSettlementStore.settle()` runs the existing pieces in order and commits all effects or none:

1. Closed input check. Four strings only.
2. Scenario text built from **store state**, never from a caller snapshot.
3. G2 `verifyAndPrepare`: real native Rust signature check, source/statement match, then actual Core/5 through the bridge.
4. Local registry check (below), against the store state and round.
5. Successor head derivation and a second Core/5 run on the committed state.
6. Under a mutex: re-read, re-check authority, compare the state digest, re-run Core/5, validate the exact derived head, copy-on-write, publish one new root.

No boolean from the caller or from a host callback marks a check as passed. The signature artifact is JSON text; the native binary path is deployment configuration.

## Interface

```ts
new LocalSettlementStore(initialScenarioText, bindingsText, crypto, options?)
store.snapshot()                                   // frozen, provenance 'local-stipulation'
store.prepareIntent({source, action, signing})     // G2 prepareOwnerIntent on a scenario built from store state
store.settle({source, action, signatureText, expectedStateDigest})
store.revoke({bindingId, fromRound?})              // local operator action
deriveSuccessorHead(domain, preHead, frameSha256, effectsWithoutAdvanceHead)
validateSuccessor(domain, preHead, frameSha256, {effects, candidatePost})
```

- `initialScenarioText` is a `moriarty-local-scenario/1` file with `replay: "unused"`. Its claims are an explicit local stipulation. `post_head` is accepted for file compatibility and ignored.
- `crypto` is `{binaryPath, timeoutMs?}`, validated and copied at construction.
- `options` is `{faultHook?}` only. The hook receives a step name and can only abort by throwing; its return value is ignored. Steps: `balances, allowances, obligations, replay, work, head, receipt, swap`.
- All inputs are closed. Extra fields such as `signatureValid`, `authorityValid`, `ledger_accepted` or `qualification` are refused with `ATOMIC_INPUT_SCHEMA`. Getters, symbols, non-plain prototypes and oversize text are refused too.
- `prepareIntent` owner bytes do not depend on store state. Tests compare statement, frame and signing message across stores with different balances, work, round and head.

### Bindings document

`moriarty-local-authority-bindings/1`, `kind: "local-stipulation"`, closed:

```text
{ledgerDomain:{id,chain,network}, revision:DEC,
 bindings:[{bindingId, domain:{id,chain,network}, account, keyRef,
            scheme, publicKeyHex, validFromRound, validUntilRound, revokedFromRound|null}]}
```

`ledgerDomain.id` must equal the scenario domain. Binding ids and `(domain, account, keyRef)` are unique; at most 64 bindings.

Resolution order and codes: `AUTH_DOMAIN_MISMATCH` (statement domain differs from the store, or the only binding for that account/keyRef is for another chain/network/id), `AUTH_NO_BINDING`, `AUTH_KEY_MISMATCH` (scheme or key), `AUTH_REVOKED` (`round >= revokedFromRound`), `AUTH_NOT_YET_VALID`, `AUTH_EXPIRED`. The round is the store state's round.

### Results

Every result carries `qualification: "local-stipulation"`, `ledger_accepted: false`, `authority_valid: null`, the four Core `requiredPremises` and the four `unverifiedBindings`. Nothing is named Accepted, Authenticated or LedgerCommitted. Statuses:

- `LocalCommitted`: adds `localAuthorityMatched: true` (the explicit local registry matched, nothing external), `bindingId`, `replayKey`, `frameSha256`, derived `head`, pre/post state digest, `effects`, the new `snapshot`, and the list of local checks performed. Listing those checks does not close any Core premise.
- `AlreadyCommitted`: exact repeat (same replay tuple and same frame digest). No write.
- `Rejected`: `layer` (`input|source|verification|signature|authority|core|atomic`), `code`, `signature_valid` (`true`, `false`, or `null` when the verdict is unknown, e.g. transport failure), `stateChanged: false`.
- `revoke` returns `LocalBindingRevoked` or `Rejected`.

## State, digest and head

Store root: ledger domain, Core/5 state, predecessor head, receipts, registry (revision and bindings). The state digest is `SHA256("moriarty-local-settlement-state/1\0" || u32BE(len) || canonicalJson(root))`, so a revocation or any cell change moves it.

Successor head: `SHA256("moriarty-local-successor/1\0" || u32BE(len) || canonicalJson({domain, effects, frameSha256, preHead}))`, where `effects` are the financial effects without `AdvanceHead`. The consumer runs Core/5 once with a placeholder successor to get those effects, derives the head, runs Core/5 again with it, and `validateSuccessor` requires exactly one trailing `AdvanceHead` from `preHead` to the derived head, equal to the post-state head. The caller cannot supply a post-state or head.

Receipts map `["domain","signer","nonce"]` to `{frameSha256, head, effectsSha256, bindingId}`. A repeat is `AlreadyCommitted` only when the frame digest matches and the native check passes. A different statement with the same nonce never consumes again; it gets the Core code (`S0_HISTORY_STALE` or `S0_HISTORY_REPLAY`) or `ATOMIC_STALE_STATE` in a race.

## Concurrency and faults

Native verification runs outside the mutex. The commit section is synchronous and runs inside a promise-chain mutex shared with `revoke`. It re-reads the root, re-resolves authority on the current registry, then compares the digest to the one read at prepare time. Precedence at commit: repeat check, authority, digest, Core. A revocation that lands between prepare and commit therefore reports `AUTH_REVOKED`; any other change reports `ATOMIC_STALE_STATE`. A stale `expectedStateDigest` is refused before any native process is spawned, unless the replay tuple already has a receipt (so an acknowledgement-loss retry still works). At most 32 settlements may be in flight (`ATOMIC_BUSY`); at most 4096 receipts (`ATOMIC_CAPACITY`).

Commit builds new values for balances, allowances, obligations, replay set, work, head and receipt, then assigns the new root once. Any throw before that discards the draft (`ATOMIC_COMMIT_FAULT`). Snapshots are deep-frozen and never alias mutable state.

## Tests

`tests/atomic.test.mjs`, 68 tests, real pinned native binary (`MORIARTY_CRYPTO_BINARY`, portable default `experiments/midnight-crypto/target/debug/moriarty-midnight-crypto` from the repository root after `cargo build`). Keys are ephemeral ECDSA keys held in memory. A fake binary is used only for transport negatives. Expected values for the five fixtures are hand-computed in the test file (fee transfer, zero-fee transfer, repay 500, 3000, 101000), then cross-checked against Core/5 called directly. R1-R16 from `plans/S2.md` are covered, with the differences below.

Mutation check (ten deliberate breakages of authority re-check, digest compare, frame compare, head derivation, write order, domain, revocation, registry digest, early stale refusal, key compare): every one made at least one test fail. Three first-round survivors (commit-time frame compare, cross-network binding, early stale refusal) led to added tests.

## Differences from `plans/S2.md`

- R1: two concurrent deliveries of the *same* frame give one `LocalCommitted` and one `AlreadyCommitted`, not `ATOMIC_STALE_STATE`. The plan's `ATOMIC_STALE_STATE` still applies to different statements (R2, R2b).
- R7: the plan lists `AUTH_REVOKED` for a revocation seen at accept. Because the registry revision is in the digest, a plain digest check would report stale; the commit re-checks authority first so the code stays `AUTH_REVOKED`.
- R15/R16 cannot be reached end to end because the caller supplies no candidate. They are unit tests of `validateSuccessor` / `deriveSuccessorHead`, the same functions `settle` calls. The settle path itself compares re-prepared effects and post-state (`ATOMIC_CANDIDATE_MISMATCH`), which only a nondeterministic Core could trigger.
- The registry names the ledger domain explicitly. The scenario carries only the domain id, so chain and network need an explicit local source.

## Limits

- In memory, one process, no persistence or crash recovery. "Crash" cases test discarding an uncommitted draft only.
- The registry and the state are local stipulations. Nothing proves that an account is controlled by a key; `authority_valid` stays `null`.
- `revoke` has no authenticated revoker. Rotation and registration do not exist after construction.
- Core/5 stage shape only: 2 or 3 ordered balances, one allowance, at most one obligation. After a repay brings the obligation to `Settled`, S0 refuses further stages (`S0_STAGE_UNSUPPORTED`); `snapshot()` and `prepareIntent` still work.
- Success-only. Fees on failure, refunds and partial outcomes are outside Core/5, and local rollback is not native transaction rollback.
- Coupled to the shape G2's `verifyAndPrepare` returns (`signature.statement`, `signature.frame_sha256`, `local`, `status`). A change there breaks these tests rather than silently weakening a check.
- Only ECDSA was exercised with real signatures (Node has no BIP340). Schnorr appears only as a key-mismatch case.
- No Rust, proof, wallet, network or Preview path was touched.

## Open decision, not made here

The registry is the S2 plan's explicit local binding. The alternative is a self-certifying account (account id includes a key hash, as `UserAddress` does in the pinned ledger). It would remove the registry but changes beta account identity and cannot apply to existing wallet identities. It needs its own recorded decision before any Preview claim. The signature-route choice (in-circuit, ledger intent signature, or off-chain with on-chain capability) is also open.
