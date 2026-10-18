# Mechanical renderer integration proposal

Status: UNREVIEWED SOURCE CANDIDATE. This helper and its interface are proposals for the final Opus integrator.
Fresh Muse review and a full combined non-author Opus review remain pending.
No source adoption, publication or builder reliance follows from this packet.

The helper is `_render_prepared_outputs(code_hex, policy, case_rows, manifest_fields)`.
It is private, so it adds no public builder API.
The supplied file is a `.py.txt` fragment, not an imported module.
The existing prefix already binds `legacy` and `json`.
The fragment adds no import or module assignment.

The requested result is six relative-path-to-bytes entries from already-checked arguments.
The falsifiable predicate is limited to deterministic rendering and copying with the pinned dependencies.
No existing source changes are permitted in this task.

## Required arguments and dependencies

The final integrator owns every precondition below.
The helper performs no policy, schema, cryptographic or bridge admission.
Do not use this helper as a public input validator.

| Argument | Required already-checked value |
|---|---|
| `code_hex` | The exact compiled-code string from Opus-owned pinned extraction. No newline is included. |
| `policy` | The Opus-checked policy bytes for every prepared row. |
| `case_rows` | An exact four-element tuple or list, ordered T1, T5, T6, T7. |
| Each row | An ordinary exact dict with `env`, `token`, `quantity`, `vk`, `proof`, and `manifest`. |
| Row `env` | The already-translated, already-checked legacy Data tree for that row. |
| Row token/quantity/VK/proof | The exact values prepared by Opus for that row. The helper constructs no mutation. |
| Row `manifest` | The six prepared public metadata fields listed below. Their consistency with the row is Opus-owned. |
| `manifest_fields` | All prescribed final manifest fields except `cases`. All values are already checked public JSON data. |
| Dependencies | The unchanged pinned legacy module and standard `json` module already bound by the prefix. |

Each row's manifest dict supplies `envelopeCborHex`, `tokenNameHex`, `quantity`,
`envVkDigestHex`, `envStatementDigestHex`, and `proofHexSha256`.
The helper copies those fields without computing them.
It adds only the fixed historical case ID and its context filename.
The final integrator owns the case identities, declared expectations and all byte/hash equalities.
No acceptance or rejection outcome is inferred from a filename.

Use only already-checked builtin data without caller-defined hooks.
Keep inputs and dependency modules unchanged during the call.
All metadata must be finite, acyclic and suitable for standard JSON serialization.
These are interface preconditions, not new source admission rules.
Outside this domain, ordinary Python or dependency exceptions can propagate.
The helper supplies no new refusal code or exception mapping.
Opus owns the final stage order and error disposition.

## Finite behavior

The helper unpacks exactly four prepared rows.
It calls `legacy.script_context` once for each row.
It passes the supplied policy, environment, token, quantity, VK and proof unchanged.
It calls `legacy.uplc_term` on each resulting legacy context.
It never calls a host encoder, consumer, hash function, legacy CBOR encoder or case mutation helper.

The four case IDs remain:

- `K10-T1-positive-genuine-issue`
- `K10-T5-quantity-two-reject`
- `K10-T6-token-name-bitflip-reject`
- `K10-T7-vk-digest-bitflip-reject`

The helper returns `script.hex`, `manifest.json` and four `contexts/<case-id>.uplc.txt` entries.
It formats script bytes as `code_hex + "\n"` in UTF-8.
It formats each context as `"(con data (%s))\n" % legacy.uplc_term(context)` in UTF-8.
It formats the manifest with `json.dumps(manifest, indent=2, sort_keys=True) + "\n"` in UTF-8.
Standard JSON defaults remain unchanged, including `ensure_ascii=True`.

The output dict and both manifest dict levels are new local objects.
The helper only reads nested metadata values during serialization.
It never changes supplied objects, their nested structures or dependency modules.
Only the final bytes escape through the new output dict.
The helper performs no I/O, clock, environment, randomness, network, subprocess or native operation.

## Manifest handoff

Opus supplies all manifest scalars, nested dicts and lists through `manifest_fields`.
The helper copies them and adds the four copied case records.
It does not select or construct labels, flags, limitations, provenance or setup/profile assertions.
Preserve the original legacy strings for script classification, preimage and algorithm in the supplied metadata.
Their source is S05 lines 512-525.
Their presence does not mean this helper computed any hash.

Supply the T2-T4 historical coverage gaps through `manifest_fields["coverageGaps"]`.
Their original names and `NOT_GENERATED_HISTORICAL_FIXTURE_ONLY` status come from S03 and S02.
No T2-T4 context is produced here.
The helper neither creates proofs nor derives negative expectations.

Copy `consumerFormat` and `consumerLabels` as opaque values.
The helper does not pin their format, key count or literal values.
The current worktree v1 consumer is inspected context only, not a permitted builder dependency.
Builder reliance on v1 remains prohibited by the current assignment.
CP r2 policy is live according to that assignment and is not implemented or assessed here.
The frozen CB contract/test snapshot still names the old consumer format and eight labels.
An explicit CB revision and a fresh full combined Opus audit remain required before reliance.
This mechanical interface does not resolve that dependency revision.

## Source citations and integration limits

S01 CB10 and S02 `/outputs` prescribe the six output files and formatting.
S03 `CASES`, `CASE_KEYS`, `ROSTER` and `test_09` prescribe the historical identities and metadata names.
S05 `script_context` lines 341-363 and `uplc_term` lines 268-282 supply the reused render path.
S05 lines 485-502 supply the original context layout and case metadata layout.
S05 lines 503-543 supply original manifest fields, strings and formatting.
S06-S09 identify the prefix imports, immutable append seam and incomplete integration scope.
S10 and S11 are current host/consumer context. The helper calls neither one.
All IDs resolve through `SOURCE-MANIFEST.json`.

Append the fragment only within the integrator-owned candidate after the immutable prefix seam.
Keep the existing prefix and legacy builder bytes unchanged.
Call the helper only after the final Opus-owned preparation and checks have completed.
Opus owns all security checks, case construction, hashes, byte equalities, policy and bridge validation.
Opus also owns JSON admission, filesystem/argv rules, stage order and the exclusive writer.
This packet implements none of those functions and does not supply `plan_outputs` or `main`.
No complete-builder GREEN, native/VM, profile, Groth16, PCD or ledger result is observed here.
The genuine current-language, financial, full20, ACTUS/DeFi and general PCD objectives remain open.
Preview-first and later Preprod acceptance remain separate open obligations.
