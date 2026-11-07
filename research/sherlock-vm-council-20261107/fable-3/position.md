# fable-3 position: delivery workflow, EARS-to-evidence mapping, deployment path

Council run `sherlock-vm-council-20261107`. Lens: DoctorWatson workflow, requirement-to-task-to-evidence mapping, local → stagenet → mainnet path. Written 2026-11-07 before reading any other position. Every claim carries a path (relative to `/home/hoskinson/Projects/Moriarty`, commit `8fd34ed79`) or a URL. Nothing in the checkout was changed; no plan, attempt or task was started.

## 0. What the engine actually is today

There are two DoctorWatson surfaces, and the council workflow must target the right one.

1. **`scripts/doctorwatson` (current).** Two lines: `cd models/doctorwatson && exec python3 -m factory "$@"` (`scripts/doctorwatson:2`). Subcommands, confirmed from `scripts/doctorwatson --help` and each `<cmd> --help` on 2026-11-07: `next`, `check [ID…] [--set S] [--tier T] [--hypothesis TEXT]`, `preflight`, `heavy --id ID [--budget KEY] [--hypothesis TEXT] [--pipe-stdin] [--gpu] [--deadline EPOCH] [--max-wall S] -- CMD…`, `q {submit,status,cancel,wait,reorder,hold,release}`, `reap [--execute]`, `stop-rule`, `holdout {verify,seal}`, `validate`, `bus {doctor,status,tail,inbox,publish,review,watch}`, `clean`, `state-commit`, `contract-check ID_OR_PATH`, `spawn --vendor {claude,codex,grok} --as NAME --role {implementer,quint,auditor,lead,helper} --brief FILE …`. The frozen contract is `models/doctorwatson/factory/CONTRACT.md`; the charter is `AGENTS.md:13-24`. State is `qa/sherlock/predicates.yaml` (spec), `qa/sherlock/status.json` and `qa/sherlock/runs.jsonl` (results), `qa/sherlock/budgets.json` (measured caps) (`CONTRACT.md:14-24`).
2. **`scripts/doctorwatson.sh` (legacy).** `--inspect/--execute/--cleanup/--design*/--host` over `models/doctorwatson/{delivery,run_selected,reconciliation,cleanup}.py`. Its three docs each open with "Superseded 2026-10-24 … technical descriptions below stand as history of the legacy tooling" (`docs/DOCTORWATSON.md:3`, `docs/DOCTORWATSON-DELIVERY.md:3`, `docs/DOCTORWATSON-CI.md:3`). `AGENTS.md:9-11`: "All earlier routing, review, adoption-vote, campaign, lane and evidence rules are retired; their files remain only as history and grant or block nothing."

Consequence: the consensus `workflow.md` must be written against surface 1. Anything that says "plan JSON", "attempt directory", "consumed attempt", "Root adoption" or "`--inspect`" is surface 2 and will not run.

Current factory state (read-only, 2026-11-07 20:49Z):
- `scripts/sherlock-next`: A 7/10, B 4/9, C 5/6, U 34/35, F 11/13; FIRST FAILING A8 (`scripts/qa/check-a8.sh`).
- `DW_BUS=off scripts/doctorwatson stop-rule` → exit 6, `STOP: escalate to owner`, "295.0 h since the last flip (limit 24 h)", last failing trace `qa/sherlock/traces/20261026T105549Z-U023-dispatch-receipt.log`. The factory has been in STOP since 2026-10-27 by its own rule (`models/doctorwatson/factory/stoprule.py:1,9`). `status.json` head is `7e22b5823`, not `8fd34ed79`; `status.json` has not been written since 2026-10-26T13:52:10Z.
- `scripts/doctorwatson preflight` → OK (3454 GiB free, load 1.91/20, heavy lock free).
- Failing rows in `status.json`: F4 (exit 3 at the time), U023 (exit 1), C4 blocked by U023, A8 (exit 1), A9/A10/B4/B8 blocked, B6/B7 exit 1, F11 exit 1.
- `qa/sherlock/runs.jsonl`: 394 runs, 118 with a hypothesis; U023 13 runs (11 fail, 2 unimplemented), `U023-dispatch-receipt` 1 run exit 4 (budget exceeded at the 16,000 s wall cap, `docs/sherlock/HANDOFF-2026-10-26.md:16`).

