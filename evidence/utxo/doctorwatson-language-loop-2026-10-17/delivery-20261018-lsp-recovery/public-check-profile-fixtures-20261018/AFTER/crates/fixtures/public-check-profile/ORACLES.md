# Independent public check expectations

These are six normal accepted-source fixtures, not simulated transactions.
The frozen expected JSON was authored before any candidate command execution.
No Moriarty parser, checker, evaluator, Program builder, report constructor or
snapshot updater generated the expectations. Byte spans and SHA-256 are fixture
metadata counted with Python standard-library operations on the declared ASCII
source. Source hashes identify report inputs, never ABIs, keys or deployments.

| Profile | Case | Financial distinction | Complete report contract |
| --- | --- | --- | --- |
| beta/1 | beta-invoice | Buyer pays Seller25USD with Treasury0.20USD fee; gross25.20USD | Named agreement; all8 declaration kinds/names/full spans/name spans in source order; pay→Payment local_s0; no diagnostics |
| beta/1 | beta-repayment | Payer repays12USD toward named Debt; no transfer recipient or fee leg | All7 declarations including obligation; repay_loan→Repayment local_s0; all spans and no diagnostics |
| Core/1 | core1-payment | Immediate amount input transfer and fixed2-unit fee | One action, UInt input, ordered Transfer expr0 and Fee expr1, no diagnostics |
| Core/1 | core1-loan | Register40-unit debt; later transfer/discharge selected independently | Two actions with complete input/statement arrays; global expr indices0,1,2; no diagnostics |
| Core/2 | core2-escrow | Fund escrow, carry release duty across delivery event, then release/discharge | Complete Transfer/Liability/Await/Transfer/Discharge ordering and expr0–4; ready:Bool binder |
| Core/2 | core2-rebate | Treasury rebate duty followed by approval, auditor fee and buyer payout | Complete Liability/Await/Fee/Transfer/Discharge ordering and expr0–4; approved:Bool binder |

Every full expected object also fixes accepted status, relative input filename,
exact source digest, language where present, and all empty diagnostic arrays.
Only staged reports contain the exact local-reference-stage / none / untracked
labels. Comparing whole JSON catches wrong dispatch, field omission/addition,
action loss or reordering, incorrect binders, changed statement routing and
missing stage qualification. No output field is deleted or normalized.

Core check intentionally omits expression tables, assets, state and full Program
serialization. Amounts and state execution are not established by its report;
source hashes bind the literal checked inputs. Beta check reports declaration
and action metadata, not settled financial effects. These fixtures therefore
cannot qualify compiler lowering, native relations, proofs, authenticity,
authority, target settlement or any parent roadmap exit.
