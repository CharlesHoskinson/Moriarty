# PROPOSED P3 Source Revision r2: Physical Arena, Slice, Environment and Auxiliary-Trace Design for CandidateA

**Status.** This is a source-design candidate written by the designated owner (Opus 5.5) acting as **author only**. It is unadopted. It is not a vote, an adoption, a roadmap milestone, a circuit edit, an emitter acceptance criterion or an execution record.

- **Supersession.** It replaces r1 (`e418610606c793c23ccbc6877d1981af9b2076a64f2ae718eae3179135543bf1`) as a candidate.
- **Prior reports stay immutable.** These remain unchanged:
  - r1 itself;
  - the owner challenge (`19d238028b27686c02890478d82c7246d72e6257c3390267de09a6b511986a3e`);
  - the Muse normal review (`d566c8ed266462c6bfe87880eeca522eb323f42c923e95abac67e52e36efd911`).
- **No votes carry over.** These bytes need a fresh owner audit, a fresh Muse audit and a Root adoption.
- **Inputs.** I used only the supplied texts. I recomputed no hash, and every SHA256 here is as supplied. I ran no tool, parser, model, evaluator, test or native action.
- **Limits that still hold:**
  - Native authority is 0.
  - The old 90s/30s caps, counters and failures are unchanged.
  - C4 holds: no dependent circuit edit happens before admitted A102 compilation, and a compile failure means registration revert.
  - No numeric value, grant, provisioning or admission is made.
- **No security decision is assigned to GPT.**

---

## 0. Fixed inputs and scope

This revision leaves the following unchanged.

**Full39 P1 semantic map.** These bytes are not altered:
- proposal `6823badbbca40cb945881ae689cd812f4ddca3aceee9b7ec8dbfde5cf9e122c2`;
- manifest `a742817c5f64c46eda6c5988d2a3c9551f50139ceb1632a70dc8860c70ff159b`;
- Root adoption `e09dedcd…0c05e`, together with its conditions 1–10 and the F1–F7 readings.

**The seven frozen L4 files:**
- crosswalk `22c2ffcb…81f0`;
- design `3738ca71…0e0e`;
- interfaces `86c228a4…6a`;
- extensions `110772…451f`;
- proposal `c6b2e23a…94c`;
- tasks `7aea6c38…b4dd`;
- provenance `25f0c5ac…a0`.

**R5 rule facts:** JSON `122289ad…b0a5` and Markdown `48580e13…f8bd`.

**Rust and K sources:**
- `reference.rs` `e1bb1098…181b`;
- `reference_types.rs` `4b4bb364…317f`;
- `main.k` `b3746ff5…1d41`.

**L4 structure.** All of the following stay as they are:
- the tables, columns, widths, tags and block order;
- the StateBlock and ProgramBlock columns, including unused syntax;
- `initialFuel` and `stackLimit`;
- typed `ValueRef{space,index}`;
- immutable occurrence links;
- saved roots;
- all L4 semantics.

**Still unselected:** F4 boundary canonical projection and MD2.

**Scope of CandidateA.** It covers only UInt, Bool, List and Tree. B, general ADTs and financial behaviour are out of scope.

**Where new material goes.** Everything new lives in a **separate proposed extension profile**, `sherlock-k-a-aux/1` (§3). That profile adds an AuxBlock operand. It edits none of the seven L4 files, and I do not claim that any existing L4 codec implements it.

---

## 1. Arena policy (SELECTED)

The selected policy is **Policy A: monotone append, with preflight before commit**, as in r1.

- **Appends.** StateBlock arenas (Data with DataFields, EnvCells, ValueSlots) get append-only rows at the active end, and only for the storage of the selected success branch.
- **Failure branches append nothing.** This covers A204 None, A153 Err10/Err11, A157 Err11 and A214 miss.
- **Helper work never enters StateBlock.** It lives in AuxBlock (§3).

**Rejected alternatives:**
- Policy B, speculative append, leaks occupancy whose size depends on the failure position.
- Policy C, in-segment compaction, needs an in-step rebuild, which L4 forbids.

---

## 2. Common machinery

### 2.1 Notation

**Pre-state lengths.** `D0, DF0, E0, V0` are the pre-state active lengths. `K0` and `KF0` are the active Constants and ConstantFields lengths.

**Abbreviations.**
- `A:` means an alias: the reference is copied and not dereferenced.
- `n` is the pre-state `controlLength`.
- `X` is `CR0.exprId`.

**Node read.** `node(r)` is `Data[r.index]` if `r.space=Data`, otherwise `Constants[r.index]`. It is guarded by `r.index < (space=Data ? D0 : K0)`.

**Field read.** `fld(r,i)` reads field `i` of the node `r` points to. It is guarded by:
- `i < node(r).fieldLength`, and
- `node(r).fieldStart + i < (space=Data ? DF0 : KF0)`.

**Sort.** `sort(r) := node(r).typeId`, under INV-TY (§2.4).

**Name equality** is `nameId` equality, under D-NAMES (§2.5).

**Charged guard.** `cg := Pending ∧ n>0 ∧ ¬(CR0.tag=Return ∧ depth=0) ∧ CR0.tag≠Error ∧ remainingFuel≠0`.

**Width aliases** (from crosswalk `width_rules`):
- `VR := Bool + id(max(K,D))`;
- `ER := Bool + id(E)`;
- `NODE := enum(3) + id(T) + id(Q) + u8[16] + Bool + start(max(DF,KF)) + count(max(DF,KF))`.

### 2.2 Append primitives

These are the only StateBlock arena writes.

- **`appendData(kind, typeId, ctorId, uintBytes:u8[16], boolValue, [f0..f_{c-1}])`** writes:
  - `Data[D0]` with `fieldStart = c>0 ? DF0 : 0` and `fieldLength = c`;
  - in the same step (L4 G1), `DataFields[DF0+i] = f_i`.

  It returns `{Data, D0}`. No rule appends more than one Data node.
- **`appendEnv(nameId, vref, parent)`** writes `EnvCells[E0+j]` for the j-th cell of the step. It returns `{1, E0+j}`. The parent is always an existing root or an earlier cell of the same step.
- **`appendSlot(vref)`** writes `ValueSlots[V0+j]` in increasing order.

**One family per rule.** No rule appends to more than one of the families {Data+DataFields}, {EnvCells}, {ValueSlots}.

### 2.3 Step phases and refusal order (P3-4 repair)

1. **Dispatch reads.** The charged guard and all guarded pre-state and ProgramBlock reads happen here. The enabled AuxBlock record is fully constrained (§3).
2. **Branch selection.** The branch is a pure function of ProgramBlock, pre and the constrained aux outputs. This includes semantic checks such as A135/A140 `depth < stackLimit`. No free selector exists.
3. **Capacity preflight** for the selected branch, covering every family it needs, in this fixed order:
   1. `EqScratch.rows`;
   2. Data;
   3. DataFields;
   4. EnvCells;
   5. ValueSlots;
   6. ControlRows (`OBL.P6.C>=2` for A114, A117, A120, A123, A126, A131, A208, A220);
   7. FrameSlots (`OBL.P6.S`, `depth+1 ≤ S` for A135).

   Within a family, the locator is the first overflowing index (F5). A zero-arena-append rule can still be refused by ControlRows or FrameSlots; that is P6.C/P6.S.
4. **Commit.** Appends happen, then the masks of §4 and Root writes (fuel decrement on charged rules only), with preservation of every other cell.

**Representation failures come first.** These are failed index guards (G4 dependency) and the enforced checks E2/E3 of §2.4. They make the step have no accepting assignment, classified as a WrongRepresentation dependency. They are never Capacity and never a semantic outcome.

**What Capacity means.** A Capacity refusal produces no accepting post-image. Host tooling reports `Capacity{table,needed,limit,locator}`. It never becomes error 3–11, false, Exhausted or Rejected. Semantic failure branches need zero rows in every family, so they are accepted even at full occupancy.

**Why the EqScratch check is first.** An Eq with no terminal row has no defined branch at all.

### 2.4 Invariants: preconditions versus enforced checks (P3-6 repair)

**Obligations (proposed):**
- **INV-IMM.** `post.X[i] = pre.X[i]` for `i < X0`.
- **INV-PAD.** Rows at or above the active length are zero.
- **INV-SLOT.** No ValueSlots write below V0.
- **INV-NOFAIL.** A semantic failure branch appends nothing to StateBlock.
- **INV-CAP.** A refusal has no post-image.
- **INV-AUX.** AuxBlock never reaches StateBlock, and a disabled aux record is all-zero.
- **INV-ORD for appended rows.** This holds by construction: appended fields and parents reference rows that were active in pre, or cells appended earlier in the same step.

**INV-TY is a precondition on every reached read, in both spaces:**
- **INV-TY-D** (Data nodes): a StepInv/chaining dependency, still unselected (L04).
- **INV-TY-K** (Constants): a dependency on ProgramBlock canonicality, constTyping and program binding (F4/L04, unselected).

P3 claims nothing about images outside the representation domain.

**Checks this design enforces:**
- **E1.** An index guard on every node, field, cell, slot, parameter, function, arm and pattern read. The guard primitive itself is a G4 dependency.
- **E2.** In EqScratch: `hd ⇒ nR.fieldLength = nL.fieldLength ≤ 2`.
- **E3.** In EnvGetScan: `cParent.present ⇒ cParent.index < cur.index`.
- **E4.** Appended Data nodes carry fixed, correct constructor columns.

**Not enforced:** kind/typeId/ctorId consistency, zero unused columns, field node types, and Names distinctness.

### 2.5 D-NAMES (P3-7 repair)

Every nameId equality or inequality depends on L4 Names being distinct, exact and canonical, bound to the statement. That covers:
- A108/A111 lookup;
- A145 shadow decode;
- A200 first match;
- A204 duplicate test;
- A214 `h≠t` and `l≠r`.

This is a **gating dependency**, not a discharged fact. Emitter acceptance waits for it.

---

## 3. AuxBlock extension (X1/X2, SELECTED and PROPOSED)

### 3.0 Profile, placement, labels, packing, enables, dimensions

