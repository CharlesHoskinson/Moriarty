# Canonical runtime type carriers

Three Rust compiles and ten new tests passed for the canonical TypeId migration: five owner V8 tests, four builtin runtime cases and one nominal-storage case. The [result summary](../../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json) identifies the exact source and receipts.

`mori_core::sherlock::helper_machine::shared::TypeIdC` reexports `shared::adt::TypeIdC(pub usize)`. Source `CoreFunction.return_type` and `CoreParameter.sort` retain `SortTag`; conversion preserves UInt = 0, Bool = 1, List = 2 and Tree = 3.

| Carrier | Field |
| --- | --- |
| `FunctionRowC` | `return_type: TypeIdC` |
| `ParameterRowC` | `sort: TypeIdC` |
| `HelperTaskC::PairTask` | `sort: TypeIdC` |
| `ControlC::Bound` | `return_type: TypeIdC` |
| `FrameRowC::ReturnF` | `return_type: TypeIdC` |
| `HelperControlC::Scalar` | `expected: Option<TypeIdC>` |

`host_owner_engine().prepare_core(...)` produces the actual `ProgramView` signatures. The existing owner consumes those rows through admission and execution. The tests follow their identities into task, control, frame and helper state, then compare bound values and results with independent expectations.

Compile-artifact diagnostics preserve builtin wire strings `"UInt"`, `"Bool"`, `"List"` and `"Tree"`. Other stored IDs render as `{"TypeId": id}` through the integer wire representation. V8 still refuses nominal parameter and return signatures at the actual rows, with validation precedence preserved.

IDs 4, 5 and 17 roundtrip through all six storage roles. This establishes storage only; generic nominal admission, ConstructorFields and ordered MatchPlan runtime remain unfinished. Absolute independent B clocks remain unbound. Original UIREQ034 and all 25 refinements remain open.

See the [inspection guide](../how-to/inspect-canonical-runtime-types.md) and [runtime explanation](../explanation/whole-program-types-and-runtime.md#canonical-typeid-through-the-existing-owner). The [Root compatibility adoption record](../../results/sherlock-canonical-runtime-types-20261018/ROOT-CANONICAL-TYPEID-COMPATIBILITY-ADOPTION.json), installed with this migration, records the retained original-clause scope.
