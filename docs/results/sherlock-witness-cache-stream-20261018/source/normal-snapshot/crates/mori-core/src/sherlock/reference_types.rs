use std::collections::BTreeMap;
use std::sync::Arc;

pub const PROFILE: &str = "sherlock-k-a/1";
pub const K_SOURCE_SHA256: &str =
    "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41";

/// Opaque source symbol. K comparisons use canonical signed-decimal names.
/// Other spellings belong to the typed host API, outside that common domain.
pub type Name = String;
pub type Environment = BTreeMap<Name, Value>;
pub type ExprRef = Arc<Expr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tree {
    Leaf(u128),
    Branch(Arc<Tree>, Arc<Tree>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    UInt(u128),
    Bool(bool),
    List(Vec<u128>),
    Tree(Tree),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    UInt,
    Bool,
    List,
    Tree,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Lt,
    Cons,
    Branch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Leaf,
    Not,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    Var(Name),
    Nil,
    Cons(Name, Name),
    Leaf(Name),
    Branch(Name, Name),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm {
    pub pattern: Pattern,
    pub body: ExprRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Value),
    Var(Name),
    Let(Name, ExprRef, ExprRef),
    Binary(BinaryOp, ExprRef, ExprRef),
    Unary(UnaryOp, ExprRef),
    If(ExprRef, ExprRef, ExprRef),
    Call(Name, Vec<ExprRef>),
    Match(ExprRef, Vec<Arm>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Name,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Name,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: ExprRef,
}

/// Function order is significant: lookup takes the first matching name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub functions: Vec<Function>,
    pub entry: ExprRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Frame {
    Let(Name, ExprRef, Environment),
    Left(BinaryOp, ExprRef, Environment),
    Right(BinaryOp, Value),
    Unary(UnaryOp),
    If(ExprRef, ExprRef, Environment),
    Match(Vec<Arm>, Environment),
    Args(Name, Vec<Value>, Vec<ExprRef>, Environment),
    Return(Type, Environment),
    Scope(Environment),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Control {
    Eval(ExprRef),
    Return(Value),
    Push(Frame, Box<Control>, Environment),
    Invoke(Name, Vec<Value>),
    Enter(Option<Function>, Vec<Value>),
    Bound(Option<Environment>, Type, ExprRef),
    Cases(Value, Vec<Arm>, Environment),
    CaseResult(Option<Environment>, Value, ExprRef, Vec<Arm>, Environment),
    PrimitiveResult(Result<Value, u8>),
    Error(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Pending,
    Done(Value),
    Rejected(u8),
    Exhausted,
}

impl Outcome {
    /// Host-diagnostic classifier for the frozen `sherlock-k-a/1` A1 resource-limit partition.
    ///
    /// Returns `true` iff `self` is [`Outcome::Exhausted`] (zero-fuel termination) or
    /// [`Outcome::Rejected(4)`] (K-A frame-cap overflow at `D >=Int L`). All other outcomes—
    /// including [`Outcome::Pending`], [`Outcome::Done`], and all other [`Outcome::Rejected`]
    /// codes (such as semantic, typing, binding, pattern, or arithmetic errors `3, 5..=11` and
    /// unreachable literal code `2`)—return `false`.
    ///
    /// Scope: host diagnostic only on the `sherlock-k-a/1` typed common domain. This predicate
    /// carries zero native or cryptographic authority, does not transfer across Quint
    /// (`sherlock-quint-a/1`) or Candidate B (`sherlock-helper-microstep/1`), and does not make
    /// the unmetered host evaluator safe for untrusted inputs. No consumer may interpret a
    /// resource-limit outcome as a semantic, typing, or financial rejection.
    pub const fn is_resource_limit(&self) -> bool {
        matches!(self, Self::Exhausted | Self::Rejected(4))
    }
}

/// Complete post-startup K boundary state on the typed host common domain.
/// Frames are stored top first. `None` control represents K's `.K`.
/// Snapshots are observations; no arbitrary-state import is exposed.
///
/// Machine invariant: every `Snapshot` produced by [`super::reference::Machine`] satisfies
/// `remaining_fuel <= initial_fuel`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub program: Arc<Program>,
    pub control: Option<Control>,
    pub environment: Environment,
    pub frames: Vec<Frame>,
    pub stack_limit: usize,
    pub remaining_fuel: u128,
    pub initial_fuel: u128,
    pub outcome: Outcome,
}

impl Snapshot {
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// Returns `initial_fuel - remaining_fuel` for machine-valid observations.
    ///
    /// # Preconditions
    /// Requires the machine invariant `self.remaining_fuel <= self.initial_fuel`.
    /// For hand-constructed snapshots where this invariant is not guaranteed, use
    /// [`Snapshot::checked_charged_steps`] so invalid counters are reported as `None`
    /// rather than silently clamped, wrapped, or panicked.
    pub fn charged_steps(&self) -> u128 {
        self.initial_fuel - self.remaining_fuel
    }

    /// Checked host-diagnostic step counter returning `None` if `remaining_fuel > initial_fuel`.
    pub const fn checked_charged_steps(&self) -> Option<u128> {
        self.initial_fuel.checked_sub(self.remaining_fuel)
    }
}

/// A source rule transition, including free terminal publication.
/// Line numbers refer only to `K_SOURCE_SHA256`, not to executed K rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepRecord {
    pub rule_start_line: usize,
    pub charge: u8,
    pub before: Snapshot,
    pub after: Snapshot,
}
