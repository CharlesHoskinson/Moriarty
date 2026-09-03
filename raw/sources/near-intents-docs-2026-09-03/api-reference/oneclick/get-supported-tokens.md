> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Get supported tokens

> Retrieves a list of tokens currently supported by the 1Click API for asset swaps.

Each token entry includes its blockchain, contract address (if available), price in USD, and other metadata such as symbol and decimals.



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml get /v0/tokens
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
  /v0/tokens:
    get:
      tags:
        - OneClick
      summary: Get supported tokens
      description: >-
        Retrieves a list of tokens currently supported by the 1Click API for
        asset swaps.


        Each token entry includes its blockchain, contract address (if
        available), price in USD, and other metadata such as symbol and
        decimals.
      operationId: getTokens
      parameters:
        - name: ondoTokens
          required: false
          in: query
          description: Show Ondo tokens if the query is set
          schema:
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/TokenResponse'
components:
  schemas:
    TokenResponse:
      type: object
      properties:
        assetId:
          type: string
          description: Unique asset identifier
          example: nep141:wrap.near
        decimals:
          type: number
          description: Number of decimals for the token
          example: 24
        blockchain:
          type: string
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
          description: Blockchain associated with the token
          example: near
        symbol:
          type: string
          description: Token symbol (e.g. BTC, ETH)
          example: wNEAR
        price:
          type: number
          description: Current price of the token in USD
          example: '2.79'
        priceUpdatedAt:
          format: date-time
          type: string
          description: Date when the token price was last updated
          example: '2025-03-28T12:23:00.070Z'
        contractAddress:
          type: string
          description: Contract address of the token
          example: wrap.near
        coingeckoId:
          type: string
          description: >-
            CoinGecko coin id, when known. Use it to fetch price history from
            CoinGecko.
          example: wrapped-near
      required:
        - assetId
        - decimals
        - blockchain
        - symbol
        - price
        - priceUpdatedAt

````