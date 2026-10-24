//! Witness-mutation soundness harness (T1 oracle of the DoctorWatson factory).
//!
//! Idea (ZK circuit fuzzing, arXiv 2504.14881 and ZKFuzz arXiv 2504.11961):
//! take a circuit whose valid witness `MockProver` accepts, perturb ONE
//! assigned advice cell, and require `MockProver` to reject. A mutated
//! witness that is still accepted means that cell is under-constrained.
//!
//! # Mechanism (and why)
//!
//! The pinned halo2 fork is `midnight-proofs 0.8.3` (with the
//! `circuit-params` feature enabled by `midnight-zk-stdlib`). Its
//! `MockProver` keeps its advice table private and offers only read access
//! (`advice()`), so a finished prover cannot be edited. The harness instead
//! intercepts the witness while it is being assigned:
//!
//! * [`Mutated`] is a wrapper `Circuit` that delegates `configure`,
//!   `params` and `synthesize` to the inner circuit unchanged.
//! * Its `FloorPlanner` is [`MutPlanner`], which runs the inner circuit's own
//!   floor planner on a [`MutAssign`] wrapper around the real `Assignment`
//!   (`MockProver`, or its `RowSizer` dry-run).
//! * `MutAssign::assign_advice` forwards everything, but records the value of
//!   every advice cell (column, row, region, value) and, when a mutation plan
//!   is installed, substitutes the planned value for the planned cell.
//!
//! The plan lives in a thread-local (the floor-planner trait cannot carry
//! state), installed by an RAII guard around each `MockProver::run`.
//! `MockProver::run` is single-threaded, so thread-local state is sound.
//!
//! Only the cell written into the table is changed. Values that downstream
//! synthesis code derived from the original (witness-side `AssignedCell`
//! values) are not recomputed, so this is a true single-cell perturbation.
//! `MockProver::run` picks its own `n = 2^k` through a `RowSizer` dry run, so
//! `k` is not a parameter here; it is fixed by the circuit under test
//! (for `MidnightCircuit` through its constructor).
//!
//! Instance (public) cells are not mutated: the statement is the valid one.
//! Mutations of the kinds [`MutationKind::PlusOne`], `MinusOne`, `Zero`,
//! `Random` and `Swap` are generated deterministically from a seed.
//! Single-cell mutations must be rejected. Swaps are reported separately
//! because a swap of two cells that a symmetric constraint treats alike
//! (for example `a + b = c`) is legitimately accepted.

use midnight_curves::pairing::group::ff::{Field, FromUniformBytes};
use midnight_proofs::circuit::{Layouter, Value, layouter::SyncDeps};
use midnight_proofs::dev::MockProver;
use midnight_proofs::plonk::{
    Advice, Any, Assignment, Challenge, Circuit, Column, ConstraintSystem, Error, Fixed,
    FloorPlanner, Instance, Selector,
};
use midnight_proofs::utils::rational::Rational;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use std::any::Any as DynAny;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::ops::Range;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

/// One recorded advice assignment.
#[derive(Clone, Debug)]
pub struct AdviceCell<F> {
    pub column: usize,
    pub row: usize,
    /// Region name plus namespace path active when the cell was assigned.
    pub region: Arc<str>,
    pub value: F,
}

/// Kind of a mutation of the witness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationKind {
    PlusOne,
    MinusOne,
    Zero,
    Random,
    /// Exchange the value with the cell at `(column, row)`.
    Swap {
        column: usize,
        row: usize,
    },
}

/// A cell the harness may legitimately leave unchecked. A written reason is
/// mandatory so that an allowlist cannot silently hide an under-constrained
/// cell. An entry matches a cell only when ALL its conditions hold.
#[derive(Clone, Debug)]
pub struct Allow {
    /// Advice column index.
    pub column: usize,
    /// Rows covered (use `0..usize::MAX` for all rows).
    pub rows: Range<usize>,
    /// The cell's region label must contain this text (empty matches all).
    pub region_contains: &'static str,
    /// The cell is covered only where ALL these fixed columns are zero at the
    /// cell's row (zero fixed coefficients make the advice slot unused by the
    /// gate). Empty means no such condition.
    pub fixed_zero: &'static [usize],
    pub reason: &'static str,
}

