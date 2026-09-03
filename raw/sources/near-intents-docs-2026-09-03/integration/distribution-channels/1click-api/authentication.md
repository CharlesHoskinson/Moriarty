> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# API Keys

> Obtain a JWT token for authenticated API access

Unauthenticated requests incur a **0.2% (20 basis points)** platform fee. Authenticated requests using a JWT token are **fee-free** - you only pay network gas costs and market maker spreads.

<Callout color="#ff7043" icon="key">
  Get your JWT token from the <a href="https://partners.near-intents.org/home" class="underline font-medium">Partner Dashboard</a> for authenticated API access and fee-free swaps.
</Callout>

***

## How to Use Your API Key

Once you have your JWT token from the Partner Dashboard, send it as `X-API-Key` on API requests. `Authorization: Bearer` is also accepted for the same partner JWT.

<CodeGroup>
  ```bash cURL theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "X-API-Key: YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "originAsset": "nep141:wrap.near",
      "destinationAsset": "nep141:usdt.tether-token.near",
      "amount": "1000000000000000000000000",
      "recipient": "your-account.near",
      "recipientType": "INTENTS",
      "refundTo": "your-account.near",
      "refundType": "INTENTS"
    }'
  ```

  ```javascript JavaScript theme={null}
  const response = await fetch('https://1click.chaindefuser.com/v0/quote', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-Key': 'YOUR_JWT_TOKEN'
    },
    body: JSON.stringify({
      dry: false,
      swapType: 'EXACT_INPUT',
      originAsset: 'nep141:wrap.near',
      destinationAsset: 'nep141:usdt.tether-token.near',
      amount: '1000000000000000000000000',
      recipient: 'your-account.near',
      recipientType: 'INTENTS',
      refundTo: 'your-account.near',
      refundType: 'INTENTS'
    })
  });
  ```

  ```python Python theme={null}
  import requests

  response = requests.post(
    'https://1click.chaindefuser.com/v0/quote',
    headers={
      'Content-Type': 'application/json',
      'X-API-Key': 'YOUR_JWT_TOKEN'
    },
    json={
      'dry': False,
      'swapType': 'EXACT_INPUT',
      'originAsset': 'nep141:wrap.near',
      'destinationAsset': 'nep141:usdt.tether-token.near',
      'amount': '1000000000000000000000000',
      'recipient': 'your-account.near',
      'recipientType': 'INTENTS',
      'refundTo': 'your-account.near',
      'refundType': 'INTENTS'
    }
  )
  ```
</CodeGroup>

<Tip>
  Store your JWT token securely and never commit it to version control. Use environment variables or secure secret management systems.
</Tip>

***

## Authenticating end users (Confidential Intents)

Your Partner JWT authenticates **your integration** and waives the platform fee. It does not let you read a user's confidential data. Reading private balances or confidential transaction history requires a separate **User-Session** token that proves the account owner (an end user, or a solver checking its own balance) actually controls the account, since none of that data is exposed by any public endpoint.

### Get a User-Session token

`public_key` and `signature` aren't something you type in, they come from having the account owner's wallet sign a message:

1. Build a `MultiPayload` message: `recipient` set to `"intents.near"`, a fresh `nonce`, and `message` set to a stringified JSON object with an empty `intents` array plus a `deadline` and the account's `signer_id`. The empty `intents` array is what makes this a proof of ownership instead of a real swap, see [Signing Intents](/integration/verifier-contract/signing-intents) for the full `MultiPayload` spec across every supported wallet standard.
2. Have that account's wallet sign the message. The wallet returns the `public_key` that signed and the resulting `signature`.
3. Send `payload`, `public_key`, and `signature` together as `signedData` to `/v0/auth/authenticate`.

<CodeGroup>
  ```bash cURL theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/auth/authenticate \
    -H "Content-Type: application/json" \
    -d '{
      "signedData": {
        "standard": "nep413",
        "payload": {
          "recipient": "intents.near",
          "nonce": "Vij2xgAlKBKzAEiS6N1S/hfrNi8/We0ieTmcMBti1YE=",
          "message": "{\"deadline\":\"2026-07-16T12:00:00.000Z\",\"intents\":[],\"signer_id\":\"your-account.near\"}"
        },
        "public_key": "ed25519:YOUR_PUBLIC_KEY",
        "signature": "ed25519:YOUR_SIGNATURE"
      }
    }'
  ```

  ```typescript TypeScript SDK theme={null}
  import { createIntentSignerNEP413, IntentsSDK } from '@defuse-protocol/intents-sdk';
  import { UserAuthService } from '@defuse-protocol/one-click-sdk-typescript';

  // Step 1 + 2: wire up a signer backed by the account's wallet
  const signer = createIntentSignerNEP413({
    accountId: userAccountId,
    signMessage: async (_payload, hash) => {
      const { publicKey, signature } = await userWallet.signMessage(hash);
      return { publicKey: publicKey.toString(), signature: Buffer.from(signature).toString('base64') };
    },
  });

  // Build a payload with an empty intents array and sign it
  const sdk = new IntentsSDK({ referral: 'your-integration', env: 'production' });
  const { signed } = await sdk
    .intentBuilder()
    .setDeadline(new Date(Date.now() + 5 * 60_000))
    .buildAndSign(signer);

  // Step 3: exchange the signed payload for a User-Session token
  const auth = await UserAuthService.authenticate({ signedData: signed });
  // auth.accessToken, auth.refreshToken, auth.expiresIn, auth.refreshExpiresIn
  ```
</CodeGroup>

| Field                            | Description                                                                                                                            |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `accessToken`                    | Short-lived bearer token. Send it as `Authorization: Bearer <accessToken>` on `GET /v0/account/balances` and `GET /v0/account/history` |
| `refreshToken`                   | Exchange this for a new `accessToken` without re-signing                                                                               |
| `expiresIn` / `refreshExpiresIn` | Lifetime of `accessToken` / `refreshToken`, in seconds                                                                                 |

### Refresh a token

When `accessToken` expires, exchange `refreshToken` for a new one instead of re-signing:

```bash theme={null}
curl -X POST https://1click.chaindefuser.com/v0/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{ "refreshToken": "YOUR_REFRESH_TOKEN" }'
```

Returns a new `{ accessToken, expiresIn }`.

<Tip>
  See this flow wired into a real solver in [Confidential Example → Authentication](/integration/market-makers/confidential-example#authentication).
</Tip>
