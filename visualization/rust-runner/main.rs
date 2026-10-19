#![allow(dead_code, unused_imports)]

extern crate mori_lexer;

#[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/draft_source.rs"]
pub mod sherlock_draft;

#[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/draft_typecheck.rs"]
pub mod sherlock_draft_types;

#[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/draft_image_bridge.rs"]
pub mod sherlock_draft_image;

pub mod sherlock {
    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/compare.rs"]
    pub mod compare;
    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/reference.rs"]
    mod reference;
    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/reference_stream.rs"]
    mod reference_stream;
    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/reference_display.rs"]
    pub mod reference_display;
    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/reference_types.rs"]
    mod reference_types;

    pub use reference::Machine;
    pub use reference_stream::{VisitResult, VisitStop};
    pub use reference_types::*;

    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/program_image.rs"]
    pub(crate) mod program_image;

    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/image_reference.rs"]
    pub(crate) mod image_reference;

    #[path = "/home/charles/sherlock-bench/rust-runner/reference_slice.rs"]
    pub(crate) mod reference_slice;

    #[path = "/home/charles/sherlock-bench/rust-runner/machine_image.rs"]
    pub(crate) mod machine_image;

    #[path = "/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/arithmetic_witness.rs"]
    pub(crate) mod arithmetic_witness;

    pub mod trace_export {
        use super::arithmetic_witness::{materialize_arithmetic, materialize_fuel, ArithmeticOp};
        use super::image_reference::ImageMachine;
        use super::machine_image::{encode_snapshot, HostImageLimits};
        use super::program_image::{ExprId, ExprPayload};
        use super::reference_slice::{advance_m, SliceBoundaryKind};
        use super::reference_types::*;
        use super::Machine;
        use crate::sherlock_draft_image::prepare_draft_image;

        fn display_expr(e: &Expr) -> String {
            match e {
                Expr::Literal(v) => value_summary(v),
                Expr::Var(n) => n.clone(),
                Expr::Let(n, val, body) => format!("let {n} = {} in {}", display_expr(val), display_expr(body)),
                Expr::Binary(op, l, r) => format!("({} {op:?} {})", display_expr(l), display_expr(r)),
                Expr::Unary(op, x) => format!("{op:?}({})", display_expr(x)),
                Expr::If(c, y, n) => format!("if {} {{ ... }}", display_expr(c)),
                Expr::Match(s, arms) => format!("match {} ({} arms)", display_expr(s), arms.len()),
                Expr::Call(name, args) => {
                    let a: Vec<String> = args.iter().map(|x| display_expr(x)).collect();
                    format!("{name}({})", a.join(", "))
                }
            }
        }

