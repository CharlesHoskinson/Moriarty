> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Check swap execution status

> Retrieves the current status of a swap using the unique deposit address from the quote, if quote response included deposit memo, it is required as well.

The response includes the state of the swap (e.g., pending, processing, success, refunded) and any associated swap and transaction details.



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml get /v0/status
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
  /v0/status:
    get:
      tags:
        - OneClick
      summary: Check swap execution status
      description: >-
        Retrieves the current status of a swap using the unique deposit address
        from the quote, if quote response included deposit memo, it is required
        as well.


        The response includes the state of the swap (e.g., pending, processing,
        success, refunded) and any associated swap and transaction details.
      operationId: getExecutionStatus
      parameters:
        - name: depositAddress
          required: true
          in: query
          schema:
            type: string
        - name: depositMemo
          required: false
          in: query
          schema:
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/GetExecutionStatusResponse'
        '401':
          description: Unauthorized - API key or JWT token is invalid
        '404':
          description: Deposit address not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/BadRequestResponse'
      security:
        - X-API-Key: []
        - JWT-auth: []
components:
  schemas:
    GetExecutionStatusResponse:
      type: object
      properties:
        correlationId:
          type: string
          description: Unique identifier for request tracing and debugging
          example: 550e8400-e29b-41d4-a716-446655440000
        quoteResponse:
          description: Quote response from the original request
          allOf:
            - $ref: '#/components/schemas/QuoteResponse'
        status:
          type: string
          enum:
            - KNOWN_DEPOSIT_TX
            - PENDING_DEPOSIT
            - INCOMPLETE_DEPOSIT
            - PROCESSING
            - SUCCESS
            - REFUNDED
            - FAILED
        updatedAt:
          format: date-time
          type: string
          description: Last time the state was updated
        swapDetails:
          description: Details of actual swaps and withdrawals
          allOf:
            - $ref: '#/components/schemas/SwapDetails'
      required:
        - correlationId
        - quoteResponse
        - status
        - updatedAt
        - swapDetails
    BadRequestResponse:
      type: object
      properties:
        message:
          type: string
          example: error message
      required:
        - message
    QuoteResponse:
      type: object
      properties:
        correlationId:
          type: string
          description: Unique identifier for request tracing and debugging
          example: 550e8400-e29b-41d4-a716-446655440000
        timestamp:
          format: date-time
          type: string
          description: Timestamp in ISO format that was used to derive the deposit address
          example: '2019-08-24T14:15:22Z'
        signature:
          type: string
          description: >-
            Signature of the 1Click service confirming the quote for the
            specific deposit address. Must be saved on the client side (along
            with the whole quote) in order to resolve any disputes or mistakes.
        quoteRequest:
          description: User request
          allOf:
            - $ref: '#/components/schemas/QuoteRequest'
        quote:
          description: >-
            Response containing the deposit address for sending the `amount` of
            `originAsset` and the expected output amount.
          allOf:
            - $ref: '#/components/schemas/Quote'
      required:
        - correlationId
        - timestamp
        - signature
        - quoteRequest
        - quote
    SwapDetails:
      type: object
      properties:
        intentHashes:
          description: All intent hashes that took part in this swap
          type: array
          items:
            type: string
        nearTxHashes:
          description: All NEAR transactions executed for this swap
          type: array
          items:
            type: string
        amountIn:
          type: string
          description: Exact amount of `originToken` after the trade was settled
          example: '1000'
        amountInFormatted:
          type: string
          description: >-
            Exact amount of `originToken` in readable format after the trade was
            settled
          example: '0.1'
        amountInUsd:
          type: string
          description: Exact amount of `originToken` equivalent in USD
          example: '0.1'
        amountOut:
          type: string
          description: Exact amount of `destinationToken` after the trade was settled
          example: '9950000'
        amountOutFormatted:
          type: string
          description: >-
            Exact amount of `destinationToken` in readable format after the
            trade was settled
          example: '9.95'
        amountOutUsd:
          type: string
          description: Exact amount of `destinationToken` equivalent in USD
          example: '9.95'
        slippage:
          type: number
          description: Actual slippage
          example: 50
        originChainTxHashes:
          description: Hashes and explorer URLs for all transactions on the origin chain
          type: array
          items:
            $ref: '#/components/schemas/TransactionDetails'
        destinationChainTxHashes:
          description: >-
            Hashes and explorer URLs for all transactions on the destination
            chain
          type: array
          items:
            $ref: '#/components/schemas/TransactionDetails'
        refundedAmount:
          type: string
          description: Amount of `originAsset` transferred to `refundTo`
          example: '1000'
        refundedAmountFormatted:
          type: string
          description: Refunded amount in readable format
          example: '0.1'
        refundedAmountUsd:
          type: string
          description: Refunded amount equivalent in USD
          example: '0.1'
        refundReason:
          type: string
          description: Reason for refund
          example: PARTIAL_DEPOSIT
        depositedAmount:
          type: string
          description: Amount deposited to `depositAddress` onchain
        depositedAmountFormatted:
          type: string
          description: Amount deposited in readable format
        depositedAmountUsd:
          type: string
          description: Amount deposited equivalent in USD
        withdrawFee:
          type: string
          description: >-
            Fee charged for withdrawing assets to the recipient in the smallest
            unit of the destination asset. This fee is already accounted for in
            the final amountOut result
          example: '10000'
        referral:
          type: string
          description: Referral identifier
          example: referral
      required:
        - intentHashes
        - nearTxHashes
        - originChainTxHashes
        - destinationChainTxHashes
    QuoteRequest:
      type: object
      properties:
        dry:
          type: boolean
          description: |-
            Flag indicating whether this is a dry run request.
            If `true`, the response will **NOT** contain the following fields:
            - `depositAddress`
            - `timeWhenInactive`
            - `deadline`
          example: true
        depositMode:
          type: string
          description: >-
            What deposit address mode you will get in the response, most chain
            supports only `SIMPLE` and some(for example `stellar`) only `MEMO`:

            - `SIMPLE` - usual deposit with only deposit address.

            - `MEMO` - some chains will **REQUIRE** the `memo` together with
            `depositAddress` for swap to work.


            `depositMode` applies only to deposit transaction metadata.
          enum:
            - SIMPLE
            - MEMO
          default: SIMPLE
          example: SIMPLE
        swapType:
          type: string
          description: >-
            How to interpret `amount` (and refunds) when performing the swap:


            - `EXACT_INPUT` — requests the output amount for an exact input.
              - If deposit is less than `amountIn`, the deposit is refunded by deadline.
              - If deposit is above than `amountIn`, the swap is processed and the excess is refunded to `refundTo` address after swap is complete.

            - `EXACT_OUTPUT` — requests the input amount for an exact output.
              - The output amount (`amountOut`) is fixed; slippage is applied to the **input** side.
              - The quote response includes `amountIn` (the proposed input with slippage tolerance baked in) and `minAmountIn` (the minimum input required).
              - **Important**: `amountIn` will appear higher than `minAmountIn` by approximately your `slippageTolerance`. This is **not** price degradation—it is a buffer to ensure successful execution. Any input amount above what is actually needed for the swap is **refunded** to your `refundTo` address after the swap completes.
              - If the deposit is above `amountIn`, the swap is processed and the excess is refunded to `refundTo` address after swap is complete.
              - If the deposit is less than `minAmountIn`, the deposit is refunded by deadline.

            - `FLEX_INPUT` — a flexible input amount that allows for partial
            deposits and variable amounts.
              - `slippage` applies both to `amountOut` and `amountIn` and defines an acceptable range (`minAmountIn` and `minAmountOut`).
              - Any amount higher than `minAmountIn` is accepted and converted to the output asset as long as `minAmountOut` is met.
              - The deposit must be at least `minAmountIn`, after applying the slippage constraint to the quoted amount. Deposits below `minAmountIn` are refunded if the total received by the deadline is below this amount.
              - If deposits exceed the upper bound, the swap is still processed

            - `ANY_INPUT` — collects deposits and converts them to a destination
            asset. Used for fee aggregation and similar use cases. Only
            available for authorized partners.
          enum:
            - EXACT_INPUT
            - EXACT_OUTPUT
            - FLEX_INPUT
            - ANY_INPUT
        slippageTolerance:
          type: number
          description: >-
            Slippage tolerance for the swap. This value is in basis points
            (1/100th of a percent), e.g. 100 for 1% slippage.
          example: 100
        originAsset:
          type: string
          description: ID of the origin asset.
          example: nep141:arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near
        depositType:
          type: string
          description: >-
            Type of deposit address:

            - `ORIGIN_CHAIN` - deposit address on the origin chain.

            - `INTENTS` - the account ID within NEAR Intents to which you should
            transfer assets.

            - `CONFIDENTIAL_INTENTS` - the account ID within Confidential
            Intents to which assets are to be transferred. Fund the swap from a
            Confidential Intents account by submitting a signed transfer intent
            to the quote `depositAddress`. Direct token transfers are not
            supported.
          enum:
            - ORIGIN_CHAIN
            - INTENTS
            - CONFIDENTIAL_INTENTS
        destinationAsset:
          type: string
          description: ID of the destination asset.
          example: nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near
        amount:
          type: string
          description: >-
            Amount to swap as the base amount. It is interpreted as the input or
            output amount based on the `swapType` flag and is specified in the
            smallest unit of the currency (e.g., wei for ETH). Must be an
            integer string; decimal values like "0.01" are invalid—use base
            units (e.g. "10000000000000000" for 0.01 with 18 decimals).
          example: '1000'
        refundTo:
          type: string
          description: Address used for refunds.
          example: '0x2527D02599Ba641c19FEa793cD0F167589a0f10D'
        refundType:
          type: string
          description: >-
            Type of refund address:

            - `ORIGIN_CHAIN` - assets are refunded to the `refundTo` address on
            the origin chain.

            - `INTENTS` - assets are refunded to the `refundTo` Intents account.

            - `CONFIDENTIAL_INTENTS` - assets are refunded to the `refundTo`
            Confidential Intents account. On Confidential Intents, 1Click
            settles refunds via signed transfer intent rather than direct token
            transfer.
          enum:
            - ORIGIN_CHAIN
            - INTENTS
            - CONFIDENTIAL_INTENTS
        recipient:
          type: string
          description: >-
            Recipient address. The format must match `recipientType`.
            Destination routing metadata, when required, must be encoded
            according to the destination chain address format.
          example: 13QkxhNMrTPxoCkRdYdJ65tFuwXPhL5gLS2Z5Nr6gjRK
        connectedWallets:
          description: Addresses of connected wallets.
          example:
            - 0x123...
            - 0x456...
          type: array
          items:
            type: string
        sessionId:
          type: string
          description: Unique client session identifier for 1Click.
          example: session_abc123
        virtualChainRecipient:
          type: string
          description: EVM address of a transfer recipient in a virtual chain
          example: '0xb4c2fbec9d610F9A3a9b843c47b1A8095ceC887C'
        virtualChainRefundRecipient:
          type: string
          description: EVM address of a refund recipient in a virtual chain
          example: '0xb4c2fbec9d610F9A3a9b843c47b1A8095ceC887C'
        customRecipientMsg:
          type: string
          description: >-
            **HIGHLY EXPERIMENTAL** Message to pass to `ft_transfer_call` when
            withdrawing assets to NEAR.


            Otherwise, `ft_transfer` will be used.


            **WARNING**: Funds will be lost if used with non NEP-141 tokens, in
            case of insufficient `storage_deposit` or if the recipient does not
            implement `ft_on_transfer` method.
          example: smart-contract-recipient.near
        recipientType:
          type: string
          description: >-
            Type of recipient address:

            - `DESTINATION_CHAIN` - assets are transferred to the chain of
            `destinationAsset`.

            - `INTENTS` - assets are transferred to an account inside Intents.

            - `CONFIDENTIAL_INTENTS` - assets are transferred to the `recipient`
            account inside Confidential Intents. On Confidential Intents, 1Click
            settles swap output via signed transfer intent rather than direct
            token transfer.
          enum:
            - DESTINATION_CHAIN
            - INTENTS
            - CONFIDENTIAL_INTENTS
        deadline:
          format: date-time
          type: string
          description: >-
            Timestamp in ISO format that identifies when the user refund begins
            if the swap isn't completed by then. It must exceed the time
            required for the deposit transaction to be mined. For example,
            Bitcoin may require around one hour depending on the fees paid.
          example: '2019-08-24T14:15:22Z'
        confidentiality:
          type: string
          description: >-
            Confidentiality mode for this quote. PUBLIC is the default; BASIC or
            ADVANCED request Confidential Intents handling.
          enum:
            - public
            - basic
            - advanced
          default: public
          example: basic
        referral:
          type: string
          description: >-
            Distribution channel / venue identifier (for example: ledger,
            trustwallet). Keep lowercase for consistency.
          example: ledger
        rebates:
          description: >-
            Rebate distribution for this quote. Provide up to 3 confidential
            Intents recipients (for example, `rebate-recipient.near` on
            confidential Intents); each item defines a recipient and its percent
            share, and all shares must add up to 100.
          example:
            - recipient: rebate-recipient.near
              share: 20
            - recipient: partner-2.near
              share: 80
          type: array
          items:
            $ref: '#/components/schemas/Rebate'
        quoteWaitingTimeMs:
          type: number
          description: >-
            Time in milliseconds the user is willing to wait for a quote from
            the relay.

            **Defaults to 0ms delay if not specified, if you want to receive the
            fastest quote.
          example: 3000
          default: 0
        appFees:
          description: List of recipients and their fees
          type: array
          items:
            $ref: '#/components/schemas/AppFee'
        insured:
          type: boolean
          description: >-
            Opt into an insured swap. When `true`, an additional 0.02% (2 basis
            points) insurance fee is charged on top of any other fees.
          default: false
          example: false
      required:
        - dry
        - swapType
        - slippageTolerance
        - originAsset
        - depositType
        - destinationAsset
        - amount
        - refundTo
        - refundType
        - recipient
        - recipientType
        - deadline
    Quote:
      type: object
      properties:
        depositAddress:
          type: string
          description: >-
            The deposit address on the chain of `originAsset` when `depositType`
            is `ORIGIN_CHAIN`.


            The deposit address inside NEAR Intents (the verifier smart
            contract) when `depositType` is `INTENTS`.


            The account ID within Confidential Intents to which assets are to be
            transferred when `depositType` is `CONFIDENTIAL_INTENTS`. Fund the
            swap by submitting a signed transfer intent to this value. Direct
            token transfers are not supported.
          example: '0x76b4c56085ED136a8744D52bE956396624a730E8'
        depositMemo:
          type: string
          description: >-
            Some deposit addresses **REQUIRE** a `memo` together with
            `depositAddress` for the deposit to be processed. This field is
            deposit-side metadata.
          example: '1111111'
        chainDepositAddresses:
          description: >-
            Deposit addresses across all bridge-supported blockchains for
            funding the same Intents account. Present only for public
            `ANY_INPUT` quotes (`depositType` `INTENTS`); the confidential rail
            does not support multi-chain funding addresses yet. Each entry
            forwards to the Intents account in `depositAddress`, so the user may
            deposit from any listed chain.
          type: array
          items:
            $ref: '#/components/schemas/ChainDepositAddress'
        amountIn:
          type: string
          description: Amount of the origin asset
          example: '1000000'
        amountInFormatted:
          type: string
          description: Amount of the origin asset in readable format
          example: '1'
        amountInUsd:
          type: string
          description: Amount of the origin assets equivalent in USD
          example: '1'
        minAmountIn:
          type: string
          description: Minimum amount of the origin asset that will be used for the swap
          example: '995000'
        amountOut:
          type: string
          description: Amount of the destination asset
          example: '9950000'
        amountOutFormatted:
          type: string
          description: Amount of the destination asset in readable format
          example: '9.95'
        amountOutUsd:
          type: string
          description: Amount of the destination asset equivalent in USD
          example: '9.95'
        minAmountOut:
          type: string
          description: Minimum output amount after slippage is applied
          example: '9900000'
        deadline:
          format: date-time
          type: string
          description: Time when the deposit address becomes inactive and funds may be lost
          example: '2025-03-04T15:00:00Z'
        timeWhenInactive:
          format: date-time
          type: string
          description: >-
            Time when the deposit address becomes cold, causing swap processing
            to take longer
          example: '2025-03-04T15:00:00Z'
        timeEstimate:
          type: number
          description: >-
            Estimated time in seconds for the swap to be executed after the
            deposit transaction is confirmed
          example: 120
        virtualChainRecipient:
          type: string
          description: EVM address of a transfer recipient in a virtual chain
          example: '0xb4c2fbec9d610F9A3a9b843c47b1A8095ceC887C'
        virtualChainRefundRecipient:
          type: string
          description: EVM address of a refund recipient in a virtual chain
          example: '0xb4c2fbec9d610F9A3a9b843c47b1A8095ceC887C'
        customRecipientMsg:
          type: string
          description: >-
            **HIGHLY EXPERIMENTAL** Message passed to `ft_transfer_call` when
            withdrawing assets to NEAR.


            Otherwise, `ft_transfer` will be used.


            **WARNING**: Funds will be lost if used with non NEP-141 tokens, in
            case of insufficient `storage_deposit` or if the recipient does not
            implement `ft_on_transfer` method.
          example: smart-contract-recipient.near
        refundFee:
          type: string
          description: >-
            Fee charged for refunding assets to the refund address in the
            smallest unit of the origin asset
          example: '10000'
        withdrawFee:
          type: string
          description: >-
            Fee charged for withdrawing assets to the recipient in the smallest
            unit of the destination asset. This fee is already accounted for in
            the final amountOut result
          example: '10000'
      required:
        - amountIn
        - amountInFormatted
        - amountInUsd
        - minAmountIn
        - amountOut
        - amountOutFormatted
        - amountOutUsd
        - minAmountOut
        - timeEstimate
    TransactionDetails:
      type: object
      properties:
        hash:
          type: string
          description: Transaction hash
          example: '0x123abc456def789'
        explorerUrl:
          type: string
          description: Explorer URL for the transaction
      required:
        - hash
        - explorerUrl
    Rebate:
      type: object
      properties:
        recipient:
          type: string
          description: >-
            Confidential Intents account identifier that receives this rebate
            share. Rebate delivery uses signed transfer intent on Confidential
            Intents; direct token transfers are not supported.
          example: rebate-recipient.near
        share:
          type: number
          description: Rebate share for this recipient in percent.
          example: 20
          minimum: 0
          maximum: 100
      required:
        - recipient
        - share
    AppFee:
      type: object
      properties:
        recipient:
          type: string
          description: Account ID within Intents to which this fee will be transferred
          example: recipient.near
        fee:
          type: number
          description: >-
            Fee for this recipient as part of amountIn in basis points (1/100th
            of a percent), e.g. 100 for 1% fee
          example: 100
      required:
        - recipient
        - fee
    ChainDepositAddress:
      type: object
      properties:
        blockchain:
          type: string
          description: >-
            Blockchain on which this deposit address accepts funds for the
            quote.
          enum:
            - near
            - eth
            - base
            - arb
            - btc
            - sol
            - ton
            - dash
            - doge
            - xrp
            - zec
            - gnosis
            - bera
            - bsc
            - pol
            - tron
            - sui
            - movement
            - op
            - avax
            - stellar
            - aptos
            - cardano
            - ltc
            - xlayer
            - monad
            - bch
            - adi
            - plasma
            - scroll
            - starknet
            - aleo
            - hypercore
            - fogo
          example: eth
        address:
          type: string
          description: >-
            Deposit address on `blockchain`. Funds sent here are credited to the
            same Intents account.
          example: '0x76b4c56085ED136a8744D52bE956396624a730E8'
        memo:
          type: string
          description: >-
            Memo required together with `address` on chains that need it (e.g.
            Stellar, XRP, TON).
          example: '1111111'
      required:
        - blockchain
        - address
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