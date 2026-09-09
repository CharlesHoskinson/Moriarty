# Transfer-only execution in K

Status: implemented locally; K execution and independent result acceptance pending.

This SP03.2 increment adds K execution for an existing funded-source capability:
a Transfer-only action moves cash without discharging an obligation. K charges one
ordinary work unit and checks state invariants before the transfer. Rejections
return only code/index. The successor grammar and source evaluator are unchanged.

The [independent cases](INDEPENDENT-CASES.md) contain ten new Transfer-only cases
and six unchanged repayment regressions. `check-source-cases.mjs` exercises the real
source parser, elaborator and preparation entry point, comparing complete results.
Offline synthetic KAST tests check the codec but are not K execution evidence.

The [resource proposal](resource-proposal.json) permits one fresh bounded compile
and sixteen sequential K invocations after exact candidate/resource reviews and
root admission. Previous four compile attempts and 33 krun invocations remain
charged. No proof, Midnight transaction, semantic freeze or sprint completion is
claimed. General action sequences, rounding, ProRata, public Pending/Complete
observations and full Core/correspondence work remain open.
