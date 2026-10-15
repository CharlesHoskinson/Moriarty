//! Independent finite FeePool expectations through the public pure CLI consumers.
//! No SDK query/check, keys, parameter generation, proof, VM, or network execution.
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn example() -> PathBuf {
    root().join("examples/core/fee-pool")
}
fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-fee-pool-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self, n: &str) -> PathBuf {
        self.0.join(n)
    }
    fn write(&self, n: &str, v: &Value) {
        std::fs::write(self.path(n), serde_json::to_vec_pretty(v).unwrap()).unwrap();
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn command() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
    c.args(["--color", "never"]);
    c
}
fn invoke(mut c: Command) -> Output {
    let argv = c
        .get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let o = c.output().unwrap();
    if let Some(dir) = std::env::var_os("MORI_FEE_POOL_CAPTURE_DIR") {
        let d = PathBuf::from(dir).join(format!(
            "cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(
            d.join("argv.json"),
            serde_json::to_vec_pretty(&argv).unwrap(),
        )
        .unwrap();
        std::fs::write(d.join("stdout"), &o.stdout).unwrap();
        std::fs::write(d.join("stderr"), &o.stderr).unwrap();
        std::fs::write(d.join("exit"), o.status.code().unwrap_or(-1).to_string()).unwrap();
    }
    o
}
fn status_text(o: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}
fn success(o: Output) -> Value {
    assert!(
        o.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    serde_json::from_slice(&o.stdout).unwrap_or(Value::String(
        String::from_utf8_lossy(&o.stdout).into_owned(),
    ))
}
fn simulate(source: &Path, action: &str, scenario: &Path, cap: Option<u128>) -> Output {
    let mut c = command();
    c.arg("simulate")
        .arg(source)
        .args(["--action", action, "--scenario"])
        .arg(scenario);
    if let Some(cap) = cap {
        c.args(["--logical-cap", &cap.to_string()]);
    }
    invoke(c)
}
fn program(name: &str) -> mori_core::Program {
    mori_core::parse(&std::fs::read_to_string(example().join(name)).unwrap()).unwrap()
}
#[test]
fn independent_manifest_exercises_actual_public_test_and_every_action() {
    let manifest = read(example().join("cases/expected.json"));
    assert!(manifest["cases"].as_array().unwrap().len() >= 123);
    let p = program("fee-pool.mori");
    let positive = manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["source"] == "fee-pool.mori" && c["expected"]["status"] == "accepted")
        .map(|c| c["action"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        positive,
        p.actions.iter().map(|a| a.name.as_str()).collect()
    );
    let mut c = command();
    c.arg("test").arg(example());
    let report = success(invoke(c));
    assert_eq!(report["status"], "passed");
    let root_manifest = read(example().join("mori-tests.json"));
    assert_eq!(
        report["cases"].as_array().unwrap().len(),
        root_manifest["cases"].as_array().unwrap().len()
    );
    for case in manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["expected"]["status"] == "accepted")
    {
        let mut c = command();
        c.arg("test")
            .arg(example().join("cases").join(case["name"].as_str().unwrap()));
        let report = success(invoke(c));
        assert_eq!(report["status"], "passed", "{}", case["name"]);
    }
}
#[test]
fn frozen_inventory_and_actual_core_limits() {
    let p = program("fee-pool.mori");
    mori_core::validate(&p).unwrap();
    let uints = "epoch epochClose gross metered target backstop watcherHeld stipendLimit opWeight daWeight rewardSeatCount allocatedCount opTotal daTotal watcherTotal opRemaining daRemaining opAWeight opBWeight daSeatWeight watcherWeight ordinaryAmount stipendAmount opAAmount opBAmount daAmount l2Amount midnightAmount dustAmount spendAmount claimedTotal carried";
    let bools = "recorded settled finished opARegistered opBRegistered daRegistered watcherRegistered ordinaryRecorded stipendRecorded opAAllocated opBAllocated daAllocated spendIssued ordinaryClaimed stipendClaimed opAClaimed opBClaimed daClaimed l2Claimed midnightClaimed dustClaimed spendClaimed";
    let states = uints
        .split_whitespace()
        .map(|n| (n, mori_core::Type::UInt))
        .chain(bools.split_whitespace().map(|n| (n, mori_core::Type::Bool)))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        p.states
            .iter()
            .map(|s| (s.name.as_str(), s.ty))
            .collect::<BTreeMap<_, _>>(),
        states
    );
    assert_eq!(
        (p.states.len(), p.actions.len(), p.assets.len()),
        (54, 23, 1)
    );
    let expected_inputs = [
        (
            "intake",
            "epochId grossAmount closeObservedAt initialTarget watcherStipendLimit",
        ),
        ("registerOperatorA", "weight attested effectiveWeight"),
        ("registerOperatorB", "weight attested effectiveWeight"),
        ("registerDa", "weight attested effectiveWeight"),
        ("registerWatcher", "weight attested effectiveWeight"),
        ("submitOrdinary", "amount"),
        ("submitStipend", "amount"),
        ("settle", "opQ opR daQ daR watcherQ watcherR l2Q l2R refill"),
        ("allocateOperatorA", "denominator quotient remainder"),
        ("allocateOperatorB", "denominator quotient remainder"),
        ("allocateDa", "denominator quotient remainder"),
        ("finish", "refill"),
        ("spendWatcher", "kind amount"),
        ("setTarget", "newTarget"),
        ("claimOrdinary", "observedAt"),
        ("claimStipend", "observedAt"),
        ("claimOperatorA", "observedAt"),
        ("claimOperatorB", "observedAt"),
        ("claimDa", "observedAt"),
        ("claimL2", "observedAt"),
        ("claimMidnight", "observedAt"),
        ("claimDust", "observedAt"),
        ("claimWatcherSpend", "observedAt"),
    ];
    for (name, inputs) in expected_inputs {
        let action = p.actions.iter().find(|a| a.name == name).unwrap();
        assert_eq!(
            action
                .inputs
                .iter()
                .map(|i| i.name.as_str())
                .collect::<Vec<_>>(),
            inputs.split_whitespace().collect::<Vec<_>>(),
            "inputs {name}"
        );
        for input in &action.inputs {
            assert_eq!(
                input.ty,
                if input.name == "attested" {
                    mori_core::Type::Bool
                } else {
                    mori_core::Type::UInt
                }
            );
        }
    }
    assert_eq!(
        serde_json::to_value(&p.assets[0]).unwrap(),
        json!({"name":"FEES","identity":{"network":"local","issuer":"fee-pool-source","reference":"SERVICE","scale":0}})
    );
    for (action, creditor, duty) in [
        ("claimOrdinary", "cost-provider", "cost_ordinary"),
        ("claimStipend", "watcher", "cost_stipend"),
        ("claimOperatorA", "operator-a", "reward_operator_a"),
        ("claimOperatorB", "operator-b", "reward_operator_b"),
        ("claimDa", "da-provider", "reward_da"),
        ("claimL2", "l2-treasury", "treasury_l2"),
        ("claimMidnight", "midnight-treasury", "treasury_midnight"),
        ("claimDust", "l2-treasury", "treasury_dust"),
        ("claimWatcherSpend", "watcher", "watcher_spend"),
    ] {
        let a = p.actions.iter().find(|a| a.name == action).unwrap();
        let transfers = a
            .statements
            .iter()
            .filter_map(|s| {
                if let mori_core::Statement::Transfer {
                    asset, from, to, ..
                } = s
                {
                    Some((asset.as_str(), from.as_str(), to.as_str()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(transfers, vec![("FEES", "fee-pool", creditor)]);
        assert!(
            a.statements
                .iter()
                .any(|s| matches!(s,mori_core::Statement::Discharge{name,..} if name==duty))
        );
    }

    assert!(p.expressions.len() <= mori_core::MAX_EXPRESSIONS);
    assert_eq!(p.states.len() + p.actions.len() + p.assets.len(), 78);
    let mut duties = BTreeSet::new();
    for a in &p.actions {
        assert!(a.statements.len() <= mori_core::MAX_STATEMENTS);
        assert!(a.inputs.len() <= 256);
        for stmt in &a.statements {
            if let mori_core::Statement::Liability { name, .. } = stmt {
                duties.insert(name);
            }
        }
    }
    assert_eq!(duties.len(), 9);
    let metrics = json!({"states":p.states.len(),"actions":p.actions.len(),"assets":p.assets.len(),"declarations":78,"expressions":p.expressions.len(),"statements":p.actions.iter().map(|a|(&a.name,a.statements.len())).collect::<BTreeMap<_,_>>()});
    if let Some(dir) = std::env::var_os("MORI_FEE_POOL_CAPTURE_DIR") {
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            PathBuf::from(dir).join("core-counts.json"),
            serde_json::to_vec_pretty(&metrics).unwrap(),
        )
        .unwrap();
    }
}
#[test]
fn actual_poststate_chains_and_conserves_shared_custody() {
    let p = program("fee-pool.mori");
    let manifest = read(example().join("cases/expected.json"));
    let names = [
        "intake-funded",
        "register-OperatorA",
        "register-OperatorB",
        "register-Da",
        "register-Watcher",
        "submit-ordinary",
        "submit-stipend",
        "settle-original-surplus",
        "allocate-OperatorB",
        "allocate-OperatorA",
        "allocate-Da",
        "finish-dust",
        "target-set-after-finish",
        "watcher-bounty",
        "claim-cost_ordinary",
        "claim-cost_stipend",
        "claim-reward_operator_a",
        "claim-reward_operator_b",
        "claim-reward_da",
        "claim-treasury_l2",
        "claim-treasury_midnight",
        "claim-treasury_dust",
        "claim-watcher_spend",
    ];
    let mut previous: Option<mori_core::Snapshot> = None;
    for name in names {
        let c = manifest["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["name"] == name)
            .unwrap();
        let mut s: mori_core::Scenario =
            serde_json::from_value(read(example().join(c["scenario"].as_str().unwrap()))).unwrap();
        if let Some(prev) = previous {
            assert_eq!(s.state, prev.state, "prestate {name}");
            assert_eq!(s.balances, prev.balances, "balances {name}");
            assert_eq!(s.liabilities, prev.liabilities, "duties {name}");
            s.state = prev.state;
            s.balances = prev.balances;
            s.liabilities = prev.liabilities;
        }
        let out = mori_core::simulate(&p, c["action"].as_str().unwrap(), &s).unwrap();
        assert_eq!(
            serde_json::to_value(&out.post).unwrap(),
            c["expected"]["post"],
            "post {name}"
        );
        assert_eq!(
            serde_json::to_value(&out.effects).unwrap(),
            c["expected"]["effects"],
            "effects {name}"
        );
        let n = |k: &str| match out.post.state[k] {
            mori_core::Value::UInt(n) => n.0,
            _ => panic!("UInt"),
        };
        let pool = out
            .post
            .balances
            .iter()
            .find(|b| b.account == "fee-pool")
            .unwrap()
            .amount
            .0;
        let outstanding: u128 = out
            .post
            .liabilities
            .iter()
            .filter(|l| l.debtor == "fee-pool")
            .map(|l| l.amount.0)
            .sum();
        assert_eq!(
            pool,
            outstanding + n("carried") + n("backstop") + n("watcherHeld"),
            "custody {name}"
        );
        assert_eq!(n("gross"), pool + n("claimedTotal"), "gross {name}");
        previous = Some(out.post);
    }
    let final_state = previous.unwrap();
    assert_eq!(
        final_state
            .balances
            .iter()
            .find(|b| b.account == "fee-pool")
            .unwrap()
            .amount
            .0,
        2
    );
    assert!(
        final_state
            .liabilities
            .iter()
            .filter(|l| l.debtor == "fee-pool")
            .all(|l| l.amount.0 == 0)
    );
    assert_eq!(
        final_state
            .liabilities
            .iter()
            .find(|l| l.name == "unrelated_duty")
            .unwrap()
            .amount
            .0,
        11
    );
    assert_eq!(
        final_state
            .balances
            .iter()
            .find(|b| b.account == "unrelated-observer")
            .unwrap()
            .amount
            .0,
        42
    );
}
#[test]
fn public_simulate_budget_boundary_and_atomic_refusals() {
    let source = example().join("fee-pool.mori");
    let scenario = example().join("cases/settle-original-surplus.json");
    let accepted = success(simulate(&source, "settle", &scenario, Some(100000)));
    let used = accepted["logical"]["used"]
        .as_str()
        .unwrap()
        .parse::<u128>()
        .unwrap();
    assert!(used > 0);
    let exact = success(simulate(&source, "settle", &scenario, Some(used)));
    assert_eq!(accepted["execution"], exact["execution"]);
    let o = simulate(&source, "settle", &scenario, Some(used - 1));
    assert!(!o.status.success());
    let refusal: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(refusal["status"], "rejected");
    assert!(refusal.get("execution").is_none());
    assert_eq!(refusal["error"]["code"], "mori::core::meter");
    for (file, action) in [
        ("claim-credit-overflow-atomic", "claimOrdinary"),
        ("claim-insufficient-custody-atomic", "claimOrdinary"),
        ("watcher-held-u64-overflow", "settle"),
        ("zero-denominator", "allocateOperatorA"),
    ] {
        let path = example().join(format!("cases/{file}.json"));
        let before = std::fs::read(&path).unwrap();
        let o = simulate(&source, action, &path, None);
        assert!(!o.status.success(), "{file}");
        let v: Value = serde_json::from_slice(&o.stdout).unwrap();
        assert_eq!(v["status"], "rejected");
        assert!(
            v.get("execution").is_none() && v.get("post").is_none() && v.get("effects").is_none()
        );
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
}
fn static_command(
    source: &str,
    bindings: &Path,
    limits: &Path,
    destination: &Path,
    inspect: bool,
    policy: Option<&Path>,
) -> Command {
    let mut c = command();
    c.arg("compile-call")
        .arg(example().join(source))
        .args(["--all-actions", "--static-bindings"])
        .arg(bindings)
        .args([
            "--authorization",
            if policy.is_some() {
                "signed-v2"
            } else {
                "unsigned-mapped-roles"
            },
            "--work-limits",
        ])
        .arg(limits);
    if let Some(p) = policy {
        c.arg("--signer-policy").arg(p);
    }
    c.arg(if inspect { "--inspect-static" } else { "--out" })
        .arg(destination);
    c
}
#[test]
fn public_source_static_all_actions_and_separate_process_reopen_both_programs() {
    for (source, bindings, count) in [
        ("fee-pool.mori", "static-bindings.json", 23),
        ("contrasting.mori", "contrasting.static-bindings.json", 4),
    ] {
        let t = Temp::new();
        let binding = example().join(bindings);
        let limits = example().join("work-limits.json");
        let out = t.path("out");
        let o = invoke(static_command(source, &binding, &limits, &out, false, None));
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        assert!(
            status_text(&o)
                .contains("source-static-mapped-roles-unproven-uninstalled-unauthenticated")
        );
        let marker = read(out.join("static-compile.json"));
        assert_eq!(
            marker["profile"],
            "moriarty-mapped-role-source-static-bundle/1"
        );
        assert_eq!(
            marker["qualification"],
            "source-static-mapped-roles-unproven-uninstalled-unauthenticated"
        );
        assert_eq!(
            marker["authorization"],
            json!({"mode":"unsigned-mapped-roles"})
        );
        assert_eq!(marker["actions"].as_array().unwrap().len(), count);
        let o = invoke(static_command(source, &binding, &limits, &out, true, None));
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        assert!(status_text(&o).contains("source-static-recompile-admitted"));
        assert!(status_text(&o).contains(
            "; qualification source-static-mapped-roles-unproven-uninstalled-unauthenticated"
        ));
        assert_eq!(std::fs::read_dir(&out).unwrap().count(), 2 * count + 2);
        let names = marker["actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["name"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            program(source)
                .actions
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
        );
        let mut max_groups = 0usize;
        let mut max_public = 0usize;
        for i in 0..count {
            let m = read(out.join(format!("action-{i:06}.manifest.json")));
            assert_eq!(m["format"], "moriarty-mapped-role-source-static-relation/2");
            assert_eq!(
                m["qualification"],
                "source-static-mapped-debits-unauthenticated"
            );
            assert_eq!(m["compiler_hash"], marker["build"]);
            assert!(m.get("signed").is_none());
            let p = program(source);
            let action = &p.actions[i];
            let debit_roles = action
                .statements
                .iter()
                .filter_map(|s| match s {
                    mori_core::Statement::Transfer { from, .. }
                    | mori_core::Statement::Fee { from, .. } => Some(from.as_str()),
                    _ => None,
                })
                .collect::<BTreeSet<_>>();
            let binding_json = read(&binding);
            let mut debit_rows = binding_json["roles"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| debit_roles.contains(r["role"].as_str().unwrap()))
                .cloned()
                .collect::<Vec<_>>();
            debit_rows.sort_by(|a, b| a["role"].as_str().unwrap().cmp(b["role"].as_str().unwrap()));
            assert_eq!(
                m["debit_sources"],
                json!(debit_rows),
                "debit inventory {}",
                action.name
            );
            assert_eq!(
                m["open_predicates"],
                json!([
                    "debit-source custody and funding ownership",
                    "role and beneficiary authorization",
                    "authenticated predecessor and deployment",
                    "economic intention authority",
                    "final intent/history/PCD",
                    "native proof and ledger settlement"
                ])
            );
            let groups = m["groups"].as_array().unwrap();
            max_groups = max_groups.max(groups.len());
            max_public = max_public.max(
                2 + groups
                    .iter()
                    .map(|g| g["field_count"].as_u64().unwrap() as usize)
                    .sum::<usize>(),
            );
        }
        let mut exact_limits = read(&limits);
        exact_limits["max_actions"] = json!(count);
        exact_limits["max_operations"] = json!(max_groups);
        exact_limits["max_public_inputs"] = json!(max_public);
        t.write("exact-limits.json", &exact_limits);
        let exact = invoke(static_command(
            source,
            &binding,
            &t.path("exact-limits.json"),
            &out,
            true,
            None,
        ));
        assert!(
            exact.status.success(),
            "exact static work boundary: {}",
            String::from_utf8_lossy(&exact.stderr)
        );
        for (dimension, bound) in [
            ("max_actions", count),
            ("max_operations", max_groups),
            ("max_public_inputs", max_public),
        ] {
            assert!(bound > 0);
            let mut low = exact_limits.clone();
            low[dimension] = json!(bound - 1);
            t.write("low-limits.json", &low);
            let refused = t.path(dimension);
            let o = invoke(static_command(
                source,
                &binding,
                &t.path("low-limits.json"),
                &refused,
                false,
                None,
            ));
            assert!(!o.status.success(), "one-under {dimension}");
            assert!(!refused.exists());
        }
        if let Some(dir) = std::env::var_os("MORI_FEE_POOL_CAPTURE_DIR") {
            let v = json!({"source":source,"actions":count,"max_operation_groups":max_groups,"max_public_fields":max_public,"total_bundle_bytes":std::fs::read_dir(&out).unwrap().map(|f|f.unwrap().metadata().unwrap().len()).sum::<u64>()});
            std::fs::write(
                PathBuf::from(dir).join(format!("static-work-{source}.json")),
                serde_json::to_vec_pretty(&v).unwrap(),
            )
            .unwrap();
        }
        if let Some(dir) = std::env::var_os("MORI_FEE_POOL_CAPTURE_DIR") {
            let dir = PathBuf::from(dir).join(format!("static-{source}"));
            std::fs::create_dir_all(&dir).unwrap();
            for f in std::fs::read_dir(&out).unwrap() {
                let f = f.unwrap();
                std::fs::copy(f.path(), dir.join(f.file_name())).unwrap();
            }
        }
        // Caller-supplied current source and bindings remain authoritative on reopen.
        for field in ["network", "asset", "recipient"] {
            let mut altered = read(&binding);
            match field {
                "network" => altered["network"] = json!("alternate-domain"),
                "asset" => altered["assets"][0]["identity"]["reference"] = json!("WRONG"),
                _ => altered["duties"][0]["creditor"] = json!("unregistered-recipient"),
            }
            t.write("altered-bindings.json", &altered);
            assert!(
                !invoke(static_command(
                    source,
                    &t.path("altered-bindings.json"),
                    &limits,
                    &out,
                    true,
                    None
                ))
                .status
                .success(),
                "altered {field}"
            );
        }
        let changed_source = t.path("changed-source.mori");
        let mut raw = std::fs::read_to_string(example().join(source)).unwrap();
        raw.push_str("\n// changed current source bytes\n");
        std::fs::write(&changed_source, raw).unwrap();
        assert!(
            !invoke(static_command(
                changed_source.to_str().unwrap(),
                &binding,
                &limits,
                &out,
                true,
                None
            ))
            .status
            .success()
        );
        // Reopen must compare current inputs and artifact bytes, rather than trust its own marker.
        let changed = out.join("action-000000.ir.bin");
        let mut bytes = std::fs::read(&changed).unwrap();
        bytes[0] ^= 1;
        std::fs::write(changed, bytes).unwrap();
        assert!(
            !invoke(static_command(source, &binding, &limits, &out, true, None))
                .status
                .success()
        );
    }
}
#[test]
fn signed_whole_program_rejects_real_mixed_payers_and_stale_inputs_publish_nothing() {
    let t = Temp::new();
    for (source, bindings) in [
        ("fee-pool.mori", "static-bindings.json"),
        ("contrasting.mori", "contrasting.static-bindings.json"),
    ] {
        let out = t.path(source);
        let mut c = command();
        c.arg("compile-call")
            .arg(example().join(source))
            .args([
                "--all-actions",
                "--authorization",
                "unsigned",
                "--static-bindings",
            ])
            .arg(example().join(bindings))
            .arg("--work-limits")
            .arg(example().join("work-limits.json"))
            .arg("--out")
            .arg(&out);
        let o = invoke(c);
        assert!(!o.status.success());
        assert!(
            String::from_utf8_lossy(&o.stderr)
                .contains("mori::ledger::source-profile: one-public-payer")
        );
        assert!(!out.exists());
    }
    let mut b = read(example().join("static-bindings.json"));
    b["payer"] = json!("fee-pool");
    t.write("bindings.json", &b);
    let mut policy = read(root().join("examples/core/static-compile/signer-policy.json"));
    policy["owner"] = json!("fee-pool");
    policy["beneficiary"] = json!("operator-a");
    t.write("policy.json", &policy);
    let limits = example().join("work-limits.json");
    let out = t.path("signed");
    let o = invoke(static_command(
        "fee-pool.mori",
        &t.path("bindings.json"),
        &limits,
        &out,
        false,
        Some(&t.path("policy.json")),
    ));
    assert!(!o.status.success());
    let message = format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(
        message.contains("mori::ledger::source-profile: one-public-payer"),
        "must reach the unchanged strict native profile mixed-payer refusal: {message}"
    );
    assert!(!out.exists());
    b["source_hash"] = json!("0".repeat(64));
    t.write("stale.json", &b);
    assert!(
        !invoke(static_command(
            "fee-pool.mori",
            &t.path("stale.json"),
            &limits,
            &t.path("stale"),
            false,
            None
        ))
        .status
        .success()
    );
    assert!(!t.path("stale").exists());
}
#[test]
fn malformed_uint_and_extra_destination_refuse_the_actual_cli() {
    let t = Temp::new();
    let original = read(example().join("cases/claim-cost_ordinary.json"));
    for literal in ["-1", "01", "340282366920938463463374607431768211456"] {
        let mut s = original.clone();
        s["inputs"]["observedAt"]["value"] = json!(literal);
        t.write("scenario.json", &s);
        assert!(
            !simulate(
                &example().join("fee-pool.mori"),
                "claimOrdinary",
                &t.path("scenario.json"),
                None
            )
            .status
            .success()
        );
    }
    let mut s = original;
    s["inputs"]["recipient"] = json!({"type":"UInt","value":"1"});
    t.write("extra.json", &s);
    let o = simulate(
        &example().join("fee-pool.mori"),
        "claimOrdinary",
        &t.path("extra.json"),
        None,
    );
    assert!(!o.status.success());
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["error"]["code"], "mori::core::input");
}
