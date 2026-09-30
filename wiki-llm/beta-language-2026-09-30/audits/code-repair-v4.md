# Final developer-experience repair batch

Repository observations on the reviewed v3 consumer artifact, followed by focused
RED/GREEN probes against changed production code. Original reviews/trials remain
immutable; they approve their original scopes only.

| Finding | Disposition |
| --- | --- |
| Astra supplemental Low: CLI replacement-decodes invalid UTF-8 | Fatal decoder, explicit unsupported BOM policy; read errors happen before formatting writes; original bytes preserved |
| Grok v3 Low: inspection budget refusal mislabels checked source | `InspectionRejected` plus original `authoringStatus`; empty partial projections |
| S1/S2/S3/G1/G2: missing repay example/schema, unclear units and replay | Explicit independently derived repay template/example and packed getting-started reference |
| S1/S2/S3/G1/G2: failed tests show no difference | Exact original comparison, at most four first mismatch paths with bounded value summaries; whole report limited to524288 bytes |
| S1/S3/G1/G2: formatter flattens enclosing-record/call layout | Delimiter-aware commas: records multiline, named calls compact; meaningful idempotence/comment/elaboration comparison |
| G1: scenario identity pointer does not identify domain/asset | Separate `/domain` and `/asset` messages with expected and actual IDs; unknown keys use escaped RFC6901 child pointers; source action span explicitly distinguished |
| G1/G2: const hover calls money identity metadata | Exact Qty asset/atoms or immutable checked value; identity declarations retain unverified warning |
| G2: slash error wrongly says ASCII | Unsupported-character diagnostic states missing division explicitly |
| G2: second init yields raw EEXIST | `BETA_INIT_EXISTS`, no overwrite; other filesystem errors still report command failure |
| G2: rejected expansion lacks local qualification field | All beta expansion/simulation failures carry `local-stipulation-only`, no effects/post |
| S1/S2: command-specific help missing | Per-command help and template option documented |
| Grok v3 Low: public check metadata is additive | Retain bounded JSON-safe reference/field-use spans; explicitly document result keys and extensible contract, preserve closed-summary dissent |

No Source/6/Core/5 financial predicates were changed. Caps/expiry/funds/authority/
replay continue to be judged by actual Core in original order. Source fixtures
remain explicit, never weakened to force a pass. Quantity precision/nominal assets,
unsupported execution, external premises/bindings and immutable inputs remain.

Preference dispositions: retain full machine `simulate` result; readable `check`
and inspect provide shorter views. Percentage division/rounding, hidden financial
defaults, automatic balance generation, silent chained payroll and authenticated
address shortcuts remain outside beta. Richer native editor activation and provider
adapter adoption remain unperformed. Website framework preferences are retained
for a separately scoped site build; this delivery supplies the tested guide and
content/layout plan requested by the user.
