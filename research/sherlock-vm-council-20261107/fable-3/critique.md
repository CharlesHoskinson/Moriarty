# fable-3 cross-critique

Run `sherlock-vm-council-20261107`. Written 2026-11-07 after reading all six positions and the supplements posted by 20:56Z (`astra-1/assurance-proposed.md`, `astra-1/refinements.md`, `astra-2/security-clauses.md`, `astra-3/proposed-text.md`, `astra-3/observations.md`, `astra-3/workflow-critique-notes.md`, `wiki/midnight-ledger-9-hardfork.md`). Paths are relative to `/home/hoskinson/Projects/Moriarty` at `8fd34ed79`. My lens is the delivery workflow, so I critique mainly what each position says about engine, mapping, gates and the network path, and I note where I verified a peer's code claim myself.

## Where the six positions already agree (no issue needed)

1. The engine is `scripts/doctorwatson` (`python3 -m factory`) with `next/check/heavy/stop-rule`; the plan/attempt engine in `scripts/doctorwatson.sh` is history (fable-1 C8, fable-2 K7, fable-3 §0, astra-1 K4, astra-2 K4, astra-3 Keep 4). `AGENTS.md:9-11` and `docs/DOCTORWATSON*.md:3` settle it.
2. A8 is failing because the redeployed anchor `2391d62f…` has a `ContractDeploy` and no call, not because recursion is unsound (fable-1 I8, fable-2 I6, fable-3 I2, astra-3 "Diagnose A8", `astra-3/observations.md:9-23`). I reproduced the same `FAIL: only a ContractDeploy (block 1032519)` reasoning from `scripts/qa/check-a8.sh` and the fixture.
3. A8 accepts any `ContractCall` at the address and A10 picks the newest statement by mtime (astra-1 "Strengthen target receipt identity", astra-3 proposed-text "Exact endpoint acceptance", fable-2 I6). Verified: `scripts/qa/check-a8.sh:30-35`, `scripts/qa/check-a10.sh:8-13` (`max(files, key=lambda p: p.stat().st_mtime_ns)`).
4. The 16,000 s dispatch run is a test-cost problem to be shrunk, not a cap to be raised (fable-1 I2, fable-2 I9, fable-3 I7, astra-1 Cut 4, astra-3 "Treat C6 as a test-cost problem"). `AGENTS.md:17-18` already mandates the shrink.
5. Stagenet (`shielded.tools`) is a pre-release ledger 9 and its acceptance does not transfer to mainnet; Preview is the rehearsal network (all six; my RPC read `specVersion 2000000` vs Preview `2001100`, astra-3 and fable-2 read the same).
6. Only C0 (phase profile) is agreed in recursion-perf; no speedup is adopted on an estimate (fable-1 C5, fable-2 I1/K6, astra-1 Cut 3, astra-2 Cut 3, astra-3 assurance).
7. Mainnet needs an explicit path with rollback that does not exist yet (fable-1 M5, fable-2 M6, fable-3 I3/M6, astra-1 M5, astra-2 M6, astra-3 M5).

## fable-1 (chair, VM architecture)

Agree:
- K8/I7: workflow compiles to `predicates.yaml` rows and the existing commands; a `source:` anchor checker is needed. fable-1 adds a fact I missed: `SETS = ("A","B","C","U","F")` is closed (`models/doctorwatson/factory/predicates.py:11`; `validate` rejects any other set at `:101-102`; `status.py:36` and `cli.py:38` iterate it). So the exact addition list for the consensus change is: (a) `predicates.py:11` gains a set letter for the new change (or new rows reuse A/B/U/F), (b) `TIERS` at `:12` gains `T4` only if the council keeps mainnet in scope, (c) a committed `scripts/qa/check-traceability.py` run as an F predicate rather than a new `workflow validate` subcommand (no `cli.py` change; the F set already hosts `validate` and `check-f*.sh`). I prefer (c) over a new subcommand because `CONTRACT.md:5` says interfaces change only through the coordinator.
- I2: the T1/T2 split of the dispatch receipt with a <2 h budget and per-circuit timing is more specific than my I7 and I adopt it.
- M4 then M5 (stagenet, then mainnet readiness with Preview acceptance before mainnet). I agree on the ordering but see the disagreement below on the network name.

