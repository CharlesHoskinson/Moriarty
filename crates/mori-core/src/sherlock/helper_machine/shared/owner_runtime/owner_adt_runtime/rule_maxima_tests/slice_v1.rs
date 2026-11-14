//! Task V.1 of `openspec/changes/sherlock-modular-leaf-20261111` (row T6): the slice program
//! probe. Runs the candidate programs on the reference owner machine and records, per step, the
//! rule, the disposition, the appends per kind, the frame and task operations, the heap and ROM
//! rows read, and the image lanes before and after (`encode_root_and_frames`). With
//! `SLICE_OUT=<dir>` it writes `slice_v1.json` there.
use super::*;
use crate::sherlock::helper_machine::c6_phys::{C6PhysCaps, C6Root, encode_root_and_frames};
use crate::sherlock::helper_machine::c6_phys_helper::{
    encode_helper_record, encode_result_record, encode_task_record,
};

fn root_of(img: &MachineImage) -> C6Root {
    C6Root {
        control: img.control,
        control_origin: img.control_origin,
        env_root: img.env_root,
        stack_limit: img.stack_limit,
        outcome: img.outcome,
        initial_fuel: img.initial_fuel,
        remaining_work: img.remaining_work,
        consumed_work: img.consumed_work,
        helper_work: img.helper_work,
        helper_visits: img.helper_visits,
        reference_cursor: img.reference_cursor,
        alloc: img.alloc,
        active: img.active,
        seg_active: img.seg_base.active(),
        seg_alloc: img.seg_base.alloc(),
        seg_consumed_work: img.seg_base.consumed_work(),
    }
}

fn strs(v: &[u128]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

/// The image lanes: root and frames, the helper record, the result record, the tasks.
fn lanes(img: &MachineImage) -> serde_json::Value {
    let root = encode_root_and_frames(&root_of(img), &img.frames, &C6PhysCaps::test_caps())
        .expect("encode image lanes");
    serde_json::json!({
        "root_frames": strs(&root),
        "helper": strs(&encode_helper_record(img.helper)),
        "result": strs(&encode_result_record(img.result)),
        "tasks": img.tasks.iter().map(|t| strs(&encode_task_record(*t))).collect::<Vec<_>>(),
    })
}

/// DATA lanes `[form, p1, ctor, fstart, flen]` of the reference (`encode_data_row`).
fn data_lanes(r: &crate::sherlock::helper_machine::raw::DataRow) -> Vec<String> {
    use crate::sherlock::helper_machine::raw::DataRow as D;
    let l: [u128; 5] = match r {
        D::U128 { n } => [0, *n, 0, 0, 0],
        D::Bool { b } => [1, u128::from(*b), 0, 0, 0],
        D::Nil => [2, 0, 0, 0, 0],
        D::Cons { fields: f } => [3, 0, 0, f.start.0 as u128, f.len as u128],
        D::Leaf { fields: f } => [4, 0, 0, f.start.0 as u128, f.len as u128],
        D::Branch { fields: f } => [5, 0, 0, f.start.0 as u128, f.len as u128],
        D::Constructor {
            type_id,
            ctor_id,
            fields: f,
        } => [
            6,
            type_id.0 as u128,
            ctor_id.0 as u128,
            f.start.0 as u128,
            f.len as u128,
        ],
    };
    l.iter().map(|x| x.to_string()).collect()
}

fn mem_name(m: &Mem) -> &'static str {
    match m {
        Mem::Data => "DATA",
        Mem::Field => "FIELD",
        Mem::Env => "ENV",
        Mem::Slot => "SLOT",
    }
}

