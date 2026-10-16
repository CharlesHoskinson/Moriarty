FRESH NON-AUTHOR SOURCE AUDIT plus AUTHOR-ONLY RUNNER/CONFIG ADAPTATION. Reviewer: Opus 5.5 high, designated security owner. Basis: the supplied text only. I used no tools, ran no command, wrote no file and recomputed no hash. Every test remains UNRUN.

SCOPE OF THIS VOTE. This vote covers two things only: the 15 compile-closure leaves supplied, and the execution boundary described below.
- It does NOT review the replacements or CONFIG.json returned here. I authored them, so this vote is not a review of them.
- The patched run_arithmetic.py, the CONFIG.json and the final CANDIDATE.json each need fresh exact-byte Muse and Opus execution-preflight reviews before anything runs.
- The final CANDIDATE.json is exactly 22 rows: the 19 SOURCE-CANDIDATE rows, plus SOURCE-CANDIDATE.json, CONFIG.json and run_arithmetic.py.
- The runner refuses, before attempt-01 exists, unless both of these are regular files under /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/arithmetic-consumer-01: execution-preflight-muse/REPORT.md and execution-preflight-opus/REPORT.md.
- Each report must carry exactly one line 'VERDICT: APPROVE_WITH_SCOPE' and exactly one line 'CANDIDATE_MANIFEST_SHA256: <sha256 of final CANDIDATE.json>'. Root writes both lines.
- Root must confirm that CONFIG.source_candidate equals the materialized SOURCE-CANDIDATE.json rows. The runner also enforces this. I copied those rows verbatim and did not recompute them.

COMPILATION INTERFACE (source reading, not a compile).
- Harness: #[path] mod sherlock loads arithmetic_consumer_shim.rs.
- A module file loaded through #[path] resolves its child modules like a mod.rs file, i.e. in its own directory. So the plain declarations resolve to sherlock/reference_types.rs, arithmetic_witness.rs and arithmetic_witness_normal_tests.rs.
- #[path] layout_a loads sherlock/arithmetic_consumer_layout_shim.rs. Its nine #[path] layout_a/*.rs targets resolve under sherlock/layout_a/.
- Expected dep-info is therefore exactly the 15 closure files. There is no include!, env!, extern crate, unsafe, build script, proc macro, or fs/net/process/env/thread use. Only std, builtin derives and macro_rules are used.
- Visibility checks out:
  - pub(super) items in arithmetic_witness are reachable from the descendant crate::sherlock::layout_a::arithmetic_fuel.
  - pub(super) encode_observed_a and materialize_numeric_aux_a are reachable from sibling or child modules of layout_a.
  - The pub use reference_types::* glob supplies BinaryOp, Control, Environment, Expr, Frame, Outcome, Program, Snapshot, Value and PROFILE.
  - &p.full_program (&Arc<Program>) coerces to &Program at the AImage struct-field coercion site.
  - Option<&T> is Copy, so frame and control remain usable after is_some_and and filter.
  - The then_some borrows of header.n_l/n_r end before header moves into NumericAuxA.
  - Integer literals unify: the case arrays reach u128 through 1u128<<120, 0u128 or u128::MAX. The [255;16] and [0;16] literals compared against [u8;16] infer u8. The second fuel loop's tag falls back to i32 and is only matched, which is fine.
- Edition-2024 points: no reserved identifiers; no explicit ref or & patterns under default binding modes; the expr fragments in add! are harmless.
- Expected warnings only, mostly dead_code across the decoder/encoder and reference_types. No deny-by-default lint is apparent.
- libtest names are exactly the 9 proposed: 5 under sherlock::arithmetic_witness_normal_tests and 4 under sherlock::layout_a::arithmetic_fuel_normal_tests.
- Residual: the unchanged layout leaves (state_decode, state_encode, prepare, constants, limits, rows, state) have not been compiled before in this module shape. I spot-checked their borrows, coercions, closures and macros and found no error. Compilation is established only by the run, and the first failure stops it.

CONSUMER vs ADOPTED DESIGN.
- Step decoding: the consumer first runs decode_observed_a in Step mode. That call does all of the following, and any refusal propagates as Err:
  - checks the profile;
  - compares the full Program structurally;
  - checks ProgramBlock equality including inactive rows;
  - checks D, fixed lengths and zero padding;
  - checks O1, O2 and O3;
  - checks the Push spine;
  - checks every node and field;
  - checks the frame and control zero columns.
- Fuel and enables, derived from actual rows only:
  - en_evt = outcome_tag 0 AND control_length != 0.
  - CR0 = control_rows[0]; FS0 = frame_slots[0] when depth != 0. This matches the encoder's head-first and top-first order.
  - term = (CR0.tag 1 AND depth 0) OR CR0.tag 9.
  - materialize_fuel computes FP-1/2 and FD-1/2, with charged = en_chg.
  - en_bin = charged AND Return AND FS0.tag 2.
- Selectors and header:
  - op maps 0/1/2/4 only; Eq, Cons and Branch give None (ARH-0).
  - The header is zero unless en_ar holds. Then opL = FS0.valueRef and opR = CR0.valueRef (ARH-1).
  - nL and nR come from a space-selected fetch guarded by the active length (ARH-2).
  - uu holds only when both typeIds are 0 (ARH-3). The decoder ties typeId 0 to kind 0, so uintBytes are meaningful (INV-TY).
