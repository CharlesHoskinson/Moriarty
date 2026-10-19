# A102 Poseidon child: C4 source cut

This draft publishes the reviewed source for one Sherlock A102 `Eval(Literal) -> Return` child proof and its artifact consumers. C4 compilation and execution are unobserved at this frozen publication cut. It is not a linked two-transition parent, terminal decider, recursive-proof result or stagenet result.

`mori_backend::sherlock_child` uses the existing full `LiteralLayout::emit(LocalPublic)` relation with the pinned low-level Poseidon transcript. The instance contains the binding followed by every positional statement lane. The existing Blake2b literal-proof interface remains separate. `ChildKey::reconstruct` performs native degree selection, SRS loading and verifying-key construction; `prove` performs real proof work and self-verification. These operations are not ordinary source inspection.

The native test source adds a producer, a frozen-artifact verify-only consumer, and a separate negative-case consumer. C4 writes reconstructed VK, instance and claim artifacts before proving, then records phase markers and preserves the joined returned artifacts after success. These records identify completed calls; they do not reveal progress inside a running SDK call. The selectors remain ignored by default and have not been executed for this cut.

Five product files are selected:

- the new child module;
- its crate-root registration;
- three direct dependency declarations;
- the corresponding three existing-package lock edges;
- the native consumer source, including the reviewed T1 artifact phases and T5 documentation correction.

The complete [source delta](source-delta.patch), [diffstat](source-diffstat.txt) and [portable source pins](SOURCE-PROVENANCE.json) identify the exact bytes. Root's [installation receipt](metadata/ROOT-ACTUAL-INSTALL-RESULT.json) records their prior installation into the held native source tree; that receipt records no Cargo or native execution.

The [fresh C4 review](reviews/source-C4-T1/FINAL-RESPONSE.txt) says `APPROVE_SOURCE_WITH_LIMITS` for the T1/T5 changes and describes the source as compile-plausible. Its conditional compile disposition requires exact current inputs, the actual guard/helper dispositions, offline dependencies and unchanged limits. It grants no selector execution eligibility. Its unread-body list and T2/T3/T4 limitations remain part of the opinion. Prior qualified [revision02](reviews/source-C3-revision02/FINAL-RESPONSE.txt) and [revision03](reviews/source-C3-revision03/FINAL-RESPONSE.txt) opinions are retained in full with their raw provider receipts; they are not presented as final-round rereads.

A killed or failed native call remains a failure. Phase files do not make a partial directory eligible as a produced child: frozen-input selection must require the successful producer report, `host_checks_passed: true`, and all four exact artifact identities. C4 still cannot persist a proof before an internal self-verification returns, and SDK/helper panics may leave no error record. No performance improvement is claimed.

Cross-graph proof/key byte compatibility with Fenrir's separately pinned dependency graph is unobserved. Pairing completion, genuine A80 linkage, terminal statement/anchor integration, canonical projection, full StepInv/F4/G4/dispatch and source authentication remain separate requirements. Existing Am10/T3/k17, C4/R2 and prior failed/timeout records retain their exact scopes. This archive contains source/review provenance; it includes no ELF, SRS, proving key, proof payload, wallet or private oracle data.
