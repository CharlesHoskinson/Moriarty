//! Mechanical dispatch-predicate partition (task 3.2, SAS-013) and a
//! concrete evaluator for the dispatch IR. Test-only; emits no relation.
//!
//! Derivation. [`emit_selector`] runs the dispatch owner's own `facts` and
//! `path` code (this module's parent, unchanged) once over symbolic inputs
//! and keeps every supported rule's selected bit. The *roots* are the value
//! facts the guards read (`Facts` fields: control lanes, depth, stack limit,
//! top frame, Eval/control-origin PROG rows, PROG function count, helper
//! record, result record, live task count, top task, the Validate marker and
//! the Equal `shallow_differ` value reads) and the raw lanes the boolean facts
//! are derived from (outcome, remaining work, live task count). The backward
//! slice of the selected bits stops at the roots, so every boolean fact
//! (`running`, `site`, `site_defined`, `no_helper`, `no_result`, `depth_zero`,
//! `fuel_zero`, `live_pos`, `any_plan`, every guard) is evaluated through the
//! dispatch code's own instructions. The constants each root is compared with
//! are read from the slice, never written here.
//!
//! The partition is the leaf set of a decision tree over the roots: a node
//! evaluates every selected bit in three-valued logic and splits the first
//! root (or root-versus-root order atom) that leaves a bit undetermined. Each
//! root ranges over its compared constants plus `other` (a value equal to
//! none of them); a root-versus-root comparison (`depth < stack_limit`, the
//! helper cursors against their lengths, the Find cursor against the PROG
//! function count) is an order atom `lt | eq | gt` whenever one side is
//! `other`. Over-approximations (each makes the domain larger, never
//! smaller): roots are independent (the top frame is free at depth 0, the top
//! task is free at live 0, the PROG rows are free of `k1`/origin), and order
//! atoms are not closed under transitivity. Exclusivity checked on this
//! domain therefore holds on every pre-state whose facts the circuit
//! computes, provided the premises in [`PREMISES`] hold.
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{L_OUTCOME, L_REMAINING_WORK};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use serde_json::{Value, json};

use super::{
    C6DispatchEmitter, C6DispatchError, C6DispatchInputs, Em, entry_kind, helper_kind_of,
    is_publication, legacy_site_of, supported_rules,
};

pub(crate) const PARTITION_FORMAT: &str = "moriarty-sherlock-dispatch-partition/1";

/// Premises the partition does not split on; each is an assertion of the
/// dispatch circuit (refusing every rule when it fails) or a retained premise
/// of `c6_dispatch_step.rs` discharged elsewhere.
pub(crate) const PREMISES: &[&str] = &[
    "every pre IMAGE and HELPER lane is < 2^128 (ConstrainBits in facts)",
    "enable is boolean (ConstrainToBoolean)",
    "Eval control: the PROG row at k1 exists (gated Assert first_found)",
    "Cases site: the control-origin PROG row exists (gated Assert second_found)",
    "Validate pending top task: the referenced value is found (gated Assert, checked value read)",
    "Equal pending top task: both referenced values are found (gated Asserts, checked value reads)",
    "PROG canonicality: enforced by the dispatch PROG parser walk",
    "IMAGE/HELPER canonicality (frame tags, padding, helper/task tags): retained premise, discharged by the paired step fragment over the same lanes",
    "DATA canonicality: only where a value read is gated on",
    "top frame = zero row at depth 0 and top task = zero row at live 0 (mux): not modelled, over-approximated",
];

pub(crate) const OVER_APPROXIMATIONS: &[&str] = &[
    "roots are independent: PROG rows are free of k1 and the control origin, the top frame is free at depth 0, the top task is free at live 0",
    "order atoms (lt/eq/gt) are not closed under transitivity or under the constants of either side",
    "`other` stands for every value outside a root's compared constants, including values no canonical state reaches",
];

/// Facts once, then every supported rule's selected bit, over `inputs`.
pub(crate) struct Selector {
    pub ins: Vec<Instruction>,
    /// (root name, identifier) for every value fact the guards read.
    pub roots: Vec<(String, Identifier)>,
    pub paths: Vec<(RuleIdC, Operand)>,
}

