# Matched surface specimens

Specified-only reading-study fragments. All three forms below compute the same swap output, in the same arithmetic order, from the same immutable pre-state. They omit the same outer declarations, authorization and effects. They are not additional accepted Moriarty frontends.

## Financial blocks

```text
let adjusted = amount_in * fee_numerator;
let numerator = adjusted * pre.reserve_b;
let denominator = pre.reserve_a * fee_denominator + adjusted;
let output = floor_div(numerator, denominator);
requires output >= min_out;
```

## S-expressions

```text
(let adjusted (* amount_in fee_numerator))
(let numerator (* adjusted pre.reserve_b))
(let denominator (+ (* pre.reserve_a fee_denominator) adjusted))
(let output (floor_div numerator denominator))
(requires (>= output min_out))
```

## Layout-based declarations

```text
let adjusted = amount_in * fee_numerator
let numerator = adjusted * pre.reserve_b
let denominator = pre.reserve_a * fee_denominator + adjusted
let output = floor_div numerator denominator
requires output >= min_out
```

For a second task, use the same three forms to calculate `n = notional * rate_numerator * day_numerator`, `d = rate_denominator * day_denominator` and `interest = floor_div(n, d)`. Multiplication associates left in every specimen. Calculating interest creates neither a token payment nor a discharged debt.

Pilot the tasks with six participants, then use a 24-person formative comparison with equal representation from application developers, financial-domain specialists and contract reviewers. Record actual background rather than infer expertise from job titles. Counterbalance syntax and task variants with a Latin-square assignment; provide identical explanations and diagnostic support.

Measure correct identification and repair of financially consequential errors first. Include wrong recipients/assets, fee treatment, deadline boundaries, Boolean grouping and a state-read-after-write trap. Measure time and preference separately. Include held-out and unaided-reading tasks. Report participant/task-aware uncertainty and all failures; this sample is formative, not a production-safety or universal-usability claim. Reconsider the block recommendation if an alternative reduces consequential misunderstandings without impairing authoring and diagnosis.
