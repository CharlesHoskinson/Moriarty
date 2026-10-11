use std::process::Command;

#[test]
fn signed_verification_and_simulation_escape_the_complete_display() {
    // Genuine fixture signed with the public deterministic QA key [3; 32].
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/display-fixture.json")).unwrap();
    let directory =
        std::env::temp_dir().join(format!("mori-verified-review-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(directory.clone());
    std::fs::write(
        directory.join("probe.mori"),
        include_str!("fixtures/display-probe.mori"),
    )
    .unwrap();
    std::fs::write(
        directory.join("scenario.json"),
        include_str!("fixtures/display-scenario.json"),
    )
    .unwrap();
    std::fs::write(
        directory.join("authority.json"),
        fixture["authority"].to_string(),
    )
    .unwrap();
    std::fs::write(directory.join("signed.json"), fixture["signed"].to_string()).unwrap();
    for command in ["verify-intent", "simulate"] {
        let mut invoke = Command::new(env!("CARGO_BIN_EXE_mori"));
        invoke
            .arg(command)
            .arg(directory.join("probe.mori"))
            .args(["--action", "run", "--scenario"])
            .arg(directory.join("scenario.json"));
        if command == "verify-intent" {
            invoke
                .arg("--authority")
                .arg(directory.join("authority.json"))
                .arg("--signature")
                .arg(directory.join("signed.json"))
                .args(["--at", "15", "--review"]);
        }
        let output = invoke.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.is_ascii(),
            "{command}: full display must escape controls"
        );
        assert!(stdout.chars().all(|c| !c.is_control() || c == '\n'));
        let json_start = stdout.find('{').unwrap();
        let result: serde_json::Value = serde_json::from_str(&stdout[json_start..]).unwrap();
        assert_eq!(result["status"], "accepted");
        if command == "verify-intent" {
            assert_eq!(result["signature_valid"], true);
        }
        assert_eq!(
            result["execution"]["effects"][0]["name"],
            "event\u{9b}2J\u{202e}"
        );
    }
    let mut malformed = fixture["authority"].clone();
    malformed["\u{1b}[2JFAKE SUCCESS\u{9b}2J\u{202e}\n\r"] = true.into();
    std::fs::write(directory.join("malformed.json"), malformed.to_string()).unwrap();
    for color in ["never", "always"] {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .args(["--color", color, "intent"])
            .arg(directory.join("probe.mori"))
            .args(["--action", "run", "--scenario"])
            .arg(directory.join("scenario.json"))
            .arg("--authority")
            .arg(directory.join("malformed.json"))
            .args(["--scheme", "schnorr_bip340", "--public-key"])
            .arg(fixture["authority"]["public_key_hex"].as_str().unwrap())
            .args(["--framing", "raw", "--review"])
            .output()
            .unwrap();
        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(!stderr.contains(['\u{9b}', '\u{202e}', '\r']));
        assert!(!stderr.contains("\u{1b}[2J"));
        assert!(stderr.contains("unknown field"));
        assert!(output.stdout.is_empty());
    }
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "always", "simulate"])
        .arg(directory.join("missing\u{1b}[2J\u{9b}2J\u{202e}.mori"))
        .args(["--action", "run", "--scenario"])
        .arg(directory.join("scenario.json"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains(['\u{9b}', '\u{202e}']));
    assert!(!stderr.contains("\u{1b}[2J"));
}
