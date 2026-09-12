# Concrete proposed live resource values — NOT ADMITTED

Read-only preparation,2026-09-11. The companion JSON records exact current master digest/mtime, Decimal arithmetic, immutable evidence hashes and missing instance inputs. No live budget, lock, store, wallet or grant was changed. These values require source approval and separate substantive resource votes; this document is not a final projection.

| Proposed component | Value |
| --- | ---: |
| New MC02 grant G |1960seconds |
| New dispatch quota N |2 |
| One funding preflight |125seconds |
| One prepaid financial executor |1835seconds |
| Historical residual credit closure C |7seconds |
| Separately reviewed nonspendable closure reconciliation D |1second |
| Proposed represented master ceiling |212851seconds |
| Master used after fully reserving G |212850.0473305040214seconds |
| Remaining master rounding slack, not spendable |0.9526694959786seconds |
| Aggregate dispatch ceiling |85 |
| Preserved MC01 worker count/ceiling |52/52 |
| External dispatch count after both new debits |33 |
| Aggregate count after both new debits |85 |

The observed unchanged master ceiling is210890; exact U=2323.0473305040214+165+108960+300+99135=210883.0473305040214. Hence C=ceil(210890-U)=7 and D=ceil(U+C)-210890=1. Decimal parsing used the original JSON number tokens, not a binary-float intermediary. The observed master SHA-256 is `a12ca70bceacd912a6681caee0d7fe1c627ffc188fbe77bbfa7ca0c2ce0f0748`. Any changed master requires recomputation and corresponding current binding before adoption.

G reserves only this observer and executor pair. After preflight, S=125 and remaining1835; after runtime debit, S=1960 and remaining0, with n=N=2. The exact matching one-shot prepaid runtime must be permitted at this equality boundary; a second debit or retry must fail. Original MC01 package117710 and all historical rows/planning/counters remain unchanged. The extra D is explicitly nonspendable closure reconciliation, not another second of user credit. Source authoring, actual-result audits and broader AFK work are not included in this1960second grant and require their own applicable existing or reviewed allowances.

## Proposed financial bounds from retained precedent

The new financial case would be cumulative **Preview loan attempt3**, with at most4 guarded public submissions in order **deploy → initialize → accrue → settle**. Source/collector author attempts, local loan and user-stopped author06 are not extra Preview loan attempts. The proposal would reuse the *numerical bounds* of the separately consumed historical loan envelopes, never their credit: native DUST fee ceiling2000000000000000SPECK and gross USD_TEST_ASSET input ceiling20000000000test units. Actual final asset color must be bound to the reviewed generated contract/fixture; it cannot be copied from an old deployment. Initialize mints fixed test assets; settle consumes the full borrower UTXO, pays lender and returns change.

The retained reviewed subset is27reserved submissions and8100000000000027SPECK reserved, against12000000000000010SPECK admitted ceilings. Each historical Preview allocation reserved4submissions/1200000000000004SPECK against its separate2e15SPECK cap. These figures include local11 plus four Preview allocations; they are **partial historical totals**, not a global account balance, actual paid fees or current funding. Later local records and older unquantified costs remain preserved separately.

Adding the proposed maxima produces only a descriptive partial subtotal:31known-plus-proposed maximum submissions,10100000000000027SPECK known reservations plus new cap,14000000000000010SPECK known admitted ceilings plus new cap. Do not write these as actual reservations or actual spend before corresponding commitment. Native DUST reservations and indexer paid/estimated fee units have unresolved correspondence. Current available DUST/token funding is UNOBSERVED, not zero or inferred sufficient.

## Remaining exact inputs

- Both full source audits on frozen accounting/observer/executor bytes, including actual1835second executor/launcher fit and SDK submission guard.
- Immutable concrete projection, execution context/manifest, original wallet/role/network expectations and source/runtime closure.
- Both fresh resource votes on that exact projection/context, G/N/C/D, attempt3 and financial bounds; no prior vote is reused.
- Reviewed canonical history/token adoption, two actual loan attempts plus shared-wallet swap/pending/persistence lineage, exact store/master correspondence and exclusive ownership.
- Separately admitted one-shot real funding observation and parent containment/input-integrity evidence, fresh enough for runtime debit/launch.
- Concrete deadline fitting the user AFK window, observer/runtime budgets, cleanup and separately funded independent result review.

No live action becomes authorized by these arithmetic values alone. Preserve all consumed/unknown history and fail closed on any missing applicable input.
