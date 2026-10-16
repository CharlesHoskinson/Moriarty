# PROPOSED `sherlock-k-a-aux/2`: arithmetic and fuel witness profile for CandidateA

**Status.** This is a source-design candidate. I wrote it as the designated security and physical-design owner (Opus 5.5, high), acting **as author only**. It is unadopted.

It is not any of the following:
- a vote or an adoption;
- a circuit edit or an emitter acceptance criterion;
- an execution record;
- a runtime, resource, numeric, operator, configuration, binding or circuit grant.

How it was produced:
- I used only the supplied texts.
- I ran no tool, parser, model, SDK, native action, test or evaluator.
- I recomputed no hash. Every identity here is as supplied.
- Every specimen and lemma is a paper derivation. All K, Quint, Clean and circuit checks are **UNRUN**.

**Limits that still hold:**
- Native authority stays 0.
- The old 90s/30s caps, counters and terminal failures are unchanged.
- C4 is unchanged: no dependent circuit source edit happens before admitted A102 compilation, and a compile failure means registration revert.
- Full39 conditions 1–10 and F1–F7 stay binding.
- The adopted P3 conditions 1–8 and errata R2-1/3/4/5/6 stay binding, together with the preserved Muse R2-4 dissent.
- No security decision is delegated to GPT.

**What this candidate resolves (source level only):**
- **P3 R2-2.** `aux/1` lacked the u128 remainder, carry, borrow and high witnesses.
- **P4 exact 16-byte fuel decrement.** The fuel decrement is now a profile relation.
- **The `remainingFuel≠0` conjunct of `cg`.** Every aux/1 enable uses it, and aux/1 left it unrealized.

---

## 0. Fixed inputs, scope and what is unchanged

**Unchanged:**
- The full39 P1 map (`6823badb…22c2`).
- The R5 rule facts (`122289ad…b0a5`).
- The seven frozen L4 files.
- The Rust and K sources (`e1bb1098…181b`, `4b4bb364…317f`, `b3746ff5…1d41`).
- The adopted P3 design and catalog bytes (`b2d6fda4…91eb`, `3f33a0dc…b214`).
- The literal-step fragment (`b5d2ca73…0839`).

**The aux/1 profile is preserved exactly.** Its four schemas, packing, enables, labels, dimensions, refusal names and lemmas (EnvGetScan, FindScan, BindPreflight, EqScratch) are carried unchanged as the *prefix* of aux/2.

**aux/2 is a new, separately versioned, PROPOSED profile.**
- An aux/1 image is not an aux/2 image, and the reverse is also not true.
- Profile identity and dimensions must be bound into layout identity and the statement before any native use. That is P3 condition 6 and L04, and it remains open.

**In scope:**
1. A153 with `binaryOp ∈ {Add, Sub, Mul, Lt}`: the success paths, the Err10 path and the Err11 type-rejection path. This includes operand fetch, sort test, limb witnesses, branch selection and the commit relation.
2. The fuel conjunct of `cg`: a zero probe for all 39 events.
3. The exact 16-byte charged decrement for all 36 charged rules, and exact preservation for the 3 free publications.

**Out of scope and kept separate:**
- **Eq** stays in aux/1 EqScratch, unchanged.
- **Boolean operations stay separate:**
  - A157 Not/Leaf;
  - the A161/A165/A169 Bool tests;
  - Cons/Branch sort reads and Err11.
- **Open programme items:**
  - P4 full FS/CR spine wiring;
  - P5 global exactly-one dispatch and preservation wiring;
  - StepInv and chaining;
  - F4 and MD2;
  - L04 and L05;
  - the G4 primitive implementations;
  - B, general ADTs and financial behaviour.

---

## 1. Qualified interface facts

### 1.1 Facts read from `literal_step.rs` (`b5d2ca73…0839`), by reading only

**Instruction variants it emits:**
- `CondSelect{bit,a,b,output}`
- `ConstrainEq{a,b}`
- `ConstrainToBoolean{val}`
- `ConstrainBits{val,bits}`
- `LessThan{a,b,bits:128,output}`
- `TestEq{a,b,output}`
- `Add{a,b,output}`
- `Mul{a,b,output}`

**Other names it uses:** `Operand::Variable(Identifier)`, `immediate(u128)`, and `emit_enabled_fetch_block(&TableShape, &EnabledFetchInputs{enable, active_rows, index, rows}, scope)`. The last one returns `{instructions, selected[].value}`.

**What is not shown.**
- No subtraction, division or bit-decomposition instruction appears.
- The definitions all live in `super::*`, which was **not supplied**.
- The semantics below are therefore *inferred from names and usage*. They are UNVERIFIED.

**Idioms the fragment uses:**
- Gating: `gated_eq(en,a,b) := ConstrainEq(CondSelect(en,a,b), b)`. When `en=1` this forces `a=b`; when `en=0` it is vacuous.
- AND: `CondSelect(x, y, 0)`.
- A zero test is `TestEq(x,0)` followed by an *added* `ConstrainToBoolean`.
- Every image cell gets an unconditional range constraint, even when disabled.

**The A102 fragment already contains a byte-wise fuel decrement:**
- `borrow₀ = enable`, `next = borrow ∧ [pre_i=0]` and `pre_i + 256·next = post_i + borrow` (gated).
- A final ungated `borrow₁₆ = 0`.
- An all-zero test gated to 0.

Its intermediates are emitter temporaries, not supplied witness columns. Under C4 that fragment is **not edited**. Equivalence to FuelDec (§8) is integration obligation **IO-3**.

**Header offsets used by the fragment.** It uses `program.root[5]` (activeExpressions) and `program.root[14]` (activeConstants). Both are consistent with the L4 ProgramHeader order.

### 1.2 Unknowns, stated as explicit G4 requirements (not assumed)

| ID | Requirement this design needs | Status |
|---|---|---|
| G4-FIELD | The native field characteristic satisfies **p > 2^20**, derived in §10. Nothing assumes p > 2^128, and no 128-bit value is ever packed into one field element. | UNKNOWN; must be established for the pinned native field |
| G4-ISA-1 | `Add`/`Mul` constrain the output to the field sum or product. They are not unconstrained hints. | UNKNOWN |
| G4-ISA-2 | `TestEq(a,b,o)` constrains `o = [a=b]`, sound and complete. The added `ConstrainToBoolean` is kept anyway. | UNKNOWN; the A102 zero test depends on the same requirement |
| G4-ISA-3 | `CondSelect(bit,a,b)` yields `bit?a:b` for Boolean `bit`. | Inferred from usage |
| G4-ISA-4 | `ConstrainBits(v,w)` forces the canonical representative of `v` into `[0,2^w)` for `w ≤ 12`. `ConstrainToBoolean` forces `{0,1}`. `ConstrainEq` is field equality. | UNKNOWN |
| G4-FETCH | The space-polymorphic guarded node fetch (E1) exists. It selects Data or Constants by `space`, guards `index < (space?D0:K0)`, and makes its output fully constrained. The existing `emit_enabled_fetch_block` reads one table only. | OPEN (P3 E1) |
| G4-BYTE | Serialized multi-byte lanes are reconstituted into logical carriers. This is the same unwaived obligation `literal_step.rs` names for "byte reconstitution". | OPEN |
| G4-APPEND | The dynamic-index write `Data[D0]` used by `appendData`. | OPEN (P3/G4) |

**`LessThan{bits:128}` is not used anywhere in aux/2.** Its soundness in a field of unknown size is not established.

---

## 2. Integer definitions used by the specimens (honest-witness functions, not constraints)

**Byte encoding.** A UInt128 `x` is `LE16(x) = (x_0..x_15)` with `x = Σ x_i·256^i` and `x_i ∈ [0,255]`. Hex lists are written byte 0 first, e.g. `[FF,00×15]` = 255.

**ADD** (`a+b`). Set `c_0 = 0`. For `i = 0..15`:
- `r_i = (a_i+b_i+c_i) mod 256`;
- `c_{i+1} = ⌊(a_i+b_i+c_i)/256⌋ ∈ {0,1}`.

Rust `checked_add` and K `uiChecked` give `Ok(r)` iff `c_16 = 0`, and `Err(10)` otherwise.

