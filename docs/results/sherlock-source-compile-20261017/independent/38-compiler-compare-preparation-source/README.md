# R38 additive preparation and bounds comparator

`compare_preparation_bounds.py` extends the exact sealed R26 source through a pinned pure-Python import. It verifies R26 freeze/all8 members/source hash before loading it, and R26 in turn verifies the R20 correction02 freeze/all41 members. No old file is edited. Usage for Root after source inspection:

```sh
python3 compare_preparation_bounds.py /path/to/already-captured-artifact.json --case quote
python3 compare_preparation_bounds.py /path/to/already-captured-artifact.json --case eligibility
```

Exit0 means all selected predicates match, with explicit unsupported fields; exit1 means selected mismatch; exit2 means input/contract error. `comparison_complete` remains false. This packet does not rerun the producer and contains no capture/result data.

The extension compares all22 `/admission/bounds` fields exactly to the frozen explicit config/layout bounds, including decimal-string types, values and exact keys. The pointer and owner-record wire rule come from the Root-supplied body-free declaration binding. It does not infer target qualification.

For `/preparation`, frozen ScalarActionReport/LetSite/EmitSite/SourceOccurrence declarations select exact key inventories and presence. Exact independently frozen values are action index/name, empty inputs, ordered let statement indices/names, emit statement/name/role, occurrence count and ordered body_paths/action indices, return type and generated function name. Arrays retain order/cardinality/multiplicity; no absent nullable field is treated as equivalent to a present null. The unchanged report serde uses JSON numeric action/statement/path indices. Root's public alias declaration confirms `crate::ir::ExprId=u16`, so raw expression references select numeric JSON integers within0..65535, unlike owner-record decimal strings. All selected types distinguish bool from integer. No target host ABI upper width is invented for unvalued usize metadata.

Raw IDs themselves remain unknown: let/emit expression IDs, optional occurrence expression IDs, per-occurrence statement-index values, node_kind/symbol_domain spellings and source_expression values are explicitly UNSELECTED by exact path. Their declared types/presence are checked without filling values. The public CoreExpr/Value enums allow partial structural type checks for non-null source expressions, but the selected frozen snapshot lacks the CoreUInt serializer definition. A CoreExpr Literal/Value UInt payload therefore has presence only, with its payload type/value explicitly unsupported. No serializer shape is guessed. `PUBLIC-PREDICATES.json` provides every selected value/cardinality/key set and exact unsupported path.

Raw preparation is retained in the report alongside R26's raw/normalized program view and hygienic mapping. Known parser-ID slots in synthetic tests use visibly synthetic numeric placeholders; those are not authentic expectations.98 local checks compare synthetic golden copies and changed selected cells, including every bound in both cases, report values/types/required nullable fields, let/occurrence order/cardinality, and explicit noncomparison of unsupported exact values. Only inspected Pythonstdlib runs.

Expected semantic values come exclusively from the already sealed R20 source-report projections and config/identity/layout. New public facts come only from the frozen body-free field/alias/pointer declarations; no producer body, R31 artifact, actual comparison report or output dump was read. Root has existing captures and will compare those same artifacts after this source freeze; no claim is made that this comparator predates those captures. This author has not received or read actual artifacts. Full preparation-value/full-artifact/runtime/native/general-compiler and security judgments remain outside this packet.
