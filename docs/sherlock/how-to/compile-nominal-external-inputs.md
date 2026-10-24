# Compile a nominal function with external arguments

The public paired-input path has been exercised through the real shared Args,
reader, selected compile handler, publisher and qualified normal Core. The
complete backend-linked CLI executable remains unrun for this source cut.

The new public examples contain recursive generic UInt/Bool inputs and full ADT
results. To select the nonempty example with a built `mori` executable:

```sh
mori compile crates/mori/tests/fixtures/nominal-external-inputs/nonempty-backfold.mori \
  --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-nominal-body-typed/1 \
  --entry --nominal-function gather \
  --nominal-inputs crates/mori/tests/fixtures/nominal-external-inputs/nonempty-backfold.inputs.json \
  --execution-config crates/mori/tests/fixtures/nominal-external-inputs/config.json \
  --target crates/mori/tests/fixtures/nominal-external-inputs/target.json \
  --out NEWDIR --observe
```

Use a new output directory. For the contrasting empty case, select
`empty-cargo.mori`, `empty-cargo.inputs.json` and function `settle`, with the same
fixed config/target. The target file is a descriptor; this HostOnly path does
not invoke a backend. Omit `--observe` for preparation without owner execution.

Read `sherlock-artifact.json`. With observation, `preparation` contains the
complete admitted program and `nominal_binding`; `execution` contains the actual
summary/image. Values remain visible concrete entry constants. Keep the complete
artifact when comparing recursive values; leaf/count summaries lose type, field
and ordering information.

[Results and retained evidence](../../results/sherlock-nominal-external-inputs-20261020/README.md)
distinguish observed admission/Halted from independent comparison and full
backend/native/proof/target acceptance. The [reference](../reference/nominal-external-inputs.md)
describes the exact input shape and refusal stages.
