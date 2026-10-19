//! Ordinary C6 literal traversal continuing the SAME C1-C5 private assembly.
//! Source-only: owner constant backing/checked writes/G-5 remain pending.
//! No source evaluation, admission, alternative arena, or runtime representation.

use super::adt::{CtorIdC, TypeIdC};
use super::ext_program_c::{
    lower_ext_c1_to_c5, ExtCContinuation, ExtCProgramWriter, LiteralOccurrence,
    MatchOccurrence, NullaryConstructOccurrence,
};
use super::owner_ext_t3_uses::{BodyOwner, ExtValue, T3UseSite};
use super::owner_ext_type_builder::ExtTypeBuildError;
use super::{CoreValue, PreparedExtProgramSource};

/// Private owner interface over the same reserved expression slots and unified
/// constant/constant-field arenas as C1-C5. Declaration-only: no guard, failure
/// policy, capacity default, constant type judgment or backing is authored here.
pub(super) trait ExtC6ProgramWriter {
    /// Bind the real C1 Literal slot, source owner/local ordinal and immutable
    /// retained value. The owner owns consistency/resource checks before work.
    fn begin_literal(&mut self, literal: &LiteralOccurrence<'_>)
        -> Result<(), ExtTypeBuildError>;
    /// Visit every ExtValue node once in literal-local SOURCE PREORDER, including
    /// Core nodes. Bound source work before extending the private heap worklist.
    /// Ordinals must agree with the already completed T3UseWalker; nested Core
    /// list/tree nodes do not introduce additional ExtValue/T3 ordinals.
    fn visit_literal_value(&mut self, owner: BodyOwner, expr: usize,
        ordinal: usize, source: &ExtValue) -> Result<(), ExtTypeBuildError>;
    /// Read the final existing LiteralCtor T3 record; never restart Pass T.
    fn literal_constructor_use(&self, site: T3UseSite)
        -> Result<(TypeIdC, CtorIdC), ExtTypeBuildError>;
    /// Append to the same constant arena and return its actual root ConstId.
    /// Preserve the current owner emit_literal/emit_const_node ordering for all
    /// Core forms, including the exact dedicated List/Tree/marker behavior.
    fn append_core_literal(&mut self, source: &CoreValue) -> Result<usize, ExtTypeBuildError>;
    /// Append ONE fresh canonical Constructor row and its ordered field IDs.
    /// Owner supplies G-5 typing/markerFree/arity/capacity/error checks; every
    /// supplied field ID is a real result of an earlier append in this arena.
    /// This operation never interns identical-looking occurrences.
    fn append_constructor_constant(&mut self, type_id: TypeIdC, ctor_id: CtorIdC,
        fields: &[usize]) -> Result<usize, ExtTypeBuildError>;
    /// Fill the previously unfilled real C1 Literal slot with its actual ConstId.
    fn finish_literal(&mut self, expr_id: usize, constant: usize)
        -> Result<(), ExtTypeBuildError>;
    /// Bind/check the real C1 Construct row and its final IDs/empty fields.
    fn begin_nullary_construct(&mut self, source: &NullaryConstructOccurrence)
        -> Result<(), ExtTypeBuildError>;
    /// Set that existing Construct row's constant column, preserving its other
    /// payloads. The actual new constant was appended for this ExprId alone.
    fn finish_nullary_construct(&mut self, expr_id: usize, constant: usize)
        -> Result<(), ExtTypeBuildError>;
    fn literal_traversal_invariant(&self) -> ExtTypeBuildError;
}

// Private borrowed source/assembly metadata only. No owned value is evaluated,
// copied into a second IR, or assigned a predicted runtime/constant result.
enum ValueSource<'s> {
    Core(&'s CoreValue),
    Constructor { type_id: TypeIdC, ctor_id: CtorIdC, fields: &'s [ExtValue] },
}
struct ValueTemp<'s> {
    source: ValueSource<'s>,
    parent: Option<usize>, // source-preorder temporary index, never a ConstId
    fields: Vec<usize>,   // actual previously appended child ConstIds, in order
}
enum ValueVisit<'s> {
    Enter { source: &'s ExtValue, parent: Option<usize> },
    Exit { index: usize },
}

/// Continue the actual C1-C5 row assembly; return its original match source-use
/// mappings for P. This does not finalize/export ProgramView or admit execution.
pub(super) fn lower_ext_c6<'s>(
    continuation: ExtCContinuation<'s>,
    writer: &mut impl ExtC6ProgramWriter,
) -> Result<Vec<MatchOccurrence<'s>>, ExtTypeBuildError> {
    let ExtCContinuation { literals, nullary_constructs, matches } = continuation;
    // C6 phase 1: literal ExprId order supplied by the actual C1 occurrence map.
    for literal in &literals {
        writer.begin_literal(literal)?;
        let constant = emit_literal_tree(literal, writer)?;
        writer.finish_literal(literal.expr_id, constant)?;
    }
    // C6 phase 2: nullary Construct ExprId order, AFTER every literal, even
    // when a nullary Construct occurs earlier than a later Literal expression.
    for source in &nullary_constructs {
        writer.begin_nullary_construct(source)?;
        let constant = writer.append_constructor_constant(source.type_id, source.ctor_id, &[])?;
        writer.finish_nullary_construct(source.expr_id, constant)?;
    }
    Ok(matches)
}

