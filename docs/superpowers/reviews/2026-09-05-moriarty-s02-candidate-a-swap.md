# Candidate A stateful swap source review

Disposition: accept this bounded local semantic unit. This is not Council acceptance,
independent Python correspondence, exhaustive model checking, or S02 completion.

Source: `3c648ec50c7eb63fd4fc78742846a722446f0892`.
Root read the complete harness and 19 tests. The native `quint_policy_review`
agent also reviewed the non-authored harness/test scope; it disclosed authorship
of the imported projection dependency. No unresolved source finding was reported.

The single shared state begins at empty canonical N6 with real funding wallets.
It records actual Alice/Time1 and Bob/Time2 deposits, settlement, voluntary refund,
empty/Alice-only/funded deadline cleanup, and a retained deadline-input rejection.
The latter permits only actual NoInput cleanup. At most four records are admitted;
Close has no action and no unconditional stutter is present. Diagnostics retain
their payload, fail safety, and do not become Core rejections or terminal witnesses.

Root independently executed 19 tests and 1,000 Rust sampled traces (seed argument
42, maximum eight steps). Both exited zero. All eight action witnesses were
positive and all 1,000 traces reached terminal. Exact commands, printed seeds,
counts and terminal output are in the adjacent evidence bundle, not reconstructed.

Safety re-evaluates producer results and checks request/history identity, balances,
conservation, applicable ordered effects and rollback. These are local consistency
checks, not an independent semantic oracle. The two preserved ITFs came from
negated desired witnesses: their expected exit one demonstrates target reachability,
not a failure of coreTraceSafety. The complete eight-file RED closure is preserved;
RED had one passing and 18 failing tests; GREEN retained all 19 tests.

The bundle pins the pre-installment program source at the source commit. Later
program additions do not retroactively change that tested closure. Source/artifact
hashes and claim limits are in
`evidence/s02-model-comparison/candidate-a-swap/manifest.json`.
