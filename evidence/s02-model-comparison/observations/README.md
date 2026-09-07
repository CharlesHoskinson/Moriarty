# S02 observation carrier evidence

Status: deterministic tests and sampled structural evidence only. This unit
implements the first part of the delegated common-foundation decision, not its
authorization/recovery machine or a candidate interpreter.

The manifest pins model files, command receipts, the Quint entry script, and the
observed version. It explicitly does not claim a complete toolchain lock. Each
JSON command receipt stores combined tool output and its exit status.

At the pinned content, 34 observation tests and the unchanged 10 effect and 16
consumption tests passed. The sampled harness checked two declared structural
examples in each of 1,000 traces and ended after three states. These are not
executions of Core, an installment, or a refund. Its state-dependent safety flag
means the declared examples satisfy the structural predicates only.

The tests first failed against fail-closed stubs in `4cd6731`. Self-review then
added two failing emission tests in `2f1a856`: wallet-to-wallet transfers and two
deposits were incorrectly admitted by the first payment filter. The final
predicate rejects both and retains accepted deposits with no Core payment.

The generic continuation is preserved by equality; the neutral harness strings
are not Candidate A continuation encodings. `projectionSupportsCoreEvidence`
checks carrier applicability only, never the existence of independent evidence.
The validator does not execute Core to derive balances, continuations, or
reduction counts. Complete independent codecs and correspondence remain required.

All common authorization/recovery, candidate A–D, mutation, Quint/Apalache, and
requested Council acceptance obligations remain open. This receipt passes no
S02 or XML release gate.

## Independent review correction

`current.json` now selects `minimum-time-correction/manifest.json`. The original
manifest and outputs above remain immutable historical evidence. Independent
review found a missing nonnegative minimum-time guard and incomplete public
rollback-mutation coverage. Three new negative-time tests failed before the
guard was added. The corrected unit passes 37 deterministic tests and retains
both structural witnesses in 1,000 sampled traces. The claim boundary is unchanged.