**SUB** (`a−b`, also used for Lt). Set `bo_0 = 0`. For `i = 0..15`:
- `r_i = (a_i−b_i−bo_i) mod 256`;
- `bo_{i+1} = [a_i−b_i−bo_i < 0] ∈ {0,1}`.

`bo_16 = [a<b]`. Sub gives `Ok(r)` iff `bo_16 = 0`, and `Err(10)` otherwise. Lt gives `Ok(Bool(bo_16))`.

**MUL** (`a·b`, full 256-bit). Define `S_k = Σ_{i+j=k, 0≤i,j≤15} a_i·b_j` for `k = 0..31`; `S_31 = 0`. Set `c_0 = 0`. For each k:
- `p_k = (S_k+c_k) mod 256`;
- `c_{k+1} = ⌊(S_k+c_k)/256⌋`.

Then `r = p_0..p_15` and `h = p_16..p_31`, with `c_32 = 0`. Rust `checked_mul` and K give `Ok(r)` iff `h = 0`, and `Err(10)` otherwise.

**Fuel** (`f−1`, for `f ≥ 1`). Set `bw_0 = 1` and `bw_{i+1} = bw_i·[f_i=0]`. Then `post_i = f_i − bw_i + 256·bw_{i+1}`.

---

## 3. Normal specimens (derived from the Rust/K semantics before any constraint is stated)

All values are exact hand derivations from §2, and none of them has been run.

**Append notation.** "append UInt(x)" means `appendData(0,0,0,LE16(x),0,[])`. "append Bool(v)" means `appendData(1,1,0,0^16,v,[])`. Err paths append nothing and write `CR0'={PrimitiveResult, ok=0, errorCode}`.

### 3.1 Add

| ID | a | b | carries | r | cy=c_16 | Rust/K result |
|---|---|---|---|---|---|---|
| A-1 | `[00×16]` (UInt0) | `[00×16]` | all 0 | `[00×16]` | 0 | Ok(UInt 0), append UInt(0) |
| A-2 | `[FF×16]` (max) | `[00×16]` | all 0 | `[FF×16]` | 0 | Ok(max) |
| A-3 | `[FF,00×15]` (255) | `[01,00×15]` | c_1=1, c_2..16=0 | `[00,01,00×14]` (256) | 0 | Ok(256): carry propagation stops |
| A-4 | max | `[01,00×15]` | c_1..c_16=1 | `[00×16]` (constrained, not stored) | 1 | **Err(10)**, no append |
| A-5 | max | max | c_1..c_16=1 | `[FE,FF×15]` (constrained) | 1 | **Err(10)** |

### 3.2 Sub

| ID | a | b | borrows | r | bo_16 | result |
|---|---|---|---|---|---|---|
| S-1 | UInt1 `[01,00×15]` | UInt1 | all 0 | `[00×16]` | 0 | Ok(0) |
| S-2 | UInt0 | UInt1 | bo_1..bo_16=1 | `[FF×16]` (constrained) | 1 | **Err(10)** |
| S-3 | `[00,01,00×14]` (256) | `[01,00×15]` | bo_1=1, rest 0 | `[FF,00×15]` (255) | 0 | Ok(255): borrow propagation stops |
| S-4 | max | max | all 0 | `[00×16]` | 0 | Ok(0) |

### 3.3 Lt (Sub chain, remainder constrained but never stored)

| ID | a | b | borrows | r (not stored) | bo_16 | result |
|---|---|---|---|---|---|---|
| L-1 | 3 `[03,00×15]` | 5 `[05,00×15]` | bo_1..bo_16=1 | `[FE,FF×15]` | 1 | Ok(Bool true), append Bool(1) |
| L-2 | 5 | 3 | all 0 | `[02,00×15]` | 0 | Ok(Bool false) |
| L-3 | 7 | 7 | all 0 | `[00×16]` | 0 | Ok(Bool false): equality |
| L-4 | 2^120 `[00×15,01]` | 255 `[FF,00×15]` | bo_1..bo_15=1, bo_16=0 | `[01,FF×14,00]` | 0 | Ok(Bool false): the high byte decides |
| L-5 | 255 | 2^120 | bo_1..bo_15=0, bo_16=1 | `[FF,00×14,FF]` | 1 | Ok(Bool true) |

### 3.4 Mul

| ID | a | b | carries (nonzero only) | r | h | result |
|---|---|---|---|---|---|---|
| M-1 | UInt0 | max | none | 0 | 0 | Ok(0) |
| M-2 | UInt1 | max | none (S_k=255 for k≤15) | `[FF×16]` | 0 | Ok(max) |
| M-3 | 255 | 255 | c_1=254 | `[01,FE,00×14]` (65025) | 0 | Ok(65025) |
| M-4 | 2^127 `[00×15,80]` | 2 `[02,00×15]` | c_16=1 | `[00×16]` | `[01,00×15]` | **Err(10)**: the carry crosses the 128-bit boundary |
| M-5 | 2^64 `[00×8,01,00×7]` | 2^64 | none | 0 | `[01,00×15]` | **Err(10)** |
| M-6 | max | max | c_k = 255k−1 (1≤k≤16); c_k = 255(32−k) (17≤k≤31) | `[01,00×15]` | `[FE,FF×15]` | **Err(10)** |

**M-6 derivation, the full 128-bit high/low split.** It uses `65025 = 254·256+1`.
- **k=0:** `T = 65025`, so `p_0 = 1` and `c_1 = 254`.
- **1≤k≤15:** `T_k = 65025(k+1) + 255k−1 = 256·(255k+254)`, so `p_k = 0` and `c_{k+1} = 255(k+1)−1`.
  - At k=15: `T = 1,040,400+3,824 = 1,044,224 = 256·4079`.
- **k=16:** `T = 65025·15+4079 = 979,454 = 254 + 256·3825`, so `p_16 = FE`.
- **17≤k≤30:** `T_k = 65025(31−k) + 255(32−k) = 256·255(31−k) + 255`, so `p_k = FF` and `c_{k+1} = 255(31−k)`.
- **k=31:** `T = c_31 = 255`, so `p_31 = FF` and `c_32 = 0`.
- **Totals:** `hSum = 254+15·255 = 4079`. The maximum carry is **4079**.
- **Check:** `(2^128−1)^2 = 2^128·(2^128−2) + 1`, which matches r and h.

### 3.5 Type rejection and separation from Eq and Bool operations

| ID | Step | Expected |
|---|---|---|
| T-1 | Add(UInt 1, Bool true) | Err(11). `uu=0`, every limb block is all-zero, no append. |
| T-2 | Lt(List Nil (Constant), UInt 3) | Err(11), same shape. |
| T-3 | Eq(UInt 1, Bool true) | ArithScratch is all-zero (`en_ar=0`). aux/1 EqScratch gives MISMATCH, so Ok(Bool false), not Err11. |
| T-4 | Cons(UInt 1, Nil) | ArithScratch is all-zero. The Cons path is outside aux/2. |
| P-1 | Add(L={C,0}=Constants[0] UInt 7, R={D,2}=Data[2] UInt 5) | `opL={0,0}` and `opR={1,2}`. nL is fetched from Constants and nR from Data. `r=[0C,00×15]`, Ok(12), append UInt(12) at Data[D0], `CR0'={PrimitiveResult,1,{Data,D0}}`. |

### 3.6 Fuel

**Notation.** `f` is `pre.remainingFuel` and `f'` is `post.remainingFuel`. Each row is checked against Rust `step` (lines 37–73) and K lines 80–86.

| ID | Rule | f | Probe `z` / `isZero` | FuelDec `bw_1..16` | f' |
|---|---|---|---|---|---|
| F-1 | A102, charged | `[01,00×15]` | z=`[0,1×15]`, isZero=0 | all 0 | `[00×16]` (1 → 0) |
| F-2 | A111, Err3 charged | `[00,00,01,00×13]` (65536) | z=`[1,1,0,1×13]`, isZero=0 | bw_1=bw_2=1, rest 0 | `[FF,FF,00×14]` (65535): multi-byte borrow |
| F-3 | A153 ERR10, charged | `[00×15,01]` (2^120) | z_0..14=1, z_15=0 | bw_1..bw_15=1, bw_16=0 | `[FF×15,00]` |
| F-4 | A135, charged | `[FF×16]` | all 0 | all 0 | `[FE,FF×15]` |
| F-5a | After F-1, control `Return`, depth 0 → **A080** (free) | 0 | isZero=1 | all 0 (disabled) | 0 preserved, Done |
| F-5b | After F-1, control `Eval` → **A084** (free) | 0 | isZero=1 | all 0 | 0 preserved, Exhausted |
| F-6 | **A082** (free), Error(4), at f=5 | `[05,00×15]` | isZero=0 | all 0 | preserved, Rejected(4) |
| F-7 | **A082** at f=0 | 0 | isZero=1 | all 0 | preserved; publication precedes exhaustion |
| F-8 | A225 Error(code) propagation, charged | 3 | isZero=0 | all 0 | 2 |