## 1. Keep

K1. **The predicate loop and its five commands.** `next → check → heavy → stop-rule`, with `status.json`/`runs.jsonl`/git as the only truth (`CONTRACT.md:89-99,116-119`). The 2026-10-24 lesson is explicit: the old approach "used review gates, consumed attempts, and failure traces that nobody opened"; with the factory "A2 and A3 passed within hours" (`wiki-llm/lessons/sherlock.md:29-31`). Keep `check` exit codes 0/1/2/3/4 (`CONTRACT.md:53`) and the `UNIMPLEMENTED` convention (`CONTRACT.md:52`).

K2. **Measure-then-cap budgets and the hypothesis rule.** `heavy` refuses an identical re-run of a failed command at the same head without `--hypothesis` (exit 5) (`CONTRACT.md:96`); it worked in practice (`wiki-llm/lessons/factory-and-agents.md:43-44`). Budgets in `qa/sherlock/budgets.json` already hold measured A2/A3/A4/A6/B3/B4/B5 costs (e.g. `a3-parent-k20` 315.88 s, 15.76 GB; `b5-prefix-batches` 5558.9 s).

K3. **Predicate sets A/B/C/U/F with `source:` back-references** (`qa/sherlock/predicates.yaml`). The U set is one predicate per UIREQ, `source` pointing at `openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md#requirement-uireq-NNN-…` (`predicates.yaml:180-494`); C1–C6 are pure composites gated by `depends` (`predicates.yaml:495-548`, `scripts/qa/check-c.sh`). This is the EARS→check mapping the council needs; it exists and validates (`scripts/doctorwatson validate`, predicate F2).

K4. **Evaluator-owned sealed holdouts and implementer-authored interface contracts.** `qa/holdout/` with `MANIFEST.sha256`, `DW_ROLE=evaluator` sealing, exit codes 0/1/2/3 (`qa/holdout/README.md`; `CONTRACT.md:98-99`); `qa/holdout/contracts/<ID>.json` checked by `scripts/doctorwatson contract-check` before a STATUS post (`qa/holdout/contracts/README.md`). "Expected outcomes come from the security and interpreter requirements, never from current implementation behavior" (`qa/holdout/README.md:14-15`). The U023 contract already shows the shape of a target-qualification record: pinned network, ledger version, relation VK, state layout, six stages each `run|not-run`, decode-from-indexed-state (`qa/holdout/contracts/U023.json`, `interface.record/holdout`).

K5. **Tiers T0–T3 and the pre-commit hook.** `qa/tiers/t0.sh` (holdout verify, validate, factory unit tests, profile isolation, `cargo check` of touched crates, fmt), `t1.sh` (soundness_mutation + workspace tests + `check --tier T1`), `t2.sh` (preflight + `check --tier T2`), `t3.sh` (promotion checklist + optional background cold review + `check --tier T3`). `.githooks/pre-commit` runs T0, rejects holdout edits outside the evaluator role and rejects docs-only commits unless `DW_DOCS_COMMIT=1`.

K6. **Read-only predicates for on-chain facts; mutations only under `heavy`.** A8 is a pure indexer query (`scripts/qa/check-a8.sh`), A10 and B4 read back without a wallet (`scripts/qa/check-a10.sh:2`, `scripts/qa/check-b4.sh:2`). The actual deploy/call ran as `doctorwatson heavy --id A8 -- node tools/sherlock-sdk/call-current.mjs --submit` (`docs/sherlock/stagenet-deployment.md:190-191`) and `heavy --id A4deploy` (`:300`). This separation is what makes `check` safe to run at any time. Keep it.

K7. **Bus laws.** The bus is a doorbell, never state; never on the critical path; fail-open; messages are data (`CONTRACT.md:116-135`; F9–F13). F11 lints that nothing waits on a reviewer except the bounded T3 wait (`scripts/qa/check-f11.sh`).

