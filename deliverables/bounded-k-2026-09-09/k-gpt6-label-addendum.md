# GPT-6 label-change preexecution addendum

Verdict: **extend the prior source approval and design/resource vote to the new manifest.** No blocking finding arose from this delta. All earlier execution conditions remain, including separate exact Fable review and root final binding/workload checks.

New manifest SHA-256: `c4eedd0ef6898b69e0e35c355a1eeaeaedacd1460e569ca57acf4d6a2741070a`.
Prior manifest SHA-256: `4775dc47b54f170aa3a19e94090a72e4b11ecf9da74c9396e803a79c4756e199`, preserved as `candidate-pre-label-fix.json`.
New `moriarty.k` SHA-256: `0e695ac53e7daa04de226e4f95a20d1f46bebaee02e89db7977a9b1be314c0cd`.

I restored the development workflow and refreshed guarded status. I verified all 27 current candidate/supporting bindings. The only changed binding is `moriarty.k`. Exactly ten `klabel(name)` attributes became `symbol(name)`. Reversing that substitution reproduces the old K source SHA-256 exactly. No financial rule, fixture, codec, runner, command or resource change is included.

The official manual in the pinned K checkout (`docs/user_manual.md`, symbol and legacy klabel sections) recommends `symbol(name)` for explicit external symbol names. It identifies bare `klabel(name)` with the overload mechanism. This change therefore fits the codec's expected external labels. Actual compilation and emitted labels remain untested by this reviewer.

The supplied `kompile-O0-help.txt` is help output, not compilation evidence. Root reports exit 0 for that probe. The cgroup preflight receipt records memory.max 4294967296 and memory.swap.max 0 for its test service. This supports the available memory controls, not execution of the prospective campaign. The reviewed systemd command and resource limits are unchanged.

## Brief cache binding inspection

The inspected K checkout HEAD is `4a46d1231473b599c699160132fd6e76a5c46406`, matching the toolchain revision. `Kompile.java` names `cache.bin`. `DefinitionParsing.java` saves that cache in definition compilation paths. The ordinary krun script routes program parsing through kparse and kast. `KastFrontEnd.java` uses `KRead.prettyRead`; the program parser uses `CompiledDefinition.parseSingleTerm`. I found no call from this ordinary path to the cache.bin save paths. Rule-pattern parsing also does not call those save paths in the inspected method.

No cache.bin mutation blocker was established by this bounded source inspection. I did not run K or prove that every compiled artifact remains immutable at runtime. Keep the artifact comparison and first-failure stop unchanged. If a runtime mutation occurs, retain its evidence and obtain a reviewed correction instead of excluding artifacts speculatively or retrying.

This addendum does not claim K compatibility, traces, proof, full correspondence, stage completion or ledger acceptance. I changed no implementation or runtime data. The only authored files are this addendum and its JSON receipt.
