# DESIGN009: The Full-Product Multiplication Board and Typed Completion Dock

- **Kind:** Creative concept companion only. It adds no circuit iteration, website code, or deployment.
- **Authorship:** Muse HIGH, designated concept author for this entry. This is not an Opus contribution.
- **Status:** The finished site launches only after the whole Sherlock goal is complete and accepted. This note is not publication.

## Continuity from DESIGN008

DESIGN008 ended on the Sub5 composer spine: sixteen byte lanes, a borrow ladder, the exit/failure partition, the typed handoff, and a source-audited parent backplane. DESIGN009 keeps that entire board intact and adds a neighbouring bay to the east, on the same 4px spine, in the same copper and ink.

The new bay is wider than anything before it. Subtraction borrowed one byte at a time; multiplication spreads across the whole width of both operands at once. The visual language does not change — flat unlit Moriarty wordmark, tabular labels that never reflow, Tron-like luminous routes on dark layered board — but the geometry does. Where Sub had a ladder, Mul has a field.

Add, Sub and Mul remain distinct types throughout. Their bays may rhyme in silhouette, but keyed sockets, lane labels and source clickthrough keep every carrier named for its own operation.

## What the source actually contains

`emit_b_scalar_arithmetic_mul` builds a full 128×128→256 product. Each operand is read as eight 16-bit limbs. All 64 limb products are formed, then gathered into bounded column sums with carry, using the captured DivModPowerOfTwo order — quotient, then modulus — to yield sixteen product limbs. The result is kept as two distinct rails: `product_low` and `product_high`.

That split is the board's central fact. A nonzero upper half is overflow. For a UInt with no overflow, the low value is staged. For a UInt with overflow, error 10 is staged. For a valid non-UInt, error 11 is staged. Nothing is truncated silently; the high rail is displayed precisely because it decides.

Downstream sits the typed completion dock. `emit_b_scalar_exit_mul_u128` appends an applied exit at the physical active Data index, while logical allocation and bounds remain a separate accounting. When the logical cap refuses, the board retains `Exhausted(DataNodes)` and appends nothing. `emit_b_scalar_failure_mul` transfers only codes 10 and 11. Typed helper and counter conversions, `BMulPublishState::to_admin_publish_state`, `to_legacy_handoff`, and the checked legacy inverse move every lane without relabelling Add, Sub or Mul, and without casting one operation's state into another's.

The dock's downstream connector is drawn explicitly open. Parent registration is not yet present in source, and genuine Mul `RightF.op=2` handling and its P1 chain are not connected. An open connector is truthful. A closed Mul publication trace would not be.

## Visual alternatives

- **A. Limb-product field.** An 8×8 grid of limb-product cells sits between two operand rails at the top and left. Column sums run down the field's diagonal, carry passes cell to cell, and sixteen product limbs leave along the bottom into the separated low and high rails. The completion dock mounts beneath as a second, quieter deck.
- **B. Column-carry river.** The 64 products collapse into a single flowing river of column sums, carry rendered as a raised channel that narrows as it resolves. More elegant, but individual limb products — the most inspectable unit — dissolve into the flow.
- **C. Dock-first board.** The typed completion dock dominates and the multiplication field shrinks to a feeder tile. This foregrounds the newest completion source, but understates the full-product relation, which is DESIGN009's reason to exist.

**Recommendation: A, with B's carry channel drawn inside it.** The field preserves every limb product as a selectable, source-linked object. The river's carry channel, laid along the diagonal, still shows accumulation at a glance. C's dock remains present at full width below, subordinate but unmissable.

## Cumulative assembly, interaction and source clickthrough

The assembled board reads top to bottom. Two operand rails, each segmented into eight labelled 16-bit limbs, feed the product field. Selecting any cell highlights its two parent limbs and opens clickthrough to `emit_b_scalar_arithmetic_mul`. Selecting a column highlights its bounded sum and carry step. The sixteen product limbs exit into two rails: `product_low` leading toward the dock's value seat, `product_high` leading toward a single labelled gate — nonzero means overflow, error 10.