---

## 4. Profile `sherlock-k-a-aux/2`

### 4.1 Placement, operand and labels

**Operand.** `StepOperandsA` is extended to `{program, pre, suppliedPost, suppliedAux2}`.
- AuxBlock2 is a step-local private witness.
- It is not an observation, not part of ObservedRecordA, and not StateBlock (INV-AUX).

**Labels.**
- No aux column is public.
- The labels and native binding for ProgramBlock, pre and post remain L04-open.
- I claim no privacy, hiding or binding.

**Dimensions.**
- aux/2 adds **no new dimension**. Every new record has fixed shape.
- NODE and VR widths use the existing symbolic L4 widths. Only the language bit widths are numeric: 8-bit bytes, a 12-bit carry, 1-bit flags.
- No new refusal name is introduced.

### 4.2 Packing (fixed shape, disabled ⇒ all-zero)

```text
AuxBlock2 = AuxBlock1 (exactly aux/1: EnvGetScan ‖ FindScan ‖ BindPreflight ‖ EqScratch, unchanged)
          ‖ ArithScratch ‖ FuelProbe ‖ FuelDec
```

**Packing rules.**
- Columns are flattened in the listed order by the L4 record rules. VR is `(space,index)`. NODE is `(kind,typeId,ctorId,uintBytes[0..16),boolValue,fieldStart,fieldLength)`.
- Arrays are in ascending index order.
- The serialized lane widths follow the L4 `width_rules`, for example `unsigned(12)` → 2 LE byte lanes with the high 4 bits zero.
- `packStepWitnessA` only orders and shape-checks. It decides nothing.
- No column is a truth or acceptance bit. Every column below is constrained to be a function of `(program, pre)`.

### 4.3 Records and columns

**ArithScratch** (A153, op ∈ {Add, Sub, Mul, Lt}). Logical lanes: 55 header + 32 + 32 + 65 = **184**.

| # | Column | Type | Defined by |
|---|---|---|---|
| H1 | `opL` | VR | ARH-1 |
| H2 | `opR` | VR | ARH-1 |
| H3 | `nL` | NODE | ARH-2 |
| H4 | `nR` | NODE | ARH-2 |
| H5–H8 | `selAdd, selSub, selMul, selLt` | Bool | ARH-0 |
| H9 | `uu` | Bool | ARH-3 |
| H10 | `ovf` | Bool | ARH-5 |
| H11 | `ltOut` | Bool | ARH-6 |
| ADD.1 | `ADD.r[0..16)` | u8 | ADD-i |
| ADD.2 | `ADD.c[1..17)` | Bool | ADD-i |
| SUB.1 | `SUB.r[0..16)` | u8 | SUB-i |
| SUB.2 | `SUB.bo[1..17)` | Bool | SUB-i |
| MUL.1 | `MUL.r[0..16)` | u8 | MUL-k |
| MUL.2 | `MUL.h[0..16)` | u8 | MUL-k |
| MUL.3 | `MUL.c[1..32)` | unsigned(12) | MUL-k |
| MUL.4 | `MUL.hSum` | unsigned(12) | MUL-HS |
| MUL.5 | `MUL.hZero` | Bool | MUL-HZ |

**FuelProbe** (32 lanes): `z[0..16)`: Bool, then `zp[1..17)`: Bool.

**FuelDec** (16 lanes): `bw[1..17)`: Bool.

The total new logical lane count is **232**.

**What is deliberately not a column.**
- The products `a_i·b_j`.
- The column sums `S_k`.
- The derived enables.
- The zero-test inverses.

These are emitter-internal instruction outputs of the pinned ops. The inverses are internal to `TestEq` under G4-ISA-2. They are deterministic functions of bound operands, not supplied witnesses.

---

## 5. Enables and dispatch predicates

These are derived from pre, never supplied. Their lowering through `TestEq`/`CondSelect` is P5-owned; the formulas are fixed here.

```text
isPend  := [pre.outcomeTag = 0]          nz_n := 1 − [pre.controlLength = 0]
en_evt  := isPend · nz_n                 (= 1 on every admitted ObservedRecordA, by O3/O4)
tRet0   := [CR0.tag = 1] · [depth = 0]   tErr := [CR0.tag = 9]      term := tRet0 + tErr
isZero  := FuelProbe.zp[16]
en_chg  := en_evt · (1 − term) · (1 − isZero)      -- this IS P3's cg; aux/2 realizes its fuel conjunct
en_A080 := en_evt · tRet0     en_A082 := en_evt · tErr     en_A084 := en_evt · (1 − term) · isZero
Partition: en_evt = en_A080 + en_A082 + en_A084 + en_chg   (Rust/K precedence: Done, Rejected, Exhausted, charged)

en_bin  := en_chg · [CR0.tag = 1] · (1 − [depth = 0]) · [FS0.tag = 2]   (depth>0 already implied by en_chg)
op      := FS0.binaryOp            (Add=0, Sub=1, Mul=2, Eq=3, Lt=4, Cons=5, Branch=6)
en_ar   := selAdd + selSub + selMul + selLt
enADD := selAdd·uu   enSUB := (selSub + selLt)·uu   enMUL := selMul·uu   enLT := selLt·uu
```

**Exclusivity.**
- `en_ar` and aux/1 `en_eq` are disjoint because they require different `binaryOp` values.
- `en_ar` is disjoint from `en_env`, `en_find` and `en_bind`, because those require `CR0.tag ≠ Return`.
- So the aux/1 "at most one rule-specific record" property still holds.
- FuelProbe and FuelDec are cross-cutting. They coexist with whichever rule-specific record is enabled.

---

## 6. ArithScratch relations

These are polynomial identities. The `[x=y]` terms are realized by `TestEq` (G4-ISA-2).

```text
ARH-0  selAdd = en_bin·[op=0]   selSub = en_bin·[op=1]   selMul = en_bin·[op=2]   selLt = en_bin·[op=4]
ARH-1  opL = en_ar · FS0.valueRef      opR = en_ar · CR0.valueRef        (componentwise; left = Right-frame value)
ARH-2  nL = en_ar ? node(opL) : 0      nR = en_ar ? node(opR) : 0
       node(r): Data[r.index] if r.space=1 else Constants[r.index]; guard r.index < (r.space ? D0 : K0)  (E1, G4-FETCH)
ARH-3  uu = en_ar · [nL.typeId = 0] · [nR.typeId = 0]       (sort := typeId under INV-TY, as P3 §2.1)
       a_i := nL.uintBytes[i],  b_i := nR.uintBytes[i]

ADD-i  (i=0..15, c_0:=0)   enADD · (a_i + b_i + ADD.c_i − ADD.r_i − 256·ADD.c_{i+1}) = 0
SUB-i  (i=0..15, bo_0:=0)  enSUB · (a_i + 256·SUB.bo_{i+1} − b_i − SUB.bo_i − SUB.r_i) = 0
MUL-k  (k=0..31, c_0:=0, c_32:=0, p_k := MUL.r_k for k<16, MUL.h_{k−16} for k≥16,
        S_k := Σ_{i+j=k, 0≤i,j≤15} a_i·b_j)
                            enMUL · (S_k + MUL.c_k − p_k − 256·MUL.c_{k+1}) = 0
MUL-HS MUL.hSum = Σ_{j<16} MUL.h_j                       (unconditional, linear)
MUL-HZ MUL.hZero = enMUL · [MUL.hSum = 0]
ARH-5  ovf   = ADD.c_16 + selSub·SUB.bo_16 + enMUL − MUL.hZero
ARH-6  ltOut = selLt · SUB.bo_16

ZERO   for every column x of block X ∈ {ADD,SUB,MUL}: (1 − enX)·x = 0;
       for H1–H4: (1 − en_ar)·x = 0
RANGE  (unconditional) u8 columns: 8 bits; Bool columns: Boolean; MUL.c, MUL.hSum: 12 bits;
       NODE/VR lanes per L4 widths
```

