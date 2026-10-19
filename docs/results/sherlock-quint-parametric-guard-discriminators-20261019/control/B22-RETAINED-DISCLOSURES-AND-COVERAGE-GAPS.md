Exact nonblocking disclosures/coverage gaps from the full prior source-operation opinion; retained for every result publication. No new source change is proposed.

### Disclosure conditions: the guard is stronger than the emitters

The README's "checked physical shapes and selected scalar predicates" wording does not enumerate these. Per critical check 1, each must be recorded as a model strengthening or domain restriction, not as emitter equivalence. This can go in Root's review record; it needs no edit to the frozen `.qnt` files.

- **D1. Padding zero for every Program and State table.** This comes from `literal_host::representation`/`complete_representation`, which are host-side and off-chain. The A102 emitter checks only Types/Ctors/CFT/Constants padding (via uiValid). The A80 emitter checks no padding.
- **D2. `entryExprId < activeExpressions` in `pProgramHeaderAllowed`.** Neither the schema nor either Step emitter has this. It also imposes `activeExpressions ≥ 1` on standalone A80, which the A80 emitter does not require.
- **D3. `depth ≤ stackLimit` in `pStateHeaderAllowed`.** Neither the schema nor either emitter has this.
- **D4. `selectedId < O` capacity.** The real `id("occurrenceId", o)` lane enforces only `ceil(log2 O)` bits. With activeExpressions > O, the real system admits rows the model refuses.
- **D5. `activeConstantFields == 0` unconditionally.** Real uiValid requires this only when KF = 0. When KF > 0 it admits active ConstantFields rows even without constructors. This is a scalar-domain restriction.
- **D6. Ranges and binding.**
  - Range checks are root-only: cells must be `< 2¹²⁸`, and table cells are unranged. The real per-lane bit widths and maximums (outcomeTag ≤ 3, booleans, id widths, rule_tag ≤ 38) are external.
  - `PChild` carries no capacity record. Child capacity binding is only implicit, through the shared `layoutArg`.

### Coverage gaps (non-blocking; finite samples, record as retained)

1. **Scalar guard rejection is never exercised.** No negative covers a constructor Constant, Bool with nonzero bytes, boolValue 2, a wrong fixed table, or activeCF ≠ 0. A mutation of `pScalarConstant` to `true` survives all eight runs. Bool(false) is never sampled.
2. **The padding predicates never reject.** All three padding mutants reject through full-image equality (including `pMutantProgramTypePadding`, which rejects via `leftChild.program == boundProgram`). Deleting `pPaddingZero` from the header predicates survives all eight runs. This still satisfies check 3 as stated (shape-valid post-padding mutants reject), but the precondition padding guard is not discriminated.
3. **Envelope-mismatch rejection is untested.** Examples are `(3,0)` and `(0,1)` in the relations. Literal envelopes are asserted only positively.
4. **Many A102/A80 precondition negatives are absent.** These include non-Pending outcome, controlLength ≠ 1, non-Eval tag, a stray control cell, depth > 0, nonempty frames, space = 2, and negative indices.
5. **Several start, layout and join negatives are absent.**
   - `pCompleteStartAllowed`: only the selector/arena case is tested. There are no individual entry ≠ 0, remaining ≠ initial, or nonempty-arena-at-selector-0 cases.
   - `pLayoutAllowed` and `pTablesShape` are never tested for rejection.
   - `pPairJoin` with `left.after ≠ right.before` is not tested directly.
6. **Expectations share structure with the transitions.** `pHandDone` uses the same slice/concat formula as `pPublicationNext`. The independent content is the hand fuel bytes, the constant indices, the fixed arenas and the absolute offsets. This is disclosed as same-author consistency, not a blind oracle.

