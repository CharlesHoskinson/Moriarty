# PR210 finite ABI count correction

**Source finding:** the merged reserve `allocate` action has **five** touched balance descriptors, five `pre_balance` slots and five `post_balance` slots. Its current producer emits **53 financial public ABI slots**, not51. The actual merged financial test includes the fifth balance row; the reported four-row/51-slot ancillary review wording does not describe this code.

Pinned actual main is `7396d7d316ab72957bca9709483f4638daa0d5cb`. `SOURCE-PINS.json` records every full source/blob/hash. All seven applicable source files copied here are byte-identical to merged PR210 (`e3eb6d12`). W/I/J/K and Git were not modified. No compiler, Cargo, SDK, VM, model, proof or other product execution occurred. The existing actual two-pass PR210 receipt remains immutable; this is new source inspection, not a rerun/result replacement. No security judgment is supplied.

## Definitions and complete four-action counts

For these unsigned Core/1 source actions the actual producer allocates:

`N = 2 + S + I + B + 2D + E + S + B + 2D = 2 + 2S + I + 2B + 4D + E`.

`S` counts declared states, `I` action inputs, `B` unique touched `(source asset alias, account)` keys, `D` source-wide registered duty names, and `E` public effect-value slots (not effect rows or statement count). Each invocation of Builder.public appends exactly one ABI binding ([crates/mori-backend/src/lib.rs:626](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/lib.rs#L626)). It first appends2 contexts, then pre-states/inputs/touched pre-balances/duty presence+amount ([crates/mori-backend/src/lib.rs:790](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/lib.rs#L790)), then ordered effect fields ([crates/mori-backend/src/lib.rs:946](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/lib.rs#L946)), then post-states/balances/duty fields ([crates/mori-backend/src/lib.rs:1107](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/lib.rs#L1107)). Expression evaluation, lets and predicates add no public ABI entries by themselves. Unsigned source has no authorization/private extensions; the test separately checks those arrays empty.

| Program.action (actual names) | S | I | B | D | Effect rows | E effect slots | Total slots |
|---|---:|---:|---:|---:|---:|---:|---:|
| InstallmentInvoice.register |4|0|0|2|2|8|**26**|
| InstallmentInvoice.collect |4|2|3|2|7|13|**39**|
| ConditionalReserve.register |3|0|0|3|3|12|**32**|
| ConditionalReserve.allocate |3|3|5|3|9|20|**53**|

The source action names are `register`/`collect` and `register`/`allocate`. Neither source has an action named `settle`. Source literals are at [crates/fixtures/financial-report-contract/installment-invoice.mori:1](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/fixtures/financial-report-contract/installment-invoice.mori#L1) and [crates/fixtures/financial-report-contract/conditional-reserve.mori:1](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/fixtures/financial-report-contract/conditional-reserve.mori#L1). Source-wide duty registry order is name order from a BTreeMap ([crates/mori-backend/src/liabilities.rs:42](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/liabilities.rs#L42)).

| ABI block | Invoice.register | Invoice.collect | Reserve.register | Reserve.allocate |
|---|---|---|---|---|
| context |0–1|0–1|0–1|0–1|
| pre_state |2–5|2–5|2–4|2–4|
| input |none|6–7|none|5–7|
| pre_balance |none|8–10|none|8–12|
| pre_liability presence/amount |6–9|11–14|5–10|13–18|
| effect |10–17|15–27|11–22|19–38|
| post_state |18–21|28–31|23–25|39–41|
| post_balance |none|32–34|none|42–46|
| post_liability presence/amount |22–25|35–38|26–31|47–52|

`ALL-FOUR-SLOT-IDENTITIES.csv` and `COUNTS.json` list every position, role and identity for all four actions. These are **producer-source-derived inventories**, not independently frozen normative ABI packing or freshly observed Artifact values. The elementary integer checks used to format this table do not run a compiler/parser/evaluator/model.

## Reserve allocation's five balance rows

Actual producer `keys` is a BTreeSet of `(asset alias, account)` populated from **both endpoints** of every Transfer/Fee statement ([crates/mori-backend/src/lib.rs:820](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/lib.rs#L820)). For reserve allocate those statements produce:

| Exact source alias/account | Input slot | Output slot | Origin |
|---|---:|---:|---|
| BOND / auditor |8|42|BOND transfer recipient|
| BOND / reserve |9|43|BOND transfer payer|
| CASH / reserve |10|44|CASH transfer and fee payer|
| CASH / supplier |11|45|CASH transfer recipient|
| CASH / treasury |12|46|CASH fee recipient|

Asset aliases resolve to the exact network/issuer/reference/scale identities from the source. Reusing `reserve` under two assets yields two distinct rows. Both transfers and the fee count; duties do not add token-balance rows. Sorting is by the producer's **alias/account** keys; source asset declaration order does not determine this balance order.

The20 reserve effect slots are: statement5 liability4 (19–22),6 transfer1 (23),7 fee1 (24),8 discharge4 (25–28),9 transfer1 (29),10 discharge4 (30–33),11 Next2 (34–35),12 Next2 (36–37),14 Emit1 (38). Liability fields are before_presence,before_amount,after_presence,after_amount; Next fields are before,after. Statement indices0/1/2/4 Requires,3 Let,13 Ensures add no effect slots. Replacing B=5 with B=4 subtracts exactly one pre and one post slot, yielding the incorrect51.

For comparison, Invoice.collect has three touched CASH rows (payer/seller/treasury), and its13 effect slots are statement4 Transfer1,5 Fee1,6 Discharge4,7/8/9 Next2 each,11 Emit1. Its states4+inputs2+duties2 therefore yield39. Register actions have no touched balance keys; each source declaration produces4 effect slots, yielding26 and32 respectively.

## What the merged test actually asserts

- [crates/mori-backend/tests/financial_observation_relation.rs:429](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L429) derives a touched exact asset/account set from independent expected execution Transfer/Fee effects, including both endpoints. [crates/mori-backend/tests/financial_observation_relation.rs:442](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L442) requires the compiled balance_frame be unique and **equal to that entire set**. For the supplied reserve source/expected effects that set has five members, not four.
- [crates/mori-backend/tests/financial_observation_relation.rs:479](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L479) fills **every** balance descriptor's opening and closing positions and checks role/name/type/exact asset through `put`/`binding`; it does not truncate the vector to four. `put` rejects duplicate assignment ([crates/mori-backend/tests/financial_observation_relation.rs:49](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L49)).
- [crates/mori-backend/tests/financial_observation_relation.rs:521](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L521) compares the entire ordered effect sequence/field widths and fills each descriptor position. At [crates/mori-backend/tests/financial_observation_relation.rs:563](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L563) every element of a vector sized to `circuit.public_inputs.len()` must have an explicit observation. Extra unassigned positions or duplicate descriptor positions cannot satisfy those test assertions.
- [crates/mori-backend/tests/financial_observation_relation.rs:116](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L116) checks per-role state/input order against the expected Program, every source-wide duty frame and empty unsigned extensions. [crates/mori-backend/tests/financial_observation_relation.rs:198](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L198) additionally checks register's public ABI count by formula `2+2S+4D+4*register_statement_count`, giving26/32. Context positions0/1 are explicit in [crates/mori-backend/tests/financial_observation_relation.rs:387](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L387).
- For collect/allocate, the test does **not** hard-code the literal total39/53 or independently freeze every absolute ABI position. It uses actual descriptors/unique role+name lookups while checking observed values. Complete global packing, context values, whole IR and Artifact byte expectations remain separate open oracle obligations. Source-derived absolute positions here do not close them.
- [crates/mori-backend/tests/financial_observation_relation.rs:570](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/tests/financial_observation_relation.rs#L570) compiles the full sources, checks source-wide metadata for both actions and runs its existing selected SDK relation observations for collect/allocate. Register metadata/count checks do not imply a separately executed register SDK observation. The public compiler uses this same ordinary backend Artifact through `compile_reported` ([crates/mori-backend/src/source_projection.rs:35](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori-backend/src/source_projection.rs#L35)); its source-projection metadata does not rewrite the Artifact. Public publication starts at [crates/mori/src/commands/compile.rs:43](https://github.com/CharlesHoskinson/Moriarty/blob/7396d7d316ab72957bca9709483f4638daa0d5cb/crates/mori/src/commands/compile.rs#L43).

## Disposition

Within this finite count issue, the mismatch is **ancillary prose arithmetic**, not missing coverage of a fifth touched balance in the merged test source. Preserve old owner/reviewer wholes and actual two-pass evidence. A new qualified source correction may replace reported4/51 with5/53, and should use the four exact totals26/39/32/53 if all actions are discussed. This source inspection neither alters an acceptance judgment nor invents fresh product evidence. Original full Artifact, grammar, four-golden, financial/native/target and all U/M/MC/SP/G exits remain open with their prior dispositions.
