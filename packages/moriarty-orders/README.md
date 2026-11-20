# Moriarty funded O1 orders

This local release candidate contains the funded O1 fill and cancellation clients, two signed Midnight transaction histories and a Linux arm64 strict Rust ledger verifier. Replay runs offline on Linux arm64 with glibc, Node.js 22.19 or later and npm. Obtain the archive and its expected SHA-256 from the local delivery record. In a new directory, replace the example archive path with its absolute path and run the following commands.

```sh
# Compare the digest below with release.json before installing.
sha256sum /absolute/path/moriarty-lang-orders-0.1.0-beta.2.tgz
npm install --offline --ignore-scripts --no-audit --no-fund /absolute/path/moriarty-lang-orders-0.1.0-beta.2.tgz
./node_modules/.bin/moriarty-orders replay o1-fill
./node_modules/.bin/moriarty-orders replay cancel-unfilled
```

Each command prints JSON with the ledger address, asset balances and a strict verification result for each transaction. The account order is owner, executor, recipient, custody. Each pair contains A followed by B. The fill delivers 200 B to the recipient and 100 A to the executor. Cancellation returns 100 A to the owner. The second replay verifies the owner's subsequent signed spend of the returned asset. Both histories include a stale fill whose financial effects fail after the order closes. Each application pays native DUST. The application fee is zero.

## Client operations

Import `issueEnrollment`, `signOrder`, `fundOrder`, `prepareFill`, `acceptFill`, `cancelOrder` and `readAcceptedOrder` from `@moriarty-lang/orders`. The generated Compact module is available as `@moriarty-lang/orders/contract`. On a trusted Midnight connection, read the installed contract from the accepted ledger head and pin its network, address, order program hash, the verifier digest of every entry point and its maintenance committee (the owner wallet alone, threshold one). Enroll the application signing key with the owner wallet using `issueEnrollment(ledger.L, ledger.runtime, deployment, head, ownerWallet.key)`. Assign the returned enrollment to `deployment.enrollment`. The account, recipient, executor, assets and terms come from that deployment.

```js
await fundOrder(ledger, instance, deployment, ownerWallet);
const head = readAcceptedOrder(reader, ledger.runtime, instance.contract.generated, deployment);
const grant = {
  terms: head.terms, owner: head.owner, revision: head.revision, action: 0n,
  contract: {bytes: ledger.runtime.encodeContractAddress(instance.address)}
};
const signature = signOrder(ledger.runtime, instance.contract.generated.pureCircuits, grant, ownerScalar);
await acceptFill(ledger, instance, deployment, {
  revision: head.revision, input: 100n, delivery: 200n, fee: 0n, now: relativeBlockSeconds, signature
}, executorWallet);
```

To recover an open funded order, sign the current revision with action `1n` and call `cancelOrder(ledger, instance, deployment, {revision, signature}, feeWallet)`. The ledger sends the remaining A to the enrolled owner in that transaction. State-dependent requests must read the current revision. Once closed, the order rejects further fills and cancellations.

The caller supplies the authenticated ledger, wallet and proving adapter. The reader exposes `network` and `index(address)`. The adapter exposes the Midnight ledger module as `L`, Compact `runtime`, accepted `state`, and `network`. Its `prepareCall(instance, entry, args)`, `assemble(calls)`, `fundAndProve(tx, contract, payer)` and `apply(tx, label)` implement a funded proof and submission. Its `call(instance, entry, args, label, payer)` performs these steps together. A wallet supplies its native address and signing capability. The caller sets the instance address, the generated module and the proof provider for the deployed contract. Build proving assets with Compact 0.36.0 from the included sources when using the client against a new deployment.

```sh
compactc node_modules/@moriarty-lang/orders/source/acceptance.compact ./o1-build
```

## Verification scope

The trust anchor for these replays is the local archive digest. For each contract call the bundled Rust verifier validates the V3 proof under the installed key. Native well-formedness checks verify wallet signatures, DUST spending and fee balance. The bootstrap reward uses the ledger’s proof-free reward envelope. The verifier applies each recorded transaction under its specified local block context and checks the immediate post-state and finalized state digests. It decodes balances with the bundled ledger and generated contract interface. The ledger is a controlled, in-memory deployment with synthetic assets. Each order's one-member maintenance committee is the owner wallet. The adapter refuses a changed committee, entry point or verifier, and any recorded maintenance update. Switching a deployment or verifier requires a fresh owner decision and enrollment. The example clients require separate wallet and proving infrastructure for new transactions. Local replay uses the archive’s public ledger bytes. The bundled native and WASM library attributions are indexed in `notices/index.json`, and npm dependencies retain their own license files. These are maintainer replays. Public release, independent reproduction and live venue participation remain open.
