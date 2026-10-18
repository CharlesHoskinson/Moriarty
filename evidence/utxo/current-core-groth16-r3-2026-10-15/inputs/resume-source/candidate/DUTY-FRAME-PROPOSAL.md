# Proposal (not implemented): duty-frame option and balance-row presence

**Status: PROPOSAL ONLY.** Nothing here is implemented in this successor's source. It does not widen the first-slice relation, the E0–E5 scope, the expected tests or any claim. Adopting it needs a design vote under the two-vote rule, an implementation, frozen independent expectations and a fresh Opus review. F-02 (external duties), F-03 (conservative bound), F-04 (balance-row presence) and F-05 (unregistered-duty frame, extra rows) **stay open** until then.

Author: `claude-opus-5-5`, review-repair source author. These are design suggestions from reading the current source, not reviewed decisions.

## 1. Facts from current source

- Core admits scenario liabilities whose names have no source `Liability`, with any debtor and creditor over a declared asset (non-empty, at most 1,024 bytes, unique names). It compares identity against the source only for names in the program's registry (`runtime.rs:629-661`, `liability_identities`). Such rows are untouched by the action: Core never changes them.
- The compiler represents exactly the source registry (`plan.rs:322-346`). A duty touched or read without a source introduction is refused as an "external duty" (F-02). An untouched unregistered row has no slot, and `witness::frame_unchanged` only checks host-side that it is unchanged. That check is not in the relation.
- Balance cells have no presence bit. Core requires an explicit row for each Transfer/Fee account (`runtime.rs:609-627`), so the relation admits a claimed pre-balance of 0 for a missing row (F-04, M-14).
- No statement slot commits to "there are no other rows". Frame completeness is a target obligation (`adapter::TARGET_OBLIGATIONS`).

## 2. Proposed duty-frame option (F-05, untouched unregistered duties)

- `CompileOptions` gains `duty_frame: DutyFrame`, with no default, like `CapBinding`:
  - `DutyFrame::RegistryOnly`, which is today's behaviour;
  - `DutyFrame::Include(Vec<DutyRef>)`, where `DutyRef { name, asset_name, debtor, creditor }`.
- **Compile-time refusals:**
  - a `DutyRef` name equal to a registry name (either an identity conflict or redundant);
  - duplicate names;
  - an unknown asset;
  - empty or over-long accounts (the same bounds as frame cells).
- **Plan:** included duties become `DutyKey { touched: false, frame: true }` rows after the registry rows, sorted by name. They count in the trace bound and the 1,024-liability bound.
- **Layout:** each included duty gets pre and post `(present: Bool, amount: UInt)` slots like a registry duty. Its identity enters the shape digest as a constant.
- **Constraints:** entry canonicality `(1 − present)·amount = 0`, then `post.present = pre.present` and `post.amount = pre.amount` by wire identity. This is the same mechanism as an untouched registry duty. No expression or statement can reach these rows. A touch still refuses as an external duty, so present-zero versus absent is preserved exactly.
- **Witness mapping:** `body()` maps included rows by name and identity. The host-side `frame_unchanged` check then covers only rows that are neither registry nor included.
- **Not covered:** external duties that an action touches (F-02) still need a dynamic identity cell, with prover-supplied identity bound in-circuit. That is a different and harder design.

## 3. Proposed balance presence bit (F-04)

- Each represented cell gets a `PreCellPresent` Bool slot.
- **Touched cells:** the constraint `present = 1`, because Core refuses the scenario otherwise. This makes the missing-row claim unsatisfiable *as a statement*. Authenticating that the claimed row really exists in the consumed state is still a target obligation.
- **Frame cells (`FrameCells::Include`):** may be absent, canonical `(false, 0)`, with post equal to pre.
- This changes every layout, and hence every shape digest. It needs a new layout version and a vote. M-14 would then move from expected SATISFIABLE to expected UNSATISFIABLE. That would be a new frozen expectation, authored independently, never by editing the frozen reviewer files.

## 4. Remaining extra-row gap (F-05 "extra rows", F-03)

Rows outside both represented sets still cannot be excluded by a transition relation over a partial snapshot. A complete answer needs either a statement-level commitment to the complete canonical snapshot or a target-authenticated row set. That belongs with the reviewed ABI and P3 design (V6 P3-3 and P3-10 domain/frame binding). It is not proposed for the compiler alone. Until it exists, the conservative trace bound (F-03) stays and no frame-completeness claim is made.
