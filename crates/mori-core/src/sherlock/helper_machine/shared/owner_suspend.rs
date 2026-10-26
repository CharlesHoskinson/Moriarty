//! Profile /2 suspension on the pure route (child of `shared`).
//!
//! Suspension design note §6.2. Two owner transitions are added beside the
//! /1 rules, never in place of one:
//! - `BAwaitEnter`: at `Eval{e}` where `e` is an admitted await row, charge 1,
//!   check the §6.1 resumability table, copy the guard operands, and publish
//!   `control := Await{s, e}`, `outcome := Waiting{s}`. Frames, env, heap
//!   and the other counters are untouched; that image is the checkpoint.
//! - `BResumeAccept`: the one atomic relation enabled on a Waiting image. The
//!   straight-line guard is evaluated over (observation values, captured
//!   operands) without microsteps; only a guard result of exactly `true`
//!   builds a successor: `n_s` fresh DATA rows and env cells, `Eval{E}`, and a
//!   fixed `1 + |G_s|` charge. Anything else builds nothing.
//!
//! Await rows are the outer `let` nodes of the /2 draft lowering
//! (`crate::sherlock_draft::AwaitSiteSrc`); the site table below binds each
//! to its admitted expression row and binder name ids and is part of the /2
//! admission identity (`AwaitTableC::digest`).
//!
//! This is host semantics only (tier T1). No C6 fragment, circuit or native
//! relation for these transitions exists yet.
use super::*;
use crate::sherlock::{BinaryOp as SrcBinaryOp, Expr, Type, UnaryOp as SrcUnaryOp, Value};
use crate::sherlock_draft::AwaitSiteSrc;

/// A scalar of the observation domain (§3.3: UInt or Bool only).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScalarC {
    UInt(u128),
    Bool(bool),
}

impl ScalarC {
    pub fn sort(&self) -> Type {
        match self {
            Self::UInt(_) => Type::UInt,
            Self::Bool(_) => Type::Bool,
        }
    }
}

/// The straight-line guard program of one site. Operands are the site's
/// binders (`Binder(i)`) and captured guard operands (`Operand(i)`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GuardC {
    Lit(ScalarC),
    Binder(usize),
    Operand(usize),
    Add(Box<GuardC>, Box<GuardC>),
    Sub(Box<GuardC>, Box<GuardC>),
    Mul(Box<GuardC>, Box<GuardC>),
    Eq(Box<GuardC>, Box<GuardC>),
    Lt(Box<GuardC>, Box<GuardC>),
    Not(Box<GuardC>),
}

/// Why the guard did not return `true` (not-ready reasons, §5.1).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuardResult {
    True,
    False,
    Error(&'static str),
}

impl GuardC {
    fn eval(&self, binders: &[ScalarC], operands: &[ScalarC]) -> Result<ScalarC, &'static str> {
        let uint = |g: &GuardC| match g.eval(binders, operands)? {
            ScalarC::UInt(n) => Ok(n),
            ScalarC::Bool(_) => Err("sort"),
        };
        Ok(match self {
            Self::Lit(v) => *v,
            Self::Binder(i) => *binders.get(*i).ok_or("binder")?,
            Self::Operand(i) => *operands.get(*i).ok_or("operand")?,
            Self::Add(l, r) => ScalarC::UInt(uint(l)?.checked_add(uint(r)?).ok_or("overflow")?),
            Self::Sub(l, r) => ScalarC::UInt(uint(l)?.checked_sub(uint(r)?).ok_or("underflow")?),
            Self::Mul(l, r) => ScalarC::UInt(uint(l)?.checked_mul(uint(r)?).ok_or("overflow")?),
            Self::Lt(l, r) => ScalarC::Bool(uint(l)? < uint(r)?),
            Self::Eq(l, r) => {
                // Every node is evaluated (no short circuit), as in the circuit.
                let a = l.eval(binders, operands)?;
                let b = r.eval(binders, operands)?;
                if a.sort() != b.sort() {
                    return Err("sort");
                }
                ScalarC::Bool(a == b)
            }
            Self::Not(x) => match x.eval(binders, operands)? {
                ScalarC::Bool(b) => ScalarC::Bool(!b),
                ScalarC::UInt(_) => return Err("sort"),
            },
        })
    }
    pub fn run(&self, binders: &[ScalarC], operands: &[ScalarC]) -> GuardResult {
        match self.eval(binders, operands) {
            Ok(ScalarC::Bool(true)) => GuardResult::True,
            Ok(ScalarC::Bool(false)) => GuardResult::False,
            Ok(ScalarC::UInt(_)) => GuardResult::Error("sort"),
            Err(code) => GuardResult::Error(code),
        }
    }
    fn canonical(&self, out: &mut String) {
        use std::fmt::Write;
        let bin = |out: &mut String, tag: &str, l: &GuardC, r: &GuardC| {
            out.push('(');
            out.push_str(tag);
            out.push(' ');
            l.canonical(out);
            out.push(' ');
            r.canonical(out);
            out.push(')');
        };
        match self {
            Self::Lit(ScalarC::UInt(n)) => {
                let _ = write!(out, "u{n}");
            }
            Self::Lit(ScalarC::Bool(b)) => {
                let _ = write!(out, "b{}", u8::from(*b));
            }
            Self::Binder(i) => {
                let _ = write!(out, "x{i}");
            }
            Self::Operand(i) => {
                let _ = write!(out, "o{i}");
            }
            Self::Add(l, r) => bin(out, "+", l, r),
            Self::Sub(l, r) => bin(out, "-", l, r),
            Self::Mul(l, r) => bin(out, "*", l, r),
            Self::Eq(l, r) => bin(out, "==", l, r),
            Self::Lt(l, r) => bin(out, "<", l, r),
            Self::Not(x) => {
                out.push_str("(not ");
                x.canonical(out);
                out.push(')');
            }
        }
    }
}