/// Harness parameters.
#[derive(Clone, Debug)]
pub struct Config {
    pub seed: u64,
    /// At most this many distinct cells receive single-cell mutations
    /// (chosen by a seeded shuffle; `None` means all).
    pub max_cells: Option<usize>,
    /// Hard cap on single-cell mutation runs (`None` means no cap).
    pub max_mutations: Option<usize>,
    /// Number of swap mutations (pairs of cells with different values).
    pub swaps: usize,
    pub allow: Vec<Allow>,
}

/// One mutation that `MockProver` accepted.
#[derive(Clone, Debug)]
pub struct Accepted {
    pub column: usize,
    pub row: usize,
    pub region: String,
    pub kind: MutationKind,
    pub original: String,
    pub mutated: String,
}

/// Result of running the harness on one circuit.
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub circuit: String,
    pub cells_total: usize,
    pub cells_tested: usize,
    pub allowlisted_cells: usize,
    pub mutations_run: usize,
    /// Mutations skipped because they would not change the cell value.
    pub noop_skipped: usize,
    pub rejected: usize,
    /// Mutations whose run or verification panicked (counted as rejected).
    pub panicked: usize,
    /// Single-cell mutations accepted: under-constrained cells.
    pub accepted: Vec<Accepted>,
    /// Accepted swaps (informational unless the pair is not symmetric).
    pub accepted_swaps: Vec<Accepted>,
}

impl Report {
    /// Human-readable listing of every accepted single-cell mutation as
    /// `(circuit, column, row)`.
    pub fn describe_accepted(&self) -> String {
        let mut out = String::new();
        for a in &self.accepted {
            let _ = writeln!(
                out,
                "  circuit={} column={} row={} region={:?} mutation={:?} {} -> {}",
                self.circuit, a.column, a.row, a.region, a.kind, a.original, a.mutated
            );
        }
        out
    }

    pub fn summary(&self) -> String {
        format!(
            "{}: cells={} tested={} allowlisted={} mutations={} rejected={} (panicked={}) noop={} accepted={} accepted_swaps={}",
            self.circuit,
            self.cells_total,
            self.cells_tested,
            self.allowlisted_cells,
            self.mutations_run,
            self.rejected,
            self.panicked,
            self.noop_skipped,
            self.accepted.len(),
            self.accepted_swaps.len()
        )
    }
}

// ---------------------------------------------------------------- state

struct State<F> {
    /// (column, row) -> replacement value.
    plan: BTreeMap<(usize, usize), F>,
    /// Record every assigned cell (only for the unmutated run).
    record: bool,
    recorded: BTreeMap<(usize, usize), AdviceCell<F>>,
    region: String,
    namespaces: Vec<String>,
    /// Cached `namespace/region` label of the current position.
    label: Arc<str>,
}

impl<F> State<F> {
    fn refresh_label(&mut self) {
        let label = match self.namespaces.last() {
            Some(ns) => format!("{ns}/{}", self.region),
            None => self.region.clone(),
        };
        self.label = Arc::from(label);
    }
}

thread_local! {
    static STATE: RefCell<Option<Box<dyn DynAny>>> = const { RefCell::new(None) };
}

fn with_state<F: Field, R>(f: impl FnOnce(&mut State<F>) -> R) -> Option<R> {
    STATE.with(|s| {
        let mut guard = s.borrow_mut();
        guard
            .as_mut()
            .and_then(|b| b.downcast_mut::<State<F>>())
            .map(f)
    })
}

struct StateGuard;

impl StateGuard {
    fn install<F: Field>(plan: BTreeMap<(usize, usize), F>, record: bool) -> Self {
        STATE.with(|s| {
            *s.borrow_mut() = Some(Box::new(State::<F> {
                plan,
                record,
                recorded: BTreeMap::new(),
                region: String::new(),
                namespaces: Vec::new(),
                label: Arc::from(""),
            }));
        });
        StateGuard
    }

    fn take_recorded<F: Field>(&self) -> BTreeMap<(usize, usize), AdviceCell<F>> {
        with_state::<F, _>(|st| std::mem::take(&mut st.recorded)).unwrap_or_default()
    }
}

impl Drop for StateGuard {
    fn drop(&mut self) {
        STATE.with(|s| *s.borrow_mut() = None);
    }
}

// ------------------------------------------------- wrapper circuit/planner

/// Wrapper circuit that lets the harness observe and override advice cells.
pub struct Mutated<'a, C> {
    inner: &'a C,
}

/// Floor planner running the inner circuit's planner over [`MutAssign`].
pub struct MutPlanner<P>(std::marker::PhantomData<P>);

