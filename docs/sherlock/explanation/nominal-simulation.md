# Simulating a body-typed nominal program

`mori simulate` accepts the existing explicit nominal body-typed source profile
and a closed entry. It parses and checks the source, admits the lowered program,
creates genesis and runs that same real owner pair. The source still declares
`language "moriarty-sherlock-nominal/1"`; the body-typed profile selects the
all-body checking route. It does not promote Core/1 or structural nominal input.

These two public invocations use the checked-in fixed configuration and target:

```sh
mori simulate crates/mori/tests/fixtures/nominal-simulate/map-queue.mori \
  --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-nominal-body-typed/1 --entry \
  --execution-config crates/mori/tests/fixtures/nominal-simulate/config.json \
  --target crates/mori/tests/fixtures/nominal-simulate/target.json

mori simulate crates/mori/tests/fixtures/nominal-simulate/mirror-arbor.mori \
  --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-nominal-body-typed/1 --entry \
  --execution-config crates/mori/tests/fixtures/nominal-simulate/config.json \
  --target crates/mori/tests/fixtures/nominal-simulate/target.json
```

The actual selected handler for these arguments passed through an SDK-free direct
consumer. Both captures exited successfully with body checking completed and an
observed Halted outcome. The whole default backend CLI binary was not built or
run by these checks.

| Example | Complete source result checked by the positive test | Captured owner events | Consumed owner work |
| --- | --- | ---: | ---: |
| MapQueue | `Envelope<Queue<UInt>,Bool>` wrapping the queue `[5,10]` and `true` | 360 | 359 |
| MirrorArbor | `Stamp<Arbor<Bool>,UInt>` wrapping `Fork(Fork(Tip(false),Tip(true)),Tip(false))` and `41` | 546 | 545 |

The tests check the full typed constructor trees, including concrete generic type
arguments, every field name, declaration order and child value. Their expected
values were authored before production changes. They also compare every full
StepEventC in order against fresh genesis on the same admitted program. This is
an in-memory check against a shared owner producer, not an independent semantic
oracle. The vectors were not separately persisted.

The handler emits the existing `sherlock-nominal-host-observation/1` envelope on
stdout. It retains the complete preparation with all 23 ProgramView fields,
original source/config/target identities, actual terminal image, summary and owner
resource report. Only actual Halted maps to command success. The production sink
retains no events: output reports the real forwarded count and
`event_capture: "not-retained"`. This is not a complete before/after state trace
or an import/replay codec.

The eight ordinary checks passed: test-consumer link with 14 source/include
inputs, exact three-test list, the two generic tests and the existing 24-shape
argument/selection test, normal handler link with 11 inputs, and the two public
handler captures. All compiler and runtime stderr streams were empty. The normal
Core library previously qualified for PR363 was reused unchanged; no Core build
or prior passing source05 test was repeated. PR363 merged as
`f49b0e20677142b1e588f2878b371f061d84e773`.

The [positive test source](../../../crates/mori/src/commands/simulate_nominal_tests.rs)
and [bounded actual results](../../results/sherlock-nominal-simulate-20261020/ACTUAL-RESULTS.json)
name the tested scope. The linked lossless raw archive retains all eight ordinary
receipts, complete stdout artifacts and Root's preserved postcapture interpretation
correction; it contains no compiled binaries or private oracle bodies.

The command adds no numerical defaults, flags, decoder, guard, wire tag or engine.
Existing draft and legacy formats and selection/refusal behavior remain intact;
the 24-shape test covers argument selection, not actual draft execution after this
refactor. External nominal-input flags remain on `mori compile`; this increment
selects the source's closed entry.

Failure/refusal, Rejected/Exhausted, serialization and sink-panic runtime branches
remain unexercised here. Overlapping first-arm matching, complete state continuity,
independent semantics, witness/constraint plans, native export, proof, target
qualification, formal refinement and full B04/EARS acceptance remain open.
