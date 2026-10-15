//! Ordinary source-owned metadata consumer; no evaluator or target execution.
#[cfg(test)]
mod tests {
    use mori_cardano::{AccountBinding, AccountClass, InspectionTarget, inspect_cardano};
    use mori_core::Program;
    use serde_json::{Value, json};

    fn verify(
        source: &str,
        program: &str,
        bindings: &str,
        expected_layout: &str,
        expected_outer: &str,
    ) {
        let expected_program: Program = serde_json::from_str(program).unwrap();
        let expected_bindings: Value = serde_json::from_str(bindings).unwrap();
        let accounts: Vec<AccountBinding> = expected_bindings["accounts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| AccountBinding {
                name: a["name"].as_str().unwrap().to_owned(),
                class: match a["class"].as_str().unwrap() {
                    "ScriptCustodied" => AccountClass::ScriptCustodied,
                    "KeyHeldExternal" => AccountClass::KeyHeldExternal,
                    _ => panic!("Unexpected independent binding class"),
                },
            })
            .collect();
        let inspected = inspect_cardano(source, &accounts, InspectionTarget::Unit).unwrap();
        let (source_hash, program_hash) = match expected_program.name.as_str() {
            "InstallmentInvoice" => (
                "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9",
                "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9",
            ),
            "ConditionalReserve" => (
                "395e647534160fde45325232e5f1650a8bd48ce1455a68b28af203bf8bdbe357",
                "5e3751b24330656fa4d831b2574e13b24c5f1661a573de6d85370d6be50d4621",
            ),
            _ => panic!("Unexpected independent source fixture"),
        };
        let mut actual = serde_json::to_value(&inspected).unwrap();
        let actual_source = actual.as_object_mut().unwrap().remove("source").unwrap();
        assert_eq!(
            actual_source,
            json!({
                "format":"moriarty-core-source-export/1", "language":"moriarty/1", "semantics":"moriarty-core/1",
                "source_sha256":source_hash, "program_hash":program_hash, "program":expected_program
            })
        );
        let actual_layout = actual.as_object_mut().unwrap().remove("layout").unwrap();
        assert_eq!(
            actual_layout,
            serde_json::from_str::<Value>(expected_layout).unwrap()
        );
        assert_eq!(
            actual,
            serde_json::from_str::<Value>(expected_outer).unwrap()
        );
        assert!(inspected.require_layout_hash().is_err());
    }

    #[test]
    fn invoice_composes_actual_export_with_independent_complete_metadata() {
        verify(
            include_str!("../../fixtures/installment-invoice.mori"),
            include_str!("../../fixtures/installment-invoice.expected-program.json"),
            include_str!("../../fixtures/installment-invoice.projection-bindings.json"),
            include_str!("../../fixtures/installment-invoice.expected-layout.json"),
            include_str!("../../fixtures/installment-invoice.expected-outer-fields.json"),
        );
        let accounts = vec![
            AccountBinding {
                name: "buyer".into(),
                class: AccountClass::KeyHeldExternal,
            },
            AccountBinding {
                name: "escrow".into(),
                class: AccountClass::ScriptCustodied,
            },
            AccountBinding {
                name: "fees".into(),
                class: AccountClass::KeyHeldExternal,
            },
            AccountBinding {
                name: "seller".into(),
                class: AccountClass::KeyHeldExternal,
            },
            AccountBinding {
                name: "spectator".into(),
                class: AccountClass::KeyHeldExternal,
            },
        ];
        let r = inspect_cardano(
            include_str!("../../fixtures/installment-invoice.mori"),
            &accounts,
            InspectionTarget::Unit,
        )
        .unwrap();
        let actual = serde_json::to_value(r).unwrap();
        assert_eq!(
            actual["layout"]["framed_cells"],
            json!([{"account":"escrow","asset":{"network":"oracle-unit","issuer":"invoice-issuer","reference":"cash","scale":0}}])
        );
        assert_eq!(actual["layout"]["projected_balance_cells"], 5);
    }

    #[test]
    fn reserve_composes_actual_export_with_independent_complete_metadata() {
        verify(
            include_str!("../../fixtures/conditional-reserve.mori"),
            include_str!("../../fixtures/conditional-reserve.expected-program.json"),
            include_str!("../../fixtures/conditional-reserve.projection-bindings.json"),
            include_str!("../../fixtures/conditional-reserve.expected-layout.json"),
            include_str!("../../fixtures/conditional-reserve.expected-outer-fields.json"),
        );
    }

    #[test]
    fn unresolved_real_target_is_an_explicit_error_not_an_invented_layout_hash() {
        let accounts = crate::p2::bindings(include_str!(
            "../../fixtures/installment-invoice.projection-bindings.json"
        ));
        assert_eq!(
            inspect_cardano(
                include_str!("../../fixtures/installment-invoice.mori"),
                &accounts,
                InspectionTarget::PinnedCardano
            ),
            Err(mori_cardano::InspectionError::Unresolved(
                "pinned_target_identity"
            ))
        );
    }
}

#[cfg(test)]
mod p2;

#[cfg(test)]
mod staged;
