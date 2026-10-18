# Additive nominal identifier correction

Root's actual frontend reported QNT101 for local `fields` at line212 and
lambda parameter `field` at line218. The first author audit incorrectly treated
builtin.qnt as complete and did not inspect lambda binders. The installed
names/base.js builtinNames array is the collector's actual collision inventory.

Revision02 changes only build() local fields->fieldTypes and field->fieldValue
plus their lexical references. Record labels `.fields` and `fields:` remain
unchanged. Fixtures, bounds, values, work, transition formulas, source metadata,
all tests, README and expected literals retain their exact bytes. The original
source/freezes/proposal and root failure receipts are preserved unchanged.

The new text-only audit inventories all concrete declaration/local/annotated
parameter/lambda/match binders in the3 executable sources against the installed
collector inventory and lexer keywords:284 binder occurrences, no remaining
listed collision. This is not a Quint parser/type/effect or runtime result.
Actual corrected frontend/tests/simulation remain unrun by the author and need
fresh exact-source non-author review followed by Root selection/execution.
