//! Helper/result/task section of sherlock-c6-phys/1 (C6P1), Chunk 3B.
//!
//! Codec over the three owned `ImageSnapshot` fields `helper`, `result`,
//! `tasks`. It admits nothing, constructs no `Admitted`/`MachineImage`/
//! `ImageSnapshot`, steps nothing, stores no task cursor (derived from the
//! task count) and checks no cross-table membership or owner admission
//! (Chunk 4). Reserved fields (`parent_resume`, PatternTag reserved variants)
//! are carried as representation only; they are not admitted here.
//!
//! Layout: magic, version, `HELPER_SECTION_TAG`, task count; helper record
//! (23 lanes) at 4; result record (4) at 27; then tasks (7 each) at 31,
//! bottom-to-top. Lane 0 of each record is its tag: helper None=0, Scalar=1,
//! EnvGet, EnvPut, Find, Bind, Append, Validate, Equal, Match, Plan, Build=11;
//! result None=0, then 1..=19 in `HelperResultC` declaration order; task
//! ValidateTask=0, EqualTask=1, PairTask=2. Fields follow in `shared.rs`
//! declaration order. Helpers start with common [site, origin_reference_cursor
//! (full u128), parent_resume [p,v]]. Widths: usize/u8/bool/enum/u128 1;
//! RefC 2 [0|1, i]; EnvRootC and non-env Option<usize> 2 (None [0,0], Some
//! [1,v]; Chunk 1 env encoding, meaning per field); Option<TypeIdC> 2 likewise;
//! Option<RefC> 3 (None [0,0,0], Some [1,tag,i]); SliceC 2 [start,len], raw
//! (an empty slice keeps its start); ScalarStaged 2 [tag 0..=6, payload] with
//! None payload 0. Enum tags are the explicit literal maps below.
//! Unused lanes are zero; bools are 0/1; u8 lanes refuse > 255 (UsizeOverflow).
//!
//! Caps (index strictly below, count at most): Const max_constants, Data
//! max_data_nodes, EnvRootC max_env_cells; names max_names; Bind.function,
//! Find.found, FindFound max_functions (index), Find.row_cursor max_functions
//! (count); EnvGet.cursor, EnvPut.new_root, Bind.dup_cursor, EnvUpdated.cell
//! max_env_cells (index); Plan.arm, Match.arm, PlanMatched.arm max_arms;
//! Plan.source_match max_expressions; Plan.node max_plan_nodes; Bind.params
//! parameters end, len <= max_arity; Bind.args/Append.src/Build.src/AppendDone.seq
//! value_slots end, len <= max_arity; Plan.path field_paths end, no arity;
//! p_cursor/a_cursor/src_cursor/path_cursor count <= own slice len;
//! Append.dest_start count <= max_value_slots; Build.field_start count <=
//! max_data_field_slots; task cursors count <= max_arity; Match.x/y names only
//! when the phase is past Arm and the pat_tag binds them (else raw placeholders);
//! Scalar.expected and PairTask.sort `max(max_types, 4)` (builtin-capable);
//! Build.type_id plain `max_types` (user nominal, as 3A Constructor);
//! parent_resume, origin, reg_a, U128Value.n and u8 codes carry no cap. A slice
//! checks end overflow, then arity, then end. Caps follow lane order.
//!
//! `decode_helper_tables` first-error order: 1. fewer than 31 lanes:
//! `Truncated`; 2. `BadMagic`; 3. `BadVersion`; 4. tag `BadTag`; 5. task count
//! structurally; 6. task count against `max_helper_tasks`; 7. section length
//! (checked; `UsizeOverflow`), `Truncated`, then `TrailingLanes` (no allocation
//! before); 8. helper record structurally (ascending record-relative lanes)
//! then caps; 9. result likewise; 10. tasks in order, each likewise.

use super::c6_phys::{C6_PHYS_MAGIC, C6_PHYS_VERSION, C6PhysError, C6Result};
use super::c6_phys_data::{
    C6StateCaps, cap_count, cap_env, cap_index, cap_ref, get_env, get_ref, put_env, put_ref, rd,
    rd_bool, u, zero,
};
use super::complete_observation::{HostSnapshot, ImageSnapshot};
use super::shared::adt::{CtorIdC, TypeIdC};
use super::shared::{
    AppendPhase, BindPhase, BindStatus, BoolSelector, BuildPhase, HelperCommon, HelperControlC,
    HelperResultC, HelperTaskC, MatchPhase, NodeKind, PatternTag, PlanAfterWalk, PlanPhase, RefC,
    ScalarMode, ScalarPhase, ScalarStaged, SiteTag, SliceC, TaskVisitC, TriStatus,
};

pub const HELPER_SECTION_TAG: u128 = 0x4845_4C50;
pub const HELPER_HEADER_LANES: usize = 4;
pub const HELPER_LANES: usize = 23;
pub const RESULT_LANES: usize = 4;
pub const TASK_LANES: usize = 7;
pub const L_HELPER_RECORD: usize = 4;
pub const L_RESULT_RECORD: usize = 27;
pub const L_TASKS: usize = 31;
pub const HELPER_FIXED_LANES: usize = 31;
const SECTION: &str = "state_helper";
const HELPER_S: &str = "state.helper";
const RESULT_S: &str = "state.result";
const TASK_S: &str = "state.task";

// ---------- local primitives (3A conventions) ----------

fn rd_u8(section: &'static str, value: u128) -> C6Result<u8> {
    u8::try_from(value).map_err(|_| C6PhysError::UsizeOverflow { section, value })
}
fn cap_opt_ref(section: &'static str, v: Option<RefC>, c: &C6StateCaps) -> C6Result<()> {
    match v {
        Some(r) => cap_ref(section, r, c),
        None => Ok(()),
    }
}
/// Non-environment optional index against its own table; never `cap_env`.
fn cap_opt_index(section: &'static str, v: Option<usize>, limit: usize) -> C6Result<()> {
    match v {
        Some(i) => cap_index(section, i, limit),
        None => Ok(()),
    }
}
fn cap_slice(
    section: &'static str,
    s: SliceC,
    table: usize,
    max_len: Option<usize>,
) -> C6Result<()> {
    let end = s
        .start
        .checked_add(s.len)
        .ok_or(C6PhysError::UsizeOverflow {
            section,
            value: u(s.start),
        })?;
    if let Some(max_len) = max_len {
        cap_count(section, s.len, max_len)?;
    }
    cap_count(section, end, table)
}
/// Builtin TypeIds 0..=3 exist independently of `type_rows` (Core policy).
fn builtin_type_cap(c: &C6StateCaps) -> usize {
    c.phys.max_types.max(4)
}

macro_rules! lane_enum {
    ($to:ident, $from:ident, $ty:ident, { $($v:ident = $n:literal),+ $(,)? }) => {
        fn $to(v: $ty) -> u128 { match v { $($ty::$v => $n),+ } }
        fn $from(section: &'static str, tag: u128) -> C6Result<$ty> {
            match tag { $($n => Ok($ty::$v),)+ _ => Err(C6PhysError::BadTag { section, tag }) }
        }
    };
}
lane_enum!(site_tag, site_from, SiteTag, { EvalLiteral = 0, EvalVar = 1, EvalLet = 2, EvalBinary = 3,
    EvalUnary = 4, EvalIf = 5, EvalMatch = 6, EvalCall = 7, Push = 8, RetLet = 9, RetLeft = 10,
    RetRight = 11, RetUnary = 12, RetIf = 13, RetMatch = 14, RetArgs = 15, RetCall = 16, RetScope = 17,
    Invoke = 18, Enter = 19, Bound = 20, Cases = 21, CaseResult = 22, Prim = 23, EvalConstruct = 24,
    RetCtorFields = 25, Materialize = 26 });
lane_enum!(mode_tag, mode_from, ScalarMode, { Add = 0, Sub = 1, Mul = 2, Lt = 3, Cons = 4, Branch = 5,
    Leaf = 6, Not = 7, TestBool = 8, TestType = 9 });
lane_enum!(sphase_tag, sphase_from, ScalarPhase, { First = 0, Second = 1, Done = 2 });
lane_enum!(node_tag, node_from, NodeKind, { Cons = 0, Leaf = 1, Branch = 2 });
lane_enum!(sel_tag, sel_from, BoolSelector, { True = 0, False = 1, NotBool = 2 });
lane_enum!(tri_tag, tri_from, TriStatus, { Pending = 0, Positive = 1, Negative = 2 });
lane_enum!(bphase_tag, bphase_from, BindPhase, { Pairing = 0, UnwindType = 1, UnwindDup = 2,
    UnwindInsert = 3, Propagate = 4, Finished = 5 });
lane_enum!(bstatus_tag, bstatus_from, BindStatus, { Ok = 0, Bad = 1 });
lane_enum!(aphase_tag, aphase_from, AppendPhase, { Copy = 0, Tail = 1, Done = 2 });
lane_enum!(mphase_tag, mphase_from, MatchPhase, { Arm = 0, Tag = 1, Distinct = 2, Field = 3, Bind = 4,
    Ready = 5, NoMatchReady = 6, Done = 7 });
lane_enum!(pphase_tag, pphase_from, PlanPhase, { Walk = 0, Test = 1, Bind = 2, Ready = 3,
    NoMatchReady = 4, Done = 5 });
lane_enum!(walk_tag, walk_from, PlanAfterWalk, { Test = 0, Bind = 1 });
lane_enum!(build_tag, build_from, BuildPhase, { Copy = 0, Done = 1 });
lane_enum!(pat_tag_of, pat_from, PatternTag, { PVar = 0, PNil = 1, PCons = 2, PLeaf = 3, PBranch = 4,
    WildcardExtensionReserved = 5, ConstructorExtensionReserved = 6 });
lane_enum!(visit_tag, visit_from, TaskVisitC, { Unvisited = 0, Fields = 1 });

