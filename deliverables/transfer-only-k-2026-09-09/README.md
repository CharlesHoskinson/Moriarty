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

## Result audit limits

Both result audits passed: [GPT-6](gpt6-result.md) and [Opus](opus-result.md).
The following are the Opus review's five scope limits, quoted verbatim as requested
by that reviewer. Its claims about what it read describe that review's supplied-text
scope; the separate GPT-6 audit inspected the actual files and compiled artifacts.

> 1. **Codec reconstruction, not K evidence.** For Transfer-only, K never emits obligation or allocation-ID fields; `codec.decode` copies them from the input. "Debt and allocation history preserved" is therefore established *by construction on the K path*. It is real evidence about the **source** evaluator (which produces its own full record) and about K only insofar as K accepted a one-action packet at one work unit. Phrase the lesson as design intent plus source evidence, not as a K result.
> 2. **Artifact re-verification.** `compiledArtifactsUnchanged: 177` is a run-time assertion inside `run.py`/root. `check-k-results.py --retained` skips the artifact comparison when `.build` is absent, so the retained packet alone cannot re-establish it. Also read "unchanged" as unchanged since *this* compile, not across earlier suites (which had 174 artifacts under a different definition).
> 3. **Unreviewed bytes in my scope.** `language-tests.txt` (236 passes), `codec-green.txt` (12), `execution.stdout`, and the non-empty `compile.stderr` were supplied as hashes only. All `trace-*.stderr` and `compile.stdout` are the empty-file hash.
> 4. **Provenance nit (non-blocking).** `root-admission.json` binds `opus-preexecution.json`; the README links `opus-preexecution.md`, whose bytes are not in the admission record. Both are in the manifest. Treat the `.json` as the bound vote.
> 5. **Domain.** Finite agreement over 16 records inside the 2-balance / 1-allowance / 1-obligation, identity-conversion, empty-history projection. Not correspondence, not a semantic freeze, not ProRata/rounding/multi-action, not ledger settlement.

Root disposition: these limits constrain publication. For this path, debt
preservation is a property of the complete K-plus-codec result: K emits no debt
changes and the trusted codec copies those fields. It is not an independently
computed debt output or a proved invariant of K. The raw changed financial values
are computed by K and compared separately. General correspondence remains open.

Two review wording corrections are retained here without editing the original:
there are sixteen raw result terms (four `preparedTransfer`, three `prepared`, nine
`rejected`), not ten. The codec accepts a synthetic `<out>`-only wrapper in offline
tests; it does not require every generated cell to exist. Actual retained outputs
include an empty terminal `<k>` cell, which GPT-6 independently checked. Finite
input variation alone does not rule out host computation or hardcoding; source
inspection, exact compilation bindings and actual command evidence support the
scoped computation claim together. The bound Opus preexecution vote is
[the JSON receipt](opus-preexecution.json); its Markdown is a readable rendering.
