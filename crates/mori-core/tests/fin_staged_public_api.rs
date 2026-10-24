//! External-crate use of the stable public staged financial producer.
//! Cargo integration-test discovery is the consumption seam. UNRUN.
use mori_core::sherlock::helper_machine::shared::fin_carrier::{
    FinBindError, FinStagedPrepareError, NameId,
};
use mori_core::sherlock::helper_machine::shared::owner_host::prepare_fin_staged_source;
use mori_core::sherlock::helper_machine::shared::{
    BoundsC, FinCapacitiesC, FinPreparedStagedSource, LayoutCapacitiesC, LayoutSchemaC,
};

const SRC: &str = r##"language "moriarty/2";
agreement Collide {
  asset go = { network: "n", issuer: "i", reference: "r", scale: 0 };
  state release: UInt = 0;
  action release() { next release = 1; }
  action Collide(x: UInt) { next release = x; }
}
"##;

fn layout(name_rows: usize, name_bytes: usize) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("t".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: BoundsC::default(),
        capacities: LayoutCapacitiesC {
            name_rows,
            name_bytes,
            occurrence_path_slots: 64,
            value_slots: 64,
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}

fn caps() -> FinCapacitiesC {
    FinCapacitiesC {
        max_states: 8,
        max_assets: 8,
        max_balances: 64,
        max_liabilities: 64,
    }
}

#[test]
fn external_public_path_prepares_readonly_source() {
    let prepared: FinPreparedStagedSource =
        prepare_fin_staged_source(SRC, &layout(64, 4096), &caps()).expect("prepare");
    assert_eq!(prepared.program().name, "Collide");
    assert_eq!(prepared.program_hash(), prepared.staged().program_hash());
    assert_eq!(prepared.plan_hash(), prepared.staged().plan_hash());
    assert_eq!(
        prepared.bound_source().resolve_name("release"),
        Ok(NameId(2))
    );
    assert_eq!(prepared.bound_source().states()[0].text, "release");
    assert_eq!(prepared.bound_source().assets()[0].text, "go");
    assert_eq!(prepared.bound_source().actions()[0].text, "release");
    assert_eq!(prepared.action_inputs("Collide").expect("inputs").len(), 1);
    assert_eq!(
        prepared.fin_plan().actions.len(),
        prepared.stage_plan().actions.len()
    );
    assert_eq!(prepared.action("absent"), None);
    assert_eq!(prepared.statement("release", 99), None);
    let _ = prepared.extras();
    let _ = prepared.bound_source();
    let _ = prepared.fin_plan();
}

#[test]
fn external_public_path_names_bind_and_stage_errors() {
    assert_eq!(
        prepare_fin_staged_source(SRC, &layout(2, 4096), &caps()).unwrap_err(),
        FinStagedPrepareError::Bind(FinBindError::NameRowsExceeded {
            needed: 3,
            limit: 2
        })
    );
    let stage_error =
        prepare_fin_staged_source("language \"moriarty/1\";\n", &layout(64, 4096), &caps())
            .unwrap_err();
    assert!(matches!(stage_error, FinStagedPrepareError::Stage(_)));
}
