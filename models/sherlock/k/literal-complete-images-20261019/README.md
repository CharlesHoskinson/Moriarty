# Complete logical-image literal counterpart

The source-specific definition compiled successfully: exit 0 in 14.951 s. Its finite two-event K observation subsequently matched an independently prefrozen complete expectation, including all 335 statement lanes. Root's host/K comparison also passed 2,262 typed recursive checks with zero mismatches across all eight machine positions, full tables and padding, both envelopes and all statement lanes. [Qualified source and result metadata](../../../../docs/results/sherlock-k-literal-complete-runtime-20261019/README.md) preserves the source audits, original framing failure, synthetic 20-case unit and successful tail.

The parser correction changes only empty K-continuation framing. Original runtime children 01–05 were reused; the failed original 06 remains immutable. Unit 20 and new 06/07/08 passed, with all 89 actual files frozen before expected access. SAFE, raw K images, host capture and independent expectation/constructor bodies remain private. Public metadata cannot by itself reproduce those instance comparisons. Source/result publication review is pending Root.

There is no original main.k rule-fired trace, context authentication, binding of the caller context to this counterpart source, general refinement, native proof, recursive verification, stagenet or global EARS acceptance. `context_binds_counterpart_source=false`. The historical design note below retains the original proposal premises; its earlier uncompiled, unrun and pending statuses are superseded only by the finite observations above.

## Original source-design note


Source-only, unreviewed, uncompiled and unrun. This isolated K family computes
one positive UInt-literal `Init -> Return -> Done` path over the existing complete
logical-image geometry. It does not modify or import the legacy nine-cell
`main.k` or the separate nominal/ng model. Neither cached compiled definition
contains this new module. A new source-specific definition is required.

`literal-complete-images.k` declares both modules:

- syntax: `MORIARTY-LITERAL-COMPLETE-IMAGES-SYNTAX`
- semantics: `MORIARTY-LITERAL-COMPLETE-IMAGES`
- explicit input sort: `LCRun`

The sole non-project import is the pinned K distribution's `domains.md`.
No registration in `main.k`, source hash replacement, or relabeling of old K
results is intended. The exact new module path belongs in a separately reviewed
changed-source kompile command, using the existing tool/capture limits.

## Carrier and computation

`lcMachine(context, capacities, program, state)` retains all supplied fields.
`lcImage(root, tables)` retains every allocated row and every padding lane,
including zero-capacity tables. Context is two explicit UInt128 integers; its
range is checked, but its relationship to identity hashes is an external premise.
It has no default. No made-up context or A102-only context is accepted implicitly.

The positive input premise requires a complete empty Init, positive sixteen-byte
little-endian fuel equal to initial fuel, empty arenas/frames/environment, Pending
outcome, and entry Eval(0). The runtime stackLimit may be below or equal to
capacities.stack_limit; all transitions preserve that actual runtime value.
It selects the actual entry Literal row and its active
Constant-space UInt reference. The value is not restricted to seven. Program
builtins, unused tables and padding are preserved as supplied; the actual checked
producer/source-preparation association is an external premise. This model does
not implement general program admission, uiValid for List/Tree, or all 39 events.

`lcAdvance` dispatches on the actual control tag, then computes the next carrier.
The A102 counterpart subtracts one from actual LE fuel with borrow and writes
Return with the selected Program ValueRef. The A80 counterpart publishes that
actual Return reference, clears ControlRows and consumes no fuel, including at
fuel zero. Both retain the entire immutable Program/context/capacities. Every
other StateRoot lane and all five non-control arenas are preserved. Completion
stops on Done. Observation budget is explicit and separate from semantic fuel;
this complete-pair entry requires at least two event slots. Budget-zero/chunk
observation, error and startup-exhaustion behavior are outside this entry.

The model emits a chronological `lcTrace` containing full initial/after/final
carriers and two complete before/after events. The values `(102,3,1)` and `(80,0,0)`
are declared source-origin/envelope metadata in this new counterpart, not a claim
that the engine applied the original main.k line-102/line-80 rules. Functional K
reductions are not rebranded as raw original-model rule traces. There is no proved
refinement between this model and that model or the backend.

`lcFinish` emits the statement as actual K data, in order:

1. context limbs (2)
2. complete Program root and tables in row-major order
3. complete Init, intermediate and Done roots/tables, in that order
4. both actual event envelopes, `(rule_tag, charge)`

