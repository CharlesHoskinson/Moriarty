# Root admission: case-sharded checker supplement

Admitted scope: the schema3 checker implementation and focused tests only.
This is not acceptance of the actual 78-shard package, Task6, A4, A5, or Council.

Root read the complete implementation/test diff and original author report.
The read-only audit completed with exit 0. It checked all ten original command
stages, all 420 source archive members, stdout/stderr hashes, recorded parent and
child process-start cache arguments, stable per-command source/runtime pins,
the final current bytes, and all 3329 shared Python runtime files.

Final source pins:

- Checker: `ce055ac1a611f6eecc74248da64962a5c899ebbf282d41f3ca243dde51365c66`.
- Tests: `f6e547364e3d7707237654bb135229a414857ac64a414aaaa2178ce06957db26`.
- Original report: `e1d229988183686dae7c2f8e6121f6399047558d1d9eef75aba43309efe89fc2`.
- Original receipt bundle: `db5b882bab652d273e31f1701ee91e3334cb843891d48a8a6f2dbb4ea17171c1`.

The final run passed 79 tests: 37 checker tests and 42 unchanged utility tests.
The intentional omitted raw-field equality produced an importable behavioral
failure. A separate test-first correction rejects unknown metadata before it
can accumulate. The initial full run's suffix fixture failed at the earlier
input-hash binding check; its corrected fixture rebinds the temporary event
hashes and reaches strict EOF validation. All original failures are retained.

The report's narrative dispatch base is `fdb81c7`. The actual recorder base in
all ten command inputs is `9d26d9ba1e5cd970ff7be599473901be0d1eb33f`.
The latter is the evidence base for this admission. The original report was
not rewritten. These are distinct from earlier plan and utility adoption bases.

Source review confirms exact schema3 pin inventories, all 78 global wrapper
names, transitive source closure, frozen/import-origin checks, chunked hashing,
lockstep raw/event windows, local adjacency and global ordinals, independently
replayed full events/states, first unsigned-state identity, and strict EOF and
metadata checks. The shared utility is lexical transport only. Synthetic focused
fixtures and resource/parser failures are not semantic-mutant or native-export
evidence. Complete package admission still requires actual native files and
Task6 controls.

The producer edit window is released after this admission. Its recorder may add
the two now-frozen streaming utility/test paths to its fixed Python closure.
No producer pilot or final export is admitted by this review.

Reproduction: restore the original bundle at the experimental repository root
and use `audit.py` with the shared Python environment retained in checker Task1.
The audit's current-byte checks apply to this admitted source version; later
authorized edits require a new receipt set, not changes to these originals.
