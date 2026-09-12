> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Get ANY_INPUT withdrawals

> Retrieves all withdrawals by ANY_INPUT quote with filtering, pagination and sorting



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml get /v0/any-input/withdrawals
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
  /v0/any-input/withdrawals:
    get:
      tags:
        - OneClick
      summary: Get ANY_INPUT withdrawals
      description: >-
        Retrieves all withdrawals by ANY_INPUT quote with filtering, pagination
        and sorting
      operationId: getAnyInputQuoteWithdrawals
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
        - name: timestampFrom
          required: false
          in: query
          description: Filter withdrawals from this timestamp (ISO string)
          schema:
            type: string
        - name: page
          required: false
          in: query
          description: 'Page number for pagination (default: 1)'
          schema:
            type: number
        - name: limit
          required: false
          in: query
          description: 'Number of withdrawals per page (max: 50, default: 50)'
          schema:
            type: number
        - name: sortOrder
          required: false
          in: query
          description: Sort order
          schema:
            enum:
              - asc
              - desc
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/GetAnyInputQuoteWithdrawals'
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
    GetAnyInputQuoteWithdrawals:
      type: object
      properties:
        asset:
          type: string
          description: ID of the destination asset.
          example: nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near
        recipient:
          type: string
          description: Recipient address
          example: '0x2527d02599ba641c19fea793cd0f167589a0f10d'
        affiliateRecipient:
          type: string
          description: Affiliate recipient address
          example: '0x2527d02599ba641c19fea793cd0f167589a0f10d'
        withdrawals:
          description: Details of withdrawals
          allOf:
            - $ref: '#/components/schemas/AnyInputQuoteWithdrawal'
      required:
        - asset
        - recipient
        - affiliateRecipient
    BadRequestResponse:
      type: object
      properties:
        message:
          type: string
          example: error message
      required:
        - message
    AnyInputQuoteWithdrawal:
      type: object
      properties:
        status:
          type: string
          enum:
            - SUCCESS
            - FAILED
          description: Withdrawal status
        amountOutFormatted:
          type: string
          description: Amount out in readable format
          example: '9.95'
        amountOutUsd:
          type: string
          description: Amount out in USD
          example: '9.95'
        amountOut:
          type: string
          description: Amount out in smallest unit
          example: '9950000'
        withdrawFeeFormatted:
          type: string
          description: Withdrawal fee in readable format
          example: '0.01'
        withdrawFee:
          type: string
          description: Withdrawal fee in smallest unit
          example: '10000'
        withdrawFeeUsd:
          type: string
          description: Withdrawal fee in USD
          example: '0.01'
        timestamp:
          type: string
          description: Timestamp of withdrawal
          example: '2019-08-24T14:15:22Z'
        hash:
          type: string
          description: Transaction hash
          example: '0x123abc456def789'
      required:
        - status
        - amountOutFormatted
        - amountOutUsd
        - amountOut
        - withdrawFeeFormatted
        - withdrawFee
        - withdrawFeeUsd
        - timestamp
        - hash
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