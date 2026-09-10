# Source candidate02: lexical diagnostic order

Implementation candidate; independent correction/result audits remain pending.
This repairs GPT-6 source69 R1 without reopening D1–D4 or adding source syntax.
The review found one medium diagnostic-order defect. Its C1 parser phase and C2
outer transport bound controls passed; final D5 acceptance is still a review gate.

## Change and reproduced evidence

`access_field(unknown(), "bad field")` previously selected the later field
metadata's SOURCE_LITERAL_SHAPE instead of the earlier SOURCE_CALL.
`access_field(i128(1+2), "bad field")` selected the later metadata span instead
of the earlier malformed literal expression. Nested access_field preserved the
same defect. All three public APIs reproduced all three witnesses: nine failed
cases in `correction-red-01.tap`, alongside27 passing controls.

After checking arity, expression-source-types.ts now lowers the first operand
once before checking second-operand field metadata. It retains the lowered record
for node construction. This preserves phase4 lexical order and leaves whole-Core
typing in phase5. Thus `access_field(missing, "bad field")` still reports source
metadata failure before the missing identifier's later Core typing failure.
No evaluation happens during lowering, and no node/work change results.

Only two product files change relative to original commit
`894caefe5359d141889696e0da0be0283b0ebce2`:

- `experiments/moriarty-language/src/successor/expression-source-types.ts`:
  reorder access_field lowering, with no grammar/type/runtime changes.
- `experiments/moriarty-language/tests/expression-source-v1.test.mjs`:
  36 real-source tests across elaborate/check/evaluate. Each asserts the full
  rejection, including exact UTF-8 span after multibyte text, path and zero work.

Adjacent class inspection: binary/rounding/index children lower left-to-right;
amount/shares/rate/price validate literal operands in source order; record and
collection fields/items map in source order; generic type metadata precedes its
call arguments as authored. Existing arity admission precedes operand traversal.
No second production defect was found in these paths. Added controls exercise
the literal, index and floor-div paths, first-operand Core-type versus later
source errors, and arity versus nested source errors. Existing tests retain
successful AccessField lowering, all40 coverage, spans, dead branches and rollback.

## Preservation and qualification

The original worktree remains at its original commit with its review artifacts;
it was not edited. Original `source-candidate-01.json`, proposals, source patch,
verification outputs and IMPLEMENTATION-01 remain byte-for-byte unchanged here.
The original manifest describes that original commit, not this corrected tree.
For its two changed pins, `candidate-01-original/` additionally stores the exact
old source/test bytes at their original relative paths, beside a copied original
manifest. All other original manifest paths still match their original bytes.
The copied raw GPT-6 review prompt/start/terminal and extracted verdict are retained
unchanged; Grok's original review was still running at this correction freeze.
No old verdict is presented as approval of these corrected bytes.

Verification performed in isolated worktree
`/home/charl/Moriarty-expression-source-correction`, branch
`feat/sp02-expression-source-correction`:

| Command | Result | Retained output |
| --- | --- | --- |
| node --test --test-name-pattern='source diagnostic lexical order' experiments/moriarty-language/tests/expression-source-v1.test.mjs (before fix) | exit1;27 pass,9 fail | correction-red-01.tap |
| same targeted command after fix | exit0;36 pass | correction-green-01.tap |
| npm --prefix experiments/moriarty-language run build | exit0 | correction-build-01.txt |
| npm --prefix experiments/moriarty-language test | exit0;506 pass,7 suites,0 skip/fail | correction-package-tests-01.txt |
| npm --prefix experiments/moriarty-language run expression-demo | exit0; counter12, funds100, QuoteNotice, B40, remaining960 | correction-demo-01.txt |
| git diff --check | exit0 | correction-diff-check-01.txt |

No financial/native/Preview campaign ran. Atomic/funded and original source
profile regression checks remain in the passing package suite. Full agreement
authoring, financial semantics, K/proof correspondence, mandatory PCD and full
SP01–SP12 acceptance remain open. The pure48 source proposal is separate and no
future source functionality is included in this candidate.
