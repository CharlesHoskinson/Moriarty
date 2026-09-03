> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Submit a signed intent

> Submits a signed intent to execute.

  After generating an intent for a quote or limit order via `/generate-intent` and having the user sign it with their wallet, submit the signed intent through this endpoint.

  The system validates the signature, processes the intent, and returns the intent hash upon successful submission.

  **Request Type Variants:**
  - `swap_transfer`: Submit a signed swap intent (requires signedData object)



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml post /v0/submit-intent
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
  /v0/submit-intent:
    post:
      tags:
        - OneClick
      summary: Submit a signed intent
      description: |-
        Submits a signed intent to execute.

          After generating an intent for a quote or limit order via `/generate-intent` and having the user sign it with their wallet, submit the signed intent through this endpoint.

          The system validates the signature, processes the intent, and returns the intent hash upon successful submission.

          **Request Type Variants:**
          - `swap_transfer`: Submit a signed swap intent (requires signedData object)
      operationId: submitIntent
      parameters: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              oneOf:
                - $ref: '#/components/schemas/SubmitSwapTransferIntentRequest'
              discriminator:
                propertyName: type
                mapping:
                  swap_transfer:
                    $ref: '#/components/schemas/SubmitSwapTransferIntentRequest'
      responses:
        '200':
          description: Successfully submitted intent
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/SubmitIntentResponse'
        '400':
          description: Bad Request - Invalid signature or intent data
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/BadRequestResponse'
        '401':
          description: Unauthorized - API key or JWT token is invalid
      security:
        - X-API-Key: []
        - JWT-auth: []
components:
  schemas:
    SubmitSwapTransferIntentRequest:
      type: object
      properties:
        type:
          type: string
          description: The type of intent action
          enum:
            - swap_transfer
        signedData:
          oneOf:
            - $ref: '#/components/schemas/MultiPayload'
      required:
        - type
        - signedData
    SubmitIntentResponse:
      type: object
      properties:
        intentHash:
          type: string
          description: The hash of the submitted intent
          example: 44XpLRAuZKoVGs9T4qbSNv33MDKMePPAibA52geVLWFw
        correlationId:
          type: string
          description: Unique identifier for tracing this request
          example: 550e8400-e29b-41d4-a716-446655440000
      required:
        - intentHash
        - correlationId
    BadRequestResponse:
      type: object
      properties:
        message:
          type: string
          example: error message
      required:
        - message
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
  securitySchemes:
    X-API-Key:
      type: apiKey
      in: header
      name: X-API-Key
      description: API key for partner authentication (recommended)
    JWT-auth:
      scheme: bearer
      bearerFormat: JWT
      type: http
      description: JWT token in Authorization header (legacy, use X-API-Key instead)

````