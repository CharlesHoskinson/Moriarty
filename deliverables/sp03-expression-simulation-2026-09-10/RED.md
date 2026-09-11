# Red test record

Before adapter implementation:

```text
node --test tests/expression-cli.test.mjs
exit 1
tests 16; pass 13; fail 3
```

All three new simulator subprocess tests failed because the public CLI returned
`{"status":"CliRejected","code":"CLI_USAGE","input":"arguments"}` for
the exact `simulate --profile ... --schema ... --snapshots ... SOURCE.mori`
form.