**Why `ovf` is Boolean and implies `uu`.** Every disabled block is zero, so at most one summand of ARH-5 is nonzero. When it is nonzero it is in {0,1}. In particular, `enMUL − hZero ∈ {0,1}` because `hZero ≤ enMUL`.

**Lt constrains the remainder even though Lt never stores it.** `enSUB = 1` for Lt, so every SUB-i binds `SUB.r`.

---

## 7. A153 non-Eq branch selection, pre/post binding and commit

**Branches.** Each is a pure function of pre and the constrained aux, and their sum is `en_ar`.

```text
bERR11 := en_ar − uu            bERR10 := ovf
bLT    := enLT                  bOK    := uu − enLT − ovf
```

| Branch | Post relation (gated by branch) | StateBlock Δ | Capacity |
|---|---|---|---|
| bOK | `appendData(0,0,0,res,0,[])` at `D0`, where `res_i = ADD.r_i + selSub·SUB.r_i + MUL.r_i`; `activeData' = D0+1`; `CR0' = {tag 8 PrimitiveResult, primitiveIsOk 1, valueRef {1, D0}}`, inactive columns 0 | (1,0,0,0) | Data `D0+1 ≤ D` |
| bLT | `appendData(1,1,0,0^16, ltOut, [])`; the same CR0' | (1,0,0,0) | Data `D0+1 ≤ D` |
| bERR10 | no append, `activeData' = D0`; `CR0' = {8, primitiveIsOk 0, errorCode 10}` | 0 | none: accepted at `D0 = D` (P3 EX9) |
| bERR11 | no append; `CR0' = {8, 0, errorCode 11}` | 0 | none |

**On every branch:**
- POP (the P4 mask);
- `controlLength' = 1` (W1);
- `Root.environment` preserved;
- fuel handled by §8;
- every other cell preserved (P5).

**The order is Rust's.** The `(op, UInt, UInt)` match is tried first, and every other combination falls to `_ => Err(11)`, matching K lines 261–268. Inside the UInt case, the checked arithmetic gives Err10 or Ok.

**Overflow paths keep a fully constrained witness and append nothing (INV-NOFAIL).**

---

## 8. Fuel relations

```text
FP-1  z_i  = en_evt · [pre.remainingFuel_i = 0]                    (i=0..15)
FP-2  zp_{i+1} = zp_i · z_i,  zp_0 := en_evt;   isZero := zp_16
FD-1  bw_{i+1} = bw_i · z_i,  bw_0 := en_chg
FD-2  post.remainingFuel_i + bw_i = pre.remainingFuel_i + 256·bw_{i+1}      (i=0..15, every step image)
FD-3  bw_16 = 0                                                     (non-underflow, unconditional)
FD-4  envelope.charge = en_chg                                      (charge ∈ {0,1}; rule-tag binding is P5)
RANGE z, zp, bw Boolean; pre/post fuel bytes 8-bit (existing image ranges)
```

**How FD-2 covers every case at once.**

*Charged events.* Sum FD-2 over i with weight `256^i`. This gives `f' + bw_0 = f + 2^128·bw_16`, so `f' = f − 1`.

*Free and disabled events.* FuelDec is all-zero (`bw ≡ 0` because `bw_0 = en_chg = 0`), so FD-2 gives `f' = f` exactly. This covers A080, A082 and A084.

*Exhaustion semantics are unchanged.*
- A charged step at `f = 1` performs its transition and yields `f' = 0`.
- The *next* event is A080 or A082 if the control is terminal, and A084 otherwise.

*Consistency of FD-3.* FD-3 is redundant inside the domain: `en_chg ⇒ isZero = 0 ⇒` some byte is nonzero, which kills the borrow chain. It is enforced anyway, matching the A102 fragment's final `borrow = 0`.

**FuelProbe is enabled on all 39 events.** It is a dispatch input: without it A084 cannot be distinguished from a charged rule. **FuelDec is disabled (all-zero) on A080, A082 and A084.**

---

## 9. All 39 rules: fuel record status

| Rules | Charge | FuelProbe | FuelDec | Fuel effect | Semantic-failure branches still charged |
|---|---|---|---|---|---|
| A080 | 0 | on | **off** | preserve | — (Done) |
| A082 | 0 | on | **off** | preserve | — (Rejected(code), any u8) |
| A084 | 0 | on (`isZero=1` required) | **off** | preserve (0) | — (Exhausted) |
| A102, A114, A117, A120, A123, A126, A129, A131, A135, A145, A149, A161, A165, A173, A177, A181, A185, A195, A200, A208, A217, A220, A223 | 1 | on | on | `f−1` | — |
| A111 (Err3), A140 (Err4), A169 (Err5), A190 (Err6), A202 (Err7), A206 (Err8), A212 (Err9), A225 (Error code) | 1 | on | on | `f−1` | yes |
| A153 | 1 | on | on | `f−1` | ERR10 and ERR11, plus EQ false and the ARITH/LT/CONS/BRANCH successes |
| A157 | 1 | on | on | `f−1` | ERR11 |
| A204 | 1 | on | on | `f−1` | FAILURE (`bindingsPresent = 0`) |
| A214 | 1 | on | on | `f−1` | Miss |

**Counts.** There are 36 charged rows (23 + 8 + 5) and 3 free rows.

**Capacity refusals produce no post-image**, so there is no fuel effect and no StepRecord (INV-CAP).

---

## 10. Field-characteristic bound and carry sizing

**Approach.** Each gated relation is `E = 0 (mod p)` for a signed integer expression `E` with all inputs range-constrained. If `|E| < p`, the field equality is an integer equality.

| Relation | Carry (signed contribution) | Integer range of E | Required |
|---|---|---|---|
| ADD-i | `c ∈ {0,1}` (+) | `[−511, 511]` | p > 511 |
| SUB-i | borrow-out `−bo ∈ {−1,0}` | `[−511, 511]` | p > 511 |
| MUL-k | `c ∈ [0,4079]` honest, `[0,4095]` admitted | `S_k ≤ 65025·m_k ≤ 1,040,400`, with `m_k = min(k+1, 31−k)`, `S_31 = 0`; E ∈ `[−(255+256·4095), 1,040,400+4095]` = `[−1,048,575, 1,044,495]` ⊂ `(−2^20, 2^20)` | **p > 2^20** |
| MUL-HS | — | `[−4080, 4095]` | p > 4095 |
| FD-2 | `−bw ∈ {−1,0}` | `[−511, 256]` | p > 511 |
| Field values of `a_i·b_j ≤ 65025` and `S_k` | — | exact if p > 1,040,400 | subsumed |

**Carry sizing.** The honest maximum MUL carry is `c_{k+1} ≤ (1,040,400 + c_max)/256`, which gives `c_max ≤ 4080`. The value is attained at 4079 (M-6, `c_16`). The 12-bit range is therefore complete.

**The 12-bit range is also sound.** Under it, E is integer-exact, and base-256 digits with ranged carries are unique (Lemma L-2).

**Requirement: G4-FIELD is `p > 2^20`.** If the pinned field fails this, aux/2 is inapplicable as written. The nibble-limb alternative in §14 would then apply, and it needs new reviewed bytes.

---

## 11. Lowering map to pinned ops (requirement only; nothing emitted here)

| Relation form | Lowering |
|---|---|
| `[x=y]` | `TestEq` + `ConstrainToBoolean` (as `test_zero` in literal_step) |
| `u·v` (Boolean AND, gated product) | `CondSelect(u, v, 0)` or `Mul` |
| gated equation `en·(L−R) = 0` | `gated_eq(en, L, R)` = `ConstrainEq(CondSelect(en,L,R), R)` |
| disabled-zero `(1−en)·x = 0` | `ConstrainEq(x, CondSelect(en, x, 0))` |
| linear sums, `256·c` | `Add`, `Mul` by `immediate(256)` |
| ranges | `ConstrainBits(v, 8 / 12)`, `ConstrainToBoolean` |
| node fetch | G4-FETCH (a two-table space-selected variant of `emit_enabled_fetch_block`) |