pub(crate) fn emit_selector(
    inputs: &C6DispatchInputs,
    scope: &str,
) -> Result<Selector, C6DispatchError> {
    C6DispatchEmitter::check_lengths(inputs)?;
    let mut d = C6DispatchEmitter::new(scope)?;
    let mut em = Em {
        scope: d.scope.clone(),
        next: d.next,
        ins: Vec::new(),
    };
    em.cbool(inputs.enable.clone());
    let f = d.facts(&mut em, inputs)?;
    let mut paths = Vec::new();
    for r in supported_rules() {
        let p = C6DispatchEmitter::path(&mut em, &f, r)?;
        paths.push((r, p));
    }
    let mut named: Vec<(String, Operand)> = vec![
        ("outcome".into(), inputs.pre_image[L_OUTCOME].clone()),
        (
            "remaining_work".into(),
            inputs.pre_image[L_REMAINING_WORK].clone(),
        ),
        ("live".into(), inputs.pre_helper[3].clone()),
        ("ct".into(), f.ct.clone()),
    ];
    for (i, x) in f.k.iter().enumerate().skip(1) {
        named.push((format!("k{i}"), x.clone()));
    }
    named.push(("depth".into(), f.depth.clone()));
    named.push(("stack_limit".into(), f.stack_limit.clone()));
    for (i, x) in f.top.iter().enumerate() {
        named.push((format!("top{i}"), x.clone()));
    }
    for (i, x) in f.eval_row.iter().enumerate() {
        named.push((format!("eval{i}"), x.clone()));
    }
    for (i, x) in f.origin_row.iter().enumerate() {
        named.push((format!("origin{i}"), x.clone()));
    }
    named.push(("functions_count".into(), f.functions_count.clone()));
    named.push(("htag".into(), f.htag.clone()));
    for (i, x) in f.h.iter().enumerate().skip(1) {
        named.push((format!("h{i}"), x.clone()));
    }
    named.push(("rt".into(), f.rt.clone()));
    named.push(("r1".into(), f.r1.clone()));
    for (i, x) in f.task.iter().enumerate() {
        named.push((format!("task{i}"), x.clone()));
    }
    named.push(("validate_marker".into(), f.validate_marker.clone()));
    named.push(("equal_differ".into(), f.equal_differ.clone()));
    let mut seen = HashSet::new();
    let roots = named
        .into_iter()
        .filter_map(|(n, op)| match op {
            Operand::Variable(id) if seen.insert(id.clone()) => Some((n, id)),
            _ => None,
        })
        .collect();
    Ok(Selector {
        ins: em.ins,
        roots,
        paths,
    })
}