impl<P: FloorPlanner> FloorPlanner for MutPlanner<P> {
    fn synthesize<F: Field, CS: Assignment<F> + SyncDeps, C: Circuit<F>>(
        cs: &mut CS,
        circuit: &C,
        config: C::Config,
        constants: Vec<Column<Fixed>>,
    ) -> Result<(), Error> {
        // `circuit` is the `Mutated` wrapper itself; its `synthesize`
        // delegates to the inner circuit, so the inner planner can use it.
        with_state::<F, _>(|st| {
            st.namespaces.clear();
            st.region.clear();
            st.refresh_label();
        });
        let mut wrapped = MutAssign { inner: cs };
        P::synthesize(&mut wrapped, circuit, config, constants)
    }
}

impl<F: Field, C: Circuit<F>> Circuit<F> for Mutated<'_, C> {
    type Config = C::Config;
    type FloorPlanner = MutPlanner<C::FloorPlanner>;
    // `midnight-zk-stdlib` enables `circuit-params` on `midnight-proofs`
    // for this whole dependency graph, so the associated type exists.
    type Params = C::Params;

    fn without_witnesses(&self) -> Self {
        unreachable!("qa_mutation::Mutated is only used with a concrete witness")
    }

    fn params(&self) -> Self::Params {
        self.inner.params()
    }

    fn configure_with_params(meta: &mut ConstraintSystem<F>, params: Self::Params) -> Self::Config {
        C::configure_with_params(meta, params)
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        C::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        self.inner.synthesize(config, layouter)
    }
}

struct MutAssign<'a, CS> {
    inner: &'a mut CS,
}

impl<F: Field, CS: Assignment<F>> Assignment<F> for MutAssign<'_, CS> {
    fn enter_region<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        let name: String = name_fn().into();
        with_state::<F, _>(|st| {
            st.region = name.clone();
            st.refresh_label();
        });
        self.inner.enter_region(move || name);
    }

    fn annotate_column<A, AR>(&mut self, annotation: A, column: Column<Any>)
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        self.inner.annotate_column(annotation, column);
    }

    fn exit_region(&mut self) {
        self.inner.exit_region();
    }

    fn enable_selector<A, AR>(
        &mut self,
        annotation: A,
        selector: &Selector,
        row: usize,
    ) -> Result<(), Error>
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        self.inner.enable_selector(annotation, selector, row)
    }

    fn query_instance(&self, column: Column<Instance>, row: usize) -> Result<Value<F>, Error> {
        self.inner.query_instance(column, row)
    }

    fn assign_advice<V, VR, A, AR>(
        &mut self,
        annotation: A,
        column: Column<Advice>,
        row: usize,
        to: V,
    ) -> Result<(), Error>
    where
        V: FnOnce() -> Value<VR>,
        VR: Into<Rational<F>>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        let key = (column.index(), row);
        let value: Value<F> = to().map(|v| v.into()).evaluate();
        let value = value.map(|v| {
            with_state::<F, _>(|st| {
                if st.record {
                    let region = st.label.clone();
                    st.recorded.insert(
                        key,
                        AdviceCell {
                            column: key.0,
                            row,
                            region,
                            value: v,
                        },
                    );
                }
                st.plan.get(&key).copied().unwrap_or(v)
            })
            .unwrap_or(v)
        });
        self.inner
            .assign_advice(annotation, column, row, move || value)
    }

    fn assign_fixed<V, VR, A, AR>(
        &mut self,
        annotation: A,
        column: Column<Fixed>,
        row: usize,
        to: V,
    ) -> Result<(), Error>
    where
        V: FnOnce() -> Value<VR>,
        VR: Into<Rational<F>>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        self.inner.assign_fixed(annotation, column, row, to)
    }

    fn copy(
        &mut self,
        left_column: Column<Any>,
        left_row: usize,
        right_column: Column<Any>,
        right_row: usize,
    ) -> Result<(), Error> {
        self.inner
            .copy(left_column, left_row, right_column, right_row)
    }

    fn fill_from_row(
        &mut self,
        column: Column<Fixed>,
        row: usize,
        to: Value<Rational<F>>,
    ) -> Result<(), Error> {
        self.inner.fill_from_row(column, row, to)
    }

    fn get_challenge(&self, challenge: Challenge) -> Value<F> {
        self.inner.get_challenge(challenge)
    }

    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        let name: String = name_fn().into();
        with_state::<F, _>(|st| {
            st.namespaces.push(name.clone());
            st.refresh_label();
        });
        self.inner.push_namespace(move || name);
    }

    fn pop_namespace(&mut self, gadget_name: Option<String>) {
        with_state::<F, _>(|st| {
            st.namespaces.pop();
            st.refresh_label();
        });
        self.inner.pop_namespace(gadget_name);
    }
}

