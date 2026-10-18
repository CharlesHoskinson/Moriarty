//! shared::owner_ext_prepare_types -- proposed nonsecurity T1/T2 orchestration.
//!
//! The input is the designated owner's checked declaration view. Every source
//! read, type-expression resolution, key discovery, capacity check and table
//! mutation remains in that owner's exact API. This module only supplies the
//! prescribed source order and drains the owner's FIFO without native recursion.
//!
//! This leaf returns real type/constructor/field tables. It does not implement
//! T3's whole-program source traversal, pass B/F/C/P, or prepare/admitExt.
//! The supplied locator is forwarded unchanged; its normative call-site binding
//! remains a designated-owner dependency. Module placement is prescribed by Root.

use super::owner_ext_type_builder::{
    BuiltAdtTypeTables, CheckedTypeDeclView, ExtTypeBuildError, OwnerExtTypeBuilder,
};
use super::LocatorC;
use super::nominal_declaration_order::for_each_constructor_field;

/// Instantiate all non-parameterized declarations and every nominal instance
/// discovered from their fields, preserving T1 preassignment and T2 FIFO order.
///
/// A parameterized declaration that is unused by these fields is not instantiated
/// here. T3 must subsequently process the remaining source type uses in I-2 order;
/// this standalone declaration leaf makes no claim to provide that traversal.
pub fn instantiate_declared_types<'a>(
    view: &'a CheckedTypeDeclView<'a>,
    locator: Option<LocatorC>,
) -> Result<BuiltAdtTypeTables, ExtTypeBuildError> {
    let mut builder = OwnerExtTypeBuilder::new_checked(view)?;

    // Assign every non-parameterized declaration's final TypeId before resolving
    // any field. In particular, mutually recursive declarations see those IDs.
    for &decl in view.non_parameterized_decls() {
        builder.register_non_parameterized_decl(decl)?;
    }

    drain_pending_keys(view, &mut builder, locator)?;
    builder.finish()
}

pub(super) fn drain_pending_keys<'a>(
    view: &CheckedTypeDeclView<'a>,
    builder: &mut OwnerExtTypeBuilder<'a>,
    locator: Option<LocatorC>,
) -> Result<(), ExtTypeBuildError> {
    while builder.has_pending_keys() {
        let key = builder.begin_dequeue_next_key()?;
        for ctor_ordinal in 0..key.ctor_count {
            let ctor_id = builder.begin_constructor(&key, ctor_ordinal, locator.clone())?;
            for_each_constructor_field(view, key.decl, ctor_ordinal, |field, source_type| {
                // Only ordering belongs to this leaf. The real owner resolves
                // types, discovers keys and performs every checked table write.
                let field_type = builder.resolve_type_expr(
                    source_type,
                    Some(key.type_id),
                    locator.clone(),
                )?;
                builder.append_ctor_field(
                    &key,
                    ctor_id,
                    ctor_ordinal,
                    field.field_ordinal,
                    field_type,
                    locator.clone(),
                )
            })?;
        }
        builder.complete_dequeued_key(key)?;
    }
    Ok(())
}
