# Opus 5.5 security review: REQUEST_CHANGES

## Candidate identity
- **Candidate:** `f14df735dbcb56eb21e3b9584e92f3744748f36d`. I confirmed this by reading the ref `refs/heads/docs/spec-wiki-plugin-reconcile-20261014`, which the worktree HEAD points to. I could not run `git status`, so I can't prove the worktree has no uncommitted changes. The `__pycache__` files present are covered by `.gitignore:2`.
- **Baseline:** `fc6353bf…`. I compared against the `mid-roadmap-20261014` worktree at `11a7d19f`, which the dossier says has the same tree. I did not check that claim myself. Line counts of SECURITY-REVIEW, SECURITY-ROUTING and AUDIT-PLAN match the candidate (195/4/66).
- **Method:** Read, Grep and Glob only. No commands, tests, SDK, native or ledger actions. No private, PK, SRS or provider material. No GPT security text was used as a judgment.

## Scope of this approval decision
This covers only the currentness, guidance and specification review: AGENTS.md, plugin SKILL/reference/README/manifest, the OSR-1014 addendum, the wiki currentness page with its three README paragraphs, the dossier README and the planned plugin refresh. It is not a milestone, product or security certification. It does not approve exhaustive semantic coverage or any new crypto verification.

## What holds
- **Security ownership:** All security work routes to Opus in AGENTS.md:7-8, SKILL.md:27, execution-focus.md:15,54-60, plugin README:9, `reconciliation-2026-10-14.md:34` and `requirements.md:5`.
- **Native authority and permissionless rule:** Native authority stays zero and frozen counters and budgets are untouched (AGENTS.md:20, execution-focus.md:58-60). The permissionless boundary is kept (AGENTS.md:84, SKILL.md:9).
- **No runtime change in the repo:** `plugin-audit.json` lists only guidance and manifest files plus `test_guidance.py` as changed. `hooks.json` and every script are unchanged. Hooks carry no routing text, so no stale roster gets injected.
- **Guidance removed from the old SKILL and reference is preserved elsewhere:** the F0/P3 mutation controls are in SP04.3 (`sp04…md:74`), the isolation caveat is at PARALLEL-DEVELOPMENT.md:143, and the frozen signed-call Opus+Muse condition is at PARALLEL-DEVELOPMENT.md:333.
- **OSR rows stay within their sources:** All 15 OSR rows are specified-only with acceptance open. Each traces to an actual SEC row, and the hypothesis labels on SEC-03, -08, -17 and -18 survive. Cross-stack, curve and target transfer is refused (OSR-02, -06, -15). OSR-15's facts match `NATIVE-RESULT.md:19,24-25`.
- **Cross-linked clauses exist:** every clause named in the addendum's cross-link table exists as a requirement or scenario in MC03, MC04, MC05 or MC08.
- **README paragraphs:** the three paragraphs match the Opus author text exactly.
- **New UNI-018–024 clauses:** these are nonsecurity and weaken nothing.
- **Frozen material:** historical rosters, frozen originals and contract counters are untouched, per the receipts.

## Findings

1. **High: the planned refresh would change installed runtime and security controls beyond the reviewed bytes.**
   - The mutable source `/home/charl/plugins/moriarty-dev` (manifest `0.1.0+codex.20260912173932`) contains a different, hardened runtime that the repository does not have:
     - `hooks/hooks.json` uses an embedded `python3 -I` bootstrap that pins the whole Python runtime by SHA-256 (`0b40d8c3…`) and falls back to a retained copy under `~/.local/share/moriarty-dev/runtimes/`.
     - It has extra modules: `scripts/moriarty_dev/{hook_protocol,bootstrap,compatibility,deployment}.py`. Its `hook.py:14` imports `hook_protocol`; the repo's `hook.py:12-15` does not.
     - It has `COMPATIBILITY.md` and `tests/test_{compatibility,upgrade}.py`.
   - The repo's `hooks.json:10` runs `python3 "${PLUGIN_ROOT:-${CLAUDE_PLUGIN_ROOT}}/…/hook.py"` with no isolation and no pin.
   - Repopulating the source with "exactly canonical tracked plugin files" and running `codex plugin add` would therefore:
     - drop the runtime pin and the `-I` isolation;
     - drop the input/output bounds (8 MiB / 2 KiB) and the 0.75 s deadline;
     - bypass the documented upgrade path (`COMPATIBILITY.md:45-50`, `deployment.py install`). That path snapshots and restores cache roots that active sessions still use, and warns that "the deleted PreToolUse hook can prevent" a later restoration call.
   - The candidate's plugin README:94-103 and `plugin-audit.md:25-36` prescribe exactly this procedure and never mention the divergence.
   - I inspected the mutable source only, not the installed cache. That the cache matches it is an inference from the version recorded in `plugin-audit.md:38`.
   - **Suggested replacement for README:96:** "Before synchronization, compare every file in the configured mutable source and the installed cache with the reviewed package. The inspected personal source (0.1.0+codex.20260912173932) contains runtime files absent from this repository (a pinned isolated hook bootstrap, `hook_protocol`/`bootstrap`/`compatibility`/`deployment` modules, COMPATIBILITY.md and upgrade tests). Replacing it with repository bytes changes installed hook and runtime controls. Do not refresh until that runtime is reconciled into the repository through a separately reviewed change with Opus security review, or the user explicitly selects a different scope."

