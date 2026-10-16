Top finding: no blocking mathematical/interface defect in scope.

The prior Opus “mul total max 979454” wording is not the global maximum.
For `MAX×MAX` at `k=15`, there are 16 diagonal products.

`16×255×255 = 1,040,400`.
Carry-in `c[15] = 3824`.
Total is therefore `1,044,224`.

`979,454` is the `k=16` total: `15×65025 + c[16]=4079`.
That is not the maximum.

This does not block the code.
`materialize_arithmetic` accumulates each diagonal in `u32`.
Even the loose bound `1,040,400 + 4079 = 1,044,479` is far below `u32::MAX`.
Its carry-out is at most `4079`.
That fits `u16` and the stated 12-bit carry design.
No `u128` product is used or needed.

Other source checks agree.

- Add uses `u16`; maximum is `511`.
- Sub/Lt borrow recurrence never underflows after the borrow choice.
- Mul checks all 32 diagonals, including the zero high diagonal.
- `h_sum` is at most `4080` in general and `4079` for M6.
- Fuel probe, charge, borrow decrement, and zero-fuel preservation match F1–F8.
- Type separation preserves selectors while forcing `uu=false` and zero numeric blocks when either operand is not `type_id==0`.
- Eq/Cons map to no arithmetic operation.

Actual consumer flow also agrees.

`materialize_numeric_aux_a` first runs Step-mode `decode_observed_a`.
It then derives `en_evt`, terminal status, charge, `en_bin`, operation, header references, and operand nodes from actual root, control, frame, Constants, and Data rows.
No supplied operation, operand bytes, guard, or charge overrides those rows.
P1, A3, S3, L4, M6, T1–T4, and the fuel cases traverse encoder-built, Step-decoded images.

Execution evidence is internally consistent as supplied.

Standalone `rustc 1.99` compilation exited `0`.
The dep-info closure is exactly the stated 15 files.
The run log shows exactly 9 distinct current tests, all `ok`, with `0 failed`.
The separate prior pure attempt had 5 tests.
They are not additive as 14 current tests.

Postflight records show unchanged sources, snapshot, compiler, interpreter, reviewed candidate, and preflight reports.
Compile stderr contains only dead-code warnings.
Hashes and path metadata were taken as supplied, not independently recomputed.

Identity as supplied: Muse preflights returned `muse-wonder-1.0`, provider `meta`, profile `tbh`.
Opus preflights returned `claude-opus-5-5`.
This reviewer is Muse Code powered by muse-wonder-1.0.

Dissent: none on code or execution scope.
The only dissent is from the prior Opus global-maximum wording above.

Limits remain strict.
Product `mod.rs`/`lib.rs` and the full product crate were not compiled.
Canonical decoding was compiled but not exercised.
No transition, reachability, appendData, aux/2 packing, ABI, native binding, constraint, certification, hostile, refusal, Cargo, SDK, backend, VM, model, proof, key, ledger, or cache behavior is approved.
Native authority remains `0`, C4 is unchanged, and all G4/IO1–8/full-roadmap exits remain open.

VERDICT: APPROVE_WITH_SCOPE
ACTUAL_RESULT_MANIFEST_SHA256: 5c10e513f219ce5ed3b9b2948437c630f24a95eb5d645f3e8057b34735a60c84
