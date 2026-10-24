//! SOURCE_ONLY / UNRUN. Ordinary config projection/selection tests written first.
use super::*;
use serde_json::json;

#[test]
fn preprocessing_fields_roundtrip_and_project_without_extent_aliases() {
    let input = json!({"max_source_bytes":"1","max_tokens":"2","max_functions":"3",
        "max_parameters_per_function":"4","max_total_parameters":"5","max_ast_nodes":"6",
        "max_parse_depth":"7","max_structural_ast_depth":"8","max_type_depth":"9","max_copy_depth":"10",
        "max_copy_expr_nodes":"11","max_copy_match_arms":"12","max_copy_name_bytes":"13",
        "max_copy_list_elements":"14","max_copy_tree_nodes":"15","max_copy_tree_depth":"16"});
    let extents: DraftPreprocessingExtents = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::to_value(extents).unwrap(), input);
    assert_eq!(extents.parser_budget(), crate::sherlock_draft::DraftParseBudget {
        max_source_bytes:1, max_tokens:2, max_functions:3,
        max_parameters_per_function:4, max_total_parameters:5, max_ast_occurrences:6,
        max_parser_recursion_depth:7, max_structural_ast_depth:8,
    });
    // These two supplied extents remain separate from the parser projection.
    // The real checker consumes the exact usize max_type_depth.0; no helper/cap.
    assert_eq!(extents.max_type_depth.0, 9);
    assert_eq!(extents.max_copy_depth.0, 10);
    assert_eq!(extents.copy_budget(), crate::sherlock::helper_machine::source_input_bridge::SourceCopyBudget {
        max_functions:3, max_parameters_per_function:4, max_total_parameters:5,
        max_expr_nodes:11, max_copy_depth:10, max_match_arms:12,
        max_name_bytes:13, max_list_elements:14, max_tree_nodes:15, max_tree_depth:16,
    });
}

#[test]
fn normal_core_and_draft_selection_preserve_their_existing_profiles() {
    assert!(matches!(select_source(None, Some("pay"), false), Ok(SourceSelection::CoreAction("pay"))));
    assert!(matches!(select_source(Some("moriarty/1"), Some("pay"), false), Ok(SourceSelection::CoreAction("pay"))));
    assert!(matches!(select_source(Some(SOURCE_PROFILE), None, true), Ok(SourceSelection::DraftEntry)));
    assert_eq!(SOURCE_PROFILE, "moriarty-sherlock-draft/1");
    assert_eq!(SOURCE_SEMANTICS, "moriarty-sherlock-dedicated/1");
    assert_eq!(ARTIFACT_FORMAT, "sherlock-host-artifact/2");
}
