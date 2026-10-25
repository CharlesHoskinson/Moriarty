//! UIREQ-023 native relation inventory for actual dedicated-draft owner runs.
//!
//! HAND-AUTHORED TABLE. Every row below was written by reading the native
//! fragment sources under `crates/mori-backend/src/sherlock_machine/`; nothing
//! is generated from those sources, from owner metadata or from a run. The
//! visible test `tests/sherlock_native_inventory.rs` fails when an exercised
//! rule of factorial or tree fold has no row, when a row names a rule neither
//! program exercises, when a named fragment file, entry function or rule name
//! is absent, or when a fragment file appears that no row or listed exclusion
//! claims.
//!
//! The report only describes. It runs no SDK load, check, compile, proof or
//! verification and grants no native acceptance: every listed C6 fragment is
//! source-only, and a rule without a fragment stays an explicit gap.
use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::wire::OwnerWire;
use crate::sherlock::helper_machine::shared::StepEventC;

pub const INVENTORY_FORMAT: &str = "moriarty-sherlock-native-inventory/1";
pub const TABLE_PROVENANCE: &str = "hand-authored";
pub const FRAGMENT_DIR: &str = "crates/mori-backend/src/sherlock_machine";

/// Finite-family class keyed by control/top-frame/helper state (design.md).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    Schedule,
    Scalar,
    CallBind,
    FrameReturn,
    DataMatch,
}

impl Family {
    pub fn name(self) -> &'static str {
        match self {
            Self::Schedule => "Schedule",
            Self::Scalar => "Scalar",
            Self::CallBind => "CallBind",
            Self::FrameReturn => "FrameReturn",
            Self::DataMatch => "DataMatch",
        }
    }

    /// The UIREQ-023 constraint owner class answerable for this family.
    pub fn owner(self) -> &'static str {
        match self {
            Self::Schedule => "dispatch",
            Self::Scalar => "scalar",
            Self::FrameReturn => "frame",
            Self::CallBind | Self::DataMatch => "data",
        }
    }
}

/// One emitter on the common `sherlock-c6-phys/1` StepInput/StepOutput carriers.
#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    /// File name inside [`FRAGMENT_DIR`].
    pub file: &'static str,
    /// Public emitter the file must define.
    pub entry: &'static str,
    /// Selected-step consumer file, when one exists.
    pub consumer: Option<&'static str>,
}

