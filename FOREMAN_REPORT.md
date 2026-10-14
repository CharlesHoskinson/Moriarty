S2 prepares bounded first-divergence export through the actual replay CLI.

The developer uses `mori replay ... --challenge-out FILE` to export a local diagnostic.
The diagnostic binds independently reconstructed roots and the canonical candidate digest.
Input failures publish no diagnostic. Equality publishes no diagnostic.
The exporter caps escaped JSON plus its newline at 4096 bytes.
Atomic publication refuses existing destinations and competing writers.
The original replay record format and hash frames remain unchanged.

Checks passed: 28 Rust tests across four replay suites, Quint typechecking, five seeded Quint tests, formatting, and whitespace checks.
All builds used the shared mutex, two Cargo jobs, one Rust test thread, and offline dependency access.

The first CLI test failed because the parser did not support `--challenge-out`.
A later backend build reused another worktree library and could not find the new symbols.
The author removed only project package artifacts and rebuilt the current checkout.
The complete failures and final outputs remain in [FOREMAN_REPORT.json](FOREMAN_REPORT.json).

Fresh independent full-candidate audits remain required. U0, U2, and M6 remain open.
No native proof, SRS, key generation, ledger, network, or provider operation ran.
The author did not commit or push this candidate.

The JSON report records the exact owned and input digests.
Use those digests to check measurement freshness before recovery or integration.

Complete owned patch follows.

