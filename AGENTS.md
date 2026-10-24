# Moriarty: factory charter

Moriarty is a bounded financial language for Midnight. Programs compile to ZKIRv3
and run on Midnight; the language scope is in
[docs/MORIARTY-PRODUCT-CONTRACT.md](docs/MORIARTY-PRODUCT-CONTRACT.md). The current
goal is to finish the Sherlock recursion processor.

The repository is developed by the **DoctorWatson factory**. Its interfaces are in
[models/doctorwatson/factory/CONTRACT.md](models/doctorwatson/factory/CONTRACT.md). All
earlier routing, review, adoption-vote, campaign, lane and evidence rules are
retired; their files remain only as history and grant or block nothing.

## The loop
1. Run `scripts/sherlock-next`. Work on the first failing predicate it names.
2. Make it pass by changing code or tests. Run heavy work through
   `scripts/doctorwatson heavy`, which runs preflight and applies measured caps.
3. When something fails, read the trace, write a one-line hypothesis and retry with
   `--hypothesis`. After two failures of the same kind, shrink the reproducer.
4. Run `scripts/doctorwatson check <ID>`. Commit; the pre-commit hook runs T0.
5. Run `scripts/doctorwatson stop-rule`. If it says STOP, bring the owner the trace and
   your best hypothesis. Don't write another document.

Reviews are inputs. Nothing waits on a reviewer. The one review point is the T3
stagenet promotion: a cold review through `codex exec -s read-only` that
proceeds after one hour without an answer.

## Fixed configuration
- **Commits:** branch `main`. No AI `Co-authored-by` trailers and no Claude
  attribution in commits or PRs.
- **Inner-loop commits:** each one touches code or tests. `qa/holdout/` is
  evaluator-owned. The hook enforces both.
- **Rust:** the toolchain is pinned in `rust-toolchain.toml` (1.99.0). Rust changes pass
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`
  and `cargo test --workspace --locked`; see [RUST_GUIDELINES.md](RUST_GUIDELINES.md).
- **Secrets:** never commit credentials, wallet seeds, private witnesses or
  unredacted environments. Preserve existing wallets, keys and contract state.
- **Product boundary:** maintainer tooling never becomes a prerequisite for public
  developers to compile, prove or deploy.
- **Model CLIs:** use subscription logins only. Never set `OPENAI_API_KEY`,
  `CODEX_API_KEY` or `XAI_API_KEY`.
