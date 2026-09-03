> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Get user token balances

> Returns token balances for the authenticated user from private balance sources



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml get /v0/account/balances
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
  /v0/account/balances:
    get:
      tags:
        - Account
      summary: Get user token balances
      description: >-
        Returns token balances for the authenticated user from private balance
        sources
      operationId: getBalances
      parameters:
        - name: tokenIds
          required: false
          in: query
          description: >-
            Comma-separated list of token IDs to query. If empty, returns all
            non-zero balances.
          style: form
          explode: false
          schema:
            type: array
            items:
              type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/GetBalancesResponse'
        '401':
          description: Unauthorized - User session token is invalid or expired
      security:
        - User-Session: []
components:
  schemas:
    GetBalancesResponse:
      type: object
      properties:
        balances:
          description: List of token balances
          type: array
          items:
            $ref: '#/components/schemas/BalanceEntry'
      required:
        - balances
    BalanceEntry:
      type: object
      properties:
        tokenId:
          type: string
          description: Token identifier
          example: nep141:wrap.near
        available:
          type: string
          description: Available balance (as string to preserve precision)
          example: '1000000000000000000000000'
        source:
          type: string
          description: Balance source
          enum:
            - private
          example: private
      required:
        - tokenId
        - available
        - source
  securitySchemes:
    User-Session:
      scheme: bearer
      bearerFormat: JWT
      type: http
      description: User session token obtained from /auth/authenticate

````