**Profile.** `sherlock-k-a-aux/1` is PROPOSED as a separate extension file set. It extends `StepOperandsA` to `{program, pre, suppliedPost, suppliedAux}`.
- AuxBlock is a step-local witness. It is not an observation, not a part of ObservedRecordA, and not StateBlock.
- The seven L4 files and `table_capacities` are not edited. The new table names exist only in the extension.

**Labels (proposed by the owner).**
- AuxBlock is a private step witness.
- The aux dimension parameters are layout constants.
- No aux column is public.
- ProgramBlock, pre and post labels and native statement binding remain L04-open.
- I claim no privacy, hiding or native binding.

**Packing.** AuxBlock is the concatenation EnvGetScan ‖ FindScan ‖ BindPreflight ‖ EqScratch.
- Within each record: header columns, then rows in ascending order, then (for BindPreflight) the strict upper-triangular matrix in row-major order.
- Columns follow the listed order, flattened by the L4 record rules.
- The shape is fixed regardless of rule. A disabled record is all-zero.
- `packStepWitnessA` only orders and shape-checks. It decides nothing, and no column is a truth or acceptance bit. Every column is either constrained below or absent.

**Enables.** These are derived from pre and never supplied. At most one is true, because they require distinct `CR0.tag` values.
- `en_env := cg ∧ CR0.tag=Eval ∧ Expressions[X].tag=Var`
- `en_find := cg ∧ CR0.tag=Invoke`
- `en_bind := cg ∧ CR0.tag=Enter ∧ CR0.functionPresent=1`
- `en_eq := cg ∧ CR0.tag=Return ∧ depth>0 ∧ FS0.tag=Right ∧ FS0.binaryOp=Eq`

**Dimensions (symbolic; no numbers).**

| Dimension | Definition | Status |
|---|---|---|
| `R_env` | `max(1, E)` | Derived |
| `R_find` | `F` | Derived |
| `R_bind` | `P` | Derived |
| `W_eq` | `1 + D + K` | Derived |
| `Q_eq` | New dimension, `Q_eq ≥ 1` | X2. Its value is an L05 selection |

**Refusal names.**
- `Capacity{table:"EqScratch.rows", needed:"> Q_eq", limit:Q_eq, locator:"A153 Eq, AuxBlock.EqScratch row Q_eq-1"}`. This is reachable.
- `EnvGetScan.rows`, `FindScan.rows`, `BindPreflight.rows` and `EqScratch.stack` are declared but unreachable in the representation domain (lemmas below).

### 3.1 EnvGetScan (A108/A111; P3-1 repair)

**Header:** `resFound:Bool`, `resValue:VR`.

**Row columns**, for t in `[0, R_env)`:
- `act:Bool`
- `cur:ER`
- `cName:id(N)`
- `cValue:VR`
- `cParent:ER`
- `match:Bool`
- `term:enum(NONE, FOUND, ABSENT)`

**Constraints:**
- **E-1.** `act_0 = en_env`. `cur_0 = act_0 ? Root.environment : 0`.
- **E-2.** Let `rd_t := act_t ∧ cur_t.present`.
  - If `rd_t`: guard `cur_t.index < E0`, and `(cName, cValue, cParent)_t = EnvCells[cur_t.index]`.
  - Otherwise these three columns are zero.
- **E-3** (enforced, check E3). `rd_t ∧ cParent_t.present ⇒ cParent_t.index < cur_t.index`.
- **E-4.** `match_t = rd_t ∧ cName_t = Expressions[X].nameId`.
- **E-5.** `term_t` is:
  - NONE if `¬act_t`;
  - ABSENT if `act_t ∧ ¬cur_t.present`;
  - FOUND if `match_t`;
  - ABSENT if `rd_t ∧ ¬match_t ∧ ¬cParent_t.present`;
  - NONE otherwise.
- **E-6.** `act_{t+1} = act_t ∧ term_t=NONE` and `cur_{t+1} = act_{t+1} ? cParent_t : 0`. Inactive rows are all-zero.
- **E-7.** `act_{R_env-1} ⇒ term_{R_env-1} ≠ NONE`.
- **E-8.** `resFound = Σ_t[term_t=FOUND]` and `resValue = Σ_t[term_t=FOUND]·cValue_t`. Exactly one terminal row exists, so at most one term is nonzero.

**Selection.**
- `en_env ∧ resFound=1` selects A108, with `CR0'.valueRef = A:resValue`.
- `en_env ∧ resFound=0` selects A111.

**Lemma (no off-by-one).**
1. The cursor indices are `< E0` and strictly decrease (E-3). So an active row t with a present cursor has visited t+1 distinct cells, which gives `t ≤ E0−1`.
2. ABSENT is concluded **on the row of the last mismatch**, when the parent is absent.
3. An absent root concludes on row 0. This covers `E0=0`, where L4 slice and option validity force `Root.environment={0,0}`.
4. A full-chain miss over `E0=E` cells concludes on row `E−1`, which is the last row.

So the terminal row is always `≤ max(1,E)−1`. A violation of E-3 is WrongRepresentation (INV-ORD), not Capacity.

**Semantics.** FOUND is the newest cell on the chain with that nameId. That equals the L4 newest-first decode, which equals `environment.get` (reference.rs 142–145; K108–113). This holds under D-NAMES; whole-NameId canonicality is a stated dependency, not proved.

### 3.2 FindScan (A200; P3-2 repair)

**Header:** `outPresent:Bool`, `outId:id(F)`.

**Row columns**, for i in `[0, F)`: `act:Bool`, `fName:id(N)`, `match:Bool`, `seen:Bool`, `first:Bool`.

**Constraints:**
- **F-1.** `act_i = en_find ∧ i < activeFunctions`.
- **F-2.** `fName_i = act_i ? Functions[i].nameId : 0`.
- **F-3.** `match_i = act_i ∧ fName_i = CR0.nameId`.
- **F-4.** `seen_i = seen_{i−1} ∨ match_i`, with `seen_{−1} = 0`. `first_i = match_i ∧ ¬seen_{i−1}`.
- **F-5.** `outPresent = (F>0 ? seen_{F−1} : 0)` and `outId = Σ_i i·first_i`.

**What this gives:**
- `outPresent=1` exactly when some active row matches, and then `outId` is the **least** matching index.
- `outPresent=0` exactly when no active row matches, and then `outId=0`.
- There is no one-sided flag.

A200 writes `CR0' = {Enter, functionPresent=outPresent, functionId=outId, valueSlice=A:CR0.valueSlice}` (reference.rs 91–100; K281–284).

**Rows never refuse**, because `activeFunctions ≤ F`. For machine-produced states, the first name match is also the least structurally-equal row.

### 3.3 BindPreflight (A204; P3-10 repair)

**Header:**
- `psStart:start(P)`
- `k:count(P)`
- `retType:id(T)`
- `body:id(X)`
- `vsStart:start(V)`
- `vsLen:count(V)`
- `lenOk:Bool`
- `ok:Bool`

**Row columns**, for i in `[0, P)`:
- `reached:Bool`
- `pName:id(N)`
- `pType:id(T)`
- `arg:VR`
- `argType:id(T)`
- `typeOk:Bool`
- `dup:Bool`
- `pass:Bool`

**Matrix:** `eqName[i][j]:Bool` for `i < j < P`, in row-major order.

**No diagnostic column.** The r1 "first-failure index and reason" is **removed**.

**Constraints:**
- **B-1.** If `en_bind`:
  - `(psStart, k, retType, body)` are `Functions[CR0.functionId]`'s parameterSlice start and length, returnType and bodyExprId, with guard `functionId < activeFunctions`;
  - `(vsStart, vsLen) = CR0.valueSlice`.

  Otherwise all are zero.
- **B-2.** `lenOk = en_bind ∧ k = vsLen`.
- **B-3.** `reached_i = lenOk ∧ i<k ∧ (i = k−1 ∨ pass_{i+1})`. This is the reverse visiting order.
- **B-4.** Reads are gated by `reached_i`:
  - `(pName, pType)_i = Parameters[psStart+i]`;
  - `arg_i = ValueSlots[vsStart+i].value`;
  - `argType_i = node(arg_i).typeId`, with the space guard.

  Unreached rows are all-zero.
- **B-5.** `typeOk_i = reached_i ∧ argType_i = pType_i`.
- **B-6.** `eqName[i][j] = typeOk_i ∧ j<k ∧ pName_j = pName_i`. The type check comes before the duplicate check. Since `reached_i ⇒ reached_j` for `i<j<k`, `pName_j` has already been read.
- **B-7.** `dup_i = OR_j eqName[i][j]` and `pass_i = typeOk_i ∧ ¬dup_i`.
- **B-8.** `ok = lenOk ∧ (k=0 ∨ pass_0)`.

**Equivalence.** `ok ⇔ bind(parameters, values) ≠ None` (reference.rs 341–355; K290–298):
- the length is checked first;
- pairs are visited in reverse;
- `has_type` comes before `contains_key`;
- when pair i is reached, `contains_key` ranges exactly over `{pName_j : i<j<k}`.

**Commit (only if `ok`).** For `j = 0..k−1` with `p = k−1−j`:
- `appendEnv(pName_p, A:arg_p, j=0 ? {0,0} : {1, E0+j−1})`.
- `bindings = k>0 ? {1, E0+k−1} : {0,0}` and `bindingsPresent = 1`. This includes `Some(empty)` when `k=0`.

**Failure.** `bindingsPresent=0`, `bindings={0,0}`, and zero EnvCells.

**Either way.** `CR0' = {Bound, bindingsPresent, bindings, typeId=retType, exprId=A:body}`.

**Bounds.**
- Rows never refuse, because `k ≤ activeParameters ≤ P`.
- The fixed-shape cost is `P(P−1)/2` comparisons per enabled step, independent of k.
- No StateBlock scratch is retained.

### 3.4 EqScratch (A153 Eq; P3-3 repair)

The realization is SELECTED: a **stack vector**.

**Row columns**, for t in `[0, Q_eq)`:
- `act:Bool`
- `sp:count(W_eq)`
- `st[0..W_eq):(L:VR, R:VR)`
- `nL:NODE`
- `nR:NODE`
- `hd:Bool`
- `fL[0..2):VR`
- `fR[0..2):VR`
- `term:enum(NONE, MISMATCH, EMPTY)`

There is no header and no result flag.

