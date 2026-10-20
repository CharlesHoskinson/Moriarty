# Observe a body-typed nominal program

The production Args and selected-handler route passed ten new tests, one changed
legacy test and two retained observations using a newly qualified full normal
Core library. See the [ordinary results](../../results/sherlock-nominal-host-observe-20261020/README.md).
Both complete source-symbolic terminal results also matched separately
prefrozen independent expectations. The complete backend-linked `mori` executable
was not built in these checks.

Select the existing body-typed source profile and explicit observation:

```sh
mori compile program.mori \
  --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-nominal-body-typed/1 --entry --observe \
  --execution-config nominal-config.json --target target.json --out NEWDIR
```

Use the same explicit [nominal configuration](../reference/nominal-compile-command.md).
The source declares `language "moriarty-sherlock-nominal/1"`. Supported closed
programs pass the existing bounded parser, adopted all-body judgments and full
owner admission/genesis; observation runs that exact returned pair once. No
fixture name, evaluator result or replacement literal selects the route.

Read `NEWDIR/sherlock-artifact.json` using the separate
[observation format](../reference/nominal-host-observation.md). Its `preparation`
is the unchanged compile artifact, whose evaluation flag stays false. Actual
post-run state is under `execution`. A completed owner summary can be Halted,
Rejected or Exhausted; completion alone does not mean source success. The two
retained positive specimens reported Halted; separate real tests exercised
checked-subtraction rejection and zero-fuel exhaustion.

Omitting `--observe` retains compile-only behavior. Structural nominal and Draft
observe requests still refuse before config reading. Native export, proof,
financial support, H1 and compiler correspondence remain separate qualifications.
The target is a descriptor; this HostOnly path invokes no native backend. These
internal maintenance checks do not add permissions for public command use.
