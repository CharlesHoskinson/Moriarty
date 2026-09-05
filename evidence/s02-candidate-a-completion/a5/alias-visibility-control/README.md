# Generic-alias compilation control

Experiment observation, 2026-09-05 UTC, experimental base
`9ccbf0ed571e4055bc05962e5681fdf2fa75ad96`.

The isolated control reproduces the frontend mechanism suspected in the
preserved Candidate A funding-pilot compilation failure. A generic record
contains an unqualified `Key` alias. Its consumer imports the generic record,
but does not import `Key` directly. Typechecking exits 0; compilation exits 1
with QNT404 `Type alias 'Key' not found` and empty generated output.

A second consumer changes only its module name and adds
`import keys.Key from "./keys"`. Typechecking and compilation both exit 0.
The generated JSON is 16,540 bytes, SHA256
`fd8e96c2ced2081dd5635ae8a42916e831895ae40fb5adb49d18b9189d6173c5`.
Raw inspection confirms the nested zero-valued record, the explicit stutter
step, both zero comparisons, and the `q::init`, `q::step`, `q::inv` bindings.
These observations establish compilation behavior only.

All four commands used the unchanged pinned Quint 0.32.0 runtime and recording
helper, with a 120-second child wall limit and 4096-MiB Node/JVM settings.
Their wrappers each exit 0 because they check the required child outcome;
the original compiler child still exits 1. Original source and runtime pins
remain unchanged. The inherited helper metadata about pilot fairness,
deadlocks and classification is preserved verbatim and is inapplicable to
this frontend control. No simulator, solver or model checker ran.

Root audited each command's actual terminal record, output hashes, dispatch,
live input pins and source archive. The four archives contain 83, 93, 104 and
114 members. `audit.py` repeats those checks from the retained files and checks
the generated declaration structure. Its inner init/safety checks are
structural; full expression inspection is recorded in the independent review.

`original-receipts.tar.gz` retains the original control files, author index,
plan, independent reviews and root audit. `archive-validation.json` pins its
complete membership. Extract into a fresh directory only. Existing shared
runtime and Python archives remain references in the original author index.
Do not rerun the exclusive recorder in the preserved stage directories.

The actual Moriarty adapters and both funding pilots remain unchanged. The
full pilot's earlier QNT404 receipt remains in `../pilot-compile-stop/`.
The next gate is a reviewed compilation view that adds the explicit
`consumption.AuthorityKey` import to the affected original and factored
adapters while preserving all original files and semantic bodies. It must
retain paired provenance and pass the prescribed checks before paired
generated-input inspection. The toy result does not establish that this
full-model remedy succeeds, that factoring reduces checker resources, or
that Candidate A is safe. A4/A5, Council and integration remain open.
