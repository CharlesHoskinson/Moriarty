# Source bytes are part of the statement context

Draft for independent documentation review, based on Sherlock commit `35931362`. This page describes source identity, not a claim of proof or semantic correspondence.

Sherlock's A102 and complete literal adapters hash exact source text into their relation identities. The A102 list contains nine files; the complete list contains thirteen. Both include `literal_bridge.rs`. Each context then incorporates its relation hash, alongside its layout and declared identities. See the [A102 source](../../../crates/mori-backend/src/sherlock_machine/literal_host.rs) and [complete source](../../../crates/mori-backend/src/sherlock_machine/literal_complete_bridge.rs).

A logging-only edit to a listed file changes the bytes consumed by that identity construction. It therefore creates a new identity calculation input even when the diagnostic change leaves the intended transition unchanged. A hash difference does not by itself identify a semantic defect; semantic equivalence also does not establish equality of the exact source-bound context.

A saved proof or result remains evidence for its recorded input cut. Retain its source list, context and actual result together. An observability patch must not silently inherit the old context or promote an earlier proof result to the changed cut. The recorded logging candidate remains deferred; this page does not select a build, proof, verification or instrumentation edit.

The [published A80/source-capture integration](../../results/sherlock-a80-kcapture-source-integration-20261019/README.md) preserves a related distinction: its frozen WW executable produced passing scoped results, while the newly integrated canonical source cut remains uncompiled and untested. Exact matches within a bounded comparison readset do not establish whole-workspace equality.
