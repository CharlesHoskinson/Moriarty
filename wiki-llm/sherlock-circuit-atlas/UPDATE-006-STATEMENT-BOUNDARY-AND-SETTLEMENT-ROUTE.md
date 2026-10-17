# Iteration 006: Statement boundary and settlement route

This is an unreviewed source and evidence design update to the circuit atlas log. It adds no circuit board. No new handwritten bridge, wrapper or circuit source had returned at this source freeze. Iteration 005 remains the latest circuit source iteration, and it, both design companions and every earlier note stay unchanged. Exact file pins are in the adjacent INPUT-MEMBERSHIP and PUBLIC-SOURCE-PINS packet. The materializer records author identity, terminal and pins outside this note.

Passages marked *Visitor copy* use the single internal premise from Design 006: Sherlock v1 is finished and accepted, and every rendered element resolves to a binding. Everything else in this note is author-facing.

## The source this update reads

`crates/mori-backend/src/sherlock_machine/literal_step.rs` defines `literal_schema`. From `LiteralCapacities`, it builds named logical columns in two blocks:

- a program header followed by fifteen program tables;
- a state Root followed by six state tables.

Zero-capacity tables are still represented.

`emit_literal_step` takes these inputs:

- one shared `program` image;
- a full `pre` image;
- an independently supplied `post` image;
- a caller `enable`;
- `LiteralEnvelope { rule_tag, charge }`.

It range-checks every cell. It gates `rule_tag` to wire tag 3 and `charge` to 1. The host record line 102 is a separate thing and is not an operand.

It then checks the pre state: Pending with zero outcome payload, `controlLength` 1, and CR0 Eval with every other control cell zero. It fetches the Expressions row at CR0 `exprId` through the real `emit_enabled_fetch_block` in `enabled_fetch.rs`. It checks the Literal shape and `occurrenceId`, and requires `valueRef.index` to be below `activeConstants`.

Under `enable`, the post state is constrained as follows:

- CR0 becomes Return and carries the selected `valueRef`, and later control rows are zero.
- Every other Root cell, and the Data, DataFields, EnvCells, ValueSlots and FrameSlots tables, equal pre.
- `remainingFuel` decrements across sixteen little-endian bytes, with a nonzero-pre fold and a final borrow of 0.
- `initialFuel` is preserved byte by byte.

The fragment aliases the selected constant reference. It never fetches or materializes the Constants node, and it never publishes Done. It emits no PublicInput, Output, witness packer or public statement. It returns a `LiteralStepBlock` holding the instructions, the selected expression operands and the operand aliases.

The file header still reads "Unregistered". `models/sherlock/literal-step/MAPPING.md` treats that comment as retained provenance. The current parent registration has no consumer, ABI or execution wrapper.

## Visitor copy

> *Visitor copy:* The literal step is the shortest move Sherlock makes. Before it, the machine is evaluating an expression that names a constant. After it, the machine is returning a reference to that constant, and one unit of fuel has been spent.
>
> This board holds everything the step reads and writes. The program runs along the left edge as one image that both sides share. The machine before the step runs along the lower edge, and the machine after it along the upper edge. Most cells cross straight from below to above unchanged. Copper marks the cells that change: the control row, its constant reference and the fuel bytes, borrowed one byte at a time.
>
> On the right edge, the board's public statement leaves the circuit. The route then continues through the native proof, an independent verifier, submission to Midnight Stagenet and the ledger readback. Each stage has its own card holding its own record. Open any card to see what that stage established and the source that produced it.

## Recommended composition: edge connector and route rail

The A102 board keeps its bench position from iteration 004, and its mesh stays as it is. This update adds layout around the board, not inside it.

- **Program bank (west).** The header strip is followed by the fifteen table banks in L4 order. Every row is drawn, including inactive rows and zero-capacity tables. A single bank feeds hairline taps to both state sides, which shows that one image serves pre and post.
- **Pre bank (south) and post bank (north).** Each holds the Root strip and six table banks. Preservation straps run straight north. The sixteen `remainingFuel` stages sit inside each Root strip, with byte 0 at the left and the borrow chain drawn between them.
- **Envelope pads (southwest).** These are two pads beside the enable via, silkscreened `rule_tag` and `charge`, with etched constant targets. They are captioned as logical source fields.
- **Statement slot (east).** This is an edge-connector outline with no pads, fingers or traces into the banks. Internally it is labelled *proposal: populated only from the returned bridge's own schema*.
- **Route rail (off-board, east).** Separate plates sit in route order: statement and artifact, native build and local proof, independent verification, stagenet submission, and settlement readback. Each plate is a separate DOM card. Physical gaps between the plates keep any stage from implying the next.