/// Roots whose value is a boolean produced by a checked read (no `other`).
const BOOLEAN_ROOTS: [&str; 2] = ["validate_marker", "equal_differ"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Dom {
    C(u128),
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Ord3 {
    Lt,
    Eq,
    Gt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Atom {
    Var(usize),
    Ord(usize, usize),
}

#[derive(Clone, Copy, Debug)]
enum Ref {
    Imm(u128),
    Root(usize),
    Tmp(usize),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    TestEq(Ref, Ref),
    LessThan(Ref, Ref),
    Mul(Ref, Ref),
    Add(Ref, Ref),
    Not(Ref),
    CondSelect(Ref, Ref, Ref),
}

#[derive(Clone, Copy, Debug)]
enum Av {
    K(u128),
    O(usize),
    U(Atom),
}

fn imm_u128(f: &Fr) -> u128 {
    u128::try_from(*f).expect("dispatch immediate fits u128")
}

/// One partition class: the atoms its decision-tree path fixed and the
/// supported rules whose selected bit is 1 there.
#[derive(Clone, Debug)]
pub(crate) struct Class {
    pub vars: Vec<(usize, Dom)>,
    pub ords: Vec<((usize, usize), Ord3)>,
    pub selected: Vec<RuleIdC>,
}

pub(crate) struct Partition {
    pub root_names: Vec<String>,
    pub consts: Vec<BTreeSet<u128>>,
    pub boolean: Vec<bool>,
    pub rules: Vec<RuleIdC>,
    pub classes: Vec<Class>,
    pub slice_len: usize,
}

struct Builder {
    ops: Vec<Op>,
    paths: Vec<(RuleIdC, Ref)>,
    consts: Vec<BTreeSet<u128>>,
    boolean: Vec<bool>,
}

#[derive(Clone, Default)]
struct Asg {
    vars: Vec<Option<Dom>>,
    ords: BTreeMap<(usize, usize), Ord3>,
}

impl Builder {
    fn new(sel: &Selector) -> Self {
        let root_ix: HashMap<&Identifier, usize> = sel
            .roots
            .iter()
            .enumerate()
            .map(|(i, (_, id))| (id, i))
            .collect();
        let mut def: HashMap<&Identifier, usize> = HashMap::new();
        for (i, ins) in sel.ins.iter().enumerate() {
            if let Some(o) = output_of(ins) {
                def.insert(o, i);
            }
        }
        // Backward slice from the selected bits, stopping at roots.
        let mut need = BTreeSet::new();
        let mut stack: Vec<&Identifier> = sel
            .paths
            .iter()
            .filter_map(|(_, p)| match p {
                Operand::Variable(id) => Some(id),
                _ => None,
            })
            .collect();
        while let Some(id) = stack.pop() {
            if root_ix.contains_key(id) {
                continue;
            }
            let Some(&i) = def.get(id) else {
                panic!("slice reaches undefined non-root variable {}", id.0);
            };
            if need.insert(i) {
                for o in operands_of(&sel.ins[i]) {
                    if let Operand::Variable(v) = o {
                        stack.push(v);
                    }
                }
            }
        }
        let mut tmp: HashMap<&Identifier, usize> = HashMap::new();
        let mut ops = Vec::new();
        let r = |o: &Operand, tmp: &HashMap<&Identifier, usize>| -> Ref {
            match o {
                Operand::Immediate(f) => Ref::Imm(imm_u128(f)),
                Operand::Variable(id) => {
                    if let Some(&k) = root_ix.get(id) {
                        Ref::Root(k)
                    } else {
                        Ref::Tmp(*tmp.get(id).expect("slice order"))
                    }
                }
            }
        };
        for &i in &need {
            let ins = &sel.ins[i];
            let op = match ins {
                Instruction::TestEq { a, b, .. } => Op::TestEq(r(a, &tmp), r(b, &tmp)),
                Instruction::LessThan { a, b, bits, .. } => {
                    assert_eq!(*bits, 128, "dispatch LessThan width");
                    Op::LessThan(r(a, &tmp), r(b, &tmp))
                }
                Instruction::Mul { a, b, .. } => Op::Mul(r(a, &tmp), r(b, &tmp)),
                Instruction::Add { a, b, .. } => Op::Add(r(a, &tmp), r(b, &tmp)),
                Instruction::Not { a, .. } => Op::Not(r(a, &tmp)),
                Instruction::CondSelect { bit, a, b, .. } => {
                    Op::CondSelect(r(bit, &tmp), r(a, &tmp), r(b, &tmp))
                }
                other => panic!("unmodelled instruction in the selector slice: {other:?}"),
            };
            tmp.insert(output_of(ins).expect("slice op has an output"), ops.len());
            ops.push(op);
        }
        let paths = sel
            .paths
            .iter()
            .map(|(rule, p)| (*rule, r(p, &tmp)))
            .collect();
        let n = sel.roots.len();
        let mut consts = vec![BTreeSet::new(); n];
        for op in &ops {
            if let Op::TestEq(a, b) = op {
                match (a, b) {
                    (Ref::Root(x), Ref::Imm(c)) | (Ref::Imm(c), Ref::Root(x)) => {
                        consts[*x].insert(*c);
                    }
                    _ => {}
                }
            }
        }
        let boolean: Vec<bool> = sel
            .roots
            .iter()
            .map(|(name, _)| BOOLEAN_ROOTS.contains(&name.as_str()))
            .collect();
        for (i, b) in boolean.iter().enumerate() {
            if *b {
                consts[i] = [0, 1].into_iter().collect();
            }
        }
        Builder {
            ops,
            paths,
            consts,
            boolean,
        }
    }

    fn get(&self, vals: &[Av], asg: &Asg, x: Ref) -> Av {
        match x {
            Ref::Imm(c) => Av::K(c),
            Ref::Tmp(t) => vals[t],
            Ref::Root(r) => match asg.vars[r] {
                Some(Dom::C(v)) => Av::K(v),
                Some(Dom::Other) => Av::O(r),
                None => Av::U(Atom::Var(r)),
            },
        }
    }

    /// Root-versus-root comparison: constants decide, else the order atom.
    /// Two assigned constants decide; otherwise the order atom does (an
    /// unassigned side is not split over constants it is never compared
    /// with here; the atom ranges over lt/eq/gt).
    fn cmp_roots(&self, asg: &Asg, x: usize, y: usize) -> Result<Ord3, Atom> {
        if let (Some(Dom::C(a)), Some(Dom::C(b))) = (asg.vars[x], asg.vars[y]) {
            return Ok(match a.cmp(&b) {
                std::cmp::Ordering::Less => Ord3::Lt,
                std::cmp::Ordering::Equal => Ord3::Eq,
                std::cmp::Ordering::Greater => Ord3::Gt,
            });
        }
        let (lo, hi, flip) = if x < y { (x, y, false) } else { (y, x, true) };
        match asg.ords.get(&(lo, hi)) {
            None => Err(Atom::Ord(lo, hi)),
            Some(o) => Ok(match (o, flip) {
                (Ord3::Lt, true) => Ord3::Gt,
                (Ord3::Gt, true) => Ord3::Lt,
                (o, _) => *o,
            }),
        }
    }

    fn eval(&self, asg: &Asg) -> Vec<Av> {
        let mut vals: Vec<Av> = Vec::with_capacity(self.ops.len());
        for op in &self.ops {
            let v = match *op {
                Op::TestEq(a, b) | Op::LessThan(a, b) => {
                    let lt = matches!(op, Op::LessThan(..));
                    if let (Ref::Root(x), Ref::Root(y)) = (a, b) {
                        match self.cmp_roots(asg, x, y) {
                            Err(atom) => Av::U(atom),
                            Ok(o) => {
                                Av::K(u128::from(if lt { o == Ord3::Lt } else { o == Ord3::Eq }))
                            }
                        }
                    } else {
                        match (self.get(&vals, asg, a), self.get(&vals, asg, b)) {
                            (Av::U(t), _) | (_, Av::U(t)) => Av::U(t),
                            (Av::K(x), Av::K(y)) => {
                                Av::K(u128::from(if lt { x < y } else { x == y }))
                            }
                            (Av::O(r), Av::K(c)) | (Av::K(c), Av::O(r)) if !lt => {
                                assert!(
                                    self.consts[r].contains(&c),
                                    "comparison of `other` with an uncollected constant"
                                );
                                Av::K(0)
                            }
                            other => panic!("unmodelled comparison {other:?}"),
                        }
                    }
                }
                Op::Mul(a, b) => match (self.get(&vals, asg, a), self.get(&vals, asg, b)) {
                    (Av::K(0), _) | (_, Av::K(0)) => Av::K(0),
                    (Av::K(x), Av::K(y)) => Av::K(x.checked_mul(y).expect("small product")),
                    (Av::U(t), _) | (_, Av::U(t)) => Av::U(t),
                    other => panic!("unmodelled product {other:?}"),
                },
                Op::Add(a, b) => match (self.get(&vals, asg, a), self.get(&vals, asg, b)) {
                    (Av::K(x), Av::K(y)) => Av::K(x.checked_add(y).expect("small sum")),
                    (Av::U(t), _) | (_, Av::U(t)) => Av::U(t),
                    other => panic!("unmodelled sum {other:?}"),
                },
                Op::Not(a) => match self.get(&vals, asg, a) {
                    Av::K(0) => Av::K(1),
                    Av::K(1) => Av::K(0),
                    Av::U(t) => Av::U(t),
                    other => panic!("Not of a non-boolean {other:?}"),
                },
                Op::CondSelect(bit, a, b) => match self.get(&vals, asg, bit) {
                    Av::K(1) => self.get(&vals, asg, a),
                    Av::K(0) => self.get(&vals, asg, b),
                    Av::U(t) => Av::U(t),
                    other => panic!("CondSelect on a non-boolean {other:?}"),
                },
            };
            vals.push(v);
        }
        vals
    }

    fn domain(&self, atom: Atom) -> Vec<Result<Dom, Ord3>> {
        match atom {
            Atom::Var(r) => {
                let mut d: Vec<Result<Dom, Ord3>> =
                    self.consts[r].iter().map(|c| Ok(Dom::C(*c))).collect();
                if !self.boolean[r] {
                    d.push(Ok(Dom::Other));
                }
                d
            }
            Atom::Ord(..) => vec![Err(Ord3::Lt), Err(Ord3::Eq), Err(Ord3::Gt)],
        }
    }

    fn split(&self, asg: &mut Asg, out: &mut Vec<Class>) {
        let vals = self.eval(asg);
        let mut selected = Vec::new();
        let mut blocker = None;
        for (rule, p) in &self.paths {
            match self.get(&vals, asg, *p) {
                Av::K(1) => selected.push(*rule),
                Av::K(0) => {}
                Av::U(t) => {
                    blocker = Some(t);
                    break;
                }
                other => panic!("{rule:?}: non-boolean selected bit {other:?}"),
            }
        }
        let Some(atom) = blocker else {
            out.push(Class {
                vars: asg
                    .vars
                    .iter()
                    .enumerate()
                    .filter_map(|(i, d)| d.map(|d| (i, d)))
                    .collect(),
                ords: asg.ords.iter().map(|(k, v)| (*k, *v)).collect(),
                selected,
            });
            return;
        };
        for v in self.domain(atom) {
            match (atom, v) {
                (Atom::Var(r), Ok(d)) => asg.vars[r] = Some(d),
                (Atom::Ord(a, b), Err(o)) => {
                    asg.ords.insert((a, b), o);
                }
                _ => unreachable!(),
            }
            self.split(asg, out);
            match atom {
                Atom::Var(r) => asg.vars[r] = None,
                Atom::Ord(a, b) => {
                    asg.ords.remove(&(a, b));
                }
            }
        }
    }
}

fn output_of(ins: &Instruction) -> Option<&Identifier> {
    Some(match ins {
        Instruction::TestEq { output, .. }
        | Instruction::LessThan { output, .. }
        | Instruction::Mul { output, .. }
        | Instruction::Add { output, .. }
        | Instruction::Not { output, .. }
        | Instruction::CondSelect { output, .. } => output,
        _ => return None,
    })
}

fn operands_of(ins: &Instruction) -> Vec<&Operand> {
    match ins {
        Instruction::TestEq { a, b, .. }
        | Instruction::LessThan { a, b, .. }
        | Instruction::Mul { a, b, .. }
        | Instruction::Add { a, b, .. } => vec![a, b],
        Instruction::Not { a, .. } => vec![a],
        Instruction::CondSelect { bit, a, b, .. } => vec![bit, a, b],
        _ => Vec::new(),
    }
}

pub(crate) fn derive_partition(sel: &Selector) -> Partition {
    let b = Builder::new(sel);
    let mut asg = Asg {
        vars: vec![None; sel.roots.len()],
        ords: BTreeMap::new(),
    };
    let mut classes = Vec::new();
    b.split(&mut asg, &mut classes);
    Partition {
        root_names: sel.roots.iter().map(|(n, _)| n.clone()).collect(),
        consts: b.consts,
        boolean: b.boolean,
        rules: sel.paths.iter().map(|(r, _)| *r).collect(),
        classes,
        slice_len: b.ops.len(),
    }
}

fn dom_str(d: Dom) -> String {
    match d {
        Dom::C(v) => v.to_string(),
        Dom::Other => "other".into(),
    }
}

fn ord_str(o: Ord3) -> &'static str {
    match o {
        Ord3::Lt => "lt",
        Ord3::Eq => "eq",
        Ord3::Gt => "gt",
    }
}

impl Partition {
    /// Classes in which more than one supported rule is selected.
    pub fn overlaps(&self) -> Vec<usize> {
        (0..self.classes.len())
            .filter(|i| self.classes[*i].selected.len() > 1)
            .collect()
    }

    /// The abstract value of a concrete root assignment.
    pub fn abstract_of(&self, r: usize, v: u128) -> Dom {
        if self.consts[r].contains(&v) {
            Dom::C(v)
        } else {
            assert!(
                !self.boolean[r],
                "boolean root {} = {v}",
                self.root_names[r]
            );
            Dom::Other
        }
    }

    /// The unique class containing a concrete root valuation.
    pub fn classify(&self, values: &[u128]) -> usize {
        let hits: Vec<usize> = (0..self.classes.len())
            .filter(|i| {
                let c = &self.classes[*i];
                c.vars
                    .iter()
                    .all(|(r, d)| self.abstract_of(*r, values[*r]) == *d)
                    && c.ords.iter().all(|((a, b), o)| {
                        let got = match values[*a].cmp(&values[*b]) {
                            std::cmp::Ordering::Less => Ord3::Lt,
                            std::cmp::Ordering::Equal => Ord3::Eq,
                            std::cmp::Ordering::Greater => Ord3::Gt,
                        };
                        got == *o
                    })
            })
            .collect();
        assert_eq!(hits.len(), 1, "a valuation lies in exactly one class");
        hits[0]
    }

    pub fn class_rule(&self, i: usize) -> Option<RuleIdC> {
        self.classes[i].selected.first().copied()
    }

    /// Group label of a class: precedence level, helper kind or site.
    pub fn group_of(&self, i: usize) -> String {
        match self.class_rule(i) {
            None => "none".into(),
            Some(r) => group_label(r),
        }
    }

    pub fn atoms_json(&self, i: usize) -> Value {
        let c = &self.classes[i];
        let mut m = serde_json::Map::new();
        for (r, d) in &c.vars {
            m.insert(self.root_names[*r].clone(), Value::String(dom_str(*d)));
        }
        for ((a, b), o) in &c.ords {
            m.insert(
                format!("{}?{}", self.root_names[*a], self.root_names[*b]),
                Value::String(ord_str(*o).into()),
            );
        }
        Value::Object(m)
    }

    pub fn to_json(&self, source_sha256: &str) -> Value {
        let mut used: BTreeSet<usize> = BTreeSet::new();
        let mut ord_atoms: BTreeSet<(usize, usize)> = BTreeSet::new();
        for c in &self.classes {
            used.extend(c.vars.iter().map(|(r, _)| *r));
            ord_atoms.extend(c.ords.iter().map(|(k, _)| *k));
        }
        let fields: Vec<Value> = used
            .iter()
            .map(|r| {
                json!({
                    "name": self.root_names[*r],
                    "constants": self.consts[*r].iter().map(|c| c.to_string()).collect::<Vec<_>>(),
                    "other": !self.boolean[*r],
                })
            })
            .collect();
        let unused: Vec<&String> = (0..self.root_names.len())
            .filter(|r| !used.contains(r))
            .map(|r| &self.root_names[r])
            .collect();
        let mut per_rule: BTreeMap<String, usize> = BTreeMap::new();
        for r in &self.rules {
            per_rule.insert(format!("{r:?}"), 0);
        }
        let mut none = 0usize;
        for c in &self.classes {
            match c.selected.first() {
                Some(r) => *per_rule.get_mut(&format!("{r:?}")).expect("rule") += 1,
                None => none += 1,
            }
        }
        let classes: Vec<Value> = (0..self.classes.len())
            .map(|i| {
                json!({
                    "id": i,
                    "group": self.group_of(i),
                    "rule": self.class_rule(i).map(|r| format!("{r:?}")),
                    "selected": self.classes[i].selected.len(),
                    "atoms": self.atoms_json(i),
                })
            })
            .collect();
        json!({
            "format": PARTITION_FORMAT,
            "source": "crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs",
            "source_sha256": source_sha256,
            "derivation": "decision tree over the value facts read by C6DispatchEmitter::facts/path (unchanged dispatch code, symbolic inputs); every boolean fact and guard is evaluated from the dispatch IR slice; constants are read from that slice",
            "label": "coverage heuristic: complete guard dependencies (mechanical), no congruence proof; exclusivity is exhaustive over the over-approximated abstract domain",
            "slice_instructions": self.slice_len,
            "fields": fields,
            "roots_not_split": unused,
            "order_atoms": ord_atoms.iter().map(|(a, b)| format!("{}?{}", self.root_names[*a], self.root_names[*b])).collect::<Vec<_>>(),
            "premises": PREMISES,
            "over_approximations": OVER_APPROXIMATIONS,
            "supported_rules": self.rules.len(),
            "class_count": self.classes.len(),
            "classes_without_rule": none,
            "classes_per_rule": per_rule,
            "overlapping_classes": self.overlaps().len(),
            "classes": classes,
        })
    }
}

/// Concrete off-circuit evaluation of dispatch IR (the instruction subset the
/// dispatch emitters use) with the ZKIR VM semantics: `Not`/`CondSelect`
/// need booleans, `LessThan`/`ConstrainBits` need values below `2^bits`,
/// `Assert`/`ConstrainEq`/`ConstrainToBoolean` refuse. Compiled once to
/// slot-indexed operations; [`Compiled::run`] returns the memory or the first
/// violated constraint.
pub(crate) struct Compiled {
    ops: Vec<COp>,
    slots: usize,
    slot_of: HashMap<Identifier, usize>,
}

#[derive(Clone, Copy)]
enum CRef {
    Imm(Fr),
    Slot(usize),
}

#[derive(Clone, Copy)]
enum COp {
    TestEq(CRef, CRef, usize),
    Add(CRef, CRef, usize),
    Mul(CRef, CRef, usize),
    Not(CRef, usize),
    CondSelect(CRef, CRef, CRef, usize),
    LessThan(CRef, CRef, u32, usize),
    Bits(CRef, u32),
    Bool(CRef),
    Eq(CRef, CRef),
    Assert(CRef),
}

fn cref(slot_of: &HashMap<Identifier, usize>, o: &Operand) -> Result<CRef, String> {
    Ok(match o {
        Operand::Immediate(f) => CRef::Imm(*f),
        Operand::Variable(id) => {
            CRef::Slot(*slot_of.get(id).ok_or_else(|| format!("unbound {}", id.0))?)
        }
    })
}

fn cslot(slot_of: &mut HashMap<Identifier, usize>, id: &Identifier) -> usize {
    let n = slot_of.len();
    *slot_of.entry(id.clone()).or_insert(n)
}

impl Compiled {
    /// `inputs` are the identifiers bound before evaluation, in slot order.
    pub fn new(ins: &[Instruction], inputs: &[Identifier]) -> Result<Self, String> {
        let mut so: HashMap<Identifier, usize> = HashMap::new();
        for id in inputs {
            cslot(&mut so, id);
        }
        assert_eq!(so.len(), inputs.len(), "distinct inputs");
        let mut ops = Vec::with_capacity(ins.len());
        for insn in ins {
            let op = match insn {
                Instruction::TestEq { a, b, output } => {
                    let (a, b) = (cref(&so, a)?, cref(&so, b)?);
                    COp::TestEq(a, b, cslot(&mut so, output))
                }
                Instruction::Add { a, b, output } => {
                    let (a, b) = (cref(&so, a)?, cref(&so, b)?);
                    COp::Add(a, b, cslot(&mut so, output))
                }
                Instruction::Mul { a, b, output } => {
                    let (a, b) = (cref(&so, a)?, cref(&so, b)?);
                    COp::Mul(a, b, cslot(&mut so, output))
                }
                Instruction::Not { a, output } => {
                    let a = cref(&so, a)?;
                    COp::Not(a, cslot(&mut so, output))
                }
                Instruction::CondSelect { bit, a, b, output } => {
                    let (c, a, b) = (cref(&so, bit)?, cref(&so, a)?, cref(&so, b)?);
                    COp::CondSelect(c, a, b, cslot(&mut so, output))
                }
                Instruction::LessThan { a, b, bits, output } => {
                    let (a, b) = (cref(&so, a)?, cref(&so, b)?);
                    COp::LessThan(a, b, *bits, cslot(&mut so, output))
                }
                Instruction::ConstrainBits { val, bits } => COp::Bits(cref(&so, val)?, *bits),
                Instruction::ConstrainToBoolean { val } => COp::Bool(cref(&so, val)?),
                Instruction::ConstrainEq { a, b } => COp::Eq(cref(&so, a)?, cref(&so, b)?),
                Instruction::Assert { cond } => COp::Assert(cref(&so, cond)?),
                other => return Err(format!("unmodelled instruction {other:?}")),
            };
            ops.push(op);
        }
        Ok(Self {
            ops,
            slots: so.len(),
            slot_of: so,
        })
    }

    pub fn slot(&self, id: &Identifier) -> usize {
        self.slot_of[id]
    }

    /// `inputs[i]` binds the i-th identifier given to [`Compiled::new`].
    pub fn run(&self, inputs: &[Fr]) -> Result<Vec<Fr>, String> {
        let zero = Fr::from(0u64);
        let one = Fr::from(1u64);
        let mut m = vec![zero; self.slots];
        m[..inputs.len()].copy_from_slice(inputs);
        let get = |m: &[Fr], x: CRef| match x {
            CRef::Imm(f) => f,
            CRef::Slot(s) => m[s],
        };
        let boolean = |f: Fr, i: usize| -> Result<bool, String> {
            if f == zero {
                Ok(false)
            } else if f == one {
                Ok(true)
            } else {
                Err(format!("non-boolean at {i}"))
            }
        };
        let bits = |f: Fr, n: u32, i: usize| -> Result<u128, String> {
            let v = u128::try_from(f).map_err(|_| format!("exceeds 2^{n} at {i}"))?;
            if n < 128 && v >> n != 0 {
                return Err(format!("exceeds 2^{n} at {i}"));
            }
            Ok(v)
        };
        for (i, op) in self.ops.iter().enumerate() {
            match *op {
                COp::TestEq(a, b, o) => m[o] = Fr::from(get(&m, a) == get(&m, b)),
                COp::Add(a, b, o) => m[o] = Fr(get(&m, a).0 + get(&m, b).0),
                COp::Mul(a, b, o) => m[o] = Fr(get(&m, a).0 * get(&m, b).0),
                COp::Not(a, o) => m[o] = Fr::from(!boolean(get(&m, a), i)?),
                COp::CondSelect(c, a, b, o) => {
                    m[o] = if boolean(get(&m, c), i)? {
                        get(&m, a)
                    } else {
                        get(&m, b)
                    }
                }
                COp::LessThan(a, b, n, o) => {
                    let x = bits(get(&m, a), n, i)?;
                    let y = bits(get(&m, b), n, i)?;
                    m[o] = Fr::from(x < y);
                }
                COp::Bits(v, n) => {
                    bits(get(&m, v), n, i)?;
                }
                COp::Bool(v) => {
                    boolean(get(&m, v), i)?;
                }
                COp::Eq(a, b) => {
                    if get(&m, a) != get(&m, b) {
                        return Err(format!("ConstrainEq at {i}"));
                    }
                }
                COp::Assert(c) => {
                    if !boolean(get(&m, c), i)? {
                        return Err(format!("Assert at {i}"));
                    }
                }
            }
        }
        Ok(m)
    }
}

/// Precedence group of a supported rule: publication, helper kind, helper
/// entry or legacy site.
pub(crate) fn group_label(r: RuleIdC) -> String {
    if is_publication(r) {
        return "publication".into();
    }
    if r == RuleIdC::BBookkeepingTaskRelease {
        return "helper:validate+equal".into();
    }
    if let Some(k) = helper_kind_of(r) {
        return format!("helper:{k}");
    }
    if entry_kind(r).is_some() {
        return "entry".into();
    }
    match legacy_site_of(r) {
        Some(s) => format!("site:{s:?}"),
        None => "unsupported".into(),
    }
}