// ------------------------------------------------------------- the driver

/// Outcome of one `MockProver` run.
#[derive(Debug)]
pub enum Outcome {
    Accepted,
    Rejected(String),
    Panicked(String),
}

fn run_once<F, C>(
    circuit: &C,
    instance: &[Vec<F>],
    plan: BTreeMap<(usize, usize), F>,
) -> (Outcome, BTreeMap<(usize, usize), AdviceCell<F>>)
where
    F: Field + FromUniformBytes<64> + Ord,
    C: Circuit<F>,
{
    let guard = StateGuard::install::<F>(plan, false);
    let wrapped = Mutated { inner: circuit };
    let result = catch_unwind(AssertUnwindSafe(|| {
        match MockProver::<F>::run(&wrapped, instance.to_vec()) {
            Ok(prover) => match prover.verify() {
                Ok(()) => Outcome::Accepted,
                Err(e) => Outcome::Rejected(format!("{} failure(s)", e.len())),
            },
            Err(e) => Outcome::Rejected(format!("synthesis error {e:?}")),
        }
    }));
    let recorded = guard.take_recorded::<F>();
    match result {
        Ok(o) => (o, recorded),
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_else(|| "non-string panic".into());
            (Outcome::Panicked(msg), recorded)
        }
    }
}

/// The unmutated assignment: advice cells and, per fixed column, which rows
/// hold an explicit zero.
pub struct Recorded<F> {
    pub cells: Vec<AdviceCell<F>>,
    pub fixed_zero: Vec<Vec<bool>>,
}

/// Run the unmutated circuit and return every advice cell it assigned.
/// Errors when `MockProver` does not accept the valid witness.
pub fn record_cells<F, C>(circuit: &C, instance: &[Vec<F>]) -> Result<Recorded<F>, String>
where
    F: Field + FromUniformBytes<64> + Ord,
    C: Circuit<F>,
{
    let guard = StateGuard::install::<F>(BTreeMap::new(), true);
    let wrapped = Mutated { inner: circuit };
    let prover = MockProver::<F>::run(&wrapped, instance.to_vec())
        .map_err(|e| format!("valid witness synthesis error: {e:?}"))?;
    prover
        .verify()
        .map_err(|e| format!("valid witness not accepted: {} failure(s)", e.len()))?;
    let fixed_zero = prover
        .fixed()
        .iter()
        .map(|col| {
            col.iter()
                .map(|c| matches!(c, midnight_proofs::dev::CellValue::Assigned(v) if *v == F::ZERO))
                .collect()
        })
        .collect();
    let cells = guard.take_recorded::<F>().into_values().collect();
    Ok(Recorded { cells, fixed_zero })
}

fn describe<F: std::fmt::Debug>(v: &F) -> String {
    short_hex(&format!("{v:?}"))
}

