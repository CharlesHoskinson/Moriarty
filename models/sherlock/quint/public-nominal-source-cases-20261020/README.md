# Nominal source cases: finite operational model

These five self-contained Quint modules extend the existing nominal-construction model with Boolean-not and two source specimens, MapQueue and MirrorArbor. The model uses the existing call, match, constructor-field, build, return and saved-environment paths. Declaration order is explicit; the same-type field-swap test distinguishes content order.

The retained validation observed a successful frontend, four exact tests (one sample each), and one MirrorArbor witness. The complete trace retains all 17 Snapshot fields in each of 257 states: 108 charged model transitions, followed by 148 inherited terminal stutters. Helper work is zero in this profile. The repeated last-event marker during stutter is one prior publication, not repeated publications.

The public source-derived expected module is included. These are same-author manual expectations frozen before the extension, independent of new Rust test predictions/results; they are not a blinded independent oracle or another reviewer vote. All five imports are local and present. Quint and its dependencies must be installed separately.

From this directory, these commands describe the model's entry points; this publication does not dispatch them:

```sh
quint typecheck nominalPublicSourceRuntime.qnt
quint test nominalPublicSourceTest.qnt --main nominalPublicSourceTest --match '^(q10MapQueueTypedReturnAndOrderTest|q10MirrorArborTypedShapeAndOrderTest|q10SameTypeFieldSwapIsDistinguishedTest|q10ExistingCallBuildPathAndPreservationTest)$' --backend typescript --max-samples 1 --seed 1
quint run nominalPublicSourceRuntime.qnt --main nominalPublicSourceRuntime --init init --step step --invariant allInv --witnesses mirroredNominalPublished --backend typescript --max-samples 1 --max-steps 256 --seed 1 --verbosity 1 --out-itf 'nominal_source_{seq}.itf.json'
```

The exact historical commands, source lineage, full raw streams, result reviews and lossless full-trace archive are in [the evidence directory](../../../../docs/results/sherlock-quint-public-nominal-source10-20261020/README.md).

The profile uses local monomorphic numeric IDs, UInt values at most 255, a frame limit of 32, helperWork=0 and a model terminal charge of 1. The 108 transitions are not Core/compiler clocks. This finite result does not establish canonical compiler IDs, generic monomorphization, Core whole-state refinement, production/native/crypto correctness, stagenet settlement or EARS closure. MapQueue's full typed return was checked inside the model test; no standalone MapQueue ITF was exported.


## October 21: runtime-table nominal identity counterpart

The additive `q11NominalIdentityTest.qnt` entry uses this same unchanged `nominalPublicSimulation.qnt` engine and adds five declaration-only carrier modules plus five identity/input/expected/test modules. Eleven modules form its complete closure; this entry does not replace or rerun the source10 entry above. The supplied compiler ProgramView23 uses actual runtime row IDs, retains every ordered table, type argument, owning type, source_order, field name/ordinal and raw option/slice.

One new frontend check and the five exact identity tests passed, with one sample per test and all 4,686 source/tool/symlink checks unchanged before and after each. The tests compare full binding records, both finite logical typed returns, a same-type field-order discriminator, full ordered input retention and duplicate constant-reference reads. The independent manual binding/tree data were frozen before source11, then explicitly exposed to its author; this is not another blind reviewer vote. See [the dated identity source/result evidence](../../../../docs/results/sherlock-quint-public-nominal-identity-20261020/README.md).

From this directory, `quint typecheck q11NominalIdentityTest.qnt` and `quint test q11NominalIdentityTest.qnt --main q11NominalIdentityTest --match '^(q11CompleteInputBindingsTest|q11BothSourceReturnsUseRuntimeIndicesTest|q11SameTypedFieldOrderDistinguishesTest|q11AllOrderedInputTablesAndOptionsRetainedTest|q11ConstantReferenceReaderPreservesSourceValuesTest)$' --backend typescript --max-samples 1 --seed 1` describe the entry points; this documentation dispatches nothing.

The full HostImage23 observer and Const/Data structural reader remain unbound to any actual final Host image/rootref. Partial positive index/slice/acyclicity/budget/UTF8 premises, UInt<=255/frame32/helper0/model-clock limits remain explicit. These passes do not establish Core owner clocks, admission, generic refinement, native/proof correctness or EARS closure.
