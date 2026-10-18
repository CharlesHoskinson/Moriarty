# Main integration verification

The user requested merging the UTXO project work into the repository. This record identifies both parents, resolution choices, exact blob preservation and the local ordinary QA result. It records integration, not product acceptance.

Local `scripts/doctorwatson.sh` passed its ordinary corpus/source check and 230 regression tests. No `--host` or `--design` run, SDK proof, signing, deployment or ledger action was selected. The command, full output and its digest are retained.

The merge retained 51 shared main files exactly, including the newer language implementations, roadmap, OpenSpec and plugin files. Rust workspace, Sherlock QA and existing hosted workflows match the main parent. All 8,601 files under the primary UTXO paths match their UTXO-parent blob identities. Both parent histories are preserved.

The four integration-edited shared files passed `git diff --check`: AGENTS, README, the source CSV and raw manifest. The historical merge contains 55,935 whitespace warnings across 203 frozen files; every affected file retains its exact UTXO-parent blob. These source/result archives were not reformatted. The full non-UTF-8 warning log remains local; its digest and diagnostic categories are recorded in VERIFICATION.json.

The corrected hash-checker test draft and D7 baseline observation retain pending-review status. No security result, native authority or full Cardano Preprod compatibility claim changes through this merge.
