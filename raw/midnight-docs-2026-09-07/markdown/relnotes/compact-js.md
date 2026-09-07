> For the complete documentation index, see [llms.txt](/llms.txt)

# Compact.js

Compact.js provides a TypeScript-based execution environment for smart contracts compiled with the Compact compiler. When a Compact smart contract is compiled, the output includes a JavaScript file and a TypeScript declaration file.

Compact.js uses these files at runtime to execute the circuits. The circuit execution results are then used by higher-level tools and frameworks (such as Midnight.js) to create and submit transactions to the Midnight blockchain.

For more information, see the [Compact language documentation](/compact.md).

***

<!-- -->

Version2.5.3

StatusAll

### [Release <!-- -->2.5.3](/relnotes/compact-js/compact-js-2-5-3)LATEST

9 July 2026

#### Artifacts

* [NPM Package](https://www.npmjs.com/package/@midnight-ntwrk/compact-js/v/2.5.3)
* [GitHub release](https://github.com/midnightntwrk/midnight-sdk/releases/tag/compact-js-v2.5.3)

#### Summary

* Fixed the CommonJS exports configuration of the package.
* Security update: bumped the `tar` dependency to 7.5.16.
* Updated development dependencies and release automation.
* First version published to npm from the migrated `midnightntwrk/midnight-sdk` repository; supersedes the 2.5.1 and 2.5.2 alignment versions.
