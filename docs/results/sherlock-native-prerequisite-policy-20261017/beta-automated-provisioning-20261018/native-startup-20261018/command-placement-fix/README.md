# Host-only command placement diagnostic — 2026-10-18 UTC

The initial diagnostic has outer exit0, but both B1 and B2 child return codes are1, with empty handoff messages and zero SCM_RIGHTS descriptors. Its raw `status: OK` fields describe diagnostic capture, not child success. The bwrap usage text is truncated inside each variant; the complete captured output is retained unchanged in [the initial raw capture](initial/actual-attempt-01/combined-output.jsonl).

After the owner's minimal command-placement correction, [the changed diagnostic](corrected/actual-attempt-01/DIAGNOSTIC-REPORT.json) records both child return codes0, byte `4c`, flags0 and one SCM_RIGHTS descriptor per variant. The outer invocation took0.083515588s. Both exact diagnostic sources, commands, receipts and raw captures are included. Raw `.bin` files are renamed `.jsonl` without changing any bytes.

The [original owner diagnosis](owner/original-actual-usage-diagnosis.txt) attributes the usage refusal to command placement inside bwrap's `--args` memfd. [R332](owner/R332-guard-source-proposal.txt) proposes the corresponding guard correction and pre-handoff output handling. The [R336 diff](owner/R336-guard-source.diff) has a [public independent exact-source APPROVE](owner/R336-independent-source-review.txt). That guard remains **UNADOPTED, pending focused tests**. These attributed source judgments do not establish installation, tested guard behavior or execution authority.

Neither diagnostic modified the guard or performed SDK build/proof actions. This archive establishes no SDK build, proof, verification or stagenet result. Earlier native refusals, consumed marker and historical negative evidence remain intact. No SDK payload, executable binary, provider input or private material is included. Root alone copies this proposal into the repository.

## Later guard tests and fresh setup — 2026-10-18 UTC

The [guard regression and fresh-setup archive](guard-regression-and-fresh-setup/README.md) contains the full independently source-approved 087f guard, final owner-corrected launch tests, one successful 30-test SDK-free run and seven successful fresh02 setup operations. The earlier pending-tests wording above is historical. Refreshed native-config admission still awaits the separately applicable normal review/adoption at this archive cut; no native job, SDK build, proof or settlement is claimed. R342's initial rejection and exact literal corrections remain visible.