/// A narrow byte-carrier emitter: not on the common C6 interface.
#[derive(Clone, Copy, Debug)]
pub struct NarrowCarrier {
    pub file: &'static str,
    pub entry: &'static str,
    /// Dotted rule spelling the file uses.
    pub names: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Row {
    pub rule: &'static str,
    pub family: Family,
    pub fragment: Option<Fragment>,
    pub narrow: &'static [NarrowCarrier],
}

const fn gap(rule: &'static str, family: Family) -> Row {
    Row {
        rule,
        family,
        fragment: None,
        narrow: &[],
    }
}

const fn c6(
    rule: &'static str,
    family: Family,
    file: &'static str,
    entry: &'static str,
    consumer: Option<&'static str>,
) -> Row {
    Row {
        rule,
        family,
        fragment: Some(Fragment {
            file,
            entry,
            consumer,
        }),
        narrow: &[],
    }
}

use Family::{CallBind, DataMatch, FrameReturn, Scalar, Schedule};

/// HAND-AUTHORED: exactly the rules factorial and tree fold exercise.
pub const TABLE: &[Row] = &[
    // Schedule: control is Eval(expr) or a terminal publication.
    Row {
        rule: "KAdminPublishValue",
        family: Schedule,
        fragment: None,
        narrow: &[NarrowCarrier {
            file: "b_admin_publication.rs",
            entry: "emit_k_admin_publish_value",
            names: "k.admin.publish.value",
        }],
    },
    gap("KMachineEvalLiteralValid", Schedule),
    gap("KMachineEvalVariablePresent", Schedule),
    gap("KMachineEvalBinary", Schedule),
    gap("KMachineEvalUnary", Schedule),
    gap("KMachineEvalIf", Schedule),
    gap("KMachineEvalMatch", Schedule),
    gap("KMachineEvalCallArguments", Schedule),
    // Scalar: active Scalar helper or primitive completion.
    c6(
        "BHelperScalarEntry",
        Scalar,
        "c6_scalar_entry_step.rs",
        "emit_c6_scalar_entry_step_block",
        Some("c6_scalar_entry_consumer.rs"),
    ),
    c6(
        "BHelperScalarTag",
        Scalar,
        "c6_scalar_tag_step.rs",
        "emit_c6_scalar_tag_step_block",
        Some("c6_scalar_tag_consumer.rs"),
    ),
    Row {
        rule: "BHelperScalarArithmetic",
        family: Scalar,
        fragment: Some(Fragment {
            file: "c6_scalar_arithmetic_step.rs",
            entry: "emit_c6_scalar_arithmetic_step_block",
            consumer: Some("c6_scalar_arithmetic_consumer.rs"),
        }),
        narrow: &[
            NarrowCarrier {
                file: "b_scalar_arithmetic.rs",
                entry: "emit_b_scalar_arithmetic_add",
                names: "b.helper.Scalar.arithmetic",
            },
            NarrowCarrier {
                file: "b_scalar_subtraction.rs",
                entry: "emit_b_scalar_arithmetic_sub",
                names: "b.helper.Scalar.arithmetic",
            },
        ],
    },
    gap("BHelperScalarConstruct", Scalar),
    Row {
        rule: "BHelperScalarExit",
        family: Scalar,
        fragment: Some(Fragment {
            file: "c6_scalar_exit_step.rs",
            entry: "emit_c6_scalar_exit_step_block",
            consumer: Some("c6_scalar_exit_consumer.rs"),
        }),
        narrow: &[
            NarrowCarrier {
                file: "b_scalar_completion.rs",
                entry: "emit_b_scalar_exit_add_u128",
                names: "b.helper.Scalar.exit",
            },
            NarrowCarrier {
                file: "b_scalar_subtraction_completion.rs",
                entry: "emit_b_scalar_exit_sub_u128",
                names: "b.helper.Scalar.exit",
            },
        ],
    },
    gap("KMachinePrimitiveValue", Scalar),
    // CallBind: invocation, parameter binding and environment lookup.
    gap("KMachineInvokeFind", CallBind),
    gap("KMachineEnterBind", CallBind),
    gap("KMachineBoundValid", CallBind),
    gap("BHelperFindEntry", CallBind),
    gap("BHelperFindRow", CallBind),
    gap("BHelperFindExit", CallBind),
    gap("BHelperBindEntry", CallBind),
    gap("BHelperBindPair", CallBind),
    gap("BHelperBindEmpty", CallBind),
    gap("BHelperBindType", CallBind),
    gap("BHelperBindInsert", CallBind),
    gap("BHelperBindExit", CallBind),
    gap("BHelperEnvGetEntry", CallBind),
    gap("BHelperEnvGetCell", CallBind),
    gap("BHelperEnvGetExit", CallBind),
    gap("BHelperAppendEntry", CallBind),
    gap("BHelperAppendEmpty", CallBind),
    gap("BHelperAppendTail", CallBind),
    gap("BHelperAppendExit", CallBind),
    // FrameReturn: Push or Return with the named top frame.
    c6(
        "KMachinePushAccept",
        FrameReturn,
        "c6_push_step.rs",
        "emit_c6_push_step_block",
        None,
    ),
    c6(
        "KMachineReturnLeft",
        FrameReturn,
        "c6_return_left_step.rs",
        "emit_c6_return_left_step_block",
        Some("c6_return_left_consumer.rs"),
    ),
    Row {
        rule: "KMachineReturnRight",
        family: FrameReturn,
        fragment: Some(Fragment {
            file: "c6_return_right_step.rs",
            entry: "emit_c6_return_right_step_block",
            consumer: Some("c6_return_right_consumer.rs"),
        }),
        narrow: &[
            NarrowCarrier {
                file: "b_return_right_completion.rs",
                entry: "emit_k_machine_return_right_add",
                names: "k.machine.return.right",
            },
            NarrowCarrier {
                file: "b_return_right_subtraction.rs",
                entry: "emit_k_machine_return_right_sub",
                names: "k.machine.return.right",
            },
        ],
    },
    gap("KMachineReturnUnary", FrameReturn),
    gap("KMachineReturnIfTrue", FrameReturn),
    gap("KMachineReturnIfFalse", FrameReturn),
    gap("KMachineReturnMatch", FrameReturn),
    gap("KMachineReturnArgumentsFinal", FrameReturn),
    c6(
        "KMachineReturnCallValid",
        FrameReturn,
        "c6_return_call_step.rs",
        "emit_c6_return_call_step_block",
        Some("c6_return_call_consumer.rs"),
    ),
    c6(
        "KMachineReturnScope",
        FrameReturn,
        "c6_return_scope_step.rs",
        "emit_c6_return_scope_step_block",
        Some("c6_return_scope_consumer.rs"),
    ),
    // DataMatch: literal validation, equality and pattern matching.
    gap("BHelperValidateEntry", DataMatch),
    gap("BHelperValidateOccurrence", DataMatch),
    gap("BHelperValidateEmpty", DataMatch),
    gap("BHelperValidateExit", DataMatch),
    gap("BHelperEqualEntry", DataMatch),
    gap("BHelperEqualPair", DataMatch),
    c6(
        "BHelperEqualEmpty",
        DataMatch,
        "c6_equal_empty_step.rs",
        "emit_c6_equal_empty_step_block",
        None,
    ),
    gap("BHelperEqualDifferent", DataMatch),
    gap("BHelperEqualExit", DataMatch),
    gap("KMachineCasesTry", DataMatch),
    gap("KMachineCasesMiss", DataMatch),
    gap("KMachineCasesHit", DataMatch),
    gap("BHelperMatchEntry", DataMatch),
    gap("BHelperMatchArm", DataMatch),
    gap("BHelperMatchOccurrence", DataMatch),
    gap("BHelperMatchField", DataMatch),
    gap("BHelperMatchBinding", DataMatch),
    gap("BHelperMatchDistinct", DataMatch),
    gap("BHelperMatchNext", DataMatch),
    gap("BHelperMatchSelected", DataMatch),
    gap("BHelperMatchFailure", DataMatch),
    gap("BHelperMatchExit", DataMatch),
];

/// HAND-AUTHORED: C6 fragments for rules neither program exercises.
pub const UNEXERCISED_C6: &[(&str, Fragment)] = &[
    (
        "BHelperScalarFailure",
        Fragment {
            file: "c6_scalar_failure_step.rs",
            entry: "emit_c6_scalar_failure_step_block",
            consumer: None,
        },
    ),
    (
        "BHelperValidateField",
        Fragment {
            file: "c6_validate_field_step.rs",
            entry: "emit_c6_validate_field_step_block",
            consumer: Some("c6_validate_field_consumer.rs"),
        },
    ),
    (
        "XAdtEvalConstructNullary",
        Fragment {
            file: "c6_construct_nullary_step.rs",
            entry: "emit_c6_construct_nullary_step_block",
            consumer: Some("c6_construct_nullary_consumer.rs"),
        },
    ),
];

/// HAND-AUTHORED: the C6 dispatch owner candidate (per-rule Core selection).
pub const DISPATCH: Fragment = Fragment {
    file: "c6_dispatch_step.rs",
    entry: "emit_c6_dispatch_block",
    consumer: Some("c6_dispatch_consumer.rs"),
};

/// HAND-AUTHORED: step fragments of the Candidate A `sherlock-k-a/1` image,
/// not the helper-microstep machine and not on the C6 interface.
pub const OTHER_MACHINE: &[(&str, &str, &str)] = &[
    ("literal_step.rs", "A102", "emit_literal_step"),
    (
        "return_publication_step.rs",
        "A80",
        "emit_return_publication_step",
    ),
    (
        "error_publication_step.rs",
        "A82",
        "emit_error_publication_step",
    ),
    (
        "fuel_exhaustion_step.rs",
        "A84",
        "emit_fuel_exhaustion_step",
    ),
];

/// Rule counts of one actual owner run, in event order of first appearance.
#[derive(Default, Debug, Clone)]
pub struct RuleTally {
    counts: BTreeMap<String, u128>,
    events: u128,
}

impl RuleTally {
    pub fn observe(&mut self, event: &StepEventC) {
        let name = match event.rule_id.owner_wire() {
            Value::String(s) => s,
            other => other.to_string(),
        };
        *self.counts.entry(name).or_insert(0) += 1;
        self.events += 1;
    }

