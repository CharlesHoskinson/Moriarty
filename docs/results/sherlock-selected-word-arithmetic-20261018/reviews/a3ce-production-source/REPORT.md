{
  "api_readback": {
    "source_citations": [
      "BODY 02 (crates/mori-core/src/sherlock/compile_artifact/draft_execution.rs)",
      "BODY 06 (crates/mori-core/src/sherlock/word_lowering.rs)",
      "BODY 09 (crates/mori-core/src/sherlock/word_arithmetic.rs)",
      "BODY 10 (ACTUAL-OUTCOME-REF-DATA-DECLARATIONS.rs.txt)"
    ],
    "full_arithmetic_result_variants_and_payloads": {
      "citation": "BODY 06",
      "verbatim_declaration": "pub enum FullArithmeticResult {\n    Add32(CheckedUInt128Add32), Sub32(CheckedUInt128Sub32), Mul32(CheckedUInt128Mul32),\n    Add64(CheckedUInt128Add64), Sub64(CheckedUInt128Sub64), Mul64(CheckedUInt128Mul64),\n}",
      "variants": [
        { "variant": "Add32", "payload_type": "CheckedUInt128Add32" },
        { "variant": "Sub32", "payload_type": "CheckedUInt128Sub32" },
        { "variant": "Mul32", "payload_type": "CheckedUInt128Mul32" },
        { "variant": "Add64", "payload_type": "CheckedUInt128Add64" },
        { "variant": "Sub64", "payload_type": "CheckedUInt128Sub64" },
        { "variant": "Mul64", "payload_type": "CheckedUInt128Mul64" }
      ]
    },
    "arithmetic_lowering_execute_and_delegate": {
      "citation": "BODY 06",
      "signature": "pub fn execute(self, left: u128, right: u128) -> FullArithmeticResult",
      "one_actual_delegate_expression": "FullArithmeticResult::Add32(checked_u128_add_32(left, right))",
      "verbatim_match_body": "match (self.layout, self.operation) {\n    (WordLayout::Word32, UInt128Operation::Add) => FullArithmeticResult::Add32(checked_u128_add_32(left, right)),\n    (WordLayout::Word32, UInt128Operation::Sub) => FullArithmeticResult::Sub32(checked_u128_sub_32(left, right)),\n    (WordLayout::Word32, UInt128Operation::Mul) => FullArithmeticResult::Mul32(checked_u128_mul_32(left, right)),\n    (WordLayout::Word64, UInt128Operation::Add) => FullArithmeticResult::Add64(checked_u128_add_64(left, right)),\n    (WordLayout::Word64, UInt128Operation::Sub) => FullArithmeticResult::Sub64(checked_u128_sub_64(left, right)),\n    (WordLayout::Word64, UInt128Operation::Mul) => FullArithmeticResult::Mul64(checked_u128_mul_64(left, right)),\n}"
    },
    "checked_method_vs_inner_checked_field": {
      "outer_method_citation": "BODY 06",
      "outer_method_signature": "pub fn checked(self) -> Result<u128, u8>",
      "outer_method_body": "match self {\n    Self::Add32(result) => result.checked, Self::Sub32(result) => result.checked,\n    Self::Mul32(result) => result.checked, Self::Add64(result) => result.checked,\n    Self::Sub64(result) => result.checked, Self::Mul64(result) => result.checked,\n}",
      "inner_field_citation": "BODY 09",
      "inner_field_type": "pub checked: Result<u128, u8>",
      "inner_component_declarations": [
        "pub struct CheckedUInt128Add32 { pub full: FullAdd32, pub checked: Result<u128, u8> }",
        "pub struct CheckedUInt128Sub32 { pub full: FullSub32, pub checked: Result<u128, u8> }",
        "pub struct CheckedUInt128Mul32 { pub full: FullProduct32, pub checked: Result<u128, u8> }",
        "pub struct CheckedUInt128Add64 { pub full: FullAdd64, pub checked: Result<u128, u8> }",
        "pub struct CheckedUInt128Sub64 { pub full: FullSub64, pub checked: Result<u128, u8> }",
        "pub struct CheckedUInt128Mul64 { pub full: FullProduct64, pub checked: Result<u128, u8> }"
      ]
    },
    "run_draft_host_with_word_layout_signature": {
      "citation": "BODY 02",
      "verbatim_signature": "pub fn run_draft_host_with_word_layout(\n    source: &str, config_source: &str, target_source: &str,\n    word_layout: WordLayout, sink: &mut dyn FnMut(&StepEventC),\n) -> Result<WordLoweredDraftHostRun, CompileFailure>"
    },
    "word_lowered_draft_host_run_fields": {
      "citation": "BODY 02",
      "verbatim_struct": "pub struct WordLoweredDraftHostRun {\n    pub word_layout: WordLayout,\n    pub run: DraftHostRun,\n}",
      "inner_draft_host_run_struct": "pub struct DraftHostRun {\n    pub preparation: DraftCompileArtifact,\n    pub completed: CompletedRun,\n}"
    },
    "outcome_enum_and_rejected_payload": {
      "citation": "BODY 10",
      "enum_name": "OutcomeC",
      "rejected_variant_and_payload": "OutcomeC::Rejected { code: u8 }",
      "verbatim_declaration": "pub enum OutcomeC { Running, Halted { value: RefC }, Rejected { code: u8 },\n    Exhausted { reason: ExhaustReason } }"
    }
  },
  "source_audit": {
    "verdict": "APPROVE_SOURCE_ONLY_UNRUN_FOR_ORDINARY_QA_DISPATCH",
    "findings_by_body": [
      {
        "body": "BODY 01 (crates/mori-core/src/lib.rs)",
        "observation": "Exports `#[path = \"sherlock/word_arithmetic.rs\"] pub mod sherlock_word_arithmetic;` and `#[path = \"sherlock/word_lowering.rs\"] pub mod sherlock_word_lowering;` alongside `pub mod sherlock_compile_artifact;`. No native or circuit claims are introduced."
      },
      {
        "body": "BODY 02 (crates/mori-core/src/sherlock/compile_artifact/draft_execution.rs)",
        "observation": "`run_draft_host_with_word_layout` shares `run_draft_host_impl` with `run_draft_host`. It parses config/target, runs `recursive_source::prepare_checked_draft_bounded` and `recursive_source::admit_checked_draft_bounded` with unchanged budgets/bounds/layout/fuel/stack, projects the genesis preparation artifact, and dispatches `Some(layout) => host_owner_engine_with_word_layout(layout).run(&admitted, &mut image, &mut forwarding_sink)`. Every `StepEventC` is forwarded to `sink` via `forwarding_sink`. Returns `Ok(WordLoweredDraftHostRun { word_layout, run })` where semantic `OutcomeC::Rejected { code: 10 }` is carried inside `run.completed.summary.outcome` and `run.observe()?.image.outcome`, distinct from `Err(CompileFailure)`."
      },
      {
        "body": "BODY 07 (crates/mori-core/src/sherlock/helper_machine/shared/owner_host.rs)",
        "observation": "`host_owner_engine_with_word_layout(layout: crate::sherlock_word_lowering::WordLayout) -> impl OwnerEngine` constructs `CheckedOwnerEngine::new_reviewed(crate::sherlock::helper_machine::families::WordFamilyPlans::new(layout), PrepareDeps { prepare_core: owner_prepare::prepare_core, check_program_view: owner_prepare::check_program_view, admit: owner_prepare::admit, admit_run: owner_prepare::admit_run })`. Owner guards, preparation dependencies, and `observe_bound_image` AB-binding (`PreImage::bind` + `implied_seg_base`) are unchanged."
      },
      {
        "body": "BODY 03 & BODY 04 (families/mod.rs & families/scalar.rs)",
        "observation": "`WordFamilyPlans` holds `layout: crate::sherlock_word_lowering::WordLayout` and forwards all non-scalar `FamilyPlans` methods to the exact same family functions as `HostFamilyPlans`, while `plan_scalar` calls `scalar::dispatch_word(sel, pre, guards, self.layout)` -> `plan_scalar_with_word_layout(sel, pre, guards, layout)`."
      },
      {
        "body": "BODY 05 (fragments/scalar.proposed.rs)",
        "observation": "`plan_scalar_lowered` evaluates real source operands: `RuleIdC::BHelperScalarEntry` binds `operand_a = left` from `FrameRowC::RightF` and `operand_b = Some(value)` from `ControlC::Return { value }`; `RuleIdC::BHelperScalarTag` reads `*reg_a = pre.u128_of(operand_a)?`; `RuleIdC::BHelperScalarArithmetic` reads `let n = pre.u128_of(&b)?` and for `Some(layout)` calls `checked_word_result(layout, UInt128Operation::{Add,Sub,Mul}, *reg_a, n)`. `checked_word_result` constructs `let full = ArithmeticLowering::new(layout, operation).execute(left, right);` and maps `full.checked()` (`Ok(n) => CheckedArith::Value { n }`, `Err(_) => CheckedArith::Overflow`). Overflow sets `ScalarStaged::PrimError { code: 10 }`, which `RuleIdC::BHelperScalarFailure` lowers to `HelperResultC::ScalarError { code: 10 }`. No new guards, bounds, policy, or charges are added."
      },
      {
        "body": "BODY 06, BODY 09 & BODY 11 (word_lowering.rs, word_arithmetic.rs, UIREQ-030)",
        "observation": "`ArithmeticLowering::execute` delegates directly to `checked_u128_add_32`, `checked_u128_sub_32`, `checked_u128_mul_32`, `checked_u128_add_64`, `checked_u128_sub_64`, `checked_u128_mul_64` in `BODY 09`. `BODY 09` retains full intermediates (`FullAdd32`, `FullSub32`, `FullProduct32`, `FullAdd64`, `FullSub64`, `FullProduct64`, including the full 256-bit products `low: [u32; 4], high: [u32; 4]` and `low: [u64; 2], high: [u64; 2]`), preserves distinct `CarryBit(pub bool)`, `BorrowBit(pub bool)`, `MulCarry32(pub u32)`, and `MulCarry64(pub u64)` types, and selects `Err(10)` iff `carry_out.0`, `borrow_out.0`, or `high.iter().any(|&word| word != 0)`. No circuit/native/cost winner is claimed."
      },
      {
        "body": "BODY 08 & BODY 14 (tests/sherlock_word_lowering.rs & R197 EXPECTED-NORMAL.json)",
        "observation": "`BODY 08` tests the three frozen R197 normal cases D26 (`4294967293 + 8 = 4294967301`), D27 (`18446744073709551625 - 7 = 18446744073709551618`), and D28 (`281474976710657 * 3 = 844424930131971`) across `Word32` and `Word64` both via `ArithmeticLowering::execute` full intermediates and via `run_draft_host_with_word_layout` with event sink and `OutcomeC::Halted { value }` `DataRow::U128` verification."
      }
    ],
    "mandatory_fixes": [],
    "remaining_gaps": [
      "All 7 production source files and `crates/mori-core/tests/sherlock_word_lowering.rs` are SOURCE_ONLY / UNRUN; compilation and test execution have not yet occurred.",
      "Unrelated merged nominal `shared.rs` / `owner_prepare` bodies are outside the inline 7-file word-lowering slice and are not audited here.",
      "Per BODY 12 (CURRENT-REQUIRED-GAPS.json), `BODY 08` covers only normal (Ok) cases; UIREQ-030 completion requires runtime verification of `Err(10)` / `OutcomeC::Rejected { code: 10 }` for addition overflow, subtraction underflow, and nonzero high-128 multiplication across both `Word32` and `Word64` on the real selected public path and full intermediates. New test source is authored in Task B below and requires independent non-author review before execution."
    ]
  },
  "five_normal_ops_audit": {
    "verdict": "ELIGIBLE_FOR_ROOT_EXECUTION",
    "operations_reviewed": [
      "compileComposedNormalCoreRlib",
      "compileNewPublicWordLoweringTests",
      "selected_lowering_retains_original_independent_named_full_results",
      "public_selected32_source_runs_original_add_sub_mul",
      "public_selected64_source_runs_original_add_sub_mul"
    ],
    "exact_new_selectors": [
      "selected_lowering_retains_original_independent_named_full_results",
      "public_selected32_source_runs_original_add_sub_mul",
      "public_selected64_source_runs_original_add_sub_mul"
    ],
    "audit_notes": "All 5 operations in BODY 13 (PROPOSED.json) use bounded `timeout` and `prlimit` wrappers, compile the exact snapshot `lib.rs` and `tests/sherlock_word_lowering.rs` without running an old test cohort, and invoke the binary with `--exact <selector> --test-threads=1 --nocapture` matching the 3 `#[test]` functions in BODY 08 against frozen R197 D26-D28 expectations.",
    "mandatory_fixes": []
  },
  "new_error10_test_source": {
    "status": "SOURCE_ONLY_UNRUN_NOT_SELF_APPROVED",
    "path": "crates/mori-core/tests/sherlock_word_lowering_error10.rs",
    "independent_math_basis": {
      "add_overflow_u128_max_plus_1": {
        "expression": "340282366920938463463374607431768211455 + 1",
        "equation": "(2^128 - 1) + 1 = 2^128 = 0 + 2^128 * 1",
        "word32_full": { "low": [0, 0, 0, 0], "carry_out": true },
        "word64_full": { "low": [0, 0], "carry_out": true },
        "checked": "Err(10)",
        "public_outcome": "OutcomeC::Rejected { code: 10 }"
      },
      "sub_underflow_0_minus_1": {
        "expression": "0 - 1",
        "equation": "0 - 1 = (2^128 - 1) - 2^128 * 1",
        "word32_full": {
          "low": [4294967295, 4294967295, 4294967295, 4294967295],
          "borrow_out": true
        },
        "word64_full": {
          "low": [18446744073709551615, 18446744073709551615],
          "borrow_out": true
        },
        "checked": "Err(10)",
        "public_outcome": "OutcomeC::Rejected { code: 10 }"
      },
      "mul_nonzero_high128_u128_max_times_2": {
        "expression": "340282366920938463463374607431768211455 * 2",
        "equation": "(2^128 - 1) * 2 = 2^129 - 2 = (2^128 - 2) + 2^128 * 1",
        "word32_full": {
          "low": [4294967294, 4294967295, 4294967295, 4294967295],
          "high": [1, 0, 0, 0]
        },
        "word64_full": {
          "low": [18446744073709551614, 18446744073709551615],
          "high": [1, 0]
        },
        "checked": "Err(10)",
        "public_outcome": "OutcomeC::Rejected { code: 10 }"
      }
    },
    "coverage": [
      "FullArithmeticResult::Add32 / Add64 on (u128::MAX, 1): full.low, full.carry_out == CarryBit(true), inner .checked == Err(10), and FullArithmeticResult::checked() == Err(10)",
      "FullArithmeticResult::Sub32 / Sub64 on (0, 1): full.low, full.borrow_out == BorrowBit(true), inner .checked == Err(10), and FullArithmeticResult::checked() == Err(10)",
      "FullArithmeticResult::Mul32 / Mul64 on (u128::MAX, 2): full.low, full.high (nonzero high128 limb 0 == 1), inner .checked == Err(10), and FullArithmeticResult::checked() == Err(10)",
      "Public run_draft_host_with_word_layout with WordLayout::Word32 and event sink on add overflow, sub underflow, and mul nonzero-high128 returning Ok(WordLoweredDraftHostRun) with semantic OutcomeC::Rejected { code: 10 } in both completed.summary.outcome and observe().image.outcome",
      "Public run_draft_host_with_word_layout with WordLayout::Word64 and event sink on add overflow, sub underflow, and mul nonzero-high128 returning Ok(WordLoweredDraftHostRun) with semantic OutcomeC::Rejected { code: 10 } in both completed.summary.outcome and observe().image.outcome"
    ],
    "complete_source": "//! Authored UIREQ-030 error10 integration tests for selected word lowering.\n//! SOURCE_ONLY_UNRUN_NOT_SELF_APPROVED. Requires independent non-author review\n//! before any compilation or test execution.\n//! Mathematical expectations are derived directly from integer arithmetic:\n//! - Add overflow: (2^128 - 1) + 1 = 0 + 2^128 * 1 -> Err(10)\n//! - Sub underflow: 0 - 1 = (2^128 - 1) - 2^128 * 1 -> Err(10)\n//! - Mul nonzero high-128: (2^128 - 1) * 2 = (2^128 - 2) + 2^128 * 1 -> Err(10)\nuse mori_core::sherlock::helper_machine::shared::OutcomeC;\nuse mori_core::sherlock_compile_artifact::draft_execution::run_draft_host_with_word_layout;\nuse mori_core::sherlock_word_arithmetic::{BorrowBit, CarryBit};\nuse mori_core::sherlock_word_lowering::{\n    ArithmeticLowering, FullArithmeticResult, UInt128Operation, WordLayout,\n};\n\nconst CONFIG: &str = include_str!(\"fixtures/draft-compile/config.json\");\nconst TARGET: &str = include_str!(\"fixtures/draft-compile/target.json\");\n\nconst U128_MAX: u128 = 340282366920938463463374607431768211455;\nconst U32_MAX: u32 = 4294967295;\nconst U64_MAX: u64 = 18446744073709551615;\n\n#[test]\nfn selected_lowering_retains_full_intermediates_before_error10_selection() {\n    use UInt128Operation::{Add, Mul, Sub};\n    use WordLayout::{Word32, Word64};\n\n    let add32 = ArithmeticLowering::new(Word32, Add).execute(U128_MAX, 1);\n    match add32 {\n        FullArithmeticResult::Add32(r) => {\n            assert_eq!(r.full.low, [0, 0, 0, 0]);\n            assert_eq!(r.full.carry_out, CarryBit(true));\n            assert_eq!(r.checked, Err(10));\n        }\n        other => panic!(\"wrong selected component: {other:?}\"),\n    }\n    assert_eq!(add32.checked(), Err(10));\n\n    let add64 = ArithmeticLowering::new(Word64, Add).execute(U128_MAX, 1);\n    match add64 {\n        FullArithmeticResult::Add64(r) => {\n            assert_eq!(r.full.low, [0, 0]);\n            assert_eq!(r.full.carry_out, CarryBit(true));\n            assert_eq!(r.checked, Err(10));\n        }\n        other => panic!(\"wrong selected component: {other:?}\"),\n    }\n    assert_eq!(add64.checked(), Err(10));\n\n    let sub32 = ArithmeticLowering::new(Word32, Sub).execute(0, 1);\n    match sub32 {\n        FullArithmeticResult::Sub32(r) => {\n            assert_eq!(r.full.low, [U32_MAX, U32_MAX, U32_MAX, U32_MAX]);\n            assert_eq!(r.full.borrow_out, BorrowBit(true));\n            assert_eq!(r.checked, Err(10));\n        }\n        other => panic!(\"wrong selected component: {other:?}\"),\n    }\n    assert_eq!(sub32.checked(), Err(10));\n\n    let sub64 = ArithmeticLowering::new(Word64, Sub).execute(0, 1);\n    match sub64 {\n        FullArithmeticResult::Sub64(r) => {\n            assert_eq!(r.full.low, [U64_MAX, U64_MAX]);\n            assert_eq!(r.full.borrow_out, BorrowBit(true));\n            assert_eq!(r.checked, Err(10));\n        }\n        other => panic!(\"wrong selected component: {other:?}\"),\n    }\n    assert_eq!(sub64.checked(), Err(10));\n\n    let mul32 = ArithmeticLowering::new(Word32, Mul).execute(U128_MAX, 2);\n    match mul32 {\n        FullArithmeticResult::Mul32(r) => {\n            assert_eq!(r.full.low, [U32_MAX - 1, U32_MAX, U32_MAX, U32_MAX]);\n            assert_eq!(r.full.high, [1, 0, 0, 0]);\n            assert_eq!(r.checked, Err(10));\n        }\n        other => panic!(\"wrong selected component: {other:?}\"),\n    }\n    assert_eq!(mul32.checked(), Err(10));\n\n    let mul64 = ArithmeticLowering::new(Word64, Mul).execute(U128_MAX, 2);\n    match mul64 {\n        FullArithmeticResult::Mul64(r) => {\n            assert_eq!(r.full.low, [U64_MAX - 1, U64_MAX]);\n            assert_eq!(r.full.high, [1, 0]);\n            assert_eq!(r.checked, Err(10));\n        }\n        other => panic!(\"wrong selected component: {other:?}\"),\n    }\n    assert_eq!(mul64.checked(), Err(10));\n}\n\nfn run_error10_source(layout: WordLayout, expression: &str) {\n    let source = format!(\n        \"language \\\"moriarty-sherlock-draft/1\\\"; program WordArithmeticError10 {{ entry {expression}; }}\"\n    );\n    let mut actual_events = 0usize;\n    let selected = run_draft_host_with_word_layout(\n        &source,\n        CONFIG,\n        TARGET,\n        layout,\n        &mut |_| actual_events += 1,\n    )\n    .unwrap_or_else(|err| panic!(\"selected host run returned CompileFailure: {err:?}\"));\n    assert_eq!(selected.word_layout, layout);\n    assert!(actual_events > 0, \"actual owner run, not compile-only preparation\");\n    assert_eq!(selected.run.completed.events_forwarded, actual_events);\n    match &selected.run.completed.summary.outcome {\n        OutcomeC::Rejected { code } => assert_eq!(*code, 10),\n        other => panic!(\"expected semantic Rejected {{ code: 10 }} in summary, got {other:?}\"),\n    }\n    let observation = selected\n        .run\n        .observe()\n        .unwrap_or_else(|err| panic!(\"real bound observation failed: {err:?}\"));\n    match observation.image.outcome {\n        OutcomeC::Rejected { code } => assert_eq!(*code, 10),\n        other => panic!(\"expected semantic Rejected {{ code: 10 }} in observation, got {other:?}\"),\n    }\n}\n\n#[test]\nfn public_selected32_source_rejects_overflow_underflow_and_high128_mul_with_error10() {\n    run_error10_source(\n        WordLayout::Word32,\n        \"340282366920938463463374607431768211455 + 1\",\n    );\n    run_error10_source(WordLayout::Word32, \"0 - 1\");\n    run_error10_source(\n        WordLayout::Word32,\n        \"340282366920938463463374607431768211455 * 2\",\n    );\n}\n\n#[test]\nfn public_selected64_source_rejects_overflow_underflow_and_high128_mul_with_error10() {\n    run_error10_source(\n        WordLayout::Word64,\n        \"340282366920938463463374607431768211455 + 1\",\n    );\n    run_error10_source(WordLayout::Word64, \"0 - 1\");\n    run_error10_source(\n        WordLayout::Word64,\n        \"340282366920938463463374607431768211455 * 2\",\n    );\n}\n"
  },
  "later_fresh_nonauthor_test_review_required": true
}

