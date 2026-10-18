#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn help_distinguishes_core_consumers_from_source6_proposals() {
        let command = Cli::command();
        let compile = command.find_subcommand("compile").unwrap();
        let compile_help = compile.get_long_about().unwrap().to_string();
        for header in [r#"language "moriarty/1";"#, r#"language "moriarty/2";"#] {
            assert!(compile_help.contains(header), "{compile_help}");
        }
        assert!(
            compile_help.contains("Beta/1 and Source/6 text are not compiler inputs."),
            "{compile_help}"
        );

        let simulate = command.find_subcommand("simulate").unwrap();
        let simulate_help = simulate.get_long_about().unwrap().to_string();
        for profile in ["moriarty/1", "moriarty/2"] {
            assert!(simulate_help.contains(profile), "{simulate_help}");
        }
        assert!(
            simulate_help.contains("moriarty-beta/1 and Source/6 text are not simulation inputs."),
            "{simulate_help}"
        );

        let expand = command.find_subcommand("expand").unwrap();
        let expand_help = expand.get_long_about().unwrap().to_string();
        assert!(expand_help.contains("moriarty/1"), "{expand_help}");
        let source6 = expand
            .get_arguments()
            .find(|argument| argument.get_id().as_str() == "emit_source6_transfer")
            .unwrap();
        let source6_help = source6.get_long_help().unwrap().to_string();
        for boundary in [
            "moriarty-beta/1",
            "local-stipulation-only",
            "Source/6 output is not compiled or simulated by this option.",
        ] {
            assert!(source6_help.contains(boundary), "{source6_help}");
        }
    }
}
