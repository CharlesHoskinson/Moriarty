fn check_and_source_inspect_route_core_two_without_legacy_consumers() {
    let f = Files::new(RESERVE);
    let output = f.command("check").arg("--json").output().unwrap();
    let checked = accepted(&output);
    assert_eq!(checked["language"], "moriarty/2");
    let inspected = accepted(&f.command("inspect").output().unwrap());
    assert_eq!(inspected["semantics"], "moriarty-core/2");
    assert_eq!(
        inspected["stage_plan"]["actions"][0]["segment_cost"],
        json!(["11", "15", "10"])
    );
}
