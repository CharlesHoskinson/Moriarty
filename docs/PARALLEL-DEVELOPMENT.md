# Parallel Moriarty source development

The October 13 user instruction selects three `gpt-6.1-sol` authors and three `grok-4.7` authors, all at high effort.
This instruction supersedes the single active source delivery restriction for this workflow.
It preserves every U0–U7, M1–M17, MC and SP acceptance requirement.
This procedure governs maintainer work. It does not authorize public Moriarty programs.

[The task manifest](../openspec/parallel-language-workflow.json) defines the initial wave.
The manifest describes source tasks. It is not a scheduler, campaign registry or native admission record.
No six-worker launch has been demonstrated by this document.
The graph provides structural pointers. Luna does not certify product behavior.

## Initial source tasks

| Lane | Author at high effort | Executable local deliverable | Readiness limit |
|---|---|---|---|
| S1 | `gpt-6.1-sol` | Field-level enforcement and trust report through `mori inspect` | Report current constraints and missing premises without asserting native acceptance |
| S2 | `gpt-6.1-sol` | Bounded first-divergence diagnostic through the existing replay consumer | A diagnostic is not a deployed fraud proof |
| S3 | `gpt-6.1-sol` | Existing-source two-asset partial escrow and exclusive recovery examples | Document, recipient and time inputs remain supplied local premises |
| G1 | `grok-4.7` | Checked-u128 primitive correspondence and hostile-witness cases | Ordinary constraint checks do not certify native soundness or cost |
| G2 | `grok-4.7` | Evaluator-integrated sponsorship and reserved recovery accounting in `policy.rs` | Local reservation accounting does not authenticate funding or settlement |
| G3 | `grok-4.7` | Exact canonical-price batch and swap source conformance | General directed rounding and complete financial families remain open |

Each task owns exact paths in the manifest.
The integrator alone owns shared Core IR, module exports, Cargo files, CLI dispatch and shared native plumbing.
Each author must request a shared interface change from the integrator.
Existing signed nonce source remains outside this wave's write sets.
FeePool and broader source/relation contrasts form a later independent wave.

Source preparation has no complete-milestone prerequisite when existing interfaces suffice.
Acceptance still requires the artifacts in [ROADMAP.md](../ROADMAP.md).
The manifest separates source task dependencies from accepted artifact dependencies.
Its empty initial source dependency lists permit concurrent preparation. They do not declare milestones complete.

## Start an author

Freeze one integration base before creating worker worktrees.
The manifest initially pins `a91e76b27f1a6a94bc54270061325eadfeb9d1e9`.
Refresh that pin and the graph together when the integration commit changes.
Do not resume an unrelated historical branch.

The tracked task manifest is a bootstrap template, not a self-updating current-commit claim.
After review and authorized integration, create a clean external worktree for the recorded integration commit.
The graph output must remain outside that source worktree.
These commands work from the primary checkout and preserve the producer's private-path guards.
Choose unused worktree and graph output paths for each wave.

```bash
MORIARTY_REPO=/home/charl/Moriarty
MORIARTY_BASE=$(git -C "$MORIARTY_REPO" rev-parse HEAD)
MORIARTY_WAVE=$(date -u +%Y%m%dT%H%M%SZ)
MORIARTY_CLEAN="/home/charl/moriarty-parallel-clean-$MORIARTY_WAVE"
MORIARTY_RUNTIME=/home/charl/Moriarty/.moriarty-dev/runtime/parallel-foundation-20261013
MORIARTY_WAVE_RECORD="$MORIARTY_RUNTIME/wave-$MORIARTY_WAVE"
mkdir "$MORIARTY_WAVE_RECORD"
MORIARTY_GRAPH="$MORIARTY_WAVE_RECORD/graph"
MORIARTY_MANIFEST="$MORIARTY_WAVE_RECORD/parallel-language-workflow.json"
git -C "$MORIARTY_REPO" worktree add --detach "$MORIARTY_CLEAN" "$MORIARTY_BASE"
/home/charl/.local/share/uv/tools/graphifyy/bin/python \
  "$MORIARTY_CLEAN/scripts/build_context_graph.py" --repo "$MORIARTY_CLEAN" --out "$MORIARTY_GRAPH"
python3 - "$MORIARTY_CLEAN" "$MORIARTY_GRAPH" "$MORIARTY_MANIFEST" <<'PY'
import json
import subprocess
import sys
from pathlib import Path
source, graph, destination = map(Path, sys.argv[1:])
ready = json.loads((graph / 'graph-ready.json').read_text())
commit = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
assert ready['ready'] is True and ready['commit'] == commit
manifest = json.loads((source / 'openspec/parallel-language-workflow.json').read_text())
manifest['candidate_base'] = commit
destination.write_text(json.dumps(manifest, indent=2) + '\n')
print(destination)
PY
python3 "$MORIARTY_CLEAN/scripts/build_task_context.py" --repo "$MORIARTY_CLEAN" \
  --manifest "$MORIARTY_MANIFEST" --graph-dir "$MORIARTY_GRAPH" --lane S1
```

