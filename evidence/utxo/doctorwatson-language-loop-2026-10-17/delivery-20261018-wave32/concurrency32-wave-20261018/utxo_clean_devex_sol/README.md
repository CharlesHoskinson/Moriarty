# Public capability selection walkthrough

One documentation path changes: docs/NATIVE-CORE.md, based on exact source 64c6dedff1d99b33c23832330d7911a5fad9ed59. before/ and after/ contain its complete bodies; docs.patch is the minimal unified patch. The walkthrough connects the existing capabilities report to real fixtures for Core/1, Core/2 stages, Rust beta transfer expansion and separate Node beta preparation. It clarifies the existing introductory native-feature wording and labels the following native command examples.

The public commands explicitly select the Rust Cargo CLI or built Node beta CLI. They show finite expected envelopes/values, complete existing output/assertion links, and the distinct compilation path. No code or new capability/status claim is added. No Rust LSP/MCP claim is introduced. CLI help edits remain with dmu_pr74_cleanup; that author confirmed its cli.rs/expand.rs lane is disjoint.

SOURCE-PINS.json records 35 actual public source/fixture/test blob IDs and SHA256 hashes. EXPECTATIONS.json records selected existing literal outcomes and complete existing repayment expectations. These expectations are source-derived: this author had already read the capabilities implementation/tests and then inspected the fixture oracles and CLI implementations. They are not fresh independent financial oracles or new runtime evidence.

Offline checks: the patch applies in --check mode to the exact before copy; all 14 new links exist at the immutable base; all five new shell blocks parse with bash -n. VALIDATION.json records these checks. No product command, build, test, provider, SDK, native or security operation ran, and previous passing checks were not rerun. All shared checkouts remain untouched. Original U/M/MC/SP/G, requirement/family and target acceptance obligations remain unchanged.

Root can inspect/apply the patch through its normal integration path. Read-only patch check at the target checkout:

```sh
git -C /home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018 apply --check /home/charl/moriarty-c2-opus55-restored-20261017/concurrency32-wave-20261018/utxo_clean_devex_sol/docs.patch
```

Run the walkthrough's ordinary commands only under root's existing execution scope. Native examples already present later in the guide retain their separate requirements. This source-only docs candidate does not require another product test solely to restate existing behavior.