**What the lowering avoids.** No `LessThan`, no subtraction instruction, and no packed 128-bit value. Subtraction appears only as rearranged additive equalities.

---

## 12. Lemmas (paper only) and specimen conformance

**L-1 (Add/Sub exactness).**
- Assume byte ranges, Boolean carries and p > 511.
- Then the ADD-i and SUB-i relations hold over ℤ.
- Telescoping gives `a+b = r + 2^128·c_16` and `a−b = r − 2^128·bo_16`, with `r ∈ [0,2^128)`.
- Hence `c_16 = [a+b ≥ 2^128]` and `bo_16 = [a<b]`.
- Ok, Err10 and Lt therefore equal Rust `checked_add`/`checked_sub`/`<` and K `uiChecked`/`<Int`.
- The witness is unique: each digit is a residue and each carry a quotient.

**L-2 (Mul exactness).**
- Assume p > 2^20, 8-bit `p_k` and 12-bit carries.
- Then every MUL-k holds over ℤ.
- `Σ 256^k·MUL-k` gives `a·b = Σ_{k<32} p_k·256^k − 256^32·c_32 + c_0`. With `c_0 = c_32 = 0`, this is `a·b = r + 2^128·h` with `h < 2^128`.
- **Completeness:** the honest carries are at most 4079 < 2^12.
- **Uniqueness:** base-256 representation.
- `hZero = [h=0]` because `hSum = Σ h_j < p` and `Σ h_j = 0 ⇔ h = 0`.
- Ok iff `h = 0` matches `checked_mul` and K.

**L-3 (type separation).**
- `uu = 0 ⇒` every limb block is all-zero ⇒ `ovf = ltOut = 0` ⇒ `bERR11 = 1` and nothing else.
- No limb witness exists on the Err11 path, and arithmetic cannot run on non-UInt sorts.

**L-4 (fuel).**
- FD-2 with ranged bytes and Boolean `bw` is integer-exact for p > 511.
- On charged events `f' = f − 1` exactly. On free events `f' = f`.
- `en_chg = 1` requires `f ≠ 0`, so there is no wrap.

**L-5 (determinism).** Every aux/2 column is a function of `(program, pre)`:
- the selectors from `op`;
- the fetched rows from the guarded fetch;
- the limbs by L-1 and L-2;
- the fuel columns from the pre bytes and `en_chg`.

So the Clean determinism check is well-posed.

**L-6 (exactly one fuel regime).** By §5, `en_evt` is partitioned into A080, A082, A084 and `cg`.

**Conformance checks against the specimens.**
- **A-3, byte 0:** `255+1+0 = 0 + 256·1`. **Byte 1:** `0+0+1 = 1 + 0`. Both satisfy ADD-i.
- **M-3, k=0:** `65025 + 0 = 1 + 256·254`. **k=1:** `0 + 254 = 254 + 0`. Both satisfy MUL-k.
- **F-2, byte 0:** `255 + 1 = 0 + 256`. **Byte 2:** `0 + 1 = 1 + 0`. Both satisfy FD-2.
- **The rest.** Every §3 witness vector is the §2 function, so L-1, L-2 and L-4 apply to all specimens. That conclusion remains a paper claim until it is run.

---

## 13. Negative checks (owner-authored, optional, UNRUN; each must have no accepting assignment)

| # | Hostile witness | Killed by |
|---|---|---|
| N1 | Add(255,1) with `c_1 = 0` | ADD-0 forces `r_0 = 256`, which violates the 8-bit range |
| N2 | Add(1,1) claiming Err10 (`c_16 = 1`) | L-1: `r = 2 − 2^128 < 0` is impossible |
| N3 | Add(max,1) claiming Ok | `r = 2^128` is impossible |
| N4 | Lt(3,5) answering false | SUB chain forces `bo_16 = 1` |
| N5 | Lt with SUB.r left free or zero | SUB-i is enabled for Lt |
| N6 | Mul with an aliased carry ≥ 2^12, or a field-wrapped carry | 12-bit range and G4-FIELD |
| N7 | Mul(2^127,2) claiming Ok with `h` hidden in `c_32` | `c_32 := 0`, MUL-31, `hZero` |
| N8 | Mul with `h ≠ 0` and `hZero = 1` | MUL-HS / MUL-HZ (`TestEq`) |
| N9 | ERR10 with a Data append | bERR10 commit: `activeData' = D0` |
| N10 | ERR11 with nonzero limb columns | ZERO |
| N11 | ArithScratch enabled on Eq or Cons | ARH-0 derives the selectors from `op` |
| N12 | Swapped operands (`b−a`) | ARH-1 binds `opL` to the Right frame |
| N13 | A charged rule at `f = 0` | `en_chg = 0`: A084 only, P5 conflict |
| N14 | Fuel wrap `0 → max` | FD-3, and `bw_0 = 0` |
| N15 | Double decrement, or decrement on A080/A082/A084 | FD-1 and FD-2 force `bw` |
| N16 | `envelope.charge = 1` on a free publication | FD-4 |

---

## 14. Alternatives (disclosed briefly; byte limbs SELECTED)

**(a) 16-bit limbs.**
- Gain: 64 products instead of 256.
- Cost: byte→limb recomposition witnesses, and p above roughly 2^37.
- Not selected.

**(b) Packed 128-bit field equations.**
- Needs p > 2^257 and 128-bit range decompositions.
- Rejected, because the field is unknown.

**(c) `LessThan{bits:128}` for Lt.**
- Rejected: its semantics are unverified, and the Sub borrow already gives Lt.

**(d) Nibble limbs.**
- Allows a smaller field.
- Held as the fallback if G4-FIELD fails. Not selected.

**(e) Fuel decrement without witness columns, as in the A102 temporaries.**
- Rejected for the profile, because R2-2 and P4 require an explicit, reviewable witness.
- The A102 fragment itself is left untouched.

---

## 15. Remaining premises, non-grants and integration obligations

**Premises not discharged:**
- G4-FIELD, G4-ISA-1..4, G4-FETCH, G4-BYTE and G4-APPEND.
- INV-TY-D and INV-TY-K, i.e. `typeId` as sort and `uintBytes` meaningful for typeId 0.
- D-NAMES is unaffected.
- The O1–O4 observation domain.

**Integration obligations:**

| ID | Obligation |
|---|---|
| **IO-1** | P5 must bind `en_evt`, `term`, `en_chg`, A080/A082/A084, `en_bin`, the rule tag and its exactly-one property to these formulas. |
| **IO-2** | P4 must wire POP, W1 and the Root writes. |
| **IO-3** | After admitted A102 compilation (C4), prove or bind equivalence between the A102 fragment's internal fuel chain and FuelDec. That requires a reviewed edit, or reviewed operand binding to suppliedAux2. |
| **IO-4** | Cons, Branch and A157 sort reads still have no witness record. |
| **IO-5** | Reconcile with the exact text of the binding erratum R2-1 ("arithmetic ranges"). That text was not supplied, and aux/2 claims consistency only with its published summary. |
| **IO-6** | Bind profile identity into L04 (P3 condition 6). |
| **IO-7** | There is no emitter acceptance until the P3 §8 items and this profile are reviewed (P3 condition 5). |
| **IO-8** | The K, Quint and Clean checks, positive and negative, are unperformed and neither demanded nor authorized. |

**Non-grants.**
- No runtime, resource, numeric, operator, configuration, binding or circuit admission.
- Native authority stays 0.
- No adoption.

**Review.**
- Adoption needs fresh independent designated-owner and Muse audits of these exact bytes, followed by a Root record.
- Any byte change voids review of the changed bytes.

---

## 16. Manifest

