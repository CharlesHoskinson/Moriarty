# Public CLI help profile correction

The existing capability catalogue already separates Core1/Core2 native-feature
compilation from beta-to-Source6 text emission. Generic CLI summaries omit that
boundary: compile says a bounded Moriarty program, simulate says any action and
expand calls default IR native. This two-file patch names the actual Core input
profiles and describes explicit beta expansion as local proposal text.

The patch changes doc-comment/long-help text in cli.rs and expand.rs. It adds a
Clap command-schema regression inside cli.rs; it does not snapshot help output
or spawn a binary. EXPECTED-HELP-CONTRACT.json records the profile contract.
Source bodies, exact base blob pins and SHA256s are in source/, candidate/ and
MANIFEST.json. No feature semantics, security/native gate or target route changes.

Existing package README/getting-started notes identify the separate packed Node
CLI and npx mori launcher. They retain Source6/Core5 PreparedUnqualified scope.
No launcher or external-install docs are changed, avoiding the other author.

Source-only, unapplied and runtime UNRUN. The manifest supplies one bounded
proposed root unit-test command; root must inspect its composed source and
execution authority first. No Cargo, Node, model, native or provider execution
was performed in this wave. Existing roadmap and acceptance obligations remain.
