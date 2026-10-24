//! SOURCE_ONLY / UNRUN. Literal expectations were frozen before the report leaf.
//! The original UIREQ002 scenario is observed only through the public producer.
use mori_core::sherlock_compile_artifact::compile_draft_host;
use serde_json::Value;

const SOURCE: &str = include_str!("fixtures/draft-profile/fragment.mori");
const CONFIG: &str = include_str!("fixtures/draft-compile/config.json");
const TARGET: &str = include_str!("fixtures/draft-compile/target.json");
const EXPECTED: &str = include_str!("fixtures/draft-profile/EXPECTED.json");

#[test]
fn public_admitted_first_order_fragment_reports_complete_supported_profile() {
    let artifact = compile_draft_host(SOURCE, CONFIG, TARGET).unwrap_or_else(|failure| {
        panic!(
            "public preparation failed: {}",
            serde_json::to_string(&failure).unwrap()
        )
    });
    let actual = serde_json::to_value(&artifact).expect("public returned artifact");
    let expected: Value = serde_json::from_str(EXPECTED).expect("independent literal report");
    // Full report equality catches absent fields and unexpected support claims.
    // Reading the returned JSON also catches a leaf that exists but is not wired.
    assert_eq!(actual["supported_profile"], expected);
    assert_eq!(actual["execution_profile"], expected["execution_profile"]);
    assert_eq!(actual["source"]["language"], expected["source_language"]);
    assert_eq!(actual["source"]["semantics"], expected["source_semantics"]);
    // shared::IdentityC fixes 2 as the selected B profile; ProgramView has no
    // profile member. Compare the real owner projection, not invented fields.
    assert_eq!(actual["admission"]["identity"]["profile_id"], 2);
    assert_eq!(actual["admission"]["bounds"], expected["extents"]["bounds"]);
    assert_eq!(actual["config"]["layout"], expected["extents"]["layout"]);
    assert_eq!(
        actual["config"]["preprocessing"],
        expected["extents"]["preprocessing"]
    );
    // This producer still observes one freshly admitted genesis, never a run.
    assert_eq!(actual["observation"]["evaluation_performed"], false);
    assert_eq!(actual["evidence"]["host_observation"], "genesis-only");
    assert_eq!(
        actual["correspondence"]["proof"]["semantic_correspondence"],
        "unproved"
    );
    assert_eq!(actual["evidence"]["acceptance"], "unestablished");
}

#[test]
fn public_profile_reports_this_invocations_distinct_declared_extents() {
    // Independently chosen explicit inputs, not values obtained from subject
    // output or report helpers. All table capacities exceed the base fixture.
    let bounds = [
        ("code_rows", "97"),
        ("function_rows", "5"),
        ("code_child_slots", "101"),
        ("parameter_slots", "103"),
        ("pattern_rows", "107"),
        ("match_plan_rows", "109"),
        ("match_arm_rows", "113"),
        ("type_rows", "127"),
        ("constructor_rows", "131"),
        ("constructor_field_slots", "137"),
        ("max_arity", "139"),
        ("constant_nodes", "149"),
        ("constant_field_slots", "151"),
        ("data_nodes", "157"),
        ("data_field_slots", "163"),
        ("environment_cells", "167"),
        ("frames", "173"),
        ("helper_tasks", "179"),
        ("serialization_bytes", "196613"),
        ("max_reference_work", "8191"),
        ("max_helper_work", "8209"),
        ("step_cap", "8231"),
    ];
    let mut config: Value = serde_json::from_str(CONFIG).unwrap();
    let mut expected: Value = serde_json::from_str(EXPECTED).unwrap();
    for (field, literal) in bounds {
        config["bounds"][field] = literal.into();
        expected["extents"]["bounds"][field] = literal.into();
    }
    for (field, literal) in [("max_source_bytes", "8191"), ("max_type_depth", "37")] {
        config["preprocessing"][field] = literal.into();
        expected["extents"]["preprocessing"][field] = literal.into();
    }
    config["layout"]["capacities"]["name_bytes"] = "521".into();
    expected["extents"]["layout"]["capacities"]["name_bytes"] = "521".into();
    config["fuel"]["value"] = "73".into();
    expected["extents"]["fuel"]["value"] = "73".into();
    config["stack"]["value"] = "11".into();
    expected["extents"]["stack"]["value"] = "11".into();
    let artifact =
        compile_draft_host(SOURCE, &config.to_string(), TARGET).unwrap_or_else(|failure| {
            panic!(
                "public preparation failed: {}",
                serde_json::to_string(&failure).unwrap()
            )
        });
    let actual = serde_json::to_value(&artifact).unwrap();
    assert_eq!(actual["supported_profile"], expected);
    assert_eq!(actual["admission"]["bounds"], expected["extents"]["bounds"]);
    assert_eq!(actual["observation"]["evaluation_performed"], false);
}
