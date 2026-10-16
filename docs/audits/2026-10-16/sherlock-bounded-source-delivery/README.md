# Two bounded Core-source programs: captured host results

The recorded attempt built the actual production Core library, ran the 14
selected Opus-authored tests, and compared two actual source-program projections
against independently frozen complete expectations. All eight child commands
exited 0. This package records that local capability; fresh final Opus and Muse
source/result reviews and Opus publication review remain pending. Root owns
integration and publication. [RESULT.json](RESULT.json) separates those statuses.

| Fixed source program | Actual argument selection | Independent complete result | Captured fuel runs |
| --- | --- | --- | --- |
| `WeightedTotal.quote` | UInt `(4,9)` | `Done(UInt39)`, 34 charged steps, maximum depth 4 | 36: fuel 0..35 |
| `ThresholdCheck.inspect` | UInt `(7,10)` | `Done(Bool false)`, 32 charged steps, maximum depth 4 | 34: fuel 0..33 |

The actual producer calls production `mori_core::parse`, the bounded
`observe_scalar_action`, existing Core validation and `prepare_scalar_action`,
then the existing `Machine::run_steps`. It does not substitute a parser or
handwritten Sherlock program for the source path. Source inputs are
`moriarty/1` / `moriarty-core/1`; host observations declare `sherlock-k-a/1` at
the post-startup boundary. The Core adapter projects only the final emit value.
Ordered financial emit effects remain in Core and are not executed here.

The [actual comparison](records/bounded-source-01/07-compare-independent-oracles.stdout)
reports two complete program/preparation identities, 17 source occurrences,
68 shared pending states, 70 actual fuel-prefix runs and 1,259 records, with no
first difference. Those records contain 1,189 charged events and 70 free terminal
publications. One complete trace per program contains 66 charged events and two
free publications. Prefix totals repeat work; 1,259 is not the work of one run.
Every rule, charge, initial state, before/after snapshot, environment, saved
frame order, terminal outcome, fuel and no-next declaration is retained.

The expected source arithmetic, preparation reports and complete states were
hand-derived in a separate oracle lane before producer execution. The archived
`SOURCE-NORMAL-PROVENANCE.md` records that isolation, exact expected schedules,
source hashes and freeze hashes. It was not rewritten after execution. Existing
N01–N08 oracle files were not changed. Source/oracle `UNRUN` headers remain
unchanged historical preparation labels; the observed results are in this
separate package.

The [attempt receipt](records/bounded-source-01/result.json) retains every exact
command, argument, environment, timeout, exit and stdout/stderr digest. It records
the production library's 25-source rustc dep-info closure, the five-input test
harness closure and three-input producer closure. The narrow harness exercises
a second instance of the exact bounded-consumer bytes, linked against the real
Core library. The source producer separately exercises the production export.
The [test listing](records/bounded-source-01/03-list-exact-tests.stdout) and
[test results](records/bounded-source-01/04-run-exact-tests.stdout) preserve all
14 exact names, with 14 passed and zero failed, ignored, measured or filtered.
The suite includes the Opus-owned refusal and rejection cases; this package
does not supply a new security judgment.

The producer's caller-stated bounds are local choices, without API defaults:

| Bound | Value |
| --- | ---: |
| Source actions | 8 |
| Source expressions | 64 |
| Source statements | 16 |
| Scalar arguments | 4 |
| Projection occurrences | 64 |
| AST nesting | 16 |
| Stack limit | 8 |
| Step records | 64 |
| Retained slots | 1,048,576 |

The [Opus preflight](records/bounded-source-preflight-opus/REPORT.md) judged H1
satisfied only for these fixed hash-pinned fixtures, literal scalar arguments
and bounded local diagnostic run, subject to its M1–M5 conditions. The
[Muse preflight](records/bounded-source-preflight-muse/REPORT.md) supplies its
separate scoped agreement. Requested/returned identities, effort and terminal
status are retained in [REVIEW-STATUS.json](REVIEW-STATUS.json). These votes cover
the stated execution job, not merge, supported API status or result acceptance.
H1 stays in force. Other inputs, bounds, generic callers or CLI/LSP/MCP/network
wiring do not inherit this judgment. Operation, slot and nesting counts are not
measured byte, wall-time or native-stack guarantees.

Each child used a fresh attempt directory, a 30-second timeout, process-group
termination on timeout, and `env -i` with only `PATH`, `LANG` and a fresh
`TMPDIR`. The receipt claims no host memory or disk cap. The full dependency
inventory contains 2,326 regular files and 14,539,190,934 bytes; before/after
inventories match. Source, snapshot and selected tool inventories also match.
Six direct cached extern artifacts, two proc-macro artifacts, transitive cached
dependencies, the rustc 1.99.0 sysroot and linker retain their recorded local
trust boundary. Hash equality does not establish their source authenticity or
which transitive binaries the linker actually loaded. No cached binary bytes
are published in this package.

The [public archive](public-evidence.tar.gz) contains the exact 75-file attempt
source snapshot, complete actual observation JSON, original attempt receipts,
public command logs, dep-info, harness, tool/dependency inventories and the
public preflight reports/manifests. [ARCHIVE-MEMBERS.json](ARCHIVE-MEMBERS.json)
binds each member; [SOURCE-BINDINGS.json](SOURCE-BINDINGS.json) separates the
75-file contextual snapshot from the smaller actual compile closures. The
original artifact inventory also names excluded compiled objects by hash; it
is not the archive membership list. No executables, rlibs, proc-macro binaries,
14.5 GB cache, private Lean material, credentials, raw provider output or review
prompts are included. Public source names and paths remain unredacted diagnostics.

The [packaging script](package_public_evidence.py) reads only its explicit public
attempt/preflight paths, checks selected source and receipt digests, verifies
actual counts and test membership, builds the archive twice identically, and
checks every archived byte in memory. It executes no captured command. Invocation:

```sh
python3 -I -B docs/audits/2026-10-16/sherlock-bounded-source-delivery/package_public_evidence.py \
  . /absolute/path/to/sherlock-parallel-20261016
```

Actual K execution and source-name correspondence remain open: the source
programs retain opaque host symbols outside the K numeric-name grammar. This
attempt ran no Cargo, K, Quint, Lean, Clean, handwritten circuit, SDK/native,
proof, ledger or target operation. No full source grammar, financial effects,
Candidate B representation/bounds, canonical codec, compiler/circuit refinement,
physical cost, DUST or fee qualification follows. All U0–U7, M1–M17 and MC/SP/G
parent exits remain open; native authority remains zero. Developer use remains
permissionless.
