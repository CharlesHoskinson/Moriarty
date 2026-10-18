
// Public consumer coverage for already admitted nominal arithmetic. Expected
// bytes are hand-derived fixtures, not output from the checker or expander.
#[test]
fn quantity_min_selects_both_operands_through_public_source6_expansion() {
    let files = Files::new();
    fs::write(
        files.0.join("capped.mori"),
        fixture!("capped-remittance.mori"),
    )
    .unwrap();
    fs::write(
        files.0.join("capped.json"),
        fixture!("capped-remittance.scenario.json"),
    )
    .unwrap();

    let checked = files.invoke(&["check", "capped.mori", "--json"]);
    assert_eq!(checked.status.code(), Some(0));
    assert!(checked.stderr.is_empty());
    let report: Value = serde_json::from_slice(&checked.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    let actions = report["actions"].as_array().unwrap();
    assert_eq!(actions.len(), 2);
    for (action, name) in actions.iter().zip(["pay_tight", "pay_loose"]) {
        assert_eq!(action["name"], name);
        assert_eq!(action["support"], "local_s0");
    }

    // 75 * 3 = 225 atoms. The tight minimum chooses b=210; the loose minimum
    // chooses a=225. The atoms(...,0) fee retains its input cell but adds no
    // fee Credit proposal. Both are complete Source/6 text comparisons.
    for (action, expected) in [
        ("pay_tight", fixture!("capped-tight.source6")),
        ("pay_loose", fixture!("capped-loose.source6")),
    ] {
        let output = files.invoke(&[
            "--quiet",
            "expand",
            "capped.mori",
            "--action",
            action,
            "--scenario",
            "capped.json",
            "--emit-source6-transfer",
        ]);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        assert_eq!(output.stdout.as_slice(), expected);
    }
}
