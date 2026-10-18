//! Ordinary declaration/source-occurrence order over the exact checked owner API.
//! Source-only, UNRUN; owner_ext_type_builder is not installed yet.
//! No validation, feature/refusal classification, limits, guards or table writes.
use super::owner_ext_type_builder::{CheckedTypeDeclView, DeclIdxC, ExtTypeBuildError, TypeExprSrc};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeclarationFieldOccurrence {
    pub declaration: DeclIdxC,
    pub constructor_ordinal: usize,
    pub field_ordinal: usize,
}

/// Visit complete field-type roots in source field order for one constructor.
/// This ordinary order adapter propagates every actual owner read/callback error.
pub fn for_each_constructor_field<'s, F>(
    view: &CheckedTypeDeclView<'s>, declaration: DeclIdxC, constructor_ordinal: usize,
    mut visit: F,
) -> Result<(), ExtTypeBuildError>
where F: FnMut(DeclarationFieldOccurrence, &'s TypeExprSrc) -> Result<(), ExtTypeBuildError> {
    for field_ordinal in 0..view.field_count(declaration, constructor_ordinal)? {
        let expression=view.field_type_expr(declaration, constructor_ordinal, field_ordinal)?;
        visit(DeclarationFieldOccurrence { declaration, constructor_ordinal, field_ordinal },expression)?;
    }
    Ok(())
}

/// Local declaration/type-source occurrences only; never a ProgramImage
/// SourceOccurrenceId or an expanded runtime-step identity. Apply child positions
/// are retained even for equal-looking/shared-looking type expressions.
/// Global Pass-A classification/order is deliberately supplied by the owner.
pub fn walk_declaration_type_occurrences<'s, F>(
    view: &CheckedTypeDeclView<'s>, mut visit: F,
) -> Result<(), ExtTypeBuildError>
where F: FnMut(DeclarationFieldOccurrence, &[usize], &'s TypeExprSrc) -> Result<(), ExtTypeBuildError> {
    for declaration in 0..view.decl_count() {
        let declaration=DeclIdxC(declaration);
        for constructor_ordinal in 0..view.ctor_count(declaration)? {
            for_each_constructor_field(view,declaration,constructor_ordinal,|field,root| {
                let mut pending=vec![(root,Vec::<usize>::new())];
                while let Some((expression,path))=pending.pop() {
                    visit(field,&path,expression)?;
                    if let TypeExprSrc::Apply { args,.. }=expression {
                        for (index,child) in args.iter().enumerate().rev() {
                            let mut child_path=path.clone();
                            child_path.push(index);
                            pending.push((child,child_path));
                        }
                    }
                }
                Ok(())
            })?;
        }
    }
    Ok(())
}
