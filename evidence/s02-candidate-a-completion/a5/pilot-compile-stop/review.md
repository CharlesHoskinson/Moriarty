# Funding pilot: preserved experiment, stopped compilation gate

Root records the original Task3 experiment without claiming Task3 or A5
acceptance. Source review matched all three files to the adopted plan, with
only the specified five variant declaration/import differences. The separate
nonauthor source review found no material mismatch and disclosed its earlier
dependency authorship. This is not a cross-provider Council verdict.

The original recursive typecheck and full 22-prefix test passed. Both models
passed 100 sampled traces with all eight witnesses positive. The root audit
checked the exact commands, outer postcheck results, all 353 native source
archive members, 25 dispatch archive members and 76 indexed files. Source,
runtime and dispatch pins remained unchanged.

Original compilation then exited 1 in 214.022 seconds, without timeout. Its
maximum recorded RSS was 1985960 KiB. Seven QNT404 diagnostics report the missing
`AuthorityKey` alias during name resolution. The retained `input.qnt.json` is
zero-byte original stdout, not valid JSON or a zero-sized successful baseline.
Its stderr SHA256 is
`606a4bf274f5499111b50b3897cc06064a2c953d3ef9fdbcf57e567baccba842`.
The outer wrapper also exited 1 and prevented further execution.

Factored compilation, both checker invocations and the `neverPrepared` control
were not run. There are no valid paired sizes or JSON object counts. H1's
tractability hypothesis remains unresolved; this frontend failure is neither
an InlinePass resource outcome nor a semantic architecture counterexample.

## Read-only cause investigation

The installed pinned compiler source establishes this failure boundary:

1. `quintAnalyzer.js:27` replaces declarations after type-application expansion.
2. `types/typeApplicationResolution.js:60` expands parameterized type bodies.
3. `cliCommands.js:423` resolves names again before flattening. Its failure
   branch emits the retained `name resolution failed` message.
4. `names/resolver.js:108` checks an alias in the current module's collector;
   `names/base.js:30` excludes hidden imported names when copying a module.

The original adapter instantiates `ExecutionState` and `EvidenceBundle` at
lines 18 and 19 without a direct consumption import. `AuthorityKey` is defined
in `consumption.qnt:5`; generic type bodies refer to it at the reported original
positions. Both root and the nonauthor diagnostic reviewer inspected this
chain. All five compiler files match the admitted runtime manifest.

Inference: expansion exposes unqualified references in the adapter context,
where the nontransitive import does not supply the alias. Preserved original
source IDs explain diagnostics pointing into generic definitions. This cause
is strongly supported by source inspection but the transformed intermediate
IR and remedy have not yet been reproduced. `--flatten=false` does not bypass
the earlier name-resolution step.

No frozen source, helper or compiler was changed, and no retry was made. A
small isolated alias-visibility control requires its own reviewed plan before
execution. Any later compilation view must preserve exact paired provenance,
all semantic bodies/types/routes and the original failed receipts. No resource
escalation or actual-model repair is authorized by this preservation commit.

## Evidence recovery

`original-receipts.tar.gz` preserves the original five stages, their separate
dispatch/outer-terminal records, author report/index, source review, exact pilot
files and root audits. `archive-validation.json` binds all original members.
Extract only into a fresh directory for replay, never over the working tree.
The shared runtime archive remains in `../kernel-task1/runtime.tar.gz.part-*`;
the reused Python snapshot remains in the A4 checker Task1 original archive.
They are referenced by their original digests, not duplicated or replaced.
