# S0 common-state positive witnesses

**Model:** provisional `../s0.qnt`, Quint 0.32.0.  
**Corpus:** `s0_common_witnesses.qnt`.  
**Execution:** TypeScript backend; seed `0x5`; three fixed traces.

`seedBalanceOnly` adds an authenticated balance cell without adding allowance
rows. Existing `seedAccount` behavior is retained for payer setup and older
corpora. The three witnesses use work budget 1, work spent 0, round 0,
head 0, and no consumed replay key before submission.

The expected ordered lines and obligation successors are literal values in
the witness module. They do not call `transferLines`, `repayLines`, `repaid`,
or `replayKey`. The tests supply those independent expected lines to submit
and check them against the model's committed lines. Before and after each
submission they compare complete balance, remaining-allowance, spent-allowance,
obligation and signed-intent maps, plus head, round, replay, work, last effects
and commit count. Complete allowance-map equality excludes fabricated receiver
rows; explicit post-state absence checks cover R/F and C as appropriate.

| Case | Literal initial state | Literal resulting state |
| --- | --- | --- |
| `T-10-1` | Balances O100/R0/F0; only O allowance11/spent0; no debt | O89/R10/F1; only O allowance0/spent11; ordered debit11, recipient credit10, fee credit1, allowance11, replay and head |
| `R-30` | Balances O100/C0; only O allowance100/spent0; L principal1000/accrued10/outstanding1010 | O70/C30; only O allowance70/spent30; L principal980/accrued0/outstanding980; ordered debit30, bound creditor credit30, debt write, allowance30, replay and head |
| `R-near-bound` | O balance1/allowance1/spent U−1; C balance U−1; L principal S−1/accrued1/outstanding S | O balance0/allowance0/spent U; C balance U; L principal S−1/accrued0/outstanding S−1; ordered debit1, bound creditor credit1, debt write, allowance1, replay and head |

Here U=340282366920938463463374607431768211455 and
S=170141183460469231731687303715884105727. All three accepted observations
are checked as the full Decision with `diagnosticWork=0`. Each committed
successor has head1, round0, work remaining0/spent1 and commit count1.

## Common instance mapping

The nonces retain the K positive-case strings `T-10-1`, `R-30`, and
`R-near-bound`, with digest labels `digest-<case>`. Obligation ID L is retained.
K heads `h0` and `h1` map to Quint integers 0 and 1. K repayment account P is
renamed O in this requested instance; the debtor, signer, payer, debit,
allowance and replay signer are renamed consistently. Transfer accounts O/R/F
and creditor C retain their names. K's original default round7 is normalized
to round0 here; both rounds lie in the retained signed interval [0,10]. The
Quint selected domain D and asset A are fixed import premises. Additional
Quint policy/evidence labels `p` and `e` are abstract annotations.

This record describes the Quint instance and intended comparison map. It does
not execute a K/Quint comparison. Any finite correspondence result must compare
the actual normalized K and Quint observations independently.

## Executed commands

The full command record, outputs, exit codes, timestamp and SHA256 values are
in [common-command-results.json](common-command-results.json). From the
checkout root:

```text
$ quint --version
0.32.0
(exit 0)

$ quint typecheck experiments/moriarty-language/formal/quint/mil4/s0.qnt
(no output; exit 0)

$ quint typecheck experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt
(no output; exit 0)

$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt --main s0_common_witnesses --match '.*Test' --seed 0x5 --backend typescript

  s0_common_witnesses
    ok transferCommonTest passed 1 test(s)
    ok repayThirtyCommonTest passed 1 test(s)
    ok repayNearBoundCommonTest passed 1 test(s)

  3 passing (98ms)
(exit 0)
```

The same record contains successful typechecks and regression executions for
`s0_witnesses` (15 passing), `s0_divergence_witnesses` (14 passing), and
`s0_diagnostic_witnesses` (3 passing). All five Quint files typecheck. The prior
common witness attempt using `seedAccount` for receiver/creditor setup failed
all three literal pre-state map checks (exit1); its exact output and candidate
digests are retained in
[common-before-command-results.json](common-before-command-results.json).

## Evidence limits

Signature verification, snapshot authentication, native qualification and
atomic ledger readiness are stipulated external premises. The setup actions
represent authenticated cells; they do not establish cell authenticity. The
model has an abstract single-asset balance map and integer head, with no
signature bytes, cryptographic head commitments, accepted retained failure,
phase/duty record, or native/ledger implementation. No `quint verify` was run.
These are fixed executions, not exhaustive proof or a general correspondence
result. UInt128 literals exceed the default Rust evaluator's i64 range, so the
TypeScript backend was selected explicitly.
