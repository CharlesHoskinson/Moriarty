fn reported_core_profiles_select_real_source_only_cli_consumers() {
    let f = Files::new();
    let report = json(&f.run(&["capabilities", "--json"]));
    for (id, language, semantics, body) in [
        (
            "rust-core1",
            "moriarty/1",
            "moriarty-core/1",
            "next count = count + 1;",
        ),
        (
            "rust-core2",
            "moriarty/2",
            "moriarty-core/2",
            "await review(ok: Bool) when ok; next count = count + 1;",
        ),
    ] {
        let profile = row(&report, "profiles", id);
        assert_eq!(profile["sourceProfile"], language);
        assert_eq!(profile["semanticsProfile"], semantics);
        let source = format!(
            "{}\nagreement CapabilityProbe {{ state count: UInt = 0; action run() {{ {body} }} }}\n",
            profile["sourceHeader"].as_str().unwrap()
        );
        fs::write(f.0.join("source.mori"), source).unwrap();
        let before = f.snapshot();
        let checked = json(&f.run(&["check", "source.mori", "--json"]));
        assert_eq!(checked["status"], "accepted");
        assert_eq!(checked["language"], language);
        let inspected = json(&f.run(&["inspect", "source.mori"]));
        assert_eq!(inspected["language"], language);
        assert_eq!(inspected["semantics"], semantics);
        assert_eq!(inspected.get("enforcement_report"), Some(&Value::Null));
        if language == "moriarty/2" {
            assert_eq!(inspected["status"], "accepted");
            assert_eq!(inspected["qualification"], "local-reference-stage");
            assert_eq!(inspected["authentication"], "none");
            assert_eq!(inspected["consumption"], "untracked");
            assert!(row(&report, "consumers", "core2-inspect")["outputProfile"].is_null());
            assert!(inspected.get("format").is_none());
            assert_eq!(inspected["stage_plan"]["format"], "moriarty-stage-plan/1");
        } else {
            assert!(inspected.get("status").is_none());
            assert_eq!(inspected["qualification"], "source-only-unqualified");
            assert_eq!(inspected["format"], "moriarty-enforcement-report/1");
        }
        assert_eq!(before, f.snapshot());
    }
}
