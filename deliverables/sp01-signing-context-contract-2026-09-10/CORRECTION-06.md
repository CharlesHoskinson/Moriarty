# Source graph currentness correction 06

The05 checker verified the seed history and selected predecessors but omitted
currentness at two boundaries: importing the seed head and reporting the final
live set. Fresh GPT6 review found both defects. Grok05 passed its narrower static
checks and did not identify these cases; that dissent is retained. The combined
GPT6 review files for Boolean and signing intentionally have identical digests:
each raw receipt contains both reviews. Neither receipt is a substitute for the
other candidate's verdict.

branch-checker-06.py repairs only these existing currentness obligations. The05
schema, commitment domain, profile, financial rules and exact occurrence binding
are unchanged. This is a source checker correction, not a registered wire upgrade.

After complete seed validation, the verified head must not appear in the supplied
base consumed set. This rejects SEED_CURRENTNESS before importing balances,
remaining quotas, debt, duties or work and before graph financial derivation.
Validating a linear history proves its transitions within this source model; it
does not assert that the final resource is currently available. The graph import
is the explicit boundary requiring that additional premise, even for an empty
graph. Validation remains pure and does not mutate a ledger or consumption set.

Selected predecessors still require local liveness and external availability.
After deriving the entire graph, every live output must be absent from external
consumedOutputs before checking exact equality with the declared current list.
This includes a final join, an untouched sibling and the root of an empty graph.
The checker rejects CURRENTNESS rather than dropping an unavailable resource,
which could otherwise silently drop its debt or duties. Consumption of unrelated
identities does not invalidate an otherwise available history.

The retained red run uses a byte-identical copy of05 with the six new06 tests:
five fail for missing checks or wrong first failure; the unrelated-consumption
positive passes. After the two boundary guards, all22 branch tests pass, including
the previous16. The unchanged33 context tests are run separately. The tests do
not authenticate consumed sets, establish signatures or prove native currentness.

All67 original05 pins remain unchanged. Full38 financial ABI/signing scope,
cryptographic context, K/proof correspondence and Midnight acceptance remain open.
Independent source rereview is required before accepting this correction.
