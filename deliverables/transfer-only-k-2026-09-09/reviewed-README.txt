# Transfer-only execution in K

Status: actual K execution passed; independent result reviews and publication acceptance are recorded in `acceptance.json` when available.

This SP03.2 increment adds K execution for an existing funded-source capability:
a Transfer-only action moves cash without discharging an obligation. K charges one
ordinary work unit and checks state invariants before the transfer. Rejections
return only code/index. The successor grammar and source evaluator are unchanged.

The [independent cases](INDEPENDENT-CASES.md) contain ten new Transfer-only cases
and six unchanged repayment regressions. `check-source-cases.mjs` exercises the real
source parser, elaborator and preparation entry point, comparing complete results.
Offline synthetic KAST tests check the codec but are not K execution evidence.

The [resource proposal](resource-proposal.json) permits one fresh bounded compile
and sixteen sequential K invocations after exact candidate/resource reviews and
root admission. Previous four compile attempts and 33 krun invocations remain
charged. No proof, Midnight transaction, semantic freeze or sprint completion is
claimed. General action sequences, rounding, ProRata, public Pending/Complete
observations and full Core/correspondence work remain open.

## Observed result

One compile and all sixteen K invocations succeeded. The ten new Transfer-only
cases produced four Prepared and six Rejected results; the six unchanged repayment
regressions produced three Prepared and three Rejected results. Every complete
record matched independent expectations, source preparation and the actual
successor `simulate-cli.ts simulate` command, including exit codes.

[Raw inputs, outputs and command receipts](attempt-01/) and the
[execution result](execution-result.json) retain the observations. The root checked
all 177 compiled artifact hashes unchanged and all source bindings. The bounded
systemd service completed in 45.714 seconds (root supervisor 45.726 seconds),
with reported rounded memory peak 480.4M and zero swap. The active-service snapshot
confirms 4GiB memory, zero swap, 512-second runtime and SIGKILL containment. Final
transient-service queries are post-collection observations, not active limits.
Cumulative history is five compile attempts and 49 krun calls; the six regression
cases in this run do not add distinct coverage to earlier suites.

The [GPT-6 preexecution vote](gpt6-preexecution.md) and
[Opus preexecution vote](opus-preexecution.md) bind the exact source and resource
proposal before [root admission](root-admission.json). Opus resolved to
`claude-opus-5` at medium effort. Result audits are separate from those votes.

Verification commands (no new K execution):

```sh
python3 -m unittest discover -s experiments/moriarty-language/formal/k -p 'test_*.py'
node deliverables/transfer-only-k-2026-09-09/check-source-cases.mjs
python3 deliverables/transfer-only-k-2026-09-09/check-k-results.py --retained
```

The retained check uses raw saved K output; it cannot recheck absent compiled
artifacts on a fresh clone. Without `--retained`, it also checks local compiled
artifacts when this worktree's retained `.build` exists. It never invokes K.
The [language test log](language-tests.txt) records 236 passes; the
[offline codec log](codec-green.txt) records twelve passes. The
[CLI observations](cli-observations.json) preserve actual output and exit status;
temporary source files were removed after execution, while exact source and
invocation content remain in [source observations](source-observations.json).
The [inspected wiki save](wiki-inspect.json) updates four existing pages, and
[strict lint](wiki-lint.json) reports zero issues. No source or claim ledger was
promoted by these local observations.
