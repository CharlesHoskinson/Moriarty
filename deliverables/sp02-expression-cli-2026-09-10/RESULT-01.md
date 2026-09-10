# SP02 expression check/format CLI candidate

The new `experiments/moriarty-language/src/cli.ts` provides explicit-profile
checking and formatting for actual `.mori` files through the existing /1 APIs.
This is an unaudited implementation candidate; publication remains pending.

The exact contract is
`experiments/moriarty-language/spec/successor/expression-cli.md`, recorded before
adapter implementation. Check requires a trusted canonical schema file and emits
the unchanged SourceChecked or Rejected API record. Format uses the existing
formatter with no schema. File transport is bounded, fatal UTF-8, regular files
only; no source/schema file is written. Unsupported profiles reject before files
are opened, and the source header is still checked by the explicit profile parser.
No source type, grammar, lowering, runtime or original syntax CLI file changes.

Isolated branch: feat/sp02-expression-cli, based on
`1124735998b281d197c9435f6d8a201ce72e71c6`.
The correction02 source pins remain unchanged. Full SP02, source40 result
acceptance and future pure48 source decisions remain separate obligations.

## Verification

- `red-01.txt`: before src/cli.ts existed, all10 initial CLI tests failed.
- `green-01.txt`: initial implementation failed all10 because Node strip-only
  execution rejects TypeScript parameter properties. Explicit constructor field
  assignments repaired this; the initial output remains preserved.
- `green-02.txt`: all10 initial real CLI tests pass.
- `build-02.txt`: package build exit0 after that repair.
- `package-tests-01.txt`: full516-test suite passes, including funded/atomic and
  original source profiles.
- `package-tests-02.txt`: final517-test suite passes with an additional formatted
  output expansion control (65148-byte input exceeds65536 after formatting;
  FORMAT_BOUND reports before any stdout is written). No skips or failures.
- `demo-check-01.json` and empty stderr: real example check emits SourceChecked,
  profile moriarty-expression-source/1 and staticWorkBound40.
- `demo-format-01.mori` and empty stderr: real example formatting succeeds.
- `diff-check-01.txt`: git diff --check exit0.

The eleven CLI tests run real Node subprocesses. They compare successful check
records, static/parser/R1 error records and formatter output directly with APIs;
cover UTF-8 spans and malformed/bounded/nonregular inputs, fixed CLI argument
admission and profile rejection, no import side effects, source byte preservation,
and exact source/schema65536 versus65537-byte transport. File paths contain spaces;
regular symlink targets work; FIFO and device inputs reject without blocking.

No financial/native/Preview campaign ran. No tests assert that model/design
approval substitutes for full agreement authoring or financial acceptance.