2. **Medium: the dossier README overstates review status.** `docs/audits/2026-10-14/reconciliation/README.md:63-65` says "A separate Opus coverage continuation addresses the public handoffs; final independent reviews are retained…". The continuation is still pending, and its final reviews don't exist yet. The Opus first pass also never saw `security-handoffs-wiki.json` (42 path entries; `OPUS-SECURITY-REQUIREMENTS.md:342` records it as absent).
   - **Suggested replacement:** "A separate Opus coverage continuation of the public handoffs, including the wiki handoff file, is pending. It is not an independent approval. Final independent reviews will be retained with their frozen candidate identities when they complete."

3. **Low–Medium: OSR-08's first bullet is too broad.** `security-reconciliation-2026-10-14.md:148` reads as if every unsigned artifact must be refused. SEC-20 only says unsigned output must not be cited as authorization; it does not require refusing a single-payer unsigned artifact.
   - **Suggested text:** "the consumer SHALL recompile under the single-payer rule and SHALL refuse with `one-public-payer` any artifact with more than one public payer;"

4. **Low: stale anchors in the wiki currentness page.**
   - `SECURITY-CURRENTNESS-2026-10-14.md:38` calls AGENTS.md:100 "stale", but the candidate already fixed that line (now AGENTS.md:106). Suggest: "Resolved at candidate AGENTS.md:106 (Preview and Fenrir separate)."
   - The README insertions shifted line numbers by +3. In `:34`, the signed-intent README references should now be `:6,23-24,26`. In `:37`, the native-kernel README reference should be `:25`.

5. **Low: "Maps to" lists drop some SEC predicates.**

   | OSR row | Missing predicates | From |
   |---|---|---|
   | OSR-02 | U2 | SEC-02 |
   | OSR-02 | M2, U7 | SEC-16 |
   | OSR-03 | U2 | SEC-03 |
   | OSR-04 | M17 | SEC-04 |
   | OSR-05 | M13 | SEC-07 |
   | OSR-06 | M9, U0 | SEC-05 |

   No control is weakened, but the traceability is incomplete. Add them.

6. **Low: OSR-11's evidence line (`:201`) overstates what WS-B B1 covers.** B1 (`parameters::tests`) checks the pinned loader. It does not exercise caller-supplied parameters reaching `generate_call_keys`. The oversized-transaction scenario has no current-local control at all, because SEC-14 found no encoded-transaction ceiling. Split the evidence line per scenario and add "no current-local control exists" for the oversized case.

7. **Low: two rows state source readings as current facts.** `:113` ("Today the source reports `accepted`") and `:225` ("Current state: succeeds (DL)") should say "Source reading at `cb8280ef` indicates…; not executed", as OSR-01 and OSR-09 already do. OSR-13 should also restate that double release is a hypothesis.

8. **Low: SEC-15 appears in two roles.** Line 42 says SEC-15 gets no requirement, but line 65 lists it as a source of OSR-02. Clarify that OSR-02 uses SEC-15 only for the qualification that the Oct 11 evidence is historical and backend-only.

9. **Low: execution-focus.md:58 omits two blocked commands.** It does not name `prepare-call` or `prepare-signed-call`, which run the native VM (SECURITY-REVIEW §4:124). Add "`prepare-call`, `prepare-signed-call`" to the list of commands that invoke blocked SDK operations.

10. **Low: SKILL.md:35 says "the linked Opus-owned execution boundary" but has no link.** Use `[Opus-owned execution boundary](references/execution-focus.md#opus-owned-execution-boundary)`. That link stays inside the package, so the isolated-package link test still applies.

11. **Informational: AGENTS.md wording.**
    - The explicit baseline sentence "Existing frozen signed-call native gates retain their Opus 5.5 and Muse audits" (baseline AGENTS.md:60) is gone. It survives at PARALLEL-DEVELOPMENT.md:333, but AGENTS.md:9 now says historical rosters "do not select new work", which could be misread for a future run under a frozen gate. Suggest restoring the sentence after AGENTS.md:9.
    - AGENTS.md:141 ("replaces Opus for pending and future reviews") sits under the historical heading. Suggest adding "(superseded for security by the October 14 Opus assignment)".

## Evidence and acceptance limits
- **Receipts not re-run:** I did not re-run anything recorded in `root-verification.json`. That includes byte-identity of the machine registers, the strict loader, `openspec validate` and the nine guidance tests.
- **No hashes computed:** the plugin divergence in finding 1 comes from file inventories and content reads.
- **OSR "currently accepted" statements** are source readings at `cb8280ef`, not executions.
- **Uninspected:** the installed cache, `~/.local/share/moriarty-dev`, and the mutable source's `cli.py`, `store.py`, `records.py`, `policy.py`, `runner.py` and `notifications.py` bytes beyond targeted greps.

## Remaining gaps
- **Pending reviews:** the Opus coverage continuation (49 of 57 OpenSpec handoffs metadata-only; 42 wiki path entries unread by Opus) and the fresh Muse nonsecurity audit of `f14df735`.
- **Security workstreams:** WS-A through WS-D are unexecuted. Native authority is zero and Fenrir's identity is unknown.
- **Plugin runtime:** the repo runtime and the deployed hardened runtime need reconciliation before any refresh. Host hook interception is unverified.
- **Product and roadmap:** full product and security audits and every U/M/MC/SP/G exit remain open.

I made no edits. The plan-mode Write and ExitPlanMode tools weren't available in this session, so this review is delivered here only.