/// One admitted await site of a /2 pure program.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AwaitSiteC {
    pub site: u16,
    pub label: String,
    /// The outer `let` row: `ControlC::Await.expr` and the Eval it replaces.
    pub expr: usize,
    /// The body `E` row that `BResumeAccept` continues with.
    pub body: usize,
    /// Binder name id (env cell binder), name and sort, in declared order.
    pub binders: Vec<(usize, String, Type)>,
    /// Guard operand name id, name and sort, in admitted operand order.
    pub operands: Vec<(usize, String, Type)>,
    pub guard: GuardC,
    /// `|G_s|`; the resume relation charges `1 + |G_s|`.
    pub guard_nodes: u128,
}

impl AwaitSiteC {
    pub fn accept_charge(&self) -> u128 {
        1 + self.guard_nodes
    }
}

/// The site table of one /2 pure admission.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct AwaitTableC {
    pub sites: Vec<AwaitSiteC>,
}

impl AwaitTableC {
    pub fn site(&self, site: u16) -> Option<&AwaitSiteC> {
        self.sites.iter().find(|s| s.site == site)
    }
    pub fn at_expr(&self, expr: usize) -> Option<&AwaitSiteC> {
        self.sites.iter().find(|s| s.expr == expr)
    }
    /// Canonical text of the table; its SHA-256 joins the admission identity.
    pub fn canonical(&self) -> String {
        use std::fmt::Write;
        let sort = |t: &Type| match t {
            Type::UInt => "UInt",
            Type::Bool => "Bool",
            Type::List => "List",
            Type::Tree => "Tree",
        };
        let mut out = String::from("sherlock-await-table/2;");
        for s in &self.sites {
            let _ = write!(
                out,
                "site {} {:?} expr {} body {} nodes {};",
                s.site, s.label, s.expr, s.body, s.guard_nodes
            );
            for (id, name, t) in &s.binders {
                let _ = write!(out, "binder {id} {name:?} {};", sort(t));
            }
            for (id, name, t) in &s.operands {
                let _ = write!(out, "operand {id} {name:?} {};", sort(t));
            }
            out.push_str("guard ");
            s.guard.canonical(&mut out);
            out.push(';');
        }
        out
    }
    pub fn digest(&self) -> [u8; 32] {
        use sha2::Digest;
        sha2::Sha256::digest(self.canonical().as_bytes()).into()
    }
}

