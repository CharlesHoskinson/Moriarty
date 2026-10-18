# Public original-clause selector

Use the existing public command, from `/tmp` or any other directory:

```bash
bash /home/charl/Moriarty/.worktrees/utxo/scripts/doctorwatson-utxo.sh --requirement EUC-001
```

At pinned commit `b3b4f9166832d3905492654edbc8f720cef8642b`, `EUC-001` is a
unique original catalogue ID. Expected exit: **0**, while its acceptance and gate
remain **open**. The expected original location is:

```text
/home/charl/Moriarty/.worktrees/utxo/openspec/changes/eutxo-compiler-correspondence/specs/eutxo-compiler/spec.md:3
EUC-001 Explicit factory and version routing
```

The original heading at line 3 is exactly
`### Requirement: EUC-001 Explicit factory and version routing`. The command
retains both original positive/refusal scenarios and the legacy profile/target
applicability. Its selector arguments invoke the ordinary metadata reader and
skip the shell's no-argument unittest-discovery block.

`DWU-01` is a separate reporting-tool requirement at
`openspec/changes/utxo-doctorwatson/requirements.md:7`. It occurs in neither the
148-row original baseline nor the evidence ID inventory, so the existing exact
selector correctly returned 1 and selected zero rows. `EUC-001` is not an alias
for `DWU-01`. The historical failed receipt is retained unchanged.

`PUBLIC-ARGV.json` supplies the exact argv and expected scope/exit.
`DIAGNOSIS.json` and `SOURCE-PINS.json` retain source lines, ID counts and hashes.
This agent did not run the subject command; exit 0 is the expectation for the
unchanged pinned inventory, not a new observation or acceptance result. No new
route, source edit, test-only entrypoint or shared mutation is needed.
