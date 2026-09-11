# Expression source command line

The existing SP02.3 adapter checks and formats the explicit expression-source/1
and financial-expression-source/1 profiles. This SP03.2 addition locally
simulates those profiles and awaits its scoped result audit. It does not add
language semantics or execute financial operations. Each explicit profile
selects its own reviewed expression source API.

Run from the repository root with exactly these positional argument forms:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-expression-source/1 --schema SCHEMA_JSON SOURCE.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-expression-source/1 SOURCE.mori
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-expression-source/1 --schema SCHEMA_JSON SOURCE.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-expression-source/1 SOURCE.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-expression-source/1 --schema SCHEMA_JSON --snapshots SNAPSHOTS_JSON SOURCE.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema SCHEMA_JSON --snapshots SNAPSHOTS_JSON SOURCE.mori
```

Paths resolve against the working directory. SCHEMA_JSON is the trusted canonical
schema JSON consumed by the selected source factory; it is not a type-alias registry.
The financial profile requires its runtime's variantTypes schema map. Its
factory/formatter are createFinancialExpressionSourceV1/formatFinancialExpressionSource;
the original profile uses createExpressionSourceV1/formatExpressionSource.
The --profile value is the literal profile identifier, not a path or inferred
source header. Flags appear in the shown order, once each. Unknown commands,
missing/duplicate/extra/reordered flags and empty arguments reject CLI_USAGE.
Unknown --profile values reject CLI_PROFILE before opening files. Formatting
accepts no schema; checking requires one. There is no stdin, output-file or
in-place mode, profile inference, callback, or financial execution.
Use a path such as ./--example.mori for a filename beginning with --.

Simulation takes the exact ordered form `simulate --profile PROFILE --schema
SCHEMA_JSON --snapshots SNAPSHOTS_JSON SOURCE.mori`. It is a local-only adapter
for the selected source API's `evaluate(source, snapshotCanonicalJSON)` method.
It accepts neither caller-supplied Core nor a check-only shortcut. The source
profile and trusted schema select the same factories as checking. Snapshots are
canonical JSON with exactly `Pre`, `Args`, `Obs`, and `workInitial`, as defined
by that API. The CLI does not compile or run K, create a proof, contact a
network or service, or perform a wallet or ledger action.

On success it writes exactly one LF-terminated JSON record to stdout, no
stderr, and exits0:

```json
{"judgmentResult":"SourceSimulated","sourceProfile":"PROFILE","pre":SNAPSHOT_PRE,"initialWork":"SNAPSHOT_WORK_INITIAL","result":API_SUCCESS_RESULT}
```

`PROFILE`, `SNAPSHOT_PRE`, `SNAPSHOT_WORK_INITIAL`, and `API_SUCCESS_RESULT`
stand for their JSON values. `result` is the exact successful result returned by
the selected API. The wrapper does not add a success `workUsed` field; consumers
can derive consumed work from `initialWork` and the exact result's
`workRemaining` where present. The CLI parses the snapshots for `pre` and
`initialWork` only after evaluation succeeds. An API record with
`status: "Rejected"` is written unchanged to stderr, stdout stays empty, and
the command exits1. In particular, successful `status: "ExpressionPrepared"`
is not a rejection. Rejected output contains no CLI-created tentative post-state
or descriptor fields.

Checking writes the exact successful SourceChecked API record as one JSON line
to stdout, followed by LF. It writes no stderr and exits0. A language rejection
writes the exact API Rejected record, without added/removed fields, as one JSON
line to stderr and exits1; stdout is empty. This includes its code, span, nodePath
and workUsed. Check and format accept no AST, Core, or snapshot. Simulation
accepts only its snapshots file; no command accepts caller-supplied AST or Core.

Formatting writes exactly the selected profile formatter's result to stdout and exits0,
with no stderr. It uses the same parser and formatter as the API, so it does not
perform trusted-schema checking. Syntax or formatted-output-bound failures write
one JSON Rejected record
to stderr with the SuccessorSyntaxError code and source byte span, empty nodePath
and workUsed="0", then exit1. A decoder error has no valid source span and is a
CLI transport failure instead. Formatting never writes the source file.

CLI admission/read failures write exactly
`{"status":"CliRejected","code":CODE,"input":INPUT}` as one JSON stderr line,
with empty stdout. INPUT is arguments, schema, source or snapshots. Codes/exits are:

| Code | Input | Exit | Meaning |
| --- | --- | --- | --- |
| CLI_USAGE | arguments | 2 | Invocation differs from the forms above |
| CLI_PROFILE | arguments | 2 | Explicit profile identifier is unsupported |
| CLI_IO | schema, source or snapshots | 2 | File cannot be opened/read or is not regular |
| INPUT_BOUND | schema | 1 | Schema exceeds65536 bytes |
| SOURCE_BOUND | source | 1 | Source exceeds65536 bytes |
| INPUT_BOUND | snapshots | 1 | Snapshots exceed2,000,000 bytes |
| INVALID_UTF8 | schema, source or snapshots | 1 | File bytes are not valid UTF-8 |
| CLI_INTERNAL | arguments | 2 | Unexpected internal failure; no host error detail exposed |

All failures have empty stdout; no partial formatted output is emitted. No
exception stack, OS path/error detail or file contents are added to diagnostics.
Unexpected internal errors cannot be reported as a successful source check.

Admission order: validate complete CLI shape, validate --profile; for check read
and decode schema then source, for format read and decode source, and for
simulation read and decode schema then source then snapshots; call the selected
API.
Transport admission necessarily precedes language phases. After valid transport,
check results equal the API exactly. A schema file with a trailing newline is not
canonical and the existing API determines its rejection. A BOM is retained by
the fatal UTF-8 decoder and handled by the existing parser/canonical admission.

The reader follows syntax-cli.ts's existing regular-file pattern: open with
O_RDONLY|O_NONBLOCK where available, check the opened descriptor with fstat,
read at most65537 bytes for schema/source or2,000,001 bytes for snapshots,
reject if the extra byte exists, and close in finally.
Directories, devices and FIFOs reject; stdin is not opened. Symlinks to regular
files are allowed, with type checked on the opened descriptor. No unbounded
readFile shortcut or stat-then-open assumption is used.

## Implementation and verification plan

Create src/cli.ts as a small direct-entry adapter using existing public source
APIs and the bounded reader pattern. Preserve successor/syntax-cli.ts unchanged.
Create tests/expression-cli.test.mjs with real subprocess invocations against
temporary regular files, compared to API check and formatter results. Verify
success, wrong-type/dead-branch/R1/UTF-8 source spans, missing/extra/reordered
arguments, unknown profile, source header mismatch, malformed canonical schema,
nonregular/missing paths, invalid UTF-8, exact65536 and65537-byte bounds, formatting
idempotence and unchanged source/schema bytes. Include file paths with spaces.
Run package build/test and real check/format/simulate demo commands, retaining failures
and outputs under a separate CLI delivery record. Freeze separately for audit.

This adapter does not close SP02: full agreement/schema authoring, multi-action
selection, genesis/constants, missing financial source families, complete CLI
surface and actual-participant syntax evaluation retain their existing owners.
The source40 and pure48 candidates retain their separate acceptance gates.