/// Ordinary connected C1-C6 call sequence for the eventual owner private entry.
/// The caller retains the SAME checked writer/assembly and immutable prepared
/// borrow. No self-reference, public incomplete view, or substitute engine.
pub(super) fn lower_ext_c1_to_c6<'s, W>(
    prepared: &'s PreparedExtProgramSource,
    writer: &mut W,
) -> Result<Vec<MatchOccurrence<'s>>, ExtTypeBuildError>
where
    W: ExtCProgramWriter + ExtC6ProgramWriter,
{
    let continuation = lower_ext_c1_to_c5(prepared, writer)?;
    lower_ext_c6(continuation, writer)
}

fn emit_literal_tree(
    literal: &LiteralOccurrence<'_>,
    writer: &mut impl ExtC6ProgramWriter,
) -> Result<usize, ExtTypeBuildError> {
    let mut temps: Vec<ValueTemp<'_>> = Vec::new();
    let mut stack = vec![ValueVisit::Enter { source: literal.value, parent: None }];
    while let Some(visit) = stack.pop() {
        match visit {
            ValueVisit::Enter { source, parent } => {
                let index = temps.len();
                writer.visit_literal_value(literal.owner, literal.local_expr, index, source)?;
                let source = match source {
                    ExtValue::Core { value } => ValueSource::Core(value),
                    ExtValue::Constructor { fields, .. } => {
                        let (type_id, ctor_id) = writer.literal_constructor_use(
                            T3UseSite::LiteralCtor { owner: literal.owner,
                                expr: literal.local_expr, value: index })?;
                        ValueSource::Constructor { type_id, ctor_id, fields }
                    }
                };
                let source_fields = match &source {
                    ValueSource::Core(_) => None,
                    ValueSource::Constructor { fields, .. } => Some(*fields),
                };
                temps.push(ValueTemp { source, parent, fields: Vec::new() });
                stack.push(ValueVisit::Exit { index });
                if let Some(fields) = source_fields {
                    for field in fields.iter().rev() {
                        stack.push(ValueVisit::Enter { source: field, parent: Some(index) });
                    }
                }
            }
            ValueVisit::Exit { index } => {
                let t = &temps[index];
                let constant = match &t.source {
                    ValueSource::Core(value) => writer.append_core_literal(value)?,
                    ValueSource::Constructor { type_id, ctor_id, .. } =>
                        writer.append_constructor_constant(*type_id, *ctor_id, &t.fields)?,
                };
                if let Some(parent) = t.parent { temps[parent].fields.push(constant); }
                else { return Ok(constant); }
            }
        }
    }
    // The owner supplies this internal-inconsistency diagnostic. No fake root,
    // nominal type judgment or new refusal rule is invented by the traversal.
    Err(writer.literal_traversal_invariant())
}