Disagree:
- M4 is labelled "Stagenet" with `SherlockRun` compiled by Compact 0.36.0. Compact 0.36.0 artifacts target ledger 9.1.0.x (`https://docs.midnight.network/relnotes/ledger-9.md`: 0.36.0 generates only ZKIR v3 for the new era), and the stagenet runtime is `specVersion 2000000`, node `2.0.0-d9729c13` (my RPC read; `astra-3/observations.md:27-35`; fable-2 M6). Whether a 0.36.0-compiled contract deploys on that runtime is unknown. If the chair keeps M4 on stagenet, M4 must say "stagenet if it accepts 0.36.0 artifacts, else Preview", or simply move M4 to Preview as fable-2 M6 and astra-3 milestone 2 do. Evidence: `tools/sherlock-sdk/stagenet-session.mjs:24-31` hard-codes `stagenet`; nothing in the repo tests a 0.36.0 build against it.
- I7's "per-change set prefix" would split the predicate namespace per OpenSpec change. `status.json` counts per set (`status.py:36-40`) and `next` prints one line per set. A set per change means the headline counts stop meaning anything after two changes. Use the existing sets and let the `source:` anchor carry the change identity; add a set only if the council needs a sixth headline number.

Concede: fable-1's M0 "re-pin and re-key first" over my M0 "re-arm first". My M0 was a cheap status fix; re-pinning is the real first milestone because midnight-js 5 throws `EraMismatchError` for ledger-8-era transactions (`ledger-9.md` via `wiki/midnight-ledger-9-hardfork.md`) and Preview is already on 9.1.0.1, so every mutating operation on a current network needs the new SDK set before it can run at all.

## fable-2 (recursion and proving)

Agree:
- M0 gate G0 (positive local acceptance plus the full refusal set at the new pins, with a key manifest I7) is exactly the `check-a6.sh` pattern and can be an A-set predicate with a budget. The manifest fields in I7 (relation SHA-256, VK SHA-256, header version, circuits/zk-stdlib/proofs versions, ledger rev, Compact and runtime versions, template SHA-256) are the right content for my proposed `qa/sherlock/target-pins.json`; I merge the two.
- I6 (A8 reads the post-call state and checks `stmt` against the bundle) and I8 (recapture the acceptRun template with 0.36.0).
- C5 (B chain non-gating for this change). In factory terms this needs nothing: B rows stay in `predicates.yaml`, `next` still reports them, but the consensus `tasks.md` does not list B4/B5/B8/B9 as milestones. One caution: `firstFailing` is computed in topological `depends` order with ties by file order (`CONTRACT.md:67`), so a failing B row can still be named first once A8 passes. If that is unwanted, B rows get a `depends` on the new change's exit predicate, which is a YAML edit.
- M6 "rollback = new deployment with a new address and manifest; the decider VK is hand-written, so redeploy rather than patch." This matches `wiki/midnight-ledger-9-hardfork.md` (no maintenance authority → redeploy) and my §4 rollback.

Disagree:
- I1 "two concurrent heavy proofs when the sum fits the cap". `heavy` takes one exclusive flock on `heavy.lock` (`CONTRACT.md:96`); the 10-25 retrospective's F7 finding is that the lock is too coarse, and the fix landed as a ticket queue (`q`), not as a multi-slot lock (`wiki-llm/doctorwatson-workflow-review-2026-10-25.md:58,70`). Running two proofs at once needs either `--gpu`-style second lock or a `DW_HEAVY_SLOT`-aware admission in `heavy.py`; that is an engine change and must be named as such, not assumed.
- M5 after M4: running the C0 profile only after end-to-end P1-P3 means the 7.7 h fact(3) estimate drives M4's budgets. The profile's tasks 5.1-7.1 (`openspec/changes/recursion-phase-profile/tasks.md:44-60`) are cheap relative to M4 and already have a wrapper; I would put the leaf/node/decider profile inside M3, so M4's budget keys are measured, not estimated. fable-2's own K6 ("no estimate counts") argues for this.

