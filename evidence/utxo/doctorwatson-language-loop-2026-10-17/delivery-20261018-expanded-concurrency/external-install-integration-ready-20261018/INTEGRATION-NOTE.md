# Integration-ready local install and starter documentation

Base: `/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018` at `823ce081c00786a0c55b92af81009f9e7c5e6a16`.
Status: **unapplied ordinary documentation patch**. Root is sole integrator. The previous `external-install-current-source-20261018` packet remains unchanged.

`EXTERNAL-INSTALL.patch` contains the complete applicable changes to two documents. `before/` preserves the original documents and selected real consumers; `after/` contains both complete proposed documents. No implementation source is changed.

The beta guide now builds and binds its defined `dist/cli.js` entrypoint explicitly, then uses fresh temporary parents for the actual transfer and repayment starters. The outside-checkout sequence packs into a shell-created temporary project and installs `moriarty-lang-beta-0.1.0-beta.1.tgz`, the archive name matching the current package manifest. An absolute installed-package launcher is defined before entering the generated project. No editable absolute paths appear as working commands. The real beta profile is `moriarty-beta/1`: transfer uses `invoice.mori`/`pay`, repayment uses `repayment.mori`/`repay_loan`, and the test manifest is `mori.tests.json`.

The Rust guide supplies concrete init/check/simulate/test instructions from the repository root, selecting the existing source package/binary `mori` with `--no-default-features --bin mori`. Its parent directory is derived with `mktemp`, not an editable NEW_DIR placeholder. Current init emits `language "moriarty/1";`, `source.mori`, `scenario.json`, `mori-tests.json`, and POSIX-quoted absolute current-executable commands in README. Transfer is `pay`; repayment is `repay`. Default native examples remain separate; `source6-expansion` is not silently selected.

Real errors corrected:

| Issue | Correction |
|---|---|
| The local beta guide suggests npx in the install project, while generated README commands use bare mori | Bind the exact beta CLI before changing directory; no global install or project metadata prerequisite |
| Rust documentation asks users to replace generated bare mori although init already uses quoted current_exe | Describe and retain the actual generated absolute launcher |
| Earlier candidate commands require editing absolute paths or NEW_DIR | Derive new archive/project/starter paths and use concrete actual arguments |
| Rust and beta share a command name but accept different profiles and manifests | Select their actual launcher/bin paths and preserve their syntax and filename distinctions |

The proposed beta transfer/repayment constructs match the actual frontend profile/action branches; the Rust generated language/asset/action/transfer/liability/discharge constructs match actual Core parser arms. No new source syntax or unsupported application is authored. Counter/increment in the untouched native examples also matches its actual source. These are literal source comparisons, not executed parser acceptances.

Current-source changes were inspected: Rust check now uses `check_source::dispatch`, with a changed module register. Exactly 31 of the earlier 33 selected files remain byte-identical. The changed register/check files and newly captured dispatcher/parser sources were read; no old acceptance transfers. `CURRENT-SOURCE-COMPARISON.json` states that finite scope. All 39 captured files match the current checkout and pinned HEAD blobs.

The existing entrypoint/build declarations identify the binaries that the instructions select. Their installed or built artifact existence is conditional on completing the prospective install/build steps; this task did not observe those steps. No binary version, build success, dependency availability or fresh external install is claimed. Local beta preparation remains PreparedUnqualified; Rust remains local reference execution. Neither satisfies the contract's independent new-program/proof/submission exit.

The installed develop skill and K AGENTS were restored. Root startup status is inherited as operational history only; the metadata-writing status command was not rerun in this no-shared-write lane. Historical SP01 blockers are not selected work and supply no source acceptance.

`DOCUMENTATION-CHECKS.json` records eight minimal literal checks, exact two-document patch regeneration and current source pins. No contrived regression tests were added for these reversible prose changes. Installer/npm/Cargo/build, actual parser or CLI runtime, provider/network, native/SDK, proof, deployment and ledger actions were not run. No global-tool, Git or shared-worktree mutation or security judgment occurred. Existing proof/deployment instructions elsewhere in the full guide remain outside this ordinary install/start correction and were not executed.
