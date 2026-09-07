> For the complete documentation index, see [llms.txt](/llms.txt)

# Compact developer tools

Compact is Midnight's dedicated smart contract programming language, designed for building secure, efficient, and adaptable decentralized applications.

Compact developer tools are a command-line utility for installing, updating, managing, and running the Compact toolchain and compiler.

[Link to related documentation](/compact.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version0.5.2

StatusAll

### [Release <!-- -->0.5.2](/relnotes/compact-tools/compact-tools-0-5-2)LATEST

18 August 2026

#### Artifacts

* [Compact developer tools](https://github.com/midnightntwrk/compact/releases/tag/compact-v0.5.2)

#### Summary

* `compact compile --help` now forwards to the underlying `compactc`, so the help output lists every flag the selected compiler accepts, including `--feature-zkir-v3`.
* Update an existing installation with `compact self update`.