From the arithmetic bay, three staged outcomes cross into the dock on keyed lanes: value, 10, 11. In the dock, applied exit and cap refusal are drawn as physically different things — an append mark at the active Data index for the first, a held pad retaining `Exhausted(DataNodes)` for the second, with the logical allocation counter shown beside, never merged into, the physical index. Failure transfer accepts only its two codes; other codes meet a blank, unlit socket.

Typed adapters in the dock are drawn as straight, via-free pass-throughs: lanes enter and leave with identical labels, because the source moves them without relabelling. The final adapter reaches the downstream connector, which terminates in an open, capped socket labelled as awaiting its parent connection.

Interaction is selection, not simulation. Every glyph — limb, cell, column, rail, pad, adapter, socket — selects and opens its source: file, symbol and lane role. There is no browser evaluator, no proving animation, and no animated execution. Any glow is decorative selection only, never measured activity.

## Visitor copy (finished release)

> Multiplication takes the whole board. Two numbers arrive as eight pieces each, and every piece meets every other — sixty-four small products spread across the field like a city seen from above. Follow one column down and watch its sum settle, its carry handed to the next column, until sixteen limbs stand in a row. Then the board asks its one honest question: did anything spill into the upper half? If not, the value below is yours, exact and complete. If so, the board says so plainly, with its error intact. Cross into the dock and the result is received without being renamed, seated, and carried forward exactly as it arrived. Touch any cell, any lane, any connector — the source that built it opens beside you.

## Accessibility, DOM and reduced motion

- **DOM equivalent.** A structured table mirrors the board: one row per operand limb, one per product column, labelled rows for `product_low`, `product_high`, value, 10, 11 and `Exhausted(DataNodes)`, and one row per dock adapter. Each row carries the same label and the same source clickthrough as its glyph.
- **Keyboard order.** Focus follows assembly order: operand A limbs, operand B limbs, product columns, low rail, high rail, staged outcomes, dock adapters, open connector.
- **Not colour alone.** Overflow, refusal and failure are carried by labels, notch glyphs and rail position as well as tone.
- **Reduced motion.** Nothing flows or pulses. Selection is a static highlight, layer changes cut, and the carry channel is drawn as a fixed stepped profile.

## INTERNAL — Designer/provenance gap register

*This register is not visitor copy.*

- **Source status.** The corrected Mul arithmetic and completion bodies received a source/static API opinion of plausibility for the public wrapper returning `EmittedBScalarArithmeticMul` and both full bodies. That is not Rust compilation, SDK execution, typechecking, evaluation, or chip proof.
- **Open connections.** Parent module registration is missing. Genuine Mul `RightF.op=2` and the Mul P1 chain are not connected. The open downstream connector in this design records exactly that; no closed Mul return-right or publication trace may be drawn.
- **Unrun work.** Completion test authorship was selected and in progress at this cut; no Mul completion test source or pass is assumed. New arithmetic tests and all SDK and proof stages are unrun.
- **Parallel evidence, correctly scoped.** K evidence passed six operations over a bounded set of K states, partial boundary pairs and own-accounting checks, with the expected record opened only by the independent child — it is not whole-state or Mul circuit evidence. The Clean B17 result is a sufficient missing-function kernel pass, not a general compiler or refinement theorem. The P10 SDK-free checks produced ten method passes and one method with seven PermissionErrors on read-only fixtures, with no native attempt. None of these light the Mul board as executed.
- **Source authorship.** The Mul source author was configured Muse, provider meta, producer `muse-wonder-1.0-contributor` — not Opus. Authors are not their own independent reviewers, and source authorship is not an audit vote.
- **This note's authorship.** Requested and written as the Muse HIGH concept contribution above. Any Root-selected fallback for any part of this entry must retain its actual identity in the provenance record and must never be relabelled as Opus authorship.