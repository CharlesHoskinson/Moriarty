# Experiment boundaries and unresolved specification issues

Status: local experiment, not an adopted codec/consumer contract.
The candidate is pinned to repair-02. The read-only repair-03 comparison records
the same exact byte rules and two consumer-order changes (tag35 debtor/creditor
and tag16/17 B05/B07 fact precedence). These are not codec/provider results.
This comparison does not adopt the design or implement those consumer anchors.

## Independent executable review defect and scoped repair

The original codec passed 72 tests but discarded captured descriptor values
and reread caller properties. The independent GPT review requested changes for
a high Proxy admission/coherence defect: valid selector checks could precede
different encoded selector bytes; Source asset A could precede policy asset
Other; ID values could become non-ASCII after validation. The preserved original
files remain under `history/pre-proxy-repair-01/`. Four failed local reproductions
are retained in `proxy-reproduction.tap`.

Proxy repair-01 captures each original record/array descriptor once, recursively
snapshots nested IDs and operation/file records, copies intrinsic byte views,
and memoizes shared objects. All validation and encoding reads frozen owned
records. Candidate production captures all three inputs before its phases and
derives policy identities from the same captured definition/package. Returned
policy records and nested IDs/operation are immutable and have no caller aliases.
Byte storage is owned; produced payload/envelope buffers remain mutable output
values and are not authentication objects. Hostile complete Proxy graphs preserve
all nine independently serialized frozen image vectors.

The local tests demonstrate their listed Proxy cases and stable captured content.
Capture is a synchronous per-field structural view, not a proof that arbitrary
Proxy traps are pure or an authenticated atomic observation of an external
artifact. Fresh independent review of the repaired executable candidate remains
pending; the original audit does not approve the changed bytes.

## Remaining scope boundaries

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
