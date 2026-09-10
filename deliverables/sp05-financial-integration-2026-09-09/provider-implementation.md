# SP05 provider implementation — 2026-09-09

Repository observation: added callable financial SDK providers in `/home/charl/Moriarty/.worktrees/sp05-financial-integration/experiments/moriarty-midnight-financial/ledger/providers.mjs` and its adjacent production-path tests. No commits. These source results are not independent approval, financial ledger settlement, native financial proof evidence or SP05 acceptance.

Startup: installed Moriarty develop skill read; checkout AGENTS.md and guarded status inspected. Status reported stale operational-history/admission issues; authorized source repair proceeded without campaign dispatch. Inspected latest R5/R6/R7 rejection and actual retained wallet.ts / facade 4.1.0 / ledger-v8 / Midnight.js 4.1.1 provider sources. Routine approved fixed unshielded loan/swap scope; no new design packet.

## Callable interface

`await createFinancialProviders({ walletContext, networkConfig, zkConfigPath, privateStateConfig, limits, ledger, sdk?, onEvent? })`

- Existing wallet handle is `walletContext.wallet`, with existing unshielded keystore and shielded/DUST key handles. No restore or key derivation occurs.
- Network fields match retained runtime: `indexer`, `indexerWS`, `proofServer`.
- `zkConfigPath` is an absolute proven-asset directory containing the pinned NodeZkConfigProvider keys/zkir layout. This provider does not attest asset provenance or run compilation.
- `privateStateConfig` requires absolute `midnightDbName`, `privateStateStoreName`, `privateStoragePasswordProvider`; optional `signingKeyStoreName`. Other options reject. Account ID derives from the existing wallet's Bech32 address; password callback is forwarded without invocation or output during construction.
- `limits`: absolute epoch-millisecond `deadlineMs`, positive safe-integer `submissions`, nonnegative bigint `dustFee`, and `grossByAsset` object keyed by native lowercase 64-hex token types with nonnegative bigint caps. Caps are copied into internal state; callers cannot mutate counters. Unknown assets reject.
- `ledger` must be the pinned native module used by the SDK; native transaction/intent/offer classes and native marker deserialization are checked. SDK constructor injection supports explicitly inert source tests. `loadFinancialSdk()` imports the four actual SDK constructors after checking recorded versions and direct entry SHA256 pins; it does not attest transitive dependency bytes.
- Returns standard `walletProvider`, `midnightProvider`, `privateStateProvider`, `publicDataProvider`, `zkConfigProvider`, `proofProvider`, plus `cleanup()`, `stop(reason)`, `getState()`, `checkDeadline()`, `execute(label, thunk)`.

`walletProvider.balanceTx` validates input native proof/pre-binding markers and budget shape before wallet effects, invokes actual facade balance with `tokenKindsToBalance: ['unshielded','dust']`, validates the exact UNBOUND_TRANSACTION recipe and its native base/balancing markers, checks all input owners and gross asset caps, explicitly signs, cryptographically verifies every unshielded input signature, reserves persistent per-asset gross/DUST/submission allowances, finalizes, compares complete merged `eraseProofs().serialize()` bytes with a frozen signed snapshot, and requires finalized native proof/binding deserialization. Signature addition itself must preserve signature-erased semantic bytes. Gross inputs cover both offer classes and every recipe part; refunds never reduce reservations. Native DustSpend.vFee sums determine fee reservation; no fixed fee assertion exists.

`midnightProvider.submitTx` accepts only exact issued finalized bytes, once, from this provider instance. Submission uses the actual facade method and reconciles its returned ID with native identifiers. `onEvent` receives public `{kind:'submitted', txId, identifiers, transactionHash}` immediately on observed response, including responses arriving after the outer deadline. Reservations and public identifiers persist on failure/ambiguity; the shared stopped state blocks subsequent provider effects. Concurrent financial operations reject. Events omit exception text, private-state values and witness material.

## Reproduced source checks

Baseline red: `node --test experiments/moriarty-midnight-financial/ledger/providers.test.mjs` failed with missing production `providers.mjs`, exposing absent callable provider. Later explicit flag test failed because funding selected `all`; changing the pinned option to unshielded+DUST produced green.

Final check command: `node --test experiments/moriarty-midnight-financial/ledger/providers.test.mjs` — 16 tests pass. `node --check experiments/moriarty-midnight-financial/ledger/providers.mjs` passes. All wallet/prover/indexer/submission effects in behavioral tests are inert. One constructor-only test imports actual pinned SDK providers without wallet, storage reads/writes or network calls.

Native success fixtures use proof-free `Transaction.prove` with both callbacks throwing on any actual prover request. Native API conversion invoked zero callbacks; no cryptographic proof bodies, contract calls, real proof generation or ledger validation occurs. This conversion yields actual unbound proof markers and preserves semantic bytes. Earlier `.mockProve()` experiment was unsuitable because it changes binding commitment bytes and produces a bound transaction; it is not used by final tests.

Coverage includes actual constructor options/interfaces; native signed base and separate balancing transaction merge; both offer classes and per-asset persistent reservation; full-refund gross charging; exhausted/unknown asset and submission rejection; malformed limits before wallet effects; pre-proof input rejection; unknown recipe/diagnostic Maps; registration exclusion; missing signature rejection; changed fallible effects/extra finalized intent; exact single-use submission; deadline with unresolved finalization and later resolution; ambiguity stop and retained identifiers; shared query/wallet stop and idempotent wallet cleanup.

## Explicit gaps and excluded families

Requires an already funded and DUST-registered wallet. All DUST registrations reject before signing, including unreadable or invalid signatures and fee authorization. Their authorization capacity is therefore never admitted; registration support and accounting remain broader open requirements. Shielded offers, transients, rewards, other recipe kinds and broader financial families reject.

No positive native DustSpend fixture was completed. One bounded synthetic local-state attempt confirmed that passing DustPublicKey to `dustNonce` fails the pinned JS DustSecretKey type assertion; the prior retained review reports the complementary seq-zero limitation. No further probing followed. Nonzero native fee accounting is implemented by native getters but is not empirically verified with a positive DustSpend fixture, proof, or live settlement. No full contract-call proven native fixture or actual wallet finalization was run.

Cleanup calls and bounds `wallet.stop()` at five seconds; the pinned level provider closes databases per operation. The pinned indexer exposes no dispose/cancel API, and in-flight wallet/provider promises cannot be forcibly cancelled by this wrapper. Cleanup therefore reports `containmentComplete:false`, the pending count and `indexerDisposal:'not-exposed-by-pinned-sdk'` even after wallet stop succeeds. Deadlines stop new effects; they do not prove all underlying work terminated. Full R7 resource containment remains open.

The signed recipe is the reservation/finalization authorization boundary; the wrapper does not independently prove the balancer's added effects refine the original contract call. Root's driver, observed receipts, contract/source admission and comparator remain separate requirements. Public endpoints, asset provenance/current admission and real token balances must be verified before dispatch.

No wallet restore, network, registration, compiler, cryptographic proof generation, public submission, credential output, installation, new harness or commit was performed. Independent fresh GPT-6 Astra and Claude Opus exact-candidate reviews remain required.
