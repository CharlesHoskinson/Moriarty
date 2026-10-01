# Signed intent to settlement iteration

Active autonomous iteration, based on main2905cb6d. User requested three exact
Sonnet5.5 high workers and three GPT6.1 Sol workers, plus independent result
reviews, while continuing work during AFK time.

- [Plan](PLAN.md): checklist, scopes, resources and full acceptance conditions.
- [Signing protocol](PROTOCOL.md): exact closed statement, bytes, hashes and APIs.
- [Worker proposals](plans/): independent design reports, including dissent.
- [Process repair](REPAIR-PROCESS.md): retained timeout defect and regression.
- [Public API types](REPAIR-TYPES.md) and [security repairs](REPAIR-CRYPTO.md).
- [Local atomic consumer](LOCAL-SETTLEMENT.md) and [security cases](SECURITY-CASES.md).
- [Developer walkthrough](DEVEX-RESULT.md) and [native numerical result](NATIVE-RESULT.md).
- [Measured stage](RESULT.md): actual tests and preserved acceptance gaps.
- [Review repair](REPAIR-AUDIT.md): supplementary rejection detail escaping.

Repository observation: versioned Rust intent encoding/verification and the
actual asynchronous beta consumer are implemented locally. Integration tests,
security coverage, local atomic settlement, developer flow and the numerical
proof experiment are implemented and measured. Fresh independent whole-candidate
audits are pending; no reviewed delivery or Preview acceptance is claimed.

The native goal is armed in the host. Six worker seats execute in waves with
at most three concurrent workers. Exact provider call receipts remain external
until the reviewed evidence freeze; separate calls do not imply provider-session
continuity. Historical plans/tests/audits from earlier iterations are preserved.

Open product obligations: authenticated account key lifecycle, authentic state
and head, compiler/native complete financial proof correspondence, atomic
financial ledger acceptance, Preview transfer and repayment finality. The local
key registry and numerical circuit are precursors with explicit limited scope.
No wallet identity or existing contract state may be replaced to manufacture
successful evidence.
