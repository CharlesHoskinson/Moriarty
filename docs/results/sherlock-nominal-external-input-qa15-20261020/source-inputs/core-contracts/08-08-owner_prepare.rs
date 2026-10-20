pub fn prepare_ext_program_types(
    program: super::owner_ext_source_program::ExtProgram,
    layout: &LayoutSchemaC,
    domain: super::owner_ext_source_program::ExtDomain,
    locator: Option<LocatorC>,
) -> Result<PreparedExtProgramSource, super::owner_ext_type_builder::ExtTypeBuildError> {
    use super::nominal_declaration_order::walk_declaration_type_occurrences;
    use super::owner_ext_type_builder::{CheckedTypeDeclView, ExtTypeBuildError};
    if layout.boundary_capacities_present {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature {
            feature: FeatureKind::BoundarySection,
            locator: None,
        }));
    }
    if !domain.is_finance_absent() {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature {
            feature: FeatureKind::FinancialInput,
            locator: None,
        }));
    }
    let (interned_names, decl_name_ids) = collect_ext_program_names(&program, layout)?;
    let pass_t = {
        let view = CheckedTypeDeclView::new_checked(
            domain.grammar_domain,
            &program.types,
            &decl_name_ids,
            &layout.bounds,
        )?;
        walk_declaration_type_occurrences(&view, |occ, _path, ty_expr| {
            view.validate_declaration_type_occurrence(occ, ty_expr)
        })?;
        run_pass_t(domain.grammar_domain, &view, &program.functions, &program.entry, locator)?
    };
    Ok(PreparedExtProgramSource {
        program,
        domain,
        interned_names,
        decl_name_ids,
        tables: pass_t.tables,
        t3_records: pass_t.records,
    })
}
