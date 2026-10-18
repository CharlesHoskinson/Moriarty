{
  "role_and_coverage": {
    "reviewer_cid": "d2f4cab5-a214-4450-af91-35feb60db39f",
    "cwd": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/142-r2b-final-nonsecurity/review-session",
    "primary_model": "claude-opus-5-5",
    "actual_model": "gemini-4-argon-high",
    "effort": "high",
    "fallback_status": "standing_observed_opus_quota_outage_fallback",
    "backendcanonical_attestation": false,
    "tools_used": 0,
    "execution_performed": false,
    "authorship_separation": {
      "assignment_A_nonauthor_reviewer": "d2f4cab5-a214-4450-af91-35feb60db39f authored zero bytes of the 7 changed production Rust files, 2 docs, or crates/mori-core/tests/sherlock_word_lowering.rs (authored by ordinary delegated Codex plus 29f6efee-2b5e-4f3f-b663-bad4292ff558 for host_owner_engine_with_word_layout in owner_host.rs).",
      "assignment_B_security_test_author": "d2f4cab5-a214-4450-af91-35feb60db39f authored crates/mori-core/tests/sherlock_word_lowering_error10.rs as Root's designated security test author. It is NEW_SOURCE_ONLY_UNRUN_NOT_SELF_APPROVED and requires a separate later non-author review by 29f6efee-2b5e-4f3f-b663-bad4292ff558 scheduled by Root."
    },
    "inspected_bodies": [
      {"body": "BODY 01", "path": "crates/mori-core/src/lib.rs", "bytes": 1859, "sha256": "1bb580dc308964a3065f6c248c072ed0aa32d55291e4daac46e949b1bde770cb"},
      {"body": "BODY 02", "path": "crates/mori-core/src/sherlock/compile_artifact/draft_execution.rs", "bytes": 8096, "sha256": "3c9192fe7239dde42f13ff05210f3eb7b2ccdcfec2ee3f3e6c489e386858f2a4"},
      {"body": "BODY 03", "path": "crates/mori-core/src/sherlock/helper_machine/families/mod.rs", "bytes": 5161, "sha256": "1e6923439155cf6ab39e996351f7785d49995d76b0656b2a059f4c869d0f185f"},
      {"body": "BODY 04", "path": "crates/mori-core/src/sherlock/helper_machine/families/scalar.rs", "bytes": 650, "sha256": "f1d0ea6216b28884ee30c159a941ca787943ac8cbc1fb7a7c081689fde05d8ea"},
      {"body": "BODY 05", "path": "crates/mori-core/src/sherlock/helper_machine/fragments/scalar.proposed.rs", "bytes": 15392, "sha256": "b80bc107e98e5a35e06cef7e127059a85b8c032e5272a0fd570f199860d843ea"},
      {"body": "BODY 06", "path": "crates/mori-core/src/sherlock/word_lowering.rs", "bytes": 3640, "sha256": "a7c366cdb5c1640991a1ad12486203cc40a8be449184e2e7dd6fc4762def734f"},
      {"body": "BODY 07", "path": "crates/mori-core/src/sherlock/helper_machine/shared/owner_host.rs", "bytes": 6975, "sha256": "7cd051dfdb38426753a015b52964093a81433a8fa925616c1a5f3d56f504786b"},
      {"body": "BODY 08", "path": "crates/mori-core/tests/sherlock_word_lowering.rs", "bytes": 4898, "sha256": "00ff50d3ff928b47fe03d1b412f61192f355868e5cf31b51f8d304a2db5571e5"},
      {"body": "BODY 09", "path": "docs/sherlock/reference/typed-word-arithmetic.md", "bytes": 9207, "sha256": "51e3dd30ef40ab5fb09b0202e88a9d561ab8ba84dd278349d03c2a1a4e05a282"},
      {"body": "BODY 10", "path": "docs/sherlock/how-to/use-typed-word-arithmetic.md", "bytes": 3885, "sha256": "5c8d56e2046bf937b01701bbcc3f77ba7337721126c1643c6589dc8b2676f1ef"},
      {"body": "BODY 11", "path": "crates/mori-core/src/sherlock/helper_machine/shared/owner_host.rs (preimage)", "bytes": 6480, "sha256": "b6d0524431df4e73898853b073ba9c3f5a212fd83f3dd4015c6a4e1eda1d0017"},
      {"body": "BODY 12", "path": "214-r2-030-word-lowering/OWNER-CONSTRUCTOR-RESPONSE.json", "bytes": 2117, "sha256": "aaad99a4d5c4bb18d1297954cb49d27ca953841122de3a34181251bd90ec7146"},
      {"body": "BODY 13", "path": "214-r2-030-word-lowering/OWNER-MATERIALIZATION.json", "bytes": 1351, "sha256": "ffc1a219aa20e47fd4b77721ba8b8aa488b4349746bd2dc3b2f6a2444d0fa409"},
      {"body": "BODY 14", "path": "214-r2-030-word-lowering/API-DECLARATIONS.rs.txt", "bytes": 3893, "sha256": "301e4fdbbe417f789a56594173c85c52611fc85b3575f78087274d33638fc780"},
      {"body": "BODY 15", "path": "214-r2-030-word-lowering/OWNER-RELEVANT-APIS.rs.txt", "bytes": 5444, "sha256": "7222c9d6e9d5948fc13e97d35df407027c44dbbaad5a989e85f275c2789d4789"},
      {"body": "BODY 16", "path": "crates/mori-core/src/sherlock/word_arithmetic.rs", "bytes": 12214, "sha256": "f10af3a3c7ba9c00368f61964523a4643f58970147f863b963d4854533bfcf45"},
      {"body": "BODY 17", "path": "197-r2-030-word-arith-v1/API-DECLARATIONS-v1.rs.txt", "bytes": 2149, "sha256": "61b764e2e069650470322706f9814017db5aa4e173a153a46a528a862583823d"},
      {"body": "BODY 18", "path": "197-r2-030-word-arith-v1/independent_normal_tests.rs", "bytes": 8073, "sha256": "90caacfef1151bd08505aa188d9fb7ab588a02e8836108b63a3519749eb6443d"},
      {"body": "BODY 19", "path": "197-r2-030-word-arith-v1/EXPECTED-NORMAL.json", "bytes": 1957, "sha256": "c887f52315f38ab9375f2136dd2b7a982b9ecfc9fb9b77525225673ce9c04416"},
      {"body": "BODY 20", "path": "197-r2-030-word-arith-v1/SCOPE-QUALIFICATION.json", "bytes": 2257, "sha256": "dd610798e3b3f0e5484417a53bf763051be29eb010ab94a3d9f35c8656c709e4"},
      {"body": "BODY 21", "path": "214-r2-030-word-lowering/UIREQ030-FULL-ORIGINAL.md", "bytes": 2743, "sha256": "ed95f8b14ddaa1240a76bb626ab1fd482169ebdc128588f8db7a8a3d33dd859f"},
      {"body": "BODY 22", "path": "214-r2-030-word-lowering/CURRENT030-MISSING-AND-ROOT-SCOPE.json", "bytes": 1445, "sha256": "a5a43bc84be81036c66d8e989ac964be70f03e384e3e37c471b07ef31bf2fa12"},
      {"body": "BODY 23", "path": "214-r2-030-word-lowering/PROPOSED.json", "bytes": 3899, "sha256": "9de71c99a5fa77cf931b114d608e3be4f86b24d23c1a54d2b4534572ba7b2615"},
      {"body": "BODY 24", "path": "crates/mori-core/tests/fixtures/draft-compile/config.json", "bytes": 329, "sha256": "0763c35f18e3b7e004f862939c70f4dc0332b0d323453bd1626da17cc6d51b1b"},
      {"body": "BODY 25", "path": "crates/mori-core/tests/fixtures/draft-compile/target.json", "bytes": 616, "sha256": "b7b9069a120a50e0e088d9c8ae4405aa88015cd93bb185f33658cfdf257df840"}
    ]
  },
  "source_review": {
    "verdict": "APPROVE",
    "mandatory_fixes": [],
    "remaining_gaps": [
      "UIREQ-030.P5 (proof-system / circuit cost comparison and native word-layout winner selection) remains open and out of scope of this host lowering slice.",
      "UIREQ-030.P6 (formal refinement obligations for typed word arithmetic against scalar UInt128 semantics) remains open.",
      "Candidate source and tests are currently SOURCE_ONLY / UNRUN; the 5 proposed normal QA operations in PROPOSED.json and the newly authored crates/mori-core/tests/sherlock_word_lowering_error10.rs (after independent non-author review by 29f6efee-2b5e-4f3f-b663-bad4292ff558) must be compiled and executed via Root-selected dispatches before claiming runtime evidence."
    ],
    "findings": [
      {
        "component": "crates/mori-core/src/sherlock/word_lowering.rs & crates/mori-core/src/lib.rs",
        "assessment": "SOUND",
        "detail": "lib.rs (1bb580dc...) exports pub mod sherlock_word_arithmetic and pub mod sherlock_word_lowering. word_lowering.rs (a7c366cd...) defines WordLayout::{Word32, Word64}, UInt128Operation::{Add, Sub, Mul}, ArithmeticLowering, and FullArithmeticResult::{Word32Add, Word32Sub, Word32Mul, Word64Add, Word64Sub, Word64Mul}. ArithmeticLowering::execute delegates directly to CheckedUInt128Add32/Sub32/Mul32/Add64/Sub64/Mul64::execute in word_arithmetic.rs (f10af3a3...), preserving the full named intermediate record (low: [u32; 4] / [u64; 2], carry_out: CarryBit, borrow_out: BorrowBit, high: [u32; 4] / [u64; 2]) before FullArithmeticResult::checked() invokes the underlying .checked() -> Result<u128, u32>."
      },
      {
        "component": "crates/mori-core/src/sherlock/helper_machine/fragments/scalar.proposed.rs & families/{mod.rs, scalar.rs}",
        "assessment": "SOUND",
        "detail": "families/mod.rs (1e692343...) preserves HostFamilyPlans unchanged and introduces pub(crate) struct WordFamilyPlans { layout: WordLayout } whose plan_scalar dispatches via scalar::dispatch_word(sel, pre, guards, self.layout) while all other 10 family methods delegate identically to HostFamilyPlans. In scalar.proposed.rs (b80bc107...), plan_scalar passes Option<WordLayout> only to RuleIdC::BHelperScalarArithmetic; when Some(layout), ScalarMode::{Add, Sub, Mul} invoke checked_word_result(layout, op, *reg_a, n), which calls ArithmeticLowering::new(layout, operation).execute(left, right) and maps full.checked() Ok(n) => CheckedArith::Value { n } and Err(_) => CheckedArith::Overflow => ScalarStaged::PrimError { code: 10 }. When None, pure_leaves::checked_add/sub/mul remain unchanged. Evaluation order (left = *reg_a from ScalarPhase::First, right = n from ScalarPhase::Second), non-U128 type errors (code: 11), guards (GScalarAllocNode, GScalarFields1, GScalarFields2), and BHelperScalarFailure (HelperResultC::ScalarError { code: 10 }) are preserved."
      },
      {
        "component": "crates/mori-core/src/sherlock/helper_machine/shared/owner_host.rs & compile_artifact/draft_execution.rs",
        "assessment": "SOUND",
        "detail": "owner_host.rs (7cd051df..., 6,975 B) matches the exact 495-byte insertion (sha256: 40a74de0...) at byte offset 1963 into preimage BODY 11 (b6d05244..., 6,480 B) authored by 29f6efee-2b5e-4f3f-b663-bad4292ff558. host_owner_engine_with_word_layout(layout) constructs CheckedOwnerEngine::new_reviewed(WordFamilyPlans::new(layout), PrepareDeps { ... }) with the exact same reviewed prepare dependencies as host_owner_engine(). draft_execution.rs (3c9192fe...) preserves run_draft_host via run_draft_host_impl(..., None) and adds run_draft_host_with_word_layout(..., word_layout) returning Result<WordLoweredDraftHostRun, DraftRunError>, genuinely routing source compilation and bounded draft execution through the selected WordLayout."
      },
      {
        "component": "docs/sherlock/reference/typed-word-arithmetic.md & docs/sherlock/how-to/use-typed-word-arithmetic.md",
        "assessment": "SOUND",
        "detail": "Both documentation files (51e3dd30... and 5c8d56e2...) accurately match the exported Rust signatures, explicitly record that the R214 lowering composition is SOURCE_ONLY / UNRUN, preserve the scope distinction from R201 (component-level normal leaf evidence) and R206 (component-level boundary/error10 leaf evidence), make no native/circuit cost or winner claim, and list UIREQ-030.P5 and UIREQ-030.P6 as open."
      }
    ]
  },
  "normal_QA_review": {
    "verdict": "APPROVE_PROPOSED_FIVE_ORDINARY_OPERATIONS",
    "exact3newselectors": [
      "selected_lowering_retains_original_independent_named_full_results",
      "public_selected32_source_runs_original_add_sub_mul",
      "public_selected64_source_runs_original_add_sub_mul"
    ],
    "mandatory_fixes": [],
    "findings": [
      {
        "aspect": "Test source correctness & R197 oracle preservation (crates/mori-core/tests/sherlock_word_lowering.rs, 00ff50d3...)",
        "detail": "selected_lowering_retains_original_independent_named_full_results verifies all 6 combinations of {Word32, Word64} x {Add, Sub, Mul} on the exact R197 EXPECTED-NORMAL.json (c887f523...) D26/D27/D28 inputs, asserting exact equality of FullAdd32, FullAdd64, FullSub32, FullSub64, FullProduct32, FullProduct64 (all limb arrays, CarryBit(false), BorrowBit(false), high limb arrays) and full.checked() == Ok(expected). public_selected32_source_runs_original_add_sub_mul and public_selected64_source_runs_original_add_sub_mul run the 3 source expressions through run_draft_host_with_word_layout using existing fixtures/draft-compile/{config.json,target.json}, checking lowered.word_layout, actual_events > 0, OutcomeC::Halted { value: RefC::Data(index) } in both completed.summary.outcome and observation.image.outcome, and DataRow::U128 { n } in both completed.final_state.data and observation.image.data."
      },
      {
        "aspect": "Proposed 5-operation ordinary QA manifest (PROPOSED.json, 9de71c99...)",
        "detail": "The 5 operations (compileComposedNormalCoreRlib, compileNewPublicWordLoweringTests, runSelectedLoweringRetainsOriginalIndependentNamedFullResults, runPublicSelected32SourceRunsOriginalAddSubMul, runPublicSelected64SourceRunsOriginalAddSubMul) use --offline, bounded single-test --exact execution (--test-threads=1), no git/network/filesystem writes outside target, and match the exact 3 test selectors in tests/sherlock_word_lowering.rs."
      }
    ]
  },
  "authored_error10_tests": {
    "status": "NEW_SOURCE_ONLY_UNRUN_NOT_SELF_APPROVED",
    "path": "crates/mori-core/tests/sherlock_word_lowering_error10.rs",
    "complete_source": "use mori_core::sherlock::compile_artifact::draft_execution::run_draft_host_with_word_layout;\nuse mori_core::sherlock::helper_machine::OutcomeC;\nuse mori_core::sherlock_word_arithmetic::{\n    BorrowBit, CarryBit, FullAdd32, FullAdd64, FullProduct32, FullProduct64, FullSub32, FullSub64,\n};\nuse mori_core::sherlock_word_lowering::{\n    ArithmeticLowering, FullArithmeticResult, UInt128Operation, WordLayout,\n};\n\nconst DRAFT_CONFIG_JSON: &str = include_str!(\"fixtures/draft-compile/config.json\");\nconst DRAFT_TARGET_JSON: &str = include_str!(\"fixtures/draft-compile/target.json\");\nconst UINT128_MAX: u128 = 340282366920938463463374607431768211455u128;\n\nfn assert_public_error10_rejection(source: &str, layout: WordLayout) {\n    let lowered = run_draft_host_with_word_layout(DRAFT_CONFIG_JSON, DRAFT_TARGET_JSON, source, layout)\n        .expect(\"draft host execution with selected word layout should complete with a machine outcome\");\n    assert_eq!(lowered.word_layout, layout);\n\n    let observation = lowered.run.observe();\n    let completed = lowered.run.completed();\n    assert!(\n        completed.summary.actual_events > 0,\n        \"helper machine must execute events before rejecting with error 10\"\n    );\n    assert_eq!(completed.summary.outcome, OutcomeC::Rejected { code: 10 });\n    assert_eq!(observation.image.outcome, &OutcomeC::Rejected { code: 10 });\n}\n\n#[test]\nfn selected_lowering_retains_full_intermediates_and_error10_for_word32_and_word64() {\n    let add32 = ArithmeticLowering::new(WordLayout::Word32, UInt128Operation::Add)\n        .execute(UINT128_MAX, 1);\n    assert_eq!(\n        add32,\n        FullArithmeticResult::Word32Add(FullAdd32 {\n            low: [0, 0, 0, 0],\n            carry_out: CarryBit(true),\n        })\n    );\n    assert_eq!(add32.checked(), Err(10));\n\n    let add64 = ArithmeticLowering::new(WordLayout::Word64, UInt128Operation::Add)\n        .execute(UINT128_MAX, 1);\n    assert_eq!(\n        add64,\n        FullArithmeticResult::Word64Add(FullAdd64 {\n            low: [0, 0],\n            carry_out: CarryBit(true),\n        })\n    );\n    assert_eq!(add64.checked(), Err(10));\n\n    let sub32 = ArithmeticLowering::new(WordLayout::Word32, UInt128Operation::Sub)\n        .execute(0, 1);\n    assert_eq!(\n        sub32,\n        FullArithmeticResult::Word32Sub(FullSub32 {\n            low: [4294967295, 4294967295, 4294967295, 4294967295],\n            borrow_out: BorrowBit(true),\n        })\n    );\n    assert_eq!(sub32.checked(), Err(10));\n\n    let sub64 = ArithmeticLowering::new(WordLayout::Word64, UInt128Operation::Sub)\n        .execute(0, 1);\n    assert_eq!(\n        sub64,\n        FullArithmeticResult::Word64Sub(FullSub64 {\n            low: [18446744073709551615, 18446744073709551615],\n            borrow_out: BorrowBit(true),\n        })\n    );\n    assert_eq!(sub64.checked(), Err(10));\n\n    let mul32 = ArithmeticLowering::new(WordLayout::Word32, UInt128Operation::Mul)\n        .execute(UINT128_MAX, 2);\n    assert_eq!(\n        mul32,\n        FullArithmeticResult::Word32Mul(FullProduct32 {\n            low: [4294967294, 4294967295, 4294967295, 4294967295],\n            high: [1, 0, 0, 0],\n        })\n    );\n    assert_eq!(mul32.checked(), Err(10));\n\n    let mul64 = ArithmeticLowering::new(WordLayout::Word64, UInt128Operation::Mul)\n        .execute(UINT128_MAX, 2);\n    assert_eq!(\n        mul64,\n        FullArithmeticResult::Word64Mul(FullProduct64 {\n            low: [18446744073709551614, 18446744073709551615],\n            high: [1, 0],\n        })\n    );\n    assert_eq!(mul64.checked(), Err(10));\n}\n\n#[test]\nfn public_selected32_source_rejects_overflow_underflow_and_high_product_with_error10() {\n    assert_public_error10_rejection(\n        \"340282366920938463463374607431768211455 + 1\",\n        WordLayout::Word32,\n    );\n    assert_public_error10_rejection(\"0 - 1\", WordLayout::Word32);\n    assert_public_error10_rejection(\n        \"340282366920938463463374607431768211455 * 2\",\n        WordLayout::Word32,\n    );\n}\n\n#[test]\nfn public_selected64_source_rejects_overflow_underflow_and_high_product_with_error10() {\n    assert_public_error10_rejection(\n        \"340282366920938463463374607431768211455 + 1\",\n        WordLayout::Word64,\n    );\n    assert_public_error10_rejection(\"0 - 1\", WordLayout::Word64);\n    assert_public_error10_rejection(\n        \"340282366920938463463374607431768211455 * 2\",\n        WordLayout::Word64,\n    );\n}\n",
    "independent_derivation": [
      {
        "case": "UInt128MAX + 1 (340282366920938463463374607431768211455 + 1)",
        "math": "(2^128 - 1) + 1 = 2^128 = 0 + 1 * 2^128",
        "word32_full": "FullAdd32 { low: [0, 0, 0, 0], carry_out: CarryBit(true) }",
        "word64_full": "FullAdd64 { low: [0, 0], carry_out: CarryBit(true) }",
        "checked_and_host_outcome": "full.checked() == Err(10); BHelperScalarArithmetic maps Err(_) to CheckedArith::Overflow -> ScalarStaged::PrimError { code: 10 } -> BHelperScalarFailure -> HelperResultC::ScalarError { code: 10 } -> OutcomeC::Rejected { code: 10 }."
      },
      {
        "case": "0 - 1",
        "math": "0 - 1 = -1 = (2^128 - 1) - 1 * 2^128",
        "word32_full": "FullSub32 { low: [4294967295, 4294967295, 4294967295, 4294967295], borrow_out: BorrowBit(true) }",
        "word64_full": "FullSub64 { low: [18446744073709551615, 18446744073709551615], borrow_out: BorrowBit(true) }",
        "checked_and_host_outcome": "full.checked() == Err(10); BHelperScalarArithmetic maps Err(_) to CheckedArith::Overflow -> ScalarStaged::PrimError { code: 10 } -> BHelperScalarFailure -> OutcomeC::Rejected { code: 10 }."
      },
      {
        "case": "UInt128MAX * 2 (340282366920938463463374607431768211455 * 2)",
        "math": "(2^128 - 1) * 2 = 2^129 - 2 = (2^128 - 2) + 1 * 2^128",
        "word32_full": "FullProduct32 { low: [4294967294, 4294967295, 4294967295, 4294967295], high: [1, 0, 0, 0] }",
        "word64_full": "FullProduct64 { low: [18446744073709551614, 18446744073709551615], high: [1, 0] }",
        "checked_and_host_outcome": "high limbs are nonzero (high[0] == 1), so full.checked() == Err(10); BHelperScalarArithmetic maps Err(_) to CheckedArith::Overflow -> ScalarStaged::PrimError { code: 10 } -> BHelperScalarFailure -> OutcomeC::Rejected { code: 10 }."
      }
    ],
    "input_coverage": {
      "layouts": ["WordLayout::Word32", "WordLayout::Word64"],
      "operations": ["UInt128Operation::Add", "UInt128Operation::Sub", "UInt128Operation::Mul"],
      "sources": [
        "340282366920938463463374607431768211455 + 1",
        "0 - 1",
        "340282366920938463463374607431768211455 * 2"
      ],
      "test_functions": [
        "selected_lowering_retains_full_intermediates_and_error10_for_word32_and_word64",
        "public_selected32_source_rejects_overflow_underflow_and_high_product_with_error10",
        "public_selected64_source_rejects_overflow_underflow_and_high_product_with_error10"
      ]
    }
  },
  "fresh_nonauthor_review_required": true,
  "scope_and_non_inference_attestation": {
    "source_only_unrun": true,
    "authored_error10_self_approved": false,
    "later_reviewer_for_authored_error10": "29f6efee-2b5e-4f3f-b663-bad4292ff558 (scheduled solely by Root after its reserved nominal task)",
    "whole_uireq030_closed": false,
    "native_circuit_winner_or_refinement_claimed": false
  }
}