- No caller-supplied guard, op, operand or charge exists in the API.
- After a successful Step decode the fetch refusal is unreachable, because the decoder already validated both refs. Its AuxBlock2-labelled WrongRepresentation is defensive and untested, and it is not a claimed refusal.
- Not done, and correctly not claimed: transitions, appendData, branch commit, aux/2 packing, native binding, constraints and reachability.

TEST TRACES (all pass on paper).
- Encoder layout: in binary(op,a,b,f), Constants[0]=a and Constants[1]=b. The encoder appends the Return value first, so Data[0]=b and Data[1]=a. Then FS0.value={D,1} and CR0.value={D,0}.
- P1: Data[2] is a copy of UInt5, active_data=3, FS0 points to Constants[0]=7 and CR0 to {D,2}.
  - Header {C,0}/{D,2}; nL is the Constants row and nR is Data[2].
  - r=12, selAdd and uu set, no ovf, charged, fuel 3 becomes 2.
  - The stale Data[1] still decodes; Step mode does not require reachability.
- A3, S3, L4 and M6 at fuel 1:
  - Every case: header equals the actual rows, uu and charged hold, post is 0.
  - A3: 256, c1 set, c16 clear.
  - S3: 255, bo1 set, bo16 clear.
  - L4: remainder [01,FF x14,00], no ltOut, no ovf.
  - M6: r=1, h=[FE,FF x15], c16=4079, c32=0, hSum=4079, ovf set.
- T1 Add(UInt1,Bool): nR has typeId 1, so uu=0 and every block is zero. hZero is false.
- T2 Lt(Nil from Constants[0], UInt3): nL has typeId 2, so uu=0.
- T3 Eq and T4 Cons: all selectors are 0, the header is default, and the step is still charged.
- Fuel, first loop: Eval(entry) with a stale Right frame. 1 gives 0; 65536 gives 65535; 2^120 gives 2^120-1; MAX gives MAX-1; 3 gives 2. Charged in every case, uu=0, and O1 holds against initial MAX.
- Fuel, second loop, at depth 0 with zeroed frames:
  - Return at fuel 0 (A080 shape): preserved, bw zero, zp16 set.
  - Eval at fuel 0 (A084 shape): preserved, zp16 set.
  - Error(4) at fuel 5: preserved, zp16 clear.
  - Error at fuel 0: preserved, zp16 set.
- The fuel tests reuse owner values on ordinary controls and claim no rule line. The claims say so.

SECURITY LIMITS.
- These are honest host materialization tests only.
- No constraint soundness, decoder refusal, hostile witness, N1-N16, or Canonical decode mode (compiled, not exercised) is tested.
- No hostile case is required for this narrow scope, and I authored none.
- Allocations are fixed-small. A same-UID child is not contained beyond the inherited guard limits.

NONSECURITY NOTES FOR MUSE.
- T1/T2 assert selectors, uu and zero blocks but not the header lanes or typeIds, despite the test name. The claims are narrowed to match.
- The first fuel loop keeps a stale depth-1 Right frame under Eval. It decodes but is not a claimed reachable state.
- The unchanged witness notes from the prior audit (T-3/T-4 loop; c32 and bw16 test-asserted) still apply.

RUNNER/CONFIG ADAPTATION (author only; needs fresh reviews).
- Changes, all minimal substring replacements to the prior Opus runner:
  - root changed to arithmetic-consumer-01;
  - fresh FORMAT, CONFIG_FORMAT and CRATE identities, so receipts cannot alias the witness run;
  - CLOSURE_SIZE, TEST_COUNT = 15, 9;
  - docstring counts updated;
  - observed, scope, tests, constructor_startup and product_wiring claims narrowed to the consumer scope.
- Unchanged:
  - the strict JSON and manifest gates;
  - exact candidate/ membership;
  - the review gate before attempt creation;
  - verified guard bytes executed without a re-read;
  - the preflight override: snapshot hashes bound to reviewed rows, toolchain 1.99.0, pinned rustc;
  - the 30 s wall clock and new session with killpg;
  - the minimal env and no shell;
  - the single direct rustc --test --edition=2024 --emit=dep-info,link of arithmetic_harness.rs;
  - the exact dep-info closure check;
  - exact 9-test membership, all ok, 0 filtered;
  - artifact rehash before execution;
  - postflights for sources, interpreter, compiler, manifest, reports and membership;
  - first-failure stop.
- Invocation: python3 -I -B <root>/run_arithmetic.py, with no arguments.
- CONFIG pins the 19 source rows verbatim, the 15-file closure, the 19 snapshot members and the 9 test names.

RESIDUALS (recorded, not fixed).
- Memory, disk and process counts are unbounded.
- Descendants that outlive a normally exiting leader are not killed.
- 0444 file modes are not a boundary against a same-UID child.
- The sysroot, linker, libc, Python stdlib and kernel are unhashed.
- Product mod.rs/lib.rs wiring and the crate::sherlock paths the new leaves use are not checked against the product crate.

UNCHANGED. Native authority stays 0. C4, the aux/1 prefix, schemas, profiles and the public API are unchanged. Owner E1-E7 and conditions 1-7, and Muse E-FUEL and conditions 1-6, stay binding. No acceptance, adoption or execution authorization is made here.
