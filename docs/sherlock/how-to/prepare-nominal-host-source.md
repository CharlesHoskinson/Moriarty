# Prepare a nominal host artifact

The nominal source profile exposes closed-entry preparation through the selected `mori compile` handler. This cut has observed handler/API checks; the complete backend-linked CLI invocation below has not been executed end to end. It is usage syntax for the integrated source, not a new run receipt.

```sh
mori compile program.mori \
  --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-nominal/1 --entry \
  --execution-config nominal-config.json --target target.json --out NEWDIR
```

Use explicit parser extents, owner bounds, HostOnly layout, extension capacities and fuel/stack inputs in `sherlock-nominal-host-config/1`. The checked-in handler fixtures supply concrete source/config/target examples. There are no numeric defaults. The target is a descriptor; it does not select a network or native backend. `--action` conflicts with the nominal entry and nominal `--observe` is unsupported.

The handler prepares and admits the program, then writes `NEWDIR/sherlock-artifact.json`. Inspect its qualification, `body_typechecking`, plan dispositions and evidence fields. Generic expression-body typechecking is not performed and no source function is evaluated. This artifact cannot be used as a native proof, executable reopen/checkpoint or deployment result.

[Observed evidence](../../results/sherlock-public-nominal-compile-20261019/README.md) retains the five direct-handler tests and two full artifact captures. Wrapped-countdown has one complete23-field comparison after explicit format alignment; the mutual case does not have an independent complete-table oracle. Existing frozen source-reference docs retain their historical source-only cut.
