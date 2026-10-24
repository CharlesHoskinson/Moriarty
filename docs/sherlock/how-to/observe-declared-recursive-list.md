# Observe a declared recursive nominal list

Use the public Rust body-typed nominal compiler when you need the admitted owner image and complete owner-step records for a source declaration. This guide follows one [actual ordinary host case](../../results/sherlock-original007-recursive-nominal-list-20261020/README.md), recorded on 2026-10-20. It introduces no CLI command or native operation.

## Declare the type and ordered value

The complete [executed source](../../results/sherlock-original007-recursive-nominal-list-20261020/source/original007-recursive-list.mori) is:

```text
language "moriarty-sherlock-nominal/1";
program Original007RecursiveList {
  type ListUInt128 {
    Nil;
    Cons(head: UInt, tail: ListUInt128);
  }
  entry construct ListUInt128::Cons(
    1,
    construct ListUInt128::Cons(2, construct ListUInt128::Nil()));
}
```

`ListUInt128` is a user-declared nominal type. Its `tail` field refers to the same type; the entry constructs a finite value. There is no function declaration or recursive function call in this fixture. `Cons` takes `head` before `tail`, and `Nil` has no fields. These constructors retain their nominal identities; their names do not make them the dedicated builtin List constructors.

## Supply explicit inputs

Use `compile_nominal_source_host_bounded` from `mori_core::sherlock_compile_artifact::nominal_body_compile`. Supply the source, every `DraftParseBudget` field, an `ExtCompileConfig` and optional `LocatorC`. The [checked configuration](../../results/sherlock-original007-recursive-nominal-list-20261020/source/configuration.rs) uses `ExtDomain::GENERIC_ABSENT`, fuel 4096, stack 64 and explicit bounds/layout/capacities. Those are this fixture's inputs, not product defaults.

The wrapper performs bounded parsing, type preparation and body checking, then passes the same owned program into `compile_ext_host` for actual owner admission and genesis. A successful compile has not evaluated the entry. Check `typed.report()` and capture the initial owner image before stepping. The existing [body-typed compilation guide](compile-body-typed-nominal.md) describes the error variants and repeated type-preparation boundary.

## Keep the admitted pair and record the real owner

The [executed consumer](../../results/sherlock-original007-recursive-nominal-list-20261020/source/consumer.rs) uses these public calls. This is an API excerpt, not a separately compiled standalone example; the linked consumer supplies its configuration, error handling, file writes and assertions.

```rust
let typed = compile_nominal_source_host_bounded(
    SOURCE, parser_budget(), configuration(), None,
);
```

After handling the result, `typed.owner().admitted()` and `typed.owner().genesis()` borrow the real pair for `complete_observation::capture`. The consumer then transfers ownership:

```rust
let (_, _, owner) = typed.into_parts();
let (admitted, mut image, preparation) = owner.into_parts();
```

Call `complete_observation::record_step(&admitted, &mut image)` on that pair. It captures the full before image, calls the existing fixed owner once, then captures the after image. `ObservedStep::Event` contains before/event/after; `ObservedStep::Terminal` contains before/after and creates no event. Retain the complete result before interpreting it. The executed consumer flushes every JSONL record, uses a 512-call ceiling, stops on actual Terminal and captures the final image. It does not re-admit, select a replacement evaluator or synthesize a result from the source.

## Read the captured value and its limits

The [actual final document](../../results/sherlock-original007-recursive-nominal-list-20261020/actual/outputs/final.json) preserves the copied owner snapshot, root reference and separately resolved ordered value:

```text
Cons(UInt1, Cons(UInt2, Nil))
```

The consumer resolves type/constructor IDs through the admitted declaration tables, follows Const/Data references and checks the entire value. It also checks actual ConstructorFields frame/source sites, boundary continuity, unchanged admission/preparation and preservation of already-active value prefixes. The named expectation was frozen before this consumer but is source-aware; it is not an implementation-unexposed full-trace oracle.

Three actual phases passed: external consumer compilation, listing exactly one test, and running `original007_declared_recursive_list_has_ordered_nominal_value` once. The same qualified Core187 `be9d28c9…` rlib was reused. All nine outputs are retained; [records.jsonl.gz](../../results/sherlock-original007-recursive-nominal-list-20261020/actual/outputs/records.jsonl.gz) expands to 846,469 bytes, with 70 Event records and one Terminal from 71 owner calls. Those counts are observations, not reference/helper work clocks. Exact independent B clocks and a full independent state/event trace remain open.

See the [DATA representation connection](../reference/c6-physical-host-codecs.md#declared-recursive-list-owner-rows) for these nominal rows and the [evidence-layer explanation](../explanation/nominal-text-to-admission.md#observed-body-typed-owner-path-october-20) for scope. This finite case does not close UIREQ007, its binder/formal/full-consumer/refinement obligations, or native/proof/target requirements.
