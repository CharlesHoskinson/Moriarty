# Independent security repairs

The independent reference suite initially returned7passes and2failures. The
[retained failure](evidence/S1-root-reference-check.txt) exposed two distinct issues.

The production signed-intent codec delegated compressed SEC1 parsing to k256,
which accepted a noncanonical05prefix alias. The protocol requires02or03; Rust
now rejects other prefixes before deserialization. The original experimental
primitive endpoint is preserved. This repair enforces the frozen wire format.

The independent Python oracle initially challenged the raw frame for BIP340.
Midnight's pinned Schnorr signing trait hashes the message with SHA256 before
BIP340. The oracle was corrected to match that actual trait, without importing
the production codec or removing cases. Golden fixtures retain exact expectations.
Both fixes pass the original nine reference tests and actual runner mutations.

Readable CLI integration also exposed Node24 strip-only rejection of TypeScript
constructor parameter properties. The retained failed display test shows that
error; explicit declared fields now work in source execution and the bundle.
Malformed caller artifacts now return BETA_SIGNATURE_SCHEMA (judgment, exit1);
BETA_CRYPTO_RESPONSE remains reserved for inability to trust a native response
(exit2). Source mismatch includes a typed pointer and claimed/computed values.
