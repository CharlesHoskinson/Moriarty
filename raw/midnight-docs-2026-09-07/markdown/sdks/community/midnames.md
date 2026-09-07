# Midnames

> For the complete documentation index, see [llms.txt](/llms.txt)

Midnames has a suite of tools and services to help you interact with the Midnight Network. This includes a name service that allows you to register human-readable names for Midnight addresses, and a playground where you can write and compile Compact code.

## Compact Playground[​](#compact-playground "Direct link to Compact Playground")

You can access the playground at [playground.midnames.com](https://playground.midnames.com/). There, you can write and compile Compact code, and interact with it using the integrated Simulator.

You'll find the examples from the [Compact documentation](https://docs.midnight.com/compact) in the "Examples" section of the playground. You can also create your own projects and share them with others.

### Compilation[​](#compilation "Direct link to Compilation")

You can compile your Compact code using either a remote or local compiler. The remote compiler sends code to a server. The local compiler uses a WASM-based version that runs in your browser, but has some limitations, such as failing to generate proper error messages.

Local compilation

The local compiler is slow and may take a while to compile larger contracts. The errors it generates are not helpful.

## Name service[​](#name-service "Direct link to Name service")

Midnames also runs a name service at [midnight.domains](https://midnight.domains/). The name service allows you to register human-readable names that resolve to Midnight addresses, making it easier to interact with contracts and addresses on the Midnight Network.

You can find a guide on how to buy a domain at [docs.midnight.domains](https://docs.midnight.domains/guides/buy_domain/).

### SDK[​](#sdk "Direct link to SDK")

The midnight.domains SDK is available as an npm package. You can install it using:

```
npm install @midnames/sdk
```

The SDK provides functions to interact with the name service, such as registering a name, resolving a name to an address, and adding metadata to your domains. You can find the documentation for the SDK at [docs.midnight.domains](https://docs.midnight.domains/reference/resolve/).