**Constraints:**
- **Q-1** (initial row). `act_0 = en_eq`. If `act_0`: `sp_0 = 1`, `st_0[0] = (A:FS0.valueRef, A:CR0.valueRef)` (left, right), and all other slots zero.
- **Q-2** (stack top and reads). On an active row: `(L_t, R_t) := st_t[sp_t−1]`, `nL = node(L_t)` and `nR = node(R_t)`, each space-guarded. Slots `j ≥ sp_t` are zero.
- **Q-3** (head equality). `hd_t = act_t ∧` nL and nR agree on kind, typeId, ctorId, uintBytes and boolValue. It excludes fieldStart, fieldLength, space and index.
- **Q-4** (enforced, check E2). `hd_t ⇒ nR.fieldLength = nL.fieldLength ≤ 2`. Define `c_t := hd_t ? nL.fieldLength : 0`.
- **Q-5** (field reads). For `i < 2`: if `i < c_t`, then `fL[i] = fld(L_t, i)` and `fR[i] = fld(R_t, i)`, each guarded by its own node. Otherwise they are zero.
- **Q-6** (terminal classification). Let `b = sp_t − 1` and `spNext = b + c_t`. Then `term_t` is:
  - NONE if `¬act_t`;
  - MISMATCH if `¬hd_t`;
  - EMPTY if `spNext = 0`;
  - NONE otherwise.
- **Q-7** (transition). If `act_t ∧ term_t = NONE`, then:
  - `act_{t+1} = 1` and `sp_{t+1} = spNext`;
  - `st_{t+1}[j] = st_t[j]` for `j < b`;
  - for `c=2`: `st_{t+1}[b] = (fL[1], fR[1])` and `st_{t+1}[b+1] = (fL[0], fR[0])`, so field 0 (head or left) ends on top;
  - for `c=1`: `st_{t+1}[b] = (fL[0], fR[0])`;
  - slots `≥ spNext` are zero.

  Otherwise row t+1 is all-zero.
- **Q-8** (last row). `act_{Q_eq−1} ⇒ term_{Q_eq−1} ≠ NONE`. Without that, the step has no accepting assignment and is refused as `EqScratch.rows`.
- **Q-9** (stack width). `spNext ≤ W_eq`.

**Derived facts.**
- `act` is a forced prefix with no gaps.
- Exactly one terminal row exists: the last active row.
- `eq := Σ_t[term_t = EMPTY]`.

**Result, only after a valid terminal row.** `appendData(1, 1, 0, 0^16, eq, [])` produces `{Data, D0}`, and `CR0' = {PrimitiveResult, primitiveIsOk=1, valueRef={Data, D0}}`.

**Capacity order:** `EqScratch.rows` first, then `D0+1 ≤ D`.

**What can never happen.** No path leads from a budget to false, Err10, Err11, error 4, Exhausted or any semantic error. The term depends only on `hd` and `spNext`.

**Lemmas (paper only).**
- **Soundness.** MISMATCH occurs only at a same-path pair reached through head-equal ancestors, so the values are unequal.
- **Completeness.** The stack top is forced and push/pop are forced, so EMPTY implies every corresponding pair was visited and head-equal. That gives `Value` equality, including List head-before-tail being equivalent to Vec length plus elements, and the Tree shape.
- **No shortcuts.** There is no memoization and no same-reference shortcut. Repeated DAG occurrences are revisited, so a true result needs `ufd(L)` rows, which can be `2^Θ(steps)`.
- **Stack bound.** `sp ≤ 1 + depth(top) ≤ D0 + K0 < W_eq`, because references are strictly ordered (INV-ORD precondition). So `EqScratch.stack` cannot be hit in the domain.

---

## 4. Rule catalog (all 39; P3-8 repair, Muse pins)

**Masks.**
- **POP:** `FS(i)' = FS(i+1)` for `i < depth−1`, `FS(depth−1)' = 0`, `depth' = depth−1`.
- **PUSH:** `FS0' = CR0.pendingFrame`, `FS(i+1)' = FSi`, `depth+1`.
- **R0:** replace FS0 only.
- **W1:** `CR0'` is written, `controlLength'=1`, and the inactive columns are zero.
- **W2:** `CR0'` is `Push`, `CR1'` is `Eval`, and `controlLength'=2`.

These FS/CR masks implement the full39 relations. P4 component wiring stays open.

**Δ** is `(D, DF, E, V)`. "0" means every arena row and active length is preserved.

| Rule | Δ | FS / CR | Root and references | Capacity |
|---|---|---|---|---|
| A080 | 0 | – / clear `CRi<n`, n'=0 | Done, `A:CR0.valueRef`; not canonical (F3) | none |
| A082 | 0 | – / clear | Rejected `CR0.errorCode` | none |
| A084 | 0 | – / clear all `i<n` | Exhausted | none |
| A102 | 0 | – / W1 Return `A:E.valueRef` | fuel−1 | none |
| A108 | 0 | – / W1 Return `A:resValue` | EnvGetScan FOUND | none |
| A111 | 0 | – / W1 Error 3 | EnvGetScan ABSENT | none |
| A114, A117, A123, A126, A131 | 0 | – / W2, `Push.environment = A:Rho`, `pF.savedEnv = A:Rho` | per full39 | C ≥ 2 |
| A120 | 0 | – / W2, `Push.environment = A:Rho`; `pF = Unary{unaryOp}`, **savedEnv zero** | – | C ≥ 2 |
| A129 | 0 | – / W1 Invoke `(0,0)` | – | none |
| A135 | 0 | PUSH / unbox | env' = `A:CR0.environment`; branch only if `depth < stackLimit` | S: `depth+1 ≤ S` |
| A140 | 0 | – / CR0' = Error 4, `CRi'=0` for i≥1 | – | none |
| A145 | (0,0,1,0) | R0 Scope / W1 Eval(expr0) | §4.1 | E |
| A149 | 0 | R0 Right / W1 Eval(expr0) | env' = savedEnv | none |
| A153 | {0, (1,0,0,0), (1,2,0,0)} | POP / W1 PrimitiveResult | §4.1, §3.4 | EqScratch.rows, D, DF |
| A157 | {0, (1,0,0,0), (1,1,0,0)} | POP / W1 | §4.1 | D, DF |
| A161 / A165 | 0 | POP / W1 Eval(expr0 / expr1) | env' = savedEnv | none |
| A169 / A190 | 0 | – / W1 Error 5 / 6 | no pop, no restore | none |
| A173 | 0 | POP / W1 Cases | env' = savedEnv | none |
| A177 | (0,0,0,1 or m+1) | R0 Args / W1 Eval(nextId) | §4.1 | V |
| A181 | (0,0,0,1 or m+1) | POP / W1 Invoke | §4.1 | V |
| A185 / A195 | 0 | POP / CR0' = Return `A:CR0.valueRef` (**post-relation only; MD2 unselected**) | env' = savedEnv | none |
| A200 | 0 | – / W1 Enter | FindScan | none |
| A202, A206, A212 | 0 | – / W1 Error 7, 8, 9 | – | none |
| A204 | (0,0,0 or k,0) | – / W1 Bound | §3.3 | E (only if ok) |
| A208 | 0 | – / W2 `Push{Return(typeId, savedEnv=A:Rho), env=A:CR0.bindings}`, CR1 Eval(`A:CR0.exprId`) | – | C ≥ 2 |
| A214 | (0,0,{0,1,2},0) | – / W1 CaseResult | §4.1 | E (only on match) |
| A217 | 0 | – / W1 Cases | – | none |
| A220 | 0 | – / W2 `Push{Scope(A:CR0.environment), env=A:CR0.bindings}` | – | C ≥ 2 |
| A223 / A225 | 0 | – / W1 Return / Error(code) | – | none |

**Fuel.** All charged rules decrement fuel by one. A080, A082 and A084 do not (F1).

**Preservation.** Every unlisted Root column, FS and CR row, arena row, active length and ProgramBlock cell is preserved.

### 4.1 Allocating rules

**A145.**
- Commits `appendEnv(FS0.nameId, A:CR0.valueRef, A:FS0.savedEnv)`.
- `Root.environment' = {1, E0}`.
- `FS0' = {Scope, savedEnv = A:FS0.savedEnv}` and `CR0' = Eval(A:FS0.expr0)`.
- The saved root is preserved, so Scope cannot see the new binding. Constant-space values stay Constant.

**A153 non-Eq.** `L = A:FS0.valueRef` and `R = A:CR0.valueRef`. Sort reads are enabled for every op except Eq.

| Op | Condition | Branch |
|---|---|---|
| Add, Sub, Mul | `sL=0 ∧ sR=0` | ARITH, else ERR11 |
| Lt | `sL=0 ∧ sR=0` | LT, else ERR11 |
| Cons | `sL=0 ∧ sR=2` | CONS, else ERR11 |
| Branch | `sL=3 ∧ sR=3` | BRANCH, else ERR11 |

**Arithmetic relations,** where a and b are the integers held in `uintBytes`:
- `a+b = r + 2^128·cy`;
- `a−b = r − 2^128·β`;
- `a·b = r + 2^128·h`, with `h < 2^128`.

Ok holds iff `cy=0`, `β=0` or `h=0` respectively; otherwise the branch is ERR10. Lt gives `Bool(β)`.

**A153 appends.**
- ARITH Ok: `appendData(0, 0, 0, LE16(r), 0, [])`.
- LT: `appendData(1, 1, 0, 0^16, β, [])`.
- CONS: `appendData(2, 2, 1, 0^16, 0, [A:L, A:R])`.
- BRANCH: `appendData(2, 3, 3, 0^16, 0, [A:L, A:R])`.
- ERR10 and ERR11: zero appends, and `CR0' = {PrimitiveResult, primitiveIsOk=0, errorCode}`.

The limb realization is O-ARITH and remains open.

**A157.**
- LEAF (`Leaf ∧ sort=0`): `appendData(2, 3, 2, 0^16, 0, [A:CR0.valueRef])`.
- NOT (`Not ∧ sort=1`): `appendData(1, 1, 0, 0^16, ¬b, [])`.
- Anything else is ERR11 with zero appends.