Program, pre and post banks are complete logical operand banks taken from `LiteralSchema` and `LiteralImage`. They are not selected public inputs or a native ABI. The statement slot never borrows its shape from them.

**Alternative: through-stack.** The route plates stack as layers beneath the bench and are read downward. This is more compact, but stacked layers suggest a shared substrate and continuous vias. The edge rail is the recommendation.

## Interaction and source clickthrough

Selecting a pad opens a drawer with the following:

- the source path and symbol;
- the exact column name that `literal_schema` emits, such as `remainingFuel.7` or `pendingFrame.savedEnv.index`;
- the lane width, Boolean flag and maximum;
- the operand class: shared program, pre operand or post witness;
- the published pin.

New shortcuts are scoped to board focus, and existing iteration 005 keys keep their meanings:

- `F` follows the fuel borrow chain.
- `E` isolates the envelope.
- `G` moves focus along the statement slot and route plates.

Selecting a route plate opens its card. The card lists that stage's recorded identity fields in their full original text and clicks through to the source that produced them. Without WebGL, every bank renders as a DOM table with all columns and rows, and the route renders as an ordered list of cards. Under reduced motion, transitions become cuts. Every state has a non-colour twin in dash, glyph or word.

## Light and materials

The instruction layer is static. Traces are ink on warm ink grounds, and the 4px spine and tabular mono are unchanged. Copper is proposed only at the post write terminals: the CR0 tag, the CR0 `valueRef` and the fuel bytes. Route plates are unlit. Nothing animates proof generation, verification or ledger activity. A trace activity layer requires an accepted source, statement and capture binding.

Emission and cyan remain internal proposals pending design and contrast review. The wordmark stays flat and unlit, in the sans at weight 600 with tight tracking.

## Launch bindings

Launch bindings L1–L7 carry forward unchanged: stillness from actual dependencies, history versus final membership, host versus circuit bindings, lossless u128 text, passive text-node rendering, plates from the `frames` array diff, and no superlatives. This update adds the following bindings:

- the statement slot is populated only from the returned bridge source and its own schema;
- each route plate shows only actual recorded identities for its stage;
- the target plate binds the release's actual target, toolchain and transaction identities, and never the public descriptor;
- the banks show the accepted netlist's capacities, and exports are exact bytes.

## Grounding (author-facing)

| Subject | Recorded fact |
|---|---|
| Source anatomy | `literal_schema` and `emit_literal_step` with the real enabled fetch, as read above. Compile, SDK, test, proof and native checks are UNRUN. No instruction or gate count, k, fee or speed is claimed. |
| Route | Per the `CRITICAL-PATH.md` source memo, which I did not verify body-by-body against every inventoried file: the first break is the statement and artifact bridge, before wallet or endpoint configuration. The memo's proposed literal fixture would test one rule only, not Done, recursion, financial behaviour or release. |
| Separate paths | Existing Core `ProofBundle`, the financial LedgerAbi consumer and the Compact Preview scripts are not Sherlock evidence. |
| Bridge | It is being authored separately and had not returned at this freeze. No names, ABI, pin order, visibility, codec, netlist, keys, proof or transaction exist here. |
| Target | `STAGENET-PUBLIC-TARGET.json` is a timestamped read-only observation of Midnight Stagenet, ledger 9. It establishes no interoperability, and its Sherlock, compiler, wallet, contract and transport bindings are null. Its sampled finalized header is not settlement readback. |

## Next inputs (internal)

The statement slot fills when the returned bridge source arrives with its statement and artifact schema. The proof and verification plates fill from recorded native build, local proof and independent verifier identities and results. The submission and readback plates fill from an actual stagenet transaction record and a full state and effect decoding. The activity layer waits for an accepted capture correspondence. Until then, each port stays an unpopulated outline.