[![MARLOWE](/logo-full-white.svg)![MARLOWE](/logo-full-black.svg)](/)

[Documentation](https://docs.marlowe-lang.org/docs/introduction)[Tools](/)[Blog](/blog/)

Why Marlowe?
------------

* **Accessible & Flexible**:
  With a low barrier to entry and dedicated libraries for [TypeScript (opens in a new tab)](https://github.com/marlowe-lang/marlowe-ts-sdk), [Rust (opens in a new tab)](https://github.com/OlofBlomqvist/marlowe-rs) and others, Marlowe lets you quickly learn and create smart contracts on the fly.
* **Built-In Security**:
  Marlowe’s comprehensive validation tools and inherent safety features significantly reduce audit overhead, making it a secure choice for DApp development and implementation of financial smart contracts.
* **Seamless Integration**:
  Our Runtime abstracts the complexities of the Cardano UTxO model, ensuring smooth deployment and easy interaction with your contracts.

![Marlowe Execution Graph](/imgs/blog/marlowe-graph.png)

Tools
-----

Marlowe provides a comprehensive suite of tools for smart contract development and deployment:

### Playground

Create and test smart contracts in a pure and visual environment. Perfect for the beginners:

* Design with JavaScript, Haskell, Blockly, or Marlowe
* Test and simulate contract execution
* Validate your contracts before deployment

[Try Playground → (opens in a new tab)](https://playground.marlowe-lang.org/#/)

![Marlowe Playground Interface](/imgs/blog/playground.png)

### TypeScript SDK

Build applications with Marlowe smart contracts:

* Complete TypeScript SDK for application integration
* High-level abstractions for simplified contract execution
* Built-in REST client for Runtime API integration

[Explore SDK → (opens in a new tab)](https://github.com/marlowe-lang/marlowe-ts-sdk/tree/main)

![Marlowe TypeScript SDK](/imgs/blog/ts-sdk.png)

### Runtime

Deploy and manage Marlowe contracts on the Cardano blockchain:

* Well-documented REST API with OpenAPI specification
* Abstracts UTxO complexity for simplified Cardano blockchain interaction
* Full web wallet and CIP-30 compatibility

[Explore Runtime REST API → (opens in a new tab)](https://preprod.100.runtime.marlowe-lang.org/openapi.html)

![Marlowe Runtime](/imgs/blog/runtime.png)

### Runner

A web interface for deploying and managing Marlowe contracts on Cardano preprod:

* Easy deployment process using your favourite web wallet
* Monitor and manage contract execution
* Available on the preprod testnet for immediate testing
* An example of a pure frontend DApp which uses Marlowe Runtime as the only backend

[Use Runner → (opens in a new tab)](https://preprod.runner.marlowe-lang.org/)

![Marlowe Runner Interface](/imgs/blog/runner.png)