/// Fixed-width writer over a zero-initialized record; offsets are static.
struct Wr<const N: usize> {
    o: [u128; N],
    at: usize,
}
impl<const N: usize> Wr<N> {
    fn new() -> Self {
        Self { o: [0; N], at: 0 }
    }
    fn v(&mut self, x: u128) {
        self.o[self.at] = x;
        self.at += 1;
    }
    fn us(&mut self, x: usize) {
        self.v(u(x));
    }
    fn r(&mut self, x: RefC) {
        put_ref(&mut self.o, self.at, x);
        self.at += 2;
    }
    fn opt(&mut self, x: Option<usize>) {
        put_env(&mut self.o, self.at, x);
        self.at += 2;
    }
    fn opt_ref(&mut self, x: Option<RefC>) {
        if let Some(r) = x {
            self.o[self.at] = 1;
            put_ref(&mut self.o, self.at + 1, r);
        }
        self.at += 3;
    }
    fn slice(&mut self, s: SliceC) {
        self.us(s.start);
        self.us(s.len);
    }
    fn head(&mut self, kind: u128, c: HelperCommon) {
        self.v(kind);
        self.v(site_tag(c.site));
        self.v(c.origin_reference_cursor);
        self.opt(c.parent_resume);
    }
    fn staged(&mut self, s: ScalarStaged) {
        use ScalarStaged as S;
        let (t, x) = match s {
            S::None => (0, 0),
            S::U128Value { n } => (1, n),
            S::BoolValue { b } => (2, u128::from(b)),
            S::PendingNode { tag } => (3, node_tag(tag)),
            S::PrimError { code } => (4, u128::from(code)),
            S::Selector { sel } => (5, sel_tag(sel)),
            S::TypeTest { ok } => (6, u128::from(ok)),
        };
        self.v(t);
        self.v(x);
    }
}

/// Structural reader in ascending lanes; `l` is a statically sized record.
struct Rd<'a> {
    l: &'a [u128],
    at: usize,
    s: &'static str,
}
impl Rd<'_> {
    fn raw(&mut self) -> u128 {
        let v = self.l[self.at];
        self.at += 1;
        v
    }
    fn us(&mut self) -> C6Result<usize> {
        let v = self.raw();
        rd(self.s, v)
    }
    fn byte(&mut self) -> C6Result<u8> {
        let v = self.raw();
        rd_u8(self.s, v)
    }
    fn tag<T>(&mut self, f: fn(&'static str, u128) -> C6Result<T>) -> C6Result<T> {
        let v = self.raw();
        f(self.s, v)
    }
    fn r(&mut self) -> C6Result<RefC> {
        let v = get_ref(self.s, self.l, self.at)?;
        self.at += 2;
        Ok(v)
    }
    fn opt(&mut self) -> C6Result<Option<usize>> {
        let v = get_env(self.s, self.l, self.at)?;
        self.at += 2;
        Ok(v)
    }
    fn opt_ref(&mut self) -> C6Result<Option<RefC>> {
        match self.raw() {
            0 => {
                zero(self.s, self.l, self.at, self.at + 2)?;
                self.at += 2;
                Ok(None)
            }
            1 => self.r().map(Some),
            tag => Err(C6PhysError::BadTag {
                section: self.s,
                tag,
            }),
        }
    }
    fn slice(&mut self) -> C6Result<SliceC> {
        Ok(SliceC {
            start: self.us()?,
            len: self.us()?,
        })
    }
    fn common(&mut self) -> C6Result<HelperCommon> {
        Ok(HelperCommon {
            site: self.tag(site_from)?,
            origin_reference_cursor: self.raw(),
            parent_resume: self.opt()?,
        })
    }
    fn staged(&mut self) -> C6Result<ScalarStaged> {
        use ScalarStaged as S;
        let (tag, x, s) = (self.raw(), self.raw(), self.s);
        Ok(match tag {
            0 => {
                zero(s, self.l, self.at - 1, self.at)?;
                S::None
            }
            1 => S::U128Value { n: x },
            2 => S::BoolValue { b: rd_bool(s, x)? },
            3 => S::PendingNode {
                tag: node_from(s, x)?,
            },
            4 => S::PrimError { code: rd_u8(s, x)? },
            5 => S::Selector {
                sel: sel_from(s, x)?,
            },
            6 => S::TypeTest { ok: rd_bool(s, x)? },
            tag => return Err(C6PhysError::BadTag { section: s, tag }),
        })
    }
}

// ---------- helper record ----------

pub fn encode_helper_record(h: Option<HelperControlC>) -> [u128; HELPER_LANES] {
    use HelperControlC as H;
    let mut w = Wr::<HELPER_LANES>::new();
    let Some(h) = h else { return w.o };
    match h {
        H::Scalar {
            common,
            mode,
            operand_a,
            operand_b,
            expected,
            phase,
            reg_a,
            staged,
        } => {
            w.head(1, common);
            w.v(mode_tag(mode));
            w.r(operand_a);
            w.opt_ref(operand_b);
            w.opt(expected.map(|t| t.0));
            w.v(sphase_tag(phase));
            w.v(reg_a);
            w.staged(staged);
        }
        H::EnvGet {
            common,
            binder,
            cursor,
            found,
            status,
        } => {
            w.head(2, common);
            w.us(binder);
            w.opt(cursor);
            w.opt_ref(found);
            w.v(tri_tag(status));
        }
        H::EnvPut {
            common,
            binder,
            value,
            base,
            new_root,
            status,
        } => {
            w.head(3, common);
            w.us(binder);
            w.r(value);
            w.opt(base);
            w.opt(new_root);
            w.v(tri_tag(status));
        }
        H::Find {
            common,
            function_name,
            row_cursor,
            found,
            status,
        } => {
            w.head(4, common);
            w.us(function_name);
            w.us(row_cursor);
            w.opt(found);
            w.v(tri_tag(status));
        }
        H::Bind {
            common,
            function,
            params,
            args,
            p_cursor,
            a_cursor,
            partial,
            dup_cursor,
            phase,
            status,
        } => {
            w.head(5, common);
            w.us(function);
            w.slice(params);
            w.slice(args);
            w.us(p_cursor);
            w.us(a_cursor);
            w.opt(partial);
            w.opt(dup_cursor);
            w.v(bphase_tag(phase));
            w.v(bstatus_tag(status));
        }
        H::Append {
            common,
            src,
            value,
            src_cursor,
            dest_start,
            staged,
            phase,
        } => {
            w.head(6, common);
            w.slice(src);
            w.r(value);
            w.us(src_cursor);
            w.us(dest_start);
            w.opt_ref(staged);
            w.v(aphase_tag(phase));
        }
        H::Validate {
            common,
            root,
            status,
        } => {
            w.head(7, common);
            w.r(root);
            w.v(tri_tag(status));
        }
        H::Equal {
            common,
            left,
            right,
            status,
        } => {
            w.head(8, common);
            w.r(left);
            w.r(right);
            w.v(tri_tag(status));
        }
        H::Match {
            common,
            scrutinee,
            arm,
            saved_env,
            partial,
            pat_tag,
            x,
            y,
            field_idx,
            staged,
            phase,
            status,
        } => {
            w.head(9, common);
            w.r(scrutinee);
            w.us(arm);
            w.opt(saved_env);
            w.opt(partial);
            w.v(pat_tag_of(pat_tag));
            w.us(x);
            w.us(y);
            w.v(u128::from(field_idx));
            w.opt_ref(staged);
            w.v(mphase_tag(phase));
            w.v(tri_tag(status));
        }
        H::Plan {
            common,
            source_match,
            scrutinee,
            saved_env,
            partial,
            node,
            phase,
            after_walk,
            cur,
            path,
            path_cursor,
            arm,
            status,
        } => {
            w.head(10, common);
            w.us(source_match);
            w.r(scrutinee);
            w.opt(saved_env);
            w.opt(partial);
            w.us(node);
            w.v(pphase_tag(phase));
            w.v(walk_tag(after_walk));
            w.r(cur);
            w.slice(path);
            w.us(path_cursor);
            w.opt(arm);
            w.v(tri_tag(status));
        }
        H::Build {
            common,
            type_id,
            ctor_id,
            src,
            src_cursor,
            field_start,
            staged,
            node,
            phase,
        } => {
            w.head(11, common);
            w.us(type_id.0);
            w.us(ctor_id.0);
            w.slice(src);
            w.us(src_cursor);
            w.us(field_start);
            w.opt_ref(staged);
            w.opt_ref(node);
            w.v(build_tag(phase));
        }
    }
    w.o
}

/// Structural helper decode: tag, representability, presence and padding.
pub fn decode_helper_record(l: &[u128; HELPER_LANES]) -> C6Result<Option<HelperControlC>> {
    use HelperControlC as H;
    let mut d = Rd {
        l: &l[..],
        at: 1,
        s: HELPER_S,
    };
    let h = match l[0] {
        0 => None,
        1 => Some(H::Scalar {
            common: d.common()?,
            mode: d.tag(mode_from)?,
            operand_a: d.r()?,
            operand_b: d.opt_ref()?,
            expected: d.opt()?.map(TypeIdC),
            phase: d.tag(sphase_from)?,
            reg_a: d.raw(),
            staged: d.staged()?,
        }),
        2 => Some(H::EnvGet {
            common: d.common()?,
            binder: d.us()?,
            cursor: d.opt()?,
            found: d.opt_ref()?,
            status: d.tag(tri_from)?,
        }),
        3 => Some(H::EnvPut {
            common: d.common()?,
            binder: d.us()?,
            value: d.r()?,
            base: d.opt()?,
            new_root: d.opt()?,
            status: d.tag(tri_from)?,
        }),
        4 => Some(H::Find {
            common: d.common()?,
            function_name: d.us()?,
            row_cursor: d.us()?,
            found: d.opt()?,
            status: d.tag(tri_from)?,
        }),
        5 => Some(H::Bind {
            common: d.common()?,
            function: d.us()?,
            params: d.slice()?,
            args: d.slice()?,
            p_cursor: d.us()?,
            a_cursor: d.us()?,
            partial: d.opt()?,
            dup_cursor: d.opt()?,
            phase: d.tag(bphase_from)?,
            status: d.tag(bstatus_from)?,
        }),
        6 => Some(H::Append {
            common: d.common()?,
            src: d.slice()?,
            value: d.r()?,
            src_cursor: d.us()?,
            dest_start: d.us()?,
            staged: d.opt_ref()?,
            phase: d.tag(aphase_from)?,
        }),
        7 => Some(H::Validate {
            common: d.common()?,
            root: d.r()?,
            status: d.tag(tri_from)?,
        }),
        8 => Some(H::Equal {
            common: d.common()?,
            left: d.r()?,
            right: d.r()?,
            status: d.tag(tri_from)?,
        }),
        9 => Some(H::Match {
            common: d.common()?,
            scrutinee: d.r()?,
            arm: d.us()?,
            saved_env: d.opt()?,
            partial: d.opt()?,
            pat_tag: d.tag(pat_from)?,
            x: d.us()?,
            y: d.us()?,
            field_idx: d.byte()?,
            staged: d.opt_ref()?,
            phase: d.tag(mphase_from)?,
            status: d.tag(tri_from)?,
        }),
        10 => Some(H::Plan {
            common: d.common()?,
            source_match: d.us()?,
            scrutinee: d.r()?,
            saved_env: d.opt()?,
            partial: d.opt()?,
            node: d.us()?,
            phase: d.tag(pphase_from)?,
            after_walk: d.tag(walk_from)?,
            cur: d.r()?,
            path: d.slice()?,
            path_cursor: d.us()?,
            arm: d.opt()?,
            status: d.tag(tri_from)?,
        }),
        11 => Some(H::Build {
            common: d.common()?,
            type_id: TypeIdC(d.us()?),
            ctor_id: CtorIdC(d.us()?),
            src: d.slice()?,
            src_cursor: d.us()?,
            field_start: d.us()?,
            staged: d.opt_ref()?,
            node: d.opt_ref()?,
            phase: d.tag(build_from)?,
        }),
        tag => {
            return Err(C6PhysError::BadTag {
                section: HELPER_S,
                tag,
            });
        }
    };
    zero(HELPER_S, l, d.at, HELPER_LANES)?;
    Ok(h)
}