Concede: fable-2's §Midnight table is more complete than mine on serialization/storage (`midnight-storage 3.0.0` API break) and on the public-input limit question (Q2). My §6 did not cover the storage break; it is an M0 source-change item.

## astra-1 (assurance)

Agree, and verified:
- `status.py` `_update` merges old entries under a freshly read HEAD without checking each entry's source closure (`models/doctorwatson/factory/status.py:77-112`; read today). `view` counts retained `passes` flags (`:34-47`). So `status.json` "head" is the head of the last write, not of each result. ASR-002 (stale on dependency change) is right in principle. Exact, minimal factory addition: `runs.jsonl` already records `head` per run and `status.json` records `run` per predicate, so a read-only `next --fresh` (or a `scripts/qa/check-f*.sh` predicate) can mark every passing entry whose run `head` differs from HEAD as `stale` without changing `passes`. A full declared-dependency closure per predicate (ASR-002's "unrelated file change may retain evidence only through a complete declared dependency closure") is a larger design; I would start with the head-based stale flag and a per-predicate `inputs:` list in `predicates.yaml` (schema change at `predicates.py:95-160`).
- Mutation accounting (ASR-003/004): astra-2 and astra-3 independently verified it in `crates/mori-backend/src/qa_mutation.rs`. I did not read the file; I accept it on two independent verifications and because T1 runs that harness (`qa/tiers/t1.sh:13`), so the fix lands in a tier that already exists.
- ASR-007 (receipt bound to the candidate manifest, not mtime). Same as item 3 above.

Disagree:
- "Add typed negative outcomes and source-aware evidence validation to the factory" in M0 before new heavy proofs. The factory's unit of truth is the predicate exit code (0/1/2/3/4, `CONTRACT.md:53`); typed outcomes belong in the check's receipt JSON and in the holdout runner (`qa/holdout/lib/*.py`), not in `heavy`/`check`. Keep the engine small; put the taxonomy in the receipt schema that the holdout decodes (the U023 contract already does this for stages, `qa/holdout/contracts/U023.json`).
- M4 runs "stagenet and ledger-9 Preview checks with separate evidence records". Agree on separate records, but doing both doubles wallet, proof-server and agent time for a class-4 result on a network no user runs (astra-3 conceded this at `workflow-critique-notes.md:19`). Stagenet should be optional diagnostic only.

Concede: ASR-012 ("a report generated from a task plan must leave unexecuted and unproved claims open") should be the rule for the consensus `traceability.md` itself: every row starts as `unimplemented` (exit 2) and the only way to a closed row is a `runs.jsonl` line.

## astra-2 (security)

Agree:
- Milestone 1 "pin the target and the next executable failure" and milestone 6 "promote the qualified tuple to Mainnet; a test-network receipt alone cannot establish Mainnet currentness" match my M2/M6.
- Keeping security acceptance checks attached to concrete predicates (K4) rather than to a review gate.

Disagree (workflow placement, not substance):
- The six-versus-seven lane mismatch (verified by astra-1 and astra-3 in source; I did not verify it) and the canonicality invariant are design issues for fable-1's lens. From my lens the question is only: which predicate fails first when they are unfixed? Today none does, because U023 scenario 2 and the GP chain have no predicate below U023. The consensus change needs an A-style predicate for "leaf schema accepted by the IVC consumer" with a visible test, so the mismatch is a red `check`, not a paragraph. That is a `predicates.yaml` row plus a `cargo test` filter, nothing more.
- "Treat changes as migrations requiring state/duty preservation and applicable authority." For the current anchors this is moot: they are rc-era contracts on a non-production network with no liabilities. The migration rule should bind from the first Preview deployment that carries a maintenance authority onward; before that, redeploy.

