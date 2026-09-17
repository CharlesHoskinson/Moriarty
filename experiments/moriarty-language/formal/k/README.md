# Bounded funded financial semantics in K

## Offline lifecycle corpus prerequisite

From the repository root, run:

```sh
npm --prefix experiments/moriarty-language run lifecycle-corpus
node --test experiments/moriarty-language/tests/lifecycle-k-corpus.test.mjs
```

`lifecycle-corpus.mjs` checks and elaborates the frozen `loan-lifecycle.mori`,
then executes its four actions through Source `/5` and Core `/4`. Each evaluator
consumes its own previous ordinary state, financial state and remaining work.
Complete results are compared with the independent
[financial templates](../../../../deliverables/language-to-ledger-2026-09-12/design-review/lifecycle-source-expectations.json)
and [source-specific work counts](../../../../deliverables/language-to-ledger-2026-09-12/lifecycle/root-work-expectations.json)
in the current checkout. Both files are required. The source hash must match
the work oracle. Costs are 99/65/88/86, leaving 413/348/260/174 work from seed 512;
spent work is 116/181/269/355 and the closure reserve stays 16.

The bounded negative subset covers duplicate accrual, a repeated period with a
fresh identity, time 1059 before the 1060 boundary, an incurred liability cap after
partial repayment, insufficient repayment funding, the settled-source guard,
a false final Ensure, a malformed unrelated allowance, and mismatched work.
Every rejection compares complete Source/Core envelopes with independently
specified diagnostics, checks that no state/effects are published, checks unchanged
inputs and repeats the rejected call. Eight cases also repeat a valid continuation
from the accepted predecessor. The settled-source guard has no valid continuation
in this source. The early-period and funding cases reuse their failed identities;
the final Ensure failure uses 86 diagnostic work and commits no debit.

The JSON report retains complete inputs, results, independent expectations and
comparison failures. A failed check, elaboration, incomplete run, missing evidence
or mismatch exits nonzero. The structural comparator detects missing/extra own
keys (including `undefined`), types, debt-component changes, allowances, work,
effects and array order. Mutation tests exercise the comparator and the CLI.

This is a Task4 prerequisite with `kExecuted: false` and
`kStatus: "not-executed"` on every report. It does not invoke or admit K, reproduce
the retained backend failure, implement a Core-to-K loader, cover the full planned
failure matrix, or establish correspondence, proof or ledger acceptance. The K
work and admission requirements below remain open for this lifecycle.

## Existing bounded K definition

The current definition supports one Transfer, optionally followed by one Repay,
with AccrualFirst, PrincipalFirst or ProRata allocation and explicit none/floor/ceil
conversion rounding. [Numeric execution evidence](../../../../deliverables/numeric-k-2026-09-09/README.md)
records the current candidate's execution and review status. This remains a
provisional local projection, not full Core semantics or a correspondence theorem.

## Admission and financial checks

`codec.admit(text)` accepts at most 65,536 UTF-8 bytes of closed JSON records,
canonical UInt128 decimal quantities and valid enum/identifier syntax. Duplicate
JSON keys reject. The fixed shape remains two initial balances, one allowance,
one obligation, empty used-ID lists and [Transfer] or [Transfer, Repay]. Other
well-formed shapes are UNSUPPORTED_PROJECTION; malformed records are MALFORMED_INPUT.
Both stop before K and are distinct from K's financial rejection records.

Conversion fields and allocation policy are passed into K. The host does not
filter zero mantissa, oversized scale, invalid sums, insufficient funding or any
other financial failure. K checks positive mantissa, scale ≤18, debt invariants,
work and action predicates in reference order. A Transfer-only packet validates
its supplied obligation but does not allocate against it. A settled ProRata
obligation with all debt components zero is therefore legal for Transfer-only.

K uses stages so exponentiation is reached only after the scale guard and division
only after the product overflow guard. Settlement uses nominal*mantissa / 10^scale;
none requires exactness, floor truncates and ceil adds one for a nonzero remainder.
Positive nominal payment with zero settlement rejects DUST. Converted settlement,
not nominal quantity, is checked against the preceding transfer's funding.
ProRata checks nominal*principal fits UInt128 before dividing by positive total debt.
Allocation component bounds are checked before finalization. All failures erase
the continuation and expose only code/index; there is no observable partial output.

The [root README](../../../../README.md) presents the control layer using
Felleisen–Hieb reduction semantics. Transfer-only has 15 guards and 18 successful
presentation steps; repayment has 28 guards across its stages and 37 steps.
Actual action-work remains one or two. No mechanized theorem is claimed.

## Codec boundary

`encode` passes the real fields in distinct `transferPacket` or `packet` constructors,
including nested `conversion(mantissa,scale,rounding)`. K returns every changed
financial number. `preparedTransfer` contains seven changed amounts/counters plus
receiver index and digest; `prepared` contains thirteen amounts/counters, debt
status, receiver index and digest. Their KAST v4 arities and UInt128 outputs are
checked, and success shape, input digest and recipient index must match the input.
A one-action packet cannot return a repayment-index rejection.

The trusted decoder copies unchanged metadata, constructs ordered effects, and
appends tombstones. For Transfer-only it copies complete debt and allocation history;
K does not emit those unchanged fields. This is the combined codec/K path's result,
not an independent debt calculation or proved invariant. The decoder validates the
observed empty K continuation when present but permits an out-only synthetic wrapper
for offline tests; it does not authenticate arbitrary KAST messages. Actual compiled
runs and source/artifact bindings provide the runtime evidence. Raw K terms outside
codec admission and general source/Core translations are outside the claim domain.

## Commands and allocation limits

Existing suites remain at one compile ≤180s, at most 16 krun attempts ≤20s each,
and 512s aggregate. The new closed `numeric` suite permits one compile ≤180s,
at most 64 sequential krun attempts ≤20s each, and 1512s aggregate including wrapper
overhead. Root must admit the exact candidate and resource proposal, then supply
whole-tree systemd containment with MemoryMax=4G, MemorySwapMax=0, the selected
aggregate deadline, control-group SIGKILL and the nonblocking shared flock.

```sh
python3 run.py --suite numeric compile-and-traces --all
```

`initial` remains the default. `branches` and `transfer-only` retain their original
sixteen-call ceilings. Source bindings include the selected fixture bytes.
Changing the definition/codec/runner/toolchain or selected fixtures prevents reuse
of a compiled binding. Compile-attempt and krun-counter files are retained even on
failure; never remove them to retry. There is no automatic fallback or recompile.
Each experiment uses its separately admitted fresh build and preserves prior charges.
`prove` still fails PROOF_UNIMPLEMENTED. Separate compile/evaluate/traces processes
cannot be used to reset or extend the combined experiment's aggregate budget.

## Evidence and remaining work

[Initial sixteen](../../../../deliverables/bounded-k-2026-09-09/README.md),
[sixteen additional branches](../../../../deliverables/repayment-k-branches-2026-09-09/README.md)
and [ten distinct Transfer-only cases](../../../../deliverables/transfer-only-k-2026-09-09/README.md)
retain their original immutable receipts; they do not silently become executions
of this revised K definition. The numeric suite reruns all 42 distinct earlier cases
alongside 22 newly independent numeric expectations, comparing complete outputs
against source preparation and the actual successor CLI.

Arbitrary action sequences, larger collections, nonempty history/replay, new Core
constructs, public Pending/Complete behavior, source elaboration preservation,
general semantic theorems and compiler/proof/ledger correspondence remain open.
No Preview transaction or financial settlement is implied by local K execution.
