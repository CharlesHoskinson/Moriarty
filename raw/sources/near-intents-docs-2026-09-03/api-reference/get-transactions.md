> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Get transactions



## OpenAPI

````yaml https://explorer.near-intents.org/api/v0/openapi.yaml get /api/v0/transactions
openapi: 3.0.0
info:
  version: 0.0.0
  title: Intents Explorer API
  description: >-
    API for Intents Explorer. All endpoints require JWT authentication.


    ## Rate Limiting


    All API endpoints are rate-limited per partner. Each partner can make a
    maximum of **1 request every 5 seconds**. Rate limits are enforced per
    partner ID (extracted from the JWT token).


    If you exceed the rate limit, you will receive a `429 Too Many Requests`
    response with details about how long to wait before making another request.
servers: []
security:
  - bearerAuth: []
paths:
  /api/v0/transactions:
    get:
      summary: Get transactions
      parameters:
        - schema:
            type: number
            minimum: 1
            maximum: 1000
            description: >-
              The number of transactions to return. Default: 50. Max: 1000. Min:
              1.
          required: false
          description: >-
            The number of transactions to return. Default: 50. Max: 1000. Min:
            1.
          name: numberOfTransactions
          in: query
        - schema:
            type: string
            description: >-
              DEPRECATED: Use lastDepositAddress and lastDepositMemo instead.
              Combined deposit address and memo value for pagination.
          required: false
          description: >-
            DEPRECATED: Use lastDepositAddress and lastDepositMemo instead.
            Combined deposit address and memo value for pagination.
          name: lastDepositAddressAndMemo
          in: query
        - schema:
            type: string
            description: >-
              Last known deposit address for pagination. Use the depositAddress
              from the last transaction of the previous page.
          required: false
          description: >-
            Last known deposit address for pagination. Use the depositAddress
            from the last transaction of the previous page.
          name: lastDepositAddress
          in: query
        - schema:
            type: string
            description: >-
              Last known deposit memo for pagination. Use the depositMemo from
              the last transaction of the previous page.
          required: false
          description: >-
            Last known deposit memo for pagination. Use the depositMemo from the
            last transaction of the previous page.
          name: lastDepositMemo
          in: query
        - schema:
            type: string
            enum:
              - next
              - prev
            description: >-
              Direction of pagination. `next` - get older transactions, `prev` -
              get newer transactions. Default: `next`.
          required: false
          description: >-
            Direction of pagination. `next` - get older transactions, `prev` -
            get newer transactions. Default: `next`.
          name: direction
          in: query
        - schema:
            type: string
            description: Search by deposit address, recipient, sender, or tx hash.
          required: false
          description: Search by deposit address, recipient, sender, or tx hash.
          name: search
          in: query
        - schema:
            anyOf:
              - type: string
                enum:
                  - near
                  - eth
                  - base
                  - arb
                  - btc
                  - sol
                  - ton
                  - doge
                  - xrp
                  - zec
                  - gnosis
                  - bera
                  - bsc
                  - pol
                  - tron
                  - sui
                  - op
                  - avax
                  - cardano
                  - stellar
                  - aptos
                  - ltc
                  - monad
                  - xlayer
                  - starknet
                  - bch
                  - adi
                  - plasma
                  - scroll
                  - aleo
                  - movement
                  - dash
                  - abs
                  - hypercore
                  - fogo
              - type: array
                items:
                  type: string
                  enum:
                    - near
                    - eth
                    - base
                    - arb
                    - btc
                    - sol
                    - ton
                    - doge
                    - xrp
                    - zec
                    - gnosis
                    - bera
                    - bsc
                    - pol
                    - tron
                    - sui
                    - op
                    - avax
                    - cardano
                    - stellar
                    - aptos
                    - ltc
                    - monad
                    - xlayer
                    - starknet
                    - bch
                    - adi
                    - plasma
                    - scroll
                    - aleo
                    - movement
                    - dash
                    - abs
                    - hypercore
                    - fogo
            description: >-
              Filter by origin chain IDs. Repeat the parameter to pass multiple
              ids (matched as OR). Swap-specific: setting this excludes masking
              transactions.
          required: false
          description: >-
            Filter by origin chain IDs. Repeat the parameter to pass multiple
            ids (matched as OR). Swap-specific: setting this excludes masking
            transactions.
          name: fromChainId
          in: query
        - schema:
            anyOf:
              - type: string
              - type: array
                items:
                  type: string
            description: >-
              Filter by origin token IDs. Repeat the parameter to pass multiple
              ids (matched as OR). Overrides fromChainId. Swap-specific: setting
              this excludes masking transactions.
          required: false
          description: >-
            Filter by origin token IDs. Repeat the parameter to pass multiple
            ids (matched as OR). Overrides fromChainId. Swap-specific: setting
            this excludes masking transactions.
          name: fromTokenId
          in: query
        - schema:
            anyOf:
              - type: string
                enum:
                  - near
                  - eth
                  - base
                  - arb
                  - btc
                  - sol
                  - ton
                  - doge
                  - xrp
                  - zec
                  - gnosis
                  - bera
                  - bsc
                  - pol
                  - tron
                  - sui
                  - op
                  - avax
                  - cardano
                  - stellar
                  - aptos
                  - ltc
                  - monad
                  - xlayer
                  - starknet
                  - bch
                  - adi
                  - plasma
                  - scroll
                  - aleo
                  - movement
                  - dash
                  - abs
                  - hypercore
                  - fogo
              - type: array
                items:
                  type: string
                  enum:
                    - near
                    - eth
                    - base
                    - arb
                    - btc
                    - sol
                    - ton
                    - doge
                    - xrp
                    - zec
                    - gnosis
                    - bera
                    - bsc
                    - pol
                    - tron
                    - sui
                    - op
                    - avax
                    - cardano
                    - stellar
                    - aptos
                    - ltc
                    - monad
                    - xlayer
                    - starknet
                    - bch
                    - adi
                    - plasma
                    - scroll
                    - aleo
                    - movement
                    - dash
                    - abs
                    - hypercore
                    - fogo
            description: >-
              Filter by destination chain IDs. Repeat the parameter to pass
              multiple ids (matched as OR). Swap-specific: setting this excludes
              masking transactions.
          required: false
          description: >-
            Filter by destination chain IDs. Repeat the parameter to pass
            multiple ids (matched as OR). Swap-specific: setting this excludes
            masking transactions.
          name: toChainId
          in: query
        - schema:
            anyOf:
              - type: string
              - type: array
                items:
                  type: string
            description: >-
              Filter by destination token IDs. Repeat the parameter to pass
              multiple ids (matched as OR). Overrides toChainId. Swap-specific:
              setting this excludes masking transactions.
          required: false
          description: >-
            Filter by destination token IDs. Repeat the parameter to pass
            multiple ids (matched as OR). Overrides toChainId. Swap-specific:
            setting this excludes masking transactions.
          name: toTokenId
          in: query
        - schema:
            type: string
            description: Deposit memo to narrow search by deposit address.
          required: false
          description: Deposit memo to narrow search by deposit address.
          name: depositMemo
          in: query
        - schema:
            type: string
            description: >-
              Filter by referral. Swap-specific: setting this excludes masking
              transactions.
          required: false
          description: >-
            Filter by referral. Swap-specific: setting this excludes masking
            transactions.
          name: referral
          in: query
        - schema:
            type: string
            description: >-
              Filter by an affiliate stored in the fees. Swap-specific: setting
              this excludes masking transactions.
          required: false
          description: >-
            Filter by an affiliate stored in the fees. Swap-specific: setting
            this excludes masking transactions.
          name: affiliate
          in: query
        - schema:
            type: string
            description: >-
              Filter by comma-separated statuses. Allowed values: FAILED,
              INCOMPLETE_DEPOSIT, PENDING_DEPOSIT, PROCESSING, REFUNDED,
              SUCCESS. Default: FAILED, PROCESSING, REFUNDED, SUCCESS.
          required: false
          description: >-
            Filter by comma-separated statuses. Allowed values: FAILED,
            INCOMPLETE_DEPOSIT, PENDING_DEPOSIT, PROCESSING, REFUNDED, SUCCESS.
            Default: FAILED, PROCESSING, REFUNDED, SUCCESS.
          name: statuses
          in: query
        - schema:
            type: string
            description: >-
              Show transactions from test accounts and with test-related
              referrals.
          required: false
          description: >-
            Show transactions from test accounts and with test-related
            referrals.
          name: showTestTxs
          in: query
        - schema:
            type: number
            nullable: true
            minimum: 0
            description: >-
              Filter by minimum USD price for both input and output amounts.
              Swap-specific: setting this excludes masking transactions.
          required: false
          description: >-
            Filter by minimum USD price for both input and output amounts.
            Swap-specific: setting this excludes masking transactions.
          name: minUsdPrice
          in: query
        - schema:
            type: number
            nullable: true
            minimum: 0
            description: >-
              Filter by maximum USD price for both input and output amounts.
              Swap-specific: setting this excludes masking transactions.
          required: false
          description: >-
            Filter by maximum USD price for both input and output amounts.
            Swap-specific: setting this excludes masking transactions.
          name: maxUsdPrice
          in: query
        - schema:
            type: string
            nullable: true
            format: date-time
            description: Filter by end timestamp (non-inclusive). Use ISO 8601 format.
          required: false
          description: Filter by end timestamp (non-inclusive). Use ISO 8601 format.
          name: endTimestamp
          in: query
        - schema:
            type: number
            nullable: true
            description: Filter by end timestamp (non-inclusive). Use Unix timestamp.
          required: false
          description: Filter by end timestamp (non-inclusive). Use Unix timestamp.
          name: endTimestampUnix
          in: query
        - schema:
            type: string
            nullable: true
            format: date-time
            description: Filter by start timestamp (non-inclusive). Use ISO 8601 format.
          required: false
          description: Filter by start timestamp (non-inclusive). Use ISO 8601 format.
          name: startTimestamp
          in: query
        - schema:
            type: number
            nullable: true
            description: Filter by start timestamp (non-inclusive). Use Unix timestamp.
          required: false
          description: Filter by start timestamp (non-inclusive). Use Unix timestamp.
          name: startTimestampUnix
          in: query
      responses:
        '200':
          description: Transactions
          content:
            application/json:
              schema:
                type: array
                items:
                  type: object
                  properties:
                    originAsset:
                      type: string
                      example: >-
                        nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near
                    destinationAsset:
                      type: string
                      example: >-
                        nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1
                    depositAddress:
                      type: string
                      example: GDJ4JZXZELZD737NVFORH4PSSQDWFDZTKW3AIDKHYQG23ZXBPDGGQBJK
                      description: The deposit address.
                    depositMemo:
                      type: string
                      nullable: true
                      example: '12345'
                      description: The deposit memo, if it exists.
                    depositAddressAndMemo:
                      type: string
                      example: >-
                        GDJ4JZXZELZD737NVFORH4PSSQDWFDZTKW3AIDKHYQG23ZXBPDGGQBJK_12345
                      description: >-
                        DEPRECATED: Use depositAddress and depositMemo instead.
                        Combined deposit address and memo.
                    recipient:
                      type: string
                      example: somebody.near
                    status:
                      type: string
                      enum:
                        - SUCCESS
                        - FAILED
                        - INCOMPLETE_DEPOSIT
                        - PENDING_DEPOSIT
                        - PROCESSING
                        - REFUNDED
                      example: SUCCESS
                    createdAt:
                      type: string
                      example: '2025-12-31T12:00:00.000Z'
                    createdAtTimestamp:
                      type: number
                      example: 1767182400
                    intentHashes:
                      type: string
                      nullable: true
                      example: GnGk38hvi92tTWDYMMS8CWYnVT4fixmfBrnqSErCDMTu
                    referral:
                      type: string
                      nullable: true
                      example: some-referral
                    amountInFormatted:
                      type: string
                      example: '22.130108'
                      description: >-
                        The same amount as amountIn, converted to the origin
                        asset's decimals.
                    amountOutFormatted:
                      type: string
                      example: '22.113697'
                      description: >-
                        The same amount as amountOut, converted to the
                        destination asset's decimals.
                    appFees:
                      type: array
                      items:
                        type: object
                        properties:
                          recipient:
                            type: string
                            example: some.near
                          fee:
                            type: number
                            example: 50
                        required:
                          - recipient
                          - fee
                      example:
                        - fee: 50
                          recipient: some.near
                        - fee: 5
                          recipient: somebody.near
                    nearTxHashes:
                      type: array
                      items:
                        type: string
                      example:
                        - 6XqqDwoaopgg39QsEiFGs9HfwP2Vum9tCCyqHDYXWBBH
                        - EVcgKukwf38XsYcgvkEgiMPWR7qwLfLK5rsVtjgctPBn
                    originChainTxHashes:
                      type: array
                      items:
                        type: string
                      example:
                        - >-
                          0x9bcff372aee89b648c922b850573b22387c31d693079f5e37cd255814e2d615a
                    destinationChainTxHashes:
                      type: array
                      items:
                        type: string
                      example:
                        - >-
                          0x9bcff372aee89b648c922b850573b22387c31d693079f5e37cd255814e2d615a
                    amountIn:
                      type: string
                      example: '22130108'
                      description: >-
                        Amount swapped, in the origin asset's smallest unit.
                        Reads 0 while the swap is in flight, because publishing
                        the size of an unfinished swap invites front-running.
                    amountInUsd:
                      type: string
                      example: '22.1272'
                      description: USD value of amountIn.
                    amountOut:
                      type: string
                      example: '22113697'
                      description: >-
                        Amount the recipient receives, in the destination
                        asset's smallest unit. Reads 0 while the swap is in
                        flight, see amountIn.
                    amountOutUsd:
                      type: string
                      example: '22.1108'
                      description: USD value of amountOut.
                    refundTo:
                      type: string
                      example: somebody.near
                    senders:
                      type: array
                      items:
                        type: string
                      example:
                        - '0x1234567890abcdef1234567890abcdef12345678'
                    refundReason:
                      type: string
                      nullable: true
                      example: AMOUNT_MORE_THAN_BALANCE
                      description: >-
                        Known values: AMOUNT_LESS_THAN_MIN_AMOUNT_OUT,
                        AMOUNT_MORE_THAN_BALANCE, INTENT_SUBMIT_FAILED,
                        PARTIAL_DEPOSIT, CIRCUIT_BREAKER_BLOCKED,
                        FEE_ESTIMATION_FAILED, NO_LIQUIDITY,
                        AMOUNT_LESS_THAN_MIN_WITHDRAWABLE, INSUFFICIENT_BALANCE,
                        REFUND_ADD_FAILED, UNKNOWN
                    refundFee:
                      type: string
                      nullable: true
                      example: '1000'
                    refundFeeFormatted:
                      type: string
                      nullable: true
                      example: '0.001'
                    depositType:
                      type: string
                      enum:
                        - ORIGIN_CHAIN
                        - INTENTS
                        - CONFIDENTIAL_INTENTS
                      example: ORIGIN_CHAIN
                    recipientType:
                      type: string
                      enum:
                        - DESTINATION_CHAIN
                        - INTENTS
                        - CONFIDENTIAL_INTENTS
                      example: DESTINATION_CHAIN
                    refundType:
                      type: string
                      enum:
                        - ORIGIN_CHAIN
                        - INTENTS
                        - CONFIDENTIAL_INTENTS
                      example: ORIGIN_CHAIN
                  required:
                    - originAsset
                    - destinationAsset
                    - depositAddress
                    - depositMemo
                    - depositAddressAndMemo
                    - recipient
                    - status
                    - createdAt
                    - createdAtTimestamp
                    - intentHashes
                    - referral
                    - amountInFormatted
                    - amountOutFormatted
                    - appFees
                    - nearTxHashes
                    - originChainTxHashes
                    - destinationChainTxHashes
                    - amountIn
                    - amountInUsd
                    - amountOut
                    - amountOutUsd
                    - refundTo
                    - senders
                    - refundFeeFormatted
                    - depositType
                    - recipientType
                    - refundType
        '400':
          description: Bad Request
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: number
                    enum:
                      - 400
                  error:
                    type: object
                    properties:
                      code:
                        type: string
                      message:
                        type: string
                    required:
                      - code
                      - message
                required:
                  - status
                  - error
        '401':
          description: Unauthorized
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: number
                    enum:
                      - 400
                  error:
                    type: object
                    properties:
                      code:
                        type: string
                      message:
                        type: string
                    required:
                      - code
                      - message
                required:
                  - status
                  - error
        '429':
          description: >-
            Too Many Requests - Rate limit exceeded. Maximum 1 request per 5
            seconds per partner.
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: number
                    enum:
                      - 429
                  error:
                    type: object
                    properties:
                      code:
                        type: string
                      message:
                        type: string
                    required:
                      - code
                      - message
                required:
                  - status
                  - error
      security:
        - bearerAuth: []
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
      description: JWT token for API authentication.

````