    pub fn rules(&self) -> impl Iterator<Item = &str> {
        self.counts.keys().map(String::as_str)
    }

    pub fn events(&self) -> u128 {
        self.events
    }
}

fn fragment_wire(f: &Fragment) -> Value {
    json!({
        "file": format!("{FRAGMENT_DIR}/{}", f.file),
        "entry": f.entry,
        "consumer": f.consumer.map(|c| format!("{FRAGMENT_DIR}/{c}")),
        "interface": "sherlock-c6-phys/1",
        "evidence": "source-only, uncompiled and unrun",
    })
}

fn row_wire(row: &Row) -> Value {
    json!({
        "rule": row.rule,
        "family": row.family.name(),
        "owner": row.family.owner(),
        "fragment": row.fragment.as_ref().map(fragment_wire),
        "narrow_carriers": row.narrow.iter().map(|n| json!({
            "file": format!("{FRAGMENT_DIR}/{}", n.file),
            "entry": n.entry,
            "names": n.names,
            "interface": "narrow byte carrier, not sherlock-c6-phys/1",
        })).collect::<Vec<_>>(),
    })
}

/// The whole hand-authored table, identical for every program.
pub fn table_wire() -> Value {
    let mut dispatch = fragment_wire(&DISPATCH);
    dispatch["rule"] = json!("dispatch");
    dispatch["supported"] = json!(
        "every enabled pure RuleIdC except the XAdtHelperPlan*/XAdtHelperBuild* helper groups"
    );
    json!({
        "provenance": TABLE_PROVENANCE,
        "note": "written by hand from the fragment sources; not generated from fragments, owner metadata or runs",
        "rows": TABLE.iter().map(row_wire).collect::<Vec<_>>(),
        "unexercised_c6_fragments": UNEXERCISED_C6.iter().map(|(rule, f)| {
            let mut v = fragment_wire(f);
            v["rule"] = json!(rule);
            v
        }).collect::<Vec<_>>(),
        "dispatch_fragments": [dispatch],
        "other_machine_fragments": OTHER_MACHINE.iter().map(|(file, rule, entry)| json!({
            "file": format!("{FRAGMENT_DIR}/{file}"),
            "rule": rule,
            "entry": entry,
            "machine": "sherlock-k-a/1",
        })).collect::<Vec<_>>(),
    })
}

fn owner_wire(owner: &str) -> Value {
    let rules: Vec<&str> = TABLE
        .iter()
        .filter(|r| r.family.owner() == owner)
        .map(|r| r.rule)
        .collect();
    let realized: Vec<&str> = TABLE
        .iter()
        .filter(|r| r.family.owner() == owner && r.fragment.is_some())
        .map(|r| r.rule)
        .collect();
    let families: Vec<&str> = [Schedule, Scalar, CallBind, FrameReturn, DataMatch]
        .into_iter()
        .filter(|f| f.owner() == owner)
        .map(Family::name)
        .collect();
    let (host, section) = match owner {
        "dispatch" => (
            "helper_machine::shared::owner_runtime::owner_step (selected_for)",
            json!(null),
        ),
        "scalar" => (
            "helper_machine::shared::owner_runtime Scalar helper plan",
            json!([
                format!("{FRAGMENT_DIR}/c6_checked_value_read.rs"),
                format!("{FRAGMENT_DIR}/c6_helper_canonical.rs"),
            ]),
        ),
        "frame" => (
            "helper_machine::shared::owner_runtime frame stack",
            json!("full frame canonicality emitted inline by each C6 frame fragment"),
        ),
        _ => (
            "helper_machine::shared::owner_runtime DATA/env/helper tasks",
            json!([
                format!("{FRAGMENT_DIR}/c6_data_canonical.rs"),
                format!("{FRAGMENT_DIR}/c6_helper_canonical.rs"),
            ]),
        ),
    };
    let mut v = json!({
        "families": families,
        "host_owner": host,
        "section_constraints": section,
        "rules": rules,
        "rules_with_c6_fragment": realized,
    });
    if owner == "dispatch" {
        v["native_selection"] = json!(null);
        v["candidate"] = json!(format!("{FRAGMENT_DIR}/{}", DISPATCH.file));
        v["status"] = json!(
            "open: a source-only dispatch owner candidate exists (per-rule Core selection over the C6 pre carriers); native_selection stays null until its tests are compiled and run"
        );
    }
    v
}

/// The UIREQ-023 inventory for one actual owner run.
pub fn report(label: &str, tally: &RuleTally) -> Value {
    let by_rule: BTreeMap<&str, &Row> = TABLE.iter().map(|r| (r.rule, r)).collect();
    let mut exercised = Vec::new();
    let mut unlisted = Vec::new();
    let mut gaps = Vec::new();
    for (rule, count) in &tally.counts {
        match by_rule.get(rule.as_str()) {
            Some(row) => {
                if row.fragment.is_none() {
                    gaps.push(rule.clone());
                }
                let mut v = row_wire(row);
                v["count"] = json!(count.to_string());
                exercised.push(v);
            }
            None => {
                unlisted.push(rule.clone());
                exercised.push(json!({
                    "rule": rule,
                    "count": count.to_string(),
                    "table": "unlisted",
                    "fragment": null,
                }));
            }
        }
    }
    let table = table_wire();
    let digest = crate::intent::hash_bytes(table.to_string().as_bytes());
    let with_fragment = exercised.len() - gaps.len() - unlisted.len();
    json!({
        "format": INVENTORY_FORMAT,
        "program": label,
        "execution_profile": "sherlock-helper-microstep/1",
        "table_provenance": TABLE_PROVENANCE,
        "table_sha256": digest,
        "realization": {
            "kind": "finite-family",
            "universal": false,
            "per_program": false,
            "basis": "one source-only emitter per selected RuleIdC over the shared C6 carriers; the same table serves every program",
        },
        "interface": {
            "name": "sherlock-c6-phys/1",
            "bounded": "fixed padded capacities (C6PhysCaps / C6PaddedCarrierGeometry)",
            "statement_prefix": ["enable", "disposition_is_refused_helper_work", "host_schema", "carrier_version"],
            "step_input_sections": ["META", "PROG", "CTX", "IMAGE", "DATA", "HELPER"],
            "step_output_sections": ["META", "PROG", "CTX", "IMAGE", "DATA", "HELPER"],
            "statement_source": "selected-step consumer headers (c6_*_consumer.rs)",
            "statement_modes": ["LocalPublic", "LedgerCall"],
        },
        "owners": {
            "dispatch": owner_wire("dispatch"),
            "scalar": owner_wire("scalar"),
            "frame": owner_wire("frame"),
            "data": owner_wire("data"),
            "sections": {
                "META": format!("{FRAGMENT_DIR}/c6_meta_canonical.rs"),
                "PROG+CTX": format!("{FRAGMENT_DIR}/c6_program_context_canonical.rs"),
                "DATA": format!("{FRAGMENT_DIR}/c6_data_canonical.rs"),
                "HELPER": format!("{FRAGMENT_DIR}/c6_helper_canonical.rs"),
                "IMAGE": "inline in each C6 fragment",
            },
        },
        "recursion_adapter": {
            "route": "ledger-9 custom decider",
            "relation": "crates/mori-recursion/src/decider.rs",
            "wired_children": [
                "crates/mori-backend/src/sherlock_child.rs (Candidate A A102)",
                "crates/mori-backend/src/sherlock_link.rs (Candidate A A102 -> A80)",
            ],
            "c6_fragments_wired": false,
            "status": "open: no C6 fragment is a recursion child yet",
        },
        "step_records": {
            "producer": "host materializeStep (sherlock_compile_artifact::draft_witness::materialize_boundary over the WitnessPlan)",
            "native_produces_step_records": false,
        },
        "prerequisites": {"verify_proof": false, "ledger10": false},
        "events": tally.events.to_string(),
        "exercised": exercised,
        "summary": {
            "rules": tally.counts.len(),
            "with_c6_fragment": with_fragment,
            "gaps": gaps,
            "unlisted": unlisted,
            "complete": gaps.is_empty() && unlisted.is_empty(),
        },
        "evidence": "inventory only: no SDK load, compile, proof or verification ran",
        "table": table,
    })
}
