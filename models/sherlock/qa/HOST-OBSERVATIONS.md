# Normal host observation contract

Frozen before the emitter, 2026-10-16. This is a private diagnostic projection of
existing `TraceView`, `Snapshot` and `StepRecord` fields on the eight committed
normal `sherlock-k-a/1` inputs. It is not a canonical codec, state-import API,
public ABI, new semantics or authenticated execution receipt.

Independent expectations remain the four JSON files and K source pinned by
`check.py::FROZEN/SOURCE`. Programs are N01–N08 in source order. For each case,
every literal five-column `ordinary_fuel_prefixes` row selects one actual run.
The producer imports only typed input builders, never expected trace/output data.
Before comparing any run, compare all eight actual program/entry renderings with
the frozen input code/entry terms, retaining function/parameter/argument order.

The supplied JSON contains declared producer/profile/source identity, boundary
`post-startup`, a complete ordered program identity list, then 216 ordered runs.
Each run has its case ID, complete initial snapshot, every full record's rule,
charge, before and after snapshot, complete terminal snapshot and a Boolean
indicating whether a further transition existed. Snapshot fields are code_k,
entry_k, control_k, environment_k, frames_top_first_k, depth, stack_limit,
remaining_fuel, initial_fuel, output_k and charged_steps. K rendering is diagnostic
text on the existing canonical-decimal-name common domain; it is not typed
structural equality or a proof of a codec.

For a literal row [F, boundary, remaining, output, publication_rule], reconstruct
state j, for every j=0..boundary, from the literal state j and fixed fields;
replace initial_fuel by F and remaining_fuel by F-literal charged_steps. All other
state fields remain literal. The initial state is j=0. Each j=1..boundary creates
one record using its literal incoming rule and charge 1, retaining complete
before/after states. Append the literal free-publication record, charge 0, whose
after state changes only control_k to .K and output_k to the literal output.
Terminal is that after state, and no further transition exists. This is the
committed corpus reconstruction contract, not an evaluator or inferred work.

Concrete frozen discriminator: N01 fuel 0 starts Eval(Literal(UInt7)), has one
rule84/charge0 record to Exhausted, keeps work0/fuel0 and has no next step.
N01 fuel1 has rule102/charge1 to Return(UInt7), then rule80/charge0 to Done(UInt7),
work1/fuel0. Fuel2 has the same two records and terminal remaining1. N02 fuel7
has seven charged records then publication; all environments, saved frames,
code/entry, controls and raw outcomes are retained at every intervening prefix.

A comparison must reject missing runs, records, state fields, identity/order,
wrong work/charges/outcomes and omitted terminal publication, reporting the first
field/rule/prefix and both raw records/states. Parsing a supplied file establishes
only comparison with that artifact. Producer/source identities are declarations;
actual compiler/execution command, pinned sources, output digest and terminal
receipt come separately from the inspected execution wrapper.

`Machine::new` constructs the post-startup state directly. It does not execute or
emit K uiRun/rule74. Constructor startup is unsupported as an executed edge here.
All K/Quint/Lean/Clean/handwritten/native execution remains separately unrun.
The existing A host evaluator has unmetered helper/allocation/native-stack risks;
this adapter is limited to fixed trusted normal typed ASTs and literal budgets,
not a public source/CLI/LSP/MCP input path. Root owns compile/run admission and
wrapper bounds. Default Python QA must continue to perform no evaluator execution.