Keep these wave variables in the same shell for the subsequent examples.
Use the same new graph and rebound runtime manifest for every worker, context and saved-context check.
The assembler verifies source blob pins and graph digests before emitting context.
Do not rebind the manifest to hide changed code in an old graph.
Retain earlier graph bundles unchanged. Do not use them for the refreshed wave.

The following commands illustrate S1. Repeat them with each manifest lane ID.

```bash
MORIARTY_WORKER="/home/charl/moriarty-source-S1-$MORIARTY_WAVE"
git -C "$MORIARTY_CLEAN" worktree add -b "work/parallel-S1-$MORIARTY_WAVE" \
  "$MORIARTY_WORKER" "$MORIARTY_BASE"
cd "$MORIARTY_WORKER"
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
python3 "$MORIARTY_CLEAN/scripts/build_task_context.py" --repo "$MORIARTY_WORKER" \
  --manifest "$MORIARTY_MANIFEST" --graph-dir "$MORIARTY_GRAPH" --lane S1
```

Load the installed develop skill or its checked-in `SKILL.md` before selecting work.
Read the compact context and the complete affected source before editing.
Ask an independent reviewer to define expected results and hostile cases.
Use a fresh author session in that worktree with the manifest's exact model and high effort.
Keep the same author through repairs of that capability.
Record requested and returned model identities separately.
An observed `grok-4.7-build` identity does not establish equivalence to requested `grok-4.7`.
An unavailable provider cannot approve work or trigger a silent replacement.

Actionful external workers use the existing installed Foreman queue and lane runner.
Apply [the installed Foreman skill](/home/charl/.agents/skills/foreman/SKILL.md) to its dispatch lifecycle.
The current user model and full-candidate review instructions supersede older Foreman model defaults for this scope.
Verify READY and the installed runtime before dispatch:

```bash
/home/charl/foreman/skills/foreman/scripts/foreman-setup.sh --profile soft
node /home/charl/foreman/skills/foreman/runtime/dist/architecture-policy.js verify-install \
  --skill-root /home/charl/foreman/skills/foreman
```

Require exit zero and the actual Pass verdict.
Bind the existing Endstop contract identity, digest, candidate and action to every actionful queue request.
Use the active family release block when that contract has an activated family.
Do not create a new contract to continue terminal work.
Do not infer an active profile or armed workers from these examples.

The existing queue form for a contract without an activated family is:

```text
lane-queue.sh add GROUP --endstop-state-root ABS --endstop-contract-id ID
--endstop-contract-sha SHA256 --endstop-action ACTION
--endstop-candidate-sha SHA256 -- CMD [ARGS...]
```

`CMD` owns the complete round through the installed `lane-run.sh --round` entry point.
Supply the target's fixed verification command and an attempt-fresh report path.
Preserve containment refusals, worktree ownership, limits, terminal events and cleanup.
Process containment does not prove filesystem, network or credential isolation.
The default installed queue caps may admit fewer than the target three Sol authors at once.
Do not bypass those caps to claim six concurrent authors.

These verified CLI forms are worker payloads inside that lifecycle, not bare dispatch instructions:

```text
/home/charl/.npm-global/bin/codex exec --model gpt-6.1-sol
--config 'model_reasoning_effort="high"' --cd TASK_WORKTREE
--sandbox workspace-write --ephemeral --json
--output-last-message REPORT_FILE PROMPT

/home/charl/.local/bin/grok --cwd TASK_WORKTREE --model grok-4.7
--reasoning-effort high --no-subagents --disable-web-search
--permission-mode acceptEdits --output-format json --prompt-file PROMPT_FILE
```

Write reports in the task's worktree before exit.
Use a verified read-only host path for fresh review sessions.
The read-only study launcher is not a generic implementation dispatcher.

