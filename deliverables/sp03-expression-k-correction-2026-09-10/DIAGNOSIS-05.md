# Correction 05 diagnosis

## Observations

`execution-04/FAILURE.json` records a successful compile and 105 matching fixture observations. Trace 106, `metadata-depth-5947`, then exited 139 in `kore-expand-macros`; no K rule result was produced. The retained trace command used the pinned `krun` with a positional KAST input and the repository `expression-parser.py` parser.

The retained compiled-artifact manifest records `macros.kore` with SHA-256 `3585c084b81281ec26971b2e42c56b9968a802c5d645a81062296350493491cf`. The observed file content was ten bytes: `// macros\n`. The same manifest records `syntaxDefinition.kore` SHA-256 `852a4de6a49c7f1464fc0adea4ec2b9fb7c12a6801ca0e3042c0bccdf2aeabb3`; inspected aliases were `weakExistsFinally`, `weakAlwaysFinally`, and `allPathGlobally`.

The pinned parser wrapper execs pinned `kast --input json --output kore --definition <compiled definition> <input>`. The pinned `krun` wrapper calls `kore-expand-macros` by default and its `--no-expand-macros` option bypasses that phase. The generated request KAST vocabulary across all 115 retained fixtures is `expressionRequest`, `ejObject`, `ejCons`, `ejNil`, `ejPair`, `ejString`, `ejArray`, `ejBool`, `ejNumber`, and `ejNull`. The deepest retained valid request has structural depth 35,726; traversal must be iterative.

The post-terminal service record says `LimitSTACKSoft=8388608`, but it is not a measurement of the failed child. It is therefore not a diagnosis of stack exhaustion and this candidate does not amend a stack resource limit.

## Candidate predicate

The runner appends `--no-expand-macros` only when it can verify all of these conditions after compilation: `macros.kore` is exactly the retained empty artifact, the compiled syntax alias set is exactly the retained three-name set, the request KAST has only the fixed transport vocabulary and no alias label, and the fixed generated configuration cell vocabulary is disjoint from the compiled aliases. The existing runner increments `krunInvocations` before it builds the argv; a `MACRO_GUARD` failure therefore remains recorded as a conservative consumed attempt, while no subprocess is dispatched.

The configuration check matters because the compiled initializer contributes generated cells before the macro phase. It is a static fail-closed check of the reviewed initializer vocabulary, not a dynamic inspection of an initializer term. The source and compiled-artifact binding must both match before the guard runs. It does not infer that an empty `macros.kore` makes arbitrary macro processing an identity. The candidate is limited to the reviewed compiled artifact, the repository parser boundary, and the expression transport/configuration vocabularies. The proposed one-compile, full-115-trace execution remains required to check actual behavior for every constructor and adversarial fixture.

## Offline verification

The replacement runner tests intercept `command` and never invoke K. They verify the final argv contains `--no-expand-macros`, malformed macro artifacts and injected aliases fail closed, and the retained `metadata-depth-5947` request reaches the mocked command boundary. The latter executes the iterative guard over the retained deep input.
