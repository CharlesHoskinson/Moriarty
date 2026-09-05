# S02 persistent signing review

Initial candidate `cbd1f1d`, base `a6f9c30`; correction `f2941d4`.
Independent reviewer: `/root/quint_policy_review`, native GPT-6 Astra, high
reasoning. This is one source review, not the three-vendor Council gate.

## Initial review

No blocking issue in the admitted Alice-parent and Alice/Bob swap signing
fixtures. Exact checked policy content, signer identity, relevant snapshots,
independent signer registration, and complete parent/signature/revision
consistency were confirmed. All twenty-four original manifest pins matched.

One low-priority admission issue: a structurally valid Bob installment-parent
policy could prepare, but could never sign because the parent fixture is
Alice-only. The reviewer's focused REPL probe reported
`(validPolicy, canPrepare, canSign, repeatPrepare) = (true, true, false, false)`.
This was fail-closed and outside the harness, not an authority bypass. Preparation
should reject unsupported parent keys to avoid leaving an unusable check.

## Correction and re-review

Commit `5257439` preserves the new regression test for Bob and Mallory before
the guard correction. The RED invocation initially yielded a live handle; the
controller applied the correction before collecting terminal output. The
separate completion receipt records the expected assertion failure. This timing
is disclosed, not represented as an awaited RED result before implementation.

Commit `f2941d4` rejects non-Alice parent preparation while leaving swap keys and
Alice parent preparation unchanged. Corrected execution reports 41 passing tests
and 1,000 sampled signing traces. Both profiles reach both signatures; all four
check/sign action witnesses occur in every trace. This is sampled evidence, not
exhaustive model checking.

Narrow independent re-review: clean; the admission issue is resolved. All eight
correction source/receipt pins matched, the current pointer resolves, the old
manifest remains unchanged, and the RED source commit retains the unfixed guard.
No new findings. The reviewer did not rerun the routine suites.

## Remaining boundaries

- Signatures are finite symbolic tokens, not cryptographic verification.
- Registering a parent creates its exact unclaimed bookkeeping entry; no money
  moves and nonce 0 is not consumed by signing.
- Cancellation/recovery contexts used by guard tests are constructed fixtures,
  not traces of cancellation or recovery actions.
- Signing completion is terminal only for this signing harness, not financial
  completion of the contract.
- `validSigningState` validates context and record-map domain, not authenticity
  of arbitrarily fabricated historical signing records. Reachable actions guard
  creation and use of each check. Later execution verifies actual signed cells.
- Full evidence verification, atomic financial commits, races, recovery execution,
  A–D candidates, correspondence, and all relevant Council gates remain open.
