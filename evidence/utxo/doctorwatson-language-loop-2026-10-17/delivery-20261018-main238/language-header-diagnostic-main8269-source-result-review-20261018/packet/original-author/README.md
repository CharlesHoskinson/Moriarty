# Core language header diagnostic correction

At main79, public language() already returns Core1 and Core2, while its unknown
profile message lists only Core1. Missing keyword/quoted-profile errors also
omit supported-profile guidance. This patch changes only language() diagnostic
messages, preserving error codes, offsets, token consumption and accepted dispatch.
The one-profile parse_profile messages remain correct and unchanged.

One focused integration regression calls the public language() API for both
supported headers and ordinary unsupported/missing header cases. It fixes literal
expected messages and locations; it does not execute a program or assess security.
REGRESSION.rs and EXPECTED-DIAGNOSTICS.json were prepared before candidate edits.
Rust red/green execution is UNRUN because this task authorizes source only.

source/ and candidate/ contain complete before/after bodies; MANIFEST.json has
exact main79 Git blobs/SHA256s and the proposed root selected test command.
Root integrates and performs fresh source/result review. No shared writes,
provider call, build, native execution or active help-candidate edit occurred.