fn helper_caps(h: &HelperControlC, c: &C6StateCaps) -> C6Result<()> {
    use HelperControlC as H;
    let p = &c.phys;
    match *h {
        H::Scalar {
            operand_a,
            operand_b,
            expected,
            ..
        } => {
            cap_ref("state.helper.operand_a", operand_a, c)?;
            cap_opt_ref("state.helper.operand_b", operand_b, c)?;
            cap_opt_index(
                "state.helper.expected",
                expected.map(|t| t.0),
                builtin_type_cap(c),
            )
        }
        H::EnvGet {
            binder,
            cursor,
            found,
            ..
        } => {
            cap_index("state.helper.binder", binder, p.max_names)?;
            cap_opt_index("state.helper.cursor", cursor, p.max_env_cells)?;
            cap_opt_ref("state.helper.found", found, c)
        }
        H::EnvPut {
            binder,
            value,
            base,
            new_root,
            ..
        } => {
            cap_index("state.helper.binder", binder, p.max_names)?;
            cap_ref("state.helper.value", value, c)?;
            cap_env("state.helper.base", base, c)?;
            cap_opt_index("state.helper.new_root", new_root, p.max_env_cells)
        }
        H::Find {
            function_name,
            row_cursor,
            found,
            ..
        } => {
            cap_index("state.helper.function_name", function_name, p.max_names)?;
            cap_count("state.helper.row_cursor", row_cursor, p.max_functions)?;
            cap_opt_index("state.helper.found", found, p.max_functions)
        }
        H::Bind {
            function,
            params,
            args,
            p_cursor,
            a_cursor,
            partial,
            dup_cursor,
            ..
        } => {
            cap_index("state.helper.function", function, p.max_functions)?;
            cap_slice(
                "state.helper.params",
                params,
                p.max_parameters,
                Some(p.max_arity),
            )?;
            cap_slice(
                "state.helper.args",
                args,
                p.max_value_slots,
                Some(p.max_arity),
            )?;
            cap_count("state.helper.p_cursor", p_cursor, params.len)?;
            cap_count("state.helper.a_cursor", a_cursor, args.len)?;
            cap_env("state.helper.partial", partial, c)?;
            cap_opt_index("state.helper.dup_cursor", dup_cursor, p.max_env_cells)
        }
        H::Append {
            src,
            value,
            src_cursor,
            dest_start,
            staged,
            ..
        } => {
            cap_slice(
                "state.helper.src",
                src,
                p.max_value_slots,
                Some(p.max_arity),
            )?;
            cap_ref("state.helper.value", value, c)?;
            cap_count("state.helper.src_cursor", src_cursor, src.len)?;
            cap_count("state.helper.dest_start", dest_start, p.max_value_slots)?;
            cap_opt_ref("state.helper.staged", staged, c)
        }
        H::Validate { root, .. } => cap_ref("state.helper.root", root, c),
        H::Equal { left, right, .. } => {
            cap_ref("state.helper.left", left, c)?;
            cap_ref("state.helper.right", right, c)
        }
        H::Match {
            scrutinee,
            arm,
            saved_env,
            partial,
            pat_tag,
            x,
            y,
            staged,
            phase,
            ..
        } => {
            cap_ref("state.helper.scrutinee", scrutinee, c)?;
            cap_index("state.helper.arm", arm, p.max_arms)?;
            cap_env("state.helper.saved_env", saved_env, c)?;
            cap_env("state.helper.partial", partial, c)?;
            if phase != MatchPhase::Arm {
                match pat_tag {
                    PatternTag::PVar | PatternTag::PLeaf => {
                        cap_index("state.helper.x", x, p.max_names)?
                    }
                    PatternTag::PCons | PatternTag::PBranch => {
                        cap_index("state.helper.x", x, p.max_names)?;
                        cap_index("state.helper.y", y, p.max_names)?;
                    }
                    PatternTag::PNil
                    | PatternTag::WildcardExtensionReserved
                    | PatternTag::ConstructorExtensionReserved => {}
                }
            }
            cap_opt_ref("state.helper.staged", staged, c)
        }
        H::Plan {
            source_match,
            scrutinee,
            saved_env,
            partial,
            node,
            cur,
            path,
            path_cursor,
            arm,
            ..
        } => {
            cap_index("state.helper.source_match", source_match, p.max_expressions)?;
            cap_ref("state.helper.scrutinee", scrutinee, c)?;
            cap_env("state.helper.saved_env", saved_env, c)?;
            cap_env("state.helper.partial", partial, c)?;
            cap_index("state.helper.node", node, p.max_plan_nodes)?;
            cap_ref("state.helper.cur", cur, c)?;
            cap_slice("state.helper.path", path, p.max_field_paths, None)?;
            cap_count("state.helper.path_cursor", path_cursor, path.len)?;
            cap_opt_index("state.helper.arm", arm, p.max_arms)
        }
        H::Build {
            type_id,
            ctor_id,
            src,
            src_cursor,
            field_start,
            staged,
            node,
            ..
        } => {
            cap_index("state.helper.type_id", type_id.0, p.max_types)?;
            cap_index("state.helper.ctor_id", ctor_id.0, p.max_constructors)?;
            cap_slice(
                "state.helper.src",
                src,
                p.max_value_slots,
                Some(p.max_arity),
            )?;
            cap_count("state.helper.src_cursor", src_cursor, src.len)?;
            cap_count(
                "state.helper.field_start",
                field_start,
                c.max_data_field_slots,
            )?;
            cap_opt_ref("state.helper.staged", staged, c)?;
            cap_opt_ref("state.helper.node", node, c)
        }
    }
}

pub fn decode_helper_record_checked(
    l: &[u128; HELPER_LANES],
    c: &C6StateCaps,
) -> C6Result<Option<HelperControlC>> {
    let h = decode_helper_record(l)?;
    if let Some(h) = &h {
        helper_caps(h, c)?;
    }
    Ok(h)
}

// ---------- result record ----------

pub fn encode_result_record(r: Option<HelperResultC>) -> [u128; RESULT_LANES] {
    use HelperResultC as R;
    let mut w = Wr::<RESULT_LANES>::new();
    let Some(r) = r else { return w.o };
    match r {
        R::Validated { ok } => {
            w.v(1);
            w.v(u128::from(ok));
        }
        R::EnvFound { value } => {
            w.v(2);
            w.r(value);
        }
        R::EnvMissing => w.v(3),
        R::EnvUpdated { cell } => {
            w.v(4);
            w.us(cell);
        }
        R::FindFound { function } => {
            w.v(5);
            w.us(function);
        }
        R::FindMissing => w.v(6),
        R::BindOk { env } => {
            w.v(7);
            w.opt(env);
        }
        R::BindBad => w.v(8),
        R::AppendDone { seq } => {
            w.v(9);
            w.slice(seq);
        }
        R::ScalarValue { value } => {
            w.v(10);
            w.r(value);
        }
        R::ScalarError { code } => {
            w.v(11);
            w.v(u128::from(code));
        }
        R::ScalarSelector { sel } => {
            w.v(12);
            w.v(sel_tag(sel));
        }
        R::ScalarTypeTest { ok } => {
            w.v(13);
            w.v(u128::from(ok));
        }
        R::EqualDone { value } => {
            w.v(14);
            w.r(value);
        }
        R::MatchMatched { env } => {
            w.v(15);
            w.opt(env);
        }
        R::MatchNoMatch => w.v(16),
        R::PlanMatched { arm, env } => {
            w.v(17);
            w.us(arm);
            w.opt(env);
        }
        R::PlanNoMatch => w.v(18),
        R::BuildDone { value } => {
            w.v(19);
            w.r(value);
        }
    }
    w.o
}