Concede: nothing of mine conflicts with astra-2; the GHSA field-alias advisory (`https://github.com/midnightntwrk/midnight-ledger/security/advisories/GHSA-wr7g-rr4v-jmj8`) is a good argument for the I8 pins check to record the ledger rev, not only the version string.

## astra-3 (critic)

Agree, and I adopt these corrections to my position:
- Point 2: do not assume every current key is invalid. My §6 said "every key in `stagenet-verifier.json` is expected to be invalid". Correct statement: the release notes name specific circuit classes whose keys change; our hand-written node/decider must be keygen-regressed at the new pins (fable-2 Q1, M0) and the affected closure regenerated. I withdraw the blanket claim and keep the mechanism (decider key depends on parent VK, `wiki-llm/lessons/sherlock.md:19-25`) as the reason the regression matters.
- Point 3: live RPC/indexer checks out of T0. Agreed: T0 validates the static `target-pins.json` against `Cargo.toml`/SDK lockfile; the live `midnight_ledgerVersion`/`specVersion` comparison goes into `qa/tiers/t3.sh` before `check --tier T3`, as a `dw_step` with exit 3 on unreachable. T0 must never depend on a remote endpoint.
- Point 6: a stop-rule reset is not a success target. Agreed; M0's F4/F11 re-run is bookkeeping and must keep the STOP trace and owner disposition. I reword M0 as "record the STOP, then continue with M0-repin".
- Point 7: rollback must address accepted liabilities and old keys; reverting a fixture commit does not reverse chain state. Agreed; my §4 said as much but the astra-3 wording is sharper and I adopt it.
- Point 1 and concession: stagenet as optional diagnostic, Preview as the first production-compatible public target, named operations with pre/post predicates.

Disagree:
- Point 5: "an additional tier is one implementation choice, not a prerequisite." True, but the alternative (mainnet rows at `tier: T3` with `network: mainnet`) puts an irreversible, fee-bearing operation into a tier whose runner prints a stagenet checklist and launches a background reviewer (`qa/tiers/t3.sh:40-51,56-70`). A separate `t4.sh` with an owner-authorization step is the smaller risk. Either way `predicates.py:12` changes or `network:` is added; I keep T4 as my recommendation and accept `network:` on T3 as the fallback if the chair prefers no `TIERS` change.
- Milestone 2 "requalify the existing recursive endpoint … first use a qualified non-Mainnet target" before re-pinning. With rc-era pins the only reachable network is stagenet; with new pins the only reachable networks are Preview and mainnet. Requalifying on stagenet first spends a day on class-4 evidence that we have agreed does not transfer. I side with fable-1/fable-2: re-pin first (M0), then requalify on Preview.

## What I now concede overall

1. Re-pin (fable-1 M0, fable-2 M0) precedes everything mutating; my "re-arm" M0 shrinks to a bookkeeping step.
2. Blanket "all keys invalid" withdrawn (astra-3).
3. Live checks move from T0 to the T3 runner (astra-3).
4. The dispatch receipt follows fable-1 I2's T1/T2 split.
5. `SETS` is closed (fable-1); the exact-additions list grows by one line.
6. Status freshness is a real gap (astra-1); minimal fix is a head-based stale flag, not a new database.

## Issues I ask the chair to list

1. Network for M4: Preview (fable-2, fable-3, astra-3) vs stagenet with 0.36.0 artifacts (fable-1). Needs one answer; stagenet's acceptance of 0.36.0 builds is untested.
2. Mainnet in this change: T4 tier (fable-3) vs `network:` on T3 (astra-3) vs successor change (fable-3 Q1). Also whether `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` is amended.
3. Phase profile placement: before M4 (fable-3) or after (fable-2 M5).
4. Concurrency of heavy proofs (fable-2 I1) requires an engine change to the single flock; in or out of scope.
5. Staleness: head-based flag (fable-3) vs declared-dependency closure (astra-1 ASR-002); which lands in this change.
