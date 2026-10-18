# External install documentation correction

Exact source: `64c6dedff1d99b33c23832330d7911a5fad9ed59`.
Status: unapplied two-document correction; root alone integrates. Only this worker directory was written. Earlier packets, shared checkouts and Git are unchanged.

`EXTERNAL-INSTALL.patch` changes only `packages/moriarty-beta/GETTING-STARTED.md` and `docs/NATIVE-CORE.md`. Complete bodies are under `before/` and `after/`. It fixes the local beta launcher's missing connection to generated README commands, replaces editable install/start paths with shell-derived temporary paths and the actual archive filename, and corrects Rust documentation that still asks users to replace bare `mori` even though init already writes its quoted absolute executable path. It does not modify CLI help implementation or the existing help paragraph.

The public consumer trace is root README → beta package guide → `package.json` bin/prepack and `build.mjs` → `src/cli.ts` init/check/test/simulate → actual named starter declarations and beta frontend. The beta artifact is `dist/cli.js`, profile `moriarty-beta/1`, transfer `invoice.mori`/`pay`, repayment `repayment.mori`/`repay_loan`, tests `mori.tests.json`. The archive name is derived from the current package name/version; no registry publication is claimed.

The separate Rust trace is package `mori`/`src/main.rs` → command dispatch → initializer/current_exe and check_source/parse/simulate/test consumers. Proposed local commands select `--no-default-features --bin mori`; generated files use `language "moriarty/1";`, `source.mori`, `scenario.json`, `mori-tests.json`, with actions `pay` and `repay`. The default-native examples remain separate. All proposed source forms come from actual generator literals and are present in the corresponding parser arms; this is literal consistency, not executed parser acceptance. Binary artifact existence still depends on the prospective install/build steps, which were not run.

`MANIFEST.json` pins 39 selected public files by exact Git blob and SHA-256. Thirty-seven match the previous 823ce packet; changes in `cli.rs` and the module register add capabilities without changing the selected starter consumer. The changed files were inspected. No whole-tree equivalence or inherited acceptance is asserted. Previous source exposure is explicit: this author prepared the earlier candidates, and EXPECTATIONS.json is a source-derived literal contract, not an independent financial oracle or fresh review.

The callable predicate verifies those source pins, complete before/after bytes, exact patch regeneration, bin/archive/profile/action/manifest literals, native-off selection and absence of new placeholder commands:

```sh
python3 /home/charl/moriarty-c2-opus55-restored-20261017/concurrency32-wave-20261018/dwu_ears_general_4/check.py --repo /home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018
```

Observed result: source-literal-checks-pass, 39 source pins, two changed documents. Patch SHA-256: `b4bbf7c949dae601fe9d369f043c7228e2b0318909b220cd10d33c7dee45004c`. EXPECTATIONS.json SHA-256: `f7b67d01ab11ea536b5b799d80bdbb8df0c2dd9eb8059a015066fcae3b22ff3e`.

Develop, exact AGENTS and the product contract were restored. Root startup status is inherited as operational only; the known metadata-writing status path was not rerun in this no-shared-write lane. No installer, npm/Cargo/build, parser or product runtime, provider/network, native/SDK/proof/key/wallet/deployment/ledger operation ran. No security judgment is supplied. Beta remains PreparedUnqualified; Rust remains local reference execution. A clean independent external installation, new-program real proof and developer-owned Midnight submission remain unestablished. The complete retained requirement/target/finance/composition exits are unchanged.
