# Experimental bounded-atomic frontend

Requires Node.js 24 (native TypeScript stripping) and an installed `tsc` executable.
There are no npm runtime dependencies. `npm run build` and `npm run typecheck`
invoke the actual TypeScript compiler with strict checking and `noEmit`; execution
uses the TypeScript sources directly.

```sh
npm test
npm run typecheck
```

The source-driven APIs in `src/frontend.ts` accept original UTF-8 source bytes
(or a JavaScript scalar string) and the exact registered `bounds.json` bytes:

- `parse(source, bounds)` returns SourceAST or one closed Diagnostic. It checks
  complete syntax and stage-4 declaration/AST obligations.
- `check(source, bounds)` returns TypedProgram or one closed Diagnostic through
  typed-program and shape bounds. Manifest bounds are checked by elaboration.
- `elaborate(source, bounds)` returns BoundProgram or one closed Diagnostic.
- `compile(source, bounds)` retains the throwing convenience interface and returns
  `{source, typed, bound, metrics}`. Its FrontendError has the normative code/span
  and a `diagnostic` property containing the closed diagnostic record.
- `parseSource(source)` is the throwing complete-syntax and closed-AST-shape
  convenience API; declaration uniqueness, aggregate bounds, and type acceptance
  require the preceding profile APIs.

The public boundaries construct closed records from original source. They do not
accept caller-provided AST, annotations, Core, or manifests as trusted inputs.

All public bounds-taking APIs admit only the registered 8,861-byte document whose
`MORIARTY-BOUNDS-bounded-atomic/1` digest is
`ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c`.
A matching registry name or schema version is insufficient. Unknown fields,
changed domains or limits, JSON reformatting, missing fields, malformed JSON, and
invalid byte containers reject with the closed `PROGRAM_ENCODING` stage-8
Diagnostic. Oversized strings are rejected before UTF-8 encoding; byte views are
size-checked before copying or hashing. The pinned local registry is verified
before use and returned only as independent copies.

Source validation uses the admitted registry, so earlier source errors retain
numeric stage priority. `parse` reaches source stage 4, while `check` and
`elaborate` reach the applicable stages through 7 before bounds admission at 8.
Tests with deliberately reduced limits call explicitly nonadmitted internal
validation/lowering helpers; there is no public bounds-override option.

`derive` recompiles against the registered bounds and uses those registered limits
for execution. The backend entry point checks the source/program/registry binding
before authentication or commit callbacks. A forged Core/manifest produced using
an altered internal test configuration cannot pass this admission boundary.

`canonicalEncode` and `canonicalDecode` are generic canonical JSON value tools,
not profile record acceptance validators. They reject unsupported scalar values,
noncanonical bytes, duplicate keys, and sparse/decorated/accessor containers.
`decodeCanonicalRecord(bytes, validate)` requires an explicit schema validator;
the callback must enforce exact required/unknown fields and applicable bounds.
No generic value decoder establishes acceptance of an arbitrary Moriarty record.

Expression parsing uses explicit stacks. Redundant parentheses do not consume
semantic depth or the JavaScript call stack. The source-byte, aggregate AST, and
semantic expression limits continue to apply jointly. Tests cover 12,000 redundant
parentheses, a 1,500-level structurally deep rejection, and semantic depths 16/17.

Stage preflight checks declaration/AST errors across the source before resolution,
and resolution before type/policy/status errors. It selects one diagnostic by
stage, byte span, and code. Independent stage-6 errors and global stage-7 encoding
bounds have rejection controls. This is incremental implementation evidence, not
an admitted complete frontend correspondence result.

The two previously disclosed ordering gaps are fixed. The parser retains generic
settlement literals in a separate internal syntax-tree type until complete parsing
finishes. The stage-4 shape gate then requires TextLiteral/AmountLiteral and orders
malformed settlement declarations against other declaration errors. Invalid shapes
never escape the public boundary as SourceAST. Valid shapes with an empty asset or
zero quantum remain semantic stage-6 errors.

