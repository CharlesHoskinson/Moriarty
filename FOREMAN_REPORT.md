# Toolchain checkout repair

Ordinary `rustc` and `cargo` now select Rust 1.99.0 in this checkout.
The repository already tracked the exact pin at the base commit.
The non-cone sparse patterns omitted `/rust-toolchain.toml`.
The absent file caused rustup to select the host default, version 1.98.1.

The repair command was `git sparse-checkout add /rust-toolchain.toml`.
The command passed and materialized the tracked pin.
The pin specifies version 1.99.0, the minimal profile, rustfmt and clippy.
All required components were already installed.
The pin and build documentation match their original HEAD bytes.
The index and tracked working tree are clean.

Fresh `rustc --version` and `cargo --version` both report 1.99.0.
`rustup show active-toolchain` identifies the checkout pin.
`cargo metadata --offline --locked --no-deps --format-version 1` passed.
`git diff --check` passed.
Cargo commands used `/tmp/moriarty-language-wave1-build.lock`.
The full command outputs are in `checks.json` beside this report.

An initial draft assumed that the source pin was missing.
The draft changed TOML key order and added build documentation.
Inspection established that the base already contained the correct pin.
The task restored both files and retained the earlier checks separately.
An attempted `sparse-checkout add --no-cone` reported an unknown option.
The corrected command omitted that option and passed.

The coordinated worker agent also repaired G1-G3 sparse checkouts.
It reported ordinary Rust and Cargo versions 1.99.0 for each checkout.
Its receipt is `../headless-ready-rust-pin-preflight.json`.
This report does not independently attest those worker checks.

No source patch remains, so this task needs no new source publication or audits.
No installation, build, test, native query, proof, key read, commit or push occurred.
Metadata validates workspace manifests, not compiled code or complete dependency caches.
This repair establishes no Fenrir tuple, M2 qualification or language milestone closure.

Base and candidate commit: `7cb60de83208c741a5c3b4da081ce3d71d4c2595`.
Candidate tree: `1bca8244f7b9e5541afb3e863d2bb5ea6254ae8c`.
Tree SHA-256: `983e981d4b0dd9f21d74cabe50d73455884c0a081697103f459e237f39fdf562`.
Empty patch SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
Tracked pin SHA-256: `72f7dd8c9fdf9c1fcd8eb8bf9326d4bc66833e4ebf4bc2932636e2b85efcb7af`.
See `freeze.json` for the complete scoped source digest map.