Authorized source edits do not call the plugin's `run` verb.
Registered campaign dispatches still use `run --action` with their current admission.
The plugin's primary reservation remains shared across linked worktrees.
The current stale loan catalog is not a six-author launcher.
Do not alter that catalog or its historical binding merely to dispatch these source tasks.

## Compact author context

`scripts/build_task_context.py` reads the manifest, fixed public graph artifacts and explicitly named safe source text.
It emits one bounded JSON packet on stdout. It executes no verification command.
The packet includes current source hashes, excerpts, coverage, closure-relevant inputs and unresolved acceptance dependencies.
Missing planned files remain specified deliverables. Their checks remain inapplicable until those files exist.

The command rejects overlapping write sets, shared-owner collisions, dependency cycles and excluded paths.
It rejects stale graph digests, inconsistent graph commits, changed code/interfaces and stale saved contexts.
It reads current Git commit metadata only. It does not traverse Git history.
It never opens private PKs, private markers, SRS, provider files, proving caches, credentials or binary artifacts.
The command refuses every `protected_excluded` inventory row before source inspection or content reads.
No public-source or formal-model exception overrides that protected class.
Archive graph nodes supply no inline content.
Explicit current-language models under `experiments/moriarty-language/formal/quint/language/` may use archive-inventory direct reads.
Their hashes and inventory-only graph coverage remain visible. Model QA does not establish source correspondence or native soundness.
The manifest lists applicable existing Quint commands. The assembler never executes them.

The frozen graph describes its recorded commit, not every dirty worktree byte.
Explicit current workflow documents can differ from that graph.
The command hashes those documents and omits their stale graph hints.
Changed code or interfaces require a refreshed graph before this command can produce a packet.
This refusal limits this context command. It does not prohibit independently authorized source inspection or editing.

Graph coverage has three distinct layers: tracked inventory, AST structure and prose anchors.
The current graph does not parse `.mori` or `.qnt` semantics.
Explicit `.mori` inputs therefore use direct source reads with an inventory-only graph label.
Neighbors use `source_file`, `source_location` and graph links inside the explicit source set.
Verify cited lines in current source. Collapsed graph edges can omit additional line locations.
Do not interpret inventory completeness, community labels or graph adjacency as semantic coverage.

Save a packet in the existing task runtime directory if needed.
Check the packet immediately before using its bytes:

```bash
MORIARTY_CONTEXT="$MORIARTY_WAVE_RECORD/S1-context.json"
python3 "$MORIARTY_CLEAN/scripts/build_task_context.py" --repo "$MORIARTY_WORKER" \
  --manifest "$MORIARTY_MANIFEST" --graph-dir "$MORIARTY_GRAPH" \
  --lane S1 --review-round 1 > "$MORIARTY_CONTEXT"
python3 "$MORIARTY_CLEAN/scripts/build_task_context.py" --repo "$MORIARTY_WORKER" \
  --manifest "$MORIARTY_MANIFEST" --graph-dir "$MORIARTY_GRAPH" \
  --lane S1 --review-round 1 --check-context "$MORIARTY_CONTEXT"
```

The check compares the complete normalized saved packet with a fresh packet.
Whitespace and object key order may differ. Changed excerpts, routing, objectives or write sets cause refusal.
The sidecar is author context. Do not register it as the existing closed `candidateManifest` format.
A compact packet never replaces a full candidate or actual-result review.

## Verification and integration

Run only the named applicable commands for the source task.
Start with one ordinary build/test process, two Cargo jobs and one Rust test thread.
Permit at most two ordinary processes only after current CPU, RAM and build footprint checks show capacity.
Cap aggregate jobs across worktrees. Separate target directories avoid locks but duplicate memory and disk use.
Do not enable ignored tests, proving parameter environments or broad native suites in ordinary checks.

Return all failures together with focused patches and current source context.
After two failures of the same class, reproduce the defect and change the approach.
Preserve the two-cycle and thirty-minute administration stop rules.
Apply those rules per capability and to the integrated work, not only to one task's status.

Freeze owned and input digest maps after relevant callable checks pass.
Record the integration base separately from the exact candidate commit.
Use the existing canonical owned-map SHA-256 for candidate identity where applicable.
Retain source and actual-result receipts separately.

Check these Git identities before integration:

