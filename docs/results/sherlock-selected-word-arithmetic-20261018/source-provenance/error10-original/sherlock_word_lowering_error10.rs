//! Authored UIREQ-030 error10 integration tests for selected word lowering.
//! SOURCE_ONLY_UNRUN_NOT_SELF_APPROVED. Requires independent non-author review
//! before any compilation or test execution.
//! Mathematical expectations are derived directly from integer arithmetic:
//! - Add overflow: (2^128 - 1) + 1 = 0 + 2^128 * 1 -> Err(10)
//! - Sub underflow: 0 - 1 = (2^128 - 1) - 2^128 * 1 -> Err(10)
//! - Mul nonzero high-128: (2^128 - 1) * 2 = (2^128 - 2) + 2^128 * 1 -> Err(10)
use mori_core::sherlock::helper_machine::shared::OutcomeC;
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host_with_word_layout;
use mori_core::sherlock_word_arithmetic::{BorrowBit, CarryBit};
use mori_core::sherlock_word_lowering::{
    ArithmeticLowering, FullArithmeticResult, UInt128Operation, WordLayout,
};

const CONFIG: &str = include_str!("fixtures/draft-compile/config.json");
const TARGET: &str = include_str!("fixtures/draft-compile/target.json");

const U128_MAX: u128 = 340282366920938463463374607431768211455;
const U32_MAX: u32 = 4294967295;
const U64_MAX: u64 = 18446744073709551615;

#[test]
fn selected_lowering_retains_full_intermediates_before_error10_selection() {
    use UInt128Operation::{Add, Mul, Sub};
    use WordLayout::{Word32, Word64};

    let add32 = ArithmeticLowering::new(Word32, Add).execute(U128_MAX, 1);
    match add32 {
        FullArithmeticResult::Add32(r) => {
            assert_eq!(r.full.low, [0, 0, 0, 0]);
            assert_eq!(r.full.carry_out, CarryBit(true));
            assert_eq!(r.checked, Err(10));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    assert_eq!(add32.checked(), Err(10));

    let add64 = ArithmeticLowering::new(Word64, Add).execute(U128_MAX, 1);
    match add64 {
        FullArithmeticResult::Add64(r) => {
            assert_eq!(r.full.low, [0, 0]);
            assert_eq!(r.full.carry_out, CarryBit(true));
            assert_eq!(r.checked, Err(10));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    assert_eq!(add64.checked(), Err(10));

    let sub32 = ArithmeticLowering::new(Word32, Sub).execute(0, 1);
    match sub32 {
        FullArithmeticResult::Sub32(r) => {
            assert_eq!(r.full.low, [U32_MAX, U32_MAX, U32_MAX, U32_MAX]);
            assert_eq!(r.full.borrow_out, BorrowBit(true));
            assert_eq!(r.checked, Err(10));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    assert_eq!(sub32.checked(), Err(10));

    let sub64 = ArithmeticLowering::new(Word64, Sub).execute(0, 1);
    match sub64 {
        FullArithmeticResult::Sub64(r) => {
            assert_eq!(r.full.low, [U64_MAX, U64_MAX]);
            assert_eq!(r.full.borrow_out, BorrowBit(true));
            assert_eq!(r.checked, Err(10));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    assert_eq!(sub64.checked(), Err(10));

    let mul32 = ArithmeticLowering::new(Word32, Mul).execute(U128_MAX, 2);
    match mul32 {
        FullArithmeticResult::Mul32(r) => {
            assert_eq!(r.full.low, [U32_MAX - 1, U32_MAX, U32_MAX, U32_MAX]);
            assert_eq!(r.full.high, [1, 0, 0, 0]);
            assert_eq!(r.checked, Err(10));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    assert_eq!(mul32.checked(), Err(10));

    let mul64 = ArithmeticLowering::new(Word64, Mul).execute(U128_MAX, 2);
    match mul64 {
        FullArithmeticResult::Mul64(r) => {
            assert_eq!(r.full.low, [U64_MAX - 1, U64_MAX]);
            assert_eq!(r.full.high, [1, 0]);
            assert_eq!(r.checked, Err(10));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    assert_eq!(mul64.checked(), Err(10));
}

fn run_error10_source(layout: WordLayout, expression: &str) {
    let source = format!(
        "language \"moriarty-sherlock-draft/1\"; program WordArithmeticError10 {{ entry {expression}; }}"
    );
    let mut actual_events = 0usize;
    let selected = run_draft_host_with_word_layout(
        &source,
        CONFIG,
        TARGET,
        layout,
        &mut |_| actual_events += 1,
    )
    .unwrap_or_else(|err| panic!("selected host run returned CompileFailure: {err:?}"));
    assert_eq!(selected.word_layout, layout);
    assert!(actual_events > 0, "actual owner run, not compile-only preparation");
    assert_eq!(selected.run.completed.events_forwarded, actual_events);
    match &selected.run.completed.summary.outcome {
        OutcomeC::Rejected { code } => assert_eq!(*code, 10),
        other => panic!("expected semantic Rejected {{ code: 10 }} in summary, got {other:?}"),
    }
    let observation = selected
        .run
        .observe()
        .unwrap_or_else(|err| panic!("real bound observation failed: {err:?}"));
    match observation.image.outcome {
        OutcomeC::Rejected { code } => assert_eq!(*code, 10),
        other => panic!("expected semantic Rejected {{ code: 10 }} in observation, got {other:?}"),
    }
}

#[test]
fn public_selected32_source_rejects_overflow_underflow_and_high128_mul_with_error10() {
    run_error10_source(
        WordLayout::Word32,
        "340282366920938463463374607431768211455 + 1",
    );
    run_error10_source(WordLayout::Word32, "0 - 1");
    run_error10_source(
        WordLayout::Word32,
        "340282366920938463463374607431768211455 * 2",
    );
}

#[test]
fn public_selected64_source_rejects_overflow_underflow_and_high128_mul_with_error10() {
    run_error10_source(
        WordLayout::Word64,
        "340282366920938463463374607431768211455 + 1",
    );
    run_error10_source(WordLayout::Word64, "0 - 1");
    run_error10_source(
        WordLayout::Word64,
        "340282366920938463463374607431768211455 * 2",
    );
}
