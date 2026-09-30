# Experiment boundaries and unresolved specification issues

Status: local experiment, not an adopted codec/consumer contract.
The candidate is pinned to repair-02. The parent reported a newer repair-03
for two static consumer-order findings (tag35 debtor/creditor and tag16/17
B05/B07 fact precedence). These are not codec/provider results. Comparison
with repair-03 is required before any design claim; this experiment does not
implement those consumer anchors.

1. **Policy identity domain versus context.** The policy prose references mapping A,
   while H-H20 requires a genuine policy Core ID differing from earlier context
   to survive image formation and fail a later body comparison. The local policy
   serializer checks the Core ID's nominal sort and common identifier subtype.
   It does not assert its equality to external Source/Core selection. Candidate
   composition separately derives both IDs from the checked Source/package
   inputs and requires those inputs to agree. The test for a differing standalone
   policy Core ID establishes different encoded content only. A full artifact
   admission or tag12 judgment remains unresolved and is not implemented.

2. **Actual package correspondence.** Fixed labels and role bytes can be serialized
   for arbitrary raw bytes. No lexical export scan can establish that those bytes
   implement the selected functions, have the same declared semantics, form a
   complete module closure or were loaded/executed. Synthetic file vectors test
   bytes only. Actual current-file vectors identify the exact files read; they
   do not discharge the spec's separate B06 correspondence obligations.

3. **Artifact decoder and diagnostics.** The proposal does not select general
   image-parser codes or artifact transport. This experiment has a producer and
   a content comparator over closed projected records, with a local exception
   class and field paths. It does not invent a decoder, consumer anchors or
   tag8/11/12 diagnostic schedule. The explicitly specified standalone candidate
   keyRef domain error is retained with null consumer anchor/fact path and null
   published post/effects. These local errors cannot be reclassified as an
   authenticated consumer result.

4. **Typed projection boundary.** The producer accepts already projected records.
   Source version/profile, selector/constructor, included identifier domains,
   scales, policy rounds/nominals and decoded keyRef are checked. It does not
   establish successful Source formation, validate excluded AST fields or run
   W-D2F's complete D sweep. A future Source adapter must parse/check the whole
   candidate before projecting and must derive the kind from the signed action.
   No caller claim of successful formation supplies that missing adapter.

5. **Vector independence.** Python struct/to_bytes serialization and Node Buffer
   serialization are separate implementations written by this experiment's
   author. Their agreement detects implementation differences; it is not a
   separate-person/provider audit or a proof of unique SHA256 images.
