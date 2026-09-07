> For the complete documentation index, see [llms.txt](/llms.txt)

# Frequently asked questions

Find answers to common questions about the Midnight Network, development tools, and troubleshooting. If you cannot find the answer you need, see the [getting help](/troubleshoot/getting-help.md) page for additional support resources.

## General questions[​](#general-questions "Direct link to General questions")

This section answers some of the most common questions about Midnight Network.

### Is there a Midnight white paper?[​](#is-there-a-midnight-white-paper "Direct link to Is there a Midnight white paper?")

Yes. Midnight provides technical white papers covering the network architecture and economic model:

* **Nightpaper**: Technical overview of the Midnight Network architecture and privacy-preserving smart contract system.
* **Tokenomics incentives**: Detailed explanation of the NIGHT token economics and network incentive mechanisms.

For more information, see [midnight.network/whitepaper](https://midnight.network/whitepaper).

### What tokens are available for use on Testnet? Are there gas fees?[​](#what-tokens-are-available-for-use-on-testnet-are-there-gas-fees "Direct link to What tokens are available for use on Testnet? Are there gas fees?")

Testnet uses only one token: test DUST (**tDUST**), which is a test token used for Midnight Testnet testing purposes only. Visit the [token acquisition page](/guides/acquire-tokens.md) to find out more. This may change in future versions of Midnight, and may include the calibration of the gas fees against the amount work performed by a computation.

## Developer questions[​](#developer-questions "Direct link to Developer questions")

This section answers some of the most common questions about developing on Midnight Network.

### Where do I go if I need help troubleshooting my code?[​](#where-do-i-go-if-i-need-help-troubleshooting-my-code "Direct link to Where do I go if I need help troubleshooting my code?")

The [Getting help](/troubleshoot/getting-help.md) section of this site describes multiple ways to communicate with the Midnight team and your fellow developers. Your questions, including those about troubleshooting your code, are welcome.

### What types of DApps can I build on the Midnight Testnet?[​](#what-types-of-dapps-can-i-build-on-the-midnight-testnet "Direct link to What types of DApps can I build on the Midnight Testnet?")

Theoretically, any DApp that does not require one contract to call another from within its circuits. This includes private payment DApps, private auction DApps, and DApps that enable shielded identity verification.

### What types of DApps can *not* yet be built on Testnet?[​](#what-types-of-dapps-can-not-yet-be-built-on-testnet "Direct link to what-types-of-dapps-can-not-yet-be-built-on-testnet")

DApps that require an oracle (for pricing data info or other external data), such as a DeFi lending DApp requiring Bitcoin pricing data.

### Can I reuse Solidity code on Midnight?[​](#can-i-reuse-solidity-code-on-midnight "Direct link to Can I reuse Solidity code on Midnight?")

No, Midnight DApps are created in TypeScript and Compact, a custom programming language, to build zero-knowledge circuits that generate privacy proofs.

### What are the key unique concepts or coding patterns I need to know to create DApps on Midnight?[​](#what-are-the-key-unique-concepts-or-coding-patterns-i-need-to-know-to-create-dapps-on-midnight "Direct link to What are the key unique concepts or coding patterns I need to know to create DApps on Midnight?")

One of the key ideas in Midnight is the distinction between information that you want to place in the public record and information that you want to keep private. For example, the assertion that someone is over 25 might be useful to place in the public space of a contract, while the details of the person's birthday and precise age might be kept private. This kind of thinking about what is truly needed in the public sphere is a core aspect of Midnight programming.

After writing the contract in Midnight’s contract language, the DApp is written in standard TypeScript. This implies that the coding experience of existing JavaScript and TypeScript programmers can be applied to creating Midnight DApps.

### How does Midnight work at a high level?[​](#how-does-midnight-work-at-a-high-level "Direct link to How does Midnight work at a high level?")

See [Midnight's architecture](/concepts/how-midnight-works/midnight-combined-model.md) and the section of this site about [How Midnight works](/concepts.md).

### What is the current Testnet block time (time to finality)?[​](#what-is-the-current-testnet-block-time-time-to-finality "Direct link to What is the current Testnet block time (time to finality)?")

Testnet block time is 6 seconds. This time is governed by network parameters that are subject to adjustment. Finality will occur typically one or two blocks after block creation (so within 18 seconds).

### I'm getting `ERR_UNSUPPORTED_DIR_IMPORT`. What should I do?[​](#im-getting-err_unsupported_dir_import-what-should-i-do "Direct link to im-getting-err_unsupported_dir_import-what-should-i-do")

This error typically occurs when Node.js tries to import a directory instead of a specific file, which can happen if your terminal environment is stale after updating `~/.zshrc`, changing Node versions, or setting environment variables.

To fix this:

* Open a new terminal window (don’t just run `source ~/.zshrc`) after changing your shell config or switching Node versions.
* Ensure you're using the correct Node version (Midnight requires Node 22+). Run:
  <!-- -->
  ```
  nvm use
  ```
* Clear any module cache:
  <!-- -->
  ```
  rm -rf node_modules/.cache
  ```