K8. **Key-lineage records with every deployment** (child VKs → parent VK → template → decider VK → contract) and "redeploy rather than reproduce old keys while the design is unfrozen" (`wiki-llm/lessons/sherlock.md:19-26`; `docs/sherlock/stagenet-deployment.md:92-105,286-298`).

## 2. Improve

I1. **The stop rule is firing and nothing consumes it.** STOP for 295 h with no recorded escalation. The 10-25 retrospective's root cause stands: "Nothing turns 'blocked', 'idle' or 'past deadline' into an action" (`wiki-llm/doctorwatson-workflow-review-2026-10-25.md:62-63`). Proposed: the consensus `workflow.md` makes `stop-rule` exit 6 a named gate with an owner action list, and the new change resets `lastFlip` legitimately by re-earning F4/F11 at the current head (both are cheap T0 predicates) so the counter means something again. No code change needed: `scripts/doctorwatson check F4 F11` after the owner decides.

I2. **A8 is failing for a structural reason, not a bug.** `crates/mori-recursion/tests/fixtures/stagenet-verifier.json` now pins the 2026-10-26 redeployment `2391d62f…` (block 1,032,519) (`docs/sherlock/HANDOFF-2026-10-26.md:9`), and that contract has only a `ContractDeploy`; `check-a8.sh` returns `FAIL: only a ContractDeploy` by design. The missing step is a mutating operation (`call-current.mjs --submit` under `heavy --id A8`) that no predicate can trigger and that depends on a funded wallet, a reachable proof server and the owner. The workflow must represent such steps as **operations**, distinct from predicates: an operation has an id, a budget, a precondition predicate set, a `heavy` command, a post-condition predicate, and an authorization line. Today this list lives only in prose (`HANDOFF-2026-10-26.md:23-25`).

I3. **Add a `tier: T4` (mainnet) and a `network:` field, or an explicit decision not to.** There is no mainnet path anywhere in the factory. `rg -i mainnet` finds no hit under `qa/`, `scripts/`, `models/doctorwatson/`, `tools/sherlock-sdk/` or `openspec/changes/sherlock-*`. The only rule is `openspec/MORIARTY-COMPLETION-PROGRAM.md:269`: "Use local Docker before public attempts; make no mainnet submission." `predicates.py:12` fixes `TIERS = ("T0","T1","T2","T3")`. Exact additions: (a) `TIERS` gains `T4`; (b) predicate schema gains optional `network: undeployed|stagenet|preview|mainnet`; (c) `qa/tiers/t4.sh` with a checklist mirroring `t3.sh:40-51` plus owner authorization, Blockfrost project, DUST registration lead time, and a mandatory rollback plan; (d) `validate` enforces that a T4 predicate `depends` on at least one T3 predicate with the same statement/key lineage. Until (a)–(d) exist, `workflow.md` must say mainnet is **out of scope for this change** and keep `MORIARTY-COMPLETION-PROGRAM.md:269` in force.

I4. **Re-point the ledger-9 validation target.** See §6. Stagenet RPC reports `midnight_ledgerVersion` `=1.0.0` and `specVersion 2000000`; Preview reports `crate-ledger-9.1.0.1`, `specVersion 2001100` (my read-only queries, 2026-11-07 ~20:50Z, `https://rpc.stagenet.shielded.tools`, `https://rpc.preview.midnight.network`). Mainnet is documented at 9.1.0.1 / node 2.1.101 / specVersion 2001100 (`https://docs.midnight.network/relnotes/ledger-9.md`, `https://docs.midnight.network/relnotes/support-matrix.md`). Stagenet is therefore an rc-era chain, not the mainnet runtime. The T3 tier should be parameterised by network, and the promotion ladder should be `undeployed (local docker) → preview → mainnet`, with stagenet kept only as a historical pin. That is a change to `scripts/qa/check-a8.sh:8` (`URL`), `scripts/qa/assemble-u023-record.py:31-32` (INDEXER/RPC), `tools/sherlock-sdk/*.mjs` endpoints, `qa/holdout/contracts/U023.json` (`pins.network:'stagenet'`), and a fresh fixture file beside `stagenet-verifier.json`.

