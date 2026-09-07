> For the complete documentation index, see [llms.txt](/llms.txt)

# Node

Node is a core component of the Midnight network, responsible for syncing, validating transactions, and maintaining the chain state.

[Link to related documentation](/nodes.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version1.0.1

StatusAll

### [Release <!-- -->1.0.1](/relnotes/node/node-1-0-1)LATEST

14 July 2026

#### Artifacts

* [Midnight node](https://hub.docker.com/r/midnightntwrk/midnight-node)
* [GitHub release](https://github.com/midnightntwrk/midnight-node/releases/tag/node-1.0.1)

#### Summary

* `unsafe_allow_symlinks` now has a default value, so nodes without a TOML configuration file boot without the missing-field error from v1.0.0.
* Regenerated the Preview network genesis and chain spec.
* Removed the incorrect `NIGHT` asset name from the Preview reserve configuration.
* Backported the ledger version bump and updated dependencies to fix the release build.
* Ships with `toolkit-1.0.0` and `runtime-1.0.0`; no runtime upgrade is required.