fn guard_of(
    e: &Expr,
    binders: &[(String, Type)],
    operands: &[(String, Type)],
) -> Result<GuardC, String> {
    let two = |l: &Expr, r: &Expr| -> Result<(Box<GuardC>, Box<GuardC>), String> {
        Ok((
            Box::new(guard_of(l, binders, operands)?),
            Box::new(guard_of(r, binders, operands)?),
        ))
    };
    Ok(match e {
        Expr::Literal(Value::UInt(n)) => GuardC::Lit(ScalarC::UInt(*n)),
        Expr::Literal(Value::Bool(b)) => GuardC::Lit(ScalarC::Bool(*b)),
        Expr::Var(name) => {
            if let Some(i) = binders.iter().position(|(n, _)| n == name) {
                GuardC::Binder(i)
            } else if let Some(i) = operands.iter().position(|(n, _)| n == name) {
                GuardC::Operand(i)
            } else {
                return Err(format!("guard name {name:?} is neither binder nor operand"));
            }
        }
        Expr::Binary(op, l, r) => {
            let (l, r) = two(l, r)?;
            match op {
                SrcBinaryOp::Add => GuardC::Add(l, r),
                SrcBinaryOp::Sub => GuardC::Sub(l, r),
                SrcBinaryOp::Mul => GuardC::Mul(l, r),
                SrcBinaryOp::Eq => GuardC::Eq(l, r),
                SrcBinaryOp::Lt => GuardC::Lt(l, r),
                _ => return Err("constructor in await guard".into()),
            }
        }
        Expr::Unary(SrcUnaryOp::Not, x) => GuardC::Not(Box::new(guard_of(x, binders, operands)?)),
        _ => return Err("await guard is not straight-line".into()),
    })
}

/// Bind the parsed sites to the admitted program rows. Each site's
/// `expression_index` must name a `let` row binding its first binder, nested
/// `let` rows must bind the remaining binders in order, and every binder and
/// operand name must be an admitted name.
pub fn bind_await_table(a: &Admitted, sites: &[AwaitSiteSrc]) -> Result<AwaitTableC, String> {
    let view = a
        .body
        .view
        .as_ref()
        .ok_or("a /2 pure admission carries a program view")?;
    let name_id = |name: &str| -> Result<usize, String> {
        view.names
            .iter()
            .position(|n| n == name)
            .ok_or_else(|| format!("await name {name:?} is not an admitted name"))
    };
    let mut table = AwaitTableC::default();
    for s in sites {
        let mut row = s.expression_index;
        let mut binders = Vec::with_capacity(s.binders.len());
        for (name, ty) in &s.binders {
            match view.expressions.get(row) {
                Some(ExpressionRowC::Let { binder, body, .. })
                    if view.names.get(*binder).map(String::as_str) == Some(name.as_str()) =>
                {
                    binders.push((*binder, name.clone(), *ty));
                    row = *body;
                }
                _ => {
                    return Err(format!(
                        "await site {} does not map to its admitted let row",
                        s.site
                    ));
                }
            }
        }
        let operands = s
            .operands
            .iter()
            .map(|(name, ty)| Ok((name_id(name)?, name.clone(), *ty)))
            .collect::<Result<Vec<_>, String>>()?;
        let guard = guard_of(&s.guard, &s.binders, &s.operands)?;
        table.sites.push(AwaitSiteC {
            site: s.site,
            label: s.label.clone(),
            expr: s.expression_index,
            body: row,
            binders,
            operands,
            guard,
            guard_nodes: s.guard_nodes as u128,
        });
    }
    Ok(table)
}

fn scalar_of(a: &Admitted, img: &MachineImage, r: RefC) -> Option<ScalarC> {
    let view = a.body.view.as_ref()?;
    match r {
        RefC::Const(i) => match view.constants.get(i)? {
            RawConstRow::U128 { n, .. } => Some(ScalarC::UInt(*n)),
            RawConstRow::Bool { b, .. } => Some(ScalarC::Bool(*b)),
            _ => None,
        },
        RefC::Data(i) => match img.data.get(i)? {
            RawDataRow::U128 { n } => Some(ScalarC::UInt(*n)),
            RawDataRow::Bool { b } => Some(ScalarC::Bool(*b)),
            _ => None,
        },
    }
}

fn env_lookup(img: &MachineImage, binder: usize) -> Option<RefC> {
    let mut at = img.env_root;
    let mut steps = 0usize;
    while let Some(i) = at {
        let cell = img.env.get(i)?;
        if cell.binder == binder {
            return Some(cell.value);
        }
        at = cell.previous;
        steps += 1;
        if steps > img.env.len() {
            return None;
        }
    }
    None
}

/// The values of the site's guard operands in the image's current env.
pub fn guard_operands(
    a: &Admitted,
    img: &MachineImage,
    site: &AwaitSiteC,
) -> Result<Vec<ScalarC>, String> {
    site.operands
        .iter()
        .map(|(id, name, ty)| {
            let v = env_lookup(img, *id)
                .and_then(|r| scalar_of(a, img, r))
                .ok_or_else(|| format!("guard operand {name:?} is not a scalar in scope"))?;
            if v.sort() != *ty {
                return Err(format!("guard operand {name:?} has the wrong sort"));
            }
            Ok(v)
        })
        .collect()
}