```diff
diff --git a/crates/mori-backend/src/replay.rs b/crates/mori-backend/src/replay.rs
index 6aa01b66..27f1b2b6 100644
--- a/crates/mori-backend/src/replay.rs
+++ b/crates/mori-backend/src/replay.rs
@@ -227,7 +227,11 @@ pub fn replay(
     candidate.validate().map_err(ReplayError::Input)?;
     let actual =
         generate(source, action, scenario, target, authorization).map_err(ReplayError::Input)?;
-    let a = serde_json::to_value(&actual)
+    compare_records(&actual, candidate)
+}
+// Keep one comparison order for ordinary replay and challenge export.
+fn compare_records(actual: &ReplayRecord, candidate: &ReplayRecord) -> Result<(), ReplayError> {
+    let a = serde_json::to_value(actual)
         .map_err(|e| ReplayError::Input(error("serialization", e.to_string())))?;
     let b = serde_json::to_value(candidate)
         .map_err(|e| ReplayError::Input(error("serialization", e.to_string())))?;
@@ -312,3 +316,107 @@ pub fn replay(
     }
     unequal(&a["record_hash"], &b["record_hash"], "/record_hash")
 }
+
+pub const CHALLENGE_FORMAT: &str = "moriarty-reference-challenge/1";
+/// Local diagnostic output cap, including ASCII escapes and the final newline.
+/// This is not a Fenrir transaction limit or a measured replay certificate.
+pub const MAX_CHALLENGE_BYTES: usize = 4096;
+
+/// Public identities copied exclusively from independent reconstruction.
+/// The input and record hashes also bind the attachment-free signed statement.
+/// Signature bytes, R, s, proving keys and private witnesses are excluded.
+#[derive(Debug, Clone, Serialize)]
+pub struct ReplayChallengeContext {
+    pub language: String,
+    pub semantics: String,
+    pub network: String,
+    pub anchor: String,
+    pub program_name: String,
+    pub program_address: Option<String>,
+    pub action: String,
+    pub source_hash: String,
+    pub program_hash: String,
+    pub target_hash: String,
+    pub scenario_hash: String,
+    pub zkir_hash: String,
+    pub authority_hash: Option<String>,
+    pub authorization_mode: String,
+    pub at: Option<UInt>,
+    pub input_hash: String,
+    pub trace_hash: String,
+    pub record_hash: String,
+}
+/// A bounded first-divergence diagnostic, not a proof or fraud-game transcript.
+/// The candidate digest hashes the entire canonical typed record, including
+/// its possibly false declared hashes. It never trusts its record_hash label.
+#[derive(Debug, Clone, Serialize)]
+pub struct ReplayChallenge {
+    pub format: String,
+    pub qualification: String,
+    pub trusted: ReplayChallengeContext,
+    pub candidate_digest: String,
+    pub divergence: ReplayDifference,
+}
+impl ReplayChallenge {
+    /// Prepare complete bytes before any file creation or publication.
+    pub fn to_json(&self) -> Result<String, Error> {
+        let text = mori_core::json::export(self).map_err(|e| error("limit", e.to_string()))?;
+        if text.len() >= MAX_CHALLENGE_BYTES {
+            return Err(error(
+                "limit",
+                "Escaped challenge and final newline exceed 4096 bytes.",
+            ));
+        }
+        Ok(text)
+    }
+}
+/// Reconstruct once from separate roots and use the ordinary replay order.
+/// Equality returns no challenge. Invalid input returns no diagnostic record.
+/// Exporters must call to_json before publication to enforce the output cap.
+pub fn challenge(
+    source: &str,
+    action: &str,
+    scenario: &Scenario,
+    target: &Target,
+    authorization: ReplayAuthorization<'_>,
+    candidate: &ReplayRecord,
+) -> Result<Option<ReplayChallenge>, ReplayError> {
+    candidate.validate().map_err(ReplayError::Input)?;
+    let actual =
+        generate(source, action, scenario, target, authorization).map_err(ReplayError::Input)?;
+    let divergence = match compare_records(&actual, candidate) {
+        Ok(()) => return Ok(None),
+        Err(ReplayError::Divergence(d)) => d,
+        Err(error) => return Err(error),
+    };
+    let candidate_digest = public_hash(candidate).map_err(ReplayError::Input)?;
+    let b = actual.body;
+    let claim = b.claim;
+    let trusted = ReplayChallengeContext {
+        language: b.language,
+        semantics: b.semantics,
+        network: b.network,
+        anchor: b.anchor,
+        program_name: b.program_name,
+        program_address: b.program_address,
+        action: claim.action,
+        source_hash: claim.source_hash,
+        program_hash: claim.program_hash,
+        target_hash: claim.target_hash,
+        scenario_hash: claim.scenario_hash,
+        zkir_hash: b.zkir_hash,
+        authority_hash: b.authority_hash,
+        authorization_mode: claim.authorization_mode,
+        at: claim.authorization.map(|a| a.at),
+        input_hash: b.input_hash,
+        trace_hash: b.trace_hash,
+        record_hash: actual.record_hash,
+    };
+    Ok(Some(ReplayChallenge {
+        format: CHALLENGE_FORMAT.into(),
+        qualification: "local-reference-replay-diagnostic".into(),
+        trusted,
+        candidate_digest,
+        divergence,
+    }))
+}
diff --git a/crates/mori/src/commands/replay.rs b/crates/mori/src/commands/replay.rs
index 945feef5..670aaaa3 100644
--- a/crates/mori/src/commands/replay.rs
+++ b/crates/mori/src/commands/replay.rs
@@ -90,17 +90,34 @@ pub struct Args {
     /// Public sidecar to compare; never supplies trusted roots
     #[arg(long, value_name = "FILE")]
     pub record: PathBuf,
+    /// Export a bounded local first-divergence diagnostic to a new file
+    #[arg(long, value_name = "FILE")]
+    pub challenge_out: Option<PathBuf>,
     #[command(flatten)]
     pub authorization: AuthorizationArgs,
 }
 pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
-    let target = super::core_io::read_json(&args.target)?;
-    let authorization = args.authorization.load(&target)?;
-    let candidate = ReplayRecord::from_json(&super::core_io::read_source(&args.record)?)
-        .map_err(super::core_io::operation)?;
-    let source = super::core_io::read_source(&args.input.file)?;
-    let scenario = super::core_io::read_json(&args.input.scenario)?;
-    let result = mori_backend::replay::replay(
+    // File/schema/group errors have the same input classification as failed
+    // reconstruction. Neither case creates or publishes a challenge.
+    let inputs = (|| {
+        let target = super::core_io::read_json(&args.target)?;
+        let authorization = args.authorization.load(&target)?;
+        let candidate = ReplayRecord::from_json(&super::core_io::read_source(&args.record)?)
+            .map_err(super::core_io::operation)?;
+        let source = super::core_io::read_source(&args.input.file)?;
+        let scenario = super::core_io::read_json(&args.input.scenario)?;
+        Ok::<_, mori_diagnostics::Report>((target, authorization, candidate, source, scenario))
+    })();
+    let (target, authorization, candidate, source, scenario) = match inputs {
+        Ok(inputs) => inputs,
+        Err(error) => {
+            super::core_io::print(
+                &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"input","detail":error.to_string()}}),
+            )?;
+            return Ok(ExitCode::FAILURE);
+        }
+    };
+    let result = mori_backend::replay::challenge(
         &source,
         &args.input.action,
         &scenario,
@@ -110,18 +127,41 @@ pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
     );
     let (report, status) = match result {
         // Successful full equality includes independently reconstructed record_hash.
-        Ok(()) => (
+        Ok(None) => (
             serde_json::json!({"status":"accepted","qualification":"local-reference-replay","record_hash":candidate.record_hash}),
             ExitCode::SUCCESS,
         ),
-        Err(ReplayError::Divergence(d)) => (
-            serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
-            ExitCode::FAILURE,
-        ),
+        Ok(Some(challenge)) => {
+            let d = &challenge.divergence;
+            if let Some(path) = &args.challenge_out {
+                let published = challenge
+                    .to_json()
+                    .map_err(super::core_io::operation)
+                    .and_then(|text| {
+                        super::proof_io::publish_new(path, format!("{text}\n").as_bytes())
+                    });
+                if let Err(error) = published {
+                    super::core_io::print(
+                        &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"output","detail":error.to_string()},"divergence":d}),
+                    )?;
+                    return Ok(ExitCode::FAILURE);
+                }
+            }
+            (
+                serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
+                ExitCode::FAILURE,
+            )
+        }
         Err(ReplayError::Input(error)) => (
             serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"input","detail":error}}),
             ExitCode::FAILURE,
         ),
+        // challenge converts semantic divergence to a diagnostic. Preserve an
+        // explicit refusal if that API ever gains another divergence branch.
+        Err(ReplayError::Divergence(d)) => (
+            serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
+            ExitCode::FAILURE,
+        ),
     };
     super::core_io::print(&report)?;
     Ok(status)
diff --git a/crates/mori-backend/tests/replay_challenge.rs b/crates/mori-backend/tests/replay_challenge.rs
new file mode 100644
index 00000000..1270da68
--- /dev/null
+++ b/crates/mori-backend/tests/replay_challenge.rs
@@ -0,0 +1,225 @@
+//! Local reference challenge output. No proof, parameter or ledger APIs.
+use mori_backend::{
+    Target,
+    authorization::{ExpectedAuthority, SignedIntent},
+    replay::{self, ReplayAuthorization as Auth, ReplayError, ReplayRecord},
+};
+use mori_core::{Scenario, UInt};
+use serde_json::{Value, json};
+struct Roots {
+    source: String,
+    scenario: Scenario,
+    target: Target,
+}
+impl Roots {
+    fn new() -> Self {
+        let o: Value = serde_json::from_str(include_str!(
+            "../../mori-core/tests/fixtures/bound-replay-oracle.json"
+        ))
+        .unwrap();
+        let c = &o["contrasting"];
+        Self {
+            source: c["source"].as_str().unwrap().into(),
+            scenario: serde_json::from_value(c["scenario"].clone()).unwrap(),
+            target: serde_json::from_value(c["unsigned_target"].clone()).unwrap(),
+        }
+    }
+    fn record(&self) -> ReplayRecord {
+        replay::generate(
+            &self.source,
+            "collect",
+            &self.scenario,
+            &self.target,
+            Auth::Unsigned,
+        )
+        .unwrap()
+    }
+    fn challenge(&self, r: &ReplayRecord) -> Result<Option<replay::ReplayChallenge>, ReplayError> {
+        replay::challenge(
+            &self.source,
+            "collect",
+            &self.scenario,
+            &self.target,
+            Auth::Unsigned,
+            r,
+        )
+    }
+}
+fn edit(r: &ReplayRecord, path: &str, value: Value) -> ReplayRecord {
+    let mut v = serde_json::to_value(r).unwrap();
+    *v.pointer_mut(path).unwrap() = value;
+    ReplayRecord::from_json(&v.to_string()).unwrap()
+}
+#[test]
+fn challenge_binds_trusted_context_and_candidate_bytes_with_early_semantics() {
+    let roots = Roots::new();
+    let actual = roots.record();
+    assert!(roots.challenge(&actual).unwrap().is_none());
+    let mut bad = edit(
+        &actual,
+        "/body/execution/steps/1/effects/0/amount",
+        json!("4"),
+    );
+    bad.body.execution.steps[7].index = 99;
+    bad.record_hash = "00".repeat(32);
+    bad.body.claim.public_values[0] = "00".repeat(32);
+    let c = roots.challenge(&bad).unwrap().unwrap();
+    assert_eq!(c.divergence.step, Some(1));
+    assert_eq!(
+        c.divergence.field,
+        "/body/execution/steps/1/effects/0/amount"
+    );
+    assert_eq!(c.trusted.record_hash, actual.record_hash);
+    assert_eq!(c.trusted.input_hash, actual.body.input_hash);
+    assert_eq!(c.trusted.target_hash, actual.body.claim.target_hash);
+    assert_eq!(c.trusted.scenario_hash, actual.body.claim.scenario_hash);
+    assert_eq!(
+        c.candidate_digest,
+        mori_core::intent::hash_bytes(&mori_core::intent::canonical(&bad).unwrap())
+    );
+    assert_ne!(c.candidate_digest, bad.record_hash);
+    assert!(c.to_json().unwrap().len() + 1 <= replay::MAX_CHALLENGE_BYTES);
+    let mut late = bad.clone();
+    late.body.execution.steps[7].index = 98;
+    assert_ne!(
+        roots.challenge(&late).unwrap().unwrap().candidate_digest,
+        c.candidate_digest
+    );
+}
+#[test]
+fn missing_extra_and_last_steps_derive_ordinal_from_traversal() {
+    let roots = Roots::new();
+    let actual = roots.record();
+    for (mut bad, ordinal) in [(actual.clone(), 7), (actual.clone(), 8)] {
+        if ordinal == 7 {
+            bad.body.execution.steps.pop();
+        } else {
+            bad.body
+                .execution
+                .steps
+                .push(actual.body.execution.steps[7].clone());
+        }
+        let c = roots.challenge(&bad).unwrap().unwrap();
+        assert_eq!(c.divergence.step, Some(ordinal));
+        assert_eq!(
+            c.divergence.field,
+            format!("/body/execution/steps/{ordinal}")
+        );
+    }
+    let mut bad = actual.clone();
+    bad.body.execution.steps[7].index = u16::MAX;
+    assert_eq!(
+        roots.challenge(&bad).unwrap().unwrap().divergence.step,
+        Some(7)
+    );
+}
+#[test]
+fn malformed_typed_record_and_invalid_trusted_inputs_stay_input_failures() {
+    let mut roots = Roots::new();
+    let r = roots.record();
+    let mut bad = r.clone();
+    bad.record_hash = "not-a-digest".into();
+    assert!(matches!(roots.challenge(&bad), Err(ReplayError::Input(_))));
+    roots.source = "bad source".into();
+    assert!(matches!(roots.challenge(&r), Err(ReplayError::Input(_))));
+    let mut roots = Roots::new();
+    roots.target.network = "different-domain".into();
+    assert!(matches!(roots.challenge(&r), Err(ReplayError::Input(_))));
+    let roots = Roots::new();
+    let bad = edit(&r, "/body/anchor", json!("other-domain"));
+    let c = roots.challenge(&bad).unwrap().unwrap();
+    assert_eq!(c.divergence.field, "/body/anchor");
+    assert_eq!(c.trusted.anchor, roots.target.anchor);
+}
+#[test]
+fn encoded_byte_bound_includes_ascii_escaping_and_newline() {
+    let mut roots = Roots::new();
+    roots.target.anchor = "🦀".repeat(256);
+    let r = roots.record();
+    let bad = edit(&r, "/body/execution/steps/0/index", json!(99));
+    let c = roots.challenge(&bad).unwrap().unwrap();
+    assert!(c.to_json().is_err());
+    let roots = Roots::new();
+    let bad = edit(&roots.record(), "/body/execution/steps/0/index", json!(99));
+    let mut c = roots.challenge(&bad).unwrap().unwrap();
+    let original_len = c.to_json().unwrap().len();
+    c.trusted
+        .anchor
+        .push_str(&"x".repeat(replay::MAX_CHALLENGE_BYTES - original_len - 1));
+    assert_eq!(c.to_json().unwrap().len() + 1, replay::MAX_CHALLENGE_BYTES);
+    c.trusted.anchor.push('x');
+    assert!(c.to_json().is_err());
+    c.divergence.field = "x".repeat(replay::MAX_CHALLENGE_BYTES);
+    assert!(c.to_json().is_err());
+}
+#[test]
+fn signed_external_attachment_is_validated_and_never_exported() {
+    let source = include_str!(
+        "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori"
+    );
+    let scenario: Scenario = serde_json::from_str(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json")).unwrap();
+    let target = Target::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json")).unwrap();
+    let authority = ExpectedAuthority::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json")).unwrap();
+    let signature = SignedIntent::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json")).unwrap();
+    let auth = Auth::Signed {
+        authority: &authority,
+        at: UInt(15),
+        signature: &signature,
+    };
+    let r = replay::generate(source, "settle", &scenario, &target, auth).unwrap();
+    let mut bad = r.clone();
+    bad.body.execution.steps[0].index = 99;
+    let c = replay::challenge(source, "settle", &scenario, &target, auth, &bad)
+        .unwrap()
+        .unwrap();
+    assert_eq!(c.trusted.authority_hash, r.body.authority_hash);
+    assert_eq!(c.trusted.at, Some(UInt(15)));
+    let text = c.to_json().unwrap();
+    assert!(!text.contains(&signature.signature_hex));
+    assert!(!text.contains(&signature.signature_hex[..128]));
+    assert!(!text.contains(&signature.signature_hex[128..]));
+    for token in [
+        "signature_hex",
+        "message_hex",
+        "proof_hex",
+        "private_inputs",
+        "private_transcript",
+        "public_key_hex",
+    ] {
+        assert!(!text.contains(token), "{token}");
+    }
+    let mut changed = authority.clone();
+    changed.predecessor = "other-predecessor".into();
+    assert!(matches!(
+        replay::challenge(
+            source,
+            "settle",
+            &scenario,
+            &target,
+            Auth::Signed {
+                authority: &changed,
+                at: UInt(15),
+                signature: &signature
+            },
+            &bad
+        ),
+        Err(ReplayError::Input(_))
+    ));
+    let mut invalid = signature.clone();
+    invalid.signature_hex = "00".repeat(96);
+    assert!(matches!(
+        replay::challenge(
+            source,
+            "settle",
+            &scenario,
+            &target,
+            Auth::Signed {
+                authority: &authority,
+                at: UInt(15),
+                signature: &invalid
+            },
+            &bad
+        ),
+        Err(ReplayError::Input(_))
+    ));
+}
diff --git a/crates/mori/tests/replay_challenge.rs b/crates/mori/tests/replay_challenge.rs
new file mode 100644
index 00000000..94d9b363
--- /dev/null
+++ b/crates/mori/tests/replay_challenge.rs
@@ -0,0 +1,385 @@
+//! Actual replay CLI diagnostics only; no native proof or ledger APIs.
+use mori_backend::{
+    Target,
+    replay::{self, ReplayAuthorization},
+};
+use mori_core::Scenario;
+use serde_json::{Value, json};
+use std::{
+    path::PathBuf,
+    process::{Command, Output},
+    sync::atomic::{AtomicUsize, Ordering},
+};
+static NEXT: AtomicUsize = AtomicUsize::new(0);
+struct Files(PathBuf);
+impl Drop for Files {
+    fn drop(&mut self) {
+        let _ = std::fs::remove_dir_all(&self.0);
+    }
+}
+impl Files {
+    fn new() -> Self {
+        let path = std::env::temp_dir().join(format!(
+            "mori-challenge-{}-{}",
+            std::process::id(),
+            NEXT.fetch_add(1, Ordering::Relaxed)
+        ));
+        std::fs::create_dir(&path).unwrap();
+        let f = Self(path);
+        let o: Value = serde_json::from_str(include_str!(
+            "../../mori-core/tests/fixtures/bound-replay-oracle.json"
+        ))
+        .unwrap();
+        let c = &o["contrasting"];
+        f.write("source.mori", c["source"].as_str().unwrap());
+        f.write("scenario.json", c["scenario"].to_string());
+        f.write("target.json", c["unsigned_target"].to_string());
+        f.write("record.json", f.record().to_string());
+        f
+    }
+    fn path(&self, name: &str) -> PathBuf {
+        self.0.join(name)
+    }
+    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
+        std::fs::write(self.path(name), bytes).unwrap();
+    }
+    fn read(&self, name: &str) -> Value {
+        serde_json::from_slice(&std::fs::read(self.path(name)).unwrap()).unwrap()
+    }
+    fn record(&self) -> Value {
+        let source = std::fs::read_to_string(self.path("source.mori")).unwrap();
+        let scenario: Scenario = serde_json::from_value(self.read("scenario.json")).unwrap();
+        let target: Target = serde_json::from_value(self.read("target.json")).unwrap();
+        serde_json::to_value(
+            replay::generate(
+                &source,
+                "collect",
+                &scenario,
+                &target,
+                ReplayAuthorization::Unsigned,
+            )
+            .unwrap(),
+        )
+        .unwrap()
+    }
+    fn command(&self) -> Command {
+        self.command_action("collect")
+    }
+    fn command_action(&self, action: &str) -> Command {
+        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
+        c.args(["--color", "never", "replay"])
+            .arg(self.path("source.mori"))
+            .args(["--action", action, "--scenario"])
+            .arg(self.path("scenario.json"))
+            .arg("--target")
+            .arg(self.path("target.json"))
+            .arg("--record")
+            .arg(self.path("record.json"))
+            .arg("--challenge-out")
+            .arg(self.path("challenge.json"));
+        c
+    }
+    fn run(&self) -> Output {
+        self.command().output().unwrap()
+    }
+    fn tamper(&self, path: &str, value: Value) {
+        let mut r = self.read("record.json");
+        *r.pointer_mut(path).unwrap() = value;
+        r["record_hash"] = "00".repeat(32).into();
+        self.write("record.json", r.to_string());
+    }
+}
+fn report(o: &Output) -> Value {
+    serde_json::from_slice(&o.stdout)
+        .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&o.stderr)))
+}
+#[test]
+fn actual_cli_exports_early_semantic_difference_and_trusted_roots() {
+    let f = Files::new();
+    let actual = f.record();
+    let mut bad = actual.clone();
+    bad["body"]["execution"]["steps"][0]["index"] = 99.into();
+    bad["body"]["execution"]["steps"][7]["index"] = 98.into();
+    bad["body"]["claim"]["public_values"][0] = "00".repeat(32).into();
+    bad["record_hash"] = "00".repeat(32).into();
+    f.write("record.json", bad.to_string());
+    let bytes_before = std::fs::read(f.path("record.json")).unwrap();
+    let out = f.run();
+    assert!(!out.status.success());
+    assert_eq!(
+        report(&out)["error"],
+        json!({"kind":"divergence", "step":0,"field":"/body/execution/steps/0/index"})
+    );
+    let challenge = f.read("challenge.json");
+    assert_eq!(challenge["format"], "moriarty-reference-challenge/1");
+    assert_eq!(
+        challenge["qualification"],
+        "local-reference-replay-diagnostic"
+    );
+    assert_eq!(
+        challenge["divergence"],
+        json!({"step":0,"field":"/body/execution/steps/0/index"})
+    );
+    for field in [
+        "record_hash",
+        "input_hash",
+        "trace_hash",
+        "zkir_hash",
+        "network",
+        "anchor",
+    ] {
+        let expected = if field == "record_hash" {
+            &actual[field]
+        } else {
+            &actual["body"][field]
+        };
+        assert_eq!(&challenge["trusted"][field], expected, "{field}");
+    }
+    for field in [
+        "source_hash",
+        "program_hash",
+        "scenario_hash",
+        "target_hash",
+        "action",
+    ] {
+        assert_eq!(challenge["trusted"][field], actual["body"]["claim"][field]);
+    }
+    assert_eq!(std::fs::read(f.path("record.json")).unwrap(), bytes_before);
+}
+#[test]
+fn first_effect_last_and_absent_steps_are_exported_in_order() {
+    for (path, value, step) in [
+        ("/body/execution/steps/1/effects/0/amount", json!("4"), 1),
+        ("/body/execution/steps/7/index", json!(99), 7),
+    ] {
+        let f = Files::new();
+        f.tamper(path, value);
+        let out = f.run();
+        assert_eq!(report(&out)["error"]["field"], path);
+        assert_eq!(f.read("challenge.json")["divergence"]["step"], step);
+    }
+    let f = Files::new();
+    let mut r = f.read("record.json");
+    r["body"]["execution"]["steps"]
+        .as_array_mut()
+        .unwrap()
+        .pop();
+    f.write("record.json", r.to_string());
+    assert_eq!(
+        report(&f.run())["error"]["field"],
+        "/body/execution/steps/7"
+    );
+    assert_eq!(f.read("challenge.json")["divergence"]["step"], 7);
+}
+#[test]
+fn equality_and_input_refusals_never_publish_challenges() {
+    let f = Files::new();
+    let out = f.run();
+    assert!(
+        out.status.success(),
+        "{}",
+        String::from_utf8_lossy(&out.stderr)
+    );
+    assert_eq!(report(&out)["status"], "accepted");
+    assert!(!f.path("challenge.json").exists());
+    for bytes in [
+        "{}".into(),
+        " ".repeat(1_048_577),
+        "{\"body\":1,\"body\":2}".into(),
+    ] {
+        f.write("record.json", bytes);
+        let out = f.run();
+        assert!(!out.status.success());
+        assert_eq!(report(&out)["error"]["kind"], "input");
+        assert!(!f.path("challenge.json").exists());
+    }
+    f.write("record.json", f.record().to_string());
+    f.write("source.mori", "bad source");
+    assert_eq!(report(&f.run())["error"]["kind"], "input");
+    assert!(!f.path("challenge.json").exists());
+}
+#[test]
+fn cross_source_and_domain_do_not_supply_their_own_trusted_context() {
+    let f = Files::new();
+    let actual = f.record();
+    f.tamper("/body/anchor", json!("attacker-anchor"));
+    let out = f.run();
+    assert_eq!(report(&out)["error"]["field"], "/body/anchor");
+    assert_eq!(
+        f.read("challenge.json")["trusted"]["anchor"],
+        actual["body"]["anchor"]
+    );
+    std::fs::remove_file(f.path("challenge.json")).unwrap();
+    f.write("record.json", actual.to_string());
+    let source = std::fs::read_to_string(f.path("source.mori")).unwrap();
+    f.write("source.mori", format!("{source}\n"));
+    assert_eq!(report(&f.run())["error"]["kind"], "divergence");
+    assert_eq!(
+        f.read("challenge.json")["trusted"]["source_hash"],
+        f.record()["body"]["claim"]["source_hash"]
+    );
+}
+#[test]
+fn publication_refuses_existing_files_directories_symlinks_and_races() {
+    let f = Files::new();
+    f.tamper("/body/execution/steps/0/index", json!(99));
+    f.write("challenge.json", "existing");
+    assert!(!f.run().status.success());
+    assert_eq!(
+        std::fs::read(f.path("challenge.json")).unwrap(),
+        b"existing"
+    );
+    std::fs::remove_file(f.path("challenge.json")).unwrap();
+    std::fs::create_dir(f.path("challenge.json")).unwrap();
+    assert!(!f.run().status.success());
+    std::fs::remove_dir(f.path("challenge.json")).unwrap();
+    #[cfg(unix)]
+    {
+        std::os::unix::fs::symlink(f.path("missing"), f.path("challenge.json")).unwrap();
+        assert!(!f.run().status.success());
+        assert!(!f.path("missing").exists());
+        std::fs::remove_file(f.path("challenge.json")).unwrap();
+    }
+    let a = f
+        .command()
+        .stdout(std::process::Stdio::piped())
+        .stderr(std::process::Stdio::piped())
+        .spawn()
+        .unwrap();
+    let b = f
+        .command()
+        .stdout(std::process::Stdio::piped())
+        .stderr(std::process::Stdio::piped())
+        .spawn()
+        .unwrap();
+    let a = a.wait_with_output().unwrap();
+    let b = b.wait_with_output().unwrap();
+    assert!(!a.status.success());
+    assert!(!b.status.success());
+    let kinds = [
+        report(&a)["error"]["kind"].clone(),
+        report(&b)["error"]["kind"].clone(),
+    ];
+    assert_eq!(kinds.iter().filter(|v| **v == "divergence").count(), 1);
+    assert_eq!(kinds.iter().filter(|v| **v == "output").count(), 1);
+    assert_eq!(f.read("challenge.json")["divergence"]["step"], 0);
+    assert!(!std::fs::read_dir(&f.0).unwrap().any(|e| {
+        e.unwrap()
+            .file_name()
+            .to_string_lossy()
+            .starts_with(".mori-proof")
+    }));
+}
+#[test]
+fn oversized_escaped_sidecar_is_refused_before_publication() {
+    let f = Files::new();
+    let mut t = f.read("target.json");
+    t["anchor"] = "🦀".repeat(256).into();
+    f.write("target.json", t.to_string());
+    f.write("record.json", f.record().to_string());
+    f.tamper("/body/execution/steps/0/index", json!(99));
+    let out = f.run();
+    assert!(!out.status.success());
+    assert!(!f.path("challenge.json").exists());
+    assert_eq!(report(&out)["error"]["kind"], "output");
+}
+
+#[test]
+fn signed_cli_validates_independent_authority_and_never_exports_attachment() {
+    let f = Files::new();
+    for (name, bytes) in [
+        (
+            "source.mori",
+            include_str!(
+                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori"
+            ),
+        ),
+        (
+            "scenario.json",
+            include_str!(
+                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json"
+            ),
+        ),
+        (
+            "target.json",
+            include_str!(
+                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json"
+            ),
+        ),
+        (
+            "authority.json",
+            include_str!(
+                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json"
+            ),
+        ),
+        (
+            "signed.json",
+            include_str!(
+                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json"
+            ),
+        ),
+    ] {
+        f.write(name, bytes);
+    }
+    let source = std::fs::read_to_string(f.path("source.mori")).unwrap();
+    let scenario = serde_json::from_value(f.read("scenario.json")).unwrap();
+    let target = serde_json::from_value(f.read("target.json")).unwrap();
+    let authority = mori_backend::authorization::ExpectedAuthority::from_json(
+        &f.read("authority.json").to_string(),
+    )
+    .unwrap();
+    let signature =
+        mori_backend::authorization::SignedIntent::from_json(&f.read("signed.json").to_string())
+            .unwrap();
+    let actual = replay::generate(
+        &source,
+        "settle",
+        &scenario,
+        &target,
+        ReplayAuthorization::Signed {
+            authority: &authority,
+            at: mori_core::UInt(15),
+            signature: &signature,
+        },
+    )
+    .unwrap();
+    f.write("record.json", serde_json::to_string(&actual).unwrap());
+    f.tamper("/body/execution/steps/1/effects/0/amount", json!("69"));
+    let signed_command = || {
+        let mut c = f.command_action("settle");
+        c.args(["--profile", "moriarty-jubjub-owner-intent/1", "--authority"])
+            .arg(f.path("authority.json"))
+            .args(["--at", "15", "--signature"])
+            .arg(f.path("signed.json"));
+        c
+    };
+    let out = signed_command().output().unwrap();
+    assert_eq!(report(&out)["error"]["kind"], "divergence");
+    let bytes = std::fs::read_to_string(f.path("challenge.json")).unwrap();
+    assert!(!bytes.contains(&signature.signature_hex));
+    assert!(!bytes.contains(&signature.signature_hex[..128]));
+    assert!(!bytes.contains(&signature.signature_hex[128..]));
+    for label in [
+        "signature_hex",
+        "message_hex",
+        "proof_hex",
+        "private_transcript",
+        "private_inputs",
+        "public_key_hex",
+    ] {
+        assert!(!bytes.contains(label), "{label}");
+    }
+    assert_eq!(
+        f.read("challenge.json")["trusted"]["authority_hash"],
+        serde_json::to_value(&actual.body.authority_hash).unwrap()
+    );
+    std::fs::remove_file(f.path("challenge.json")).unwrap();
+    let mut a = f.read("authority.json");
+    a["predecessor"] = "other-predecessor".into();
+    f.write("authority.json", a.to_string());
+    assert_eq!(
+        report(&signed_command().output().unwrap())["error"]["kind"],
+        "input"
+    );
+    assert!(!f.path("challenge.json").exists());
+}
```
