//! Ordinary compiler structure checks from the independently frozen specimens.
//! These tests parse IR only; they never invoke SDK check/model, VM or proving.
use midnight_zkir_v3::{Instruction, IrSource, ir::Operand};
use mori_backend::{Target, stage::{StagedArtifact, compile_staged}};
use mori_core::{Type, UInt};
use std::path::{Path, PathBuf};

const ESCROW: &str = r#"language "moriarty/2";
agreement ContinuationEscrowOracle {
  asset PAY = { network: "local", issuer: "continuation-escrow-oracle", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver(amount: UInt) {
    transfer(PAY, "alice", "escrow", amount);
    liability("release", PAY, "escrow", "bob", amount);
    await delivery(ready: Bool) when ready;
    transfer(PAY, "escrow", "bob", amount);
    discharge("release", amount);
  }
}
"#;
const RESERVE: &str = r#"language "moriarty/2";
agreement ReserveContinuationOracle {
  asset CREDIT = { network: "local", issuer: "continuation-reserve-oracle", reference: "UNIT", scale: 0 };
  state basis: UInt = 40;
  state allocated: UInt = 0;
  state marker: UInt = 77;
  action allocate(extra: UInt) {
    let held = basis + extra;
    next basis = basis - 5;
    liability("allocation", CREDIT, "reserve", "target", held);
    await approval(quote: UInt) when quote >= held;
    next allocated = held;
    transfer(CREDIT, "reserve", "target", held);
    discharge("allocation", held);
    next basis = basis + quote;
    await settlement(ready: Bool) when ready;
    requires quote <= 50;
    fee(CREDIT, "reserve", "treasury", extra);
    emit "settled_quote"(quote);
  }
}
"#;

fn compile(source: &str) -> StagedArtifact {
    compile_staged(source, &Target::ledger9("local", "stage-oracle"), Path::new("unused-output")).unwrap()
}
fn effects(a: &StagedArtifact) -> Vec<Vec<(u32, &str)>> {
    a.circuits.iter().map(|c| c.effect_frame.iter().map(|e| (e.step, e.kind.as_str())).collect()).collect()
}
fn captured(a: &StagedArtifact) -> Vec<(Vec<&str>, Vec<&str>)> {
    a.circuits.iter().map(|c| (c.capture_in.iter().map(|s| s.name.as_str()).collect(), c.capture_out.iter().map(|s| s.name.as_str()).collect())).collect()
}
fn assert_serialized_ir(a: &StagedArtifact) {
    for c in &a.circuits {
        let ir = IrSource::load(c.zkir.as_bytes()).unwrap();
        assert_eq!(ir.instructions.len(), c.instruction_count as usize);
        assert_eq!(ir.instructions.iter().filter(|i| matches!(i, Instruction::PublicInput { .. })).count(), c.public_inputs.len());
        assert_eq!(ir.instructions.iter().filter(|i| matches!(i, Instruction::Impact { .. })).count(), c.public_inputs.len());
        assert!(matches!(ir.instructions.last(), Some(Instruction::Output { .. })));
    }
}
fn assert_public_copy(circuit: &mori_backend::stage::StagedCircuit, from: (&str, &str), to: (&str, &str)) {
    let ir = IrSource::load(circuit.zkir.as_bytes()).unwrap();
    let operands = ir.instructions.iter().filter_map(|i| match i {
        Instruction::PublicInput { output, .. } => Some(Operand::Variable(output.clone())),
        _ => None,
    }).collect::<Vec<_>>();
    let position = |(role, name): (&str, &str)| circuit.public_inputs.iter()
        .position(|b| b.role == role && b.name == name).unwrap();
    let before = &operands[position(from)];
    let after = &operands[position(to)];
    assert!(ir.instructions.iter().any(|i| match i {
        Instruction::ConstrainEq { a, b } => (a == after && b == before) || (a == before && b == after),
        _ => false,
    }), "source value {:?} must feed {:?}", from, to);
}
#[test]
fn delivery_has_two_real_circuits_with_original_effect_ordinals() {
    let a = compile(ESCROW);
    assert_eq!(a.circuits.iter().map(|c| (c.segment, c.entry_site, c.exit_site, c.replay_steps)).collect::<Vec<_>>(), vec![(0,0,1,2), (1,1,0,3)]);
    assert_eq!(effects(&a), vec![vec![(0,"transfer"),(1,"liability")], vec![(3,"transfer"),(4,"liability")]]);
    assert_eq!(captured(&a), vec![(vec![],vec!["amount"]),(vec!["amount"],vec![])]);
    for c in &a.circuits {
        assert_eq!(c.balance_frame.iter().map(|b| b.account.as_str()).collect::<Vec<_>>(), vec!["alice","bob","escrow"]);
        assert_eq!(c.liability_frame.iter().map(|l| l.name.as_str()).collect::<Vec<_>>(), vec!["release"]);
    }
    assert_eq!(a.resources.logical[0].plan_total, UInt(11));
    assert_eq!(a.resources.logical[0].segments.iter().map(|s| (s.segment_logical,s.committed_before)).collect::<Vec<_>>(), vec![(UInt(5),UInt(0)),(UInt(6),UInt(5))]);
    assert_serialized_ir(&a);
}
#[test]
fn reserve_preserves_inputs_lets_and_accepted_events_across_three_circuits() {
    let a = compile(RESERVE);
    assert_eq!(a.circuits.iter().map(|c| (c.segment,c.entry_site,c.exit_site,c.replay_steps)).collect::<Vec<_>>(), vec![(0,0,1,3),(1,1,2,5),(2,2,0,4)]);
    assert_eq!(effects(&a), vec![vec![(1,"state"),(2,"liability")],vec![(4,"state"),(5,"transfer"),(6,"liability"),(7,"state")],vec![(10,"fee"),(11,"emit")]]);
    assert_eq!(captured(&a), vec![(vec![],vec!["extra","held"]),(vec!["extra","held"],vec!["extra","held","quote"]),(vec!["extra","held","quote"],vec![])]);
    assert_eq!(a.circuits.iter().map(|c| c.event.iter().map(|b| (b.name.as_str(),b.ty)).collect::<Vec<_>>()).collect::<Vec<_>>(), vec![vec![],vec![("quote",Type::UInt)],vec![("ready",Type::Bool)]]);
    for c in &a.circuits {
        assert_eq!(c.capture_width,3);
        assert_eq!(c.balance_frame.iter().map(|b| b.account.as_str()).collect::<Vec<_>>(),vec!["reserve","target","treasury"]);
        assert_eq!(c.liability_frame[0].name,"allocation");
        assert_eq!(c.public_inputs.iter().filter(|b| b.role=="input").count(), usize::from(c.segment==0));
    }
    // held is the pre-suspension let, not basis recomputed after it changed.
    assert_public_copy(&a.circuits[1], ("pre_pending_capture", "held"), ("effect", "4.after"));
    assert_public_copy(&a.circuits[1], ("pre_pending_capture", "held"), ("effect", "5.amount"));
    // quote is an earlier accepted event, carried through the second await.
    assert_public_copy(&a.circuits[2], ("pre_pending_capture", "quote"), ("effect", "11.value"));
    assert_eq!(a.resources.logical[0].plan_total,UInt(36));
    assert_eq!(a.resources.logical[0].segments.iter().map(|s| (s.segment_logical,s.committed_before)).collect::<Vec<_>>(),vec![(UInt(11),UInt(0)),(UInt(15),UInt(11)),(UInt(10),UInt(26))]);
    assert_serialized_ir(&a);
}
#[test]
fn generic_scalar_segments_and_zero_await_actions_keep_declaration_order() {
    let source = r#"language "moriarty/2";
    agreement Scalars {
      state n: UInt = 0; state enabled: Bool = true;
      action z_first(x: UInt, flag: Bool) {
        let y = x * 3 + 1;
        let permitted = flag and enabled;
        await accept(z: UInt, ok: Bool) when ok and permitted and z >= y;
        requires not (z not = y) or z > y;
        next n = n + z - 1;
        ensures n >= z - 1;
        emit "value"(y);
      }
      action a_second() { next n = n + 2; }
    }"#;
    let a=compile(source);
    assert_eq!(a.circuits.iter().map(|c| (c.action.as_str(),c.segment)).collect::<Vec<_>>(),vec![("z_first",0),("z_first",1),("a_second",0)]);
    assert_eq!(captured(&a),vec![(vec![],vec!["x","flag","y","permitted"]),(vec!["x","flag","y","permitted"],vec![]),(vec![],vec![])]);
    assert_eq!(a.circuits[2].replay_steps,1);
    assert_eq!(a.circuits[2].capture_width,0);
    assert!(a.circuits[2].pending_positions.is_none());
    let ir=IrSource::load(a.circuits[1].zkir.as_bytes()).unwrap();
    assert!(ir.instructions.iter().any(|i| matches!(i,Instruction::Mul {..})));
    assert!(ir.instructions.iter().any(|i| matches!(i,Instruction::Assert {..})));
    assert_serialized_ir(&a);
}
#[test]
fn every_segment_carries_the_original_program_duty_registry() {
    let source=r#"language "moriarty/2";
    agreement Registry {
      asset A = { network: "local", issuer: "issuer", reference: "unit", scale: 0 };
      action a(x: UInt) { await go(ready: Bool) when ready; emit "x"(x); }
      action b(x: UInt) { liability("duty", A, "debtor", "creditor", x); }
    }"#;
    let a=compile(source);
    assert_eq!(a.circuits.len(),3);
    for c in &a.circuits { assert_eq!(c.liability_frame.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(),vec!["duty"]); }
    assert_serialized_ir(&a);
}

/// Full byte comparison uses a separately generated, baseline-owned fixture.
/// The candidate is never allowed to generate or update expected bytes.
#[test]
#[ignore = "requires Opus-classified baseline fixture generation and MORIARTY_STAGE_V1_BASELINE"]
fn legacy_artifact_and_every_ir_file_match_baseline_bytes() {
    let root = PathBuf::from(std::env::var_os("MORIARTY_STAGE_V1_BASELINE")
        .expect("set MORIARTY_STAGE_V1_BASELINE to the immutable baseline fixture directory"));
    let source = std::fs::read_to_string(root.join("source.mori")).unwrap();
    let target = Target::from_json(&std::fs::read_to_string(root.join("target.json")).unwrap()).unwrap();
    let program = mori_core::parse(&source).unwrap();
    let artifact = mori_backend::compile(&program, &target).unwrap();
    assert_eq!(serde_json::to_vec_pretty(&artifact).unwrap(), std::fs::read(root.join("artifact.json")).unwrap());
    for (index,circuit) in artifact.circuits.iter().enumerate() {
        let name = format!("{index:03}-{}.zkir", circuit.action);
        assert_eq!(circuit.zkir.as_bytes(), std::fs::read(root.join(name)).unwrap());
    }
}

#[test]
fn legacy_counter_abi_and_payment_effects_match_existing_frozen_snapshots() {
    let counter = mori_core::parse(include_str!("programs/counter.mori")).unwrap();
    let a = mori_backend::compile(&counter, &Target::ledger9("local", "counter-anchor")).unwrap();
    let actual = a.circuits.iter().map(|c| (&c.action,
        c.public_inputs.iter().map(|p| (&p.role, &p.name)).collect::<Vec<_>>())).collect::<Vec<_>>();
    let snapshot = include_str!("snapshots/compile__authored_counter_abi.snap");
    let expected = snapshot.split_once("\n---\n").unwrap().1.trim_end();
    assert_eq!(format!("{actual:#?}"), expected);

    let payment = mori_core::parse(include_str!("programs/payment.mori")).unwrap();
    let a = mori_backend::compile(&payment, &Target::ledger9("local", "payment-anchor")).unwrap();
    let snapshot = include_str!("snapshots/compile__authored_payment_effects.snap");
    let expected = snapshot.split_once("\n---\n").unwrap().1.trim_end();
    assert_eq!(format!("{:#?}", a.circuits[0].effect_frame), expected);
}
