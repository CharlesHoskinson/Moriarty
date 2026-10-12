# Versioned proving-key transport budgets

Root adopted two substantive independent design votes and the independent
expectations SHA256 `014b8382d6721c26a4468c1dc7c633b3c5b6d1474bcc8d92e53b46aa62d91556`
before authoring. This source repair grants no native execution or retry.

Keep the NativeWorkLimits DTO unchanged. Exact `/1` preserves its existing
admission, unsigned historically uncapped public fields, Signed ordinary
envelopes, PK min(prover, leaf) and flat aggregate semantics. Exact `/2` adds
hard ceilings: PK 67108864 bytes, public leaf/aggregate 16777216, input/JSON
1048576 and VK 65536. The 1 MiB class covers existing input JSON/markers;
generated readback, verification and signing-package outputs retain their
existing public output class. Positive checked caller limits and smaller caps apply.
SRS admission/cumulative counters and native phase counts remain unchanged.
V2 export rechecks retained source, predecessor and ledger parameters against
the current input budget before copying leaves. Its deployment descriptor and
Signed replacement use min(current input, leaf, INPUT); each is serialized once
before export leaf/PK copies. V1 keeps its old source/descriptor checks and
serialization order. Keyset markers remain in their existing separate class.

Only a validated keyset export gets a typed internal selected-PK singleton
context. Generic ArtifactLeaves/proof/package checks remain public-only.
Generation, keyset export, Signed replacement, proving import and both reopen
prechecks use separate public/prover/checked physical counts in v2. Every
non-PK export byte, including both markers, counts public; cold readers charge
only their acquired public subset. Physical ceiling is checked public cap plus
PK cap. Neither PK budget nor transport physical count funds SRS work.

Bound marker acquisition before parsing, and declared PK caps/physical addition
before opening. Retain nonblocking regular-file checks, declared-length bounds,
sentinel and exact hash. Cold public readers ignore optional PK/proving leaves
without stat/open/hash. Imported PK remains opaque; fresh all-action native
regeneration, all VK comparison and selected PK comparison still precede use.

Use tiny opaque fixtures and pure real-size count boundaries, preserving every
v1 control. No derived owners, native phases, genuine SRS or large test vectors.
