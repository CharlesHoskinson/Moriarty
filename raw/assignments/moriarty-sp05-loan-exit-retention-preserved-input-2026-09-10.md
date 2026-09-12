# SP05 next-step follow-up — corrected checkout and amendment path

This file corrects, but does not edit, `/tmp/moriarty-sp05-next-terra.md`.
The prior report was observed on stale checkout identity `e99aaec`. The exact
checkout revalidated for this follow-up is `/home/charl/Moriarty` at
`b7c6821504a35227c31d1b185289c94dba9dec17` on `main`.

The cited immutable evidence remains present with the same SHA-256 values:

| Evidence | SHA-256 |
| --- | --- |
| local loan command result | `368ed9e1430fa5fb8b9013f5bbbc3fe43dc7659af7c2c3448c0058bc6da461a1` |
| local loan historical readback result | `bc14a95da69c2b0a6741c28372429c9b22f7e55304e877b065173d91ea914777` |
| local swap reviewed result | `5698178e26a300b3b2770aeb1b1ee404b146c0da9ecffb53be234ed72db366da` |
| Preview loan-exit result | `3bd82765aa47a2b771ce121bb3b68e5e91285a7de7f54b3b88884adcd7e33cbd` |
| Preview swap-exit result | `22cf3dfdce8791d51c19bb2d2bd307545d2e44f89bc3d5f8d7acc165e03994bd` |
| Preview loan terminal observation | `157aabcc8af4a14a0a2faba37423cf80906b19c012c447fb6b7a44c5dfa513b9` |
| Preview swap terminal observation | `d06dd191a91320ab53d2f1461695ee6f9a711304d7c86cc05e740dc2d5193255` |

The guarded status remains operational-history blocked and offers only
`sp01-loan-report`; it does not admit an SP05 action. No actual operation was
run for this follow-up.

## Correction: the old cap does not prohibit a new reviewed allocation

The retained loan result's `retryAllowed: false` and two-attempt case cap apply
to its consumed allocation. They make a replay under that allocation invalid.
They do **not** by themselves create an immutable, repository-wide prohibition
against a separately reviewed bounded resource amendment. The user has
explicitly authorized such amendments.

Any successor must be a new candidate and allocation. It must retain the old
two loan attempts and all their charges as history; it must not call itself a
reset, replay, refund, or repair of the missing old exit. It instead produces
new evidence for the still-open raw-exit predicate.

## Limits that cannot be amended away

| Limit or fact | Why it remains binding |
| --- | --- |
| The old loan raw process exit is unavailable | The service was already `not-found`; its default code/status cannot be reclassified as main exit zero. Only a new actual run can supply a raw capture. |
| Historical attempts, reservations, outputs, and audits | They are immutable evidence and consumed charges. An amendment adds a charge; it never refunds, rewrites, or hides an old attempt. |
| Actual financial and command predicates | `FINANCIAL_COMPLETE`, financial effects, containment, and main exit are separate predicates. An amendment cannot waive main exit zero by using stdout or financial results. |
| Existing wallet identities and safety constraints | Preserve the existing wallet/roles/build identity and private material. No new keys, wallet reset, rollback, or secret publication is permitted. |
| SP05/MC02 and mandatory-PCD acceptance requirements | A new loan run can only contribute to I2 evidence. It cannot mark SP05, MC02, network acceptance, financial acceptance, proof acceptance, or mandatory PCD complete by itself. |

## Limits that may be proposed in a new amendment

The old `caseAttemptCap: 2`, its four-submission limit, 2,000,000,000,000,000
SPECK reservation, gross asset ceiling, deadline, process resource limits, and
the associated `retryAllowed: false` are values of the completed allocation.
They are not reusable authority, but a fresh amendment may propose new bounded
values after a new source candidate and current reviews. The historical charge
ledger remains an input to its resource calculation.

No approved amendment for a new loan raw-exit-evidence allocation was found.
The successful `preview-swap-exit-01` resource packet is mechanism provenance,
not authorization for a loan run.

## Concrete path to new raw-exit evidence

1. **Repair and freeze an exact loan exit-retention candidate.** Reuse the
   successful swap mechanism only as inspected provenance. The candidate must
   use `Type=exec` and `RemainAfterExit=yes`, record a nonzero startup
   `InvocationID`, and before explicit stop retain one loaded active/exited
   `systemctl show` observation with `MainPID=0`, `Result=success`,
   `ExecMainCode=1`, `ExecMainStatus=0`, and the same InvocationID. Its
   terminal predicate must reject `not-found`, default values, mismatched IDs,
   and post-stop observations. The existing reference is
   `deliverables/sp05-financial-integration-2026-09-09/preview-swap-exit-01/execute-once.py`,
   `terminal_predicate.py`, and `terminal-observation-01.json`.
2. **Test that repair before any runtime authorization.** Add controlled
   terminal-predicate tests for the original loan's unloaded fields and for the
   required retained fields. Verify that a `FINANCIAL_COMPLETE` stdout line
   cannot turn either failure into exit zero. This is source work; no service,
   wallet, or network action is involved.
3. **Produce a bounded, reviewed resource amendment.** Bind exact source,
   plan, generated artifacts, full build, public command, and historical-charge
   inputs. Start with a conservative four-stage envelope derived from the
   successful swap: at most four submissions and a separately justified DUST,
   gross-asset, wall-time, CPU, memory, disk, and cleanup ceiling. Do not copy
   the swap's 2e15 SPECK/2550-second values as automatic authority. The
   amendment must state its new allocation ID, preserve the old loan attempts,
   and stop on the first failed predicate.
4. **Obtain current independent source and resource reviews, then exclusive
   admission.** The admission must bind both reviewer identities, exact hashes,
   current wallet/build/protocol/role identity, live resource counters, and
   the new allocation. The existing swap reviews and completed source reviews
   cannot substitute for this changed loan executor and resource packet.
5. **Satisfy dispatch-time gates before the single new run.** Confirm: no
   pending marker or pending coins; spendable registered DUST; matching existing
   wallet and pinned build; matching network/genesis/protocol; vacant unit,
   timer, cgroup, port, and containers; required memory/disk; no concurrent
   heavy process; and exact loan source/profile/plan hashes. Any failure stops
   before or at the bounded allocation and is retained.
6. **Run once only after admission, then preserve terminal ordering.** Capture
   the raw terminal observation and selected stdout before explicit stop; then
   observe cgroup/prover/container containment before timer cancellation.
   Retain transaction bytes, canonical finality, complete states/effects, and
   the old as well as new charge records. Fresh independent result audits then
   assess the new evidence. A successful result still does not promote SP05
   without the MC02 reconciliation and remaining acceptance requirements.

## Current recommendation

The concrete next development task is step 1: a source-only, test-first loan
exit-retention repair candidate. It has a directly reproduced defect
(`preview-loan-exit-01/terminal-observation-01.json`) and a demonstrated changed
strategy (the retained swap exit mechanism). The immediately following task is
the bounded resource amendment and current review/admission package. A
read-only reconciliation can continue in parallel, but it cannot satisfy the
missing raw-exit predicate.

Nothing in this follow-up authorizes dispatch. Current guarded status is still
blocked for SP05 operations, and no K, native, wallet, network, transaction,
or service command was executed.
