//! SOURCE_ONLY / UNRUN. Complete literal image and ordered-statement equality.
#[path = "/home/charl/Moriarty/.worktrees/sherlock-native-handwritten-20261019/crates/mori-backend/src/sherlock_machine/literal_host.rs"]
mod literal_host;
#[path = "/home/charl/Moriarty/.worktrees/sherlock-native-handwritten-20261019/crates/mori-backend/src/sherlock_machine/literal_witness.rs"]
mod literal_witness;
use mori_core::sherlock::helper_machine::shared::pure_leaves::{
    borrowed_bool, borrowed_kind, borrowed_u128, BorrowedValueRow,
};
use mori_core::sherlock::helper_machine::shared::{
    ActiveLengths, AllocCounters, BoundsC, ControlC, DigestC, ExpressionRowC, FinanceInput,
    GrammarDomain, LayoutCapacitiesC, LogicalKind, OccurrenceRowC, OutcomeC, RunIntC,
};
use mori_core::sherlock::reference_types::{Expr, Value as ReferenceValue, K_SOURCE_SHA256};
use mori_core::sherlock_compile_artifact::draft_contract::{
    ARTIFACT_FORMAT, SOURCE_PROFILE, SOURCE_SEMANTICS,
};
use mori_core::sherlock_compile_artifact::draft_reference::compile_draft_with_reference;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::Arc;
const SOURCE: &str = include_str!("../inputs/literal-seven.mori");
const CONFIG: &str = include_str!("../inputs/draft-config.json");
const TARGET: &str = include_str!("../inputs/target.json");
// Pinned bytes hashed only as ordinary identifiers; neither file is compiled here.
const K_MAIN_SOURCE: &[u8] = include_bytes!("/home/charl/Moriarty/.worktrees/sherlock-native-handwritten-20261019/models/sherlock/k/main.k");
const REFERENCE_SOURCE: &[u8] = include_bytes!("/home/charl/Moriarty/.worktrees/sherlock-native-handwritten-20261019/crates/mori-core/src/sherlock/reference.rs");
fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}
fn image(v: &literal_host::ImageValues) -> Value {
    json!({"root": v.root, "tables": v.tables})
}
fn main() {
    let prepared = compile_draft_with_reference(SOURCE, CONFIG, TARGET)
        .unwrap_or_else(|failure| panic!(
            "real public draft source preparation refused at stage {}: {}",
            failure.stage,
            serde_json::to_string(&failure)
                .unwrap_or_else(|error| format!("unserialized CompileFailure: {error}")),
        ));
    let expected: Value = serde_json::from_str(include_str!("../EXPECTED-A102-INPUT-AND-STATEMENT.json")).unwrap();
    // EXP-2: ordinary SHA-256 identifiers of the exact compiled and pinned bytes.
    // Finite source/statement agreement only; not authentication or correspondence.
    let source_sha256 = sha256_hex(SOURCE.as_bytes());
    let reference_file_sha256 = sha256_hex(REFERENCE_SOURCE);
    assert_eq!(SOURCE.len(), 72);
    assert_eq!(expected["source"]["bytes"].as_u64(), Some(72));
    assert_eq!(expected["source"]["sha256"].as_str(), Some(source_sha256.as_str()));
    assert_eq!(sha256_hex(K_MAIN_SOURCE), K_SOURCE_SHA256);
    assert_eq!(expected["identity"]["program_source_sha256"].as_str(), Some(source_sha256.as_str()));
    assert_eq!(expected["identity"]["k_source_sha256"].as_str(), Some(K_SOURCE_SHA256));
    assert_eq!(expected["identity"]["reference_source_sha256"].as_str(),
        Some(reference_file_sha256.as_str()));
    let report = prepared.preparation();
    assert_eq!(report.format, ARTIFACT_FORMAT);
    assert_eq!(report.source.language, SOURCE_PROFILE);
    assert_eq!(expected["source_domain"]["dialect"].as_str(), Some(SOURCE_PROFILE));
    assert_eq!(report.source.semantics, SOURCE_SEMANTICS);
    assert_eq!(report.source.program, "LiteralSeven");
    assert_eq!(expected["source_domain"]["program_name"].as_str(), Some(report.source.program.as_str()));
    assert_eq!(json!(report.source.selection), json!({"kind": "entry"}));
    assert_eq!(report.source.source_digest, source_sha256);
    assert_eq!(report.config.file_digest, sha256_hex(CONFIG.as_bytes()));
    assert_eq!(report.target.file_digest, sha256_hex(TARGET.as_bytes()));
    assert_eq!(report.target.descriptor, serde_json::from_str::<Value>(TARGET).unwrap());
    assert_eq!([report.evidence.source_preparation, report.evidence.host_observation,
        report.evidence.native, report.evidence.target, report.evidence.acceptance],
        ["performed", "genesis-only", "unrun", "unrun", "unestablished"]);
    let reference = prepared.reference_program();
    assert!(reference.functions.is_empty());
    assert_eq!(*reference.entry, Expr::Literal(ReferenceValue::UInt(7)));
    // SEC-1: unstepped B genesis content under this draft config (fuel Nat 64,
    // stack Nat 8). The separate A profile below keeps fuel 1 and stack 0.
    let b = prepared.observe_b_genesis().unwrap_or_else(|violation| panic!(
        "actual admitted B genesis observation refused: kind={:?} clause={:?} rule={:?} locator={:?}",
        violation.kind, violation.clause, violation.rule, violation.locator));
    let identity_b = b.admission.identity;
    assert_eq!(identity_b.profile_id, 2);
    assert_eq!(identity_b.grammar_domain, GrammarDomain::DedicatedSorts);
    assert_eq!(identity_b.finance, FinanceInput::Absent);
    assert_eq!(identity_b.layout_version,
        ("moriarty".to_owned(), "sherlock-helper-microstep-hostonly/1".to_owned()));
    assert_eq!(identity_b.field_id, None);
    assert_eq!(identity_b.layout_digest, DigestC::Unselected);
    assert_eq!(identity_b.bounds_digest, DigestC::Unselected);
    assert_eq!(identity_b.program_membership_digest, DigestC::Unselected);
    let bounds = b.admission.bounds;
    assert_eq!(*bounds, BoundsC { code_rows: 64, function_rows: 4, code_child_slots: 16,
        parameter_slots: 8, pattern_rows: 8, match_plan_rows: 8, match_arm_rows: 8,
        type_rows: 8, constructor_rows: 8, constructor_field_slots: 16, max_arity: 8,
        constant_nodes: 16, constant_field_slots: 16, data_nodes: 64, data_field_slots: 64,
        environment_cells: 64, frames: 64, helper_tasks: 64, serialization_bytes: 65536,
        max_reference_work: 4096, max_helper_work: 4096, step_cap: 4096 });
    let layout = b.admission.layout;
    assert_eq!(layout.bounds, bounds);
    assert_eq!(*layout.capacities, LayoutCapacitiesC { name_rows: 32, name_bytes: 256,
        occurrence_path_slots: 256, value_slots: 1024 });
    assert_eq!(layout.layout_version, &identity_b.layout_version);
    assert!(layout.widths.is_none() && !layout.boundary_capacities_present);
    assert_eq!(layout.field_id, None);
    assert_eq!(layout.scalar_layout_id, None);
    let view = b.admission.view;
    assert_eq!(view.entry, 0);
    assert!(matches!(view.expressions.as_slice(), [ExpressionRowC::Literal { constant: 0 }]));
    assert!(matches!(view.occurrences.as_slice(),
        [OccurrenceRowC { owner_is_function: false, function: 0, .. }]));
    assert!(view.functions.is_empty() && view.parameters.is_empty() && view.children.is_empty()
        && view.arms.is_empty() && view.patterns.is_empty() && view.pattern_children.is_empty()
        && view.plan_nodes.is_empty() && view.constant_fields.is_empty());
    assert_eq!(view.constants.len(), 1);
    let constant = BorrowedValueRow::Const(&view.constants[0]);
    assert_eq!(borrowed_kind(constant), LogicalKind::UInt);
    assert_eq!(borrowed_u128(constant), Some(7));
    assert_eq!(borrowed_bool(constant), None);
    let genesis = &b.image;
    assert_eq!(genesis.run_id.fuel_input, RunIntC::Nat(64));
    assert_eq!(genesis.run_id.stack_limit_input, RunIntC::Nat(8));
    assert_eq!(&genesis.run_id.identity, identity_b);
    assert_eq!(genesis.control, &ControlC::Eval { expr: 0 });
    assert_eq!(genesis.control_origin, 0);
    assert_eq!(genesis.outcome, &OutcomeC::Running);
    assert_eq!(genesis.env_root, None);
    assert!(genesis.helper.is_none() && genesis.result.is_none());
    assert_eq!([genesis.initial_fuel, genesis.remaining_work, genesis.consumed_work], [64, 64, 0]);
    assert_eq!([genesis.helper_work, genesis.helper_visits, genesis.reference_cursor], [0, 0, 0]);
    assert_eq!(genesis.stack_limit, 8);
    assert_eq!(*genesis.alloc, AllocCounters::default());
    assert_eq!(*genesis.active, ActiveLengths::default());
    assert!(genesis.data.is_empty() && genesis.data_fields.is_empty() && genesis.env.is_empty()
        && genesis.frames.is_empty() && genesis.tasks.is_empty() && genesis.value_slots.is_empty());
    assert_eq!((genesis.depth(), genesis.task_live()), (0, 0));
    let identity = literal_host::LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.into(),
        reference_source_sha256: reference_file_sha256.into(),
        program_source_sha256: source_sha256.into(),
    };
    let capacities = literal_host::LiteralCapacities {
        program: [0,0,1,0,1,0,0,0,0,0,4,4,5,1,0], state: [0,0,0,0,0,1], stack_limit: 0,
    };
    let (produced, statement) = literal_witness::build_checked_literal_transition_request(
        Arc::clone(prepared.reference_program()), &capacities, 1, 0, &identity, 252,
    ).unwrap_or_else(|error| panic!("actual checked A102 constructor refused: {error}"));
    assert!(Arc::ptr_eq(&produced.prepared.full_program, prepared.reference_program()));
    let actual = json!({"program":image(&produced.request.values.program),
        "pre":image(&produced.request.values.pre), "post":image(&produced.request.values.post),
        "rule_tag":produced.request.values.rule_tag,"charge":produced.request.values.charge});
    assert_eq!(actual, expected["complete_expected_values"]);
    assert_eq!(json!(statement.iter().map(ToString::to_string).collect::<Vec<_>>()),
        expected["complete_expected_ordered_statement"]);
    assert_eq!(produced.step_record.rule_start_line, 102);
    assert_eq!(produced.step_record.charge, 1);
    assert_eq!(produced.selected_expr_id, 0);
    assert_eq!(produced.selected_constant_ref.space, false);
    assert_eq!(produced.selected_constant_ref.index, 0);
    println!("real-source A102 input: 252 lanes matched same-author specification; B genesis content and source/K/reference SHA-256 identifiers checked; no native/proof/ledger claim");
}
