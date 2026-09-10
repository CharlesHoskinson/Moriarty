# One Preview loan command-result attempt

PROPOSED ONLY. No admission, dispatch, wallet read, network request or service start has occurred for this successor. The plan keeps deadlineMs=0 and is deliberately non-executable until the admitted executor derives its bounded deadline.

Reuse the corrected `experiments/moriarty-midnight-financial/ledger/preview-bootstrap.mjs --run --plan <absolute path> --sha256 <exact digest>` entrypoint. The executor is the original loan executor with new names and updated prior-charge accounting. It also verifies every proposal packet hash against admission before arming.

The proposal binds the frozen exit-completion source candidate. Fresh GPT6 and Grok source reviews must bind that exact source digest; fresh resource votes must bind this exact proposal, including its executor/plan/commands/static-check hashes. An exclusive execution-admission.json must retain both reviewers' normalized decisions and actual raw-review hashes. No missing review or old approval authorizes this attempt. Admission must include every source file, packet file, referenced raw review and both existing full-build reviews required by the executor. The proposal runtimeClosure additionally binds all27 local literal-import modules plus the existing9 PIN_PATHS and wrappers. Its sole extra imported generator beyond the CLI candidate is byte-identical to the original reviewed loan candidate. Installed SDK hashes and retained full build artifacts remain checked by the existing executor/loader.

After both reviews and admission, the existing one-shot invocation is:

```sh
python3 /home/charl/Moriarty/.worktrees/sp05-preview-exit/deliverables/sp05-financial-integration-2026-09-09/preview-loan-exit-01/execute-once.py
```

One consumed attempt creates `sp05-preview-loan-exit-20260910-01`. Maximum4 submissions,2e15 SPECK native DUST,20e9 USD_TEST_ASSET gross. It uses the original Preview seed/address/cache, existing roles/password, and pinned full loan build. No private content was read during preparation. Current wallet synchronization and spendable DUST remain dispatch-time gates. Original snapshots/backups/stores and all old allocations remain intact; no chmod, fallback creation, state rollback or retry is granted.

Timing stays unchanged: activation/setup120s; operation deadline arming+2490s; bootstrap hard exit by+2496s; independent kill timer+2500s; terminal containment planned by+2531s inside2550s. Manager TimeoutStartSec is derived from the remaining setup window and its exact argv is durably retained before activation. Prover is the only startable existing container:5GiB memory/10GiB combined swap/4CPU/restart=no/loopback16300. Launcher4GiB/no swap/2CPU. Local node and indexer remain stopped. The executor never cancels the timer.

Known cumulative reservations before this attempt:19 and5700000000000019 SPECK. With this new ceiling: at most23 reservations and7700000000000019 reserved-plus-cap SPECK. Known admitted ceilings total8000000000000010 before and10000000000000010 with this attempt. These are not global paid-fee totals: all older unquantified Preview, compiler, proof, service, failed-attempt and review charges remain consumed.

During the attempt retain every public native identifier from `run/public-transactions/*.json` and stage receipts, and deliver actual pending/confirmed/unknown notifications through the existing Moriarty notification workflow. Only public fields are eligible; raw sdk.stdout/sdk.stderr remain private. Stop on the first ambiguous result, failure, cap or deadline. No resubmission follows.

After the launcher becomes terminal, capture its actual main exit independently:

```sh
systemctl --user show moriarty-sp05-preview-loan-exit-01.service --property=ActiveState,SubState,Result,ExecMainCode,ExecMainStatus,MainPID,ControlGroup
```

Retain the exact selected properties with observation time. Main exit0 requires ExecMainCode=1 and ExecMainStatus=0; a result file alone cannot establish this. Read at most the final8192 bytes of the private stdout, select exactly one final JSON object with exactly `status,containmentComplete,networkAcceptance,proofAcceptance,financialAcceptance`, require status FINANCIAL_COMPLETE and all four booleans false, and retain that exact public line plus its SHA256. If absent/malformed/ambiguous, retain a closed missing-summary classification and the raw private log unchanged; do not invent stdout from integration-result.json. No arbitrary SDK line or private diagnostic is published.

Copy the exact public plan/preflight, all four stage summaries, integration-result.json, reservations, and public native transaction bytes/metadata into a new exclusive actual-run/ evidence directory. Record source/destination hashes and preserve every partial result if main exit is nonzero. Validate all four full financial/state/native effects, cumulative fee<=admitted cap, false acceptance flags, successful persistence/stop, zero pending and the source's FINANCIAL_COMPLETE/containment=false relationship. Retain source pin, admission, actual resolved argv, stdout and actual exit as distinct evidence.

Regardless of command exit, independently observe terminal launcher state and cgroup absence, proof server exited, and node/indexer still exited. Keep the original forced stop timer active until these observations are retained. If the proof service remains active after normal termination, the existing admitted timeout/stop/kill pattern applies; no new runtime allowance is granted. Only after independently retained containment may the timer be canceled, with its own public record. CLI0 never cancels containment or substitutes for it.

Finally obtain separate GPT6 and Grok actual-result reviews. This packet itself provides no acceptance, successful command exit or financial execution evidence. A repeated fresh deployment uses the installed native deployment address randomization; it does not reset either prior loan or swap.

Source-only checks (safe; never import the executor):

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/preview-loan-exit-01/check-static.py
```
