<!-- source: https://docs.midnight.network/relnotes/compact-tools.md fetched_at: 2026-11-07T20:48:45Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Compact developer tools

Compact is Midnight's dedicated smart contract programming language, designed for building secure, efficient, and adaptable decentralized applications.

Compact developer tools are a command-line utility for installing, updating, managing, and running the Compact toolchain and compiler.

[Link to related documentation](https://docs.midnight.network/compact.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version0.5.3

StatusAll

### [Release <!-- -->0.5.3](https://docs.midnight.network/relnotes/compact-tools/compact-tools-0-5-3.md)LATEST

29 September 2026

#### Artifacts

* [Compact developer tools](https://github.com/midnightntwrk/compact/releases/tag/compact-v0.5.3)

#### Summary

Maintenance release: toolchain installs no longer need unzip, and running compact update again repairs a failed install.

* `compact update` unpacks toolchain archives itself, so installing a toolchain no longer needs `unzip`.
* Running `compact update <version>` again after a failed install now reinstalls that version instead of reporting `already installed` and failing.
* `compact` no longer sets a missing compiler as the default, and commands that find a broken default now print the `compact update <version>` command that repairs it.
* `compact update` deletes an unreadable toolchain archive and downloads it once more instead of failing on every retry.
* `--help` output wraps to the width of your terminal.
* Known issue: with toolchain 0.22.0 or 0.23.0 installed by 0.5.3, `include "std";` fails to compile. Use `import CompactStandardLibrary;` instead.
