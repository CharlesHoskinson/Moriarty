> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Authenticate User with Signed Data

> Exchange a signed message for a User-Session access token

Send this endpoint a signed message from your user, and you'll get back a **User-Session** token. That token is what lets the user's confidential balances and transaction history (`GET /v0/account/balances`, `GET /v0/account/history`) be revealed. It's separate from your Partner JWT, which authenticates your integration, not an individual user.

<Info>
  This is part of **Confidential Intents**. See [Authenticating end users](/integration/distribution-channels/1click-api/authentication#authenticating-end-users-confidential-intents) for the full guide.
</Info>

## Getting a signature to send

Your user's NEAR wallet produces `public_key` and `signature`. The steps:

1. Build a NEP-413 payload: `recipient` set to `"intents.near"`, a fresh random `nonce`, and `message` set to a stringified JSON object with an empty `intents` array plus a `deadline` and the user's `signer_id`. The empty `intents` array is what makes this a proof of ownership instead of a real swap.
2. Have the user's wallet sign that payload. The wallet returns the `publicKey` that signed it and the resulting `signature`, both prefixed `ed25519:`.
3. Send `payload`, `public_key`, and `signature` together as `signedData` on this endpoint.

If you're on `@defuse-protocol/intents-sdk`, `createIntentSignerNEP413` and `buildAndSign()` do steps 1 and 2 for you, that's what the TypeScript SDK example below uses.

## Example request

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

  ```javascript JavaScript theme={null}
  const response = await fetch('https://1click.chaindefuser.com/v0/auth/authenticate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      signedData: {
        standard: 'nep413',
        payload: {
          recipient: 'intents.near',
          nonce: 'Vij2xgAlKBKzAEiS6N1S/hfrNi8/We0ieTmcMBti1YE=',
          message: JSON.stringify({
            deadline: '2026-07-16T12:00:00.000Z',
            intents: [],
            signer_id: 'your-account.near'
          })
        },
        public_key: 'ed25519:YOUR_PUBLIC_KEY',
        signature: 'ed25519:YOUR_SIGNATURE'
      }
    })
  });

  const auth = await response.json();
  ```

  ```typescript TypeScript SDK theme={null}
  import { createIntentSignerNEP413, IntentsSDK } from '@defuse-protocol/intents-sdk';
  import { UserAuthService } from '@defuse-protocol/one-click-sdk-typescript';

  // Step 1 + 2: wire up a signer backed by the user's wallet
  const signer = createIntentSignerNEP413({
    accountId: userAccountId,
    signMessage: async (_payload, hash) => {
      const { publicKey, signature } = await userWallet.signMessage(hash);
      return { publicKey: publicKey.toString(), signature: Buffer.from(signature).toString('base64') };
    },
  });

  // Build a payload with an empty intents array (just a signature, not a real swap) and sign it
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

## Example response

```json theme={null}
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expiresIn": 3600,
  "refreshExpiresIn": 2592000
}
```

<Tip>
  `signedData` is the same `MultiPayload` signed-message format used for [Signing Intents](/integration/verifier-contract/signing-intents), signed here with an empty `intents` array. It's used purely as proof of account ownership, not a real swap.
</Tip>

<Warning>
  Store `refreshToken` securely. Anyone holding it can mint new `accessToken`s for this account until it expires.
</Warning>


## OpenAPI

````yaml post /v0/auth/authenticate
openapi: 3.0.0
info:
  title: 1Click Swap API
  description: API for One-Click Swaps
  version: 0.1.10
  contact: {}
servers:
  - url: https://1click.chaindefuser.com
security: []
tags: []
paths:
  /v0/auth/authenticate:
    post:
      tags:
        - User Auth
      summary: Authenticate user with signed data
      description: Verifies wallet signature and issues session tokens
      operationId: authenticate
      parameters: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/AuthenticateRequestDto'
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/AuthenticateResponseDto'
        '400':
          description: Invalid signed data
        '401':
          description: Signature verification failed
components:
  schemas:
    AuthenticateRequestDto:
      type: object
      properties:
        signedData:
          oneOf:
            - $ref: '#/components/schemas/MultiPayload'
      required:
        - signedData
    AuthenticateResponseDto:
      type: object
      properties:
        accessToken:
          type: string
          description: JWT access token for API calls
        refreshToken:
          type: string
          description: JWT refresh token for getting new access tokens
        expiresIn:
          type: number
          description: Access token expiration time in seconds
        refreshExpiresIn:
          type: number
          description: Refresh token expiration time in seconds
      required:
        - accessToken
        - refreshToken
        - expiresIn
        - refreshExpiresIn
    MultiPayload:
      description: >-
        Assuming wallets want to interact with Intents protocol, besides
        preparing the data in a certain form, they have to have the capability
        to sign raw messages (off-chain signatures) using an algorithm we
        understand. This enum solves that problem.


        For example, because we support ERC-191 and know how to verify messages
        with that standard, we can allow wallets, like Metamask, sign messages
        to perform intents without having to support new cryptographic
        primitives and signing standards.
      oneOf:
        - description: >-
            NEP-413: The standard for message signing in Near Protocol. For more
            details, refer to
            [NEP-413](https://github.com/near/NEPs/blob/master/neps/nep-0413.md).
          type: object
          required:
            - payload
            - public_key
            - signature
            - standard
          properties:
            payload:
              description: >-
                See
                [NEP-413](https://github.com/near/NEPs/blob/master/neps/nep-0413.md)
              type: object
              required:
                - message
                - nonce
                - recipient
              properties:
                callbackUrl:
                  type: string
                  nullable: true
                message:
                  type: string
                  x-parseJson:
                    type: object
                    required:
                      - deadline
                      - signer_id
                    properties:
                      deadline:
                        type: string
                      signer_id:
                        description: >-
                          NEAR Account Identifier.


                          This is a unique, syntactically valid, human-readable
                          account identifier on the NEAR network.


                          [See the crate-level docs for information about
                          validation.](index.html#account-id-rules)


                          Also see [Error kind
                          precedence](AccountId#error-kind-precedence).


                          ## Examples


                          ``` use near_account_id::AccountId;


                          let alice: AccountId = "alice.near".parse().unwrap();


                          assert!("ƒelicia.near".parse::<AccountId>().is_err());
                          // (ƒ is not f) ```
                        type: string
                      intents:
                        description: >-
                          Sequence of intents to execute in given order. Empty
                          list is also a valid sequence, i.e. it doesn't do
                          anything, but still invalidates the `nonce` for the
                          signer WARNING: Promises created by different intents
                          are executed concurrently and does not rely on the
                          order of the intents in this structure
                        type: array
                        items:
                          oneOf:
                            - description: See [`AddPublicKey`]
                              type: object
                              required:
                                - intent
                                - public_key
                              properties:
                                intent:
                                  type: string
                                  enum:
                                    - add_public_key
                                public_key:
                                  examples:
                                    - >-
                                      ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                    - >-
                                      secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                                  type: string
                                  description: 'Encoding: base58'
                              additionalProperties: false
                            - description: See [`RemovePublicKey`]
                              type: object
                              required:
                                - intent
                                - public_key
                              properties:
                                intent:
                                  type: string
                                  enum:
                                    - remove_public_key
                                public_key:
                                  examples:
                                    - >-
                                      ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                    - >-
                                      secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                                  type: string
                                  description: 'Encoding: base58'
                              additionalProperties: false
                            - description: See [`Transfer`]
                              type: object
                              required:
                                - intent
                                - receiver_id
                                - tokens
                              properties:
                                intent:
                                  type: string
                                  enum:
                                    - transfer
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                min_gas:
                                  description: >-
                                    Minimum gas for `mt_on_transfer()`


                                    Remaining gas will be distributed evenly
                                    across all Function Call Promises created
                                    during execution of current receipt.
                                  type:
                                    - string
                                    - 'null'
                                msg:
                                  description: Message to pass to `mt_on_transfer`
                                  type: string
                                receiver_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                state_init:
                                  description: >-
                                    Optionally initialize the receiver's
                                    contract (Deterministic AccountId) via
                                    [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                    right before calling `mt_on_transfer()` (in
                                    the same receipt).
                                  anyOf:
                                    - oneOf:
                                        - type: object
                                          required:
                                            - code
                                            - data
                                            - version
                                          properties:
                                            code:
                                              oneOf:
                                                - type: object
                                                  required:
                                                    - hash
                                                  properties:
                                                    hash:
                                                      type: string
                                                  additionalProperties: false
                                                - type: object
                                                  required:
                                                    - account_id
                                                  properties:
                                                    account_id:
                                                      description: >-
                                                        NEAR Account Identifier.


                                                        This is a unique, syntactically valid,
                                                        human-readable account identifier on the
                                                        NEAR network.


                                                        [See the crate-level docs for
                                                        information about
                                                        validation.](index.html#account-id-rules)


                                                        Also see [Error kind
                                                        precedence](AccountId#error-kind-precedence).


                                                        ## Examples


                                                        ``` use near_account_id::AccountId;


                                                        let alice: AccountId =
                                                        "alice.near".parse().unwrap();


                                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                        // (ƒ is not f) ```
                                                      type: string
                                                  additionalProperties: false
                                            data:
                                              type: object
                                              additionalProperties:
                                                type: string
                                            version:
                                              type: string
                                              enum:
                                                - v1
                                          additionalProperties: false
                                    - type: 'null'
                                tokens:
                                  type: object
                                  additionalProperties:
                                    type: string
                              additionalProperties: false
                            - description: See [`FtWithdraw`]
                              type: object
                              required:
                                - amount
                                - intent
                                - receiver_id
                                - token
                              properties:
                                amount:
                                  type: string
                                intent:
                                  type: string
                                  enum:
                                    - ft_withdraw
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                min_gas:
                                  description: >-
                                    Optional minimum required Near gas for
                                    created Promise to succeed: *
                                    `ft_transfer`:      minimum: 15TGas,
                                    default: 15TGas * `ft_transfer_call`:
                                    minimum: 30TGas, default: 50TGas


                                    Remaining gas will be distributed evenly
                                    across all Function Call Promises created
                                    during execution of current receipt.
                                  type:
                                    - string
                                    - 'null'
                                msg:
                                  description: >-
                                    Message to pass to `ft_transfer_call`.
                                    Otherwise, `ft_transfer` will be used. NOTE:
                                    No refund will be made in case of
                                    insufficient `storage_deposit` on `token`
                                    for `receiver_id`
                                  type:
                                    - string
                                    - 'null'
                                receiver_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                storage_deposit:
                                  description: >-
                                    Optionally make `storage_deposit` for
                                    `receiver_id` on `token`. The amount will be
                                    subtracted from user's NEP-141 `wNEAR`
                                    balance. NOTE: the `wNEAR` will not be
                                    refunded in case of fail
                                  type:
                                    - string
                                    - 'null'
                                token:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                              additionalProperties: false
                            - description: See [`NftWithdraw`]
                              type: object
                              required:
                                - intent
                                - receiver_id
                                - token
                                - token_id
                              properties:
                                intent:
                                  type: string
                                  enum:
                                    - nft_withdraw
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                min_gas:
                                  description: >-
                                    Optional minimum required Near gas for
                                    created Promise to succeed: *
                                    `nft_transfer`:      minimum: 15TGas,
                                    default: 15TGas * `nft_transfer_call`:
                                    minimum: 30TGas, default: 50TGas


                                    Remaining gas will be distributed evenly
                                    across all Function Call Promises created
                                    during execution of current receipt.
                                  type:
                                    - string
                                    - 'null'
                                msg:
                                  description: >-
                                    Message to pass to `nft_transfer_call`.
                                    Otherwise, `nft_transfer` will be used.
                                    NOTE: No refund will be made in case of
                                    insufficient `storage_deposit` on `token`
                                    for `receiver_id`
                                  type:
                                    - string
                                    - 'null'
                                receiver_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                storage_deposit:
                                  description: >-
                                    Optionally make `storage_deposit` for
                                    `receiver_id` on `token`. The amount will be
                                    subtracted from user's NEP-141 `wNEAR`
                                    balance. NOTE: the `wNEAR` will not be
                                    refunded in case of fail
                                  type:
                                    - string
                                    - 'null'
                                token:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                token_id:
                                  type: string
                              additionalProperties: false
                            - description: See [`MtWithdraw`]
                              type: object
                              required:
                                - amounts
                                - intent
                                - receiver_id
                                - token
                                - token_ids
                              properties:
                                amounts:
                                  type: array
                                  items:
                                    type: string
                                intent:
                                  type: string
                                  enum:
                                    - mt_withdraw
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                min_gas:
                                  description: >-
                                    Optional minimum required Near gas for
                                    created Promise to succeed per token: *
                                    `mt_batch_transfer`:      minimum: 20TGas,
                                    default: 20TGas * `mt_batch_transfer_call`:
                                    minimum: 35TGas, default: 50TGas


                                    Remaining gas will be distributed evenly
                                    across all Function Call Promises created
                                    during execution of current receipt.
                                  type:
                                    - string
                                    - 'null'
                                msg:
                                  description: >-
                                    Message to pass to `mt_batch_transfer_call`.
                                    Otherwise, `mt_batch_transfer` will be used.
                                    NOTE: No refund will be made in case of
                                    insufficient `storage_deposit` on `token`
                                    for `receiver_id`
                                  type:
                                    - string
                                    - 'null'
                                receiver_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                storage_deposit:
                                  description: >-
                                    Optionally make `storage_deposit` for
                                    `receiver_id` on `token`. The amount will be
                                    subtracted from user's NEP-141 `wNEAR`
                                    balance. NOTE: the `wNEAR` will not be
                                    refunded in case of fail
                                  type:
                                    - string
                                    - 'null'
                                token:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                token_ids:
                                  type: array
                                  items:
                                    type: string
                              additionalProperties: false
                            - description: See [`NativeWithdraw`]
                              type: object
                              required:
                                - amount
                                - intent
                                - receiver_id
                              properties:
                                amount:
                                  type: string
                                intent:
                                  type: string
                                  enum:
                                    - native_withdraw
                                receiver_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                              additionalProperties: false
                            - description: See [`StorageDeposit`]
                              type: object
                              required:
                                - amount
                                - contract_id
                                - deposit_for_account_id
                                - intent
                              properties:
                                amount:
                                  type: string
                                contract_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                deposit_for_account_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                intent:
                                  type: string
                                  enum:
                                    - storage_deposit
                              additionalProperties: false
                            - description: See [`TokenDiff`]
                              type: object
                              required:
                                - diff
                                - intent
                              properties:
                                diff:
                                  type: object
                                  additionalProperties:
                                    type: string
                                intent:
                                  type: string
                                  enum:
                                    - token_diff
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                referral:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type:
                                    - string
                                    - 'null'
                              additionalProperties: false
                            - description: See [`SetAuthByPredecessorId`]
                              type: object
                              required:
                                - enabled
                                - intent
                              properties:
                                enabled:
                                  type: boolean
                                intent:
                                  type: string
                                  enum:
                                    - set_auth_by_predecessor_id
                              additionalProperties: false
                            - description: See [`AuthCall`]
                              type: object
                              required:
                                - contract_id
                                - intent
                                - msg
                              properties:
                                attached_deposit:
                                  description: >-
                                    Optionally, attach deposit to
                                    [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                    call. The amount will be subtracted from
                                    user's NEP-141 `wNEAR` balance.


                                    NOTE: the `wNEAR` will not be refunded in
                                    case of fail.
                                  type: string
                                contract_id:
                                  description: >-
                                    Callee for
                                    [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                  type: string
                                intent:
                                  type: string
                                  enum:
                                    - auth_call
                                min_gas:
                                  description: >-
                                    Optional minimum gas required for created
                                    promise to succeed. By default, only
                                    [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                    is required.


                                    Remaining gas will be distributed evenly
                                    across all Function Call Promises created
                                    during execution of current receipt.
                                  type:
                                    - string
                                    - 'null'
                                msg:
                                  description: >-
                                    `msg` to pass in
                                    [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                  type: string
                                state_init:
                                  description: >-
                                    Optionally initialize the receiver's
                                    contract (Deterministic AccountId) via
                                    [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                    right before calling
                                    [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                    (in the same receipt).
                                  anyOf:
                                    - oneOf:
                                        - type: object
                                          required:
                                            - code
                                            - data
                                            - version
                                          properties:
                                            code:
                                              oneOf:
                                                - type: object
                                                  required:
                                                    - hash
                                                  properties:
                                                    hash:
                                                      type: string
                                                  additionalProperties: false
                                                - type: object
                                                  required:
                                                    - account_id
                                                  properties:
                                                    account_id:
                                                      description: >-
                                                        NEAR Account Identifier.


                                                        This is a unique, syntactically valid,
                                                        human-readable account identifier on the
                                                        NEAR network.


                                                        [See the crate-level docs for
                                                        information about
                                                        validation.](index.html#account-id-rules)


                                                        Also see [Error kind
                                                        precedence](AccountId#error-kind-precedence).


                                                        ## Examples


                                                        ``` use near_account_id::AccountId;


                                                        let alice: AccountId =
                                                        "alice.near".parse().unwrap();


                                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                        // (ƒ is not f) ```
                                                      type: string
                                                  additionalProperties: false
                                            data:
                                              type: object
                                              additionalProperties:
                                                type: string
                                            version:
                                              type: string
                                              enum:
                                                - v1
                                          additionalProperties: false
                                    - type: 'null'
                              additionalProperties: false
                            - description: >-
                                Mint a set of tokens from the signer to a
                                specified account id, within the intents
                                contract.
                              type: object
                              required:
                                - intent
                                - receiver_id
                                - tokens
                              properties:
                                intent:
                                  type: string
                                  enum:
                                    - imt_mint
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                min_gas:
                                  description: >-
                                    Minimum gas for `mt_on_transfer()`


                                    Remaining gas will be distributed evenly
                                    across all Function Call Promises created
                                    during execution of current receipt.
                                  type:
                                    - string
                                    - 'null'
                                msg:
                                  description: Message to pass to `mt_on_transfer`
                                  type: string
                                receiver_id:
                                  description: Receiver of the minted tokens
                                  type: string
                                state_init:
                                  description: >-
                                    Optionally initialize the receiver's
                                    contract (Deterministic AccountId) via
                                    [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                    right before calling `mt_on_transfer()` (in
                                    the same receipt).
                                  anyOf:
                                    - oneOf:
                                        - type: object
                                          required:
                                            - code
                                            - data
                                            - version
                                          properties:
                                            code:
                                              oneOf:
                                                - type: object
                                                  required:
                                                    - hash
                                                  properties:
                                                    hash:
                                                      type: string
                                                  additionalProperties: false
                                                - type: object
                                                  required:
                                                    - account_id
                                                  properties:
                                                    account_id:
                                                      description: >-
                                                        NEAR Account Identifier.


                                                        This is a unique, syntactically valid,
                                                        human-readable account identifier on the
                                                        NEAR network.


                                                        [See the crate-level docs for
                                                        information about
                                                        validation.](index.html#account-id-rules)


                                                        Also see [Error kind
                                                        precedence](AccountId#error-kind-precedence).


                                                        ## Examples


                                                        ``` use near_account_id::AccountId;


                                                        let alice: AccountId =
                                                        "alice.near".parse().unwrap();


                                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                        // (ƒ is not f) ```
                                                      type: string
                                                  additionalProperties: false
                                            data:
                                              type: object
                                              additionalProperties:
                                                type: string
                                            version:
                                              type: string
                                              enum:
                                                - v1
                                          additionalProperties: false
                                    - type: 'null'
                                tokens:
                                  description: >-
                                    The token_ids will be wrapped to bind the
                                    token ID to the minter authority (i.e.
                                    signer of this intent). The final string
                                    representation of the token will be as
                                    follows: `imt:<minter_id>:<token_id>`
                                  type: object
                                  additionalProperties:
                                    type: string
                              additionalProperties: false
                            - description: >-
                                Burn a set of imt tokens, within the intents
                                contract.
                              type: object
                              required:
                                - intent
                                - minter_id
                                - tokens
                              properties:
                                intent:
                                  type: string
                                  enum:
                                    - imt_burn
                                memo:
                                  type:
                                    - string
                                    - 'null'
                                minter_id:
                                  description: >-
                                    NEAR Account Identifier.


                                    This is a unique, syntactically valid,
                                    human-readable account identifier on the
                                    NEAR network.


                                    [See the crate-level docs for information
                                    about
                                    validation.](index.html#account-id-rules)


                                    Also see [Error kind
                                    precedence](AccountId#error-kind-precedence).


                                    ## Examples


                                    ``` use near_account_id::AccountId;


                                    let alice: AccountId =
                                    "alice.near".parse().unwrap();


                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                    // (ƒ is not f) ```
                                  type: string
                                tokens:
                                  description: >-
                                    The token_ids will be wrapped to bind the
                                    token ID to the minter authority. The final
                                    string representation of the token will be
                                    as follows: `imt:<minter_id>:<token_id>`
                                  type: object
                                  additionalProperties:
                                    type: string
                              additionalProperties: false
                nonce:
                  type: string
                  description: 'Encoding: base64'
                recipient:
                  type: string
              additionalProperties: false
            public_key:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            signature:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            standard:
              type: string
              enum:
                - nep413
          additionalProperties: false
        - description: >-
            ERC-191: The standard for message signing in Ethereum, commonly used
            with `personal_sign()`. For more details, refer to
            [EIP-191](https://eips.ethereum.org/EIPS/eip-191).
          type: object
          required:
            - payload
            - signature
            - standard
          properties:
            payload:
              description: >-
                See
                [ERC-191](https://github.com/ethereum/ercs/blob/master/ERCS/erc-191.md)
              type: string
              x-parseJson:
                type: object
                required:
                  - deadline
                  - nonce
                  - signer_id
                  - verifying_contract
                properties:
                  deadline:
                    type: string
                  intents:
                    description: >-
                      Sequence of intents to execute in given order. Empty list
                      is also a valid sequence, i.e. it doesn't do anything, but
                      still invalidates the `nonce` for the signer WARNING:
                      Promises created by different intents are executed
                      concurrently and does not rely on the order of the intents
                      in this structure
                    type: array
                    items:
                      oneOf:
                        - description: See [`AddPublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - add_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`RemovePublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - remove_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`Transfer`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - transfer
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: See [`FtWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                            - token
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - ft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `ft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `ft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `ft_transfer_call`.
                                Otherwise, `ft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`NftWithdraw`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - token
                            - token_id
                          properties:
                            intent:
                              type: string
                              enum:
                                - nft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `nft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `nft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `nft_transfer_call`.
                                Otherwise, `nft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_id:
                              type: string
                          additionalProperties: false
                        - description: See [`MtWithdraw`]
                          type: object
                          required:
                            - amounts
                            - intent
                            - receiver_id
                            - token
                            - token_ids
                          properties:
                            amounts:
                              type: array
                              items:
                                type: string
                            intent:
                              type: string
                              enum:
                                - mt_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed per token: *
                                `mt_batch_transfer`:      minimum: 20TGas,
                                default: 20TGas * `mt_batch_transfer_call`:
                                minimum: 35TGas, default: 50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `mt_batch_transfer_call`.
                                Otherwise, `mt_batch_transfer` will be used.
                                NOTE: No refund will be made in case of
                                insufficient `storage_deposit` on `token` for
                                `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_ids:
                              type: array
                              items:
                                type: string
                          additionalProperties: false
                        - description: See [`NativeWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - native_withdraw
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`StorageDeposit`]
                          type: object
                          required:
                            - amount
                            - contract_id
                            - deposit_for_account_id
                            - intent
                          properties:
                            amount:
                              type: string
                            contract_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            deposit_for_account_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            intent:
                              type: string
                              enum:
                                - storage_deposit
                          additionalProperties: false
                        - description: See [`TokenDiff`]
                          type: object
                          required:
                            - diff
                            - intent
                          properties:
                            diff:
                              type: object
                              additionalProperties:
                                type: string
                            intent:
                              type: string
                              enum:
                                - token_diff
                            memo:
                              type:
                                - string
                                - 'null'
                            referral:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type:
                                - string
                                - 'null'
                          additionalProperties: false
                        - description: See [`SetAuthByPredecessorId`]
                          type: object
                          required:
                            - enabled
                            - intent
                          properties:
                            enabled:
                              type: boolean
                            intent:
                              type: string
                              enum:
                                - set_auth_by_predecessor_id
                          additionalProperties: false
                        - description: See [`AuthCall`]
                          type: object
                          required:
                            - contract_id
                            - intent
                            - msg
                          properties:
                            attached_deposit:
                              description: >-
                                Optionally, attach deposit to
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                call. The amount will be subtracted from user's
                                NEP-141 `wNEAR` balance.


                                NOTE: the `wNEAR` will not be refunded in case
                                of fail.
                              type: string
                            contract_id:
                              description: >-
                                Callee for
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            intent:
                              type: string
                              enum:
                                - auth_call
                            min_gas:
                              description: >-
                                Optional minimum gas required for created
                                promise to succeed. By default, only
                                [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                is required.


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                `msg` to pass in
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling
                                [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                (in the same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                          additionalProperties: false
                        - description: >-
                            Mint a set of tokens from the signer to a specified
                            account id, within the intents contract.
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_mint
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: Receiver of the minted tokens
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority (i.e. signer of this
                                intent). The final string representation of the
                                token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: >-
                            Burn a set of imt tokens, within the intents
                            contract.
                          type: object
                          required:
                            - intent
                            - minter_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_burn
                            memo:
                              type:
                                - string
                                - 'null'
                            minter_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority. The final string
                                representation of the token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                  nonce:
                    examples:
                      - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                    type: string
                    description: 'Encoding: base64'
                  signer_id:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
                  verifying_contract:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
            signature:
              description: >-
                There is no public key member because the public key can be
                recovered via `ecrecover()` knowing the data and the signature.
                Encoding: base58
              type: string
              pattern: '^secp256k1:'
            standard:
              type: string
              enum:
                - erc191
          additionalProperties: false
        - description: >-
            TIP-191: The standard for message signing in Tron. For more details,
            refer to
            [TIP-191](https://github.com/tronprotocol/tips/blob/master/tip-191.md).
          type: object
          required:
            - payload
            - signature
            - standard
          properties:
            payload:
              description: >-
                See
                [TIP-191](https://github.com/tronprotocol/tips/blob/master/tip-191.md)
              type: string
              x-parseJson:
                type: object
                required:
                  - deadline
                  - nonce
                  - signer_id
                  - verifying_contract
                properties:
                  deadline:
                    type: string
                  intents:
                    description: >-
                      Sequence of intents to execute in given order. Empty list
                      is also a valid sequence, i.e. it doesn't do anything, but
                      still invalidates the `nonce` for the signer WARNING:
                      Promises created by different intents are executed
                      concurrently and does not rely on the order of the intents
                      in this structure
                    type: array
                    items:
                      oneOf:
                        - description: See [`AddPublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - add_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`RemovePublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - remove_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`Transfer`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - transfer
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: See [`FtWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                            - token
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - ft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `ft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `ft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `ft_transfer_call`.
                                Otherwise, `ft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`NftWithdraw`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - token
                            - token_id
                          properties:
                            intent:
                              type: string
                              enum:
                                - nft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `nft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `nft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `nft_transfer_call`.
                                Otherwise, `nft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_id:
                              type: string
                          additionalProperties: false
                        - description: See [`MtWithdraw`]
                          type: object
                          required:
                            - amounts
                            - intent
                            - receiver_id
                            - token
                            - token_ids
                          properties:
                            amounts:
                              type: array
                              items:
                                type: string
                            intent:
                              type: string
                              enum:
                                - mt_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed per token: *
                                `mt_batch_transfer`:      minimum: 20TGas,
                                default: 20TGas * `mt_batch_transfer_call`:
                                minimum: 35TGas, default: 50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `mt_batch_transfer_call`.
                                Otherwise, `mt_batch_transfer` will be used.
                                NOTE: No refund will be made in case of
                                insufficient `storage_deposit` on `token` for
                                `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_ids:
                              type: array
                              items:
                                type: string
                          additionalProperties: false
                        - description: See [`NativeWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - native_withdraw
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`StorageDeposit`]
                          type: object
                          required:
                            - amount
                            - contract_id
                            - deposit_for_account_id
                            - intent
                          properties:
                            amount:
                              type: string
                            contract_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            deposit_for_account_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            intent:
                              type: string
                              enum:
                                - storage_deposit
                          additionalProperties: false
                        - description: See [`TokenDiff`]
                          type: object
                          required:
                            - diff
                            - intent
                          properties:
                            diff:
                              type: object
                              additionalProperties:
                                type: string
                            intent:
                              type: string
                              enum:
                                - token_diff
                            memo:
                              type:
                                - string
                                - 'null'
                            referral:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type:
                                - string
                                - 'null'
                          additionalProperties: false
                        - description: See [`SetAuthByPredecessorId`]
                          type: object
                          required:
                            - enabled
                            - intent
                          properties:
                            enabled:
                              type: boolean
                            intent:
                              type: string
                              enum:
                                - set_auth_by_predecessor_id
                          additionalProperties: false
                        - description: See [`AuthCall`]
                          type: object
                          required:
                            - contract_id
                            - intent
                            - msg
                          properties:
                            attached_deposit:
                              description: >-
                                Optionally, attach deposit to
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                call. The amount will be subtracted from user's
                                NEP-141 `wNEAR` balance.


                                NOTE: the `wNEAR` will not be refunded in case
                                of fail.
                              type: string
                            contract_id:
                              description: >-
                                Callee for
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            intent:
                              type: string
                              enum:
                                - auth_call
                            min_gas:
                              description: >-
                                Optional minimum gas required for created
                                promise to succeed. By default, only
                                [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                is required.


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                `msg` to pass in
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling
                                [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                (in the same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                          additionalProperties: false
                        - description: >-
                            Mint a set of tokens from the signer to a specified
                            account id, within the intents contract.
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_mint
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: Receiver of the minted tokens
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority (i.e. signer of this
                                intent). The final string representation of the
                                token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: >-
                            Burn a set of imt tokens, within the intents
                            contract.
                          type: object
                          required:
                            - intent
                            - minter_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_burn
                            memo:
                              type:
                                - string
                                - 'null'
                            minter_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority. The final string
                                representation of the token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                  nonce:
                    examples:
                      - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                    type: string
                    description: 'Encoding: base64'
                  signer_id:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
                  verifying_contract:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
            signature:
              description: >-
                There is no public key member because the public key can be
                recovered via `ecrecover()` knowing the data and the signature.
                Encoding: base58
              type: string
              pattern: '^secp256k1:'
            standard:
              type: string
              enum:
                - tip191
          additionalProperties: false
        - description: >-
            Raw Ed25519: The standard used by Solana Phantom wallets for message
            signing. For more details, refer to [Phantom Wallet's
            documentation](https://docs.phantom.com/solana/signing-a-message).
          type: object
          required:
            - payload
            - public_key
            - signature
            - standard
          properties:
            payload:
              type: string
              x-parseJson:
                type: object
                required:
                  - deadline
                  - nonce
                  - signer_id
                  - verifying_contract
                properties:
                  deadline:
                    type: string
                  intents:
                    description: >-
                      Sequence of intents to execute in given order. Empty list
                      is also a valid sequence, i.e. it doesn't do anything, but
                      still invalidates the `nonce` for the signer WARNING:
                      Promises created by different intents are executed
                      concurrently and does not rely on the order of the intents
                      in this structure
                    type: array
                    items:
                      oneOf:
                        - description: See [`AddPublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - add_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`RemovePublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - remove_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`Transfer`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - transfer
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: See [`FtWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                            - token
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - ft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `ft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `ft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `ft_transfer_call`.
                                Otherwise, `ft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`NftWithdraw`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - token
                            - token_id
                          properties:
                            intent:
                              type: string
                              enum:
                                - nft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `nft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `nft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `nft_transfer_call`.
                                Otherwise, `nft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_id:
                              type: string
                          additionalProperties: false
                        - description: See [`MtWithdraw`]
                          type: object
                          required:
                            - amounts
                            - intent
                            - receiver_id
                            - token
                            - token_ids
                          properties:
                            amounts:
                              type: array
                              items:
                                type: string
                            intent:
                              type: string
                              enum:
                                - mt_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed per token: *
                                `mt_batch_transfer`:      minimum: 20TGas,
                                default: 20TGas * `mt_batch_transfer_call`:
                                minimum: 35TGas, default: 50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `mt_batch_transfer_call`.
                                Otherwise, `mt_batch_transfer` will be used.
                                NOTE: No refund will be made in case of
                                insufficient `storage_deposit` on `token` for
                                `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_ids:
                              type: array
                              items:
                                type: string
                          additionalProperties: false
                        - description: See [`NativeWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - native_withdraw
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`StorageDeposit`]
                          type: object
                          required:
                            - amount
                            - contract_id
                            - deposit_for_account_id
                            - intent
                          properties:
                            amount:
                              type: string
                            contract_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            deposit_for_account_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            intent:
                              type: string
                              enum:
                                - storage_deposit
                          additionalProperties: false
                        - description: See [`TokenDiff`]
                          type: object
                          required:
                            - diff
                            - intent
                          properties:
                            diff:
                              type: object
                              additionalProperties:
                                type: string
                            intent:
                              type: string
                              enum:
                                - token_diff
                            memo:
                              type:
                                - string
                                - 'null'
                            referral:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type:
                                - string
                                - 'null'
                          additionalProperties: false
                        - description: See [`SetAuthByPredecessorId`]
                          type: object
                          required:
                            - enabled
                            - intent
                          properties:
                            enabled:
                              type: boolean
                            intent:
                              type: string
                              enum:
                                - set_auth_by_predecessor_id
                          additionalProperties: false
                        - description: See [`AuthCall`]
                          type: object
                          required:
                            - contract_id
                            - intent
                            - msg
                          properties:
                            attached_deposit:
                              description: >-
                                Optionally, attach deposit to
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                call. The amount will be subtracted from user's
                                NEP-141 `wNEAR` balance.


                                NOTE: the `wNEAR` will not be refunded in case
                                of fail.
                              type: string
                            contract_id:
                              description: >-
                                Callee for
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            intent:
                              type: string
                              enum:
                                - auth_call
                            min_gas:
                              description: >-
                                Optional minimum gas required for created
                                promise to succeed. By default, only
                                [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                is required.


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                `msg` to pass in
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling
                                [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                (in the same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                          additionalProperties: false
                        - description: >-
                            Mint a set of tokens from the signer to a specified
                            account id, within the intents contract.
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_mint
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: Receiver of the minted tokens
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority (i.e. signer of this
                                intent). The final string representation of the
                                token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: >-
                            Burn a set of imt tokens, within the intents
                            contract.
                          type: object
                          required:
                            - intent
                            - minter_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_burn
                            memo:
                              type:
                                - string
                                - 'null'
                            minter_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority. The final string
                                representation of the token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                  nonce:
                    examples:
                      - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                    type: string
                    description: 'Encoding: base64'
                  signer_id:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
                  verifying_contract:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
            public_key:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            signature:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            standard:
              type: string
              enum:
                - raw_ed25519
          additionalProperties: false
        - type: object
          required:
            - authenticator_data
            - client_data_json
            - payload
            - standard
            - public_key
            - signature
          properties:
            authenticator_data:
              description: >-
                Base64Url-encoded
                [authenticatorData](https://w3c.github.io/webauthn/#authenticator-data).
                Encoding: base64
              type: string
            client_data_json:
              description: >-
                Serialized
                [clientDataJSON](https://w3c.github.io/webauthn/#dom-authenticatorresponse-clientdatajson)
              type: string
            payload:
              type: string
              x-parseJson:
                type: object
                required:
                  - deadline
                  - nonce
                  - signer_id
                  - verifying_contract
                properties:
                  deadline:
                    type: string
                  intents:
                    description: >-
                      Sequence of intents to execute in given order. Empty list
                      is also a valid sequence, i.e. it doesn't do anything, but
                      still invalidates the `nonce` for the signer WARNING:
                      Promises created by different intents are executed
                      concurrently and does not rely on the order of the intents
                      in this structure
                    type: array
                    items:
                      oneOf:
                        - description: See [`AddPublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - add_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`RemovePublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - remove_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`Transfer`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - transfer
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: See [`FtWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                            - token
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - ft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `ft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `ft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `ft_transfer_call`.
                                Otherwise, `ft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`NftWithdraw`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - token
                            - token_id
                          properties:
                            intent:
                              type: string
                              enum:
                                - nft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `nft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `nft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `nft_transfer_call`.
                                Otherwise, `nft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_id:
                              type: string
                          additionalProperties: false
                        - description: See [`MtWithdraw`]
                          type: object
                          required:
                            - amounts
                            - intent
                            - receiver_id
                            - token
                            - token_ids
                          properties:
                            amounts:
                              type: array
                              items:
                                type: string
                            intent:
                              type: string
                              enum:
                                - mt_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed per token: *
                                `mt_batch_transfer`:      minimum: 20TGas,
                                default: 20TGas * `mt_batch_transfer_call`:
                                minimum: 35TGas, default: 50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `mt_batch_transfer_call`.
                                Otherwise, `mt_batch_transfer` will be used.
                                NOTE: No refund will be made in case of
                                insufficient `storage_deposit` on `token` for
                                `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_ids:
                              type: array
                              items:
                                type: string
                          additionalProperties: false
                        - description: See [`NativeWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - native_withdraw
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`StorageDeposit`]
                          type: object
                          required:
                            - amount
                            - contract_id
                            - deposit_for_account_id
                            - intent
                          properties:
                            amount:
                              type: string
                            contract_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            deposit_for_account_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            intent:
                              type: string
                              enum:
                                - storage_deposit
                          additionalProperties: false
                        - description: See [`TokenDiff`]
                          type: object
                          required:
                            - diff
                            - intent
                          properties:
                            diff:
                              type: object
                              additionalProperties:
                                type: string
                            intent:
                              type: string
                              enum:
                                - token_diff
                            memo:
                              type:
                                - string
                                - 'null'
                            referral:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type:
                                - string
                                - 'null'
                          additionalProperties: false
                        - description: See [`SetAuthByPredecessorId`]
                          type: object
                          required:
                            - enabled
                            - intent
                          properties:
                            enabled:
                              type: boolean
                            intent:
                              type: string
                              enum:
                                - set_auth_by_predecessor_id
                          additionalProperties: false
                        - description: See [`AuthCall`]
                          type: object
                          required:
                            - contract_id
                            - intent
                            - msg
                          properties:
                            attached_deposit:
                              description: >-
                                Optionally, attach deposit to
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                call. The amount will be subtracted from user's
                                NEP-141 `wNEAR` balance.


                                NOTE: the `wNEAR` will not be refunded in case
                                of fail.
                              type: string
                            contract_id:
                              description: >-
                                Callee for
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            intent:
                              type: string
                              enum:
                                - auth_call
                            min_gas:
                              description: >-
                                Optional minimum gas required for created
                                promise to succeed. By default, only
                                [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                is required.


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                `msg` to pass in
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling
                                [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                (in the same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                          additionalProperties: false
                        - description: >-
                            Mint a set of tokens from the signer to a specified
                            account id, within the intents contract.
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_mint
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: Receiver of the minted tokens
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority (i.e. signer of this
                                intent). The final string representation of the
                                token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: >-
                            Burn a set of imt tokens, within the intents
                            contract.
                          type: object
                          required:
                            - intent
                            - minter_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_burn
                            memo:
                              type:
                                - string
                                - 'null'
                            minter_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority. The final string
                                representation of the token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                  nonce:
                    examples:
                      - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                    type: string
                    description: 'Encoding: base64'
                  signer_id:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
                  verifying_contract:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
            standard:
              type: string
              enum:
                - webauthn
            public_key:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            signature:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
          description: >-
            [COSE EdDSA (-8)
            algorithm](https://www.iana.org/assignments/cose/cose.xhtml#algorithms):
            ed25519 curve
          additionalProperties: false
        - type: object
          required:
            - authenticator_data
            - client_data_json
            - payload
            - standard
            - public_key
            - signature
          properties:
            authenticator_data:
              description: >-
                Base64Url-encoded
                [authenticatorData](https://w3c.github.io/webauthn/#authenticator-data).
                Encoding: base64
              type: string
            client_data_json:
              description: >-
                Serialized
                [clientDataJSON](https://w3c.github.io/webauthn/#dom-authenticatorresponse-clientdatajson)
              type: string
            payload:
              type: string
              x-parseJson:
                type: object
                required:
                  - deadline
                  - nonce
                  - signer_id
                  - verifying_contract
                properties:
                  deadline:
                    type: string
                  intents:
                    description: >-
                      Sequence of intents to execute in given order. Empty list
                      is also a valid sequence, i.e. it doesn't do anything, but
                      still invalidates the `nonce` for the signer WARNING:
                      Promises created by different intents are executed
                      concurrently and does not rely on the order of the intents
                      in this structure
                    type: array
                    items:
                      oneOf:
                        - description: See [`AddPublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - add_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`RemovePublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - remove_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`Transfer`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - transfer
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: See [`FtWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                            - token
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - ft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `ft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `ft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `ft_transfer_call`.
                                Otherwise, `ft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`NftWithdraw`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - token
                            - token_id
                          properties:
                            intent:
                              type: string
                              enum:
                                - nft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `nft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `nft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `nft_transfer_call`.
                                Otherwise, `nft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_id:
                              type: string
                          additionalProperties: false
                        - description: See [`MtWithdraw`]
                          type: object
                          required:
                            - amounts
                            - intent
                            - receiver_id
                            - token
                            - token_ids
                          properties:
                            amounts:
                              type: array
                              items:
                                type: string
                            intent:
                              type: string
                              enum:
                                - mt_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed per token: *
                                `mt_batch_transfer`:      minimum: 20TGas,
                                default: 20TGas * `mt_batch_transfer_call`:
                                minimum: 35TGas, default: 50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `mt_batch_transfer_call`.
                                Otherwise, `mt_batch_transfer` will be used.
                                NOTE: No refund will be made in case of
                                insufficient `storage_deposit` on `token` for
                                `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_ids:
                              type: array
                              items:
                                type: string
                          additionalProperties: false
                        - description: See [`NativeWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - native_withdraw
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`StorageDeposit`]
                          type: object
                          required:
                            - amount
                            - contract_id
                            - deposit_for_account_id
                            - intent
                          properties:
                            amount:
                              type: string
                            contract_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            deposit_for_account_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            intent:
                              type: string
                              enum:
                                - storage_deposit
                          additionalProperties: false
                        - description: See [`TokenDiff`]
                          type: object
                          required:
                            - diff
                            - intent
                          properties:
                            diff:
                              type: object
                              additionalProperties:
                                type: string
                            intent:
                              type: string
                              enum:
                                - token_diff
                            memo:
                              type:
                                - string
                                - 'null'
                            referral:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type:
                                - string
                                - 'null'
                          additionalProperties: false
                        - description: See [`SetAuthByPredecessorId`]
                          type: object
                          required:
                            - enabled
                            - intent
                          properties:
                            enabled:
                              type: boolean
                            intent:
                              type: string
                              enum:
                                - set_auth_by_predecessor_id
                          additionalProperties: false
                        - description: See [`AuthCall`]
                          type: object
                          required:
                            - contract_id
                            - intent
                            - msg
                          properties:
                            attached_deposit:
                              description: >-
                                Optionally, attach deposit to
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                call. The amount will be subtracted from user's
                                NEP-141 `wNEAR` balance.


                                NOTE: the `wNEAR` will not be refunded in case
                                of fail.
                              type: string
                            contract_id:
                              description: >-
                                Callee for
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            intent:
                              type: string
                              enum:
                                - auth_call
                            min_gas:
                              description: >-
                                Optional minimum gas required for created
                                promise to succeed. By default, only
                                [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                is required.


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                `msg` to pass in
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling
                                [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                (in the same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                          additionalProperties: false
                        - description: >-
                            Mint a set of tokens from the signer to a specified
                            account id, within the intents contract.
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_mint
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: Receiver of the minted tokens
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority (i.e. signer of this
                                intent). The final string representation of the
                                token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: >-
                            Burn a set of imt tokens, within the intents
                            contract.
                          type: object
                          required:
                            - intent
                            - minter_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_burn
                            memo:
                              type:
                                - string
                                - 'null'
                            minter_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority. The final string
                                representation of the token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                  nonce:
                    examples:
                      - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                    type: string
                    description: 'Encoding: base64'
                  signer_id:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
                  verifying_contract:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
            standard:
              type: string
              enum:
                - webauthn
            public_key:
              type: string
              pattern: '^p256:'
              description: 'Encoding: base58'
            signature:
              type: string
              pattern: '^p256:'
              description: 'Encoding: base58'
          description: >-
            [COSE ES256 (-7)
            algorithm](https://www.iana.org/assignments/cose/cose.xhtml#algorithms):
            NIST P-256 curve (a.k.a secp256r1) over SHA-256
          additionalProperties: false
        - description: >-
            TonConnect: The standard for data signing in TON blockchain
            platform. For more details, refer to [TonConnect
            documentation](https://docs.tonconsole.com/academy/sign-data).
          type: object
          required:
            - address
            - domain
            - payload
            - public_key
            - signature
            - standard
            - timestamp
          properties:
            address:
              description: >-
                Wallet address in either
                [Raw](https://docs.ton.org/v3/documentation/smart-contracts/addresses/address-formats#raw-address)
                representation or
                [user-friendly](https://docs.ton.org/v3/documentation/smart-contracts/addresses/address-formats#user-friendly-address)
                format
              allOf:
                - type: string
            domain:
              description: dApp domain
              type: string
            payload:
              description: >-
                See
                <https://docs.tonconsole.com/academy/sign-data#choosing-the-right-format>
              oneOf:
                - type: object
                  required:
                    - text
                    - type
                  properties:
                    text:
                      type: string
                      x-parseJson:
                        type: object
                        required:
                          - deadline
                          - nonce
                          - signer_id
                          - verifying_contract
                        properties:
                          deadline:
                            type: string
                          intents:
                            description: >-
                              Sequence of intents to execute in given order.
                              Empty list is also a valid sequence, i.e. it
                              doesn't do anything, but still invalidates the
                              `nonce` for the signer WARNING: Promises created
                              by different intents are executed concurrently and
                              does not rely on the order of the intents in this
                              structure
                            type: array
                            items:
                              oneOf:
                                - description: See [`AddPublicKey`]
                                  type: object
                                  required:
                                    - intent
                                    - public_key
                                  properties:
                                    intent:
                                      type: string
                                      enum:
                                        - add_public_key
                                    public_key:
                                      examples:
                                        - >-
                                          ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                        - >-
                                          secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                                      type: string
                                      description: 'Encoding: base58'
                                  additionalProperties: false
                                - description: See [`RemovePublicKey`]
                                  type: object
                                  required:
                                    - intent
                                    - public_key
                                  properties:
                                    intent:
                                      type: string
                                      enum:
                                        - remove_public_key
                                    public_key:
                                      examples:
                                        - >-
                                          ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                        - >-
                                          secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                                      type: string
                                      description: 'Encoding: base58'
                                  additionalProperties: false
                                - description: See [`Transfer`]
                                  type: object
                                  required:
                                    - intent
                                    - receiver_id
                                    - tokens
                                  properties:
                                    intent:
                                      type: string
                                      enum:
                                        - transfer
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    min_gas:
                                      description: >-
                                        Minimum gas for `mt_on_transfer()`


                                        Remaining gas will be distributed evenly
                                        across all Function Call Promises
                                        created during execution of current
                                        receipt.
                                      type:
                                        - string
                                        - 'null'
                                    msg:
                                      description: Message to pass to `mt_on_transfer`
                                      type: string
                                    receiver_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    state_init:
                                      description: >-
                                        Optionally initialize the receiver's
                                        contract (Deterministic AccountId) via
                                        [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                        right before calling `mt_on_transfer()`
                                        (in the same receipt).
                                      anyOf:
                                        - oneOf:
                                            - type: object
                                              required:
                                                - code
                                                - data
                                                - version
                                              properties:
                                                code:
                                                  oneOf:
                                                    - type: object
                                                      required:
                                                        - hash
                                                      properties:
                                                        hash:
                                                          type: string
                                                      additionalProperties: false
                                                    - type: object
                                                      required:
                                                        - account_id
                                                      properties:
                                                        account_id:
                                                          description: >-
                                                            NEAR Account Identifier.


                                                            This is a unique, syntactically valid,
                                                            human-readable account identifier on the
                                                            NEAR network.


                                                            [See the crate-level docs for
                                                            information about
                                                            validation.](index.html#account-id-rules)


                                                            Also see [Error kind
                                                            precedence](AccountId#error-kind-precedence).


                                                            ## Examples


                                                            ``` use near_account_id::AccountId;


                                                            let alice: AccountId =
                                                            "alice.near".parse().unwrap();


                                                            assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                            // (ƒ is not f) ```
                                                          type: string
                                                      additionalProperties: false
                                                data:
                                                  type: object
                                                  additionalProperties:
                                                    type: string
                                                version:
                                                  type: string
                                                  enum:
                                                    - v1
                                              additionalProperties: false
                                        - type: 'null'
                                    tokens:
                                      type: object
                                      additionalProperties:
                                        type: string
                                  additionalProperties: false
                                - description: See [`FtWithdraw`]
                                  type: object
                                  required:
                                    - amount
                                    - intent
                                    - receiver_id
                                    - token
                                  properties:
                                    amount:
                                      type: string
                                    intent:
                                      type: string
                                      enum:
                                        - ft_withdraw
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    min_gas:
                                      description: >-
                                        Optional minimum required Near gas for
                                        created Promise to succeed: *
                                        `ft_transfer`:      minimum: 15TGas,
                                        default: 15TGas * `ft_transfer_call`:
                                        minimum: 30TGas, default: 50TGas


                                        Remaining gas will be distributed evenly
                                        across all Function Call Promises
                                        created during execution of current
                                        receipt.
                                      type:
                                        - string
                                        - 'null'
                                    msg:
                                      description: >-
                                        Message to pass to `ft_transfer_call`.
                                        Otherwise, `ft_transfer` will be used.
                                        NOTE: No refund will be made in case of
                                        insufficient `storage_deposit` on
                                        `token` for `receiver_id`
                                      type:
                                        - string
                                        - 'null'
                                    receiver_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    storage_deposit:
                                      description: >-
                                        Optionally make `storage_deposit` for
                                        `receiver_id` on `token`. The amount
                                        will be subtracted from user's NEP-141
                                        `wNEAR` balance. NOTE: the `wNEAR` will
                                        not be refunded in case of fail
                                      type:
                                        - string
                                        - 'null'
                                    token:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                  additionalProperties: false
                                - description: See [`NftWithdraw`]
                                  type: object
                                  required:
                                    - intent
                                    - receiver_id
                                    - token
                                    - token_id
                                  properties:
                                    intent:
                                      type: string
                                      enum:
                                        - nft_withdraw
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    min_gas:
                                      description: >-
                                        Optional minimum required Near gas for
                                        created Promise to succeed: *
                                        `nft_transfer`:      minimum: 15TGas,
                                        default: 15TGas * `nft_transfer_call`:
                                        minimum: 30TGas, default: 50TGas


                                        Remaining gas will be distributed evenly
                                        across all Function Call Promises
                                        created during execution of current
                                        receipt.
                                      type:
                                        - string
                                        - 'null'
                                    msg:
                                      description: >-
                                        Message to pass to `nft_transfer_call`.
                                        Otherwise, `nft_transfer` will be used.
                                        NOTE: No refund will be made in case of
                                        insufficient `storage_deposit` on
                                        `token` for `receiver_id`
                                      type:
                                        - string
                                        - 'null'
                                    receiver_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    storage_deposit:
                                      description: >-
                                        Optionally make `storage_deposit` for
                                        `receiver_id` on `token`. The amount
                                        will be subtracted from user's NEP-141
                                        `wNEAR` balance. NOTE: the `wNEAR` will
                                        not be refunded in case of fail
                                      type:
                                        - string
                                        - 'null'
                                    token:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    token_id:
                                      type: string
                                  additionalProperties: false
                                - description: See [`MtWithdraw`]
                                  type: object
                                  required:
                                    - amounts
                                    - intent
                                    - receiver_id
                                    - token
                                    - token_ids
                                  properties:
                                    amounts:
                                      type: array
                                      items:
                                        type: string
                                    intent:
                                      type: string
                                      enum:
                                        - mt_withdraw
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    min_gas:
                                      description: >-
                                        Optional minimum required Near gas for
                                        created Promise to succeed per token: *
                                        `mt_batch_transfer`:      minimum:
                                        20TGas, default: 20TGas *
                                        `mt_batch_transfer_call`: minimum:
                                        35TGas, default: 50TGas


                                        Remaining gas will be distributed evenly
                                        across all Function Call Promises
                                        created during execution of current
                                        receipt.
                                      type:
                                        - string
                                        - 'null'
                                    msg:
                                      description: >-
                                        Message to pass to
                                        `mt_batch_transfer_call`. Otherwise,
                                        `mt_batch_transfer` will be used. NOTE:
                                        No refund will be made in case of
                                        insufficient `storage_deposit` on
                                        `token` for `receiver_id`
                                      type:
                                        - string
                                        - 'null'
                                    receiver_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    storage_deposit:
                                      description: >-
                                        Optionally make `storage_deposit` for
                                        `receiver_id` on `token`. The amount
                                        will be subtracted from user's NEP-141
                                        `wNEAR` balance. NOTE: the `wNEAR` will
                                        not be refunded in case of fail
                                      type:
                                        - string
                                        - 'null'
                                    token:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    token_ids:
                                      type: array
                                      items:
                                        type: string
                                  additionalProperties: false
                                - description: See [`NativeWithdraw`]
                                  type: object
                                  required:
                                    - amount
                                    - intent
                                    - receiver_id
                                  properties:
                                    amount:
                                      type: string
                                    intent:
                                      type: string
                                      enum:
                                        - native_withdraw
                                    receiver_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                  additionalProperties: false
                                - description: See [`StorageDeposit`]
                                  type: object
                                  required:
                                    - amount
                                    - contract_id
                                    - deposit_for_account_id
                                    - intent
                                  properties:
                                    amount:
                                      type: string
                                    contract_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    deposit_for_account_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    intent:
                                      type: string
                                      enum:
                                        - storage_deposit
                                  additionalProperties: false
                                - description: See [`TokenDiff`]
                                  type: object
                                  required:
                                    - diff
                                    - intent
                                  properties:
                                    diff:
                                      type: object
                                      additionalProperties:
                                        type: string
                                    intent:
                                      type: string
                                      enum:
                                        - token_diff
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    referral:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type:
                                        - string
                                        - 'null'
                                  additionalProperties: false
                                - description: See [`SetAuthByPredecessorId`]
                                  type: object
                                  required:
                                    - enabled
                                    - intent
                                  properties:
                                    enabled:
                                      type: boolean
                                    intent:
                                      type: string
                                      enum:
                                        - set_auth_by_predecessor_id
                                  additionalProperties: false
                                - description: See [`AuthCall`]
                                  type: object
                                  required:
                                    - contract_id
                                    - intent
                                    - msg
                                  properties:
                                    attached_deposit:
                                      description: >-
                                        Optionally, attach deposit to
                                        [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                        call. The amount will be subtracted from
                                        user's NEP-141 `wNEAR` balance.


                                        NOTE: the `wNEAR` will not be refunded
                                        in case of fail.
                                      type: string
                                    contract_id:
                                      description: >-
                                        Callee for
                                        [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                      type: string
                                    intent:
                                      type: string
                                      enum:
                                        - auth_call
                                    min_gas:
                                      description: >-
                                        Optional minimum gas required for
                                        created promise to succeed. By default,
                                        only
                                        [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                        is required.


                                        Remaining gas will be distributed evenly
                                        across all Function Call Promises
                                        created during execution of current
                                        receipt.
                                      type:
                                        - string
                                        - 'null'
                                    msg:
                                      description: >-
                                        `msg` to pass in
                                        [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                      type: string
                                    state_init:
                                      description: >-
                                        Optionally initialize the receiver's
                                        contract (Deterministic AccountId) via
                                        [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                        right before calling
                                        [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                        (in the same receipt).
                                      anyOf:
                                        - oneOf:
                                            - type: object
                                              required:
                                                - code
                                                - data
                                                - version
                                              properties:
                                                code:
                                                  oneOf:
                                                    - type: object
                                                      required:
                                                        - hash
                                                      properties:
                                                        hash:
                                                          type: string
                                                      additionalProperties: false
                                                    - type: object
                                                      required:
                                                        - account_id
                                                      properties:
                                                        account_id:
                                                          description: >-
                                                            NEAR Account Identifier.


                                                            This is a unique, syntactically valid,
                                                            human-readable account identifier on the
                                                            NEAR network.


                                                            [See the crate-level docs for
                                                            information about
                                                            validation.](index.html#account-id-rules)


                                                            Also see [Error kind
                                                            precedence](AccountId#error-kind-precedence).


                                                            ## Examples


                                                            ``` use near_account_id::AccountId;


                                                            let alice: AccountId =
                                                            "alice.near".parse().unwrap();


                                                            assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                            // (ƒ is not f) ```
                                                          type: string
                                                      additionalProperties: false
                                                data:
                                                  type: object
                                                  additionalProperties:
                                                    type: string
                                                version:
                                                  type: string
                                                  enum:
                                                    - v1
                                              additionalProperties: false
                                        - type: 'null'
                                  additionalProperties: false
                                - description: >-
                                    Mint a set of tokens from the signer to a
                                    specified account id, within the intents
                                    contract.
                                  type: object
                                  required:
                                    - intent
                                    - receiver_id
                                    - tokens
                                  properties:
                                    intent:
                                      type: string
                                      enum:
                                        - imt_mint
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    min_gas:
                                      description: >-
                                        Minimum gas for `mt_on_transfer()`


                                        Remaining gas will be distributed evenly
                                        across all Function Call Promises
                                        created during execution of current
                                        receipt.
                                      type:
                                        - string
                                        - 'null'
                                    msg:
                                      description: Message to pass to `mt_on_transfer`
                                      type: string
                                    receiver_id:
                                      description: Receiver of the minted tokens
                                      type: string
                                    state_init:
                                      description: >-
                                        Optionally initialize the receiver's
                                        contract (Deterministic AccountId) via
                                        [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                        right before calling `mt_on_transfer()`
                                        (in the same receipt).
                                      anyOf:
                                        - oneOf:
                                            - type: object
                                              required:
                                                - code
                                                - data
                                                - version
                                              properties:
                                                code:
                                                  oneOf:
                                                    - type: object
                                                      required:
                                                        - hash
                                                      properties:
                                                        hash:
                                                          type: string
                                                      additionalProperties: false
                                                    - type: object
                                                      required:
                                                        - account_id
                                                      properties:
                                                        account_id:
                                                          description: >-
                                                            NEAR Account Identifier.


                                                            This is a unique, syntactically valid,
                                                            human-readable account identifier on the
                                                            NEAR network.


                                                            [See the crate-level docs for
                                                            information about
                                                            validation.](index.html#account-id-rules)


                                                            Also see [Error kind
                                                            precedence](AccountId#error-kind-precedence).


                                                            ## Examples


                                                            ``` use near_account_id::AccountId;


                                                            let alice: AccountId =
                                                            "alice.near".parse().unwrap();


                                                            assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                            // (ƒ is not f) ```
                                                          type: string
                                                      additionalProperties: false
                                                data:
                                                  type: object
                                                  additionalProperties:
                                                    type: string
                                                version:
                                                  type: string
                                                  enum:
                                                    - v1
                                              additionalProperties: false
                                        - type: 'null'
                                    tokens:
                                      description: >-
                                        The token_ids will be wrapped to bind
                                        the token ID to the minter authority
                                        (i.e. signer of this intent). The final
                                        string representation of the token will
                                        be as follows:
                                        `imt:<minter_id>:<token_id>`
                                      type: object
                                      additionalProperties:
                                        type: string
                                  additionalProperties: false
                                - description: >-
                                    Burn a set of imt tokens, within the intents
                                    contract.
                                  type: object
                                  required:
                                    - intent
                                    - minter_id
                                    - tokens
                                  properties:
                                    intent:
                                      type: string
                                      enum:
                                        - imt_burn
                                    memo:
                                      type:
                                        - string
                                        - 'null'
                                    minter_id:
                                      description: >-
                                        NEAR Account Identifier.


                                        This is a unique, syntactically valid,
                                        human-readable account identifier on the
                                        NEAR network.


                                        [See the crate-level docs for
                                        information about
                                        validation.](index.html#account-id-rules)


                                        Also see [Error kind
                                        precedence](AccountId#error-kind-precedence).


                                        ## Examples


                                        ``` use near_account_id::AccountId;


                                        let alice: AccountId =
                                        "alice.near".parse().unwrap();


                                        assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                        // (ƒ is not f) ```
                                      type: string
                                    tokens:
                                      description: >-
                                        The token_ids will be wrapped to bind
                                        the token ID to the minter authority.
                                        The final string representation of the
                                        token will be as follows:
                                        `imt:<minter_id>:<token_id>`
                                      type: object
                                      additionalProperties:
                                        type: string
                                  additionalProperties: false
                          nonce:
                            examples:
                              - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                            type: string
                            description: 'Encoding: base64'
                          signer_id:
                            description: >-
                              NEAR Account Identifier.


                              This is a unique, syntactically valid,
                              human-readable account identifier on the NEAR
                              network.


                              [See the crate-level docs for information about
                              validation.](index.html#account-id-rules)


                              Also see [Error kind
                              precedence](AccountId#error-kind-precedence).


                              ## Examples


                              ``` use near_account_id::AccountId;


                              let alice: AccountId =
                              "alice.near".parse().unwrap();


                              assert!("ƒelicia.near".parse::<AccountId>().is_err());
                              // (ƒ is not f) ```
                            type: string
                          verifying_contract:
                            description: >-
                              NEAR Account Identifier.


                              This is a unique, syntactically valid,
                              human-readable account identifier on the NEAR
                              network.


                              [See the crate-level docs for information about
                              validation.](index.html#account-id-rules)


                              Also see [Error kind
                              precedence](AccountId#error-kind-precedence).


                              ## Examples


                              ``` use near_account_id::AccountId;


                              let alice: AccountId =
                              "alice.near".parse().unwrap();


                              assert!("ƒelicia.near".parse::<AccountId>().is_err());
                              // (ƒ is not f) ```
                            type: string
                    type:
                      type: string
                      enum:
                        - text
            public_key:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            signature:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            standard:
              type: string
              enum:
                - ton_connect
            timestamp:
              description: UNIX timestamp (in seconds or RFC3339) at the time of singing
              allOf:
                - anyOf:
                    - type: string
                      format: date-time
                    - writeOnly: true
                      allOf:
                        - type: integer
                          format: int64
          additionalProperties: false
        - description: >-
            SEP-53: The standard for signing data off-chain for Stellar
            accounts. See
            [SEP-53](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0053.md)
          type: object
          required:
            - payload
            - public_key
            - signature
            - standard
          properties:
            payload:
              type: string
              x-parseJson:
                type: object
                required:
                  - deadline
                  - nonce
                  - signer_id
                  - verifying_contract
                properties:
                  deadline:
                    type: string
                  intents:
                    description: >-
                      Sequence of intents to execute in given order. Empty list
                      is also a valid sequence, i.e. it doesn't do anything, but
                      still invalidates the `nonce` for the signer WARNING:
                      Promises created by different intents are executed
                      concurrently and does not rely on the order of the intents
                      in this structure
                    type: array
                    items:
                      oneOf:
                        - description: See [`AddPublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - add_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`RemovePublicKey`]
                          type: object
                          required:
                            - intent
                            - public_key
                          properties:
                            intent:
                              type: string
                              enum:
                                - remove_public_key
                            public_key:
                              examples:
                                - >-
                                  ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm
                                - >-
                                  secp256k1:3aMVMxsoAnHUbweXMtdKaN1uJaNwsfKv7wnc97SDGjXhyK62VyJwhPUPLZefKVthcoUcuWK6cqkSU4M542ipNxS3
                              type: string
                              description: 'Encoding: base58'
                          additionalProperties: false
                        - description: See [`Transfer`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - transfer
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: See [`FtWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                            - token
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - ft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `ft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `ft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `ft_transfer_call`.
                                Otherwise, `ft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`NftWithdraw`]
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - token
                            - token_id
                          properties:
                            intent:
                              type: string
                              enum:
                                - nft_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed: * `nft_transfer`:     
                                minimum: 15TGas, default: 15TGas *
                                `nft_transfer_call`: minimum: 30TGas, default:
                                50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `nft_transfer_call`.
                                Otherwise, `nft_transfer` will be used. NOTE: No
                                refund will be made in case of insufficient
                                `storage_deposit` on `token` for `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_id:
                              type: string
                          additionalProperties: false
                        - description: See [`MtWithdraw`]
                          type: object
                          required:
                            - amounts
                            - intent
                            - receiver_id
                            - token
                            - token_ids
                          properties:
                            amounts:
                              type: array
                              items:
                                type: string
                            intent:
                              type: string
                              enum:
                                - mt_withdraw
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Optional minimum required Near gas for created
                                Promise to succeed per token: *
                                `mt_batch_transfer`:      minimum: 20TGas,
                                default: 20TGas * `mt_batch_transfer_call`:
                                minimum: 35TGas, default: 50TGas


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                Message to pass to `mt_batch_transfer_call`.
                                Otherwise, `mt_batch_transfer` will be used.
                                NOTE: No refund will be made in case of
                                insufficient `storage_deposit` on `token` for
                                `receiver_id`
                              type:
                                - string
                                - 'null'
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            storage_deposit:
                              description: >-
                                Optionally make `storage_deposit` for
                                `receiver_id` on `token`. The amount will be
                                subtracted from user's NEP-141 `wNEAR` balance.
                                NOTE: the `wNEAR` will not be refunded in case
                                of fail
                              type:
                                - string
                                - 'null'
                            token:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            token_ids:
                              type: array
                              items:
                                type: string
                          additionalProperties: false
                        - description: See [`NativeWithdraw`]
                          type: object
                          required:
                            - amount
                            - intent
                            - receiver_id
                          properties:
                            amount:
                              type: string
                            intent:
                              type: string
                              enum:
                                - native_withdraw
                            receiver_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                          additionalProperties: false
                        - description: See [`StorageDeposit`]
                          type: object
                          required:
                            - amount
                            - contract_id
                            - deposit_for_account_id
                            - intent
                          properties:
                            amount:
                              type: string
                            contract_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            deposit_for_account_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            intent:
                              type: string
                              enum:
                                - storage_deposit
                          additionalProperties: false
                        - description: See [`TokenDiff`]
                          type: object
                          required:
                            - diff
                            - intent
                          properties:
                            diff:
                              type: object
                              additionalProperties:
                                type: string
                            intent:
                              type: string
                              enum:
                                - token_diff
                            memo:
                              type:
                                - string
                                - 'null'
                            referral:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type:
                                - string
                                - 'null'
                          additionalProperties: false
                        - description: See [`SetAuthByPredecessorId`]
                          type: object
                          required:
                            - enabled
                            - intent
                          properties:
                            enabled:
                              type: boolean
                            intent:
                              type: string
                              enum:
                                - set_auth_by_predecessor_id
                          additionalProperties: false
                        - description: See [`AuthCall`]
                          type: object
                          required:
                            - contract_id
                            - intent
                            - msg
                          properties:
                            attached_deposit:
                              description: >-
                                Optionally, attach deposit to
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                                call. The amount will be subtracted from user's
                                NEP-141 `wNEAR` balance.


                                NOTE: the `wNEAR` will not be refunded in case
                                of fail.
                              type: string
                            contract_id:
                              description: >-
                                Callee for
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            intent:
                              type: string
                              enum:
                                - auth_call
                            min_gas:
                              description: >-
                                Optional minimum gas required for created
                                promise to succeed. By default, only
                                [`MIN_GAS_DEFAULT`](AuthCall::MIN_GAS_DEFAULT)
                                is required.


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: >-
                                `msg` to pass in
                                [`.on_auth`](::defuse_auth_call::AuthCallee::on_auth)
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling
                                [`.on_auth()`](::defuse_auth_call::AuthCallee::on_auth)
                                (in the same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                          additionalProperties: false
                        - description: >-
                            Mint a set of tokens from the signer to a specified
                            account id, within the intents contract.
                          type: object
                          required:
                            - intent
                            - receiver_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_mint
                            memo:
                              type:
                                - string
                                - 'null'
                            min_gas:
                              description: >-
                                Minimum gas for `mt_on_transfer()`


                                Remaining gas will be distributed evenly across
                                all Function Call Promises created during
                                execution of current receipt.
                              type:
                                - string
                                - 'null'
                            msg:
                              description: Message to pass to `mt_on_transfer`
                              type: string
                            receiver_id:
                              description: Receiver of the minted tokens
                              type: string
                            state_init:
                              description: >-
                                Optionally initialize the receiver's contract
                                (Deterministic AccountId) via
                                [`state_init`](https://github.com/near/NEPs/blob/master/neps/nep-0616.md#stateinit-action)
                                right before calling `mt_on_transfer()` (in the
                                same receipt).
                              anyOf:
                                - oneOf:
                                    - type: object
                                      required:
                                        - code
                                        - data
                                        - version
                                      properties:
                                        code:
                                          oneOf:
                                            - type: object
                                              required:
                                                - hash
                                              properties:
                                                hash:
                                                  type: string
                                              additionalProperties: false
                                            - type: object
                                              required:
                                                - account_id
                                              properties:
                                                account_id:
                                                  description: >-
                                                    NEAR Account Identifier.


                                                    This is a unique, syntactically valid,
                                                    human-readable account identifier on the
                                                    NEAR network.


                                                    [See the crate-level docs for
                                                    information about
                                                    validation.](index.html#account-id-rules)


                                                    Also see [Error kind
                                                    precedence](AccountId#error-kind-precedence).


                                                    ## Examples


                                                    ``` use near_account_id::AccountId;


                                                    let alice: AccountId =
                                                    "alice.near".parse().unwrap();


                                                    assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                                    // (ƒ is not f) ```
                                                  type: string
                                              additionalProperties: false
                                        data:
                                          type: object
                                          additionalProperties:
                                            type: string
                                        version:
                                          type: string
                                          enum:
                                            - v1
                                      additionalProperties: false
                                - type: 'null'
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority (i.e. signer of this
                                intent). The final string representation of the
                                token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                        - description: >-
                            Burn a set of imt tokens, within the intents
                            contract.
                          type: object
                          required:
                            - intent
                            - minter_id
                            - tokens
                          properties:
                            intent:
                              type: string
                              enum:
                                - imt_burn
                            memo:
                              type:
                                - string
                                - 'null'
                            minter_id:
                              description: >-
                                NEAR Account Identifier.


                                This is a unique, syntactically valid,
                                human-readable account identifier on the NEAR
                                network.


                                [See the crate-level docs for information about
                                validation.](index.html#account-id-rules)


                                Also see [Error kind
                                precedence](AccountId#error-kind-precedence).


                                ## Examples


                                ``` use near_account_id::AccountId;


                                let alice: AccountId =
                                "alice.near".parse().unwrap();


                                assert!("ƒelicia.near".parse::<AccountId>().is_err());
                                // (ƒ is not f) ```
                              type: string
                            tokens:
                              description: >-
                                The token_ids will be wrapped to bind the token
                                ID to the minter authority. The final string
                                representation of the token will be as follows:
                                `imt:<minter_id>:<token_id>`
                              type: object
                              additionalProperties:
                                type: string
                          additionalProperties: false
                  nonce:
                    examples:
                      - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
                    type: string
                    description: 'Encoding: base64'
                  signer_id:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
                  verifying_contract:
                    description: >-
                      NEAR Account Identifier.


                      This is a unique, syntactically valid, human-readable
                      account identifier on the NEAR network.


                      [See the crate-level docs for information about
                      validation.](index.html#account-id-rules)


                      Also see [Error kind
                      precedence](AccountId#error-kind-precedence).


                      ## Examples


                      ``` use near_account_id::AccountId;


                      let alice: AccountId = "alice.near".parse().unwrap();


                      assert!("ƒelicia.near".parse::<AccountId>().is_err()); //
                      (ƒ is not f) ```
                    type: string
            public_key:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            signature:
              type: string
              pattern: '^ed25519:'
              description: 'Encoding: base58'
            standard:
              type: string
              enum:
                - sep53
          additionalProperties: false

````