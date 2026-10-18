# K6 exact first-proof authorization proposal (corrected R2 candidate)

## Status

This is a concrete reviewed-launch candidate authored for fresh independent review. It is not authority, not an actual RR6 AUTH, and grants no execution, proof, ledger or Preprod result. The candidate AUTH must live only as AUTH-CANDIDATE.json in a future source candidate; it must never be placed at RR6/AUTH.json on the basis of this proposal alone. Authorship gives no execution authority, and this author is not the auditor.

## Corrections to the preserved R1 candidate

The preserved old AUTH candidate (9616 bytes, SHA-256 40add9f4) was valid JSON but failed strict guard comparison on exactly one path: /nativeArgvs/N11 length. The corrected N11 is [/task/bin/mori-k2, prove, --source, /task/in/installment-invoice.mori, --request, /task/in/invoice-partial.request.json, --setup-record, /task/stage/n11-key-size.setup.json, --proving-key, /task/keys-ro/invoice-partial.pk, --out-proof, /task/out/n11-must-not-exist.proof.json]. All other AUTH typed fields are preserved for root strict comparison. The corrected AUTH has exactly eleven fields, four pins, nineteen order entries, seventeen native argvs, nineteen caps, HOSTCAPS, aggregate, counts, ninety publication paths and twenty-seven effects, with no placeholder and no self-hash cycle.

The preserved old LAUNCH-PROPOSAL.json (9510 bytes, e66c677a) was invalid JSON and created no source data manifest. This proposal replaces it with a new balanced complete JSON proposal. No parser pass is claimed until root parses and performs pure guard-data comparison.

Old prose incorrectly attributed K5 retention/adoption verification to custody.verify_runtime_preconditions. The actual custody code checks ROOT26, K5M, K5g and setup-record pins only and contains no retention or adoption check. This proposal therefore places an explicit PUBLIC root/launcher pre-AUTH/prelaunch gate in the launcher and launch proposal; it does not invent a custody runtime guard.

## Operative source and materialization

Future M6 contains exactly four public source files, files mode 0444 and directory mode 0555, with no bytecode and no selftest.py: operator.py (50480 bytes, 90c313d1), custody.py (25208 bytes, a2362a13), claims.py (49656 bytes, 97a500e6) and probe-k6.sh (12741 bytes, 19638b9c). Root creates RR6 M6-SOURCE-ATTESTATION.json, AUTHORIZATION-RECORD.json, AUTH.json, launcher.py, LAUNCH-RECORD.json, LAUNCH.stdout and LAUNCH.stderr create-exclusively, and asserts M6, T6 and every RR6 fresh path absent before effects. Root materializes only its own proposal/candidate records, then must obtain fresh full independent source, effect, custody, resource and authority review before any native operation.

The future root authorization record is format root-k6-authorization-record/1 with exact fields format, executionAuthorized, scope, authPath, authSha256, launcherPath, launcherSha256, m6SourcePins, sourceAdoptionSha256, freshSourceReviewSha256, ordinaryActualAdoption, newAuthProposalReview, k5ActualAdoptionSha256, k5ExactRunScopeSha256, counts and createdByRoot. The record binds the AUTH hash and launcher hash; its own hash is supplied externally to the launcher. Future positive-review and adoption hashes are required externally bound values and are not invented here.

## Public retention and ordinary gates

Before AUTH reliance or spawn, root/launcher verify only the public K5 actual adoption record (2339 bytes, 6ce800db): retention clause DV-K2-1-R, followup current-cardano-k2-k6-proof-roundtrip-20261016, retain in place unchanged, and no operations until a new exact K6 AUTH. The K5 exact run-scope (7862 bytes, 3adaf145) is historical K5 AUTH, not new K6 AUTH, and its permissions are not reused. Public key metadata or path strings are not read permission; root and launcher perform no host PK stat, hash, body read, copy, delete, rechmod or retention extension.

The corrected finite ordinary result is 18 checks, exit 0, failed [], 8.180702 wall seconds and 0.195199 CHILDREN CPU seconds, with no native or PK activity. Its fresh actual-result review is underway but is not included here and is not assumed adopted. Root source adoption (bfc7854a) is literal FOR with fresh AGREE 2 each on its listed clauses and N10 ABSTAIN; it is source-only and grants no execution authority. Both the ordinary adoption gate and a fresh review of this new AUTH/launcher proposal must close before private or native action.

## Guarded one-shot launcher

The launcher is a complete stdlib one-shot parent capturer with no import-time I/O. It requires an external expected AUTH SHA-256 and an external expected root authorization-record SHA-256, with no defaults. Before spawn it verifies the bounded public authorization record and AUTH bytes, the pinned launcher bytes, the four M6 source files, executionAuthorized, scope, counts and provenance, the public K5 retention gate, ordinary adoption and new-proposal review flags in the trusted record. It never imports or executes operative helpers and never touches protected private trees; all protected work is delegated to operator R1/R0 under the future AUTH.

The sole child is /usr/bin/python3 -I M6/operator.py --expected-auth-sha256 <trusted SHA>, with stdin DEVNULL, close_fds, cwd M6, and minimal env PATH=/usr/bin:/bin, HOME=/nonexistent, LC_ALL=C and TZ=UTC. There is exactly one operator invocation, zero retries and no extra setup, probe, toolchain or native call by the launcher.

Parent stdout and stderr are streamed during the run with a 4194304-byte retained prefix per stream, full-stream hash, observed byte count and overflow flag. Overflow continues drain/discard and records FAIL after terminal with constant bounded memory. Only RR6 LAUNCH.stdout, LAUNCH.stderr and LAUNCH-RECORD.json are written, create-exclusively. No outer CPU, address-space or wall envelope is added; boundedness relies on the reviewed operator aggregate 7200 CPU and 12600 wall seconds, per-host 300 CPU and 600 wall nominal plus sixty-second grace, and the AUTH per-command caps.

If a pipe holder survives parent exit, the collector waits at most sixty seconds in total after parent exit, then records drainComplete false and incomplete/FAIL. It makes no absolute all-descendant kill claim and does not reset native budgets. This sixty-second public choice is a first-author FOR position requiring fresh AGREE. The launch record stores actual parent exit, elapsed time, CHILDREN CPU, prefix hashes, counts, overflow and drain completion; it infers no proof or PK read count if the parent failed early. Parent capture failure cannot be PASS and there is no silent second attempt or private cleanup.

After terminal state, root intake is only the explicit ninety T6 publication allow paths plus receipts/FREEZE-MANIFEST.json and T6 LAYOUT-STATUS.json if present, never a recursive T6 walk. Dynamic result hashes belong in a root manifest after actual terminal state, not in AUTH predictions. A fresh non-author native actual-result review is required before proof adoption, with no automatic ledger, mainnet or Preprod transaction.

## Limitations and open predicates

Predecessor counters, the failed R1 source AUTH history, ordinary predecessor 15/17 evidence and wall costs are preserved without reset. N10 remains conditional: a Compile refusal is not descriptor-guard evidence. The unjoined daemon-reader residual, Class-0 forgeable controller, false VK-matrix link, and unproved spectator, frame, context and ledger authenticity remain disclosed. Source readiness, finite ordinary evidence, proposed authority, actual proof and target acceptance are distinct. All twenty UTXO families, ACTUS and DeFi obligations, general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod remain required and open, with no percentage of full compatibility claimed.

End of K6 authorization proposal documentation.