```bash
git rev-parse HEAD
git status --short
MORIARTY_CANDIDATE_COMMIT=$(git rev-parse HEAD)
MORIARTY_INTEGRATION_COMMIT=$(git -C "$MORIARTY_REPO" rev-parse HEAD)
git merge-base "$MORIARTY_CANDIDATE_COMMIT" "$MORIARTY_INTEGRATION_COMMIT"
git diff --name-only "$MORIARTY_BASE" "$MORIARTY_CANDIDATE_COMMIT"
```

Compare changed paths with the exact write set.
Recompute every owned/input digest.
The integrator merges or cherry-picks one candidate at a time.
Refresh a stale base and rerun affected consumers before freezing new bytes.
A conflict-free Git merge does not establish interface compatibility.

`proving::source_closure` binds Core, policy, relation, signed, CLI and Cargo source bytes.
G1 arithmetic and G2 policy changes can invalidate unrelated signed contexts or derived material.
Preserve those bindings. Do not reuse old manifests, signing contexts, keys or audit hashes after their closure changes.
This workflow neither opens nor regenerates native material.
Freeze the complete integrated tree for fresh independent audits after composition.

## Fresh reviews

Every final candidate requires fresh non-author Sol and Grok reviews at high effort.
Round 1 uses these reviewer seats:

| Author lane | Fresh Sol reviewer seat | Fresh Grok reviewer seat |
|---|---|---|
| S1 | S2 | G1 |
| S2 | S3 | G2 |
| S3 | S1 | G3 |
| G1 | S1 | G2 |
| G2 | S2 | G3 |
| G3 | S3 | G1 |

A reviewer seat means a fresh session, not reuse of that seat's author context.
The same-provider reviewer is a different author lane. It must not have authored the candidate under review.
Rotate the same-provider non-author seat and the opposite-provider seat when changed candidates require another round.
Use `--review-round N` to display the next assignment.
Never substitute the candidate's author for a missing reviewer.
Preserve actual identities, dissent, scope and terminal status.

Lane reviewer rotation applies only to that lane's candidate.
The complete composed tree uses two separate reviewer roles outside the six author lanes:

| Integrated reviewer | Requested model and effort | Scope |
|---|---|---|
| R-S | `gpt-6.1-sol` high | Fresh read-only full composed-candidate review |
| R-G | `grok-4.7` high | Fresh read-only full composed-candidate review |

R-S and R-G have no implementation assignments.
Neither reviewer may have authored any file or shared integration change in the current composed candidate.
Do not rotate S1–S3 or G1–G3 authors into these integrated review roles.
The manifest and packets record these roles. The lead verifies actual session independence and observed identities.
The recorded plan does not establish a launched reviewer or an approval.

Give both reviewers the complete frozen candidate and applicable contracts.
Actual-result reviewers must also inspect the retained actual output bytes and controls.
Source approval does not approve an execution result.
Component verdicts do not approve a later merged candidate.
Keep consequential design/resource majority decisions separate from routine repairs and result audits.

Use the existing ingestion command only when candidate, authority and scope match its registered action:

```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . review --receipt /path/to/receipt.json
```

The current loan review action has stale inputs and a design-only scope.
Keep unmatched source/result receipts beside their candidate instead of forcing them through that action.
The plugin does not enforce this full reviewer roster.
Its identity substring check can reject same-model session identities.
Report such a refusal. Do not omit the author or invent model identities to bypass it.

## Native work remains separately admitted

Existing frozen signed-call native work retains its Opus 5.5 and Muse audit conditions.
Future Sol/Grok source review does not replace those conditions.
Degree discovery, SRS initialization, key generation, proving, Real verification, ledger apply and Preview need exact current admission.
Preserve physical resource limits, accounting, debit ceilings, stop rules and public transaction notifications.
Six author slots grant no native concurrency or budget.
`NativeWorkLimits` bounds public shapes and bytes. It does not establish physical host capacity.
Do not repeat completed native work or restart an old goal because this workflow exists.

## Check this workflow implementation

```bash
python3 -B -m unittest discover -s tests -p test_parallel_context.py -v
python3 -B "$MORIARTY_CLEAN/scripts/build_task_context.py" --repo "$MORIARTY_CLEAN" \
  --manifest "$MORIARTY_MANIFEST" --graph-dir "$MORIARTY_GRAPH" --lane G1
```

These commands check the workflow utility and author context only.
They do not close a language, native or financial acceptance milestone.
