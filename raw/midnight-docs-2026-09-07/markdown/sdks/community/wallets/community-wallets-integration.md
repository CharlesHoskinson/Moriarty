> For the complete documentation index, see [llms.txt](/llms.txt)

# Integrate a wallet

Connect a community wallet in a browser DApp through the DApp Connector API (CAIP-372). This page covers discovery, connect, ZK proving, fees, and portable React code. To choose a wallet first, see the [overview](/sdks/community/wallets/community-wallets-overview.md). For headless or agent flows with no extension, see the [CLI and MCP workflow](/sdks/community/wallets/community-wallets-cli-mcp.md).

| Item               | Detail                                                                                                                                                                                                          |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| DApp Connector API | `@midnight-ntwrk/dapp-connector-api` v4.0.1                                                                                                                                                                     |
| Local proof server | `http://localhost:6300` (Lace and local networks); CLI connector on `ws://localhost:9932`                                                                                                                       |
| Networks           | `mainnet`, `preview`, `preprod`, `undeployed` (local). Only `'mainnet'` is connector-standard; other ids are wallet-defined (see [Connect and the ConnectedAPI surface](#connect-and-the-connectedapi-surface)) |

## The DApp Connector API (CAIP-372)[​](#the-dapp-connector-api-caip-372 "Direct link to The DApp Connector API (CAIP-372)")

A Midnight browser wallet injects an Initial API under the global `window.midnight`, keyed by an identifier. The shape is compatible with the draft [CAIP-372](https://github.com/ChainAgnostic/CAIPs/pull/372/files) (Chain Agnostic Improvement Proposal), so the same DApp code works across wallets. ([npm](https://www.npmjs.com/package/@midnight-ntwrk/dapp-connector-api), [connector repo and spec](https://github.com/midnightntwrk/midnight-dapp-connector-api), [API reference](https://docs.midnight.network/api-reference/dapp-connector))

```
type InitialAPI = {

  rdns: string;        // reverse-DNS id, stable per product

  name: string;        // display name (sanitize before rendering, XSS)

  icon: string;        // URL or data: URL (render via <img>, not innerHTML)

  apiVersion: string;  // version of @midnight-ntwrk/dapp-connector-api implemented

  connect: (networkId: string) => Promise<ConnectedAPI>;

};
```

Support both discovery paths. Wallets such as Lace and 1AM inject under a fixed, friendly key (`window.midnight.mnLace`, `window.midnight['1am']`). The v4 specification instead installs each wallet under its own key and exposes a stable `rdns` field. Discover wallets by scanning `Object.values(window.midnight)` and matching on `rdns` or `name`. Before using a wallet, check its `apiVersion` against the range your DApp supports. The spec notes event-based discovery, like Ethereum's EIP-6963, as a possible future addition, but the current mechanism is the shared `window.midnight` object. ([React wallet-connect guide](https://docs.midnight.network/guides/react-wallet-connect))

## Connect and the ConnectedAPI surface[​](#connect-and-the-connectedapi-surface "Direct link to Connect and the ConnectedAPI surface")

`connect(networkId)` prompts you to authorize, then resolves to a ConnectedAPI that spans Midnight's three asset kinds (shielded, unshielded, and DUST). Only `'mainnet'` is standardized by the connector spec; every other network id is wallet-defined and they differ. The CLI connector, for example, expects capitalized `'Preview'`, `'PreProd'`, and `'Undeployed'`, which is why the [CLI snippet](/sdks/community/wallets/community-wallets-cli-mcp.md#develop-and-test-without-a-wallet) uses `networkId: "Preview"`. Check your target wallet's docs for the exact strings.

| Method                                                                    | Purpose                                                     |
| ------------------------------------------------------------------------- | ----------------------------------------------------------- |
| `getShieldedAddresses()` / `getUnshieldedAddress()` / `getDustAddress()`  | Bech32m addresses per asset kind                            |
| `getShieldedBalances()` / `getUnshieldedBalances()`                       | Per-token balances                                          |
| `getDustBalance()`                                                        | `{ balance, cap }`: DUST regenerates toward `cap`           |
| `makeTransfer(outputs, {payFees})` / `makeIntent(inputs, outputs, {...})` | Build a transfer or an unbalanced intent                    |
| `balanceUnsealedTransaction(tx)` / `balanceSealedTransaction(tx)`         | Balance and pay fees for a contract-built transaction       |
| `signData(data, {encoding, keyType})`                                     | Sign arbitrary data with the unshielded key                 |
| `submitTransaction(tx)`                                                   | Broadcast a sealed transaction (the wallet acts as relayer) |
| `getProvingProvider(keyMaterialProvider)`                                 | Delegate ZK proving to the wallet                           |
| `getConfiguration()` / `getConnectionStatus()` / `hintUsage(methodNames)` | Config; status (with networkId); pre-request hints          |

Feature-detect, because coverage varies

The connector is a contract, but not every wallet implements every method. Lace does not implement `getProvingProvider()` or `signData()` (see [Lace](/sdks/community/wallets/community-wallets-overview.md#lace)). Always check before calling, for example `typeof api.getProvingProvider === "function"`.

The #1 connect gotcha: call connect() synchronously in the click handler

Lace opens a real authorization pop-up. The browser silently blocks it if the browser lost transient user activation, for example if you `await`ed something, or used `setTimeout` or an RxJS `interval` first. Page-load auto-reconnect has no user gesture, so poll `window.midnight` until the wallet injects it, then connect. Extensions inject slightly after `DOMContentLoaded`, and users may need to refresh after installing.

## Where ZK proofs come from[​](#where-zk-proofs-come-from "Direct link to Where ZK proofs come from")

Every Midnight transaction needs a ZK proof. Who generates that proof is the main difference between wallets, and it maps onto the custody models:

* **Local proof server (Lace).** Lace requires a local proof server (Settings, then Midnight, then Local, at `http://localhost:6300`). Witness data stays on your machine, but you must run the server. Lace does not expose `getProvingProvider()`, so a DApp cannot delegate proving to it.
* **In-browser WASM (1AM).** 1AM compiles Midnight's prover, a Halo2-based zk-SNARK over the BLS12-381 curve, into a few MB of WASM and proves in the tab. There is no separate process, though proving time still depends on circuit size and the one-time cold-start key load. 1AM implements `getProvingProvider()` and offers a hosted Proof Station.
* **Delegated (`getProvingProvider`).** The v4 connector abstracts proving behind this method; the connector deprecates the old `Configuration.proverServerUri`. Obtain a proving provider from the wallet when it is available, and fall back to a configured proof server for Lace.

## Fees and DUST[​](#fees-and-dust "Direct link to Fees and DUST")

You pay fees in DUST, which a wallet generates by holding NIGHT and regenerates over time. `getDustBalance()` returns `{ balance, cap }`, and `submitTransaction` uses the wallet as a relayer with a `payFees` option. A wallet can therefore sponsor fees, but sponsorship is a capability the connector allows, not a guarantee that any given wallet ships. On a fresh wallet, DUST takes time to generate: about 12 hours on Lace mainnet or testnet, and about 5 minutes on a local network. Overspending surfaces as `BalanceCheckOverspend` (138) (see [Troubleshooting](/sdks/community/wallets/community-wallets-reference.md#troubleshooting-and-error-reference)).

## The portable integration code[​](#the-portable-integration-code "Direct link to The portable integration code")

This works for any standard-connector wallet (Lace, 1AM). The [CLI connector](/sdks/community/wallets/community-wallets-cli-mcp.md#develop-and-test-without-a-wallet) is a no-extension development fallback.

```
import { setNetworkId } from "@midnight-ntwrk/midnight-js-network-id";

import type { InitialAPI, ConnectedAPI } from "@midnight-ntwrk/dapp-connector-api";



// 1. Discover (friendly key first, then the v4 rdns scan)

const wallets = Object.values(window.midnight ?? {})

  .filter((w): w is InitialAPI => !!w?.name && !!w?.apiVersion);



// 2. Connect: MUST be synchronous in the click handler (pop-up blocking)

//    networkId: only 'mainnet' is standard; others are wallet-defined (CLI uses 'Preview' etc.)

async function connect(w: InitialAPI, networkId = "preprod"): Promise<ConnectedAPI> {

  const api = await w.connect(networkId);             // user authorizes here

  const status = await api.getConnectionStatus();

  if (status.status !== "connected") throw new Error("wallet disconnected");

  setNetworkId(status.networkId);                     // align the DApp to the wallet's network

  return api;

}

// To submit: build the tx (api.makeTransfer or your contract), prove (feature-detect

// api.getProvingProvider, else local proof server), then api.submitTransaction(tx).
```

## Where wallets diverge[​](#where-wallets-diverge "Direct link to Where wallets diverge")

Handle these differences between connectors:

* **Proving:** feature-detect `getProvingProvider`. 1AM has it; Lace does not, so fall back to the local proof server and do not hard-code the deprecated `proverServerUri`.
* **`signData`:** not on Lace, so feature-detect before any signature-based authentication.
* **Pop-up:** Lace opens an auth pop-up, so keep `connect()` synchronous. The reconnect path has no gesture, so poll `window.midnight`, then connect.
* **Network:** always reconcile the DApp to `getConnectionStatus().networkId`.
* **DUST:** read `getDustBalance()`, handle both `{ balance: 0n }` and the `cap`, and do not assume sponsored fees.
* **Non-standard tiles:** Ctrl and Gero may not inject a conformant connector, so feature-detect and degrade to Lace or 1AM.

## React: the production pattern[​](#react-the-production-pattern "Direct link to React: the production pattern")

The Edda Labs midnight-starter-template (demo at `counter.nebula.builders`) wires Lace and 1AM through a copy-pasteable wallet widget. Use it rather than re-deriving the logic:

```
import { useWallet } from "@/modules/midnight/wallet-widget/hooks/useWallet";



function ConnectButton() {

  const { connectWallet, disconnect, status, dustBalance } = useWallet();

  if (status?.status === "connected")

    return <button onClick={() => disconnect()}>Connected · DUST {String(dustBalance?.balance)}</button>;

  return <button onClick={() => connectWallet("mnLace", "preprod")}>Connect Lace</button>;

}
```

Key files in the repo, under `frontend-vite-react/src/modules/midnight/`:

* `wallet-widget/api/walletController.ts`: discovery, pop-up-safe connect, proof-server check
* `wallet-widget/hooks/useWallet.ts` and `contexts/wallet.tsx`: state
* `wallet-widget/ui/midnightWallet.tsx`: connect modal
* `counter-sdk/`: wiring a connected wallet into a Compact call

([starter template](https://github.com/eddalabs/midnight-starter-template), [React guide](https://docs.midnight.network/guides/react-wallet-connect))
