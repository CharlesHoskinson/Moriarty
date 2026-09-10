# Financial expression source and CLI implementation candidate

Status: pending fresh independent GPT-6 Astra and Grok4.6 source/result audits.
No author acceptance, financial transition or ledger claim is made.

The explicit `moriarty-financial-expression-source/1` profile now connects real
`.mori` through checked Core to the published48-constructor expression runtime.
It implements approved P1–P5, including the P4 ARITH_DENOMINATOR correction.
The preserved proposal02 SHA256 is
`e843239755bc02925215b7419ce89da416da5e022a465e6fec931369ed985361`;
DECISION-02 and both original/narrow review records remain unchanged.

The candidate also contains the pending SP02.3 CLI adapter and explicit routing
for both source profiles. Review the CLI contract/transport/result handling as
one scope and the financial expression source extension as another. Both require
fresh implementation/result review; prior design votes are not those results.

## Observable behavior

- Eight added constructors are authored through the approved generic calls,
  Option/Variant projections, scalar accessors and lazy conditional expression.
  The source fixture financial-all48.mori reaches every constructor and evaluates.
- UInt256 and all five indexed numeric intermediate types, plus Variant, are
  source-expressible without aliases, including nested records/options/collections.
- Literal amount/shares retain their old one-node forms. Generic dynamic forms,
  explicit width conversions and unit projections retain all nodes, spans and work.
- Whole-source static checking includes both conditional arms. Dynamic execution
  enters only the selected arm; exact types, paths and work are retained. The
  original source/1 binders and admission catalogs remain separate.
- Additive financial runtime `.check()` performs schema/shape/whole-Core typing
  without snapshots or reduction; `.evaluate()` retains its existing path/result.
- `financial-vault-quote.mori` computes floor4×3/10=1 with explicit256-bit
  intermediate arithmetic, constructs holder-indexed shares and a Variant, writes
  an ordinary last field and emits a local quote descriptor. B43, actual work41.
  Selecting an absent Option rejects OPTION_NONE at source span397..417,
  nodePath[2,0,1], workUsed19; no state/descriptors are published.
- Nineteen retained independent vault arithmetic cases run through actual source:
  four direction/rounding rules, full UInt128 products, narrowing overflow, input
  bounds, small/zero results and explicit initialized-domain guards. This is
  local arithmetic evidence, not vault solvency, funding or a financial operation.

## Verification and retained failures

Commands ran in `/home/charl/Moriarty-financial-expression-source`, branch
feat/sp02-financial-expression-source, originally based on integrated source02
commit2eb6f41. The CLI commits were separately cherry-picked before P48 edits.

| Command | Result | Output |
| --- | --- | --- |
| node --test .../financial-expression-source-v1.test.mjs before implementation | exit1, new module absent | red-01.txt |
| npm --prefix experiments/moriarty-language run build | exit0 | build-02.txt |
| npm --prefix experiments/moriarty-language test | exit0;678 tests,7 suites,0 failures/skips | package-tests-02.txt |
| node experiments/moriarty-language/examples/financial-expression-source.mjs | exit0; actual quote and selected-None behavior | demo-02.json |
| node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/financial-vault-quote.schema.json experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori | exit0; SourceChecked B43 | cli-check-01.json |
| node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-expression-source/1 experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori | exit0; formatted source | cli-format-01.mori |

Earlier outputs remain: green-02 contains a test-fixture host numeric JSON value
rejected by the canonical serializer; green-04 contains the vault test's reserved
parameter name quantity. The fixtures were corrected to canonical string data
and the explicit source parameter q. Neither was a production defect. Green-01,
green-03, profile-01 and green-05 retain their scoped successful runs; final tests
include later endpoint and conditional direct-Emit checks. CLI's earlier absent
file and Node strip-only failures remain in its original delivery record.

The first staged product diff check found a blank final line in the new grammar;
it was removed without changing productions. `product-diff-check-02.txt` records
exit0 after that cleanup. `full-diff-check-02.txt` records exit2 for whitespace
in preserved raw reviews/build/test output; those evidence bytes are not rewritten.
Earlier diff outputs are retained.

The full suite includes both older source profiles, original expression runtime,
published financial pure runtime, atomic/funded regressions, all48 source coverage,
all new type forms and scalar overloads, exact metadata errors, signed minima,
wrong-type/dead branches, selected/unselected failures, work/spans/rollback,
variant/schema bounds and cycles, parser/formatter bounds and association, hostile
transport/result ownership, and real CLI/API equality for both profiles.

## Preservation and limits

The source40 runtime/source files remain unchanged except the shared frontend and
formatter's profile-gated additions and the package demo entry. The financial
runtime change is only additive static `.check()` plumbing. No financial type
or reduction rule was altered. Original runtime/source manifests and all review
inputs still describe their original commits. `original-profile-files/` archives
the exact previous shared/parser/formatter/package/runtime/CLI bytes with a
commit/hash map in preserved-profile-files-01.json. Original CLI candidate pins
are preserved at its original commit and these archives, not relabeled as the
extended CLI candidate. Root README and its grammar mirror were not edited.

No financial/native/Preview campaign ran. Check/format/evaluate remains one-action
trusted-schema expression authoring. Source-defined full schemas/financial identity
and denomination authority, genesis/constants, multiple actions, identity escaping,
full financial source families, CLI completion and actual-participant syntax
evaluation remain SP02 obligations. Financial transitions, K/proof correspondence,
mandatory PCD, ACTUS/DeFi and Preview acceptance retain their existing gates.
