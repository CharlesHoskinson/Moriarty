# Post-freeze B20 ITF readback (Root only; no execution in this proposal)

Use the single **actual frozen** ITF produced by the reviewed capture. First persist/fsync complete raw ITF, stdout/stderr, exact command, actual receipt and hashes. Confirm the post-pin checks and byte caps before opening it for comparison. Preserve raw bytes; every following value is derived after-freeze observation, not a replacement for the source ITF. Do not run Quint again, import the evaluator or fill actual fields from expected source.

Read JSON losslessly. The installed pinned `dist/src/itf.js::toItf` encodes Quint integers as `{"#bigint":"decimal"}`, Lists as arrays and records as objects. Decode only these actual concrete forms to exact integers/arrays/records. Preserve metadata; reject missing/wrongly typed fields and unexpected value encodings rather than coerce/pad/default them. Top-level trace metadata may exist, but vars must be exactly `["execution"]`; exactly3 states with metadata indices0,1,2 are required.

For each raw state retain complete `execution` with exact fields program/context/states/envelopes. Require cumulative nested image lengths1,2,3 and envelope lists[],[(3,1)],[(3,1),(0,0)]. Context must be actual[17,29], identical in all3rawstates. Every complete Program is structurally identical. Rawstate0 nestedimage0 equals rawstate1 image0 and rawstate2 image0. Rawstate1 image1 equals rawstate2 image1. No terminalpadding/fourthstate/filtering is permitted. Freeze decoded-source binding/hash and all actual comparison results.

Re-evaluate the **actual source predicate** compositionInv from these frozen observations, separate from simulator `[ok]`/witness stdout:

1. Check the fixed-fixture Program root16/table15 shape: rowcounts[0,0,1,0,1,0,0,0,0,0,4,4,5,1,0], widths[2,1,4,1,11,1,6,2,3,2,3,4,1,22,2]. Every actual state image has root45 and6tables, each2rows with widths[22,2,5,2,17,36]. All actual remainingFuel bytes are integers0..255.
2. Check initPremise literally: actual Program.header[0]=0,[5]=1,[14]=1; Expression0 exactly11zeros. Origin root0..4,6,7,41..44 zero; controlLength[5]=1; stackLimit[8]=2; remainingFuel[9:25] equals initialFuel[25:41], positive exactLE16; all6origin tables allzero. Context length2. These are fixed-fixture observations, not general admission/uiValid/bitwidth/maxima validation.
3. At terminal rawstate2, terminalWitness is nestedstates.length==3. Take origin/intermediate/done from its actual three nested images; exact literalEnvelope(3,1),returnEnvelope(0,0). Re-evaluate relation with exact integer math and rawactualcells: Program.Expressions selected by origin.ControlRows[0].exprId; originroot unchanged in middle except remainingFuel9..24; middlefuel = originfuel-1; first5arenas identical; middleCR0 tag1,valueref lanes2/3 from actual selectedLiteral,allotherlaneszero; every remaining controlrowzero. Done root equalsmiddle exceptcontrolLength5=0,outcome41=1,42/43 frommiddleCR0.2/3,error44=0; allfirst5arenas unchanged andallcontrols zero. Sumactual envelopecharges1; originfuel-donefuel==1. All listed rows/lanes must already be present, never synthesized.
4. Re-evaluate exactStatementLength from **actual** Program and all3actual images; preserve the full complete inputs and integer range results. This source check alone shares the model flattening convention and is not the N2 absolute-layout check below.

Reconstruct a separate731-lane **derived logical statement vector from actual ITF fields** using the pinned Rust schema order, not a call to the Quint model/helper. Write the complete vector as a derived artifact with raw ITF/hash binding. ITF stores execution images, not an emitted `publicStatement` variable: label this reconstruction accurately; the two test passes separately cover the model publicStatement assertions.

Handwritten half-open logical spans (all0-based):

- context[0,2); Program[2,88): root[2,18), Occurrences0[18,22), Expressions0[22,33), Types0..3[33,45) in3-lane rows, Constructors0..3[45,61) in4-lane rows, ConstructorFieldTypes0..4[61,66) in1-lane rows, Constants0[66,88). All other Program tables have0rows. TotalProgram86.
- Init[88,301); Intermediate[301,514); Done[514,727). Eachactual State image is213lanes. Relative to each block start: root[0,45), Data2x22[45,89), DataFields2x2[89,93), EnvCells2x5[93,103), ValueSlots2x2[103,107), FrameSlots2x17[107,141), ControlRows2x36[141,213). Compare every fullrow to its actual rawimage source, with no row omission/padding.
- Envelopes[727,731) exactly[3,1,0,0]. Total731.

For the sole captured UInt7 case additionally report actualcells: vector66=0,69=7; initremaining97=1,98=0; middleremaining310=0,311=0; doneremaining523=0,524=0; middlecontrolLength306=1,CR0tag442=1; donecontrolLength519=0,outcomeTag555=1. Check allremainingfuelbytes and initialfuel preservation, notjustthese spotcells.

Persist `ITF-POSTFREEZE-READBACK.json` with all individual results, actual counts/shapes/charges/fuels/continuity/731spans/spotvalues, complete derived vector artifact pin, and any first divergence showing both raw locations. No source-derived expected value may populate a missing actual field. A failure remains failure; no repair/replay.

Scope remains same-author internal finite model evidence. Selected-reference0/exprId0 and zero-arena sensitivity gaps remain, as do imported-file syntaxN1, missingBoolrawcapture, uiValid/admission, other rules, independent production correspondence/refinement/circuit/native/crypto/ledger/wholeEARS. No acceptance follows from hashes or a reconstructed vector.