        fn json_escape(s: &str) -> String {
            let mut out = String::with_capacity(s.len() + 4);
            for ch in s.chars() {
                match ch {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out
        }

        fn rule_name(line: usize) -> &'static str {
            match line {
                80 => "Done (Free Publication)",
                82 => "Rejected (Free Publication)",
                84 => "Exhausted (Free Publication)",
                102 => "Literal",
                108 => "Var found",
                111 => "Var missing -> error(3)",
                114 => "Let",
                117 => "Binary left",
                120 => "Unary",
                123 => "If",
                126 => "Match",
                129 => "Call()",
                131 => "Call(args)",
                135 => "Push frame",
                140 => "Frame limit -> error(4)",
                145 => "Bind let",
                149 => "Right operand",
                153 => "Binary primitive",
                157 => "Unary primitive",
                161 => "If true",
                165 => "If false",
                169 => "If non-bool -> error(5)",
                173 => "Start cases",
                177 => "Next arg",
                181 => "Finish args",
                185 => "Return type ok",
                190 => "Return type mismatch -> error(6)",
                195 => "Restore scope",
                200 => "Lookup fn",
                202 => "Missing fn -> error(7)",
                204 => "Bind params",
                206 => "Bind arity/type -> error(8)",
                208 => "Schedule body",
                212 => "No match -> error(9)",
                214 => "Try pattern",
                217 => "Pattern miss",
                220 => "Match arm body",
                223 => "Primitive ok",
                225 => "Primitive error(10/11)",
                _ => "Transition",
            }
        }

        fn value_summary(v: &Value) -> String {
            match v {
                Value::UInt(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::List(items) => {
                    let parts: Vec<String> = items.iter().map(|n| n.to_string()).collect();
                    format!("[{}]", parts.join(", "))
                }
                Value::Tree(t) => tree_summary(t),
            }
        }

        fn tree_summary(t: &Tree) -> String {
            match t {
                Tree::Leaf(n) => format!("leaf({n})"),
                Tree::Branch(l, r) => format!("branch({}, {})", tree_summary(l), tree_summary(r)),
            }
        }

        fn value_type_name(v: &Value) -> &'static str {
            match v {
                Value::UInt(_) => "UInt",
                Value::Bool(_) => "Bool",
                Value::List(_) => "List",
                Value::Tree(_) => "Tree",
            }
        }

        fn value_json(v: &Value) -> String {
            match v {
                Value::UInt(n) => {
                    let bytes = n.to_le_bytes();
                    let limbs: Vec<String> = bytes.iter().map(|b| b.to_string()).collect();
                    format!(
                        "{{\"kind\":\"UInt\",\"value\":\"{n}\",\"number\":{},\"limbs\":[{}],\"summary\":\"{}\"}}",
                        if *n <= 9_007_199_254_740_991 { n.to_string() } else { "9007199254740991".to_string() },
                        limbs.join(","),
                        json_escape(&value_summary(v))
                    )
                }
                Value::Bool(b) => format!(
                    "{{\"kind\":\"Bool\",\"value\":{b},\"summary\":\"{b}\"}}"
                ),
                Value::List(items) => {
                    let els: Vec<String> = items.iter().map(|n| n.to_string()).collect();
                    format!(
                        "{{\"kind\":\"List\",\"elements\":[{}],\"length\":{},\"summary\":\"{}\"}}",
                        els.join(","),
                        items.len(),
                        json_escape(&value_summary(v))
                    )
                }
                Value::Tree(t) => {
                    format!(
                        "{{\"kind\":\"Tree\",\"tree\":{},\"summary\":\"{}\"}}",
                        tree_json(t),
                        json_escape(&value_summary(v))
                    )
                }
            }
        }

        fn tree_json(t: &Tree) -> String {
            match t {
                Tree::Leaf(n) => format!("{{\"kind\":\"Leaf\",\"value\":{n},\"label\":\"leaf({n})\"}}"),
                Tree::Branch(l, r) => format!(
                    "{{\"kind\":\"Branch\",\"left\":{},\"right\":{},\"label\":\"branch\"}}",
                    tree_json(l),
                    tree_json(r)
                ),
            }
        }

        fn line_number(source: &str, byte_offset: u32) -> usize {
            let end = (byte_offset as usize).min(source.len());
            source[..end].bytes().filter(|&b| b == b'\n').count() + 1
        }

        pub fn compile_and_trace_json(
            source: &str,
            fuel: u128,
            stack_limit: usize,
            slice_width: usize,
        ) -> String {
            let prep = match prepare_draft_image(source) {
                Ok(p) => p,
                Err(err) => {
                    return format!(
                        "{{\"ok\":false,\"engine\":\"rustc-moriarty-native\",\"error\":\"{}\"}}",
                        json_escape(&format!("{err:?}"))
                    );
                }
            };

            let image = prep.image.clone();
            let mut expr_rows_json = Vec::new();
            for (idx, row) in image.expressions.iter().enumerate() {
                let site = &prep.image_sources[idx].source;
                let span = site.span;
                let line = line_number(source, span.start);
                let text_slice = &source[(span.start as usize).min(source.len())..(span.end as usize).min(source.len())];
                let (kind_str, op_str, name_str) = match &row.payload {
                    ExprPayload::Literal(cid) => ("Literal", "", format!("const#{}", cid.0)),
                    ExprPayload::Var(n) => ("Var", "", n.clone()),
                    ExprPayload::Let(n) => ("Let", "=", n.clone()),
                    ExprPayload::Binary(op) => ("Binary", match op {
                        BinaryOp::Add => "+",
                        BinaryOp::Sub => "-",
                        BinaryOp::Mul => "*",
                        BinaryOp::Eq => "==",
                        BinaryOp::Lt => "<",
                        BinaryOp::Cons => "cons",
                        BinaryOp::Branch => "branch",
                    }, String::new()),
                    ExprPayload::Unary(op) => ("Unary", match op {
                        UnaryOp::Leaf => "leaf",
                        UnaryOp::Not => "not",
                    }, String::new()),
                    ExprPayload::If => ("If", "if", String::new()),
                    ExprPayload::Call(n) => ("Call", "call", n.clone()),
                    ExprPayload::Match(_) => ("Match", "match", String::new()),
                };
                let owner_str = match site.source.function_index {
                    None => "entry".to_string(),
                    Some(fi) => format!("fn[{}]:{}", fi, prep.draft.program.functions[fi].name),
                };
                let path_parts: Vec<String> = site.source.child_path.iter().map(|p| p.to_string()).collect();
                let ast_path = format!("{}/{}", owner_str, path_parts.join("."));
                let children: Vec<String> = image.ordered_children[row.children.start..row.children.start + row.children.len]
                    .iter()
                    .map(|c| c.0.to_string())
                    .collect();
                expr_rows_json.push(format!(
                    "{{\"id\":{idx},\"kind\":\"{kind_str}\",\"op\":\"{}\",\"name\":\"{}\",\"children\":[{}],\"line\":{line},\"start\":{},\"end\":{},\"astPath\":\"{}\",\"text\":\"{}\"}}",
                    json_escape(op_str),
                    json_escape(&name_str),
                    children.join(","),
                    span.start,
                    span.end,
                    json_escape(&ast_path),
                    json_escape(text_slice)
                ));
            }

            let mut fn_rows_json = Vec::new();
            for f in &image.functions {
                let params: Vec<String> = image.ordered_parameters[f.parameters.start..f.parameters.start + f.parameters.len]
                    .iter()
                    .map(|p| format!("{{\"name\":\"{}\",\"type\":\"{:?}\"}}", json_escape(&p.name), p.ty))
                    .collect();
                fn_rows_json.push(format!(
                    "{{\"id\":{},\"name\":\"{}\",\"params\":[{}],\"returnType\":\"{:?}\",\"bodyExpr\":{}}}",
                    f.source_table_index,
                    json_escape(&f.name),
                    params.join(","),
                    f.return_type,
                    f.body.0
                ));
            }

            let mut machine = ImageMachine::new(image.clone(), fuel, stack_limit);
            let mut steps_json = Vec::new();
            let mut last_focus_value: Option<Value> = None;

            let limits = HostImageLimits {
                rows: 4096,
                cells: 8192,
                name_bytes: 16384,
            };

            let max_host_steps = 4096usize;
            let mut step_idx = 0usize;
            while step_idx < max_host_steps {
                let Some(record) = machine.step() else {
                    break;
                };
                let ctrl_origin = machine.control_origin();
                let frame_origins: Vec<ExprId> = machine.frame_origins().to_vec();

                let (ctrl_kind, ctrl_summary, ctrl_val_opt) = match &record.after.control {
                    Some(Control::Eval(e)) => ("Eval", display_expr(e), None),
                    Some(Control::Return(v)) => ("Return", format!("Return({})", value_summary(v)), Some(v.clone())),
                    Some(Control::Push(_, _, _)) => ("Push", "Push(frame)".to_string(), None),
                    Some(Control::Invoke(name, args)) => {
                        let a: Vec<String> = args.iter().map(value_summary).collect();
                        ("Invoke", format!("Invoke({}({}))", name, a.join(", ")), args.first().cloned())
                    }
                    Some(Control::Enter(f, args)) => {
                        let fn_name = f.as_ref().map(|func| func.name.as_str()).unwrap_or("?");
                        ("Enter", format!("Enter({fn_name})"), args.first().cloned())
                    }
                    Some(Control::Bound(_, ty, _)) => ("Bound", format!("Bound(: {ty:?})"), None),
                    Some(Control::Cases(v, _, _)) => ("Cases", format!("Cases({})", value_summary(v)), Some(v.clone())),
                    Some(Control::CaseResult(_, v, _, _, _)) => ("CaseResult", format!("CaseResult({})", value_summary(v)), Some(v.clone())),
                    Some(Control::PrimitiveResult(Ok(v))) => ("PrimitiveResult", format!("PrimOk({})", value_summary(v)), Some(v.clone())),
                    Some(Control::PrimitiveResult(Err(code))) => ("PrimitiveResult", format!("PrimErr({code})"), None),
                    Some(Control::Error(code)) => ("Error", format!("Error({code})"), None),
                    None => match &record.after.outcome {
                        Outcome::Done(v) => ("Return", format!("Done({})", value_summary(v)), Some(v.clone())),
                        Outcome::Rejected(code) => ("Error", format!("Rejected({code})"), None),
                        Outcome::Exhausted => ("Error", "Exhausted".to_string(), None),
                        Outcome::Pending => ("Eval", "Pending".to_string(), None),
                    },
                };

                if let Some(ref v) = ctrl_val_opt {
                    last_focus_value = Some(v.clone());
                } else {
                    for fr in &record.after.frames {
                        match fr {
                            Frame::Right(_, v) => {
                                last_focus_value = Some(v.clone());
                                break;
                            }
                            Frame::Args(_, vals, _, _) if !vals.is_empty() => {
                                last_focus_value = vals.last().cloned();
                                break;
                            }
                            _ => {}
                        }
                    }
                    if last_focus_value.is_none() {
                        if let Some((_, v)) = record.after.environment.iter().last() {
                            last_focus_value = Some(v.clone());
                        }
                    }
                }

                let origin_id = ctrl_origin.0;
                let site = &prep.image_sources[origin_id.min(prep.image_sources.len() - 1)].source;
                let origin_line = line_number(source, site.span.start);
                let origin_text = &source[(site.span.start as usize).min(source.len())..(site.span.end as usize).min(source.len())];
                let owner_str = match site.source.function_index {
                    None => "entry".to_string(),
                    Some(fi) => format!("fn[{}]:{}", fi, prep.draft.program.functions[fi].name),
                };
                let path_parts: Vec<String> = site.source.child_path.iter().map(|p| p.to_string()).collect();
                let origin_ast_path = format!("{}/{}", owner_str, path_parts.join("."));

                let mut frames_json = Vec::new();
                for (fidx, fr) in record.after.frames.iter().enumerate() {
                    let f_origin = frame_origins.get(fidx).map(|e| e.0).unwrap_or(0);
                    let f_site = &prep.image_sources[f_origin.min(prep.image_sources.len() - 1)].source;
                    let f_owner = match f_site.source.function_index {
                        None => "entry".to_string(),
                        Some(fi) => format!("fn[{}]:{}", fi, prep.draft.program.functions[fi].name),
                    };
                    let f_path: Vec<String> = f_site.source.child_path.iter().map(|p| p.to_string()).collect();
                    let f_ast_path = format!("{}/{}", f_owner, f_path.join("."));

                    let (fkind, flabel, fdetail, fenv) = match fr {
                        Frame::Let(name, body, env) => (
                            "Let",
                            format!("let {name} = [hole]"),
                            display_expr(body),
                            Some(env),
                        ),
                        Frame::Left(op, right, env) => (
                            "Left",
                            format!("[hole] {op:?} {}", display_expr(right)),
                            format!("op: {op:?}"),
                            Some(env),
                        ),
                        Frame::Right(op, left_val) => (
                            "Right",
                            format!("{} {op:?} [hole]", value_summary(left_val)),
                            format!("lhs = {}", value_summary(left_val)),
                            None,
                        ),
                        Frame::Unary(op) => (
                            "Unary",
                            format!("{op:?}([hole])"),
                            format!("unary {op:?}"),
                            None,
                        ),
                        Frame::If(yes, no, env) => (
                            "If",
                            "if [hole] { ... } else { ... }".to_string(),
                            format!("then: {} | else: {}", display_expr(yes), display_expr(no)),
                            Some(env),
                        ),
                        Frame::Match(arms, env) => (
                            "Match",
                            format!("match [hole] ({} arms)", arms.len()),
                            "pattern dispatch".to_string(),
                            Some(env),
                        ),
                        Frame::Args(name, evaluated, remaining, env) => (
                            "Args",
                            format!("{name}({}/{} args)", evaluated.len(), evaluated.len() + remaining.len()),
                            format!("done: {}, pending: {}", evaluated.len(), remaining.len()),
                            Some(env),
                        ),
                        Frame::Return(ty, env) => (
                            "Return",
                            format!("return : {ty:?}"),
                            format!("contract check {ty:?}"),
                            Some(env),
                        ),
                        Frame::Scope(env) => (
                            "Scope",
                            "restore scope".to_string(),
                            format!("{} bindings", env.len()),
                            Some(env),
                        ),
                    };

                    let bindings_json: Vec<String> = fenv
                        .map(|e| {
                            e.iter()
                                .map(|(k, v)| {
                                    format!(
                                        "{{\"name\":\"{}\",\"valueSummary\":\"{}\",\"type\":\"{}\"}}",
                                        json_escape(k),
                                        json_escape(&value_summary(v)),
                                        value_type_name(v)
                                    )
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    frames_json.push(format!(
                        "{{\"index\":{fidx},\"kind\":\"{fkind}\",\"label\":\"{}\",\"detail\":\"{}\",\"originExprId\":{f_origin},\"astPath\":\"{}\",\"envId\":{},\"bindings\":[{}]}}",
                        json_escape(&flabel),
                        json_escape(&fdetail),
                        json_escape(&f_ast_path),
                        fidx + 1,
                        bindings_json.join(",")
                    ));
                }

                let active_env_bindings: Vec<String> = record
                    .after
                    .environment
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{{\"name\":\"{}\",\"valueSummary\":\"{}\",\"type\":\"{}\"}}",
                            json_escape(k),
                            json_escape(&value_summary(v)),
                            value_type_name(v)
                        )
                    })
                    .collect();

                let (mi_exprs, mi_vals, mi_envs, mi_binds, mi_frames, mi_ctrls, mi_rows, mi_cells, mi_names) =
                    match encode_snapshot(&record.after, limits) {
                        Ok(img) => {
                            let u = img.usage();
                            (
                                prep.image.expressions.len(),
                                u.rows.saturating_sub(prep.image.expressions.len() + record.after.frames.len()),
                                record.after.frames.len() + 1,
                                record.after.environment.len(),
                                record.after.frames.len(),
                                usize::from(record.after.control.is_some()),
                                u.rows,
                                u.cells,
                                u.name_bytes,
                            )
                        }
                        Err(_) => (prep.image.expressions.len(), 0, 1, 0, record.after.frames.len(), 1, 0, 0, 0),
                    };

                let (aux1_kind, aux1_target, aux1_probes_json) = match record.rule_start_line {
                    108 | 111 => {
                        let var_name = match &record.before.control {
                            Some(Control::Eval(e)) => display_expr(e),
                            _ => "x".to_string(),
                        };
                        let probes: Vec<String> = record
                            .before
                            .environment
                            .iter()
                            .enumerate()
                            .map(|(pi, (k, _))| {
                                format!(
                                    "{{\"index\":{pi},\"name\":\"{}\",\"hit\":{}}}",
                                    json_escape(k),
                                    k == &var_name
                                )
                            })
                            .collect();
                        ("EnvGetScan", var_name, probes)
                    }
                    200 | 202 => {
                        let fn_target = match &record.before.control {
                            Some(Control::Invoke(n, _)) => n.clone(),
                            _ => "fn".to_string(),
                        };
                        let probes: Vec<String> = prep
                            .draft
                            .program
                            .functions
                            .iter()
                            .enumerate()
                            .map(|(fi, f)| {
                                format!(
                                    "{{\"index\":{fi},\"name\":\"{}\",\"hit\":{}}}",
                                    json_escape(&f.name),
                                    f.name == fn_target
                                )
                            })
                            .collect();
                        ("FindScan", fn_target, probes)
                    }
                    204 | 206 | 214 | 217 => ("BindPreflight", "params".to_string(), vec![]),
                    153 if matches!(record.before.frames.first(), Some(Frame::Right(BinaryOp::Eq, _))) => {
                        ("EqScratch", "==".to_string(), vec![])
                    }
                    _ => ("None", String::new(), vec![]),
                };

                let mut arith_active = false;
                let mut arith_op_str = "none";
                let mut lhs_str = "0".to_string();
                let mut rhs_str = "0".to_string();
                let mut res_str = "0".to_string();
                let mut lhs_bytes = [0u8; 16];
                let mut rhs_bytes = [0u8; 16];
                let mut arith_witness = materialize_arithmetic(None, None, None);

                if record.rule_start_line == 153 {
                    if let (Some(Frame::Right(op, Value::UInt(l))), Some(Control::Return(Value::UInt(r)))) =
                        (record.before.frames.first(), &record.before.control)
                    {
                        lhs_bytes = l.to_le_bytes();
                        rhs_bytes = r.to_le_bytes();
                        lhs_str = l.to_string();
                        rhs_str = r.to_string();
                        let aop = match op {
                            BinaryOp::Add => {
                                arith_op_str = "+";
                                Some(ArithmeticOp::Add)
                            }
                            BinaryOp::Sub => {
                                arith_op_str = "-";
                                Some(ArithmeticOp::Sub)
                            }
                            BinaryOp::Mul => {
                                arith_op_str = "*";
                                Some(ArithmeticOp::Mul)
                            }
                            BinaryOp::Lt => {
                                arith_op_str = "<";
                                Some(ArithmeticOp::Lt)
                            }
                            BinaryOp::Eq => {
                                arith_op_str = "==";
                                Some(ArithmeticOp::Sub)
                            }
                            _ => None,
                        };
                        if aop.is_some() {
                            arith_active = true;
                            arith_witness = materialize_arithmetic(aop, Some(&lhs_bytes), Some(&rhs_bytes));
                            res_str = match &record.after.control {
                                Some(Control::PrimitiveResult(Ok(v))) => value_summary(v),
                                Some(Control::PrimitiveResult(Err(code))) => format!("overflow(err {code})"),
                                _ => "0".to_string(),
                            };
                        }
                    }
                }

                let result_bytes = if arith_witness.sel_add {
                    arith_witness.add.r
                } else if arith_witness.sel_sub || arith_witness.sel_lt {
                    arith_witness.sub.r
                } else if arith_witness.sel_mul {
                    arith_witness.mul.r
                } else {
                    [0u8; 16]
                };

                let carries_vec: Vec<String> = if arith_witness.sel_mul {
                    arith_witness.mul.c[..17].iter().map(|c| c.to_string()).collect()
                } else {
                    arith_witness.add.c.iter().map(|&b| u8::from(b).to_string()).collect()
                };
                let borrows_vec: Vec<String> = arith_witness
                    .sub
                    .bo
                    .iter()
                    .map(|&b| u8::from(b).to_string())
                    .collect();
                let mut diag_vec = Vec::with_capacity(32);
                for k in 0..16 {
                    diag_vec.push(arith_witness.mul.r[k].to_string());
                }
                for k in 0..16 {
                    diag_vec.push(arith_witness.mul.h[k].to_string());
                }

                let pre_fuel_bytes = record.before.remaining_fuel.to_le_bytes();
                let is_term = matches!(record.rule_start_line, 80 | 82);
                let fuel_witness = materialize_fuel(true, is_term, &pre_fuel_bytes);

                let lhs_b_str: Vec<String> = lhs_bytes.iter().map(|b| b.to_string()).collect();
                let rhs_b_str: Vec<String> = rhs_bytes.iter().map(|b| b.to_string()).collect();
                let res_b_str: Vec<String> = result_bytes.iter().map(|b| b.to_string()).collect();
                let pre_fb_str: Vec<String> = pre_fuel_bytes.iter().map(|b| b.to_string()).collect();
                let post_fb_str: Vec<String> = fuel_witness.post.iter().map(|b| b.to_string()).collect();
                let zp_str: Vec<String> = fuel_witness.zp.iter().map(|&b| u8::from(b).to_string()).collect();
                let bw_str: Vec<String> = fuel_witness.bw.iter().map(|&b| u8::from(b).to_string()).collect();

                let ctrl_val_json = ctrl_val_opt.as_ref().map(value_json).unwrap_or_else(|| "null".to_string());
                let focus_val_json = last_focus_value.as_ref().map(value_json).unwrap_or_else(|| "null".to_string());

                steps_json.push(format!(
                    "{{\"index\":{step_idx},\"ruleId\":\"A{:03}\",\"ruleLine\":{},\"ruleName\":\"{}\",\"charge\":{},\"controlKind\":\"{ctrl_kind}\",\"controlSummary\":\"{}\",\"controlValue\":{ctrl_val_json},\"focusValue\":{focus_val_json},\"before\":{{\"fuel\":{},\"frameCount\":{}}},\"after\":{{\"fuel\":{},\"stackLimit\":{},\"control\":{{\"kind\":\"{ctrl_kind}\",\"exprId\":{origin_id},\"summary\":\"{}\",\"value\":{ctrl_val_json}}},\"controlOrigin\":{{\"exprId\":{origin_id},\"line\":{origin_line},\"start\":{},\"end\":{},\"astPath\":\"{}\",\"text\":\"{}\"}},\"frames\":[{}],\"envRoots\":[{{\"id\":0,\"bindings\":[{}]}}],\"machineImage\":{{\"expressions\":{mi_exprs},\"values\":{mi_vals},\"environments\":{mi_envs},\"bindings\":{mi_binds},\"frames\":{mi_frames},\"controls\":{mi_ctrls},\"usage\":{{\"rows\":{mi_rows},\"cells\":{mi_cells},\"nameBytes\":{mi_names}}}}},\"aux1\":{{\"activeKind\":\"{aux1_kind}\",\"targetName\":\"{}\",\"probes\":[{}]}},\"aux2\":{{\"arithmetic\":{{\"active\":{arith_active},\"op\":\"{arith_op_str}\",\"lhs\":\"{}\",\"rhs\":\"{}\",\"result\":\"{}\",\"overflow\":{},\"lhsBytes\":[{}],\"rhsBytes\":[{}],\"resultBytes\":[{}],\"carries\":[{}],\"borrows\":[{}],\"diagonals\":[{}]}},\"fuel\":{{\"before\":{},\"after\":{},\"beforeBytes\":[{}],\"afterBytes\":[{}],\"zeroProbe\":[{}],\"borrows\":[{}]}}}}}}}}",
                    record.rule_start_line,
                    record.rule_start_line,
                    json_escape(rule_name(record.rule_start_line)),
                    record.charge,
                    json_escape(&ctrl_summary),
                    record.before.remaining_fuel,
                    record.before.frames.len(),
                    record.after.remaining_fuel,
                    record.after.stack_limit,
                    json_escape(&ctrl_summary),
                    site.span.start,
                    site.span.end,
                    json_escape(&origin_ast_path),
                    json_escape(origin_text),
                    frames_json.join(","),
                    active_env_bindings.join(","),
                    json_escape(&aux1_target),
                    aux1_probes_json.join(","),
                    json_escape(&lhs_str),
                    json_escape(&rhs_str),
                    json_escape(&res_str),
                    arith_witness.ovf,
                    lhs_b_str.join(","),
                    rhs_b_str.join(","),
                    res_b_str.join(","),
                    carries_vec.join(","),
                    borrows_vec.join(","),
                    diag_vec.join(","),
                    record.before.remaining_fuel,
                    record.after.remaining_fuel,
                    pre_fb_str.join(","),
                    post_fb_str.join(","),
                    zp_str.join(","),
                    bw_str.join(",")
                ));

                step_idx += 1;
            }

            let mut slice_machine = Machine::new(prep.draft.program.clone(), fuel, stack_limit);
            let mut slices_json = Vec::new();
            let mut slice_idx = 0usize;
            let mut step_cursor = 0usize;
            let width = slice_width.max(1);
            while slice_idx < 512 {
                let slice = advance_m(&mut slice_machine, width);
                if slice.records.is_empty() && slice_idx > 0 {
                    break;
                }
                let count = slice.records.len();
                let start_step = step_cursor;
                let end_step = step_cursor + count.saturating_sub(1);
                step_cursor += count;
                let total_work = slice.prefix_work.last().copied().unwrap_or(0);
                let pwork: Vec<String> = slice.prefix_work.iter().map(|w| w.to_string()).collect();
                let is_terminal = slice
                    .boundaries
                    .first()
                    .map(|b| b.kind == SliceBoundaryKind::Terminal)
                    .unwrap_or(false);
                slices_json.push(format!(
                    "{{\"index\":{slice_idx},\"startStep\":{start_step},\"endStep\":{end_step},\"count\":{count},\"prefixWork\":[{}],\"work\":{total_work},\"terminal\":{is_terminal}}}",
                    pwork.join(",")
                ));
                slice_idx += 1;
                if is_terminal || count == 0 {
                    break;
                }
            }

            let final_snap = machine.run();
            let outcome_json = match &final_snap.outcome {
                Outcome::Done(v) => format!(
                    "{{\"kind\":\"Done\",\"value\":{},\"summary\":\"{}\"}}",
                    value_json(v),
                    json_escape(&value_summary(v))
                ),
                Outcome::Rejected(code) => format!(
                    "{{\"kind\":\"Rejected\",\"code\":{code},\"summary\":\"Rejected(error {code})\"}}"
                ),
                Outcome::Exhausted => {
                    "{\"kind\":\"Exhausted\",\"summary\":\"Exhausted (Fuel = 0)\"}".to_string()
                }
                Outcome::Pending => "{\"kind\":\"Pending\",\"summary\":\"Pending\"}".to_string(),
            };

            format!(
                "{{\"ok\":true,\"engine\":\"rustc-moriarty-sherlock-k-a/1\",\"programName\":\"{}\",\"entryType\":\"{:?}\",\"programImage\":{{\"expressions\":[{}],\"functions\":[{}]}},\"steps\":[{}],\"slices\":[{}],\"sliceWidth\":{width},\"outcome\":{outcome_json}}}",
                json_escape(&prep.draft.name),
                prep.types.entry_type,
                expr_rows_json.join(","),
                fn_rows_json.join(","),
                steps_json.join(","),
                slices_json.join(",")
            )
        }
    }
}

fn main() {
    use std::io::Read;
    let args: Vec<String> = std::env::args().collect();
    let mut fuel: u128 = 120;
    let mut stack_limit: usize = 32;
    let mut slice_width: usize = 4;
    let mut file_path: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--fuel" if i + 1 < args.len() => {
                fuel = args[i + 1].parse().unwrap_or(120);
                i += 2;
            }
            "--stack-limit" if i + 1 < args.len() => {
                stack_limit = args[i + 1].parse().unwrap_or(32);
                i += 2;
            }
            "--slice-width" if i + 1 < args.len() => {
                slice_width = args[i + 1].parse().unwrap_or(4);
                i += 2;
            }
            path => {
                file_path = Some(path.to_string());
                i += 1;
            }
        }
    }

    let source = if let Some(path) = file_path {
        std::fs::read_to_string(path).unwrap_or_default()
    } else {
        let mut buf = String::new();
        let _ = std::io::stdin().read_to_string(&mut buf);
        buf
    };

    let json = sherlock::trace_export::compile_and_trace_json(&source, fuel, stack_limit, slice_width);
    println!("{json}");
}