**A177/A181.** With `done = (s0, m) = FS0.completedSlice` and `v = A:CR0.valueRef`:
- `EXT := m>0 ∧ s0+m = V0`.
- **EXT:** `appendSlot(v)`, giving `newDone = (s0, m+1)`.
- **COPY:** `ValueSlots[V0+i] := ValueSlots[s0+i]` for `i < m`, guarded, then `ValueSlots[V0+m] := v`, giving `newDone = (V0, m+1)`.
- **Then:**
  - A177 sets `FS0' = {Args, nameId, newDone, tail (MD7), A:savedEnv}` and `CR0' = Eval(ExprChildren[s.start].expression)`.
  - A181 POPs and sets `CR0' = Invoke(nameId, newDone)`.
- In both, `env' = A:FS0.savedEnv`.
- All writes are at indices `≥ V0`, which keeps every co-holder safe.

**A214.** With `v = A:CR0.valueRef`, `ρ = A:CR0.environment`, and pattern `p = Patterns[Arms[s.start].patternId]`, guarded:

| Pattern | Match condition | Cells appended | bindings |
|---|---|---|---|
| Var x | true (no node read) | `(x, v, ρ)` | `{1, E0}` |
| Nil | `kind=2 ∧ typeId=2 ∧ ctorId=0` | none | **ρ, exact alias** |
| Cons(h,t) | `typeId=2 ∧ ctorId=1 ∧ h≠t` | `(h, fld0, ρ)`, `(t, fld1, {1,E0})` | `{1, E0+1}` |
| Leaf x | `typeId=3 ∧ ctorId=2` | `(x, fld0, ρ)` | `{1, E0}` |
| Branch(l,r) | `typeId=3 ∧ ctorId=3 ∧ l≠r` | `(l, fld0, ρ)`, `(r, fld1, {1,E0})` | `{1, E0+1}` |
| Miss | – | none | `present=0`, `{0,0}` |

- The Nil alias is a **deterministic selection, not a uniqueness claim** (P3-9).
- Field reads happen only after the shape and binder tests pass.
- `CR0' = {CaseResult, bindingsPresent, bindings, A:v, A:Arms[s.start].bodyExprId, tail (MD7), ρ}`.

---

## 5. Bounds and separate quantities (P3-5 repair)

**Generic per-step bounds.** These hold for any representation-domain image, including Step mode:
- `ΔD ≤ 1`;
- `ΔDF ≤ 2`;
- `ΔE ≤ max(2, k_max)`, where `k_max` is the largest active `parameterSlice.len` and `k_max ≤ P`; this is a ProgramBlock fact, not a reachability assumption;
- `ΔV ≤ V0 + 1`, because COPY has `m ≤ V0` by slice validity;
- `controlLength' ≤ 2`;
- `depth' ≤ depth + 1`.

**The a_max bound is not assumed.** `ΔV ≤ a_max` would need a candidate invariant: for each Args frame, `completed.len + remaining.len + 1 ≤ a_max`. That invariant is **unselected and not assumed**.

**Valid generic envelope.** Over `T_c` charged steps:
- `D_end ≤ D_start + T_c`;
- `DF_end ≤ DF_start + 2·T_c`;
- `E_end ≤ E_start + T_c·max(2, k_max)`;
- `V_end + 1 ≤ (V_start + 1)·2^{T_c}`.

These are sound upper bounds only. **The r1 "sufficient static admission test" and the manifest's `per_step_delta_max.ValueSlots = a_max` are withdrawn.** The exact need is the trace sum, refused per step (F5).

**Aux per-step need:**
- EnvGetScan: `≤ max(1, E0)` rows;
- FindScan: `activeFunctions` rows;
- BindPreflight: `k` rows;
- EqScratch: `≤ ufd(L) ≤ 2^{D0+K0} − 1` rows.

Fuel, live DAG size or arena occupancy alone cannot exclude EqScratch refusal. Q_eq is its own dimension.

**Quantities kept separate.** None implies another:
- logical work (fuel; helpers add 0);
- StateBlock occupancy;
- copy volume (`Σ` COPY m);
- the unfolded boundary envelope (F3);
- Q_eq rows and the other aux rows;
- DUST/fees;
- proof rows and circuit k.

L05 numeric admission stays open. No capacity is a source outcome.

---

## 6. Hand-derived examples

These are paper derivations only. Capacities in them are illustrative and are not selected values.

### EX1: alias, preserved root and A108 (P3-11)

**Program:** `let x = 7 in leaf(x)`. Constants table: `K0 = UInt7` (kind 0, type 0). **Start:** `D0 = DF0 = E0 = V0 = 0`, `env = {0,0}`, depth 0.

1. **A114.** `CR0' = Push{Let{x, expr0=body, savedEnv={0,0}}, env={0,0}}`, `CR1' = Eval(lit)`, n' = 2. Needs C ≥ 2.
2. **A135.** `FS0' = Let`, depth 1, `env' = {0,0}`, `CR0' = Eval(lit)`, `CR1' = 0`.
3. **A102.** `CR0' = Return{C,0}`. No Constants read.
4. **A145.** `EnvCells[0] = {x, {C,0}, {0,0}}`, `env' = {1,0}`, `FS0' = Scope{{0,0}}`, `CR0' = Eval(Unary(Leaf, Var x))`, E' = 1. Refused if E = 0.
5. **A120.** `CR0' = Push{Unary{Leaf}, savedEnv=0, env={1,0}}`, `CR1' = Eval(Var x)`.
6. **A135.** `FS0' = Unary`, `FS1 = Scope{{0,0}}`, depth 2, `CR0' = Eval(Var x)`.
7. **A108.** EnvGetScan row 0: `cur = {1,0}`, `cName = x`, so FOUND. `resValue = {C,0}`. Rows ≥ 1 are zero. `CR0' = Return{C,0}`.
8. **A157 Leaf.** `Data[0] = {2, 3, 2, 0^16, 0, fs=0, fl=1}` and `DataFields[0] = {C,0}`. POP: `FS0' = Scope{{0,0}}`, `FS1' = 0`, depth 1. `CR0' = PrimitiveResult{1, {D,0}}`. env stays `{1,0}`.

**Preservation.** Scope `{0,0}` still decodes as the empty map. Refused as `Capacity{Data, 1, 0}` if D = 0.

### EX2: shared equality (true), Q_eq boundary and a wrong activity trace

**Data:**
- `d0 = UInt1`;
- `d1 = Leaf`, `DF0 = {D,0}`;
- `d2 = Branch`, `DF1 = DF2 = {D,1}`.

Pre: `D0 = 3`, `DF0 = 3`.

**Constants:**
- `K0 = UInt1`;
- `K1 = Leaf`, `KF0 = {C,0}`;
- `K2 = UInt1`;
- `K3 = Leaf`, `KF1 = {C,2}`;
- `K4 = Branch`, `KF2 = {C,1}`, `KF3 = {C,3}`.

**Trace of `Eq(L={D,2}, R={C,4})`:**

| Row | Pair | Result | Stack afterwards (top last) | sp |
|---|---|---|---|---|
| 0 | (D2, C4) | hd, c = 2 | [(D1,C3), (D1,C1)] | 2 |
| 1 | (D1, C1) | hd, c = 1 | [(D1,C3), (D0,C0)] | 2 |
| 2 | (D0, C0) | hd, c = 0 | [(D1,C3)] | 1 |
| 3 | (D1, C3) | hd, c = 1 | [(D0,C2)] | 1 |
| 4 | (D0, C2) | spNext = 0 | – | EMPTY |

**Outcome.**
- With `Q_eq = 5`: row 4 is the **last row** and terminal, so the step is accepted. `eq = 1`, `Data[3] = {1, 1, 0, 0^16, 1, 0, 0}`, POP.
- With `Q_eq = 4`: row 3 is active and nonterminal, so Q-8 fails and the step is refused as `EqScratch.rows`. It never yields false.
- With `Q_eq ≥ 5` but `D = 3`: Data refusal.
- With `Q_eq = 4` and `D = 3`: EqScratch.rows is reported first.

**Wrong activity trace, rejected by definition.** A witness with `act_2 = 0` after row 1 (term NONE, sp = 2) violates Q-7, which requires `act_{t+1} = act_t ∧ term_t = NONE`. A witness with `term_1 = EMPTY` violates Q-6. No eq column exists to forge.

### EX3: mismatches with this example's own tables

**Constants:** `K0 = Nil`. **Data:** `d0 = UInt1`.

**`Eq({D,0}, {C,0})`.** Row 0 has kind 0 ≠ 2, so MISMATCH. Bool false is appended. It is not Err11.

**Lists.** Data:
- `d0 = 1`;
- `d1 = Nil`;
- `d2 = Cons(DF0 = {D,0}, DF1 = {D,1})`, i.e. `[1]`;
- `d3 = 2`;
- `d4 = Cons(DF2 = {D,3}, DF3 = {D,1})`;
- `d5 = Cons(DF4 = {D,0}, DF5 = {D,4})`, i.e. `[1,2]`.

**`Eq(d2, d5)`:**
- row 0: hd, push (d1,d4) then (d0,d0);
- row 1: 1 = 1;
- row 2: Nil vs Cons, ctorId 0 ≠ 1, so MISMATCH.

### EX4: argument slices, with V0 symbolic (P3-11)

**Program:** `f(a, g(x), c)`, where the pre-state V0 is `v`.

**Explicit assumption:** evaluating g's body appends no ValueSlots.

| Step | done | Branch | Writes | newDone | V' |
|---|---|---|---|---|---|
| A177(f), a | (0,0) | COPY | `VS[v] = ra` | (v, 1) | v+1 |
| A181(g), x | (0,0) | COPY | `VS[v+1] = rx`; Invoke g | (v+1, 1) | v+2 |
| A177(f), rg | (v, 1) | COPY (v+1 ≠ v+2) | `VS[v+2] = ra`, `VS[v+3] = rg` | (v+2, 2) | v+4 |
| A181(f), c | (v+2, 2) | EXT (v+4 = V0) | `VS[v+4] = rc` | (v+2, 3) | v+5 |

**Outcome.**
- `(v, 1)` and `(v+1, 1)` still decode as `[ra]` and `[rx]`.
- If g's body had appended w slots, the third row would write at `v+2+w`, and the final step would still be EXT.
- The last step is refused iff `V < v+5`.

### EX5: BindPreflight