Lexically valid keywords in identifier positions produce PARSE_ERROR at stage 3.
Forbidden token spellings such as constructor/prototype produce LEXICAL_TOKEN at
stage 2. Numeric stage priority applies before source position: a genuine lexical
error later in the source outranks an earlier contextual keyword misuse. Within
one stage, source-position ordering is preserved. Regression tests cover both
orders, multibyte text before the offending token, declaration errors mixed with
later syntax errors, and Const/State/Episode/Guard/BareAmount distinctions.

These tests establish the named rejection predicates. They do not prove exhaustive
parser/checker/lowering correspondence or complete diagnostic coverage. Exact
registered-bounds admission is enforced; arbitrary alternate registry documents
are rejected rather than treated as configurable profile inputs.

The source specification and original profile-04 materializations remain unchanged.
Tests compare the complete canonical AST, TypedProgram, and BoundProgram bytes of
both examples with those artifacts. No proof, PCD, native lowering, financial
conformance, or durable acceptance result follows from frontend tests.

## Run the developer example

From this package directory, run `npm run demo`. Use
`node examples/simulate.mjs --json` for the complete inputs and candidate traces.
The example reads the original `spec/examples/loan.mori` and `swap.mori`
source files through the parser, checker, elaborator and evaluator.

The loan uses an unsigned exact-plan construction followed by exact comparison;
the swap uses a gross debit cap and minimum net output. Each example shows two
transitions and an adverse input. The loan episode closes with its two dues
settled and 4,500,000,000 micro-USD notional still outstanding. The swap outputs
19,743 AssetB units for 10,000 AssetA units before the provider closes the pool.

These are local simulations with explicit simulated authentication. Every attempt
to use acceptance without a deployment-owned proof backend returns PROOF_INVALID.
The example does not sign an authority or move assets. `src/lower-compact.ts` and
[compact/MAPPING.md](compact/MAPPING.md) describe the restricted generated numeric
kernels; their generated ledger snapshots do not implement financial settlement.

## Run a computed funded repayment

From `experiments/moriarty-language/`, the financial expression CLI can evaluate a computed
payment and, with `--repayment-state`, prepare the retained Transfer/Repay kernel
in one local step. See
[spec/successor/funded-expression-source.md](spec/successor/funded-expression-source.md).

```sh
node src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema spec/successor/examples/expression-funded-payment.schema.json --snapshots spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state spec/successor/examples/expression-funded-payment.state.json spec/successor/examples/expression-funded-payment.mori
```

The example adds two Quantity arguments, guards a nonnegative sum, updates ordinary
`paid`, and emits the computed cash and nominal. Default 10+20 pays 30 against due
100. Success is local preparation only.

The source-defined agreement profile declares that same schema in the `.mori`
file and runs without `--schema`. See
[spec/successor/financial-agreement-source.md](spec/successor/financial-agreement-source.md).

```sh
node src/cli.ts check --profile moriarty-financial-agreement-source/1 spec/successor/examples/source-defined-payment.mori
node src/cli.ts simulate --profile moriarty-financial-agreement-source/1 --snapshots spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state spec/successor/examples/expression-funded-payment.state.json spec/successor/examples/source-defined-payment.mori
```

The `/2` profile keeps those shared declarations and admits multiple named
actions in one file. Check still inspects every action. Simulate requires
`--action` and executes exactly one selected funded action. See
[spec/successor/financial-agreement-source-v2.md](spec/successor/financial-agreement-source-v2.md).

```sh
node src/cli.ts check --profile moriarty-financial-agreement-source/2 spec/successor/examples/multiple-action-payment.mori
node src/cli.ts simulate --profile moriarty-financial-agreement-source/2 --action repay --snapshots spec/successor/examples/multiple-action-payment.snapshots.json --repayment-state spec/successor/examples/expression-funded-payment.state.json spec/successor/examples/multiple-action-payment.mori
```
