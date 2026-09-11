# Focused green test record

```text
node --test tests/expression-cli.test.mjs
exit 0
tests 16; pass 16; fail 0
```

The three added public-CLI tests cover independent successful results for both
profiles, API rejection preservation and rollback, canonical/UTF-8/BOM and
bound transport behavior, exact argv order, unknown profile before opens, and
nonregular snapshots.
