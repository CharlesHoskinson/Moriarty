Source public-main/crates/mori/src/commands/check.rs 21-60; fullfilephysical.
```
21: pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
22:     let name = args.file.display().to_string();
23:     let source = super::core_io::read_source(&args.file)?;
24: 
25:     if !args.json {
26:         shell.status("Checking", &name);
27:     }
28:     let started = Instant::now();
29:     if mori_lexer::first_token_text(&source).is_ok_and(|token| token == "language") {
30:         let staged = mori_core::language(&source).is_ok_and(|language| language == "moriarty/2");
31:         let result = if staged {
32:             mori_core::stage::load(&source).map(|program| program.program().clone())
33:         } else {
34:             mori_core::parse(&source)
35:         };
36:         if args.json {
37:             let report = match &result {
38:                 Ok(program) => {
39:                     serde_json::json!({"status":"accepted", "file":name,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
40:                 }
41:                 Err(error) => {
42:                     serde_json::json!({"status":"rejected","file":name,"diagnostics":[error]})
43:                 }
44:             };
45:             if staged {
46:                 super::simulate::print_stage_report(report)?;
47:             } else {
48:                 super::core_io::print(&report)?;
49:             }
50:         } else {
51:             match &result {
52:                 Ok(_) => shell.status("Finished", "bounded core check"),
53:                 Err(error) => shell.error(error),
54:             }
55:         }
56:         return Ok(if result.is_ok() {
57:             ExitCode::SUCCESS
58:         } else {
59:             ExitCode::FAILURE
60:         });
```

Source public-main/crates/mori/src/commands/simulate.rs 109-115; fullfilephysical.
```
109:     let source = super::core_io::read_source(&args.file)?;
110:     // Hook: /2 routing. A /2 start keeps every core failure labelled.
111:     if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {
112:         return run_stage_start(&args, &source, action, scenario_path);
113:     }
114:     let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
115:     let scenario = super::core_io::read_json(scenario_path)?;
```

Source public-main/crates/mori/src/commands/simulate.rs 136-147; fullfilephysical.
```
136:     match mori_core::simulate(&program, action, &scenario) {
137:         Ok(execution) => {
138:             super::core_io::print(
139:                 &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
140:             )?;
141:             Ok(ExitCode::SUCCESS)
142:         }
143:         Err(error) => {
144:             super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
145:             Ok(ExitCode::FAILURE)
146:         }
147:     }
```

Source public-main/crates/mori/src/commands/test.rs 62-89; fullfilephysical.
```
62:         let source = core_io::read_source(&args.dir.join(&case.source))?;
63:         let scenario: mori_core::Scenario = core_io::read_json(&args.dir.join(&case.scenario))?;
64:         let result = mori_core::parse(&source)
65:             .and_then(|p| mori_core::simulate(&p, &case.action, &scenario));
66:         let passed = match (&case.expected, &result) {
67:             (Expected::Accepted { post, effects }, Ok(actual)) => {
68:                 post == &actual.post && effects == &actual.effects
69:             }
70:             (Expected::Rejected { code }, Err(error)) => code == &error.code,
71:             _ => false,
72:         };
73:         if !passed {
74:             failed += 1;
75:         }
76:         reports.push(serde_json::json!({"name":case.name,"passed":passed,"actual":result}));
77:     }
78:     shell.status(
79:         "Tested",
80:         format!("{} passed; {} failed", reports.len() - failed, failed),
81:     );
82:     let report = aggregate_report(failed == 0, reports)?;
83:     println!("{report}");
84:     Ok(if failed == 0 {
85:         ExitCode::SUCCESS
86:     } else {
87:         ExitCode::FAILURE
88:     })
89: }
```

Source public-main/crates/mori/Cargo.toml 10-25; fullfilephysical.
```
10: [features]
11: default = ["native"]
12: native = [
13:     "dep:mori-backend",
14:     "dep:mori-ledger",
15:     "dep:futures",
16:     "dep:rand",
17:     "dep:midnight-ledger",
18:     "dep:midnight-base-crypto",
19:     "dep:midnight-coin-structure",
20:     "dep:midnight-onchain-state",
21:     "dep:midnight-onchain-runtime",
22:     "dep:midnight-storage",
23:     "dep:midnight-transient-crypto",
24:     "dep:midnight-serialize",
25: ]
```

Source public-main/RUST_GUIDELINES.md 55-60; fullfilephysical.
```
55: - Code only tests use lives in test modules. Unadopted prototypes go in
56:   `experiments/` or behind a feature, not in `src/` gated by `#[cfg(test)]`.
57: - Never hash source text to identify an ABI, deployment, key or build. Any
58:   reformat changes it. Hash the produced artifact or a declared version.
59: - No absolute paths. Locate files from `env!("CARGO_MANIFEST_DIR")`.
60: 
```

Source public-main/RUST_GUIDELINES.md 253-258; fullfilephysical.
```
253:   The CLI feature-boundary work has a narrow exception for real binary tests
254:   of public dispatch, feature availability, frozen complete output, process
255:   exits and I/O. This exception does not make a shared evaluator an independent
256:   oracle. Never snapshot `--help`; use clap schema assertions for that contract.
257: - Existing external fixture paths and test-support mounts are retained during
258:   the feature-boundary cut; they are not precedents for new paths or mounts.
```
