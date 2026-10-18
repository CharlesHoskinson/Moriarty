# Guard regression and fresh setup — 2026-10-18 UTC

The complete [guard source](source/bounded_sdk_compile.py) is the independently source-approved `087f3a37…` afterimage (74,823 bytes). The [final launch test](source/test_launch_handoff.py) is `1a7b4218…` (29,867 bytes). One actual isolated SDK-free run passed all 30 tests: 22 unchanged configuration/source tests and eight launch tests. [Command/result/raw streams](tests/actual-attempt-01/RESULT.json) record exit 0, 0.118594944s, 0 stdout and 4,972 stderr bytes with all four input pins unchanged. These implementation-informed mocked tests establish no blind R2 or native result.

The [R342 rejection](reviews/R342-revision02-reject/FINAL-RESPONSE.txt) remains unchanged. It identified BF-1/BF-2, prescribed exact fixes and conditionally classified the corrected ordinary suite. [Root materialization](source/ROOT-LITERAL-MATERIALIZATION.json) records those literal fixes. The earlier [original test author's memfd correction](reviews/R343-original-author-memfd-correction/FINAL-RESPONSE.txt) changes index 5 to 6; it is not another independent approval. The existing [R336 full independent source opinion](../owner/R336-independent-source-review.txt) and [status](reviews/R336-STATUS.json) retain their exact scope and authorship.

Fresh02 [OP-0 through OP-6 all passed](fresh02/RESULT.json). Each operation retains exact argv, result and complete stdout/stderr; [Root's actual selection](fresh02/ROOT-SELECTED-COMMANDS.json) uses sudo only for OP-2, OP-4 and OP-5. OP-3 is ordinary, correcting the unselected proposal. The [request](fresh02/request.actual.json) and small [Q-OBS observation](fresh02/Q-OBS.json) bind the new setup; Q-OBS SHA256 is `b2cc9f761c6ad01c188ab03a05baf11b5b89855b207b915cd5aad405b230002d`. The large old-home stat capture has [identity-only metadata](fresh02/OMITTED-OLD-HOME-STAT-IDENTITY.json); its body and all SDK payload are omitted.

[Source identities](source/SOURCE-IDENTITIES.json) retain the historical 12-member freeze and eleven unchanged members. The exact unchanged [22-test source](../../../../134-final-r2b-policy-freeze/snapshots/10-test_config_source.py) and [config fixture](../../../../134-final-r2b-policy-freeze/snapshots/03-config.example.json) remain public at their original byte hashes. The unchanged [operator source](../../../../285-sdkfree-mock15-source-result/snapshots/operator_provisioner.py) is identified by the actual setup result.

At this preparation cut, normal nonsecurity review/adoption of the refreshed native configuration is pending. Source approval, passing mocked tests and successful setup do not assert config admission, a selected native job, SDK build, proof, verification, target settlement or new EARS closure. No native job was selected by these receipts. The beta instruction removes human-only operation/approval requirements; historical contrary wording in preserved reviews is not a current human gate. Existing source/resource/correctness and acceptance requirements remain.

The parent archive's earlier “pending focused tests” text describes its prior cut; these dated test/setup receipts are the later observation. This archive contains no provider input/transport payload, executable binary, huge stat/flat inventory, wallet or private SDK material. Publication preparation executed no test, setup operation or native action.

The later [R346 normal-source review](reviews/R346/FINAL-RESPONSE.txt) approved
the exact guard and observed test consistency. Root then [adopted the refreshed
configuration](fresh02/ROOT-SOURCE-AND-CONFIG-ADOPTION.json); automated prelaunch
verification passed. The [real native attempt](native-attempt-01/RESULT.json)
started its authenticated A0 worker and exited 2 in 9.297351s: [A0 stderr](native-attempt-01/packet/A0.stderr)
records `staging size changed`. No Cargo/compiler command started. The [metadata
diagnosis](native-attempt-01/ACTUAL-STAGING-SIZE-DIAGNOSIS.json) found thirteen
registry-index files with larger sizes than the October 15 staging manifest.
The [failure and cleanup](native-attempt-01/CLEANUP-OBSERVATION.json) are preserved;
evidence collection and privileged teardown passed. No build or target success
is inferred. The next input refresh must preserve the source tuple and caps.
