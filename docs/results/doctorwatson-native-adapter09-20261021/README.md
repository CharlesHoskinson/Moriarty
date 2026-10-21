# DoctorWatson provisioned-host adapter

The adapter now uses the existing provisioned host, checks actual guard/helper APIs and all seven input pins, retains outer timing/counters, preserves a fresh build artifact before cleanup, and records unknown outcomes and cleanup debt. It never replays OP0–OP6. Two independent Gemini 4 Argon HIGH source reviews ran under the standing Opus outage fallback; author approvals are not counted.

Root ran the exact source closure once through `./scripts/doctorwatson.sh --test-selected`: 175 ordinary mocked tests passed, including 46 adapter cases. Fourteen unchanged runtime/context files match the public parent exactly; the eight reviewed candidate paths are the complete source change. These mocks do not execute the SDK.

The subsequent single real selected operation failed before Cargo: `TERMINAL_REFUSAL: P9 successful F result receipt content mismatch`. Guard exit was 2. The native phase worker recorded zero CPU; envelope and supervisor each recorded 160738 microseconds. The marker was absent; the existing abort-prelaunch cleanup succeeded and all three mounts/cgroups were absent. There is no current build, executable, recursive proof, target result or EARS closure. The exact attempt remains consumed; the root-cause diagnosis is separate work, with no unchanged retry or budget reset.

Review B's C4 prose mistakenly enumerates q_obs/resource_policy in its seven-pin list. The canonical source and independent review A correctly bind guard, config, operator, request, capture, helper and GNU time. Preserve this clerical discrepancy rather than treating it as a changed code requirement. Historical `c` remains null, retained policy limits and stopped C13 remain unchanged.

These are internal factory source and scoped result records. They introduce no project-review permission requirement for public Moriarty developers. CI ran locally; no automatic hosted workflow was added.
