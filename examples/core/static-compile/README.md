# Public source-static native call bundles

These examples compile every declared `language "moriarty/1";` action to actual
canonical tagged native-call ZKIR and its existing relation manifest. They use
public static bindings and fixed public signer-policy points. They need no
request, predecessor, genesis, frame, key, parameters or SRS files.

From the repository root, choose a new output directory:

```sh
mori compile-call examples/core/static-compile/numeric-state.mori \
  --all-actions --authorization unsigned \
  --static-bindings examples/core/static-compile/numeric-state.bindings.json \
  --work-limits examples/core/static-compile/work-limits.json --out numeric-static

mori compile-call examples/core/static-compile/numeric-state.mori \
  --all-actions --authorization unsigned \
  --static-bindings examples/core/static-compile/numeric-state.bindings.json \
  --work-limits examples/core/static-compile/work-limits.json \
  --inspect-static numeric-static

mori compile-call examples/core/static-compile/financial-duty.mori \
  --all-actions --authorization signed-v2 \
  --signer-policy examples/core/static-compile/signer-policy.json \
  --static-bindings examples/core/static-compile/financial-duty.bindings.json \
  --work-limits examples/core/static-compile/work-limits.json --out financial-static

mori compile-call examples/core/static-compile/financial-duty.mori \
  --all-actions --authorization signed-v2 \
  --signer-policy examples/core/static-compile/signer-policy.json \
  --static-bindings examples/core/static-compile/financial-duty.bindings.json \
  --work-limits examples/core/static-compile/work-limits.json \
  --inspect-static financial-static
```

The numeric source captures a prior UInt, updates UInt and Bool fields and
checks postconditions across two actions. The financial source emits two
payments to the same recipient, a separate fee and an accrued duty; repayment
retains the subtraction and residual-duty checks. These are compilation
examples, with no claim of ledger effects.

The bundle contains `source.mori`, source-order ordinal pairs
`action-000000.ir.bin` / `action-000000.manifest.json`, and
`static-compile.json`. Export reports `source-static-unproven-uninstalled`.
Inspect requires independent caller source, bindings, authorization and limits;
it recompiles and reports `source-static-recompile-admitted` after exact byte
comparison. Marker presence alone establishes no authority.

New input and marker JSON reject explicit null, unknown fields, decoded duplicate
keys, trailing values and excessive depth/nodes. Omit `economic_bounds` for None.
This rule does not change historical relation manifests or legacy DTO codecs.
The public contract-address and network fields are supplied local premises.
They do not authenticate a deployed contract or account.

The explicit work-limits /2 file bounds input bytes, every action's groups and
public fields, each source/IR/manifest/marker leaf and the entire bundle. Input
budgets count the larger raw/typed representation, including the limits file;
typed legacy None serialization is conservatively sized internally. The four
unused native fields are `1` only to satisfy their positive schema: max_degree,
max_srs_bytes, max_prover_key_bytes and max_verifier_key_bytes. Compilation never
measures degree or accesses that material. Do not hand this example limits file
to keygen as degree, resource or SRS authorization.

Publication reserves a new directory and writes complete exclusive leaves,
with the marker last. Failure may leave a partial directory. This is not atomic
whole-directory publication or crash durability. Reopening rejects symlinked
roots/leaves, nonregular files and any extra entry. It uses Unix no-follow handle
reads; unsupported hosts refuse safe reopening. Stable ancestor directories are
required; arbitrary ancestor replacement or simultaneous filesystem isolation
is not promised.

The source closure includes this static module and CLI consumer. Changes make
old exact signing contexts, IR, key identities and audit applicability stale.
This delivery opens, regenerates and qualifies no keys, parameters or SRS.
Existing signed manifest qualification retains its historical installed label;
that label does not install these key-free bundles. M1/M2/U2, semantic/native
correspondence, 64 KiB transaction and k19 feasibility, funding, custody, state,
time, history, replay consumption, PCD and financial settlement remain open.
