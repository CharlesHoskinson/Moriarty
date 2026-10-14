# Moriarty init implementation report

`mori init NEW_DIR` creates a usable Core transfer starter. `--template repay` creates a usable repayment starter.
Both projects contain source.mori, scenario.json, mori-tests.json and README.md.

The transfer starter moves 25 units from alice100 to bob0. Its exact post-state contains alice75 and bob25.
The repayment starter transfers 30 units and discharges debt100 to debt70. Its ordered effects are Transfer then Liability.
The manifests contain independently specified literal expectations. Initialization never simulates a program to generate expected values.

The generated transfer action is `pay`. Existing Core semantics reserve the identifier `transfer`.
The original independent diagnosis remains unchanged. The first actual GREEN attempt recorded this source refusal before filesystem creation.

Initialization validates source, decodes Scenario and expected Snapshot/Effect values, and serializes JSON before directory creation.
It atomically reserves the destination leaf with create_dir. It creates fixed files with create_new and never removes failed project contents.
Existing directories, regular files, live symlinks and dangling symlinks refuse. Missing parents refuse without creating ancestors.
An I/O failure can leave a partial project. Stable directory paths and ancestors are required.
The implementation makes no adversarial directory replacement guarantee. The concurrent test covers competing destination reservations.

The RED suite recorded the original production panic and exit101. All seven new tests failed against the unchanged handler.
The first regression build lacked two public tracked fixtures in the sparse checkout. Exact baseline fixture materialization corrected that build environment.
The next run passed repayment and filesystem tests but exposed the reserved transfer action name. Renaming the generated action corrected that failure.
The earlier full run passed init7, check_compile_io14 and logical_meter4.
The final prose revision passed init7. Both final generated projects passed all README commands.
Rustfmt and git diff --check passed.

Financial tests compare complete post-state and ordered effects. Independent mutations challenge the actual manifest consumer separately for post-state and effects.
Repayment tests check full discharge with the present-zero liability tombstone. Overdischarge returns the exact liability refusal without input mutation or partial execution output.
Transfer tests check zero amount and insufficient funds. Filesystem tests check preservation, relative Unicode paths and concurrent reservations.

The source candidate changes only crates/mori/src/commands/init.rs, crates/mori/tests/init.rs and docs/NATIVE-CORE.md.
The saved index contains all three files. The live index has no staged changes.
The saved patch includes the new test file. The live working patch excludes that untracked file.
No commit or push occurred. Fresh independent exact-candidate audits remain the root agent's responsibility.

Ordinary Rust compilation and local CLI reference consumers supplied these observations. No proof, native constraint, network or ledger execution occurred.
These results do not establish ledger settlement, proof correspondence, broader language completion or roadmap acceptance.

Candidate tree: 12fb7bb6dc14e8f0aae75059325f2799d42b5a45.
Candidate manifest: /home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/init/candidate.json.
Full saved-index patch: /home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/init/full-candidate.patch.
Final test log: /home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/init/final3-init.log.
Durable projects and CLI evidence: /home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/init/generated-final-projects.json.