pub fn decode_result_record(l: &[u128; RESULT_LANES]) -> C6Result<Option<HelperResultC>> {
    use HelperResultC as R;
    let mut d = Rd {
        l: &l[..],
        at: 1,
        s: RESULT_S,
    };
    let r = match l[0] {
        0 => None,
        1 => Some(R::Validated {
            ok: rd_bool(RESULT_S, d.raw())?,
        }),
        2 => Some(R::EnvFound { value: d.r()? }),
        3 => Some(R::EnvMissing),
        4 => Some(R::EnvUpdated { cell: d.us()? }),
        5 => Some(R::FindFound { function: d.us()? }),
        6 => Some(R::FindMissing),
        7 => Some(R::BindOk { env: d.opt()? }),
        8 => Some(R::BindBad),
        9 => Some(R::AppendDone { seq: d.slice()? }),
        10 => Some(R::ScalarValue { value: d.r()? }),
        11 => Some(R::ScalarError { code: d.byte()? }),
        12 => Some(R::ScalarSelector {
            sel: d.tag(sel_from)?,
        }),
        13 => Some(R::ScalarTypeTest {
            ok: rd_bool(RESULT_S, d.raw())?,
        }),
        14 => Some(R::EqualDone { value: d.r()? }),
        15 => Some(R::MatchMatched { env: d.opt()? }),
        16 => Some(R::MatchNoMatch),
        17 => Some(R::PlanMatched {
            arm: d.us()?,
            env: d.opt()?,
        }),
        18 => Some(R::PlanNoMatch),
        19 => Some(R::BuildDone { value: d.r()? }),
        tag => {
            return Err(C6PhysError::BadTag {
                section: RESULT_S,
                tag,
            });
        }
    };
    zero(RESULT_S, l, d.at, RESULT_LANES)?;
    Ok(r)
}

fn result_caps(r: &HelperResultC, c: &C6StateCaps) -> C6Result<()> {
    use HelperResultC as R;
    let p = &c.phys;
    match *r {
        R::EnvFound { value }
        | R::ScalarValue { value }
        | R::EqualDone { value }
        | R::BuildDone { value } => cap_ref("state.result.value", value, c),
        R::EnvUpdated { cell } => cap_index("state.result.cell", cell, p.max_env_cells),
        R::FindFound { function } => cap_index("state.result.function", function, p.max_functions),
        R::BindOk { env } | R::MatchMatched { env } => cap_env("state.result.env", env, c),
        R::AppendDone { seq } => cap_slice(
            "state.result.seq",
            seq,
            p.max_value_slots,
            Some(p.max_arity),
        ),
        R::PlanMatched { arm, env } => {
            cap_index("state.result.arm", arm, p.max_arms)?;
            cap_env("state.result.env", env, c)
        }
        R::Validated { .. }
        | R::EnvMissing
        | R::FindMissing
        | R::BindBad
        | R::ScalarError { .. }
        | R::ScalarSelector { .. }
        | R::ScalarTypeTest { .. }
        | R::MatchNoMatch
        | R::PlanNoMatch => Ok(()),
    }
}

pub fn decode_result_record_checked(
    l: &[u128; RESULT_LANES],
    c: &C6StateCaps,
) -> C6Result<Option<HelperResultC>> {
    let r = decode_result_record(l)?;
    if let Some(r) = &r {
        result_caps(r, c)?;
    }
    Ok(r)
}

// ---------- task record ----------

pub fn encode_task_record(t: HelperTaskC) -> [u128; TASK_LANES] {
    let mut w = Wr::<TASK_LANES>::new();
    match t {
        HelperTaskC::ValidateTask { r, visit, cursor } => {
            w.v(0);
            w.r(r);
            w.v(visit_tag(visit));
            w.us(cursor);
        }
        HelperTaskC::EqualTask {
            left,
            right,
            visit,
            cursor,
        } => {
            w.v(1);
            w.r(left);
            w.r(right);
            w.v(visit_tag(visit));
            w.us(cursor);
        }
        HelperTaskC::PairTask { binder, sort, arg } => {
            w.v(2);
            w.us(binder);
            w.us(sort.0);
            w.r(arg);
        }
    }
    w.o
}

pub fn decode_task_record(l: &[u128; TASK_LANES]) -> C6Result<HelperTaskC> {
    let mut d = Rd {
        l: &l[..],
        at: 1,
        s: TASK_S,
    };
    let t = match l[0] {
        0 => HelperTaskC::ValidateTask {
            r: d.r()?,
            visit: d.tag(visit_from)?,
            cursor: d.us()?,
        },
        1 => HelperTaskC::EqualTask {
            left: d.r()?,
            right: d.r()?,
            visit: d.tag(visit_from)?,
            cursor: d.us()?,
        },
        2 => HelperTaskC::PairTask {
            binder: d.us()?,
            sort: TypeIdC(d.us()?),
            arg: d.r()?,
        },
        tag => {
            return Err(C6PhysError::BadTag {
                section: TASK_S,
                tag,
            });
        }
    };
    zero(TASK_S, l, d.at, TASK_LANES)?;
    Ok(t)
}

fn task_caps(t: &HelperTaskC, c: &C6StateCaps) -> C6Result<()> {
    match *t {
        HelperTaskC::ValidateTask { r, cursor, .. } => {
            cap_ref("state.task.r", r, c)?;
            cap_count("state.task.cursor", cursor, c.phys.max_arity)
        }
        HelperTaskC::EqualTask {
            left,
            right,
            cursor,
            ..
        } => {
            cap_ref("state.task.left", left, c)?;
            cap_ref("state.task.right", right, c)?;
            cap_count("state.task.cursor", cursor, c.phys.max_arity)
        }
        HelperTaskC::PairTask { binder, sort, arg } => {
            cap_index("state.task.binder", binder, c.phys.max_names)?;
            cap_index("state.task.sort", sort.0, builtin_type_cap(c))?;
            cap_ref("state.task.arg", arg, c)
        }
    }
}

pub fn decode_task_record_checked(
    l: &[u128; TASK_LANES],
    c: &C6StateCaps,
) -> C6Result<HelperTaskC> {
    let t = decode_task_record(l)?;
    task_caps(&t, c)?;
    Ok(t)
}

// ---------- owned carrier and section ----------

/// The three owned image fields, exactly as captured: helper/result presence,
/// every raw field, task order and multiplicity. No task cursor is stored.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6HelperTables {
    pub helper: Option<HelperControlC>,
    pub result: Option<HelperResultC>,
    pub tasks: Vec<HelperTaskC>,
}

impl C6HelperTables {
    pub fn from_image(image: &ImageSnapshot) -> Self {
        Self {
            helper: image.helper,
            result: image.result,
            tasks: image.tasks.clone(),
        }
    }
    /// Diagnostic: overwrite exactly these three fields of a caller-owned image.
    pub fn replace_tables_of(self, image: &mut ImageSnapshot) {
        image.helper = self.helper;
        image.result = self.result;
        image.tasks = self.tasks;
    }
}

fn section_len(tasks: usize) -> C6Result<usize> {
    tasks
        .checked_mul(TASK_LANES)
        .and_then(|n| n.checked_add(HELPER_FIXED_LANES))
        .ok_or(C6PhysError::UsizeOverflow {
            section: SECTION,
            value: u(tasks),
        })
}

/// Canonical encoder: count cap, then helper, result and task caps, before allocation.
pub fn encode_helper_tables(t: &C6HelperTables, c: &C6StateCaps) -> C6Result<Vec<u128>> {
    cap_count("state.tasks", t.tasks.len(), c.max_helper_tasks)?;
    if let Some(h) = &t.helper {
        helper_caps(h, c)?;
    }
    if let Some(r) = &t.result {
        result_caps(r, c)?;
    }
    for task in &t.tasks {
        task_caps(task, c)?;
    }
    let mut out = Vec::with_capacity(section_len(t.tasks.len())?);
    out.extend([
        C6_PHYS_MAGIC,
        C6_PHYS_VERSION,
        HELPER_SECTION_TAG,
        u(t.tasks.len()),
    ]);
    out.extend(encode_helper_record(t.helper));
    out.extend(encode_result_record(t.result));
    for task in &t.tasks {
        out.extend(encode_task_record(*task));
    }
    Ok(out)
}

/// Decoder; see the module documentation for the exact first-error order.
/// Returns owned tables only, never an image, admission or machine state.
pub fn decode_helper_tables(lanes: &[u128], c: &C6StateCaps) -> C6Result<C6HelperTables> {
    if lanes.len() < HELPER_FIXED_LANES {
        return Err(C6PhysError::Truncated {
            section: SECTION,
            needed: HELPER_FIXED_LANES,
            actual: lanes.len(),
        });
    }
    if lanes[0] != C6_PHYS_MAGIC {
        return Err(C6PhysError::BadMagic { actual: lanes[0] });
    }
    if lanes[1] != C6_PHYS_VERSION {
        return Err(C6PhysError::BadVersion { actual: lanes[1] });
    }
    if lanes[2] != HELPER_SECTION_TAG {
        return Err(C6PhysError::BadTag {
            section: SECTION,
            tag: lanes[2],
        });
    }
    let n = rd(SECTION, lanes[3])?;
    cap_count("state.tasks", n, c.max_helper_tasks)?;
    let needed = section_len(n)?;
    if lanes.len() < needed {
        return Err(C6PhysError::Truncated {
            section: SECTION,
            needed,
            actual: lanes.len(),
        });
    }
    if lanes.len() > needed {
        return Err(C6PhysError::TrailingLanes {
            section: SECTION,
            extra: lanes.len() - needed,
        });
    }
    let truncated = || C6PhysError::Truncated {
        section: SECTION,
        needed,
        actual: lanes.len(),
    };
    let hl: &[u128; HELPER_LANES] = (&lanes[L_HELPER_RECORD..L_RESULT_RECORD])
        .try_into()
        .map_err(|_| truncated())?;
    let helper = decode_helper_record_checked(hl, c)?;
    let rl: &[u128; RESULT_LANES] = (&lanes[L_RESULT_RECORD..L_TASKS])
        .try_into()
        .map_err(|_| truncated())?;
    let result = decode_result_record_checked(rl, c)?;
    let mut tasks = Vec::with_capacity(n);
    for chunk in lanes[L_TASKS..].chunks_exact(TASK_LANES) {
        let tl: &[u128; TASK_LANES] = chunk.try_into().map_err(|_| truncated())?;
        tasks.push(decode_task_record_checked(tl, c)?);
    }
    Ok(C6HelperTables {
        helper,
        result,
        tasks,
    })
}

/// Normal owned-snapshot caller. `caps` is an independent caller assertion
/// (retained original layout/ext/budgets), not bound to `host.admission`.
pub fn helper_lanes_from_host(host: &HostSnapshot, caps: &C6StateCaps) -> C6Result<Vec<u128>> {
    encode_helper_tables(&C6HelperTables::from_image(&host.image), caps)
}