For the already selected scalar profile, Program is 86 lanes and State is 81:
`2 + 86 + 3*81 + 4 = 335`. It is not the unrelated 731-lane Quint fixture.
The adapter derives coordinate metadata from actual capacities but never fills
actual lane values from that metadata or from expected output.

## Real consumer and adapter

The proposed files in `models/sherlock/qa/` are:

- `literal_complete_images_from_source.rs`: real public draft preparation,
  retained reference Arc, real `produce_checked_literal_complete_execution_request`,
  and actual complete `layout.context_limbs()`; no recreated transition producer.
- `literal_complete_images_export.rs`: writes only the actual Program, Init,
  capacities, caller context and observation budget. It does not read the
  producer's intermediate/Done images or statement. It neither validates identity
  nor invokes a backend operation.
- `literal_complete_images.py`: standard-library-only data renderer and decoder;
  no interpreter, process launcher, expected-output import or hashing scheme.

The Rust consumer takes six explicit positional paths: SOURCE CONFIG TARGET
K_MAIN REFERENCE FRESH_OUTPUT. The minimal scalar profile is explicitly fixed
there to program `[0,0,1,0,1,0,0,0,0,0,4,4,5,1,0]`, state `[0,0,0,0,0,1]`,
stack 0, A semantic fuel 1, statement bound 335 and observation budget 2.
K_MAIN must be the pinned legacy `models/sherlock/k/main.k` bytes in which
ruleStartLine 102 resolves, exactly as LiteralSourceIdentity specifies. It must
not be this new counterpart file. The complete-layout context identifies the
existing backend relation and caller-asserted source identities; it does not
identify or attest this new counterpart K source. The new source/definition is
bound separately by its own source and compile-result pins.

Its producer link depends on the real backend crate: it is SDK-PENDING and has
not been declared SDK-free. Its public source identity consists of SHA-256
identifiers of actual supplied bytes; authentication/network selection is not
established by hashing. A B genesis profile in the existing public preparation
remains separate from this A carrier profile.

The Python input schema has exact keys `format`, `context`, `capacities`,
`program`, `init`, `observation_budget`. Format is
`moriarty-k-literal-complete-images-input/1`; capacities has `program`, `state`,
`stack_limit`; each image has `root`, `tables`. All integer JSON parsing is
lossless; booleans/floats are not accepted as integer lanes. No fields are defaulted.

After the real input/context is frozen, the intended data-only invocations are:

```
python3 -B models/sherlock/qa/literal_complete_images.py render \
  --input INPUT.json --output FRESH-query.term
python3 -B models/sherlock/qa/literal_complete_images.py decode \
  --input ACTUAL-kprint.txt --output FRESH-observation.json
```

These are proposed invocations, not executed receipts. The decoder consumes the
whole emitted `lcResult`, optionally within the single `<k>...</k>` configuration.
It rejects unknown/unevaluated constructors, wrong arity, extra tokens, missing
allocated rows and a statement length inconsistent with emitted capacities.
Its output retains the actual full trace/events/statement. The `logical_layout`
field is explicitly derived coordinate metadata. No transition is recomputed.

## Next discriminators and retained limits

The first useful new observation is the real-source scalar profile with complete
Program/three State images, actual complete context, envelopes and all 335 lanes.
It tests new carrier retention rather than replaying the old nine-cell N01 pass.
Further normal input proposals are a multi-byte fuel borrow (256), additional
allocated inactive control padding, and a contrasting valid UInt literal. Each
requires explicit input/context binding and independent expectations before its
own execution. No exact work-clock/cryptographic/native claim follows.

Full-image independent expectation: **NOT_ISSUED**. The old protected nine-cell
oracle stays protected and cannot supply absent image/context fields. A later
independent author must receive only frozen safe Program/Init/capacities/context,
not this implementation or producer intermediate/Done values; freeze its complete
expectation before K output exists. Existing producer/model-source exposure must
be disclosed, rather than described as blind independence.

Pending: independent source/operation review; actual backend closure/admissibility;
actual producer context/input freeze; source-specific K compile and real kprint
framing; independent full-image oracle; raw actual result freeze and complete
comparison. Init authentication, general F4/StepInv, OBL.G4 byte/field encoding,
projectA canonical-boundary materialization, general dispatch/other value sorts,
financial semantics, cross-layer refinement, native proof and target/stagenet
acceptance remain unestablished. This source changes no EARS/adoption status.
