# Google Material Design 3 (`m3.material.io`) — Prose Architecture & The 39 Silicon Floor Modules (`SHERLOCK_K_RULES`)

## 1. Explaining the 39 Silicon Floor Modules (`A080..A225`, `sherlock-k-a/1` ROM)
When you enter **Inside Hollow Chip** (`interior`) or **Cross-Section** (`frames`) and look down to the bottom of the deep subterranean chamber (`y = -3.38`), you see **39 physical 3D silicon chiplets** set in gold-rimmed sockets in a `6 × 7` floor grid.

### Plain-English Explanation (Top Orientation Card & Editorial Card)
- **What they are**: Those 39 modules on the floor of the chip are Sherlock's entire instruction set (`sherlock-k-a/1`). Every single step of every program—whether calculating `fact(4)`, checking a 40-unit loan installment, or unwinding a recursive call—must fire **exactly one** of these 39 fixed rules.
- **Why there are only 39 modules**: Ordinary CPUs have hundreds of complex instructions and hidden microcode. Because every step of a Sherlock run must be checked by a mathematical proof (`ConstraintPlan`), Sherlock strips its hardware down to 39 tiny, non-overlapping transition rules (`A080 Done` through `A225 PrimErr`).
- **What happens when a step runs**: On each step (`StepRecord`), the single floor module that governs that step physically rises out of its gold socket and lights up its status bar:
  - **3 Terminal Modules (`0F` free)**: `A080 Done` (teal), `A082 Rejected` (crimson), `A084 Exhausted` (amber).
  - **10 Expression Dispatch Modules (`1F`)**: `A102 Literal`, `A108 VarFound`, `A111 VarMissing`, `A114 Let`, `A117 BinLeft`, `A120 Unary`, `A123 If`, `A126 Match`, `A129 Call0`, `A131 CallArgs`.
  - **2 Frame Stack Guard Modules (`1F`)**: `A135 PushFrame` (allocates a continuation ring slot), `A140 FrameErr` (halts if recursion exceeds `max_frames`).
  - **12 Continuation Frame Modules (`1F`)**: `A145 BindLet`, `A149 RightOp`, `A153 BinPrim`, `A157 UnPrim`, `A161 IfTrue`, `A165 IfFalse`, `A169 IfErr`, `A173 StartCases`, `A177 NextArg`, `A181 FinishArgs`, `A185 RetOk`, `A190 RetErr`, `A195 RestoreScope`.
  - **12 Function, Pattern & Primitive Modules (`1F`)**: `A200 LookupFn`, `A202 MissingFn`, `A204 BindParams`, `A206 BindErr`, `A208 SchedBody`, `A212 NoMatchErr`, `A214 TryPat`, `A218 MatchBody`, `A220 NextCase`, `A223 PrimOk`, `A225 PrimErr`.

---

## 2. Integrating `Sherlock Execution Engine Report.md`
From `/mnt/c/Users/charl/OneDrive/Desktop/Sherlock Execution Engine Report.md`, we integrate:
1. **The 5-Layer System Architecture Table**:
   - **Layer 1: Moriarty** (Authoring layer — compiles agreement into frozen `ProgramImage`)
   - **Layer 2: Off-Chain Executor ("The Solver")** (Untrusted operator — proposes a concrete transaction path)
   - **Layer 3: Sherlock** (Deterministic execution engine — steps through `StepRecord` chain under `MachineImage` bounds)
   - **Layer 4: Cryptographic Proof Machinery (`WitnessPlan` & `ConstraintPlan`)** (Converts `StepRecord` trace into private witness wires and rigid algebraic laws)
   - **Layer 5: Midnight Verification & Settlement** (Verifies proof in milliseconds and commits ledger state)
2. **Concrete Financial Agreement Walkthrough (40-Unit Loan Installment & Rejected Variations A/B)**:
   - Shows how an authorized 40-unit payment from Borrower (`100 -> 60`) to Lender (`0 -> 40`) passes every rule (`A080 Done`), while **Variation A** (37 units + 3-unit fee) and **Variation B** (40 units routed to `Account_Solver`) deterministically fail the `ConstraintPlan`.
