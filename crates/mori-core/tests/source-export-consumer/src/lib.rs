//! Bounded public source-export consumer. No evaluator or target/SDK calls.
#[cfg(test)]
mod tests {
    use mori_core::{Program, export_source};

    const INVOICE: &str = include_str!("../../source-export-fixtures/installment-invoice.mori");
    const RESERVE: &str = include_str!("../../source-export-fixtures/conditional-reserve.mori");

    #[test]
    fn invoice_matches_every_independently_expected_program_field() {
        let expected: Program = serde_json::from_str(include_str!(
            "../../source-export-fixtures/installment-invoice.expected-program.json"
        ))
        .unwrap();
        let exported = export_source(INVOICE).unwrap();
        assert_eq!(exported.format, "moriarty-core-source-export/1");
        assert_eq!(exported.language, "moriarty/1");
        assert_eq!(exported.semantics, "moriarty-core/1");
        assert_eq!(exported.program, expected);
        assert_eq!(
            exported.source_sha256,
            "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9"
        );
        assert_eq!(
            exported.program_hash,
            "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9"
        );
    }

    #[test]
    fn reserve_matches_every_independently_expected_program_field() {
        let expected: Program = serde_json::from_str(include_str!(
            "../../source-export-fixtures/conditional-reserve.expected-program.json"
        ))
        .unwrap();
        let exported = export_source(RESERVE).unwrap();
        assert_eq!(exported.program, expected);
        assert_eq!(
            exported.source_sha256,
            "395e647534160fde45325232e5f1650a8bd48ce1455a68b28af203bf8bdbe357"
        );
        assert_eq!(
            exported.program_hash,
            "5e3751b24330656fa4d831b2574e13b24c5f1661a573de6d85370d6be50d4621"
        );
        // The public envelope preserves the existing typed JSON interchange.
        let json = serde_json::to_value(&exported).unwrap();
        assert_eq!(json["program"], serde_json::to_value(&expected).unwrap());
    }

    #[test]
    fn whitespace_changes_source_identity_without_changing_program_identity() {
        let original = export_source(INVOICE).unwrap();
        let spaced = export_source(&format!("\n{INVOICE}\n")).unwrap();
        assert_ne!(original.source_sha256, spaced.source_sha256);
        assert_eq!(original.program_hash, spaced.program_hash);
        assert_eq!(original.program, spaced.program);
    }

    #[test]
    fn source_semantics_change_the_exported_program_and_its_identity() {
        let original = export_source(INVOICE).unwrap();
        let changed = export_source(&INVOICE.replacen(
            "state marker: UInt = 7;",
            "state marker: UInt = 8;",
            1,
        ))
        .unwrap();
        assert_ne!(original.source_sha256, changed.source_sha256);
        assert_ne!(original.program_hash, changed.program_hash);
        assert_ne!(original.program.states, changed.program.states);
        assert_eq!(original.program.actions, changed.program.actions);
    }

    #[test]
    fn unsupported_source_profile_is_not_relabelled() {
        assert!(export_source(&INVOICE.replacen("moriarty/1", "moriarty/2", 1)).is_err());
    }

    #[test]
    fn source_validation_errors_are_not_exported_as_programs() {
        assert!(
            export_source("language \"moriarty/1\"; agreement Bad { state value: UInt = true; }")
                .is_err()
        );
    }
}
