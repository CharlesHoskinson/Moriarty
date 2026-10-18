// Test-only complete width fixture. Fixed public QA authority, no wallet data.

use mori_core::Scenario;
use serde_json::json;
use sha2::{Digest, Sha256};
pub fn fixture(
    assets: usize,
    duties: usize,
    action: &str,
    target: &Target,
    authority: &ExpectedAuthority,
) -> (String, Scenario, Target, ExpectedAuthority) {
    let digest = |bytes: &[u8]| mori_core::intent::hex(&Sha256::digest(bytes));
    let declarations: String = (0..assets).map(|i| format!("  asset A{i:02} = {{ network: \"local\", issuer: \"signed-native-qa\", reference: \"r{i:02}\", scale: 0 }};\n")).collect();
    let originate: String = (0..duties)
        .map(|i| {
            format!(
                "    liability(\"d{i:02}\", A{:02}, \"alice\", \"bob\", 2);\n",
                i % assets
            )
        })
        .collect();
    let finances: String = (0..assets).map(|i| format!("    transfer(A{i:02}, \"alice\", \"bob\", 1);\n    fee(A{i:02}, \"alice\", \"treasury\", 1);\n")).collect();
    let progress: String = (0..duties)
        .map(|i| format!("    discharge(\"d{i:02}\", 1);\n"))
        .collect();
    let source = format!(
        "language \"moriarty/1\";\nagreement SignedShape {{\n{declarations}  state paid: UInt = 0;\n  action originate() {{\n{originate}  }}\n  action settle() {{\n{finances}{progress}    next paid = paid + 1;\n    emit \"paid\"(paid);\n  }}\n  action observe() {{ emit \"paid\"(paid); }}\n}}\n"
    );
    let asset = |i| json!({"network":"local","issuer":"signed-native-qa","reference":format!("r{i:02}"),"scale":0});
    let scenario = json!({"state":{"paid":{"type":"UInt","value":"0"}},"inputs":{},
        "balances":(0..assets).flat_map(|i| [json!({"account":"alice","asset":asset(i),"amount":"10"}), json!({"account":"bob","asset":asset(i),"amount":"0"}), json!({"account":"treasury","asset":asset(i),"amount":"0"})]).collect::<Vec<_>>(),
        "liabilities":(0..duties).map(|i| json!({"name":format!("d{i:02}"),"asset":asset(i % assets),"debtor":"alice","creditor":"bob","amount":"2"})).collect::<Vec<_>>()});
    let mut t = target.clone();
    t.signed_policy.as_mut().unwrap().program_address = "local/SignedShape".into();
    let mut a = serde_json::to_value(authority).unwrap();
    a["program_address"] = "local/SignedShape".into();
    a["target_hash"] = digest(&mori_core::intent::canonical(&t).unwrap()).into();
    a["limits"] = (0..assets).map(|i| json!({"asset":asset(i),"beneficiary":"bob","gross_cap":"2","fee_cap":"1","net_floor":"0","liability_cap":if action == "observe" { (2 * duties / assets).to_string() } else { (duties / assets).to_string() }})).collect();
    (
        source,
        serde_json::from_value(scenario).unwrap(),
        t,
        ExpectedAuthority::from_json(&a.to_string()).unwrap(),
    )
}