/// What `BAwaitEnter` published.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AwaitEnter {
    /// Waiting{site}; the captured guard operands (§3.4 capture section).
    Waiting { site: u16, operands: Vec<ScalarC> },
    /// The §6.1 entry check failed after the entry charge.
    Exhausted { reason: ExhaustReason },
}

/// `BAwaitEnter` when the image is at `Eval` of an await row; `None` leaves
/// the step to the ordinary owner. Requires a Running image with no helper or
/// result; the caller has already ruled out P2 (remaining work 0 publishes
/// the ordinary uncharged Fuel exhaustion instead).
pub fn await_enter(
    a: &Admitted,
    img: &mut MachineImage,
    table: &AwaitTableC,
) -> Result<Option<AwaitEnter>, String> {
    PreImage::bind(a, img).map_err(|v| format!("{v:?}"))?;
    if img.outcome != OutcomeC::Running || img.helper.is_some() || img.result.is_some() {
        return Ok(None);
    }
    let ControlC::Eval { expr } = img.control else {
        return Ok(None);
    };
    let Some(site) = table.at_expr(expr) else {
        return Ok(None);
    };
    if img.remaining_work == 0 {
        return Ok(None);
    }
    let operands = guard_operands(a, img, site)?;
    // Entry charge of 1, a reference event (SI-03/SI-04 hold).
    img.remaining_work -= 1;
    img.consumed_work = img.consumed_work.checked_add(1).ok_or("consumed + 1")?;
    img.reference_cursor = img.reference_cursor.checked_add(1).ok_or("cursor + 1")?;
    let n = site.binders.len();
    let b = &a.body.bounds;
    let reason = if img.remaining_work < site.accept_charge() {
        Some(ExhaustReason::Fuel)
    } else if b.data_nodes.saturating_sub(img.active.data) < n {
        Some(ExhaustReason::DataNodes)
    } else if b.environment_cells.saturating_sub(img.active.env) < n {
        Some(ExhaustReason::EnvironmentCells)
    } else {
        None
    };
    if let Some(reason) = reason {
        img.outcome = OutcomeC::Exhausted { reason };
        return Ok(Some(AwaitEnter::Exhausted { reason }));
    }
    img.control = ControlC::Await {
        site: site.site,
        expr,
    };
    img.outcome = OutcomeC::Waiting { site: site.site };
    Ok(Some(AwaitEnter::Waiting {
        site: site.site,
        operands,
    }))
}

/// SI-29 on a Waiting image at `site`: fuel covers the accept charge
/// `1 + |G|`, and free DATA rows and env cells cover the `n` binders.
pub fn check_resumable(a: &Admitted, img: &MachineImage, site: &AwaitSiteC) -> Result<(), String> {
    let n = site.binders.len();
    let b = &a.body.bounds;
    if img.remaining_work < site.accept_charge()
        || b.data_nodes.saturating_sub(img.active.data) < n
        || b.environment_cells.saturating_sub(img.active.env) < n
    {
        return Err("Waiting image violates the resumability invariant SI-29".into());
    }
    Ok(())
}