/// Shorten `Fq(0x000...01)` style debug output to `0x1`.
fn short_hex(s: &str) -> String {
    match s.find("0x") {
        Some(i) => {
            let digits: String = s[i + 2..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();
            let t = digits.trim_start_matches('0');
            format!("0x{}", if t.is_empty() { "0" } else { t })
        }
        None => s.to_string(),
    }
}

/// Diagnostic: the unmutated assignment of every advice and fixed column at
/// `row`, plus the enabled gate names, to judge whether a cell reported as
/// accepted is a genuinely free (zero-coefficient) cell or a real gap.
pub fn inspect_row<F, C>(circuit: &C, instance: &[Vec<F>], row: usize) -> Result<String, String>
where
    F: Field + FromUniformBytes<64> + Ord + std::fmt::Debug,
    C: Circuit<F>,
{
    let _guard = StateGuard::install::<F>(BTreeMap::new(), false);
    let wrapped = Mutated { inner: circuit };
    let prover = MockProver::<F>::run(&wrapped, instance.to_vec()).map_err(|e| format!("{e:?}"))?;
    let mut out = String::new();
    for (i, col) in prover.advice().iter().enumerate() {
        let _ = writeln!(out, "advice[{i}][{row}] = {}", cell_text(&col[row]));
    }
    for (i, col) in prover.fixed().iter().enumerate() {
        let _ = writeln!(out, "fixed[{i}][{row}] = {}", cell_text(&col[row]));
    }
    Ok(out)
}

fn cell_text<F: Field + std::fmt::Debug>(c: &midnight_proofs::dev::CellValue<F>) -> String {
    match c {
        midnight_proofs::dev::CellValue::Assigned(v) => describe(v),
        other => format!("{other:?}"),
    }
}

/// Run the mutation suite on one circuit with a valid witness.
///
/// `circuit_name` labels findings. `instance` is the valid public input
/// (one vector per instance column, as for `MockProver::run`).
pub fn run_suite<F, C>(
    circuit_name: &str,
    circuit: &C,
    instance: &[Vec<F>],
    cfg: &Config,
) -> Result<Report, String>
where
    F: Field + FromUniformBytes<64> + Ord + std::fmt::Debug,
    C: Circuit<F>,
{
    let Recorded { cells, fixed_zero } = record_cells(circuit, instance)?;
    let mut report = Report {
        circuit: circuit_name.to_string(),
        cells_total: cells.len(),
        ..Report::default()
    };
    let allowed = |c: &AdviceCell<F>| {
        cfg.allow.iter().any(|a| {
            a.column == c.column
                && a.rows.contains(&c.row)
                && c.region.contains(a.region_contains)
                && a.fixed_zero.iter().all(|&f| {
                    fixed_zero
                        .get(f)
                        .and_then(|col| col.get(c.row))
                        .copied()
                        .unwrap_or(false)
                })
        })
    };
    for a in &cfg.allow {
        assert!(
            !a.reason.trim().is_empty(),
            "allowlist entry needs a reason"
        );
    }
    let (skipped, mut candidates): (Vec<_>, Vec<_>) = cells.iter().partition(|c| allowed(c));
    report.allowlisted_cells = skipped.len();

    let mut rng = StdRng::seed_from_u64(cfg.seed);
    candidates.shuffle(&mut rng);
    if let Some(max) = cfg.max_cells {
        candidates.truncate(max);
    }
    report.cells_tested = candidates.len();

    let one = F::ONE;
    for cell in &candidates {
        let random = F::random(&mut rng);
        let mutants = [
            (MutationKind::PlusOne, cell.value + one),
            (MutationKind::MinusOne, cell.value - one),
            (MutationKind::Zero, F::ZERO),
            (MutationKind::Random, random),
        ];
        for (kind, new) in mutants {
            if cfg.max_mutations.is_some_and(|m| report.mutations_run >= m) {
                break;
            }
            if new == cell.value {
                report.noop_skipped += 1;
                continue;
            }
            let plan = BTreeMap::from([((cell.column, cell.row), new)]);
            let (outcome, _) = run_once(circuit, instance, plan);
            report.mutations_run += 1;
            match outcome {
                Outcome::Rejected(_) => report.rejected += 1,
                Outcome::Panicked(_) => {
                    report.rejected += 1;
                    report.panicked += 1;
                }
                Outcome::Accepted => report.accepted.push(Accepted {
                    column: cell.column,
                    row: cell.row,
                    region: cell.region.to_string(),
                    kind,
                    original: describe(&cell.value),
                    mutated: describe(&new),
                }),
            }
        }
    }

    // Swaps between two distinct allowed-or-not cells of different value.
    let all: Vec<&AdviceCell<F>> = cells.iter().collect();
    let mut done = 0usize;
    let mut attempts = 0usize;
    while done < cfg.swaps && all.len() > 1 && attempts < cfg.swaps * 50 {
        attempts += 1;
        let a = all[(rng.next_u64() as usize) % all.len()];
        let b = all[(rng.next_u64() as usize) % all.len()];
        if a.value == b.value || allowed(a) || allowed(b) {
            continue;
        }
        done += 1;
        let plan = BTreeMap::from([((a.column, a.row), b.value), ((b.column, b.row), a.value)]);
        let (outcome, _) = run_once(circuit, instance, plan);
        report.mutations_run += 1;
        match outcome {
            Outcome::Rejected(_) => report.rejected += 1,
            Outcome::Panicked(_) => {
                report.rejected += 1;
                report.panicked += 1;
            }
            Outcome::Accepted => report.accepted_swaps.push(Accepted {
                column: a.column,
                row: a.row,
                region: a.region.to_string(),
                kind: MutationKind::Swap {
                    column: b.column,
                    row: b.row,
                },
                original: describe(&a.value),
                mutated: describe(&b.value),
            }),
        }
    }
    Ok(report)
}
