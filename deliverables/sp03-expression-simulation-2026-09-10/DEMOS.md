# Shipped fixture demonstrations

```text
node src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema spec/successor/examples/financial-vault-quote.schema.json --snapshots spec/successor/examples/financial-vault-quote.snapshots.json spec/successor/examples/financial-vault-quote.mori
exit 0
{"judgmentResult":"SourceSimulated","sourceProfile":"moriarty-financial-expression-source/1","pre":{"last":"0"},"initialWork":"1000","result":{"status":"ExpressionPrepared","post":{"last":"1"},"descriptors":[{"operation":"Notice","fields":{"allocation":"1","offered":"4"}}],"workRemaining":"959"}}

node src/cli.ts simulate --profile moriarty-expression-source/1 --schema spec/successor/examples/expression-counter.schema.json --snapshots spec/successor/examples/expression-counter.snapshots.json spec/successor/examples/expression-counter.mori
exit 0
{"judgmentResult":"SourceSimulated","sourceProfile":"moriarty-expression-source/1","pre":{"counter":"10","funds":"100"},"initialWork":"1000","result":{"status":"ExpressionPrepared","post":{"counter":"12","funds":"100"},"descriptors":[{"operation":"QuoteNotice","fields":{"amount":"5","count":"2"}}],"workRemaining":"960"}}
```