/// The atomic `BResumeAccept` relation. `values` must already match the
/// site layout (R5) and the guard must already have returned `true`; this
/// checks both again and refuses (without touching `img`) otherwise.
pub fn resume_accept(
    a: &Admitted,
    img: &mut MachineImage,
    table: &AwaitTableC,
    operands: &[ScalarC],
    values: &[ScalarC],
) -> Result<(), String> {
    PreImage::bind(a, img).map_err(|v| format!("{v:?}"))?;
    let (OutcomeC::Waiting { site: s }, ControlC::Await { site: c, expr }) =
        (img.outcome, img.control)
    else {
        return Err("resume of an image that is not Waiting".into());
    };
    let site = table
        .site(s)
        .filter(|site| site.site == c && site.expr == expr)
        .ok_or("Waiting site is not an admitted await site")?;
    if values.len() != site.binders.len()
        || values
            .iter()
            .zip(&site.binders)
            .any(|(v, (_, _, t))| v.sort() != *t)
    {
        return Err("observation layout differs from the site".into());
    }
    if site.guard.run(values, operands) != GuardResult::True {
        return Err("the guard did not accept the observation".into());
    }
    // SI-29 makes this unreachable from a machine-produced Waiting image.
    check_resumable(a, img, site)?;
    let n = values.len();
    let charge = site.accept_charge();
    let mut env_root = img.env_root;
    for (value, (binder, _, _)) in values.iter().zip(&site.binders) {
        let row = img.data.len();
        img.data.push(match value {
            ScalarC::UInt(n) => RawDataRow::U128 { n: *n },
            ScalarC::Bool(b) => RawDataRow::Bool { b: *b },
        });
        let cell = img.env.len();
        img.env.push(EnvCellC {
            binder: *binder,
            value: RefC::Data(row),
            previous: env_root,
        });
        env_root = Some(cell);
    }
    img.env_root = env_root;
    img.alloc.data_nodes += n;
    img.alloc.environment_cells += n;
    img.active.data = img.data.len();
    img.active.env = img.env.len();
    img.remaining_work -= charge;
    img.consumed_work = img.consumed_work.checked_add(charge).ok_or("consumed")?;
    img.reference_cursor = img.reference_cursor.checked_add(charge).ok_or("cursor")?;
    img.control = ControlC::Eval { expr: site.body };
    img.control_origin = site.body;
    img.outcome = OutcomeC::Running;
    Ok(())
}

/// A plain JSON view of a value reference (scalars and UInt lists).
pub fn value_json(a: &Admitted, img: &MachineImage, r: RefC) -> serde_json::Value {
    use serde_json::json;
    if let Some(v) = scalar_of(a, img, r) {
        return match v {
            ScalarC::UInt(n) => json!({"type": "UInt", "value": n.to_string()}),
            ScalarC::Bool(b) => json!({"type": "Bool", "value": b}),
        };
    }
    json!({"type": "Ref", "value": format!("{r:?}")})
}

fn function_called(a: &Admitted, origin: usize) -> Option<String> {
    let view = a.body.view.as_ref()?;
    match view.expressions.get(origin)? {
        ExpressionRowC::Call { function_name, .. } => view.names.get(*function_name).cloned(),
        _ => None,
    }
}

/// Bottom-to-top frame summary: each frame's kind; a return frame names the
/// function it returns from (the callee of its origin call row); a pending
/// right-operand frame carries its operator and decoded left value.
pub fn frames_summary(a: &Admitted, img: &MachineImage) -> Vec<serde_json::Value> {
    use serde_json::json;
    img.frames
        .iter()
        .map(|f| match f {
            FrameRowC::ReturnF { origin, .. } => json!({
                "kind": "ReturnF",
                "function": function_called(a, *origin),
                "origin": origin.to_string(),
            }),
            FrameRowC::RightF { op, left, origin } => json!({
                "kind": "RightF",
                "op": format!("{op:?}"),
                "left": value_json(a, img, *left),
                "origin": origin.to_string(),
            }),
            FrameRowC::LeftF { op, origin, .. } => {
                json!({"kind": "LeftF", "op": format!("{op:?}"), "origin": origin.to_string()})
            }
            FrameRowC::LetF { origin, .. } => json!({"kind": "LetF", "origin": origin.to_string()}),
            FrameRowC::UnaryF { op, origin } => {
                json!({"kind": "UnaryF", "op": format!("{op:?}"), "origin": origin.to_string()})
            }
            FrameRowC::IfF { origin, .. } => json!({"kind": "IfF", "origin": origin.to_string()}),
            FrameRowC::MatchF { origin, .. } => {
                json!({"kind": "MatchF", "origin": origin.to_string()})
            }
            FrameRowC::ArgsF {
                function_name,
                origin,
                ..
            } => json!({
                "kind": "ArgsF",
                "function": a.body.view.as_ref().and_then(|v| v.names.get(*function_name).cloned()),
                "origin": origin.to_string(),
            }),
            FrameRowC::ScopeF { origin, .. } => {
                json!({"kind": "ScopeF", "origin": origin.to_string()})
            }
            FrameRowC::ConstructorFieldsF { origin, .. } => {
                json!({"kind": "ConstructorFieldsF", "origin": origin.to_string()})
            }
        })
        .collect()
}

/// Counters a /2 report reads from the image.
pub fn consumed_work(img: &MachineImage) -> u128 {
    img.consumed_work
}
pub fn outcome(img: &MachineImage) -> OutcomeC {
    img.outcome
}
pub fn control(img: &MachineImage) -> ControlC {
    img.control
}