pub fn roundtrip_helper_tables_owned(
    t: &C6HelperTables,
    caps: &C6StateCaps,
) -> C6Result<C6HelperTables> {
    decode_helper_tables(&encode_helper_tables(t, caps)?, caps)
}

/// Diagnostic inverse on a caller-owned host clone: only helper/result/tasks change.
pub fn host_with_decoded_helper_tables(
    host: &HostSnapshot,
    lanes: &[u128],
    caps: &C6StateCaps,
) -> C6Result<HostSnapshot> {
    let tables = decode_helper_tables(lanes, caps)?;
    let mut out = host.clone();
    tables.replace_tables_of(&mut out.image);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sherlock::helper_machine::c6_phys::{C6CodecBudgets, fixtures};
    use crate::sherlock::helper_machine::c6_phys_data::DATA_SECTION_TAG;
    use crate::sherlock::helper_machine::complete_observation::{
        ObservedStep, capture, record_step,
    };
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::owner_ext_source_program::{
        ExtDomain, ExtProgram,
    };
    use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{
        ExtExpr, ExtFunction, ExtParamSrc, ExtValue,
    };
    use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
        CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
    };
    use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;
    use crate::sherlock::helper_machine::shared::{
        Admitted, BoundsC, CoreValue, ExtCapacitiesC, FinanceInput, LayoutCapacitiesC,
        LayoutSchemaC, MachineImage, OwnerEngine, RunIntC, admit_ext_program,
    };
    use HelperControlC as H;

    fn caps() -> C6StateCaps {
        C6StateCaps::test_caps()
    }
    fn exceeded<T>(section: &'static str, needed: usize, limit: usize) -> C6Result<T> {
        Err(C6PhysError::CapacityExceeded {
            section,
            needed,
            limit,
        })
    }
    fn com(site: SiteTag) -> HelperCommon {
        HelperCommon {
            site,
            origin_reference_cursor: u128::MAX,
            parent_resume: Some(3),
        }
    }
    const SITES: [SiteTag; 27] = {
        use SiteTag::*;
        [
            EvalLiteral,
            EvalVar,
            EvalLet,
            EvalBinary,
            EvalUnary,
            EvalIf,
            EvalMatch,
            EvalCall,
            Push,
            RetLet,
            RetLeft,
            RetRight,
            RetUnary,
            RetIf,
            RetMatch,
            RetArgs,
            RetCall,
            RetScope,
            Invoke,
            Enter,
            Bound,
            Cases,
            CaseResult,
            Prim,
            EvalConstruct,
            RetCtorFields,
            Materialize,
        ]
    };

    fn all_helpers() -> Vec<HelperControlC> {
        let c = com(SiteTag::RetCtorFields);
        vec![
            H::Scalar {
                common: c,
                mode: ScalarMode::TestType,
                operand_a: RefC::Data(5),
                operand_b: Some(RefC::Const(0)),
                expected: Some(TypeIdC(6)),
                phase: ScalarPhase::Second,
                reg_a: u128::MAX,
                staged: ScalarStaged::U128Value { n: u128::MAX },
            },
            H::EnvGet {
                common: c,
                binder: 7,
                cursor: Some(0),
                found: Some(RefC::Data(0)),
                status: TriStatus::Positive,
            },
            H::EnvPut {
                common: c,
                binder: 3,
                value: RefC::Const(9),
                base: None,
                new_root: Some(4),
                status: TriStatus::Negative,
            },
            H::Find {
                common: c,
                function_name: 2,
                row_cursor: 5,
                found: None,
                status: TriStatus::Pending,
            },
            H::Bind {
                common: c,
                function: 3,
                params: SliceC { start: 4, len: 2 },
                args: SliceC { start: 9, len: 0 },
                p_cursor: 2,
                a_cursor: 0,
                partial: Some(1),
                dup_cursor: Some(0),
                phase: BindPhase::UnwindDup,
                status: BindStatus::Bad,
            },
            H::Append {
                common: c,
                src: SliceC { start: 6, len: 3 },
                value: RefC::Data(8),
                src_cursor: 3,
                dest_start: 11,
                staged: Some(RefC::Data(2)),
                phase: AppendPhase::Tail,
            },
            H::Validate {
                common: c,
                root: RefC::Const(4),
                status: TriStatus::Negative,
            },
            H::Equal {
                common: c,
                left: RefC::Const(3),
                right: RefC::Data(3),
                status: TriStatus::Positive,
            },
            H::Match {
                common: c,
                scrutinee: RefC::Data(6),
                arm: 5,
                saved_env: Some(2),
                partial: None,
                pat_tag: PatternTag::PBranch,
                x: 9,
                y: 10,
                field_idx: 1,
                staged: None,
                phase: MatchPhase::Field,
                status: TriStatus::Pending,
            },
            H::Plan {
                common: c,
                source_match: 12,
                scrutinee: RefC::Const(7),
                saved_env: None,
                partial: Some(5),
                node: 13,
                phase: PlanPhase::Walk,
                after_walk: PlanAfterWalk::Bind,
                cur: RefC::Data(7),
                path: SliceC { start: 15, len: 2 },
                path_cursor: 1,
                arm: Some(4),
                status: TriStatus::Pending,
            },
            H::Build {
                common: c,
                type_id: TypeIdC(5),
                ctor_id: CtorIdC(6),
                src: SliceC { start: 2, len: 2 },
                src_cursor: 2,
                field_start: 14,
                staged: None,
                node: Some(RefC::Data(9)),
                phase: BuildPhase::Done,
            },
        ]
    }
    fn all_results() -> Vec<HelperResultC> {
        use HelperResultC as R;
        vec![
            R::Validated { ok: true },
            R::Validated { ok: false },
            R::EnvFound {
                value: RefC::Data(4),
            },
            R::EnvMissing,
            R::EnvUpdated { cell: 5 },
            R::FindFound { function: 3 },
            R::FindMissing,
            R::BindOk { env: None },
            R::BindOk { env: Some(0) },
            R::BindBad,
            R::AppendDone {
                seq: SliceC { start: 7, len: 0 },
            },
            R::AppendDone {
                seq: SliceC { start: 3, len: 2 },
            },
            R::ScalarValue {
                value: RefC::Const(2),
            },
            R::ScalarError { code: 255 },
            R::ScalarSelector {
                sel: BoolSelector::NotBool,
            },
            R::ScalarTypeTest { ok: true },
            R::EqualDone {
                value: RefC::Data(1),
            },
            R::MatchMatched { env: Some(6) },
            R::MatchNoMatch,
            R::PlanMatched {
                arm: 5,
                env: Some(2),
            },
            R::PlanNoMatch,
            R::BuildDone {
                value: RefC::Data(9),
            },
        ]
    }
    fn all_tasks() -> Vec<HelperTaskC> {
        vec![
            HelperTaskC::ValidateTask {
                r: RefC::Data(3),
                visit: TaskVisitC::Fields,
                cursor: 1,
            },
            HelperTaskC::ValidateTask {
                r: RefC::Const(0),
                visit: TaskVisitC::Unvisited,
                cursor: 0,
            },
            HelperTaskC::EqualTask {
                left: RefC::Const(3),
                right: RefC::Data(3),
                visit: TaskVisitC::Fields,
                cursor: 2,
            },
            HelperTaskC::PairTask {
                binder: 4,
                sort: TypeIdC(3),
                arg: RefC::Data(2),
            },
            HelperTaskC::PairTask {
                binder: 4,
                sort: TypeIdC(9),
                arg: RefC::Const(2),
            },
        ]
    }

    /// Changing any single lane must refuse or decode to a different value.
    fn every_lane_significant<const N: usize, T: PartialEq>(
        lanes: [u128; N],
        value: &T,
        decode: fn(&[u128; N]) -> C6Result<T>,
    ) {
        for at in 0..N {
            let mut changed = lanes;
            changed[at] ^= 0x55;
            assert!(
                decode(&changed).map_or(true, |v| &v != value),
                "lane {at} was ignored"
            );
        }
    }
    fn rt_helper(h: HelperControlC) {
        let l = encode_helper_record(Some(h));
        assert_eq!(decode_helper_record_checked(&l, &caps()), Ok(Some(h)));
        every_lane_significant(l, &Some(h), decode_helper_record);
    }
    fn map_exact<T: Copy + PartialEq + std::fmt::Debug>(
        all: &[T],
        to: fn(T) -> u128,
        from: fn(&'static str, u128) -> C6Result<T>,
    ) {
        for (i, v) in all.iter().enumerate() {
            assert_eq!(to(*v), i as u128);
            assert_eq!(from("t", i as u128), Ok(*v));
        }
        let n = all.len() as u128;
        assert_eq!(
            from("t", n),
            Err(C6PhysError::BadTag {
                section: "t",
                tag: n
            })
        );
    }

    #[test]
    fn literal_enum_maps_are_exhaustive_and_declaration_ordered() {
        map_exact(&SITES, site_tag, site_from);
        {
            use ScalarMode::*;
            map_exact(
                &[
                    Add, Sub, Mul, Lt, Cons, Branch, Leaf, Not, TestBool, TestType,
                ],
                mode_tag,
                mode_from,
            );
        }
        {
            use ScalarPhase::*;
            map_exact(&[First, Second, Done], sphase_tag, sphase_from);
        }
        {
            use NodeKind::*;
            map_exact(&[Cons, Leaf, Branch], node_tag, node_from);
        }
        {
            use BoolSelector::*;
            map_exact(&[True, False, NotBool], sel_tag, sel_from);
        }
        {
            use TriStatus::*;
            map_exact(&[Pending, Positive, Negative], tri_tag, tri_from);
        }
        {
            use BindPhase::*;
            map_exact(
                &[
                    Pairing,
                    UnwindType,
                    UnwindDup,
                    UnwindInsert,
                    Propagate,
                    Finished,
                ],
                bphase_tag,
                bphase_from,
            );
        }
        map_exact(
            &[BindStatus::Ok, BindStatus::Bad],
            bstatus_tag,
            bstatus_from,
        );
        {
            use AppendPhase::*;
            map_exact(&[Copy, Tail, Done], aphase_tag, aphase_from);
        }
        {
            use MatchPhase::*;
            map_exact(
                &[Arm, Tag, Distinct, Field, Bind, Ready, NoMatchReady, Done],
                mphase_tag,
                mphase_from,
            );
        }
        {
            use PlanPhase::*;
            map_exact(
                &[Walk, Test, Bind, Ready, NoMatchReady, Done],
                pphase_tag,
                pphase_from,
            );
        }
        map_exact(
            &[PlanAfterWalk::Test, PlanAfterWalk::Bind],
            walk_tag,
            walk_from,
        );
        map_exact(&[BuildPhase::Copy, BuildPhase::Done], build_tag, build_from);
        {
            use PatternTag::*;
            map_exact(
                &[
                    PVar,
                    PNil,
                    PCons,
                    PLeaf,
                    PBranch,
                    WildcardExtensionReserved,
                    ConstructorExtensionReserved,
                ],
                pat_tag_of,
                pat_from,
            );
        }
        map_exact(
            &[TaskVisitC::Unvisited, TaskVisitC::Fields],
            visit_tag,
            visit_from,
        );
    }

    #[test]
    fn all_variants_phases_staged_and_options_roundtrip_exactly() {
        // SYNTHETIC codec vectors over every variant; not admitted images.
        let helpers = all_helpers();
        let kinds: std::collections::BTreeSet<u128> = helpers
            .iter()
            .map(|h| encode_helper_record(Some(*h))[0])
            .collect();
        assert_eq!(kinds.len(), 11);
        for h in &helpers {
            rt_helper(*h);
        }
        every_lane_significant(encode_helper_record(None), &None, decode_helper_record);
        for site in SITES {
            rt_helper(H::Validate {
                common: com(site),
                root: RefC::Data(1),
                status: TriStatus::Pending,
            });
        }
        for resume in [None, Some(0), Some(7)] {
            rt_helper(H::Validate {
                common: HelperCommon {
                    parent_resume: resume,
                    ..com(SiteTag::Prim)
                },
                root: RefC::Const(1),
                status: TriStatus::Positive,
            });
        }
        let H::Scalar {
            common,
            operand_a,
            reg_a,
            ..
        } = helpers[0]
        else {
            panic!()
        };
        {
            use ScalarStaged as S;
            let staged = [
                S::None,
                S::U128Value { n: u128::MAX },
                S::BoolValue { b: true },
                S::BoolValue { b: false },
                S::PendingNode {
                    tag: NodeKind::Cons,
                },
                S::PendingNode {
                    tag: NodeKind::Leaf,
                },
                S::PendingNode {
                    tag: NodeKind::Branch,
                },
                S::PrimError { code: 10 },
                S::PrimError { code: 255 },
                S::Selector {
                    sel: BoolSelector::True,
                },
                S::Selector {
                    sel: BoolSelector::False,
                },
                S::Selector {
                    sel: BoolSelector::NotBool,
                },
                S::TypeTest { ok: true },
                S::TypeTest { ok: false },
            ];
            use ScalarMode::*;
            for (k, mode) in [
                Add, Sub, Mul, Lt, Cons, Branch, Leaf, Not, TestBool, TestType,
            ]
            .into_iter()
            .enumerate()
            {
                for phase in [ScalarPhase::First, ScalarPhase::Second, ScalarPhase::Done] {
                    for operand_b in [None, Some(RefC::Const(0)), Some(RefC::Data(0))] {
                        for expected in [None, Some(TypeIdC(0)), Some(TypeIdC(3))] {
                            rt_helper(H::Scalar {
                                common,
                                mode,
                                operand_a,
                                operand_b,
                                expected,
                                phase,
                                reg_a,
                                staged: staged[k % staged.len()],
                            });
                        }
                    }
                }
            }
            for s in staged {
                rt_helper(H::Scalar {
                    common,
                    mode: TestType,
                    operand_a,
                    operand_b: None,
                    expected: None,
                    phase: ScalarPhase::Done,
                    reg_a,
                    staged: s,
                });
            }
        }
        let H::Bind {
            common,
            function,
            params,
            args,
            p_cursor,
            a_cursor,
            partial,
            dup_cursor,
            ..
        } = helpers[4]
        else {
            panic!()
        };
        {
            use BindPhase::*;
            for phase in [
                Pairing,
                UnwindType,
                UnwindDup,
                UnwindInsert,
                Propagate,
                Finished,
            ] {
                for status in [BindStatus::Ok, BindStatus::Bad] {
                    rt_helper(H::Bind {
                        common,
                        function,
                        params,
                        args,
                        p_cursor,
                        a_cursor,
                        partial,
                        dup_cursor,
                        phase,
                        status,
                    });
                }
            }
        }
        for phase in [AppendPhase::Copy, AppendPhase::Tail, AppendPhase::Done] {
            let H::Append {
                common,
                src,
                value,
                src_cursor,
                dest_start,
                staged,
                ..
            } = helpers[5]
            else {
                panic!()
            };
            rt_helper(H::Append {
                common,
                src,
                value,
                src_cursor,
                dest_start,
                staged,
                phase,
            });
        }
        {
            use MatchPhase::*;
            use PatternTag::*;
            for phase in [Arm, Tag, Distinct, Field, Bind, Ready, NoMatchReady, Done] {
                for pat_tag in [
                    PVar,
                    PNil,
                    PCons,
                    PLeaf,
                    PBranch,
                    WildcardExtensionReserved,
                    ConstructorExtensionReserved,
                ] {
                    let H::Match {
                        common,
                        scrutinee,
                        arm,
                        saved_env,
                        partial,
                        x,
                        y,
                        field_idx,
                        staged,
                        status,
                        ..
                    } = helpers[8]
                    else {
                        panic!()
                    };
                    rt_helper(H::Match {
                        common,
                        scrutinee,
                        arm,
                        saved_env,
                        partial,
                        pat_tag,
                        x,
                        y,
                        field_idx,
                        staged,
                        phase,
                        status,
                    });
                }
            }
        }
        {
            use PlanPhase::*;
            for phase in [Walk, Test, Bind, Ready, NoMatchReady, Done] {
                for after_walk in [PlanAfterWalk::Test, PlanAfterWalk::Bind] {
                    let H::Plan {
                        common,
                        source_match,
                        scrutinee,
                        saved_env,
                        partial,
                        node,
                        cur,
                        path,
                        path_cursor,
                        arm,
                        status,
                        ..
                    } = helpers[9]
                    else {
                        panic!()
                    };
                    rt_helper(H::Plan {
                        common,
                        source_match,
                        scrutinee,
                        saved_env,
                        partial,
                        node,
                        phase,
                        after_walk,
                        cur,
                        path,
                        path_cursor,
                        arm,
                        status,
                    });
                }
            }
        }
        let H::Build {
            common,
            type_id,
            ctor_id,
            src,
            src_cursor,
            field_start,
            staged,
            node,
            ..
        } = helpers[10]
        else {
            panic!()
        };
        rt_helper(H::Build {
            common,
            type_id,
            ctor_id,
            src,
            src_cursor,
            field_start,
            staged,
            node,
            phase: BuildPhase::Copy,
        });
        let results = all_results();
        let tags: std::collections::BTreeSet<u128> = results
            .iter()
            .map(|r| encode_result_record(Some(*r))[0])
            .collect();
        assert_eq!(tags.len(), 19);
        for r in results.iter().copied().map(Some).chain([None]) {
            let l = encode_result_record(r);
            assert_eq!(decode_result_record_checked(&l, &caps()), Ok(r));
            every_lane_significant(l, &r, decode_result_record);
        }
        for t in all_tasks() {
            let l = encode_task_record(t);
            assert_eq!(decode_task_record_checked(&l, &caps()), Ok(t));
            every_lane_significant(l, &t, decode_task_record);
        }
        // Distinctions that must survive: Const/Data, None/Some(0), raw empty start, u128 MAX.
        let opt = |found| {
            encode_helper_record(Some(H::EnvGet {
                common: com(SiteTag::EvalVar),
                binder: 0,
                cursor: None,
                found,
                status: TriStatus::Pending,
            }))
        };
        let encs = [
            opt(None),
            opt(Some(RefC::Const(0))),
            opt(Some(RefC::Data(0))),
        ];
        assert!(encs[0] != encs[1] && encs[1] != encs[2] && encs[0] != encs[2]);
        assert_ne!(
            encode_result_record(Some(HelperResultC::BindOk { env: None })),
            encode_result_record(Some(HelperResultC::BindOk { env: Some(0) }))
        );
        assert_ne!(
            encode_result_record(Some(HelperResultC::AppendDone {
                seq: SliceC { start: 7, len: 0 }
            })),
            encode_result_record(Some(HelperResultC::AppendDone {
                seq: SliceC::empty()
            }))
        );
    }

    #[test]
    fn declared_lane_maps_header_and_section_inverse() {
        let helpers = all_helpers();
        assert_eq!(
            encode_helper_record(Some(helpers[10])),
            [
                11,
                25,
                u128::MAX,
                1,
                3,
                5,
                6,
                2,
                2,
                2,
                14,
                0,
                0,
                0,
                1,
                1,
                9,
                1,
                0,
                0,
                0,
                0,
                0
            ]
        );
        assert_eq!(
            encode_helper_record(Some(helpers[6]))[..9],
            [7, 25, u128::MAX, 1, 3, 0, 4, 2, 0]
        );
        assert_eq!(
            encode_result_record(Some(HelperResultC::PlanMatched {
                arm: 5,
                env: Some(2)
            })),
            [17, 5, 1, 2]
        );
        assert_eq!(encode_task_record(all_tasks()[2]), [1, 0, 3, 1, 3, 1, 2]);
        assert_eq!(encode_task_record(all_tasks()[3]), [2, 4, 3, 1, 2, 0, 0]);
        let t = C6HelperTables {
            helper: Some(helpers[10]),
            result: Some(HelperResultC::PlanMatched {
                arm: 5,
                env: Some(2),
            }),
            tasks: all_tasks(),
        };
        let lanes = encode_helper_tables(&t, &caps()).unwrap();
        assert_eq!(
            lanes[..4],
            [C6_PHYS_MAGIC, C6_PHYS_VERSION, HELPER_SECTION_TAG, 5]
        );
        assert_eq!(lanes.len(), HELPER_FIXED_LANES + 5 * TASK_LANES);
        assert_eq!(
            lanes[L_HELPER_RECORD..L_RESULT_RECORD],
            encode_helper_record(t.helper)
        );
        assert_eq!(
            lanes[L_RESULT_RECORD..L_TASKS],
            encode_result_record(t.result)
        );
        assert_eq!(
            lanes[L_TASKS + 2 * TASK_LANES..L_TASKS + 3 * TASK_LANES],
            encode_task_record(t.tasks[2])
        );
        let decoded = decode_helper_tables(&lanes, &caps()).unwrap();
        assert_eq!(decoded, t);
        assert_eq!(encode_helper_tables(&decoded, &caps()).unwrap(), lanes);
        assert_eq!(roundtrip_helper_tables_owned(&t, &caps()).unwrap(), t);
        let differs =
            |m: C6HelperTables| assert_ne!(encode_helper_tables(&m, &caps()).unwrap(), lanes);
        let mut m = t.clone();
        m.tasks.swap(0, 1);
        differs(m);
        let mut m = t.clone();
        m.tasks.push(m.tasks[0]);
        differs(m);
        let mut m = t.clone();
        m.result = None;
        differs(m);
        let mut m = t.clone();
        m.helper = None;
        differs(m);
        let empty = C6HelperTables {
            helper: None,
            result: None,
            tasks: Vec::new(),
        };
        assert_eq!(
            roundtrip_helper_tables_owned(&empty, &caps()).unwrap(),
            empty
        );
    }

    #[test]
    fn refusals_follow_the_declared_first_error_order() {
        let t = C6HelperTables {
            helper: Some(all_helpers()[1]),
            result: Some(HelperResultC::Validated { ok: true }),
            tasks: vec![HelperTaskC::ValidateTask {
                r: RefC::Data(2),
                visit: TaskVisitC::Fields,
                cursor: 1,
            }],
        };
        let lanes = encode_helper_tables(&t, &caps()).unwrap();
        assert_eq!(lanes.len(), 38);
        let dec = |l: &[u128]| decode_helper_tables(l, &caps());
        let set = |pairs: &[(usize, u128)]| {
            let mut m = lanes.clone();
            for &(k, v) in pairs {
                m[k] = v;
            }
            m
        };
        assert_eq!(
            dec(&lanes[..30]),
            Err(C6PhysError::Truncated {
                section: SECTION,
                needed: 31,
                actual: 30
            })
        );
        assert_eq!(
            dec(&set(&[(0, 0), (2, 0)])),
            Err(C6PhysError::BadMagic { actual: 0 })
        );
        assert_eq!(
            dec(&set(&[(1, 2)])),
            Err(C6PhysError::BadVersion { actual: 2 })
        );
        assert_eq!(
            dec(&set(&[(2, DATA_SECTION_TAG)])),
            Err(C6PhysError::BadTag {
                section: SECTION,
                tag: DATA_SECTION_TAG
            })
        );
        assert_eq!(
            dec(&set(&[(3, u128::MAX)])),
            Err(C6PhysError::UsizeOverflow {
                section: SECTION,
                value: u128::MAX
            })
        );
        assert_eq!(dec(&set(&[(3, 65)])), exceeded("state.tasks", 65, 64));
        assert_eq!(
            dec(&set(&[(3, 2)])),
            Err(C6PhysError::Truncated {
                section: SECTION,
                needed: 45,
                actual: 38
            })
        );
        let mut trailing = lanes.clone();
        trailing.push(0);
        assert_eq!(
            dec(&trailing),
            Err(C6PhysError::TrailingLanes {
                section: SECTION,
                extra: 1
            })
        );
        assert_eq!(
            dec(&set(&[(4, 12)])),
            Err(C6PhysError::BadTag {
                section: HELPER_S,
                tag: 12
            })
        );
        assert_eq!(
            dec(&set(&[(12, 2)])),
            Err(C6PhysError::BadTag {
                section: HELPER_S,
                tag: 2
            })
        );
        assert_eq!(
            dec(&set(&[(12, 0)])),
            Err(C6PhysError::NonZeroPadding {
                section: HELPER_S,
                lane: 9,
                value: 1
            })
        );
        assert_eq!(
            dec(&set(&[(16, 1)])),
            Err(C6PhysError::NonZeroPadding {
                section: HELPER_S,
                lane: 12,
                value: 1
            })
        );
        // Structure precedes caps; the helper record precedes the result record.
        assert_eq!(
            dec(&set(&[(9, 999), (15, 7)])),
            Err(C6PhysError::BadTag {
                section: HELPER_S,
                tag: 7
            })
        );
        assert_eq!(
            dec(&set(&[(9, 256), (28, 2)])),
            exceeded("state.helper.binder", 257, 256)
        );
        assert_eq!(
            dec(&set(&[(28, 2)])),
            Err(C6PhysError::BadBool {
                section: RESULT_S,
                value: 2
            })
        );
        assert_eq!(
            dec(&set(&[(34, 2)])),
            Err(C6PhysError::BadTag {
                section: TASK_S,
                tag: 2
            })
        );
        assert_eq!(dec(&set(&[(35, 9)])), exceeded("state.task.cursor", 9, 8));
        // u8 lanes refuse overflow; no truncation.
        let mut l = encode_result_record(Some(HelperResultC::ScalarError { code: 255 }));
        l[1] = 256;
        assert_eq!(
            decode_result_record(&l),
            Err(C6PhysError::UsizeOverflow {
                section: RESULT_S,
                value: 256
            })
        );
        let mut l = encode_helper_record(Some(all_helpers()[0]));
        l[16] = 0;
        l[15] = 4;
        l[16] = 256;
        assert_eq!(
            decode_helper_record(&l),
            Err(C6PhysError::UsizeOverflow {
                section: HELPER_S,
                value: 256
            })
        );
        l[15] = 0;
        l[16] = 1;
        assert_eq!(
            decode_helper_record(&l),
            Err(C6PhysError::NonZeroPadding {
                section: HELPER_S,
                lane: 16,
                value: 1
            })
        );
        let mut l = encode_helper_record(Some(all_helpers()[8]));
        l[15] = 256;
        assert_eq!(
            decode_helper_record(&l),
            Err(C6PhysError::UsizeOverflow {
                section: HELPER_S,
                value: 256
            })
        );
        // The encoder applies the same caps.
        let mut tight = caps();
        tight.max_helper_tasks = 0;
        assert_eq!(
            encode_helper_tables(&t, &tight),
            exceeded("state.tasks", 1, 0)
        );
        assert_eq!(
            decode_helper_tables(&lanes, &tight).map(|_| ()),
            exceeded("state.tasks", 1, 0)
        );
    }

    #[test]
    fn field_specific_caps_cursors_and_type_policies() {
        let chk = |h: HelperControlC, c: &C6StateCaps| helper_caps(&h, c);
        let mut c = caps();
        let base = com(SiteTag::Invoke);
        let env_get = |binder| H::EnvGet {
            common: base,
            binder,
            cursor: None,
            found: None,
            status: TriStatus::Pending,
        };
        assert!(chk(env_get(255), &c).is_ok());
        assert_eq!(
            chk(env_get(256), &c),
            exceeded("state.helper.binder", 257, 256)
        );
        let find = |row_cursor, found| H::Find {
            common: base,
            function_name: 0,
            row_cursor,
            found,
            status: TriStatus::Pending,
        };
        assert!(
            chk(find(64, None), &c).is_ok(),
            "one-past-end row cursor (FindEmpty)"
        );
        assert_eq!(
            chk(find(65, None), &c),
            exceeded("state.helper.row_cursor", 65, 64)
        );
        assert_eq!(
            chk(find(0, Some(64)), &c),
            exceeded("state.helper.found", 65, 64)
        );
        // Non-environment options use their own table, not max_env_cells.
        c.phys.max_env_cells = 0;
        assert!(chk(find(0, Some(5)), &c).is_ok());
        assert_eq!(
            chk(
                H::EnvGet {
                    common: base,
                    binder: 0,
                    cursor: Some(0),
                    found: None,
                    status: TriStatus::Pending
                },
                &c
            ),
            exceeded("state.helper.cursor", 1, 0)
        );
        let plan = |arm, path| H::Plan {
            common: base,
            source_match: 0,
            scrutinee: RefC::Const(0),
            saved_env: None,
            partial: None,
            node: 0,
            phase: PlanPhase::Ready,
            after_walk: PlanAfterWalk::Test,
            cur: RefC::Const(0),
            path,
            path_cursor: 0,
            arm,
            status: TriStatus::Pending,
        };
        assert!(
            chk(plan(Some(63), SliceC { start: 0, len: 100 }), &c).is_ok(),
            "field paths carry no arity cap"
        );
        assert_eq!(
            chk(plan(Some(64), SliceC::empty()), &c),
            exceeded("state.helper.arm", 65, 64)
        );
        let resume = HelperCommon {
            parent_resume: Some(usize::MAX),
            ..base
        };
        assert!(
            chk(
                H::Validate {
                    common: resume,
                    root: RefC::Const(0),
                    status: TriStatus::Pending
                },
                &c
            )
            .is_ok(),
            "reserved boundary identity is representation only"
        );
        let append = |src_cursor| H::Append {
            common: base,
            src: SliceC { start: 0, len: 3 },
            value: RefC::Const(0),
            src_cursor,
            dest_start: 1024,
            staged: None,
            phase: AppendPhase::Copy,
        };
        assert!(chk(append(3), &c).is_ok());
        assert_eq!(
            chk(append(4), &c),
            exceeded("state.helper.src_cursor", 4, 3)
        );
        let bind = |args| H::Bind {
            common: base,
            function: 0,
            params: SliceC::empty(),
            args,
            p_cursor: 0,
            a_cursor: 0,
            partial: None,
            dup_cursor: None,
            phase: BindPhase::Pairing,
            status: BindStatus::Ok,
        };
        assert_eq!(
            chk(
                bind(SliceC {
                    start: usize::MAX,
                    len: 1
                }),
                &c
            ),
            Err(C6PhysError::UsizeOverflow {
                section: "state.helper.args",
                value: usize::MAX as u128
            })
        );
        assert_eq!(
            chk(bind(SliceC { start: 0, len: 9 }), &c),
            exceeded("state.helper.args", 9, 8)
        );
        assert_eq!(
            chk(
                bind(SliceC {
                    start: 1020,
                    len: 5
                }),
                &c
            ),
            exceeded("state.helper.args", 1025, 1024)
        );
        // Match placeholders are raw before the Arm read; bound names afterwards.
        c.phys.max_names = 0;
        let mat = |phase| H::Match {
            common: base,
            scrutinee: RefC::Const(0),
            arm: 0,
            saved_env: None,
            partial: None,
            pat_tag: PatternTag::PCons,
            x: 0,
            y: 0,
            field_idx: 0,
            staged: None,
            phase,
            status: TriStatus::Pending,
        };
        assert!(chk(mat(MatchPhase::Arm), &c).is_ok());
        assert_eq!(
            chk(mat(MatchPhase::Field), &c),
            exceeded("state.helper.x", 1, 0)
        );
        // Builtin-capable TypeIds use max(max_types, 4); user Build type_id does not.
        let mut c = caps();
        c.phys.max_types = 0;
        let scalar = |e| H::Scalar {
            common: base,
            mode: ScalarMode::TestType,
            operand_a: RefC::Const(0),
            operand_b: None,
            expected: Some(TypeIdC(e)),
            phase: ScalarPhase::First,
            reg_a: 0,
            staged: ScalarStaged::None,
        };
        assert!(chk(scalar(3), &c).is_ok());
        assert_eq!(chk(scalar(4), &c), exceeded("state.helper.expected", 5, 4));
        let pair = |s| {
            task_caps(
                &HelperTaskC::PairTask {
                    binder: 0,
                    sort: TypeIdC(s),
                    arg: RefC::Const(0),
                },
                &c,
            )
        };
        assert!(pair(3).is_ok());
        assert_eq!(pair(4), exceeded("state.task.sort", 5, 4));
        let build = H::Build {
            common: base,
            type_id: TypeIdC(0),
            ctor_id: CtorIdC(0),
            src: SliceC::empty(),
            src_cursor: 0,
            field_start: 0,
            staged: None,
            node: None,
            phase: BuildPhase::Copy,
        };
        assert_eq!(chk(build, &c), exceeded("state.helper.type_id", 1, 0));
    }

    /// Local fixture retaining its own ORIGINAL admitting layout.
    fn admitted_with_layout() -> (Admitted, MachineImage, LayoutSchemaC) {
        let bounds = BoundsC {
            code_rows: 128,
            function_rows: 16,
            code_child_slots: 128,
            parameter_slots: 32,
            pattern_rows: 32,
            match_plan_rows: 32,
            match_arm_rows: 32,
            type_rows: 16,
            constructor_rows: 16,
            constructor_field_slots: 32,
            max_arity: 8,
            constant_nodes: 128,
            constant_field_slots: 128,
            data_nodes: 128,
            data_field_slots: 128,
            environment_cells: 128,
            frames: 64,
            helper_tasks: 64,
            serialization_bytes: 1_000_000,
            max_reference_work: 1_000_000,
            max_helper_work: 1_000_000,
            step_cap: 10_000,
        };
        let layout = LayoutSchemaC {
            layout_version: ("c6-phys-helper-test".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds,
            capacities: LayoutCapacitiesC {
                name_rows: 128,
                name_bytes: 4096,
                occurrence_path_slots: 4096,
                value_slots: 4096,
            },
            ext_capacities: Some(ExtCapacitiesC {
                plan_case_slots: 64,
                field_path_slots: 64,
                pattern_child_slots: 64,
                ..ExtCapacitiesC::all_zero()
            }),
            boundary_capacities_present: false,
        };
        let boxed = || TypeExprSrc::Named { name: "Box".into() };
        let var = |name: &str| ExtExpr::Var { name: name.into() };
        let lit = ExtExpr::Literal {
            value: ExtValue::Core {
                value: CoreValue::UInt {
                    n: KIntC::InRange(7),
                },
            },
        };
        let mk = ExtFunction {
            name: "mk".into(),
            parameters: vec![ExtParamSrc {
                name: "x".into(),
                ty: TypeExprSrc::UInt,
            }],
            return_type: boxed(),
            body: ExtExpr::Construct {
                ty: boxed(),
                ctor: "MkBox".into(),
                fields: vec![var("x")],
            },
        };
        // `type Box = MkBox { item: UInt }`, `fn mk(x: UInt) -> Box`, entry `let p = mk(7) in p`.
        let program = ExtProgram {
            types: vec![TypeDeclSrc {
                name: "Box".into(),
                parameters: Vec::new(),
                ctors: vec![CtorDeclSrc {
                    name: "MkBox".into(),
                    fields: vec![CtorFieldSrc {
                        name: "item".into(),
                        ty: TypeExprSrc::UInt,
                    }],
                }],
            }],
            assets: Vec::new(),
            states: Vec::new(),
            inputs: Vec::new(),
            functions: vec![mk],
            entry: ExtExpr::Let {
                name: "p".into(),
                value: Box::new(ExtExpr::Call {
                    name: "mk".into(),
                    args: vec![lit],
                }),
                body: Box::new(var("p")),
            },
        };
        let (admitted, _report) = fixtures::must(
            admit_ext_program(program, ExtDomain::GENERIC_ABSENT, &bounds, &layout, None),
            "generic nominal admission",
        );
        let image = fixtures::must(
            host_owner_engine().admit_run(
                &admitted,
                &RunIntC::Nat(fixtures::FUEL),
                &RunIntC::Nat(64),
                &FinanceInput::Absent,
            ),
            "admit run",
        );
        (admitted, image, layout)
    }

    fn bump_origin(h: &mut HelperControlC) {
        match h {
            H::Scalar { common, .. }
            | H::EnvGet { common, .. }
            | H::EnvPut { common, .. }
            | H::Find { common, .. }
            | H::Bind { common, .. }
            | H::Append { common, .. }
            | H::Validate { common, .. }
            | H::Equal { common, .. }
            | H::Match { common, .. }
            | H::Plan { common, .. }
            | H::Build { common, .. } => common.origin_reference_cursor ^= 1,
        }
    }

    #[test]
    fn owner_reachable_helper_states_roundtrip_and_clone_replace_whole_host() {
        // OWNER-REACHABLE: real generic admission, real fixed-owner steps; caps from the retained layout.
        let (admitted, mut image, layout) = admitted_with_layout();
        let caps = C6StateCaps::from_layout(
            &layout,
            C6CodecBudgets {
                type_expr_nodes: 0,
                template_bytes: 0,
            },
        );
        let (mut seen_helper, mut seen_task) = ([false; 11], [false; 3]);
        let (mut expected_some, mut results, mut mutated, mut terminal) =
            (false, 0usize, false, false);
        for _ in 0..4096 {
            let host = fixtures::must(capture(&admitted, &image), "capture");
            let lanes = helper_lanes_from_host(&host, &caps).unwrap();
            let decoded = decode_helper_tables(&lanes, &caps).unwrap();
            assert_eq!(decoded, C6HelperTables::from_image(&host.image));
            assert_eq!(encode_helper_tables(&decoded, &caps).unwrap(), lanes);
            assert_eq!(
                host_with_decoded_helper_tables(&host, &lanes, &caps).unwrap(),
                host
            );
            assert_eq!(
                lanes[3],
                host.image.tasks.len() as u128,
                "task cursor derived from the count only"
            );
            if let Some(h) = host.image.helper {
                seen_helper[encode_helper_record(Some(h))[0] as usize - 1] = true;
                expected_some |= matches!(
                    h,
                    H::Scalar {
                        expected: Some(_),
                        ..
                    }
                );
            }
            for task in &host.image.tasks {
                seen_task[encode_task_record(*task)[0] as usize] = true;
            }
            if host.image.result.is_some() {
                results += 1;
            }
            if !mutated && host.image.helper.is_some() && !host.image.tasks.is_empty() {
                mutated = true;
                let differs =
                    |m: &HostSnapshot| assert_ne!(helper_lanes_from_host(m, &caps).unwrap(), lanes);
                let mut m = host.clone();
                bump_origin(m.image.helper.as_mut().unwrap());
                differs(&m);
                let mut m = host.clone();
                m.image.tasks.push(m.image.tasks[0]);
                differs(&m);
                let mut m = host.clone();
                m.image.result = Some(HelperResultC::PlanNoMatch);
                differs(&m);
                let mut m = host.clone();
                m.image.frames.clear();
                m.image.data.clear();
                m.image.value_slots.clear();
                m.image.reference_cursor ^= 1;
                m.image.control_origin ^= 1;
                assert_eq!(
                    helper_lanes_from_host(&m, &caps).unwrap(),
                    lanes,
                    "other state is not in this section"
                );
                let mut m = host.clone();
                C6HelperTables {
                    helper: None,
                    result: None,
                    tasks: Vec::new(),
                }
                .replace_tables_of(&mut m.image);
                assert_ne!(m, host);
                decoded.clone().replace_tables_of(&mut m.image);
                assert_eq!(m, host, "only helper/result/tasks are replaced");
            }
            match record_step(&admitted, &mut image) {
                Ok(ObservedStep::Event(_)) => {}
                Ok(ObservedStep::Terminal { .. }) => {
                    terminal = true;
                    break;
                }
                Err(_) => panic!("owner step"),
            }
        }
        assert!(terminal && mutated && expected_some && results > 0);
        for (k, name) in [
            (0, "Scalar"),
            (1, "EnvGet"),
            (2, "EnvPut"),
            (3, "Find"),
            (4, "Bind"),
            (5, "Append"),
            (6, "Validate"),
            (10, "Build"),
        ] {
            assert!(seen_helper[k], "{name} helper is owner-reachable here");
        }
        assert!(
            seen_task[0] && seen_task[2],
            "ValidateTask and PairTask are owner-reachable here"
        );
    }
}
