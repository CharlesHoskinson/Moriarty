# STK04 bounded cashflow specimen

The unchanged `actus-tests-stk.json` is supplied by ACTUS Financial Research Foundation, https://github.com/actusfrf/actus-tests, revision f7a8064872b69db1f0beabac771c99dc3ce0c397, under CC BY-SA 4.0; see ACTUS-LICENSE.md. The file retains all STK fixture members and all their fields. The new source and oracle adapt only stk04's finite two-event cashflow slice, with attribution. Preserve applicable attribution/share-alike requirements for this adaptation.

`source.mori` uses public Rust Core/1 source, not a fixture dispatcher. Price 1200 and price 3000 are contract terms, not evaluator-produced expected values. Dates are losslessly encoded YYYYMMDDhhmmss integers and compared in source; no date, calendar, EOM, scheduling, oracle or wall-clock facility is claimed. The two explicit actions select this fixture's supplied events. The original terms, to, observations and all ordered results remain in the fixture and oracle.

The test in the existing `financial_pair` target parses, validates and simulates the source, carries the actual purchase poststate into termination, compares all snapshots and local/aggregate ordered effects against frozen literals, and compares all seven present result fields against the original fixture. The projection mechanically renders the actual emitted timestamp, reports the actual event tag, reads the actual Transfer currency and direction, and reads the three zero-valued financial report states. It has no payoff formula and does not read the expected results. Strict result-object equality permits no dropped or extra result fields.

Amounts use scale 0 integral fixture units. `actus-reference/fixture-only/EUR` is a local comparison identity, not a deployed token or network. Investor/counterparty names and opening balances 2000/3000 are explicit author-selected harness data, absent from ACTUS. Remaining balances 800/4200 and 3800/1200 follow the two supplied cashflows. There are no fees, reserves or duty effects. Stock notionalPrincipal term 1000 is preserved; the fixture's output principal is zero and is not silently treated as a loan liability.

The one-shot purchased/terminated guards and local state layout are proposed program policy. The fixture supplies only the normal trace; it does not independently specify repeat-call behavior. Empty opening and closing duty lists do not test outstanding-debt preservation. The source does not deliver shares/equity, generate schedules/dividends, implement interest, accept external market data, or establish compiler/native/ledger correspondence.

`oracle.json` was authored from the supplied fixture/specification before any evaluation. It is author-derived, not independently authored or non-author reviewed. Step statement kinds, ordered effects and snapshots are frozen; complete ExprIds, raw statement bytes, Step.value, logical work and CLI rendering are outside this case. No parser, runtime or Cargo command was executed during preparation.

Future exact-candidate check (not run):

    cargo test -p mori-core --locked --test financial_pair actus_stk04_cashflows_match_every_present_result_field_and_linked_prefix -- --exact

This is a local cashflow-source proposal. It does not close STK family coverage, any ACTUS discrepancy, or any U0–U7/M1–M17/MC/SP/G/native/target/release exit. Keep all 277 required fixtures, all their fields, 18 executable types, 32 taxonomy dispositions and DS01–07.