**Setup:** `f(x:UInt, y:Bool, x:UInt)`, with `psStart = 0` and `k = 3`.

**Values `[UInt1, BoolT, UInt2]`:**
- Row 2: reached, typeOk, no j > 2, pass.
- Row 1: reached, typeOk, `eqName[1][2]` (y vs x) = 0, pass.
- Row 0: reached, typeOk, `eqName[0][2] = 1`, so dup.
- `ok = 0`. Zero cells: `Bound{0, {0,0}, retType, A:body}`. Accepted at `E0 = E`.

**Values `[UInt1, UInt7, UInt2]`:**
- Row 1: `argType 0 ≠ 1`, so typeOk = 0 and `eqName[1][*] = 0`.
- Row 0 is unreached: no ValueSlots or Parameters read, and the row is all-zero.

**Success `g(a:UInt, b:Bool)` on `[r1, rT]` with `E0 = 10`:**
- `EnvCells[10] = {b, rT, {0,0}}`;
- `EnvCells[11] = {a, r1, {1,10}}`;
- bindings `{1,11}`. Needs E ≥ 12.

**Zero parameters with an empty slice:** `ok = 1`, `bindingsPresent = 1`, `{0,0}`.

**Length mismatch:** all rows zero, `ok = 0`.

### EX6: A214 shadowing, Nil alias and the full-capacity branch

**Saved environment:** `EnvCells[0] = {z, rz, {0,0}}`, `EnvCells[1] = {h, r9, {1,0}}`, so `ρ = {1,1}`.

**Nil on Constants `K0 = Nil`:** `bindings = {1,1}`, `present = 1`. With `ρ = {0,0}`, the result is `present = 1`, `{0,0}`.

**Cons(h,t) on Data:** `d0 = 5`, `d1 = 6`, `d2 = Nil`, `d3 = Cons(DF0 = {D,1}, DF1 = {D,2})`, `d4 = Cons(DF2 = {D,0}, DF3 = {D,3})`, with `E0 = 2`:
- `EnvCells[2] = {h, {D,0}, {1,1}}`;
- `EnvCells[3] = {t, {D,3}, {1,2}}`;
- bindings `{1,3}`.

This decodes as `t ↦ [6]`, `h ↦ 5` (shadowing 9), `z ↦ rz`. ρ still decodes `h ↦ r9`. No Data rows are appended.

**Capacity.** With E = 3: `Capacity{EnvCells, 4, 3, index 3}`.

**Cons(h,h):** a miss, accepted even at `E0 = E`.

### EX7: EnvGetScan full-chain miss and E0 = 0

**Setup:** `E = E0 = 3`, so `R_env = 3`. Cells:
- `0 = {a, ra, {0,0}}`;
- `1 = {b, rb, {1,0}}`;
- `2 = {c, rc, {1,1}}`.

`Root.environment = {1,2}`; look up d.

| Row | cur | Name | Parent | term |
|---|---|---|---|---|
| 0 | {1,2} | c | {1,1} | NONE |
| 1 | {1,1} | b | {1,0} | NONE |
| 2 | {1,0} | a | {0,0} | **ABSENT** on the last row |

So A111 Error 3.

**With `E0 = 0`:** `Root.environment = {0,0}`, so row 0 is ABSENT and `R_env = max(1, E)`.

**Wrong trace.** A witness that sets `term_1 = ABSENT` while `cParent_1.present = 1` violates E-5.

### EX8: FindScan

**Functions:** `0: p`, `1: q`, `2: q`.
- Invoke q: `first_1 = 1`, `outPresent = 1`, `outId = 1`. Row 2 matches but is not first.
- Invoke p: `outId = 0` with `outPresent = 1`, i.e. `Some(row0)`.
- Invoke r: `outPresent = 0`, `outId = 0`.

**Wrong witness.** `outPresent = 0` alongside `match_1 = 1` violates F-5.

### EX9: full-capacity branches

- **`Add(2^128−1, 1)` at `D0 = D`:** ERR10, accepted.
- **A120 at C = 1:** `Capacity{ControlRows, 2, 1}`. Never error 4.
- **A135 with `depth = S < stackLimit`:** `Capacity{FrameSlots}`.
- **A135 with `depth = stackLimit`:** A140 Error 4, accepted even when `depth = S`.
- **A204 failure, or A214 miss, at `E0 = E`:** accepted.

---

## 7. Future checks (none run, none proved)

**Positive checks:**
- **K and Rust:** `decode(post)` equals SNAPSHOT8 for each allocating rule.
  - `ok ⇔ bind ≠ None`;
  - the A214 table equals `matching`;
  - `eq = (Value ==)`;
  - the FindScan output equals `iter().find`;
  - the EnvGetScan output equals `map.get`.
- **Quint (bounded):**
  - for all small DAGs, including sharing, and all `Q_eq ≥` need, eq is exact;
  - every permutation of k ≤ small with duplicates;
  - interleaved calls exercising EXT/COPY.
- **Clean:** every aux column is a function of `(program, pre, dims)`; this is the determinism check.
- **Circuit:** accepting traces exist at exactly-needed row counts (EX2, EX7).

**Negative checks.** Each of these must have no accepting assignment:
- dropping a stack pair;
- a non-forced act;
- `outPresent = 0` with a match;
- a forged duplicate;
- reading pair 0 after a TYPE failure;
- cells appended on a failure branch;
- a COPY that writes a slot below V0;
- EXT with m = 0;
- Q_eq exhaustion encoded as MISMATCH;
- Err10 refused at `D0 = D`;
- a Nil match producing a cell;
- duplicate Names rows, which is a D-NAMES negative.

---

## 8. Repair dispositions

| Item | Disposition |
|---|---|
| P3-1 | Fixed in §3.1: same-row ABSENT, `R_env = max(1,E)`, E0 = 0 and full-chain miss proven (EX7) |
| P3-2 | Fixed in §3.2: two-sided least-match fold, `functionId = 0` on None |
| P3-3 | Fixed in §3.4: act recurrence, forced shifts, terminal rule, enforced fieldLength, EqScratch-then-Data order |
| P3-4 | Fixed in §2.3: P6.C and P6.S included in preflight, with a fixed family order |
| P3-5 | Fixed in §5: `ΔV ≤ V0+1`, a_max invariant unselected, static admission claim withdrawn |
| P3-6 | Fixed in §2.4: INV-TY-D and INV-TY-K are preconditions; E1–E4 are listed as enforced |
| P3-7 | Fixed in §2.5: D-NAMES gating, not discharged |
| P3-8 | Fixed in §4: A120 pending savedEnv is zero, Push env is Rho |
| P3-9 | Fixed in §4.1: Nil alias is a deterministic selection |
| P3-10 | Fixed in §3.3: diagnostic removed, masks derived |
| P3-11 | Fixed: u8[16] written explicitly; EX1 includes A108; EX3 has its own tables |
| Muse pins | A120 and A185/A195 as above; EnvGetScan row count resolved; X1/X2 supplied; bound domains stated |
| X1(a)–(d) | Fixed in §3.0: placement and labels proposed, every column constrained, masks given, packing and witness binding with no truth bit |
| X2(a)–(c) | Fixed: Q_eq is a symbolic dimension, the refusal name is EqScratch.rows, and the dimension is not statically excludable (§5) |

---

## 9. Out of scope, dependencies and review

**Out of scope or open:**
- P2 owner obligations beyond the four aux components;
- P4 component wiring;
- P5 global exactly-one dispatch and preservation wiring;
- G4 guard primitives;
- F4;
- MD2;
- StepInv and chaining;
- Constant typing enforcement;
- D-NAMES enforcement;
- native statement binding and labels for ProgramBlock, pre and post;
- O-ARITH lowering;
- L05 numeric admission;
- B, general ADTs, financial behaviour, proof, ledger and Midnight stagenet;
- U0–U7, M1–M17, MC, SP and G;
- C4.

The full39 conditions 1–10 remain binding.

**Review.** Adoption needs:
- a fresh designated-owner security audit, independent of this authorship;
- a fresh Muse normal audit on these exact bytes;
- a Root adoption record.

Any byte change voids those reviews for the changed bytes. Until then, nothing here is an emitter criterion.

---

## 10. Manifest

