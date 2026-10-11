use std::process::Command;

#[test]
fn signing_review_escapes_identities_and_preserves_exact_statement() {
    let directory = std::env::temp_dir().join(format!("mori-intent-review-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(directory.clone());
    let hostile = "issuer\u{1b}[2J\u{1b}[H\n\r\u{7f}\u{9b}\u{202e}\u{1f4b0}";
    let asset =
        serde_json::json!({"network": "local", "issuer": hostile, "reference": "usd", "scale": 0});
    let source = format!(
        "language \"moriarty/1\"; agreement Review {{ asset USD = {{ network: \"local\", issuer: {}, reference: \"usd\", scale: 0 }}; action run() {{ transfer(USD, \"alice\", \"bob\", 1); }} }}",
        serde_json::to_string(hostile).unwrap()
    );
    std::fs::write(directory.join("review.mori"), source).unwrap();
    let scenario = serde_json::json!({"state": {}, "inputs": {}, "balances": [
        {"account": "alice", "asset": asset, "amount": "2"},
        {"account": "bob", "asset": asset, "amount": "0"}
    ], "liabilities": []});
    let key = "531fe6068134503d2723133227c867ac8fa6c83c537e9a44c3c5bdbdcb1fe337";
    let authority = serde_json::json!({
        "network": "local", "anchor": "address\"\\\u{202e}", "program_address": "address\"\\\u{202e}",
        "target_hash": "local-target", "signer": "alice", "public_key_hex": key,
        "scheme": "schnorr_bip340", "nonce": "7", "predecessor": "head\"\\\u{202e}",
        "not_before": "10", "not_after": "20", "limits": [{
            "asset": asset, "beneficiary": "bob", "gross_cap": "1", "fee_cap": "0",
            "net_floor": "1", "liability_cap": "0"
        }]
    });
    std::fs::write(directory.join("scenario.json"), scenario.to_string()).unwrap();
    std::fs::write(directory.join("authority.json"), authority.to_string()).unwrap();
    let invoke = |review| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_mori"));
        command
            .arg("intent")
            .arg(directory.join("review.mori"))
            .args(["--action", "run", "--scenario"])
            .arg(directory.join("scenario.json"))
            .arg("--authority")
            .arg(directory.join("authority.json"))
            .args([
                "--scheme",
                "schnorr_bip340",
                "--public-key",
                key,
                "--framing",
                "raw",
            ]);
        if review {
            command.arg("--review");
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    let plain = invoke(false);
    let review = invoke(true);
    assert!(
        review.is_ascii(),
        "the entire human review must escape Unicode controls"
    );
    assert!(review.chars().all(|c| !c.is_control() || c == '\n'));
    let (header, statement) = review
        .split_once("Exact canonical statement and signing bytes:\n")
        .unwrap();
    assert!(header.contains("issuer\\u{1b}[2J\\u{1b}[H\\n\\r\\u{7f}\\u{9b}\\u{202e}"));
    assert!(header.chars().all(|c| !c.is_control() || c == '\n'));
    assert!(header.is_ascii());
    let plain: serde_json::Value = serde_json::from_str(&plain).unwrap();
    let statement: serde_json::Value = serde_json::from_str(statement).unwrap();
    assert_eq!(plain, statement);
    assert_eq!(statement["intent"]["assets"][0]["issuer"], hostile);
}
