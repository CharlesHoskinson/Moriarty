# Later ordinary checks, not invoked or newly authorized

The candidate has no resolved lockfile. Resolve/adopt dependencies and obtain the required design/dependency/build-script/resource decisions first. Then the integrator must inspect/select checks under actual authority. Do not treat these proposals as a dispatch queue.

After an approved lock exists, focused normal test selections are:

```text
cargo test -p mori --no-default-features --features mcp --locked --bin mori commands::check_source::tests::complete_cli_reports_match_frozen_public_expectations -- --exact --test-threads=1
cargo test -p mori --no-default-features --features mcp --locked --bin mori commands::mcp::server::tests::normal_sdk_session_lists_and_checks_all_public_profile_fixtures -- --exact --test-threads=1
cargo test -p mori --no-default-features --features mcp --locked --bin mori commands::mcp::server::tests::public_mcp_subcommand_selects_the_server -- --exact --test-threads=1
cargo test -p mori --no-default-features --locked --bin mori commands::mcp::tests::unavailable_feature_returns_an_explicit_command_error -- --exact --test-threads=1
```

The crate's proposed SDK client dev-dependency applies to tests, including the last selection. No claim that feature-off tests avoid third-party build scripts is made. The public binary and normal standard-SDK stdio client still need their own real process/session check and both protocol lifecycle qualifications. No process launcher or custom framing harness is included here.

Rustfmt/clippy/build checks are UNRUN. Broad workspace checks retain their existing native/SDK boundaries; do not substitute them automatically. A source grammar parse or JSON fixture match does not establish that Rust type checks, dependency resolution succeeds or any command returns the expected exit.
