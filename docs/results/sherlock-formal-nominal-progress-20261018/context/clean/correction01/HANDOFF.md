# Clean B01 minimal source correction01

This is an additive correction to the exact reviewed B01, not an edit of the frozen original.
The complete fresh no-tools Gemini4ArgonHIGH report is pinned below. Its source verdict was REVISE;
its ordinary finite frontend classification did not report an execution or approve any later batch.

ConstructorFields.lean has exactly two changes: eight outside-comment uses of the invalid reserved
value binder prefix become fieldPrefix; the nil/nil proof branch uses
`simp [runReturned, beginFrom, schedule]` instead of `rfl`, allowing List.append_nil to rewrite the
variable fieldPrefix++[]. All declaration names/data fields/argument order and semantic propositions
remain unchanged modulo binder alpha-renaming. Every original block docstring/comment remains
byte-identical. CanonicalTypeId.lean and ConstructorFieldsNormal.lean copies are byte-identical.
SOURCE.diff is the entire correction. No proof premise or semantic definition was weakened.

The root causes were checked against installed Lean4.33.1 source: Parser/Syntax.lean registers
prefix as a command keyword at74; Init/Data/List/Basic.lean:621 proves append_nil inductively.
This is source diagnosis; the author ran NO Lean/Lake/parser/elaborator/kernel/model/native process.
The revised proof is SOURCE_CORRECTED_UNRUN until Root's actual selected checks return.

PROPOSED-ARGV.json supplies all four original bounded ordinary frontend commands with only the
corrected source root, a fresh correction-specific artifact/scratch root and distinct -r1 unit names.
The frozen Semantics input stays at its original pinned readset path. Same Lean4.33.1,200s,4GiB,
swap0,tasks256,network-off and16MiB output limit; no Lake, fetching or native operation.
Root creates correction01/proposed-local-check/artifacts/Clean/Sherlock and scratch before selection.
TOOLCHAIN-AND-ENV-REFERENCE.json pins the unchanged original inventory and new explicit env paths.
No original proposed-command/toolchain/source bytes were modified.

DEPENDENCY-INSPECTION.json records B02's unchanged positional uses and the independent same-class
B03 binder issue. B02 still needs its separate review/kernel evidence. A separate additive
../batch03-correction01 supplies only the 12 outside-comment prefix->armPrefix binder renames;
its normal examples are unchanged. No B03 semantic/proof adjustment or success claim is made.
DEPENDENT-B02-PROPOSED-ARGV.json and B03's PROPOSED-ARGV.json point later checks at the corrected
single artifact root and remain UNRUN/separately subject to Root selection. Never use old dependency
artifacts as though they were built from the corrected source.

PUBLIC-SEMANTIC-INTERFACE-correction01.json updates declaration-only source pins for the independent
oracle owner; no populated expected cells were read or changed. The semantic interface is unchanged.
STATIC-CORRECTION-CHECKS.json records source token/width/docstring checks and unchanged original
B01–03 hashes. These text checks establish no Lean result or broad axiom-closure fact.
The full B01 conditional evaluation/caller/helper/bounded/native/financial/target limitations remain.
Root's B04 interface assignment is deferred until this concrete correction is selected and checked.
