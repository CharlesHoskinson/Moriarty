// U031 REQUIREMENTS MODEL, pending a Rust constraint/export-plan API.
// Separate capability checks for constraints and witness computation, preserving
// ordered typed operands. No JSON/foreign format is promoted to Midnight.
module planExports {
  type Feature = Arithmetic | Lookup | Interaction | CustomGate | Structured | NativeClosure
  type Format = ConstraintJson | Foreign | Midnight
  type Disposition = Supported | Unsupported
  type Sort = Field | UInt128
  type Operation = {id: int, inputs: List[int], outputs: List[int],
    inputTypes: List[Sort], outputTypes: List[Sort], source: int, component: int,
    constraintFeatures: Set[Feature], witnessFeatures: Set[Feature]}
  type Capabilities = {constraints: Set[Feature], witnesses: Set[Feature],
    field: bool, layout: bool, midnightExporter: bool}
  type Report = {manifest: List[Operation], constraintPlan: List[int], witnessPlan: List[int],
    capabilities: Capabilities, format: Format, constraintDisposition: Disposition,
    witnessDisposition: Disposition, constraintOmissions: Set[int], witnessOmissions: Set[int],
    midnightClaim: bool}
  type State = {prepared: bool, report: Report}
  var state: State

  pure val structured: Operation = {id: 0, inputs: List(2, 1, 2), outputs: List(3),
    inputTypes: List(UInt128, UInt128, UInt128), outputTypes: List(UInt128), source: 10, component: 20,
    constraintFeatures: Set(Arithmetic), witnessFeatures: Set(Structured)}
  pure val closure: Operation = {id: 1, inputs: List(3), outputs: List(4),
    inputTypes: List(UInt128), outputTypes: List(UInt128), source: 10, component: 21,
    constraintFeatures: Set(Arithmetic), witnessFeatures: Set(NativeClosure)}
  pure val lookup: Operation = {id: 2, inputs: List(4, 1), outputs: List(5),
    inputTypes: List(UInt128, UInt128), outputTypes: List(Field), source: 11, component: 22,
    constraintFeatures: Set(Lookup), witnessFeatures: Set(Structured)}
  pure val interaction: Operation = {id: 3, inputs: List(5), outputs: List(6),
    inputTypes: List(Field), outputTypes: List(Field), source: 11, component: 23,
    constraintFeatures: Set(Interaction, CustomGate), witnessFeatures: Set(Structured)}
  pure val basicCaps: Capabilities = {constraints: Set(Arithmetic), witnesses: Set(Structured),
    field: true, layout: true, midnightExporter: false}
  pure val allCaps: Capabilities = {constraints: Set(Arithmetic, Lookup, Interaction, CustomGate),
    witnesses: Set(Structured, NativeClosure), field: true, layout: true, midnightExporter: true}
  pure def constraintsSupported(ops: List[Operation], caps: Capabilities): bool =
    caps.field and caps.layout and ops.indices().forall(ix => ops.nth(ix).constraintFeatures.subseteq(caps.constraints))
  pure def witnessSupported(ops: List[Operation], caps: Capabilities): bool =
    caps.field and caps.layout and ops.indices().forall(ix => ops.nth(ix).witnessFeatures.subseteq(caps.witnesses))
  pure def missingConstraints(ops: List[Operation], caps: Capabilities): Set[int] =
    ops.indices().filter(ix => not(caps.field and caps.layout and
      ops.nth(ix).constraintFeatures.subseteq(caps.constraints))).map(ix => ops.nth(ix).id)
  pure def missingWitness(ops: List[Operation], caps: Capabilities): Set[int] =
    ops.indices().filter(ix => not(caps.field and caps.layout and
      ops.nth(ix).witnessFeatures.subseteq(caps.witnesses))).map(ix => ops.nth(ix).id)
  pure def prepare(ops: List[Operation], caps: Capabilities, format: Format): Report = {
    val ids = ops.foldl(List(), (ids, op) => ids.append(op.id))
    val constraintsOk = constraintsSupported(ops, caps)
    val witnessOk = witnessSupported(ops, caps)
    {manifest: ops, constraintPlan: ids, witnessPlan: ids, capabilities: caps, format: format,
      constraintDisposition: if (constraintsOk) Supported else Unsupported,
      witnessDisposition: if (witnessOk) Supported else Unsupported,
      constraintOmissions: missingConstraints(ops, caps), witnessOmissions: missingWitness(ops, caps),
      midnightClaim: constraintsOk and witnessOk and format == Midnight and caps.midnightExporter}
  }
  action init = state' = {prepared: false, report: prepare(List(), basicCaps, ConstraintJson)}
  action request(ops: List[Operation], caps: Capabilities, format: Format): bool =
    state' = {prepared: true, report: prepare(ops, caps, format)}
  action step = {
    nondet ops = Set(List(structured), List(structured, closure),
      List(structured, lookup), List(structured, interaction)).oneOf()
    nondet caps = Set(basicCaps, allCaps,
      {...allCaps, field: false}, {...allCaps, layout: false}, {...allCaps, midnightExporter: false}).oneOf()
    nondet format = Set(ConstraintJson, Foreign, Midnight).oneOf()
    request(ops, caps, format)
  }
  val unsupportedWitness = state.prepared and state.report.witnessDisposition == Unsupported
  val constraintsOnly = state.prepared and state.report.constraintDisposition == Supported
    and state.report.witnessDisposition == Unsupported
  val midnightSupported = state.prepared and state.report.midnightClaim
  val modelInv = {
    val r = state.report
    and {
      r.constraintPlan == r.manifest.foldl(List(), (ids, op) => ids.append(op.id)),
      r.witnessPlan == r.manifest.foldl(List(), (ids, op) => ids.append(op.id)),
      (r.constraintDisposition == Supported) == constraintsSupported(r.manifest, r.capabilities),
      (r.witnessDisposition == Supported) == witnessSupported(r.manifest, r.capabilities),
      r.constraintOmissions == missingConstraints(r.manifest, r.capabilities),
      r.witnessOmissions == missingWitness(r.manifest, r.capabilities),
      r.manifest.indices().forall(ix => {
        val op = r.manifest.nth(ix)
        op.inputs.length() == op.inputTypes.length() and op.outputs.length() == op.outputTypes.length()
          and Set(10, 11).contains(op.source) and 20.to(23).contains(op.component)
      }),
      r.midnightClaim implies (r.format == Midnight and r.capabilities.midnightExporter
        and r.constraintDisposition == Supported and r.witnessDisposition == Supported),
      // Ordered, repeated operands are significant; plans do not turn them into sets.
      if (state.prepared) r.manifest.nth(0).inputs == List(2, 1, 2) else true,
    }
  }
}
