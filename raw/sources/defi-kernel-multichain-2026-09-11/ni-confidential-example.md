> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Confidential Example

> Migrate the AMM Solver to support confidential intents

This guide walks through the changes needed to upgrade the [example AMM Solver](https://github.com/hairy-pointer/near-intents-amm-solver/tree/feat/confidential-amm-solver-mode) from public intents to confidential intents. If you haven't already, read the [public Example Solver](./example) guide first to understand the baseline implementation.

***

## Overview

Migrating to confidential intents requires changes in four areas:

| Area                     | Change                                                      |
| ------------------------ | ----------------------------------------------------------- |
| **Configuration**        | Add private relay URL, contract, salt, and treasury account |
| **Token identifiers**    | Convert public asset IDs to confidential `imt:` format      |
| **Nonce generation**     | Switch from deterministic nonces to versioned nonces        |
| **WebSocket connection** | Connect to the private relay with acknowledgement support   |

The core quoting logic and AMM pricing remain unchanged — the same `token_diff` intent structure works for both public and confidential swaps.

***

## Configuration

### Environment variables

Add the confidential intents configuration to your environment file:

```bash theme={null}
# Solver mode
SOLVER_MODE=confidential

# Private relay connection (contact Defuse team for access)
PRIVATE_RELAY_WS_URL=wss://your-private-relay-url.chaindefuser.com/ws

# Confidential intents contract (used in signed payloads)
PRIVATE_INTENTS_CONTRACT=intents.far
PRIVATE_INTENTS_CONTRACT_SALT=e110f317

# Treasury account for asset wrapping
PRIVATE_TREASURY_ACCOUNT_ID=51e8f94d77b5e90dc9852ca6113771e11e8382ce69473a75e313e41665de7cbe

# 1Click API for balance queries and liquidity management
ONE_CLICK_BASE_URL=https://1click.chaindefuser.com

# Stable identifier for guaranteed delivery
SOLVER_INSTANCE_ID=my-solver-1

# Partner JWT (used for both relay auth and 1Click API)
PARTNER_JWT=your_partner_jwt_here
```

<Info>
  `PRIVATE_RELAY_WS_URL`, `PRIVATE_INTENTS_CONTRACT`, `PRIVATE_INTENTS_CONTRACT_SALT`, `PRIVATE_TREASURY_ACCOUNT_ID`, and `ONE_CLICK_BASE_URL` are production constants shared by all solver operators. Only `SOLVER_INSTANCE_ID` and `PARTNER_JWT` are solver-specific.
</Info>

### Solver mode

Create a mode toggle to switch between public and confidential:

```typescript theme={null}
export enum SolverMode {
  PUBLIC = 'public',
  CONFIDENTIAL = 'confidential',
}

export const solverMode = process.env.SOLVER_MODE === 'confidential' 
  ? SolverMode.CONFIDENTIAL 
  : SolverMode.PUBLIC;

export const isConfidentialMode = solverMode === SolverMode.CONFIDENTIAL;
```

***

## Token identifiers

### Converting public to private asset IDs

Confidential assets wrap the public token ID with the treasury account:

```typescript theme={null}
const privateTreasuryAccountId = process.env.PRIVATE_TREASURY_ACCOUNT_ID;

export function privateAssetIdentifier(assetId: string): string {
  return `imt:${privateTreasuryAccountId}:${assetId}`;
}

// Example:
// Input:  "nep141:wrap.near"
// Output: "imt:51e8f94d...7cbe:nep141:wrap.near"
```

The solver uses these wrapped identifiers when:

* Subscribing to quote requests
* Checking token pair support
* Building `token_diff` intents

<Info>
  For 1Click API balance queries, use the **public** asset identifiers (without the `imt:` prefix). The API strips the prefix from response keys. See the [Balance queries](#balance-queries) section for details.
</Info>

### Token configuration

Update the tokens configuration to conditionally use private identifiers:

```typescript theme={null}
export const publicTokens = [
  `nep141:${process.env.AMM_TOKEN1_ID}`,
  `nep141:${process.env.AMM_TOKEN2_ID}`
];

export const tokens = isConfidentialMode 
  ? publicTokens.map(token => privateAssetIdentifier(token)) 
  : publicTokens;
```

***

## Nonce generation

### Versioned nonces

Confidential intents require nonces that encode the contract salt and deadline. Use `VersionedNonceBuilder` from the intents SDK:

```typescript theme={null}
import { VersionedNonceBuilder } from '@defuse-protocol/intents-sdk';

public generateVersionedNonce(deadline: Date): string {
  const saltHex = process.env.PRIVATE_INTENTS_CONTRACT_SALT;
  const salt = Buffer.from(saltHex, 'hex');
  
  return VersionedNonceBuilder.encodeNonce(salt, deadline);
}
```

<Warning>
  The salt must be exactly 4 bytes. The production salt `e110f317` is a shared constant used by all solvers — do not change it. See the [main confidential intents page](/integration/market-makers/confidential-intents#versioned-nonces) for more details.
</Warning>

### Choosing the nonce strategy

In the quoter service, select the nonce based on the solver mode:

```typescript theme={null}
const quoteDeadlineMs = params.min_deadline_ms + quoteDeadlineExtraMs;
const deadline = new Date(Date.now() + quoteDeadlineMs);

// Use versioned nonces for confidential, deterministic for public
const nonce = isConfidentialMode 
  ? this.intentsService.generateVersionedNonce(deadline) 
  : currentState.nonce;
```

Unlike public intents where the nonce only changes after a trade settles, confidential intents use a fresh nonce for each quote. This removes the throughput limitation of one trade per nonce.

***

## Intent signing

### Recipient contract

Confidential intents sign against a different contract ID:

```typescript theme={null}
export const intentsContract = process.env.INTENTS_CONTRACT || 'intents.near';
export const privateIntentsContract = process.env.PRIVATE_INTENTS_CONTRACT || '';

export const activeIntentsContract = isConfidentialMode 
  ? privateIntentsContract 
  : intentsContract;
```

The `recipient` field in the signed payload uses this active contract:

```typescript theme={null}
const messageStr = JSON.stringify(message);
const recipient = activeIntentsContract; // "intents.far" for confidential
const nonce = isConfidentialMode 
  ? this.intentsService.generateVersionedNonce(deadline) 
  : currentState.nonce;

const quoteHash = serializeIntent(messageStr, recipient, nonce, 'nep413');
const signature = await nearService.signMessage(quoteHash);
```

### Intent structure

The `token_diff` intent structure is identical for both modes — only the asset identifiers and recipient change:

```typescript theme={null}
const message: IMessage = {
  signer_id: nearService.getIntentsAccountId(),
  deadline: deadline.toISOString(),
  intents: [
    {
      intent: 'token_diff',
      diff: {
        // Confidential asset identifiers
        [params.defuse_asset_identifier_in]: params.exact_amount_in 
          ? params.exact_amount_in 
          : amount,
        [params.defuse_asset_identifier_out]: `-${
          params.exact_amount_out ? params.exact_amount_out : amount
        }`,
      },
    },
  ],
};
```

***

## WebSocket connection

### Private relay URL

Configure the WebSocket to connect to the private relay with an instance ID for guaranteed delivery:

```typescript theme={null}
const publicRelayWsUrl = process.env.RELAY_WS_URL || 'wss://solver-relay-v2.chaindefuser.com/ws';
const privateRelayWsUrl = process.env.PRIVATE_RELAY_WS_URL || '';
const solverInstanceId = process.env.SOLVER_INSTANCE_ID?.trim();

function withSolverInstanceId(url: string): string {
  if (!solverInstanceId) return url;
  
  const relayUrl = new URL(url);
  relayUrl.searchParams.set('instance_id', solverInstanceId);
  return relayUrl.toString();
}

export const wsRelayUrl = isConfidentialMode 
  ? withSolverInstanceId(privateRelayWsUrl) 
  : publicRelayWsUrl;
```

### Authentication

Both relays require a Partner JWT in the connection headers:

```typescript theme={null}
const partnerJwt = process.env.PARTNER_JWT?.trim();

export const wsRelayOptions: ClientOptions | undefined = partnerJwt
  ? {
      headers: {
        Authorization: `Bearer ${partnerJwt}`,
      },
    }
  : undefined;
```

Then pass the options when creating the WebSocket:

```typescript theme={null}
this.wsConnection = wsRelayOptions 
  ? new WebSocket(wsRelayUrl, wsRelayOptions) 
  : new WebSocket(wsRelayUrl);
```

***

## Guaranteed delivery

### Quote status acknowledgements

The private relay uses an acknowledgement mechanism to guarantee delivery of quote status updates. When subscribing to `quote_status_extended`, pass a third parameter to enable acknowledgements:

```typescript theme={null}
private getSubscribeParams(eventKind: RelayEventKind) {
  if (isConfidentialMode && eventKind === RelayEventKind.QUOTE_STATUS_EXTENDED) {
    // [eventKind, filter, enableAcknowledgements]
    return [eventKind, null, true];
  }
  
  return [eventKind];
}
```

### Processing with acknowledgements

Each `quote_status` event includes a sequence number. The solver must acknowledge receipt before processing:

```typescript theme={null}
private async acknowledgeQuoteStatus(
  params: Record<string, unknown>, 
  logger: LoggerService
): Promise<boolean> {
  if (!isConfidentialMode) {
    return true; // No acknowledgement needed for public relay
  }

  if (typeof params.subscription !== 'string' || typeof params.seq !== 'number') {
    logger.debug('Skipping quote status without subscription id and seq');
    return false;
  }

  try {
    await this.sendRequestToRelay(
      RelayMethod.ACKNOWLEDGE, 
      [params.subscription, params.seq], 
      logger
    );
    return true;
  } catch (error) {
    logger.error('Error acknowledging quote status', error);
    return false;
  }
}
```

Call this before processing the quote status:

```typescript theme={null}
case RelayEventKind.QUOTE_STATUS_EXTENDED:
  if (!(await this.acknowledgeQuoteStatus(req.params, logger))) {
    return;
  }
  await this.processQuoteStatus(req.params.data);
  break;
```

<Tip>
  The `instance_id` in the WebSocket URL enables the relay to track which events have been acknowledged. If the solver disconnects, unacknowledged events are redelivered on reconnect.
</Tip>

***

## Balance queries

### 1Click API integration

Confidential balances are not visible on the public NEAR chain. Query them through the 1Click API instead.

<Warning>
  The 1Click API returns balances with the `imt:<minter>:` prefix **stripped**. When querying balances, pass the **public** asset identifiers (e.g., `nep141:wrap.near`), not the IMT-wrapped versions. The response will use these same public identifiers as keys.
</Warning>

```typescript theme={null}
import { AccountService, UserAuthService, OpenAPI } from '@defuse-protocol/one-click-sdk-typescript';

private userToken: { accessToken: string; expiresAtMs: number } | null = null;

private async getBalancesFromOneClick(publicTokenIds: string[]): Promise<string[]> {
  // Authenticate and configure the SDK
  const userToken = await this.getOneClickUserToken();
  OpenAPI.TOKEN = userToken;
  
  // Query using PUBLIC asset IDs (not IMT-wrapped)
  const response = await AccountService.getBalances(publicTokenIds);
  const balancesByTokenId = new Map(
    response.balances.map(({ tokenId, available }) => [tokenId, available])
  );

  return publicTokenIds.map(tokenId => balancesByTokenId.get(tokenId) ?? '0');
}
```

### Authentication

The 1Click API requires an authenticated user token. Generate one by signing an intent:

```typescript theme={null}
import { createIntentSignerNEP413, IntentsSDK, VersionedNonceBuilder } from '@defuse-protocol/intents-sdk';

private async getOneClickUserToken(): Promise<string> {
  // Return cached token if still valid
  if (this.userToken && this.userToken.expiresAtMs > Date.now()) {
    return this.userToken.accessToken;
  }

  const signer = createIntentSignerNEP413({
    accountId: this.nearService.getIntentsAccountId(),
    signMessage: async (_payload, hash) => {
      const signature = await this.nearService.signMessage(hash);
      return {
        publicKey: signature.publicKey.toString(),
        signature: Buffer.from(signature.signature).toString('base64'),
      };
    },
  });

  const sdk = new IntentsSDK({ referral: 'solver', env: 'production' });
  const { signed } = await sdk
    .intentBuilder()
    .setDeadline(new Date(Date.now() + 5 * 60_000))
    .setNonceRandomBytes(VersionedNonceBuilder.createTimestampedNonceBytes(new Date()))
    .buildAndSign(signer);

  const auth = await UserAuthService.authenticate({ signedData: signed });
  
  this.userToken = {
    accessToken: auth.accessToken,
    expiresAtMs: Date.now() + auth.expiresIn * 1000 - 60_000,
  };

  return this.userToken.accessToken;
}
```

### Unified balance method

Switch between on-chain and 1Click queries based on mode. Note that for confidential mode, you need to convert IMT-wrapped token IDs back to public format for the API call:

```typescript theme={null}
public async getBalances(tokenIds: string[]): Promise<string[]> {
  if (isConfidentialMode) {
    // Convert IMT-wrapped IDs to public format for the API
    const publicTokenIds = tokenIds.map(id => this.toPublicAssetId(id));
    return this.getBalancesFromOneClick(publicTokenIds);
  }
  
  return this.getBalancesOnContract(tokenIds);
}

// Strip the IMT prefix to get the public asset identifier
private toPublicAssetId(imtTokenId: string): string {
  const prefix = `imt:${privateTreasuryAccountId}:`;
  if (imtTokenId.startsWith(prefix)) {
    return imtTokenId.slice(prefix.length);
  }
  return imtTokenId;
}
```

***

## Solver response format

When responding to confidential quote requests, solvers must return their signed intents in the `private_signed_data` bundle. This structure contains up to three signed intents:

```typescript theme={null}
interface PrivateSignedData {
  shield?: SignedData;   // Optional: shield public liquidity to Treasury
  swap: SignedData;      // Required: the token_diff intent for the swap
  recover?: SignedData;  // Required if shield present: fallback burn if swap fails
  solver_id?: string;    // Your solver identifier
}
```

`SignedData` is the same signed-payload shape used everywhere else in the protocol (see [publish\_intent](/integration/market-makers/message-bus/rpc#publish_intent)):

```typescript theme={null}
interface SignedData {
  standard: 'nep413' | 'erc191' | 'raw_ed25519';
  payload: { recipient: string; nonce: string; message: string };
  public_key: string;
  signature: string;
}
```

<Info>
  For the full rules on when each of `shield`/`swap`/`recover` is required (not just their types), see [Confidential Intents → Solver response format](/integration/market-makers/confidential-intents#solver-response-format).
</Info>

Each of `shield`, `swap`, and `recover` is signed the same way, by the solver's own NEAR key, using the same `near-api-js` signing flow the [Example Solver](/integration/market-makers/example#assembly-and-signing) uses for public `token_diff` intents (build the message, sign it, bs58-encode the result into `public_key`/`signature`). The only thing that changes per intent is the `message` content, `swap` signs a `token_diff`, `shield`/`recover` sign whatever transfer/burn intent your Treasury integration expects.

Here's what a fully populated `private_signed_data` bundle looks like on the wire, shielding public liquidity before the swap:

```json theme={null}
{
  "quote_output": { "amount_out": "300" },
  "private_signed_data": {
    "shield": {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzAEiS6N1S/hfrNi8/We0ieTmcMBti1YE=",
        "message": "{\"deadline\":\"2026-07-16T12:05:00.000Z\",\"intents\":[{\"intent\":\"transfer\",\"tokens\":{\"nep141:ft1.near\":\"500\"},\"receiver_id\":\"<PRIVATE_TREASURY_ACCOUNT_ID>\",\"memo\":\"<encrypted-recipient>\"}],\"signer_id\":\"your-solver.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:5tk3UyFcAgnd6D4ZAuzEdZqMrneRSiTqe48ptjbjYHwiCy2vTw38uDB3KusW2cEsF3TGcqZXoQmRaeNs2erhPpqu"
    },
    "swap": {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.far",
        "nonce": "Vij2xgAlKBKzAEiS6N1S/hcQLaHzX2s1fNKhBDblXT4=",
        "message": "{\"deadline\":\"2026-07-16T12:05:00.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:ft2.near\":\"-300\",\"nep141:ft1.near\":\"500\"}},{\"intent\":\"imt_burn\",\"minter_id\":\"<PRIVATE_TREASURY_ACCOUNT_ID>\",\"tokens\":{\"nep141:ft1.near\":\"500\"},\"memo\":\"your-solver.near\"}],\"signer_id\":\"your-solver.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:WQXG37prMyT4f1vp6JSvamsjqDR5fSnDiinSPaCoq9sPcDgFGPRiMWX7csqqudDbzc8i6wrfpemgpVX2wQDmwww"
    },
    "recover": {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.far",
        "nonce": "Vij2xgAlKBKzAEiS6N1S/hcQLaHzX2s1fNKhBDblXT4=",
        "message": "{\"deadline\":\"2026-07-16T12:05:00.000Z\",\"intents\":[{\"intent\":\"imt_burn\",\"minter_id\":\"<PRIVATE_TREASURY_ACCOUNT_ID>\",\"tokens\":{\"nep141:ft1.near\":\"500\"},\"memo\":\"your-solver.near\"}],\"signer_id\":\"your-solver.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:2erhPpquWQXG37prMyT4f1vp6JSvamsjqDR5fSnDiinSPaCoq9sPcDgFGPRiMWX7csqqudDbzc8i"
    },
    "solver_id": "your-solver.near"
  }
}
```

* **`shield`** signs a `transfer` intent on the **public** contract (`recipient: "intents.near"`), moving your liquidity into the Treasury account. `memo` carries the encrypted recipient the Private PoA Bridge needs to mint the matching IMT on FAR.
* **`swap`** signs against the **private** contract (`recipient: "intents.far"`) and bundles two intents together: the `token_diff` (the actual swap) plus an `imt_burn` that unshields the received funds immediately instead of leaving them shielded. Its `nonce` must exactly match `recover`'s.
* **`recover`** also signs against `intents.far`, a single `imt_burn` for the amount that was shielded, using the same `nonce` as `swap` so only one of the two can ever execute.

### Building the response

When quoting, construct the `private_signed_data` bundle with your signed swap intent:

```typescript theme={null}
// Build the swap intent (token_diff)
const swapIntent = await this.buildSignedSwapIntent(params, deadline);

// Return the quote response with private_signed_data
// quote_output uses the same shape as public quoting: amount_out for
// exact_amount_in requests, amount_in for exact_amount_out requests
const quoteResponse = {
  quote_output: {
    amount_out: calculatedAmount,
  },
  private_signed_data: {
    swap: swapIntent.signedData,
    solver_id: process.env.SOLVER_ID,
  },
};
```

If you need to shield public liquidity for the swap, include all three intents:

```typescript theme={null}
const quoteResponse = {
  quote_output: { amount_out: calculatedAmount },
  private_signed_data: {
    shield: shieldIntent.signedData,    // Transfer to Treasury
    swap: swapIntent.signedData,        // The actual swap
    recover: recoverIntent.signedData,  // Fallback burn (same nonce as swap)
    solver_id: process.env.SOLVER_ID,
  },
};
```

<Warning>
  The `recover` intent must share the same nonce as the `swap` intent. This ensures only one of them can execute — either the swap succeeds, or the recover burns the shielded tokens back to public.
</Warning>

***

## Depositing liquidity

Before quoting confidential swaps, deposit liquidity from public Intents into your confidential balance.

### Using the deposit script

The example solver includes a script for shielding liquidity:

```bash theme={null}
NODE_ENV=local \
ONE_CLICK_ASSET_ID=nep141:wrap.near \
ONE_CLICK_AMOUNT=100000000000000000000000 \
npm run one-click:deposit-confidential
```

Run this for each token in your pair:

```bash theme={null}
# Deposit wNEAR
NODE_ENV=local \
ONE_CLICK_ASSET_ID=nep141:wrap.near \
ONE_CLICK_AMOUNT=100000000000000000000000 \
npm run one-click:deposit-confidential

# Deposit USDT
NODE_ENV=local \
ONE_CLICK_ASSET_ID=nep141:usdt.tether-token.near \
ONE_CLICK_AMOUNT=1000000 \
npm run one-click:deposit-confidential
```

<Warning>
  Amounts are in the token's smallest unit. For 24-decimal tokens like wNEAR, `100000000000000000000000` equals 0.1 wNEAR.
</Warning>

### Withdrawing liquidity

To move liquidity back to public Intents:

```bash theme={null}
NODE_ENV=local \
ONE_CLICK_ASSET_ID=nep141:wrap.near \
ONE_CLICK_AMOUNT=100000000000000000000000 \
npm run one-click:withdraw-confidential
```

***

## Running the solver

With liquidity deposited, start the solver in confidential mode:

```bash theme={null}
NODE_ENV=local npm start
```

The solver will:

1. Connect to the private relay with your instance ID
2. Subscribe to confidential quote requests
3. Query balances from the 1Click API
4. Respond with signed quotes using versioned nonces
5. Acknowledge quote status events for guaranteed delivery

***

## Quick reference

| Public Intents                              | Confidential Intents                          |
| ------------------------------------------- | --------------------------------------------- |
| `SOLVER_MODE=public`                        | `SOLVER_MODE=confidential`                    |
| `wss://solver-relay-v2.chaindefuser.com/ws` | Private relay URL (contact Defuse team)       |
| `intents.near`                              | `intents.far`                                 |
| `nep141:wrap.near`                          | `imt:<treasury>:nep141:wrap.near`             |
| Deterministic nonce                         | Versioned nonce with salt                     |
| On-chain balance query                      | 1Click API balance query                      |
| No acknowledgements                         | `quote_status_extended` with acknowledgements |

***

## Next steps

<CardGroup cols={2}>
  <Card title="Confidential Overview" icon="lock" href="/integration/market-makers/confidential-intents">
    Learn more about the confidential intents architecture
  </Card>

  <Card title="Guaranteed Delivery" icon="check-double" href="/integration/market-makers/message-bus/guaranteed-delivery">
    Deep dive into the acknowledgement protocol
  </Card>
</CardGroup>