```json
{
  "format": "sherlock-candidateA-aux2-arith-fuel-source/1",
  "status": "PROPOSED_UNADOPTED_SOURCE_DESIGN; author_only; not_a_vote; native_authority=0; execution=UNRUN; no_numeric_values_except_language_bitwidths; no_grants",
  "author": {"role": "designated security and physical-design owner, author capacity only", "model": "claude-opus-5-5", "effort": "high", "tools_used": false, "hashes_recomputed": false, "gpt_security_decision": false},
  "resolves_at_source_level": ["P3 R2-2 missing u128 remainder/carry/borrow/high witnesses", "P4 exact 16-byte fuel decrement witness", "fuel conjunct of P3 cg (remainingFuel!=0) unrealized in aux/1"],
  "inputs": {
    "source_commit": "192972dc09a1f2dfcccfaf369a25276fc23e891e",
    "current_publication_reference": "user-stated PR97 commit e0740ced... (truncated as supplied; not verifiable here); publication-binding names source_commit 8194011357795e77f37cfc6686ab28f88bbbc08a, PR #97",
    "files": [
      {"path": "models/sherlock/machine-circuit/arena-policy-source-20261016/P3-ARENA-DESIGN.md", "bytes": 65930, "sha256": "b2d6fda4b07804406f8f6d1c4ed9aa4d86031ccffb4bbb6e0be2c7cdab8891eb"},
      {"path": "models/sherlock/machine-circuit/arena-policy-source-20261016/A-ARENA-OPERATIONS.json", "bytes": 27137, "sha256": "3f33a0dc7fe831022b109ae0d60f2eb8cc9167e94bbc3d27c95a61ebdddbb214"},
      {"path": "models/sherlock/machine-circuit/arena-policy-source-20261016/ROOT-COMPLETE-P3-SOURCE-ADOPTION.json", "bytes": 5733, "sha256": "782a2db684298d9acc77850a40d9f79ec40934774a2c338f51134f3e04aefd3d"},
      {"path": "models/sherlock/machine-circuit/arena-policy-source-20261016/publication-binding.json", "bytes": 27588, "sha256": "e1dee8336aefa018a0093fe28c309f7d9a9e1ba6b8e39e207780c2ddd5aa1d11"},
      {"path": "models/sherlock/machine-circuit/A-REFERENCE-OUTPUTS-EVAL.json", "bytes": 54334, "sha256": "6823badbbca40cb945881ae689cd812f4ddca3aceee9b7ec8dbfde5cf9e122c2"},
      {"path": "models/sherlock/machine-circuit/A-RULE-OPERATIONS.json", "bytes": 616420, "sha256": "122289ad639de0e0d527106e2da8cfd19dd9ddf2fb16781a3ca742ab0731b0a5"},
      {"path": "crates/mori-core/src/sherlock/reference.rs", "bytes": 15742, "sha256": "e1bb1098d6e19ca3bca30a09bb90b11366fa9f70726aba9165d54f14056b181b"},
      {"path": "crates/mori-core/src/sherlock/reference_types.rs", "bytes": 5882, "sha256": "4b4bb364a0c8182e117a9c36efb5d91cb7e405f653a6d3870f54dc03eefa317f"},
      {"path": "models/sherlock/k/main.k", "bytes": 16249, "sha256": "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41"},
      {"path": "crates/mori-backend/src/sherlock_machine/literal_step.rs", "bytes": 19788, "sha256": "b5d2ca73b0e6b561f554902c3f4e50ec7743cc7ca04b9e7e9acb0c3aca10b839"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/design.md", "bytes": 32861, "sha256": "3738ca71fe8ad87d8ddba0f2251dcc54214a160b131550d5b8a49472763ade0e"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/interfaces.md", "bytes": 12574, "sha256": "86c228a4f9c7ab68122eec3accf7193210793b46117ade8df543d52660839e6a"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/extensions-and-obligations.md", "bytes": 15808, "sha256": "110772352b932f1e72bb54c5895bb6c180e0efe091189fb8606917e7d87e451f"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/crosswalk.json", "bytes": 30291, "sha256": "22c2ffcb41644a51f0edef3271c4bae1368512e2dcd3b4bfb7aacd3bd91481f0"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/proposal.md", "bytes": 7007, "sha256": "c6b2e23a4d475421b4b221032d1d0de4940c59242bf41f89788669af4c73694c"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/tasks.md", "bytes": 2041, "sha256": "7aea6c38cb80753b4dd781435ee9893c22ef5d41bcce0560da5acd8d00158cb4"},
      {"path": "openspec/changes/sherlock-machine-circuit-layout/OWNER-REPAIR-PROVENANCE.md", "bytes": 5873, "sha256": "25f0c5acce8d8e58a42852956d71dbf35f6c9741a04907273ddc382b1efb91a0"}
    ],
    "aux1_source_manifest_sha256": "e7c34e524a6f6aa002307124a9f2f0ad6f856c9522135cd74b8f65a1e9f7984f",
    "identities_as_supplied": true
  },
  "unchanged": ["full39 P1 map bytes and conditions 1-10, F1-F7", "R5 rule facts", "seven frozen L4 files", "Rust and K sources", "adopted P3 MD/JSON bytes, conditions 1-8, errata R2-1/3/4/5/6, Muse R2-4 dissent", "sherlock-k-a-aux/1 schemas, packing, enables, labels, dimensions, refusal names, lemmas", "literal_step.rs A102 fragment (C4)", "old 90s/30s caps, counters, terminal failures"],
  "profile": {
    "id": "sherlock-k-a-aux/2",
    "status": "PROPOSED",
    "separate_from_aux1": true,
    "operand": "StepOperandsA extended to {program, pre, suppliedPost, suppliedAux2}; AuxBlock2 not observation, not ObservedRecordA, not StateBlock",
    "labels": {"AuxBlock2": "private step witness", "public_aux_columns": "none", "program_pre_post_labels": "L04 open", "privacy_or_native_binding_claim": false},
    "packing_order": ["aux/1 AuxBlock exactly (EnvGetScan, FindScan, BindPreflight, EqScratch)", "ArithScratch", "FuelProbe", "FuelDec"],
    "packing_rules": "columns in listed order, L4 record flattening (VR=space,index; NODE=kind,typeId,ctorId,uintBytes[0..16),boolValue,fieldStart,fieldLength), arrays ascending, L4 width_rules for serialization, fixed shape, disabled record all-zero; packStepWitnessA orders and shape-checks only; no truth or acceptance column",
    "new_dimensions": [],
    "new_refusal_names": [],
    "new_logical_lanes": 232
  },
  "isa_observed_in_literal_step": {
    "instructions": ["CondSelect{bit,a,b,output}", "ConstrainEq{a,b}", "ConstrainToBoolean{val}", "ConstrainBits{val,bits}", "LessThan{a,b,bits:128,output}", "TestEq{a,b,output}", "Add{a,b,output}", "Mul{a,b,output}"],
    "helpers": ["immediate(u128)", "emit_enabled_fetch_block(TableShape, EnabledFetchInputs{enable,active_rows,index,rows}, scope)"],
    "definitions_supplied": false,
    "semantics": "inferred from names and usage; UNVERIFIED",
    "not_used_by_aux2": ["LessThan"]
  },
  "g4_requirements": {
    "G4-FIELD": "native field characteristic p > 2^20 (derived bound; no assumption p > 2^128; no packed 128-bit values)",
    "G4-ISA-1": "Add/Mul constrain outputs (not hints)",
    "G4-ISA-2": "TestEq(a,b,o) constrains o=[a=b] sound and complete; ConstrainToBoolean also applied",
    "G4-ISA-3": "CondSelect(bit,a,b)=bit?a:b for Boolean bit",
    "G4-ISA-4": "ConstrainBits(v,w) forces canonical representative in [0,2^w) for w<=12; ConstrainToBoolean forces {0,1}; ConstrainEq is field equality",
    "G4-FETCH": "space-polymorphic guarded node fetch: Data if space=1 else Constants, guard index < (space?D0:K0), output fully constrained (P3 E1)",
    "G4-BYTE": "serialized multi-byte lane reconstitution to logical carriers (existing unwaived obligation)",
    "G4-APPEND": "dynamic-index Data[D0] write for appendData (P3/G4)"
  },
  "enables": {
    "en_evt": "[pre.outcomeTag=0]*(1-[pre.controlLength=0]); =1 on every admitted record by O3/O4",
    "term": "[CR0.tag=1]*[depth=0] + [CR0.tag=9]",
    "isZero": "FuelProbe.zp[16]",
    "en_chg": "en_evt*(1-term)*(1-isZero) (= P3 cg)",
    "en_A080": "en_evt*[CR0.tag=1]*[depth=0]",
    "en_A082": "en_evt*[CR0.tag=9]",
    "en_A084": "en_evt*(1-term)*isZero",
    "partition": "en_evt = en_A080 + en_A082 + en_A084 + en_chg",
    "en_bin": "en_chg*[CR0.tag=1]*(1-[depth=0])*[FS0.tag=2]",
    "en_ar": "selAdd+selSub+selMul+selLt",
    "enADD": "selAdd*uu", "enSUB": "(selSub+selLt)*uu", "enMUL": "selMul*uu", "enLT": "selLt*uu",
    "exclusivity": "en_ar disjoint from aux/1 en_eq (binaryOp) and en_env/en_find/en_bind (CR0.tag); FuelProbe/FuelDec cross-cutting"
  },
  "records": {
    "ArithScratch": {
      "rules": ["A153 binaryOp in {Add=0,Sub=1,Mul=2,Lt=4}"],
      "header": [["opL", "VR", "ARH-1"], ["opR", "VR", "ARH-1"], ["nL", "NODE", "ARH-2"], ["nR", "NODE", "ARH-2"], ["selAdd", "Bool", "ARH-0"], ["selSub", "Bool", "ARH-0"], ["selMul", "Bool", "ARH-0"], ["selLt", "Bool", "ARH-0"], ["uu", "Bool", "ARH-3"], ["ovf", "Bool", "ARH-5"], ["ltOut", "Bool", "ARH-6"]],
      "ADD": [["r[0..16)", "u8", "ADD-i"], ["c[1..17)", "Bool", "ADD-i"]],
      "SUB": [["r[0..16)", "u8", "SUB-i"], ["bo[1..17)", "Bool", "SUB-i"]],
      "MUL": [["r[0..16)", "u8", "MUL-k"], ["h[0..16)", "u8", "MUL-k"], ["c[1..32)", "unsigned(12)", "MUL-k"], ["hSum", "unsigned(12)", "MUL-HS"], ["hZero", "Bool", "MUL-HZ"]],
      "logical_lanes": 184,
      "relations": {
        "ARH-0": "selAdd=en_bin*[op=0]; selSub=en_bin*[op=1]; selMul=en_bin*[op=2]; selLt=en_bin*[op=4]; op=FS0.binaryOp",
        "ARH-1": "opL=en_ar*FS0.valueRef; opR=en_ar*CR0.valueRef (left operand is Right-frame value)",
        "ARH-2": "nL=en_ar?node(opL):0; nR=en_ar?node(opR):0; node guarded per G4-FETCH",
        "ARH-3": "uu=en_ar*[nL.typeId=0]*[nR.typeId=0]; a_i=nL.uintBytes[i]; b_i=nR.uintBytes[i]",
        "ADD-i": "enADD*(a_i+b_i+c_i-r_i-256*c_{i+1})=0, i=0..15, c_0=0",
        "SUB-i": "enSUB*(a_i+256*bo_{i+1}-b_i-bo_i-r_i)=0, i=0..15, bo_0=0",
        "MUL-k": "enMUL*(S_k+c_k-p_k-256*c_{k+1})=0, k=0..31, c_0=c_32=0, p_k=r_k (k<16) or h_{k-16}, S_k=sum_{i+j=k} a_i*b_j",
        "MUL-HS": "hSum=sum_j h_j",
        "MUL-HZ": "hZero=enMUL*[hSum=0]",
        "ARH-5": "ovf=ADD.c_16+selSub*SUB.bo_16+enMUL-MUL.hZero",
        "ARH-6": "ltOut=selLt*SUB.bo_16",
        "ZERO": "(1-enX)*x=0 for block X columns; (1-en_ar)*x=0 for opL,opR,nL,nR",
        "RANGE": "u8: 8 bits; Bool: Boolean; MUL.c and hSum: 12 bits; NODE/VR per L4 widths; unconditional"
      },
      "branches": {
        "bERR11": "en_ar-uu", "bERR10": "ovf", "bLT": "enLT", "bOK": "uu-enLT-ovf", "sum": "en_ar"
      },
      "commit": {
        "bOK": "appendData(0,0,0,res,0,[]) at D0, res_i=ADD.r_i+selSub*SUB.r_i+MUL.r_i; activeData'=D0+1; CR0'={8,primitiveIsOk=1,valueRef={1,D0}}; capacity Data D0+1<=D",
        "bLT": "appendData(1,1,0,0^16,ltOut,[]); same CR0'; capacity Data D0+1<=D",
        "bERR10": "no append; activeData'=D0; CR0'={8,0,errorCode=10}; no capacity",
        "bERR11": "no append; CR0'={8,0,errorCode=11}; no capacity",
        "all": "POP (P4), W1, Root.environment preserved, fuel per FuelDec, everything else preserved (P5)"
      }
    },
    "FuelProbe": {
      "enabled": "en_evt (all 39 events); dispatch input, not a decrement witness",
      "columns": [["z[0..16)", "Bool", "FP-1"], ["zp[1..17)", "Bool", "FP-2"]],
      "relations": {"FP-1": "z_i=en_evt*[pre.remainingFuel_i=0]", "FP-2": "zp_{i+1}=zp_i*z_i, zp_0=en_evt; isZero=zp_16"}
    },
    "FuelDec": {
      "enabled": "en_chg (36 charged rules incl. all semantic-failure branches); all-zero on A080/A082/A084",
      "columns": [["bw[1..17)", "Bool", "FD-1"]],
      "relations": {"FD-1": "bw_{i+1}=bw_i*z_i, bw_0=en_chg", "FD-2": "post.remainingFuel_i+bw_i=pre.remainingFuel_i+256*bw_{i+1}, every step image", "FD-3": "bw_16=0", "FD-4": "envelope.charge=en_chg"},
      "consequences": "charged: post=pre-1 exactly; free: post=pre; no wrap; exhaustion semantics unchanged"
    }
  },
  "field_bounds": {
    "ADD-i": "[-511,511]", "SUB-i": "[-511,511]", "MUL-k": "[-1048575,1044495] subset (-2^20,2^20)", "MUL-HS": "[-4080,4095]", "FD-2": "[-511,256]",
    "carries": {"ADD": "c in {0,1}", "SUB": "borrow-out -bo in {-1,0}", "MUL": "c in [0,4079] honest (max at M-6 c_16), [0,4095] admitted", "FUEL": "-bw in {-1,0}"},
    "convolution": "m_k=min(k+1,31-k) terms for k<=30, S_31=0, S_k<=65025*m_k<=1040400",
    "required": "p > 2^20"
  },
  "fuel_rules": {
    "free": {"A080": "Done; preserve", "A082": "Rejected(code any u8); preserve", "A084": "Exhausted; isZero=1; preserve"},
    "charged_no_failure": ["A102", "A114", "A117", "A120", "A123", "A126", "A129", "A131", "A135", "A145", "A149", "A161", "A165", "A173", "A177", "A181", "A185", "A195", "A200", "A208", "A217", "A220", "A223"],
    "charged_failure_rules": {"A111": "Err3", "A140": "Err4", "A169": "Err5", "A190": "Err6", "A202": "Err7", "A206": "Err8", "A212": "Err9", "A225": "Error(code)"},
    "charged_with_failure_branches": {"A153": "ERR10, ERR11", "A157": "ERR11", "A204": "FAILURE bindingsPresent=0", "A214": "Miss"},
    "counts": {"charged": 36, "free": 3},
    "capacity_refusal": "no post-image, no fuel effect, no StepRecord"
  },
  "specimens": {
    "notation": "LE16 hex, byte0 first; all derived by hand from Rust/K semantics; UNRUN",
    "add": [
      {"id": "A-1", "a": "00x16", "b": "00x16", "carries": "none", "r": "00x16", "result": "Ok(0)"},
      {"id": "A-2", "a": "FFx16", "b": "00x16", "carries": "none", "r": "FFx16", "result": "Ok(max)"},
      {"id": "A-3", "a": "FF,00x15", "b": "01,00x15", "carries": "c_1=1", "r": "00,01,00x14", "result": "Ok(256)"},
      {"id": "A-4", "a": "FFx16", "b": "01,00x15", "carries": "c_1..c_16=1", "r": "00x16", "result": "Err(10), no append"},
      {"id": "A-5", "a": "FFx16", "b": "FFx16", "carries": "c_1..c_16=1", "r": "FE,FFx15", "result": "Err(10)"}
    ],
    "sub": [
      {"id": "S-1", "a": "01,00x15", "b": "01,00x15", "borrows": "none", "r": "00x16", "result": "Ok(0)"},
      {"id": "S-2", "a": "00x16", "b": "01,00x15", "borrows": "bo_1..bo_16=1", "r": "FFx16", "result": "Err(10)"},
      {"id": "S-3", "a": "00,01,00x14", "b": "01,00x15", "borrows": "bo_1=1", "r": "FF,00x15", "result": "Ok(255)"},
      {"id": "S-4", "a": "FFx16", "b": "FFx16", "borrows": "none", "r": "00x16", "result": "Ok(0)"}
    ],
    "lt": [
      {"id": "L-1", "a": "03,00x15", "b": "05,00x15", "borrows": "bo_1..bo_16=1", "r_unstored": "FE,FFx15", "result": "Ok(Bool true)"},
      {"id": "L-2", "a": "05,00x15", "b": "03,00x15", "borrows": "none", "r_unstored": "02,00x15", "result": "Ok(Bool false)"},
      {"id": "L-3", "a": "07,00x15", "b": "07,00x15", "borrows": "none", "r_unstored": "00x16", "result": "Ok(Bool false)"},
      {"id": "L-4", "a": "00x15,01", "b": "FF,00x15", "borrows": "bo_1..bo_15=1, bo_16=0", "r_unstored": "01,FFx14,00", "result": "Ok(Bool false)"},
      {"id": "L-5", "a": "FF,00x15", "b": "00x15,01", "borrows": "bo_16=1 only", "r_unstored": "FF,00x14,FF", "result": "Ok(Bool true)"}
    ],
    "mul": [
      {"id": "M-1", "a": "00x16", "b": "FFx16", "carries": "none", "r": "00x16", "h": "00x16", "result": "Ok(0)"},
      {"id": "M-2", "a": "01,00x15", "b": "FFx16", "carries": "none", "r": "FFx16", "h": "00x16", "result": "Ok(max)"},
      {"id": "M-3", "a": "FF,00x15", "b": "FF,00x15", "carries": "c_1=254", "r": "01,FE,00x14", "h": "00x16", "result": "Ok(65025)"},
      {"id": "M-4", "a": "00x15,80", "b": "02,00x15", "carries": "c_16=1", "r": "00x16", "h": "01,00x15", "result": "Err(10)"},
      {"id": "M-5", "a": "00x8,01,00x7", "b": "00x8,01,00x7", "carries": "none", "r": "00x16", "h": "01,00x15", "result": "Err(10)"},
      {"id": "M-6", "a": "FFx16", "b": "FFx16", "carries": "c_k=255k-1 (1<=k<=16); c_k=255(32-k) (17<=k<=31)", "r": "01,00x15", "h": "FE,FFx15", "hSum": 4079, "result": "Err(10)"}
    ],
    "type_and_space": [
      {"id": "T-1", "step": "Add(UInt1,Bool true)", "result": "Err(11); uu=0; limb blocks zero; no append"},
      {"id": "T-2", "step": "Lt(Nil,UInt3)", "result": "Err(11)"},
      {"id": "T-3", "step": "Eq(UInt1,Bool true)", "result": "ArithScratch zero; aux/1 EqScratch MISMATCH -> Ok(Bool false)"},
      {"id": "T-4", "step": "Cons(UInt1,Nil)", "result": "ArithScratch zero; outside aux/2"},
      {"id": "P-1", "step": "Add({C,0}=UInt7,{D,2}=UInt5)", "result": "opL={0,0}, opR={1,2}; r=0C,00x15; Ok(12); append at Data[D0]"}
    ],
    "fuel": [
      {"id": "F-1", "rule": "A102", "pre": "01,00x15", "bw": "none", "post": "00x16"},
      {"id": "F-2", "rule": "A111", "pre": "00,00,01,00x13", "bw": "bw_1=bw_2=1", "post": "FF,FF,00x14"},
      {"id": "F-3", "rule": "A153 ERR10", "pre": "00x15,01", "bw": "bw_1..bw_15=1", "post": "FFx15,00"},
      {"id": "F-4", "rule": "A135", "pre": "FFx16", "bw": "none", "post": "FE,FFx15"},
      {"id": "F-5a", "rule": "A080 after F-1", "pre": "00x16", "isZero": 1, "FuelDec": "zero", "post": "00x16 preserved, Done"},
      {"id": "F-5b", "rule": "A084 after F-1", "pre": "00x16", "isZero": 1, "FuelDec": "zero", "post": "00x16 preserved, Exhausted"},
      {"id": "F-6", "rule": "A082", "pre": "05,00x15", "FuelDec": "zero", "post": "preserved"},
      {"id": "F-7", "rule": "A082 at fuel 0", "pre": "00x16", "isZero": 1, "FuelDec": "zero", "post": "preserved; publication precedes exhaustion"},
      {"id": "F-8", "rule": "A225", "pre": "03,00x15", "post": "02,00x15"}
    ]
  },
  "lemmas_paper_only": ["L-1 Add/Sub integer exactness and uniqueness for p>511", "L-2 Mul exactness, completeness (carry<=4079<2^12), uniqueness for p>2^20", "L-3 type separation: uu=0 implies limb blocks zero and bERR11", "L-4 fuel: charged post=pre-1, free post=pre, no wrap", "L-5 determinism: every aux/2 column a function of (program,pre)", "L-6 exactly one fuel regime per event"],
  "negative_checks_unrun": ["N1 ADD carry dropped", "N2 false overflow", "N3 false Ok on overflow", "N4 Lt flipped", "N5 Lt remainder free", "N6 Mul aliased carry", "N7 Mul high hidden in c_32", "N8 hZero forged", "N9 ERR10 with append", "N10 ERR11 nonzero limbs", "N11 Arith enabled on Eq/Cons", "N12 swapped operands", "N13 charged at fuel 0", "N14 fuel wrap", "N15 double or free decrement", "N16 charge envelope forged"],
  "alternatives": {"selected": "byte limbs with per-limb carries", "rejected_or_held": ["16-bit limbs (recomposition, p>~2^37)", "packed 128-bit equations (p>2^257)", "LessThan{bits:128} for Lt (unverified)", "nibble limbs (fallback if G4-FIELD fails; needs new reviewed bytes)", "A102-style emitter temporaries without witness columns"]},
  "integration_obligations": {
    "IO-1": "P5 binds en_evt, term, en_chg, A080/A082/A084, en_bin, rule tag and exactly-one to these formulas",
    "IO-2": "P4 wires POP, W1 and Root writes",
    "IO-3": "after admitted A102 compile (C4), bind or prove equivalence of the A102 fragment fuel chain to FuelDec",
    "IO-4": "Cons/Branch and A157 sort reads still lack a witness record",
    "IO-5": "reconcile with exact R2-1 erratum text (not supplied)",
    "IO-6": "profile identity and dimensions bound into L04 (P3 condition 6)",
    "IO-7": "no emitter acceptance until P3 section 8 items and this profile are reviewed (P3 condition 5)",
    "IO-8": "K/Quint/Clean positive and negative checks unperformed; not demanded or authorized"
  },
  "out_of_scope": ["Eq (aux/1 EqScratch)", "Bool operations (A157 Not/Leaf, A161/A165/A169)", "Cons/Branch", "P4 full spine", "P5 dispatch", "StepInv and chaining", "F4", "MD2", "L04", "L05", "G4 implementations", "B", "general ADTs", "financial", "proof", "ledger", "Midnight stagenet", "U0-U7/M1-M17/MC/SP/G"],
  "retained_gates": ["C4 admitted A102 compilation before dependent circuit edits; compile failure requires registration revert", "full39 conditions 1-10 and F1-F7", "P3 conditions 1-8, errata R2-1/3/4/5/6, Muse R2-4 dissent", "old 90s/30s caps, counters, terminal failures", "native_authority=0"],
  "requires": ["fresh independent designated-owner security audit of these exact bytes", "fresh Muse normal audit of these exact bytes", "Root adoption record", "any byte change voids reviews for changed bytes"],
  "proposed_materialization_paths_root_selects": ["models/sherlock/machine-circuit/aux2-arith-fuel-source-20261016/AUX2-ARITH-FUEL-DESIGN.md", "models/sherlock/machine-circuit/aux2-arith-fuel-source-20261016/AUX2-ARITH-FUEL.json"],
  "grants": "none"
}
```

PROPOSED_AUX2_ARITH_FUEL_SOURCE_DESIGN_UNADOPTED_UNRUN