/// The ROM rows of a program view as reference lanes, per FP-9.2 table id (0 `expressions` to
/// 21 `names`). Only the tables that the slice candidates use are written; any other non-empty
/// table panics, so no row is silently left out of `prog_root`.
/// - expressions: `[kind, lane1..lane6]` in the `encode_program` field order;
/// - occurrences: `[owner_is_function, function, path.start, path.len]`;
/// - types: `[name_id, decl.present, decl.value, args.start, args.len, ctor_start, ctor_len]`;
/// - constructors: `[owning_type, name_id, source_order, field_start, field_len]`;
/// - ctor_fields: `[name_id, type_id]`;
/// - names: the name bytes.
fn rom_rows(v: &ProgramView) -> serde_json::Value {
    use serde_json::json;
    let n = |x: usize| x.to_string();
    let exprs: Vec<Vec<String>> = v
        .expressions
        .iter()
        .map(|e| match e {
            ExpressionRowC::Var { binder } => vec![n(1), n(*binder), n(0), n(0), n(0), n(0), n(0)],
            _ => panic!("slice ROM export: expression row kind not written"),
        })
        .collect();
    let occs: Vec<Vec<String>> = v
        .occurrences
        .iter()
        .map(|o| {
            vec![
                n(usize::from(o.owner_is_function)),
                n(o.function),
                n(o.path.start),
                n(o.path.len),
            ]
        })
        .collect();
    let types: Vec<Vec<String>> = v
        .types
        .iter()
        .map(|t| {
            vec![
                n(t.name_id),
                n(usize::from(t.decl.is_some())),
                n(t.decl.unwrap_or(0)),
                n(t.args.start),
                n(t.args.len),
                n(t.ctor_start.0),
                n(t.ctor_len),
            ]
        })
        .collect();
    let ctors: Vec<Vec<String>> = v
        .constructors
        .iter()
        .map(|c| {
            vec![
                n(c.owning_type.0),
                n(c.name_id),
                n(c.source_order),
                n(c.field_start),
                n(c.field_len),
            ]
        })
        .collect();
    let cfields: Vec<Vec<String>> = v
        .ctor_fields
        .iter()
        .map(|f| vec![n(f.name_id), n(f.type_id.0)])
        .collect();
    let empty =
        |name: &str, len: usize| assert_eq!(len, 0, "slice ROM export: table {name} not written");
    empty("occurrence_paths", v.occurrence_paths.len());
    empty("children", v.children.len());
    empty("functions", v.functions.len());
    empty("parameters", v.parameters.len());
    empty("arms", v.arms.len());
    empty("patterns", v.patterns.len());
    empty("pattern_children", v.pattern_children.len());
    empty("type_args", v.type_args.len());
    empty("decl_param_counts", v.decl_param_counts.len());
    empty("decl_ctor_counts", v.decl_ctor_counts.len());
    empty("decl_templates", v.decl_templates.len());
    empty("plan_nodes", v.plan_nodes.len());
    empty("plan_cases", v.plan_cases.len());
    empty("field_paths", v.field_paths.len());
    empty("constants", v.constants.len());
    empty("constant_fields", v.constant_fields.len());
    json!({
        "entry": v.entry,
        "0": exprs, "1": occs, "9": types, "11": ctors, "12": cfields,
        "21": v.names.iter().map(|s| s.bytes().map(|b| b.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>(),
    })
}

/// One candidate run: the step records, or the refusal text.
fn probe(p: &Program) -> serde_json::Value {
    use serde_json::json;
    let parsed = match crate::sherlock_nominal_source::parse_nominal_source(&p.source) {
        Ok(x) => x,
        Err(e) => {
            return json!({"name": p.name, "source": p.source, "refused": format!("parse: {e:?}"), "steps": []});
        }
    };
    let (bounds, layout) = bounds_for(p);
    let Ok((admitted, _)) = admit_ext_program(
        parsed.program,
        ExtDomain::GENERIC_ABSENT,
        &bounds,
        &layout,
        None,
    ) else {
        return json!({"name": p.name, "source": p.source, "refused": "admission", "steps": []});
    };
    let view_rows: Vec<(String, usize)> = admitted
        .body
        .view
        .as_ref()
        .map(|v| {
            x_table_lens(v)
                .into_iter()
                .map(|(n, l)| (n.to_string(), l))
                .collect()
        })
        .unwrap_or_default();
    let rom = if p.name == "unbound" {
        admitted
            .body
            .view
            .as_ref()
            .map(rom_rows)
            .unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };
    let mut img = host_owner_engine()
        .admit_run(
            &admitted,
            &RunIntC::Nat(p.fuel),
            &RunIntC::Nat(p.stack),
            &FinanceInput::Absent,
        )
        .expect("admit_run");
    let fam = HostFamilyPlans;
    let g = OwnerGuards::new_owner();
    let mut steps = vec![];
    let genesis = lanes(&img);
    for step in 0..10_000usize {
        if img.outcome != OutcomeC::Running {
            break;
        }
        let publishes = img.helper.is_none()
            && img.result.is_none()
            && (matches!(img.control, ControlC::Error { .. })
                || (matches!(img.control, ControlC::Return { .. }) && img.frames.is_empty()));
        let planned = if publishes || img.remaining_work == 0 {
            None
        } else {
            let pre = PreImage::bind(&admitted, &img).expect("bind");
            Some(plan_logged(&pre, &fam, &g).expect("plan"))
        };
        let before = lanes(&img);
        let pre_control = format!("{:?}", img.control);
        let pre_data = img.data.len();
        let ev = match owner_step(&admitted, &mut img, &fam, &g).expect("owner_step") {
            StepResult::Terminal => break,
            StepResult::Event { event } => event,
        };
        let new_data: Vec<Vec<String>> = img.data[pre_data..].iter().map(data_lanes).collect();
        let rec = match &planned {
            Some(pl) => json!({
                "appends": pl.appends.iter().map(mem_name).collect::<Vec<_>>(),
                "frame_push_pop_rewrite": [pl.frame.0, pl.frame.1, pl.frame.2],
                "task_push_pop_rewrite": [pl.task.0, pl.task.1, pl.task.2],
                "heap_reads": pl.reads.iter().chain(pl.sel_reads.iter()).filter(|r| matches!(r, Rd::M(..))).map(|r| format!("{r:?}")).collect::<Vec<_>>(),
                "rom_reads_logged": pl.reads.iter().chain(pl.sel_reads.iter()).filter(|r| matches!(r, Rd::R(..))).map(|r| format!("{r:?}")).collect::<Vec<_>>(),
                "rom_rows_probe": pl.x_sel.union(&pl.x_fam).map(|(t, i)| format!("{t}[{i}]")).collect::<Vec<_>>(),
            }),
            None => json!({"publication": true}),
        };
        steps.push(json!({
            "step": step,
            "rule": rule_name(&ev.rule_id),
            "disposition": format!("{:?}", ev.disposition),
            "pre_control": pre_control,
            "post_control": format!("{:?}", img.control),
            "post_outcome": format!("{:?}", img.outcome),
            "new_data_rows": new_data,
            "plan": rec,
            "lanes_before": before,
            "lanes_after": lanes(&img),
        }));
    }
    json!({
        "name": p.name,
        "source": p.source,
        "fuel": p.fuel.to_string(),
        "stack": p.stack.to_string(),
        "rom_table_lens": view_rows,
        "rom_rows": rom,
        "genesis_lanes": genesis,
        "outcome": format!("{:?}", img.outcome),
        "frames_at_end": img.frames.len(),
        "tasks_at_end": img.tasks.len(),
        "data_at_end": img.data.len(),
        "fields_at_end": img.data_fields.len(),
        "env_at_end": img.env.len(),
        "slots_at_end": img.value_slots.len(),
        "steps": steps,
    })
}

#[test]
fn slice_v1_candidates() {
    let want = ["literal", "boolconst", "underflow", "overflow"];
    let mut progs: Vec<Program> = corpus()
        .into_iter()
        .filter(|p| want.contains(&p.name))
        .collect();
    assert_eq!(progs.len(), want.len());
    let p = |name, body: &str| Program {
        name,
        source: format!("language \"moriarty-sherlock-nominal/1\"; program P {{ {body} }}"),
        fuel: 4096,
        stack: 64,
        data_nodes: 512,
    };
    // Entry forms that might avoid a task step: a builtin nullary constructor, a declared
    // nullary constructor, an unbound variable (admission is expected to refuse it).
    progs.push(p("nil", "entry nil;"));
    progs.push(p("unitctor", "type U { A(); } entry construct U::A();"));
    progs.push(p("unbound", "entry x;"));
    let runs: Vec<serde_json::Value> = progs.iter().map(probe).collect();
    for r in &runs {
        let rules: Vec<&str> = r["steps"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["rule"].as_str().unwrap())
            .collect();
        eprintln!(
            "SLICEV1 {} outcome={} refused={} rules={:?}",
            r["name"], r["outcome"], r["refused"], rules
        );
    }
    // The slice program (V.1): `entry x;`, the run with no stack step, no append and one ROM row.
    let slice = runs.iter().find(|r| r["name"] == "unbound").unwrap();
    let rules: Vec<&str> = slice["steps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["rule"].as_str().unwrap())
        .collect();
    assert_eq!(
        rules,
        [
            "BHelperEnvGetEntry",
            "BHelperEnvGetEmpty",
            "BHelperEnvGetFailure",
            "KMachineEvalVariableAbsent",
            "KAdminPublishError"
        ]
    );
    for st in slice["steps"].as_array().unwrap() {
        if let Some(pl) = st["plan"].as_object().filter(|o| o.contains_key("appends")) {
            assert!(pl["appends"].as_array().unwrap().is_empty(), "no append");
            assert_eq!(pl["frame_push_pop_rewrite"], serde_json::json!([0, 0, 0]));
            assert_eq!(pl["task_push_pop_rewrite"], serde_json::json!([0, 0, 0]));
            assert_eq!(pl["rom_rows_probe"], serde_json::json!(["expressions[0]"]));
        }
    }
    if let Ok(dir) = std::env::var("SLICE_OUT") {
        std::fs::write(
            format!("{dir}/slice_v1.json"),
            serde_json::to_string_pretty(&serde_json::json!({"runs": runs})).unwrap(),
        )
        .unwrap();
    }
}
