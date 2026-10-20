# EnvGet traversal: explanation and evidence reference

## Explanation

On a nonmatching environment cell, Core copies the EnvGet helper and changes only cursor to that cell's previous pointer. Found, Pending status and common fields remain. A hit later stages the matching value and Positive status; Exit removes the helper and stages EnvFound in the result register. A cursor reaching None still needs a separate Empty row, which this positive slice does not silently insert. The new relation composes these rows over complete Host records and complete event/work framing, reusing unchanged Q41 hit/Exit/charging/event helpers.

Two pre-code public manual examples exercise a skipped off-chain matching binder. The one-miss path2→0 skips cell1's wrong matching value; the two-miss path3→1→0 skips cell2. Full expected Host/event equality and intermediate prefix tests discriminate pointer traversal, early found/status changes, wrong work and lost retained state. This improves semantic coverage but remains a same-author synthetic model check.

## How to inspect

Read the [source README](../../../models/sherlock/quint/envget-nonmatching-traversal-20261020/README.md) and [dated result archive](../../results/sherlock-quint-envget-nonmatching-traversal-20261020/README.md). Both source and result opinions, original COMMAND/RESULT/streams, retained actual operators, complete source/input freezes and manual derivation are supplied. Gzip archives decompress byte-exact to their original records under the archive manifest. This guide prescribes no new model, owner or native execution.

## Reference and limits

The runtime machine is Q42Trace{hosts,events,finished,within_slice}. Four actual model states retain Host/event prefix lengths1/0,2/1,3/2,4/3. Every Host includes complete admission and image; every event retains all28 declared fields. The selected CLI invariant was normalOwnerWorkPreserved. The witness completeTraversalWitness requires finished and within_slice and hosts.length4/events.length3; the literal CLI counter is1of1. The printed47ms simulation timing differs from9.340274096s wholeCLI wall and supplies no native performance claim.

The carriers are unadmitted, not actual Owner-produced prestates. Admission/reachability/StepInv, correct external positive owner premises, concrete host widths/caps and common real Core/K comparison remain separate. A useful next correspondence step is a separately authorized real Owner before-input for the shared one-miss nested-let case, followed by this unchanged general relation and a complete four-Host/three-event comparison. That actual adapter/capture is not supplied by these passes. No general refinement, security, finance, native/circuit/proof/stagenet or EARS closure follows.
