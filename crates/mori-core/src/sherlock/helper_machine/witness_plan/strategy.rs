//! Optional memoization and scratch-slot lifetimes for the existing witness DAG.
//!
//! This executes only `dag::operation`, never B/source transitions. Exact typed
//! keys use full equality, not hashes: program/profile/layout/admission, graph,
//! binding paths/types, and each operation's complete immutable argument values.
//! Every Read and the unchanged plan validation still execute on warm calls.
//! A cache retains one graph, with one most-recent argument/result entry per
//! non-Read node. There is no global cache, eviction budget or admission policy.
//! Logical slots/operations below are not measured bytes, allocator traffic,
//! total clones, elapsed time, B work, or a physical-memory improvement claim.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotStorage { RetainAll, LastUse }

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct StrategyDiagnostics {
    /// Every node in the original stable order, including memo hits.
    pub scheduled_nodes: usize,
    pub cache_hits: usize,
    /// Only non-Read misses; all Read nodes always execute separately.
    pub cache_misses: usize,
    pub scope_reset: bool,
    /// Values cloned out of memo entries, excluding key/argument/other clones.
    pub copied_cached_slot_values: usize,
    pub reclaimed_slots: usize,
    /// Scratch slots only; excludes retained raw records, plan, sink and cache.
    pub peak_live_slots: usize,
    pub retained_slots_at_publication: usize,
    pub cached_value_slots: usize,
    /// Top-level argument cells retained as exact keys, not recursive bytes.
    pub cached_argument_slots: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrategyEvaluation {
    /// Complete decoded output; HostDiagnostics counts only executed operations.
    /// `retained_slots` is the scratch count immediately before publication.
    pub witness: WitnessEvaluation,
    pub scheduling: StrategyDiagnostics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BindingShape {
    id: BindingId,
    path: InputPath,
    value_type: SourceType,
    location: Location,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct CacheScope {
    format: &'static str,
    lifetime: LifetimeUnit,
    admission: AdmissionSnapshot,
    nodes: Vec<Node>,
    bindings: Vec<BindingShape>,
}
impl CacheScope {
    fn new(plan: &WitnessPlan, input: &HostSnapshot) -> Self {
        Self {
            format: plan.format, lifetime: plan.lifetime,
            admission: input.admission.clone(), nodes: plan.nodes.clone(),
            bindings: plan.inputs.iter().map(|b| BindingShape {
                id: b.id, path: b.path, value_type: b.value_type, location: b.location,
            }).collect(),
        }
    }
}
struct MemoEntry {
    /// Full values preserve all raw tags, ordered multiplicity and occurrences.
    arguments: Vec<SlotValue>,
    value: SlotValue,
}
#[derive(Default)]
pub struct WitnessCache {
    scope: Option<CacheScope>,
    entries: Vec<Option<MemoEntry>>,
}
impl WitnessCache {
    pub fn clear(&mut self) { self.scope = None; self.entries.clear(); }
    pub fn value_slots(&self) -> usize {
        self.entries.iter().filter(|entry| entry.is_some()).count()
    }
    pub fn argument_slots(&self) -> usize {
        self.entries.iter().filter_map(Option::as_ref).map(|e| e.arguments.len()).sum()
    }
    fn bind(&mut self, plan: &WitnessPlan, input: &HostSnapshot) -> bool {
        let scope = CacheScope::new(plan, input);
        if self.scope.as_ref() == Some(&scope) { return false; }
        self.scope = Some(scope);
        self.entries = (0..plan.nodes.len()).map(|_| None).collect();
        true
    }
}

pub fn evaluate_last_use(plan: &WitnessPlan, input: &HostSnapshot)
    -> Result<StrategyEvaluation, WitnessFailure>
{
    evaluate_with_storage(plan, input, SlotStorage::LastUse, None)
}
pub fn evaluate_memoized(plan: &WitnessPlan, input: &HostSnapshot,
    cache: &mut WitnessCache, storage: SlotStorage)
    -> Result<StrategyEvaluation, WitnessFailure>
{
    evaluate_with_storage(plan, input, storage, Some(cache))
}

/// Preserve the baseline's complete validation, order and operation semantics.
/// Last-use only releases scratch values after their final ordered consumer;
/// boundary live-outs remain through publication. Cached copies are accounted
/// separately and are not reported as reclaimed scratch or semantic savings.
pub fn evaluate_with_storage(plan: &WitnessPlan, input: &HostSnapshot,
    storage: SlotStorage, mut cache: Option<&mut WitnessCache>)
    -> Result<StrategyEvaluation, WitnessFailure>
{
    let at = Location::new("plan", None, "structure");
    let schedule = dag::validate(plan)?;
    let mut scheduling = StrategyDiagnostics::default();
    if let Some(memo) = cache.as_deref_mut() {
        scheduling.scope_reset = memo.bind(plan, input);
    }
    let mut release = vec![Vec::new(); schedule.len()];
    if storage == SlotStorage::LastUse {
        for definition in &plan.definitions {
            // validate() compared these exact definitions to the existing
            // recomputation; every non-live-out last-use is a schedule position.
            if !definition.live_out {
                release[definition.last_use_order].push(definition.value);
            }
        }
    }
    let mut slots: Vec<Option<SlotValue>> = vec![None; plan.nodes.len()];
    let mut diagnostics = HostDiagnostics::default();
    let mut live = 0;
    for (position, id) in schedule.into_iter().enumerate() {
        let node = &plan.nodes[id.0];
        let result = {
            let mut args = Vec::new();
            for dependency in &node.dependencies {
                let value = slots.get(dependency.node.0).and_then(Option::as_ref)
                    .ok_or_else(|| dag::node_failure(node, FailureKind::MissingDependency {
                        dependency: dependency.node,
                    }))?;
                if value.value_type() != dependency.value_type {
                    return Err(dag::node_failure(node, FailureKind::WrongType {
                        expected: dependency.value_type, actual: value.value_type(),
                    }));
                }
                args.push(value);
            }
            // Reads are never bypassed: InputChanged/type/location behavior is
            // the existing operation's behavior, including warm evaluations.
            let memoizable = !matches!(node.operation, Operation::Read(_));
            let cached = if memoizable {
                cache.as_deref().and_then(|memo| memo.entries[id.0].as_ref())
                    .filter(|entry| entry.arguments.len() == args.len()
                        && entry.arguments.iter().zip(&args).all(|(a, b)| a == *b))
            } else { None };
            if let Some(entry) = cached {
                scheduling.cache_hits += 1;
                scheduling.copied_cached_slot_values += 1;
                entry.value.clone()
            } else {
                let value = dag::operation(plan, input, node, &args, &mut diagnostics)?;
                diagnostics.evaluated_nodes += 1;
                if memoizable {
                    if let Some(memo) = cache.as_deref_mut() {
                        scheduling.cache_misses += 1;
                        memo.entries[id.0] = Some(MemoEntry {
                            arguments: args.iter().map(|value| (**value).clone()).collect(),
                            value: value.clone(),
                        });
                    }
                }
                value
            }
        };
        if result.value_type() != node.output_type {
            return Err(dag::node_failure(node, FailureKind::WrongType {
                expected: node.output_type, actual: result.value_type(),
            }));
        }
        slots[id.0] = Some(result);
        live += 1;
        scheduling.scheduled_nodes += 1;
        scheduling.peak_live_slots = scheduling.peak_live_slots.max(live);
        for dead in &release[position] {
            if slots[dead.0].take().is_some() {
                live -= 1;
                scheduling.reclaimed_slots += 1;
            }
        }
    }
    diagnostics.retained_slots = live;
    scheduling.retained_slots_at_publication = live;
    if let Some(memo) = cache.as_deref() {
        scheduling.cached_value_slots = memo.value_slots();
        scheduling.cached_argument_slots = memo.argument_slots();
    }
    let Some(SlotValue::Snapshot(decoded)) = slots.get_mut(plan.output.0).and_then(Option::take) else {
        return Err(fail(at, FailureKind::IncompleteOutputs));
    };
    Ok(StrategyEvaluation {
        witness: WitnessEvaluation { decoded, diagnostics }, scheduling,
    })
}
