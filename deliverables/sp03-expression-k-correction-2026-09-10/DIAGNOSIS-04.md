# Expression K parser diagnosis 04

The admitted correction-03 compiler exited 113 with 41 inner-parser errors and zero trace invocations. The retained stderr identifies `==K` and `=/=K` in all eight expression K modules as the failing token class.

The pinned primary source is `expression-toolchain.lock.json`, whose `domains.md` digest is `eb5b7c3cd001d5fe3cbd481598bb53c3abfd417c409135276008d50edf79aa0d`. Its lines 2320-2355 define `==K` and `=/=K` in `K-EQUAL`; lines 54-59 show that it must be imported when these generic equality operators are used.

`MORIARTY-EXPRESSION-JSON` is the root of the required expression module chain. Adding its `imports K-EQUAL` exposes the grammar to JSON, wire, type, schema, shape, value, inference, and V1 modules without changing any rule, evaluator, parser, or fixture byte. The compiler has not been invoked after this repair.

The focused regression is `ExpressionRunner.test_expression_equality_syntax_is_imported_at_root`. It failed before the import and passes after it. It checks the retained source invariant only; parser acceptance remains the sole predicate for the proposed admitted compile.