```json
{
  "format": "sherlock-candidateA-p3-source-revision/2",
  "status": "PROPOSED_UNADOPTED_SOURCE_DESIGN; author_only; not_a_vote; native_authority=0; execution=UNRUN; no_numeric_values; no_grants",
  "supersedes_candidate_sha256": "e418610606c793c23ccbc6877d1981af9b2076a64f2ae718eae3179135543bf1",
  "repairs": {"owner_challenge_sha256": "19d238028b27686c02890478d82c7246d72e6257c3390267de09a6b511986a3e", "muse_normal_sha256": "d566c8ed266462c6bfe87880eeca522eb323f42c923e95abac67e52e36efd911", "prior_votes_inherited": false},
  "basis": {"p1_full39_proposal_sha256": "6823badbbca40cb945881ae689cd812f4ddca3aceee9b7ec8dbfde5cf9e122c2", "p1_manifest_sha256": "a742817c5f64c46eda6c5988d2a3c9551f50139ceb1632a70dc8860c70ff159b", "p1_root_adoption_sha256": "e09dedcd432e5d39a69551581d93291c486487c186d85bdd61cf1efb9dd0c05e", "l4_crosswalk_sha256": "22c2ffcb41644a51f0edef3271c4bae1368512e2dcd3b4bfb7aacd3bd91481f0", "l4_design_sha256": "3738ca71fe8ad87d8ddba0f2251dcc54214a160b131550d5b8a49472763ade0e", "r5_json_sha256": "122289ad639de0e0d527106e2da8cfd19dd9ddf2fb16781a3ca742ab0731b0a5", "reference_rs_sha256": "e1bb1098d6e19ca3bca30a09bb90b11366fa9f70726aba9165d54f14056b181b", "main_k_sha256": "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41", "hashes_recomputed": false},
  "policy": "A_monotone_append_preflight_commit",
  "unchanged": ["full39 P1 semantic map bytes", "seven frozen L4 files", "L4 tables columns widths tags block order", "ProgramBlock", "initialFuel and stackLimit", "ValueRef space typing", "immutable occurrence links", "saved roots", "F4 UNSELECTED", "MD2 UNSELECTED"],
  "notation": {"node(r)": "Data[r.index] if space=Data else Constants[r.index]; guard index < (Data?D0:K0)", "fld(r,i)": "field i; guard i < node(r).fieldLength and fieldStart+i < (Data?DF0:KF0)", "sort(r)": "node(r).typeId under INV-TY", "name_eq": "nameId equality under D-NAMES", "cg": "Pending and n>0 and not(CR0.tag=Return and depth=0) and CR0.tag!=Error and remainingFuel!=0", "VR": "Bool + id(max(K,D))", "ER": "Bool + id(E)", "NODE": "enum(3)+id(T)+id(Q)+u8[16]+Bool+start(max(DF,KF))+count(max(DF,KF))"},
  "append_primitives": {"appendData": "Data[D0]={kind,typeId,ctorId,uintBytes u8[16],boolValue,fieldStart=c>0?DF0:0,fieldLength=c}; DataFields[DF0+i]=f_i same step (G1); returns {Data,D0}", "appendEnv": "EnvCells[E0+j]={nameId,vref,parent}; returns {1,E0+j}; parent is pre-active root or earlier same-step cell", "appendSlot": "ValueSlots[V0+j] increasing order", "one_family_per_rule": true},
  "step_phases": ["1 dispatch: cg, guarded reads, enabled aux record constrained", "2 branch selection: pure function of ProgramBlock, pre, aux outputs", "3 capacity preflight of selected branch in refusal_order", "4 commit appends then FS/CR masks and Root writes; preserve all else"],
  "refusal_order": ["WrongRepresentation dependency: guard failure or enforced check E2/E3 (precedes all)", "EqScratch.rows", "Data", "DataFields", "EnvCells", "ValueSlots", "ControlRows (P6.C>=2)", "FrameSlots (P6.S)"],
  "refusal_rules": {"failure_branches_need_zero_in_all_families": true, "capacity_never_semantic": "never error 3-11, false, Exhausted, Rejected", "within_family_locator": "first overflowing index (F5)"},
  "invariants": {"INV-IMM": "post.X[i]=pre.X[i] for i<X0", "INV-PAD": "rows >= active length zero", "INV-SLOT": "no ValueSlots write below V0", "INV-NOFAIL": "semantic failure branch appends no StateBlock row", "INV-CAP": "refusal has no post-image", "INV-AUX": "AuxBlock never StateBlock; disabled record all-zero", "INV-ORD-appended": "by construction"},
  "inv_ty": {"preconditions": ["INV-TY-D every reached Data read: StepInv/chaining dependency unselected", "INV-TY-K every reached Constant read: ProgramBlock canonicality/constTyping/F4/L04 unselected"], "enforced_checks": ["E1 index guard on every node/field/cell/slot/parameter/function/arm/pattern read (G4 primitive dependency)", "E2 EqScratch hd implies nR.fieldLength=nL.fieldLength<=2", "E3 EnvGetScan cParent.present implies cParent.index<cur.index", "E4 appended Data nodes have fixed correct constructor columns"], "not_enforced": ["kind/typeId/ctorId consistency", "unused columns zero", "field node types", "Names distinctness"]},
  "d_names": "All nameId equality/inequality (A108/A111, A145 decode, A200, A204 dup, A214 binders) gated on D-NAMES; NOT discharged",
  "aux_extension": {
    "profile": "sherlock-k-a-aux/1",
    "status": "PROPOSED",
    "separate_from_frozen_L4": true,
    "existing_l4_codec_implements": false,
    "operand": "StepOperandsA extended to {program, pre, suppliedPost, suppliedAux}; AuxBlock not observation, not ObservedRecordA, not StateBlock",
    "label_policy": {"AuxBlock": "private step witness", "aux_dimensions": "layout constants", "public_aux_columns": "none", "program_pre_post_labels": "L04 open", "privacy_or_native_binding_claim": false},
    "packing_order": ["EnvGetScan header then rows", "FindScan header then rows", "BindPreflight header then rows then eqName strict upper triangle row-major", "EqScratch rows"],
    "packing_rules": "columns in listed order, flattened by L4 record rules, fixed shape, disabled record all-zero; packStepWitnessA orders and shape-checks only; no truth or acceptance column",
    "enables": {"en_env": "cg and CR0.tag=Eval and Expressions[X].tag=Var", "en_find": "cg and CR0.tag=Invoke", "en_bind": "cg and CR0.tag=Enter and CR0.functionPresent=1", "en_eq": "cg and CR0.tag=Return and depth>0 and FS0.tag=Right and FS0.binaryOp=Eq", "at_most_one": "distinct CR0 tags"},
    "dimensions": {"R_env": "max(1,E) derived", "R_find": "F derived", "R_bind": "P derived", "W_eq": "1+D+K derived", "Q_eq": "new symbolic dimension, Q_eq>=1, L05 selection, no number"},
    "refusal_names": {"EqScratch.rows": "Capacity{table:EqScratch.rows, needed:>Q_eq, limit:Q_eq, locator:A153 Eq AuxBlock.EqScratch row Q_eq-1}; reachable", "EnvGetScan.rows": "declared; unreachable in domain", "FindScan.rows": "declared; unreachable", "BindPreflight.rows": "declared; unreachable", "EqScratch.stack": "declared; unreachable in domain"},
    "records": {
      "EnvGetScan": {
        "rules": ["A108", "A111"],
        "rows": "R_env",
        "header": [["resFound", "Bool", "sum_t [term_t=FOUND]"], ["resValue", "VR", "sum_t [term_t=FOUND]*cValue_t"]],
        "row_columns": [["act", "Bool", "E-1, E-6"], ["cur", "ER", "E-1, E-6"], ["cName", "id(N)", "E-2"], ["cValue", "VR", "E-2"], ["cParent", "ER", "E-2"], ["match", "Bool", "E-4"], ["term", "enum(NONE,FOUND,ABSENT)", "E-5"]],
        "constraints": ["E-1 act_0=en_env; cur_0=act_0?Root.environment:0", "E-2 rd_t=act_t and cur_t.present; if rd_t: cur_t.index<E0 and (cName,cValue,cParent)=EnvCells[cur_t.index] else zero", "E-3 enforced: rd_t and cParent_t.present implies cParent_t.index<cur_t.index", "E-4 match_t=rd_t and cName_t=Expressions[X].nameId", "E-5 term_t= NONE if not act_t; ABSENT if act_t and not cur_t.present; FOUND if match_t; ABSENT if rd_t and not match_t and not cParent_t.present; else NONE", "E-6 act_{t+1}=act_t and term_t=NONE; cur_{t+1}=act_{t+1}?cParent_t:0; inactive rows all-zero", "E-7 act_{R_env-1} implies term_{R_env-1}!=NONE"],
        "selection": "en_env and resFound=1 -> A108 with CR0.valueRef=A:resValue; en_env and resFound=0 -> A111",
        "lemma": "strictly decreasing indices < E0 so terminal row <= max(1,E)-1; E0=0 forces root {0,0} -> ABSENT row 0; full-chain miss with E0=E terminates ABSENT on row E-1 same row as last mismatch"
      },
      "FindScan": {
        "rules": ["A200"],
        "rows": "R_find",
        "header": [["outPresent", "Bool", "F>0 ? seen_{F-1} : 0"], ["outId", "id(F)", "sum_i i*first_i"]],
        "row_columns": [["act", "Bool", "en_find and i<activeFunctions"], ["fName", "id(N)", "act_i ? Functions[i].nameId : 0"], ["match", "Bool", "act_i and fName_i=CR0.nameId"], ["seen", "Bool", "seen_{i-1} or match_i; seen_{-1}=0"], ["first", "Bool", "match_i and not seen_{i-1}"]],
        "constraints": ["outPresent=1 iff some active match and outId is least matching index", "outPresent=0 iff no active match and outId=0"],
        "write": "CR0={Enter,functionPresent=outPresent,functionId=outId,valueSlice=A:CR0.valueSlice}",
        "lemma": "activeFunctions<=F so never refuses"
      },
      "BindPreflight": {
        "rules": ["A204"],
        "rows": "R_bind",
        "header": [["psStart", "start(P)", "en_bind ? Functions[CR0.functionId].parameterSlice.start : 0; guard functionId<activeFunctions"], ["k", "count(P)", "en_bind ? parameterSlice.len : 0"], ["retType", "id(T)", "en_bind ? returnType : 0"], ["body", "id(X)", "en_bind ? bodyExprId : 0"], ["vsStart", "start(V)", "en_bind ? CR0.valueSlice.start : 0"], ["vsLen", "count(V)", "en_bind ? CR0.valueSlice.len : 0"], ["lenOk", "Bool", "en_bind and k=vsLen"], ["ok", "Bool", "lenOk and (k=0 or pass_0)"]],
        "row_columns": [["reached", "Bool", "lenOk and i<k and (i=k-1 or pass_{i+1})"], ["pName", "id(N)", "reached_i ? Parameters[psStart+i].nameId : 0"], ["pType", "id(T)", "reached_i ? Parameters[psStart+i].typeId : 0"], ["arg", "VR", "reached_i ? ValueSlots[vsStart+i].value : 0"], ["argType", "id(T)", "reached_i ? node(arg_i).typeId (space guard) : 0"], ["typeOk", "Bool", "reached_i and argType_i=pType_i"], ["dup", "Bool", "OR_j eqName[i][j]"], ["pass", "Bool", "typeOk_i and not dup_i"]],
        "matrix": [["eqName[i][j] for i<j<P", "Bool", "typeOk_i and j<k and pName_j=pName_i"]],
        "removed_columns": ["firstFailureIndex", "firstFailureReason"],
        "commit_success": "for j=0..k-1, p=k-1-j: appendEnv(pName_p, A:arg_p, j=0?{0,0}:{1,E0+j-1}); bindings=k>0?{1,E0+k-1}:{0,0}; bindingsPresent=1",
        "commit_failure": "no append; bindingsPresent=0; bindings={0,0}",
        "write": "CR0={Bound,bindingsPresent,bindings,typeId=retType,exprId=A:body}",
        "equivalence": "ok iff bind() != None: length first, reverse pairs, type before dup, dup over successfully bound later names",
        "lemma": "k<=activeParameters<=P so never refuses; cost P(P-1)/2 comparisons fixed shape"
      },
      "EqScratch": {
        "rules": ["A153 op Eq"],
        "rows": "Q_eq",
        "realization": "stack_vector SELECTED",
        "header": [],
        "row_columns": [["act", "Bool", "Q-1, Q-7"], ["sp", "count(W_eq)", "Q-1, Q-7"], ["st[0..W_eq)", "(VR,VR) each", "Q-1, Q-2, Q-7"], ["nL", "NODE", "node(st_t[sp_t-1].L) space-guarded if act_t else 0"], ["nR", "NODE", "node(st_t[sp_t-1].R) space-guarded if act_t else 0"], ["hd", "Bool", "act_t and equal kind,typeId,ctorId,uintBytes,boolValue"], ["fL[0..2)", "VR", "i<c_t ? fld(L_t,i) : 0"], ["fR[0..2)", "VR", "i<c_t ? fld(R_t,i) : 0"], ["term", "enum(NONE,MISMATCH,EMPTY)", "Q-6"]],
        "constraints": ["Q-1 act_0=en_eq; if act_0: sp_0=1, st_0[0]=(A:FS0.valueRef,A:CR0.valueRef), other slots 0", "Q-2 active row: (L_t,R_t)=st_t[sp_t-1]; slots j>=sp_t zero", "Q-3 hd_t as column definition", "Q-4 enforced: hd_t implies nR.fieldLength=nL.fieldLength<=2; c_t=hd_t?nL.fieldLength:0", "Q-5 field reads only for i<c_t, each own-node guarded", "Q-6 b=sp_t-1; spNext=b+c_t; term_t= NONE if not act_t; MISMATCH if not hd_t; EMPTY if spNext=0; else NONE", "Q-7 if act_t and term_t=NONE: act_{t+1}=1, sp_{t+1}=spNext, st_{t+1}[j]=st_t[j] for j<b; c=2: st_{t+1}[b]=(fL[1],fR[1]), st_{t+1}[b+1]=(fL[0],fR[0]); c=1: st_{t+1}[b]=(fL[0],fR[0]); slots>=spNext zero; else row t+1 all-zero", "Q-8 act_{Q_eq-1} implies term_{Q_eq-1}!=NONE else no accepting assignment -> Capacity EqScratch.rows", "Q-9 spNext<=W_eq"],
        "derived": "exactly one terminal row = last active row; eq=sum_t [term_t=EMPTY]",
        "result_append": "only after valid terminal: appendData(1,1,0,0^16,eq,[]) -> {Data,D0}; CR0={PrimitiveResult,1,{Data,D0}}",
        "capacity_order": ["EqScratch.rows", "Data D0+1<=D"],
        "excluded": ["same-reference shortcut", "pair memoization", "budget to false", "budget to semantic error"],
        "lemmas": ["MISMATCH implies unequal", "EMPTY implies structural equality", "repeated DAG occurrences revisited; rows needed <= ufd(L), exponential possible", "sp <= 1+depth(top) <= D0+K0 < W_eq"]
      }
    }
  },
  "fs_masks": {"NONE": "FrameSlots and depth preserved", "R0": "FS0 replaced only", "POP": "FS(i)'=FS(i+1) for i<depth-1; FS(depth-1)'=0; depth'=depth-1", "PUSH": "FS0'=CR0.pendingFrame; FS(i+1)'=FSi for i<depth; depth'=depth+1"},
  "cr_masks": {"W1": "CR0 written, controlLength'=1, inactive columns zero", "W2": "CR0=Push, CR1=Eval, controlLength'=2", "UNBOX": "CRi'=CR(i+1) for i<n-1; CR(n-1)'=0; controlLength'=n-1", "CLEAR": "CRi'=0 for i<n; controlLength'=0"},
  "p4_status": "masks implement full39 relations; P4 component wiring open",
  "rules": {
    "A080": {"charge": 0, "aux": "none", "appends": [], "fs": "NONE", "cr": "CLEAR", "root": "outcomeTag=Done; outcomeValue=A:CR0.valueRef; outcomeError=0; fuel preserved", "capacity": [], "note": "post-image not canonical (F3)"},
    "A082": {"charge": 0, "aux": "none", "appends": [], "fs": "NONE", "cr": "CLEAR", "root": "outcomeTag=Rejected; outcomeError=CR0.errorCode; fuel preserved", "capacity": []},
    "A084": {"charge": 0, "aux": "none", "appends": [], "fs": "NONE", "cr": "CLEAR all i<n", "root": "outcomeTag=Exhausted; fuel preserved", "capacity": []},
    "A102": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Return{valueRef=A:Expressions[X].valueRef}", "root": "fuel-1", "capacity": []},
    "A108": {"charge": 1, "aux": "EnvGetScan", "selector": "resFound=1", "appends": [], "fs": "NONE", "cr": "W1 Return{valueRef=A:resValue}", "root": "fuel-1", "capacity": []},
    "A111": {"charge": 1, "aux": "EnvGetScan", "selector": "resFound=0", "appends": [], "fs": "NONE", "cr": "W1 Error{3}", "root": "fuel-1", "capacity": []},
    "A114": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Let{nameId,expr0=ch(1),savedEnv=A:Rho},environment=A:Rho}; CR1=Eval(ch(0))", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A117": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Left{binaryOp,expr0=ch(1),savedEnv=A:Rho},environment=A:Rho}; CR1=Eval(ch(0))", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A120": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Unary{unaryOp},pendingFrame.savedEnv=0,environment=A:Rho}; CR1=Eval(ch(0))", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A123": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=If{expr0=ch(1),expr1=ch(2),savedEnv=A:Rho},environment=A:Rho}; CR1=Eval(ch(0))", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A126": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Match{armSlice=A:E.armSlice,savedEnv=A:Rho},environment=A:Rho}; CR1=Eval(ch(0))", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A129": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Invoke{nameId,valueSlice=(0,0)}", "root": "fuel-1", "capacity": []},
    "A131": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Args{nameId,completedSlice=(0,0),remainingExprSlice=MD7 tail,savedEnv=A:Rho},environment=A:Rho}; CR1=Eval(ch(0))", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A135": {"charge": 1, "aux": "none", "selector": "depth<stackLimit", "appends": [], "fs": "PUSH", "cr": "UNBOX", "root": "environment=A:CR0.environment; fuel-1", "capacity": ["FrameSlots depth+1<=S (P6.S), never error4"]},
    "A140": {"charge": 1, "aux": "none", "selector": "depth>=stackLimit", "appends": [], "fs": "NONE", "cr": "CR0=Error{4}; CRi=0 for 1<=i<n; controlLength=1", "root": "fuel-1", "capacity": []},
    "A145": {"charge": 1, "aux": "none", "appends": [{"table": "EnvCells", "row": "E0", "value": "{FS0.nameId, A:CR0.valueRef, A:FS0.savedEnv}"}], "fs": "R0 FS0={Scope,savedEnv=A:FS0.savedEnv}", "cr": "W1 Eval{A:FS0.expr0}", "root": "environment={1,E0}; fuel-1", "capacity": ["EnvCells E0+1<=E"]},
    "A149": {"charge": 1, "aux": "none", "appends": [], "fs": "R0 FS0={Right,binaryOp=FS0.binaryOp,valueRef=A:CR0.valueRef}", "cr": "W1 Eval{A:FS0.expr0}", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": []},
    "A153": {"charge": 1, "aux": "EqScratch iff binaryOp=Eq", "selector": {"Add|Sub|Mul": "sL=0 and sR=0 ? ARITH : ERR11", "Lt": "sL=0 and sR=0 ? LT : ERR11", "Eq": "EQ", "Cons": "sL=0 and sR=2 ? CONS : ERR11", "Branch": "sL=3 and sR=3 ? BRANCH : ERR11"}, "branches": {"ARITH_OK": {"appends": [{"table": "Data", "value": "{0,0,0,LE16(r),0,[]}"}], "result": "{Data,D0}", "capacity": ["Data D0+1<=D"]}, "ARITH_OVF": {"appends": [], "result": "Err(10)"}, "LT": {"appends": [{"table": "Data", "value": "{1,1,0,0^16,beta,[]}"}], "result": "{Data,D0}", "capacity": ["Data D0+1<=D"]}, "EQ": {"appends": [{"table": "Data", "value": "{1,1,0,0^16,eq,[]}"}], "result": "{Data,D0}", "capacity": ["EqScratch.rows terminal within Q_eq", "Data D0+1<=D"]}, "CONS": {"appends": [{"table": "Data", "value": "{2,2,1,0^16,0,[A:L,A:R]}"}, {"table": "DataFields", "rows": "DF0,DF0+1"}], "result": "{Data,D0}", "capacity": ["Data D0+1<=D", "DataFields DF0+2<=DF"]}, "BRANCH": {"appends": [{"table": "Data", "value": "{2,3,3,0^16,0,[A:L,A:R]}"}, {"table": "DataFields", "rows": "DF0,DF0+1"}], "result": "{Data,D0}", "capacity": ["Data D0+1<=D", "DataFields DF0+2<=DF"]}, "ERR11": {"appends": [], "result": "Err(11)"}}, "relations": "a+b=r+2^128*cy; a-b=r-2^128*beta; a*b=r+2^128*h, h<2^128; ok iff cy=0/beta=0/h=0; Lt=Bool(beta)", "fs": "POP", "cr": "W1 PrimitiveResult{primitiveIsOk,valueRef or errorCode}", "root": "fuel-1"},
    "A157": {"charge": 1, "aux": "none", "branches": {"LEAF": {"appends": [{"table": "Data", "value": "{2,3,2,0^16,0,[A:CR0.valueRef]}"}, {"table": "DataFields", "rows": "DF0"}], "capacity": ["Data D0+1<=D", "DataFields DF0+1<=DF"]}, "NOT": {"appends": [{"table": "Data", "value": "{1,1,0,0^16,not b,[]}"}], "capacity": ["Data D0+1<=D"]}, "ERR11": {"appends": []}}, "fs": "POP", "cr": "W1 PrimitiveResult", "root": "fuel-1"},
    "A161": {"charge": 1, "aux": "none", "appends": [], "fs": "POP", "cr": "W1 Eval{A:FS0.expr0}", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": []},
    "A165": {"charge": 1, "aux": "none", "appends": [], "fs": "POP", "cr": "W1 Eval{A:FS0.expr1}", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": []},
    "A169": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Error{5}", "root": "fuel-1; no restore", "capacity": []},
    "A173": {"charge": 1, "aux": "none", "appends": [], "fs": "POP", "cr": "W1 Cases{valueRef=A:CR0.valueRef,armSlice=A:FS0.armSlice,environment=A:FS0.savedEnv}", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": []},
    "A177": {"charge": 1, "aux": "none", "selector": "EXT iff m>0 and s0+m=V0", "branches": {"EXT": {"slots": "V0:=A:CR0.valueRef", "newDone": "(s0,m+1)", "delta_V": "1"}, "COPY": {"slots": "V0+i:=ValueSlots[s0+i] for i<m (guarded); V0+m:=A:CR0.valueRef", "newDone": "(V0,m+1)", "delta_V": "m+1"}}, "fs": "R0 FS0={Args,nameId,completedSlice=newDone,remainingExprSlice=MD7 tail,savedEnv=A:FS0.savedEnv}", "cr": "W1 Eval{ExprChildren[s.start].expression}", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": ["ValueSlots V0+delta_V<=V"]},
    "A181": {"charge": 1, "aux": "none", "selector": "same as A177", "fs": "POP", "cr": "W1 Invoke{nameId,valueSlice=newDone}", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": ["ValueSlots V0+delta_V<=V"]},
    "A185": {"charge": 1, "aux": "none", "appends": [], "fs": "POP", "cr": "post-relation CR0=Return{A:CR0.valueRef}; MD2 UNSELECTED", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": []},
    "A190": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Error{6}", "root": "fuel-1; no restore", "capacity": []},
    "A195": {"charge": 1, "aux": "none", "appends": [], "fs": "POP", "cr": "post-relation CR0=Return{A:CR0.valueRef}; MD2 UNSELECTED", "root": "environment=A:FS0.savedEnv; fuel-1", "capacity": []},
    "A200": {"charge": 1, "aux": "FindScan", "appends": [], "fs": "NONE", "cr": "W1 Enter{functionPresent=outPresent,functionId=outId,valueSlice=A:CR0.valueSlice}", "root": "fuel-1", "capacity": []},
    "A202": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Error{7}", "root": "fuel-1", "capacity": []},
    "A204": {"charge": 1, "aux": "BindPreflight", "branches": {"SUCCESS": {"appends": "k EnvCells per BindPreflight.commit_success", "bindings": "k>0?{1,E0+k-1}:{0,0}", "bindingsPresent": 1, "capacity": ["EnvCells E0+k<=E"]}, "FAILURE": {"appends": [], "bindings": "{0,0}", "bindingsPresent": 0, "capacity": []}}, "fs": "NONE", "cr": "W1 Bound{bindingsPresent,bindings,typeId=retType,exprId=A:body}", "root": "fuel-1"},
    "A206": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Error{8}", "root": "fuel-1", "capacity": []},
    "A208": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Return{typeId=CR0.typeId,savedEnv=A:Rho},environment=A:CR0.bindings}; CR1=Eval{A:CR0.exprId}", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A212": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Error{9}", "root": "fuel-1", "capacity": []},
    "A214": {"charge": 1, "aux": "none", "reads": "Arms[s.start] guarded; Patterns[patternId]; node(v) unless Var; fields only after shape and binder tests", "branches": {"Var": {"cells": ["{name0, A:v, rho}"], "bindings": "{1,E0}"}, "Nil": {"guard": "kind=2 and typeId=2 and ctorId=0", "cells": [], "bindings": "rho exact alias (deterministic selection, not uniqueness)"}, "Cons": {"guard": "typeId=2 and ctorId=1 and name0!=name1", "cells": ["{name0, A:fld(v,0), rho}", "{name1, A:fld(v,1), {1,E0}}"], "bindings": "{1,E0+1}"}, "Leaf": {"guard": "typeId=3 and ctorId=2", "cells": ["{name0, A:fld(v,0), rho}"], "bindings": "{1,E0}"}, "Branch": {"guard": "typeId=3 and ctorId=3 and name0!=name1", "cells": ["{name0, A:fld(v,0), rho}", "{name1, A:fld(v,1), {1,E0}}"], "bindings": "{1,E0+1}"}, "Miss": {"cells": [], "bindingsPresent": 0, "bindings": "{0,0}"}}, "rho": "A:CR0.environment", "fs": "NONE", "cr": "W1 CaseResult{bindingsPresent,bindings,valueRef=A:v,exprId=A:Arms[s.start].bodyExprId,armSlice=MD7 tail,environment=rho}", "root": "fuel-1", "capacity": ["EnvCells E0+cells<=E on match only"]},
    "A217": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Cases{A:CR0.valueRef,A:CR0.armSlice,A:CR0.environment}", "root": "fuel-1", "capacity": []},
    "A220": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W2 CR0=Push{pendingFrame=Scope{savedEnv=A:CR0.environment},environment=A:CR0.bindings}; CR1=Eval{A:CR0.exprId}", "root": "fuel-1", "capacity": ["ControlRows 2<=C"]},
    "A223": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Return{A:CR0.valueRef}", "root": "fuel-1", "capacity": []},
    "A225": {"charge": 1, "aux": "none", "appends": [], "fs": "NONE", "cr": "W1 Error{CR0.errorCode}", "root": "fuel-1", "capacity": []}
  },
  "empty_append_rules": ["A080", "A082", "A084", "A102", "A108", "A111", "A114", "A117", "A120", "A123", "A126", "A129", "A131", "A135", "A140", "A149", "A161", "A165", "A169", "A173", "A185", "A190", "A195", "A200", "A202", "A206", "A208", "A212", "A217", "A220", "A223", "A225"],
  "preservation": "every unlisted Root column, FS/CR row, arena row, active length and ProgramBlock cell preserved",
  "bounds": {"generic_per_step": {"Data": "1", "DataFields": "2", "EnvCells": "max(2,k_max), k_max=max active parameterSlice.len <= P", "ValueSlots": "V0+1", "controlLength": "2", "depth": "depth+1"}, "a_max_bound": "only under unselected INV-ARGS: completed.len+remaining.len+1<=a_max per Args frame; not assumed", "generic_envelope_T_c": {"D": "D_start+T_c", "DF": "DF_start+2*T_c", "E": "E_start+T_c*max(2,k_max)", "V": "V_end+1 <= (V_start+1)*2^T_c"}, "envelope_is_admission": false, "r1_static_admission_claim": "WITHDRAWN", "exact_need": "trace sum, refused per step at first overflow (F5)", "aux_need_per_step": {"EnvGetScan": "<=max(1,E0)", "FindScan": "activeFunctions", "BindPreflight": "k", "EqScratch": "<=ufd(L)<=2^(D0+K0)-1"}},
  "quantities_separate": ["logical work (fuel; helpers 0)", "StateBlock occupancy", "copy volume", "unfolded boundary envelope (F3)", "Q_eq and aux rows", "DUST/fees", "proof rows and circuit k"],
  "examples": ["EX1 alias, preserved root, A114/A135/A102/A145/A120/A135/A108/A157", "EX2 shared Eq true, Q_eq last-row boundary, wrong activity trace rejected", "EX3 mismatches with own tables", "EX4 symbolic V0 argument slices, COPY/COPY/COPY/EXT", "EX5 BindPreflight dup, type, success, zero, length", "EX6 A214 Nil alias, Cons shadow, capacity", "EX7 EnvGetScan full-chain miss and E0=0", "EX8 FindScan first, row0, none", "EX9 full-capacity branches"],
  "future_checks": {"run": false, "proved": false, "positive": ["decode(post)=SNAPSHOT8 allocating rules", "ok iff bind!=None", "A214 table=matching", "eq=Value ==", "FindScan=iter().find", "EnvGetScan=map.get", "aux columns functions of program,pre,dims"], "negative": ["dropped stack pair", "non-forced act", "outPresent=0 with match", "forged dup", "read pair 0 after type failure", "cells on failure branch", "COPY write below V0", "EXT with m=0", "Q_eq exhaustion as MISMATCH", "Err10 refused at D0=D", "Nil producing cell", "duplicate Names rows"]},
  "repair_dispositions": {"P3-1": "FIXED same-row ABSENT, R_env=max(1,E)", "P3-2": "FIXED two-sided least-match fold", "P3-3": "FIXED act recurrence, forced shifts, terminal, enforced fieldLength, refusal order", "P3-4": "FIXED P6.C/P6.S in preflight", "P3-5": "FIXED DeltaV<=V0+1, static admission withdrawn", "P3-6": "FIXED INV-TY-D/K preconditions, E1-E4 enforced", "P3-7": "FIXED D-NAMES gating", "P3-8": "FIXED A120 pending savedEnv zero, Push env Rho", "P3-9": "FIXED Nil alias deterministic selection", "P3-10": "FIXED diagnostic removed, masks derived", "P3-11": "FIXED u8[16], EX1 A108, EX3 own tables", "Muse_pins": "FIXED A120, A185/A195 post-relation, EnvGetScan rows, X1/X2, bound domains", "X1": "SELECTED and PROPOSED four schemas, labels, packing, enables", "X2": "SELECTED Q_eq symbolic dimension, refusal name EqScratch.rows, not statically excludable"},
  "out_of_scope": ["P2 obligations beyond four aux components", "P4 component wiring", "P5 global dispatch and preservation wiring", "G4 guard primitives", "F4", "MD2", "StepInv and chaining", "Constant typing enforcement", "D-NAMES enforcement", "native statement binding and program/pre/post labels", "O-ARITH lowering", "L05 numeric admission", "B", "general ADTs", "financial", "proof", "ledger", "Midnight stagenet", "U0-U7/M1-M17/MC/SP/G", "C4 A102 compile gate"],
  "requires": ["fresh independent designated-owner security audit of these exact bytes", "fresh Muse normal audit of these exact bytes", "Root adoption record", "full39 conditions 1-10 remain binding", "any byte change voids reviews for changed bytes"]
}
```

PROPOSED_COMPLETE_P3_SOURCE_REVISION_UNADOPTED_UNRUN