# Shared final A2/A3 regressions

One final source/input snapshot binds both accepted lifecycle closures to forty
boundary tests, fifteen adapter tests and 441 Python tests. All three commands
terminated with exit0. No captured source/input or pinned tool changed before
or during any command. These are finite tests, not model checking or Council.

The source snapshot contains 2673 files: a conservative tracked Moriarty source
and input superset, all eight final lifecycle modules, the recorder, nine graph
inputs and the explicit external DeFiFormal test inputs. Paths and exact original
bytes are retained in source-inputs.tar.gz inside regression-evidence.tar.gz.
The snapshot sourceCommit describes HEAD at capture; its eight unit hashes also
bind then-uncommitted final installment files now accepted at bfc7832. Swap Task3
is accepted at 926b350. Commit ancestry alone is not the source-byte identity.

Reproduction requires restoring the captured repository and external fixture
paths, or explicitly mapping their archive paths into the same test environment.
Python distributions are recorded by installed version, not archived virtualenv
bytes. The interpreter, Node, Git, Quint CLI and Rust backend have binary hashes;
this is not a hermetic operating-system/toolchain image. Rust version provenance
is an installation-directory label, with its unsupported version probe disclosed
in prior receipts. The recorder preserves separate raw stdout and stderr.

Boundary runtime: 40 passing, 127683ms test execution, 786.069965825s command wall.
Adapter runtime: 15 passing, 30624ms test execution, 156.569805865s command wall.
Python runtime: 441 passed in20.24s, 20.793576739s command wall.
The slower Quint wall times include compilation. No duplicate jobs were started.

The validation command checks every archived original, all inner snapshot bytes,
command identities, terminal results, streams, tool identities and final unit
pins. Local A2/A3 requirement acceptance is mapped separately in acceptance.json.
A4 export/replay, A5 bounded verification, A6 Council and A7 integration remain
open; this artifact does not close S02 or any XML release gate.
