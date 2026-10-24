//! New ordinary selected production-lowering tests. SOURCE_ONLY / UNRUN.
//! Expected mathematical values/limbs are existing independently frozen R197
//! literals, not outputs of this candidate. No rejection/hostile cases added.
use mori_core::sherlock::helper_machine::shared::{OutcomeC, RefC};
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host_with_word_layout;
use mori_core::sherlock_word_arithmetic::{BorrowBit, CarryBit};
use mori_core::sherlock_word_lowering::{
    ArithmeticLowering, FullArithmeticResult, UInt128Operation, WordLayout,
};

const CONFIG: &str = include_str!("fixtures/draft-compile/config.json");
const TARGET: &str = include_str!("fixtures/draft-compile/target.json");

#[test]
fn selected_lowering_retains_original_independent_named_full_results() {
    use UInt128Operation::{Add, Mul, Sub};
    use WordLayout::{Word32, Word64};
    let a = ArithmeticLowering::new(Word32, Add);
    assert_eq!(a.component_name(), "checked_u128_add_32");
    match a.execute(4294967293, 8) {
        FullArithmeticResult::Add32(r) => {
            assert_eq!(r.full.low, [5, 1, 0, 0]);
            assert_eq!(r.full.carry_out, CarryBit(false));
            assert_eq!(r.checked, Ok(4294967301));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    match ArithmeticLowering::new(Word64, Add).execute(4294967293, 8) {
        FullArithmeticResult::Add64(r) => {
            assert_eq!(r.full.low, [4294967301, 0]);
            assert_eq!(r.full.carry_out, CarryBit(false));
            assert_eq!(r.checked, Ok(4294967301));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    match ArithmeticLowering::new(Word32, Sub).execute(18446744073709551625, 7) {
        FullArithmeticResult::Sub32(r) => {
            assert_eq!(r.full.low, [2, 0, 1, 0]);
            assert_eq!(r.full.borrow_out, BorrowBit(false));
            assert_eq!(r.checked, Ok(18446744073709551618));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    match ArithmeticLowering::new(Word64, Sub).execute(18446744073709551625, 7) {
        FullArithmeticResult::Sub64(r) => {
            assert_eq!(r.full.low, [2, 1]);
            assert_eq!(r.full.borrow_out, BorrowBit(false));
            assert_eq!(r.checked, Ok(18446744073709551618));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    match ArithmeticLowering::new(Word32, Mul).execute(281474976710657, 3) {
        FullArithmeticResult::Mul32(r) => {
            assert_eq!(r.full.low, [3, 196608, 0, 0]);
            assert_eq!(r.full.high, [0, 0, 0, 0]);
            assert_eq!(r.checked, Ok(844424930131971));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
    match ArithmeticLowering::new(Word64, Mul).execute(281474976710657, 3) {
        FullArithmeticResult::Mul64(r) => {
            assert_eq!(r.full.low, [844424930131971, 0]);
            assert_eq!(r.full.high, [0, 0]);
            assert_eq!(r.checked, Ok(844424930131971));
        }
        other => panic!("wrong selected component: {other:?}"),
    }
}

fn run_source(layout: WordLayout, expression: &str, expected: u128) {
    let source = format!(
        "language \"moriarty-sherlock-draft/1\"; program WordArithmetic {{ entry {expression}; }}"
    );
    let mut actual_events = 0usize;
    let selected = run_draft_host_with_word_layout(&source, CONFIG, TARGET, layout, &mut |_| {
        actual_events += 1
    })
    .unwrap_or_else(|_| panic!("normal selected source run failed"));
    assert_eq!(selected.word_layout, layout);
    assert!(
        actual_events > 0,
        "actual owner run, not compile-only preparation"
    );
    let observation = selected
        .run
        .observe()
        .unwrap_or_else(|_| panic!("real bound observation failed"));
    let value = match observation.image.outcome {
        OutcomeC::Halted { value } => *value,
        other => panic!("expected normal halt, got {other:?}"),
    };
    let index = match value {
        RefC::Data(index) => index,
        other => panic!("arithmetic result is not owner data: {other:?}"),
    };
    match &observation.image.data[index] {
        mori_core::sherlock::helper_machine::raw::DataRow::U128 { n } => assert_eq!(*n, expected),
        _ => panic!("wrong arithmetic result kind"),
    }
}

#[test]
fn public_selected32_source_runs_original_add_sub_mul() {
    run_source(WordLayout::Word32, "4294967293 + 8", 4294967301);
    run_source(
        WordLayout::Word32,
        "18446744073709551625 - 7",
        18446744073709551618,
    );
    run_source(WordLayout::Word32, "281474976710657 * 3", 844424930131971);
}

#[test]
fn public_selected64_source_runs_original_add_sub_mul() {
    run_source(WordLayout::Word64, "4294967293 + 8", 4294967301);
    run_source(
        WordLayout::Word64,
        "18446744073709551625 - 7",
        18446744073709551618,
    );
    run_source(WordLayout::Word64, "281474976710657 * 3", 844424930131971);
}
