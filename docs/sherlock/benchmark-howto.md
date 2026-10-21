# Check post-deployment benchmark evidence

Use this after eligible existing operations have produced actual current stagenet
proof, verification, deployment, inclusion and readback receipts. This command
only reads supplied data. For fields and limitations, see the
[reference](benchmark-reference.md); for native selection and unchanged limits,
continue the [existing loop](implementation-loop.md).

1. Select exact workload catalog members and freeze complete independent expected
   effects before the subject execution. Keep supported scope and incomplete
   oracles explicit. The current linked/Fenrir public scalar expectations do not
   supply complete independent native-effect acceptance. Finish those missing
   predicates before qualifying the corresponding deployed case.
2. Select repetitions and cold/warm profiles within existing eligible operation
   limits. Reuse applicable passed setup/keygen receipts. Obtain new observations
   only through the existing owner/entrypoint when its source/resource predicates
   hold; a benchmark flag starts no compiler, proof or transaction.
3. Retain the actual public receipts/artifacts beneath one selected receipt root.
   Keep proof and statement bytes, exact current source/compiler/ledger/circuit
   identities, network/target/anchor, deployed transaction IDs, complete readback
   effects and all failures/censoring. Exclude recovery material and private keys.
4. Freeze a policy and current context using the reference fields. Bind each
   receipt role's semantic fields to the real schema through reviewed JSON
   pointers. In particular, select the actual verification verdict. Declare all
   required phase/layer metrics and exact units; leave unavailable data explicit.
   A required unavailable metric leaves completion open. Pin the exact current
   source closure and workload/oracle artifacts rather than copying old labels.
5. Collect receipt-bound samples with the existing metrics collector. Preserve
   subprocess measurement scope/timer resolution and distinguish cold/warm cache
   identities. Preserve complete raw observations and sample IDs. Choose an
   initial baseline with an explicit future tolerance disposition when there is
   no justified applicable SLA; use comparison only with prior comparable actual
   samples and explicitly selected metric tolerances.
6. From the checkout, run the existing Python checker, substituting your retained
   paths. All four benchmark inputs are required together:

   ```bash
   python3 models/sherlock/qa/check.py --repo . \
     --requirement-evidence openspec/changes/sherlock-completion-convergence/current-evidence.json \
     --benchmark-policy retained/benchmark-policy.json \
     --benchmark-samples retained/benchmark-samples.json \
     --benchmark-current retained/benchmark-current.json \
     --benchmark-receipt-root retained
   ```

   Add `--benchmark-baseline retained/prior-samples.json` only for an explicitly
   selected comparison policy. Capture actual stdout/stderr and exit code in the
   existing evidence record. Exit 1 or 2 leaves the selected stage incomplete;
   resolve the named missing predicate without relabeling data or replaying setup.
7. Review the exact candidate, policy, source closure and actual receipt semantics.
   Pin the recomputed report and its dependencies in the existing requirement
   evidence/currentness records and retain the owner disposition separately.
   A `complete` benchmark result qualifies only its declared case/profile scope.
   Keep larger repetitions, unsupported scale stages and every retained full
   Sherlock/formal/native/financial/release requirement separately open until
   its actual evidence exists.

For an ordinary code check, run `python3 -m unittest discover -s
models/sherlock/benchmarks -p 'test_acceptance.py' -v` and the existing QA tests.
Their synthetic parser fixtures validate the benchmark implementation only.
No native/stagenet success is produced by those tests.
