# Midnight installed-SDK seam observation

On October 23, 2026, one local synthetic fixture exercised the installed
`@midnightntwrk/ledger-v9` 1.0.0-rc.3 API. The
[public receipt](../../evidence/sherlock/public/o4-installed-sdk-seam-2026-10-23.json)
records the source, review and adopted-result hashes.

The SDK called the fake provider in order: `check(raw, key)`, `lookupKey(key)`,
then `prove(raw, key, binding)`. The binding was a nonzero `bigint` below
2^248. An owned sentinel rejected the last callback before any proof returned
or verifier-key tag was inspected. The run also observed two distinct seals,
a byte-equal bound clone, preserved disjoint intent bytes, segment changes and
three merge/segment refusals. Synthetic stored Proof V2/V3 envelopes round-tripped;
seven codec vectors were refused. These envelopes test storage parsing, not
cryptographic validity. The sentinel leaves the transaction unproven (`PreProof`).
Binding and cloning exercised unproven transactions, and the successful merge
used their `PreBinding` variant. Finalized/Proof DUST-only merge remains
unobserved. Stored-object parsing is separate from the unobserved
prove-to-stored-tag mapping.

The subject exited successfully in 0.1663308899878757 seconds and emitted the
exact expected 1,205-byte JSON line with empty stderr. The recorded whole
invocation, including all Root observer overhead, took 2.592682805 seconds.
The subject timing includes SDK loading and all API checks; neither timing is
a proof-generation or verification benchmark. It reported 25 caller-visible
wrappers owned and freed once, and invocation-owned cleanup completed. Internal SDK getter
reclamation and memory peak remain unobserved. The source has 26 assertion
sites; that is not a count of executed test cases.

The [fixture](../../tools/sherlock-sdk/public-installed-callback-codec-smoke.mjs)
is published byte for byte as a one-off lab fixture. It retains the absolute
public local SDK import used for this observation and requires ledger-v9 rc.3
at that embedded location. It is not portable as shipped. Changing the import
changes the fixture bytes, so this recorded result would not validate that
modified fixture. Its historical source-only and Root-selection comments are
preserved; they describe the maintainer experiment, not public product
permissions or a Sherlock CLI gate.

The imported DUST helper, no-op override changes, post-sentinel key-tag mapping
and positive bare-transport decoding remain unobserved. Binding did execute
real native Schnorr cryptographic operations. No recursive/Halo2 contract proof
was generated or natively verified; no wallet, real key material, SRS or network
was used. Deployed-key identity, fees, DUST, node verification features, resource
fit, a genuine proof-carrying D call and target settlement remain separate
obligations. Full16 and full Sherlock completion remain open.