I5. **Make the EARS→predicate map a checked artifact, not a table in prose.** Today `predicates.yaml` has `source:` but no reverse check that every requirement ID in the new `requirements.md` has at least one predicate, and no check that a predicate's `source` anchor exists. Exact addition: a T0 check `scripts/qa/check-traceability.py` that parses `openspec/changes/sherlock-vm-consensus-20261107/requirements.md` IDs, `traceability.md` rows and `predicates.yaml`, and fails on an unmapped requirement, a dangling anchor, or a predicate without a holdout or visible check. This is one Python file plus one `F` predicate; it fits `CONTRACT.md:7-10` (stdlib + PyYAML).

I6. **Receipts must be committed, not left in `target/`.** The old host loss orphaned every gitignored receipt (`wiki-llm/doctorwatson-improvement-log.md:2317-2318` per my helper's read; `HANDOFF-2026-10-26.md:7`: A5–A10 and B3–B9 passes "are from the old host … The owner decided to re-earn them"). `qa/sherlock/receipt-traces/` exists for this. Rule for `workflow.md`: a T3/T4 pass is not a pass until its public receipt (tx hash, block, address, VK lineage, state hash) is committed under `qa/sherlock/receipt-traces/<ID>/` in the same commit as the status flip; `check-a10.sh`-style readback predicates should read the committed receipt first and the indexer second.

I7. **Give the U023 "last engineering" an executable gate.** The C6 dispatch receipt run hit its 16,000 s wall cap without a receipt (`HANDOFF-2026-10-26.md:16`, run `20261026T105549Z-U023-dispatch-receipt`, exit 4). SF-14 and the charter say: after two failures of the same class, shrink the reproducer (`AGENTS.md:17-18`). The next attempt must not be "raise `--max-wall`"; it should be a split predicate (per-rule fragment receipts, each under its own budget key) so each run fits a measured cap. This is a predicate-design improvement, not an engine change.

I8. **Pins drift silently.** `docs/sherlock/stagenet-deployment.md:112-114` pins Compact 0.34.0, runtime 0.19.0, midnight-js 5.0.0-beta.7, wallet-sdk 2.0.0-beta.2, proof server 9.0.0-rc.5_experimental; `HANDOFF-2026-10-26.md:45` pins `onchain-runtime-v4` 4.0.0-rc.3. The T3 checklist says "compiler and SDK pins match the repo pins" (`qa/tiers/t3.sh:44`) but nothing checks it. Exact addition: `qa/sherlock/target-pins.json` (network → ledger crate, node, compact toolchain, midnight-js, wallet-sdk, proof-server image, indexer) and a T0 predicate that compares it with `tools/sherlock-sdk/package.json` (or the SDK root lockfile) and with the live `midnight_ledgerVersion`/`specVersion` of the pinned RPC (read-only, exit 3 if unreachable).

## 3. Cut

C1. **The legacy `scripts/doctorwatson.sh --inspect/--execute/--cleanup` routes from the consensus workflow.** They are superseded (`docs/DOCTORWATSON*.md:3`) and their vocabulary (plans, attempts, reconciliations, adoption) contradicts `AGENTS.md`. Keep the files as history; do not reference them in `workflow.md`. The `--design*` Quint checks of the *legacy* delivery flow can stay as optional model checks but are not a gate.

C2. **"Root adopts", "two auditors must approve", "Fable medium + GPT-6 high approval" style gates** in older task lists (`openspec/changes/mc03-native-recursive-proof/tasks.md:14,47,61`). `AGENTS.md:22-25`: "Reviews are inputs. Nothing waits on a reviewer. The one review point is the T3 stagenet promotion." F11 enforces it. The new `tasks.md` must contain no approval checkbox other than the T3/T4 owner authorization for a mutating network operation.

C3. **Stagenet as the acceptance target for ledger 9.** Keep its evidence as history; stop spending wallet, proof-server and agent time there (see §6). This also retires the B4/B8 stagenet rows in their current form; they come back as preview rows.

C4. **Documentation-only cycles in the inner loop.** Already enforced by the hook (`.githooks/pre-commit:40-56`). The council's own output is a milestone docs commit (`DW_DOCS_COMMIT=1`) and should be the only one until a predicate flips.

C5. **Hand-maintained status prose** such as `docs/sherlock/requirements-status.md` and `openspec/changes/sherlock-completion-convergence/current-evidence.json` as sources of truth. `status.json`, `runs.jsonl` and the holdout exit codes are the truth (`CONTRACT.md:118-119`); narrative status files are derived views and must say so.

## 4. Implementation plan (ordered milestones)

Each milestone names its predicates, its gate (a tier runner or `check` call), and the owner action it needs. "Flip" means `status.flip` in `status.json`.

**M0 — Re-arm the factory at the council head (T0, same day).**
- `scripts/doctorwatson validate`; `scripts/doctorwatson check --set F` (re-earn F4, F11 at `8fd34ed79`; F11's failing trace is gitignored, so re-run and read it); `scripts/doctorwatson state-commit`.
- Gate: `scripts/doctorwatson stop-rule` exits 0 after the flip. If no F predicate flips, the owner records the STOP and the reason in `wiki-llm/lessons/` (`DW_DOCS_COMMIT=1`).

**M1 — Council change lands as a milestone docs commit (T0).**
- `openspec/changes/sherlock-vm-consensus-20261107/{proposal,design,requirements,assurance,workflow,tasks,traceability,dissent}.md` plus `specs/*/spec.md`.
- New predicates added to `qa/sherlock/predicates.yaml` as `check: scripts/qa/unimplemented.sh <ID>` first (`CONTRACT.md:52`), each with `source:` pointing at the new `requirements.md` ID. New F predicate `F14` = traceability check (I5), `F15` = target-pins check (I8).
- Gate: T0 passes; `scripts/sherlock-next` names the new first failing predicate.

**M2 — Preview target qualification (T3 on preview; mutating; owner-authorized).**
- Operation OP-PV-1: create preview wallet, faucet funding, DUST registration (`https://docs.midnight.network/guides/networks-and-environments.md`). Operation OP-PV-2: `heavy --id A4deploy-preview -- node tools/sherlock-sdk/deploy-current.mjs` with the recompiled anchor (Compact 0.36.0, see §6) and a new fixture `crates/mori-recursion/tests/fixtures/preview-verifier.json`. Operation OP-PV-3: `heavy --id A8-preview -- node tools/sherlock-sdk/call-current.mjs --submit`.
- Predicates: A6′ (local ledger-9.1.0.1 acceptance, same shape as `check-a6.sh`), A8′ (indexer `ContractCall` on preview), A10′ (readback), A9′ (holdout `rejections-preview`, evaluator-owned). All read-only.
- Gate: `qa/tiers/t3.sh --review` with `--base` at the M1 commit; receipts committed under `qa/sherlock/receipt-traces/` (I6).

**M3 — Recursion-design milestones as predicates (T2).** Whatever the council fixes for the VM/recursion design (fable-1, fable-2 lenses) enters as A-chain-style predicates with budgets: child proofs, parent, terminal decider, independent verification, five rejection cases. The existing A2–A7 scripts are the template (`scripts/qa/check-a2.sh … check-a7.sh`). Each new circuit layer adds one budget key and a key-lineage row. Gate: `qa/tiers/t2.sh`.

**M4 — U023 closure by split receipts (T1/T2).** Per-fragment predicates `U023a…U023e` (KAdminPublishValue, BHelperValidate×4, BHelperEqual×4 per `HANDOFF-2026-10-26.md:17,29`), each `heavy` run with its own budget key, then the `entry 7;` six-step chain (`HANDOFF:30`) with `assemble-u023-record.py` re-pointed at preview. C4 flips when U023 flips (`predicates.yaml:522-530`).

**M5 — Rollup rows on preview (T2/T3).** B3 local (already measured, `b3-full-batch-local`), then B4′/B5/B8/B9 on preview. Gate: `qa/tiers/t3.sh`.

**M6 — Mainnet readiness (T4, new; owner-only).** Only after `scripts/sherlock-next` shows no failing T0–T3 predicate for the chosen key lineage. Steps: T4 tier (I3), Blockfrost project, cNIGHT→DUST registration (about 12 h lead), mainnet readiness checklist (`networks-and-environments.md`), one deploy, one call, readback, committed receipts, rollback plan (below). No faucet exists; fees are real. `MORIARTY-COMPLETION-PROGRAM.md:269` is amended in the same change, or M6 does not start.

**Rollback (all networks).** Contract state cannot be undone; the A9 settle cases consume statements (`stagenet-deployment.md:228-231`). Rollback therefore means: (1) never install a key on a contract without a maintenance authority (`HANDOFF:302-307` orphan lesson; `deploy-current.mjs` now validates the password first); (2) a bad deployment is abandoned, its address recorded as orphaned in the receipt, and the fixture re-pinned; (3) `IrInsert/IrRemove` maintenance updates (ledger 9.1.0.0 notes, `https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0`) allow key replacement on a maintained contract, with the known defect midnight-node #2375 (`VerifierKeyRemove` fails on 9.1.0.0 clients) noted; (4) the git side is `git revert` of the fixture/receipt commit plus `scripts/doctorwatson check` to re-derive status. Nothing deletes history (`CONTRACT.md:23`: `runs.jsonl` is append-only).

## 5. Assurance strategy (delivery-workflow view)

A requirement counts as covered only when the chain requirement → predicate → check → evidence → gate is closed and machine-checked:

| Link | Mechanism | Exists? |
|---|---|---|
| requirement → predicate | `predicates.yaml` `source:` anchor | yes (`predicates.yaml`) |
| predicate → check | `check:` script, exit code contract | yes (`CONTRACT.md:53`) |
| check → hidden expectation | evaluator holdout, sealed manifest | yes (`qa/holdout/`) |
| interface freeze before holdout | `contract-check` | yes, 7 contracts (`qa/holdout/contracts/`) |
| evidence persistence | `runs.jsonl` + traces; receipts | traces gitignored; receipts partly in `target/` → **I6** |
| gate | tiers T0–T3, hook | yes; T4 missing → **I3** |
| reverse completeness | none | **I5** |
| pins/currentness | T3 checklist prose only | **I8** |

Evidence classes, in the order the factory already distinguishes them (`docs/DOCTORWATSON-CI.md:163-171` is legacy but the tiering is still right): (1) visible regression tests (`check-uNNN.sh` run them first, `scripts/qa/check-u021.sh:16-19`), (2) sealed holdout scenario (the arbiter, `check-u021.sh:20-22`), (3) measured heavy proof under budget, (4) independent verifier on exact artifacts (A5, `check-a5.sh`), (5) local ledger acceptance with `proof-verifying` on (A6; without the feature bad proofs pass, `wiki-llm/lessons/sherlock.md:11-13`), (6) network inclusion read back from the indexer by a submitter-independent reader (A8/A10), (7) rejection cases on the network (A9). What does **not** count: a passing `check` whose `check:` is `unimplemented.sh` (exit 2), a receipt in `target/`, a `BLOCKED-BY` row, an SDK WASM `wellFormed` accept (`stagenet-deployment.md:184-186`), stagenet evidence for a mainnet claim (§6), a reviewer verdict (reviews are inputs, `AGENTS.md:22`), or any statement in a `.md` file not backed by a `runs.jsonl` line.

Formal models in this lane: `models/doctorwatson/*.qnt` model the legacy flow; the factory package has stdlib unit tests (`models/doctorwatson/factory/tests/`, run in T0). If the council wants a model of the new workflow, it should be a small Quint model of predicate/tier/network state (checked by `quint test`, 60 s cap, as `scripts/doctorwatson.sh:130-140` does today) and must be labelled "model of the design, not of the shell" (`docs/DOCTORWATSON.md:235-237`).

## 6. Midnight ledger 9 impact (delivery view)

Facts, with sources. My helper's web reading is marked [H]; my own read-only RPC queries are marked [RPC]; repository facts carry paths.

- Mainnet upgraded to ledger 9 on 2026-11-06 via governance, node 2.1.101, `midnight_ledgerVersion` → `crate-ledger-9.1.0.1`, specVersion 2001100 [H: `https://docs.midnight.network/relnotes/ledger-9.md`, `…/relnotes/node/node-2-1-101.md`]. Preview was relaunched as a fresh ledger-9 chain (old contracts gone); Preprod stays on ledger 8.1.3 [H: `…/relnotes/support-matrix.md`].
- [RPC 2026-11-07 ~20:50Z] `https://rpc.preview.midnight.network` → `1.0.1 (tag: crate-ledger-9.1.0.1 …)`, specVersion 2001100, txVersion 4. `https://rpc.stagenet.shielded.tools` → `=1.0.0`, specVersion 2000000, txVersion 4. The stagenet indexer answers HTTP 200. **Stagenet is a different, older ledger-9 runtime than mainnet.** All current A/B evidence (contracts `80a0291c…`, `ac93b80e…`, `2391d62f…`) is rc-era (`stagenet-deployment.md`), produced with ledger-v9 1.0.0-rc.3, proof server 9.0.0-rc.5_experimental, Compact 0.34.0 (`stagenet-deployment.md:47,112-114`).
- Proof system: ledger 9.1.0.0 moves proving to zk-stdlib v2 (midnight-zk-stdlib 2.4.0, midnight-circuits 7.3.0), proofs carried as `ProofVersioned::V3`; `proof_verify` dispatches by version; `ContractOperation` gains a `v3` verifier-key field under V4 variants [H: `https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0`]. **Verifier keys change** for some circuits and "recompiling with 0.36.0 gives different verifier keys even with unchanged source" [H: ledger-9.md]. Our decider key is derived from the parent VK (`wiki-llm/lessons/sherlock.md:19-25`), so every key in `stagenet-verifier.json` is expected to be invalid for mainnet-era artifacts. The whole lineage must be regenerated once on the final toolchain.
- ZKIR: proof server supports ZKIR v3 via midnight-zkir 3.2.0; toolchain 0.36.0 emits only ZKIR v3; no v4 found [H: ledger-9.1.0.0 notes, ledger-9.md]. `wiki/midnight-repositories.md` pins `midnight-ledger` `ledger-8` `a8ab82b` and `midnight-zkir` `zkir-v3` `2ffe2d17` — stale [H]. The ZKIR v3 surface question in `wiki/open-questions.md:55` stays open.
- Contract call semantics: contract-to-contract calls (unshielded), events, ECDSA/Keccak/secp256k1 in the runtime, `UnlockToTreasury`, split-phase `apply_guaranteed_only`, stricter decoding (`noop 0` and non-canonical `field` rejected) [H: ledger-9.1.0.0 notes, node 1.0.400 notes]. Relevance to Sherlock: the `acceptValidity`/`settleValidity` anchor is unshielded-state only, so stricter decoding is the main risk; A6′ local acceptance on 9.1.0.1 is the discriminating check.
- Fees: `parallelism_factor` replaced by `validation_factor`/`guaranteed_factor`/`fallible_factor`; new `min_block_price` floor 10 [H]. B8's fee figures (`stagenet-deployment.md:11,194`) are not transferable; B8′ must be re-measured on preview and later mainnet. Known defect #838: a call paid with exactly one DUST spend can fail (`OutsideTimeToDismiss`); use two or more spends [H].
- DUST resets at the upgrade; new NIGHT needs `registerNightUtxosForDustGeneration`; cap takes about 7 days [H]. Mainnet DUST registration should start about 12 h before any M6 operation [H: networks-and-environments.md].
- Tooling pins for mainnet/preview: Compact toolchain 0.36.0 (language 0.28.0), compact-runtime 0.21.0, midnight-js 5.0.0 (ESM-only, `providers.config:{networkId,ttlSeconds}`, `setNetworkId()` removed), wallet SDK 2.0.0, indexer 4.4.101 (full re-index), proof server 9.0.0 (9.0.1 tag exists 07 Nov), on-chain runtime v4 4.0.0 [H: support-matrix.md, ledger-9.md]. Every pin in `stagenet-deployment.md:112-114` and `HANDOFF-2026-10-26.md:10,45` is stale.
- Path: `undeployed` local network (node :9944, indexer :8088, proof server :6300, pre-funded genesis wallet) → Preview (faucet 5,000 tNIGHT per request) → Mainnet (Blockfrost project required; Midnight-hosted preprod/mainnet endpoints retired 2026-10-09) [H: networks-and-environments.md].
- Deprecation: ledger-8 transactions are rejected on mainnet now; midnight-js 5 throws `EraMismatchError` for them; no fixed date for preprod [H].

Unconfirmed (my helper could not source these; I did not either): the proof-system/curve naming in the release notes, whether stagenet will be upgraded or retired, the proof-server Docker tag format, and whether `@midnightntwrk/ledger-v9` npm final equals crate 9.1.0.1.

Addendum after `MIDNIGHT-READY` (midnight-scout, bus 2026-11-07T20:53:53Z, `wiki/midnight-ledger-9-hardfork.md`): fork block 3,346,139 at 2026-11-06T20:51:24Z (spec 1000300 → 2001100) is confirmed; Preview was reset as a new chain with ledger 9 from block 1572 at 2026-11-07T03:27Z, so any preview pin must record genesis/chain identity, not only the network name (astra-3 made the same point); contracts are limited to 256 operations; repo pins `ledger-9.1.0.1 a2b30128`, `zkir-3.2.0 446c7fec`, `compactc-v0.36.0 b5d9e4df`. This matches my helper's reading and the RPC facts above; nothing in §6 changes except that the fork block is no longer unconfirmed. astra-3 also notes that `Cargo.toml` already pins ledger-v9 at `04c9c5d9`, so the stale wiki row is not the build pin; the I8 pins check must compare the Cargo pin, the SDK lockfile and the live chain, not the wiki.

## 7. Open questions

Q1. Does the council want T4/mainnet in this change at all, or an explicit "preview is the exit" with mainnet as a successor change? I lean to the latter, because `MORIARTY-COMPLETION-PROGRAM.md:269` forbids mainnet submission and the owner's stated exit (`docs/DOCTORWATSON.md:14-21`, 2026-10-21) is a stagenet transaction, now better read as "a ledger-9.1.0.1 network transaction".
Q2. Who is the evaluator for the new holdouts (`DW_ROLE=evaluator`)? `qa/sherlock/roles.json` names `dw-evaluator` but v1 "publishes nothing".
Q3. Is `shielded.tools` stagenet going to be upgraded to 9.1.0.1? If yes, the re-pointing in I4 is a config change; if no, stagenet rows are retired.
Q4. U023 dispatch receipt: split into per-rule receipts (my proposal) or raise `--max-wall` (handoff's first suggestion, `HANDOFF:16`)? The charter's two-failures rule favours the split.
Q5. Should `check` gain a `--network` filter, or is `network:` metadata plus naming (`A8-preview`) enough? Metadata plus naming needs no CLI change.

## 8. Confidence

- High (verified by running or reading the code): the command surface, state files, tier scripts, hook, predicate list, current counts, STOP state, stagenet/preview RPC versions, the absence of any mainnet path.
- Medium: the ledger-9 change list and pins (helper's reading of official docs; two dates conflict; I spot-checked only the RPC versions).
- Low: whether stagenet will be upgraded; whether the decider key lineage survives the 0.34.0→0.36.0 recompile (release notes say some circuits' keys change; ours is a hand-built decider relation, so the parent-VK dependency is the real driver).
- Caveat on method: I did not run any `check`, `heavy` or SDK command; everything mutating is described, not exercised.
